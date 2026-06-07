use std::collections::BTreeMap;

use chrono::{Datelike, NaiveDate};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use super::lot_detail::price_opt;
use super::pnl_color;
use crate::app::AppState;
use crate::app::lots::TaxLotStore;
use crate::schwab::{QTY_EPSILON, is_long_term};

/// A P&L figure split by holding period (short vs long-term).
#[derive(Default, Clone, Copy)]
struct TermPnl {
    short: f64,
    long: f64,
}

impl TermPnl {
    fn total(&self) -> f64 {
        self.short + self.long
    }

    fn add(&mut self, other: &TermPnl) {
        self.short += other.short;
        self.long += other.long;
    }

    fn row(&self) -> [f64; 3] {
        [self.short, self.long, self.total()]
    }
}

/// Aggregate account-wide P&L across every tracked symbol.
///
/// Realized is split by both sale year and holding period (both are
/// unambiguous for a closed lot). Unrealized only gets a holding-period split:
/// "unrealized in 2023" has no clear meaning (purchase year? year-end value?),
/// so it's carried as a single mark-to-now figure.
pub struct AccountPnl {
    realized_by_year: BTreeMap<i32, TermPnl>,
    unrealized: TermPnl,
}

impl AccountPnl {
    fn realized_total(&self) -> TermPnl {
        let mut total = TermPnl::default();
        for p in self.realized_by_year.values() {
            total.add(p);
        }
        total
    }

    fn grand_total(&self) -> TermPnl {
        let mut total = self.realized_total();
        total.add(&self.unrealized);
        total
    }
}

pub fn compute(state: &AppState) -> AccountPnl {
    let today = chrono::Local::now().date_naive();
    compute_from(&state.lot_store, |symbol| price_opt(state, symbol), today)
}

/// Core aggregation, decoupled from `AppState` for testing. `price_of` supplies
/// the current price used to mark open lots (`None` if unknown); `today` dates
/// the holding period.
fn compute_from(
    store: &TaxLotStore,
    price_of: impl Fn(&str) -> Option<f64>,
    today: NaiveDate,
) -> AccountPnl {
    let mut realized_by_year: BTreeMap<i32, TermPnl> = BTreeMap::new();
    let mut unrealized = TermPnl::default();

    // Realized: grouped by sale year and split short/long-term by each consumed
    // lot's holding period. Per-consumed-lot realized sums back to `sale.realized_pnl`.
    for sales in store.sales.values() {
        for sale in sales {
            let entry = realized_by_year.entry(sale.sale_date.year()).or_default();
            for cl in &sale.consumed_lots {
                let realized = (sale.sale_price - cl.cost_per_share) * cl.quantity;
                if is_long_term(cl.open_date, sale.sale_date) {
                    entry.long += realized;
                } else {
                    entry.short += realized;
                }
            }
        }
    }

    // Unrealized: open lots marked to the latest known price, split by current
    // holding period only (no year axis).
    for (symbol, lots) in &store.lots {
        let current_price = price_of(symbol);
        for lot in lots {
            if lot.remaining.abs() < QTY_EPSILON {
                continue;
            }
            // Unknown price: mark at the lot's own cost so it contributes 0
            // unrealized rather than a phantom loss against a $0 mark.
            let mark = current_price.unwrap_or(lot.cost_per_share);
            let value = (mark - lot.cost_per_share) * lot.remaining;
            if is_long_term(lot.open_date, today) {
                unrealized.long += value;
            } else {
                unrealized.short += value;
            }
        }
    }

    AccountPnl {
        realized_by_year,
        unrealized,
    }
}

/// Height (including borders) the panel needs for the given data.
pub fn panel_height(pnl: &AccountPnl) -> u16 {
    // header + year rows + divider + realized + unrealized + divider + total
    let rows = 1 + pnl.realized_by_year.len() + 1 + 1 + 1 + 1 + 1;
    rows as u16 + 2
}

pub fn render(f: &mut Frame, area: Rect, pnl: &AccountPnl) {
    let label = Style::default().fg(Color::DarkGray);
    let bold = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    let current_year = chrono::Local::now().date_naive().year();

    let mut lines: Vec<Line> = vec![header_row(
        "Realized",
        &["Short-Term", "Long-Term", "Total"],
    )];

    // Realized rows, one per sale year.
    for (&year, p) in &pnl.realized_by_year {
        let period = if year == current_year {
            format!("{year} (YTD)")
        } else {
            year.to_string()
        };
        lines.push(value_row(&period, &p.row(), label));
    }

    // Subtotals and grand total.
    lines.push(divider(3));
    lines.push(value_row(
        "Total Realized",
        &pnl.realized_total().row(),
        label,
    ));
    lines.push(value_row("Unrealized", &pnl.unrealized.row(), label));
    lines.push(divider(3));
    lines.push(value_row("Total", &pnl.grand_total().row(), bold));

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Account P&L — realized by year, short / long-term ");
    f.render_widget(Paragraph::new(lines).block(block), area);
}

/// A bold yellow header: a left-aligned label column plus right-aligned columns.
fn header_row<'a>(label: &str, cols: &[&str]) -> Line<'a> {
    let style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);
    let mut spans = vec![Span::styled(format!(" {label:<14}"), style)];
    for c in cols {
        spans.push(Span::styled(format!("{c:>13}"), style));
    }
    Line::from(spans)
}

/// A data row: a left-aligned label and P&L-colored, right-aligned values.
fn value_row<'a>(label: &str, vals: &[f64], label_style: Style) -> Line<'a> {
    let mut spans = vec![Span::styled(format!(" {label:<14}"), label_style)];
    for &v in vals {
        spans.push(Span::styled(
            format!("{v:>+13.2}"),
            Style::default().fg(pnl_color(v)),
        ));
    }
    Line::from(spans)
}

fn divider<'a>(n_cols: usize) -> Line<'a> {
    Line::from(Span::styled(
        " ".to_string() + &"-".repeat(14 + 13 * n_cols),
        Style::default().fg(Color::DarkGray),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schwab::{ConsumedLot, LotSale, TaxLot};

    fn date(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    #[test]
    fn realized_split_by_year_and_term() {
        let mut store = TaxLotStore::default();

        // 2023 sale: a long-term lot (held ~2 years) and a short-term lot (held ~1 month).
        store.sales.insert(
            "AAA".to_string(),
            vec![LotSale {
                sale_date: date("2023-06-01"),
                quantity: 20.0,
                sale_price: 100.0,
                seq: 0,
                consumed_lots: vec![
                    ConsumedLot {
                        open_date: date("2021-01-01"), // long-term
                        quantity: 10.0,
                        cost_per_share: 90.0,
                    },
                    ConsumedLot {
                        open_date: date("2023-05-01"), // short-term
                        quantity: 10.0,
                        cost_per_share: 80.0,
                    },
                ],
                realized_pnl: 300.0,
            }],
        );

        let pnl = compute_from(&store, |_| Some(0.0), date("2024-01-01"));

        // Realized in 2023 is split ST/LT; per-lot sums back to the sale total.
        let y2023 = pnl.realized_by_year.get(&2023).copied().unwrap();
        assert!((y2023.long - 100.0).abs() < 1e-9); // (100-90)*10
        assert!((y2023.short - 200.0).abs() < 1e-9); // (100-80)*10
        assert!((y2023.total() - 300.0).abs() < 1e-9);
        assert!((pnl.realized_total().total() - 300.0).abs() < 1e-9);
    }

    #[test]
    fn unrealized_split_by_term_only() {
        let mut store = TaxLotStore::default();
        store.lots.insert(
            "BBB".to_string(),
            vec![
                // Long-term open lot, opened 2022.
                TaxLot {
                    open_date: date("2022-01-01"),
                    quantity: 10.0,
                    cost_per_share: 50.0,
                    remaining: 10.0,
                    realized_pnl: 0.0,
                    seq: 0,
                },
                // Short-term open lot, opened 2024.
                TaxLot {
                    open_date: date("2024-05-01"),
                    quantity: 5.0,
                    cost_per_share: 60.0,
                    remaining: 5.0,
                    realized_pnl: 0.0,
                    seq: 1,
                },
            ],
        );

        // Mark to $70.
        let pnl = compute_from(&store, |_| Some(70.0), date("2024-06-01"));

        // Unrealized is split by holding period only.
        assert!((pnl.unrealized.long - 200.0).abs() < 1e-9); // (70-50)*10
        assert!((pnl.unrealized.short - 50.0).abs() < 1e-9); // (70-60)*5

        // No realized activity, and the grand total reconciles.
        assert!(pnl.realized_by_year.is_empty());
        assert!((pnl.grand_total().total() - 250.0).abs() < 1e-9);
    }

    #[test]
    fn unknown_price_marks_at_cost() {
        let mut store = TaxLotStore::default();
        store.lots.insert(
            "CCC".to_string(),
            vec![TaxLot {
                open_date: date("2022-01-01"),
                quantity: 10.0,
                cost_per_share: 50.0,
                remaining: 10.0,
                realized_pnl: 0.0,
                seq: 0,
            }],
        );

        // Price unknown: the lot is marked at its own cost, so it contributes
        // zero unrealized rather than a phantom -$500 loss against a $0 mark.
        let pnl = compute_from(&store, |_| None, date("2024-06-01"));

        assert!(pnl.unrealized.long.abs() < 1e-9);
        assert!(pnl.unrealized.short.abs() < 1e-9);
        assert!(pnl.grand_total().total().abs() < 1e-9);
    }
}
