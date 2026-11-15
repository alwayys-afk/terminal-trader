//! The MCP tools. Read-only: nothing here can place, change or cancel an
//! order.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

use chrono::{Local, NaiveDate, Utc};
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::tool::{InputResponses, RequestState};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    CallToolResponse, CallToolResult, ContentBlock, ElicitRequest, ElicitRequestParams,
    ElicitationAction, Implementation, InputRequest, InputRequiredResult, ProtocolVersion,
    ServerCapabilities, ServerConfig,
};
use rmcp::service::RequestContext;
use rmcp::{RoleServer, ServerHandler, tool, tool_handler, tool_router};
use schemars::JsonSchema;
use serde::Deserialize;
use tokio::sync::Mutex;
use tracing::{info, warn};

use super::accounts::{AccountDirectory, AccountEntry};
use super::chain::{ChainFilter, ChainFilterInput};
use super::format;
use super::login::{LoginManager, LoginOutcome, PendingLogin};
use crate::app::lot_store_path_for;
use crate::app::lots::TaxLotStore;
use crate::schwab::{AuthError, SchwabAuth, SchwabClient};

const LOGIN_REQUIRED: &str = "Not logged in to Schwab";
const MAX_QUOTE_SYMBOLS: usize = 50;
/// Schwab's orders endpoint only looks back this far.
const MAX_ORDER_LOOKBACK_DAYS: i64 = 60;

const INSTRUCTIONS: &str = "Read-only access to the user's Charles Schwab accounts through \
terminal-trader. Accounts are named by nickname (see list_accounts); account numbers are never \
shown. If a tool says the user isn't logged in, call `login`. Output is a `# title` line then \
CSV with a header row; empty cells mean unknown. Money is in dollars, IV in percent, greeks are \
per share (theta per day, vega per 1 IV point); multiply by 100 per standard option contract.";

#[derive(Clone)]
pub struct TraderServer {
    state: Arc<State>,
    tool_router: ToolRouter<Self>,
}

struct State {
    /// `Err` holds why the server can't talk to Schwab (missing credentials).
    auth: Result<SchwabAuth, String>,
    client: Mutex<Option<SchwabClient>>,
    accounts: Mutex<Option<AccountDirectory>>,
    login: LoginManager,
}

// ── Tool parameters ──────────────────────────────────────────────────────────

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct LoginParams {
    /// Wait (up to the 5-minute login window) for a login already started
    /// with `login` to finish, instead of starting a new one.
    #[serde(default)]
    pub wait: Option<bool>,
    /// Start a new login even if the saved one is still valid.
    #[serde(default)]
    pub force: Option<bool>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct AccountParams {
    /// Account nickname from list_accounts (case-insensitive; a unique prefix
    /// works). Omit for all accounts.
    #[serde(default)]
    pub account: Option<String>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct OrdersParams {
    /// Account nickname from list_accounts. Omit for all accounts.
    #[serde(default)]
    pub account: Option<String>,
    /// `open` (can still fill), `all` (default), or a Schwab status such as
    /// FILLED, CANCELED, WORKING, REJECTED, EXPIRED.
    #[serde(default)]
    pub status: Option<String>,
    /// Orders entered on or after this date (YYYY-MM-DD). Default: 7 days
    /// ago. Schwab only keeps 60 days.
    #[serde(default)]
    pub from_date: Option<String>,
    /// Orders entered on or before this date (YYYY-MM-DD). Default: now.
    #[serde(default)]
    pub to_date: Option<String>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct QuotesParams {
    /// Up to 50 symbols: tickers (AAPL), indexes ($SPX) or OSI option
    /// symbols ("AAPL  260717C00215000").
    pub symbols: Vec<String>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct LotsParams {
    /// Account nickname from list_accounts. Omit for all accounts.
    #[serde(default)]
    pub account: Option<String>,
    /// Only lots of this symbol.
    #[serde(default)]
    pub symbol: Option<String>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct ExpirationsParams {
    /// Underlying symbol, e.g. SPY or $SPX.
    pub symbol: String,
    /// Only expirations within this many days. Default: all.
    #[serde(default)]
    pub max_dte: Option<u32>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct ChainParams {
    /// Underlying symbol, e.g. SPY or $SPX.
    pub symbol: String,
    /// `call`, `put` or `all` (default).
    #[serde(default)]
    pub side: Option<String>,
    /// One expiration (YYYY-MM-DD). Overrides the other date filters.
    #[serde(default)]
    pub expiration: Option<String>,
    /// Earliest expiration (YYYY-MM-DD).
    #[serde(default)]
    pub from_date: Option<String>,
    /// Latest expiration (YYYY-MM-DD).
    #[serde(default)]
    pub to_date: Option<String>,
    /// Minimum days to expiration.
    #[serde(default)]
    pub min_dte: Option<u32>,
    /// Maximum days to expiration. With no date filter at all the window is
    /// today to 45 days out.
    #[serde(default)]
    pub max_dte: Option<u32>,
    /// Strikes on each side of the current price (Schwab's strikeCount, max
    /// 50). Default 8 unless min_strike/max_strike is given; 0 means no limit.
    #[serde(default)]
    pub strike_count: Option<u32>,
    /// Lowest strike to include.
    #[serde(default)]
    pub min_strike: Option<f64>,
    /// Highest strike to include.
    #[serde(default)]
    pub max_strike: Option<f64>,
    /// Row cap, default 120, max 400. Rows past it are cut and reported.
    #[serde(default)]
    pub max_rows: Option<u32>,
}

// ── Server ───────────────────────────────────────────────────────────────────

impl TraderServer {
    /// A server for the user's Schwab app credentials. `auth` is `Err` with a
    /// message when they aren't configured; the server still starts so the
    /// client can see its tools and that message.
    pub fn new(auth: Result<SchwabAuth, String>) -> TraderServer {
        TraderServer {
            state: Arc::new(State {
                auth,
                client: Mutex::new(None),
                accounts: Mutex::new(None),
                login: LoginManager::default(),
            }),
            tool_router: Self::tool_router(),
        }
    }

    fn auth(&self) -> Result<&SchwabAuth, String> {
        self.state.auth.as_ref().map_err(String::clone)
    }

    async fn client(&self) -> Result<SchwabClient, String> {
        let auth = self.auth()?;
        let mut slot = self.state.client.lock().await;
        if let Some(c) = slot.as_ref() {
            return Ok(c.clone());
        }
        let client = SchwabClient::new(auth.clone())
            .await
            .map_err(|e| error_text(&e))?;
        *slot = Some(client.clone());
        Ok(client)
    }

    async fn directory(
        &self,
        client: &SchwabClient,
        refresh: bool,
    ) -> Result<AccountDirectory, String> {
        let mut slot = self.state.accounts.lock().await;
        if !refresh && let Some(d) = slot.as_ref() {
            return Ok(d.clone());
        }
        let accounts = client.get_accounts().await.map_err(|e| error_text(&e))?;
        let dir = AccountDirectory::new(&accounts);
        *slot = Some(dir.clone());
        Ok(dir)
    }

    /// The client and the accounts a call refers to.
    async fn accounts_for(
        &self,
        name: Option<&str>,
    ) -> Result<(SchwabClient, Vec<AccountEntry>), String> {
        let client = self.client().await?;
        let dir = self.directory(&client, false).await?;
        let picked = dir.resolve(name)?.into_iter().cloned().collect();
        Ok((client, picked))
    }

    /// Scrub account numbers and hashes from tool output, ok or error.
    async fn finish(&self, result: Result<String, String>) -> Result<String, String> {
        let dir = self.state.accounts.lock().await;
        let Some(dir) = dir.as_ref() else {
            return result;
        };
        match result {
            Ok(text) => Ok(dir.redact(&text)),
            Err(text) => Err(dir.redact(&text)),
        }
    }

    fn auth_status_text(&self) -> Result<String, String> {
        let auth = self.auth()?;
        let now = Utc::now();
        Ok(match auth.saved_token_expiry() {
            None => format!("logged_in=false\nreason=no saved login\n{LOGIN_HINT}\n"),
            Some((_, refresh)) if refresh <= now => format!(
                "logged_in=false\nreason=login expired {} (Schwab requires a new login every 7 days)\n{LOGIN_HINT}\n",
                format::relative(refresh, now)
            ),
            Some((access, refresh)) => format!(
                "logged_in=true\naccess_token_expires={} (refreshed automatically)\nlogin_expires={} (then a new browser login is needed)\n",
                format::relative(access, now),
                format::relative(refresh, now)
            ),
        })
    }

    async fn after_login(&self, outcome: &LoginOutcome) {
        if outcome.is_ok() {
            // Rebuild the client from the new token on next use.
            *self.state.client.lock().await = None;
        }
    }

    async fn login_result_text(
        &self,
        login: &PendingLogin,
        outcome: Option<LoginOutcome>,
    ) -> String {
        match outcome {
            Some(Ok(())) => {
                self.after_login(&Ok(())).await;
                "Logged in to Schwab.".to_string()
            }
            Some(Err(e)) => format!("Login failed: {e}. Call `login` to try again."),
            None => format!(
                "Still waiting for the browser login ({} min left). Call `login` with wait=true \
                 after the user finishes, or check `auth_status`.",
                login.remaining().as_secs().div_ceil(60)
            ),
        }
    }

    async fn run_login(
        &self,
        ctx: RequestContext<RoleServer>,
        request_state: Option<String>,
        responses: Option<rmcp::model::InputResponses>,
        params: LoginParams,
    ) -> Result<CallToolResponse, String> {
        let auth = self.auth()?.clone();

        // Second round of an MRTR elicitation: the user saw the URL.
        if let Some(state) = request_state {
            let Some(login) = self.state.login.current().await.filter(|l| l.id == state) else {
                return Err("That login request has expired. Call `login` again.".to_string());
            };
            let accepted = responses
                .as_ref()
                .and_then(|r| r.values().next())
                .and_then(|v| v.get("action"))
                .and_then(|a| a.as_str())
                .is_none_or(|a| a == "accept");
            if !accepted {
                return Ok(text_result(format!(
                    "The user dismissed the login prompt. The login link stays valid for {} min: {}",
                    login.remaining().as_secs().div_ceil(60),
                    login.url
                )));
            }
            let outcome = login.wait(login.remaining()).await;
            return Ok(text_result(self.login_result_text(&login, outcome).await));
        }

        if params.wait.unwrap_or(false) {
            let Some(login) = self.state.login.current().await else {
                return Err(
                    "No login is in progress. Call `login` without wait to start one.".to_string(),
                );
            };
            if let Some(outcome) = login.outcome() {
                return Ok(text_result(
                    self.login_result_text(&login, Some(outcome)).await,
                ));
            }
            let outcome = login.wait(login.remaining()).await;
            return Ok(text_result(self.login_result_text(&login, outcome).await));
        }

        if !params.force.unwrap_or(false) {
            match auth.get_valid_token().await {
                Ok(_) => {
                    return Ok(text_result(format!(
                        "Already logged in.\n{}",
                        self.auth_status_text()?
                    )));
                }
                Err(AuthError::LoginRequired { .. }) => {}
                Err(e) => warn!("checking the saved login before a new one: {e}"),
            }
        }

        let started = self.state.login.start(&auth).await?;
        let login = started.login;
        info!(new = started.is_new, "MCP login waiting for the browser");
        let message = "Log in to Schwab in your browser. After logging in, your browser will warn \
                       that the connection to 127.0.0.1 isn't private: that's terminal-trader's \
                       own local certificate. Click Advanced, then Proceed.";

        let url_elicitation = ctx
            .client_capabilities()
            .and_then(|c| c.elicitation)
            .is_some_and(|e| e.url.is_some());
        if url_elicitation {
            let stateless = ctx
                .protocol_version()
                .is_some_and(|v| v >= ProtocolVersion::V_2026_07_28);
            let params = ElicitRequestParams::UrlElicitationParams {
                meta: None,
                message: message.to_string(),
                url: login.url.clone(),
                elicitation_id: login.id.clone(),
            };
            if stateless {
                // 2026-07-28 forbids server-to-client requests: ask the
                // client to show the URL and call us back.
                let mut requests = BTreeMap::new();
                requests.insert(
                    "schwab_login".to_string(),
                    InputRequest::Elicitation(ElicitRequest::new(params)),
                );
                return Ok(InputRequiredResult::new(Some(requests), Some(login.id.clone())).into());
            }
            match url::Url::parse(&login.url) {
                Ok(url) => {
                    match ctx
                        .peer
                        .elicit_url_with_timeout(
                            message,
                            url,
                            login.id.clone(),
                            Some(login.remaining()),
                        )
                        .await
                    {
                        Ok(ElicitationAction::Accept) => {
                            let outcome = login.wait(login.remaining()).await;
                            return Ok(text_result(self.login_result_text(&login, outcome).await));
                        }
                        Ok(_) => {
                            return Ok(text_result(format!(
                                "The user dismissed the login prompt. The login link stays valid for {} min: {}",
                                login.remaining().as_secs().div_ceil(60),
                                login.url
                            )));
                        }
                        Err(e) => warn!("URL elicitation failed, returning the URL instead: {e}"),
                    }
                }
                Err(e) => warn!("login URL didn't parse: {e}"),
            }
        }

        // No elicitation: hand the URL to the model and return while the
        // listener keeps waiting.
        let opened = if started.is_new {
            "Opened the user's browser to the Schwab login."
        } else {
            "A login is already waiting for the browser."
        };
        Ok(text_result(format!(
            "{opened} If it didn't open, give the user this link:\n{}\n\n{message}\n\nThe login \
             link works for {} min. Once the user says they're done, call `login` with wait=true \
             to confirm (or `auth_status`).",
            login.url,
            login.remaining().as_secs().div_ceil(60)
        )))
    }
}

const LOGIN_HINT: &str = "Call the `login` tool to log in.";

fn text_result(text: String) -> CallToolResponse {
    CallToolResult::success(vec![ContentBlock::text(text)]).into()
}

/// A tool error message for a failed Schwab call. A missing or expired login
/// becomes an instruction to call `login`.
fn error_text(e: &anyhow::Error) -> String {
    for cause in e.chain() {
        if let Some(AuthError::LoginRequired { reason }) = cause.downcast_ref::<AuthError>() {
            return format!("{LOGIN_REQUIRED} ({reason}). {LOGIN_HINT} Then retry.");
        }
    }
    format!("Schwab request failed: {e:#}")
}

fn parse_day(s: &str, field: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d")
        .map_err(|_| format!("{field} must be YYYY-MM-DD, got {s:?}"))
}

fn local_midnight_utc(d: NaiveDate) -> chrono::DateTime<Utc> {
    d.and_hms_opt(0, 0, 0)
        .and_then(|t| t.and_local_timezone(Local).earliest())
        .map(|t| t.with_timezone(&Utc))
        .unwrap_or_else(|| d.and_hms_opt(0, 0, 0).unwrap().and_utc())
}

#[tool_router]
impl TraderServer {
    #[tool(
        description = "Whether terminal-trader has a usable Schwab login, and when it expires. \
                       Never refreshes or prompts. Logins last 7 days.",
        annotations(read_only_hint = true, open_world_hint = false)
    )]
    async fn auth_status(&self) -> Result<String, String> {
        let mut text = self.auth_status_text()?;
        if let Some(login) = self.state.login.current().await
            && login.outcome().is_none()
            && !login.remaining().is_zero()
        {
            text.push_str(&format!(
                "login_in_progress=true (waiting for the browser, {} min left)\n",
                login.remaining().as_secs().div_ceil(60)
            ));
        }
        Ok(text)
    }

    #[tool(
        description = "Log in to Schwab: opens the user's browser to Schwab's login page and \
                       receives the redirect on https://127.0.0.1:8182. If the client supports \
                       URL elicitation the user is shown the link and this waits for the login. \
                       Otherwise it returns the link right away; show it to the user, and once \
                       they're done call login again with wait=true. Does nothing if already \
                       logged in unless force=true.",
        annotations(
            read_only_hint = false,
            destructive_hint = false,
            open_world_hint = true
        )
    )]
    async fn login(
        &self,
        ctx: RequestContext<RoleServer>,
        RequestState(request_state): RequestState,
        InputResponses(responses): InputResponses,
        Parameters(params): Parameters<LoginParams>,
    ) -> Result<CallToolResponse, String> {
        self.run_login(ctx, request_state, responses, params).await
    }

    #[tool(
        description = "The user's Schwab accounts, by nickname. Pass a nickname as `account` to \
                       the other tools. Account numbers are never shown.",
        annotations(read_only_hint = true, open_world_hint = true)
    )]
    async fn list_accounts(&self) -> Result<String, String> {
        let result = async {
            let client = self.client().await?;
            let dir = self.directory(&client, true).await?;
            let mut out = format!("# {} accounts\n", dir.entries().len());
            for e in dir.entries() {
                out.push_str(&e.label);
                out.push('\n');
            }
            Ok(out)
        }
        .await;
        self.finish(result).await
    }

    #[tool(
        description = "Positions per account. Columns: sym (OSI symbol for options), type \
                       (EQ/OPT/MF/CASH/FI), qty (negative = short), avg (average cost per \
                       share/contract-share), price, mv (market value $), day_pnl, open_pnl.",
        annotations(read_only_hint = true, open_world_hint = true)
    )]
    async fn get_positions(
        &self,
        Parameters(p): Parameters<AccountParams>,
    ) -> Result<String, String> {
        let result = async {
            let (client, accounts) = self.accounts_for(p.account.as_deref()).await?;
            let mut out = String::new();
            for e in accounts {
                match client.get_positions(&e.hash).await {
                    Ok((positions, _)) => out.push_str(&format::positions(&e.label, &positions)),
                    Err(err) => {
                        out.push_str(&format!("# {}: error: {}\n", e.label, error_text(&err)))
                    }
                }
            }
            Ok(out)
        }
        .await;
        self.finish(result).await
    }

    #[tool(
        description = "Balances per account as key=value lines: liquidation_value, cash (net of \
                       margin debit), buying_power, option_buying_power, available_funds, \
                       equity, maintenance_requirement, and more. Missing keys aren't reported \
                       for that account type.",
        annotations(read_only_hint = true, open_world_hint = true)
    )]
    async fn get_balances(
        &self,
        Parameters(p): Parameters<AccountParams>,
    ) -> Result<String, String> {
        let result = async {
            let (client, accounts) = self.accounts_for(p.account.as_deref()).await?;
            let mut out = String::new();
            for e in accounts {
                match client.get_balances(&e.hash).await {
                    Ok(b) => out.push_str(&format::balances(&e.label, &b)),
                    Err(err) => {
                        out.push_str(&format!("# {}: error: {}\n", e.label, error_text(&err)))
                    }
                }
            }
            Ok(out)
        }
        .await;
        self.finish(result).await
    }

    #[tool(
        description = "Orders per account, newest first. Default: the last 7 days, all \
                       statuses. Columns: id, entered (local time), sym, instr (BUY, \
                       SELL_TO_OPEN, ...), qty, filled, type (LIMIT, MARKET, ...), price, stop, \
                       status, tif, fill_px (average fill).",
        annotations(read_only_hint = true, open_world_hint = true)
    )]
    async fn get_orders(&self, Parameters(p): Parameters<OrdersParams>) -> Result<String, String> {
        let result = async {
            let today = Local::now().date_naive();
            let earliest = today - chrono::Duration::days(MAX_ORDER_LOOKBACK_DAYS);
            let from_day = match p.from_date.as_deref() {
                Some(s) => parse_day(s, "from_date")?,
                None => today - chrono::Duration::days(7),
            };
            let from_day = from_day.max(earliest);
            let to = match p.to_date.as_deref() {
                Some(s) => {
                    let d = parse_day(s, "to_date")?;
                    local_midnight_utc(d + chrono::Duration::days(1)).min(Utc::now())
                }
                None => Utc::now(),
            };
            let from = local_midnight_utc(from_day);
            if to <= from {
                return Err("to_date is before from_date".to_string());
            }
            let status = p.status.unwrap_or_default();
            let range = format!(
                "{from_day}..{}{}",
                to.with_timezone(&Local).date_naive(),
                if status.is_empty() {
                    String::new()
                } else {
                    format!(", status {status}")
                }
            );

            let (client, accounts) = self.accounts_for(p.account.as_deref()).await?;
            let mut out = String::new();
            for e in accounts {
                match client.get_orders_between(&e.hash, from, to).await {
                    Ok(orders) => {
                        let kept: Vec<_> = orders
                            .into_iter()
                            .filter(|o| format::status_matches(&status, &o.status))
                            .collect();
                        out.push_str(&format::orders(&e.label, &kept, &range));
                    }
                    Err(err) => {
                        out.push_str(&format!("# {}: error: {}\n", e.label, error_text(&err)))
                    }
                }
            }
            Ok(out)
        }
        .await;
        self.finish(result).await
    }

    #[tool(
        description = "Quotes for up to 50 symbols. Columns: sym, last, bid, ask, mark, chg, \
                       chg_pct, vol, open, high, low, prev_close, wk52_hi, wk52_lo.",
        annotations(read_only_hint = true, open_world_hint = true)
    )]
    async fn get_quotes(&self, Parameters(p): Parameters<QuotesParams>) -> Result<String, String> {
        let result = async {
            let mut symbols: Vec<String> = Vec::new();
            for s in &p.symbols {
                let s = s.trim().to_uppercase();
                if !s.is_empty() && !symbols.contains(&s) {
                    symbols.push(s);
                }
            }
            if symbols.is_empty() {
                return Err("symbols is empty".to_string());
            }
            if symbols.len() > MAX_QUOTE_SYMBOLS {
                return Err(format!("at most {MAX_QUOTE_SYMBOLS} symbols per call"));
            }
            let client = self.client().await?;
            let refs: Vec<&str> = symbols.iter().map(String::as_str).collect();
            let quotes = client.get_quotes(&refs).await.map_err(|e| error_text(&e))?;
            Ok(format::quotes(&symbols, &quotes))
        }
        .await;
        self.finish(result).await
    }

    #[tool(
        description = "Open tax lots (stocks/ETFs; options aren't tracked) from terminal-trader's \
                       lot store, priced with live quotes. Lots are as of the last sync, which the \
                       terminal-trader app does when it runs. Columns: sym, opened, qty \
                       (remaining shares), cost_sh, basis, price, unreal ($), unreal_pct, term \
                       (ST/LT), days_to_lt (days until long-term). Ends with ST/LT unrealized \
                       totals.",
        annotations(read_only_hint = true, open_world_hint = true)
    )]
    async fn get_lots(&self, Parameters(p): Parameters<LotsParams>) -> Result<String, String> {
        let result = async {
            let (client, accounts) = self.accounts_for(p.account.as_deref()).await?;
            let filter = p.symbol.as_deref().map(|s| s.trim().to_uppercase());

            let mut stores = Vec::new();
            for e in accounts {
                let path = lot_store_path_for(&e.hash);
                let store = tokio::task::spawn_blocking(move || TaxLotStore::load(&path))
                    .await
                    .map_err(|e| format!("reading the lot store failed: {e}"))?;
                stores.push((e.label, store));
            }

            let mut symbols: HashSet<&str> = HashSet::new();
            for (_, store) in &stores {
                for (sym, lots) in &store.lots {
                    if filter.as_deref().is_none_or(|f| f == sym)
                        && lots
                            .iter()
                            .any(|l| l.remaining > crate::schwab::QTY_EPSILON)
                    {
                        symbols.insert(sym);
                    }
                }
            }
            let mut prices: HashMap<String, f64> = HashMap::new();
            let mut price_note = String::new();
            let symbols: Vec<&str> = symbols.into_iter().collect();
            for chunk in symbols.chunks(100) {
                match client.get_quotes(chunk).await {
                    Ok(quotes) => {
                        for (sym, q) in quotes {
                            let p = if q.last_price > 0.0 {
                                q.last_price
                            } else {
                                q.mark
                            };
                            prices.insert(sym, p);
                        }
                    }
                    Err(e) => {
                        price_note = format!("prices unavailable: {}\n", error_text(&e));
                    }
                }
            }

            let today = Local::now().date_naive();
            let mut out = price_note;
            for (label, store) in &stores {
                let lots = store
                    .lots
                    .iter()
                    .filter(|(sym, _)| filter.as_deref().is_none_or(|f| f == sym.as_str()));
                let rows = format::lot_rows(lots, &prices, today);
                out.push_str(&format::lots(label, &rows, store.last_sync_date.as_deref()));
            }
            Ok(out)
        }
        .await;
        self.finish(result).await
    }

    #[tool(
        description = "Listed option expirations for an underlying. Columns: date, dte, kind (W \
                       weekly, M standard 3rd-Friday monthly, Q quarterly, EOM end of month; \
                       'nonstd' marks adjusted roots).",
        annotations(read_only_hint = true, open_world_hint = true)
    )]
    async fn get_expirations(
        &self,
        Parameters(p): Parameters<ExpirationsParams>,
    ) -> Result<String, String> {
        let result = async {
            let symbol = p.symbol.trim().to_uppercase();
            if symbol.is_empty() {
                return Err("symbol is required".to_string());
            }
            let client = self.client().await?;
            let mut list = client
                .get_expirations(&symbol)
                .await
                .map_err(|e| error_text(&e))?;
            let total = list.len();
            if let Some(max) = p.max_dte {
                list.retain(|e| e.days_to_exp <= max as i32);
            }
            Ok(format::expirations(&symbol, &list, total - list.len()))
        }
        .await;
        self.finish(result).await
    }

    #[tool(
        description = "A filtered option chain. Defaults: expirations from today to 45 days out, \
                       8 strikes each side of the money, calls and puts, at most 120 rows (a \
                       TRUNCATED line says how many more matched). Use get_expirations first to \
                       pick dates. Columns: exp, dte, kind, side (C/P), strike, bid, ask, mark, \
                       last, vol, oi, iv (%), delta, gamma, theta ($/day per share), vega ($ per \
                       IV point per share), theo (Schwab's theoretical value), itm (1 if in the \
                       money).",
        annotations(read_only_hint = true, open_world_hint = true)
    )]
    async fn get_option_chain(
        &self,
        Parameters(p): Parameters<ChainParams>,
    ) -> Result<String, String> {
        let result = async {
            let input = ChainFilterInput {
                symbol: &p.symbol,
                side: p.side.as_deref(),
                expiration: p.expiration.as_deref(),
                from_date: p.from_date.as_deref(),
                to_date: p.to_date.as_deref(),
                min_dte: p.min_dte,
                max_dte: p.max_dte,
                strike_count: p.strike_count,
                min_strike: p.min_strike,
                max_strike: p.max_strike,
                max_rows: p.max_rows,
            };
            let filter = ChainFilter::from_input(&input, Local::now().date_naive())?;
            let client = self.client().await?;
            let snap = client
                .get_option_chain_filtered(&filter.query())
                .await
                .map_err(|e| error_text(&e))?;
            Ok(super::chain::render(&snap, &filter))
        }
        .await;
        self.finish(result).await
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for TraderServer {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new(
                "terminal-trader",
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions(INSTRUCTIONS)
    }
}
