use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

use anyhow::{Context, Result};
use tokio::sync::mpsc;

use self::lots::TaxLotStore;
use chrono::NaiveDate;

use crate::schwab::{
    AccountInfo, AccountSummary, AssetType, DataService, Instruction, LotSelectionMethod, Order,
    Position, Quote, SchwabAuth, SchwabClient, SellLeg, TaxLot, is_long_term,
};
use crate::tui::command::{CommandMode, ListNav};

pub mod lots;
pub mod operations;
pub mod seed;
mod symbols;
use self::symbols::{index_symbol, load_symbols};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Page {
    Positions,
    Orders,
    /// Viewing tax lots for a specific symbol.
    LotDetail(String),
    /// Operations (corporate actions) management page.
    Operations,
}

/// Messages sent from the background task to the TUI.
pub enum DataMsg {
    Positions(Vec<Position>),
    Quotes(HashMap<String, Quote>),
    Orders(Vec<Order>),
    AccountInfo(AccountInfo),
    StreamerStatus(StreamerStatus),
    Error(String),
    /// Reply to a PlaceOrder / CancelOrder / ReplaceOrder command.
    /// First field is a success label (e.g. "Order placed"); second is the result.
    OrderResult(String, Result<(), String>),
    /// Response to SearchInstruments: (symbol, description) pairs for the given query.
    InstrumentSuggestions {
        query: String,
        hits: Vec<(String, String)>,
    },
    /// Updated lot store from the backend (after transaction processing or mutation).
    LotStore(Box<TaxLotStore>),
    /// A new symbol to index into the local autocomplete DB.
    IndexSymbol {
        symbol: String,
        description: String,
    },
    /// Backend has switched to a new account. TUI clears stale per-account state
    /// (positions, orders, account info, quotes) and updates the active hash;
    /// fresh data arrives via subsequent Positions/Orders/AccountInfo messages.
    AccountSwitched {
        hash: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum StreamerStatus {
    #[default]
    Connecting,
    Live,
    Reconnecting,
}

/// Commands sent from the TUI to the background task.
pub enum AppCommand {
    SubscribeToQuote(String),
    /// Search for instruments via the API. Response arrives as InstrumentSuggestions.
    SearchInstruments {
        query: String,
        projection: schwab_marketdata::types::GetInstrumentsProjection,
    },
    ClearCmdSymbols,
    PlaceOrder {
        symbol: String,
        instruction: Instruction,
        quantity: f64,
        limit_price: Option<f64>,
        /// Tax lot selection method for sells (auto-selected per leg). `None`
        /// for buys or when no lots are tracked.
        tax_lot_method: Option<LotSelectionMethod>,
    },
    CancelOrder {
        order_id: i64,
    },
    ReplaceOrder {
        order_id: i64,
        symbol: String,
        instruction: Instruction,
        quantity: f64,
        limit_price: Option<f64>,
        tax_lot_method: Option<LotSelectionMethod>,
    },
    /// Mutate the lot store (operations page actions).
    LotStore(LotCommand),
    /// Switch the active account. Backend saves the current lot store, swaps
    /// to the new account hash + lot file, and triggers a re-seed.
    SwitchAccount(String),
}

/// Commands from the TUI to mutate the lot store.
pub enum LotCommand {
    ConfirmOperation(usize),
    CycleLotMethod(usize),
    /// Cycle the account-level default lot method (applies to sells with no
    /// explicit per-transaction method).
    CycleDefaultLotMethod,
    EditOperation {
        op_index: usize,
        kind: lots::OperationKind,
    },
    EditTransaction {
        txn_index: usize,
        symbol: String,
        quantity: f64,
        price: f64,
        lot_method: Option<LotSelectionMethod>,
    },
    DeleteOperation(usize),
    DeleteTransaction(usize),
    AddOperation(lots::Operation),
    AddTransaction {
        date: NaiveDate,
        symbol: String,
        action: lots::TradeAction,
        quantity: f64,
        price: f64,
    },
    Rebuild,
}

#[derive(Default)]
pub struct PositionsState {
    items: Vec<Position>,
    selected_idx: usize,
    pub filter: String,
    pub filter_active: bool,
    pub show_accumulation: bool,
    pub last_refresh: Option<Instant>,
    /// When true, row order is frozen — data updates in place but no re-sorting.
    pub sort_locked: bool,
    /// When true, show the aggregate account-wide P&L panel (short/long-term by year).
    pub show_pnl: bool,
}

impl ListNav for PositionsState {
    fn item_count(&self) -> usize {
        match self.active_filter() {
            None => self.items.len(),
            Some(f) => self.items.iter().filter(|p| p.symbol.contains(&f)).count(),
        }
    }

    fn selected_idx(&self) -> usize {
        self.selected_idx
    }

    fn selected_idx_mut(&mut self) -> &mut usize {
        &mut self.selected_idx
    }
}

impl PositionsState {
    pub fn items(&self) -> &[Position] {
        &self.items
    }

    pub fn set_items(&mut self, items: Vec<Position>) {
        self.items = items;
        self.clamp_selection();
    }

    /// Replace items while preserving the existing row order for symbols that
    /// remain present. New symbols are appended at the end. Used when the
    /// positions page is sort-locked so a refresh doesn't shuffle the view.
    pub fn merge_items(&mut self, items: Vec<Position>) {
        let mut by_symbol: HashMap<String, Position> =
            items.into_iter().map(|p| (p.symbol.clone(), p)).collect();
        let mut merged = Vec::with_capacity(by_symbol.len());
        for old in &self.items {
            if let Some(p) = by_symbol.remove(&old.symbol) {
                merged.push(p);
            }
        }
        merged.extend(by_symbol.into_values());
        self.items = merged;
        self.clamp_selection();
    }

    pub fn refresh_text(&self) -> Option<String> {
        self.last_refresh
            .map(|t| format!("positions {}s ago", t.elapsed().as_secs()))
    }

    pub fn held_qty(&self, symbol: &str) -> Option<f64> {
        self.items
            .iter()
            .find(|p| p.symbol == symbol)
            .map(|p| p.quantity)
    }

    /// Uppercased filter string, or None if empty. Used by item_count and sorted_filtered_positions.
    pub fn active_filter(&self) -> Option<String> {
        if self.filter.is_empty() {
            None
        } else {
            Some(self.filter.to_uppercase())
        }
    }
}

#[derive(Default)]
pub struct OrdersState {
    items: Vec<Order>,
    selected_idx: usize,
}

impl ListNav for OrdersState {
    fn item_count(&self) -> usize {
        self.items.len()
    }

    fn selected_idx(&self) -> usize {
        self.selected_idx
    }

    fn selected_idx_mut(&mut self) -> &mut usize {
        &mut self.selected_idx
    }
}

impl OrdersState {
    pub fn items(&self) -> &[Order] {
        &self.items
    }

    pub fn selected_item(&self) -> Option<&Order> {
        self.items.get(self.selected_idx)
    }

    pub fn set_items(&mut self, orders: Vec<Order>) {
        self.items = orders;
        self.clamp_selection();
    }
}

#[derive(Default)]
pub struct StreamState {
    pub quotes: HashMap<String, Quote>,
    pub status: StreamerStatus,
}

impl StreamState {
    pub fn status_text(&self) -> &'static str {
        match self.status {
            StreamerStatus::Live => "LIVE",
            StreamerStatus::Connecting => "connecting...",
            StreamerStatus::Reconnecting => "reconnecting...",
        }
    }
}

pub struct PosMetrics {
    pub last_price: f64,
    pub day_pnl: f64,
    pub day_pnl_pct: f64,
    pub open_pnl: f64,
    pub market_value: f64,
}

/// Browser-style back/forward navigation history for pages.
#[derive(Default)]
pub struct NavHistory {
    back: Vec<Page>,
    forward: Vec<Page>,
}

impl NavHistory {
    /// Record a navigation: push the old page onto the back stack and clear forward.
    pub fn push(&mut self, previous_page: Page) {
        self.back.push(previous_page);
        self.forward.clear();
    }

    /// Go back: push current page onto forward stack, pop from back stack.
    pub fn go_back(&mut self, current_page: &mut Page) -> bool {
        if let Some(prev) = self.back.pop() {
            self.forward.push(current_page.clone());
            *current_page = prev;
            true
        } else {
            false
        }
    }

    /// Go forward: push current page onto back stack, pop from forward stack.
    pub fn go_forward(&mut self, current_page: &mut Page) -> bool {
        if let Some(next) = self.forward.pop() {
            self.back.push(current_page.clone());
            *current_page = next;
            true
        } else {
            false
        }
    }
}

/// State for the account picker overlay.
#[derive(Debug, Clone)]
pub struct AccountPickerState {
    pub selected: usize,
}

/// All state the TUI renders from.
pub struct AppState {
    pub positions: PositionsState,
    pub orders: OrdersState,
    pub stream: StreamState,
    pub account_info: Option<AccountInfo>,
    pub current_page: Page,
    pub status: String,
    pub command_mode: CommandMode,
    /// Symbol → description, used for instant prefix autocomplete.
    pub symbol_db: BTreeMap<String, String>,
    /// Lowercase word → symbols, used for name-based autocomplete.
    pub word_index: BTreeMap<String, Vec<String>>,
    /// Scroll offset for the lot detail timeline table.
    pub lot_detail_scroll: usize,
    /// Tax lot tracking store (read-only copy from backend).
    pub lot_store: TaxLotStore,
    /// Operations page TUI state.
    pub operations_state: operations::OperationsState,
    /// Browser-style back/forward page navigation history.
    pub nav_history: NavHistory,
    /// All Schwab accounts available on this login.
    pub accounts: Vec<AccountSummary>,
    /// Hash of the currently active account.
    pub current_account: String,
    /// Account picker overlay state, when open.
    pub account_picker: Option<AccountPickerState>,
}

impl AppState {
    pub fn new(
        symbol_db: BTreeMap<String, String>,
        word_index: BTreeMap<String, Vec<String>>,
        accounts: Vec<AccountSummary>,
        current_account: String,
    ) -> Self {
        Self {
            positions: PositionsState::default(),
            orders: OrdersState::default(),
            stream: StreamState::default(),
            account_info: None,
            current_page: Page::Positions,
            status: "Connecting...".to_string(),
            command_mode: CommandMode::Hidden,
            symbol_db,
            word_index,
            lot_detail_scroll: 0,
            lot_store: TaxLotStore::default(),
            operations_state: operations::OperationsState::default(),
            nav_history: NavHistory::default(),
            accounts,
            current_account,
            account_picker: None,
        }
    }

    /// Look up the full AccountSummary for the active account.
    pub fn current_account_summary(&self) -> Option<&AccountSummary> {
        self.accounts
            .iter()
            .find(|a| a.hash == self.current_account)
    }

    /// Navigate to a new page, recording the current page in the back stack.
    pub fn navigate_to(&mut self, page: Page) {
        let old = self.current_page.clone();
        self.current_page = page;
        self.nav_history.push(old);
    }

    pub fn refresh_text(&self) -> String {
        let mut text = format!("  |  {}", self.stream.status_text());
        if let Some(pos_text) = self.positions.refresh_text() {
            text.push_str(&format!("  ({})", pos_text));
        }
        text
    }

    /// Compute display metrics for a position, using streaming quote when available.
    ///
    /// Day P&L for shares bought today uses their purchase price, not yesterday's
    /// close (the position didn't exist at yesterday's close). Three-tier approach:
    ///   1. If lot quantity matches the API position, use lots to split old vs new shares.
    ///   2. If the entire position is new today (previous_session_qty == 0), use open P&L.
    ///   3. Otherwise fall back to (last - close) * quantity.
    pub fn pos_metrics(&self, pos: &Position) -> PosMetrics {
        if let Some(q) = self.stream.quotes.get(&pos.symbol) {
            let multiplier = if pos.asset_type == AssetType::Option {
                100.0
            } else {
                1.0
            };
            let open_pnl = (q.last_price - pos.average_cost) * pos.quantity * multiplier;
            let day_pnl = self.compute_day_pnl(pos, q.last_price, q.close_price, multiplier);
            PosMetrics {
                last_price: q.last_price,
                day_pnl,
                day_pnl_pct: q.net_percent_change,
                open_pnl,
                market_value: q.last_price * pos.quantity * multiplier,
            }
        } else {
            PosMetrics {
                last_price: pos.current_price,
                day_pnl: pos.day_pnl,
                day_pnl_pct: pos.day_pnl_pct,
                open_pnl: pos.open_pnl,
                market_value: pos.market_value,
            }
        }
    }

    /// Compute day P&L, accounting for shares bought today.
    fn compute_day_pnl(
        &self,
        pos: &Position,
        last_price: f64,
        close_price: f64,
        multiplier: f64,
    ) -> f64 {
        if close_price <= 0.0 {
            return pos.day_pnl;
        }

        let today = chrono::Local::now().date_naive();
        let lots = self.lot_store.lots_for(&pos.symbol);
        let lot_qty: f64 = lots.iter().map(|l| l.remaining).sum();

        // Tier 1: lots match — split old vs today's shares using lot data.
        if !lots.is_empty() && (lot_qty - pos.quantity).abs() < 0.01 {
            let mut day_pnl = 0.0;
            for lot in lots {
                if lot.open_date == today {
                    // Bought today: day P&L relative to purchase price
                    day_pnl += (last_price - lot.cost_per_share) * lot.remaining * multiplier;
                } else {
                    // Held overnight: day P&L relative to yesterday's close
                    day_pnl += (last_price - close_price) * lot.remaining * multiplier;
                }
            }
            return day_pnl;
        }

        // Tier 2: entirely new position today — day P&L equals live open P&L.
        if pos.previous_session_qty == 0.0 {
            return (last_price - pos.average_cost) * pos.quantity * multiplier;
        }

        // Tier 3: fall back to standard formula.
        (last_price - close_price) * pos.quantity * multiplier
    }

    /// Return positions with metrics, sorted by day P&L ascending and filtered.
    ///
    /// When sort is locked, rows are returned in the underlying `items` order
    /// (which `toggle_sort_lock` reorders to match the displayed P&L order at
    /// lock time, and `merge_items` preserves across refreshes).
    pub fn sorted_filtered_positions(&self) -> Vec<(&Position, PosMetrics)> {
        let mut computed: Vec<_> = self
            .positions
            .items()
            .iter()
            .map(|pos| (pos, self.pos_metrics(pos)))
            .collect();
        if let Some(filter) = self.positions.active_filter() {
            computed.retain(|(pos, _)| pos.symbol.contains(&filter));
        }
        if !self.positions.sort_locked {
            // Truncate to $100 buckets so near-equal positions don't jitter in the list.
            computed.sort_by_key(|(_, m)| (m.day_pnl / 100.0) as i64);
        }
        computed
    }

    /// Get the symbol of the currently selected (sorted+filtered) position.
    pub fn selected_position_symbol(&self) -> Option<String> {
        let positions = self.sorted_filtered_positions();
        positions
            .get(self.positions.selected_idx())
            .map(|(p, _)| p.symbol.clone())
    }

    /// Find the index of a symbol in the sorted+filtered position list.
    pub fn position_index_for_symbol(&self, symbol: &str) -> Option<usize> {
        self.sorted_filtered_positions()
            .iter()
            .position(|(p, _)| p.symbol == symbol)
    }

    /// Toggle sort lock on the positions page.
    ///
    /// When locking: reorder `items` to match the currently-displayed P&L sort,
    /// then freeze. When unlocking: sorting resumes on the next render. The
    /// selected row tracks the same ticker across the transition.
    pub fn toggle_sort_lock(&mut self) {
        let selected_sym = self.selected_position_symbol();

        if self.positions.sort_locked {
            self.positions.sort_locked = false;
        } else {
            // Snapshot the current display order into `items` itself.
            let order: Vec<String> = self
                .sorted_filtered_positions()
                .iter()
                .map(|(p, _)| p.symbol.clone())
                .collect();
            let mut by_symbol: HashMap<String, Position> =
                std::mem::take(&mut self.positions.items)
                    .into_iter()
                    .map(|p| (p.symbol.clone(), p))
                    .collect();
            let mut reordered = Vec::with_capacity(by_symbol.len() + order.len());
            for sym in &order {
                if let Some(p) = by_symbol.remove(sym) {
                    reordered.push(p);
                }
            }
            // Anything filtered out of the display still belongs in items.
            reordered.extend(by_symbol.into_values());
            self.positions.items = reordered;
            self.positions.sort_locked = true;
        }

        if let Some(sym) = selected_sym
            && let Some(idx) = self.position_index_for_symbol(&sym)
        {
            *self.positions.selected_idx_mut() = idx;
        }
    }

    pub fn apply(&mut self, msg: DataMsg) {
        match msg {
            DataMsg::Positions(positions) => {
                if self.positions.sort_locked {
                    self.positions.merge_items(positions);
                } else {
                    self.positions.set_items(positions);
                }
                self.positions.last_refresh = Some(Instant::now());
                self.status = "OK".to_string();
            }
            DataMsg::Quotes(quotes) => {
                self.stream.quotes = quotes;
            }
            DataMsg::AccountInfo(info) => {
                self.account_info = Some(info);
            }
            DataMsg::Orders(orders) => {
                self.orders.set_items(orders);
            }
            DataMsg::StreamerStatus(s) => {
                self.stream.status = s;
            }
            DataMsg::Error(e) => {
                self.status = format!("Error: {e}");
            }
            DataMsg::OrderResult(label, result) => {
                self.command_mode = CommandMode::Hidden;
                match result {
                    Ok(()) => self.status = label,
                    Err(e) => self.status = format!("Error: {e}"),
                }
            }
            DataMsg::LotStore(store) => {
                self.lot_store = *store;
                self.operations_state.rebuild_timeline(&self.lot_store);
            }
            DataMsg::IndexSymbol {
                symbol,
                description,
            } => {
                // The symbol DB is pre-loaded from static TSV files at startup, but those
                // don't cover every ticker. When the user searches for an unknown symbol and
                // the API returns an exact match, cache it so future autocomplete can find it.
                if !self.symbol_db.contains_key(&symbol) {
                    index_symbol(
                        &symbol,
                        &description,
                        &mut self.symbol_db,
                        &mut self.word_index,
                    );
                }
            }
            DataMsg::AccountSwitched { hash } => {
                self.current_account = hash;
                self.positions.set_items(vec![]);
                self.orders.set_items(vec![]);
                self.account_info = None;
                self.stream.quotes.clear();
                self.account_picker = None;
                self.status = match self.current_account_summary() {
                    Some(a) => format!("Switched to {}", a.display_label()),
                    None => "Switched account".to_string(),
                };
            }
            DataMsg::InstrumentSuggestions { query, hits } => {
                // Update suggestions display if we're still typing this symbol.
                let current_sym = if let CommandMode::Typing(ts) = &self.command_mode {
                    ts.parsed_symbol()
                } else {
                    None
                };
                if current_sym.as_deref() == Some(&query)
                    && let CommandMode::Typing(ref mut ts) = self.command_mode
                {
                    // Merge with existing suggestions (two API searches run in parallel).
                    let existing: std::collections::HashSet<String> = ts
                        .autocomplete
                        .suggestions()
                        .iter()
                        .map(|(s, _)| s.clone())
                        .collect();
                    let new_hits: Vec<_> = hits
                        .into_iter()
                        .filter(|(s, _)| !existing.contains(s))
                        .collect();
                    ts.autocomplete.extend_suggestions(new_hits);
                }
            }
        }
    }
}

/// Per-account tax lot file path. Keyed by the encrypted account hash so the
/// user-visible account number never lands on disk in a file name.
pub fn lot_store_path_for(account_hash: &str) -> String {
    format!("tax_lots_{account_hash}.json")
}

/// Path for the small app-level preferences file (last-used account, etc.).
pub const APP_PREFS_PATH: &str = "app_prefs.json";

/// Cross-run app preferences. Kept intentionally minimal so it can grow
/// without churning the auth or lot-store files.
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppPrefs {
    /// Encrypted account hash the user last had open. Stored as the hash (not
    /// the user-visible number) so the prefs file doesn't expose the account
    /// number on disk.
    #[serde(default)]
    pub last_account_hash: Option<String>,
}

impl AppPrefs {
    pub fn load() -> Self {
        match std::fs::read_to_string(APP_PREFS_PATH) {
            Ok(s) => serde_json::from_str(&s).unwrap_or_else(|e| {
                tracing::warn!("failed to parse {APP_PREFS_PATH}: {e} — using defaults");
                Self::default()
            }),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(APP_PREFS_PATH, data)?;
        Ok(())
    }
}

/// Compute how to split a sell across tax-lot-selection methods based on holding
/// periods.
///
/// Strategy: sell long-term lots first via FIFO (they already qualify for the
/// favorable long-term capital-gains rate, so realizing them first is free), and
/// take any short-term remainder via LIFO (sells the newest short-term shares,
/// giving the oldest short-term lots more time to cross the 1-year mark).
///
/// Returns one leg when the sell fits in a single holding-period bucket, or two
/// legs (long-term FIFO + short-term LIFO) when it spans both. Quantities are
/// measured against each lot's *remaining* open shares. With no tracked lots we
/// fall back to a single FIFO leg (the account/industry default).
pub fn compute_sell_legs(lots: &[TaxLot], sell_qty: f64) -> Vec<SellLeg> {
    if lots.is_empty() {
        return vec![SellLeg {
            quantity: sell_qty,
            method: LotSelectionMethod::Fifo,
        }];
    }

    let today = chrono::Local::now().date_naive();
    let long_term_qty: f64 = lots
        .iter()
        .filter(|l| l.remaining > crate::schwab::QTY_EPSILON)
        .filter(|l| is_long_term(l.open_date, today))
        .map(|l| l.remaining)
        .sum();

    if long_term_qty + crate::schwab::QTY_EPSILON >= sell_qty {
        // Entirely covered by long-term lots.
        vec![SellLeg {
            quantity: sell_qty,
            method: LotSelectionMethod::Fifo,
        }]
    } else if long_term_qty > crate::schwab::QTY_EPSILON {
        // Spans both: all long-term (FIFO) + short-term remainder (LIFO).
        vec![
            SellLeg {
                quantity: long_term_qty,
                method: LotSelectionMethod::Fifo,
            },
            SellLeg {
                quantity: sell_qty - long_term_qty,
                method: LotSelectionMethod::Lifo,
            },
        ]
    } else {
        // Entirely short-term.
        vec![SellLeg {
            quantity: sell_qty,
            method: LotSelectionMethod::Lifo,
        }]
    }
}

pub async fn run() -> Result<()> {
    let api_key = std::env::var("SCHWAB_API_KEY").context("SCHWAB_API_KEY not set")?;
    let app_secret = std::env::var("SCHWAB_APP_SECRET").context("SCHWAB_APP_SECRET not set")?;
    let token_path =
        std::env::var("SCHWAB_TOKEN_PATH").unwrap_or_else(|_| "schwab_tokens.json".to_string());

    let auth = SchwabAuth::new(api_key, app_secret, token_path);
    let client = SchwabClient::new(auth.clone())
        .await
        .context("authentication failed")?;

    let accounts: Vec<AccountSummary> = client
        .get_accounts()
        .await
        .context("failed to fetch accounts")?;
    let prefs = AppPrefs::load();
    let primary = prefs
        .last_account_hash
        .as_deref()
        .and_then(|hash| accounts.iter().find(|a| a.hash == hash).cloned())
        .or_else(|| accounts.first().cloned())
        .context("no accounts found")?;

    let lot_store_path = lot_store_path_for(&primary.hash);
    let lot_store = lots::TaxLotStore::load(&lot_store_path);

    let (tx, rx) = mpsc::channel::<DataMsg>(32);
    let (cmd_tx, cmd_rx) = mpsc::channel::<AppCommand>(32);
    let streamer_handle = tokio::spawn(
        DataService::new(
            auth,
            accounts.clone(),
            primary.hash.clone(),
            tx,
            cmd_rx,
            lot_store,
            lot_store_path,
        )
        .run(),
    );

    let (symbol_db, word_index) = load_symbols(&["stocks.tsv", "etfs.tsv"]);

    let tui_result = crate::tui::run_tui(
        AppState::new(symbol_db, word_index, accounts, primary.hash),
        rx,
        cmd_tx,
    )
    .await;

    if let Err(e) = streamer_handle.await {
        tracing::error!("Streamer task failed: {e:?}");
    }

    tui_result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lot(days_ago: i64, qty: f64) -> TaxLot {
        let open = chrono::Local::now().date_naive() - chrono::Duration::days(days_ago);
        TaxLot::new(open, qty, 10.0, 0)
    }

    #[test]
    fn sell_legs_empty_lots_fall_back_to_fifo() {
        let legs = compute_sell_legs(&[], 5.0);
        assert_eq!(
            legs,
            vec![SellLeg {
                quantity: 5.0,
                method: LotSelectionMethod::Fifo
            }]
        );
    }

    #[test]
    fn sell_legs_all_long_term_use_fifo() {
        let lots = vec![lot(400, 10.0)];
        let legs = compute_sell_legs(&lots, 6.0);
        assert_eq!(
            legs,
            vec![SellLeg {
                quantity: 6.0,
                method: LotSelectionMethod::Fifo
            }]
        );
    }

    #[test]
    fn sell_legs_all_short_term_use_lifo() {
        let lots = vec![lot(30, 10.0)];
        let legs = compute_sell_legs(&lots, 6.0);
        assert_eq!(
            legs,
            vec![SellLeg {
                quantity: 6.0,
                method: LotSelectionMethod::Lifo
            }]
        );
    }

    #[test]
    fn sell_legs_spanning_both_split_fifo_then_lifo() {
        // 4 long-term + 6 short-term shares; sell 5 → all 4 LT (FIFO) + 1 ST (LIFO).
        let lots = vec![lot(400, 4.0), lot(30, 6.0)];
        let legs = compute_sell_legs(&lots, 5.0);
        assert_eq!(
            legs,
            vec![
                SellLeg {
                    quantity: 4.0,
                    method: LotSelectionMethod::Fifo
                },
                SellLeg {
                    quantity: 1.0,
                    method: LotSelectionMethod::Lifo
                },
            ]
        );
    }

    #[test]
    fn sell_legs_fitting_in_long_term_stay_single_fifo() {
        // Sell less than the long-term quantity: single FIFO leg, no split.
        let lots = vec![lot(400, 4.0), lot(30, 6.0)];
        let legs = compute_sell_legs(&lots, 3.0);
        assert_eq!(
            legs,
            vec![SellLeg {
                quantity: 3.0,
                method: LotSelectionMethod::Fifo
            }]
        );
    }

    #[test]
    fn sell_legs_exactly_one_year_is_short_term() {
        // Exactly one year held is not yet long-term (must be strictly past
        // open + 1 year, calendar-based — see is_long_term), so this is all LIFO.
        let lots = vec![lot(365, 10.0)];
        let legs = compute_sell_legs(&lots, 5.0);
        assert_eq!(legs[0].method, LotSelectionMethod::Lifo);
    }
}
