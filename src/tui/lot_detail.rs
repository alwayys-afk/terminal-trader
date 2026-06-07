use std::collections::BTreeMap;

use chrono::{Datelike, NaiveDate};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, StatefulWidget, Table, TableState},
};

use super::pnl_color;
use crate::app::AppState;
use crate::schwab::{LotSale, QTY_EPSILON, TaxLot, is_long_term};

/// Latest known price for `symbol`: streamed quote first, else the last
/// position price. `None` when neither is available yet.
pub(super) fn price_opt(state: &AppState, symbol: &str) -> Option<f64> {
    state
        .stream
        .quotes
        .get(symbol)
        .map(|q| q.last_price)
        .or_else(|| {
            state
                .positions
                .items()
                .iter()
                .find(|p| p.symbol == symbol)
                .map(|p| p.current_price)
        })
}

#[derive(Default, Clone, Copy)]
struct YearlyPnl {
    realized: f64,
    unrealized: f64,
}

/// Compute per-year realized P&L from sales and per-year unrealized P&L from
/// open lots. Unrealized is grouped by the lot's open year; realized by the sale year.
fn yearly_pnl(
    lots: &[TaxLot],
    sales: &[LotSale],
    current_price: Option<f64>,
) -> BTreeMap<i32, YearlyPnl> {
    let mut by_year: BTreeMap<i32, YearlyPnl> = BTreeMap::new();

    for sale in sales {
        let year = sale.sale_date.year();
        by_year.entry(year).or_default().realized += sale.realized_pnl;
    }

    for lot in lots {
        if lot.remaining.abs() < QTY_EPSILON {
            continue;
        }
        let year = lot.open_date.year();
        // Unknown price: mark at the lot's own cost so it contributes 0
        // unrealized rather than a phantom loss against a $0 mark.
        let mark = current_price.unwrap_or(lot.cost_per_share);
        let unrealized = (mark - lot.cost_per_share) * lot.remaining;
        by_year.entry(year).or_default().unrealized += unrealized;
    }

    by_year
}

pub fn render(f: &mut Frame, area: Rect, state: &mut AppState, symbol: &str) {
    let lots = state.lot_store.lots_for(symbol);
    let sales = state.lot_store.sales_for(symbol);
    let current_price = price_opt(state, symbol);
    let yearly = yearly_pnl(lots, sales, current_price);

    // Summary: 1 position line + 1 header + N year rows + 1 divider + 1 unrealized + 1 total.
    let pnl_rows = yearly.len() + 3;
    let summary_height = 1 + 1 + pnl_rows as u16;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),              // title
            Constraint::Length(summary_height), // summary
            Constraint::Min(0),                 // table
            Constraint::Length(1),              // status bar
        ])
        .split(area);

    render_title(f, chunks[0], symbol);
    render_summary(f, chunks[1], state, symbol, &yearly, current_price);
    render_timeline_table(f, chunks[2], state, symbol);
    render_status(f, chunks[3]);
}

fn render_title(f: &mut Frame, area: Rect, symbol: &str) {
    let title = Paragraph::new(Line::from(vec![Span::styled(
        format!(" Tax Lots: {symbol}"),
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    )]));
    f.render_widget(title, area);
}

fn render_summary(
    f: &mut Frame,
    area: Rect,
    state: &AppState,
    symbol: &str,
    yearly: &BTreeMap<i32, YearlyPnl>,
    current_price: Option<f64>,
) {
    let lots = state.lot_store.lots_for(symbol);
    let total_remaining: f64 = lots.iter().map(|l| l.remaining).sum();
    let remaining_cost: f64 = lots
        .iter()
        .filter(|l| l.remaining > QTY_EPSILON)
        .map(|l| l.remaining * l.cost_per_share)
        .sum();
    let avg_cost = if total_remaining > 0.0 {
        remaining_cost / total_remaining
    } else {
        0.0
    };

    let label = Style::default().fg(Color::DarkGray);
    let value = Style::default().fg(Color::White);
    let header_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);

    let open_lots = lots.iter().filter(|l| l.remaining > QTY_EPSILON).count();

    let mut lines: Vec<Line> = Vec::new();

    // Line 1: position summary.
    lines.push(Line::from(vec![
        Span::styled(" Lots: ", label),
        Span::styled(format!("{open_lots}"), value),
        Span::styled("   Qty: ", label),
        Span::styled(format!("{total_remaining:.0}"), value),
        Span::styled("   Avg Cost: ", label),
        Span::styled(format!("${avg_cost:.2}"), value),
        Span::styled("   Last: ", label),
        Span::styled(
            current_price
                .map(|p| format!("${p:.2}"))
                .unwrap_or_else(|| "n/a".to_string()),
            value,
        ),
    ]));

    // P&L breakdown header.
    lines.push(Line::from(vec![
        Span::styled(" Period       ", header_style),
        Span::styled("Realized", header_style),
    ]));

    let current_year = chrono::Local::now().date_naive().year();
    let total_realized: f64 = yearly.values().map(|p| p.realized).sum();
    let total_unrealized: f64 = yearly.values().map(|p| p.unrealized).sum();

    // Per-year realized rows (chronological order from BTreeMap).
    for (&year, pnl) in yearly {
        let period = if year == current_year {
            format!(" {year} (YTD) ")
        } else {
            format!(" {year}       ")
        };
        lines.push(pnl_line(&period, pnl.realized, label));
    }

    lines.push(Line::from(Span::styled(" ------------------------", label)));

    lines.push(pnl_line(" Unrealized   ", total_unrealized, label));

    lines.push(pnl_line(
        " Total       ",
        total_realized + total_unrealized,
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    ));

    f.render_widget(Paragraph::new(lines), area);
}

/// Build a single P&L summary line with period label and one value.
fn pnl_line<'a>(period: &str, value: f64, label_style: Style) -> Line<'a> {
    Line::from(vec![
        Span::styled(period.to_string(), label_style),
        Span::styled(
            format!("{:>+12.2}", value),
            Style::default().fg(pnl_color(value)),
        ),
    ])
}

/// A single row in the interleaved timeline.
enum TimelineEntry<'a> {
    Lot { lot: &'a TaxLot },
    Sale { sale: &'a LotSale },
}

impl<'a> TimelineEntry<'a> {
    fn sort_key(&self) -> (NaiveDate, usize) {
        match self {
            TimelineEntry::Lot { lot } => (lot.open_date, lot.seq),
            TimelineEntry::Sale { sale } => (sale.sale_date, sale.seq),
        }
    }
}

fn render_timeline_table(f: &mut Frame, area: Rect, state: &mut AppState, symbol: &str) {
    let lots = state.lot_store.lots_for(symbol);
    let sales = state.lot_store.sales_for(symbol);

    if lots.is_empty() && sales.is_empty() {
        let msg = Paragraph::new(format!(" No tax lots tracked for {symbol}"))
            .style(Style::default().fg(Color::DarkGray))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Lots & Transactions "),
            );
        f.render_widget(msg, area);
        return;
    }

    let header = Row::new([
        "Date",
        "Action",
        "Qty",
        "Price",
        "Cost Basis",
        "Value",
        "Unrealized",
        "Realized",
        "Details",
    ])
    .style(
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )
    .height(1);

    // Build interleaved timeline entries.
    let mut entries: Vec<TimelineEntry> = Vec::new();
    for lot in lots {
        entries.push(TimelineEntry::Lot { lot });
    }
    for sale in sales {
        entries.push(TimelineEntry::Sale { sale });
    }
    entries.sort_by_key(|e| e.sort_key());
    entries.reverse();

    let current_price = price_opt(state, symbol);
    let today = chrono::Local::now().date_naive();

    let rows: Vec<Row> = entries
        .iter()
        .map(|entry| match entry {
            TimelineEntry::Lot { lot } => {
                let cost_basis = lot.remaining * lot.cost_per_share;
                // Unknown price: mark at the lot's own cost (0 unrealized).
                let mark = current_price.unwrap_or(lot.cost_per_share);
                let value = lot.remaining * mark;
                let unrealized = value - cost_basis;
                let days = today.signed_duration_since(lot.open_date).num_days();
                let long_term = is_long_term(lot.open_date, today);
                let term = if long_term { "Long" } else { "Short" };
                let term_color = if long_term {
                    Color::Green
                } else {
                    Color::Yellow
                };

                let qty_display = if (lot.remaining - lot.quantity).abs() > 0.01 {
                    format!("{:.0}/{:.0}", lot.remaining, lot.quantity)
                } else {
                    format!("{:.0}", lot.quantity)
                };

                let dim = lot.remaining.abs() < QTY_EPSILON;
                let style = if dim {
                    Style::default().fg(Color::DarkGray)
                } else {
                    Style::default()
                };

                Row::new(vec![
                    Cell::from(lot.open_date.format("%Y-%m-%d").to_string()).style(style),
                    Cell::from("BUY").style(if dim {
                        style
                    } else {
                        Style::default().fg(Color::Green)
                    }),
                    Cell::from(qty_display).style(style),
                    Cell::from(format!("${:.2}", lot.cost_per_share)).style(style),
                    Cell::from(format!("${:.2}", cost_basis)).style(style),
                    Cell::from(format!("${:.2}", value)).style(style),
                    Cell::from(format!("{:+.2}", unrealized)).style(if dim {
                        style
                    } else {
                        Style::default().fg(pnl_color(unrealized))
                    }),
                    Cell::from(format!("{:+.2}", lot.realized_pnl)).style(if dim {
                        style
                    } else {
                        Style::default().fg(pnl_color(lot.realized_pnl))
                    }),
                    Cell::from(format!("{days}d {term}")).style(if dim {
                        style
                    } else {
                        Style::default().fg(term_color)
                    }),
                ])
            }
            TimelineEntry::Sale { sale } => {
                let details: Vec<String> = sale
                    .consumed_lots
                    .iter()
                    .map(|cl| {
                        format!(
                            "{} {:.0}@${:.2}",
                            cl.open_date.format("%m/%d"),
                            cl.quantity,
                            cl.cost_per_share
                        )
                    })
                    .collect();

                Row::new(vec![
                    Cell::from(sale.sale_date.format("%Y-%m-%d").to_string()),
                    Cell::from("SELL").style(Style::default().fg(Color::Red)),
                    Cell::from(format!("{:.0}", sale.quantity)),
                    Cell::from(format!("${:.2}", sale.sale_price)),
                    Cell::from(""),
                    Cell::from(""),
                    Cell::from(""),
                    Cell::from(format!("{:+.2}", sale.realized_pnl))
                        .style(Style::default().fg(pnl_color(sale.realized_pnl))),
                    Cell::from(details.join(", ")).style(Style::default().fg(Color::DarkGray)),
                ])
            }
        })
        .collect();

    let widths = [
        Constraint::Length(12), // Date
        Constraint::Length(6),  // Action
        Constraint::Length(8),  // Qty
        Constraint::Length(10), // Price
        Constraint::Length(10), // Cost Basis
        Constraint::Length(10), // Value
        Constraint::Length(10), // Unrealized
        Constraint::Length(10), // Realized
        Constraint::Min(20),    // Details
    ];

    // Clamp scroll offset to valid range.
    let max_scroll = rows.len().saturating_sub(1);
    state.lot_detail_scroll = state.lot_detail_scroll.min(max_scroll);

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Lots & Transactions "),
        )
        .row_highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .column_spacing(1);

    let mut table_state = TableState::default().with_selected(Some(state.lot_detail_scroll));
    StatefulWidget::render(table, area, f.buffer_mut(), &mut table_state);
}

fn render_status(f: &mut Frame, area: Rect) {
    f.render_widget(
        Paragraph::new(
            " Esc back  ↑↓ scroll  g/G jump  b buy  s sell  : cmd  1 pos  2 orders  3 ops  q quit",
        )
        .style(Style::default().fg(Color::DarkGray)),
        area,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    #[test]
    fn long_term_is_calendar_based_not_365_days() {
        let open = date("2023-03-01");

        // Exactly one year later, across the 2024 leap day: 366 days elapsed, so
        // a `days > 365` rule would wrongly say long-term. It's still only one
        // year held, which is short-term.
        assert_eq!(
            open.signed_duration_since(date("2024-03-01")).num_days(),
            -366
        );
        assert!(!is_long_term(open, date("2024-03-01")));

        // One day past a year is long-term.
        assert!(is_long_term(open, date("2024-03-02")));

        // Sanity: well short and well long.
        assert!(!is_long_term(open, date("2023-09-01")));
        assert!(is_long_term(open, date("2025-01-01")));
    }
}
