//! Domain model for option chains.
//!
//! The Schwab market-data `get_chain` response is deeply nested (date-keyed
//! maps of strike-keyed maps of contract lists, all with optional fields). This
//! module flattens it into a thin, sorted snapshot the rest of the app can
//! render without touching the generated SDK types.

use std::collections::BTreeMap;

use chrono::NaiveDate;

use schwab_marketdata::types as md;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PutCall {
    Call,
    Put,
}

/// How an expiration is classified (drives monthly/weekly filters).
///
/// Schwab's letters don't mean what they look like: `S` ("standard") is the
/// classic 3rd-Friday monthly, while `M` is the *end-of-month* expiration.
/// Verified against live chain data — the 3rd-Friday dates come back `S`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpiryKind {
    Weekly,
    /// Standard 3rd-Friday expiration (Schwab type `S`) — what "monthly" means.
    Monthly,
    Quarterly,
    /// End-of-month expiration (Schwab type `M`).
    EndOfMonth,
    Other,
}

impl ExpiryKind {
    pub(crate) fn from_api(t: Option<md::ExpirationType>) -> ExpiryKind {
        match t {
            Some(md::ExpirationType::W) => ExpiryKind::Weekly,
            Some(md::ExpirationType::S) => ExpiryKind::Monthly,
            Some(md::ExpirationType::Q) => ExpiryKind::Quarterly,
            Some(md::ExpirationType::M) => ExpiryKind::EndOfMonth,
            None => ExpiryKind::Other,
        }
    }
}

/// A single option contract at a point in time. `mark`/`delta`/etc. come from
/// the chain snapshot; live updates arrive later via the streamer (Part 2).
#[derive(Debug, Clone)]
pub struct OptionContract {
    /// OSI-format symbol, e.g. `AAPL  260619C00150000`.
    pub osi_symbol: String,
    pub strike: f64,
    pub expiration: NaiveDate,
    pub put_call: PutCall,
    pub expiry_kind: ExpiryKind,
    pub bid: f64,
    pub ask: f64,
    pub mark: f64,
    pub last: f64,
    pub delta: f64,
    pub theta: f64,
    /// Greeks and model values the TUI doesn't show yet (the MCP server
    /// does). `None` when Schwab omits the field or sends its -999 "not
    /// available" sentinel.
    pub gamma: Option<f64>,
    pub vega: Option<f64>,
    pub rho: Option<f64>,
    /// Schwab's theoretical option value.
    pub theo_value: Option<f64>,
    /// Implied volatility (the chain's `volatility` field), as a percentage.
    pub iv: f64,
    pub open_interest: i64,
    pub volume: i64,
    pub days_to_exp: i32,
    pub in_the_money: bool,
}

impl OptionContract {
    /// Convert one SDK contract. `expiration` is supplied by the caller because
    /// it's parsed once from the reliable `YYYY-MM-DD` map key rather than the
    /// contract's free-form `expirationDate` timestamp.
    fn from_api(c: &md::OptionContract, expiration: NaiveDate) -> Option<OptionContract> {
        let osi_symbol = c.symbol.clone()?;
        let put_call = match c.put_call? {
            md::OptionContractPutCall::Call => PutCall::Call,
            md::OptionContractPutCall::Put => PutCall::Put,
        };
        Some(OptionContract {
            osi_symbol,
            strike: c.strike_price.unwrap_or(0.0),
            expiration,
            put_call,
            expiry_kind: ExpiryKind::from_api(c.expiration_type),
            bid: c.bid_price.unwrap_or(0.0),
            ask: c.ask_price.unwrap_or(0.0),
            mark: c.mark_price.unwrap_or(0.0),
            last: c.last_price.unwrap_or(0.0),
            delta: c.delta.unwrap_or(0.0),
            theta: c.theta.unwrap_or(0.0),
            gamma: available(c.gamma),
            vega: available(c.vega),
            rho: available(c.rho),
            theo_value: available(c.theoretical_option_value),
            iv: c.volatility.unwrap_or(0.0),
            open_interest: c.open_interest.unwrap_or(0.0) as i64,
            volume: c.total_volume.unwrap_or(0) as i64,
            days_to_exp: c.days_to_expiration.unwrap_or(0.0) as i32,
            in_the_money: c.is_in_the_money.unwrap_or(false),
        })
    }
}

/// Schwab sends -999 (and occasionally NaN) for greeks it can't compute.
fn available(v: Option<f64>) -> Option<f64> {
    v.filter(|x| x.is_finite() && *x > -999.0 + 1e-9)
}

/// Calls and puts for one expiration date, each sorted by strike ascending.
#[derive(Debug, Clone, Default)]
pub struct ExpirationSlice {
    pub calls: Vec<OptionContract>,
    pub puts: Vec<OptionContract>,
}

impl ExpirationSlice {
    /// The expiration's classification, taken from any contract in the slice
    /// (all contracts of one expiration share it).
    pub fn kind(&self) -> ExpiryKind {
        self.calls
            .first()
            .or_else(|| self.puts.first())
            .map(|c| c.expiry_kind)
            .unwrap_or(ExpiryKind::Other)
    }
}

/// A flattened option chain for one underlying.
#[derive(Debug, Clone)]
pub struct OptionChainSnapshot {
    pub underlying: String,
    pub underlying_price: f64,
    /// Expirations in chronological order.
    pub expirations: BTreeMap<NaiveDate, ExpirationSlice>,
}

impl OptionChainSnapshot {
    pub fn from_api(underlying: &str, chain: md::OptionChain) -> OptionChainSnapshot {
        let mut expirations: BTreeMap<NaiveDate, ExpirationSlice> = BTreeMap::new();

        ingest_map(&chain.call_exp_date_map, PutCall::Call, &mut expirations);
        ingest_map(&chain.put_exp_date_map, PutCall::Put, &mut expirations);

        for slice in expirations.values_mut() {
            slice.calls.sort_by(|a, b| a.strike.total_cmp(&b.strike));
            slice.puts.sort_by(|a, b| a.strike.total_cmp(&b.strike));
        }

        OptionChainSnapshot {
            underlying: underlying.to_string(),
            underlying_price: chain.underlying_price.unwrap_or(0.0),
            expirations,
        }
    }

    /// Total contract count across all expirations (handy for tests/diagnostics).
    pub fn contract_count(&self) -> usize {
        self.expirations
            .values()
            .map(|s| s.calls.len() + s.puts.len())
            .sum()
    }
}

// ── Expirations and chain queries ────────────────────────────────────────────

/// One listed expiration from `GET /expirationchain`.
#[derive(Debug, Clone, PartialEq)]
pub struct ExpirationInfo {
    pub date: NaiveDate,
    pub days_to_exp: i32,
    pub kind: ExpiryKind,
    /// False for non-standard (adjusted-deliverable) roots.
    pub standard: bool,
}

impl ExpirationInfo {
    pub fn from_api(e: &md::Expiration) -> Option<ExpirationInfo> {
        let date = e
            .expiration
            .as_deref()
            .and_then(|d| NaiveDate::parse_from_str(d.get(..10)?, "%Y-%m-%d").ok())?;
        Some(ExpirationInfo {
            date,
            days_to_exp: e.days_to_expiration.unwrap_or(0),
            kind: ExpiryKind::from_api(e.expiration_type),
            standard: e.standard.unwrap_or(true),
        })
    }
}

/// Server-side filters for `GET /chains`, so a chain request doesn't pull
/// every strike of every expiration.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ChainQuery {
    pub symbol: String,
    /// `None` for both calls and puts.
    pub side: Option<PutCall>,
    /// Strikes above and below the at-the-money strike (Schwab's
    /// `strikeCount`).
    pub strike_count: Option<u32>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

// ── Contract specs (typed commands / chain filters) ──────────────────────────

/// Which expiration an option command refers to. Relative forms (everything
/// but explicit dates) resolve against the underlying's chain via
/// [`resolve_expiry`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExpirySpec {
    /// `7/17` — the next occurrence of that month/day.
    MonthDay { month: u32, day: u32 },
    /// `7/17/27` or `2027-07-17` — an explicit date.
    Date(NaiveDate),
    /// `lotto` — the nearest expiration in the chain.
    Lotto,
    /// `weekly` — the nearest weekly expiration.
    Weekly,
    /// `monthly` — the nearest standard (3rd-Friday) monthly expiration.
    Monthly,
    /// `next week` — the last expiration within next calendar week.
    NextWeek,
    /// `july monthly` — the standard monthly expiration of that month (1-12).
    MonthMonthly(u32),
}

/// A single option contract in a typed command: `215c 7/17` after the symbol.
#[derive(Debug, Clone, PartialEq)]
pub struct ContractSpec {
    pub strike: f64,
    pub side: PutCall,
    pub expiry: ExpirySpec,
}

/// Resolve an expiration spec to a concrete date. Explicit dates resolve on
/// their own; relative forms search the chain (`Err` with a human-readable
/// reason when the chain isn't loaded yet or has no matching expiration).
pub fn resolve_expiry(
    spec: ExpirySpec,
    chain: Option<&OptionChainSnapshot>,
    today: NaiveDate,
) -> Result<NaiveDate, String> {
    use chrono::Datelike;

    match spec {
        ExpirySpec::Date(d) => {
            if d < today {
                return Err(format!("{} is in the past", d.format("%-m/%-d/%y")));
            }
            Ok(d)
        }
        ExpirySpec::MonthDay { month, day } => {
            // Next occurrence: this year if not already past, otherwise next.
            for year in [today.year(), today.year() + 1] {
                if let Some(d) = NaiveDate::from_ymd_opt(year, month, day)
                    && d >= today
                {
                    return Ok(d);
                }
            }
            Err(format!("no upcoming {month}/{day}"))
        }
        relative => {
            let chain = chain.ok_or("loading option chain...")?;
            let mut future = chain.expirations.range(today..);
            match relative {
                ExpirySpec::Lotto => future
                    .next()
                    .map(|(d, _)| *d)
                    .ok_or_else(|| "no expirations in chain".to_string()),
                ExpirySpec::Weekly => future
                    .find(|(_, s)| s.kind() == ExpiryKind::Weekly)
                    .map(|(d, _)| *d)
                    .ok_or_else(|| "no weekly expiration in chain".to_string()),
                ExpirySpec::Monthly => future
                    .find(|(_, s)| s.kind() == ExpiryKind::Monthly)
                    .map(|(d, _)| *d)
                    .ok_or_else(|| "no monthly expiration in chain".to_string()),
                ExpirySpec::NextWeek => {
                    // Next calendar week (Mon..=Sun); take its last expiration
                    // (typically the Friday weekly).
                    let days_to_next_monday = 7 - today.weekday().num_days_from_monday() as i64 % 7;
                    let monday = today + chrono::Duration::days(days_to_next_monday);
                    let sunday = monday + chrono::Duration::days(6);
                    chain
                        .expirations
                        .range(monday..=sunday)
                        .next_back()
                        .map(|(d, _)| *d)
                        .ok_or_else(|| "no expiration next week".to_string())
                }
                ExpirySpec::MonthMonthly(month) => future
                    .find(|(d, s)| d.month() == month && s.kind() == ExpiryKind::Monthly)
                    .map(|(d, _)| *d)
                    .ok_or_else(|| format!("no monthly expiration for month {month} in chain")),
                // Date/MonthDay handled above.
                _ => unreachable!(),
            }
        }
    }
}

/// Look up a contract in the chain by expiration date, side, and strike.
pub fn find_contract(
    chain: &OptionChainSnapshot,
    expiration: NaiveDate,
    side: PutCall,
    strike: f64,
) -> Option<&OptionContract> {
    let slice = chain.expirations.get(&expiration)?;
    let contracts = match side {
        PutCall::Call => &slice.calls,
        PutCall::Put => &slice.puts,
    };
    contracts.iter().find(|c| (c.strike - strike).abs() < 1e-9)
}

// ── OSI symbols ───────────────────────────────────────────────────────────────

/// A parsed OSI option symbol. The wire format is fixed-width 21 chars:
/// 6-char space-padded root, `YYMMDD`, `C`/`P`, strike ×1000 in 8 digits
/// (e.g. `AAPL  260717C00215000`).
#[derive(Debug, Clone, PartialEq)]
pub struct OsiSymbol {
    pub underlying: String,
    pub expiration: NaiveDate,
    pub put_call: PutCall,
    pub strike: f64,
}

impl OsiSymbol {
    /// Parse an OSI-format symbol. Returns `None` for anything that isn't a
    /// well-formed 21-char OSI string (equity tickers, indexes, etc.), which
    /// makes this double as the "is this an option symbol?" test.
    pub fn parse(symbol: &str) -> Option<OsiSymbol> {
        let b = symbol.as_bytes();
        if b.len() != 21 {
            return None;
        }
        let root = symbol[..6].trim_end();
        if root.is_empty() || !root.bytes().all(|c| c.is_ascii_alphanumeric()) {
            return None;
        }
        if !b[6..12].iter().all(u8::is_ascii_digit) || !b[13..21].iter().all(u8::is_ascii_digit) {
            return None;
        }
        let put_call = match b[12] {
            b'C' => PutCall::Call,
            b'P' => PutCall::Put,
            _ => return None,
        };
        let yy: i32 = symbol[6..8].parse().ok()?;
        let mm: u32 = symbol[8..10].parse().ok()?;
        let dd: u32 = symbol[10..12].parse().ok()?;
        let expiration = NaiveDate::from_ymd_opt(2000 + yy, mm, dd)?;
        let strike = symbol[13..21].parse::<u64>().ok()? as f64 / 1000.0;
        Some(OsiSymbol {
            underlying: root.to_string(),
            expiration,
            put_call,
            strike,
        })
    }

    /// Render back to the fixed-width OSI wire format.
    pub fn to_osi_string(&self) -> String {
        format!(
            "{:<6}{}{}{:08}",
            self.underlying,
            self.expiration.format("%y%m%d"),
            match self.put_call {
                PutCall::Call => 'C',
                PutCall::Put => 'P',
            },
            (self.strike * 1000.0).round() as u64,
        )
    }

    /// Compact human form, e.g. `AAPL 7/17 215C`. The year is included
    /// (`7/17/27`) only when the expiration isn't in `today`'s year.
    pub fn display(&self, today: NaiveDate) -> String {
        use chrono::Datelike;
        let date = if self.expiration.year() == today.year() {
            self.expiration.format("%-m/%-d").to_string()
        } else {
            self.expiration.format("%-m/%-d/%y").to_string()
        };
        format!(
            "{} {} {}{}",
            self.underlying,
            date,
            format_strike(self.strike),
            match self.put_call {
                PutCall::Call => 'C',
                PutCall::Put => 'P',
            },
        )
    }
}

/// Strike without trailing zeros: 215 → "215", 215.5 → "215.5".
pub fn format_strike(strike: f64) -> String {
    let s = format!("{strike:.3}");
    s.trim_end_matches('0').trim_end_matches('.').to_string()
}

/// Display form for any symbol: compact contract form for OSI option symbols,
/// unchanged for everything else. All symbol rendering in the TUI goes
/// through here.
pub fn display_symbol(symbol: &str) -> String {
    match OsiSymbol::parse(symbol) {
        Some(osi) => osi.display(chrono::Local::now().date_naive()),
        None => symbol.to_string(),
    }
}

/// Parse the date portion of a chain map key. Keys look like `2026-06-19:25`
/// (expiration date + days-to-expiration); we only need the date.
fn parse_exp_key(key: &str) -> Option<NaiveDate> {
    let date_part = key.split(':').next()?;
    NaiveDate::parse_from_str(date_part, "%Y-%m-%d").ok()
}

/// Flatten one side (calls or puts) of the chain into the expirations map.
fn ingest_map(
    exp_map: &std::collections::HashMap<String, md::OptionContractMap>,
    side: PutCall,
    out: &mut BTreeMap<NaiveDate, ExpirationSlice>,
) {
    for (date_key, strike_map) in exp_map {
        let Some(expiration) = parse_exp_key(date_key) else {
            continue;
        };
        let slice = out.entry(expiration).or_default();
        for contracts in strike_map.values() {
            for c in contracts {
                if let Some(parsed) = OptionContract::from_api(c, expiration) {
                    match side {
                        PutCall::Call => slice.calls.push(parsed),
                        PutCall::Put => slice.puts.push(parsed),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn contract(symbol: &str, strike: f64, pc: md::OptionContractPutCall) -> md::OptionContract {
        md::OptionContract {
            symbol: Some(symbol.to_string()),
            strike_price: Some(strike),
            put_call: Some(pc),
            expiration_type: Some(md::ExpirationType::W),
            bid_price: Some(1.0),
            ask_price: Some(1.2),
            ..Default::default()
        }
    }

    /// One strike map keyed by strike string, each holding a single contract.
    fn strike_map(contracts: Vec<(&str, md::OptionContract)>) -> md::OptionContractMap {
        let mut m: HashMap<String, Vec<md::OptionContract>> = HashMap::new();
        for (strike, c) in contracts {
            m.insert(strike.to_string(), vec![c]);
        }
        md::OptionContractMap::from(m)
    }

    #[test]
    fn flattens_groups_and_sorts_by_strike() {
        use md::OptionContractPutCall::{Call, Put};

        // Two expirations of calls; strikes deliberately out of order to prove
        // the snapshot sorts them.
        let mut calls = HashMap::new();
        calls.insert(
            "2026-06-19:25".to_string(),
            strike_map(vec![
                ("160", contract("AAPL  260619C00160000", 160.0, Call)),
                ("150", contract("AAPL  260619C00150000", 150.0, Call)),
            ]),
        );
        calls.insert(
            "2026-07-17:53".to_string(),
            strike_map(vec![(
                "150",
                contract("AAPL  260717C00150000", 150.0, Call),
            )]),
        );

        let mut puts = HashMap::new();
        puts.insert(
            "2026-06-19:25".to_string(),
            strike_map(vec![("150", contract("AAPL  260619P00150000", 150.0, Put))]),
        );

        let chain = md::OptionChain {
            call_exp_date_map: calls,
            put_exp_date_map: puts,
            underlying_price: Some(155.0),
            ..Default::default()
        };

        let snap = OptionChainSnapshot::from_api("AAPL", chain);

        assert_eq!(snap.underlying_price, 155.0);
        assert_eq!(snap.expirations.len(), 2);
        assert_eq!(snap.contract_count(), 4);

        // First expiration (BTreeMap orders chronologically) has both sides.
        let (first_date, first) = snap.expirations.iter().next().unwrap();
        assert_eq!(first_date, &NaiveDate::from_ymd_opt(2026, 6, 19).unwrap());
        assert_eq!(first.calls.len(), 2);
        assert_eq!(first.puts.len(), 1);

        // Calls sorted ascending by strike.
        assert_eq!(first.calls[0].strike, 150.0);
        assert_eq!(first.calls[1].strike, 160.0);

        // Field mapping + expiry classification.
        let c = &first.calls[0];
        assert_eq!(c.put_call, PutCall::Call);
        assert_eq!(c.expiry_kind, ExpiryKind::Weekly);
        assert_eq!(c.bid, 1.0);
        assert_eq!(c.ask, 1.2);
    }

    #[test]
    fn keeps_gamma_vega_theo_and_drops_sentinels() {
        // Wire field names, as the live API sends them (SCHWAB_API_ISSUES #9).
        let c: md::OptionContract = serde_json::from_value(serde_json::json!({
            "symbol": "TEST  260619C00100000",
            "putCall": "CALL",
            "strikePrice": 100.0,
            "delta": 0.52,
            "gamma": 0.031,
            "vega": 0.12,
            "rho": -999.0,
            "theoreticalOptionValue": 2.35,
            "inTheMoney": true
        }))
        .unwrap();
        let exp = NaiveDate::from_ymd_opt(2026, 6, 19).unwrap();
        let parsed = OptionContract::from_api(&c, exp).unwrap();
        assert_eq!(parsed.gamma, Some(0.031));
        assert_eq!(parsed.vega, Some(0.12));
        assert_eq!(parsed.theo_value, Some(2.35));
        assert_eq!(parsed.rho, None, "-999 means not available");
        assert!(parsed.in_the_money);
    }

    #[test]
    fn skips_contracts_missing_required_fields() {
        // No symbol → dropped; no put_call → dropped.
        let mut calls = HashMap::new();
        calls.insert(
            "2026-06-19:25".to_string(),
            strike_map(vec![
                ("150", md::OptionContract::default()),
                (
                    "160",
                    contract(
                        "AAPL  260619C00160000",
                        160.0,
                        md::OptionContractPutCall::Call,
                    ),
                ),
            ]),
        );
        let chain = md::OptionChain {
            call_exp_date_map: calls,
            ..Default::default()
        };

        let snap = OptionChainSnapshot::from_api("AAPL", chain);
        assert_eq!(
            snap.contract_count(),
            1,
            "the field-less contract is skipped"
        );
    }

    /// Build a one-contract slice with the given kind for resolver tests.
    fn slice(kind: ExpiryKind, expiration: NaiveDate) -> ExpirationSlice {
        ExpirationSlice {
            calls: vec![OptionContract {
                osi_symbol: format!("TEST  {}C00100000", expiration.format("%y%m%d")),
                strike: 100.0,
                expiration,
                put_call: PutCall::Call,
                expiry_kind: kind,
                bid: 1.0,
                ask: 1.2,
                mark: 1.1,
                last: 1.1,
                delta: 0.5,
                theta: -0.02,
                gamma: None,
                vega: None,
                rho: None,
                theo_value: None,
                iv: 30.0,
                open_interest: 10,
                volume: 5,
                days_to_exp: 5,
                in_the_money: false,
            }],
            puts: vec![],
        }
    }

    fn test_chain() -> OptionChainSnapshot {
        let d = |y, m, dd| NaiveDate::from_ymd_opt(y, m, dd).unwrap();
        let mut expirations = BTreeMap::new();
        expirations.insert(d(2026, 7, 13), slice(ExpiryKind::Weekly, d(2026, 7, 13)));
        expirations.insert(d(2026, 7, 17), slice(ExpiryKind::Monthly, d(2026, 7, 17)));
        expirations.insert(d(2026, 7, 20), slice(ExpiryKind::Weekly, d(2026, 7, 20)));
        expirations.insert(d(2026, 8, 21), slice(ExpiryKind::Monthly, d(2026, 8, 21)));
        OptionChainSnapshot {
            underlying: "TEST".to_string(),
            underlying_price: 100.0,
            expirations,
        }
    }

    #[test]
    fn resolve_expiry_relative_forms() {
        let chain = test_chain();
        // Friday 2026-07-10; nearest expiration is Monday 7/13.
        let today = NaiveDate::from_ymd_opt(2026, 7, 10).unwrap();
        let d = |y, m, dd| NaiveDate::from_ymd_opt(y, m, dd).unwrap();

        let resolve = |spec| resolve_expiry(spec, Some(&chain), today);
        assert_eq!(resolve(ExpirySpec::Lotto), Ok(d(2026, 7, 13)));
        assert_eq!(resolve(ExpirySpec::Weekly), Ok(d(2026, 7, 13)));
        assert_eq!(resolve(ExpirySpec::Monthly), Ok(d(2026, 7, 17)));
        // Next calendar week is Mon 7/13 – Sun 7/19; its last expiration is 7/17.
        assert_eq!(resolve(ExpirySpec::NextWeek), Ok(d(2026, 7, 17)));
        assert_eq!(resolve(ExpirySpec::MonthMonthly(8)), Ok(d(2026, 8, 21)));
        assert_eq!(resolve(ExpirySpec::MonthMonthly(7)), Ok(d(2026, 7, 17)));
        assert!(resolve(ExpirySpec::MonthMonthly(9)).is_err());
    }

    #[test]
    fn resolve_expiry_dates_without_chain() {
        let today = NaiveDate::from_ymd_opt(2026, 7, 10).unwrap();
        let d = |y, m, dd| NaiveDate::from_ymd_opt(y, m, dd).unwrap();

        // Explicit dates never need the chain.
        assert_eq!(
            resolve_expiry(ExpirySpec::Date(d(2026, 7, 17)), None, today),
            Ok(d(2026, 7, 17))
        );
        assert!(resolve_expiry(ExpirySpec::Date(d(2026, 7, 9)), None, today).is_err());
        // Month/day picks the next occurrence, rolling into next year if past.
        assert_eq!(
            resolve_expiry(ExpirySpec::MonthDay { month: 7, day: 17 }, None, today),
            Ok(d(2026, 7, 17))
        );
        assert_eq!(
            resolve_expiry(ExpirySpec::MonthDay { month: 1, day: 15 }, None, today),
            Ok(d(2027, 1, 15))
        );
        // Relative forms report the chain as still loading.
        assert!(resolve_expiry(ExpirySpec::Lotto, None, today).is_err());
    }

    #[test]
    fn find_contract_matches_side_and_strike() {
        let chain = test_chain();
        let d = NaiveDate::from_ymd_opt(2026, 7, 17).unwrap();
        assert!(find_contract(&chain, d, PutCall::Call, 100.0).is_some());
        assert!(find_contract(&chain, d, PutCall::Put, 100.0).is_none());
        assert!(find_contract(&chain, d, PutCall::Call, 105.0).is_none());
    }

    #[test]
    fn osi_parse_and_roundtrip() {
        let osi = OsiSymbol::parse("AAPL  260717C00215000").expect("valid OSI");
        assert_eq!(osi.underlying, "AAPL");
        assert_eq!(
            osi.expiration,
            NaiveDate::from_ymd_opt(2026, 7, 17).unwrap()
        );
        assert_eq!(osi.put_call, PutCall::Call);
        assert_eq!(osi.strike, 215.0);
        assert_eq!(osi.to_osi_string(), "AAPL  260717C00215000");

        // Fractional strike and put side.
        let put = OsiSymbol::parse("F     261218P00012500").expect("valid OSI");
        assert_eq!(put.underlying, "F");
        assert_eq!(put.put_call, PutCall::Put);
        assert_eq!(put.strike, 12.5);
        assert_eq!(put.to_osi_string(), "F     261218P00012500");
    }

    #[test]
    fn osi_parse_rejects_non_options() {
        for s in [
            "AAPL",                   // plain ticker
            "AAPL  260717X00215000",  // bad side char
            "AAPL  26071C700215000",  // non-digit date
            "AAPL  260717C0021500",   // 20 chars
            "AAPL  260717C002150000", // 22 chars
            "      260717C00215000",  // empty root
            "$SPX  2607A7C00215000",  // non-alnum root + bad date
        ] {
            assert!(OsiSymbol::parse(s).is_none(), "should reject {s:?}");
        }
    }

    #[test]
    fn osi_display_compact() {
        let osi = OsiSymbol::parse("AAPL  260717C00215000").unwrap();
        let same_year = NaiveDate::from_ymd_opt(2026, 7, 10).unwrap();
        let prev_year = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
        assert_eq!(osi.display(same_year), "AAPL 7/17 215C");
        assert_eq!(osi.display(prev_year), "AAPL 7/17/26 215C");

        let put = OsiSymbol::parse("F     261218P00012500").unwrap();
        assert_eq!(put.display(same_year), "F 12/18 12.5P");
    }

    #[test]
    fn ignores_unparseable_date_keys() {
        let mut calls = HashMap::new();
        calls.insert(
            "not-a-date".to_string(),
            strike_map(vec![(
                "150",
                contract("X", 150.0, md::OptionContractPutCall::Call),
            )]),
        );
        let chain = md::OptionChain {
            call_exp_date_map: calls,
            ..Default::default()
        };

        let snap = OptionChainSnapshot::from_api("X", chain);
        assert!(snap.expirations.is_empty());
    }
}
