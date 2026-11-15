//! Compact text output for MCP tools.
//!
//! Everything a tool returns lands in the model's context, so output is terse:
//! a `# title` line, then CSV with a header row. Numbers are rounded and
//! trailing zeros dropped; unknown values are empty cells. These functions take
//! labels, never account numbers or hashes.

use std::collections::HashMap;
use std::fmt::Write as _;

use chrono::{DateTime, Local, NaiveDate, Utc};

use crate::schwab::{
    AccountBalances, AssetType, ExpirationInfo, ExpiryKind, Order, OrderStatus, Position, PutCall,
    Quote, TaxLot, is_long_term,
};

/// `x` rounded to `dp` decimals, trailing zeros and `-0` removed.
pub fn num(x: f64, dp: usize) -> String {
    if !x.is_finite() {
        return String::new();
    }
    let s = format!("{x:.dp$}");
    let s = if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    };
    if s == "-0" { "0".to_string() } else { s }
}

/// `num` for an optional value: empty when `None`.
pub fn opt(x: Option<f64>, dp: usize) -> String {
    x.map(|v| num(v, dp)).unwrap_or_default()
}

/// A CSV cell: quoted only when it has a comma or quote.
fn cell(s: &str) -> String {
    if s.contains(',') || s.contains('"') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn row(out: &mut String, cells: &[String]) {
    let line: Vec<String> = cells.iter().map(|c| cell(c)).collect();
    out.push_str(&line.join(","));
    out.push('\n');
}

fn asset_code(t: &AssetType) -> &'static str {
    match t {
        AssetType::Equity => "EQ",
        AssetType::Option => "OPT",
        AssetType::MutualFund => "MF",
        AssetType::CashEquivalent => "CASH",
        AssetType::FixedIncome => "FI",
        _ => "OTHER",
    }
}

// ── Positions ────────────────────────────────────────────────────────────────

pub fn positions(label: &str, positions: &[Position]) -> String {
    let mut out = format!("# {label}: {} positions\n", positions.len());
    if positions.is_empty() {
        return out;
    }
    out.push_str("sym,type,qty,avg,price,mv,day_pnl,open_pnl\n");
    let mut sorted: Vec<&Position> = positions.iter().collect();
    sorted.sort_by(|a, b| b.market_value.abs().total_cmp(&a.market_value.abs()));
    for p in sorted {
        row(
            &mut out,
            &[
                p.symbol.clone(),
                asset_code(&p.asset_type).to_string(),
                num(p.quantity, 4),
                num(p.average_cost, 4),
                num(p.current_price, 4),
                num(p.market_value, 2),
                num(p.day_pnl, 2),
                num(p.open_pnl, 2),
            ],
        );
    }
    out
}

// ── Balances ─────────────────────────────────────────────────────────────────

pub fn balances(label: &str, b: &AccountBalances) -> String {
    let mut out = format!("# {label} ({})\n", b.account_type);
    let fields: [(&str, Option<f64>); 11] = [
        ("liquidation_value", b.liquidation_value),
        ("cash", b.cash),
        ("buying_power", b.buying_power),
        ("option_buying_power", b.option_buying_power),
        ("day_trading_buying_power", b.day_trading_buying_power),
        ("available_funds", b.available_funds),
        ("equity", b.equity),
        ("maintenance_requirement", b.maintenance_requirement),
        ("margin_balance", b.margin_balance),
        (
            "cash_available_for_withdrawal",
            b.cash_available_for_withdrawal,
        ),
        ("unsettled_cash", b.unsettled_cash),
    ];
    for (name, value) in fields {
        if let Some(v) = value {
            let _ = writeln!(out, "{name}={}", num(v, 2));
        }
    }
    if b.in_call {
        out.push_str("in_call=true\n");
    }
    out
}

// ── Orders ───────────────────────────────────────────────────────────────────

fn status_name(s: &OrderStatus) -> String {
    // Schwab's own SCREAMING_CASE names, via serde.
    serde_json::to_value(s)
        .ok()
        .and_then(|v| v.as_str().map(str::to_string))
        .unwrap_or_else(|| format!("{s:?}").to_uppercase())
}

fn enum_name<T: serde::Serialize + std::fmt::Debug>(v: &T) -> String {
    serde_json::to_value(v)
        .ok()
        .and_then(|v| v.as_str().map(str::to_string))
        .unwrap_or_else(|| format!("{v:?}").to_uppercase())
}

/// Statuses that can still fill or be changed.
pub fn is_open_status(s: &OrderStatus) -> bool {
    !matches!(
        s,
        OrderStatus::Filled
            | OrderStatus::Canceled
            | OrderStatus::Rejected
            | OrderStatus::Expired
            | OrderStatus::Replaced
    )
}

/// Does `order` match a status filter: `open`, `all`, or a Schwab status
/// name (`FILLED`, `canceled`, ...)?
pub fn status_matches(filter: &str, status: &OrderStatus) -> bool {
    match filter.trim().to_ascii_lowercase().as_str() {
        "" | "all" => true,
        "open" => is_open_status(status),
        "cancelled" => matches!(status, OrderStatus::Canceled),
        other => status_name(status).eq_ignore_ascii_case(other),
    }
}

pub fn orders(label: &str, orders: &[Order], range: &str) -> String {
    let mut out = format!("# {label}: {} orders, {range}\n", orders.len());
    if orders.is_empty() {
        return out;
    }
    out.push_str("id,entered,sym,instr,qty,filled,type,price,stop,status,tif,fill_px\n");
    let mut sorted: Vec<&Order> = orders.iter().collect();
    sorted.sort_by(|a, b| b.entered_time.cmp(&a.entered_time));
    for o in sorted {
        row(
            &mut out,
            &[
                o.order_id.to_string(),
                o.entered_time
                    .with_timezone(&Local)
                    .format("%Y-%m-%d %H:%M")
                    .to_string(),
                o.symbol.clone(),
                enum_name(&o.instruction),
                num(o.quantity, 4),
                num(o.filled_quantity, 4),
                enum_name(&o.order_type),
                opt(o.price, 4),
                opt(o.stop_price, 4),
                status_name(&o.status),
                o.duration.as_ref().map(enum_name).unwrap_or_default(),
                opt(o.fill_price, 4),
            ],
        );
    }
    out
}

// ── Quotes ───────────────────────────────────────────────────────────────────

pub fn quotes(requested: &[String], quotes: &HashMap<String, Quote>) -> String {
    let mut out = String::from(
        "# quotes\nsym,last,bid,ask,mark,chg,chg_pct,vol,open,high,low,prev_close,wk52_hi,wk52_lo\n",
    );
    let mut missing = Vec::new();
    for sym in requested {
        let Some(q) = quotes.get(sym) else {
            missing.push(sym.as_str());
            continue;
        };
        let nz = |x: f64| if x == 0.0 { String::new() } else { num(x, 4) };
        row(
            &mut out,
            &[
                q.symbol.clone(),
                num(q.last_price, 4),
                nz(q.bid_price),
                nz(q.ask_price),
                nz(q.mark),
                num(q.net_change, 4),
                num(q.net_percent_change, 2),
                q.volume.to_string(),
                nz(q.open_price),
                nz(q.high_price),
                nz(q.low_price),
                nz(q.close_price),
                nz(q.week_52_high),
                nz(q.week_52_low),
            ],
        );
    }
    if !missing.is_empty() {
        let _ = writeln!(out, "not found: {}", missing.join(" "));
    }
    out
}

// ── Tax lots ─────────────────────────────────────────────────────────────────

/// One open lot with its holding period and unrealized gain.
#[derive(Debug, Clone, PartialEq)]
pub struct LotRow {
    pub symbol: String,
    pub open_date: NaiveDate,
    pub qty: f64,
    pub cost_per_share: f64,
    pub basis: f64,
    pub price: Option<f64>,
    pub unrealized: Option<f64>,
    pub long_term: bool,
    /// Days until the lot turns long-term (0 once it has).
    pub days_to_lt: i64,
}

/// First day a lot opened on `open` counts as long-term: the day after its
/// one-year anniversary (see [`is_long_term`]).
pub fn long_term_date(open: NaiveDate) -> Option<NaiveDate> {
    open.checked_add_months(chrono::Months::new(12))?.succ_opt()
}

/// Open lots (remaining shares > 0) with prices from `prices`, sorted by
/// symbol then open date.
pub fn lot_rows<'a>(
    lots: impl IntoIterator<Item = (&'a String, &'a Vec<TaxLot>)>,
    prices: &HashMap<String, f64>,
    today: NaiveDate,
) -> Vec<LotRow> {
    let mut rows = Vec::new();
    for (symbol, lots) in lots {
        for lot in lots
            .iter()
            .filter(|l| l.remaining > crate::schwab::QTY_EPSILON)
        {
            let price = prices.get(symbol).copied().filter(|p| *p > 0.0);
            let basis = lot.remaining * lot.cost_per_share;
            let long_term = is_long_term(lot.open_date, today);
            let days_to_lt = if long_term {
                0
            } else {
                long_term_date(lot.open_date)
                    .map(|d| (d - today).num_days().max(0))
                    .unwrap_or(0)
            };
            rows.push(LotRow {
                symbol: symbol.clone(),
                open_date: lot.open_date,
                qty: lot.remaining,
                cost_per_share: lot.cost_per_share,
                basis,
                price,
                unrealized: price.map(|p| p * lot.remaining - basis),
                long_term,
                days_to_lt,
            });
        }
    }
    rows.sort_by(|a, b| a.symbol.cmp(&b.symbol).then(a.open_date.cmp(&b.open_date)));
    rows
}

pub fn lots(label: &str, rows: &[LotRow], last_sync: Option<&str>) -> String {
    let sync = last_sync
        .map(|d| format!(", synced {d}"))
        .unwrap_or_else(|| ", never synced".to_string());
    let mut out = format!("# {label}: {} open lots{sync}\n", rows.len());
    if rows.is_empty() {
        return out;
    }
    out.push_str("sym,opened,qty,cost_sh,basis,price,unreal,unreal_pct,term,days_to_lt\n");
    let (mut st, mut lt) = (0.0, 0.0);
    for r in rows {
        if let Some(u) = r.unrealized {
            if r.long_term {
                lt += u;
            } else {
                st += u;
            }
        }
        let pct = r
            .unrealized
            .filter(|_| r.basis.abs() > 1e-9)
            .map(|u| u / r.basis * 100.0);
        row(
            &mut out,
            &[
                r.symbol.clone(),
                r.open_date.to_string(),
                num(r.qty, 4),
                num(r.cost_per_share, 4),
                num(r.basis, 2),
                opt(r.price, 4),
                opt(r.unrealized, 2),
                opt(pct, 1),
                if r.long_term { "LT" } else { "ST" }.to_string(),
                if r.long_term {
                    String::new()
                } else {
                    r.days_to_lt.to_string()
                },
            ],
        );
    }
    let _ = writeln!(
        out,
        "total_unreal_st={},total_unreal_lt={}",
        num(st, 2),
        num(lt, 2)
    );
    out
}

// ── Expirations ──────────────────────────────────────────────────────────────

pub fn kind_code(k: ExpiryKind) -> &'static str {
    match k {
        ExpiryKind::Weekly => "W",
        ExpiryKind::Monthly => "M",
        ExpiryKind::Quarterly => "Q",
        ExpiryKind::EndOfMonth => "EOM",
        ExpiryKind::Other => "",
    }
}

pub fn expirations(symbol: &str, list: &[ExpirationInfo], omitted: usize) -> String {
    let mut out = format!("# {symbol}: {} expirations\n", list.len());
    if omitted > 0 {
        let _ = writeln!(out, "({omitted} later expirations omitted; raise max_dte)");
    }
    if list.is_empty() {
        return out;
    }
    out.push_str("date,dte,kind\n");
    for e in list {
        let mut kind = kind_code(e.kind).to_string();
        if !e.standard {
            kind.push_str(" nonstd");
        }
        row(
            &mut out,
            &[e.date.to_string(), e.days_to_exp.to_string(), kind],
        );
    }
    out
}

// ── Auth ─────────────────────────────────────────────────────────────────────

/// "in 3d 4h", "in 12m", "2h ago".
pub fn relative(t: DateTime<Utc>, now: DateTime<Utc>) -> String {
    let secs = (t - now).num_seconds();
    let abs = secs.unsigned_abs();
    let (d, h, m) = (abs / 86_400, abs % 86_400 / 3600, abs % 3600 / 60);
    let span = if d > 0 {
        format!("{d}d {h}h")
    } else if h > 0 {
        format!("{h}h {m}m")
    } else {
        format!("{m}m")
    };
    if secs >= 0 {
        format!("in {span}")
    } else {
        format!("{span} ago")
    }
}

pub fn side_code(side: PutCall) -> &'static str {
    match side {
        PutCall::Call => "C",
        PutCall::Put => "P",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_trims_and_rounds() {
        assert_eq!(num(1.5000, 4), "1.5");
        assert_eq!(num(2.0, 2), "2");
        assert_eq!(num(-0.0001, 2), "0");
        assert_eq!(num(0.123456, 3), "0.123");
        assert_eq!(num(f64::NAN, 2), "");
        assert_eq!(opt(None, 2), "");
    }

    #[test]
    fn long_term_starts_the_day_after_the_anniversary() {
        let open = NaiveDate::from_ymd_opt(2025, 3, 10).unwrap();
        let lt = long_term_date(open).unwrap();
        assert_eq!(lt, NaiveDate::from_ymd_opt(2026, 3, 11).unwrap());
        assert!(!is_long_term(open, lt.pred_opt().unwrap()));
        assert!(is_long_term(open, lt));
    }

    #[test]
    fn lot_rows_compute_term_and_unrealized() {
        let today = NaiveDate::from_ymd_opt(2026, 11, 15).unwrap();
        let mut old = TaxLot::new(NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(), 10.0, 20.0, 0);
        old.remaining = 4.0; // partly sold
        let new = TaxLot::new(NaiveDate::from_ymd_opt(2026, 11, 1).unwrap(), 5.0, 30.0, 1);
        let mut sold = TaxLot::new(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(), 1.0, 1.0, 2);
        sold.remaining = 0.0;
        let mut lots = HashMap::new();
        lots.insert("TEST".to_string(), vec![new, old, sold]);
        let mut prices = HashMap::new();
        prices.insert("TEST".to_string(), 25.0);

        let rows = lot_rows(&lots, &prices, today);
        assert_eq!(rows.len(), 2, "fully sold lots are skipped");
        let (a, b) = (&rows[0], &rows[1]);
        assert_eq!(a.open_date, NaiveDate::from_ymd_opt(2024, 1, 2).unwrap());
        assert!(a.long_term);
        assert_eq!(a.days_to_lt, 0);
        assert_eq!(a.basis, 80.0);
        assert_eq!(a.unrealized, Some(20.0)); // 4 * 25 - 80
        assert!(!b.long_term);
        // Opened 2026-11-01 -> long-term on 2027-11-02, 352 days away.
        assert_eq!(b.days_to_lt, 352);
        assert_eq!(b.unrealized, Some(-25.0)); // 5 * 25 - 150

        let text = super::lots("Test", &rows, Some("2026-11-14"));
        assert!(text.contains("TEST,2024-01-02,4,20,80,25,20,25,LT,"));
        assert!(text.contains("total_unreal_st=-25,total_unreal_lt=20"));
    }

    #[test]
    fn lot_rows_without_a_price_leave_gain_blank() {
        let today = NaiveDate::from_ymd_opt(2026, 11, 15).unwrap();
        let mut lots = HashMap::new();
        lots.insert(
            "TEST".to_string(),
            vec![TaxLot::new(
                NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
                1.0,
                10.0,
                0,
            )],
        );
        let rows = lot_rows(&lots, &HashMap::new(), today);
        assert_eq!(rows[0].unrealized, None);
    }

    #[test]
    fn order_status_filters() {
        assert!(status_matches("open", &OrderStatus::Working));
        assert!(!status_matches("open", &OrderStatus::Filled));
        assert!(status_matches("FILLED", &OrderStatus::Filled));
        assert!(status_matches("filled", &OrderStatus::Filled));
        assert!(status_matches("cancelled", &OrderStatus::Canceled));
        assert!(status_matches("all", &OrderStatus::Rejected));
        assert!(!status_matches("working", &OrderStatus::Filled));
    }

    #[test]
    fn relative_times() {
        let now = Utc::now();
        assert_eq!(relative(now + chrono::Duration::minutes(29), now), "in 29m");
        assert_eq!(relative(now + chrono::Duration::hours(50), now), "in 2d 2h");
        assert_eq!(relative(now - chrono::Duration::hours(3), now), "3h 0m ago");
    }
}
