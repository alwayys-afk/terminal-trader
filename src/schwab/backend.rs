use std::collections::{HashMap, HashSet};

use anyhow::{Result, anyhow};
use tokio::sync::mpsc;
use tracing::{error, info, warn};

use schwab_trader::types as tr;

use crate::app::lots::TaxLotStore;
use crate::app::{AppCommand, AppPrefs, DataMsg, LotCommand, StreamerStatus, lot_store_path_for};
use crate::schwab::schwab_stream::{SchwabStream, StreamEvent};
use crate::schwab::{AccountSummary, InstrumentSpec, SchwabAuth, SchwabClient, models::Quote};

/// Results from background tasks that need to update DataService state.
enum BackgroundResult {
    /// Startup lot loading completed.
    StartupLots(
        Vec<schwab_trader::types::Transaction>,
        Vec<schwab_trader::types::Transaction>,
    ),
    /// Account activity refresh completed.
    ActivityRefresh {
        ord_symbols: HashSet<String>,
        ord_methods: Vec<(i64, crate::schwab::LotSelectionMethod)>,
        pos_symbols: Option<HashSet<String>>,
        txns: Vec<schwab_trader::types::Transaction>,
    },
}

pub struct DataService {
    auth: SchwabAuth,
    accounts: Vec<AccountSummary>,
    account_hash: String,
    tx: mpsc::Sender<DataMsg>,
    cmd_rx: mpsc::Receiver<AppCommand>,
    lot_store: TaxLotStore,
    lot_store_path: String,
    /// Set true when an account-switch command caused `run_once` to return.
    /// The outer reconnect loop reads this to skip the post-disconnect sleep
    /// so the user sees a near-instant swap.
    switching: bool,
}

impl DataService {
    pub fn new(
        auth: SchwabAuth,
        accounts: Vec<AccountSummary>,
        account_hash: String,
        tx: mpsc::Sender<DataMsg>,
        cmd_rx: mpsc::Receiver<AppCommand>,
        lot_store: TaxLotStore,
        lot_store_path: String,
    ) -> Self {
        Self {
            auth,
            accounts,
            account_hash,
            tx,
            cmd_rx,
            lot_store,
            lot_store_path,
            switching: false,
        }
    }

    pub async fn run(mut self) {
        loop {
            info!("data_service: connecting");
            self.tx
                .send(DataMsg::StreamerStatus(StreamerStatus::Connecting))
                .await
                .ok();
            match self.run_once().await {
                Ok(()) => {
                    info!("data_service: clean disconnect — reconnecting");
                }
                Err(e) => {
                    error!("data_service: error — {e} — reconnecting in 5s");
                    self.tx
                        .send(DataMsg::Error(format!("Stream: {e}")))
                        .await
                        .ok();
                }
            }
            if self.switching {
                // Account switch: the next loop iteration will emit Connecting
                // immediately, so we skip both the Reconnecting flash and the
                // 5s backoff for a near-instant swap.
                self.switching = false;
            } else {
                self.tx
                    .send(DataMsg::StreamerStatus(StreamerStatus::Reconnecting))
                    .await
                    .ok();
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }

    /// Look up the user-visible account number for a given hash.
    fn number_for_hash(&self, hash: &str) -> Option<&str> {
        self.accounts
            .iter()
            .find(|a| a.hash == hash)
            .map(|a| a.number.as_str())
    }

    async fn run_once(&mut self) -> Result<()> {
        let client = SchwabClient::new(self.auth.clone())
            .await
            .map_err(|e| anyhow!("auth failed: {e}"))?;

        let mut stream = SchwabStream::connect(&client).await?;

        self.tx
            .send(DataMsg::StreamerStatus(StreamerStatus::Live))
            .await
            .ok();
        info!("data_service: live");

        // Initial REST fetch — positions, orders, quotes first for fast startup.
        let mut pos_symbols = fetch_positions(&client, &self.account_hash, &self.tx).await;
        let (mut ord_symbols, ord_methods) =
            fetch_orders(&client, &self.account_hash, &self.tx).await;
        // Recover lot methods from the orders so later lot rebuilds can attribute
        // sells correctly (transactions don't report the method).
        if self.lot_store.record_order_methods(ord_methods) {
            self.lot_store.rebuild();
            self.save_and_send().await;
        }

        let mut cmd_symbols: HashSet<String> = HashSet::new();
        let mut symbols: HashSet<String> = &pos_symbols | &ord_symbols;
        info!(
            "data_service: seeded {} symbols ({} pos, {} ord)",
            symbols.len(),
            pos_symbols.len(),
            ord_symbols.len()
        );

        // Seed quotes cache from REST and send initial snapshot
        let mut quotes_cache: HashMap<String, Quote> = if !symbols.is_empty() {
            let refs: Vec<&str> = symbols.iter().map(String::as_str).collect();
            match client.get_quotes(&refs).await {
                Ok(q) => {
                    self.tx.send(DataMsg::Quotes(q.clone())).await.ok();
                    q
                }
                Err(e) => {
                    self.tx.send(DataMsg::Error(e.to_string())).await.ok();
                    HashMap::new()
                }
            }
        } else {
            HashMap::new()
        };

        // Subscribe to live data
        if !symbols.is_empty() {
            info!(
                "data_service: subscribing LEVELONE_EQUITIES for {:?}",
                symbols
            );
            stream.subscribe_equities(&symbols).await?;
        }
        stream.subscribe_acct_activity().await?;

        // Background result channel — startup lot loading and account activity
        // refreshes send results here so the event loop stays responsive.
        let (bg_tx, mut bg_rx) = mpsc::channel::<BackgroundResult>(8);

        // Spawn transaction fetch in the background so the event loop starts
        // immediately.  On startup we fetch both Trade and ReceiveAndDeliver
        // so we can detect corporate actions (splits, mergers, name changes)
        // that happened while offline.
        {
            let bg_tx = bg_tx.clone();
            let client = client.clone();
            let account = self.account_hash.clone();
            let sync_date = self.lot_store.last_sync_date.clone();
            tokio::spawn(async move {
                let (trades, rd) = tokio::join!(
                    fetch_transactions_since(
                        &client,
                        &account,
                        &sync_date,
                        tr::TransactionType::Trade
                    ),
                    fetch_transactions_since(
                        &client,
                        &account,
                        &sync_date,
                        tr::TransactionType::ReceiveAndDeliver
                    ),
                );
                let _ = bg_tx.send(BackgroundResult::StartupLots(trades, rd)).await;
            });
        }

        // Event loop
        loop {
            tokio::select! {
                // Handle results from background tasks (lot loading, activity refresh).
                Some(result) = bg_rx.recv() => {
                    match result {
                        BackgroundResult::StartupLots(trades, rd_txns) => {
                            info!("data_service: deferred lot load — {} trades, {} receive_and_deliver", trades.len(), rd_txns.len());

                            // Detect corporate actions from ReceiveAndDeliver transactions.
                            let new_ops = crate::app::lots::detect_operations(&rd_txns, &self.lot_store.applied_activity_ids);
                            if !new_ops.is_empty() {
                                info!("data_service: detected {} new operations", new_ops.len());
                                for op in &new_ops {
                                    for &id in &op.source_activity_ids {
                                        self.lot_store.applied_activity_ids.insert(id);
                                    }
                                }
                                self.lot_store.operations.extend(new_ops);
                                self.lot_store.operations.sort_by_key(|o| o.date);
                            }

                            self.apply_transactions(&trades).await;

                            // Rebuild so newly detected operations interleave correctly
                            // with the full transaction history.
                            if !rd_txns.is_empty() {
                                self.lot_store.rebuild();
                                self.save_and_send().await;
                            }
                        }
                        BackgroundResult::ActivityRefresh { ord_symbols: new_ord, ord_methods, pos_symbols: new_pos, txns } => {
                            ord_symbols = new_ord;
                            if let Some(new_pos) = new_pos {
                                pos_symbols = new_pos;
                            }
                            // Record recovered lot methods before applying the new
                            // trades so the resulting sells attribute correctly.
                            let learned = self.lot_store.record_order_methods(ord_methods);
                            if !txns.is_empty() {
                                self.apply_transactions(&txns).await;
                            }
                            // A newly-learned method may also re-attribute trades
                            // applied in an earlier cycle — rebuild to reconcile.
                            if learned {
                                self.lot_store.rebuild();
                                self.save_and_send().await;
                            }
                            let new_symbols = &(&pos_symbols | &ord_symbols) | &cmd_symbols;
                            if new_symbols != symbols {
                                info!("data_service: symbol set changed to {:?}", new_symbols);
                                symbols = new_symbols;
                                if !symbols.is_empty() {
                                    stream.subscribe_equities(&symbols).await?;
                                }
                            }
                        }
                    }
                }

                event = stream.next_event() => {
                    match event? {
                        Some(StreamEvent::EquityTicks(ticks)) => {
                            for tick in &ticks {
                                apply_equity_tick(tick, &mut quotes_cache);
                            }
                            self.tx.send(DataMsg::Quotes(quotes_cache.clone())).await.ok();
                        }
                        Some(StreamEvent::AccountActivity(ticks)) => {
                            let message_types: Vec<_> = ticks
                                .iter()
                                .filter_map(|t| t.message_type.as_ref())
                                .collect();
                            info!("data_service: ACCT_ACTIVITY types={:?}", message_types);

                            let needs_positions = message_types.iter().any(|t| t.triggers_position_refresh());

                            let client = client.clone();
                            let tx = self.tx.clone();
                            let acct = self.account_hash.clone();
                            let sync_date = self.lot_store.last_sync_date.clone();
                            let bg_tx = bg_tx.clone();
                            tokio::spawn(async move {
                                let (ord_symbols, ord_methods) = fetch_orders(&client, &acct, &tx).await;
                                let (pos_symbols, txns) = if needs_positions {
                                    // todo: look into this flow. we should probably use the 2 years to account for the lots we can and then do nothing else
                                    // or maybe we should just find out how many of their shares are long term and how many are short term. and then we can do the current price
                                    // split into long term and short term all at the same avg price
                                    info!("data_service: fill detected — refreshing positions and transactions");
                                    let txns = fetch_transactions_since(&client, &acct, &sync_date, tr::TransactionType::Trade).await;
                                    let pos_symbols = fetch_positions(&client, &acct, &tx).await;
                                    (Some(pos_symbols), txns)
                                } else {
                                    (None, vec![])
                                };
                                let _ = bg_tx.send(BackgroundResult::ActivityRefresh { ord_symbols, ord_methods, pos_symbols, txns }).await;
                            });
                        }
                        None => return Ok(()),
                    }
                }

                Some(cmd) = self.cmd_rx.recv() => {
                    match cmd {
                        AppCommand::SearchInstruments { query, projection } => {
                            let client2 = client.clone();
                            let tx2 = self.tx.clone();
                            tokio::spawn(async move {
                                let hits = match client2
                                    .search_instruments(&query, projection)
                                    .await
                                {
                                    Ok(h) => h,
                                    Err(e) => {
                                        warn!("search_instruments failed for '{query}': {e}");
                                        vec![]
                                    }
                                };
                                // If there's an exact match for the query, tell the TUI to add it to it's indicies.
                                if let Some((sym, desc)) = hits.iter().find(|(s, _)| *s == query) {
                                    tx2.send(DataMsg::IndexSymbol {
                                        symbol: sym.clone(),
                                        description: desc.clone(),
                                    }).await.ok();
                                }
                                tx2.send(DataMsg::InstrumentSuggestions { query, hits }).await.ok();
                            });
                        }
                        AppCommand::SubscribeToQuote(sym) => {
                            if cmd_symbols.insert(sym.clone()) {
                                info!("data_service: subscribing command symbol {sym}");
                                let new_symbols = &(&pos_symbols | &ord_symbols) | &cmd_symbols;
                                if new_symbols != symbols {
                                    symbols = new_symbols;
                                    if !symbols.is_empty() {
                                        stream.subscribe_equities(&symbols).await?;
                                    }
                                }
                            }
                        }

                        AppCommand::ClearCmdSymbols => {
                            let base = &pos_symbols | &ord_symbols;
                            let to_remove: HashSet<String> = &cmd_symbols - &base;
                            cmd_symbols.clear();
                            if !to_remove.is_empty() {
                                info!("data_service: unsubscribing command symbols {:?}", to_remove);
                                for s in &to_remove {
                                    quotes_cache.remove(s);
                                    symbols.remove(s);
                                }
                                stream.unsubscribe_equities(&to_remove).await?;
                            }
                        }
                        AppCommand::PlaceOrder { symbol, instruction, quantity, limit_price, tax_lot_method } => {
                            let client2 = client.clone();
                            let tx2 = self.tx.clone();
                            let acct = self.account_hash.clone();
                            tokio::spawn(async move {
                                let spec = InstrumentSpec::Equity { symbol };
                                let api_method = tax_lot_method.map(|m| m.to_api_method());
                                let result = client2
                                    .place_order(&acct, spec, instruction, quantity, limit_price, api_method)
                                    .await
                                    .map_err(|e| e.to_string());
                                tx2.send(DataMsg::OrderResult("Order placed".to_string(), result)).await.ok();
                            });
                        }
                        AppCommand::CancelOrder { order_id } => {
                            let client2 = client.clone();
                            let tx2 = self.tx.clone();
                            let acct = self.account_hash.clone();
                            tokio::spawn(async move {
                                let result = client2
                                    .cancel_order(&acct, order_id)
                                    .await
                                    .map_err(|e| e.to_string());
                                tx2.send(DataMsg::OrderResult("Order canceled".to_string(), result)).await.ok();
                            });
                        }
                        AppCommand::ReplaceOrder { order_id, symbol, instruction, quantity, limit_price, tax_lot_method } => {
                            let client2 = client.clone();
                            let tx2 = self.tx.clone();
                            let acct = self.account_hash.clone();
                            tokio::spawn(async move {
                                let spec = InstrumentSpec::Equity { symbol };
                                let api_method = tax_lot_method.map(|m| m.to_api_method());
                                let result = client2
                                    .replace_order(&acct, order_id, spec, instruction, quantity, limit_price, api_method)
                                    .await
                                    .map_err(|e| e.to_string());
                                tx2.send(DataMsg::OrderResult("Order replaced".to_string(), result)).await.ok();
                            });
                        }
                        AppCommand::LotStore(cmd) => {
                            self.handle_lot_command(cmd);
                            self.save_and_send().await;
                        }
                        AppCommand::SwitchAccount(new_hash) => {
                            if new_hash == self.account_hash {
                                continue;
                            }
                            if self.number_for_hash(&new_hash).is_none() {
                                warn!("SwitchAccount: unknown hash {new_hash}");
                                continue;
                            }
                            // Save the current account's lot store synchronously so the
                            // swap can't race a background save against a concurrent write.
                            if let Err(e) = self.lot_store.save(&self.lot_store_path) {
                                error!("failed to save tax lots before switch: {e}");
                            }
                            let new_number = self
                                .number_for_hash(&new_hash)
                                .map(str::to_string)
                                .unwrap_or_else(|| "<unknown>".to_string());
                            info!("data_service: switching to account {new_number}");
                            self.account_hash = new_hash.clone();
                            self.lot_store_path = lot_store_path_for(&new_hash);
                            self.lot_store = TaxLotStore::load(&self.lot_store_path);
                            // Persist the new active account so next launch
                            // opens here. Best-effort — failure logs but
                            // doesn't abort the switch.
                            let new_prefs = AppPrefs {
                                last_account_hash: Some(new_hash.clone()),
                            };
                            if let Err(e) = new_prefs.save() {
                                warn!("failed to save app prefs: {e}");
                            }
                            self.send_lot_store().await;
                            self.tx
                                .send(DataMsg::AccountSwitched { hash: new_hash })
                                .await
                                .ok();
                            self.switching = true;
                            // Drop the websocket and let the outer loop re-seed.
                            return Ok(());
                        }
                    }
                }
            }
        }
    }

    /// Apply transactions to the lot store, save, and send updated store to TUI.
    async fn apply_transactions(&mut self, txns: &[schwab_trader::types::Transaction]) {
        let mut sorted: Vec<&schwab_trader::types::Transaction> = txns.iter().collect();
        sorted.sort_by_key(|t| t.time.or(t.trade_date));

        let mut modified = false;
        for txn in &sorted {
            if self.lot_store.apply_transaction(txn) {
                modified = true;
            }
        }
        if modified {
            if let Some(latest) = sorted.last().and_then(|t| t.trade_date) {
                self.lot_store.last_sync_date =
                    Some(latest.format("%Y-%m-%dT%H:%M:%S").to_string());
            }
            self.save_and_send().await;
        } else {
            // Always send current state so TUI has data on first load.
            self.send_lot_store().await;
        }
    }

    fn handle_lot_command(&mut self, cmd: LotCommand) {
        use crate::app::lots::{AppliedTransaction, TradeAction};

        match cmd {
            LotCommand::ConfirmOperation(idx) => {
                if let Some(op) = self.lot_store.operations.get_mut(idx) {
                    op.confirmed = !op.confirmed;
                }
            }
            LotCommand::CycleLotMethod(idx) => {
                let default_method = self.lot_store.default_lot_method;
                if let Some(txn) = self.lot_store.applied_transactions.get_mut(idx) {
                    txn.lot_method = Some(txn.lot_method.unwrap_or(default_method).next());
                    self.lot_store.rebuild();
                }
            }
            LotCommand::CycleDefaultLotMethod => {
                self.lot_store.default_lot_method = self.lot_store.default_lot_method.next();
                self.lot_store.rebuild();
            }
            LotCommand::EditOperation { op_index, kind } => {
                if let Some(op) = self.lot_store.operations.get_mut(op_index) {
                    op.kind = kind;
                    self.lot_store.rebuild();
                }
            }
            LotCommand::EditTransaction {
                txn_index,
                symbol,
                quantity,
                price,
                lot_method,
            } => {
                if let Some(txn) = self.lot_store.applied_transactions.get_mut(txn_index) {
                    txn.symbol = symbol;
                    txn.quantity = quantity;
                    txn.price = price;
                    txn.net_amount = if txn.action == TradeAction::Buy {
                        -(price * quantity)
                    } else {
                        price * quantity
                    };
                    txn.lot_method = lot_method;
                    self.lot_store.rebuild();
                }
            }
            LotCommand::DeleteOperation(idx) => {
                if idx < self.lot_store.operations.len() {
                    let ids = self.lot_store.operations[idx].source_activity_ids.clone();
                    for id in &ids {
                        self.lot_store.applied_activity_ids.remove(id);
                    }
                    self.lot_store.operations.remove(idx);
                    self.lot_store.rebuild();
                }
            }
            LotCommand::DeleteTransaction(idx) => {
                if idx < self.lot_store.applied_transactions.len() {
                    let id = self.lot_store.applied_transactions[idx].activity_id;
                    self.lot_store.applied_activity_ids.remove(&id);
                    self.lot_store.applied_transactions.remove(idx);
                    self.lot_store.rebuild();
                }
            }
            LotCommand::AddOperation(op) => {
                self.lot_store.operations.push(op);
                self.lot_store.operations.sort_by_key(|o| o.date);
                self.lot_store.rebuild();
            }
            LotCommand::AddTransaction {
                date,
                symbol,
                action,
                quantity,
                price,
            } => {
                let activity_id = self
                    .lot_store
                    .applied_activity_ids
                    .iter()
                    .copied()
                    .filter(|&id| id < 0)
                    .min()
                    .unwrap_or(0)
                    - 1;
                let net_amount = if action == TradeAction::Buy {
                    -(price * quantity)
                } else {
                    price * quantity
                };
                self.lot_store
                    .applied_transactions
                    .push(AppliedTransaction {
                        date,
                        symbol,
                        action,
                        quantity,
                        price,
                        net_amount,
                        activity_id,
                        lot_method: None,
                        order_id: None,
                        trade_datetime: None,
                    });
                self.lot_store.applied_activity_ids.insert(activity_id);
                self.lot_store.applied_transactions.sort_by_key(|t| t.date);
                self.lot_store.rebuild();
            }
            LotCommand::Rebuild => {
                self.lot_store.rebuild();
            }
        }
    }

    async fn save_and_send(&self) {
        self.save_lots();
        self.send_lot_store().await;
    }

    async fn send_lot_store(&self) {
        self.tx
            .send(DataMsg::LotStore(Box::new(self.lot_store.clone())))
            .await
            .ok();
    }

    fn save_lots(&self) {
        let store = self.lot_store.clone();
        let path = self.lot_store_path.clone();
        tokio::task::spawn_blocking(move || {
            if let Err(e) = store.save(&path) {
                tracing::error!("failed to save tax lots to {path}: {e}");
            }
        });
    }
}

// ── Quote cache ─────────────────────────────────────────────────────────────────

use crate::schwab::streamer_types::LevelOneEquityTick;

fn apply_equity_tick(tick: &LevelOneEquityTick, cache: &mut HashMap<String, Quote>) {
    let q = cache.entry(tick.key.clone()).or_insert_with(|| Quote {
        symbol: tick.key.clone(),
        last_price: 0.0,
        close_price: 0.0,
        bid_price: 0.0,
        ask_price: 0.0,
        bid_size: 0,
        ask_size: 0,
        open_price: 0.0,
        high_price: 0.0,
        low_price: 0.0,
        net_change: 0.0,
        net_percent_change: 0.0,
        volume: 0,
        mark: 0.0,
        week_52_high: 0.0,
        week_52_low: 0.0,
    });

    if let Some(v) = tick.bid_price {
        q.bid_price = v;
    }
    if let Some(v) = tick.ask_price {
        q.ask_price = v;
    }
    if let Some(v) = tick.last_price {
        q.last_price = v;
    }
    if let Some(v) = tick.bid_size {
        q.bid_size = v;
    }
    if let Some(v) = tick.ask_size {
        q.ask_size = v;
    }
    if let Some(v) = tick.volume {
        q.volume = v;
    }
    if let Some(v) = tick.high_price {
        q.high_price = v;
    }
    if let Some(v) = tick.low_price {
        q.low_price = v;
    }
    if let Some(v) = tick.close_price {
        q.close_price = v;
    }
    if let Some(v) = tick.open_price {
        q.open_price = v;
    }
    if let Some(v) = tick.net_change {
        q.net_change = v;
        if q.close_price > 0.0 {
            q.net_percent_change = v / q.close_price * 100.0;
        }
    }
    if let Some(v) = tick.mark {
        q.mark = v;
    }
}

// ── REST helpers ────────────────────────────────────────────────────────────────

async fn fetch_positions(
    client: &SchwabClient,
    account_hash: &str,
    tx: &mpsc::Sender<DataMsg>,
) -> HashSet<String> {
    match client.get_positions(account_hash).await {
        Ok((positions, account_info)) => {
            let symbols = positions.iter().map(|p| p.symbol.clone()).collect();
            tx.send(DataMsg::Positions(positions)).await.ok();
            tx.send(DataMsg::AccountInfo(account_info)).await.ok();
            symbols
        }
        Err(e) => {
            tx.send(DataMsg::Error(e.to_string())).await.ok();
            HashSet::new()
        }
    }
}

/// Fetch transactions since the last sync datetime.
/// If no sync date exists, does a full 2-year historical fetch.
fn sync_start_date(last_sync_date: &Option<String>) -> chrono::DateTime<chrono::Utc> {
    let now = chrono::Utc::now();
    match last_sync_date {
        Some(date_str) => chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S")
            .map(|dt| dt.and_utc())
            .or_else(|_| {
                chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                    .map(|d| d.and_hms_opt(0, 0, 0).unwrap().and_utc())
            })
            .unwrap_or_else(|_| {
                warn!("bad last_sync_date '{date_str}' — doing full fetch");
                now - chrono::Months::new(24)
            }),
        None => now - chrono::Months::new(24),
    }
}

async fn fetch_transactions_since(
    client: &SchwabClient,
    account_hash: &str,
    last_sync_date: &Option<String>,
    transaction_type: schwab_trader::types::TransactionType,
) -> Vec<schwab_trader::types::Transaction> {
    let start = sync_start_date(last_sync_date);
    let end = chrono::Utc::now();

    info!(
        "fetch_transactions({:?}): {} -> {} (incremental={})",
        transaction_type,
        start.format("%Y-%m-%dT%H:%M:%S"),
        end.format("%Y-%m-%dT%H:%M:%S"),
        last_sync_date.is_some()
    );

    match client
        .get_transactions_by_type(account_hash, start, end, transaction_type)
        .await
    {
        Ok(txns) => txns,
        Err(e) => {
            warn!("fetch_transactions({:?}) failed: {e}", transaction_type);
            vec![]
        }
    }
}

/// How many of the most-recent orders to subscribe to for live quotes.
const ORDER_SUBSCRIBE_LIMIT: usize = 20;

async fn fetch_orders(
    client: &SchwabClient,
    account_hash: &str,
    tx: &mpsc::Sender<DataMsg>,
) -> (
    HashSet<String>,
    Vec<(i64, crate::schwab::LotSelectionMethod)>,
) {
    match client.get_orders(account_hash).await {
        Ok(orders) => {
            // Only subscribe to symbols from the N most recent orders.
            let syms = orders
                .iter()
                .take(ORDER_SUBSCRIBE_LIMIT)
                .map(|o| o.symbol.clone())
                .collect();
            // Recover tax lot methods (the orders endpoint echoes them; the
            // transactions API does not — see SCHWAB_API_ISSUES.md #5).
            let methods = TaxLotStore::order_method_pairs(&orders);
            tx.send(DataMsg::Orders(orders)).await.ok();
            (syms, methods)
        }
        Err(e) => {
            tx.send(DataMsg::Error(e.to_string())).await.ok();
            (HashSet::new(), Vec::new())
        }
    }
}
