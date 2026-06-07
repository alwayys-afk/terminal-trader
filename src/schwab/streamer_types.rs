use serde::{Deserialize, Serialize};
use serde_json::Value;

// ─── Service & command enums ──────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServiceName {
    Admin,
    LeveloneEquities,
    LeveloneOptions,
    LeveloneFutures,
    LeveloneFuturesOptions,
    LeveloneForex,
    NyseBook,
    NasdaqBook,
    OptionsBook,
    ChartEquity,
    ChartFutures,
    ScreenerEquity,
    ScreenerOption,
    AcctActivity,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CommandName {
    Login,
    Logout,
    Subs,
    Add,
    Unsubs,
    View,
}

// ─── Outbound: request envelope ───────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct StreamerRequest {
    pub requests: Vec<RequestItem>,
}

#[derive(Debug, Serialize)]
pub struct RequestItem {
    pub service: ServiceName,
    pub command: CommandName,
    pub requestid: String,
    #[serde(rename = "SchwabClientCustomerId")]
    pub schwab_client_customer_id: String,
    #[serde(rename = "SchwabClientCorrelId")]
    pub schwab_client_correl_id: String,
    pub parameters: Value,
}

// Per-command parameter structs (outbound only)

#[derive(Debug, Serialize)]
pub struct LoginParams {
    #[serde(rename = "Authorization")]
    pub authorization: String,
    #[serde(rename = "SchwabClientChannel")]
    pub schwab_client_channel: String,
    #[serde(rename = "SchwabClientFunctionId")]
    pub schwab_client_function_id: String,
}

#[derive(Debug, Serialize)]
pub struct SubsEquitiesParams {
    pub keys: String,
    pub fields: String,
}

#[derive(Debug, Serialize)]
pub struct UnsubsEquitiesParams {
    pub keys: String,
}

#[derive(Debug, Serialize)]
pub struct SubsAcctActivityParams {
    pub keys: String,
    pub fields: String,
}

/// Fields we request from LEVELONE_EQUITIES. Kept here next to the struct so
/// it's obvious which field numbers correspond to which struct fields.
const EQUITY_FIELDS: &str = "0,1,2,3,4,5,8,10,11,12,17,18,33";

impl StreamerRequest {
    pub fn login(
        customer_id: &str,
        correl_id: &str,
        access_token: &str,
        channel: &str,
        function_id: &str,
    ) -> Self {
        Self {
            requests: vec![RequestItem {
                service: ServiceName::Admin,
                command: CommandName::Login,
                requestid: "0".to_string(),
                schwab_client_customer_id: customer_id.to_string(),
                schwab_client_correl_id: correl_id.to_string(),
                parameters: serde_json::to_value(LoginParams {
                    authorization: access_token.to_string(),
                    schwab_client_channel: channel.to_string(),
                    schwab_client_function_id: function_id.to_string(),
                })
                .expect("LoginParams serialization is infallible"),
            }],
        }
    }

    pub fn subs_equities<'a>(
        customer_id: &str,
        correl_id: &str,
        symbols: impl Iterator<Item = &'a String>,
    ) -> Self {
        Self {
            requests: vec![RequestItem {
                service: ServiceName::LeveloneEquities,
                command: CommandName::Subs,
                requestid: "1".to_string(),
                schwab_client_customer_id: customer_id.to_string(),
                schwab_client_correl_id: correl_id.to_string(),
                parameters: serde_json::to_value(SubsEquitiesParams {
                    keys: symbols.map(String::as_str).collect::<Vec<_>>().join(","),
                    fields: EQUITY_FIELDS.to_string(),
                })
                .expect("SubsEquitiesParams serialization is infallible"),
            }],
        }
    }

    pub fn unsubs_equities<'a>(
        customer_id: &str,
        correl_id: &str,
        symbols: impl Iterator<Item = &'a String>,
    ) -> Self {
        Self {
            requests: vec![RequestItem {
                service: ServiceName::LeveloneEquities,
                command: CommandName::Unsubs,
                requestid: "1".to_string(),
                schwab_client_customer_id: customer_id.to_string(),
                schwab_client_correl_id: correl_id.to_string(),
                parameters: serde_json::to_value(UnsubsEquitiesParams {
                    keys: symbols.map(String::as_str).collect::<Vec<_>>().join(","),
                })
                .expect("UnsubsEquitiesParams serialization is infallible"),
            }],
        }
    }

    pub fn subs_acct_activity(customer_id: &str, correl_id: &str) -> Self {
        Self {
            requests: vec![RequestItem {
                service: ServiceName::AcctActivity,
                command: CommandName::Subs,
                requestid: "2".to_string(),
                schwab_client_customer_id: customer_id.to_string(),
                schwab_client_correl_id: correl_id.to_string(),
                parameters: serde_json::to_value(SubsAcctActivityParams {
                    keys: "Account Activity".to_string(),
                    fields: "0,1,2,3".to_string(),
                })
                .expect("SubsAcctActivityParams serialization is infallible"),
            }],
        }
    }
}

// ─── Inbound: message envelope ────────────────────────────────────────────────

/// Top-level message from the streamer. One of three shapes depending on which
/// top-level key is present.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StreamerInbound {
    Notify { notify: Vec<HeartbeatNotify> },
    Response { response: Vec<ResponseFrame> },
    Data { data: Vec<DataFrame> },
}

#[derive(Debug, Deserialize)]
pub struct HeartbeatNotify {
    pub heartbeat: String,
}

#[derive(Debug, Deserialize)]
pub struct ResponseFrame {
    pub service: String,
    pub command: String,
    pub requestid: String,
    pub timestamp: Option<i64>,
    pub content: ResponseContent,
}

#[derive(Debug, Deserialize)]
pub struct ResponseContent {
    pub code: i64,
    pub msg: Option<String>,
}

/// A data frame from the streamer. `content` is kept as raw `Value` because
/// different services have different schemas; callers parse it into the
/// appropriate typed struct after matching on `service`.
#[derive(Debug, Deserialize)]
pub struct DataFrame {
    pub service: ServiceName,
    pub timestamp: i64,
    pub command: String,
    pub content: Vec<Value>,
}

// ─── LEVELONE_EQUITIES content ────────────────────────────────────────────────

/// Parsed content item from a LEVELONE_EQUITIES data frame.
///
/// The streamer only streams fields that have *changed* since the last update,
/// so every field is `Option` except `key`. The fields we subscribe to are
/// determined by `EQUITY_FIELDS` above (0,1,2,3,4,5,8,10,11,12,17,18,33).
///
/// Field numbers map as per the streamer spec:
///   1  Bid Price        2  Ask Price        3  Last Price
///   4  Bid Size         5  Ask Size         8  Total Volume
///   10 High Price       11 Low Price        12 Close Price
///   17 Open Price       18 Net Change       33 Mark Price
#[derive(Debug, Default, Deserialize)]
pub struct LevelOneEquityTick {
    pub key: String,
    #[serde(default)]
    pub delayed: bool,
    #[serde(rename = "1")]
    pub bid_price: Option<f64>,
    #[serde(rename = "2")]
    pub ask_price: Option<f64>,
    #[serde(rename = "3")]
    pub last_price: Option<f64>,
    #[serde(rename = "4")]
    pub bid_size: Option<i64>,
    #[serde(rename = "5")]
    pub ask_size: Option<i64>,
    #[serde(rename = "8")]
    pub volume: Option<i64>,
    #[serde(rename = "10")]
    pub high_price: Option<f64>,
    #[serde(rename = "11")]
    pub low_price: Option<f64>,
    #[serde(rename = "12")]
    pub close_price: Option<f64>,
    #[serde(rename = "17")]
    pub open_price: Option<f64>,
    #[serde(rename = "18")]
    pub net_change: Option<f64>,
    #[serde(rename = "33")]
    pub mark: Option<f64>,
}

// ─── ACCT_ACTIVITY content ────────────────────────────────────────────────────

/// Parsed content item from an ACCT_ACTIVITY data frame.
///
/// Field numbers:
///   1  Account (account number the activity occurred on)
///   2  Message Type
///   3  Message Data (JSON string or NULL)
#[derive(Debug, Deserialize)]
pub struct AcctActivityTick {
    pub seq: Option<i64>,
    pub key: String,
    #[serde(rename = "1")]
    pub account: Option<String>,
    #[serde(rename = "2")]
    pub message_type: Option<AcctActivityMessageType>,
    #[serde(rename = "3")]
    pub message_data: Option<String>,
}

/// Known ACCT_ACTIVITY message types, from the empirical notes in the streamer
/// docs. `Unknown` catches any future or unrecognised types.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum AcctActivityMessageType {
    #[serde(rename = "SUBSCRIBED")]
    Subscribed,
    OrderCreated,
    OrderAccepted,
    ChangeCreated,
    ChangeAccepted,
    CancelAccepted,
    ExecutionCreated,
    OrderMonitorCreated,
    OrderMonitorCompleted,
    OrderUROutCompleted,
    OrderFillCompleted,
    ExecutionRequested,
    ExecutionRequestCreated,
    ExecutionRequestCompleted,
    #[serde(other)]
    Unknown,
}

impl AcctActivityMessageType {
    /// Returns `true` for event types that should trigger a position refresh.
    pub fn triggers_position_refresh(&self) -> bool {
        matches!(
            self,
            Self::OrderUROutCompleted | Self::ExecutionCreated | Self::OrderFillCompleted
        )
    }
}
