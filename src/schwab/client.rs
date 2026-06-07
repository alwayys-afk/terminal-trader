use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tracing::{info, warn};

use schwab_trader::types as tr;

use super::{
    auth::SchwabAuth,
    models::{AccountInfo, Order, Position, Quote},
    options::OptionChainSnapshot,
};

const TRADER_BASE: &str = "https://api.schwabapi.com/trader/v1";
const MARKETDATA_BASE: &str = "https://api.schwabapi.com/marketdata/v1";

/// Minimum remaining lifetime before we proactively refresh the access token.
const REFRESH_BUFFER_SECS: i64 = 60;

struct SchwabClientInner {
    trader: Arc<schwab_trader::Client>,
    marketdata: Arc<schwab_marketdata::Client>,
    access_token: String,
    expires_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct SchwabClient {
    auth: SchwabAuth,
    inner: Arc<RwLock<SchwabClientInner>>,
    refresh_gate: Arc<Mutex<()>>,
}

impl SchwabClient {
    pub async fn new(auth: SchwabAuth) -> Result<Self> {
        let token_data = auth.get_valid_token().await?;
        let http = authed_reqwest(&token_data.access_token)?;
        Ok(Self {
            auth,
            refresh_gate: Arc::new(Mutex::new(())),
            inner: Arc::new(RwLock::new(SchwabClientInner {
                trader: Arc::new(schwab_trader::Client::new_with_client(
                    TRADER_BASE,
                    http.clone(),
                )),
                marketdata: Arc::new(schwab_marketdata::Client::new_with_client(
                    MARKETDATA_BASE,
                    http,
                )),
                access_token: token_data.access_token,
                expires_at: token_data.expires_at,
            })),
        })
    }

    /// Refresh the access token if it expires within REFRESH_BUFFER_SECS.
    async fn ensure_fresh(&self) -> Result<()> {
        // Fast path: token is still valid.
        {
            let inner = self.inner.read().await;
            if Utc::now() + chrono::Duration::seconds(REFRESH_BUFFER_SECS) < inner.expires_at {
                return Ok(());
            }
        }
        // Another caller is already refreshing — the token is still valid for up to
        // REFRESH_BUFFER_SECS, so just use it.
        let Ok(_guard) = self.refresh_gate.try_lock() else {
            return Ok(());
        };
        info!("access token expired or expiring soon, refreshing");
        let token_data = self.auth.get_valid_token().await?;
        let http = authed_reqwest(&token_data.access_token)?;

        let mut inner = self.inner.write().await;
        inner.trader = Arc::new(schwab_trader::Client::new_with_client(
            TRADER_BASE,
            http.clone(),
        ));
        inner.marketdata = Arc::new(schwab_marketdata::Client::new_with_client(
            MARKETDATA_BASE,
            http,
        ));
        inner.access_token = token_data.access_token;
        inner.expires_at = token_data.expires_at;
        info!("access token refreshed, valid until {}", inner.expires_at);
        Ok(())
    }

    pub async fn access_token(&self) -> Result<String> {
        self.ensure_fresh().await?;
        Ok(self.inner.read().await.access_token.clone())
    }

    pub async fn trader(&self) -> Arc<schwab_trader::Client> {
        Arc::clone(&self.inner.read().await.trader)
    }

    pub async fn marketdata(&self) -> Arc<schwab_marketdata::Client> {
        Arc::clone(&self.inner.read().await.marketdata)
    }

    pub async fn get_streamer_info(&self) -> Result<schwab_trader::types::StreamerInfo> {
        self.ensure_fresh().await?;
        let trader = Arc::clone(&self.inner.read().await.trader);
        let prefs = trader
            .get_user_preference()
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("GET /userPreference failed: {e}"))?
            .into_inner();

        prefs
            .streamer_info
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("no streamer info in userPreference response"))
    }

    pub async fn get_account_hashes(&self) -> Result<Vec<(String, String)>> {
        self.ensure_fresh().await?;
        let trader = Arc::clone(&self.inner.read().await.trader);
        let accounts = trader
            .get_account_numbers()
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("GET /accounts/accountNumbers failed: {e}"))?
            .into_inner();

        Ok(accounts
            .into_iter()
            .filter_map(|a| Some((a.account_number?, a.hash_value?)))
            .collect())
    }

    /// Fetch the user's accounts joined with their /userPreference entries so
    /// each row carries the user-set nickname (when one is configured).
    pub async fn get_accounts(&self) -> Result<Vec<super::AccountSummary>> {
        self.ensure_fresh().await?;
        let trader = Arc::clone(&self.inner.read().await.trader);

        let (hashes, prefs) = tokio::try_join!(
            async {
                trader
                    .get_account_numbers()
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("GET /accounts/accountNumbers failed: {e}"))
            },
            async {
                trader
                    .get_user_preference()
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("GET /userPreference failed: {e}"))
            },
        )?;

        let nicks: std::collections::HashMap<String, String> = prefs
            .into_inner()
            .accounts
            .into_iter()
            .filter_map(|a| Some((a.account_number?, a.nick_name?)))
            .filter(|(_, nick)| !nick.trim().is_empty())
            .collect();

        Ok(hashes
            .into_inner()
            .into_iter()
            .filter_map(|a| {
                let number = a.account_number?;
                let hash = a.hash_value?;
                let nickname = nicks.get(&number).cloned();
                Some(super::AccountSummary {
                    number,
                    hash,
                    nickname,
                })
            })
            .collect())
    }

    pub async fn get_positions(&self, account_hash: &str) -> Result<(Vec<Position>, AccountInfo)> {
        self.ensure_fresh().await?;
        let trader = Arc::clone(&self.inner.read().await.trader);
        let account = trader
            .get_account()
            .account_number(account_hash)
            .fields("positions")
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?
            .into_inner();

        let (raw, info) = match account.securities_account {
            Some(tr::SecuritiesAccount::MarginAccount(a)) => {
                let cur = a.current_balances.unwrap_or_default();
                let proj = a.projected_balances.unwrap_or_default();
                (
                    a.positions,
                    AccountInfo {
                        // On a margin account a leveraged (debit) position
                        // zeroes out cashBalance and carries the borrowed
                        // amount as a negative marginBalance, so the true net
                        // cash is the sum of the two.
                        cash_balance: cur.cash_balance.unwrap_or(0.0)
                            + cur.margin_balance.unwrap_or(0.0),
                        buying_power: proj.buying_power.unwrap_or(cur.buying_power.unwrap_or(0.0)),
                    },
                )
            }
            Some(tr::SecuritiesAccount::CashAccount(a)) => {
                let bal = a.current_balances.unwrap_or_default();
                let cash = bal.cash_available_for_trading.unwrap_or(0.0);
                (
                    a.positions,
                    AccountInfo {
                        cash_balance: cash,
                        buying_power: cash,
                    },
                )
            }
            None => (
                vec![],
                AccountInfo {
                    cash_balance: 0.0,
                    buying_power: 0.0,
                },
            ),
        };

        let positions = raw
            .into_iter()
            .filter_map(|p| match Position::try_from(p) {
                Ok(pos) => Some(pos),
                Err(()) => {
                    warn!("get_positions: failed to convert a position");
                    None
                }
            })
            .collect();

        Ok((positions, info))
    }

    pub async fn get_quotes(&self, symbols: &[&str]) -> Result<HashMap<String, Quote>> {
        if symbols.is_empty() {
            return Ok(HashMap::new());
        }
        self.ensure_fresh().await?;
        let marketdata = Arc::clone(&self.inner.read().await.marketdata);
        let response = marketdata
            .get_quotes()
            .symbols(symbols.join(","))
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?
            .into_inner();

        Ok(response
            .0
            .into_iter()
            .filter_map(|(sym, obj)| Quote::from_api(&sym, &obj).map(|q| (sym, q)))
            .collect())
    }

    /// Place a single-leg order for any instrument type. For options the
    /// position effect (opening/closing) is derived from the instruction
    /// (`BuyToOpen`/`SellToClose`/etc.).
    pub async fn place_order(
        &self,
        account_hash: &str,
        spec: InstrumentSpec,
        instruction: tr::Instruction,
        quantity: f64,
        limit_price: Option<f64>,
        tax_lot_method: Option<tr::TaxLotMethod>,
    ) -> Result<()> {
        self.ensure_fresh().await?;
        let order =
            build_single_leg_order(spec, instruction, quantity, limit_price, tax_lot_method);
        let trader = Arc::clone(&self.inner.read().await.trader);
        trader
            .place_order()
            .account_number(account_hash)
            .body(order)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(())
    }

    pub async fn cancel_order(&self, account_hash: &str, order_id: i64) -> Result<()> {
        self.ensure_fresh().await?;
        let trader = Arc::clone(&self.inner.read().await.trader);
        trader
            .cancel_order()
            .account_number(account_hash)
            .order_id(order_id)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(())
    }

    /// Replace (cancel + re-submit) a single-leg order for any instrument type.
    #[allow(clippy::too_many_arguments)]
    pub async fn replace_order(
        &self,
        account_hash: &str,
        order_id: i64,
        spec: InstrumentSpec,
        instruction: tr::Instruction,
        quantity: f64,
        limit_price: Option<f64>,
        tax_lot_method: Option<tr::TaxLotMethod>,
    ) -> Result<()> {
        self.ensure_fresh().await?;
        let order =
            build_single_leg_order(spec, instruction, quantity, limit_price, tax_lot_method);
        let trader = Arc::clone(&self.inner.read().await.trader);
        trader
            .replace_order()
            .account_number(account_hash)
            .order_id(order_id)
            .body(order)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(())
    }

    /// Fetch the full option chain for an underlying and flatten it into a
    /// sorted [`OptionChainSnapshot`] (calls/puts grouped by expiration).
    pub async fn get_option_chain(&self, underlying: &str) -> Result<OptionChainSnapshot> {
        self.ensure_fresh().await?;
        let marketdata = Arc::clone(&self.inner.read().await.marketdata);
        let chain = marketdata
            .get_chain()
            .symbol(underlying)
            .include_underlying_quote(true)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?
            .into_inner();
        Ok(OptionChainSnapshot::from_api(underlying, chain))
    }

    /// Search for instruments by symbol prefix or description words.
    /// Returns (symbol, description) pairs.
    pub async fn search_instruments(
        &self,
        query: &str,
        projection: schwab_marketdata::types::GetInstrumentsProjection,
    ) -> Result<Vec<(String, String)>> {
        self.ensure_fresh().await?;
        let marketdata = Arc::clone(&self.inner.read().await.marketdata);
        let resp = marketdata
            .get_instruments()
            .symbol(query)
            .projection(projection)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?
            .into_inner();

        Ok(resp
            .instruments
            .into_iter()
            .filter_map(|i| {
                let sym = i.symbol?;
                let desc = i.description.unwrap_or_default();
                Some((sym, desc))
            })
            .collect())
    }

    pub async fn get_transactions_by_type(
        &self,
        account_hash: &str,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        transaction_type: tr::TransactionType,
    ) -> Result<Vec<tr::Transaction>> {
        let mut map = HashMap::new();
        self.collect_transactions(
            account_hash,
            start_date,
            end_date,
            transaction_type,
            &mut map,
        )
        .await?;
        Ok(map.into_values().collect())
    }

    fn collect_transactions<'a>(
        &'a self,
        account_hash: &'a str,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        transaction_type: tr::TransactionType,
        map: &'a mut HashMap<i64, tr::Transaction>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            const PAGE_LIMIT: usize = 3000;

            let page = self
                .fetch_transactions_page(account_hash, start_date, end_date, transaction_type)
                .await?;
            let hit_cap = page.len() >= PAGE_LIMIT;

            for t in page {
                if let Some(id) = t.activity_id {
                    map.insert(id, t);
                }
            }

            if hit_cap {
                let mid = start_date + (end_date - start_date) / 2;
                warn!(
                    "Transaction fetch hit API cap ({PAGE_LIMIT} items) for {start_date}..{end_date}, bisecting at {mid}"
                );
                self.collect_transactions(account_hash, start_date, mid, transaction_type, map)
                    .await?;
                self.collect_transactions(account_hash, mid, end_date, transaction_type, map)
                    .await?;
            }

            Ok(())
        })
    }

    async fn fetch_transactions_page(
        &self,
        account_hash: &str,
        start_date: chrono::DateTime<chrono::Utc>,
        end_date: chrono::DateTime<chrono::Utc>,
        transaction_type: tr::TransactionType,
    ) -> Result<Vec<tr::Transaction>> {
        let fmt =
            |dt: chrono::DateTime<chrono::Utc>| dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
        self.ensure_fresh().await?;
        let trader = self.trader().await;
        let transactions = trader
            .get_transactions_by_path_param()
            .account_number(account_hash)
            .start_date(fmt(start_date))
            .end_date(fmt(end_date))
            .types(transaction_type)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?
            .into_inner();

        Ok(transactions)
    }

    pub async fn get_orders(&self, account_hash: &str) -> Result<Vec<Order>> {
        self.ensure_fresh().await?;
        let from = (chrono::Utc::now() - chrono::Duration::days(60))
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string();
        let to = chrono::Utc::now()
            .format("%Y-%m-%dT%H:%M:%S%.3fZ")
            .to_string();

        let trader = Arc::clone(&self.inner.read().await.trader);
        let orders = trader
            .get_orders_by_path_param()
            .account_number(account_hash)
            .from_entered_time(from)
            .to_entered_time(to)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?
            .into_inner();

        Ok(orders
            .into_iter()
            .filter_map(|o| {
                let id_hint = o.order_id;
                match Order::try_from(o) {
                    Ok(order) => Some(order),
                    Err(()) => {
                        warn!("get_orders: failed to convert order (id: {id_hint:?})");
                        None
                    }
                }
            })
            .collect())
    }
}

/// Which instrument a single-leg order targets. Keeps the equity/option
/// branching in one place so order building stays asset-type agnostic.
#[derive(Debug, Clone)]
pub enum InstrumentSpec {
    Equity { symbol: String },
    Option { osi_symbol: String },
}

impl InstrumentSpec {
    fn into_instrument(self) -> tr::AccountsInstrument {
        match self {
            InstrumentSpec::Equity { symbol } => {
                tr::AccountsInstrument::Equity(tr::AccountEquity(tr::AccountsBaseInstrument {
                    asset_type: tr::AccountsBaseInstrumentAssetType::Equity,
                    symbol: Some(symbol),
                    cusip: None,
                    description: None,
                    instrument_id: None,
                    name: serde_json::Value::Null,
                    net_change: None,
                }))
            }
            InstrumentSpec::Option { osi_symbol } => {
                tr::AccountsInstrument::Option(tr::AccountOption {
                    asset_type: tr::AccountOptionAssetType::Option,
                    symbol: Some(osi_symbol),
                    cusip: None,
                    description: None,
                    instrument_id: None,
                    name: serde_json::Value::Null,
                    net_change: None,
                    option_deliverables: Vec::new(),
                    option_multiplier: None,
                    put_call: None,
                    type_: None,
                    underlying_symbol: None,
                })
            }
        }
    }

    /// Only options carry an explicit leg type; equities leave it unset so the
    /// wire format matches the long-standing equity order request exactly.
    fn order_leg_type(&self) -> Option<tr::OrderLegCollectionOrderLegType> {
        match self {
            InstrumentSpec::Equity { .. } => None,
            InstrumentSpec::Option { .. } => Some(tr::OrderLegCollectionOrderLegType::Option),
        }
    }

    /// Equities keep using SEAMLESS (extended-hours routing). Options have no
    /// extended-hours session, so they route NORMAL — SEAMLESS gets rejected.
    fn session(&self) -> tr::Session {
        match self {
            InstrumentSpec::Equity { .. } => tr::Session::Seamless,
            InstrumentSpec::Option { .. } => tr::Session::Normal,
        }
    }
}

/// Derive the leg's position effect from the instruction. Buy-vs-sell is
/// already carried by the leg's `instruction` field (`BuyToOpen` vs
/// `SellToOpen` are distinct values), so this only conveys the open/close
/// dimension — it's a redundant hint Schwab expects alongside the instruction.
/// Equity Buy/Sell have no position effect; option to-open/to-close
/// instructions map to Opening/Closing.
fn position_effect_for(
    instruction: tr::Instruction,
) -> Option<tr::OrderLegCollectionPositionEffect> {
    use tr::Instruction::*;
    use tr::OrderLegCollectionPositionEffect as Pe;
    match instruction {
        BuyToOpen | SellToOpen => Some(Pe::Opening),
        BuyToClose | SellToClose => Some(Pe::Closing),
        _ => None,
    }
}

/// Build a single-leg `OrderRequest` (GTC; session per instrument type).
/// Market when `limit_price` is `None`, otherwise a limit at that price.
fn build_single_leg_order(
    spec: InstrumentSpec,
    instruction: tr::Instruction,
    quantity: f64,
    limit_price: Option<f64>,
    tax_lot_method: Option<tr::TaxLotMethod>,
) -> tr::OrderRequest {
    let session = spec.session();
    let order_leg_type = spec.order_leg_type();
    let leg = tr::OrderLegCollection {
        instruction: Some(instruction),
        instrument: Some(spec.into_instrument()),
        quantity: Some(quantity),
        order_leg_type,
        position_effect: position_effect_for(instruction),
        ..Default::default()
    };
    let (order_type, price) = match limit_price {
        Some(p) => (tr::OrderTypeRequest::Limit, Some(p)),
        None => (tr::OrderTypeRequest::Market, None),
    };
    tr::OrderRequest {
        order_type: Some(order_type),
        order_strategy_type: Some(tr::OrderStrategyType::Single),
        session: Some(session),
        duration: Some(tr::Duration::GoodTillCancel),
        price,
        order_leg_collection: vec![leg],
        tax_lot_method,
        ..Default::default()
    }
}

fn authed_reqwest(access_token: &str) -> Result<reqwest::Client> {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {access_token}"))?,
    );
    Ok(reqwest::Client::builder()
        .default_headers(headers)
        .build()?)
}
