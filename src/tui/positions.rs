use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};

use super::account_pnl;
use super::pnl_color;
use crate::app::{AppState, Page, PosMetrics};
use crate::schwab::Position;
use crate::tui::command::ListNav;

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    // Aggregate account P&L panel, only computed when toggled on.
    let account_pnl = state
        .positions
        .show_pnl
        .then(|| account_pnl::compute(state));
    let pnl_height = account_pnl
        .as_ref()
        .map(account_pnl::panel_height)
        .unwrap_or(0);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),          // title bar
            Constraint::Length(1),          // account summary
            Constraint::Length(pnl_height), // account P&L panel (0 when hidden)
            Constraint::Min(0),             // table
            Constraint::Length(1),          // status bar
        ])
        .split(area);

    let computed = state.sorted_filtered_positions();
    render_title(f, chunks[0], state);
    render_summary(f, chunks[1], state, &computed);
    if let Some(pnl) = &account_pnl {
        account_pnl::render(f, chunks[2], pnl);
    }
    render_table(f, chunks[3], state, &computed);
    render_status(f, chunks[4], state);
}

fn render_title(f: &mut Frame, area: Rect, state: &AppState) {
    let refresh_text = state.refresh_text();

    let active = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    let inactive = Style::default().fg(Color::DarkGray);

    let (pos_style, ord_style) = match state.current_page {
        Page::Positions | Page::LotDetail(_) | Page::Operations => (active, inactive),
        Page::Orders => (inactive, active),
    };

    let mut spans = vec![
        Span::styled("[1] ", inactive),
        Span::styled("POSITIONS", pos_style),
        Span::styled("  [2] ", inactive),
        Span::styled("orders", ord_style),
    ];
    spans.push(Span::styled("  [3] ", inactive));
    spans.push(Span::styled("operations", inactive));
    spans.push(Span::styled(refresh_text, inactive));

    let title = Paragraph::new(Line::from(spans));
    f.render_widget(title, area);
}

fn render_summary(
    f: &mut Frame,
    area: Rect,
    state: &AppState,
    computed: &[(&Position, PosMetrics)],
) {
    let (day_pnl, total_market_value) = computed.iter().fold((0.0, 0.0), |(pnl, mv), (_, m)| {
        (pnl + m.day_pnl, mv + m.market_value)
    });

    let label = Style::default().fg(Color::DarkGray);
    let value = Style::default().fg(Color::White);

    let mut spans = vec![
        Span::styled(" Day P&L: ", label),
        Span::styled(
            format!("{:+.2}", day_pnl),
            Style::default().fg(pnl_color(day_pnl)),
        ),
        Span::styled("   Mkt Val: ", label),
        Span::styled(format!("${:.2}", total_market_value), value),
    ];

    if let Some(info) = &state.account_info {
        spans.push(Span::styled("   Cash: ", label));
        spans.push(Span::styled(format!("${:.2}", info.cash_balance), value));
        spans.push(Span::styled("   Buying Power: ", label));
        spans.push(Span::styled(format!("${:.2}", info.buying_power), value));
    }

    if let Some(acct) = state.current_account_summary()
        && state.accounts.len() > 1
    {
        spans.push(Span::styled("   Acct: ", label));
        spans.push(Span::styled(acct.display_label(), value));
    }

    f.render_widget(Paragraph::new(Line::from(spans)), area);
}

fn render_table(f: &mut Frame, area: Rect, state: &AppState, computed: &[(&Position, PosMetrics)]) {
    let header = Row::new([
        "Symbol", "Qty", "Avg Cost", "Last", "Day P&L", "Day %", "Open P&L", "Mkt Val",
    ])
    .style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )
    .height(1);

    let total_positions = computed.len();

    let mut rows: Vec<Row> = Vec::new();
    let mut cumulative_pnl = 0.0;
    let mut cumulative_mkt_val = 0.0;
    // Track which row index corresponds to the selected position.
    // When accumulation is on, each position occupies 2 rows.
    let mut selected_row = 0;

    for (i, (pos, m)) in computed.iter().enumerate() {
        cumulative_pnl += m.day_pnl;
        cumulative_mkt_val += m.market_value;

        rows.push(Row::new(vec![
            Cell::from(pos.symbol.clone()),
            Cell::from(format!("{:.2}", pos.quantity)),
            Cell::from(format!("{:.2}", pos.average_cost)),
            Cell::from(format!("{:.2}", m.last_price)),
            Cell::from(format!("{:+.2}", m.day_pnl))
                .style(Style::default().fg(pnl_color(m.day_pnl))),
            Cell::from(format!("{:+.2}%", m.day_pnl_pct))
                .style(Style::default().fg(pnl_color(m.day_pnl_pct))),
            Cell::from(format!("{:+.2}", m.open_pnl))
                .style(Style::default().fg(pnl_color(m.open_pnl))),
            Cell::from(format!("{:.2}", m.market_value)),
        ]));

        if state.positions.show_accumulation {
            rows.push(Row::new(vec![
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(""),
                Cell::from(format!("\u{03a3} {:+.2}", cumulative_pnl))
                    .style(Style::default().fg(pnl_color(cumulative_pnl))),
                Cell::from(""),
                Cell::from(""),
                Cell::from(format!("\u{03a3} {:.2}", cumulative_mkt_val)),
            ]));
        }

        if i == state.positions.selected_idx() {
            // Point to the last row of this group so the table scrolls
            // enough to keep the accumulation row visible.
            selected_row = rows.len() - 1;
        }
    }

    let widths = [
        Constraint::Length(10), // Symbol
        Constraint::Length(8),  // Qty
        Constraint::Length(10), // Avg Cost
        Constraint::Length(10), // Last
        Constraint::Length(12), // Day P&L
        Constraint::Length(9),  // Day %
        Constraint::Length(12), // Open P&L
        Constraint::Length(12), // Mkt Val
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(" Positions "))
        .column_spacing(1)
        .row_highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    let mut table_state = TableState::default();
    if total_positions > 0 {
        table_state.select(Some(selected_row));
    }

    f.render_stateful_widget(table, area, &mut table_state);
}

/// Return the symbol of the currently selected (and possibly filtered) position.
pub fn selected_symbol(state: &AppState) -> Option<String> {
    state.selected_position_symbol()
}

fn render_status(f: &mut Frame, area: Rect, state: &AppState) {
    let accum = if state.positions.show_accumulation {
        "a accum:ON"
    } else {
        "a accum"
    };

    let lock_hint = if state.positions.sort_locked {
        "  L lock:ON"
    } else {
        "  L lock"
    };

    let pnl_hint = if state.positions.show_pnl {
        "  P pnl:ON"
    } else {
        "  P pnl"
    };

    let copy_hint = if state.status.starts_with("Error:") {
        "  y copy"
    } else {
        ""
    };

    let (text, color) = if state.positions.filter_active {
        (
            format!(" /{}\u{2588}  [Esc] clear", state.positions.filter),
            Color::Yellow,
        )
    } else if !state.positions.filter.is_empty() {
        (
            format!(
                " {}  |  filter: {}  |  {}{lock_hint}{pnl_hint}  Esc clear  b buy  s sell  f flat  Enter lots  : cmd  2 orders  3 ops  q quit{copy_hint}",
                state.status, state.positions.filter, accum
            ),
            Color::DarkGray,
        )
    } else {
        (
            format!(
                " {}  |  {}{lock_hint}{pnl_hint}  / search  ↑↓ nav  b buy  s sell  f flat  Enter lots  : cmd  2 orders  3 ops  q quit{copy_hint}",
                state.status, accum
            ),
            Color::DarkGray,
        )
    };
    f.render_widget(Paragraph::new(text).style(Style::default().fg(color)), area);
}
