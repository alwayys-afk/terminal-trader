use std::collections::{BTreeSet, HashMap};

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};

use crate::app::lots::TradeAction::Sell;
use crate::app::operations::TimelineEntry;
use crate::app::{AppState, Page};

pub fn render(f: &mut Frame, area: Rect, state: &mut AppState) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // title bar
            Constraint::Min(0),    // main content
            Constraint::Length(1), // status bar
        ])
        .split(area);

    render_title(f, rows[0], state);

    // Side-by-side: timeline on the left, reconciliation on the right.
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(65), // timeline
            Constraint::Percentage(35), // reconciliation
        ])
        .split(rows[1]);

    render_timeline_table(f, cols[0], state);
    render_reconciliation(f, cols[1], state);
    render_status(f, rows[2], state);
}

fn render_title(f: &mut Frame, area: Rect, state: &AppState) {
    let active = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    let inactive = Style::default().fg(Color::DarkGray);

    let (pos_style, ord_style, ops_style) = match state.current_page {
        Page::Positions | Page::LotDetail(_) => (active, inactive, inactive),
        Page::Orders => (inactive, active, inactive),
        Page::Operations => (inactive, inactive, active),
    };

    let title = Paragraph::new(Line::from(vec![
        Span::styled("[1] ", inactive),
        Span::styled("positions", pos_style),
        Span::styled("  [2] ", inactive),
        Span::styled("orders", ord_style),
        Span::styled("  [3] ", inactive),
        Span::styled("OPERATIONS", ops_style),
        Span::styled("    default method: ", inactive),
        Span::styled(
            state.lot_store.default_lot_method.to_string(),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    ]));
    f.render_widget(title, area);
}

fn render_timeline_table(f: &mut Frame, area: Rect, state: &AppState) {
    let ops_state = &state.operations_state;

    let header = Row::new(["Date", "Type", "Symbol", "Qty", "Price", "Method", "Status"])
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .height(1);

    let rows: Vec<Row> = ops_state
        .timeline
        .iter()
        .map(|entry| match entry {
            TimelineEntry::Transaction { txn_index } => {
                let txn = &state.lot_store.applied_transactions[*txn_index];
                // A known method (recovered from the order or set manually) shows
                // normally; an unknown sell shows the account default it will
                // follow, dimmed to mark it as inferred rather than confirmed.
                let method_cell = match (txn.action, txn.lot_method) {
                    (Sell, Some(method)) => Cell::from(method.to_string()),
                    (Sell, None) => Cell::from(Span::styled(
                        state.lot_store.default_lot_method.to_string(),
                        Style::default().fg(Color::DarkGray),
                    )),
                    _ => Cell::from(""),
                };

                Row::new(vec![
                    Cell::from(txn.date.format("%Y-%m-%d").to_string()),
                    Cell::from(txn.action.to_string()),
                    Cell::from(txn.symbol.clone()),
                    Cell::from(format!("{:.0}", txn.quantity)),
                    Cell::from(format!("${:.2}", txn.price)),
                    method_cell,
                    Cell::from(""),
                ])
                .style(Style::default().fg(Color::White))
            }
            TimelineEntry::Operation { op_index } => {
                let op = &state.lot_store.operations[*op_index];
                let symbol_text = crate::app::operations::op_symbol_text(&op.kind);
                let qty_text = crate::app::operations::op_qty_text(&op.kind);
                let (status_text, color) = if op.confirmed {
                    ("Confirmed", Color::Green)
                } else {
                    ("Pending", Color::Yellow)
                };

                Row::new(vec![
                    Cell::from(op.date.format("%Y-%m-%d").to_string()),
                    Cell::from(op.kind.to_string()),
                    Cell::from(symbol_text),
                    Cell::from(qty_text),
                    Cell::from("--"),
                    Cell::from(""),
                    Cell::from(status_text),
                ])
                .style(Style::default().fg(color))
            }
        })
        .collect();

    let widths = [
        Constraint::Length(10), // Date
        Constraint::Length(14), // Type
        Constraint::Length(16), // Symbol
        Constraint::Length(14), // Qty
        Constraint::Length(10), // Price
        Constraint::Length(6),  // Method
        Constraint::Min(10),    // Status
    ];

    let unconfirmed = state
        .lot_store
        .operations
        .iter()
        .filter(|o| !o.confirmed)
        .count();
    let title = if unconfirmed > 0 {
        format!(" Timeline ({unconfirmed} pending) ")
    } else {
        " Timeline ".to_string()
    };

    let timeline_focused = !state.operations_state.recon_focus;
    let border_style = if timeline_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style),
        )
        .column_spacing(1)
        .row_highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    let mut table_state = TableState::default();
    if !ops_state.timeline.is_empty() {
        table_state.select(Some(ops_state.selected));
    }

    f.render_stateful_widget(table, area, &mut table_state);
}

fn render_reconciliation(f: &mut Frame, area: Rect, state: &mut AppState) {
    // Compare lot-derived quantities vs positions API.
    let mut symbols = BTreeSet::new();
    for sym in state.lot_store.lots.keys() {
        symbols.insert(sym.clone());
    }
    for pos in state.positions.items() {
        symbols.insert(pos.symbol.clone());
    }

    let broker_map: HashMap<&str, (f64, f64)> = state
        .positions
        .items()
        .iter()
        .map(|p| (p.symbol.as_str(), (p.quantity, p.average_cost)))
        .collect();

    let header = Row::new([
        "Symbol", "Lot Qty", "Brk Qty", "Lot Avg$", "Brk Avg$", "Status",
    ])
    .style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )
    .height(1);

    let mut match_count = 0usize;
    let mut mismatch_count = 0usize;

    let rows: Vec<Row> = symbols
        .iter()
        .map(|sym| {
            let lots = state.lot_store.lots_for(sym);
            let lot_qty: f64 = lots.iter().map(|l| l.remaining).sum();
            let lot_cost: f64 = lots
                .iter()
                .filter(|l| l.remaining.abs() > 1e-9)
                .map(|l| l.remaining * l.cost_per_share)
                .sum();
            let lot_avg = if lot_qty.abs() > 1e-9 {
                lot_cost / lot_qty
            } else {
                0.0
            };

            let (brk_qty, brk_avg) = broker_map.get(sym.as_str()).copied().unwrap_or((0.0, 0.0));

            let qty_match = (lot_qty - brk_qty).abs() < 0.01;
            let (status_text, status_color) = if qty_match {
                match_count += 1;
                ("MATCH", Color::Green)
            } else {
                mismatch_count += 1;
                ("MISMATCH", Color::Red)
            };

            let fmt_qty = |q: f64| {
                if q.abs() < 0.001 {
                    "--".to_string()
                } else {
                    format!("{:.0}", q)
                }
            };
            let fmt_avg = |a: f64| {
                if a.abs() < 0.001 {
                    "--".to_string()
                } else {
                    format!("${:.2}", a)
                }
            };

            Row::new(vec![
                Cell::from(sym.clone()),
                Cell::from(fmt_qty(lot_qty)),
                Cell::from(fmt_qty(brk_qty)),
                Cell::from(fmt_avg(lot_avg)),
                Cell::from(fmt_avg(brk_avg)),
                Cell::from(status_text).style(Style::default().fg(status_color)),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(10), // Symbol
        Constraint::Length(8),  // Lot Qty
        Constraint::Length(8),  // Brk Qty
        Constraint::Length(10), // Lot Avg$
        Constraint::Length(10), // Brk Avg$
        Constraint::Min(8),     // Status
    ];

    let focused = state.operations_state.recon_focus;
    let title = format!(" Reconciliation ({match_count} match, {mismatch_count} mismatch) ");
    let border_style = if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    state.operations_state.recon_count = rows.len();
    if state.operations_state.recon_selected >= rows.len() && !rows.is_empty() {
        state.operations_state.recon_selected = rows.len() - 1;
    }

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style),
        )
        .column_spacing(1)
        .row_highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    let mut table_state = TableState::default();
    table_state.select(Some(state.operations_state.recon_selected));
    f.render_stateful_widget(table, area, &mut table_state);
}

fn render_status(f: &mut Frame, area: Rect, state: &AppState) {
    let ops_state = &state.operations_state;

    let text = if ops_state.adding_txn {
        let field_name = match ops_state.txn_add_step {
            0 => "Date (YYYY-MM-DD)",
            1 => "Action (buy/sell)",
            2 => "Symbol",
            3 => "Quantity",
            4 => "Price",
            _ => "",
        };
        format!(
            " Add transaction - {}: {}|  [Enter] next  [Esc] cancel",
            field_name, ops_state.edit_buffer
        )
    } else if ops_state.adding {
        let field_name = match (ops_state.add_step, ops_state.add_buffer.kind.as_str()) {
            (0, _) => "Date (YYYY-MM-DD)",
            (1, _) => "Kind (m=multiplier, n=namechange)",
            (2, "n") => "Old Symbol",
            (3, "n") => "New Symbol",
            (2, _) => "Symbol",
            (3, _) => "Old Qty",
            (4, _) => "New Qty",
            _ => "",
        };
        format!(
            " Add operation - {}: {}|  [Enter] next  [Esc] cancel",
            field_name, ops_state.edit_buffer
        )
    } else if ops_state.editing_txn.is_some() {
        let field = match ops_state.editing_txn {
            Some(crate::app::operations::TxnEditField::Symbol) => "Symbol",
            Some(crate::app::operations::TxnEditField::Quantity) => "Qty",
            Some(crate::app::operations::TxnEditField::Price) => "Price",
            Some(crate::app::operations::TxnEditField::LotMethod) => "Method (LIFO/FIFO/HIFO)",
            None => "",
        };
        format!(
            " Edit {}: {}|  [Enter] save  [Tab] next field  [Esc] cancel",
            field, ops_state.edit_buffer
        )
    } else if ops_state.editing.is_some() {
        format!(
            " Edit: {}|  [Enter] confirm  [Esc] cancel",
            ops_state.edit_buffer
        )
    } else if ops_state.confirm_delete {
        " Press d/y to confirm delete, any other key to cancel".to_string()
    } else {
        let unconfirmed = state
            .lot_store
            .operations
            .iter()
            .filter(|o| !o.confirmed)
            .count();
        let pending = if unconfirmed > 0 {
            format!("  {unconfirmed} pending")
        } else {
            String::new()
        };
        format!(
            " {}  |  ↑↓ nav  n/N pending  Tab panel  e edit  d del  t method  T default-method  a add-op  A add-txn  c confirm  r rebuild{}",
            state.status, pending
        )
    };

    f.render_widget(
        Paragraph::new(text).style(Style::default().fg(Color::DarkGray)),
        area,
    );
}
