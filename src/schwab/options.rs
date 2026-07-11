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

/// How an expiration is classified by the OCC (drives monthly/weekly filters).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpiryKind {
    Weekly,
    Monthly,
    Quarterly,
    /// Anything else the exchange reports (e.g. EOM/standard "S").
    Other,
}

impl ExpiryKind {
    fn from_api(t: Option<md::ExpirationType>) -> ExpiryKind {
        match t {
            Some(md::ExpirationType::W) => ExpiryKind::Weekly,
            Some(md::ExpirationType::M) => ExpiryKind::Monthly,
            Some(md::ExpirationType::Q) => ExpiryKind::Quarterly,
            Some(md::ExpirationType::S) | None => ExpiryKind::Other,
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
            iv: c.volatility.unwrap_or(0.0),
            open_interest: c.open_interest.unwrap_or(0.0) as i64,
            volume: c.total_volume.unwrap_or(0) as i64,
            days_to_exp: c.days_to_expiration.unwrap_or(0.0) as i32,
            in_the_money: c.is_in_the_money.unwrap_or(false),
        })
    }
}

/// Calls and puts for one expiration date, each sorted by strike ascending.
#[derive(Debug, Clone, Default)]
pub struct ExpirationSlice {
    pub calls: Vec<OptionContract>,
    pub puts: Vec<OptionContract>,
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
