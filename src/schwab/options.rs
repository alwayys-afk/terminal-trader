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
