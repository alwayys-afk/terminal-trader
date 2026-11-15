//! Narrowing an option chain to what fits in a tool result.
//!
//! The filters go to Schwab where the API supports them (side, strike count,
//! date range) and are applied again here, together with the ones it doesn't
//! (DTE range, explicit strike range), before a hard row cap.

use std::fmt::Write as _;

use chrono::NaiveDate;

use super::format::{kind_code, num, opt, side_code};
use crate::schwab::{ChainQuery, OptionChainSnapshot, OptionContract, PutCall};

/// Default strikes on each side of the money when no strike range is given.
pub const DEFAULT_STRIKE_COUNT: u32 = 8;
/// Default DTE window when no expiration filter is given.
pub const DEFAULT_MAX_DTE: i64 = 45;
pub const DEFAULT_MAX_ROWS: usize = 120;
pub const HARD_MAX_ROWS: usize = 400;

/// Validated chain filters.
#[derive(Debug, Clone, PartialEq)]
pub struct ChainFilter {
    pub symbol: String,
    pub side: Option<PutCall>,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    /// Strikes above and below the money. `None` when a strike range is set.
    pub strike_count: Option<u32>,
    pub min_strike: Option<f64>,
    pub max_strike: Option<f64>,
    pub max_rows: usize,
}

/// Raw filter inputs, as a tool receives them.
#[derive(Debug, Clone, Default)]
pub struct ChainFilterInput<'a> {
    pub symbol: &'a str,
    pub side: Option<&'a str>,
    pub expiration: Option<&'a str>,
    pub from_date: Option<&'a str>,
    pub to_date: Option<&'a str>,
    pub min_dte: Option<u32>,
    pub max_dte: Option<u32>,
    pub strike_count: Option<u32>,
    pub min_strike: Option<f64>,
    pub max_strike: Option<f64>,
    pub max_rows: Option<u32>,
}

fn parse_date(s: &str, field: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d")
        .map_err(|_| format!("{field} must be YYYY-MM-DD, got {s:?}"))
}

impl ChainFilter {
    pub fn from_input(input: &ChainFilterInput, today: NaiveDate) -> Result<ChainFilter, String> {
        let symbol = input.symbol.trim().to_uppercase();
        if symbol.is_empty() {
            return Err("symbol is required".to_string());
        }
        let side = match input.side.map(|s| s.trim().to_ascii_lowercase()) {
            None => None,
            Some(s) if s.is_empty() || s == "all" || s == "both" => None,
            Some(s) if s.starts_with('c') => Some(PutCall::Call),
            Some(s) if s.starts_with('p') => Some(PutCall::Put),
            Some(s) => return Err(format!("side must be call, put or all, got {s:?}")),
        };

        let days = |n: u32| today + chrono::Duration::days(i64::from(n));
        let (from_date, to_date) = if let Some(exp) = input.expiration {
            let d = parse_date(exp, "expiration")?;
            (d, d)
        } else {
            let any_date_filter = input.from_date.is_some()
                || input.to_date.is_some()
                || input.min_dte.is_some()
                || input.max_dte.is_some();
            let mut from = today;
            if let Some(n) = input.min_dte {
                from = from.max(days(n));
            }
            if let Some(s) = input.from_date {
                from = from.max(parse_date(s, "from_date")?);
            }
            let mut to: Option<NaiveDate> = None;
            if let Some(n) = input.max_dte {
                to = Some(days(n));
            }
            if let Some(s) = input.to_date {
                let d = parse_date(s, "to_date")?;
                to = Some(to.map_or(d, |t| t.min(d)));
            }
            let to = match to {
                Some(t) => t,
                None if any_date_filter => from + chrono::Duration::days(DEFAULT_MAX_DTE),
                None => today + chrono::Duration::days(DEFAULT_MAX_DTE),
            };
            (from, to)
        };
        if to_date < from_date {
            return Err(format!(
                "the expiration range is empty ({from_date} to {to_date})"
            ));
        }

        if let (Some(lo), Some(hi)) = (input.min_strike, input.max_strike)
            && hi < lo
        {
            return Err("max_strike is below min_strike".to_string());
        }
        let has_range = input.min_strike.is_some() || input.max_strike.is_some();
        let strike_count = match input.strike_count {
            Some(0) => None,
            Some(n) => Some(n.min(50)),
            None if has_range => None,
            None => Some(DEFAULT_STRIKE_COUNT),
        };
        let max_rows = input
            .max_rows
            .map(|n| (n as usize).clamp(1, HARD_MAX_ROWS))
            .unwrap_or(DEFAULT_MAX_ROWS);

        Ok(ChainFilter {
            symbol,
            side,
            from_date,
            to_date,
            strike_count,
            min_strike: input.min_strike,
            max_strike: input.max_strike,
            max_rows,
        })
    }

    /// The request to send to Schwab.
    pub fn query(&self) -> ChainQuery {
        ChainQuery {
            symbol: self.symbol.clone(),
            side: self.side,
            strike_count: self.strike_count,
            from_date: Some(self.from_date),
            to_date: Some(self.to_date),
        }
    }

    fn keeps(&self, c: &OptionContract) -> bool {
        c.expiration >= self.from_date
            && c.expiration <= self.to_date
            && self.side.is_none_or(|s| s == c.put_call)
            && self.min_strike.is_none_or(|lo| c.strike >= lo - 1e-9)
            && self.max_strike.is_none_or(|hi| c.strike <= hi + 1e-9)
    }
}

/// The contracts that pass `filter`, in expiration then strike order (call
/// before put at the same strike), capped at `filter.max_rows`. Returns the
/// rows and how many matching rows were cut.
pub fn select<'a>(
    snap: &'a OptionChainSnapshot,
    filter: &ChainFilter,
) -> (Vec<&'a OptionContract>, usize) {
    let mut rows: Vec<&OptionContract> = Vec::new();
    for slice in snap.expirations.values() {
        let mut both: Vec<&OptionContract> = slice
            .calls
            .iter()
            .chain(slice.puts.iter())
            .filter(|c| filter.keeps(c))
            .collect();
        both.sort_by(|a, b| {
            a.strike
                .total_cmp(&b.strike)
                .then((a.put_call == PutCall::Put).cmp(&(b.put_call == PutCall::Put)))
        });
        rows.extend(both);
    }
    let cut = rows.len().saturating_sub(filter.max_rows);
    rows.truncate(filter.max_rows);
    (rows, cut)
}

pub fn render(snap: &OptionChainSnapshot, filter: &ChainFilter) -> String {
    let (rows, cut) = select(snap, filter);
    let mut out = format!(
        "# {} chain, underlying {}, expirations {}..{}, {} rows\n",
        filter.symbol,
        num(snap.underlying_price, 2),
        filter.from_date,
        filter.to_date,
        rows.len()
    );
    if cut > 0 {
        let _ = writeln!(
            out,
            "TRUNCATED: {cut} more rows matched. Narrow the expiration range or strikes, or raise max_rows (max {HARD_MAX_ROWS})."
        );
    }
    if rows.is_empty() {
        out.push_str("no contracts matched\n");
        return out;
    }
    out.push_str(
        "exp,dte,kind,side,strike,bid,ask,mark,last,vol,oi,iv,delta,gamma,theta,vega,theo,itm\n",
    );
    for c in rows {
        let cells = [
            c.expiration.to_string(),
            c.days_to_exp.to_string(),
            kind_code(c.expiry_kind).to_string(),
            side_code(c.put_call).to_string(),
            num(c.strike, 3),
            num(c.bid, 2),
            num(c.ask, 2),
            num(c.mark, 3),
            num(c.last, 2),
            c.volume.to_string(),
            c.open_interest.to_string(),
            num(c.iv, 2),
            num(c.delta, 3),
            opt(c.gamma, 4),
            num(c.theta, 3),
            opt(c.vega, 3),
            opt(c.theo_value, 3),
            if c.in_the_money { "1" } else { "" }.to_string(),
        ];
        out.push_str(&cells.join(","));
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schwab::{ExpirationSlice, ExpiryKind};
    use std::collections::BTreeMap;

    fn d(y: i32, m: u32, dd: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, dd).unwrap()
    }

    fn contract(exp: NaiveDate, side: PutCall, strike: f64) -> OptionContract {
        OptionContract {
            osi_symbol: String::new(),
            strike,
            expiration: exp,
            put_call: side,
            expiry_kind: ExpiryKind::Weekly,
            bid: 1.0,
            ask: 1.1,
            mark: 1.05,
            last: 1.0,
            delta: 0.5,
            theta: -0.05,
            gamma: Some(0.04),
            vega: Some(0.1),
            rho: None,
            theo_value: Some(1.04),
            iv: 25.0,
            open_interest: 100,
            volume: 10,
            days_to_exp: 3,
            in_the_money: false,
        }
    }

    fn snapshot() -> OptionChainSnapshot {
        let mut expirations = BTreeMap::new();
        for exp in [d(2026, 11, 20), d(2026, 12, 18), d(2027, 3, 19)] {
            let strikes = [90.0, 95.0, 100.0, 105.0, 110.0];
            expirations.insert(
                exp,
                ExpirationSlice {
                    calls: strikes
                        .iter()
                        .map(|&k| contract(exp, PutCall::Call, k))
                        .collect(),
                    puts: strikes
                        .iter()
                        .map(|&k| contract(exp, PutCall::Put, k))
                        .collect(),
                },
            );
        }
        OptionChainSnapshot {
            underlying: "TEST".to_string(),
            underlying_price: 100.0,
            expirations,
        }
    }

    const TODAY: (i32, u32, u32) = (2026, 11, 15);

    fn filter(input: ChainFilterInput) -> ChainFilter {
        ChainFilter::from_input(&input, d(TODAY.0, TODAY.1, TODAY.2)).unwrap()
    }

    #[test]
    fn defaults_are_near_term_and_near_the_money() {
        let f = filter(ChainFilterInput {
            symbol: "test",
            ..Default::default()
        });
        assert_eq!(f.symbol, "TEST");
        assert_eq!(f.from_date, d(2026, 11, 15));
        assert_eq!(f.to_date, d(2026, 12, 30));
        assert_eq!(f.strike_count, Some(DEFAULT_STRIKE_COUNT));
        assert_eq!(f.side, None);
        let q = f.query();
        assert_eq!(q.strike_count, Some(DEFAULT_STRIKE_COUNT));
        assert_eq!(q.to_date, Some(d(2026, 12, 30)));

        // The March expiration is outside the default 45-day window.
        let snap = snapshot();
        let (rows, cut) = select(&snap, &f);
        assert_eq!(rows.len(), 20);
        assert_eq!(cut, 0);
    }

    #[test]
    fn side_strike_range_and_single_expiration() {
        let f = filter(ChainFilterInput {
            symbol: "TEST",
            side: Some("put"),
            expiration: Some("2026-12-18"),
            min_strike: Some(95.0),
            max_strike: Some(105.0),
            ..Default::default()
        });
        assert_eq!(f.strike_count, None, "a strike range replaces the count");
        let snap = snapshot();
        let (rows, _) = select(&snap, &f);
        let strikes: Vec<f64> = rows.iter().map(|c| c.strike).collect();
        assert_eq!(strikes, [95.0, 100.0, 105.0]);
        assert!(rows.iter().all(|c| c.put_call == PutCall::Put));
        assert!(rows.iter().all(|c| c.expiration == d(2026, 12, 18)));
    }

    #[test]
    fn dte_range_and_row_cap() {
        let f = filter(ChainFilterInput {
            symbol: "TEST",
            min_dte: Some(10),
            max_dte: Some(200),
            max_rows: Some(7),
            ..Default::default()
        });
        assert_eq!(f.from_date, d(2026, 11, 25));
        let snap = snapshot();
        let (rows, cut) = select(&snap, &f);
        assert_eq!(rows.len(), 7);
        // Dec and Mar expirations match: 2 x 10 contracts.
        assert_eq!(cut, 13);
        assert!(rows.iter().all(|c| c.expiration == d(2026, 12, 18)));
        // Calls before puts at a strike.
        assert_eq!(rows[0].put_call, PutCall::Call);
        assert_eq!(rows[1].put_call, PutCall::Put);
        assert_eq!(rows[0].strike, rows[1].strike);

        let text = render(&snapshot(), &f);
        assert!(text.contains("TRUNCATED: 13 more rows"));
        assert!(
            text.contains("2026-12-18,3,W,C,90,1,1.1,1.05,1,10,100,25,0.5,0.04,-0.05,0.1,1.04,")
        );
    }

    #[test]
    fn bad_input_is_rejected() {
        let today = d(TODAY.0, TODAY.1, TODAY.2);
        let bad = |input: ChainFilterInput| ChainFilter::from_input(&input, today).unwrap_err();
        assert!(bad(ChainFilterInput::default()).contains("symbol"));
        assert!(
            bad(ChainFilterInput {
                symbol: "X",
                side: Some("straddle"),
                ..Default::default()
            })
            .contains("side")
        );
        assert!(
            bad(ChainFilterInput {
                symbol: "X",
                expiration: Some("12/18"),
                ..Default::default()
            })
            .contains("YYYY-MM-DD")
        );
        assert!(
            bad(ChainFilterInput {
                symbol: "X",
                min_dte: Some(30),
                max_dte: Some(10),
                ..Default::default()
            })
            .contains("empty")
        );
    }
}
