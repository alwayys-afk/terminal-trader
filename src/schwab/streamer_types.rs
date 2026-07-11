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

/// Parameters for a LEVELONE_* request. SUBS/ADD carry the `fields` list;
/// UNSUBS omits it (serialized away when `None`).
#[derive(Debug, Serialize)]
pub struct LevelOneParams {
    pub keys: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SubsAcctActivityParams {
    pub keys: String,
    pub fields: String,
}

/// Fields we request from LEVELONE_EQUITIES. Kept here next to the struct so
/// it's obvious which field numbers correspond to which struct fields.
const EQUITY_FIELDS: &str = "0,1,2,3,4,5,8,10,11,12,17,18,33";

/// Fields we request from LEVELONE_OPTIONS. The option field numbering differs
/// from equities (see `LevelOneOptionTick`): mark is 37 (not 33), close is 7,
/// open is 15, and greeks live at 28-31. Covers price, IV, open interest, and
/// the greeks the chain view renders.
const OPTION_FIELDS: &str = "0,2,3,4,5,6,7,8,9,10,15,16,17,19,28,29,30,31,37,44";

// Per-subscription request ids. The streamer echoes these back on response
// frames, so each logical subscription gets a distinct id to tell them apart.
const REQ_LOGIN: &str = "0";
const REQ_EQUITIES: &str = "1";
const REQ_ACCT_ACTIVITY: &str = "2";
const REQ_OPTIONS: &str = "3";

/// A LEVELONE quote feed. Each feed knows its streamer service, the field list
/// it subscribes to, and its request id — so the SUBS/UNSUBS plumbing is
/// written once and parameterized by feed rather than copy-pasted per service.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelOneFeed {
    Equities,
    Options,
}

impl LevelOneFeed {
    fn service(self) -> ServiceName {
        match self {
            LevelOneFeed::Equities => ServiceName::LeveloneEquities,
            LevelOneFeed::Options => ServiceName::LeveloneOptions,
        }
    }

    fn fields(self) -> &'static str {
        match self {
            LevelOneFeed::Equities => EQUITY_FIELDS,
            LevelOneFeed::Options => OPTION_FIELDS,
        }
    }

    fn request_id(self) -> &'static str {
        match self {
            LevelOneFeed::Equities => REQ_EQUITIES,
            LevelOneFeed::Options => REQ_OPTIONS,
        }
    }

    /// Service name for log/error messages, e.g. "LEVELONE_EQUITIES".
    pub fn label(self) -> &'static str {
        match self {
            LevelOneFeed::Equities => "LEVELONE_EQUITIES",
            LevelOneFeed::Options => "LEVELONE_OPTIONS",
        }
    }
}

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
                requestid: REQ_LOGIN.to_string(),
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

    /// Build a SUBS or UNSUBS request for a LEVELONE feed. SUBS carries the
    /// feed's field list; UNSUBS omits it.
    fn level_one<'a>(
        feed: LevelOneFeed,
        command: CommandName,
        customer_id: &str,
        correl_id: &str,
        symbols: impl Iterator<Item = &'a String>,
    ) -> Self {
        let fields = matches!(command, CommandName::Subs | CommandName::Add)
            .then(|| feed.fields().to_string());
        Self {
            requests: vec![RequestItem {
                service: feed.service(),
                command,
                requestid: feed.request_id().to_string(),
                schwab_client_customer_id: customer_id.to_string(),
                schwab_client_correl_id: correl_id.to_string(),
                parameters: serde_json::to_value(LevelOneParams {
                    keys: symbols.map(String::as_str).collect::<Vec<_>>().join(","),
                    fields,
                })
                .expect("LevelOneParams serialization is infallible"),
            }],
        }
    }

    pub fn subscribe<'a>(
        feed: LevelOneFeed,
        customer_id: &str,
        correl_id: &str,
        symbols: impl Iterator<Item = &'a String>,
    ) -> Self {
        Self::level_one(feed, CommandName::Subs, customer_id, correl_id, symbols)
    }

    pub fn unsubscribe<'a>(
        feed: LevelOneFeed,
        customer_id: &str,
        correl_id: &str,
        symbols: impl Iterator<Item = &'a String>,
    ) -> Self {
        Self::level_one(feed, CommandName::Unsubs, customer_id, correl_id, symbols)
    }

    pub fn subs_acct_activity(customer_id: &str, correl_id: &str) -> Self {
        Self {
            requests: vec![RequestItem {
                service: ServiceName::AcctActivity,
                command: CommandName::Subs,
                requestid: REQ_ACCT_ACTIVITY.to_string(),
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

// ─── LEVELONE_OPTIONS content ─────────────────────────────────────────────────

/// Parsed content item from a LEVELONE_OPTIONS data frame.
///
/// Like equities, only *changed* fields are streamed, so every field is
/// `Option` except `key`. **The field numbering differs from equities** — these
/// map per the LEVELONE_OPTIONS spec, not the equity spec:
///   2  Bid Price        3  Ask Price        4  Last Price
///   5  High Price       6  Low Price        7  Close Price
///   8  Total Volume     9  Open Interest    10 Volatility (IV)
///   15 Open Price       16 Bid Size         17 Ask Size
///   19 Net Change       28 Delta            29 Gamma
///   30 Theta            31 Vega             37 Mark Price
///   44 Net % Change
#[derive(Debug, Default, Deserialize)]
pub struct LevelOneOptionTick {
    pub key: String,
    #[serde(default)]
    pub delayed: bool,
    #[serde(rename = "2")]
    pub bid_price: Option<f64>,
    #[serde(rename = "3")]
    pub ask_price: Option<f64>,
    #[serde(rename = "4")]
    pub last_price: Option<f64>,
    #[serde(rename = "5")]
    pub high_price: Option<f64>,
    #[serde(rename = "6")]
    pub low_price: Option<f64>,
    #[serde(rename = "7")]
    pub close_price: Option<f64>,
    #[serde(rename = "8")]
    pub volume: Option<i64>,
    #[serde(rename = "9")]
    pub open_interest: Option<i64>,
    #[serde(rename = "10")]
    pub volatility: Option<f64>,
    #[serde(rename = "15")]
    pub open_price: Option<f64>,
    #[serde(rename = "16")]
    pub bid_size: Option<i64>,
    #[serde(rename = "17")]
    pub ask_size: Option<i64>,
    #[serde(rename = "19")]
    pub net_change: Option<f64>,
    #[serde(rename = "28")]
    pub delta: Option<f64>,
    #[serde(rename = "29")]
    pub gamma: Option<f64>,
    #[serde(rename = "30")]
    pub theta: Option<f64>,
    #[serde(rename = "31")]
    pub vega: Option<f64>,
    #[serde(rename = "37")]
    pub mark: Option<f64>,
    #[serde(rename = "44")]
    pub net_percent_change: Option<f64>,
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Pins the LEVELONE_OPTIONS field-number → struct-field mapping. The option
    /// numbering diverges from equities (mark=37, close=7, open=15, greeks at
    /// 28-31), so a copy-paste from the equity tick would silently mis-map.
    #[test]
    fn option_tick_deserializes_by_field_number() {
        // A realistic data-frame content item: symbol in `key`, changed fields
        // as numbered strings.
        let raw = serde_json::json!({
            "key": "AAPL  260619C00250000",
            "delayed": false,
            "2": 1.23,    // bid
            "3": 1.27,    // ask
            "4": 1.25,    // last
            "5": 1.40,    // high
            "6": 1.10,    // low
            "7": 1.05,    // close
            "8": 4321,    // volume
            "9": 9876,    // open interest
            "10": 28.5,   // volatility (IV)
            "15": 1.15,   // open
            "16": 12,     // bid size
            "17": 34,     // ask size
            "19": 0.20,   // net change
            "28": 0.45,   // delta
            "29": 0.06,   // gamma
            "30": -0.03,  // theta
            "31": 0.12,   // vega
            "37": 1.25,   // mark
            "44": 19.05   // net % change
        });

        let tick: LevelOneOptionTick = serde_json::from_value(raw).unwrap();

        assert_eq!(tick.key, "AAPL  260619C00250000");
        assert_eq!(tick.bid_price, Some(1.23));
        assert_eq!(tick.ask_price, Some(1.27));
        assert_eq!(tick.last_price, Some(1.25));
        assert_eq!(tick.close_price, Some(1.05));
        assert_eq!(tick.volume, Some(4321));
        assert_eq!(tick.open_interest, Some(9876));
        assert_eq!(tick.volatility, Some(28.5));
        assert_eq!(tick.open_price, Some(1.15));
        assert_eq!(tick.bid_size, Some(12));
        assert_eq!(tick.ask_size, Some(34));
        assert_eq!(tick.delta, Some(0.45));
        assert_eq!(tick.theta, Some(-0.03));
        assert_eq!(tick.vega, Some(0.12));
        assert_eq!(tick.mark, Some(1.25));
        assert_eq!(tick.net_percent_change, Some(19.05));
    }

    /// SUBS must carry the feed's field list; UNSUBS must omit `fields`
    /// entirely (not send `null` or an empty string). Also pins each feed's
    /// service name and request id onto the wire.
    #[test]
    fn level_one_subscribe_includes_fields_unsubscribe_omits_them() {
        let keys = vec!["AAPL".to_string()];

        let subs = StreamerRequest::subscribe(LevelOneFeed::Equities, "cust", "corr", keys.iter());
        let v = serde_json::to_value(&subs.requests[0]).unwrap();
        assert_eq!(v["service"], "LEVELONE_EQUITIES");
        assert_eq!(v["requestid"], REQ_EQUITIES);
        assert_eq!(v["parameters"]["keys"], "AAPL");
        assert_eq!(v["parameters"]["fields"], EQUITY_FIELDS);

        let unsubs =
            StreamerRequest::unsubscribe(LevelOneFeed::Options, "cust", "corr", keys.iter());
        let v = serde_json::to_value(&unsubs.requests[0]).unwrap();
        assert_eq!(v["service"], "LEVELONE_OPTIONS");
        assert_eq!(v["requestid"], REQ_OPTIONS);
        assert_eq!(v["parameters"]["keys"], "AAPL");
        assert!(
            v["parameters"].get("fields").is_none(),
            "UNSUBS must omit the fields key entirely"
        );
    }

    /// Only changed fields arrive on a given tick — everything absent stays
    /// `None` rather than failing to parse.
    #[test]
    fn option_tick_tolerates_sparse_updates() {
        let raw = serde_json::json!({ "key": "SPY   260619P00400000", "37": 2.50 });
        let tick: LevelOneOptionTick = serde_json::from_value(raw).unwrap();
        assert_eq!(tick.key, "SPY   260619P00400000");
        assert_eq!(tick.mark, Some(2.50));
        assert_eq!(tick.bid_price, None);
        assert_eq!(tick.delta, None);
    }
}
