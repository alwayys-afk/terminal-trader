use std::collections::HashMap;
use std::io::{BufRead, Write};

use anyhow::{Context, Result};
use chrono::{Datelike, NaiveDate};

use super::lots::{AppliedTransaction, Operation, OperationKind, TaxLotStore, TradeAction};
use crate::app::lot_store_path_for;
use crate::schwab::{LotSelectionMethod, QTY_EPSILON, SchwabAuth, SchwabClient};

/// Authenticate, list the user's accounts, prompt for one, and return its
/// canonical lot-store path. Auto-picks when there's exactly one account.
pub async fn pick_seed_lot_store_path() -> Result<String> {
    let api_key = std::env::var("SCHWAB_API_KEY").context("SCHWAB_API_KEY not set")?;
    let app_secret = std::env::var("SCHWAB_APP_SECRET").context("SCHWAB_APP_SECRET not set")?;
    let token_path =
        std::env::var("SCHWAB_TOKEN_PATH").unwrap_or_else(|_| "schwab_tokens.json".to_string());

    let auth = SchwabAuth::new(api_key, app_secret, token_path);
    let client = SchwabClient::new(auth)
        .await
        .context("authentication failed")?;
    let accounts = client
        .get_accounts()
        .await
        .context("failed to fetch accounts")?;

    let acct = match accounts.as_slice() {
        [] => anyhow::bail!("no accounts found on this login"),
        [only] => {
            println!("Seeding into {}", only.display_label());
            only
        }
        many => {
            println!("\nWhich account should this CSV seed into?");
            for (i, a) in many.iter().enumerate() {
                println!("  {}) {}", i + 1, a.display_label());
            }
            print!("> ");
            std::io::stdout().flush().ok();
            let mut line = String::new();
            std::io::stdin().lock().read_line(&mut line)?;
            let n: usize = line.trim().parse().context("expected a number")?;
            let idx = n.checked_sub(1).context("selection must be >= 1")?;
            many.get(idx).context("selection out of range")?
        }
    };
    Ok(lot_store_path_for(&acct.hash))
}

/// Seed the tax lot store from a Schwab CSV transaction export.
///
/// Parses Buy/Sell rows into `AppliedTransaction` entries and
/// Name Change / Stock Merger pairs into `Operation` entries.
/// Deduplicates against existing data, then rebuilds lots from scratch.
pub fn seed_from_csv(
    csv_path: &str,
    lot_store_path: &str,
    lot_method: Option<LotSelectionMethod>,
) -> Result<()> {
    let mut store = TaxLotStore::load(lot_store_path);

    // An explicit --lot-method stamps each seeded *sell* with that method,
    // marking it as a known/conscious choice (buys carry none), and also adopts
    // it as the account default so unflagged/future sells follow suit. Without
    // the flag, seeded sells stay blank and follow the existing account default
    // — adjustable on the operations page. See issues #74, #78.
    match lot_method {
        Some(method) => {
            store.default_lot_method = method;
            println!(
                "lot method: stamping seeded sells as {method} and setting account default (from --lot-method)"
            );
        }
        None => println!(
            "lot method: seeded sells left blank (account default {})",
            store.default_lot_method
        ),
    }

    // Parse CSV rows.
    let rows = parse_csv(csv_path)?;
    println!("parsed {} CSV rows", rows.len());

    // Wipe the slate clean on re-seed — the CSV is the source of truth for
    // pre-API data and the tuple-based dedup is unreliable (identical fills on
    // the same day for the same symbol/qty/price).
    store.applied_transactions.clear();
    store.applied_activity_ids.clear();
    store.operations.clear();
    store.lots.clear();
    store.sales.clear();

    // Build description prefix -> symbol map from Buy/Sell rows for corporate action resolution.
    let mut desc_to_symbol: HashMap<String, String> = HashMap::new();
    // Also map first word of description -> symbol for fuzzy matching.
    let mut first_word_to_symbol: HashMap<String, String> = HashMap::new();
    for row in &rows {
        if matches!(row.action, Action::Buy | Action::Sell) {
            if let Some(prefix) = description_prefix(&row.description) {
                desc_to_symbol.entry(prefix).or_insert(row.symbol.clone());
            }
            if let Some(word) = row.description.split_whitespace().next() {
                first_word_to_symbol
                    .entry(word.to_lowercase())
                    .or_insert(row.symbol.clone());
            }
        }
    }

    // Synthetic activity IDs: start below any existing synthetic IDs.
    let mut next_synthetic_id: i64 = store
        .applied_activity_ids
        .iter()
        .copied()
        .filter(|&id| id < 0)
        .min()
        .unwrap_or(0)
        - 1;

    let mut added_txns = 0;
    let mut added_ops = 0;
    let mut skipped_other = 0;

    // Build CUSIP→ticker map: if a symbol looks like a CUSIP (starts with digit),
    // map it to the ticker from the paired row. Also map tickers seen in Buy/Sell rows.
    // This is populated in a first pass over corporate action rows.
    let mut cusip_to_ticker: HashMap<String, String> = HashMap::new();

    // Group corporate action rows by date for pairing.
    let mut corp_actions: HashMap<NaiveDate, Vec<&CsvRow>> = HashMap::new();

    for row in &rows {
        match row.action {
            Action::Buy | Action::Sell => {
                let trade_action = match row.action {
                    Action::Buy => TradeAction::Buy,
                    Action::Sell => TradeAction::Sell,
                    _ => unreachable!(),
                };

                let id = next_synthetic_id;
                next_synthetic_id -= 1;

                // Stamp the chosen method onto sells only; buys carry none.
                let stamped = if matches!(trade_action, TradeAction::Sell) {
                    lot_method
                } else {
                    None
                };

                store.applied_transactions.push(AppliedTransaction {
                    date: row.date,
                    symbol: row.symbol.clone(),
                    action: trade_action,
                    quantity: row.quantity,
                    price: row.price,
                    net_amount: row.amount,
                    activity_id: id,
                    lot_method: stamped,
                    order_id: None,
                    trade_datetime: None,
                });
                store.applied_activity_ids.insert(id);
                added_txns += 1;
            }
            Action::NameChange | Action::StockMerger | Action::ReverseSplit => {
                corp_actions.entry(row.date).or_default().push(row);
            }
            Action::Other => {
                skipped_other += 1;
            }
        }
    }

    // Pair corporate action rows into operations.
    for (date, group) in &corp_actions {
        let mut closing: Option<&CsvRow> = None;
        let mut opening: Option<&CsvRow> = None;

        for row in group {
            if row.quantity < 0.0 {
                closing = Some(row);
            } else if row.quantity > 0.0 {
                opening = Some(row);
            }
        }

        if let (Some(close), Some(open)) = (closing, opening) {
            // Resolve old_symbol from the closing row's description.
            let old_symbol = resolve_old_symbol(
                close,
                &desc_to_symbol,
                &first_word_to_symbol,
                &store,
                &cusip_to_ticker,
            );
            let mut new_symbol = resolve_new_symbol(open, &cusip_to_ticker);

            // For reverse splits the symbol doesn't change,
            // so use the resolved old_symbol if the new one is still a CUSIP.
            if matches!(close.action, Action::ReverseSplit) && is_cusip(&new_symbol) {
                new_symbol = old_symbol.clone();
            }

            // If new_symbol is still a CUSIP, try description-based matching.
            if is_cusip(&new_symbol) {
                if let Some(prefix) = description_prefix(&open.description)
                    && let Some(symbol) = desc_to_symbol.get(&prefix)
                {
                    new_symbol = symbol.clone();
                }
                if is_cusip(&new_symbol)
                    && let Some(word) = open.description.split_whitespace().next()
                    && let Some(symbol) = first_word_to_symbol.get(&word.to_lowercase())
                {
                    new_symbol = symbol.clone();
                }
            }

            // Record CUSIP→ticker mappings for chaining.
            if is_cusip(&close.symbol) && !is_cusip(&old_symbol) {
                cusip_to_ticker.insert(close.symbol.clone(), old_symbol.clone());
            }
            if is_cusip(&open.symbol) && !is_cusip(&new_symbol) {
                cusip_to_ticker.insert(open.symbol.clone(), new_symbol.clone());
            }
            let old_qty = close.quantity.abs();
            let new_qty = open.quantity.abs();

            let id1 = next_synthetic_id;
            next_synthetic_id -= 1;
            let id2 = next_synthetic_id;
            next_synthetic_id -= 1;

            let source_ids = vec![id1, id2];

            // Symbol changed → NameChange op.
            if old_symbol != new_symbol {
                println!(
                    "  name change: {} -> {} on {}",
                    old_symbol, new_symbol, date
                );
                store.operations.push(Operation {
                    date: *date,
                    kind: OperationKind::NameChange {
                        old_symbol: old_symbol.clone(),
                        new_symbol: new_symbol.clone(),
                    },
                    description: close.description.clone(),
                    source_activity_ids: source_ids.clone(),
                    confirmed: false,
                });
                added_ops += 1;
            }

            // Quantity changed → Multiplier op (on the new symbol).
            if (old_qty - new_qty).abs() > 0.01 {
                let symbol = if old_symbol != new_symbol {
                    &new_symbol
                } else {
                    &old_symbol
                };
                println!(
                    "  multiplier: {} {:.0} -> {:.0} on {}",
                    symbol, old_qty, new_qty, date
                );
                store.operations.push(Operation {
                    date: *date,
                    kind: OperationKind::Multiplier {
                        symbol: symbol.clone(),
                        old_qty,
                        new_qty,
                    },
                    description: close.description.clone(),
                    source_activity_ids: source_ids.clone(),
                    confirmed: false,
                });
                added_ops += 1;
            }

            store.applied_activity_ids.insert(id1);
            store.applied_activity_ids.insert(id2);
        } else {
            eprintln!(
                "  warning: unpaired corporate action on {} — skipping",
                date
            );
        }
    }

    // Sort by date only — preserve the CSV's original intra-day order
    // (which reflects actual execution sequence).
    store.applied_transactions.sort_by_key(|t| t.date);
    store.operations.sort_by_key(|o| o.date);

    // Find the last Saturday in the data — this is the clean boundary between
    // CSV-seeded data and API-fetched data (no transactions on Saturday).
    let last_date = store
        .applied_transactions
        .last()
        .map(|t| t.date)
        .unwrap_or_else(|| chrono::Local::now().date_naive());
    let days_since_sat = (last_date.weekday().num_days_from_sunday() + 1) % 7;
    let last_saturday = last_date - chrono::Days::new(days_since_sat as u64);

    // Drop any transactions after the Saturday cutoff so the API owns everything after.
    let before_cutoff = store.applied_transactions.len();
    store
        .applied_transactions
        .retain(|t| t.date <= last_saturday);
    let dropped = before_cutoff - store.applied_transactions.len();
    if dropped > 0 {
        println!(
            "dropped {} transactions after cutoff {}",
            dropped, last_saturday
        );
    }

    // Set last_sync_date to Saturday at noon — the API will fetch from here onward.
    store.last_sync_date = Some(format!("{}T12:00:00", last_saturday));

    // Rebuild lots from the full transaction + operation history.
    store.rebuild();

    // Save.
    store
        .save(lot_store_path)
        .context("failed to save tax lot store")?;

    println!();
    println!(
        "added {} transactions, {} operations",
        added_txns, added_ops
    );
    println!("skipped {} non-lot rows", skipped_other);
    println!(
        "store now has {} transactions, {} operations",
        store.applied_transactions.len(),
        store.operations.len()
    );

    // Print lot summary.
    let mut symbols: Vec<&String> = store.lots.keys().collect();
    symbols.sort();
    println!("\nopen positions ({} symbols):", symbols.len());
    for sym in symbols {
        let lots = &store.lots[sym];
        let qty: f64 = lots.iter().map(|l| l.remaining).sum();
        let cost: f64 = lots
            .iter()
            .filter(|l| l.remaining > QTY_EPSILON)
            .map(|l| l.remaining * l.cost_per_share)
            .sum();
        let avg = if qty > 0.0 { cost / qty } else { 0.0 };
        println!(
            "  {sym}: {qty:.0} shares, {:.0} lots, avg ${avg:.2}",
            lots.len()
        );
    }

    Ok(())
}

// ── CSV parsing ─────────────────────────────────────────────────────────────

#[derive(Debug)]
enum Action {
    Buy,
    Sell,
    NameChange,
    StockMerger,
    ReverseSplit,
    Other,
}

#[derive(Debug)]
struct CsvRow {
    date: NaiveDate,
    action: Action,
    symbol: String,
    description: String,
    quantity: f64,
    price: f64,
    amount: f64,
}

fn parse_csv(path: &str) -> Result<Vec<CsvRow>> {
    let mut rdr = csv::Reader::from_path(path).context("failed to open CSV")?;
    let mut rows = Vec::new();

    for result in rdr.records() {
        let record = result.context("failed to read CSV row")?;

        let date_str = record.get(0).unwrap_or("").trim().trim_matches('"');
        let action_str = record.get(1).unwrap_or("").trim().trim_matches('"');
        let symbol = record.get(2).unwrap_or("").trim().trim_matches('"');
        let description = record.get(3).unwrap_or("").trim().trim_matches('"');
        let qty_str = record.get(4).unwrap_or("").trim().trim_matches('"');
        let price_str = record.get(5).unwrap_or("").trim().trim_matches('"');
        let amount_str = record.get(7).unwrap_or("").trim().trim_matches('"');

        let action = match action_str {
            "Buy" => Action::Buy,
            "Sell" | "Sell Short" => Action::Sell,
            "Name Change" => Action::NameChange,
            "Stock Merger" => Action::StockMerger,
            "Reverse Split" => Action::ReverseSplit,
            "Security Transfer" if !symbol.is_empty() && !qty_str.is_empty() => Action::Buy,
            _ => Action::Other,
        };

        // Skip non-lot rows early.
        if matches!(action, Action::Other) {
            rows.push(CsvRow {
                date: NaiveDate::default(),
                action,
                symbol: String::new(),
                description: String::new(),
                quantity: 0.0,
                price: 0.0,
                amount: 0.0,
            });
            continue;
        }

        let date = NaiveDate::parse_from_str(date_str, "%m/%d/%Y")
            .with_context(|| format!("bad date: {date_str}"))?;

        let quantity: f64 = match qty_str.replace(',', "").parse() {
            Ok(q) => q,
            Err(_) => {
                eprintln!(
                    "  warning: unparseable quantity '{}' for {} {} on {} — using 0.0",
                    qty_str, action_str, symbol, date_str
                );
                0.0
            }
        };

        let price = parse_dollar(price_str);
        let amount = parse_dollar(amount_str);

        if matches!(action, Action::Buy | Action::Sell) && (quantity == 0.0 || price == 0.0) {
            eprintln!(
                "  warning: zero quantity ({}) or price ({}) for {} {} on {}",
                quantity, price, action_str, symbol, date_str
            );
        }

        rows.push(CsvRow {
            date,
            action,
            symbol: symbol.to_string(),
            description: description.to_string(),
            quantity,
            price,
            amount,
        });
    }

    // CSV is reverse-chronological; we want chronological.
    rows.reverse();
    Ok(rows)
}

fn parse_dollar(s: &str) -> f64 {
    let s = s.trim().trim_matches('"');
    if s.is_empty() {
        return 0.0;
    }
    let negative = s.starts_with('-');
    let cleaned = s
        .trim_start_matches('-')
        .trim_start_matches('$')
        .replace(',', "");
    let val: f64 = cleaned.parse().unwrap_or(0.0);
    if negative { -val } else { val }
}

// ── Corporate action helpers ────────────────────────────────────────────────

/// Check if a symbol looks like a CUSIP (starts with a digit).
fn is_cusip(s: &str) -> bool {
    s.chars().next().is_some_and(|c| c.is_ascii_digit())
}

/// Resolve the new_symbol, replacing CUSIPs with tickers if known.
fn resolve_new_symbol(opening_row: &CsvRow, cusip_to_ticker: &HashMap<String, String>) -> String {
    if is_cusip(&opening_row.symbol)
        && let Some(ticker) = cusip_to_ticker.get(&opening_row.symbol)
    {
        return ticker.clone();
    }
    opening_row.symbol.clone()
}

/// Extract a normalized company name prefix from a description for matching.
/// E.g., "MOGO INC F" -> "mogo inc", "ROCKET COMPANIES CLA A CLASS A" -> "rocket companies"
fn description_prefix(desc: &str) -> Option<String> {
    let words: Vec<&str> = desc.split_whitespace().collect();
    // Take words until we hit a common suffix (INC, CORP, HLDGS, etc.) + one more.
    let stop_words = [
        "INC",
        "CORP",
        "HLDGS",
        "TECHNOLOGIES",
        "COMPANIES",
        "GROUP",
        "MATLS",
    ];
    for (i, word) in words.iter().enumerate() {
        if stop_words.contains(&word.to_uppercase().as_str()) {
            let prefix: Vec<&str> = words[..=i].to_vec();
            return Some(prefix.join(" ").to_lowercase());
        }
    }
    // Fallback: first 2 words.
    if words.len() >= 2 {
        Some(words[..2].join(" ").to_lowercase())
    } else {
        None
    }
}

/// Resolve the old symbol for a corporate action closing row.
fn resolve_old_symbol(
    closing_row: &CsvRow,
    desc_to_symbol: &HashMap<String, String>,
    first_word_to_symbol: &HashMap<String, String>,
    store: &TaxLotStore,
    cusip_to_ticker: &HashMap<String, String>,
) -> String {
    // If the closing symbol is a known CUSIP, use the mapped ticker.
    if is_cusip(&closing_row.symbol)
        && let Some(ticker) = cusip_to_ticker.get(&closing_row.symbol)
    {
        return ticker.clone();
    }

    // The closing row's description has format like "MOGO INC XXXNAME CHANGE EFF: ..."
    // Extract the company name before "XXX".
    let desc = &closing_row.description;
    if let Some(idx) = desc.find("XXX") {
        let company_part = desc[..idx].trim();
        // Try to match against our description prefix map.
        if let Some(prefix) = description_prefix(company_part)
            && let Some(symbol) = desc_to_symbol.get(&prefix)
        {
            return symbol.clone();
        }
        // Fuzzy: try matching first 2 words.
        let words: Vec<&str> = company_part.split_whitespace().collect();
        if words.len() >= 2 {
            let key = words[..2].join(" ").to_lowercase();
            for (desc_key, symbol) in desc_to_symbol.iter() {
                if desc_key.starts_with(&key) {
                    return symbol.clone();
                }
            }
        }
        // Last fuzzy: match on just the first word of the company name.
        if let Some(word) = words.first() {
            let key = word.to_lowercase();
            if let Some(symbol) = first_word_to_symbol.get(&key) {
                return symbol.clone();
            }
        }
    }

    // Check existing lots in the store — if a symbol has lots and matches the qty, use it.
    let close_qty = closing_row.quantity.abs();
    for (sym, lots) in &store.lots {
        let total: f64 = lots.iter().map(|l| l.quantity).sum();
        if (total - close_qty).abs() < 0.01 {
            eprintln!(
                "  resolved old symbol from existing lots: {} (qty {:.0})",
                sym, total
            );
            return sym.clone();
        }
    }

    // Last resort: use the CUSIP/raw symbol from the closing row and warn.
    eprintln!(
        "  warning: could not resolve old symbol for '{}' — using '{}' (edit in operations page)",
        desc, closing_row.symbol
    );
    closing_row.symbol.clone()
}
