use chrono::{DateTime, Months, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use schwab_marketdata::types as md;
use schwab_trader::types as tr;

pub use schwab_trader::types::{
    AccountsBaseInstrumentAssetType as AssetType, Duration as OrderDuration, Instruction,
    OrderType, Session as OrderSession, Status as OrderStatus,
};

// ── Tax Lots ─────────────────────────────────────────────────────────────────

/// Epsilon for floating-point share quantity comparisons.
pub const QTY_EPSILON: f64 = 1e-9;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxLot {
    pub open_date: NaiveDate,
    /// Original quantity when the lot was opened.
    pub quantity: f64,
    pub cost_per_share: f64,
    /// Shares still open (not yet sold). Starts equal to `quantity`.
    #[serde(default)]
    pub remaining: f64,
    /// Accumulated realized P&L from sales against this lot.
    #[serde(default)]
    pub realized_pnl: f64,
    /// Execution order from rebuild(), for correct intra-day display ordering.
    #[serde(skip)]
    pub seq: usize,
}

impl TaxLot {
    pub fn new(open_date: NaiveDate, quantity: f64, cost_per_share: f64, seq: usize) -> Self {
        Self {
            open_date,
            quantity,
            remaining: quantity,
            cost_per_share,
            realized_pnl: 0.0,
            seq,
        }
    }

    /// Original cost basis (quantity at open * cost per share).
    pub fn original_cost(&self) -> f64 {
        self.quantity * self.cost_per_share
    }
}

/// Long-term if held more than one year (IRS rule). The holding period starts
/// the day after acquisition, so a lot bought on `open` is long-term only once
/// `as_of` is strictly past `open + 1 year`. Calendar-based rather than a
/// 365-day count, so it stays correct when the window spans a leap day.
pub fn is_long_term(open: NaiveDate, as_of: NaiveDate) -> bool {
    match open.checked_add_months(Months::new(12)) {
        Some(one_year) => as_of > one_year,
        None => false,
    }
}

/// A lot that was consumed (fully or partially) by a sale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumedLot {
    pub open_date: NaiveDate,
    pub quantity: f64,
    pub cost_per_share: f64,
}

impl ConsumedLot {
    pub fn total_cost(&self) -> f64 {
        self.quantity * self.cost_per_share
    }
}

/// A recorded sale/closing transaction for a symbol.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LotSale {
    pub sale_date: NaiveDate,
    pub quantity: f64,
    pub sale_price: f64,
    /// Execution order from rebuild(), for correct intra-day display ordering.
    #[serde(skip)]
    pub seq: usize,
    /// Which lots were consumed by this sale.
    pub consumed_lots: Vec<ConsumedLot>,
    pub realized_pnl: f64,
}

/// Method used to select which lots to sell from.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum LotSelectionMethod {
    #[serde(alias = "Lifo")]
    Lifo,
    #[default]
    #[serde(alias = "Fifo")]
    Fifo,
    #[serde(alias = "Hifo")]
    Hifo,
}

impl LotSelectionMethod {
    pub fn next(self) -> Self {
        match self {
            Self::Lifo => Self::Fifo,
            Self::Fifo => Self::Hifo,
            Self::Hifo => Self::Lifo,
        }
    }

    /// Map to the Schwab order-API tax lot method. HIFO maps to `HIGH_COST`,
    /// Schwab's highest-cost-first selection.
    pub fn to_api_method(self) -> tr::TaxLotMethod {
        match self {
            Self::Fifo => tr::TaxLotMethod::Fifo,
            Self::Lifo => tr::TaxLotMethod::Lifo,
            Self::Hifo => tr::TaxLotMethod::HighCost,
        }
    }

    /// Recover from a Schwab order-API tax lot method echoed back on an order.
    /// Returns `None` for methods we don't model locally.
    pub fn from_api_method(m: tr::TaxLotMethod) -> Option<Self> {
        match m {
            tr::TaxLotMethod::Fifo => Some(Self::Fifo),
            tr::TaxLotMethod::Lifo => Some(Self::Lifo),
            tr::TaxLotMethod::HighCost => Some(Self::Hifo),
            _ => None,
        }
    }
}

/// One leg of a sell order with its tax lot selection method. The auto-selection
/// strategy may split a sell across two legs (long-term FIFO + short-term LIFO).
#[derive(Debug, Clone, PartialEq)]
pub struct SellLeg {
    pub quantity: f64,
    pub method: LotSelectionMethod,
}

impl std::str::FromStr for LotSelectionMethod {
    type Err = crate::util::PrefixParseError;

    /// Case-insensitive unique-prefix parse, matching command-mode behavior:
    /// `f`/`l`/`h` (and any longer prefix) resolve uniquely; unknown or empty
    /// input is rejected rather than silently defaulting.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::util::prefix_match(
            s.trim().to_lowercase().as_str(),
            &[
                ("fifo", Self::Fifo),
                ("lifo", Self::Lifo),
                ("hifo", Self::Hifo),
            ],
        )
    }
}

impl std::fmt::Display for LotSelectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Lifo => "LIFO",
            Self::Fifo => "FIFO",
            Self::Hifo => "HIFO",
        };
        write!(f, "{s}")
    }
}

/// Extract symbol from any TransactionInstrument variant.
pub fn transaction_instrument_symbol(inst: &tr::TransactionInstrument) -> Option<&str> {
    match inst {
        tr::TransactionInstrument::TransactionEquity(e) => e.symbol.as_deref(),
        tr::TransactionInstrument::TransactionCashEquivalent(e) => e.symbol.as_deref(),
        tr::TransactionInstrument::Currency(c) => c.symbol.as_deref(),
        tr::TransactionInstrument::TransactionFixedIncome(f) => f.symbol.as_deref(),
        tr::TransactionInstrument::TransactionMutualFund(m) => m.symbol.as_deref(),
        tr::TransactionInstrument::TransactionOption(o) => o.symbol.as_deref(),
        tr::TransactionInstrument::CollectiveInvestment(c) => c.symbol.as_deref(),
        tr::TransactionInstrument::Forex(f) => f.symbol.as_deref(),
        tr::TransactionInstrument::Future(_) => None,
        tr::TransactionInstrument::Index(_) => None,
        tr::TransactionInstrument::Product(p) => p.symbol.as_deref(),
    }
}

// ── Domain types ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub cash_balance: f64,
    pub buying_power: f64,
}

/// User-visible account number, the encrypted hash used for API calls, and
/// the user-set nickname from /userPreference when one is configured.
#[derive(Debug, Clone)]
pub struct AccountSummary {
    pub number: String,
    pub hash: String,
    pub nickname: Option<String>,
}

impl AccountSummary {
    /// "Nickname (Number)" when a nickname is set, otherwise just the number.
    /// Used everywhere the account is shown to the user (picker, summary, status).
    pub fn display_label(&self) -> String {
        match &self.nickname {
            Some(n) => format!("{n} ({})", self.number),
            None => self.number.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub average_cost: f64,
    pub current_price: f64,
    pub market_value: f64,
    pub day_pnl: f64,
    pub day_pnl_pct: f64,
    pub open_pnl: f64,
    pub asset_type: AssetType,
    /// How many shares existed at the previous session's close.
    /// Used to detect same-day purchases for day P&L calculation.
    pub previous_session_qty: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub symbol: String,
    pub last_price: f64,
    pub close_price: f64,
    pub bid_price: f64,
    pub ask_price: f64,
    pub bid_size: i64,
    pub ask_size: i64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub net_change: f64,
    pub net_percent_change: f64,
    pub volume: i64,
    pub mark: f64,
    pub week_52_high: f64,
    pub week_52_low: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub order_id: i64,
    pub symbol: String,
    pub quantity: f64,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub order_type: OrderType,
    pub instruction: Instruction,
    pub status: OrderStatus,
    pub duration: Option<OrderDuration>,
    pub session: Option<OrderSession>,
    pub filled_quantity: f64,
    pub remaining_quantity: f64,
    pub cancelable: bool,
    pub editable: bool,
    pub entered_time: DateTime<Utc>,
    /// Weighted-average fill price from execution legs (None if no fills yet).
    pub fill_price: Option<f64>,
    /// Tax lot selection method, echoed back by the orders endpoint (sells only).
    /// Used to recover which method a sale used, since the transactions API does
    /// not report it. See SCHWAB_API_ISSUES.md #5.
    pub tax_lot_method: Option<LotSelectionMethod>,
}

// ── Shared helpers ────────────────────────────────────────────────────────────
// I'd be ok with just supporting stocks and options for now
fn instrument_symbol(inst: &tr::AccountsInstrument) -> Option<&str> {
    match inst {
        tr::AccountsInstrument::Equity(i) => i.0.symbol.as_deref(),
        tr::AccountsInstrument::CashEquivalent(i) => i.symbol.as_deref(),
        tr::AccountsInstrument::FixedIncome(i) => i.symbol.as_deref(),
        tr::AccountsInstrument::MutualFund(i) => i.0.symbol.as_deref(),
        tr::AccountsInstrument::Option(i) => i.symbol.as_deref(),
    }
}

fn instrument_asset_type(inst: &tr::AccountsInstrument) -> AssetType {
    match inst {
        tr::AccountsInstrument::Equity(_) => AssetType::Equity,
        tr::AccountsInstrument::CashEquivalent(_) => AssetType::CashEquivalent,
        tr::AccountsInstrument::FixedIncome(_) => AssetType::FixedIncome,
        tr::AccountsInstrument::MutualFund(_) => AssetType::MutualFund,
        tr::AccountsInstrument::Option(_) => AssetType::Option,
    }
}

// ── Conversions ───────────────────────────────────────────────────────────────

impl TryFrom<tr::Position> for Position {
    type Error = ();

    fn try_from(p: tr::Position) -> Result<Self, ()> {
        let instrument = p.instrument.ok_or(())?;
        let symbol = instrument_symbol(&instrument).ok_or(())?.to_string();
        let asset_type = instrument_asset_type(&instrument);

        let long_qty = p.long_quantity.unwrap_or(0.0);
        let short_qty = p.short_quantity.unwrap_or(0.0);
        let quantity = long_qty - short_qty;
        let market_value = p.market_value.unwrap_or(0.0);
        let average_cost = p.average_price.unwrap_or(0.0);
        let current_price = if quantity != 0.0 {
            market_value / quantity
        } else {
            average_cost
        };
        let open_pnl = if quantity >= 0.0 {
            p.long_open_profit_loss.unwrap_or(0.0)
        } else {
            p.short_open_profit_loss.unwrap_or(0.0)
        };
        let prev_long = p.previous_session_long_quantity.unwrap_or(0.0);
        let prev_short = p.previous_session_short_quantity.unwrap_or(0.0);
        let previous_session_qty = prev_long - prev_short;

        Ok(Position {
            symbol,
            quantity,
            average_cost,
            current_price,
            market_value,
            day_pnl: p.current_day_profit_loss.unwrap_or(0.0),
            day_pnl_pct: p.current_day_profit_loss_percentage.unwrap_or(0.0),
            open_pnl,
            asset_type,
            previous_session_qty,
        })
    }
}

impl Quote {
    pub fn from_api(symbol: &str, obj: &md::QuoteResponseObject) -> Option<Quote> {
        match obj {
            md::QuoteResponseObject::EquityResponse(r) => {
                let q = r.quote.as_ref()?;
                Some(Quote {
                    symbol: symbol.to_string(),
                    last_price: q.last_price.unwrap_or(0.0),
                    close_price: q.close_price.unwrap_or(0.0),
                    bid_price: q.bid_price.unwrap_or(0.0),
                    ask_price: q.ask_price.unwrap_or(0.0),
                    bid_size: q.bid_size.unwrap_or(0) as i64,
                    ask_size: q.ask_size.unwrap_or(0) as i64,
                    open_price: q.open_price.unwrap_or(0.0),
                    high_price: q.high_price.unwrap_or(0.0),
                    low_price: q.low_price.unwrap_or(0.0),
                    net_change: q.net_change.unwrap_or(0.0),
                    net_percent_change: q.net_percent_change.unwrap_or(0.0),
                    volume: q.total_volume.unwrap_or(0),
                    mark: q.mark.unwrap_or(0.0),
                    week_52_high: q.x52week_high.unwrap_or(0.0),
                    week_52_low: q.x52week_low.unwrap_or(0.0),
                })
            }
            md::QuoteResponseObject::IndexResponse(r) => {
                let q = r.quote.as_ref()?;
                Some(Quote {
                    symbol: symbol.to_string(),
                    last_price: q.last_price.unwrap_or(0.0),
                    close_price: q.close_price.unwrap_or(0.0),
                    open_price: q.open_price.unwrap_or(0.0),
                    high_price: q.high_price.unwrap_or(0.0),
                    low_price: q.low_price.unwrap_or(0.0),
                    net_change: q.net_change.unwrap_or(0.0),
                    net_percent_change: q.net_percent_change.unwrap_or(0.0),
                    volume: q.total_volume.unwrap_or(0),
                    // indices don't trade; no bid/ask, mark, or 52wk range
                    bid_price: 0.0,
                    ask_price: 0.0,
                    bid_size: 0,
                    ask_size: 0,
                    mark: 0.0,
                    week_52_high: 0.0,
                    week_52_low: 0.0,
                })
            }
            md::QuoteResponseObject::FutureResponse(r) => {
                let q = r.quote.as_ref()?;
                Some(Quote {
                    symbol: symbol.to_string(),
                    last_price: q.last_price.unwrap_or(0.0),
                    close_price: q.close_price.unwrap_or(0.0),
                    bid_price: q.bid_price.unwrap_or(0.0),
                    ask_price: q.ask_price.unwrap_or(0.0),
                    bid_size: q.bid_size.unwrap_or(0) as i64,
                    ask_size: q.ask_size.unwrap_or(0) as i64,
                    open_price: q.open_price.unwrap_or(0.0),
                    high_price: q.high_price.unwrap_or(0.0),
                    low_price: q.low_price.unwrap_or(0.0),
                    net_change: q.net_change.unwrap_or(0.0),
                    net_percent_change: q.future_percent_change.unwrap_or(0.0),
                    volume: q.total_volume.unwrap_or(0),
                    mark: q.mark.unwrap_or(0.0),
                    week_52_high: 0.0,
                    week_52_low: 0.0,
                })
            }
            md::QuoteResponseObject::OptionResponse(r) => {
                let q = r.quote.as_ref()?;
                Some(Quote {
                    symbol: symbol.to_string(),
                    last_price: q.last_price.unwrap_or(0.0),
                    close_price: q.close_price.unwrap_or(0.0),
                    bid_price: q.bid_price.unwrap_or(0.0),
                    ask_price: q.ask_price.unwrap_or(0.0),
                    bid_size: q.bid_size.unwrap_or(0) as i64,
                    ask_size: q.ask_size.unwrap_or(0) as i64,
                    open_price: q.open_price.unwrap_or(0.0),
                    high_price: q.high_price.unwrap_or(0.0),
                    low_price: q.low_price.unwrap_or(0.0),
                    net_change: q.net_change.unwrap_or(0.0),
                    net_percent_change: q.net_percent_change.unwrap_or(0.0),
                    volume: q.total_volume.unwrap_or(0),
                    mark: q.mark.unwrap_or(0.0),
                    week_52_high: 0.0,
                    week_52_low: 0.0,
                })
            }
            _ => None,
        }
    }
}

impl TryFrom<tr::Order> for Order {
    type Error = ();

    fn try_from(o: tr::Order) -> Result<Self, ()> {
        // why just the first leg of the order?
        let leg = o.order_leg_collection.into_iter().next().ok_or(())?;
        let instrument = leg.instrument.ok_or(())?;
        let symbol = instrument_symbol(&instrument).ok_or(())?.to_string();

        // Compute VWAP from execution legs across all activities.
        let fill_price = {
            let (mut total_qty, mut total_cost) = (0.0_f64, 0.0_f64);
            for activity in &o.order_activity_collection {
                for exec in &activity.execution_legs {
                    if let (Some(p), Some(q)) = (exec.price, exec.quantity) {
                        total_qty += q;
                        total_cost += p * q;
                    }
                }
            }
            if total_qty > 0.0 {
                Some(total_cost / total_qty)
            } else {
                None
            }
        };

        Ok(Order {
            order_id: o.order_id.ok_or(())?,
            symbol,
            quantity: leg.quantity.unwrap_or(0.0),
            price: o.price,
            stop_price: o.stop_price,
            order_type: o.order_type.unwrap_or(OrderType::Unknown),
            instruction: leg.instruction.ok_or(())?,
            status: o.status.unwrap_or(OrderStatus::Unknown),
            duration: o.duration,
            session: o.session,
            filled_quantity: o.filled_quantity.unwrap_or(0.0),
            remaining_quantity: o.remaining_quantity.unwrap_or(0.0),
            cancelable: o.cancelable,
            editable: o.editable,
            entered_time: o.entered_time.unwrap_or_else(chrono::Utc::now),
            fill_price,
            tax_lot_method: o
                .tax_lot_method
                .and_then(LotSelectionMethod::from_api_method),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lot_method_api_roundtrip() {
        for m in [
            LotSelectionMethod::Fifo,
            LotSelectionMethod::Lifo,
            LotSelectionMethod::Hifo,
        ] {
            assert_eq!(
                LotSelectionMethod::from_api_method(m.to_api_method()),
                Some(m)
            );
        }
    }

    #[test]
    fn lot_method_api_mapping() {
        assert_eq!(
            LotSelectionMethod::Fifo.to_api_method(),
            tr::TaxLotMethod::Fifo
        );
        assert_eq!(
            LotSelectionMethod::Lifo.to_api_method(),
            tr::TaxLotMethod::Lifo
        );
        // HIFO maps to Schwab's HIGH_COST (highest-cost-first).
        assert_eq!(
            LotSelectionMethod::Hifo.to_api_method(),
            tr::TaxLotMethod::HighCost
        );
    }

    #[test]
    fn lot_method_from_unmodeled_api_method_is_none() {
        assert_eq!(
            LotSelectionMethod::from_api_method(tr::TaxLotMethod::AverageCost),
            None
        );
    }
}
