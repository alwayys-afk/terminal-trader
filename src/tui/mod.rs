mod account_pnl;
pub mod command;
mod command_box;
#[cfg(test)]
mod command_test;
mod lot_detail;
mod operations_page;
mod orders;
mod positions;

use anyhow::Result;
use crossterm::{
    event::{Event, EventStream, KeyCode, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use futures::StreamExt;
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc;

use crate::app::{AccountPickerState, AppCommand, AppState, DataMsg, Page};
use crate::schwab::{Instruction, OrderType};
use crate::tui::command::{
    CommandMode, ListNav, Modifier, ParseResult, TokenFocus, TypingState, Verb, token_focus,
};
use ratatui::style::Color;
use schwab_marketdata::types::GetInstrumentsProjection;

pub fn pnl_color(v: f64) -> Color {
    if v > 0.0 {
        Color::Green
    } else if v < 0.0 {
        Color::Red
    } else {
        Color::White
    }
}

pub async fn run_tui(
    mut state: AppState,
    mut rx: mpsc::Receiver<DataMsg>,
    cmd_tx: mpsc::Sender<AppCommand>,
) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = event_loop(&mut terminal, &mut state, &mut rx, &cmd_tx).await;

    // Always restore terminal, even on error.
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

async fn event_loop(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    state: &mut AppState,
    rx: &mut mpsc::Receiver<DataMsg>,
    cmd_tx: &mpsc::Sender<AppCommand>,
) -> Result<()> {
    let mut events = EventStream::new();

    loop {
        if state.current_page == Page::Positions {
            state.positions.clamp_selection();
        }
        // Clone symbol out before the draw closure to avoid borrow conflict
        // (lot_detail::render needs &mut state).
        let lot_sym = match &state.current_page {
            Page::LotDetail(sym) => Some(sym.clone()),
            _ => None,
        };
        let state = &mut *state;
        terminal.draw(|f| {
            match &lot_sym {
                Some(sym) => lot_detail::render(f, f.area(), state, sym),
                None => match state.current_page {
                    Page::Positions => positions::render(f, f.area(), state),
                    Page::Orders => orders::render(f, f.area(), state),
                    Page::LotDetail(_) => unreachable!(),
                    Page::Operations => operations_page::render(f, f.area(), state),
                },
            }
            command_box::render_command_box(f, state);
            command_box::render_account_picker(f, state);
        })?;

        tokio::select! {
            // Data update from background task.
            Some(msg) = rx.recv() => {
                state.apply(msg);
            }

            // Crossterm keyboard / resize events.
            Some(Ok(event)) = events.next() => {
                match event {
                    Event::Key(key) => {
                        // Ctrl-C always quits, regardless of overlay/mode — otherwise a
                        // stuck picker or command bar would lock the user in.
                        if matches!(key.code, KeyCode::Char('c'))
                            && key.modifiers.contains(KeyModifiers::CONTROL)
                        {
                            return Ok(());
                        }
                        // Context-sensitive Esc handlers (clear filter, back out of a
                        // page) only apply when no overlay or input is consuming keys.
                        if matches!(state.command_mode, CommandMode::Hidden)
                            && !state.positions.filter_active
                            && state.account_picker.is_none()
                        {
                            match key.code {
                                KeyCode::Esc
                                    if matches!(state.current_page, Page::Positions)
                                        && !state.positions.filter.is_empty() =>
                                {
                                    state.positions.filter.clear();
                                    state.positions.sort_locked = false;
                                    state.positions.select_first();
                                    continue;
                                }
                                KeyCode::Esc
                                    if matches!(state.current_page, Page::LotDetail(_)) =>
                                {
                                    state.navigate_to(Page::Positions);
                                    continue;
                                }
                                KeyCode::Esc
                                    if matches!(state.current_page, Page::Operations) =>
                                {
                                    // If editing/adding in operations page, cancel that first.
                                    if state.operations_state.editing.is_some() {
                                        state.operations_state.editing = None;
                                        state.operations_state.edit_buffer.clear();
                                        continue;
                                    }
                                    if state.operations_state.editing_txn.is_some() {
                                        state.operations_state.editing_txn = None;
                                        state.operations_state.edit_buffer.clear();
                                        continue;
                                    }
                                    if state.operations_state.adding {
                                        state.operations_state.adding = false;
                                        state.operations_state.add_step = 0;
                                        state.operations_state.edit_buffer.clear();
                                        continue;
                                    }
                                    if state.operations_state.adding_txn {
                                        state.operations_state.adding_txn = false;
                                        state.operations_state.txn_add_step = 0;
                                        state.operations_state.edit_buffer.clear();
                                        continue;
                                    }
                                    state.navigate_to(Page::Positions);
                                    continue;
                                }
                                _ => {}
                            }
                        }
                        handle_key(state, key.code, key.modifiers, cmd_tx);
                    }
                    Event::Resize(_, _) => {}
                    _ => {}
                }
            }

            // Redraw at most every 100 ms even if nothing changes.
            _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {}
        }
    }
}

fn handle_key(
    state: &mut AppState,
    code: KeyCode,
    modifiers: KeyModifiers,
    cmd_tx: &mpsc::Sender<AppCommand>,
) {
    if state.account_picker.is_some() {
        handle_key_account_picker(state, code, cmd_tx);
        return;
    }
    match &state.command_mode {
        CommandMode::Hidden => {
            handle_key_hidden(state, code, modifiers, cmd_tx);
        }
        CommandMode::Typing(_) => {
            handle_key_typing(state, code, modifiers, cmd_tx);
        }
        CommandMode::Confirming { .. } => {
            handle_key_confirming(state, code, cmd_tx);
        }
        CommandMode::OrderSent(_) => {
            cmd_tx.try_send(AppCommand::ClearCmdSymbols).ok();
            state.command_mode = CommandMode::Hidden;
        }
        CommandMode::ConfirmCancel { .. } => {
            handle_key_confirm_cancel(state, code, cmd_tx);
        }
    }
}

fn handle_key_position_filter(state: &mut AppState, code: KeyCode) {
    match code {
        KeyCode::Esc => {
            if !state.positions.filter.is_empty() {
                state.positions.sort_locked = false;
            }
            state.positions.filter.clear();
            state.positions.filter_active = false;
        }
        KeyCode::Enter => {
            state.positions.filter_active = false;
        }
        KeyCode::Backspace => {
            if state.positions.filter.pop().is_some() {
                state.positions.sort_locked = false;
            }
            state.positions.select_first();
            if state.positions.filter.is_empty() {
                state.positions.filter_active = false;
            }
        }
        KeyCode::Char(c) => {
            state.positions.filter.push(c);
            state.positions.sort_locked = false;
            state.positions.select_first();
        }
        _ => {}
    }
}

fn handle_key_hidden(
    state: &mut AppState,
    code: KeyCode,
    _modifiers: KeyModifiers,
    cmd_tx: &mpsc::Sender<AppCommand>,
) {
    // Position filter input takes priority when active.
    if state.positions.filter_active {
        handle_key_position_filter(state, code);
        return;
    }

    // Global: copy error to clipboard.
    if code == KeyCode::Char('y') && state.status.starts_with("Error:") {
        copy_to_clipboard(&state.status);
        state.status = "Copied to clipboard".to_string();
        return;
    }

    // Account picker is global — works from any page.
    if code == KeyCode::Char('\\') {
        open_account_picker(state);
        return;
    }

    // Operations page has its own key handler.
    if state.current_page == Page::Operations {
        handle_key_operations(state, code, cmd_tx);
        return;
    }

    match code {
        // q and Ctrl-C are handled in the event loop (they return Ok(())).
        // Esc from LotDetail is handled in the event loop too.
        KeyCode::Char('1') => state.navigate_to(Page::Positions),
        KeyCode::Char('2') => state.navigate_to(Page::Orders),
        KeyCode::Char('3') => {
            state.operations_state.rebuild_timeline(&state.lot_store);
            state.navigate_to(Page::Operations);
        }
        KeyCode::Char('u') => {
            state.nav_history.go_back(&mut state.current_page);
        }
        KeyCode::Char('[') => {
            state.nav_history.go_forward(&mut state.current_page);
        }
        KeyCode::Char('a') if state.current_page == Page::Positions => {
            state.positions.show_accumulation = !state.positions.show_accumulation;
        }
        KeyCode::Char('L') if state.current_page == Page::Positions => {
            state.toggle_sort_lock();
        }
        KeyCode::Char('P') if state.current_page == Page::Positions => {
            state.positions.show_pnl = !state.positions.show_pnl;
        }
        KeyCode::Char('/') if state.current_page == Page::Positions => {
            if !state.positions.filter.is_empty() {
                state.positions.sort_locked = false;
            }
            state.positions.filter.clear();
            state.positions.filter_active = true;
        }
        // Prefill buy/sell order from selected position.
        KeyCode::Char('b') if state.current_page == Page::Positions => {
            if let Some(symbol) = state.selected_position_symbol() {
                open_position_order(state, cmd_tx, symbol, Verb::Buy, None);
            } else {
                state.status = "No position selected".to_string();
            }
        }
        KeyCode::Char('s') if state.current_page == Page::Positions => {
            if let Some(symbol) = state.selected_position_symbol() {
                open_position_order(state, cmd_tx, symbol, Verb::Sell, None);
            } else {
                state.status = "No position selected".to_string();
            }
        }
        KeyCode::Char('f') if state.current_page == Page::Positions => {
            if let Some(symbol) = state.selected_position_symbol() {
                if let Some(qty) = state.positions.held_qty(&symbol) {
                    open_position_order(state, cmd_tx, symbol, Verb::Sell, Some(qty));
                } else {
                    state.status = format!("No held quantity for {symbol}");
                }
            } else {
                state.status = "No position selected".to_string();
            }
        }
        KeyCode::Up if state.current_page == Page::Positions => {
            state.positions.select_prev();
        }
        KeyCode::Down if state.current_page == Page::Positions => {
            state.positions.select_next();
        }
        KeyCode::Char('g') if state.current_page == Page::Positions => {
            state.positions.select_first();
        }
        KeyCode::Char('G') if state.current_page == Page::Positions => {
            state.positions.select_last();
        }
        KeyCode::Enter if state.current_page == Page::Positions => {
            // Open lot detail for the selected position.
            if let Some(symbol) = positions::selected_symbol(state) {
                state.lot_detail_scroll = 0;
                state.navigate_to(Page::LotDetail(symbol));
            }
        }
        KeyCode::Up if matches!(state.current_page, Page::LotDetail(_)) => {
            state.lot_detail_scroll = state.lot_detail_scroll.saturating_sub(1);
        }
        KeyCode::Down if matches!(state.current_page, Page::LotDetail(_)) => {
            state.lot_detail_scroll = state.lot_detail_scroll.saturating_add(1);
        }
        KeyCode::Char('g') if matches!(state.current_page, Page::LotDetail(_)) => {
            state.lot_detail_scroll = 0;
        }
        KeyCode::Char('G') if matches!(state.current_page, Page::LotDetail(_)) => {
            state.lot_detail_scroll = usize::MAX;
        }
        KeyCode::Char('b') if matches!(state.current_page, Page::LotDetail(_)) => {
            if let Page::LotDetail(sym) = &state.current_page {
                open_position_order(state, cmd_tx, sym.clone(), Verb::Buy, None);
            }
        }
        KeyCode::Char('s') if matches!(state.current_page, Page::LotDetail(_)) => {
            if let Page::LotDetail(sym) = &state.current_page {
                open_position_order(state, cmd_tx, sym.clone(), Verb::Sell, None);
            }
        }
        KeyCode::Up if state.current_page == Page::Orders => {
            state.orders.select_prev();
        }
        KeyCode::Down if state.current_page == Page::Orders => {
            state.orders.select_next();
        }
        KeyCode::Char('g') if state.current_page == Page::Orders => {
            state.orders.select_first();
        }
        KeyCode::Char('G') if state.current_page == Page::Orders => {
            state.orders.select_last();
        }
        KeyCode::Char('x') if state.current_page == Page::Orders => {
            let order = state
                .orders
                .selected_item()
                .filter(|o| o.cancelable)
                .cloned();
            if let Some(order) = order {
                let summary = order_summary(&order);
                state.command_mode = CommandMode::ConfirmCancel {
                    order_id: order.order_id,
                    summary,
                };
            }
        }
        KeyCode::Char('e') if state.current_page == Page::Orders => {
            let order = state.orders.selected_item().filter(|o| o.editable).cloned();
            if let Some(order) = order {
                open_order_dialog(state, &order, cmd_tx, true, false, false);
            }
        }
        KeyCode::Char('r') if state.current_page == Page::Orders => {
            let order = state.orders.selected_item().cloned();
            if let Some(order) = order {
                open_order_dialog(state, &order, cmd_tx, false, false, true);
            }
        }
        KeyCode::Char('R') if state.current_page == Page::Orders => {
            let order = state.orders.selected_item().cloned();
            if let Some(order) = order {
                open_order_dialog(state, &order, cmd_tx, false, false, false);
            }
        }
        KeyCode::Char('o') if state.current_page == Page::Orders => {
            let order = state.orders.selected_item().cloned();
            if let Some(order) = order {
                open_order_dialog(state, &order, cmd_tx, false, true, false);
            }
        }
        // Jump from order to its position.
        KeyCode::Char('p') if state.current_page == Page::Orders => {
            if let Some(order) = state.orders.selected_item().cloned() {
                if let Some(idx) = state.position_index_for_symbol(&order.symbol) {
                    state.navigate_to(Page::Positions);
                    *state.positions.selected_idx_mut() = idx;
                } else {
                    state.status = format!("No position for {}", order.symbol);
                }
            } else {
                state.status = "No order selected".to_string();
            }
        }
        KeyCode::Char(':') => {
            state.command_mode = CommandMode::Typing(TypingState::new(None));
        }
        _ => {}
    }
}

/// Open the account picker overlay if there's more than one account to choose
/// from. Selection starts on the currently active account.
fn open_account_picker(state: &mut AppState) {
    if state.accounts.len() <= 1 {
        state.status = "Only one account available".to_string();
        return;
    }
    let selected = state
        .accounts
        .iter()
        .position(|a| a.hash == state.current_account)
        .unwrap_or(0);
    state.account_picker = Some(AccountPickerState { selected });
}

fn handle_key_account_picker(
    state: &mut AppState,
    code: KeyCode,
    cmd_tx: &mpsc::Sender<AppCommand>,
) {
    let Some(picker) = state.account_picker.as_mut() else {
        return;
    };
    let count = state.accounts.len();
    match code {
        KeyCode::Esc => {
            state.account_picker = None;
        }
        KeyCode::Up => {
            if picker.selected > 0 {
                picker.selected -= 1;
            }
        }
        KeyCode::Down => {
            if picker.selected + 1 < count {
                picker.selected += 1;
            }
        }
        KeyCode::Enter => {
            let chosen = state.accounts.get(picker.selected).cloned();
            state.account_picker = None;
            if let Some(acct) = chosen {
                if acct.hash == state.current_account {
                    return;
                }
                if let Err(e) = cmd_tx.try_send(AppCommand::SwitchAccount(acct.hash)) {
                    state.status = format!("Error: failed to switch account: {e}");
                }
            }
        }
        _ => {}
    }
}

/// Search the local symbol_db / word_index.
/// Always runs both a symbol-prefix search and a word-prefix search, merging results.
/// Symbol prefix hits come first; word hits fill remaining slots up to LIMIT.
/// If I type tsla I want TSLA, then anything with tsla as a word (e.g. the etfs with tesla)
fn local_search(
    query: &str,
    symbol_db: &std::collections::BTreeMap<String, String>,
    word_index: &std::collections::BTreeMap<String, Vec<String>>,
) -> Vec<(String, String)> {
    const LIMIT: usize = 30;
    // seen is to dedupe results between the symbol_db and word_index searches, since some symbols may also appear as words in the word_index
    let mut seen = std::collections::HashSet::new();

    let mut results = Vec::new();

    // Symbol prefix search.
    let upper = query.to_uppercase();
    for (sym, desc) in symbol_db
        .range(upper.clone()..)
        .take_while(|(k, _)| k.starts_with(&upper))
        .take(LIMIT)
    {
        seen.insert(sym.clone());
        results.push((sym.clone(), desc.clone()));
    }

    // Word prefix search.
    if results.len() < LIMIT {
        let lower = query.to_lowercase();
        'outer: for (_, syms) in word_index
            .range(lower.clone()..)
            .take_while(|(k, _)| k.starts_with(&lower))
        {
            for sym in syms {
                if seen.insert(sym.clone())
                    && let Some(desc) = symbol_db.get(sym)
                {
                    results.push((sym.clone(), desc.clone()));
                    if results.len() >= LIMIT {
                        break 'outer;
                    }
                }
            }
        }
    }

    results
}

/// Run a local search for `sym`, falling back to the API if empty, and update suggestions.
fn update_suggestions(state: &mut AppState, cmd_tx: &mpsc::Sender<AppCommand>, sym: String) {
    let results = local_search(&sym, &state.symbol_db, &state.word_index);
    if results.is_empty() {
        // Fire both DescSearch and SymbolSearch in parallel.
        for projection in [
            GetInstrumentsProjection::DescSearch,
            GetInstrumentsProjection::SymbolSearch,
        ] {
            if let Err(e) = cmd_tx.try_send(AppCommand::SearchInstruments {
                query: sym.clone(),
                projection,
            }) {
                tracing::warn!("failed to send SearchInstruments: {e}");
            }
        }
    } else if let CommandMode::Typing(ref mut ts) = state.command_mode {
        ts.autocomplete.set_suggestions(results);
    }
}

/// Keep autocomplete in sync with the current token focus.
/// Called after every input change (char insert, backspace).
fn sync_autocomplete_to_focus(state: &mut AppState) {
    if let CommandMode::Typing(ref mut ts) = state.command_mode {
        let focus = token_focus(&ts.input, ts.cursor);
        match focus {
            TokenFocus::Amount | TokenFocus::Verb | TokenFocus::Modifier => {
                ts.autocomplete.clear();
            }
            TokenFocus::Symbol => {} // handled by update_suggestions
        }
    }
}

fn handle_key_typing(
    state: &mut AppState,
    code: KeyCode,
    modifiers: KeyModifiers,
    cmd_tx: &mpsc::Sender<AppCommand>,
) {
    let alt = modifiers.contains(KeyModifiers::ALT);
    let super_key = modifiers.contains(KeyModifiers::SUPER);
    match code {
        // feel like this actually makes more sense inverted where it's for the mode I'm in what are the shortcuts
        KeyCode::Esc => {
            cmd_tx.try_send(AppCommand::ClearCmdSymbols).ok();
            state.command_mode = CommandMode::Hidden;
        }
        KeyCode::Up => {
            if let CommandMode::Typing(ref mut ts) = state.command_mode {
                ts.autocomplete.select_prev();
            }
        }
        KeyCode::Down => {
            if let CommandMode::Typing(ref mut ts) = state.command_mode {
                ts.autocomplete.select_next();
            }
        }
        KeyCode::Left => {
            if let CommandMode::Typing(ref mut ts) = state.command_mode
                && ts.cursor > 0
            {
                if alt || super_key {
                    ts.cursor = word_boundary_left(&ts.input, ts.cursor);
                } else {
                    ts.cursor = prev_char_boundary(&ts.input, ts.cursor);
                }
            }
        }
        KeyCode::Right => {
            if let CommandMode::Typing(ref mut ts) = state.command_mode
                && ts.cursor < ts.input.len()
            {
                if alt || super_key {
                    ts.cursor = word_boundary_right(&ts.input, ts.cursor);
                } else {
                    ts.cursor = next_char_boundary(&ts.input, ts.cursor);
                }
            }
        }
        KeyCode::Tab => {
            // Complete the selected suggestion into the input.
            let action = if let CommandMode::Typing(ref ts) = state.command_mode {
                if let Some((sel_sym, _)) = ts.autocomplete.selected_item() {
                    let selected = sel_sym.clone();
                    let tokens: Vec<&str> = ts.input.split_whitespace().collect();
                    let new_input = if tokens.len() >= 2 {
                        let mut parts = vec![tokens[0].to_string(), selected.clone()];
                        parts.extend(tokens[2..].iter().map(|s| s.to_string()));
                        parts.join(" ")
                    } else if !tokens.is_empty() {
                        format!("{} {}", tokens[0], selected)
                    } else {
                        selected.clone()
                    };
                    let needs_sub = ts.last_fetched_sym.as_deref() != Some(&selected);
                    Some((selected, new_input, needs_sub))
                } else {
                    None
                }
            } else {
                None
            };
            if let Some((selected, new_input, needs_sub)) = action {
                let in_db = state.symbol_db.contains_key(&selected);
                if let CommandMode::Typing(ref mut ts) = state.command_mode {
                    ts.set_input(new_input);
                    ts.last_searched_sym = Some(selected.clone());
                    if needs_sub {
                        ts.last_fetched_sym = Some(selected.clone());
                    }
                    ts.autocomplete.clear();
                }
                if needs_sub
                    && let Err(e) = cmd_tx.try_send(AppCommand::SubscribeToQuote(selected.clone()))
                {
                    tracing::warn!("failed to send SubscribeToQuote: {e}");
                }
                // Learn description if this symbol came from API fallback suggestions.
                if !in_db
                    && let Err(e) = cmd_tx.try_send(AppCommand::SearchInstruments {
                        query: selected,
                        projection: GetInstrumentsProjection::SymbolSearch,
                    })
                {
                    tracing::warn!("failed to send SearchInstruments: {e}");
                }
            }
        }
        KeyCode::Backspace => {
            let search_sym = if let CommandMode::Typing(ref mut ts) = state.command_mode
                && ts.delete_back()
            {
                let sym = ts.parsed_symbol();
                if sym != ts.last_searched_sym {
                    ts.last_searched_sym = sym.clone();
                    ts.autocomplete.clear();
                    sym
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(sym) = search_sym {
                update_suggestions(state, cmd_tx, sym);
            }
            sync_autocomplete_to_focus(state);
        }
        KeyCode::Enter => {
            // Extract the parsed command and replace_order_id without holding a borrow.
            let ready = if let CommandMode::Typing(ref ts) = state.command_mode
                && let ParseResult::Valid(cmd) = &ts.parsed
                && cmd.amount.is_some()
            {
                Some((cmd.clone(), ts.input.clone(), ts.replace_order_id))
            } else {
                None
            };
            // If the command is ready to submit, go straight to confirmation.
            // Otherwise, if suggestions are visible, Enter completes the selection (like Tab).
            if ready.is_none() {
                let has_suggestions = matches!(
                    &state.command_mode,
                    CommandMode::Typing(ts) if ts.autocomplete.has_suggestions()
                );
                if has_suggestions {
                    handle_key_typing(state, KeyCode::Tab, KeyModifiers::NONE, cmd_tx);
                    return;
                }
            }
            if let Some((cmd, input, replace_order_id)) = ready {
                let price = match &cmd.modifier {
                    Modifier::Market => Some(0.0),
                    Modifier::Limit(p) => Some(*p),
                    Modifier::Mid => state
                        .stream
                        .quotes
                        .get(&cmd.symbol)
                        .map(|q| (q.bid_price + q.ask_price) / 2.0),
                    Modifier::Ask => state.stream.quotes.get(&cmd.symbol).map(|q| q.ask_price),
                    Modifier::Bid => state.stream.quotes.get(&cmd.symbol).map(|q| q.bid_price),
                };
                if let Some(p) = price {
                    let rp = round_price(p);
                    let held_qty = state.positions.held_qty(&cmd.symbol);
                    let shares = command_box::compute_confirmed_shares(&cmd, rp, held_qty);
                    if shares < 1.0 {
                        state.status = if held_qty.is_none()
                            && matches!(cmd.amount, Some(crate::tui::command::Amount::Percent(_)))
                        {
                            format!(
                                "Error: no position found for {} to compute percent",
                                cmd.symbol
                            )
                        } else {
                            "Error: order resolves to 0 shares".to_string()
                        };
                        return;
                    }
                    state.command_mode = CommandMode::Confirming {
                        cmd,
                        price: rp,
                        input,
                        replace_order_id,
                    };
                } else {
                    state.status = "Waiting for quote data, try again in a moment".to_string();
                }
            }
        }
        // Ctrl+A / Ctrl+E: jump to start / end of line.
        KeyCode::Char('a') if modifiers.contains(KeyModifiers::CONTROL) => {
            handle_key_typing(state, KeyCode::Home, KeyModifiers::NONE, cmd_tx);
        }
        KeyCode::Char('e') if modifiers.contains(KeyModifiers::CONTROL) => {
            handle_key_typing(state, KeyCode::End, KeyModifiers::NONE, cmd_tx);
        }
        // Ctrl+W: delete previous word, Ctrl+U: delete to start of line.
        KeyCode::Char('w') if modifiers.contains(KeyModifiers::CONTROL) => {
            handle_key_typing(state, KeyCode::Backspace, KeyModifiers::ALT, cmd_tx);
        }
        KeyCode::Char('u') if modifiers.contains(KeyModifiers::CONTROL) => {
            handle_key_typing(state, KeyCode::Backspace, KeyModifiers::SUPER, cmd_tx);
        }
        KeyCode::Char(c) => {
            let (subscribe_sym, search_sym) = if let CommandMode::Typing(ref mut ts) =
                state.command_mode
            {
                ts.insert_char(c);
                let sym = ts.parsed_symbol();
                // On space past the symbol, commit and subscribe to its quote stream.
                let focus = token_focus(&ts.input, ts.cursor);
                let subscribe_sym = if matches!(focus, TokenFocus::Amount | TokenFocus::Modifier)
                    && sym != ts.last_fetched_sym
                {
                    ts.last_fetched_sym = sym.clone();
                    sym.clone()
                } else {
                    None
                };
                let search_sym = if sym != ts.last_searched_sym {
                    ts.last_searched_sym = sym.clone();
                    ts.autocomplete.clear();
                    sym
                } else {
                    None
                };
                (subscribe_sym, search_sym)
            } else {
                (None, None)
            };

            if let Some(sym) = subscribe_sym {
                if let Err(e) = cmd_tx.try_send(AppCommand::SubscribeToQuote(sym.clone())) {
                    tracing::warn!("failed to send SubscribeToQuote: {e}");
                }
                if !state.symbol_db.contains_key(&sym)
                    && let Err(e) = cmd_tx.try_send(AppCommand::SearchInstruments {
                        query: sym,
                        projection: GetInstrumentsProjection::SymbolSearch,
                    })
                {
                    tracing::warn!("failed to send SearchInstruments: {e}");
                }
            }
            if let Some(sym) = search_sym {
                update_suggestions(state, cmd_tx, sym);
            }
            sync_autocomplete_to_focus(state);
        }
        _ => {}
    }
}

/// Return to the Typing state with the original input preserved.
fn confirming_to_typing(state: &mut AppState) {
    if let CommandMode::Confirming {
        cmd,
        input,
        replace_order_id,
        ..
    } = &state.command_mode
    {
        let symbol = cmd.symbol.clone();
        let mut ts = TypingState::with_input(input.clone(), *replace_order_id);
        ts.last_fetched_sym = Some(symbol.clone());
        ts.last_searched_sym = Some(symbol);
        state.command_mode = CommandMode::Typing(ts);
    }
}

fn handle_key_confirming(state: &mut AppState, code: KeyCode, cmd_tx: &mpsc::Sender<AppCommand>) {
    match code {
        KeyCode::Esc => {
            cmd_tx.try_send(AppCommand::ClearCmdSymbols).ok();
            state.command_mode = CommandMode::Hidden;
        }
        KeyCode::Char('e') => {
            confirming_to_typing(state);
        }
        KeyCode::Enter => {
            // Extract data before dropping the borrow. Sells may expand into two
            // orders (long-term FIFO + short-term LIFO); buys stay a single order.
            let app_cmds: Vec<AppCommand> = match &state.command_mode {
                CommandMode::Confirming {
                    cmd,
                    price,
                    replace_order_id,
                    ..
                } => {
                    let price = *price;
                    let instruction = match cmd.verb {
                        Verb::Buy => Instruction::Buy,
                        Verb::Sell => Instruction::Sell,
                    };
                    let held_qty = state.positions.held_qty(&cmd.symbol);
                    let quantity = command_box::compute_confirmed_shares(cmd, price, held_qty);
                    let limit_price = match &cmd.modifier {
                        Modifier::Market => None,
                        _ => {
                            if price > 0.0 {
                                Some(price)
                            } else {
                                None
                            }
                        }
                    };

                    match replace_order_id {
                        // The LT/ST split is a placement-time decision; by now
                        // these are just independent orders. A replace edits one
                        // existing order, so carry its method forward unchanged —
                        // there's no spanning sale left to re-split.
                        Some(id) => {
                            let method = state
                                .orders
                                .items()
                                .iter()
                                .find(|o| o.order_id == *id)
                                .and_then(|o| o.tax_lot_method);
                            vec![AppCommand::ReplaceOrder {
                                order_id: *id,
                                symbol: cmd.symbol.clone(),
                                instruction,
                                quantity,
                                limit_price,
                                tax_lot_method: method,
                            }]
                        }
                        // A new sell auto-selects the method per leg and splits
                        // into one order per leg when it spans long-term and
                        // short-term lots. Buys carry no method.
                        None => {
                            let legs: Vec<(f64, Option<crate::schwab::LotSelectionMethod>)> =
                                if cmd.verb == Verb::Sell {
                                    crate::app::compute_sell_legs(
                                        state.lot_store.lots_for(&cmd.symbol),
                                        quantity,
                                    )
                                    .into_iter()
                                    .map(|l| (l.quantity, Some(l.method)))
                                    .collect()
                                } else {
                                    vec![(quantity, None)]
                                };
                            legs.into_iter()
                                .map(|(qty, method)| AppCommand::PlaceOrder {
                                    symbol: cmd.symbol.clone(),
                                    instruction,
                                    quantity: qty,
                                    limit_price,
                                    tax_lot_method: method,
                                })
                                .collect()
                        }
                    }
                }
                _ => Vec::new(),
            };
            if !app_cmds.is_empty() {
                for cmd in app_cmds {
                    if let Err(e) = cmd_tx.try_send(cmd) {
                        state.status = format!("Error: failed to send order: {e}");
                        return;
                    }
                }
                cmd_tx.try_send(AppCommand::ClearCmdSymbols).ok();
                state.command_mode = CommandMode::Hidden;
            }
        }
        _ => {}
    }
}

fn handle_key_confirm_cancel(
    state: &mut AppState,
    code: KeyCode,
    cmd_tx: &mpsc::Sender<AppCommand>,
) {
    match code {
        KeyCode::Esc => {
            state.command_mode = CommandMode::Hidden;
        }
        KeyCode::Enter => {
            if let CommandMode::ConfirmCancel { order_id, .. } = state.command_mode
                && let Err(e) = cmd_tx.try_send(AppCommand::CancelOrder { order_id })
            {
                state.status = format!("Error: failed to send cancel: {e}");
                return;
            }
            cmd_tx.try_send(AppCommand::ClearCmdSymbols).ok();
            state.command_mode = CommandMode::Hidden;
        }
        _ => {}
    }
}

/// Build and open the order-entry dialog pre-filled from an existing order.
/// `as_replace` — confirming replaces the original order (edit); false = new order.
/// `flip_side`  — invert buy↔sell (opposite order).
/// `drop_price` — omit the price (repeat for DCA); uses current market price.
fn open_order_dialog(
    state: &mut AppState,
    order: &crate::schwab::Order,
    cmd_tx: &mpsc::Sender<AppCommand>,
    as_replace: bool,
    flip_side: bool,
    drop_price: bool,
) {
    let is_buy = matches!(
        order.instruction,
        Instruction::Buy
            | Instruction::BuyToCover
            | Instruction::BuyToOpen
            | Instruction::BuyToClose
    );
    let verb = match is_buy ^ flip_side {
        true => "buy",
        false => "sell",
    };
    let input = if drop_price {
        format!("{verb} {} {:.0}", order.symbol, order.quantity)
    } else {
        let modifier = match order.order_type {
            OrderType::Market | OrderType::MarketOnClose => "market".to_string(),
            _ => order
                .price
                .map(|p| format!("{p:.2}"))
                .unwrap_or_else(|| "market".to_string()),
        };
        format!("{verb} {} {:.0} {modifier}", order.symbol, order.quantity)
    };
    let symbol = order.symbol.clone();
    let replace_order_id = if as_replace {
        Some(order.order_id)
    } else {
        None
    };
    if let Err(e) = cmd_tx.try_send(AppCommand::SubscribeToQuote(symbol.clone())) {
        tracing::warn!("failed to send SubscribeToQuote: {e}");
    }
    let mut ts = TypingState::with_input(input, replace_order_id);
    ts.last_fetched_sym = Some(symbol.clone());
    ts.last_searched_sym = Some(symbol);
    state.command_mode = CommandMode::Typing(ts);
}

/// Open the order dialog pre-filled for a position shortcut (buy/sell/flatten).
fn open_position_order(
    state: &mut AppState,
    cmd_tx: &mpsc::Sender<AppCommand>,
    symbol: String,
    verb: Verb,
    qty: Option<f64>,
) {
    let input = match qty {
        Some(q) => format!("{verb} {symbol} {q:.0}"),
        None => format!("{verb} {symbol} "),
    };
    if let Err(e) = cmd_tx.try_send(AppCommand::SubscribeToQuote(symbol.clone())) {
        tracing::warn!("failed to send SubscribeToQuote: {e}");
    }
    let mut ts = TypingState::with_input(input, None);
    ts.last_fetched_sym = Some(symbol.clone());
    ts.last_searched_sym = Some(symbol);
    state.command_mode = CommandMode::Typing(ts);
}

/// Move cursor back one character.
fn prev_char_boundary(s: &str, pos: usize) -> usize {
    s[..pos]
        .char_indices()
        .next_back()
        .map(|(i, _)| i)
        .unwrap_or(0)
}

/// Move cursor forward one character.
fn next_char_boundary(s: &str, pos: usize) -> usize {
    s[pos..]
        .char_indices()
        .nth(1)
        .map(|(i, _)| pos + i)
        .unwrap_or(s.len())
}

/// Find the start of the previous word (for Alt+Left / Alt+Backspace).
fn word_boundary_left(s: &str, pos: usize) -> usize {
    let before = &s[..pos];
    // Skip trailing whitespace, then skip non-whitespace.
    let trimmed = before.trim_end();
    if trimmed.is_empty() {
        return 0;
    }
    trimmed
        .rfind(char::is_whitespace)
        .map(|i| i + 1)
        .unwrap_or(0)
}

/// Find the start of the next word (for Alt+Right).
fn word_boundary_right(s: &str, pos: usize) -> usize {
    let after = &s[pos..];
    // Skip current non-whitespace, then skip whitespace.
    let skip_word = after.find(char::is_whitespace).unwrap_or(after.len());
    let rest = &after[skip_word..];
    let skip_space = rest
        .find(|c: char| !c.is_whitespace())
        .unwrap_or(rest.len());
    pos + skip_word + skip_space
}

fn copy_to_clipboard(text: &str) {
    use std::io::Write;
    use std::process::{Command, Stdio};
    if let Ok(mut child) = Command::new("pbcopy").stdin(Stdio::piped()).spawn() {
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes()).ok();
        }
        child.wait().ok();
    }
}

/// Round a limit price to the max decimals allowed by the exchange:
/// 2 decimals for prices >= $1, 4 decimals for prices < $1.
fn round_price(p: f64) -> f64 {
    if p >= 1.0 {
        (p * 100.0).round() / 100.0
    } else {
        (p * 10000.0).round() / 10000.0
    }
}

fn handle_key_operations(state: &mut AppState, code: KeyCode, cmd_tx: &mpsc::Sender<AppCommand>) {
    use crate::app::LotCommand;
    use crate::app::lots::{OperationKind, TradeAction};
    use crate::app::operations::{OpEditField, TimelineEntry, TxnEditField};

    // Adding a new transaction — sequential field input.
    if state.operations_state.adding_txn {
        match code {
            KeyCode::Esc => {
                state.operations_state.adding_txn = false;
                state.operations_state.txn_add_step = 0;
                state.operations_state.edit_buffer.clear();
                state.operations_state.txn_add_buffer = Default::default();
            }
            KeyCode::Enter => {
                let val = state.operations_state.edit_buffer.clone();
                match state.operations_state.txn_add_step {
                    0 => state.operations_state.txn_add_buffer.date = val,
                    1 => state.operations_state.txn_add_buffer.action = val,
                    2 => state.operations_state.txn_add_buffer.symbol = val,
                    3 => state.operations_state.txn_add_buffer.quantity = val,
                    4 => {
                        state.operations_state.txn_add_buffer.price = val;
                        finalize_add_transaction(state, cmd_tx);
                        return;
                    }
                    _ => {}
                }
                state.operations_state.txn_add_step += 1;
                state.operations_state.edit_buffer.clear();
            }
            KeyCode::Backspace => {
                state.operations_state.edit_buffer.pop();
            }
            KeyCode::Char(c) => {
                state.operations_state.edit_buffer.push(c);
            }
            _ => {}
        }
        return;
    }

    // Editing a transaction field inline.
    if let Some(txn_field) = state.operations_state.editing_txn {
        match code {
            KeyCode::Esc => {
                state.operations_state.editing_txn = None;
                state.operations_state.edit_buffer.clear();
                state.operations_state.pending_txn_edits.clear();
            }
            KeyCode::Tab => {
                // Save current field value to pending edits, advance to next.
                state
                    .operations_state
                    .pending_txn_edits
                    .insert(txn_field, state.operations_state.edit_buffer.clone());
                let next = match txn_field {
                    TxnEditField::Symbol => TxnEditField::Quantity,
                    TxnEditField::Quantity => TxnEditField::Price,
                    TxnEditField::Price => TxnEditField::LotMethod,
                    TxnEditField::LotMethod => TxnEditField::Symbol,
                };
                // Pre-fill buffer: use pending value if we already edited this field,
                // otherwise read from the lot store.
                let default_method = state.lot_store.default_lot_method;
                state.operations_state.edit_buffer =
                    if let Some(val) = state.operations_state.pending_txn_edits.get(&next) {
                        val.clone()
                    } else if let Some(TimelineEntry::Transaction { txn_index }) = state
                        .operations_state
                        .timeline
                        .get(state.operations_state.selected)
                    {
                        state
                            .lot_store
                            .applied_transactions
                            .get(*txn_index)
                            .map(|txn| match next {
                                TxnEditField::Symbol => txn.symbol.clone(),
                                TxnEditField::Quantity => format!("{:.0}", txn.quantity),
                                TxnEditField::Price => format!("{:.4}", txn.price),
                                TxnEditField::LotMethod => {
                                    txn.lot_method.unwrap_or(default_method).to_string()
                                }
                            })
                            .unwrap_or_default()
                    } else {
                        String::new()
                    };
                state.operations_state.editing_txn = Some(next);
            }
            KeyCode::Enter => {
                // Save current field, then send the full edit as a command.
                state
                    .operations_state
                    .pending_txn_edits
                    .insert(txn_field, state.operations_state.edit_buffer.clone());
                finalize_txn_edit(state, cmd_tx);
                state.operations_state.editing_txn = None;
                state.operations_state.edit_buffer.clear();
                state.operations_state.pending_txn_edits.clear();
            }
            KeyCode::Backspace => {
                state.operations_state.edit_buffer.pop();
            }
            KeyCode::Char(c) => {
                state.operations_state.edit_buffer.push(c);
            }
            _ => {}
        }
        return;
    }

    // Adding a new operation — sequential field input.
    if state.operations_state.adding {
        match code {
            KeyCode::Esc => {
                state.operations_state.adding = false;
                state.operations_state.add_step = 0;
                state.operations_state.edit_buffer.clear();
                state.operations_state.add_buffer = Default::default();
            }
            KeyCode::Enter => {
                let val = state.operations_state.edit_buffer.clone();
                let is_namechange = state.operations_state.add_buffer.kind == "n";
                match state.operations_state.add_step {
                    0 => state.operations_state.add_buffer.date = val,
                    1 => {
                        let k = val.to_lowercase();
                        state.operations_state.add_buffer.kind = match k.as_str() {
                            "n" | "namechange" | "name change" | "nc" => "n".to_string(),
                            _ => "m".to_string(),
                        };
                    }
                    2 if is_namechange => state.operations_state.add_buffer.old_symbol = val,
                    3 if is_namechange => {
                        state.operations_state.add_buffer.new_symbol = val;
                        finalize_add_operation(state, cmd_tx);
                        return;
                    }
                    2 => state.operations_state.add_buffer.symbol = val,
                    3 => state.operations_state.add_buffer.old_qty = val,
                    4 => {
                        state.operations_state.add_buffer.new_qty = val;
                        finalize_add_operation(state, cmd_tx);
                        return;
                    }
                    _ => {}
                }
                state.operations_state.add_step += 1;
                state.operations_state.edit_buffer.clear();
            }
            KeyCode::Backspace => {
                state.operations_state.edit_buffer.pop();
            }
            KeyCode::Char(c) => {
                state.operations_state.edit_buffer.push(c);
            }
            _ => {}
        }
        return;
    }

    // Editing an operation field inline.
    if state.operations_state.editing.is_some() {
        match code {
            KeyCode::Esc => {
                state.operations_state.editing = None;
                state.operations_state.edit_buffer.clear();
            }
            KeyCode::Enter => {
                let new_val = state.operations_state.edit_buffer.trim().to_string();
                if let Some(TimelineEntry::Operation { op_index }) = state
                    .operations_state
                    .timeline
                    .get(state.operations_state.selected)
                {
                    let op_index = *op_index;
                    if let Some(op) = state.lot_store.operations.get(op_index) {
                        // Build updated kind from existing + edited field.
                        let mut kind = op.kind.clone();
                        match state.operations_state.editing {
                            Some(OpEditField::Symbol1) => {
                                let val = new_val.to_uppercase();
                                match &mut kind {
                                    OperationKind::Multiplier { symbol, .. } => *symbol = val,
                                    OperationKind::NameChange { old_symbol, .. } => {
                                        *old_symbol = val
                                    }
                                }
                            }
                            Some(OpEditField::Symbol2) => {
                                if let OperationKind::NameChange { new_symbol, .. } = &mut kind {
                                    *new_symbol = new_val.to_uppercase();
                                }
                            }
                            Some(OpEditField::OldQty) => {
                                if let OperationKind::Multiplier { old_qty, .. } = &mut kind {
                                    match new_val.parse::<f64>() {
                                        Ok(q) if q > 0.0 => *old_qty = q,
                                        _ => {
                                            state.status = "Invalid quantity".to_string();
                                            state.operations_state.editing = None;
                                            state.operations_state.edit_buffer.clear();
                                            return;
                                        }
                                    }
                                }
                            }
                            Some(OpEditField::NewQty) => {
                                if let OperationKind::Multiplier { new_qty, .. } = &mut kind {
                                    match new_val.parse::<f64>() {
                                        Ok(q) if q > 0.0 => *new_qty = q,
                                        _ => {
                                            state.status = "Invalid quantity".to_string();
                                            state.operations_state.editing = None;
                                            state.operations_state.edit_buffer.clear();
                                            return;
                                        }
                                    }
                                }
                            }
                            None => {}
                        }
                        cmd_tx
                            .try_send(AppCommand::LotStore(LotCommand::EditOperation {
                                op_index,
                                kind,
                            }))
                            .ok();
                    }
                }
                state.operations_state.editing = None;
                state.operations_state.edit_buffer.clear();
            }
            KeyCode::Backspace => {
                state.operations_state.edit_buffer.pop();
            }
            KeyCode::Char(c) => {
                state.operations_state.edit_buffer.push(c);
            }
            _ => {}
        }
        return;
    }

    // Any key that isn't d/y cancels a pending delete confirmation.
    if state.operations_state.confirm_delete
        && !matches!(code, KeyCode::Char('d') | KeyCode::Char('y'))
    {
        state.operations_state.confirm_delete = false;
    }

    // Normal navigation and commands.
    match code {
        KeyCode::Char('1') => state.navigate_to(Page::Positions),
        KeyCode::Char('2') => state.navigate_to(Page::Orders),
        KeyCode::Char('u') => {
            state.nav_history.go_back(&mut state.current_page);
        }
        KeyCode::Char('[') => {
            state.nav_history.go_forward(&mut state.current_page);
        }
        KeyCode::Tab => {
            state.operations_state.recon_focus = !state.operations_state.recon_focus;
        }
        KeyCode::Up => {
            if state.operations_state.recon_focus {
                if state.operations_state.recon_selected > 0 {
                    state.operations_state.recon_selected -= 1;
                }
            } else if state.operations_state.selected > 0 {
                state.operations_state.selected -= 1;
            }
        }
        KeyCode::Down => {
            if state.operations_state.recon_focus {
                let len = state.operations_state.recon_count;
                if len > 0 && state.operations_state.recon_selected < len - 1 {
                    state.operations_state.recon_selected += 1;
                }
            } else {
                let len = state.operations_state.timeline.len();
                if len > 0 && state.operations_state.selected < len - 1 {
                    state.operations_state.selected += 1;
                }
            }
        }
        KeyCode::Char('g') => {
            if state.operations_state.recon_focus {
                state.operations_state.recon_selected = 0;
            } else {
                state.operations_state.selected = 0;
            }
        }
        KeyCode::Char('G') => {
            if state.operations_state.recon_focus {
                let len = state.operations_state.recon_count;
                if len > 0 {
                    state.operations_state.recon_selected = len - 1;
                }
            } else {
                let len = state.operations_state.timeline.len();
                if len > 0 {
                    state.operations_state.selected = len - 1;
                }
            }
        }
        KeyCode::Char('c') => {
            if let Some(TimelineEntry::Operation { op_index }) = state
                .operations_state
                .timeline
                .get(state.operations_state.selected)
            {
                cmd_tx
                    .try_send(AppCommand::LotStore(LotCommand::ConfirmOperation(
                        *op_index,
                    )))
                    .ok();
            }
        }
        KeyCode::Char('t') => {
            if let Some(TimelineEntry::Transaction { txn_index }) = state
                .operations_state
                .timeline
                .get(state.operations_state.selected)
                && state
                    .lot_store
                    .applied_transactions
                    .get(*txn_index)
                    .is_some_and(|t| t.action == TradeAction::Sell)
            {
                cmd_tx
                    .try_send(AppCommand::LotStore(LotCommand::CycleLotMethod(*txn_index)))
                    .ok();
            }
        }
        KeyCode::Char('T') => {
            cmd_tx
                .try_send(AppCommand::LotStore(LotCommand::CycleDefaultLotMethod))
                .ok();
        }
        KeyCode::Char('e') => {
            match state
                .operations_state
                .timeline
                .get(state.operations_state.selected)
            {
                Some(TimelineEntry::Operation { op_index }) => {
                    let op = &state.lot_store.operations[*op_index];
                    match &op.kind {
                        OperationKind::Multiplier { symbol, .. } => {
                            state.operations_state.edit_buffer = symbol.clone();
                            state.operations_state.editing = Some(OpEditField::Symbol1);
                        }
                        OperationKind::NameChange { old_symbol, .. } => {
                            state.operations_state.edit_buffer = old_symbol.clone();
                            state.operations_state.editing = Some(OpEditField::Symbol1);
                        }
                    }
                }
                Some(TimelineEntry::Transaction { txn_index }) => {
                    let txn = &state.lot_store.applied_transactions[*txn_index];
                    state.operations_state.edit_buffer = txn.symbol.clone();
                    state.operations_state.editing_txn = Some(TxnEditField::Symbol);
                    state.operations_state.pending_txn_edits.clear();
                }
                None => {}
            }
        }
        KeyCode::Char('d') | KeyCode::Char('y')
            if code == KeyCode::Char('d') || state.operations_state.confirm_delete =>
        {
            if !state.operations_state.confirm_delete {
                state.operations_state.confirm_delete = true;
            } else {
                state.operations_state.confirm_delete = false;
                match state
                    .operations_state
                    .timeline
                    .get(state.operations_state.selected)
                {
                    Some(TimelineEntry::Operation { op_index }) => {
                        cmd_tx
                            .try_send(AppCommand::LotStore(LotCommand::DeleteOperation(*op_index)))
                            .ok();
                    }
                    Some(TimelineEntry::Transaction { txn_index }) => {
                        cmd_tx
                            .try_send(AppCommand::LotStore(LotCommand::DeleteTransaction(
                                *txn_index,
                            )))
                            .ok();
                    }
                    None => {}
                }
            }
        }
        KeyCode::Char('a') => {
            state.operations_state.adding = true;
            state.operations_state.add_step = 0;
            state.operations_state.edit_buffer.clear();
            state.operations_state.add_buffer = Default::default();
        }
        KeyCode::Char('A') => {
            state.operations_state.adding_txn = true;
            state.operations_state.txn_add_step = 0;
            state.operations_state.edit_buffer.clear();
            state.operations_state.txn_add_buffer = Default::default();
        }
        KeyCode::Char('n') => {
            let start = state.operations_state.selected + 1;
            let timeline = &state.operations_state.timeline;
            let found = timeline[start..]
                .iter()
                .position(|e| {
                    matches!(e, TimelineEntry::Operation { op_index }
                        if !state.lot_store.operations[*op_index].confirmed)
                })
                .map(|p| start + p)
                .or_else(|| {
                    timeline.iter().position(|e| {
                        matches!(e, TimelineEntry::Operation { op_index }
                            if !state.lot_store.operations[*op_index].confirmed)
                    })
                });
            if let Some(pos) = found {
                state.operations_state.selected = pos;
            }
        }
        KeyCode::Char('N') => {
            let sel = state.operations_state.selected;
            let timeline = &state.operations_state.timeline;
            let found = if sel > 0 {
                timeline[..sel]
                    .iter()
                    .rposition(|e| {
                        matches!(e, TimelineEntry::Operation { op_index }
                            if !state.lot_store.operations[*op_index].confirmed)
                    })
                    .or_else(|| {
                        timeline.iter().rposition(|e| {
                            matches!(e, TimelineEntry::Operation { op_index }
                                if !state.lot_store.operations[*op_index].confirmed)
                        })
                    })
            } else {
                timeline.iter().rposition(|e| {
                    matches!(e, TimelineEntry::Operation { op_index }
                        if !state.lot_store.operations[*op_index].confirmed)
                })
            };
            if let Some(pos) = found {
                state.operations_state.selected = pos;
            }
        }
        KeyCode::Char('r') => {
            cmd_tx
                .try_send(AppCommand::LotStore(LotCommand::Rebuild))
                .ok();
            state.status = "Lots rebuilt".to_string();
        }
        _ => {}
    }
}

/// Build an EditTransaction command from pending edits + current store values.
fn finalize_txn_edit(state: &mut AppState, cmd_tx: &mpsc::Sender<AppCommand>) {
    use crate::app::LotCommand;
    use crate::app::operations::TxnEditField;

    let pending = &state.operations_state.pending_txn_edits;
    if let Some(crate::app::operations::TimelineEntry::Transaction { txn_index }) = state
        .operations_state
        .timeline
        .get(state.operations_state.selected)
    {
        let txn_index = *txn_index;
        if let Some(txn) = state.lot_store.applied_transactions.get(txn_index) {
            let symbol = pending
                .get(&TxnEditField::Symbol)
                .map(|v| v.trim().to_uppercase())
                .unwrap_or_else(|| txn.symbol.clone());
            let quantity = pending
                .get(&TxnEditField::Quantity)
                .and_then(|v| v.trim().parse::<f64>().ok())
                .unwrap_or(txn.quantity);
            let price = pending
                .get(&TxnEditField::Price)
                .and_then(|v| v.trim().parse::<f64>().ok())
                .unwrap_or(txn.price);
            // Prefix-parse the edited method (f/l/h or any longer prefix). An
            // unparseable entry keeps the transaction's existing method rather
            // than silently coercing to a default.
            let lot_method = pending
                .get(&TxnEditField::LotMethod)
                .and_then(|v| v.parse::<crate::schwab::LotSelectionMethod>().ok())
                .or(txn.lot_method);

            cmd_tx
                .try_send(AppCommand::LotStore(LotCommand::EditTransaction {
                    txn_index,
                    symbol,
                    quantity,
                    price,
                    lot_method,
                }))
                .ok();
        }
    }
}

fn finalize_add_operation(state: &mut AppState, cmd_tx: &mpsc::Sender<AppCommand>) {
    use crate::app::LotCommand;
    use crate::app::lots::{Operation, OperationKind};

    let buf = &state.operations_state.add_buffer;

    let date = match chrono::NaiveDate::parse_from_str(&buf.date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => {
            state.status = "Invalid date format".to_string();
            state.operations_state.adding = false;
            state.operations_state.add_step = 0;
            state.operations_state.edit_buffer.clear();
            return;
        }
    };

    let kind = if buf.kind == "n" {
        OperationKind::NameChange {
            old_symbol: buf.old_symbol.trim().to_uppercase(),
            new_symbol: buf.new_symbol.trim().to_uppercase(),
        }
    } else {
        let old_qty: f64 = match buf.old_qty.parse() {
            Ok(q) if q > 0.0 => q,
            _ => {
                state.status = "Old qty must be a positive number".to_string();
                state.operations_state.adding = false;
                state.operations_state.add_step = 0;
                state.operations_state.edit_buffer.clear();
                return;
            }
        };
        let new_qty: f64 = match buf.new_qty.parse() {
            Ok(q) if q > 0.0 => q,
            _ => {
                state.status = "New qty must be a positive number".to_string();
                state.operations_state.adding = false;
                state.operations_state.add_step = 0;
                state.operations_state.edit_buffer.clear();
                return;
            }
        };
        OperationKind::Multiplier {
            symbol: buf.symbol.trim().to_uppercase(),
            old_qty,
            new_qty,
        }
    };

    let op = Operation {
        date,
        kind,
        description: "Manual".to_string(),
        source_activity_ids: vec![],
        confirmed: true,
    };

    cmd_tx
        .try_send(AppCommand::LotStore(LotCommand::AddOperation(op)))
        .ok();

    state.operations_state.adding = false;
    state.operations_state.add_step = 0;
    state.operations_state.edit_buffer.clear();
    state.operations_state.add_buffer = Default::default();
    state.status = "Operation added".to_string();
}

fn finalize_add_transaction(state: &mut AppState, cmd_tx: &mpsc::Sender<AppCommand>) {
    use crate::app::LotCommand;
    use crate::app::lots::TradeAction;

    let buf = &state.operations_state.txn_add_buffer;

    let date = match chrono::NaiveDate::parse_from_str(&buf.date, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => {
            state.status = "Invalid date (use YYYY-MM-DD)".to_string();
            state.operations_state.adding_txn = false;
            state.operations_state.txn_add_step = 0;
            state.operations_state.edit_buffer.clear();
            return;
        }
    };

    let action: TradeAction = match buf.action.parse() {
        Ok(a) => a,
        Err(_) => {
            state.status = "Action must be buy or sell".to_string();
            state.operations_state.adding_txn = false;
            state.operations_state.txn_add_step = 0;
            state.operations_state.edit_buffer.clear();
            return;
        }
    };

    let quantity: f64 = buf.quantity.parse().unwrap_or(0.0);
    let price: f64 = buf.price.parse().unwrap_or(0.0);

    if quantity <= 0.0 || price <= 0.0 {
        state.status = "Quantity and price must be > 0".to_string();
        state.operations_state.adding_txn = false;
        state.operations_state.txn_add_step = 0;
        state.operations_state.edit_buffer.clear();
        return;
    }

    cmd_tx
        .try_send(AppCommand::LotStore(LotCommand::AddTransaction {
            date,
            symbol: buf.symbol.trim().to_uppercase(),
            action,
            quantity,
            price,
        }))
        .ok();

    state.operations_state.adding_txn = false;
    state.operations_state.txn_add_step = 0;
    state.operations_state.edit_buffer.clear();
    state.operations_state.txn_add_buffer = Default::default();
    state.status = format!("{action} transaction added");
}

fn order_summary(order: &crate::schwab::Order) -> String {
    let side = match order.instruction {
        Instruction::Buy
        | Instruction::BuyToCover
        | Instruction::BuyToOpen
        | Instruction::BuyToClose => "BUY",
        _ => "SELL",
    };
    let price_str = order
        .price
        .map(|p| format!(" @ ${p:.2}"))
        .unwrap_or_default();
    format!("{side} {:.0} {}{}", order.quantity, order.symbol, price_str)
}
