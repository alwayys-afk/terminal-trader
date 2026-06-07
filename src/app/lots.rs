use std::collections::{HashMap, HashSet};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::schwab::{
    ConsumedLot, LotSale, LotSelectionMethod, QTY_EPSILON, TaxLot, transaction_instrument_symbol,
};
use schwab_trader::types as tr;

// ── Trade action ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TradeAction {
    Buy,
    Sell,
}

impl std::fmt::Display for TradeAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TradeAction::Buy => write!(f, "Buy"),
            TradeAction::Sell => write!(f, "Sell"),
        }
    }
}

impl std::str::FromStr for TradeAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "buy" | "b" => Ok(TradeAction::Buy),
            "sell" | "s" => Ok(TradeAction::Sell),
            _ => Err(format!("invalid action: {s}")),
        }
    }
}

// ── Operation types ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationKind {
    /// Adjust share count: splits, reverse splits, merger qty changes.
    Multiplier {
        symbol: String,
        old_qty: f64,
        new_qty: f64,
    },
    /// Ticker change: name changes, mergers.
    NameChange {
        old_symbol: String,
        new_symbol: String,
    },
}

impl std::fmt::Display for OperationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationKind::Multiplier {
                old_qty, new_qty, ..
            } => {
                if new_qty < old_qty {
                    write!(f, "Reverse Split")
                } else {
                    write!(f, "Split")
                }
            }
            OperationKind::NameChange { .. } => write!(f, "Name Change"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub date: NaiveDate,
    pub kind: OperationKind,
    pub description: String,
    pub source_activity_ids: Vec<i64>,
    pub confirmed: bool,
}

/// A lightweight record of each applied transaction (for timeline display and rebuild).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppliedTransaction {
    pub date: NaiveDate,
    pub symbol: String,
    pub action: TradeAction,
    pub quantity: f64,
    pub price: f64,
    pub net_amount: f64,
    pub activity_id: i64,
    /// The lot method this sell used: stamped from its originating order once
    /// recovered (see [`TaxLotStore::record_order_methods`]), or set manually on
    /// the operations page. `None` means unknown — the lot accounting falls back
    /// to the account default, and the operations page shows the cell blank.
    #[serde(default)]
    pub lot_method: Option<LotSelectionMethod>,
    /// Schwab order id this trade settled from, used to recover the lot method
    /// the order was placed with (the transactions API never reports it).
    #[serde(default)]
    pub order_id: Option<i64>,
    /// Full trade datetime from the API (ISO format). Used for correct intra-day ordering.
    #[serde(default)]
    pub trade_datetime: Option<String>,
}

/// Persisted tax lot state for the account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxLotStore {
    /// Symbol → list of open lots. Derived via `rebuild()` — not persisted.
    #[serde(skip)]
    pub lots: HashMap<String, Vec<TaxLot>>,
    /// Symbol → list of recorded sales (closing transactions). Derived via `rebuild()`.
    #[serde(skip)]
    pub sales: HashMap<String, Vec<LotSale>>,
    /// Activity IDs of transactions already applied (for deduplication).
    #[serde(default)]
    pub applied_activity_ids: HashSet<i64>,
    /// Recorded transactions for timeline display and rebuild.
    #[serde(default)]
    pub applied_transactions: Vec<AppliedTransaction>,
    /// Auto-detected and user-managed operations (corporate actions).
    #[serde(default)]
    pub operations: Vec<Operation>,
    /// ISO date string of last successful transaction sync.
    #[serde(default)]
    pub last_sync_date: Option<String>,
    /// Default lot-selection method applied to sells that carry no explicit
    /// method (CSV-seeded sells, API-fetched sells). FIFO is the industry
    /// default; configurable per account and overridable per import via
    /// `--seed --lot-method`. See issue #74.
    #[serde(default)]
    pub default_lot_method: LotSelectionMethod,
    /// Order id → tax lot method recovered from the orders endpoint, used as an
    /// in-session inbox. The transactions API never reports the method a sale
    /// used, but the order does (see SCHWAB_API_ISSUES.md #5), so we recover it
    /// and *stamp* it onto the sell's row (`AppliedTransaction.lot_method`). The
    /// stamped rows are the durable source of truth — this map is not persisted:
    /// it's repopulated each session from fetched orders.
    #[serde(skip)]
    order_methods: HashMap<i64, LotSelectionMethod>,
    /// Monotonic counter for execution ordering of lots and sales.
    #[serde(skip)]
    seq_counter: usize,
}

impl TaxLotStore {
    pub fn load(path: &str) -> Self {
        let data = match std::fs::read_to_string(path) {
            Ok(d) => d,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                tracing::info!("tax lot file not found at {path}, starting fresh");
                return Self::default();
            }
            Err(e) => {
                tracing::error!("failed to read tax lot file {path}: {e}");
                return Self::default();
            }
        };
        match serde_json::from_str::<Self>(&data) {
            Ok(mut store) => {
                store.rebuild();
                store
            }
            Err(e) => {
                tracing::error!(
                    "failed to parse tax lot file {path}: {e} — backing up to {path}.bak"
                );
                let _ = std::fs::copy(path, format!("{path}.bak"));
                Self::default()
            }
        }
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(path, data)
    }

    fn next_seq(&mut self) -> usize {
        self.seq_counter += 1;
        self.seq_counter
    }

    pub fn lots_for(&self, symbol: &str) -> &[TaxLot] {
        self.lots.get(symbol).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn sales_for(&self, symbol: &str) -> &[LotSale] {
        self.sales.get(symbol).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Record tax lot methods recovered from the orders endpoint (order id →
    /// method pairs; only sells carry a method) into the in-session inbox, then
    /// stamp them onto any already-applied sell that is still unattributed. A
    /// method already on a row — recovered earlier or set as a manual override —
    /// is never overwritten, so re-fetching orders can't clobber a user's
    /// conscious change. Returns true if the inbox or any row changed,
    /// signalling the caller to `rebuild()`.
    pub fn record_order_methods<I>(&mut self, entries: I) -> bool
    where
        I: IntoIterator<Item = (i64, LotSelectionMethod)>,
    {
        let mut changed = false;
        for (id, method) in entries {
            if self.order_methods.insert(id, method) != Some(method) {
                changed = true;
            }
        }
        changed |= self.stamp_blanks_from_inbox();
        changed
    }

    /// Stamp the method from the in-session inbox onto every sell whose order we
    /// know but that carries no method yet. Blanks only — a row that already has
    /// a method (recovered or manually overridden) is left untouched. Returns
    /// true if any row was stamped.
    fn stamp_blanks_from_inbox(&mut self) -> bool {
        let inbox = &self.order_methods;
        let mut changed = false;
        for txn in &mut self.applied_transactions {
            if txn.lot_method.is_none()
                && let Some(method) = txn.order_id.and_then(|id| inbox.get(&id).copied())
            {
                txn.lot_method = Some(method);
                changed = true;
            }
        }
        changed
    }

    /// Extract `(order_id, method)` pairs from orders that carry a tax lot
    /// method (sells), for feeding [`record_order_methods`].
    pub fn order_method_pairs(orders: &[crate::schwab::Order]) -> Vec<(i64, LotSelectionMethod)> {
        orders
            .iter()
            .filter_map(|o| Some((o.order_id, o.tax_lot_method?)))
            .collect()
    }

    /// Effective lot method for a recorded sell: the method stamped onto the row
    /// — recovered from its order (see [`record_order_methods`]) or set as a
    /// manual override — else the account default for rows we know nothing about.
    fn resolve_method(&self, lot_method: Option<LotSelectionMethod>) -> LotSelectionMethod {
        lot_method.unwrap_or(self.default_lot_method)
    }

    /// Process a trade against existing lots. Sells close long lots; buys
    /// close short lots. Uncovered quantity opens a new lot on the opposite
    /// side. Returns consumed lots for recording the sale/cover.
    pub fn process_trade(
        &mut self,
        symbol: &str,
        mut qty: f64,
        is_sell: bool,
        method: LotSelectionMethod,
        trade_date: NaiveDate,
        trade_price: f64,
        seq: usize,
    ) -> Vec<ConsumedLot> {
        let lots = self.lots.entry(symbol.to_string()).or_default();

        // Sells close longs (remaining > 0); buys close shorts (remaining < 0).
        let mut indices: Vec<usize> = lots
            .iter()
            .enumerate()
            .filter(|(_, l)| {
                if is_sell {
                    l.remaining > QTY_EPSILON
                } else {
                    l.remaining < -QTY_EPSILON
                }
            })
            .map(|(i, _)| i)
            .collect();

        match method {
            LotSelectionMethod::Lifo => {
                indices.sort_by(|&a, &b| {
                    lots[b]
                        .open_date
                        .cmp(&lots[a].open_date)
                        .then(lots[b].seq.cmp(&lots[a].seq))
                });
            }
            LotSelectionMethod::Fifo => {
                indices.sort_by(|&a, &b| {
                    lots[a]
                        .open_date
                        .cmp(&lots[b].open_date)
                        .then(lots[a].seq.cmp(&lots[b].seq))
                });
            }
            LotSelectionMethod::Hifo => {
                indices.sort_by(|&a, &b| {
                    lots[b]
                        .cost_per_share
                        .partial_cmp(&lots[a].cost_per_share)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
        }

        let mut consumed = Vec::new();
        for &idx in &indices {
            if qty <= QTY_EPSILON {
                break;
            }
            let available = lots[idx].remaining.abs();
            let consumed_qty = available.min(qty);

            // Sell closing a long: (sell_price - cost) * qty
            // Buy covering a short: (short_open_price - cover_price) * qty
            let realized = if is_sell {
                (trade_price - lots[idx].cost_per_share) * consumed_qty
            } else {
                (lots[idx].cost_per_share - trade_price) * consumed_qty
            };

            consumed.push(ConsumedLot {
                open_date: lots[idx].open_date,
                quantity: consumed_qty,
                cost_per_share: lots[idx].cost_per_share,
            });

            if is_sell {
                lots[idx].remaining -= consumed_qty;
            } else {
                lots[idx].remaining += consumed_qty;
            }
            lots[idx].realized_pnl += realized;
            qty -= consumed_qty;
        }

        // Uncovered quantity opens a new lot on the opposite side.
        if qty > QTY_EPSILON {
            let sign = if is_sell { -1.0 } else { 1.0 };
            lots.push(TaxLot::new(trade_date, sign * qty, trade_price, seq));
        }

        consumed
    }

    /// Move all lots and sales from `old_symbol` to `new_symbol`.
    /// Used for mergers and name changes.
    pub fn rename_symbol(&mut self, old_symbol: &str, new_symbol: &str) {
        // not sure how something would have had that name before but ok
        if let Some(old_lots) = self.lots.remove(old_symbol) {
            self.lots
                .entry(new_symbol.to_string())
                .or_default()
                .extend(old_lots);
        }
        if let Some(old_sales) = self.sales.remove(old_symbol) {
            self.sales
                .entry(new_symbol.to_string())
                .or_default()
                .extend(old_sales);
        }
    }

    /// Adjust all open lots for a stock split.
    /// `ratio` = new_shares / old_shares (e.g. 2.0 for a 2:1 split).
    pub fn adjust_split(&mut self, symbol: &str, ratio: f64) {
        if let Some(lots) = self.lots.get_mut(symbol) {
            for lot in lots.iter_mut() {
                lot.quantity *= ratio;
                lot.remaining *= ratio;
                lot.cost_per_share /= ratio;
            }
        }
    }

    /// Apply a transaction to the lot store.
    /// Returns true if the store was modified.
    pub fn apply_transaction(&mut self, txn: &tr::Transaction) -> bool {
        // Only process TRADE transactions; corporate actions go through detect_operations.
        if txn.type_.as_ref() != Some(&tr::TransactionType::Trade) {
            return false;
        }

        // Deduplicate by activity_id.
        let activity_id = match txn.activity_id {
            Some(id) => {
                if !self.applied_activity_ids.insert(id) {
                    return false;
                }
                id
            }
            None => return false,
        };
        let trade_date = match txn.trade_date {
            Some(dt) => dt.date_naive(),
            None => return false,
        };

        // Prefer the `time` field (actual execution time) over `trade_date`
        // (which is often just the date at midnight).
        let trade_datetime = txn
            .time
            .or(txn.trade_date)
            .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S").to_string());

        let mut modified = false;

        for item in &txn.transfer_items {
            let symbol = match item
                .instrument
                .as_ref()
                .and_then(|i| transaction_instrument_symbol(i))
            {
                Some(s) if s != "CURRENCY_USD" => s.to_string(),
                _ => continue,
            };

            let amount = match item.amount {
                Some(a) if a != 0.0 => a,
                _ => continue,
            };

            let price = match item.price {
                Some(p) => p,
                None => continue,
            };

            let net_amount = item.cost.unwrap_or(0.0);

            let qty = amount.abs();
            let action = match item.position_effect {
                Some(tr::TransferItemPositionEffect::Opening) => TradeAction::Buy,
                Some(tr::TransferItemPositionEffect::Closing) => TradeAction::Sell,
                _ => continue,
            };

            let is_sell = action == TradeAction::Sell;
            let seq = self.next_seq();
            // Stamp the method recovered from this sell's order (if we know it)
            // onto the row; buys carry none. Falls back to the account default
            // for the lot accounting when unknown.
            let stamped = if is_sell {
                txn.order_id
                    .and_then(|id| self.order_methods.get(&id).copied())
            } else {
                None
            };
            let method = self.resolve_method(stamped);
            let consumed =
                self.process_trade(&symbol, qty, is_sell, method, trade_date, price, seq);

            if is_sell {
                let consumed_cost: f64 = consumed.iter().map(|l| l.total_cost()).sum();
                let proceeds = price * qty;
                let realized_pnl = proceeds - consumed_cost;
                let sale = LotSale {
                    sale_date: trade_date,
                    quantity: qty,
                    sale_price: price,
                    seq: self.next_seq(),
                    consumed_lots: consumed,
                    realized_pnl,
                };
                self.sales.entry(symbol.clone()).or_default().push(sale);
            }

            self.applied_transactions.push(AppliedTransaction {
                date: trade_date,
                symbol: symbol.clone(),
                action,
                quantity: qty,
                price,
                net_amount,
                activity_id,
                lot_method: stamped,
                order_id: txn.order_id,
                trade_datetime: trade_datetime.clone(),
            });
            modified = true;
        }

        modified
    }

    /// Apply a single operation to the lot store.
    pub fn apply_operation(&mut self, op: &Operation) {
        match &op.kind {
            OperationKind::NameChange {
                old_symbol,
                new_symbol,
            } => {
                self.rename_symbol(old_symbol, new_symbol);
            }
            OperationKind::Multiplier {
                symbol,
                old_qty,
                new_qty,
            } => {
                if *old_qty > 0.0 {
                    let ratio = new_qty / old_qty;
                    self.adjust_split(symbol, ratio);
                }
            }
        }
    }

    fn flush_pending_buy(&mut self, symbol: &str, pending: PendingBuy) {
        let price = pending.total_cost / pending.total_qty;
        let seq = self.next_seq();
        self.process_trade(
            symbol,
            pending.total_qty,
            false,
            pending.method,
            pending.date,
            price,
            seq,
        );
    }

    /// Full rebuild from scratch: clears derived state (lots + sales), then replays
    /// applied_transactions + operations in chronological order.
    ///
    /// Same-day buys for the same symbol within 1% of each other's price
    /// (with no intervening sells) are squashed into a single lot at the
    /// weighted-average cost.
    pub fn rebuild(&mut self) {
        self.lots.clear();
        self.sales.clear();

        // Clone inputs to avoid borrow-checker issues (we need to mutate self while iterating).
        let txns = self.applied_transactions.clone();
        let ops = self.operations.clone();

        // Build sorted event list: (sort_key, index, is_op).
        // API entries use trade_datetime (actual execution time from the `time`
        // field).  CSV entries (no trade_datetime) use date + activity_id which
        // preserves chronological order.  Operations sort before trades on the
        // same date so splits adjust lots before trades execute.
        let mut events: Vec<(String, usize, bool)> = Vec::new();
        for (i, txn) in txns.iter().enumerate() {
            let key = txn.trade_datetime.clone().unwrap_or_else(|| {
                // CSV entries have no trade_datetime — use index to preserve
                // the seeder's chronological sort.
                format!("{}T00:00:00.{:010}", txn.date, i)
            });
            events.push((key, i, false));
        }
        for (i, op) in ops.iter().enumerate() {
            events.push((format!("{}T00:00:00", op.date), i, true));
        }
        events.sort();

        self.seq_counter = 0;

        // Same-day buys within 1% of the running average price are merged into one lot.
        let mut pending_buy: HashMap<String, PendingBuy> = HashMap::new();

        for (_, idx, is_op) in events {
            if is_op {
                // Flush all pending buys before applying an operation.
                let pending: Vec<(String, PendingBuy)> = pending_buy.drain().collect();
                for (sym, p) in pending {
                    self.flush_pending_buy(&sym, p);
                }
                self.apply_operation(&ops[idx]);
            } else {
                let txn = &txns[idx];
                let is_sell = txn.action == TradeAction::Sell;
                let method = self.resolve_method(txn.lot_method);

                if is_sell {
                    // Flush pending buy for this symbol before processing the sell.
                    if let Some(p) = pending_buy.remove(&txn.symbol) {
                        self.flush_pending_buy(&txn.symbol, p);
                    }
                    let seq = self.next_seq();
                    let consumed = self.process_trade(
                        &txn.symbol,
                        txn.quantity,
                        true,
                        method,
                        txn.date,
                        txn.price,
                        seq,
                    );
                    let consumed_cost: f64 = consumed.iter().map(|l| l.total_cost()).sum();
                    let proceeds = txn.price * txn.quantity;
                    let realized_pnl = proceeds - consumed_cost;
                    let sale = LotSale {
                        sale_date: txn.date,
                        quantity: txn.quantity,
                        sale_price: txn.price,
                        seq: self.next_seq(),
                        consumed_lots: consumed,
                        realized_pnl,
                    };
                    self.sales.entry(txn.symbol.clone()).or_default().push(sale);
                } else {
                    // Buy — accumulate with pending buy if same day and within 1% of avg price.
                    let can_accumulate = pending_buy.get(&txn.symbol).is_some_and(|p| {
                        let avg_price = p.total_cost / p.total_qty;
                        p.date == txn.date && (txn.price - avg_price).abs() / avg_price <= 0.01
                    });

                    if can_accumulate {
                        let entry = pending_buy.get_mut(&txn.symbol).unwrap();
                        entry.total_cost += txn.price * txn.quantity;
                        entry.total_qty += txn.quantity;
                    } else {
                        // Flush stale pending buy (different day or price too far) then start fresh.
                        if let Some(p) = pending_buy.remove(&txn.symbol) {
                            self.flush_pending_buy(&txn.symbol, p);
                        }
                        pending_buy.insert(
                            txn.symbol.clone(),
                            PendingBuy {
                                date: txn.date,
                                total_cost: txn.price * txn.quantity,
                                total_qty: txn.quantity,
                                method,
                            },
                        );
                    }
                }
            }
        }

        for (sym, p) in pending_buy {
            self.flush_pending_buy(&sym, p);
        }
    }
}

/// Accumulator for same-day buys awaiting a flush into the lot store.
/// `total_cost` / `total_qty` yields the weighted-average price.
struct PendingBuy {
    date: NaiveDate,
    total_cost: f64,
    total_qty: f64,
    method: LotSelectionMethod,
}

// ── Detect operations from RECEIVE_AND_DELIVER transactions ─────────────
// The API always returns proper ticker symbols (not CUSIPs) on R&D transactions.
// CUSIP handling is only needed in the CSV seeder path (seed.rs).
pub fn detect_operations(
    txns: &[tr::Transaction],
    existing_activity_ids: &HashSet<i64>,
) -> Vec<Operation> {
    // Filter to RECEIVE_AND_DELIVER, skip already-seen activity_ids.
    let rd_txns: Vec<&tr::Transaction> = txns
        .iter()
        .filter(|t| t.type_.as_ref() == Some(&tr::TransactionType::ReceiveAndDeliver))
        .filter(|t| {
            t.activity_id
                .map(|id| !existing_activity_ids.contains(&id))
                .unwrap_or(false)
        })
        .collect();

    if rd_txns.is_empty() {
        return vec![];
    }

    // Group by trade_date.
    // question: does this handle symbol grouping by assuming a rename would have 2 consecutive rows?
    // question: do we have to sort to make that assumption? or is whatever is passed into here sorted already?
    let mut by_date: HashMap<NaiveDate, Vec<&tr::Transaction>> = HashMap::new();
    for txn in &rd_txns {
        if let Some(dt) = txn.trade_date {
            by_date.entry(dt.date_naive()).or_default().push(txn);
        }
    }

    let mut ops = Vec::new();

    for (date, group) in &by_date {
        // Find Closing + Opening pairs.
        // question: when would a opening one come before the closing one
        let mut closings: Vec<&tr::Transaction> = Vec::new();
        let mut openings: Vec<&tr::Transaction> = Vec::new();

        for txn in group {
            if txn
                .transfer_items
                .iter()
                .any(|item| item.position_effect == Some(tr::TransferItemPositionEffect::Closing))
            {
                closings.push(txn);
            }
            if txn
                .transfer_items
                .iter()
                .any(|item| item.position_effect == Some(tr::TransferItemPositionEffect::Opening))
            {
                openings.push(txn);
            }
        }

        // Pair closings with openings. Usually 1:1, but multiple stocks can
        // have corporate actions on the same date. Zip pairs them in order;
        // any wrong pairings show as unconfirmed and can be fixed on the operations page.
        for (closing, opening) in closings.iter().zip(openings.iter()) {
            let (close_sym, close_qty) = extract_rd_info(closing);
            let (open_sym, open_qty) = extract_rd_info(opening);

            let (Some(old_sym), Some(new_sym)) = (close_sym, open_sym) else {
                continue;
            };

            let old_qty = close_qty.abs();
            let new_qty = open_qty.abs();

            let desc = closing
                .description
                .as_deref()
                .or(opening.description.as_deref())
                .unwrap_or("");

            let mut source_ids = Vec::new();
            if let Some(id) = closing.activity_id {
                source_ids.push(id);
            }
            if let Some(id) = opening.activity_id {
                source_ids.push(id);
            }

            // Symbol changed → NameChange op.
            if old_sym != new_sym {
                info!(
                    "detected name change: {} -> {} on {}",
                    old_sym, new_sym, date
                );
                ops.push(Operation {
                    date: *date,
                    kind: OperationKind::NameChange {
                        old_symbol: old_sym.clone(),
                        new_symbol: new_sym.clone(),
                    },
                    description: desc.to_string(),
                    source_activity_ids: source_ids.clone(),
                    confirmed: false,
                });
            }

            // Quantity changed → Multiplier op (on the new symbol).
            if (old_qty - new_qty).abs() > 0.01 {
                let symbol = if old_sym != new_sym { new_sym } else { old_sym };
                info!(
                    "detected multiplier: {} {:.0} -> {:.0} on {}",
                    symbol, old_qty, new_qty, date
                );
                ops.push(Operation {
                    date: *date,
                    kind: OperationKind::Multiplier {
                        symbol,
                        old_qty,
                        new_qty,
                    },
                    description: desc.to_string(),
                    source_activity_ids: source_ids,
                    confirmed: false,
                });
            }
        }
    }

    ops.sort_by_key(|op| op.date);
    ops
}

fn extract_rd_info(txn: &tr::Transaction) -> (Option<String>, f64) {
    for item in &txn.transfer_items {
        let symbol = item
            .instrument
            .as_ref()
            .and_then(|i| transaction_instrument_symbol(i))
            .map(|s| s.to_string());
        let amount = item.amount.unwrap_or(0.0);
        if symbol.is_some() {
            return (symbol, amount);
        }
    }
    (None, 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_buy(
        symbol: &str,
        date: &str,
        price: f64,
        qty: f64,
        activity_id: i64,
    ) -> AppliedTransaction {
        AppliedTransaction {
            date: date.parse().unwrap(),
            symbol: symbol.to_string(),
            action: TradeAction::Buy,
            quantity: qty,
            price,
            net_amount: -(price * qty),
            activity_id,
            lot_method: None,
            order_id: None,
            trade_datetime: Some(format!("{date}T10:00:00")),
        }
    }

    fn make_sell(
        symbol: &str,
        date: &str,
        price: f64,
        qty: f64,
        activity_id: i64,
    ) -> AppliedTransaction {
        AppliedTransaction {
            date: date.parse().unwrap(),
            symbol: symbol.to_string(),
            action: TradeAction::Sell,
            quantity: qty,
            price,
            net_amount: price * qty,
            activity_id,
            lot_method: None,
            order_id: None,
            trade_datetime: Some(format!("{date}T10:00:00")),
        }
    }

    #[test]
    fn squash_same_day_same_price_buys() {
        let mut store = TaxLotStore::default();
        store.applied_transactions = vec![
            make_buy("AAPL", "2025-01-15", 100.0, 5.0, 1),
            make_buy("AAPL", "2025-01-15", 100.0, 3.0, 2),
            make_buy("AAPL", "2025-01-15", 100.0, 2.0, 3),
        ];
        store.applied_activity_ids = [1, 2, 3].into();
        store.rebuild();

        let lots = store.lots_for("AAPL");
        assert_eq!(
            lots.len(),
            1,
            "three same-day same-price buys should squash into one lot"
        );
        assert!((lots[0].quantity - 10.0).abs() < QTY_EPSILON);
        assert!((lots[0].cost_per_share - 100.0).abs() < 0.0001);
    }

    #[test]
    fn squash_within_one_percent() {
        let mut store = TaxLotStore::default();
        // 100.0 and 100.50 are 0.5% apart — should squash.
        store.applied_transactions = vec![
            make_buy("AAPL", "2025-01-15", 100.0, 5.0, 1),
            make_buy("AAPL", "2025-01-15", 100.50, 3.0, 2),
        ];
        store.applied_activity_ids = [1, 2].into();
        store.rebuild();

        let lots = store.lots_for("AAPL");
        assert_eq!(lots.len(), 1, "prices within 1% should squash");
        assert!((lots[0].quantity - 8.0).abs() < QTY_EPSILON);
        // Weighted avg: (100*5 + 100.5*3) / 8 = 801.50 / 8 = 100.1875
        assert!((lots[0].cost_per_share - 100.1875).abs() < 0.0001);
    }

    #[test]
    fn no_squash_beyond_one_percent() {
        let mut store = TaxLotStore::default();
        // 100.0 and 102.0 are 2% apart — should not squash.
        store.applied_transactions = vec![
            make_buy("AAPL", "2025-01-15", 100.0, 5.0, 1),
            make_buy("AAPL", "2025-01-15", 102.0, 3.0, 2),
        ];
        store.applied_activity_ids = [1, 2].into();
        store.rebuild();

        let lots = store.lots_for("AAPL");
        assert_eq!(lots.len(), 2, "prices >1% apart should not squash");
    }

    #[test]
    fn no_squash_different_days() {
        let mut store = TaxLotStore::default();
        store.applied_transactions = vec![
            make_buy("AAPL", "2025-01-15", 100.0, 5.0, 1),
            make_buy("AAPL", "2025-01-16", 100.0, 3.0, 2),
        ];
        store.applied_activity_ids = [1, 2].into();
        store.rebuild();

        let lots = store.lots_for("AAPL");
        assert_eq!(lots.len(), 2, "different-day buys should not squash");
    }

    #[test]
    fn sell_breaks_squash_chain() {
        let mut store = TaxLotStore::default();
        store.applied_transactions = vec![
            make_buy("AAPL", "2025-01-15", 100.0, 5.0, 1),
            make_sell("AAPL", "2025-01-15", 105.0, 2.0, 2),
            make_buy("AAPL", "2025-01-15", 100.0, 3.0, 3),
        ];
        store.applied_activity_ids = [1, 2, 3].into();
        store.rebuild();

        // First buy: 5 shares, sell removes 2 → 3 remaining. Second buy: new lot of 3.
        let lots = store.lots_for("AAPL");
        assert_eq!(lots.len(), 2, "sell between buys should prevent squashing");
        let total_remaining: f64 = lots.iter().map(|l| l.remaining).sum();
        assert!((total_remaining - 6.0).abs() < QTY_EPSILON);
    }

    #[test]
    fn squash_preserves_pnl_math() {
        let mut store = TaxLotStore::default();
        // 100.00 and 100.20 are 0.2% apart — squashed with weighted avg.
        // Total cost = 100*5 + 100.20*5 = 1001.00, avg = 100.10
        store.applied_transactions = vec![
            make_buy("AAPL", "2025-01-15", 100.0, 5.0, 1),
            make_buy("AAPL", "2025-01-15", 100.20, 5.0, 2),
            make_sell("AAPL", "2025-01-16", 110.0, 10.0, 3),
        ];
        store.applied_activity_ids = [1, 2, 3].into();
        store.rebuild();

        let sales = store.sales_for("AAPL");
        assert_eq!(sales.len(), 1);
        // P&L = (110 - 100.10) * 10 = 99.00
        assert!((sales[0].realized_pnl - 99.0).abs() < 0.01);
    }

    #[test]
    fn squash_different_symbols_independent() {
        let mut store = TaxLotStore::default();
        store.applied_transactions = vec![
            make_buy("AAPL", "2025-01-15", 100.0, 5.0, 1),
            make_buy("MSFT", "2025-01-15", 200.0, 3.0, 2),
            make_buy("AAPL", "2025-01-15", 100.0, 5.0, 3),
            make_buy("MSFT", "2025-01-15", 200.0, 3.0, 4),
        ];
        store.applied_activity_ids = [1, 2, 3, 4].into();
        store.rebuild();

        let aapl = store.lots_for("AAPL");
        let msft = store.lots_for("MSFT");
        assert_eq!(aapl.len(), 1);
        assert_eq!(msft.len(), 1);
        assert!((aapl[0].quantity - 10.0).abs() < QTY_EPSILON);
        assert!((msft[0].quantity - 6.0).abs() < QTY_EPSILON);
    }

    #[test]
    fn lot_method_serializes_uppercase() {
        assert_eq!(
            serde_json::to_string(&LotSelectionMethod::Lifo).unwrap(),
            "\"LIFO\""
        );
        let parsed: LotSelectionMethod = serde_json::from_str("\"LIFO\"").unwrap();
        assert_eq!(parsed, LotSelectionMethod::Lifo);
    }

    #[test]
    fn lot_method_prefix_parse() {
        use LotSelectionMethod::*;
        // Unique-prefix, case-insensitive: f/l/h and any longer prefix resolve.
        for (s, expected) in [
            ("f", Fifo),
            ("FIFO", Fifo),
            ("Li", Lifo),
            ("hifo", Hifo),
            (" h ", Hifo),
        ] {
            assert_eq!(s.parse::<LotSelectionMethod>().unwrap(), expected, "{s:?}");
        }
        // Empty matches all three (ambiguous) and unknown matches none — both err.
        assert!("".parse::<LotSelectionMethod>().is_err());
        assert!("xyz".parse::<LotSelectionMethod>().is_err());
    }

    #[test]
    fn lot_method_reads_legacy_pascalcase() {
        // Stores written before the UPPERCASE rename persisted the PascalCase
        // variant name. They must still deserialize so load() doesn't reset the
        // whole store to default. See issue #74.
        for (json, expected) in [
            ("\"Lifo\"", LotSelectionMethod::Lifo),
            ("\"Fifo\"", LotSelectionMethod::Fifo),
            ("\"Hifo\"", LotSelectionMethod::Hifo),
        ] {
            let parsed: LotSelectionMethod = serde_json::from_str(json).unwrap();
            assert_eq!(parsed, expected);
        }
    }

    #[test]
    fn default_method_is_fifo() {
        // A fresh store should match lots FIFO (industry default), not LIFO.
        assert_eq!(
            TaxLotStore::default().default_lot_method,
            LotSelectionMethod::Fifo
        );
    }

    #[test]
    fn store_default_method_drives_unmarked_sells() {
        // Two buys on different days at different prices, then a partial sell
        // that closes exactly the first (or last) lot depending on the method.
        // The sells carry lot_method: None, so they resolve against the store's
        // default_lot_method.
        let txns = || {
            vec![
                make_buy("AAPL", "2025-01-15", 100.0, 5.0, 1),
                make_buy("AAPL", "2025-01-16", 200.0, 5.0, 2),
                make_sell("AAPL", "2025-01-17", 300.0, 5.0, 3),
            ]
        };

        // The lot left with shares remaining tells us which the sell closed.
        let remaining_cost = |store: &TaxLotStore| {
            let open: Vec<&TaxLot> = store
                .lots_for("AAPL")
                .iter()
                .filter(|l| l.remaining > QTY_EPSILON)
                .collect();
            assert_eq!(open.len(), 1, "exactly one lot should have shares left");
            assert!((open[0].remaining - 5.0).abs() < QTY_EPSILON);
            open[0].cost_per_share
        };

        // FIFO: closes the older $100 lot → the $200 lot survives.
        let mut fifo = TaxLotStore::default();
        fifo.default_lot_method = LotSelectionMethod::Fifo;
        fifo.applied_transactions = txns();
        fifo.applied_activity_ids = [1, 2, 3].into();
        fifo.rebuild();
        assert!(
            (remaining_cost(&fifo) - 200.0).abs() < 0.0001,
            "FIFO keeps the newer $200 lot"
        );

        // LIFO: closes the newer $200 lot → the $100 lot survives.
        let mut lifo = TaxLotStore::default();
        lifo.default_lot_method = LotSelectionMethod::Lifo;
        lifo.applied_transactions = txns();
        lifo.applied_activity_ids = [1, 2, 3].into();
        lifo.rebuild();
        assert!(
            (remaining_cost(&lifo) - 100.0).abs() < 0.0001,
            "LIFO keeps the older $100 lot"
        );
    }

    #[test]
    fn resolve_method_precedence() {
        let store = TaxLotStore::default(); // default_lot_method = Fifo
        // No method stamped on the row → account default.
        assert_eq!(store.resolve_method(None), LotSelectionMethod::Fifo);
        // A method stamped on the row (recovered or manual) wins over the default.
        assert_eq!(
            store.resolve_method(Some(LotSelectionMethod::Hifo)),
            LotSelectionMethod::Hifo
        );
    }

    #[test]
    fn record_order_methods_stamps_blanks_only() {
        let mut store = TaxLotStore::default();
        let mut blank = make_sell("AAPL", "2025-03-15", 300.0, 5.0, 1);
        blank.order_id = Some(50);
        let mut overridden = make_sell("AAPL", "2025-03-16", 300.0, 5.0, 2);
        overridden.order_id = Some(51);
        overridden.lot_method = Some(LotSelectionMethod::Hifo);
        store.applied_transactions = vec![blank, overridden];

        // Recovering both orders stamps the blank, leaves the override untouched.
        assert!(store.record_order_methods([
            (50_i64, LotSelectionMethod::Lifo),
            (51_i64, LotSelectionMethod::Lifo),
        ]));
        assert_eq!(
            store.applied_transactions[0].lot_method,
            Some(LotSelectionMethod::Lifo)
        );
        assert_eq!(
            store.applied_transactions[1].lot_method,
            Some(LotSelectionMethod::Hifo)
        );

        // Re-recording the same methods → nothing left to stamp.
        assert!(!store.record_order_methods([
            (50_i64, LotSelectionMethod::Lifo),
            (51_i64, LotSelectionMethod::Lifo),
        ]));
    }

    #[test]
    fn record_order_methods_change_detection() {
        let mut store = TaxLotStore::default();
        assert!(store.record_order_methods([(1_i64, LotSelectionMethod::Fifo)]));
        // Same mapping again → no change.
        assert!(!store.record_order_methods([(1_i64, LotSelectionMethod::Fifo)]));
        // Changed mapping → change.
        assert!(store.record_order_methods([(1_i64, LotSelectionMethod::Lifo)]));
    }

    #[test]
    fn order_method_backfill_changes_lot_attribution() {
        // Older cheap lot ($100) + newer pricey lot ($200); a partial sell tagged
        // to order 50. FIFO consumes the cheap lot, LIFO the pricey one — the
        // recovered order method must flip the realized P&L.
        let mut sell = make_sell("AAPL", "2025-03-15", 300.0, 5.0, 3);
        sell.order_id = Some(50);

        let realized = |order_method: Option<LotSelectionMethod>| {
            let mut store = TaxLotStore::default(); // default Fifo
            store.applied_transactions = vec![
                make_buy("AAPL", "2025-01-15", 100.0, 10.0, 1),
                make_buy("AAPL", "2025-02-15", 200.0, 10.0, 2),
                sell.clone(),
            ];
            store.applied_activity_ids = [1, 2, 3].into();
            if let Some(m) = order_method {
                assert!(store.record_order_methods([(50_i64, m)]));
            }
            store.rebuild();
            store.sales_for("AAPL")[0].realized_pnl
        };

        // Default FIFO consumes the $100 lot: 1500 proceeds - 500 cost = 1000.
        assert!((realized(None) - 1000.0).abs() < 0.0001);
        // Recovered LIFO consumes the $200 lot: 1500 - 1000 = 500.
        assert!((realized(Some(LotSelectionMethod::Lifo)) - 500.0).abs() < 0.0001);
    }
}
