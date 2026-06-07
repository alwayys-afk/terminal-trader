use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier as StyleModifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::AppState;
use crate::schwab::Quote;
use crate::tui::command::{
    Amount, CommandBoxDisplay, CommandMode, ListNav, Modifier, ParseResult, ParsedCommand,
    TokenFocus, Verb, command_box_display, token_focus,
};

pub fn render_command_box(f: &mut Frame, state: &AppState) {
    match &state.command_mode {
        CommandMode::Hidden | CommandMode::OrderSent(_) => {}

        CommandMode::Typing(ts) => {
            let title = if ts.replace_order_id.is_some() {
                " Edit Order "
            } else {
                " Order "
            };
            let area = centered_rect(60, 20, f.area());
            f.render_widget(Clear, area);

            let block = Block::default().borders(Borders::ALL).title(title);
            let inner = block.inner(area);
            f.render_widget(block, area);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1), // [0] input line
                    Constraint::Length(1), // [1] blank
                    Constraint::Length(1), // [2] suggestion 0 / quote line
                    Constraint::Length(1), // [3] suggestion 1 / order preview
                    Constraint::Length(1), // [4] suggestion 2 / dollar estimate
                    Constraint::Length(1), // [5] suggestion 3
                    Constraint::Length(1), // [6] suggestion 4
                    Constraint::Length(1), // [6] suggestion 5
                    Constraint::Length(1), // [6] suggestion 6
                    Constraint::Length(1), // [6] suggestion 7
                    Constraint::Length(1), // [6] suggestion 8
                    Constraint::Length(1), // [6] suggestion 9
                    Constraint::Length(1), // [6] suggestion 10
                    Constraint::Length(1), // [7] hint
                    Constraint::Min(0),
                ])
                .split(inner);

            // Input line (with inline error/ambiguous text)
            let prefix_len = 2u16; // "> "
            let cursor_display = ts.input[..ts.cursor].chars().count() as u16;

            let input_line = match &ts.parsed {
                ParseResult::Ambiguous(cmds) => Line::from(vec![
                    Span::styled("> ", Style::default().fg(Color::Yellow)),
                    Span::raw(ts.input.clone()),
                    Span::styled(
                        format!("  [Ambiguous: {}]", cmds.join(", ")),
                        Style::default().fg(Color::Red),
                    ),
                ]),
                ParseResult::Invalid(msg) => Line::from(vec![
                    Span::styled("> ", Style::default().fg(Color::Yellow)),
                    Span::raw(ts.input.clone()),
                    Span::styled(format!("  [{msg}]"), Style::default().fg(Color::Red)),
                ]),
                _ => Line::from(vec![
                    Span::styled("> ", Style::default().fg(Color::Yellow)),
                    Span::raw(ts.input.clone()),
                ]),
            };
            f.render_widget(Paragraph::new(input_line), chunks[0]);
            f.set_cursor_position((chunks[0].x + prefix_len + cursor_display, chunks[0].y));

            // Contextual hint for the current token.
            let hint_text = match token_focus(&ts.input, ts.cursor) {
                TokenFocus::Verb => "buy · sell",
                TokenFocus::Symbol if !ts.autocomplete.has_suggestions() => "ticker symbol",
                TokenFocus::Amount => "10 shares · $500 dollars · 50% (sell only)",
                TokenFocus::Modifier => "mid(default) · market · now · bid · ask · $price",
                _ => "",
            };
            if !hint_text.is_empty() {
                f.render_widget(
                    Paragraph::new(Line::from(Span::styled(
                        format!("  {hint_text}"),
                        Style::default().fg(Color::DarkGray),
                    ))),
                    chunks[1],
                );
            }

            match command_box_display(
                &ts.input,
                ts.cursor,
                &ts.parsed,
                ts.autocomplete.has_suggestions(),
            ) {
                CommandBoxDisplay::QuotePreview => {
                    if let ParseResult::Valid(ref cmd) = ts.parsed {
                        if let Some(q) = state.stream.quotes.get(&cmd.symbol) {
                            let held_qty = state.positions.held_qty(&cmd.symbol);
                            render_typing_preview(
                                f,
                                &chunks,
                                cmd,
                                q,
                                held_qty,
                                &state.account_info,
                            );
                        } else {
                            let fetching = Line::from(Span::styled(
                                format!("  Fetching quote for {}...", cmd.symbol),
                                Style::default().fg(Color::DarkGray),
                            ));
                            f.render_widget(Paragraph::new(fetching), chunks[2]);
                        }
                    }
                }
                CommandBoxDisplay::Suggestions => {
                    render_suggestions(
                        f,
                        &chunks,
                        ts.autocomplete.suggestions(),
                        ts.autocomplete.selected_idx(),
                    );
                }
                CommandBoxDisplay::Nothing => {}
            }
        }

        CommandMode::Confirming {
            cmd,
            price,
            replace_order_id,
            ..
        } => {
            let price = *price;
            let title = if replace_order_id.is_some() {
                " Confirm Edit "
            } else {
                " Confirm Order "
            };
            let area = centered_rect(60, 7, f.area());
            f.render_widget(Clear, area);

            let block = Block::default().borders(Borders::ALL).title(title);
            let inner = block.inner(area);
            f.render_widget(block, area);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1), // order line
                    Constraint::Length(1), // estimate
                    Constraint::Length(1), // blank
                    Constraint::Length(1), // confirm hint
                    Constraint::Min(0),
                ])
                .split(inner);

            let verb_str = match cmd.verb {
                Verb::Buy => "BUY",
                Verb::Sell => "SELL",
            };
            let verb_color = match cmd.verb {
                Verb::Buy => Color::Green,
                Verb::Sell => Color::Red,
            };

            let held_qty = state.positions.held_qty(&cmd.symbol);
            let shares = compute_confirmed_shares(cmd, price, held_qty);
            let order_type_str = match &cmd.modifier {
                Modifier::Market => "MARKET".to_string(),
                _ => format!("LIMIT @ ${price:.2}"),
            };

            let order_line = Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    verb_str,
                    Style::default()
                        .fg(verb_color)
                        .add_modifier(StyleModifier::BOLD),
                ),
                Span::raw(format!(" {shares:.0} {}  {order_type_str}", cmd.symbol)),
            ]);
            f.render_widget(Paragraph::new(order_line), chunks[0]);

            let mut est_spans = vec![Span::styled(
                format!("  (~${:.0})", shares * price),
                Style::default().fg(Color::DarkGray),
            )];
            if let Some(info) = &state.account_info {
                est_spans.push(Span::styled(
                    format!(
                        "     Cash ${:.2}  BP ${:.2}",
                        info.cash_balance, info.buying_power
                    ),
                    Style::default().fg(Color::DarkGray),
                ));
            }
            f.render_widget(Paragraph::new(Line::from(est_spans)), chunks[1]);

            // For sells, show the auto-selected tax lot method(s). A sale that
            // spans long-term and short-term lots splits into two orders.
            if cmd.verb == Verb::Sell {
                let legs =
                    crate::app::compute_sell_legs(state.lot_store.lots_for(&cmd.symbol), shares);
                let mut spans = vec![Span::styled(
                    "  Tax lots: ",
                    Style::default().fg(Color::DarkGray),
                )];
                for (i, leg) in legs.iter().enumerate() {
                    if i > 0 {
                        spans.push(Span::styled("  +  ", Style::default().fg(Color::DarkGray)));
                    }
                    spans.push(Span::styled(
                        format!("{:.0} {}", leg.quantity, leg.method),
                        Style::default().fg(Color::Yellow),
                    ));
                }
                if legs.len() > 1 {
                    spans.push(Span::styled(
                        "  (auto-split)",
                        Style::default().fg(Color::DarkGray),
                    ));
                }
                f.render_widget(Paragraph::new(Line::from(spans)), chunks[2]);
            }

            let hint_line = Line::from(vec![
                Span::styled("  [Enter] Submit", Style::default().fg(Color::Green)),
                Span::styled("    [e] Edit", Style::default().fg(Color::DarkGray)),
                Span::styled("    [Esc] Cancel", Style::default().fg(Color::Red)),
            ]);
            f.render_widget(Paragraph::new(hint_line), chunks[3]);
        }

        CommandMode::ConfirmCancel { summary, .. } => {
            let area = centered_rect(60, 6, f.area());
            f.render_widget(Clear, area);

            let block = Block::default()
                .borders(Borders::ALL)
                .title(" Cancel Order ");
            let inner = block.inner(area);
            f.render_widget(block, area);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1), // order summary
                    Constraint::Length(1), // blank
                    Constraint::Length(1), // hint
                    Constraint::Min(0),
                ])
                .split(inner);

            let order_line = Line::from(vec![
                Span::styled("  Cancel: ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    summary.clone(),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(StyleModifier::BOLD),
                ),
            ]);
            f.render_widget(Paragraph::new(order_line), chunks[0]);

            let hint_line = Line::from(vec![
                Span::styled("  [Enter] Confirm Cancel", Style::default().fg(Color::Red)),
                Span::styled("   [Esc] Back", Style::default().fg(Color::DarkGray)),
            ]);
            f.render_widget(Paragraph::new(hint_line), chunks[2]);
        }
    }
}

// ── Private helpers ───────────────────────────────────────────────────────────

fn render_typing_preview(
    f: &mut Frame,
    chunks: &std::rc::Rc<[Rect]>,
    cmd: &ParsedCommand,
    q: &Quote,
    held_qty: Option<f64>,
    account_info: &Option<crate::schwab::AccountInfo>,
) {
    let mut quote_spans = vec![
        Span::styled(
            format!("  {}   ", q.symbol),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(StyleModifier::BOLD),
        ),
        Span::styled("ask ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("${:.2}   ", q.ask_price),
            Style::default().fg(Color::White),
        ),
        Span::styled("bid ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("${:.2}", q.bid_price),
            Style::default().fg(Color::White),
        ),
    ];
    if let Some(qty) = held_qty {
        quote_spans.push(Span::styled(
            format!("   held {:.2}", qty),
            Style::default().fg(Color::Yellow),
        ));
    }
    f.render_widget(Paragraph::new(Line::from(quote_spans)), chunks[2]);

    let (shares, limit_price) = compute_order_preview(cmd, q, held_qty);

    let verb_str = match cmd.verb {
        Verb::Buy => "BUY",
        Verb::Sell => "SELL",
    };
    let verb_color = match cmd.verb {
        Verb::Buy => Color::Green,
        Verb::Sell => Color::Red,
    };

    let order_type_str = match &cmd.modifier {
        Modifier::Market => "MARKET".to_string(),
        Modifier::Limit(p) => format!("LIMIT @ ${p:.2}"),
        Modifier::Mid => limit_price
            .map(|p| format!("LIMIT @ ${p:.2} (mid)"))
            .unwrap_or_else(|| "LIMIT".to_string()),
        Modifier::Ask => limit_price
            .map(|p| format!("LIMIT @ ${p:.2} (ask)"))
            .unwrap_or_else(|| "LIMIT @ ASK".to_string()),
        Modifier::Bid => limit_price
            .map(|p| format!("LIMIT @ ${p:.2} (bid)"))
            .unwrap_or_else(|| "LIMIT @ BID".to_string()),
    };

    let shares_str = shares
        .map(|s| format!("{s:.0}"))
        .unwrap_or_else(|| "?".to_string());

    let order_line = Line::from(vec![
        Span::raw("  "),
        Span::styled(
            verb_str,
            Style::default()
                .fg(verb_color)
                .add_modifier(StyleModifier::BOLD),
        ),
        Span::raw(format!(" {shares_str} shares  {order_type_str}")),
    ]);
    f.render_widget(Paragraph::new(order_line), chunks[3]);

    if let (Some(s), Some(lp)) = (shares, limit_price) {
        let mut est_spans = vec![Span::styled(
            format!("  (~${:.0})", s * lp),
            Style::default().fg(Color::DarkGray),
        )];
        if let Some(info) = account_info {
            est_spans.push(Span::styled(
                format!(
                    "     Cash ${:.2}  BP ${:.2}",
                    info.cash_balance, info.buying_power
                ),
                Style::default().fg(Color::DarkGray),
            ));
        }
        f.render_widget(Paragraph::new(Line::from(est_spans)), chunks[4]);
    }
}

/// Returns `(shares, limit_price)` for the Typing-state preview.
fn compute_order_preview(
    cmd: &ParsedCommand,
    quote: &Quote,
    held_qty: Option<f64>,
) -> (Option<f64>, Option<f64>) {
    let price = match &cmd.modifier {
        Modifier::Market => None,
        Modifier::Limit(p) => Some(*p),
        Modifier::Mid => Some((quote.bid_price + quote.ask_price) / 2.0),
        Modifier::Ask => Some(quote.ask_price),
        Modifier::Bid => Some(quote.bid_price),
    };

    let shares = match &cmd.amount {
        None => None,
        Some(Amount::Shares(s)) => Some(*s),
        Some(Amount::Dollars(d)) => price.map(|p| if p > 0.0 { (*d / p).floor() } else { 0.0 }),
        Some(Amount::Percent(pct)) => held_qty.map(|qty| (qty * pct / 100.0).floor()),
    };

    (shares, price)
}

/// Compute final shares for the Confirming state (price already locked).
pub(crate) fn compute_confirmed_shares(
    cmd: &ParsedCommand,
    price: f64,
    held_qty: Option<f64>,
) -> f64 {
    match &cmd.amount {
        None => 0.0,
        Some(Amount::Shares(s)) => *s,
        Some(Amount::Dollars(d)) => {
            if price > 0.0 {
                (*d / price).floor()
            } else {
                0.0
            }
        }
        Some(Amount::Percent(pct)) => held_qty
            .map(|qty| (qty * pct / 100.0).floor())
            .unwrap_or(0.0),
    }
}

fn render_suggestions(
    f: &mut Frame,
    chunks: &std::rc::Rc<[Rect]>,
    suggestions: &[(String, String)],
    sel: usize,
) {
    const VISIBLE: usize = 10;
    // Scroll window: keep sel in view, scrolling down as selection moves.
    let max_start = suggestions.len().saturating_sub(VISIBLE);
    let window_start = sel.saturating_sub(VISIBLE - 1).min(max_start);

    for (i, (sym, desc)) in suggestions[window_start..].iter().take(VISIBLE).enumerate() {
        let chunk_idx = i + 2; // chunks[2..6]
        if chunk_idx >= chunks.len() {
            break;
        }
        let abs_idx = window_start + i;
        let is_sel = abs_idx == sel;
        let sym_style = if is_sel {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(StyleModifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let prefix = if is_sel { "▶" } else { " " };
        let max_desc = chunks[chunk_idx].width.saturating_sub(14) as usize;
        let desc_str = if desc.len() > max_desc {
            // Truncate at a char boundary to avoid panicking on multi-byte UTF-8.
            let end = desc
                .char_indices()
                .take_while(|(i, _)| *i <= max_desc)
                .last()
                .map(|(i, c)| i + c.len_utf8())
                .unwrap_or(0);
            &desc[..end]
        } else {
            desc.as_str()
        };
        let line = Line::from(vec![
            Span::styled(format!("  {prefix} "), Style::default().fg(Color::Yellow)),
            Span::styled(format!("{:<6}", sym), sym_style),
            Span::styled(
                format!("  {}", desc_str),
                Style::default().fg(Color::DarkGray),
            ),
        ]);
        f.render_widget(Paragraph::new(line), chunks[chunk_idx]);
    }
    // Hint row after the last visible suggestion
    let hint_idx = 2 + VISIBLE;
    if chunks.len() > hint_idx {
        let position_str = if suggestions.len() > VISIBLE {
            format!("  {}/{}", sel + 1, suggestions.len())
        } else {
            String::new()
        };
        let hint = Line::from(Span::styled(
            format!("  [Tab] complete  [↑↓] navigate{position_str}"),
            Style::default().fg(Color::DarkGray),
        ));
        f.render_widget(Paragraph::new(hint), chunks[hint_idx]);
    }
}

/// Render the account picker overlay, when open. One row per account, with the
/// current account marked and the picker's selection highlighted.
pub fn render_account_picker(f: &mut Frame, state: &AppState) {
    let Some(picker) = state.account_picker.as_ref() else {
        return;
    };
    let count = state.accounts.len();
    // 2 rows of chrome (top/bottom border) + 1 hint row + 1 blank + N account rows.
    let height = (count as u16).saturating_add(4).min(f.area().height);
    let area = centered_rect(50, height, f.area());
    f.render_widget(Clear, area);

    let block = Block::default().borders(Borders::ALL).title(" Accounts ");
    let inner = block.inner(area);
    f.render_widget(block, area);

    let mut constraints: Vec<Constraint> = (0..count).map(|_| Constraint::Length(1)).collect();
    constraints.push(Constraint::Length(1)); // blank
    constraints.push(Constraint::Length(1)); // hint
    constraints.push(Constraint::Min(0));
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    for (i, acct) in state.accounts.iter().enumerate() {
        let active = acct.hash == state.current_account;
        let selected = i == picker.selected;
        let marker = if active { "*" } else { " " };
        let line_style = if selected {
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(StyleModifier::BOLD)
        } else if active {
            Style::default().fg(Color::Green)
        } else {
            Style::default()
        };
        let line = Line::from(vec![Span::styled(
            format!("  {marker} {}", acct.display_label()),
            line_style,
        )]);
        f.render_widget(Paragraph::new(line), chunks[i]);
    }

    let hint_idx = count + 1;
    if chunks.len() > hint_idx {
        let hint = Line::from(Span::styled(
            "  [↑↓] navigate  [Enter] switch  [Esc] cancel",
            Style::default().fg(Color::DarkGray),
        ));
        f.render_widget(Paragraph::new(hint), chunks[hint_idx]);
    }
}

fn centered_rect(percent_x: u16, height: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_x / 100;
    let x = r.x + r.width.saturating_sub(popup_width) / 2;
    let y = r.y + r.height.saturating_sub(height) / 2;
    Rect {
        x,
        y,
        width: popup_width.min(r.width),
        height: height.min(r.height),
    }
}
