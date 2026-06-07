#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{ClientHooks, OperationInfo, RequestBuilderExt, encode_path};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}
        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    ///Instrument's asset type
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Instrument's asset type",
    ///  "type": "string",
    ///  "enum": [
    ///    "BOND",
    ///    "EQUITY",
    ///    "FOREX",
    ///    "FUTURE",
    ///    "FUTURE_OPTION",
    ///    "INDEX",
    ///    "MUTUAL_FUND",
    ///    "OPTION"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum AssetMainType {
        #[serde(rename = "BOND")]
        Bond,
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "FOREX")]
        Forex,
        #[serde(rename = "FUTURE")]
        Future,
        #[serde(rename = "FUTURE_OPTION")]
        FutureOption,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "OPTION")]
        Option,
    }

    impl ::std::fmt::Display for AssetMainType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Bond => f.write_str("BOND"),
                Self::Equity => f.write_str("EQUITY"),
                Self::Forex => f.write_str("FOREX"),
                Self::Future => f.write_str("FUTURE"),
                Self::FutureOption => f.write_str("FUTURE_OPTION"),
                Self::Index => f.write_str("INDEX"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::Option => f.write_str("OPTION"),
            }
        }
    }

    impl ::std::str::FromStr for AssetMainType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "BOND" => Ok(Self::Bond),
                "EQUITY" => Ok(Self::Equity),
                "FOREX" => Ok(Self::Forex),
                "FUTURE" => Ok(Self::Future),
                "FUTURE_OPTION" => Ok(Self::FutureOption),
                "INDEX" => Ok(Self::Index),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "OPTION" => Ok(Self::Option),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AssetMainType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AssetMainType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AssetMainType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`Bond`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "assetType": {
    ///      "type": "string",
    ///      "enum": [
    ///        "BOND",
    ///        "EQUITY",
    ///        "ETF",
    ///        "EXTENDED",
    ///        "FOREX",
    ///        "FUTURE",
    ///        "FUTURE_OPTION",
    ///        "FUNDAMENTAL",
    ///        "INDEX",
    ///        "INDICATOR",
    ///        "MUTUAL_FUND",
    ///        "OPTION",
    ///        "UNKNOWN"
    ///      ]
    ///    },
    ///    "bondFactor": {
    ///      "type": "string"
    ///    },
    ///    "bondMultiplier": {
    ///      "type": "string"
    ///    },
    ///    "bondPrice": {
    ///      "type": "number"
    ///    },
    ///    "cusip": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "exchange": {
    ///      "type": "string"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "writeOnly": true,
    ///      "type": "string",
    ///      "enum": [
    ///        "BOND",
    ///        "EQUITY",
    ///        "ETF",
    ///        "EXTENDED",
    ///        "FOREX",
    ///        "FUTURE",
    ///        "FUTURE_OPTION",
    ///        "FUNDAMENTAL",
    ///        "INDEX",
    ///        "INDICATOR",
    ///        "MUTUAL_FUND",
    ///        "OPTION",
    ///        "UNKNOWN"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Bond {
        #[serde(
            rename = "assetType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub asset_type: ::std::option::Option<BondAssetType>,
        #[serde(
            rename = "bondFactor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bond_factor: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "bondMultiplier",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bond_multiplier: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "bondPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bond_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub exchange: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<BondType>,
    }

    impl ::std::default::Default for Bond {
        fn default() -> Self {
            Self {
                asset_type: Default::default(),
                bond_factor: Default::default(),
                bond_multiplier: Default::default(),
                bond_price: Default::default(),
                cusip: Default::default(),
                description: Default::default(),
                exchange: Default::default(),
                symbol: Default::default(),
                type_: Default::default(),
            }
        }
    }

    impl Bond {
        pub fn builder() -> builder::Bond {
            Default::default()
        }
    }

    ///`BondAssetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "BOND",
    ///    "EQUITY",
    ///    "ETF",
    ///    "EXTENDED",
    ///    "FOREX",
    ///    "FUTURE",
    ///    "FUTURE_OPTION",
    ///    "FUNDAMENTAL",
    ///    "INDEX",
    ///    "INDICATOR",
    ///    "MUTUAL_FUND",
    ///    "OPTION",
    ///    "UNKNOWN"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum BondAssetType {
        #[serde(rename = "BOND")]
        Bond,
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "ETF")]
        Etf,
        #[serde(rename = "EXTENDED")]
        Extended,
        #[serde(rename = "FOREX")]
        Forex,
        #[serde(rename = "FUTURE")]
        Future,
        #[serde(rename = "FUTURE_OPTION")]
        FutureOption,
        #[serde(rename = "FUNDAMENTAL")]
        Fundamental,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "INDICATOR")]
        Indicator,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for BondAssetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Bond => f.write_str("BOND"),
                Self::Equity => f.write_str("EQUITY"),
                Self::Etf => f.write_str("ETF"),
                Self::Extended => f.write_str("EXTENDED"),
                Self::Forex => f.write_str("FOREX"),
                Self::Future => f.write_str("FUTURE"),
                Self::FutureOption => f.write_str("FUTURE_OPTION"),
                Self::Fundamental => f.write_str("FUNDAMENTAL"),
                Self::Index => f.write_str("INDEX"),
                Self::Indicator => f.write_str("INDICATOR"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::Option => f.write_str("OPTION"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for BondAssetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "BOND" => Ok(Self::Bond),
                "EQUITY" => Ok(Self::Equity),
                "ETF" => Ok(Self::Etf),
                "EXTENDED" => Ok(Self::Extended),
                "FOREX" => Ok(Self::Forex),
                "FUTURE" => Ok(Self::Future),
                "FUTURE_OPTION" => Ok(Self::FutureOption),
                "FUNDAMENTAL" => Ok(Self::Fundamental),
                "INDEX" => Ok(Self::Index),
                "INDICATOR" => Ok(Self::Indicator),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "OPTION" => Ok(Self::Option),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for BondAssetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for BondAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for BondAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`BondType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "writeOnly": true,
    ///  "type": "string",
    ///  "enum": [
    ///    "BOND",
    ///    "EQUITY",
    ///    "ETF",
    ///    "EXTENDED",
    ///    "FOREX",
    ///    "FUTURE",
    ///    "FUTURE_OPTION",
    ///    "FUNDAMENTAL",
    ///    "INDEX",
    ///    "INDICATOR",
    ///    "MUTUAL_FUND",
    ///    "OPTION",
    ///    "UNKNOWN"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum BondType {
        #[serde(rename = "BOND")]
        Bond,
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "ETF")]
        Etf,
        #[serde(rename = "EXTENDED")]
        Extended,
        #[serde(rename = "FOREX")]
        Forex,
        #[serde(rename = "FUTURE")]
        Future,
        #[serde(rename = "FUTURE_OPTION")]
        FutureOption,
        #[serde(rename = "FUNDAMENTAL")]
        Fundamental,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "INDICATOR")]
        Indicator,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for BondType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Bond => f.write_str("BOND"),
                Self::Equity => f.write_str("EQUITY"),
                Self::Etf => f.write_str("ETF"),
                Self::Extended => f.write_str("EXTENDED"),
                Self::Forex => f.write_str("FOREX"),
                Self::Future => f.write_str("FUTURE"),
                Self::FutureOption => f.write_str("FUTURE_OPTION"),
                Self::Fundamental => f.write_str("FUNDAMENTAL"),
                Self::Index => f.write_str("INDEX"),
                Self::Indicator => f.write_str("INDICATOR"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::Option => f.write_str("OPTION"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for BondType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "BOND" => Ok(Self::Bond),
                "EQUITY" => Ok(Self::Equity),
                "ETF" => Ok(Self::Etf),
                "EXTENDED" => Ok(Self::Extended),
                "FOREX" => Ok(Self::Forex),
                "FUTURE" => Ok(Self::Future),
                "FUTURE_OPTION" => Ok(Self::FutureOption),
                "FUNDAMENTAL" => Ok(Self::Fundamental),
                "INDEX" => Ok(Self::Index),
                "INDICATOR" => Ok(Self::Indicator),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "OPTION" => Ok(Self::Option),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for BondType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for BondType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for BondType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`Candle`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "close": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "datetime": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "datetimeISO8601": {
    ///      "type": "string",
    ///      "format": "yyyy-MM-dd"
    ///    },
    ///    "high": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "low": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "open": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "volume": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Candle {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub close: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub datetime: ::std::option::Option<i64>,
        #[serde(
            rename = "datetimeISO8601",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub datetime_iso8601: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub high: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub low: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub open: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub volume: ::std::option::Option<i64>,
    }

    impl ::std::default::Default for Candle {
        fn default() -> Self {
            Self {
                close: Default::default(),
                datetime: Default::default(),
                datetime_iso8601: Default::default(),
                high: Default::default(),
                low: Default::default(),
                open: Default::default(),
                volume: Default::default(),
            }
        }
    }

    impl Candle {
        pub fn builder() -> builder::Candle {
            Default::default()
        }
    }

    ///`CandleList`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "candles": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Candle"
    ///      }
    ///    },
    ///    "empty": {
    ///      "type": "boolean"
    ///    },
    ///    "previousClose": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "previousCloseDate": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "previousCloseDateISO8601": {
    ///      "type": "string",
    ///      "format": "yyyy-MM-dd"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CandleList {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub candles: ::std::vec::Vec<Candle>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub empty: ::std::option::Option<bool>,
        #[serde(
            rename = "previousClose",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub previous_close: ::std::option::Option<f64>,
        #[serde(
            rename = "previousCloseDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub previous_close_date: ::std::option::Option<i64>,
        #[serde(
            rename = "previousCloseDateISO8601",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub previous_close_date_iso8601: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for CandleList {
        fn default() -> Self {
            Self {
                candles: Default::default(),
                empty: Default::default(),
                previous_close: Default::default(),
                previous_close_date: Default::default(),
                previous_close_date_iso8601: Default::default(),
                symbol: Default::default(),
            }
        }
    }

    impl CandleList {
        pub fn builder() -> builder::CandleList {
            Default::default()
        }
    }

    ///Indicates call or put
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Indicates call or put",
    ///  "type": "string",
    ///  "enum": [
    ///    "P",
    ///    "C"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ContractType {
        P,
        C,
    }

    impl ::std::fmt::Display for ContractType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::P => f.write_str("P"),
                Self::C => f.write_str("C"),
            }
        }
    }

    impl ::std::str::FromStr for ContractType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "P" => Ok(Self::P),
                "C" => Ok(Self::C),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ContractType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ContractType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ContractType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Dividend frequency 1 – once a year or annually 2 – 2x a year or
    /// semi-annualy 3 - 3x a year (ex. ARCO, EBRPF) 4 – 4x a year or quarterly
    /// 6 - 6x per yr or every other month 11 – 11x a year (ex. FBND, FCOR) 12 –
    /// 12x a year or monthly
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Dividend frequency 1 – once a year or annually 2 – 2x a
    /// year or semi-annualy 3 - 3x a year (ex. ARCO, EBRPF) 4 – 4x a year or
    /// quarterly 6 - 6x per yr or every other month 11 – 11x a year (ex. FBND,
    /// FCOR) 12 – 12x a year or monthly",
    ///  "type": [
    ///    "integer",
    ///    "null"
    ///  ],
    ///  "enum": [
    ///    1,
    ///    2,
    ///    3,
    ///    4,
    ///    6,
    ///    11,
    ///    12,
    ///    null
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct DivFreq(pub ::std::option::Option<DivFreqInner>);
    impl ::std::ops::Deref for DivFreq {
        type Target = ::std::option::Option<DivFreqInner>;
        fn deref(&self) -> &::std::option::Option<DivFreqInner> {
            &self.0
        }
    }

    impl ::std::convert::From<DivFreq> for ::std::option::Option<DivFreqInner> {
        fn from(value: DivFreq) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::std::option::Option<DivFreqInner>> for DivFreq {
        fn from(value: ::std::option::Option<DivFreqInner>) -> Self {
            Self(value)
        }
    }

    ///Dividend frequency 1 – once a year or annually 2 – 2x a year or
    /// semi-annualy 3 - 3x a year (ex. ARCO, EBRPF) 4 – 4x a year or quarterly
    /// 6 - 6x per yr or every other month 11 – 11x a year (ex. FBND, FCOR) 12 –
    /// 12x a year or monthly
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Dividend frequency 1 – once a year or annually 2 – 2x a
    /// year or semi-annualy 3 - 3x a year (ex. ARCO, EBRPF) 4 – 4x a year or
    /// quarterly 6 - 6x per yr or every other month 11 – 11x a year (ex. FBND,
    /// FCOR) 12 – 12x a year or monthly",
    ///  "type": "integer",
    ///  "enum": [
    ///    1,
    ///    2,
    ///    3,
    ///    4,
    ///    6,
    ///    11,
    ///    12
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct DivFreqInner(i64);
    impl ::std::ops::Deref for DivFreqInner {
        type Target = i64;
        fn deref(&self) -> &i64 {
            &self.0
        }
    }

    impl ::std::convert::From<DivFreqInner> for i64 {
        fn from(value: DivFreqInner) -> Self {
            value.0
        }
    }

    impl ::std::convert::TryFrom<i64> for DivFreqInner {
        type Error = self::error::ConversionError;
        fn try_from(value: i64) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![1_i64, 2_i64, 3_i64, 4_i64, 6_i64, 11_i64, 12_i64].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for DivFreqInner {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i64>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }

    ///Asset Sub Type (only there if applicable)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Asset Sub Type (only there if applicable)",
    ///  "type": [
    ///    "string",
    ///    "null"
    ///  ],
    ///  "enum": [
    ///    "COE",
    ///    "PRF",
    ///    "ADR",
    ///    "GDR",
    ///    "CEF",
    ///    "ETF",
    ///    "ETN",
    ///    "UIT",
    ///    "WAR",
    ///    "RGT",
    ///    null
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct EquityAssetSubType(pub ::std::option::Option<EquityAssetSubTypeInner>);
    impl ::std::ops::Deref for EquityAssetSubType {
        type Target = ::std::option::Option<EquityAssetSubTypeInner>;
        fn deref(&self) -> &::std::option::Option<EquityAssetSubTypeInner> {
            &self.0
        }
    }

    impl ::std::convert::From<EquityAssetSubType> for ::std::option::Option<EquityAssetSubTypeInner> {
        fn from(value: EquityAssetSubType) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::std::option::Option<EquityAssetSubTypeInner>> for EquityAssetSubType {
        fn from(value: ::std::option::Option<EquityAssetSubTypeInner>) -> Self {
            Self(value)
        }
    }

    ///Asset Sub Type (only there if applicable)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Asset Sub Type (only there if applicable)",
    ///  "type": "string",
    ///  "enum": [
    ///    "COE",
    ///    "PRF",
    ///    "ADR",
    ///    "GDR",
    ///    "CEF",
    ///    "ETF",
    ///    "ETN",
    ///    "UIT",
    ///    "WAR",
    ///    "RGT"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum EquityAssetSubTypeInner {
        #[serde(rename = "COE")]
        Coe,
        #[serde(rename = "PRF")]
        Prf,
        #[serde(rename = "ADR")]
        Adr,
        #[serde(rename = "GDR")]
        Gdr,
        #[serde(rename = "CEF")]
        Cef,
        #[serde(rename = "ETF")]
        Etf,
        #[serde(rename = "ETN")]
        Etn,
        #[serde(rename = "UIT")]
        Uit,
        #[serde(rename = "WAR")]
        War,
        #[serde(rename = "RGT")]
        Rgt,
    }

    impl ::std::fmt::Display for EquityAssetSubTypeInner {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Coe => f.write_str("COE"),
                Self::Prf => f.write_str("PRF"),
                Self::Adr => f.write_str("ADR"),
                Self::Gdr => f.write_str("GDR"),
                Self::Cef => f.write_str("CEF"),
                Self::Etf => f.write_str("ETF"),
                Self::Etn => f.write_str("ETN"),
                Self::Uit => f.write_str("UIT"),
                Self::War => f.write_str("WAR"),
                Self::Rgt => f.write_str("RGT"),
            }
        }
    }

    impl ::std::str::FromStr for EquityAssetSubTypeInner {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "COE" => Ok(Self::Coe),
                "PRF" => Ok(Self::Prf),
                "ADR" => Ok(Self::Adr),
                "GDR" => Ok(Self::Gdr),
                "CEF" => Ok(Self::Cef),
                "ETF" => Ok(Self::Etf),
                "ETN" => Ok(Self::Etn),
                "UIT" => Ok(Self::Uit),
                "WAR" => Ok(Self::War),
                "RGT" => Ok(Self::Rgt),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for EquityAssetSubTypeInner {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for EquityAssetSubTypeInner {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for EquityAssetSubTypeInner {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Quote info of Equity security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Quote info of Equity security",
    ///  "type": "object",
    ///  "properties": {
    ///    "assetMainType": {
    ///      "$ref": "#/components/schemas/AssetMainType"
    ///    },
    ///    "assetSubType": {
    ///      "$ref": "#/components/schemas/EquityAssetSubType"
    ///    },
    ///    "extended": {
    ///      "$ref": "#/components/schemas/ExtendedMarket"
    ///    },
    ///    "fundamental": {
    ///      "$ref": "#/components/schemas/Fundamental"
    ///    },
    ///    "quote": {
    ///      "$ref": "#/components/schemas/QuoteEquity"
    ///    },
    ///    "quoteType": {
    ///      "$ref": "#/components/schemas/QuoteType"
    ///    },
    ///    "realtime": {
    ///      "description": "is quote realtime",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "reference": {
    ///      "$ref": "#/components/schemas/ReferenceEquity"
    ///    },
    ///    "regular": {
    ///      "$ref": "#/components/schemas/RegularMarket"
    ///    },
    ///    "ssid": {
    ///      "description": "SSID of instrument",
    ///      "examples": [
    ///        1234567890
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "symbol": {
    ///      "description": "Symbol of instrument",
    ///      "examples": [
    ///        "AAPL"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct EquityResponse {
        #[serde(
            rename = "assetMainType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub asset_main_type: ::std::option::Option<AssetMainType>,
        #[serde(
            rename = "assetSubType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub asset_sub_type: ::std::option::Option<EquityAssetSubType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub extended: ::std::option::Option<ExtendedMarket>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fundamental: ::std::option::Option<Fundamental>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quote: ::std::option::Option<QuoteEquity>,
        #[serde(
            rename = "quoteType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub quote_type: ::std::option::Option<QuoteType>,
        ///is quote realtime
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub realtime: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reference: ::std::option::Option<ReferenceEquity>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub regular: ::std::option::Option<RegularMarket>,
        ///SSID of instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ssid: ::std::option::Option<i64>,
        ///Symbol of instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for EquityResponse {
        fn default() -> Self {
            Self {
                asset_main_type: Default::default(),
                asset_sub_type: Default::default(),
                extended: Default::default(),
                fundamental: Default::default(),
                quote: Default::default(),
                quote_type: Default::default(),
                realtime: Default::default(),
                reference: Default::default(),
                regular: Default::default(),
                ssid: Default::default(),
                symbol: Default::default(),
            }
        }
    }

    impl EquityResponse {
        pub fn builder() -> builder::EquityResponse {
            Default::default()
        }
    }

    ///`Error`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "detail": {
    ///      "description": "Detailed error description.",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "Search combination should not exceed 500."
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "description": "Unique error id.",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "9821320c-8500-4edf-bd46-a9299c13d2e0"
    ///      ],
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "source": {
    ///      "$ref": "#/components/schemas/ErrorSource"
    ///    },
    ///    "status": {
    ///      "description": "The HTTP status code .",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "400"
    ///      ],
    ///      "type": "string",
    ///      "enum": [
    ///        "400",
    ///        "401",
    ///        "404",
    ///        "500"
    ///      ]
    ///    },
    ///    "title": {
    ///      "description": "Short error description.",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "Missing header"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Error {
        ///Detailed error description.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub detail: ::std::option::Option<::std::string::String>,
        ///Unique error id.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub id: ::std::option::Option<::uuid::Uuid>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub source: ::std::option::Option<ErrorSource>,
        ///The HTTP status code .
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<ErrorStatus>,
        ///Short error description.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for Error {
        fn default() -> Self {
            Self {
                detail: Default::default(),
                id: Default::default(),
                source: Default::default(),
                status: Default::default(),
                title: Default::default(),
            }
        }
    }

    impl Error {
        pub fn builder() -> builder::Error {
            Default::default()
        }
    }

    ///`ErrorResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "errors": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Error"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ErrorResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub errors: ::std::vec::Vec<Error>,
    }

    impl ::std::default::Default for ErrorResponse {
        fn default() -> Self {
            Self {
                errors: Default::default(),
            }
        }
    }

    impl ErrorResponse {
        pub fn builder() -> builder::ErrorResponse {
            Default::default()
        }
    }

    ///Who is responsible for triggering these errors.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Who is responsible for triggering these errors.",
    ///  "type": "object",
    ///  "properties": {
    ///    "header": {
    ///      "description": "header name which lead to this error message.",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "Schwab-Client-CorrelId"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "parameter": {
    ///      "description": "parameter name which lead to this error message.",
    ///      "readOnly": true,
    ///      "examples": [
    ///        "fields"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "pointer": {
    ///      "description": "list of attributes which lead to this error
    /// message.",
    ///      "readOnly": true,
    ///      "examples": [
    ///        [
    ///          "/data/attributes/symbols",
    ///          "/data/attributes/cusips",
    ///          "/data/attributes/ssids"
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ErrorSource {
        ///header name which lead to this error message.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub header: ::std::option::Option<::std::string::String>,
        ///parameter name which lead to this error message.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub parameter: ::std::option::Option<::std::string::String>,
        ///list of attributes which lead to this error message.
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub pointer: ::std::vec::Vec<::std::string::String>,
    }

    impl ::std::default::Default for ErrorSource {
        fn default() -> Self {
            Self {
                header: Default::default(),
                parameter: Default::default(),
                pointer: Default::default(),
            }
        }
    }

    impl ErrorSource {
        pub fn builder() -> builder::ErrorSource {
            Default::default()
        }
    }

    ///The HTTP status code .
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "The HTTP status code .",
    ///  "readOnly": true,
    ///  "examples": [
    ///    "400"
    ///  ],
    ///  "type": "string",
    ///  "enum": [
    ///    "400",
    ///    "401",
    ///    "404",
    ///    "500"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ErrorStatus {
        #[serde(rename = "400")]
        X400,
        #[serde(rename = "401")]
        X401,
        #[serde(rename = "404")]
        X404,
        #[serde(rename = "500")]
        X500,
    }

    impl ::std::fmt::Display for ErrorStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::X400 => f.write_str("400"),
                Self::X401 => f.write_str("401"),
                Self::X404 => f.write_str("404"),
                Self::X500 => f.write_str("500"),
            }
        }
    }

    impl ::std::str::FromStr for ErrorStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "400" => Ok(Self::X400),
                "401" => Ok(Self::X401),
                "404" => Ok(Self::X404),
                "500" => Ok(Self::X500),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ErrorStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ErrorStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ErrorStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///option contract exercise type America or European
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "option contract exercise type America or European",
    ///  "type": "string",
    ///  "enum": [
    ///    "A",
    ///    "E"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ExerciseType {
        A,
        E,
    }

    impl ::std::fmt::Display for ExerciseType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::A => f.write_str("A"),
                Self::E => f.write_str("E"),
            }
        }
    }

    impl ::std::str::FromStr for ExerciseType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "A" => Ok(Self::A),
                "E" => Ok(Self::E),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ExerciseType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ExerciseType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ExerciseType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///expiration type
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "expiration type",
    ///  "type": "object",
    ///  "properties": {
    ///    "daysToExpiration": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "expiration": {
    ///      "type": "string"
    ///    },
    ///    "expirationType": {
    ///      "$ref": "#/components/schemas/ExpirationType"
    ///    },
    ///    "optionRoots": {
    ///      "type": "string"
    ///    },
    ///    "settlementType": {
    ///      "$ref": "#/components/schemas/SettlementType"
    ///    },
    ///    "standard": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Expiration {
        #[serde(
            rename = "daysToExpiration",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub days_to_expiration: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub expiration: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "expirationType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expiration_type: ::std::option::Option<ExpirationType>,
        #[serde(
            rename = "optionRoots",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub option_roots: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "settlementType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub settlement_type: ::std::option::Option<SettlementType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub standard: ::std::option::Option<bool>,
    }

    impl ::std::default::Default for Expiration {
        fn default() -> Self {
            Self {
                days_to_expiration: Default::default(),
                expiration: Default::default(),
                expiration_type: Default::default(),
                option_roots: Default::default(),
                settlement_type: Default::default(),
                standard: Default::default(),
            }
        }
    }

    impl Expiration {
        pub fn builder() -> builder::Expiration {
            Default::default()
        }
    }

    ///`ExpirationChain`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "expirationList": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Expiration"
    ///      }
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExpirationChain {
        #[serde(
            rename = "expirationList",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub expiration_list: ::std::vec::Vec<Expiration>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for ExpirationChain {
        fn default() -> Self {
            Self {
                expiration_list: Default::default(),
                status: Default::default(),
            }
        }
    }

    impl ExpirationChain {
        pub fn builder() -> builder::ExpirationChain {
            Default::default()
        }
    }

    ///M for End Of Month Expiration Calendar Cycle. (To match the last
    /// business day of the month), Q for Quarterly expirations (last business
    /// day of the quarter month MAR/JUN/SEP/DEC), W for Weekly expiration (also
    /// called Friday Short Term Expirations) and S for Expires 3rd Friday of
    /// the month (also known as regular options).
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "M for End Of Month Expiration Calendar Cycle. (To match
    /// the last business day of the month), Q for Quarterly expirations (last
    /// business day of the quarter month MAR/JUN/SEP/DEC), W for Weekly
    /// expiration (also called Friday Short Term Expirations) and S for Expires
    /// 3rd Friday of the month (also known as regular options).",
    ///  "type": "string",
    ///  "enum": [
    ///    "M",
    ///    "Q",
    ///    "S",
    ///    "W"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ExpirationType {
        M,
        Q,
        S,
        W,
    }

    impl ::std::fmt::Display for ExpirationType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::M => f.write_str("M"),
                Self::Q => f.write_str("Q"),
                Self::S => f.write_str("S"),
                Self::W => f.write_str("W"),
            }
        }
    }

    impl ::std::str::FromStr for ExpirationType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "M" => Ok(Self::M),
                "Q" => Ok(Self::Q),
                "S" => Ok(Self::S),
                "W" => Ok(Self::W),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ExpirationType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ExpirationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ExpirationType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Quote data for extended hours
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Quote data for extended hours",
    ///  "type": "object",
    ///  "properties": {
    ///    "askPrice": {
    ///      "description": "Extended market ask price",
    ///      "examples": [
    ///        124.85
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "askSize": {
    ///      "description": "Extended market ask size",
    ///      "examples": [
    ///        51771
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "bidPrice": {
    ///      "description": "Extended market bid price",
    ///      "examples": [
    ///        124.85
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "bidSize": {
    ///      "description": "Extended market bid size",
    ///      "examples": [
    ///        51771
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "lastPrice": {
    ///      "description": "Extended market last price",
    ///      "examples": [
    ///        124.85
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "lastSize": {
    ///      "description": "Regular market last size",
    ///      "examples": [
    ///        51771
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "mark": {
    ///      "description": "mark price",
    ///      "examples": [
    ///        1.1246
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "quoteTime": {
    ///      "description": "Extended market quote time in milliseconds since
    /// Epoch",
    ///      "examples": [
    ///        1621368000400
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "totalVolume": {
    ///      "description": "Total volume",
    ///      "examples": [
    ///        12345
    ///      ],
    ///      "type": "number",
    ///      "format": "int64"
    ///    },
    ///    "tradeTime": {
    ///      "description": "Extended market trade time in milliseconds since
    /// Epoch",
    ///      "examples": [
    ///        1621368000400
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExtendedMarket {
        #[serde(
            rename = "askPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_price: ::std::option::Option<f64>,
        ///Extended market ask size
        #[serde(
            rename = "askSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_size: ::std::option::Option<i32>,
        #[serde(
            rename = "bidPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_price: ::std::option::Option<f64>,
        ///Extended market bid size
        #[serde(
            rename = "bidSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_size: ::std::option::Option<i32>,
        #[serde(
            rename = "lastPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_price: ::std::option::Option<f64>,
        ///Regular market last size
        #[serde(
            rename = "lastSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_size: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mark: ::std::option::Option<f64>,
        ///Extended market quote time in milliseconds since Epoch
        #[serde(
            rename = "quoteTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub quote_time: ::std::option::Option<i64>,
        #[serde(
            rename = "totalVolume",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_volume: ::std::option::Option<f64>,
        ///Extended market trade time in milliseconds since Epoch
        #[serde(
            rename = "tradeTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trade_time: ::std::option::Option<i64>,
    }

    impl ::std::default::Default for ExtendedMarket {
        fn default() -> Self {
            Self {
                ask_price: Default::default(),
                ask_size: Default::default(),
                bid_price: Default::default(),
                bid_size: Default::default(),
                last_price: Default::default(),
                last_size: Default::default(),
                mark: Default::default(),
                quote_time: Default::default(),
                total_volume: Default::default(),
                trade_time: Default::default(),
            }
        }
    }

    impl ExtendedMarket {
        pub fn builder() -> builder::ExtendedMarket {
            Default::default()
        }
    }

    ///Quote info of Forex security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Quote info of Forex security",
    ///  "type": "object",
    ///  "properties": {
    ///    "assetMainType": {
    ///      "$ref": "#/components/schemas/AssetMainType"
    ///    },
    ///    "quote": {
    ///      "$ref": "#/components/schemas/QuoteForex"
    ///    },
    ///    "realtime": {
    ///      "description": "is quote realtime",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "reference": {
    ///      "$ref": "#/components/schemas/ReferenceForex"
    ///    },
    ///    "ssid": {
    ///      "description": "SSID of instrument",
    ///      "examples": [
    ///        1234567890
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "symbol": {
    ///      "description": "Symbol of instrument",
    ///      "examples": [
    ///        "AAPL"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ForexResponse {
        #[serde(
            rename = "assetMainType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub asset_main_type: ::std::option::Option<AssetMainType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quote: ::std::option::Option<QuoteForex>,
        ///is quote realtime
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub realtime: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reference: ::std::option::Option<ReferenceForex>,
        ///SSID of instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ssid: ::std::option::Option<i64>,
        ///Symbol of instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for ForexResponse {
        fn default() -> Self {
            Self {
                asset_main_type: Default::default(),
                quote: Default::default(),
                realtime: Default::default(),
                reference: Default::default(),
                ssid: Default::default(),
                symbol: Default::default(),
            }
        }
    }

    impl ForexResponse {
        pub fn builder() -> builder::ForexResponse {
            Default::default()
        }
    }

    ///FundStrategy "A" - Active "L" - Leveraged "P" - Passive "Q" -
    /// Quantitative "S" - Short
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "FundStrategy \"A\" - Active \"L\" - Leveraged \"P\" -
    /// Passive \"Q\" - Quantitative \"S\" - Short",
    ///  "type": [
    ///    "string",
    ///    "null"
    ///  ],
    ///  "enum": [
    ///    "A",
    ///    "L",
    ///    "P",
    ///    "Q",
    ///    "S",
    ///    null
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct FundStrategy(pub ::std::option::Option<FundStrategyInner>);
    impl ::std::ops::Deref for FundStrategy {
        type Target = ::std::option::Option<FundStrategyInner>;
        fn deref(&self) -> &::std::option::Option<FundStrategyInner> {
            &self.0
        }
    }

    impl ::std::convert::From<FundStrategy> for ::std::option::Option<FundStrategyInner> {
        fn from(value: FundStrategy) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::std::option::Option<FundStrategyInner>> for FundStrategy {
        fn from(value: ::std::option::Option<FundStrategyInner>) -> Self {
            Self(value)
        }
    }

    ///FundStrategy "A" - Active "L" - Leveraged "P" - Passive "Q" -
    /// Quantitative "S" - Short
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "FundStrategy \"A\" - Active \"L\" - Leveraged \"P\" -
    /// Passive \"Q\" - Quantitative \"S\" - Short",
    ///  "type": "string",
    ///  "enum": [
    ///    "A",
    ///    "L",
    ///    "P",
    ///    "Q",
    ///    "S"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum FundStrategyInner {
        A,
        L,
        P,
        Q,
        S,
    }

    impl ::std::fmt::Display for FundStrategyInner {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::A => f.write_str("A"),
                Self::L => f.write_str("L"),
                Self::P => f.write_str("P"),
                Self::Q => f.write_str("Q"),
                Self::S => f.write_str("S"),
            }
        }
    }

    impl ::std::str::FromStr for FundStrategyInner {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "A" => Ok(Self::A),
                "L" => Ok(Self::L),
                "P" => Ok(Self::P),
                "Q" => Ok(Self::Q),
                "S" => Ok(Self::S),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for FundStrategyInner {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for FundStrategyInner {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for FundStrategyInner {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Fundamentals of a security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Fundamentals of a security",
    ///  "type": "object",
    ///  "properties": {
    ///    "avg10DaysVolume": {
    ///      "description": "Average 10 day volume",
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "avg1YearVolume": {
    ///      "description": "Average 1 day volume",
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "declarationDate": {
    ///      "description": "Declaration date in yyyy-mm-ddThh:mm:ssZ",
    ///      "examples": [
    ///        "2021-04-28T00:00:00Z"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time",
    ///      "pattern": "yyyy-MM-dd'T'HH:mm:ssZ"
    ///    },
    ///    "divAmount": {
    ///      "description": "Dividend Amount",
    ///      "examples": [
    ///        0.88
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "divExDate": {
    ///      "description": "Dividend date in yyyy-mm-ddThh:mm:ssZ",
    ///      "examples": [
    ///        "2021-05-07T00:00:00Z"
    ///      ],
    ///      "type": "string",
    ///      "format": "yyyy-MM-dd'T'HH:mm:ssZ"
    ///    },
    ///    "divFreq": {
    ///      "$ref": "#/components/schemas/DivFreq"
    ///    },
    ///    "divPayAmount": {
    ///      "description": "Dividend Pay Amount",
    ///      "examples": [
    ///        0.22
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "divPayDate": {
    ///      "description": "Dividend pay date in yyyy-mm-ddThh:mm:ssZ",
    ///      "examples": [
    ///        "2021-05-13T00:00:00Z"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time",
    ///      "pattern": "yyyy-MM-dd'T'HH:mm:ssZ"
    ///    },
    ///    "divYield": {
    ///      "description": "Dividend yield",
    ///      "examples": [
    ///        0.7
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "eps": {
    ///      "description": "Earnings per Share",
    ///      "examples": [
    ///        4.45645
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "fundLeverageFactor": {
    ///      "description": "Fund Leverage Factor + > 0 <-",
    ///      "examples": [
    ///        -1
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "fundStrategy": {
    ///      "$ref": "#/components/schemas/FundStrategy"
    ///    },
    ///    "nextDivExDate": {
    ///      "description": "Next Dividend date",
    ///      "examples": [
    ///        "2021-02-12T00:00:00Z"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time",
    ///      "pattern": "yyyy-MM-dd'T'HH:mm:ssZ"
    ///    },
    ///    "nextDivPayDate": {
    ///      "description": "Next Dividend pay date",
    ///      "examples": [
    ///        "2021-02-12T00:00:00Z"
    ///      ],
    ///      "type": "string",
    ///      "format": "date-time",
    ///      "pattern": "yyyy-MM-dd'T'HH:mm:ssZ"
    ///    },
    ///    "peRatio": {
    ///      "description": "P/E Ratio",
    ///      "examples": [
    ///        28.599
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Fundamental {
        #[serde(
            rename = "avg10DaysVolume",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub avg10_days_volume: ::std::option::Option<f64>,
        #[serde(
            rename = "avg1YearVolume",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub avg1_year_volume: ::std::option::Option<f64>,
        ///Declaration date in yyyy-mm-ddThh:mm:ssZ
        #[serde(
            rename = "declarationDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub declaration_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "divAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub div_amount: ::std::option::Option<f64>,
        ///Dividend date in yyyy-mm-ddThh:mm:ssZ
        #[serde(
            rename = "divExDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub div_ex_date: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "divFreq",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub div_freq: ::std::option::Option<DivFreq>,
        #[serde(
            rename = "divPayAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub div_pay_amount: ::std::option::Option<f64>,
        ///Dividend pay date in yyyy-mm-ddThh:mm:ssZ
        #[serde(
            rename = "divPayDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub div_pay_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "divYield",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub div_yield: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub eps: ::std::option::Option<f64>,
        #[serde(
            rename = "fundLeverageFactor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub fund_leverage_factor: ::std::option::Option<f64>,
        #[serde(
            rename = "fundStrategy",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub fund_strategy: ::std::option::Option<FundStrategy>,
        ///Next Dividend date
        #[serde(
            rename = "nextDivExDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_div_ex_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        ///Next Dividend pay date
        #[serde(
            rename = "nextDivPayDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_div_pay_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "peRatio",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub pe_ratio: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for Fundamental {
        fn default() -> Self {
            Self {
                avg10_days_volume: Default::default(),
                avg1_year_volume: Default::default(),
                declaration_date: Default::default(),
                div_amount: Default::default(),
                div_ex_date: Default::default(),
                div_freq: Default::default(),
                div_pay_amount: Default::default(),
                div_pay_date: Default::default(),
                div_yield: Default::default(),
                eps: Default::default(),
                fund_leverage_factor: Default::default(),
                fund_strategy: Default::default(),
                next_div_ex_date: Default::default(),
                next_div_pay_date: Default::default(),
                pe_ratio: Default::default(),
            }
        }
    }

    impl Fundamental {
        pub fn builder() -> builder::Fundamental {
            Default::default()
        }
    }

    ///`FundamentalInst`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "avg10DaysVolume": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "avg1DayVolume": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "avg3MonthVolume": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "beta": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "bookValuePerShare": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "corpactionDate": {
    ///      "type": "string"
    ///    },
    ///    "currentRatio": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "declarationDate": {
    ///      "type": "string"
    ///    },
    ///    "divGrowthRate3Year": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "dividendAmount": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "dividendDate": {
    ///      "type": "string"
    ///    },
    ///    "dividendFreq": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "dividendPayAmount": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "dividendPayDate": {
    ///      "type": "string"
    ///    },
    ///    "dividendYield": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "dtnVolume": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "eps": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "epsChange": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "epsChangePercentTTM": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "epsChangeYear": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "epsTTM": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "fundLeverageFactor": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "fundStrategy": {
    ///      "type": "string"
    ///    },
    ///    "grossMarginMRQ": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "grossMarginTTM": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "high52": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "interestCoverage": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "low52": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "ltDebtToEquity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "marketCap": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "marketCapFloat": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "netProfitMarginMRQ": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "netProfitMarginTTM": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "nextDividendDate": {
    ///      "type": "string"
    ///    },
    ///    "nextDividendPayDate": {
    ///      "type": "string"
    ///    },
    ///    "operatingMarginMRQ": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "operatingMarginTTM": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "pbRatio": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "pcfRatio": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "peRatio": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "pegRatio": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "prRatio": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "quickRatio": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "returnOnAssets": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "returnOnEquity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "returnOnInvestment": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "revChangeIn": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "revChangeTTM": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "revChangeYear": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "sharesOutstanding": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "shortIntDayToCover": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "shortIntToFloat": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    },
    ///    "totalDebtToCapital": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "totalDebtToEquity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "vol10DayAvg": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "vol1DayAvg": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "vol3MonthAvg": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FundamentalInst {
        #[serde(
            rename = "avg10DaysVolume",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub avg10_days_volume: ::std::option::Option<i64>,
        #[serde(
            rename = "avg1DayVolume",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub avg1_day_volume: ::std::option::Option<i64>,
        #[serde(
            rename = "avg3MonthVolume",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub avg3_month_volume: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub beta: ::std::option::Option<f64>,
        #[serde(
            rename = "bookValuePerShare",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub book_value_per_share: ::std::option::Option<f64>,
        #[serde(
            rename = "corpactionDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub corpaction_date: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "currentRatio",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub current_ratio: ::std::option::Option<f64>,
        #[serde(
            rename = "declarationDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub declaration_date: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "divGrowthRate3Year",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub div_growth_rate3_year: ::std::option::Option<f64>,
        #[serde(
            rename = "dividendAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dividend_amount: ::std::option::Option<f64>,
        #[serde(
            rename = "dividendDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dividend_date: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "dividendFreq",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dividend_freq: ::std::option::Option<i32>,
        #[serde(
            rename = "dividendPayAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dividend_pay_amount: ::std::option::Option<f64>,
        #[serde(
            rename = "dividendPayDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dividend_pay_date: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "dividendYield",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dividend_yield: ::std::option::Option<f64>,
        #[serde(
            rename = "dtnVolume",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub dtn_volume: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub eps: ::std::option::Option<f64>,
        #[serde(
            rename = "epsChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub eps_change: ::std::option::Option<f64>,
        #[serde(
            rename = "epsChangePercentTTM",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub eps_change_percent_ttm: ::std::option::Option<f64>,
        #[serde(
            rename = "epsChangeYear",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub eps_change_year: ::std::option::Option<f64>,
        #[serde(
            rename = "epsTTM",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub eps_ttm: ::std::option::Option<f64>,
        #[serde(
            rename = "fundLeverageFactor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub fund_leverage_factor: ::std::option::Option<f64>,
        #[serde(
            rename = "fundStrategy",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub fund_strategy: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "grossMarginMRQ",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub gross_margin_mrq: ::std::option::Option<f64>,
        #[serde(
            rename = "grossMarginTTM",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub gross_margin_ttm: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub high52: ::std::option::Option<f64>,
        #[serde(
            rename = "interestCoverage",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub interest_coverage: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub low52: ::std::option::Option<f64>,
        #[serde(
            rename = "ltDebtToEquity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub lt_debt_to_equity: ::std::option::Option<f64>,
        #[serde(
            rename = "marketCap",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub market_cap: ::std::option::Option<f64>,
        #[serde(
            rename = "marketCapFloat",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub market_cap_float: ::std::option::Option<f64>,
        #[serde(
            rename = "netProfitMarginMRQ",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_profit_margin_mrq: ::std::option::Option<f64>,
        #[serde(
            rename = "netProfitMarginTTM",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_profit_margin_ttm: ::std::option::Option<f64>,
        #[serde(
            rename = "nextDividendDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_dividend_date: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "nextDividendPayDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub next_dividend_pay_date: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "operatingMarginMRQ",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub operating_margin_mrq: ::std::option::Option<f64>,
        #[serde(
            rename = "operatingMarginTTM",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub operating_margin_ttm: ::std::option::Option<f64>,
        #[serde(
            rename = "pbRatio",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub pb_ratio: ::std::option::Option<f64>,
        #[serde(
            rename = "pcfRatio",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub pcf_ratio: ::std::option::Option<f64>,
        #[serde(
            rename = "peRatio",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub pe_ratio: ::std::option::Option<f64>,
        #[serde(
            rename = "pegRatio",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub peg_ratio: ::std::option::Option<f64>,
        #[serde(
            rename = "prRatio",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub pr_ratio: ::std::option::Option<f64>,
        #[serde(
            rename = "quickRatio",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub quick_ratio: ::std::option::Option<f64>,
        #[serde(
            rename = "returnOnAssets",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub return_on_assets: ::std::option::Option<f64>,
        #[serde(
            rename = "returnOnEquity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub return_on_equity: ::std::option::Option<f64>,
        #[serde(
            rename = "returnOnInvestment",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub return_on_investment: ::std::option::Option<f64>,
        #[serde(
            rename = "revChangeIn",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub rev_change_in: ::std::option::Option<f64>,
        #[serde(
            rename = "revChangeTTM",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub rev_change_ttm: ::std::option::Option<f64>,
        #[serde(
            rename = "revChangeYear",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub rev_change_year: ::std::option::Option<f64>,
        #[serde(
            rename = "sharesOutstanding",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub shares_outstanding: ::std::option::Option<f64>,
        #[serde(
            rename = "shortIntDayToCover",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short_int_day_to_cover: ::std::option::Option<f64>,
        #[serde(
            rename = "shortIntToFloat",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short_int_to_float: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "totalDebtToCapital",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_debt_to_capital: ::std::option::Option<f64>,
        #[serde(
            rename = "totalDebtToEquity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_debt_to_equity: ::std::option::Option<f64>,
        #[serde(
            rename = "vol10DayAvg",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub vol10_day_avg: ::std::option::Option<f64>,
        #[serde(
            rename = "vol1DayAvg",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub vol1_day_avg: ::std::option::Option<f64>,
        #[serde(
            rename = "vol3MonthAvg",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub vol3_month_avg: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for FundamentalInst {
        fn default() -> Self {
            Self {
                avg10_days_volume: Default::default(),
                avg1_day_volume: Default::default(),
                avg3_month_volume: Default::default(),
                beta: Default::default(),
                book_value_per_share: Default::default(),
                corpaction_date: Default::default(),
                current_ratio: Default::default(),
                declaration_date: Default::default(),
                div_growth_rate3_year: Default::default(),
                dividend_amount: Default::default(),
                dividend_date: Default::default(),
                dividend_freq: Default::default(),
                dividend_pay_amount: Default::default(),
                dividend_pay_date: Default::default(),
                dividend_yield: Default::default(),
                dtn_volume: Default::default(),
                eps: Default::default(),
                eps_change: Default::default(),
                eps_change_percent_ttm: Default::default(),
                eps_change_year: Default::default(),
                eps_ttm: Default::default(),
                fund_leverage_factor: Default::default(),
                fund_strategy: Default::default(),
                gross_margin_mrq: Default::default(),
                gross_margin_ttm: Default::default(),
                high52: Default::default(),
                interest_coverage: Default::default(),
                low52: Default::default(),
                lt_debt_to_equity: Default::default(),
                market_cap: Default::default(),
                market_cap_float: Default::default(),
                net_profit_margin_mrq: Default::default(),
                net_profit_margin_ttm: Default::default(),
                next_dividend_date: Default::default(),
                next_dividend_pay_date: Default::default(),
                operating_margin_mrq: Default::default(),
                operating_margin_ttm: Default::default(),
                pb_ratio: Default::default(),
                pcf_ratio: Default::default(),
                pe_ratio: Default::default(),
                peg_ratio: Default::default(),
                pr_ratio: Default::default(),
                quick_ratio: Default::default(),
                return_on_assets: Default::default(),
                return_on_equity: Default::default(),
                return_on_investment: Default::default(),
                rev_change_in: Default::default(),
                rev_change_ttm: Default::default(),
                rev_change_year: Default::default(),
                shares_outstanding: Default::default(),
                short_int_day_to_cover: Default::default(),
                short_int_to_float: Default::default(),
                symbol: Default::default(),
                total_debt_to_capital: Default::default(),
                total_debt_to_equity: Default::default(),
                vol10_day_avg: Default::default(),
                vol1_day_avg: Default::default(),
                vol3_month_avg: Default::default(),
            }
        }
    }

    impl FundamentalInst {
        pub fn builder() -> builder::FundamentalInst {
            Default::default()
        }
    }

    ///Quote info of Future Option security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Quote info of Future Option security",
    ///  "type": "object",
    ///  "properties": {
    ///    "assetMainType": {
    ///      "$ref": "#/components/schemas/AssetMainType"
    ///    },
    ///    "quote": {
    ///      "$ref": "#/components/schemas/QuoteFutureOption"
    ///    },
    ///    "realtime": {
    ///      "description": "is quote realtime",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "reference": {
    ///      "$ref": "#/components/schemas/ReferenceFutureOption"
    ///    },
    ///    "ssid": {
    ///      "description": "SSID of instrument",
    ///      "examples": [
    ///        1234567890
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "symbol": {
    ///      "description": "Symbol of instrument",
    ///      "examples": [
    ///        "AAPL"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FutureOptionResponse {
        #[serde(
            rename = "assetMainType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub asset_main_type: ::std::option::Option<AssetMainType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quote: ::std::option::Option<QuoteFutureOption>,
        ///is quote realtime
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub realtime: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reference: ::std::option::Option<ReferenceFutureOption>,
        ///SSID of instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ssid: ::std::option::Option<i64>,
        ///Symbol of instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for FutureOptionResponse {
        fn default() -> Self {
            Self {
                asset_main_type: Default::default(),
                quote: Default::default(),
                realtime: Default::default(),
                reference: Default::default(),
                ssid: Default::default(),
                symbol: Default::default(),
            }
        }
    }

    impl FutureOptionResponse {
        pub fn builder() -> builder::FutureOptionResponse {
            Default::default()
        }
    }

    ///Quote info of Future security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Quote info of Future security",
    ///  "type": "object",
    ///  "properties": {
    ///    "assetMainType": {
    ///      "$ref": "#/components/schemas/AssetMainType"
    ///    },
    ///    "quote": {
    ///      "$ref": "#/components/schemas/QuoteFuture"
    ///    },
    ///    "realtime": {
    ///      "description": "is quote realtime",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "reference": {
    ///      "$ref": "#/components/schemas/ReferenceFuture"
    ///    },
    ///    "ssid": {
    ///      "description": "SSID of instrument",
    ///      "examples": [
    ///        1234567890
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "symbol": {
    ///      "description": "Symbol of instrument",
    ///      "examples": [
    ///        "AAPL"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FutureResponse {
        #[serde(
            rename = "assetMainType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub asset_main_type: ::std::option::Option<AssetMainType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quote: ::std::option::Option<QuoteFuture>,
        ///is quote realtime
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub realtime: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reference: ::std::option::Option<ReferenceFuture>,
        ///SSID of instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ssid: ::std::option::Option<i64>,
        ///Symbol of instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for FutureResponse {
        fn default() -> Self {
            Self {
                asset_main_type: Default::default(),
                quote: Default::default(),
                realtime: Default::default(),
                reference: Default::default(),
                ssid: Default::default(),
                symbol: Default::default(),
            }
        }
    }

    impl FutureResponse {
        pub fn builder() -> builder::FutureResponse {
            Default::default()
        }
    }

    ///`GetChainContractType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "CALL",
    ///    "PUT",
    ///    "ALL"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetChainContractType {
        #[serde(rename = "CALL")]
        Call,
        #[serde(rename = "PUT")]
        Put,
        #[serde(rename = "ALL")]
        All,
    }

    impl ::std::fmt::Display for GetChainContractType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Call => f.write_str("CALL"),
                Self::Put => f.write_str("PUT"),
                Self::All => f.write_str("ALL"),
            }
        }
    }

    impl ::std::str::FromStr for GetChainContractType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "CALL" => Ok(Self::Call),
                "PUT" => Ok(Self::Put),
                "ALL" => Ok(Self::All),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetChainContractType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetChainContractType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetChainContractType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetChainEntitlement`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "PN",
    ///    "NP",
    ///    "PP"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetChainEntitlement {
        #[serde(rename = "PN")]
        Pn,
        #[serde(rename = "NP")]
        Np,
        #[serde(rename = "PP")]
        Pp,
    }

    impl ::std::fmt::Display for GetChainEntitlement {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Pn => f.write_str("PN"),
                Self::Np => f.write_str("NP"),
                Self::Pp => f.write_str("PP"),
            }
        }
    }

    impl ::std::str::FromStr for GetChainEntitlement {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "PN" => Ok(Self::Pn),
                "NP" => Ok(Self::Np),
                "PP" => Ok(Self::Pp),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetChainEntitlement {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetChainEntitlement {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetChainEntitlement {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetChainExpMonth`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "JAN",
    ///    "FEB",
    ///    "MAR",
    ///    "APR",
    ///    "MAY",
    ///    "JUN",
    ///    "JUL",
    ///    "AUG",
    ///    "SEP",
    ///    "OCT",
    ///    "NOV",
    ///    "DEC",
    ///    "ALL"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetChainExpMonth {
        #[serde(rename = "JAN")]
        Jan,
        #[serde(rename = "FEB")]
        Feb,
        #[serde(rename = "MAR")]
        Mar,
        #[serde(rename = "APR")]
        Apr,
        #[serde(rename = "MAY")]
        May,
        #[serde(rename = "JUN")]
        Jun,
        #[serde(rename = "JUL")]
        Jul,
        #[serde(rename = "AUG")]
        Aug,
        #[serde(rename = "SEP")]
        Sep,
        #[serde(rename = "OCT")]
        Oct,
        #[serde(rename = "NOV")]
        Nov,
        #[serde(rename = "DEC")]
        Dec,
        #[serde(rename = "ALL")]
        All,
    }

    impl ::std::fmt::Display for GetChainExpMonth {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Jan => f.write_str("JAN"),
                Self::Feb => f.write_str("FEB"),
                Self::Mar => f.write_str("MAR"),
                Self::Apr => f.write_str("APR"),
                Self::May => f.write_str("MAY"),
                Self::Jun => f.write_str("JUN"),
                Self::Jul => f.write_str("JUL"),
                Self::Aug => f.write_str("AUG"),
                Self::Sep => f.write_str("SEP"),
                Self::Oct => f.write_str("OCT"),
                Self::Nov => f.write_str("NOV"),
                Self::Dec => f.write_str("DEC"),
                Self::All => f.write_str("ALL"),
            }
        }
    }

    impl ::std::str::FromStr for GetChainExpMonth {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "JAN" => Ok(Self::Jan),
                "FEB" => Ok(Self::Feb),
                "MAR" => Ok(Self::Mar),
                "APR" => Ok(Self::Apr),
                "MAY" => Ok(Self::May),
                "JUN" => Ok(Self::Jun),
                "JUL" => Ok(Self::Jul),
                "AUG" => Ok(Self::Aug),
                "SEP" => Ok(Self::Sep),
                "OCT" => Ok(Self::Oct),
                "NOV" => Ok(Self::Nov),
                "DEC" => Ok(Self::Dec),
                "ALL" => Ok(Self::All),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetChainExpMonth {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetChainExpMonth {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetChainExpMonth {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetChainStrategy`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "SINGLE",
    ///    "ANALYTICAL",
    ///    "COVERED",
    ///    "VERTICAL",
    ///    "CALENDAR",
    ///    "STRANGLE",
    ///    "STRADDLE",
    ///    "BUTTERFLY",
    ///    "CONDOR",
    ///    "DIAGONAL",
    ///    "COLLAR",
    ///    "ROLL"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetChainStrategy {
        #[serde(rename = "SINGLE")]
        Single,
        #[serde(rename = "ANALYTICAL")]
        Analytical,
        #[serde(rename = "COVERED")]
        Covered,
        #[serde(rename = "VERTICAL")]
        Vertical,
        #[serde(rename = "CALENDAR")]
        Calendar,
        #[serde(rename = "STRANGLE")]
        Strangle,
        #[serde(rename = "STRADDLE")]
        Straddle,
        #[serde(rename = "BUTTERFLY")]
        Butterfly,
        #[serde(rename = "CONDOR")]
        Condor,
        #[serde(rename = "DIAGONAL")]
        Diagonal,
        #[serde(rename = "COLLAR")]
        Collar,
        #[serde(rename = "ROLL")]
        Roll,
    }

    impl ::std::fmt::Display for GetChainStrategy {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Single => f.write_str("SINGLE"),
                Self::Analytical => f.write_str("ANALYTICAL"),
                Self::Covered => f.write_str("COVERED"),
                Self::Vertical => f.write_str("VERTICAL"),
                Self::Calendar => f.write_str("CALENDAR"),
                Self::Strangle => f.write_str("STRANGLE"),
                Self::Straddle => f.write_str("STRADDLE"),
                Self::Butterfly => f.write_str("BUTTERFLY"),
                Self::Condor => f.write_str("CONDOR"),
                Self::Diagonal => f.write_str("DIAGONAL"),
                Self::Collar => f.write_str("COLLAR"),
                Self::Roll => f.write_str("ROLL"),
            }
        }
    }

    impl ::std::str::FromStr for GetChainStrategy {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "SINGLE" => Ok(Self::Single),
                "ANALYTICAL" => Ok(Self::Analytical),
                "COVERED" => Ok(Self::Covered),
                "VERTICAL" => Ok(Self::Vertical),
                "CALENDAR" => Ok(Self::Calendar),
                "STRANGLE" => Ok(Self::Strangle),
                "STRADDLE" => Ok(Self::Straddle),
                "BUTTERFLY" => Ok(Self::Butterfly),
                "CONDOR" => Ok(Self::Condor),
                "DIAGONAL" => Ok(Self::Diagonal),
                "COLLAR" => Ok(Self::Collar),
                "ROLL" => Ok(Self::Roll),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetChainStrategy {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetChainStrategy {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetChainStrategy {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetInstrumentsProjection`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "symbol-search",
    ///    "symbol-regex",
    ///    "desc-search",
    ///    "desc-regex",
    ///    "search",
    ///    "fundamental"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetInstrumentsProjection {
        #[serde(rename = "symbol-search")]
        SymbolSearch,
        #[serde(rename = "symbol-regex")]
        SymbolRegex,
        #[serde(rename = "desc-search")]
        DescSearch,
        #[serde(rename = "desc-regex")]
        DescRegex,
        #[serde(rename = "search")]
        Search,
        #[serde(rename = "fundamental")]
        Fundamental,
    }

    impl ::std::fmt::Display for GetInstrumentsProjection {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::SymbolSearch => f.write_str("symbol-search"),
                Self::SymbolRegex => f.write_str("symbol-regex"),
                Self::DescSearch => f.write_str("desc-search"),
                Self::DescRegex => f.write_str("desc-regex"),
                Self::Search => f.write_str("search"),
                Self::Fundamental => f.write_str("fundamental"),
            }
        }
    }

    impl ::std::str::FromStr for GetInstrumentsProjection {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "symbol-search" => Ok(Self::SymbolSearch),
                "symbol-regex" => Ok(Self::SymbolRegex),
                "desc-search" => Ok(Self::DescSearch),
                "desc-regex" => Ok(Self::DescRegex),
                "search" => Ok(Self::Search),
                "fundamental" => Ok(Self::Fundamental),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetInstrumentsProjection {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetInstrumentsProjection {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetInstrumentsProjection {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetInstrumentsResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "instruments": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/InstrumentResponse"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetInstrumentsResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub instruments: ::std::vec::Vec<InstrumentResponse>,
    }

    impl ::std::default::Default for GetInstrumentsResponse {
        fn default() -> Self {
            Self {
                instruments: Default::default(),
            }
        }
    }

    impl GetInstrumentsResponse {
        pub fn builder() -> builder::GetInstrumentsResponse {
            Default::default()
        }
    }

    ///`GetMarketHourMarketId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "equity",
    ///    "option",
    ///    "bond",
    ///    "future",
    ///    "forex"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetMarketHourMarketId {
        #[serde(rename = "equity")]
        Equity,
        #[serde(rename = "option")]
        Option,
        #[serde(rename = "bond")]
        Bond,
        #[serde(rename = "future")]
        Future,
        #[serde(rename = "forex")]
        Forex,
    }

    impl ::std::fmt::Display for GetMarketHourMarketId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Equity => f.write_str("equity"),
                Self::Option => f.write_str("option"),
                Self::Bond => f.write_str("bond"),
                Self::Future => f.write_str("future"),
                Self::Forex => f.write_str("forex"),
            }
        }
    }

    impl ::std::str::FromStr for GetMarketHourMarketId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "equity" => Ok(Self::Equity),
                "option" => Ok(Self::Option),
                "bond" => Ok(Self::Bond),
                "future" => Ok(Self::Future),
                "forex" => Ok(Self::Forex),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetMarketHourMarketId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetMarketHourMarketId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetMarketHourMarketId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetMarketHoursMarketsItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "equity",
    ///    "option",
    ///    "bond",
    ///    "future",
    ///    "forex"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetMarketHoursMarketsItem {
        #[serde(rename = "equity")]
        Equity,
        #[serde(rename = "option")]
        Option,
        #[serde(rename = "bond")]
        Bond,
        #[serde(rename = "future")]
        Future,
        #[serde(rename = "forex")]
        Forex,
    }

    impl ::std::fmt::Display for GetMarketHoursMarketsItem {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Equity => f.write_str("equity"),
                Self::Option => f.write_str("option"),
                Self::Bond => f.write_str("bond"),
                Self::Future => f.write_str("future"),
                Self::Forex => f.write_str("forex"),
            }
        }
    }

    impl ::std::str::FromStr for GetMarketHoursMarketsItem {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "equity" => Ok(Self::Equity),
                "option" => Ok(Self::Option),
                "bond" => Ok(Self::Bond),
                "future" => Ok(Self::Future),
                "forex" => Ok(Self::Forex),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetMarketHoursMarketsItem {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetMarketHoursMarketsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetMarketHoursMarketsItem {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetMoversFrequency`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "default": 0,
    ///  "type": "integer",
    ///  "format": "int32",
    ///  "enum": [
    ///    0,
    ///    1,
    ///    5,
    ///    10,
    ///    30,
    ///    60
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct GetMoversFrequency(i32);
    impl ::std::ops::Deref for GetMoversFrequency {
        type Target = i32;
        fn deref(&self) -> &i32 {
            &self.0
        }
    }

    impl ::std::convert::From<GetMoversFrequency> for i32 {
        fn from(value: GetMoversFrequency) -> Self {
            value.0
        }
    }

    impl ::std::default::Default for GetMoversFrequency {
        fn default() -> Self {
            GetMoversFrequency(0_i32)
        }
    }

    impl ::std::convert::TryFrom<i32> for GetMoversFrequency {
        type Error = self::error::ConversionError;
        fn try_from(value: i32) -> ::std::result::Result<Self, self::error::ConversionError> {
            if ![0_i32, 1_i32, 5_i32, 10_i32, 30_i32, 60_i32].contains(&value) {
                Err("invalid value".into())
            } else {
                Ok(Self(value))
            }
        }
    }

    impl<'de> ::serde::Deserialize<'de> for GetMoversFrequency {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Self::try_from(<i32>::deserialize(deserializer)?)
                .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
        }
    }

    ///`GetMoversResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "screeners": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Screener"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct GetMoversResponse {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub screeners: ::std::vec::Vec<Screener>,
    }

    impl ::std::default::Default for GetMoversResponse {
        fn default() -> Self {
            Self {
                screeners: Default::default(),
            }
        }
    }

    impl GetMoversResponse {
        pub fn builder() -> builder::GetMoversResponse {
            Default::default()
        }
    }

    ///`GetMoversSort`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "VOLUME",
    ///    "TRADES",
    ///    "PERCENT_CHANGE_UP",
    ///    "PERCENT_CHANGE_DOWN"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetMoversSort {
        #[serde(rename = "VOLUME")]
        Volume,
        #[serde(rename = "TRADES")]
        Trades,
        #[serde(rename = "PERCENT_CHANGE_UP")]
        PercentChangeUp,
        #[serde(rename = "PERCENT_CHANGE_DOWN")]
        PercentChangeDown,
    }

    impl ::std::fmt::Display for GetMoversSort {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Volume => f.write_str("VOLUME"),
                Self::Trades => f.write_str("TRADES"),
                Self::PercentChangeUp => f.write_str("PERCENT_CHANGE_UP"),
                Self::PercentChangeDown => f.write_str("PERCENT_CHANGE_DOWN"),
            }
        }
    }

    impl ::std::str::FromStr for GetMoversSort {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "VOLUME" => Ok(Self::Volume),
                "TRADES" => Ok(Self::Trades),
                "PERCENT_CHANGE_UP" => Ok(Self::PercentChangeUp),
                "PERCENT_CHANGE_DOWN" => Ok(Self::PercentChangeDown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetMoversSort {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetMoversSort {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetMoversSort {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetMoversSymbolId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "$DJI",
    ///    "$COMPX",
    ///    "$SPX",
    ///    "NYSE",
    ///    "NASDAQ",
    ///    "OTCBB",
    ///    "INDEX_ALL",
    ///    "EQUITY_ALL",
    ///    "OPTION_ALL",
    ///    "OPTION_PUT",
    ///    "OPTION_CALL"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetMoversSymbolId {
        #[serde(rename = "$DJI")]
        Dji,
        #[serde(rename = "$COMPX")]
        Compx,
        #[serde(rename = "$SPX")]
        Spx,
        #[serde(rename = "NYSE")]
        Nyse,
        #[serde(rename = "NASDAQ")]
        Nasdaq,
        #[serde(rename = "OTCBB")]
        Otcbb,
        #[serde(rename = "INDEX_ALL")]
        IndexAll,
        #[serde(rename = "EQUITY_ALL")]
        EquityAll,
        #[serde(rename = "OPTION_ALL")]
        OptionAll,
        #[serde(rename = "OPTION_PUT")]
        OptionPut,
        #[serde(rename = "OPTION_CALL")]
        OptionCall,
    }

    impl ::std::fmt::Display for GetMoversSymbolId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Dji => f.write_str("$DJI"),
                Self::Compx => f.write_str("$COMPX"),
                Self::Spx => f.write_str("$SPX"),
                Self::Nyse => f.write_str("NYSE"),
                Self::Nasdaq => f.write_str("NASDAQ"),
                Self::Otcbb => f.write_str("OTCBB"),
                Self::IndexAll => f.write_str("INDEX_ALL"),
                Self::EquityAll => f.write_str("EQUITY_ALL"),
                Self::OptionAll => f.write_str("OPTION_ALL"),
                Self::OptionPut => f.write_str("OPTION_PUT"),
                Self::OptionCall => f.write_str("OPTION_CALL"),
            }
        }
    }

    impl ::std::str::FromStr for GetMoversSymbolId {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "$DJI" => Ok(Self::Dji),
                "$COMPX" => Ok(Self::Compx),
                "$SPX" => Ok(Self::Spx),
                "NYSE" => Ok(Self::Nyse),
                "NASDAQ" => Ok(Self::Nasdaq),
                "OTCBB" => Ok(Self::Otcbb),
                "INDEX_ALL" => Ok(Self::IndexAll),
                "EQUITY_ALL" => Ok(Self::EquityAll),
                "OPTION_ALL" => Ok(Self::OptionAll),
                "OPTION_PUT" => Ok(Self::OptionPut),
                "OPTION_CALL" => Ok(Self::OptionCall),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetMoversSymbolId {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetMoversSymbolId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetMoversSymbolId {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetPriceHistoryFrequencyType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "minute",
    ///    "daily",
    ///    "weekly",
    ///    "monthly"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetPriceHistoryFrequencyType {
        #[serde(rename = "minute")]
        Minute,
        #[serde(rename = "daily")]
        Daily,
        #[serde(rename = "weekly")]
        Weekly,
        #[serde(rename = "monthly")]
        Monthly,
    }

    impl ::std::fmt::Display for GetPriceHistoryFrequencyType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Minute => f.write_str("minute"),
                Self::Daily => f.write_str("daily"),
                Self::Weekly => f.write_str("weekly"),
                Self::Monthly => f.write_str("monthly"),
            }
        }
    }

    impl ::std::str::FromStr for GetPriceHistoryFrequencyType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "minute" => Ok(Self::Minute),
                "daily" => Ok(Self::Daily),
                "weekly" => Ok(Self::Weekly),
                "monthly" => Ok(Self::Monthly),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPriceHistoryFrequencyType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetPriceHistoryFrequencyType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetPriceHistoryFrequencyType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`GetPriceHistoryPeriodType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "day",
    ///    "month",
    ///    "year",
    ///    "ytd"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum GetPriceHistoryPeriodType {
        #[serde(rename = "day")]
        Day,
        #[serde(rename = "month")]
        Month,
        #[serde(rename = "year")]
        Year,
        #[serde(rename = "ytd")]
        Ytd,
    }

    impl ::std::fmt::Display for GetPriceHistoryPeriodType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Day => f.write_str("day"),
                Self::Month => f.write_str("month"),
                Self::Year => f.write_str("year"),
                Self::Ytd => f.write_str("ytd"),
            }
        }
    }

    impl ::std::str::FromStr for GetPriceHistoryPeriodType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "day" => Ok(Self::Day),
                "month" => Ok(Self::Month),
                "year" => Ok(Self::Year),
                "ytd" => Ok(Self::Ytd),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for GetPriceHistoryPeriodType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for GetPriceHistoryPeriodType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for GetPriceHistoryPeriodType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`Hours`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "category": {
    ///      "type": "string"
    ///    },
    ///    "date": {
    ///      "type": "string"
    ///    },
    ///    "exchange": {
    ///      "type": "string"
    ///    },
    ///    "isOpen": {
    ///      "type": "boolean"
    ///    },
    ///    "marketType": {
    ///      "type": "string",
    ///      "enum": [
    ///        "BOND",
    ///        "EQUITY",
    ///        "ETF",
    ///        "EXTENDED",
    ///        "FOREX",
    ///        "FUTURE",
    ///        "FUTURE_OPTION",
    ///        "FUNDAMENTAL",
    ///        "INDEX",
    ///        "INDICATOR",
    ///        "MUTUAL_FUND",
    ///        "OPTION",
    ///        "UNKNOWN"
    ///      ]
    ///    },
    ///    "product": {
    ///      "type": "string"
    ///    },
    ///    "productName": {
    ///      "type": "string"
    ///    },
    ///    "sessionHours": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "$ref": "#/components/schemas/Interval"
    ///        }
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Hours {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub category: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub date: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub exchange: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "isOpen",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_open: ::std::option::Option<bool>,
        #[serde(
            rename = "marketType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub market_type: ::std::option::Option<HoursMarketType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub product: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "productName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub product_name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "sessionHours",
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub session_hours:
            ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<Interval>>,
    }

    impl ::std::default::Default for Hours {
        fn default() -> Self {
            Self {
                category: Default::default(),
                date: Default::default(),
                exchange: Default::default(),
                is_open: Default::default(),
                market_type: Default::default(),
                product: Default::default(),
                product_name: Default::default(),
                session_hours: Default::default(),
            }
        }
    }

    impl Hours {
        pub fn builder() -> builder::Hours {
            Default::default()
        }
    }

    ///`HoursMarketType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "BOND",
    ///    "EQUITY",
    ///    "ETF",
    ///    "EXTENDED",
    ///    "FOREX",
    ///    "FUTURE",
    ///    "FUTURE_OPTION",
    ///    "FUNDAMENTAL",
    ///    "INDEX",
    ///    "INDICATOR",
    ///    "MUTUAL_FUND",
    ///    "OPTION",
    ///    "UNKNOWN"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum HoursMarketType {
        #[serde(rename = "BOND")]
        Bond,
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "ETF")]
        Etf,
        #[serde(rename = "EXTENDED")]
        Extended,
        #[serde(rename = "FOREX")]
        Forex,
        #[serde(rename = "FUTURE")]
        Future,
        #[serde(rename = "FUTURE_OPTION")]
        FutureOption,
        #[serde(rename = "FUNDAMENTAL")]
        Fundamental,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "INDICATOR")]
        Indicator,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for HoursMarketType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Bond => f.write_str("BOND"),
                Self::Equity => f.write_str("EQUITY"),
                Self::Etf => f.write_str("ETF"),
                Self::Extended => f.write_str("EXTENDED"),
                Self::Forex => f.write_str("FOREX"),
                Self::Future => f.write_str("FUTURE"),
                Self::FutureOption => f.write_str("FUTURE_OPTION"),
                Self::Fundamental => f.write_str("FUNDAMENTAL"),
                Self::Index => f.write_str("INDEX"),
                Self::Indicator => f.write_str("INDICATOR"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::Option => f.write_str("OPTION"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for HoursMarketType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "BOND" => Ok(Self::Bond),
                "EQUITY" => Ok(Self::Equity),
                "ETF" => Ok(Self::Etf),
                "EXTENDED" => Ok(Self::Extended),
                "FOREX" => Ok(Self::Forex),
                "FUTURE" => Ok(Self::Future),
                "FUTURE_OPTION" => Ok(Self::FutureOption),
                "FUNDAMENTAL" => Ok(Self::Fundamental),
                "INDEX" => Ok(Self::Index),
                "INDICATOR" => Ok(Self::Indicator),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "OPTION" => Ok(Self::Option),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for HoursMarketType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for HoursMarketType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for HoursMarketType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Quote info of Index security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Quote info of Index security",
    ///  "type": "object",
    ///  "properties": {
    ///    "assetMainType": {
    ///      "$ref": "#/components/schemas/AssetMainType"
    ///    },
    ///    "quote": {
    ///      "$ref": "#/components/schemas/QuoteIndex"
    ///    },
    ///    "realtime": {
    ///      "description": "is quote realtime",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "reference": {
    ///      "$ref": "#/components/schemas/ReferenceIndex"
    ///    },
    ///    "ssid": {
    ///      "description": "SSID of instrument",
    ///      "examples": [
    ///        1234567890
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "symbol": {
    ///      "description": "Symbol of instrument",
    ///      "examples": [
    ///        "AAPL"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct IndexResponse {
        #[serde(
            rename = "assetMainType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub asset_main_type: ::std::option::Option<AssetMainType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quote: ::std::option::Option<QuoteIndex>,
        ///is quote realtime
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub realtime: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reference: ::std::option::Option<ReferenceIndex>,
        ///SSID of instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ssid: ::std::option::Option<i64>,
        ///Symbol of instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for IndexResponse {
        fn default() -> Self {
            Self {
                asset_main_type: Default::default(),
                quote: Default::default(),
                realtime: Default::default(),
                reference: Default::default(),
                ssid: Default::default(),
                symbol: Default::default(),
            }
        }
    }

    impl IndexResponse {
        pub fn builder() -> builder::IndexResponse {
            Default::default()
        }
    }

    ///`Instrument`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "assetType": {
    ///      "type": "string",
    ///      "enum": [
    ///        "BOND",
    ///        "EQUITY",
    ///        "ETF",
    ///        "EXTENDED",
    ///        "FOREX",
    ///        "FUTURE",
    ///        "FUTURE_OPTION",
    ///        "FUNDAMENTAL",
    ///        "INDEX",
    ///        "INDICATOR",
    ///        "MUTUAL_FUND",
    ///        "OPTION",
    ///        "UNKNOWN"
    ///      ]
    ///    },
    ///    "cusip": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "exchange": {
    ///      "type": "string"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "writeOnly": true,
    ///      "type": "string",
    ///      "enum": [
    ///        "BOND",
    ///        "EQUITY",
    ///        "ETF",
    ///        "EXTENDED",
    ///        "FOREX",
    ///        "FUTURE",
    ///        "FUTURE_OPTION",
    ///        "FUNDAMENTAL",
    ///        "INDEX",
    ///        "INDICATOR",
    ///        "MUTUAL_FUND",
    ///        "OPTION",
    ///        "UNKNOWN"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Instrument {
        #[serde(
            rename = "assetType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub asset_type: ::std::option::Option<InstrumentAssetType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub exchange: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<InstrumentType>,
    }

    impl ::std::default::Default for Instrument {
        fn default() -> Self {
            Self {
                asset_type: Default::default(),
                cusip: Default::default(),
                description: Default::default(),
                exchange: Default::default(),
                symbol: Default::default(),
                type_: Default::default(),
            }
        }
    }

    impl Instrument {
        pub fn builder() -> builder::Instrument {
            Default::default()
        }
    }

    ///`InstrumentAssetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "BOND",
    ///    "EQUITY",
    ///    "ETF",
    ///    "EXTENDED",
    ///    "FOREX",
    ///    "FUTURE",
    ///    "FUTURE_OPTION",
    ///    "FUNDAMENTAL",
    ///    "INDEX",
    ///    "INDICATOR",
    ///    "MUTUAL_FUND",
    ///    "OPTION",
    ///    "UNKNOWN"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum InstrumentAssetType {
        #[serde(rename = "BOND")]
        Bond,
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "ETF")]
        Etf,
        #[serde(rename = "EXTENDED")]
        Extended,
        #[serde(rename = "FOREX")]
        Forex,
        #[serde(rename = "FUTURE")]
        Future,
        #[serde(rename = "FUTURE_OPTION")]
        FutureOption,
        #[serde(rename = "FUNDAMENTAL")]
        Fundamental,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "INDICATOR")]
        Indicator,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for InstrumentAssetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Bond => f.write_str("BOND"),
                Self::Equity => f.write_str("EQUITY"),
                Self::Etf => f.write_str("ETF"),
                Self::Extended => f.write_str("EXTENDED"),
                Self::Forex => f.write_str("FOREX"),
                Self::Future => f.write_str("FUTURE"),
                Self::FutureOption => f.write_str("FUTURE_OPTION"),
                Self::Fundamental => f.write_str("FUNDAMENTAL"),
                Self::Index => f.write_str("INDEX"),
                Self::Indicator => f.write_str("INDICATOR"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::Option => f.write_str("OPTION"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for InstrumentAssetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "BOND" => Ok(Self::Bond),
                "EQUITY" => Ok(Self::Equity),
                "ETF" => Ok(Self::Etf),
                "EXTENDED" => Ok(Self::Extended),
                "FOREX" => Ok(Self::Forex),
                "FUTURE" => Ok(Self::Future),
                "FUTURE_OPTION" => Ok(Self::FutureOption),
                "FUNDAMENTAL" => Ok(Self::Fundamental),
                "INDEX" => Ok(Self::Index),
                "INDICATOR" => Ok(Self::Indicator),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "OPTION" => Ok(Self::Option),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for InstrumentAssetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for InstrumentAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for InstrumentAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`InstrumentResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "assetType": {
    ///      "type": "string",
    ///      "enum": [
    ///        "BOND",
    ///        "EQUITY",
    ///        "ETF",
    ///        "EXTENDED",
    ///        "FOREX",
    ///        "FUTURE",
    ///        "FUTURE_OPTION",
    ///        "FUNDAMENTAL",
    ///        "INDEX",
    ///        "INDICATOR",
    ///        "MUTUAL_FUND",
    ///        "OPTION",
    ///        "UNKNOWN"
    ///      ]
    ///    },
    ///    "bondFactor": {
    ///      "type": "string"
    ///    },
    ///    "bondInstrumentInfo": {
    ///      "$ref": "#/components/schemas/Bond"
    ///    },
    ///    "bondMultiplier": {
    ///      "type": "string"
    ///    },
    ///    "bondPrice": {
    ///      "type": "number"
    ///    },
    ///    "cusip": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "exchange": {
    ///      "type": "string"
    ///    },
    ///    "fundamental": {
    ///      "$ref": "#/components/schemas/FundamentalInst"
    ///    },
    ///    "instrumentInfo": {
    ///      "$ref": "#/components/schemas/Instrument"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "writeOnly": true,
    ///      "type": "string",
    ///      "enum": [
    ///        "BOND",
    ///        "EQUITY",
    ///        "ETF",
    ///        "EXTENDED",
    ///        "FOREX",
    ///        "FUTURE",
    ///        "FUTURE_OPTION",
    ///        "FUNDAMENTAL",
    ///        "INDEX",
    ///        "INDICATOR",
    ///        "MUTUAL_FUND",
    ///        "OPTION",
    ///        "UNKNOWN"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct InstrumentResponse {
        #[serde(
            rename = "assetType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub asset_type: ::std::option::Option<InstrumentResponseAssetType>,
        #[serde(
            rename = "bondFactor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bond_factor: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "bondInstrumentInfo",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bond_instrument_info: ::std::option::Option<Bond>,
        #[serde(
            rename = "bondMultiplier",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bond_multiplier: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "bondPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bond_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub exchange: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fundamental: ::std::option::Option<FundamentalInst>,
        #[serde(
            rename = "instrumentInfo",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub instrument_info: ::std::option::Option<Instrument>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<InstrumentResponseType>,
    }

    impl ::std::default::Default for InstrumentResponse {
        fn default() -> Self {
            Self {
                asset_type: Default::default(),
                bond_factor: Default::default(),
                bond_instrument_info: Default::default(),
                bond_multiplier: Default::default(),
                bond_price: Default::default(),
                cusip: Default::default(),
                description: Default::default(),
                exchange: Default::default(),
                fundamental: Default::default(),
                instrument_info: Default::default(),
                symbol: Default::default(),
                type_: Default::default(),
            }
        }
    }

    impl InstrumentResponse {
        pub fn builder() -> builder::InstrumentResponse {
            Default::default()
        }
    }

    ///`InstrumentResponseAssetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "BOND",
    ///    "EQUITY",
    ///    "ETF",
    ///    "EXTENDED",
    ///    "FOREX",
    ///    "FUTURE",
    ///    "FUTURE_OPTION",
    ///    "FUNDAMENTAL",
    ///    "INDEX",
    ///    "INDICATOR",
    ///    "MUTUAL_FUND",
    ///    "OPTION",
    ///    "UNKNOWN"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum InstrumentResponseAssetType {
        #[serde(rename = "BOND")]
        Bond,
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "ETF")]
        Etf,
        #[serde(rename = "EXTENDED")]
        Extended,
        #[serde(rename = "FOREX")]
        Forex,
        #[serde(rename = "FUTURE")]
        Future,
        #[serde(rename = "FUTURE_OPTION")]
        FutureOption,
        #[serde(rename = "FUNDAMENTAL")]
        Fundamental,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "INDICATOR")]
        Indicator,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for InstrumentResponseAssetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Bond => f.write_str("BOND"),
                Self::Equity => f.write_str("EQUITY"),
                Self::Etf => f.write_str("ETF"),
                Self::Extended => f.write_str("EXTENDED"),
                Self::Forex => f.write_str("FOREX"),
                Self::Future => f.write_str("FUTURE"),
                Self::FutureOption => f.write_str("FUTURE_OPTION"),
                Self::Fundamental => f.write_str("FUNDAMENTAL"),
                Self::Index => f.write_str("INDEX"),
                Self::Indicator => f.write_str("INDICATOR"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::Option => f.write_str("OPTION"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for InstrumentResponseAssetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "BOND" => Ok(Self::Bond),
                "EQUITY" => Ok(Self::Equity),
                "ETF" => Ok(Self::Etf),
                "EXTENDED" => Ok(Self::Extended),
                "FOREX" => Ok(Self::Forex),
                "FUTURE" => Ok(Self::Future),
                "FUTURE_OPTION" => Ok(Self::FutureOption),
                "FUNDAMENTAL" => Ok(Self::Fundamental),
                "INDEX" => Ok(Self::Index),
                "INDICATOR" => Ok(Self::Indicator),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "OPTION" => Ok(Self::Option),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for InstrumentResponseAssetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for InstrumentResponseAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for InstrumentResponseAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`InstrumentResponseType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "writeOnly": true,
    ///  "type": "string",
    ///  "enum": [
    ///    "BOND",
    ///    "EQUITY",
    ///    "ETF",
    ///    "EXTENDED",
    ///    "FOREX",
    ///    "FUTURE",
    ///    "FUTURE_OPTION",
    ///    "FUNDAMENTAL",
    ///    "INDEX",
    ///    "INDICATOR",
    ///    "MUTUAL_FUND",
    ///    "OPTION",
    ///    "UNKNOWN"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum InstrumentResponseType {
        #[serde(rename = "BOND")]
        Bond,
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "ETF")]
        Etf,
        #[serde(rename = "EXTENDED")]
        Extended,
        #[serde(rename = "FOREX")]
        Forex,
        #[serde(rename = "FUTURE")]
        Future,
        #[serde(rename = "FUTURE_OPTION")]
        FutureOption,
        #[serde(rename = "FUNDAMENTAL")]
        Fundamental,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "INDICATOR")]
        Indicator,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for InstrumentResponseType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Bond => f.write_str("BOND"),
                Self::Equity => f.write_str("EQUITY"),
                Self::Etf => f.write_str("ETF"),
                Self::Extended => f.write_str("EXTENDED"),
                Self::Forex => f.write_str("FOREX"),
                Self::Future => f.write_str("FUTURE"),
                Self::FutureOption => f.write_str("FUTURE_OPTION"),
                Self::Fundamental => f.write_str("FUNDAMENTAL"),
                Self::Index => f.write_str("INDEX"),
                Self::Indicator => f.write_str("INDICATOR"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::Option => f.write_str("OPTION"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for InstrumentResponseType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "BOND" => Ok(Self::Bond),
                "EQUITY" => Ok(Self::Equity),
                "ETF" => Ok(Self::Etf),
                "EXTENDED" => Ok(Self::Extended),
                "FOREX" => Ok(Self::Forex),
                "FUTURE" => Ok(Self::Future),
                "FUTURE_OPTION" => Ok(Self::FutureOption),
                "FUNDAMENTAL" => Ok(Self::Fundamental),
                "INDEX" => Ok(Self::Index),
                "INDICATOR" => Ok(Self::Indicator),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "OPTION" => Ok(Self::Option),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for InstrumentResponseType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for InstrumentResponseType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for InstrumentResponseType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`InstrumentType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "writeOnly": true,
    ///  "type": "string",
    ///  "enum": [
    ///    "BOND",
    ///    "EQUITY",
    ///    "ETF",
    ///    "EXTENDED",
    ///    "FOREX",
    ///    "FUTURE",
    ///    "FUTURE_OPTION",
    ///    "FUNDAMENTAL",
    ///    "INDEX",
    ///    "INDICATOR",
    ///    "MUTUAL_FUND",
    ///    "OPTION",
    ///    "UNKNOWN"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum InstrumentType {
        #[serde(rename = "BOND")]
        Bond,
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "ETF")]
        Etf,
        #[serde(rename = "EXTENDED")]
        Extended,
        #[serde(rename = "FOREX")]
        Forex,
        #[serde(rename = "FUTURE")]
        Future,
        #[serde(rename = "FUTURE_OPTION")]
        FutureOption,
        #[serde(rename = "FUNDAMENTAL")]
        Fundamental,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "INDICATOR")]
        Indicator,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for InstrumentType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Bond => f.write_str("BOND"),
                Self::Equity => f.write_str("EQUITY"),
                Self::Etf => f.write_str("ETF"),
                Self::Extended => f.write_str("EXTENDED"),
                Self::Forex => f.write_str("FOREX"),
                Self::Future => f.write_str("FUTURE"),
                Self::FutureOption => f.write_str("FUTURE_OPTION"),
                Self::Fundamental => f.write_str("FUNDAMENTAL"),
                Self::Index => f.write_str("INDEX"),
                Self::Indicator => f.write_str("INDICATOR"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::Option => f.write_str("OPTION"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for InstrumentType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "BOND" => Ok(Self::Bond),
                "EQUITY" => Ok(Self::Equity),
                "ETF" => Ok(Self::Etf),
                "EXTENDED" => Ok(Self::Extended),
                "FOREX" => Ok(Self::Forex),
                "FUTURE" => Ok(Self::Future),
                "FUTURE_OPTION" => Ok(Self::FutureOption),
                "FUNDAMENTAL" => Ok(Self::Fundamental),
                "INDEX" => Ok(Self::Index),
                "INDICATOR" => Ok(Self::Indicator),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "OPTION" => Ok(Self::Option),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for InstrumentType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for InstrumentType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for InstrumentType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`Interval`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "end": {
    ///      "type": "string"
    ///    },
    ///    "start": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Interval {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub end: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub start: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for Interval {
        fn default() -> Self {
            Self {
                end: Default::default(),
                start: Default::default(),
            }
        }
    }

    impl Interval {
        pub fn builder() -> builder::Interval {
            Default::default()
        }
    }

    ///Asset Sub Type (only there if applicable)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Asset Sub Type (only there if applicable)",
    ///  "type": [
    ///    "string",
    ///    "null"
    ///  ],
    ///  "enum": [
    ///    "OEF",
    ///    "CEF",
    ///    "MMF",
    ///    null
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct MutualFundAssetSubType(pub ::std::option::Option<MutualFundAssetSubTypeInner>);
    impl ::std::ops::Deref for MutualFundAssetSubType {
        type Target = ::std::option::Option<MutualFundAssetSubTypeInner>;
        fn deref(&self) -> &::std::option::Option<MutualFundAssetSubTypeInner> {
            &self.0
        }
    }

    impl ::std::convert::From<MutualFundAssetSubType>
        for ::std::option::Option<MutualFundAssetSubTypeInner>
    {
        fn from(value: MutualFundAssetSubType) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::std::option::Option<MutualFundAssetSubTypeInner>>
        for MutualFundAssetSubType
    {
        fn from(value: ::std::option::Option<MutualFundAssetSubTypeInner>) -> Self {
            Self(value)
        }
    }

    ///Asset Sub Type (only there if applicable)
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Asset Sub Type (only there if applicable)",
    ///  "type": "string",
    ///  "enum": [
    ///    "OEF",
    ///    "CEF",
    ///    "MMF"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum MutualFundAssetSubTypeInner {
        #[serde(rename = "OEF")]
        Oef,
        #[serde(rename = "CEF")]
        Cef,
        #[serde(rename = "MMF")]
        Mmf,
    }

    impl ::std::fmt::Display for MutualFundAssetSubTypeInner {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Oef => f.write_str("OEF"),
                Self::Cef => f.write_str("CEF"),
                Self::Mmf => f.write_str("MMF"),
            }
        }
    }

    impl ::std::str::FromStr for MutualFundAssetSubTypeInner {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "OEF" => Ok(Self::Oef),
                "CEF" => Ok(Self::Cef),
                "MMF" => Ok(Self::Mmf),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for MutualFundAssetSubTypeInner {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for MutualFundAssetSubTypeInner {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MutualFundAssetSubTypeInner {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Quote info of MutualFund security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Quote info of MutualFund security",
    ///  "type": "object",
    ///  "properties": {
    ///    "assetMainType": {
    ///      "$ref": "#/components/schemas/AssetMainType"
    ///    },
    ///    "assetSubType": {
    ///      "$ref": "#/components/schemas/MutualFundAssetSubType"
    ///    },
    ///    "fundamental": {
    ///      "$ref": "#/components/schemas/Fundamental"
    ///    },
    ///    "quote": {
    ///      "$ref": "#/components/schemas/QuoteMutualFund"
    ///    },
    ///    "realtime": {
    ///      "description": "is quote realtime",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "reference": {
    ///      "$ref": "#/components/schemas/ReferenceMutualFund"
    ///    },
    ///    "ssid": {
    ///      "description": "SSID of instrument",
    ///      "examples": [
    ///        1234567890
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "symbol": {
    ///      "description": "Symbol of instrument",
    ///      "examples": [
    ///        "AAPL"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MutualFundResponse {
        #[serde(
            rename = "assetMainType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub asset_main_type: ::std::option::Option<AssetMainType>,
        #[serde(
            rename = "assetSubType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub asset_sub_type: ::std::option::Option<MutualFundAssetSubType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fundamental: ::std::option::Option<Fundamental>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quote: ::std::option::Option<QuoteMutualFund>,
        ///is quote realtime
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub realtime: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reference: ::std::option::Option<ReferenceMutualFund>,
        ///SSID of instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ssid: ::std::option::Option<i64>,
        ///Symbol of instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for MutualFundResponse {
        fn default() -> Self {
            Self {
                asset_main_type: Default::default(),
                asset_sub_type: Default::default(),
                fundamental: Default::default(),
                quote: Default::default(),
                realtime: Default::default(),
                reference: Default::default(),
                ssid: Default::default(),
                symbol: Default::default(),
            }
        }
    }

    impl MutualFundResponse {
        pub fn builder() -> builder::MutualFundResponse {
            Default::default()
        }
    }

    ///`OptionChain`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "callExpDateMap": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/OptionContractMap"
    ///      }
    ///    },
    ///    "daysToExpiration": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "interestRate": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "interval": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "isDelayed": {
    ///      "type": "boolean"
    ///    },
    ///    "isIndex": {
    ///      "type": "boolean"
    ///    },
    ///    "putExpDateMap": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/OptionContractMap"
    ///      }
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    },
    ///    "strategy": {
    ///      "type": "string",
    ///      "enum": [
    ///        "SINGLE",
    ///        "ANALYTICAL",
    ///        "COVERED",
    ///        "VERTICAL",
    ///        "CALENDAR",
    ///        "STRANGLE",
    ///        "STRADDLE",
    ///        "BUTTERFLY",
    ///        "CONDOR",
    ///        "DIAGONAL",
    ///        "COLLAR",
    ///        "ROLL"
    ///      ]
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    },
    ///    "underlying": {
    ///      "$ref": "#/components/schemas/Underlying"
    ///    },
    ///    "underlyingPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "volatility": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionChain {
        #[serde(
            rename = "callExpDateMap",
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub call_exp_date_map:
            ::std::collections::HashMap<::std::string::String, OptionContractMap>,
        #[serde(
            rename = "daysToExpiration",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub days_to_expiration: ::std::option::Option<f64>,
        #[serde(
            rename = "interestRate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub interest_rate: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub interval: ::std::option::Option<f64>,
        #[serde(
            rename = "isDelayed",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_delayed: ::std::option::Option<bool>,
        #[serde(
            rename = "isIndex",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_index: ::std::option::Option<bool>,
        #[serde(
            rename = "putExpDateMap",
            default,
            skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
        )]
        pub put_exp_date_map: ::std::collections::HashMap<::std::string::String, OptionContractMap>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub strategy: ::std::option::Option<OptionChainStrategy>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub underlying: ::std::option::Option<Underlying>,
        #[serde(
            rename = "underlyingPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub underlying_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub volatility: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for OptionChain {
        fn default() -> Self {
            Self {
                call_exp_date_map: Default::default(),
                days_to_expiration: Default::default(),
                interest_rate: Default::default(),
                interval: Default::default(),
                is_delayed: Default::default(),
                is_index: Default::default(),
                put_exp_date_map: Default::default(),
                status: Default::default(),
                strategy: Default::default(),
                symbol: Default::default(),
                underlying: Default::default(),
                underlying_price: Default::default(),
                volatility: Default::default(),
            }
        }
    }

    impl OptionChain {
        pub fn builder() -> builder::OptionChain {
            Default::default()
        }
    }

    ///`OptionChainStrategy`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "SINGLE",
    ///    "ANALYTICAL",
    ///    "COVERED",
    ///    "VERTICAL",
    ///    "CALENDAR",
    ///    "STRANGLE",
    ///    "STRADDLE",
    ///    "BUTTERFLY",
    ///    "CONDOR",
    ///    "DIAGONAL",
    ///    "COLLAR",
    ///    "ROLL"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum OptionChainStrategy {
        #[serde(rename = "SINGLE")]
        Single,
        #[serde(rename = "ANALYTICAL")]
        Analytical,
        #[serde(rename = "COVERED")]
        Covered,
        #[serde(rename = "VERTICAL")]
        Vertical,
        #[serde(rename = "CALENDAR")]
        Calendar,
        #[serde(rename = "STRANGLE")]
        Strangle,
        #[serde(rename = "STRADDLE")]
        Straddle,
        #[serde(rename = "BUTTERFLY")]
        Butterfly,
        #[serde(rename = "CONDOR")]
        Condor,
        #[serde(rename = "DIAGONAL")]
        Diagonal,
        #[serde(rename = "COLLAR")]
        Collar,
        #[serde(rename = "ROLL")]
        Roll,
    }

    impl ::std::fmt::Display for OptionChainStrategy {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Single => f.write_str("SINGLE"),
                Self::Analytical => f.write_str("ANALYTICAL"),
                Self::Covered => f.write_str("COVERED"),
                Self::Vertical => f.write_str("VERTICAL"),
                Self::Calendar => f.write_str("CALENDAR"),
                Self::Strangle => f.write_str("STRANGLE"),
                Self::Straddle => f.write_str("STRADDLE"),
                Self::Butterfly => f.write_str("BUTTERFLY"),
                Self::Condor => f.write_str("CONDOR"),
                Self::Diagonal => f.write_str("DIAGONAL"),
                Self::Collar => f.write_str("COLLAR"),
                Self::Roll => f.write_str("ROLL"),
            }
        }
    }

    impl ::std::str::FromStr for OptionChainStrategy {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "SINGLE" => Ok(Self::Single),
                "ANALYTICAL" => Ok(Self::Analytical),
                "COVERED" => Ok(Self::Covered),
                "VERTICAL" => Ok(Self::Vertical),
                "CALENDAR" => Ok(Self::Calendar),
                "STRANGLE" => Ok(Self::Strangle),
                "STRADDLE" => Ok(Self::Straddle),
                "BUTTERFLY" => Ok(Self::Butterfly),
                "CONDOR" => Ok(Self::Condor),
                "DIAGONAL" => Ok(Self::Diagonal),
                "COLLAR" => Ok(Self::Collar),
                "ROLL" => Ok(Self::Roll),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for OptionChainStrategy {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for OptionChainStrategy {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for OptionChainStrategy {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`OptionContract`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "askPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "askSize": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "bidPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "bidSize": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "closePrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "daysToExpiration": {
    ///      "type": "number",
    ///      "format": "int"
    ///    },
    ///    "deliverableNote": {
    ///      "type": "string"
    ///    },
    ///    "delta": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "exchangeName": {
    ///      "type": "string"
    ///    },
    ///    "expirationDate": {
    ///      "type": "string"
    ///    },
    ///    "expirationType": {
    ///      "$ref": "#/components/schemas/ExpirationType"
    ///    },
    ///    "gamma": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "highPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "intrinsicValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "isInTheMoney": {
    ///      "type": "boolean"
    ///    },
    ///    "isIndexOption": {
    ///      "type": "boolean"
    ///    },
    ///    "isMini": {
    ///      "type": "boolean"
    ///    },
    ///    "isNonStandard": {
    ///      "type": "boolean"
    ///    },
    ///    "isPennyPilot": {
    ///      "type": "boolean"
    ///    },
    ///    "lastPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "lastSize": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "lastTradingDay": {
    ///      "type": "number",
    ///      "format": "long"
    ///    },
    ///    "lowPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "markChange": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "markPercentChange": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "markPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "multiplier": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "netChange": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "openInterest": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "openPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "optionDeliverablesList": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OptionDeliverables"
    ///      }
    ///    },
    ///    "optionRoot": {
    ///      "type": "string"
    ///    },
    ///    "percentChange": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "putCall": {
    ///      "type": "string",
    ///      "enum": [
    ///        "PUT",
    ///        "CALL"
    ///      ]
    ///    },
    ///    "quoteTimeInLong": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "rho": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "settlementType": {
    ///      "$ref": "#/components/schemas/SettlementType"
    ///    },
    ///    "strikePrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    },
    ///    "theoreticalOptionValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "theoreticalVolatility": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "theta": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "timeValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "totalVolume": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "tradeDate": {
    ///      "type": "number",
    ///      "format": "integer"
    ///    },
    ///    "tradeTimeInLong": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "vega": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "volatility": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionContract {
        // PATCH: API returns "ask"/"bid"/"last"/"mark" not the documented
        // "askPrice"/"bidPrice"/etc. Renames adjusted to match wire.
        #[serde(
            rename = "ask",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_price: ::std::option::Option<f64>,
        #[serde(
            rename = "askSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_size: ::std::option::Option<i32>,
        #[serde(
            rename = "bid",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_price: ::std::option::Option<f64>,
        #[serde(
            rename = "bidSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_size: ::std::option::Option<i32>,
        #[serde(
            rename = "closePrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub close_price: ::std::option::Option<f64>,
        #[serde(
            rename = "daysToExpiration",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub days_to_expiration: ::std::option::Option<f64>,
        #[serde(
            rename = "deliverableNote",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub deliverable_note: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub delta: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "exchangeName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub exchange_name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "expirationDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expiration_date: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "expirationType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expiration_type: ::std::option::Option<ExpirationType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gamma: ::std::option::Option<f64>,
        #[serde(
            rename = "highPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub high_price: ::std::option::Option<f64>,
        #[serde(
            rename = "intrinsicValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub intrinsic_value: ::std::option::Option<f64>,
        #[serde(
            rename = "isInTheMoney",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_in_the_money: ::std::option::Option<bool>,
        #[serde(
            rename = "isIndexOption",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_index_option: ::std::option::Option<bool>,
        #[serde(
            rename = "isMini",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_mini: ::std::option::Option<bool>,
        #[serde(
            rename = "isNonStandard",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_non_standard: ::std::option::Option<bool>,
        #[serde(
            rename = "isPennyPilot",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_penny_pilot: ::std::option::Option<bool>,
        #[serde(
            rename = "last",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_price: ::std::option::Option<f64>,
        #[serde(
            rename = "lastSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_size: ::std::option::Option<i32>,
        #[serde(
            rename = "lastTradingDay",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_trading_day: ::std::option::Option<f64>,
        #[serde(
            rename = "lowPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub low_price: ::std::option::Option<f64>,
        #[serde(
            rename = "markChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mark_change: ::std::option::Option<f64>,
        #[serde(
            rename = "markPercentChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mark_percent_change: ::std::option::Option<f64>,
        #[serde(
            rename = "mark",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mark_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub multiplier: ::std::option::Option<f64>,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(
            rename = "openInterest",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_interest: ::std::option::Option<f64>,
        #[serde(
            rename = "openPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_price: ::std::option::Option<f64>,
        #[serde(
            rename = "optionDeliverablesList",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub option_deliverables_list: ::std::vec::Vec<OptionDeliverables>,
        #[serde(
            rename = "optionRoot",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub option_root: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "percentChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub percent_change: ::std::option::Option<f64>,
        #[serde(
            rename = "putCall",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub put_call: ::std::option::Option<OptionContractPutCall>,
        // PATCH: epoch milliseconds — overflows i32, must be i64.
        #[serde(
            rename = "quoteTimeInLong",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub quote_time_in_long: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub rho: ::std::option::Option<f64>,
        #[serde(
            rename = "settlementType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub settlement_type: ::std::option::Option<SettlementType>,
        #[serde(
            rename = "strikePrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub strike_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "theoreticalOptionValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub theoretical_option_value: ::std::option::Option<f64>,
        #[serde(
            rename = "theoreticalVolatility",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub theoretical_volatility: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub theta: ::std::option::Option<f64>,
        #[serde(
            rename = "timeValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub time_value: ::std::option::Option<f64>,
        #[serde(
            rename = "totalVolume",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_volume: ::std::option::Option<i32>,
        #[serde(
            rename = "tradeDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trade_date: ::std::option::Option<f64>,
        // PATCH: epoch milliseconds — overflows i32, must be i64.
        #[serde(
            rename = "tradeTimeInLong",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trade_time_in_long: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub vega: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub volatility: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for OptionContract {
        fn default() -> Self {
            Self {
                ask_price: Default::default(),
                ask_size: Default::default(),
                bid_price: Default::default(),
                bid_size: Default::default(),
                close_price: Default::default(),
                days_to_expiration: Default::default(),
                deliverable_note: Default::default(),
                delta: Default::default(),
                description: Default::default(),
                exchange_name: Default::default(),
                expiration_date: Default::default(),
                expiration_type: Default::default(),
                gamma: Default::default(),
                high_price: Default::default(),
                intrinsic_value: Default::default(),
                is_in_the_money: Default::default(),
                is_index_option: Default::default(),
                is_mini: Default::default(),
                is_non_standard: Default::default(),
                is_penny_pilot: Default::default(),
                last_price: Default::default(),
                last_size: Default::default(),
                last_trading_day: Default::default(),
                low_price: Default::default(),
                mark_change: Default::default(),
                mark_percent_change: Default::default(),
                mark_price: Default::default(),
                multiplier: Default::default(),
                net_change: Default::default(),
                open_interest: Default::default(),
                open_price: Default::default(),
                option_deliverables_list: Default::default(),
                option_root: Default::default(),
                percent_change: Default::default(),
                put_call: Default::default(),
                quote_time_in_long: Default::default(),
                rho: Default::default(),
                settlement_type: Default::default(),
                strike_price: Default::default(),
                symbol: Default::default(),
                theoretical_option_value: Default::default(),
                theoretical_volatility: Default::default(),
                theta: Default::default(),
                time_value: Default::default(),
                total_volume: Default::default(),
                trade_date: Default::default(),
                trade_time_in_long: Default::default(),
                vega: Default::default(),
                volatility: Default::default(),
            }
        }
    }

    impl OptionContract {
        pub fn builder() -> builder::OptionContract {
            Default::default()
        }
    }

    ///`OptionContractMap`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "additionalProperties": {
    ///    "$ref": "#/components/schemas/OptionContract"
    ///  }
    ///}
    /// ```
    /// </details>
    // PATCH: Schwab returns `"strike": [contract, ...]` per strike (not a single
    // object), so the inner value must be Vec<OptionContract>. Generated spec
    // had this wrong; corrected here.
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct OptionContractMap(
        pub ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<OptionContract>>,
    );
    impl ::std::ops::Deref for OptionContractMap {
        type Target =
            ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<OptionContract>>;
        fn deref(
            &self,
        ) -> &::std::collections::HashMap<::std::string::String, ::std::vec::Vec<OptionContract>>
        {
            &self.0
        }
    }

    impl ::std::convert::From<OptionContractMap>
        for ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<OptionContract>>
    {
        fn from(value: OptionContractMap) -> Self {
            value.0
        }
    }

    impl
        ::std::convert::From<
            ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<OptionContract>>,
        > for OptionContractMap
    {
        fn from(
            value: ::std::collections::HashMap<
                ::std::string::String,
                ::std::vec::Vec<OptionContract>,
            >,
        ) -> Self {
            Self(value)
        }
    }

    ///`OptionContractPutCall`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "PUT",
    ///    "CALL"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum OptionContractPutCall {
        #[serde(rename = "PUT")]
        Put,
        #[serde(rename = "CALL")]
        Call,
    }

    impl ::std::fmt::Display for OptionContractPutCall {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Put => f.write_str("PUT"),
                Self::Call => f.write_str("CALL"),
            }
        }
    }

    impl ::std::str::FromStr for OptionContractPutCall {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "PUT" => Ok(Self::Put),
                "CALL" => Ok(Self::Call),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for OptionContractPutCall {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for OptionContractPutCall {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for OptionContractPutCall {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`OptionDeliverables`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "assetType": {
    ///      "type": "string"
    ///    },
    ///    "currencyType": {
    ///      "type": "string"
    ///    },
    ///    "deliverableUnits": {
    ///      "type": "string"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionDeliverables {
        #[serde(
            rename = "assetType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub asset_type: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "currencyType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub currency_type: ::std::option::Option<::std::string::String>,
        // PATCH: API returns deliverableUnits as a number, not a string.
        #[serde(
            rename = "deliverableUnits",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub deliverable_units: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for OptionDeliverables {
        fn default() -> Self {
            Self {
                asset_type: Default::default(),
                currency_type: Default::default(),
                deliverable_units: Default::default(),
                symbol: Default::default(),
            }
        }
    }

    impl OptionDeliverables {
        pub fn builder() -> builder::OptionDeliverables {
            Default::default()
        }
    }

    ///Quote info of Option security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Quote info of Option security",
    ///  "type": "object",
    ///  "properties": {
    ///    "assetMainType": {
    ///      "$ref": "#/components/schemas/AssetMainType"
    ///    },
    ///    "quote": {
    ///      "$ref": "#/components/schemas/QuoteOption"
    ///    },
    ///    "realtime": {
    ///      "description": "is quote realtime",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "reference": {
    ///      "$ref": "#/components/schemas/ReferenceOption"
    ///    },
    ///    "ssid": {
    ///      "description": "SSID of instrument",
    ///      "examples": [
    ///        1234567890
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "symbol": {
    ///      "description": "Symbol of instrument",
    ///      "examples": [
    ///        "AAPL"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OptionResponse {
        #[serde(
            rename = "assetMainType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub asset_main_type: ::std::option::Option<AssetMainType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quote: ::std::option::Option<QuoteOption>,
        ///is quote realtime
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub realtime: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub reference: ::std::option::Option<ReferenceOption>,
        ///SSID of instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ssid: ::std::option::Option<i64>,
        ///Symbol of instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for OptionResponse {
        fn default() -> Self {
            Self {
                asset_main_type: Default::default(),
                quote: Default::default(),
                realtime: Default::default(),
                reference: Default::default(),
                ssid: Default::default(),
                symbol: Default::default(),
            }
        }
    }

    impl OptionResponse {
        pub fn builder() -> builder::OptionResponse {
            Default::default()
        }
    }

    ///Quote data of Equity security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Quote data of Equity security",
    ///  "type": "object",
    ///  "properties": {
    ///    "52WeekHigh": {
    ///      "description": "Higest price traded in the past 12 months, or 52
    /// weeks",
    ///      "examples": [
    ///        145.09
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "52WeekLow": {
    ///      "description": "Lowest price traded in the past 12 months, or 52
    /// weeks",
    ///      "examples": [
    ///        77.581
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "askMICId": {
    ///      "description": "ask MIC code",
    ///      "examples": [
    ///        "XNYS"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "askPrice": {
    ///      "description": "Current Best Ask Price",
    ///      "examples": [
    ///        124.63
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "askSize": {
    ///      "description": "Number of shares for ask",
    ///      "examples": [
    ///        700
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "askTime": {
    ///      "description": "Last ask time in milliseconds since Epoch",
    ///      "examples": [
    ///        1621376892336
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "bidMICId": {
    ///      "description": "bid MIC code",
    ///      "examples": [
    ///        "XNYS"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "bidPrice": {
    ///      "description": "Current Best Bid Price",
    ///      "examples": [
    ///        124.6
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "bidSize": {
    ///      "description": "Number of shares for bid",
    ///      "examples": [
    ///        300
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "bidTime": {
    ///      "description": "Last bid time in milliseconds since Epoch",
    ///      "examples": [
    ///        1621376892336
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "closePrice": {
    ///      "description": "Previous day's closing price",
    ///      "examples": [
    ///        126.27
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "highPrice": {
    ///      "description": "Day's high trade price",
    ///      "examples": [
    ///        126.99
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "lastMICId": {
    ///      "description": "Last MIC Code",
    ///      "examples": [
    ///        "XNYS"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "lastPrice": {
    ///      "examples": [
    ///        122.3
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "lastSize": {
    ///      "description": "Number of shares traded with last trade",
    ///      "examples": [
    ///        100
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "lowPrice": {
    ///      "description": "Day's low trade price",
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "mark": {
    ///      "description": "Mark price",
    ///      "examples": [
    ///        52.93
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "markChange": {
    ///      "description": "Mark Price change",
    ///      "examples": [
    ///        -0.01
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "markPercentChange": {
    ///      "description": "Mark Price percent change",
    ///      "examples": [
    ///        -0.0189
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "netChange": {
    ///      "description": "Current Last-Prev Close",
    ///      "examples": [
    ///        -0.04
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "netPercentChange": {
    ///      "description": "Net Percentage Change",
    ///      "examples": [
    ///        -0.0756
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "openPrice": {
    ///      "description": "Price at market open",
    ///      "examples": [
    ///        52.8
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "quoteTime": {
    ///      "description": "Last quote time in milliseconds since Epoch",
    ///      "examples": [
    ///        1621376892336
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "securityStatus": {
    ///      "description": "Status of security",
    ///      "examples": [
    ///        "Normal"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "totalVolume": {
    ///      "description": "Aggregated shares traded throughout the day,
    /// including pre/post market hours.",
    ///      "examples": [
    ///        20171188
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "tradeTime": {
    ///      "description": "Last trade time in milliseconds since Epoch",
    ///      "examples": [
    ///        1621376731304
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "volatility": {
    ///      "description": "Option Risk/Volatility Measurement",
    ///      "examples": [
    ///        0.0094
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct QuoteEquity {
        ///ask MIC code
        #[serde(
            rename = "askMICId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_mic_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "askPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_price: ::std::option::Option<f64>,
        ///Number of shares for ask
        #[serde(
            rename = "askSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_size: ::std::option::Option<i32>,
        ///Last ask time in milliseconds since Epoch
        #[serde(
            rename = "askTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_time: ::std::option::Option<i64>,
        ///bid MIC code
        #[serde(
            rename = "bidMICId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_mic_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "bidPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_price: ::std::option::Option<f64>,
        ///Number of shares for bid
        #[serde(
            rename = "bidSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_size: ::std::option::Option<i32>,
        ///Last bid time in milliseconds since Epoch
        #[serde(
            rename = "bidTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_time: ::std::option::Option<i64>,
        #[serde(
            rename = "closePrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub close_price: ::std::option::Option<f64>,
        #[serde(
            rename = "highPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub high_price: ::std::option::Option<f64>,
        ///Last MIC Code
        #[serde(
            rename = "lastMICId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_mic_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "lastPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_price: ::std::option::Option<f64>,
        ///Number of shares traded with last trade
        #[serde(
            rename = "lastSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_size: ::std::option::Option<i32>,
        #[serde(
            rename = "lowPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub low_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mark: ::std::option::Option<f64>,
        #[serde(
            rename = "markChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mark_change: ::std::option::Option<f64>,
        #[serde(
            rename = "markPercentChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mark_percent_change: ::std::option::Option<f64>,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(
            rename = "netPercentChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_percent_change: ::std::option::Option<f64>,
        #[serde(
            rename = "openPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_price: ::std::option::Option<f64>,
        ///Last quote time in milliseconds since Epoch
        #[serde(
            rename = "quoteTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub quote_time: ::std::option::Option<i64>,
        ///Status of security
        #[serde(
            rename = "securityStatus",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub security_status: ::std::option::Option<::std::string::String>,
        ///Aggregated shares traded throughout the day, including pre/post
        /// market hours.
        #[serde(
            rename = "totalVolume",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_volume: ::std::option::Option<i64>,
        ///Last trade time in milliseconds since Epoch
        #[serde(
            rename = "tradeTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trade_time: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub volatility: ::std::option::Option<f64>,
        #[serde(
            rename = "52WeekHigh",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub x52week_high: ::std::option::Option<f64>,
        #[serde(
            rename = "52WeekLow",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub x52week_low: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for QuoteEquity {
        fn default() -> Self {
            Self {
                ask_mic_id: Default::default(),
                ask_price: Default::default(),
                ask_size: Default::default(),
                ask_time: Default::default(),
                bid_mic_id: Default::default(),
                bid_price: Default::default(),
                bid_size: Default::default(),
                bid_time: Default::default(),
                close_price: Default::default(),
                high_price: Default::default(),
                last_mic_id: Default::default(),
                last_price: Default::default(),
                last_size: Default::default(),
                low_price: Default::default(),
                mark: Default::default(),
                mark_change: Default::default(),
                mark_percent_change: Default::default(),
                net_change: Default::default(),
                net_percent_change: Default::default(),
                open_price: Default::default(),
                quote_time: Default::default(),
                security_status: Default::default(),
                total_volume: Default::default(),
                trade_time: Default::default(),
                volatility: Default::default(),
                x52week_high: Default::default(),
                x52week_low: Default::default(),
            }
        }
    }

    impl QuoteEquity {
        pub fn builder() -> builder::QuoteEquity {
            Default::default()
        }
    }

    ///Partial or Custom errors per request
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Partial or Custom errors per request",
    ///  "type": "object",
    ///  "properties": {
    ///    "invalidCusips": {
    ///      "description": "list of invalid cusips from request",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "invalidSSIDs": {
    ///      "description": "list of invalid SSIDs from request",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "int64"
    ///      }
    ///    },
    ///    "invalidSymbols": {
    ///      "description": "list of invalid symbols from request",
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct QuoteError {
        ///list of invalid cusips from request
        #[serde(
            rename = "invalidCusips",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub invalid_cusips: ::std::vec::Vec<::std::string::String>,
        ///list of invalid SSIDs from request
        #[serde(
            rename = "invalidSSIDs",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub invalid_ssi_ds: ::std::vec::Vec<i64>,
        ///list of invalid symbols from request
        #[serde(
            rename = "invalidSymbols",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub invalid_symbols: ::std::vec::Vec<::std::string::String>,
    }

    impl ::std::default::Default for QuoteError {
        fn default() -> Self {
            Self {
                invalid_cusips: Default::default(),
                invalid_ssi_ds: Default::default(),
                invalid_symbols: Default::default(),
            }
        }
    }

    impl QuoteError {
        pub fn builder() -> builder::QuoteError {
            Default::default()
        }
    }

    ///Quote data of Forex security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Quote data of Forex security",
    ///  "type": "object",
    ///  "properties": {
    ///    "52WeekHigh": {
    ///      "description": "Higest price traded in the past 12 months, or 52
    /// weeks",
    ///      "examples": [
    ///        145.09
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "52WeekLow": {
    ///      "description": "Lowest price traded in the past 12 months, or 52
    /// weeks",
    ///      "examples": [
    ///        77.581
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "askPrice": {
    ///      "description": "Current Best Ask Price",
    ///      "examples": [
    ///        124.63
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "askSize": {
    ///      "description": "Number of shares for ask",
    ///      "examples": [
    ///        700
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "bidPrice": {
    ///      "description": "Current Best Bid Price",
    ///      "examples": [
    ///        124.6
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "bidSize": {
    ///      "description": "Number of shares for bid",
    ///      "examples": [
    ///        300
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "closePrice": {
    ///      "description": "Previous day's closing price",
    ///      "examples": [
    ///        126.27
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "highPrice": {
    ///      "description": "Day's high trade price",
    ///      "examples": [
    ///        126.99
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "lastPrice": {
    ///      "examples": [
    ///        122.3
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "lastSize": {
    ///      "description": "Number of shares traded with last trade",
    ///      "examples": [
    ///        100
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "lowPrice": {
    ///      "description": "Day's low trade price",
    ///      "examples": [
    ///        52.74
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "mark": {
    ///      "description": "Mark price",
    ///      "examples": [
    ///        52.93
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "netChange": {
    ///      "description": "Current Last-Prev Close",
    ///      "examples": [
    ///        -0.04
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "netPercentChange": {
    ///      "description": "Net Percentage Change",
    ///      "examples": [
    ///        -0.0756
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "openPrice": {
    ///      "description": "Price at market open",
    ///      "examples": [
    ///        52.8
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "quoteTime": {
    ///      "description": "Last quote time in milliseconds since Epoch",
    ///      "examples": [
    ///        1621376892336
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "securityStatus": {
    ///      "description": "Status of security",
    ///      "examples": [
    ///        "Normal"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "tick": {
    ///      "description": "Tick Price",
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "tickAmount": {
    ///      "description": "Tick Amount",
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "totalVolume": {
    ///      "description": "Aggregated shares traded throughout the day,
    /// including pre/post market hours.",
    ///      "examples": [
    ///        20171188
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "tradeTime": {
    ///      "description": "Last trade time in milliseconds since Epoch",
    ///      "examples": [
    ///        1621376731304
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct QuoteForex {
        #[serde(
            rename = "askPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_price: ::std::option::Option<f64>,
        ///Number of shares for ask
        #[serde(
            rename = "askSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_size: ::std::option::Option<i32>,
        #[serde(
            rename = "bidPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_price: ::std::option::Option<f64>,
        ///Number of shares for bid
        #[serde(
            rename = "bidSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_size: ::std::option::Option<i32>,
        #[serde(
            rename = "closePrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub close_price: ::std::option::Option<f64>,
        #[serde(
            rename = "highPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub high_price: ::std::option::Option<f64>,
        #[serde(
            rename = "lastPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_price: ::std::option::Option<f64>,
        ///Number of shares traded with last trade
        #[serde(
            rename = "lastSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_size: ::std::option::Option<i32>,
        #[serde(
            rename = "lowPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub low_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mark: ::std::option::Option<f64>,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(
            rename = "netPercentChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_percent_change: ::std::option::Option<f64>,
        #[serde(
            rename = "openPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_price: ::std::option::Option<f64>,
        ///Last quote time in milliseconds since Epoch
        #[serde(
            rename = "quoteTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub quote_time: ::std::option::Option<i64>,
        ///Status of security
        #[serde(
            rename = "securityStatus",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub security_status: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tick: ::std::option::Option<f64>,
        #[serde(
            rename = "tickAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tick_amount: ::std::option::Option<f64>,
        ///Aggregated shares traded throughout the day, including pre/post
        /// market hours.
        #[serde(
            rename = "totalVolume",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_volume: ::std::option::Option<i64>,
        ///Last trade time in milliseconds since Epoch
        #[serde(
            rename = "tradeTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trade_time: ::std::option::Option<i64>,
        #[serde(
            rename = "52WeekHigh",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub x52week_high: ::std::option::Option<f64>,
        #[serde(
            rename = "52WeekLow",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub x52week_low: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for QuoteForex {
        fn default() -> Self {
            Self {
                ask_price: Default::default(),
                ask_size: Default::default(),
                bid_price: Default::default(),
                bid_size: Default::default(),
                close_price: Default::default(),
                high_price: Default::default(),
                last_price: Default::default(),
                last_size: Default::default(),
                low_price: Default::default(),
                mark: Default::default(),
                net_change: Default::default(),
                net_percent_change: Default::default(),
                open_price: Default::default(),
                quote_time: Default::default(),
                security_status: Default::default(),
                tick: Default::default(),
                tick_amount: Default::default(),
                total_volume: Default::default(),
                trade_time: Default::default(),
                x52week_high: Default::default(),
                x52week_low: Default::default(),
            }
        }
    }

    impl QuoteForex {
        pub fn builder() -> builder::QuoteForex {
            Default::default()
        }
    }

    ///Quote data of Future security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Quote data of Future security",
    ///  "type": "object",
    ///  "properties": {
    ///    "askMICId": {
    ///      "description": "ask MIC code",
    ///      "examples": [
    ///        "XNYS"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "askPrice": {
    ///      "description": "Current Best Ask Price",
    ///      "examples": [
    ///        4083.25
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "askSize": {
    ///      "description": "Number of shares for ask",
    ///      "examples": [
    ///        36
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "askTime": {
    ///      "description": "Last ask time in milliseconds since Epoch",
    ///      "examples": [
    ///        1621376892336
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "bidMICId": {
    ///      "description": "bid MIC code",
    ///      "examples": [
    ///        "XNYS"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "bidPrice": {
    ///      "description": "Current Best Bid Price",
    ///      "examples": [
    ///        4083
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "bidSize": {
    ///      "description": "Number of shares for bid",
    ///      "examples": [
    ///        18
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "bidTime": {
    ///      "description": "Last bid time in milliseconds since Epoch",
    ///      "examples": [
    ///        1621376892336
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "closePrice": {
    ///      "description": "Previous day's closing price",
    ///      "examples": [
    ///        4123
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "futurePercentChange": {
    ///      "description": "Net Percentage Change",
    ///      "examples": [
    ///        -0.0756
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "highPrice": {
    ///      "description": "Day's high trade price",
    ///      "examples": [
    ///        4123
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "lastMICId": {
    ///      "description": "Last MIC Code",
    ///      "examples": [
    ///        "XNYS"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "lastPrice": {
    ///      "examples": [
    ///        4083
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "lastSize": {
    ///      "description": "Number of shares traded with last trade",
    ///      "examples": [
    ///        7
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "lowPrice": {
    ///      "description": "Day's low trade price",
    ///      "examples": [
    ///        4075.5
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "mark": {
    ///      "description": "Mark price",
    ///      "examples": [
    ///        4083
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "netChange": {
    ///      "description": "Current Last-Prev Close",
    ///      "examples": [
    ///        -40
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "openInterest": {
    ///      "description": "Open interest",
    ///      "examples": [
    ///        2517139
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "openPrice": {
    ///      "description": "Price at market open",
    ///      "examples": [
    ///        4114
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "quoteTime": {
    ///      "description": "Last quote time in milliseconds since Epoch",
    ///      "examples": [
    ///        1621427004585
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "quotedInSession": {
    ///      "description": "quoted during trading session",
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "securityStatus": {
    ///      "description": "Status of security",
    ///      "examples": [
    ///        "Normal"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "settleTime": {
    ///      "description": "settlement time in milliseconds since Epoch",
    ///      "examples": [
    ///        1621376892336
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "tick": {
    ///      "description": "Tick Price",
    ///      "examples": [
    ///        0.25
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "tickAmount": {
    ///      "description": "Tick Amount",
    ///      "examples": [
    ///        12.5
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "totalVolume": {
    ///      "description": "Aggregated shares traded throughout the day,
    /// including pre/post market hours.",
    ///      "examples": [
    ///        20171188
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "tradeTime": {
    ///      "description": "Last trade time in milliseconds since Epoch",
    ///      "examples": [
    ///        1621376731304
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct QuoteFuture {
        ///ask MIC code
        #[serde(
            rename = "askMICId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_mic_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "askPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_price: ::std::option::Option<f64>,
        ///Number of shares for ask
        #[serde(
            rename = "askSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_size: ::std::option::Option<i32>,
        ///Last ask time in milliseconds since Epoch
        #[serde(
            rename = "askTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_time: ::std::option::Option<i64>,
        ///bid MIC code
        #[serde(
            rename = "bidMICId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_mic_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "bidPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_price: ::std::option::Option<f64>,
        ///Number of shares for bid
        #[serde(
            rename = "bidSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_size: ::std::option::Option<i32>,
        ///Last bid time in milliseconds since Epoch
        #[serde(
            rename = "bidTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_time: ::std::option::Option<i64>,
        #[serde(
            rename = "closePrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub close_price: ::std::option::Option<f64>,
        #[serde(
            rename = "futurePercentChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub future_percent_change: ::std::option::Option<f64>,
        #[serde(
            rename = "highPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub high_price: ::std::option::Option<f64>,
        ///Last MIC Code
        #[serde(
            rename = "lastMICId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_mic_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "lastPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_price: ::std::option::Option<f64>,
        ///Number of shares traded with last trade
        #[serde(
            rename = "lastSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_size: ::std::option::Option<i32>,
        #[serde(
            rename = "lowPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub low_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mark: ::std::option::Option<f64>,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        ///Open interest
        #[serde(
            rename = "openInterest",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_interest: ::std::option::Option<i32>,
        #[serde(
            rename = "openPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_price: ::std::option::Option<f64>,
        ///Last quote time in milliseconds since Epoch
        #[serde(
            rename = "quoteTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub quote_time: ::std::option::Option<i64>,
        ///quoted during trading session
        #[serde(
            rename = "quotedInSession",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub quoted_in_session: ::std::option::Option<bool>,
        ///Status of security
        #[serde(
            rename = "securityStatus",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub security_status: ::std::option::Option<::std::string::String>,
        ///settlement time in milliseconds since Epoch
        #[serde(
            rename = "settleTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub settle_time: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tick: ::std::option::Option<f64>,
        #[serde(
            rename = "tickAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tick_amount: ::std::option::Option<f64>,
        ///Aggregated shares traded throughout the day, including pre/post
        /// market hours.
        #[serde(
            rename = "totalVolume",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_volume: ::std::option::Option<i64>,
        ///Last trade time in milliseconds since Epoch
        #[serde(
            rename = "tradeTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trade_time: ::std::option::Option<i64>,
    }

    impl ::std::default::Default for QuoteFuture {
        fn default() -> Self {
            Self {
                ask_mic_id: Default::default(),
                ask_price: Default::default(),
                ask_size: Default::default(),
                ask_time: Default::default(),
                bid_mic_id: Default::default(),
                bid_price: Default::default(),
                bid_size: Default::default(),
                bid_time: Default::default(),
                close_price: Default::default(),
                future_percent_change: Default::default(),
                high_price: Default::default(),
                last_mic_id: Default::default(),
                last_price: Default::default(),
                last_size: Default::default(),
                low_price: Default::default(),
                mark: Default::default(),
                net_change: Default::default(),
                open_interest: Default::default(),
                open_price: Default::default(),
                quote_time: Default::default(),
                quoted_in_session: Default::default(),
                security_status: Default::default(),
                settle_time: Default::default(),
                tick: Default::default(),
                tick_amount: Default::default(),
                total_volume: Default::default(),
                trade_time: Default::default(),
            }
        }
    }

    impl QuoteFuture {
        pub fn builder() -> builder::QuoteFuture {
            Default::default()
        }
    }

    ///Quote data of Option security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Quote data of Option security",
    ///  "type": "object",
    ///  "properties": {
    ///    "askMICId": {
    ///      "description": "ask MIC code",
    ///      "examples": [
    ///        "XNYS"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "askPrice": {
    ///      "description": "Current Best Ask Price",
    ///      "examples": [
    ///        124.63
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "askSize": {
    ///      "description": "Number of shares for ask",
    ///      "examples": [
    ///        700
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "bidMICId": {
    ///      "description": "bid MIC code",
    ///      "examples": [
    ///        "XNYS"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "bidPrice": {
    ///      "description": "Current Best Bid Price",
    ///      "examples": [
    ///        124.6
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "bidSize": {
    ///      "description": "Number of shares for bid",
    ///      "examples": [
    ///        300
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "closePrice": {
    ///      "description": "Previous day's closing price",
    ///      "examples": [
    ///        126.27
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "highPrice": {
    ///      "description": "Day's high trade price",
    ///      "examples": [
    ///        126.99
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "lastMICId": {
    ///      "description": "Last MIC Code",
    ///      "examples": [
    ///        "XNYS"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "lastPrice": {
    ///      "examples": [
    ///        122.3
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "lastSize": {
    ///      "description": "Number of shares traded with last trade",
    ///      "examples": [
    ///        100
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "lowPrice": {
    ///      "description": "Day's low trade price",
    ///      "examples": [
    ///        52.74
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "mark": {
    ///      "description": "Mark price",
    ///      "examples": [
    ///        52.93
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "markChange": {
    ///      "description": "Mark Price change",
    ///      "examples": [
    ///        -0.04
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "netChange": {
    ///      "description": "Current Last-Prev Close",
    ///      "examples": [
    ///        -0.04
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "netPercentChange": {
    ///      "description": "Net Percentage Change",
    ///      "examples": [
    ///        -0.0756
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "openInterest": {
    ///      "description": "Open Interest",
    ///      "examples": [
    ///        317
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "openPrice": {
    ///      "description": "Price at market open",
    ///      "examples": [
    ///        52.8
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "quoteTime": {
    ///      "description": "Last quote time in milliseconds since Epoch",
    ///      "examples": [
    ///        1621376892336
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "securityStatus": {
    ///      "description": "Status of security",
    ///      "examples": [
    ///        "Normal"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "settlemetPrice": {
    ///      "description": "Price at market open",
    ///      "examples": [
    ///        52.8
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "tick": {
    ///      "description": "Tick Price",
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "tickAmount": {
    ///      "description": "Tick Amount",
    ///      "examples": [
    ///        0
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "totalVolume": {
    ///      "description": "Aggregated shares traded throughout the day,
    /// including pre/post market hours.",
    ///      "examples": [
    ///        20171188
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "tradeTime": {
    ///      "description": "Last trade time in milliseconds since Epoch",
    ///      "examples": [
    ///        1621376731304
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct QuoteFutureOption {
        ///ask MIC code
        #[serde(
            rename = "askMICId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_mic_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "askPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_price: ::std::option::Option<f64>,
        ///Number of shares for ask
        #[serde(
            rename = "askSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_size: ::std::option::Option<i32>,
        ///bid MIC code
        #[serde(
            rename = "bidMICId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_mic_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "bidPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_price: ::std::option::Option<f64>,
        ///Number of shares for bid
        #[serde(
            rename = "bidSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_size: ::std::option::Option<i32>,
        #[serde(
            rename = "closePrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub close_price: ::std::option::Option<f64>,
        #[serde(
            rename = "highPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub high_price: ::std::option::Option<f64>,
        ///Last MIC Code
        #[serde(
            rename = "lastMICId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_mic_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "lastPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_price: ::std::option::Option<f64>,
        ///Number of shares traded with last trade
        #[serde(
            rename = "lastSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_size: ::std::option::Option<i32>,
        #[serde(
            rename = "lowPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub low_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mark: ::std::option::Option<f64>,
        #[serde(
            rename = "markChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mark_change: ::std::option::Option<f64>,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(
            rename = "netPercentChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_percent_change: ::std::option::Option<f64>,
        ///Open Interest
        #[serde(
            rename = "openInterest",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_interest: ::std::option::Option<i32>,
        #[serde(
            rename = "openPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_price: ::std::option::Option<f64>,
        ///Last quote time in milliseconds since Epoch
        #[serde(
            rename = "quoteTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub quote_time: ::std::option::Option<i64>,
        ///Status of security
        #[serde(
            rename = "securityStatus",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub security_status: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "settlemetPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub settlemet_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tick: ::std::option::Option<f64>,
        #[serde(
            rename = "tickAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tick_amount: ::std::option::Option<f64>,
        ///Aggregated shares traded throughout the day, including pre/post
        /// market hours.
        #[serde(
            rename = "totalVolume",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_volume: ::std::option::Option<i64>,
        ///Last trade time in milliseconds since Epoch
        #[serde(
            rename = "tradeTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trade_time: ::std::option::Option<i64>,
    }

    impl ::std::default::Default for QuoteFutureOption {
        fn default() -> Self {
            Self {
                ask_mic_id: Default::default(),
                ask_price: Default::default(),
                ask_size: Default::default(),
                bid_mic_id: Default::default(),
                bid_price: Default::default(),
                bid_size: Default::default(),
                close_price: Default::default(),
                high_price: Default::default(),
                last_mic_id: Default::default(),
                last_price: Default::default(),
                last_size: Default::default(),
                low_price: Default::default(),
                mark: Default::default(),
                mark_change: Default::default(),
                net_change: Default::default(),
                net_percent_change: Default::default(),
                open_interest: Default::default(),
                open_price: Default::default(),
                quote_time: Default::default(),
                security_status: Default::default(),
                settlemet_price: Default::default(),
                tick: Default::default(),
                tick_amount: Default::default(),
                total_volume: Default::default(),
                trade_time: Default::default(),
            }
        }
    }

    impl QuoteFutureOption {
        pub fn builder() -> builder::QuoteFutureOption {
            Default::default()
        }
    }

    ///Quote data of Index security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Quote data of Index security",
    ///  "type": "object",
    ///  "properties": {
    ///    "52WeekHigh": {
    ///      "description": "Higest price traded in the past 12 months, or 52
    /// weeks",
    ///      "examples": [
    ///        145.09
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "52WeekLow": {
    ///      "description": "Lowest price traded in the past 12 months, or 52
    /// weeks",
    ///      "examples": [
    ///        77.581
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "closePrice": {
    ///      "description": "Previous day's closing price",
    ///      "examples": [
    ///        126.27
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "highPrice": {
    ///      "description": "Day's high trade price",
    ///      "examples": [
    ///        126.99
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "lastPrice": {
    ///      "examples": [
    ///        122.3
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "lowPrice": {
    ///      "description": "Day's low trade price",
    ///      "examples": [
    ///        52.74
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "netChange": {
    ///      "description": "Current Last-Prev Close",
    ///      "examples": [
    ///        -0.04
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "netPercentChange": {
    ///      "description": "Net Percentage Change",
    ///      "examples": [
    ///        -0.0756
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "openPrice": {
    ///      "description": "Price at market open",
    ///      "examples": [
    ///        52.8
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "securityStatus": {
    ///      "description": "Status of security",
    ///      "examples": [
    ///        "Normal"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "totalVolume": {
    ///      "description": "Aggregated shares traded throughout the day,
    /// including pre/post market hours.",
    ///      "examples": [
    ///        20171188
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "tradeTime": {
    ///      "description": "Last trade time in milliseconds since Epoch",
    ///      "examples": [
    ///        1621376731304
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct QuoteIndex {
        #[serde(
            rename = "closePrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub close_price: ::std::option::Option<f64>,
        #[serde(
            rename = "highPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub high_price: ::std::option::Option<f64>,
        #[serde(
            rename = "lastPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_price: ::std::option::Option<f64>,
        #[serde(
            rename = "lowPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub low_price: ::std::option::Option<f64>,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(
            rename = "netPercentChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_percent_change: ::std::option::Option<f64>,
        #[serde(
            rename = "openPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_price: ::std::option::Option<f64>,
        ///Status of security
        #[serde(
            rename = "securityStatus",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub security_status: ::std::option::Option<::std::string::String>,
        ///Aggregated shares traded throughout the day, including pre/post
        /// market hours.
        #[serde(
            rename = "totalVolume",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_volume: ::std::option::Option<i64>,
        ///Last trade time in milliseconds since Epoch
        #[serde(
            rename = "tradeTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trade_time: ::std::option::Option<i64>,
        #[serde(
            rename = "52WeekHigh",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub x52week_high: ::std::option::Option<f64>,
        #[serde(
            rename = "52WeekLow",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub x52week_low: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for QuoteIndex {
        fn default() -> Self {
            Self {
                close_price: Default::default(),
                high_price: Default::default(),
                last_price: Default::default(),
                low_price: Default::default(),
                net_change: Default::default(),
                net_percent_change: Default::default(),
                open_price: Default::default(),
                security_status: Default::default(),
                total_volume: Default::default(),
                trade_time: Default::default(),
                x52week_high: Default::default(),
                x52week_low: Default::default(),
            }
        }
    }

    impl QuoteIndex {
        pub fn builder() -> builder::QuoteIndex {
            Default::default()
        }
    }

    ///Quote data of Mutual Fund security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Quote data of Mutual Fund security",
    ///  "type": "object",
    ///  "properties": {
    ///    "52WeekHigh": {
    ///      "description": "Higest price traded in the past 12 months, or 52
    /// weeks",
    ///      "examples": [
    ///        145.09
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "52WeekLow": {
    ///      "description": "Lowest price traded in the past 12 months, or 52
    /// weeks",
    ///      "examples": [
    ///        77.581
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "closePrice": {
    ///      "description": "Previous day's closing price",
    ///      "examples": [
    ///        126.27
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "nAV": {
    ///      "description": "Net Asset Value",
    ///      "examples": [
    ///        126.99
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "netChange": {
    ///      "description": "Current Last-Prev Close",
    ///      "examples": [
    ///        -0.04
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "netPercentChange": {
    ///      "description": "Net Percentage Change",
    ///      "examples": [
    ///        -0.0756
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "securityStatus": {
    ///      "description": "Status of security",
    ///      "examples": [
    ///        "Normal"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "totalVolume": {
    ///      "description": "Aggregated shares traded throughout the day,
    /// including pre/post market hours.",
    ///      "examples": [
    ///        20171188
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "tradeTime": {
    ///      "description": "Last trade time in milliseconds since Epoch",
    ///      "examples": [
    ///        1621376731304
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct QuoteMutualFund {
        #[serde(
            rename = "closePrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub close_price: ::std::option::Option<f64>,
        #[serde(
            rename = "nAV",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub n_av: ::std::option::Option<f64>,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(
            rename = "netPercentChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_percent_change: ::std::option::Option<f64>,
        ///Status of security
        #[serde(
            rename = "securityStatus",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub security_status: ::std::option::Option<::std::string::String>,
        ///Aggregated shares traded throughout the day, including pre/post
        /// market hours.
        #[serde(
            rename = "totalVolume",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_volume: ::std::option::Option<i64>,
        ///Last trade time in milliseconds since Epoch
        #[serde(
            rename = "tradeTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trade_time: ::std::option::Option<i64>,
        #[serde(
            rename = "52WeekHigh",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub x52week_high: ::std::option::Option<f64>,
        #[serde(
            rename = "52WeekLow",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub x52week_low: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for QuoteMutualFund {
        fn default() -> Self {
            Self {
                close_price: Default::default(),
                n_av: Default::default(),
                net_change: Default::default(),
                net_percent_change: Default::default(),
                security_status: Default::default(),
                total_volume: Default::default(),
                trade_time: Default::default(),
                x52week_high: Default::default(),
                x52week_low: Default::default(),
            }
        }
    }

    impl QuoteMutualFund {
        pub fn builder() -> builder::QuoteMutualFund {
            Default::default()
        }
    }

    ///Quote data of Option security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Quote data of Option security",
    ///  "type": "object",
    ///  "properties": {
    ///    "52WeekHigh": {
    ///      "description": "Higest price traded in the past 12 months, or 52
    /// weeks",
    ///      "examples": [
    ///        145.09
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "52WeekLow": {
    ///      "description": "Lowest price traded in the past 12 months, or 52
    /// weeks",
    ///      "examples": [
    ///        77.581
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "askPrice": {
    ///      "description": "Current Best Ask Price",
    ///      "examples": [
    ///        124.63
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "askSize": {
    ///      "description": "Number of shares for ask",
    ///      "examples": [
    ///        700
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "bidPrice": {
    ///      "description": "Current Best Bid Price",
    ///      "examples": [
    ///        124.6
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "bidSize": {
    ///      "description": "Number of shares for bid",
    ///      "examples": [
    ///        300
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "closePrice": {
    ///      "description": "Previous day's closing price",
    ///      "examples": [
    ///        126.27
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "delta": {
    ///      "description": "Delta Value",
    ///      "examples": [
    ///        -0.0407
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "gamma": {
    ///      "description": "Gamma Value",
    ///      "examples": [
    ///        0.0001
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "highPrice": {
    ///      "description": "Day's high trade price",
    ///      "examples": [
    ///        126.99
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "impliedYield": {
    ///      "description": "Implied Yield",
    ///      "examples": [
    ///        -0.0067
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "indAskPrice": {
    ///      "description": "Indicative Ask Price applicable only for Indicative
    /// Option Symbols",
    ///      "examples": [
    ///        126.99
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "indBidPrice": {
    ///      "description": "Indicative Bid Price applicable only for Indicative
    /// Option Symbols",
    ///      "examples": [
    ///        126.99
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "indQuoteTime": {
    ///      "description": "Indicative Quote Time in milliseconds since Epoch
    /// applicable only for Indicative Option Symbols",
    ///      "examples": [
    ///        126.99
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "lastPrice": {
    ///      "examples": [
    ///        122.3
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "lastSize": {
    ///      "description": "Number of shares traded with last trade",
    ///      "examples": [
    ///        100
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "lowPrice": {
    ///      "description": "Day's low trade price",
    ///      "examples": [
    ///        52.74
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "mark": {
    ///      "description": "Mark price",
    ///      "examples": [
    ///        52.93
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "markChange": {
    ///      "description": "Mark Price change",
    ///      "examples": [
    ///        -0.01
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "markPercentChange": {
    ///      "description": "Mark Price percent change",
    ///      "examples": [
    ///        -0.0189
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "moneyIntrinsicValue": {
    ///      "description": "Money Intrinsic Value",
    ///      "examples": [
    ///        -947.96
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "netChange": {
    ///      "description": "Current Last-Prev Close",
    ///      "examples": [
    ///        -0.04
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "netPercentChange": {
    ///      "description": "Net Percentage Change",
    ///      "examples": [
    ///        -0.0756
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "openInterest": {
    ///      "description": "Open Interest",
    ///      "examples": [
    ///        317
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "openPrice": {
    ///      "description": "Price at market open",
    ///      "examples": [
    ///        52.8
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "quoteTime": {
    ///      "description": "Last quote time in milliseconds since Epoch",
    ///      "examples": [
    ///        1621376892336
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "rho": {
    ///      "description": "Rho Value",
    ///      "examples": [
    ///        -0.3732
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "securityStatus": {
    ///      "description": "Status of security",
    ///      "examples": [
    ///        "Normal"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "theoreticalOptionValue": {
    ///      "description": "Theoretical option Value",
    ///      "examples": [
    ///        12.275
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "theta": {
    ///      "description": "Theta Value",
    ///      "examples": [
    ///        -0.315
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "timeValue": {
    ///      "description": "Time Value",
    ///      "examples": [
    ///        12.22
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "totalVolume": {
    ///      "description": "Aggregated shares traded throughout the day,
    /// including pre/post market hours.",
    ///      "examples": [
    ///        20171188
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "tradeTime": {
    ///      "description": "Last trade time in milliseconds since Epoch",
    ///      "examples": [
    ///        1621376731304
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "underlyingPrice": {
    ///      "description": "Underlying Price",
    ///      "examples": [
    ///        3247.96
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "vega": {
    ///      "description": "Vega Value",
    ///      "examples": [
    ///        1.4455
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "volatility": {
    ///      "description": "Option Risk/Volatility Measurement",
    ///      "examples": [
    ///        0.0094
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct QuoteOption {
        #[serde(
            rename = "askPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_price: ::std::option::Option<f64>,
        ///Number of shares for ask
        #[serde(
            rename = "askSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_size: ::std::option::Option<i32>,
        #[serde(
            rename = "bidPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_price: ::std::option::Option<f64>,
        ///Number of shares for bid
        #[serde(
            rename = "bidSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_size: ::std::option::Option<i32>,
        #[serde(
            rename = "closePrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub close_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub delta: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub gamma: ::std::option::Option<f64>,
        #[serde(
            rename = "highPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub high_price: ::std::option::Option<f64>,
        #[serde(
            rename = "impliedYield",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub implied_yield: ::std::option::Option<f64>,
        #[serde(
            rename = "indAskPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ind_ask_price: ::std::option::Option<f64>,
        #[serde(
            rename = "indBidPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ind_bid_price: ::std::option::Option<f64>,
        ///Indicative Quote Time in milliseconds since Epoch applicable only
        /// for Indicative Option Symbols
        #[serde(
            rename = "indQuoteTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ind_quote_time: ::std::option::Option<i64>,
        #[serde(
            rename = "lastPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_price: ::std::option::Option<f64>,
        ///Number of shares traded with last trade
        #[serde(
            rename = "lastSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_size: ::std::option::Option<i32>,
        #[serde(
            rename = "lowPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub low_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mark: ::std::option::Option<f64>,
        #[serde(
            rename = "markChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mark_change: ::std::option::Option<f64>,
        #[serde(
            rename = "markPercentChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mark_percent_change: ::std::option::Option<f64>,
        #[serde(
            rename = "moneyIntrinsicValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub money_intrinsic_value: ::std::option::Option<f64>,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(
            rename = "netPercentChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_percent_change: ::std::option::Option<f64>,
        #[serde(
            rename = "openInterest",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_interest: ::std::option::Option<f64>,
        #[serde(
            rename = "openPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_price: ::std::option::Option<f64>,
        ///Last quote time in milliseconds since Epoch
        #[serde(
            rename = "quoteTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub quote_time: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub rho: ::std::option::Option<f64>,
        ///Status of security
        #[serde(
            rename = "securityStatus",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub security_status: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "theoreticalOptionValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub theoretical_option_value: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub theta: ::std::option::Option<f64>,
        #[serde(
            rename = "timeValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub time_value: ::std::option::Option<f64>,
        ///Aggregated shares traded throughout the day, including pre/post
        /// market hours.
        #[serde(
            rename = "totalVolume",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_volume: ::std::option::Option<i64>,
        ///Last trade time in milliseconds since Epoch
        #[serde(
            rename = "tradeTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trade_time: ::std::option::Option<i64>,
        #[serde(
            rename = "underlyingPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub underlying_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub vega: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub volatility: ::std::option::Option<f64>,
        #[serde(
            rename = "52WeekHigh",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub x52week_high: ::std::option::Option<f64>,
        #[serde(
            rename = "52WeekLow",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub x52week_low: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for QuoteOption {
        fn default() -> Self {
            Self {
                ask_price: Default::default(),
                ask_size: Default::default(),
                bid_price: Default::default(),
                bid_size: Default::default(),
                close_price: Default::default(),
                delta: Default::default(),
                gamma: Default::default(),
                high_price: Default::default(),
                implied_yield: Default::default(),
                ind_ask_price: Default::default(),
                ind_bid_price: Default::default(),
                ind_quote_time: Default::default(),
                last_price: Default::default(),
                last_size: Default::default(),
                low_price: Default::default(),
                mark: Default::default(),
                mark_change: Default::default(),
                mark_percent_change: Default::default(),
                money_intrinsic_value: Default::default(),
                net_change: Default::default(),
                net_percent_change: Default::default(),
                open_interest: Default::default(),
                open_price: Default::default(),
                quote_time: Default::default(),
                rho: Default::default(),
                security_status: Default::default(),
                theoretical_option_value: Default::default(),
                theta: Default::default(),
                time_value: Default::default(),
                total_volume: Default::default(),
                trade_time: Default::default(),
                underlying_price: Default::default(),
                vega: Default::default(),
                volatility: Default::default(),
                x52week_high: Default::default(),
                x52week_low: Default::default(),
            }
        }
    }

    impl QuoteOption {
        pub fn builder() -> builder::QuoteOption {
            Default::default()
        }
    }

    ///Request one or more quote data in POST body
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Request one or more quote data in POST body",
    ///  "type": "object",
    ///  "properties": {
    ///    "cusips": {
    ///      "description": "List of cusip, max of 500 of symbols+cusip+ssids",
    ///      "examples": [
    ///        [
    ///          808524680,
    ///          594918104
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "fields": {
    ///      "description": "comma separated list of nodes in each quote<br/>
    /// possible values are quote,fundamental,reference,extended,regular. Dont
    /// send this attribute for full response.",
    ///      "examples": [
    ///        "quote,reference"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "indicative": {
    ///      "description": "Include indicative symbol quotes for all ETF
    /// symbols in request. If ETF symbol ABC is in request and indicative=true
    /// API will return quotes for ABC and its corresponding indicative quote
    /// for $ABC.IV",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean",
    ///      "enum": [
    ///        true,
    ///        false
    ///      ]
    ///    },
    ///    "realtime": {
    ///      "description": "Get realtime quotes and skip entitlement check",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean",
    ///      "enum": [
    ///        true,
    ///        false
    ///      ]
    ///    },
    ///    "ssids": {
    ///      "description": "List of Schwab securityid[SSID], max of 500 of
    /// symbols+cusip+ssids",
    ///      "examples": [
    ///        [
    ///          1516105793,
    ///          34621523
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "integer",
    ///        "format": "int64",
    ///        "maximum": 9999999999.0,
    ///        "minimum": 1.0
    ///      }
    ///    },
    ///    "symbols": {
    ///      "description": "List of symbols, max of 500 of
    /// symbols+cusip+ssids",
    ///      "examples": [
    ///        [
    ///          "MRAD",
    ///          "EATOF",
    ///          "EBIZ",
    ///          "AAPL",
    ///          "BAC",
    ///          "AAAHX",
    ///          "AAAIX",
    ///          "$DJI",
    ///          "$SPX",
    ///          "MVEN",
    ///          "SOBS",
    ///          "TOITF",
    ///          "CNSWF",
    ///          "AMZN  230317C01360000",
    ///          "DJX   231215C00290000",
    ///          "/ESH23",
    ///          "./ADUF23C0.55",
    ///          "AUD/CAD"
    ///        ]
    ///      ],
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct QuoteRequest {
        ///List of cusip, max of 500 of symbols+cusip+ssids
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub cusips: ::std::vec::Vec<::std::string::String>,
        ///comma separated list of nodes in each quote<br/> possible values are
        /// quote,fundamental,reference,extended,regular. Dont send this
        /// attribute for full response.
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fields: ::std::option::Option<::std::string::String>,
        ///Include indicative symbol quotes for all ETF symbols in request. If
        /// ETF symbol ABC is in request and indicative=true API will return
        /// quotes for ABC and its corresponding indicative quote for $ABC.IV
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub indicative: ::std::option::Option<bool>,
        ///Get realtime quotes and skip entitlement check
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub realtime: ::std::option::Option<bool>,
        ///List of Schwab securityid[SSID], max of 500 of symbols+cusip+ssids
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub ssids: ::std::vec::Vec<::std::num::NonZeroU64>,
        ///List of symbols, max of 500 of symbols+cusip+ssids
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub symbols: ::std::vec::Vec<::std::string::String>,
    }

    impl ::std::default::Default for QuoteRequest {
        fn default() -> Self {
            Self {
                cusips: Default::default(),
                fields: Default::default(),
                indicative: Default::default(),
                realtime: Default::default(),
                ssids: Default::default(),
                symbols: Default::default(),
            }
        }
    }

    impl QuoteRequest {
        pub fn builder() -> builder::QuoteRequest {
            Default::default()
        }
    }

    ///a (symbol, QuoteResponse) map. `SCHW`is an example key
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "a (symbol, QuoteResponse) map. `SCHW`is an example
    /// key",
    ///  "type": "object",
    ///  "additionalProperties": {
    ///    "$ref": "#/components/schemas/QuoteResponseObject"
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct QuoteResponse(
        pub ::std::collections::HashMap<::std::string::String, QuoteResponseObject>,
    );
    impl ::std::ops::Deref for QuoteResponse {
        type Target = ::std::collections::HashMap<::std::string::String, QuoteResponseObject>;
        fn deref(
            &self,
        ) -> &::std::collections::HashMap<::std::string::String, QuoteResponseObject> {
            &self.0
        }
    }

    impl ::std::convert::From<QuoteResponse>
        for ::std::collections::HashMap<::std::string::String, QuoteResponseObject>
    {
        fn from(value: QuoteResponse) -> Self {
            value.0
        }
    }

    impl
        ::std::convert::From<
            ::std::collections::HashMap<::std::string::String, QuoteResponseObject>,
        > for QuoteResponse
    {
        fn from(
            value: ::std::collections::HashMap<::std::string::String, QuoteResponseObject>,
        ) -> Self {
            Self(value)
        }
    }

    ///`QuoteResponseObject`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/EquityResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/OptionResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/ForexResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/FutureResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/FutureOptionResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/IndexResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/MutualFundResponse"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/QuoteError"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum QuoteResponseObject {
        EquityResponse(EquityResponse),
        OptionResponse(OptionResponse),
        ForexResponse(ForexResponse),
        FutureResponse(FutureResponse),
        FutureOptionResponse(FutureOptionResponse),
        IndexResponse(IndexResponse),
        MutualFundResponse(MutualFundResponse),
        QuoteError(QuoteError),
    }

    impl ::std::convert::From<EquityResponse> for QuoteResponseObject {
        fn from(value: EquityResponse) -> Self {
            Self::EquityResponse(value)
        }
    }

    impl ::std::convert::From<OptionResponse> for QuoteResponseObject {
        fn from(value: OptionResponse) -> Self {
            Self::OptionResponse(value)
        }
    }

    impl ::std::convert::From<ForexResponse> for QuoteResponseObject {
        fn from(value: ForexResponse) -> Self {
            Self::ForexResponse(value)
        }
    }

    impl ::std::convert::From<FutureResponse> for QuoteResponseObject {
        fn from(value: FutureResponse) -> Self {
            Self::FutureResponse(value)
        }
    }

    impl ::std::convert::From<FutureOptionResponse> for QuoteResponseObject {
        fn from(value: FutureOptionResponse) -> Self {
            Self::FutureOptionResponse(value)
        }
    }

    impl ::std::convert::From<IndexResponse> for QuoteResponseObject {
        fn from(value: IndexResponse) -> Self {
            Self::IndexResponse(value)
        }
    }

    impl ::std::convert::From<MutualFundResponse> for QuoteResponseObject {
        fn from(value: MutualFundResponse) -> Self {
            Self::MutualFundResponse(value)
        }
    }

    impl ::std::convert::From<QuoteError> for QuoteResponseObject {
        fn from(value: QuoteError) -> Self {
            Self::QuoteError(value)
        }
    }

    ///NBBO - realtime, NFL - Non-fee liable quote.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "NBBO - realtime, NFL - Non-fee liable quote.",
    ///  "type": [
    ///    "string",
    ///    "null"
    ///  ],
    ///  "enum": [
    ///    "NBBO",
    ///    "NFL",
    ///    null
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct QuoteType(pub ::std::option::Option<QuoteTypeInner>);
    impl ::std::ops::Deref for QuoteType {
        type Target = ::std::option::Option<QuoteTypeInner>;
        fn deref(&self) -> &::std::option::Option<QuoteTypeInner> {
            &self.0
        }
    }

    impl ::std::convert::From<QuoteType> for ::std::option::Option<QuoteTypeInner> {
        fn from(value: QuoteType) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<::std::option::Option<QuoteTypeInner>> for QuoteType {
        fn from(value: ::std::option::Option<QuoteTypeInner>) -> Self {
            Self(value)
        }
    }

    ///NBBO - realtime, NFL - Non-fee liable quote.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "NBBO - realtime, NFL - Non-fee liable quote.",
    ///  "type": "string",
    ///  "enum": [
    ///    "NBBO",
    ///    "NFL"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum QuoteTypeInner {
        #[serde(rename = "NBBO")]
        Nbbo,
        #[serde(rename = "NFL")]
        Nfl,
    }

    impl ::std::fmt::Display for QuoteTypeInner {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Nbbo => f.write_str("NBBO"),
                Self::Nfl => f.write_str("NFL"),
            }
        }
    }

    impl ::std::str::FromStr for QuoteTypeInner {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NBBO" => Ok(Self::Nbbo),
                "NFL" => Ok(Self::Nfl),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for QuoteTypeInner {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for QuoteTypeInner {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for QuoteTypeInner {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Reference data of Equity security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Reference data of Equity security",
    ///  "type": "object",
    ///  "properties": {
    ///    "cusip": {
    ///      "description": "CUSIP of Instrument",
    ///      "examples": [
    ///        "A23456789"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "description": "Description of Instrument",
    ///      "examples": [
    ///        "Apple Inc. - Common Stock"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "exchange": {
    ///      "description": "Exchange Code",
    ///      "examples": [
    ///        "q"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "exchangeName": {
    ///      "description": "Exchange Name",
    ///      "type": "string"
    ///    },
    ///    "fsiDesc": {
    ///      "description": "FSI Desc",
    ///      "type": "string",
    ///      "maxLength": 50
    ///    },
    ///    "htbQuantity": {
    ///      "description": "Hard to borrow quantity.",
    ///      "examples": [
    ///        100
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "htbRate": {
    ///      "description": "Hard to borrow rate.",
    ///      "examples": [
    ///        4.5
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "isHardToBorrow": {
    ///      "description": "is Hard to borrow security.",
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "isShortable": {
    ///      "description": "is shortable security.",
    ///      "examples": [
    ///        false
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "otcMarketTier": {
    ///      "description": "OTC Market Tier",
    ///      "type": "string",
    ///      "maxLength": 10
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReferenceEquity {
        ///CUSIP of Instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        ///Description of Instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///Exchange Code
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub exchange: ::std::option::Option<::std::string::String>,
        ///Exchange Name
        #[serde(
            rename = "exchangeName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub exchange_name: ::std::option::Option<::std::string::String>,
        ///FSI Desc
        #[serde(
            rename = "fsiDesc",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub fsi_desc: ::std::option::Option<ReferenceEquityFsiDesc>,
        ///Hard to borrow quantity.
        #[serde(
            rename = "htbQuantity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub htb_quantity: ::std::option::Option<i32>,
        #[serde(
            rename = "htbRate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub htb_rate: ::std::option::Option<f64>,
        ///is Hard to borrow security.
        #[serde(
            rename = "isHardToBorrow",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_hard_to_borrow: ::std::option::Option<bool>,
        ///is shortable security.
        #[serde(
            rename = "isShortable",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_shortable: ::std::option::Option<bool>,
        ///OTC Market Tier
        #[serde(
            rename = "otcMarketTier",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub otc_market_tier: ::std::option::Option<ReferenceEquityOtcMarketTier>,
    }

    impl ::std::default::Default for ReferenceEquity {
        fn default() -> Self {
            Self {
                cusip: Default::default(),
                description: Default::default(),
                exchange: Default::default(),
                exchange_name: Default::default(),
                fsi_desc: Default::default(),
                htb_quantity: Default::default(),
                htb_rate: Default::default(),
                is_hard_to_borrow: Default::default(),
                is_shortable: Default::default(),
                otc_market_tier: Default::default(),
            }
        }
    }

    impl ReferenceEquity {
        pub fn builder() -> builder::ReferenceEquity {
            Default::default()
        }
    }

    ///FSI Desc
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "FSI Desc",
    ///  "type": "string",
    ///  "maxLength": 50
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ReferenceEquityFsiDesc(::std::string::String);
    impl ::std::ops::Deref for ReferenceEquityFsiDesc {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ReferenceEquityFsiDesc> for ::std::string::String {
        fn from(value: ReferenceEquityFsiDesc) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ReferenceEquityFsiDesc {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 50usize {
                return Err("longer than 50 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ReferenceEquityFsiDesc {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ReferenceEquityFsiDesc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ReferenceEquityFsiDesc {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ReferenceEquityFsiDesc {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///OTC Market Tier
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "OTC Market Tier",
    ///  "type": "string",
    ///  "maxLength": 10
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[serde(transparent)]
    pub struct ReferenceEquityOtcMarketTier(::std::string::String);
    impl ::std::ops::Deref for ReferenceEquityOtcMarketTier {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<ReferenceEquityOtcMarketTier> for ::std::string::String {
        fn from(value: ReferenceEquityOtcMarketTier) -> Self {
            value.0
        }
    }

    impl ::std::str::FromStr for ReferenceEquityOtcMarketTier {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            if value.chars().count() > 10usize {
                return Err("longer than 10 characters".into());
            }
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::convert::TryFrom<&str> for ReferenceEquityOtcMarketTier {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ReferenceEquityOtcMarketTier {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ReferenceEquityOtcMarketTier {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl<'de> ::serde::Deserialize<'de> for ReferenceEquityOtcMarketTier {
        fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            ::std::string::String::deserialize(deserializer)?
                .parse()
                .map_err(|e: self::error::ConversionError| {
                    <D::Error as ::serde::de::Error>::custom(e.to_string())
                })
        }
    }

    ///Reference data of Forex security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Reference data of Forex security",
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "description": "Description of Instrument",
    ///      "examples": [
    ///        "Euro/USDollar Spot"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "exchange": {
    ///      "description": "Exchange Code",
    ///      "examples": [
    ///        "q"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "exchangeName": {
    ///      "description": "Exchange Name",
    ///      "type": "string"
    ///    },
    ///    "isTradable": {
    ///      "description": "is FOREX tradable",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "marketMaker": {
    ///      "description": "Market marker",
    ///      "type": "string"
    ///    },
    ///    "product": {
    ///      "description": "Product name",
    ///      "type": "string"
    ///    },
    ///    "tradingHours": {
    ///      "description": "Trading hours",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReferenceForex {
        ///Description of Instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///Exchange Code
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub exchange: ::std::option::Option<::std::string::String>,
        ///Exchange Name
        #[serde(
            rename = "exchangeName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub exchange_name: ::std::option::Option<::std::string::String>,
        ///is FOREX tradable
        #[serde(
            rename = "isTradable",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_tradable: ::std::option::Option<bool>,
        ///Market marker
        #[serde(
            rename = "marketMaker",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub market_maker: ::std::option::Option<::std::string::String>,
        ///Product name
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub product: ::std::option::Option<::std::string::String>,
        ///Trading hours
        #[serde(
            rename = "tradingHours",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trading_hours: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for ReferenceForex {
        fn default() -> Self {
            Self {
                description: Default::default(),
                exchange: Default::default(),
                exchange_name: Default::default(),
                is_tradable: Default::default(),
                market_maker: Default::default(),
                product: Default::default(),
                trading_hours: Default::default(),
            }
        }
    }

    impl ReferenceForex {
        pub fn builder() -> builder::ReferenceForex {
            Default::default()
        }
    }

    ///Reference data of Future security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Reference data of Future security",
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "description": "Description of Instrument",
    ///      "examples": [
    ///        "E-mini S&P 500 Index Futures,Jun-2021,ETH"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "exchange": {
    ///      "description": "Exchange Code",
    ///      "examples": [
    ///        "q"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "exchangeName": {
    ///      "description": "Exchange Name",
    ///      "type": "string"
    ///    },
    ///    "futureActiveSymbol": {
    ///      "description": "Active symbol",
    ///      "examples": [
    ///        "/ESM21"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "futureExpirationDate": {
    ///      "description": "Future expiration date in milliseconds since
    /// epoch",
    ///      "examples": [
    ///        1623988800000
    ///      ],
    ///      "type": "number",
    ///      "format": "int64"
    ///    },
    ///    "futureIsActive": {
    ///      "description": "Future is active",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "futureMultiplier": {
    ///      "description": "Future multiplier",
    ///      "examples": [
    ///        50
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "futurePriceFormat": {
    ///      "description": "Price format",
    ///      "examples": [
    ///        "D,D"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "futureSettlementPrice": {
    ///      "description": "Future Settlement Price",
    ///      "examples": [
    ///        4123
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "futureTradingHours": {
    ///      "description": "Trading Hours",
    ///      "examples": [
    ///        "GLBX(de=1640;0=-1700151515301600;
    /// 1=r-17001515r15301600d-15551640;7=d-16401555)"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "product": {
    ///      "description": "Futures product symbol",
    ///      "examples": [
    ///        "/ES"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReferenceFuture {
        ///Description of Instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///Exchange Code
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub exchange: ::std::option::Option<::std::string::String>,
        ///Exchange Name
        #[serde(
            rename = "exchangeName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub exchange_name: ::std::option::Option<::std::string::String>,
        ///Active symbol
        #[serde(
            rename = "futureActiveSymbol",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub future_active_symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "futureExpirationDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub future_expiration_date: ::std::option::Option<f64>,
        ///Future is active
        #[serde(
            rename = "futureIsActive",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub future_is_active: ::std::option::Option<bool>,
        #[serde(
            rename = "futureMultiplier",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub future_multiplier: ::std::option::Option<f64>,
        ///Price format
        #[serde(
            rename = "futurePriceFormat",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub future_price_format: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "futureSettlementPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub future_settlement_price: ::std::option::Option<f64>,
        ///Trading Hours
        #[serde(
            rename = "futureTradingHours",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub future_trading_hours: ::std::option::Option<::std::string::String>,
        ///Futures product symbol
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub product: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for ReferenceFuture {
        fn default() -> Self {
            Self {
                description: Default::default(),
                exchange: Default::default(),
                exchange_name: Default::default(),
                future_active_symbol: Default::default(),
                future_expiration_date: Default::default(),
                future_is_active: Default::default(),
                future_multiplier: Default::default(),
                future_price_format: Default::default(),
                future_settlement_price: Default::default(),
                future_trading_hours: Default::default(),
                product: Default::default(),
            }
        }
    }

    impl ReferenceFuture {
        pub fn builder() -> builder::ReferenceFuture {
            Default::default()
        }
    }

    ///Reference data of Future Option security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Reference data of Future Option security",
    ///  "type": "object",
    ///  "properties": {
    ///    "contractType": {
    ///      "$ref": "#/components/schemas/ContractType"
    ///    },
    ///    "description": {
    ///      "description": "Description of Instrument",
    ///      "examples": [
    ///        "AMZN Aug 20 2021 2300 Put"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "exchange": {
    ///      "description": "Exchange Code",
    ///      "examples": [
    ///        "q"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "exchangeName": {
    ///      "description": "Exchange Name",
    ///      "type": "string"
    ///    },
    ///    "expirationDate": {
    ///      "description": "date of expiration in long",
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "expirationStyle": {
    ///      "description": "Style of expiration",
    ///      "type": "string"
    ///    },
    ///    "multiplier": {
    ///      "description": "Option multiplier",
    ///      "examples": [
    ///        100
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "strikePrice": {
    ///      "description": "Strike Price",
    ///      "examples": [
    ///        2300
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "underlying": {
    ///      "description": "A company, index or fund name",
    ///      "examples": [
    ///        "AMZN Aug 20 2021 2300 Put"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReferenceFutureOption {
        #[serde(
            rename = "contractType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub contract_type: ::std::option::Option<ContractType>,
        ///Description of Instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///Exchange Code
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub exchange: ::std::option::Option<::std::string::String>,
        ///Exchange Name
        #[serde(
            rename = "exchangeName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub exchange_name: ::std::option::Option<::std::string::String>,
        ///date of expiration in long
        #[serde(
            rename = "expirationDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expiration_date: ::std::option::Option<i64>,
        ///Style of expiration
        #[serde(
            rename = "expirationStyle",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expiration_style: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub multiplier: ::std::option::Option<f64>,
        #[serde(
            rename = "strikePrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub strike_price: ::std::option::Option<f64>,
        ///A company, index or fund name
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub underlying: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for ReferenceFutureOption {
        fn default() -> Self {
            Self {
                contract_type: Default::default(),
                description: Default::default(),
                exchange: Default::default(),
                exchange_name: Default::default(),
                expiration_date: Default::default(),
                expiration_style: Default::default(),
                multiplier: Default::default(),
                strike_price: Default::default(),
                underlying: Default::default(),
            }
        }
    }

    impl ReferenceFutureOption {
        pub fn builder() -> builder::ReferenceFutureOption {
            Default::default()
        }
    }

    ///Reference data of Index security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Reference data of Index security",
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "description": "Description of Instrument",
    ///      "examples": [
    ///        "DOW JONES 30 INDUSTRIALS"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "exchange": {
    ///      "description": "Exchange Code",
    ///      "examples": [
    ///        "q"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "exchangeName": {
    ///      "description": "Exchange Name",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReferenceIndex {
        ///Description of Instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///Exchange Code
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub exchange: ::std::option::Option<::std::string::String>,
        ///Exchange Name
        #[serde(
            rename = "exchangeName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub exchange_name: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for ReferenceIndex {
        fn default() -> Self {
            Self {
                description: Default::default(),
                exchange: Default::default(),
                exchange_name: Default::default(),
            }
        }
    }

    impl ReferenceIndex {
        pub fn builder() -> builder::ReferenceIndex {
            Default::default()
        }
    }

    ///Reference data of MutualFund security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Reference data of MutualFund security",
    ///  "type": "object",
    ///  "properties": {
    ///    "cusip": {
    ///      "description": "CUSIP of Instrument",
    ///      "examples": [
    ///        "A23456789"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "description": "Description of Instrument",
    ///      "examples": [
    ///        "Apple Inc. - Common Stock"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "exchange": {
    ///      "description": "Exchange Code",
    ///      "default": "m",
    ///      "type": "string"
    ///    },
    ///    "exchangeName": {
    ///      "description": "Exchange Name",
    ///      "default": "MUTUAL_FUND",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReferenceMutualFund {
        ///CUSIP of Instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        ///Description of Instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///Exchange Code
        #[serde(default = "defaults::reference_mutual_fund_exchange")]
        pub exchange: ::std::string::String,
        ///Exchange Name
        #[serde(
            rename = "exchangeName",
            default = "defaults::reference_mutual_fund_exchange_name"
        )]
        pub exchange_name: ::std::string::String,
    }

    impl ::std::default::Default for ReferenceMutualFund {
        fn default() -> Self {
            Self {
                cusip: Default::default(),
                description: Default::default(),
                exchange: defaults::reference_mutual_fund_exchange(),
                exchange_name: defaults::reference_mutual_fund_exchange_name(),
            }
        }
    }

    impl ReferenceMutualFund {
        pub fn builder() -> builder::ReferenceMutualFund {
            Default::default()
        }
    }

    ///Reference data of Option security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Reference data of Option security",
    ///  "type": "object",
    ///  "properties": {
    ///    "contractType": {
    ///      "$ref": "#/components/schemas/ContractType"
    ///    },
    ///    "cusip": {
    ///      "description": "CUSIP of Instrument",
    ///      "examples": [
    ///        "0AMZN.TK12300000"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "daysToExpiration": {
    ///      "description": "Days to Expiration",
    ///      "examples": [
    ///        94
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "deliverables": {
    ///      "description": "Unit of trade",
    ///      "examples": [
    ///        "$6024.37 cash in lieu of shares, 212 shares of AZN"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "description": "Description of Instrument",
    ///      "examples": [
    ///        "AMZN Aug 20 2021 2300 Put"
    ///      ],
    ///      "type": "string"
    ///    },
    ///    "exchange": {
    ///      "description": "Exchange Code",
    ///      "default": "o",
    ///      "type": "string"
    ///    },
    ///    "exchangeName": {
    ///      "description": "Exchange Name",
    ///      "type": "string"
    ///    },
    ///    "exerciseType": {
    ///      "$ref": "#/components/schemas/ExerciseType"
    ///    },
    ///    "expirationDay": {
    ///      "description": "Expiration Day",
    ///      "examples": [
    ///        20
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32",
    ///      "maximum": 31.0,
    ///      "minimum": 1.0
    ///    },
    ///    "expirationMonth": {
    ///      "description": "Expiration Month",
    ///      "examples": [
    ///        8
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32",
    ///      "maximum": 12.0,
    ///      "minimum": 1.0
    ///    },
    ///    "expirationType": {
    ///      "$ref": "#/components/schemas/ExpirationType"
    ///    },
    ///    "expirationYear": {
    ///      "description": "Expiration Year",
    ///      "examples": [
    ///        2021
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "isPennyPilot": {
    ///      "description": "Is this contract part of the Penny Pilot program",
    ///      "examples": [
    ///        true
    ///      ],
    ///      "type": "boolean"
    ///    },
    ///    "lastTradingDay": {
    ///      "description": "milliseconds since epoch",
    ///      "examples": [
    ///        1629504000000
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "multiplier": {
    ///      "description": "Option multiplier",
    ///      "examples": [
    ///        100
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "settlementType": {
    ///      "$ref": "#/components/schemas/SettlementType"
    ///    },
    ///    "strikePrice": {
    ///      "description": "Strike Price",
    ///      "examples": [
    ///        2300
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "underlying": {
    ///      "description": "A company, index or fund name",
    ///      "examples": [
    ///        "AMZN Aug 20 2021 2300 Put"
    ///      ],
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ReferenceOption {
        #[serde(
            rename = "contractType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub contract_type: ::std::option::Option<ContractType>,
        ///CUSIP of Instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        ///Days to Expiration
        #[serde(
            rename = "daysToExpiration",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub days_to_expiration: ::std::option::Option<i32>,
        ///Unit of trade
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deliverables: ::std::option::Option<::std::string::String>,
        ///Description of Instrument
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        ///Exchange Code
        #[serde(default = "defaults::reference_option_exchange")]
        pub exchange: ::std::string::String,
        ///Exchange Name
        #[serde(
            rename = "exchangeName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub exchange_name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "exerciseType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub exercise_type: ::std::option::Option<ExerciseType>,
        ///Expiration Day
        #[serde(
            rename = "expirationDay",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expiration_day: ::std::option::Option<::std::num::NonZeroU32>,
        ///Expiration Month
        #[serde(
            rename = "expirationMonth",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expiration_month: ::std::option::Option<::std::num::NonZeroU32>,
        #[serde(
            rename = "expirationType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expiration_type: ::std::option::Option<ExpirationType>,
        ///Expiration Year
        #[serde(
            rename = "expirationYear",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expiration_year: ::std::option::Option<i32>,
        ///Is this contract part of the Penny Pilot program
        #[serde(
            rename = "isPennyPilot",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_penny_pilot: ::std::option::Option<bool>,
        ///milliseconds since epoch
        #[serde(
            rename = "lastTradingDay",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_trading_day: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub multiplier: ::std::option::Option<f64>,
        #[serde(
            rename = "settlementType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub settlement_type: ::std::option::Option<SettlementType>,
        #[serde(
            rename = "strikePrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub strike_price: ::std::option::Option<f64>,
        ///A company, index or fund name
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub underlying: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for ReferenceOption {
        fn default() -> Self {
            Self {
                contract_type: Default::default(),
                cusip: Default::default(),
                days_to_expiration: Default::default(),
                deliverables: Default::default(),
                description: Default::default(),
                exchange: defaults::reference_option_exchange(),
                exchange_name: Default::default(),
                exercise_type: Default::default(),
                expiration_day: Default::default(),
                expiration_month: Default::default(),
                expiration_type: Default::default(),
                expiration_year: Default::default(),
                is_penny_pilot: Default::default(),
                last_trading_day: Default::default(),
                multiplier: Default::default(),
                settlement_type: Default::default(),
                strike_price: Default::default(),
                underlying: Default::default(),
            }
        }
    }

    impl ReferenceOption {
        pub fn builder() -> builder::ReferenceOption {
            Default::default()
        }
    }

    ///Market info of security
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Market info of security",
    ///  "type": "object",
    ///  "properties": {
    ///    "regularMarketLastPrice": {
    ///      "description": "Regular market last price",
    ///      "examples": [
    ///        124.85
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "regularMarketLastSize": {
    ///      "description": "Regular market last size",
    ///      "examples": [
    ///        51771
    ///      ],
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "regularMarketNetChange": {
    ///      "description": "Regular market net change",
    ///      "examples": [
    ///        -1.42
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "regularMarketPercentChange": {
    ///      "description": "Regular market percent change",
    ///      "examples": [
    ///        -1.1246
    ///      ],
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "regularMarketTradeTime": {
    ///      "description": "Regular market trade time in milliseconds since
    /// Epoch",
    ///      "examples": [
    ///        1621368000400
    ///      ],
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct RegularMarket {
        #[serde(
            rename = "regularMarketLastPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub regular_market_last_price: ::std::option::Option<f64>,
        ///Regular market last size
        #[serde(
            rename = "regularMarketLastSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub regular_market_last_size: ::std::option::Option<i32>,
        #[serde(
            rename = "regularMarketNetChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub regular_market_net_change: ::std::option::Option<f64>,
        #[serde(
            rename = "regularMarketPercentChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub regular_market_percent_change: ::std::option::Option<f64>,
        ///Regular market trade time in milliseconds since Epoch
        #[serde(
            rename = "regularMarketTradeTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub regular_market_trade_time: ::std::option::Option<i64>,
    }

    impl ::std::default::Default for RegularMarket {
        fn default() -> Self {
            Self {
                regular_market_last_price: Default::default(),
                regular_market_last_size: Default::default(),
                regular_market_net_change: Default::default(),
                regular_market_percent_change: Default::default(),
                regular_market_trade_time: Default::default(),
            }
        }
    }

    impl RegularMarket {
        pub fn builder() -> builder::RegularMarket {
            Default::default()
        }
    }

    ///Security info of most moved with in an index
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Security info of most moved with in an index",
    ///  "type": "object",
    ///  "properties": {
    ///    "change": {
    ///      "description": "percent or value changed, by default its percent
    /// changed",
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "description": {
    ///      "description": "Name of security",
    ///      "type": "string"
    ///    },
    ///    "direction": {
    ///      "type": "string",
    ///      "enum": [
    ///        "up",
    ///        "down"
    ///      ]
    ///    },
    ///    "last": {
    ///      "description": "what was last quoted price",
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "symbol": {
    ///      "description": "schwab security symbol",
    ///      "type": "string"
    ///    },
    ///    "totalVolume": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Screener {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub change: ::std::option::Option<f64>,
        ///Name of security
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub direction: ::std::option::Option<ScreenerDirection>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub last: ::std::option::Option<f64>,
        ///schwab security symbol
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "totalVolume",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_volume: ::std::option::Option<i64>,
    }

    impl ::std::default::Default for Screener {
        fn default() -> Self {
            Self {
                change: Default::default(),
                description: Default::default(),
                direction: Default::default(),
                last: Default::default(),
                symbol: Default::default(),
                total_volume: Default::default(),
            }
        }
    }

    impl Screener {
        pub fn builder() -> builder::Screener {
            Default::default()
        }
    }

    ///`ScreenerDirection`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "up",
    ///    "down"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum ScreenerDirection {
        #[serde(rename = "up")]
        Up,
        #[serde(rename = "down")]
        Down,
    }

    impl ::std::fmt::Display for ScreenerDirection {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Up => f.write_str("up"),
                Self::Down => f.write_str("down"),
            }
        }
    }

    impl ::std::str::FromStr for ScreenerDirection {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "up" => Ok(Self::Up),
                "down" => Ok(Self::Down),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ScreenerDirection {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ScreenerDirection {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ScreenerDirection {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///option contract settlement type AM or PM
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "option contract settlement type AM or PM",
    ///  "type": "string",
    ///  "enum": [
    ///    "A",
    ///    "P"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum SettlementType {
        A,
        P,
    }

    impl ::std::fmt::Display for SettlementType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::A => f.write_str("A"),
                Self::P => f.write_str("P"),
            }
        }
    }

    impl ::std::str::FromStr for SettlementType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "A" => Ok(Self::A),
                "P" => Ok(Self::P),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for SettlementType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for SettlementType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SettlementType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`Underlying`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "ask": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "askSize": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "bid": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "bidSize": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "change": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "close": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "delayed": {
    ///      "type": "boolean"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "exchangeName": {
    ///      "type": "string",
    ///      "enum": [
    ///        "IND",
    ///        "ASE",
    ///        "NYS",
    ///        "NAS",
    ///        "NAP",
    ///        "PAC",
    ///        "OPR",
    ///        "BATS"
    ///      ]
    ///    },
    ///    "fiftyTwoWeekHigh": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "fiftyTwoWeekLow": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "highPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "last": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "lowPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "mark": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "markChange": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "markPercentChange": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "openPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "percentChange": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "quoteTime": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    },
    ///    "totalVolume": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "tradeTime": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Underlying {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub ask: ::std::option::Option<f64>,
        #[serde(
            rename = "askSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_size: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub bid: ::std::option::Option<f64>,
        #[serde(
            rename = "bidSize",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_size: ::std::option::Option<i32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub change: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub close: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub delayed: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        // PATCH: the underlying quote returns full exchange names (e.g.
        // "NASDAQ"), not the short codes (NAS/NYS/...) the generated enum
        // accepts, which made the whole chain fail to deserialize. Use a free
        // string to tolerate any exchange name.
        #[serde(
            rename = "exchangeName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub exchange_name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "fiftyTwoWeekHigh",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub fifty_two_week_high: ::std::option::Option<f64>,
        #[serde(
            rename = "fiftyTwoWeekLow",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub fifty_two_week_low: ::std::option::Option<f64>,
        #[serde(
            rename = "highPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub high_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub last: ::std::option::Option<f64>,
        #[serde(
            rename = "lowPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub low_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub mark: ::std::option::Option<f64>,
        #[serde(
            rename = "markChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mark_change: ::std::option::Option<f64>,
        #[serde(
            rename = "markPercentChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mark_percent_change: ::std::option::Option<f64>,
        #[serde(
            rename = "openPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub open_price: ::std::option::Option<f64>,
        #[serde(
            rename = "percentChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub percent_change: ::std::option::Option<f64>,
        #[serde(
            rename = "quoteTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub quote_time: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "totalVolume",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_volume: ::std::option::Option<i64>,
        #[serde(
            rename = "tradeTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trade_time: ::std::option::Option<i64>,
    }

    impl ::std::default::Default for Underlying {
        fn default() -> Self {
            Self {
                ask: Default::default(),
                ask_size: Default::default(),
                bid: Default::default(),
                bid_size: Default::default(),
                change: Default::default(),
                close: Default::default(),
                delayed: Default::default(),
                description: Default::default(),
                exchange_name: Default::default(),
                fifty_two_week_high: Default::default(),
                fifty_two_week_low: Default::default(),
                high_price: Default::default(),
                last: Default::default(),
                low_price: Default::default(),
                mark: Default::default(),
                mark_change: Default::default(),
                mark_percent_change: Default::default(),
                open_price: Default::default(),
                percent_change: Default::default(),
                quote_time: Default::default(),
                symbol: Default::default(),
                total_volume: Default::default(),
                trade_time: Default::default(),
            }
        }
    }

    impl Underlying {
        pub fn builder() -> builder::Underlying {
            Default::default()
        }
    }

    ///`UnderlyingExchangeName`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "IND",
    ///    "ASE",
    ///    "NYS",
    ///    "NAS",
    ///    "NAP",
    ///    "PAC",
    ///    "OPR",
    ///    "BATS"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
    )]
    pub enum UnderlyingExchangeName {
        #[serde(rename = "IND")]
        Ind,
        #[serde(rename = "ASE")]
        Ase,
        #[serde(rename = "NYS")]
        Nys,
        #[serde(rename = "NAS")]
        Nas,
        #[serde(rename = "NAP")]
        Nap,
        #[serde(rename = "PAC")]
        Pac,
        #[serde(rename = "OPR")]
        Opr,
        #[serde(rename = "BATS")]
        Bats,
    }

    impl ::std::fmt::Display for UnderlyingExchangeName {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Ind => f.write_str("IND"),
                Self::Ase => f.write_str("ASE"),
                Self::Nys => f.write_str("NYS"),
                Self::Nas => f.write_str("NAS"),
                Self::Nap => f.write_str("NAP"),
                Self::Pac => f.write_str("PAC"),
                Self::Opr => f.write_str("OPR"),
                Self::Bats => f.write_str("BATS"),
            }
        }
    }

    impl ::std::str::FromStr for UnderlyingExchangeName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "IND" => Ok(Self::Ind),
                "ASE" => Ok(Self::Ase),
                "NYS" => Ok(Self::Nys),
                "NAS" => Ok(Self::Nas),
                "NAP" => Ok(Self::Nap),
                "PAC" => Ok(Self::Pac),
                "OPR" => Ok(Self::Opr),
                "BATS" => Ok(Self::Bats),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for UnderlyingExchangeName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for UnderlyingExchangeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UnderlyingExchangeName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct Bond {
            asset_type: ::std::result::Result<
                ::std::option::Option<super::BondAssetType>,
                ::std::string::String,
            >,
            bond_factor: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            bond_multiplier: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            bond_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            cusip: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                ::std::option::Option<super::BondType>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for Bond {
            fn default() -> Self {
                Self {
                    asset_type: Ok(Default::default()),
                    bond_factor: Ok(Default::default()),
                    bond_multiplier: Ok(Default::default()),
                    bond_price: Ok(Default::default()),
                    cusip: Ok(Default::default()),
                    description: Ok(Default::default()),
                    exchange: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl Bond {
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::BondAssetType>>,
                T::Error: ::std::fmt::Display,
            {
                self.asset_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for asset_type: {e}"));
                self
            }
            pub fn bond_factor<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.bond_factor = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bond_factor: {e}"));
                self
            }
            pub fn bond_multiplier<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.bond_multiplier = value.try_into().map_err(|e| {
                    format!("error converting supplied value for bond_multiplier: {e}")
                });
                self
            }
            pub fn bond_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.bond_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bond_price: {e}"));
                self
            }
            pub fn cusip<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.cusip = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cusip: {e}"));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {e}"));
                self
            }
            pub fn exchange<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange: {e}"));
                self
            }
            pub fn symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for symbol: {e}"));
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::BondType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Bond> for super::Bond {
            type Error = super::error::ConversionError;
            fn try_from(value: Bond) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_type: value.asset_type?,
                    bond_factor: value.bond_factor?,
                    bond_multiplier: value.bond_multiplier?,
                    bond_price: value.bond_price?,
                    cusip: value.cusip?,
                    description: value.description?,
                    exchange: value.exchange?,
                    symbol: value.symbol?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::Bond> for Bond {
            fn from(value: super::Bond) -> Self {
                Self {
                    asset_type: Ok(value.asset_type),
                    bond_factor: Ok(value.bond_factor),
                    bond_multiplier: Ok(value.bond_multiplier),
                    bond_price: Ok(value.bond_price),
                    cusip: Ok(value.cusip),
                    description: Ok(value.description),
                    exchange: Ok(value.exchange),
                    symbol: Ok(value.symbol),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Candle {
            close: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            datetime: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            datetime_iso8601: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            high: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            low: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            open: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            volume: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        }

        impl ::std::default::Default for Candle {
            fn default() -> Self {
                Self {
                    close: Ok(Default::default()),
                    datetime: Ok(Default::default()),
                    datetime_iso8601: Ok(Default::default()),
                    high: Ok(Default::default()),
                    low: Ok(Default::default()),
                    open: Ok(Default::default()),
                    volume: Ok(Default::default()),
                }
            }
        }

        impl Candle {
            pub fn close<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.close = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for close: {e}"));
                self
            }
            pub fn datetime<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.datetime = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for datetime: {e}"));
                self
            }
            pub fn datetime_iso8601<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.datetime_iso8601 = value.try_into().map_err(|e| {
                    format!("error converting supplied value for datetime_iso8601: {e}")
                });
                self
            }
            pub fn high<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.high = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for high: {e}"));
                self
            }
            pub fn low<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.low = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for low: {e}"));
                self
            }
            pub fn open<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.open = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for open: {e}"));
                self
            }
            pub fn volume<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.volume = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for volume: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Candle> for super::Candle {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Candle,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    close: value.close?,
                    datetime: value.datetime?,
                    datetime_iso8601: value.datetime_iso8601?,
                    high: value.high?,
                    low: value.low?,
                    open: value.open?,
                    volume: value.volume?,
                })
            }
        }

        impl ::std::convert::From<super::Candle> for Candle {
            fn from(value: super::Candle) -> Self {
                Self {
                    close: Ok(value.close),
                    datetime: Ok(value.datetime),
                    datetime_iso8601: Ok(value.datetime_iso8601),
                    high: Ok(value.high),
                    low: Ok(value.low),
                    open: Ok(value.open),
                    volume: Ok(value.volume),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CandleList {
            candles: ::std::result::Result<::std::vec::Vec<super::Candle>, ::std::string::String>,
            empty: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            previous_close:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            previous_close_date:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            previous_close_date_iso8601: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for CandleList {
            fn default() -> Self {
                Self {
                    candles: Ok(Default::default()),
                    empty: Ok(Default::default()),
                    previous_close: Ok(Default::default()),
                    previous_close_date: Ok(Default::default()),
                    previous_close_date_iso8601: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                }
            }
        }

        impl CandleList {
            pub fn candles<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::Candle>>,
                T::Error: ::std::fmt::Display,
            {
                self.candles = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for candles: {e}"));
                self
            }
            pub fn empty<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.empty = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for empty: {e}"));
                self
            }
            pub fn previous_close<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.previous_close = value.try_into().map_err(|e| {
                    format!("error converting supplied value for previous_close: {e}")
                });
                self
            }
            pub fn previous_close_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.previous_close_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for previous_close_date: {e}")
                });
                self
            }
            pub fn previous_close_date_iso8601<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.previous_close_date_iso8601 = value.try_into().map_err(|e| {
                    format!("error converting supplied value for previous_close_date_iso8601: {e}")
                });
                self
            }
            pub fn symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for symbol: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<CandleList> for super::CandleList {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CandleList,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    candles: value.candles?,
                    empty: value.empty?,
                    previous_close: value.previous_close?,
                    previous_close_date: value.previous_close_date?,
                    previous_close_date_iso8601: value.previous_close_date_iso8601?,
                    symbol: value.symbol?,
                })
            }
        }

        impl ::std::convert::From<super::CandleList> for CandleList {
            fn from(value: super::CandleList) -> Self {
                Self {
                    candles: Ok(value.candles),
                    empty: Ok(value.empty),
                    previous_close: Ok(value.previous_close),
                    previous_close_date: Ok(value.previous_close_date),
                    previous_close_date_iso8601: Ok(value.previous_close_date_iso8601),
                    symbol: Ok(value.symbol),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct EquityResponse {
            asset_main_type: ::std::result::Result<
                ::std::option::Option<super::AssetMainType>,
                ::std::string::String,
            >,
            asset_sub_type: ::std::result::Result<
                ::std::option::Option<super::EquityAssetSubType>,
                ::std::string::String,
            >,
            extended: ::std::result::Result<
                ::std::option::Option<super::ExtendedMarket>,
                ::std::string::String,
            >,
            fundamental: ::std::result::Result<
                ::std::option::Option<super::Fundamental>,
                ::std::string::String,
            >,
            quote: ::std::result::Result<
                ::std::option::Option<super::QuoteEquity>,
                ::std::string::String,
            >,
            quote_type: ::std::result::Result<
                ::std::option::Option<super::QuoteType>,
                ::std::string::String,
            >,
            realtime: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            reference: ::std::result::Result<
                ::std::option::Option<super::ReferenceEquity>,
                ::std::string::String,
            >,
            regular: ::std::result::Result<
                ::std::option::Option<super::RegularMarket>,
                ::std::string::String,
            >,
            ssid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for EquityResponse {
            fn default() -> Self {
                Self {
                    asset_main_type: Ok(Default::default()),
                    asset_sub_type: Ok(Default::default()),
                    extended: Ok(Default::default()),
                    fundamental: Ok(Default::default()),
                    quote: Ok(Default::default()),
                    quote_type: Ok(Default::default()),
                    realtime: Ok(Default::default()),
                    reference: Ok(Default::default()),
                    regular: Ok(Default::default()),
                    ssid: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                }
            }
        }

        impl EquityResponse {
            pub fn asset_main_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::AssetMainType>>,
                T::Error: ::std::fmt::Display,
            {
                self.asset_main_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for asset_main_type: {e}")
                });
                self
            }
            pub fn asset_sub_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::EquityAssetSubType>>,
                T::Error: ::std::fmt::Display,
            {
                self.asset_sub_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for asset_sub_type: {e}")
                });
                self
            }
            pub fn extended<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ExtendedMarket>>,
                T::Error: ::std::fmt::Display,
            {
                self.extended = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for extended: {e}"));
                self
            }
            pub fn fundamental<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Fundamental>>,
                T::Error: ::std::fmt::Display,
            {
                self.fundamental = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fundamental: {e}"));
                self
            }
            pub fn quote<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::QuoteEquity>>,
                T::Error: ::std::fmt::Display,
            {
                self.quote = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quote: {e}"));
                self
            }
            pub fn quote_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::QuoteType>>,
                T::Error: ::std::fmt::Display,
            {
                self.quote_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quote_type: {e}"));
                self
            }
            pub fn realtime<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.realtime = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for realtime: {e}"));
                self
            }
            pub fn reference<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ReferenceEquity>>,
                T::Error: ::std::fmt::Display,
            {
                self.reference = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for reference: {e}"));
                self
            }
            pub fn regular<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::RegularMarket>>,
                T::Error: ::std::fmt::Display,
            {
                self.regular = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for regular: {e}"));
                self
            }
            pub fn ssid<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ssid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ssid: {e}"));
                self
            }
            pub fn symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for symbol: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<EquityResponse> for super::EquityResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: EquityResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_main_type: value.asset_main_type?,
                    asset_sub_type: value.asset_sub_type?,
                    extended: value.extended?,
                    fundamental: value.fundamental?,
                    quote: value.quote?,
                    quote_type: value.quote_type?,
                    realtime: value.realtime?,
                    reference: value.reference?,
                    regular: value.regular?,
                    ssid: value.ssid?,
                    symbol: value.symbol?,
                })
            }
        }

        impl ::std::convert::From<super::EquityResponse> for EquityResponse {
            fn from(value: super::EquityResponse) -> Self {
                Self {
                    asset_main_type: Ok(value.asset_main_type),
                    asset_sub_type: Ok(value.asset_sub_type),
                    extended: Ok(value.extended),
                    fundamental: Ok(value.fundamental),
                    quote: Ok(value.quote),
                    quote_type: Ok(value.quote_type),
                    realtime: Ok(value.realtime),
                    reference: Ok(value.reference),
                    regular: Ok(value.regular),
                    ssid: Ok(value.ssid),
                    symbol: Ok(value.symbol),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Error {
            detail: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<::std::option::Option<::uuid::Uuid>, ::std::string::String>,
            source: ::std::result::Result<
                ::std::option::Option<super::ErrorSource>,
                ::std::string::String,
            >,
            status: ::std::result::Result<
                ::std::option::Option<super::ErrorStatus>,
                ::std::string::String,
            >,
            title: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for Error {
            fn default() -> Self {
                Self {
                    detail: Ok(Default::default()),
                    id: Ok(Default::default()),
                    source: Ok(Default::default()),
                    status: Ok(Default::default()),
                    title: Ok(Default::default()),
                }
            }
        }

        impl Error {
            pub fn detail<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.detail = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for detail: {e}"));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::uuid::Uuid>>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {e}"));
                self
            }
            pub fn source<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ErrorSource>>,
                T::Error: ::std::fmt::Display,
            {
                self.source = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for source: {e}"));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ErrorStatus>>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {e}"));
                self
            }
            pub fn title<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.title = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for title: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Error> for super::Error {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Error,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    detail: value.detail?,
                    id: value.id?,
                    source: value.source?,
                    status: value.status?,
                    title: value.title?,
                })
            }
        }

        impl ::std::convert::From<super::Error> for Error {
            fn from(value: super::Error) -> Self {
                Self {
                    detail: Ok(value.detail),
                    id: Ok(value.id),
                    source: Ok(value.source),
                    status: Ok(value.status),
                    title: Ok(value.title),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ErrorResponse {
            errors: ::std::result::Result<::std::vec::Vec<super::Error>, ::std::string::String>,
        }

        impl ::std::default::Default for ErrorResponse {
            fn default() -> Self {
                Self {
                    errors: Ok(Default::default()),
                }
            }
        }

        impl ErrorResponse {
            pub fn errors<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::Error>>,
                T::Error: ::std::fmt::Display,
            {
                self.errors = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for errors: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ErrorResponse> for super::ErrorResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ErrorResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    errors: value.errors?,
                })
            }
        }

        impl ::std::convert::From<super::ErrorResponse> for ErrorResponse {
            fn from(value: super::ErrorResponse) -> Self {
                Self {
                    errors: Ok(value.errors),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ErrorSource {
            header: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            parameter: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            pointer: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for ErrorSource {
            fn default() -> Self {
                Self {
                    header: Ok(Default::default()),
                    parameter: Ok(Default::default()),
                    pointer: Ok(Default::default()),
                }
            }
        }

        impl ErrorSource {
            pub fn header<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.header = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for header: {e}"));
                self
            }
            pub fn parameter<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.parameter = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for parameter: {e}"));
                self
            }
            pub fn pointer<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.pointer = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pointer: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ErrorSource> for super::ErrorSource {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ErrorSource,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    header: value.header?,
                    parameter: value.parameter?,
                    pointer: value.pointer?,
                })
            }
        }

        impl ::std::convert::From<super::ErrorSource> for ErrorSource {
            fn from(value: super::ErrorSource) -> Self {
                Self {
                    header: Ok(value.header),
                    parameter: Ok(value.parameter),
                    pointer: Ok(value.pointer),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Expiration {
            days_to_expiration:
                ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            expiration: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            expiration_type: ::std::result::Result<
                ::std::option::Option<super::ExpirationType>,
                ::std::string::String,
            >,
            option_roots: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            settlement_type: ::std::result::Result<
                ::std::option::Option<super::SettlementType>,
                ::std::string::String,
            >,
            standard: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        }

        impl ::std::default::Default for Expiration {
            fn default() -> Self {
                Self {
                    days_to_expiration: Ok(Default::default()),
                    expiration: Ok(Default::default()),
                    expiration_type: Ok(Default::default()),
                    option_roots: Ok(Default::default()),
                    settlement_type: Ok(Default::default()),
                    standard: Ok(Default::default()),
                }
            }
        }

        impl Expiration {
            pub fn days_to_expiration<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.days_to_expiration = value.try_into().map_err(|e| {
                    format!("error converting supplied value for days_to_expiration: {e}")
                });
                self
            }
            pub fn expiration<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.expiration = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expiration: {e}"));
                self
            }
            pub fn expiration_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ExpirationType>>,
                T::Error: ::std::fmt::Display,
            {
                self.expiration_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for expiration_type: {e}")
                });
                self
            }
            pub fn option_roots<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.option_roots = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for option_roots: {e}"));
                self
            }
            pub fn settlement_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::SettlementType>>,
                T::Error: ::std::fmt::Display,
            {
                self.settlement_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for settlement_type: {e}")
                });
                self
            }
            pub fn standard<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.standard = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for standard: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Expiration> for super::Expiration {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Expiration,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    days_to_expiration: value.days_to_expiration?,
                    expiration: value.expiration?,
                    expiration_type: value.expiration_type?,
                    option_roots: value.option_roots?,
                    settlement_type: value.settlement_type?,
                    standard: value.standard?,
                })
            }
        }

        impl ::std::convert::From<super::Expiration> for Expiration {
            fn from(value: super::Expiration) -> Self {
                Self {
                    days_to_expiration: Ok(value.days_to_expiration),
                    expiration: Ok(value.expiration),
                    expiration_type: Ok(value.expiration_type),
                    option_roots: Ok(value.option_roots),
                    settlement_type: Ok(value.settlement_type),
                    standard: Ok(value.standard),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ExpirationChain {
            expiration_list:
                ::std::result::Result<::std::vec::Vec<super::Expiration>, ::std::string::String>,
            status: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for ExpirationChain {
            fn default() -> Self {
                Self {
                    expiration_list: Ok(Default::default()),
                    status: Ok(Default::default()),
                }
            }
        }

        impl ExpirationChain {
            pub fn expiration_list<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::Expiration>>,
                T::Error: ::std::fmt::Display,
            {
                self.expiration_list = value.try_into().map_err(|e| {
                    format!("error converting supplied value for expiration_list: {e}")
                });
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ExpirationChain> for super::ExpirationChain {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ExpirationChain,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    expiration_list: value.expiration_list?,
                    status: value.status?,
                })
            }
        }

        impl ::std::convert::From<super::ExpirationChain> for ExpirationChain {
            fn from(value: super::ExpirationChain) -> Self {
                Self {
                    expiration_list: Ok(value.expiration_list),
                    status: Ok(value.status),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ExtendedMarket {
            ask_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            ask_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            bid_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            bid_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            last_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            last_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            mark: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            quote_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            total_volume: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            trade_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        }

        impl ::std::default::Default for ExtendedMarket {
            fn default() -> Self {
                Self {
                    ask_price: Ok(Default::default()),
                    ask_size: Ok(Default::default()),
                    bid_price: Ok(Default::default()),
                    bid_size: Ok(Default::default()),
                    last_price: Ok(Default::default()),
                    last_size: Ok(Default::default()),
                    mark: Ok(Default::default()),
                    quote_time: Ok(Default::default()),
                    total_volume: Ok(Default::default()),
                    trade_time: Ok(Default::default()),
                }
            }
        }

        impl ExtendedMarket {
            pub fn ask_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_price: {e}"));
                self
            }
            pub fn ask_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_size: {e}"));
                self
            }
            pub fn bid_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_price: {e}"));
                self
            }
            pub fn bid_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_size: {e}"));
                self
            }
            pub fn last_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_price: {e}"));
                self
            }
            pub fn last_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_size: {e}"));
                self
            }
            pub fn mark<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mark = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mark: {e}"));
                self
            }
            pub fn quote_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.quote_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quote_time: {e}"));
                self
            }
            pub fn total_volume<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.total_volume = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_volume: {e}"));
                self
            }
            pub fn trade_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.trade_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for trade_time: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ExtendedMarket> for super::ExtendedMarket {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ExtendedMarket,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    ask_price: value.ask_price?,
                    ask_size: value.ask_size?,
                    bid_price: value.bid_price?,
                    bid_size: value.bid_size?,
                    last_price: value.last_price?,
                    last_size: value.last_size?,
                    mark: value.mark?,
                    quote_time: value.quote_time?,
                    total_volume: value.total_volume?,
                    trade_time: value.trade_time?,
                })
            }
        }

        impl ::std::convert::From<super::ExtendedMarket> for ExtendedMarket {
            fn from(value: super::ExtendedMarket) -> Self {
                Self {
                    ask_price: Ok(value.ask_price),
                    ask_size: Ok(value.ask_size),
                    bid_price: Ok(value.bid_price),
                    bid_size: Ok(value.bid_size),
                    last_price: Ok(value.last_price),
                    last_size: Ok(value.last_size),
                    mark: Ok(value.mark),
                    quote_time: Ok(value.quote_time),
                    total_volume: Ok(value.total_volume),
                    trade_time: Ok(value.trade_time),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ForexResponse {
            asset_main_type: ::std::result::Result<
                ::std::option::Option<super::AssetMainType>,
                ::std::string::String,
            >,
            quote: ::std::result::Result<
                ::std::option::Option<super::QuoteForex>,
                ::std::string::String,
            >,
            realtime: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            reference: ::std::result::Result<
                ::std::option::Option<super::ReferenceForex>,
                ::std::string::String,
            >,
            ssid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for ForexResponse {
            fn default() -> Self {
                Self {
                    asset_main_type: Ok(Default::default()),
                    quote: Ok(Default::default()),
                    realtime: Ok(Default::default()),
                    reference: Ok(Default::default()),
                    ssid: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                }
            }
        }

        impl ForexResponse {
            pub fn asset_main_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::AssetMainType>>,
                T::Error: ::std::fmt::Display,
            {
                self.asset_main_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for asset_main_type: {e}")
                });
                self
            }
            pub fn quote<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::QuoteForex>>,
                T::Error: ::std::fmt::Display,
            {
                self.quote = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quote: {e}"));
                self
            }
            pub fn realtime<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.realtime = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for realtime: {e}"));
                self
            }
            pub fn reference<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ReferenceForex>>,
                T::Error: ::std::fmt::Display,
            {
                self.reference = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for reference: {e}"));
                self
            }
            pub fn ssid<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ssid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ssid: {e}"));
                self
            }
            pub fn symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for symbol: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ForexResponse> for super::ForexResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ForexResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_main_type: value.asset_main_type?,
                    quote: value.quote?,
                    realtime: value.realtime?,
                    reference: value.reference?,
                    ssid: value.ssid?,
                    symbol: value.symbol?,
                })
            }
        }

        impl ::std::convert::From<super::ForexResponse> for ForexResponse {
            fn from(value: super::ForexResponse) -> Self {
                Self {
                    asset_main_type: Ok(value.asset_main_type),
                    quote: Ok(value.quote),
                    realtime: Ok(value.realtime),
                    reference: Ok(value.reference),
                    ssid: Ok(value.ssid),
                    symbol: Ok(value.symbol),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Fundamental {
            avg10_days_volume:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            avg1_year_volume:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            declaration_date: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            div_amount: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            div_ex_date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            div_freq:
                ::std::result::Result<::std::option::Option<super::DivFreq>, ::std::string::String>,
            div_pay_amount:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            div_pay_date: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            div_yield: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            eps: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            fund_leverage_factor:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            fund_strategy: ::std::result::Result<
                ::std::option::Option<super::FundStrategy>,
                ::std::string::String,
            >,
            next_div_ex_date: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            next_div_pay_date: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            pe_ratio: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for Fundamental {
            fn default() -> Self {
                Self {
                    avg10_days_volume: Ok(Default::default()),
                    avg1_year_volume: Ok(Default::default()),
                    declaration_date: Ok(Default::default()),
                    div_amount: Ok(Default::default()),
                    div_ex_date: Ok(Default::default()),
                    div_freq: Ok(Default::default()),
                    div_pay_amount: Ok(Default::default()),
                    div_pay_date: Ok(Default::default()),
                    div_yield: Ok(Default::default()),
                    eps: Ok(Default::default()),
                    fund_leverage_factor: Ok(Default::default()),
                    fund_strategy: Ok(Default::default()),
                    next_div_ex_date: Ok(Default::default()),
                    next_div_pay_date: Ok(Default::default()),
                    pe_ratio: Ok(Default::default()),
                }
            }
        }

        impl Fundamental {
            pub fn avg10_days_volume<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.avg10_days_volume = value.try_into().map_err(|e| {
                    format!("error converting supplied value for avg10_days_volume: {e}")
                });
                self
            }
            pub fn avg1_year_volume<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.avg1_year_volume = value.try_into().map_err(|e| {
                    format!("error converting supplied value for avg1_year_volume: {e}")
                });
                self
            }
            pub fn declaration_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.declaration_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for declaration_date: {e}")
                });
                self
            }
            pub fn div_amount<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.div_amount = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for div_amount: {e}"));
                self
            }
            pub fn div_ex_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.div_ex_date = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for div_ex_date: {e}"));
                self
            }
            pub fn div_freq<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::DivFreq>>,
                T::Error: ::std::fmt::Display,
            {
                self.div_freq = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for div_freq: {e}"));
                self
            }
            pub fn div_pay_amount<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.div_pay_amount = value.try_into().map_err(|e| {
                    format!("error converting supplied value for div_pay_amount: {e}")
                });
                self
            }
            pub fn div_pay_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.div_pay_date = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for div_pay_date: {e}"));
                self
            }
            pub fn div_yield<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.div_yield = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for div_yield: {e}"));
                self
            }
            pub fn eps<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.eps = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for eps: {e}"));
                self
            }
            pub fn fund_leverage_factor<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.fund_leverage_factor = value.try_into().map_err(|e| {
                    format!("error converting supplied value for fund_leverage_factor: {e}")
                });
                self
            }
            pub fn fund_strategy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::FundStrategy>>,
                T::Error: ::std::fmt::Display,
            {
                self.fund_strategy = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fund_strategy: {e}"));
                self
            }
            pub fn next_div_ex_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.next_div_ex_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for next_div_ex_date: {e}")
                });
                self
            }
            pub fn next_div_pay_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.next_div_pay_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for next_div_pay_date: {e}")
                });
                self
            }
            pub fn pe_ratio<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.pe_ratio = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pe_ratio: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Fundamental> for super::Fundamental {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Fundamental,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    avg10_days_volume: value.avg10_days_volume?,
                    avg1_year_volume: value.avg1_year_volume?,
                    declaration_date: value.declaration_date?,
                    div_amount: value.div_amount?,
                    div_ex_date: value.div_ex_date?,
                    div_freq: value.div_freq?,
                    div_pay_amount: value.div_pay_amount?,
                    div_pay_date: value.div_pay_date?,
                    div_yield: value.div_yield?,
                    eps: value.eps?,
                    fund_leverage_factor: value.fund_leverage_factor?,
                    fund_strategy: value.fund_strategy?,
                    next_div_ex_date: value.next_div_ex_date?,
                    next_div_pay_date: value.next_div_pay_date?,
                    pe_ratio: value.pe_ratio?,
                })
            }
        }

        impl ::std::convert::From<super::Fundamental> for Fundamental {
            fn from(value: super::Fundamental) -> Self {
                Self {
                    avg10_days_volume: Ok(value.avg10_days_volume),
                    avg1_year_volume: Ok(value.avg1_year_volume),
                    declaration_date: Ok(value.declaration_date),
                    div_amount: Ok(value.div_amount),
                    div_ex_date: Ok(value.div_ex_date),
                    div_freq: Ok(value.div_freq),
                    div_pay_amount: Ok(value.div_pay_amount),
                    div_pay_date: Ok(value.div_pay_date),
                    div_yield: Ok(value.div_yield),
                    eps: Ok(value.eps),
                    fund_leverage_factor: Ok(value.fund_leverage_factor),
                    fund_strategy: Ok(value.fund_strategy),
                    next_div_ex_date: Ok(value.next_div_ex_date),
                    next_div_pay_date: Ok(value.next_div_pay_date),
                    pe_ratio: Ok(value.pe_ratio),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FundamentalInst {
            avg10_days_volume:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            avg1_day_volume:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            avg3_month_volume:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            beta: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            book_value_per_share:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            corpaction_date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            current_ratio: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            declaration_date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            div_growth_rate3_year:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            dividend_amount:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            dividend_date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            dividend_freq: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            dividend_pay_amount:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            dividend_pay_date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            dividend_yield:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            dtn_volume: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            eps: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            eps_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            eps_change_percent_ttm:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            eps_change_year:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            eps_ttm: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            fund_leverage_factor:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            fund_strategy: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            gross_margin_mrq:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            gross_margin_ttm:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            high52: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            interest_coverage:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            low52: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            lt_debt_to_equity:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            market_cap: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            market_cap_float:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            net_profit_margin_mrq:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            net_profit_margin_ttm:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            next_dividend_date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            next_dividend_pay_date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            operating_margin_mrq:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            operating_margin_ttm:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            pb_ratio: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            pcf_ratio: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            pe_ratio: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            peg_ratio: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            pr_ratio: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            quick_ratio: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            return_on_assets:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            return_on_equity:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            return_on_investment:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            rev_change_in: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            rev_change_ttm:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            rev_change_year:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            shares_outstanding:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            short_int_day_to_cover:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            short_int_to_float:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            total_debt_to_capital:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            total_debt_to_equity:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            vol10_day_avg: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            vol1_day_avg: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            vol3_month_avg:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for FundamentalInst {
            fn default() -> Self {
                Self {
                    avg10_days_volume: Ok(Default::default()),
                    avg1_day_volume: Ok(Default::default()),
                    avg3_month_volume: Ok(Default::default()),
                    beta: Ok(Default::default()),
                    book_value_per_share: Ok(Default::default()),
                    corpaction_date: Ok(Default::default()),
                    current_ratio: Ok(Default::default()),
                    declaration_date: Ok(Default::default()),
                    div_growth_rate3_year: Ok(Default::default()),
                    dividend_amount: Ok(Default::default()),
                    dividend_date: Ok(Default::default()),
                    dividend_freq: Ok(Default::default()),
                    dividend_pay_amount: Ok(Default::default()),
                    dividend_pay_date: Ok(Default::default()),
                    dividend_yield: Ok(Default::default()),
                    dtn_volume: Ok(Default::default()),
                    eps: Ok(Default::default()),
                    eps_change: Ok(Default::default()),
                    eps_change_percent_ttm: Ok(Default::default()),
                    eps_change_year: Ok(Default::default()),
                    eps_ttm: Ok(Default::default()),
                    fund_leverage_factor: Ok(Default::default()),
                    fund_strategy: Ok(Default::default()),
                    gross_margin_mrq: Ok(Default::default()),
                    gross_margin_ttm: Ok(Default::default()),
                    high52: Ok(Default::default()),
                    interest_coverage: Ok(Default::default()),
                    low52: Ok(Default::default()),
                    lt_debt_to_equity: Ok(Default::default()),
                    market_cap: Ok(Default::default()),
                    market_cap_float: Ok(Default::default()),
                    net_profit_margin_mrq: Ok(Default::default()),
                    net_profit_margin_ttm: Ok(Default::default()),
                    next_dividend_date: Ok(Default::default()),
                    next_dividend_pay_date: Ok(Default::default()),
                    operating_margin_mrq: Ok(Default::default()),
                    operating_margin_ttm: Ok(Default::default()),
                    pb_ratio: Ok(Default::default()),
                    pcf_ratio: Ok(Default::default()),
                    pe_ratio: Ok(Default::default()),
                    peg_ratio: Ok(Default::default()),
                    pr_ratio: Ok(Default::default()),
                    quick_ratio: Ok(Default::default()),
                    return_on_assets: Ok(Default::default()),
                    return_on_equity: Ok(Default::default()),
                    return_on_investment: Ok(Default::default()),
                    rev_change_in: Ok(Default::default()),
                    rev_change_ttm: Ok(Default::default()),
                    rev_change_year: Ok(Default::default()),
                    shares_outstanding: Ok(Default::default()),
                    short_int_day_to_cover: Ok(Default::default()),
                    short_int_to_float: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    total_debt_to_capital: Ok(Default::default()),
                    total_debt_to_equity: Ok(Default::default()),
                    vol10_day_avg: Ok(Default::default()),
                    vol1_day_avg: Ok(Default::default()),
                    vol3_month_avg: Ok(Default::default()),
                }
            }
        }

        impl FundamentalInst {
            pub fn avg10_days_volume<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.avg10_days_volume = value.try_into().map_err(|e| {
                    format!("error converting supplied value for avg10_days_volume: {e}")
                });
                self
            }
            pub fn avg1_day_volume<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.avg1_day_volume = value.try_into().map_err(|e| {
                    format!("error converting supplied value for avg1_day_volume: {e}")
                });
                self
            }
            pub fn avg3_month_volume<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.avg3_month_volume = value.try_into().map_err(|e| {
                    format!("error converting supplied value for avg3_month_volume: {e}")
                });
                self
            }
            pub fn beta<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.beta = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for beta: {e}"));
                self
            }
            pub fn book_value_per_share<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.book_value_per_share = value.try_into().map_err(|e| {
                    format!("error converting supplied value for book_value_per_share: {e}")
                });
                self
            }
            pub fn corpaction_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.corpaction_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for corpaction_date: {e}")
                });
                self
            }
            pub fn current_ratio<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.current_ratio = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for current_ratio: {e}"));
                self
            }
            pub fn declaration_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.declaration_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for declaration_date: {e}")
                });
                self
            }
            pub fn div_growth_rate3_year<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.div_growth_rate3_year = value.try_into().map_err(|e| {
                    format!("error converting supplied value for div_growth_rate3_year: {e}")
                });
                self
            }
            pub fn dividend_amount<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.dividend_amount = value.try_into().map_err(|e| {
                    format!("error converting supplied value for dividend_amount: {e}")
                });
                self
            }
            pub fn dividend_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.dividend_date = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for dividend_date: {e}"));
                self
            }
            pub fn dividend_freq<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.dividend_freq = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for dividend_freq: {e}"));
                self
            }
            pub fn dividend_pay_amount<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.dividend_pay_amount = value.try_into().map_err(|e| {
                    format!("error converting supplied value for dividend_pay_amount: {e}")
                });
                self
            }
            pub fn dividend_pay_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.dividend_pay_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for dividend_pay_date: {e}")
                });
                self
            }
            pub fn dividend_yield<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.dividend_yield = value.try_into().map_err(|e| {
                    format!("error converting supplied value for dividend_yield: {e}")
                });
                self
            }
            pub fn dtn_volume<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.dtn_volume = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for dtn_volume: {e}"));
                self
            }
            pub fn eps<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.eps = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for eps: {e}"));
                self
            }
            pub fn eps_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.eps_change = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for eps_change: {e}"));
                self
            }
            pub fn eps_change_percent_ttm<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.eps_change_percent_ttm = value.try_into().map_err(|e| {
                    format!("error converting supplied value for eps_change_percent_ttm: {e}")
                });
                self
            }
            pub fn eps_change_year<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.eps_change_year = value.try_into().map_err(|e| {
                    format!("error converting supplied value for eps_change_year: {e}")
                });
                self
            }
            pub fn eps_ttm<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.eps_ttm = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for eps_ttm: {e}"));
                self
            }
            pub fn fund_leverage_factor<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.fund_leverage_factor = value.try_into().map_err(|e| {
                    format!("error converting supplied value for fund_leverage_factor: {e}")
                });
                self
            }
            pub fn fund_strategy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.fund_strategy = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fund_strategy: {e}"));
                self
            }
            pub fn gross_margin_mrq<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.gross_margin_mrq = value.try_into().map_err(|e| {
                    format!("error converting supplied value for gross_margin_mrq: {e}")
                });
                self
            }
            pub fn gross_margin_ttm<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.gross_margin_ttm = value.try_into().map_err(|e| {
                    format!("error converting supplied value for gross_margin_ttm: {e}")
                });
                self
            }
            pub fn high52<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.high52 = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for high52: {e}"));
                self
            }
            pub fn interest_coverage<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.interest_coverage = value.try_into().map_err(|e| {
                    format!("error converting supplied value for interest_coverage: {e}")
                });
                self
            }
            pub fn low52<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.low52 = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for low52: {e}"));
                self
            }
            pub fn lt_debt_to_equity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.lt_debt_to_equity = value.try_into().map_err(|e| {
                    format!("error converting supplied value for lt_debt_to_equity: {e}")
                });
                self
            }
            pub fn market_cap<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.market_cap = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for market_cap: {e}"));
                self
            }
            pub fn market_cap_float<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.market_cap_float = value.try_into().map_err(|e| {
                    format!("error converting supplied value for market_cap_float: {e}")
                });
                self
            }
            pub fn net_profit_margin_mrq<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.net_profit_margin_mrq = value.try_into().map_err(|e| {
                    format!("error converting supplied value for net_profit_margin_mrq: {e}")
                });
                self
            }
            pub fn net_profit_margin_ttm<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.net_profit_margin_ttm = value.try_into().map_err(|e| {
                    format!("error converting supplied value for net_profit_margin_ttm: {e}")
                });
                self
            }
            pub fn next_dividend_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.next_dividend_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for next_dividend_date: {e}")
                });
                self
            }
            pub fn next_dividend_pay_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.next_dividend_pay_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for next_dividend_pay_date: {e}")
                });
                self
            }
            pub fn operating_margin_mrq<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.operating_margin_mrq = value.try_into().map_err(|e| {
                    format!("error converting supplied value for operating_margin_mrq: {e}")
                });
                self
            }
            pub fn operating_margin_ttm<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.operating_margin_ttm = value.try_into().map_err(|e| {
                    format!("error converting supplied value for operating_margin_ttm: {e}")
                });
                self
            }
            pub fn pb_ratio<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.pb_ratio = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pb_ratio: {e}"));
                self
            }
            pub fn pcf_ratio<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.pcf_ratio = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pcf_ratio: {e}"));
                self
            }
            pub fn pe_ratio<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.pe_ratio = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pe_ratio: {e}"));
                self
            }
            pub fn peg_ratio<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.peg_ratio = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for peg_ratio: {e}"));
                self
            }
            pub fn pr_ratio<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.pr_ratio = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pr_ratio: {e}"));
                self
            }
            pub fn quick_ratio<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.quick_ratio = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quick_ratio: {e}"));
                self
            }
            pub fn return_on_assets<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.return_on_assets = value.try_into().map_err(|e| {
                    format!("error converting supplied value for return_on_assets: {e}")
                });
                self
            }
            pub fn return_on_equity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.return_on_equity = value.try_into().map_err(|e| {
                    format!("error converting supplied value for return_on_equity: {e}")
                });
                self
            }
            pub fn return_on_investment<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.return_on_investment = value.try_into().map_err(|e| {
                    format!("error converting supplied value for return_on_investment: {e}")
                });
                self
            }
            pub fn rev_change_in<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.rev_change_in = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rev_change_in: {e}"));
                self
            }
            pub fn rev_change_ttm<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.rev_change_ttm = value.try_into().map_err(|e| {
                    format!("error converting supplied value for rev_change_ttm: {e}")
                });
                self
            }
            pub fn rev_change_year<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.rev_change_year = value.try_into().map_err(|e| {
                    format!("error converting supplied value for rev_change_year: {e}")
                });
                self
            }
            pub fn shares_outstanding<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.shares_outstanding = value.try_into().map_err(|e| {
                    format!("error converting supplied value for shares_outstanding: {e}")
                });
                self
            }
            pub fn short_int_day_to_cover<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.short_int_day_to_cover = value.try_into().map_err(|e| {
                    format!("error converting supplied value for short_int_day_to_cover: {e}")
                });
                self
            }
            pub fn short_int_to_float<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.short_int_to_float = value.try_into().map_err(|e| {
                    format!("error converting supplied value for short_int_to_float: {e}")
                });
                self
            }
            pub fn symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for symbol: {e}"));
                self
            }
            pub fn total_debt_to_capital<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.total_debt_to_capital = value.try_into().map_err(|e| {
                    format!("error converting supplied value for total_debt_to_capital: {e}")
                });
                self
            }
            pub fn total_debt_to_equity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.total_debt_to_equity = value.try_into().map_err(|e| {
                    format!("error converting supplied value for total_debt_to_equity: {e}")
                });
                self
            }
            pub fn vol10_day_avg<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.vol10_day_avg = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for vol10_day_avg: {e}"));
                self
            }
            pub fn vol1_day_avg<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.vol1_day_avg = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for vol1_day_avg: {e}"));
                self
            }
            pub fn vol3_month_avg<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.vol3_month_avg = value.try_into().map_err(|e| {
                    format!("error converting supplied value for vol3_month_avg: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<FundamentalInst> for super::FundamentalInst {
            type Error = super::error::ConversionError;
            fn try_from(
                value: FundamentalInst,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    avg10_days_volume: value.avg10_days_volume?,
                    avg1_day_volume: value.avg1_day_volume?,
                    avg3_month_volume: value.avg3_month_volume?,
                    beta: value.beta?,
                    book_value_per_share: value.book_value_per_share?,
                    corpaction_date: value.corpaction_date?,
                    current_ratio: value.current_ratio?,
                    declaration_date: value.declaration_date?,
                    div_growth_rate3_year: value.div_growth_rate3_year?,
                    dividend_amount: value.dividend_amount?,
                    dividend_date: value.dividend_date?,
                    dividend_freq: value.dividend_freq?,
                    dividend_pay_amount: value.dividend_pay_amount?,
                    dividend_pay_date: value.dividend_pay_date?,
                    dividend_yield: value.dividend_yield?,
                    dtn_volume: value.dtn_volume?,
                    eps: value.eps?,
                    eps_change: value.eps_change?,
                    eps_change_percent_ttm: value.eps_change_percent_ttm?,
                    eps_change_year: value.eps_change_year?,
                    eps_ttm: value.eps_ttm?,
                    fund_leverage_factor: value.fund_leverage_factor?,
                    fund_strategy: value.fund_strategy?,
                    gross_margin_mrq: value.gross_margin_mrq?,
                    gross_margin_ttm: value.gross_margin_ttm?,
                    high52: value.high52?,
                    interest_coverage: value.interest_coverage?,
                    low52: value.low52?,
                    lt_debt_to_equity: value.lt_debt_to_equity?,
                    market_cap: value.market_cap?,
                    market_cap_float: value.market_cap_float?,
                    net_profit_margin_mrq: value.net_profit_margin_mrq?,
                    net_profit_margin_ttm: value.net_profit_margin_ttm?,
                    next_dividend_date: value.next_dividend_date?,
                    next_dividend_pay_date: value.next_dividend_pay_date?,
                    operating_margin_mrq: value.operating_margin_mrq?,
                    operating_margin_ttm: value.operating_margin_ttm?,
                    pb_ratio: value.pb_ratio?,
                    pcf_ratio: value.pcf_ratio?,
                    pe_ratio: value.pe_ratio?,
                    peg_ratio: value.peg_ratio?,
                    pr_ratio: value.pr_ratio?,
                    quick_ratio: value.quick_ratio?,
                    return_on_assets: value.return_on_assets?,
                    return_on_equity: value.return_on_equity?,
                    return_on_investment: value.return_on_investment?,
                    rev_change_in: value.rev_change_in?,
                    rev_change_ttm: value.rev_change_ttm?,
                    rev_change_year: value.rev_change_year?,
                    shares_outstanding: value.shares_outstanding?,
                    short_int_day_to_cover: value.short_int_day_to_cover?,
                    short_int_to_float: value.short_int_to_float?,
                    symbol: value.symbol?,
                    total_debt_to_capital: value.total_debt_to_capital?,
                    total_debt_to_equity: value.total_debt_to_equity?,
                    vol10_day_avg: value.vol10_day_avg?,
                    vol1_day_avg: value.vol1_day_avg?,
                    vol3_month_avg: value.vol3_month_avg?,
                })
            }
        }

        impl ::std::convert::From<super::FundamentalInst> for FundamentalInst {
            fn from(value: super::FundamentalInst) -> Self {
                Self {
                    avg10_days_volume: Ok(value.avg10_days_volume),
                    avg1_day_volume: Ok(value.avg1_day_volume),
                    avg3_month_volume: Ok(value.avg3_month_volume),
                    beta: Ok(value.beta),
                    book_value_per_share: Ok(value.book_value_per_share),
                    corpaction_date: Ok(value.corpaction_date),
                    current_ratio: Ok(value.current_ratio),
                    declaration_date: Ok(value.declaration_date),
                    div_growth_rate3_year: Ok(value.div_growth_rate3_year),
                    dividend_amount: Ok(value.dividend_amount),
                    dividend_date: Ok(value.dividend_date),
                    dividend_freq: Ok(value.dividend_freq),
                    dividend_pay_amount: Ok(value.dividend_pay_amount),
                    dividend_pay_date: Ok(value.dividend_pay_date),
                    dividend_yield: Ok(value.dividend_yield),
                    dtn_volume: Ok(value.dtn_volume),
                    eps: Ok(value.eps),
                    eps_change: Ok(value.eps_change),
                    eps_change_percent_ttm: Ok(value.eps_change_percent_ttm),
                    eps_change_year: Ok(value.eps_change_year),
                    eps_ttm: Ok(value.eps_ttm),
                    fund_leverage_factor: Ok(value.fund_leverage_factor),
                    fund_strategy: Ok(value.fund_strategy),
                    gross_margin_mrq: Ok(value.gross_margin_mrq),
                    gross_margin_ttm: Ok(value.gross_margin_ttm),
                    high52: Ok(value.high52),
                    interest_coverage: Ok(value.interest_coverage),
                    low52: Ok(value.low52),
                    lt_debt_to_equity: Ok(value.lt_debt_to_equity),
                    market_cap: Ok(value.market_cap),
                    market_cap_float: Ok(value.market_cap_float),
                    net_profit_margin_mrq: Ok(value.net_profit_margin_mrq),
                    net_profit_margin_ttm: Ok(value.net_profit_margin_ttm),
                    next_dividend_date: Ok(value.next_dividend_date),
                    next_dividend_pay_date: Ok(value.next_dividend_pay_date),
                    operating_margin_mrq: Ok(value.operating_margin_mrq),
                    operating_margin_ttm: Ok(value.operating_margin_ttm),
                    pb_ratio: Ok(value.pb_ratio),
                    pcf_ratio: Ok(value.pcf_ratio),
                    pe_ratio: Ok(value.pe_ratio),
                    peg_ratio: Ok(value.peg_ratio),
                    pr_ratio: Ok(value.pr_ratio),
                    quick_ratio: Ok(value.quick_ratio),
                    return_on_assets: Ok(value.return_on_assets),
                    return_on_equity: Ok(value.return_on_equity),
                    return_on_investment: Ok(value.return_on_investment),
                    rev_change_in: Ok(value.rev_change_in),
                    rev_change_ttm: Ok(value.rev_change_ttm),
                    rev_change_year: Ok(value.rev_change_year),
                    shares_outstanding: Ok(value.shares_outstanding),
                    short_int_day_to_cover: Ok(value.short_int_day_to_cover),
                    short_int_to_float: Ok(value.short_int_to_float),
                    symbol: Ok(value.symbol),
                    total_debt_to_capital: Ok(value.total_debt_to_capital),
                    total_debt_to_equity: Ok(value.total_debt_to_equity),
                    vol10_day_avg: Ok(value.vol10_day_avg),
                    vol1_day_avg: Ok(value.vol1_day_avg),
                    vol3_month_avg: Ok(value.vol3_month_avg),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FutureOptionResponse {
            asset_main_type: ::std::result::Result<
                ::std::option::Option<super::AssetMainType>,
                ::std::string::String,
            >,
            quote: ::std::result::Result<
                ::std::option::Option<super::QuoteFutureOption>,
                ::std::string::String,
            >,
            realtime: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            reference: ::std::result::Result<
                ::std::option::Option<super::ReferenceFutureOption>,
                ::std::string::String,
            >,
            ssid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for FutureOptionResponse {
            fn default() -> Self {
                Self {
                    asset_main_type: Ok(Default::default()),
                    quote: Ok(Default::default()),
                    realtime: Ok(Default::default()),
                    reference: Ok(Default::default()),
                    ssid: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                }
            }
        }

        impl FutureOptionResponse {
            pub fn asset_main_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::AssetMainType>>,
                T::Error: ::std::fmt::Display,
            {
                self.asset_main_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for asset_main_type: {e}")
                });
                self
            }
            pub fn quote<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::QuoteFutureOption>>,
                T::Error: ::std::fmt::Display,
            {
                self.quote = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quote: {e}"));
                self
            }
            pub fn realtime<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.realtime = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for realtime: {e}"));
                self
            }
            pub fn reference<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ReferenceFutureOption>>,
                T::Error: ::std::fmt::Display,
            {
                self.reference = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for reference: {e}"));
                self
            }
            pub fn ssid<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ssid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ssid: {e}"));
                self
            }
            pub fn symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for symbol: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<FutureOptionResponse> for super::FutureOptionResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: FutureOptionResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_main_type: value.asset_main_type?,
                    quote: value.quote?,
                    realtime: value.realtime?,
                    reference: value.reference?,
                    ssid: value.ssid?,
                    symbol: value.symbol?,
                })
            }
        }

        impl ::std::convert::From<super::FutureOptionResponse> for FutureOptionResponse {
            fn from(value: super::FutureOptionResponse) -> Self {
                Self {
                    asset_main_type: Ok(value.asset_main_type),
                    quote: Ok(value.quote),
                    realtime: Ok(value.realtime),
                    reference: Ok(value.reference),
                    ssid: Ok(value.ssid),
                    symbol: Ok(value.symbol),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FutureResponse {
            asset_main_type: ::std::result::Result<
                ::std::option::Option<super::AssetMainType>,
                ::std::string::String,
            >,
            quote: ::std::result::Result<
                ::std::option::Option<super::QuoteFuture>,
                ::std::string::String,
            >,
            realtime: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            reference: ::std::result::Result<
                ::std::option::Option<super::ReferenceFuture>,
                ::std::string::String,
            >,
            ssid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for FutureResponse {
            fn default() -> Self {
                Self {
                    asset_main_type: Ok(Default::default()),
                    quote: Ok(Default::default()),
                    realtime: Ok(Default::default()),
                    reference: Ok(Default::default()),
                    ssid: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                }
            }
        }

        impl FutureResponse {
            pub fn asset_main_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::AssetMainType>>,
                T::Error: ::std::fmt::Display,
            {
                self.asset_main_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for asset_main_type: {e}")
                });
                self
            }
            pub fn quote<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::QuoteFuture>>,
                T::Error: ::std::fmt::Display,
            {
                self.quote = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quote: {e}"));
                self
            }
            pub fn realtime<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.realtime = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for realtime: {e}"));
                self
            }
            pub fn reference<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ReferenceFuture>>,
                T::Error: ::std::fmt::Display,
            {
                self.reference = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for reference: {e}"));
                self
            }
            pub fn ssid<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ssid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ssid: {e}"));
                self
            }
            pub fn symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for symbol: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<FutureResponse> for super::FutureResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: FutureResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_main_type: value.asset_main_type?,
                    quote: value.quote?,
                    realtime: value.realtime?,
                    reference: value.reference?,
                    ssid: value.ssid?,
                    symbol: value.symbol?,
                })
            }
        }

        impl ::std::convert::From<super::FutureResponse> for FutureResponse {
            fn from(value: super::FutureResponse) -> Self {
                Self {
                    asset_main_type: Ok(value.asset_main_type),
                    quote: Ok(value.quote),
                    realtime: Ok(value.realtime),
                    reference: Ok(value.reference),
                    ssid: Ok(value.ssid),
                    symbol: Ok(value.symbol),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetInstrumentsResponse {
            instruments: ::std::result::Result<
                ::std::vec::Vec<super::InstrumentResponse>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetInstrumentsResponse {
            fn default() -> Self {
                Self {
                    instruments: Ok(Default::default()),
                }
            }
        }

        impl GetInstrumentsResponse {
            pub fn instruments<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::InstrumentResponse>>,
                T::Error: ::std::fmt::Display,
            {
                self.instruments = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instruments: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetInstrumentsResponse> for super::GetInstrumentsResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetInstrumentsResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    instruments: value.instruments?,
                })
            }
        }

        impl ::std::convert::From<super::GetInstrumentsResponse> for GetInstrumentsResponse {
            fn from(value: super::GetInstrumentsResponse) -> Self {
                Self {
                    instruments: Ok(value.instruments),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetMoversResponse {
            screeners:
                ::std::result::Result<::std::vec::Vec<super::Screener>, ::std::string::String>,
        }

        impl ::std::default::Default for GetMoversResponse {
            fn default() -> Self {
                Self {
                    screeners: Ok(Default::default()),
                }
            }
        }

        impl GetMoversResponse {
            pub fn screeners<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::Screener>>,
                T::Error: ::std::fmt::Display,
            {
                self.screeners = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for screeners: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<GetMoversResponse> for super::GetMoversResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetMoversResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    screeners: value.screeners?,
                })
            }
        }

        impl ::std::convert::From<super::GetMoversResponse> for GetMoversResponse {
            fn from(value: super::GetMoversResponse) -> Self {
                Self {
                    screeners: Ok(value.screeners),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Hours {
            category: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            is_open: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            market_type: ::std::result::Result<
                ::std::option::Option<super::HoursMarketType>,
                ::std::string::String,
            >,
            product: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            product_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            session_hours: ::std::result::Result<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::vec::Vec<super::Interval>,
                >,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for Hours {
            fn default() -> Self {
                Self {
                    category: Ok(Default::default()),
                    date: Ok(Default::default()),
                    exchange: Ok(Default::default()),
                    is_open: Ok(Default::default()),
                    market_type: Ok(Default::default()),
                    product: Ok(Default::default()),
                    product_name: Ok(Default::default()),
                    session_hours: Ok(Default::default()),
                }
            }
        }

        impl Hours {
            pub fn category<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.category = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for category: {e}"));
                self
            }
            pub fn date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.date = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for date: {e}"));
                self
            }
            pub fn exchange<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange: {e}"));
                self
            }
            pub fn is_open<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.is_open = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for is_open: {e}"));
                self
            }
            pub fn market_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::HoursMarketType>>,
                T::Error: ::std::fmt::Display,
            {
                self.market_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for market_type: {e}"));
                self
            }
            pub fn product<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.product = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for product: {e}"));
                self
            }
            pub fn product_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.product_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for product_name: {e}"));
                self
            }
            pub fn session_hours<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::collections::HashMap<
                            ::std::string::String,
                            ::std::vec::Vec<super::Interval>,
                        >,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.session_hours = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for session_hours: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Hours> for super::Hours {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Hours,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    category: value.category?,
                    date: value.date?,
                    exchange: value.exchange?,
                    is_open: value.is_open?,
                    market_type: value.market_type?,
                    product: value.product?,
                    product_name: value.product_name?,
                    session_hours: value.session_hours?,
                })
            }
        }

        impl ::std::convert::From<super::Hours> for Hours {
            fn from(value: super::Hours) -> Self {
                Self {
                    category: Ok(value.category),
                    date: Ok(value.date),
                    exchange: Ok(value.exchange),
                    is_open: Ok(value.is_open),
                    market_type: Ok(value.market_type),
                    product: Ok(value.product),
                    product_name: Ok(value.product_name),
                    session_hours: Ok(value.session_hours),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct IndexResponse {
            asset_main_type: ::std::result::Result<
                ::std::option::Option<super::AssetMainType>,
                ::std::string::String,
            >,
            quote: ::std::result::Result<
                ::std::option::Option<super::QuoteIndex>,
                ::std::string::String,
            >,
            realtime: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            reference: ::std::result::Result<
                ::std::option::Option<super::ReferenceIndex>,
                ::std::string::String,
            >,
            ssid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for IndexResponse {
            fn default() -> Self {
                Self {
                    asset_main_type: Ok(Default::default()),
                    quote: Ok(Default::default()),
                    realtime: Ok(Default::default()),
                    reference: Ok(Default::default()),
                    ssid: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                }
            }
        }

        impl IndexResponse {
            pub fn asset_main_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::AssetMainType>>,
                T::Error: ::std::fmt::Display,
            {
                self.asset_main_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for asset_main_type: {e}")
                });
                self
            }
            pub fn quote<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::QuoteIndex>>,
                T::Error: ::std::fmt::Display,
            {
                self.quote = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quote: {e}"));
                self
            }
            pub fn realtime<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.realtime = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for realtime: {e}"));
                self
            }
            pub fn reference<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ReferenceIndex>>,
                T::Error: ::std::fmt::Display,
            {
                self.reference = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for reference: {e}"));
                self
            }
            pub fn ssid<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ssid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ssid: {e}"));
                self
            }
            pub fn symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for symbol: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<IndexResponse> for super::IndexResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: IndexResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_main_type: value.asset_main_type?,
                    quote: value.quote?,
                    realtime: value.realtime?,
                    reference: value.reference?,
                    ssid: value.ssid?,
                    symbol: value.symbol?,
                })
            }
        }

        impl ::std::convert::From<super::IndexResponse> for IndexResponse {
            fn from(value: super::IndexResponse) -> Self {
                Self {
                    asset_main_type: Ok(value.asset_main_type),
                    quote: Ok(value.quote),
                    realtime: Ok(value.realtime),
                    reference: Ok(value.reference),
                    ssid: Ok(value.ssid),
                    symbol: Ok(value.symbol),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Instrument {
            asset_type: ::std::result::Result<
                ::std::option::Option<super::InstrumentAssetType>,
                ::std::string::String,
            >,
            cusip: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                ::std::option::Option<super::InstrumentType>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for Instrument {
            fn default() -> Self {
                Self {
                    asset_type: Ok(Default::default()),
                    cusip: Ok(Default::default()),
                    description: Ok(Default::default()),
                    exchange: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl Instrument {
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::InstrumentAssetType>>,
                T::Error: ::std::fmt::Display,
            {
                self.asset_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for asset_type: {e}"));
                self
            }
            pub fn cusip<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.cusip = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cusip: {e}"));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {e}"));
                self
            }
            pub fn exchange<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange: {e}"));
                self
            }
            pub fn symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for symbol: {e}"));
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::InstrumentType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Instrument> for super::Instrument {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Instrument,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_type: value.asset_type?,
                    cusip: value.cusip?,
                    description: value.description?,
                    exchange: value.exchange?,
                    symbol: value.symbol?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::Instrument> for Instrument {
            fn from(value: super::Instrument) -> Self {
                Self {
                    asset_type: Ok(value.asset_type),
                    cusip: Ok(value.cusip),
                    description: Ok(value.description),
                    exchange: Ok(value.exchange),
                    symbol: Ok(value.symbol),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InstrumentResponse {
            asset_type: ::std::result::Result<
                ::std::option::Option<super::InstrumentResponseAssetType>,
                ::std::string::String,
            >,
            bond_factor: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            bond_instrument_info:
                ::std::result::Result<::std::option::Option<super::Bond>, ::std::string::String>,
            bond_multiplier: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            bond_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            cusip: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            fundamental: ::std::result::Result<
                ::std::option::Option<super::FundamentalInst>,
                ::std::string::String,
            >,
            instrument_info: ::std::result::Result<
                ::std::option::Option<super::Instrument>,
                ::std::string::String,
            >,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                ::std::option::Option<super::InstrumentResponseType>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for InstrumentResponse {
            fn default() -> Self {
                Self {
                    asset_type: Ok(Default::default()),
                    bond_factor: Ok(Default::default()),
                    bond_instrument_info: Ok(Default::default()),
                    bond_multiplier: Ok(Default::default()),
                    bond_price: Ok(Default::default()),
                    cusip: Ok(Default::default()),
                    description: Ok(Default::default()),
                    exchange: Ok(Default::default()),
                    fundamental: Ok(Default::default()),
                    instrument_info: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl InstrumentResponse {
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<super::InstrumentResponseAssetType>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.asset_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for asset_type: {e}"));
                self
            }
            pub fn bond_factor<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.bond_factor = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bond_factor: {e}"));
                self
            }
            pub fn bond_instrument_info<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Bond>>,
                T::Error: ::std::fmt::Display,
            {
                self.bond_instrument_info = value.try_into().map_err(|e| {
                    format!("error converting supplied value for bond_instrument_info: {e}")
                });
                self
            }
            pub fn bond_multiplier<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.bond_multiplier = value.try_into().map_err(|e| {
                    format!("error converting supplied value for bond_multiplier: {e}")
                });
                self
            }
            pub fn bond_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.bond_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bond_price: {e}"));
                self
            }
            pub fn cusip<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.cusip = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cusip: {e}"));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {e}"));
                self
            }
            pub fn exchange<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange: {e}"));
                self
            }
            pub fn fundamental<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::FundamentalInst>>,
                T::Error: ::std::fmt::Display,
            {
                self.fundamental = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fundamental: {e}"));
                self
            }
            pub fn instrument_info<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Instrument>>,
                T::Error: ::std::fmt::Display,
            {
                self.instrument_info = value.try_into().map_err(|e| {
                    format!("error converting supplied value for instrument_info: {e}")
                });
                self
            }
            pub fn symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for symbol: {e}"));
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::InstrumentResponseType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<InstrumentResponse> for super::InstrumentResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InstrumentResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_type: value.asset_type?,
                    bond_factor: value.bond_factor?,
                    bond_instrument_info: value.bond_instrument_info?,
                    bond_multiplier: value.bond_multiplier?,
                    bond_price: value.bond_price?,
                    cusip: value.cusip?,
                    description: value.description?,
                    exchange: value.exchange?,
                    fundamental: value.fundamental?,
                    instrument_info: value.instrument_info?,
                    symbol: value.symbol?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::InstrumentResponse> for InstrumentResponse {
            fn from(value: super::InstrumentResponse) -> Self {
                Self {
                    asset_type: Ok(value.asset_type),
                    bond_factor: Ok(value.bond_factor),
                    bond_instrument_info: Ok(value.bond_instrument_info),
                    bond_multiplier: Ok(value.bond_multiplier),
                    bond_price: Ok(value.bond_price),
                    cusip: Ok(value.cusip),
                    description: Ok(value.description),
                    exchange: Ok(value.exchange),
                    fundamental: Ok(value.fundamental),
                    instrument_info: Ok(value.instrument_info),
                    symbol: Ok(value.symbol),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Interval {
            end: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            start: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for Interval {
            fn default() -> Self {
                Self {
                    end: Ok(Default::default()),
                    start: Ok(Default::default()),
                }
            }
        }

        impl Interval {
            pub fn end<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.end = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for end: {e}"));
                self
            }
            pub fn start<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.start = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for start: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Interval> for super::Interval {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Interval,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    end: value.end?,
                    start: value.start?,
                })
            }
        }

        impl ::std::convert::From<super::Interval> for Interval {
            fn from(value: super::Interval) -> Self {
                Self {
                    end: Ok(value.end),
                    start: Ok(value.start),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MutualFundResponse {
            asset_main_type: ::std::result::Result<
                ::std::option::Option<super::AssetMainType>,
                ::std::string::String,
            >,
            asset_sub_type: ::std::result::Result<
                ::std::option::Option<super::MutualFundAssetSubType>,
                ::std::string::String,
            >,
            fundamental: ::std::result::Result<
                ::std::option::Option<super::Fundamental>,
                ::std::string::String,
            >,
            quote: ::std::result::Result<
                ::std::option::Option<super::QuoteMutualFund>,
                ::std::string::String,
            >,
            realtime: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            reference: ::std::result::Result<
                ::std::option::Option<super::ReferenceMutualFund>,
                ::std::string::String,
            >,
            ssid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for MutualFundResponse {
            fn default() -> Self {
                Self {
                    asset_main_type: Ok(Default::default()),
                    asset_sub_type: Ok(Default::default()),
                    fundamental: Ok(Default::default()),
                    quote: Ok(Default::default()),
                    realtime: Ok(Default::default()),
                    reference: Ok(Default::default()),
                    ssid: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                }
            }
        }

        impl MutualFundResponse {
            pub fn asset_main_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::AssetMainType>>,
                T::Error: ::std::fmt::Display,
            {
                self.asset_main_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for asset_main_type: {e}")
                });
                self
            }
            pub fn asset_sub_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::MutualFundAssetSubType>>,
                T::Error: ::std::fmt::Display,
            {
                self.asset_sub_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for asset_sub_type: {e}")
                });
                self
            }
            pub fn fundamental<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Fundamental>>,
                T::Error: ::std::fmt::Display,
            {
                self.fundamental = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fundamental: {e}"));
                self
            }
            pub fn quote<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::QuoteMutualFund>>,
                T::Error: ::std::fmt::Display,
            {
                self.quote = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quote: {e}"));
                self
            }
            pub fn realtime<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.realtime = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for realtime: {e}"));
                self
            }
            pub fn reference<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ReferenceMutualFund>>,
                T::Error: ::std::fmt::Display,
            {
                self.reference = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for reference: {e}"));
                self
            }
            pub fn ssid<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ssid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ssid: {e}"));
                self
            }
            pub fn symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for symbol: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<MutualFundResponse> for super::MutualFundResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MutualFundResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_main_type: value.asset_main_type?,
                    asset_sub_type: value.asset_sub_type?,
                    fundamental: value.fundamental?,
                    quote: value.quote?,
                    realtime: value.realtime?,
                    reference: value.reference?,
                    ssid: value.ssid?,
                    symbol: value.symbol?,
                })
            }
        }

        impl ::std::convert::From<super::MutualFundResponse> for MutualFundResponse {
            fn from(value: super::MutualFundResponse) -> Self {
                Self {
                    asset_main_type: Ok(value.asset_main_type),
                    asset_sub_type: Ok(value.asset_sub_type),
                    fundamental: Ok(value.fundamental),
                    quote: Ok(value.quote),
                    realtime: Ok(value.realtime),
                    reference: Ok(value.reference),
                    ssid: Ok(value.ssid),
                    symbol: Ok(value.symbol),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OptionChain {
            call_exp_date_map: ::std::result::Result<
                ::std::collections::HashMap<::std::string::String, super::OptionContractMap>,
                ::std::string::String,
            >,
            days_to_expiration:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            interest_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            interval: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            is_delayed: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            is_index: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            put_exp_date_map: ::std::result::Result<
                ::std::collections::HashMap<::std::string::String, super::OptionContractMap>,
                ::std::string::String,
            >,
            status: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            strategy: ::std::result::Result<
                ::std::option::Option<super::OptionChainStrategy>,
                ::std::string::String,
            >,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            underlying: ::std::result::Result<
                ::std::option::Option<super::Underlying>,
                ::std::string::String,
            >,
            underlying_price:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            volatility: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for OptionChain {
            fn default() -> Self {
                Self {
                    call_exp_date_map: Ok(Default::default()),
                    days_to_expiration: Ok(Default::default()),
                    interest_rate: Ok(Default::default()),
                    interval: Ok(Default::default()),
                    is_delayed: Ok(Default::default()),
                    is_index: Ok(Default::default()),
                    put_exp_date_map: Ok(Default::default()),
                    status: Ok(Default::default()),
                    strategy: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    underlying: Ok(Default::default()),
                    underlying_price: Ok(Default::default()),
                    volatility: Ok(Default::default()),
                }
            }
        }

        impl OptionChain {
            pub fn call_exp_date_map<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::collections::HashMap<
                            ::std::string::String,
                            super::OptionContractMap,
                        >,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.call_exp_date_map = value.try_into().map_err(|e| {
                    format!("error converting supplied value for call_exp_date_map: {e}")
                });
                self
            }
            pub fn days_to_expiration<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.days_to_expiration = value.try_into().map_err(|e| {
                    format!("error converting supplied value for days_to_expiration: {e}")
                });
                self
            }
            pub fn interest_rate<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.interest_rate = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for interest_rate: {e}"));
                self
            }
            pub fn interval<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.interval = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for interval: {e}"));
                self
            }
            pub fn is_delayed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.is_delayed = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for is_delayed: {e}"));
                self
            }
            pub fn is_index<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.is_index = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for is_index: {e}"));
                self
            }
            pub fn put_exp_date_map<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::collections::HashMap<
                            ::std::string::String,
                            super::OptionContractMap,
                        >,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.put_exp_date_map = value.try_into().map_err(|e| {
                    format!("error converting supplied value for put_exp_date_map: {e}")
                });
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {e}"));
                self
            }
            pub fn strategy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::OptionChainStrategy>>,
                T::Error: ::std::fmt::Display,
            {
                self.strategy = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for strategy: {e}"));
                self
            }
            pub fn symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for symbol: {e}"));
                self
            }
            pub fn underlying<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Underlying>>,
                T::Error: ::std::fmt::Display,
            {
                self.underlying = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for underlying: {e}"));
                self
            }
            pub fn underlying_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.underlying_price = value.try_into().map_err(|e| {
                    format!("error converting supplied value for underlying_price: {e}")
                });
                self
            }
            pub fn volatility<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.volatility = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for volatility: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<OptionChain> for super::OptionChain {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OptionChain,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    call_exp_date_map: value.call_exp_date_map?,
                    days_to_expiration: value.days_to_expiration?,
                    interest_rate: value.interest_rate?,
                    interval: value.interval?,
                    is_delayed: value.is_delayed?,
                    is_index: value.is_index?,
                    put_exp_date_map: value.put_exp_date_map?,
                    status: value.status?,
                    strategy: value.strategy?,
                    symbol: value.symbol?,
                    underlying: value.underlying?,
                    underlying_price: value.underlying_price?,
                    volatility: value.volatility?,
                })
            }
        }

        impl ::std::convert::From<super::OptionChain> for OptionChain {
            fn from(value: super::OptionChain) -> Self {
                Self {
                    call_exp_date_map: Ok(value.call_exp_date_map),
                    days_to_expiration: Ok(value.days_to_expiration),
                    interest_rate: Ok(value.interest_rate),
                    interval: Ok(value.interval),
                    is_delayed: Ok(value.is_delayed),
                    is_index: Ok(value.is_index),
                    put_exp_date_map: Ok(value.put_exp_date_map),
                    status: Ok(value.status),
                    strategy: Ok(value.strategy),
                    symbol: Ok(value.symbol),
                    underlying: Ok(value.underlying),
                    underlying_price: Ok(value.underlying_price),
                    volatility: Ok(value.volatility),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OptionContract {
            ask_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            ask_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            bid_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            bid_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            close_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            days_to_expiration:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            deliverable_note: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            delta: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            expiration_date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            expiration_type: ::std::result::Result<
                ::std::option::Option<super::ExpirationType>,
                ::std::string::String,
            >,
            gamma: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            high_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            intrinsic_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            is_in_the_money:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            is_index_option:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            is_mini: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            is_non_standard:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            is_penny_pilot:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            last_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            last_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            last_trading_day:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            low_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mark_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mark_percent_change:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mark_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            multiplier: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            open_interest: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            open_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            option_deliverables_list: ::std::result::Result<
                ::std::vec::Vec<super::OptionDeliverables>,
                ::std::string::String,
            >,
            option_root: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            percent_change:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            put_call: ::std::result::Result<
                ::std::option::Option<super::OptionContractPutCall>,
                ::std::string::String,
            >,
            quote_time_in_long:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            rho: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            settlement_type: ::std::result::Result<
                ::std::option::Option<super::SettlementType>,
                ::std::string::String,
            >,
            strike_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            theoretical_option_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            theoretical_volatility:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            theta: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            time_value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            total_volume: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            trade_date: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            trade_time_in_long:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            vega: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            volatility: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for OptionContract {
            fn default() -> Self {
                Self {
                    ask_price: Ok(Default::default()),
                    ask_size: Ok(Default::default()),
                    bid_price: Ok(Default::default()),
                    bid_size: Ok(Default::default()),
                    close_price: Ok(Default::default()),
                    days_to_expiration: Ok(Default::default()),
                    deliverable_note: Ok(Default::default()),
                    delta: Ok(Default::default()),
                    description: Ok(Default::default()),
                    exchange_name: Ok(Default::default()),
                    expiration_date: Ok(Default::default()),
                    expiration_type: Ok(Default::default()),
                    gamma: Ok(Default::default()),
                    high_price: Ok(Default::default()),
                    intrinsic_value: Ok(Default::default()),
                    is_in_the_money: Ok(Default::default()),
                    is_index_option: Ok(Default::default()),
                    is_mini: Ok(Default::default()),
                    is_non_standard: Ok(Default::default()),
                    is_penny_pilot: Ok(Default::default()),
                    last_price: Ok(Default::default()),
                    last_size: Ok(Default::default()),
                    last_trading_day: Ok(Default::default()),
                    low_price: Ok(Default::default()),
                    mark_change: Ok(Default::default()),
                    mark_percent_change: Ok(Default::default()),
                    mark_price: Ok(Default::default()),
                    multiplier: Ok(Default::default()),
                    net_change: Ok(Default::default()),
                    open_interest: Ok(Default::default()),
                    open_price: Ok(Default::default()),
                    option_deliverables_list: Ok(Default::default()),
                    option_root: Ok(Default::default()),
                    percent_change: Ok(Default::default()),
                    put_call: Ok(Default::default()),
                    quote_time_in_long: Ok(Default::default()),
                    rho: Ok(Default::default()),
                    settlement_type: Ok(Default::default()),
                    strike_price: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    theoretical_option_value: Ok(Default::default()),
                    theoretical_volatility: Ok(Default::default()),
                    theta: Ok(Default::default()),
                    time_value: Ok(Default::default()),
                    total_volume: Ok(Default::default()),
                    trade_date: Ok(Default::default()),
                    trade_time_in_long: Ok(Default::default()),
                    vega: Ok(Default::default()),
                    volatility: Ok(Default::default()),
                }
            }
        }

        impl OptionContract {
            pub fn ask_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_price: {e}"));
                self
            }
            pub fn ask_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_size: {e}"));
                self
            }
            pub fn bid_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_price: {e}"));
                self
            }
            pub fn bid_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_size: {e}"));
                self
            }
            pub fn close_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.close_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for close_price: {e}"));
                self
            }
            pub fn days_to_expiration<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.days_to_expiration = value.try_into().map_err(|e| {
                    format!("error converting supplied value for days_to_expiration: {e}")
                });
                self
            }
            pub fn deliverable_note<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.deliverable_note = value.try_into().map_err(|e| {
                    format!("error converting supplied value for deliverable_note: {e}")
                });
                self
            }
            pub fn delta<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.delta = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for delta: {e}"));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {e}"));
                self
            }
            pub fn exchange_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange_name: {e}"));
                self
            }
            pub fn expiration_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.expiration_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for expiration_date: {e}")
                });
                self
            }
            pub fn expiration_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ExpirationType>>,
                T::Error: ::std::fmt::Display,
            {
                self.expiration_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for expiration_type: {e}")
                });
                self
            }
            pub fn gamma<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.gamma = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for gamma: {e}"));
                self
            }
            pub fn high_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.high_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for high_price: {e}"));
                self
            }
            pub fn intrinsic_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.intrinsic_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for intrinsic_value: {e}")
                });
                self
            }
            pub fn is_in_the_money<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.is_in_the_money = value.try_into().map_err(|e| {
                    format!("error converting supplied value for is_in_the_money: {e}")
                });
                self
            }
            pub fn is_index_option<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.is_index_option = value.try_into().map_err(|e| {
                    format!("error converting supplied value for is_index_option: {e}")
                });
                self
            }
            pub fn is_mini<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.is_mini = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for is_mini: {e}"));
                self
            }
            pub fn is_non_standard<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.is_non_standard = value.try_into().map_err(|e| {
                    format!("error converting supplied value for is_non_standard: {e}")
                });
                self
            }
            pub fn is_penny_pilot<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.is_penny_pilot = value.try_into().map_err(|e| {
                    format!("error converting supplied value for is_penny_pilot: {e}")
                });
                self
            }
            pub fn last_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_price: {e}"));
                self
            }
            pub fn last_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_size: {e}"));
                self
            }
            pub fn last_trading_day<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_trading_day = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_trading_day: {e}")
                });
                self
            }
            pub fn low_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.low_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for low_price: {e}"));
                self
            }
            pub fn mark_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mark_change = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mark_change: {e}"));
                self
            }
            pub fn mark_percent_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mark_percent_change = value.try_into().map_err(|e| {
                    format!("error converting supplied value for mark_percent_change: {e}")
                });
                self
            }
            pub fn mark_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mark_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mark_price: {e}"));
                self
            }
            pub fn multiplier<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.multiplier = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for multiplier: {e}"));
                self
            }
            pub fn net_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.net_change = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for net_change: {e}"));
                self
            }
            pub fn open_interest<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.open_interest = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for open_interest: {e}"));
                self
            }
            pub fn open_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.open_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for open_price: {e}"));
                self
            }
            pub fn option_deliverables_list<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::OptionDeliverables>>,
                T::Error: ::std::fmt::Display,
            {
                self.option_deliverables_list = value.try_into().map_err(|e| {
                    format!("error converting supplied value for option_deliverables_list: {e}")
                });
                self
            }
            pub fn option_root<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.option_root = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for option_root: {e}"));
                self
            }
            pub fn percent_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.percent_change = value.try_into().map_err(|e| {
                    format!("error converting supplied value for percent_change: {e}")
                });
                self
            }
            pub fn put_call<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::OptionContractPutCall>>,
                T::Error: ::std::fmt::Display,
            {
                self.put_call = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for put_call: {e}"));
                self
            }
            pub fn quote_time_in_long<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.quote_time_in_long = value.try_into().map_err(|e| {
                    format!("error converting supplied value for quote_time_in_long: {e}")
                });
                self
            }
            pub fn rho<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.rho = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rho: {e}"));
                self
            }
            pub fn settlement_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::SettlementType>>,
                T::Error: ::std::fmt::Display,
            {
                self.settlement_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for settlement_type: {e}")
                });
                self
            }
            pub fn strike_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.strike_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for strike_price: {e}"));
                self
            }
            pub fn symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for symbol: {e}"));
                self
            }
            pub fn theoretical_option_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.theoretical_option_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for theoretical_option_value: {e}")
                });
                self
            }
            pub fn theoretical_volatility<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.theoretical_volatility = value.try_into().map_err(|e| {
                    format!("error converting supplied value for theoretical_volatility: {e}")
                });
                self
            }
            pub fn theta<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.theta = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for theta: {e}"));
                self
            }
            pub fn time_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.time_value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for time_value: {e}"));
                self
            }
            pub fn total_volume<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.total_volume = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_volume: {e}"));
                self
            }
            pub fn trade_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.trade_date = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for trade_date: {e}"));
                self
            }
            pub fn trade_time_in_long<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.trade_time_in_long = value.try_into().map_err(|e| {
                    format!("error converting supplied value for trade_time_in_long: {e}")
                });
                self
            }
            pub fn vega<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.vega = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for vega: {e}"));
                self
            }
            pub fn volatility<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.volatility = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for volatility: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<OptionContract> for super::OptionContract {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OptionContract,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    ask_price: value.ask_price?,
                    ask_size: value.ask_size?,
                    bid_price: value.bid_price?,
                    bid_size: value.bid_size?,
                    close_price: value.close_price?,
                    days_to_expiration: value.days_to_expiration?,
                    deliverable_note: value.deliverable_note?,
                    delta: value.delta?,
                    description: value.description?,
                    exchange_name: value.exchange_name?,
                    expiration_date: value.expiration_date?,
                    expiration_type: value.expiration_type?,
                    gamma: value.gamma?,
                    high_price: value.high_price?,
                    intrinsic_value: value.intrinsic_value?,
                    is_in_the_money: value.is_in_the_money?,
                    is_index_option: value.is_index_option?,
                    is_mini: value.is_mini?,
                    is_non_standard: value.is_non_standard?,
                    is_penny_pilot: value.is_penny_pilot?,
                    last_price: value.last_price?,
                    last_size: value.last_size?,
                    last_trading_day: value.last_trading_day?,
                    low_price: value.low_price?,
                    mark_change: value.mark_change?,
                    mark_percent_change: value.mark_percent_change?,
                    mark_price: value.mark_price?,
                    multiplier: value.multiplier?,
                    net_change: value.net_change?,
                    open_interest: value.open_interest?,
                    open_price: value.open_price?,
                    option_deliverables_list: value.option_deliverables_list?,
                    option_root: value.option_root?,
                    percent_change: value.percent_change?,
                    put_call: value.put_call?,
                    quote_time_in_long: value.quote_time_in_long?,
                    rho: value.rho?,
                    settlement_type: value.settlement_type?,
                    strike_price: value.strike_price?,
                    symbol: value.symbol?,
                    theoretical_option_value: value.theoretical_option_value?,
                    theoretical_volatility: value.theoretical_volatility?,
                    theta: value.theta?,
                    time_value: value.time_value?,
                    total_volume: value.total_volume?,
                    trade_date: value.trade_date?,
                    trade_time_in_long: value.trade_time_in_long?,
                    vega: value.vega?,
                    volatility: value.volatility?,
                })
            }
        }

        impl ::std::convert::From<super::OptionContract> for OptionContract {
            fn from(value: super::OptionContract) -> Self {
                Self {
                    ask_price: Ok(value.ask_price),
                    ask_size: Ok(value.ask_size),
                    bid_price: Ok(value.bid_price),
                    bid_size: Ok(value.bid_size),
                    close_price: Ok(value.close_price),
                    days_to_expiration: Ok(value.days_to_expiration),
                    deliverable_note: Ok(value.deliverable_note),
                    delta: Ok(value.delta),
                    description: Ok(value.description),
                    exchange_name: Ok(value.exchange_name),
                    expiration_date: Ok(value.expiration_date),
                    expiration_type: Ok(value.expiration_type),
                    gamma: Ok(value.gamma),
                    high_price: Ok(value.high_price),
                    intrinsic_value: Ok(value.intrinsic_value),
                    is_in_the_money: Ok(value.is_in_the_money),
                    is_index_option: Ok(value.is_index_option),
                    is_mini: Ok(value.is_mini),
                    is_non_standard: Ok(value.is_non_standard),
                    is_penny_pilot: Ok(value.is_penny_pilot),
                    last_price: Ok(value.last_price),
                    last_size: Ok(value.last_size),
                    last_trading_day: Ok(value.last_trading_day),
                    low_price: Ok(value.low_price),
                    mark_change: Ok(value.mark_change),
                    mark_percent_change: Ok(value.mark_percent_change),
                    mark_price: Ok(value.mark_price),
                    multiplier: Ok(value.multiplier),
                    net_change: Ok(value.net_change),
                    open_interest: Ok(value.open_interest),
                    open_price: Ok(value.open_price),
                    option_deliverables_list: Ok(value.option_deliverables_list),
                    option_root: Ok(value.option_root),
                    percent_change: Ok(value.percent_change),
                    put_call: Ok(value.put_call),
                    quote_time_in_long: Ok(value.quote_time_in_long),
                    rho: Ok(value.rho),
                    settlement_type: Ok(value.settlement_type),
                    strike_price: Ok(value.strike_price),
                    symbol: Ok(value.symbol),
                    theoretical_option_value: Ok(value.theoretical_option_value),
                    theoretical_volatility: Ok(value.theoretical_volatility),
                    theta: Ok(value.theta),
                    time_value: Ok(value.time_value),
                    total_volume: Ok(value.total_volume),
                    trade_date: Ok(value.trade_date),
                    trade_time_in_long: Ok(value.trade_time_in_long),
                    vega: Ok(value.vega),
                    volatility: Ok(value.volatility),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OptionDeliverables {
            asset_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            currency_type: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            deliverable_units:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for OptionDeliverables {
            fn default() -> Self {
                Self {
                    asset_type: Ok(Default::default()),
                    currency_type: Ok(Default::default()),
                    deliverable_units: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                }
            }
        }

        impl OptionDeliverables {
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.asset_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for asset_type: {e}"));
                self
            }
            pub fn currency_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.currency_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for currency_type: {e}"));
                self
            }
            pub fn deliverable_units<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.deliverable_units = value.try_into().map_err(|e| {
                    format!("error converting supplied value for deliverable_units: {e}")
                });
                self
            }
            pub fn symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for symbol: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<OptionDeliverables> for super::OptionDeliverables {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OptionDeliverables,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_type: value.asset_type?,
                    currency_type: value.currency_type?,
                    deliverable_units: value.deliverable_units?,
                    symbol: value.symbol?,
                })
            }
        }

        impl ::std::convert::From<super::OptionDeliverables> for OptionDeliverables {
            fn from(value: super::OptionDeliverables) -> Self {
                Self {
                    asset_type: Ok(value.asset_type),
                    currency_type: Ok(value.currency_type),
                    deliverable_units: Ok(value.deliverable_units),
                    symbol: Ok(value.symbol),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OptionResponse {
            asset_main_type: ::std::result::Result<
                ::std::option::Option<super::AssetMainType>,
                ::std::string::String,
            >,
            quote: ::std::result::Result<
                ::std::option::Option<super::QuoteOption>,
                ::std::string::String,
            >,
            realtime: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            reference: ::std::result::Result<
                ::std::option::Option<super::ReferenceOption>,
                ::std::string::String,
            >,
            ssid: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for OptionResponse {
            fn default() -> Self {
                Self {
                    asset_main_type: Ok(Default::default()),
                    quote: Ok(Default::default()),
                    realtime: Ok(Default::default()),
                    reference: Ok(Default::default()),
                    ssid: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                }
            }
        }

        impl OptionResponse {
            pub fn asset_main_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::AssetMainType>>,
                T::Error: ::std::fmt::Display,
            {
                self.asset_main_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for asset_main_type: {e}")
                });
                self
            }
            pub fn quote<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::QuoteOption>>,
                T::Error: ::std::fmt::Display,
            {
                self.quote = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quote: {e}"));
                self
            }
            pub fn realtime<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.realtime = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for realtime: {e}"));
                self
            }
            pub fn reference<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ReferenceOption>>,
                T::Error: ::std::fmt::Display,
            {
                self.reference = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for reference: {e}"));
                self
            }
            pub fn ssid<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ssid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ssid: {e}"));
                self
            }
            pub fn symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for symbol: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<OptionResponse> for super::OptionResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OptionResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_main_type: value.asset_main_type?,
                    quote: value.quote?,
                    realtime: value.realtime?,
                    reference: value.reference?,
                    ssid: value.ssid?,
                    symbol: value.symbol?,
                })
            }
        }

        impl ::std::convert::From<super::OptionResponse> for OptionResponse {
            fn from(value: super::OptionResponse) -> Self {
                Self {
                    asset_main_type: Ok(value.asset_main_type),
                    quote: Ok(value.quote),
                    realtime: Ok(value.realtime),
                    reference: Ok(value.reference),
                    ssid: Ok(value.ssid),
                    symbol: Ok(value.symbol),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct QuoteEquity {
            ask_mic_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ask_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            ask_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            ask_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            bid_mic_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            bid_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            bid_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            bid_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            close_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            high_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            last_mic_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            last_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            last_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            low_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mark: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mark_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mark_percent_change:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            net_percent_change:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            open_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            quote_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            security_status: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            total_volume: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            trade_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            volatility: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            x52week_high: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            x52week_low: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for QuoteEquity {
            fn default() -> Self {
                Self {
                    ask_mic_id: Ok(Default::default()),
                    ask_price: Ok(Default::default()),
                    ask_size: Ok(Default::default()),
                    ask_time: Ok(Default::default()),
                    bid_mic_id: Ok(Default::default()),
                    bid_price: Ok(Default::default()),
                    bid_size: Ok(Default::default()),
                    bid_time: Ok(Default::default()),
                    close_price: Ok(Default::default()),
                    high_price: Ok(Default::default()),
                    last_mic_id: Ok(Default::default()),
                    last_price: Ok(Default::default()),
                    last_size: Ok(Default::default()),
                    low_price: Ok(Default::default()),
                    mark: Ok(Default::default()),
                    mark_change: Ok(Default::default()),
                    mark_percent_change: Ok(Default::default()),
                    net_change: Ok(Default::default()),
                    net_percent_change: Ok(Default::default()),
                    open_price: Ok(Default::default()),
                    quote_time: Ok(Default::default()),
                    security_status: Ok(Default::default()),
                    total_volume: Ok(Default::default()),
                    trade_time: Ok(Default::default()),
                    volatility: Ok(Default::default()),
                    x52week_high: Ok(Default::default()),
                    x52week_low: Ok(Default::default()),
                }
            }
        }

        impl QuoteEquity {
            pub fn ask_mic_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_mic_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_mic_id: {e}"));
                self
            }
            pub fn ask_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_price: {e}"));
                self
            }
            pub fn ask_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_size: {e}"));
                self
            }
            pub fn ask_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_time: {e}"));
                self
            }
            pub fn bid_mic_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_mic_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_mic_id: {e}"));
                self
            }
            pub fn bid_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_price: {e}"));
                self
            }
            pub fn bid_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_size: {e}"));
                self
            }
            pub fn bid_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_time: {e}"));
                self
            }
            pub fn close_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.close_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for close_price: {e}"));
                self
            }
            pub fn high_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.high_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for high_price: {e}"));
                self
            }
            pub fn last_mic_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_mic_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_mic_id: {e}"));
                self
            }
            pub fn last_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_price: {e}"));
                self
            }
            pub fn last_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_size: {e}"));
                self
            }
            pub fn low_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.low_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for low_price: {e}"));
                self
            }
            pub fn mark<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mark = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mark: {e}"));
                self
            }
            pub fn mark_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mark_change = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mark_change: {e}"));
                self
            }
            pub fn mark_percent_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mark_percent_change = value.try_into().map_err(|e| {
                    format!("error converting supplied value for mark_percent_change: {e}")
                });
                self
            }
            pub fn net_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.net_change = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for net_change: {e}"));
                self
            }
            pub fn net_percent_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.net_percent_change = value.try_into().map_err(|e| {
                    format!("error converting supplied value for net_percent_change: {e}")
                });
                self
            }
            pub fn open_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.open_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for open_price: {e}"));
                self
            }
            pub fn quote_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.quote_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quote_time: {e}"));
                self
            }
            pub fn security_status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.security_status = value.try_into().map_err(|e| {
                    format!("error converting supplied value for security_status: {e}")
                });
                self
            }
            pub fn total_volume<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.total_volume = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_volume: {e}"));
                self
            }
            pub fn trade_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.trade_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for trade_time: {e}"));
                self
            }
            pub fn volatility<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.volatility = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for volatility: {e}"));
                self
            }
            pub fn x52week_high<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.x52week_high = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for x52week_high: {e}"));
                self
            }
            pub fn x52week_low<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.x52week_low = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for x52week_low: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<QuoteEquity> for super::QuoteEquity {
            type Error = super::error::ConversionError;
            fn try_from(
                value: QuoteEquity,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    ask_mic_id: value.ask_mic_id?,
                    ask_price: value.ask_price?,
                    ask_size: value.ask_size?,
                    ask_time: value.ask_time?,
                    bid_mic_id: value.bid_mic_id?,
                    bid_price: value.bid_price?,
                    bid_size: value.bid_size?,
                    bid_time: value.bid_time?,
                    close_price: value.close_price?,
                    high_price: value.high_price?,
                    last_mic_id: value.last_mic_id?,
                    last_price: value.last_price?,
                    last_size: value.last_size?,
                    low_price: value.low_price?,
                    mark: value.mark?,
                    mark_change: value.mark_change?,
                    mark_percent_change: value.mark_percent_change?,
                    net_change: value.net_change?,
                    net_percent_change: value.net_percent_change?,
                    open_price: value.open_price?,
                    quote_time: value.quote_time?,
                    security_status: value.security_status?,
                    total_volume: value.total_volume?,
                    trade_time: value.trade_time?,
                    volatility: value.volatility?,
                    x52week_high: value.x52week_high?,
                    x52week_low: value.x52week_low?,
                })
            }
        }

        impl ::std::convert::From<super::QuoteEquity> for QuoteEquity {
            fn from(value: super::QuoteEquity) -> Self {
                Self {
                    ask_mic_id: Ok(value.ask_mic_id),
                    ask_price: Ok(value.ask_price),
                    ask_size: Ok(value.ask_size),
                    ask_time: Ok(value.ask_time),
                    bid_mic_id: Ok(value.bid_mic_id),
                    bid_price: Ok(value.bid_price),
                    bid_size: Ok(value.bid_size),
                    bid_time: Ok(value.bid_time),
                    close_price: Ok(value.close_price),
                    high_price: Ok(value.high_price),
                    last_mic_id: Ok(value.last_mic_id),
                    last_price: Ok(value.last_price),
                    last_size: Ok(value.last_size),
                    low_price: Ok(value.low_price),
                    mark: Ok(value.mark),
                    mark_change: Ok(value.mark_change),
                    mark_percent_change: Ok(value.mark_percent_change),
                    net_change: Ok(value.net_change),
                    net_percent_change: Ok(value.net_percent_change),
                    open_price: Ok(value.open_price),
                    quote_time: Ok(value.quote_time),
                    security_status: Ok(value.security_status),
                    total_volume: Ok(value.total_volume),
                    trade_time: Ok(value.trade_time),
                    volatility: Ok(value.volatility),
                    x52week_high: Ok(value.x52week_high),
                    x52week_low: Ok(value.x52week_low),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct QuoteError {
            invalid_cusips: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            invalid_ssi_ds: ::std::result::Result<::std::vec::Vec<i64>, ::std::string::String>,
            invalid_symbols: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for QuoteError {
            fn default() -> Self {
                Self {
                    invalid_cusips: Ok(Default::default()),
                    invalid_ssi_ds: Ok(Default::default()),
                    invalid_symbols: Ok(Default::default()),
                }
            }
        }

        impl QuoteError {
            pub fn invalid_cusips<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.invalid_cusips = value.try_into().map_err(|e| {
                    format!("error converting supplied value for invalid_cusips: {e}")
                });
                self
            }
            pub fn invalid_ssi_ds<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.invalid_ssi_ds = value.try_into().map_err(|e| {
                    format!("error converting supplied value for invalid_ssi_ds: {e}")
                });
                self
            }
            pub fn invalid_symbols<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.invalid_symbols = value.try_into().map_err(|e| {
                    format!("error converting supplied value for invalid_symbols: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<QuoteError> for super::QuoteError {
            type Error = super::error::ConversionError;
            fn try_from(
                value: QuoteError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    invalid_cusips: value.invalid_cusips?,
                    invalid_ssi_ds: value.invalid_ssi_ds?,
                    invalid_symbols: value.invalid_symbols?,
                })
            }
        }

        impl ::std::convert::From<super::QuoteError> for QuoteError {
            fn from(value: super::QuoteError) -> Self {
                Self {
                    invalid_cusips: Ok(value.invalid_cusips),
                    invalid_ssi_ds: Ok(value.invalid_ssi_ds),
                    invalid_symbols: Ok(value.invalid_symbols),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct QuoteForex {
            ask_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            ask_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            bid_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            bid_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            close_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            high_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            last_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            last_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            low_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mark: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            net_percent_change:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            open_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            quote_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            security_status: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            tick: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            tick_amount: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            total_volume: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            trade_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            x52week_high: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            x52week_low: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for QuoteForex {
            fn default() -> Self {
                Self {
                    ask_price: Ok(Default::default()),
                    ask_size: Ok(Default::default()),
                    bid_price: Ok(Default::default()),
                    bid_size: Ok(Default::default()),
                    close_price: Ok(Default::default()),
                    high_price: Ok(Default::default()),
                    last_price: Ok(Default::default()),
                    last_size: Ok(Default::default()),
                    low_price: Ok(Default::default()),
                    mark: Ok(Default::default()),
                    net_change: Ok(Default::default()),
                    net_percent_change: Ok(Default::default()),
                    open_price: Ok(Default::default()),
                    quote_time: Ok(Default::default()),
                    security_status: Ok(Default::default()),
                    tick: Ok(Default::default()),
                    tick_amount: Ok(Default::default()),
                    total_volume: Ok(Default::default()),
                    trade_time: Ok(Default::default()),
                    x52week_high: Ok(Default::default()),
                    x52week_low: Ok(Default::default()),
                }
            }
        }

        impl QuoteForex {
            pub fn ask_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_price: {e}"));
                self
            }
            pub fn ask_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_size: {e}"));
                self
            }
            pub fn bid_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_price: {e}"));
                self
            }
            pub fn bid_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_size: {e}"));
                self
            }
            pub fn close_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.close_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for close_price: {e}"));
                self
            }
            pub fn high_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.high_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for high_price: {e}"));
                self
            }
            pub fn last_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_price: {e}"));
                self
            }
            pub fn last_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_size: {e}"));
                self
            }
            pub fn low_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.low_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for low_price: {e}"));
                self
            }
            pub fn mark<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mark = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mark: {e}"));
                self
            }
            pub fn net_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.net_change = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for net_change: {e}"));
                self
            }
            pub fn net_percent_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.net_percent_change = value.try_into().map_err(|e| {
                    format!("error converting supplied value for net_percent_change: {e}")
                });
                self
            }
            pub fn open_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.open_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for open_price: {e}"));
                self
            }
            pub fn quote_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.quote_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quote_time: {e}"));
                self
            }
            pub fn security_status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.security_status = value.try_into().map_err(|e| {
                    format!("error converting supplied value for security_status: {e}")
                });
                self
            }
            pub fn tick<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.tick = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tick: {e}"));
                self
            }
            pub fn tick_amount<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.tick_amount = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tick_amount: {e}"));
                self
            }
            pub fn total_volume<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.total_volume = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_volume: {e}"));
                self
            }
            pub fn trade_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.trade_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for trade_time: {e}"));
                self
            }
            pub fn x52week_high<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.x52week_high = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for x52week_high: {e}"));
                self
            }
            pub fn x52week_low<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.x52week_low = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for x52week_low: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<QuoteForex> for super::QuoteForex {
            type Error = super::error::ConversionError;
            fn try_from(
                value: QuoteForex,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    ask_price: value.ask_price?,
                    ask_size: value.ask_size?,
                    bid_price: value.bid_price?,
                    bid_size: value.bid_size?,
                    close_price: value.close_price?,
                    high_price: value.high_price?,
                    last_price: value.last_price?,
                    last_size: value.last_size?,
                    low_price: value.low_price?,
                    mark: value.mark?,
                    net_change: value.net_change?,
                    net_percent_change: value.net_percent_change?,
                    open_price: value.open_price?,
                    quote_time: value.quote_time?,
                    security_status: value.security_status?,
                    tick: value.tick?,
                    tick_amount: value.tick_amount?,
                    total_volume: value.total_volume?,
                    trade_time: value.trade_time?,
                    x52week_high: value.x52week_high?,
                    x52week_low: value.x52week_low?,
                })
            }
        }

        impl ::std::convert::From<super::QuoteForex> for QuoteForex {
            fn from(value: super::QuoteForex) -> Self {
                Self {
                    ask_price: Ok(value.ask_price),
                    ask_size: Ok(value.ask_size),
                    bid_price: Ok(value.bid_price),
                    bid_size: Ok(value.bid_size),
                    close_price: Ok(value.close_price),
                    high_price: Ok(value.high_price),
                    last_price: Ok(value.last_price),
                    last_size: Ok(value.last_size),
                    low_price: Ok(value.low_price),
                    mark: Ok(value.mark),
                    net_change: Ok(value.net_change),
                    net_percent_change: Ok(value.net_percent_change),
                    open_price: Ok(value.open_price),
                    quote_time: Ok(value.quote_time),
                    security_status: Ok(value.security_status),
                    tick: Ok(value.tick),
                    tick_amount: Ok(value.tick_amount),
                    total_volume: Ok(value.total_volume),
                    trade_time: Ok(value.trade_time),
                    x52week_high: Ok(value.x52week_high),
                    x52week_low: Ok(value.x52week_low),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct QuoteFuture {
            ask_mic_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ask_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            ask_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            ask_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            bid_mic_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            bid_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            bid_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            bid_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            close_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            future_percent_change:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            high_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            last_mic_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            last_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            last_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            low_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mark: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            open_interest: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            open_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            quote_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            quoted_in_session:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            security_status: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            settle_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            tick: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            tick_amount: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            total_volume: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            trade_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        }

        impl ::std::default::Default for QuoteFuture {
            fn default() -> Self {
                Self {
                    ask_mic_id: Ok(Default::default()),
                    ask_price: Ok(Default::default()),
                    ask_size: Ok(Default::default()),
                    ask_time: Ok(Default::default()),
                    bid_mic_id: Ok(Default::default()),
                    bid_price: Ok(Default::default()),
                    bid_size: Ok(Default::default()),
                    bid_time: Ok(Default::default()),
                    close_price: Ok(Default::default()),
                    future_percent_change: Ok(Default::default()),
                    high_price: Ok(Default::default()),
                    last_mic_id: Ok(Default::default()),
                    last_price: Ok(Default::default()),
                    last_size: Ok(Default::default()),
                    low_price: Ok(Default::default()),
                    mark: Ok(Default::default()),
                    net_change: Ok(Default::default()),
                    open_interest: Ok(Default::default()),
                    open_price: Ok(Default::default()),
                    quote_time: Ok(Default::default()),
                    quoted_in_session: Ok(Default::default()),
                    security_status: Ok(Default::default()),
                    settle_time: Ok(Default::default()),
                    tick: Ok(Default::default()),
                    tick_amount: Ok(Default::default()),
                    total_volume: Ok(Default::default()),
                    trade_time: Ok(Default::default()),
                }
            }
        }

        impl QuoteFuture {
            pub fn ask_mic_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_mic_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_mic_id: {e}"));
                self
            }
            pub fn ask_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_price: {e}"));
                self
            }
            pub fn ask_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_size: {e}"));
                self
            }
            pub fn ask_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_time: {e}"));
                self
            }
            pub fn bid_mic_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_mic_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_mic_id: {e}"));
                self
            }
            pub fn bid_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_price: {e}"));
                self
            }
            pub fn bid_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_size: {e}"));
                self
            }
            pub fn bid_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_time: {e}"));
                self
            }
            pub fn close_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.close_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for close_price: {e}"));
                self
            }
            pub fn future_percent_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.future_percent_change = value.try_into().map_err(|e| {
                    format!("error converting supplied value for future_percent_change: {e}")
                });
                self
            }
            pub fn high_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.high_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for high_price: {e}"));
                self
            }
            pub fn last_mic_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_mic_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_mic_id: {e}"));
                self
            }
            pub fn last_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_price: {e}"));
                self
            }
            pub fn last_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_size: {e}"));
                self
            }
            pub fn low_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.low_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for low_price: {e}"));
                self
            }
            pub fn mark<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mark = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mark: {e}"));
                self
            }
            pub fn net_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.net_change = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for net_change: {e}"));
                self
            }
            pub fn open_interest<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.open_interest = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for open_interest: {e}"));
                self
            }
            pub fn open_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.open_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for open_price: {e}"));
                self
            }
            pub fn quote_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.quote_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quote_time: {e}"));
                self
            }
            pub fn quoted_in_session<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.quoted_in_session = value.try_into().map_err(|e| {
                    format!("error converting supplied value for quoted_in_session: {e}")
                });
                self
            }
            pub fn security_status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.security_status = value.try_into().map_err(|e| {
                    format!("error converting supplied value for security_status: {e}")
                });
                self
            }
            pub fn settle_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.settle_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for settle_time: {e}"));
                self
            }
            pub fn tick<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.tick = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tick: {e}"));
                self
            }
            pub fn tick_amount<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.tick_amount = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tick_amount: {e}"));
                self
            }
            pub fn total_volume<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.total_volume = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_volume: {e}"));
                self
            }
            pub fn trade_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.trade_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for trade_time: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<QuoteFuture> for super::QuoteFuture {
            type Error = super::error::ConversionError;
            fn try_from(
                value: QuoteFuture,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    ask_mic_id: value.ask_mic_id?,
                    ask_price: value.ask_price?,
                    ask_size: value.ask_size?,
                    ask_time: value.ask_time?,
                    bid_mic_id: value.bid_mic_id?,
                    bid_price: value.bid_price?,
                    bid_size: value.bid_size?,
                    bid_time: value.bid_time?,
                    close_price: value.close_price?,
                    future_percent_change: value.future_percent_change?,
                    high_price: value.high_price?,
                    last_mic_id: value.last_mic_id?,
                    last_price: value.last_price?,
                    last_size: value.last_size?,
                    low_price: value.low_price?,
                    mark: value.mark?,
                    net_change: value.net_change?,
                    open_interest: value.open_interest?,
                    open_price: value.open_price?,
                    quote_time: value.quote_time?,
                    quoted_in_session: value.quoted_in_session?,
                    security_status: value.security_status?,
                    settle_time: value.settle_time?,
                    tick: value.tick?,
                    tick_amount: value.tick_amount?,
                    total_volume: value.total_volume?,
                    trade_time: value.trade_time?,
                })
            }
        }

        impl ::std::convert::From<super::QuoteFuture> for QuoteFuture {
            fn from(value: super::QuoteFuture) -> Self {
                Self {
                    ask_mic_id: Ok(value.ask_mic_id),
                    ask_price: Ok(value.ask_price),
                    ask_size: Ok(value.ask_size),
                    ask_time: Ok(value.ask_time),
                    bid_mic_id: Ok(value.bid_mic_id),
                    bid_price: Ok(value.bid_price),
                    bid_size: Ok(value.bid_size),
                    bid_time: Ok(value.bid_time),
                    close_price: Ok(value.close_price),
                    future_percent_change: Ok(value.future_percent_change),
                    high_price: Ok(value.high_price),
                    last_mic_id: Ok(value.last_mic_id),
                    last_price: Ok(value.last_price),
                    last_size: Ok(value.last_size),
                    low_price: Ok(value.low_price),
                    mark: Ok(value.mark),
                    net_change: Ok(value.net_change),
                    open_interest: Ok(value.open_interest),
                    open_price: Ok(value.open_price),
                    quote_time: Ok(value.quote_time),
                    quoted_in_session: Ok(value.quoted_in_session),
                    security_status: Ok(value.security_status),
                    settle_time: Ok(value.settle_time),
                    tick: Ok(value.tick),
                    tick_amount: Ok(value.tick_amount),
                    total_volume: Ok(value.total_volume),
                    trade_time: Ok(value.trade_time),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct QuoteFutureOption {
            ask_mic_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            ask_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            ask_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            bid_mic_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            bid_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            bid_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            close_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            high_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            last_mic_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            last_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            last_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            low_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mark: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mark_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            net_percent_change:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            open_interest: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            open_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            quote_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            security_status: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            settlemet_price:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            tick: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            tick_amount: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            total_volume: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            trade_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        }

        impl ::std::default::Default for QuoteFutureOption {
            fn default() -> Self {
                Self {
                    ask_mic_id: Ok(Default::default()),
                    ask_price: Ok(Default::default()),
                    ask_size: Ok(Default::default()),
                    bid_mic_id: Ok(Default::default()),
                    bid_price: Ok(Default::default()),
                    bid_size: Ok(Default::default()),
                    close_price: Ok(Default::default()),
                    high_price: Ok(Default::default()),
                    last_mic_id: Ok(Default::default()),
                    last_price: Ok(Default::default()),
                    last_size: Ok(Default::default()),
                    low_price: Ok(Default::default()),
                    mark: Ok(Default::default()),
                    mark_change: Ok(Default::default()),
                    net_change: Ok(Default::default()),
                    net_percent_change: Ok(Default::default()),
                    open_interest: Ok(Default::default()),
                    open_price: Ok(Default::default()),
                    quote_time: Ok(Default::default()),
                    security_status: Ok(Default::default()),
                    settlemet_price: Ok(Default::default()),
                    tick: Ok(Default::default()),
                    tick_amount: Ok(Default::default()),
                    total_volume: Ok(Default::default()),
                    trade_time: Ok(Default::default()),
                }
            }
        }

        impl QuoteFutureOption {
            pub fn ask_mic_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_mic_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_mic_id: {e}"));
                self
            }
            pub fn ask_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_price: {e}"));
                self
            }
            pub fn ask_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_size: {e}"));
                self
            }
            pub fn bid_mic_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_mic_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_mic_id: {e}"));
                self
            }
            pub fn bid_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_price: {e}"));
                self
            }
            pub fn bid_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_size: {e}"));
                self
            }
            pub fn close_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.close_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for close_price: {e}"));
                self
            }
            pub fn high_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.high_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for high_price: {e}"));
                self
            }
            pub fn last_mic_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_mic_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_mic_id: {e}"));
                self
            }
            pub fn last_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_price: {e}"));
                self
            }
            pub fn last_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_size: {e}"));
                self
            }
            pub fn low_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.low_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for low_price: {e}"));
                self
            }
            pub fn mark<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mark = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mark: {e}"));
                self
            }
            pub fn mark_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mark_change = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mark_change: {e}"));
                self
            }
            pub fn net_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.net_change = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for net_change: {e}"));
                self
            }
            pub fn net_percent_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.net_percent_change = value.try_into().map_err(|e| {
                    format!("error converting supplied value for net_percent_change: {e}")
                });
                self
            }
            pub fn open_interest<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.open_interest = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for open_interest: {e}"));
                self
            }
            pub fn open_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.open_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for open_price: {e}"));
                self
            }
            pub fn quote_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.quote_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quote_time: {e}"));
                self
            }
            pub fn security_status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.security_status = value.try_into().map_err(|e| {
                    format!("error converting supplied value for security_status: {e}")
                });
                self
            }
            pub fn settlemet_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.settlemet_price = value.try_into().map_err(|e| {
                    format!("error converting supplied value for settlemet_price: {e}")
                });
                self
            }
            pub fn tick<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.tick = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tick: {e}"));
                self
            }
            pub fn tick_amount<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.tick_amount = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tick_amount: {e}"));
                self
            }
            pub fn total_volume<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.total_volume = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_volume: {e}"));
                self
            }
            pub fn trade_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.trade_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for trade_time: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<QuoteFutureOption> for super::QuoteFutureOption {
            type Error = super::error::ConversionError;
            fn try_from(
                value: QuoteFutureOption,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    ask_mic_id: value.ask_mic_id?,
                    ask_price: value.ask_price?,
                    ask_size: value.ask_size?,
                    bid_mic_id: value.bid_mic_id?,
                    bid_price: value.bid_price?,
                    bid_size: value.bid_size?,
                    close_price: value.close_price?,
                    high_price: value.high_price?,
                    last_mic_id: value.last_mic_id?,
                    last_price: value.last_price?,
                    last_size: value.last_size?,
                    low_price: value.low_price?,
                    mark: value.mark?,
                    mark_change: value.mark_change?,
                    net_change: value.net_change?,
                    net_percent_change: value.net_percent_change?,
                    open_interest: value.open_interest?,
                    open_price: value.open_price?,
                    quote_time: value.quote_time?,
                    security_status: value.security_status?,
                    settlemet_price: value.settlemet_price?,
                    tick: value.tick?,
                    tick_amount: value.tick_amount?,
                    total_volume: value.total_volume?,
                    trade_time: value.trade_time?,
                })
            }
        }

        impl ::std::convert::From<super::QuoteFutureOption> for QuoteFutureOption {
            fn from(value: super::QuoteFutureOption) -> Self {
                Self {
                    ask_mic_id: Ok(value.ask_mic_id),
                    ask_price: Ok(value.ask_price),
                    ask_size: Ok(value.ask_size),
                    bid_mic_id: Ok(value.bid_mic_id),
                    bid_price: Ok(value.bid_price),
                    bid_size: Ok(value.bid_size),
                    close_price: Ok(value.close_price),
                    high_price: Ok(value.high_price),
                    last_mic_id: Ok(value.last_mic_id),
                    last_price: Ok(value.last_price),
                    last_size: Ok(value.last_size),
                    low_price: Ok(value.low_price),
                    mark: Ok(value.mark),
                    mark_change: Ok(value.mark_change),
                    net_change: Ok(value.net_change),
                    net_percent_change: Ok(value.net_percent_change),
                    open_interest: Ok(value.open_interest),
                    open_price: Ok(value.open_price),
                    quote_time: Ok(value.quote_time),
                    security_status: Ok(value.security_status),
                    settlemet_price: Ok(value.settlemet_price),
                    tick: Ok(value.tick),
                    tick_amount: Ok(value.tick_amount),
                    total_volume: Ok(value.total_volume),
                    trade_time: Ok(value.trade_time),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct QuoteIndex {
            close_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            high_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            last_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            low_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            net_percent_change:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            open_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            security_status: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            total_volume: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            trade_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            x52week_high: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            x52week_low: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for QuoteIndex {
            fn default() -> Self {
                Self {
                    close_price: Ok(Default::default()),
                    high_price: Ok(Default::default()),
                    last_price: Ok(Default::default()),
                    low_price: Ok(Default::default()),
                    net_change: Ok(Default::default()),
                    net_percent_change: Ok(Default::default()),
                    open_price: Ok(Default::default()),
                    security_status: Ok(Default::default()),
                    total_volume: Ok(Default::default()),
                    trade_time: Ok(Default::default()),
                    x52week_high: Ok(Default::default()),
                    x52week_low: Ok(Default::default()),
                }
            }
        }

        impl QuoteIndex {
            pub fn close_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.close_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for close_price: {e}"));
                self
            }
            pub fn high_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.high_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for high_price: {e}"));
                self
            }
            pub fn last_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_price: {e}"));
                self
            }
            pub fn low_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.low_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for low_price: {e}"));
                self
            }
            pub fn net_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.net_change = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for net_change: {e}"));
                self
            }
            pub fn net_percent_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.net_percent_change = value.try_into().map_err(|e| {
                    format!("error converting supplied value for net_percent_change: {e}")
                });
                self
            }
            pub fn open_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.open_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for open_price: {e}"));
                self
            }
            pub fn security_status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.security_status = value.try_into().map_err(|e| {
                    format!("error converting supplied value for security_status: {e}")
                });
                self
            }
            pub fn total_volume<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.total_volume = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_volume: {e}"));
                self
            }
            pub fn trade_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.trade_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for trade_time: {e}"));
                self
            }
            pub fn x52week_high<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.x52week_high = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for x52week_high: {e}"));
                self
            }
            pub fn x52week_low<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.x52week_low = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for x52week_low: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<QuoteIndex> for super::QuoteIndex {
            type Error = super::error::ConversionError;
            fn try_from(
                value: QuoteIndex,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    close_price: value.close_price?,
                    high_price: value.high_price?,
                    last_price: value.last_price?,
                    low_price: value.low_price?,
                    net_change: value.net_change?,
                    net_percent_change: value.net_percent_change?,
                    open_price: value.open_price?,
                    security_status: value.security_status?,
                    total_volume: value.total_volume?,
                    trade_time: value.trade_time?,
                    x52week_high: value.x52week_high?,
                    x52week_low: value.x52week_low?,
                })
            }
        }

        impl ::std::convert::From<super::QuoteIndex> for QuoteIndex {
            fn from(value: super::QuoteIndex) -> Self {
                Self {
                    close_price: Ok(value.close_price),
                    high_price: Ok(value.high_price),
                    last_price: Ok(value.last_price),
                    low_price: Ok(value.low_price),
                    net_change: Ok(value.net_change),
                    net_percent_change: Ok(value.net_percent_change),
                    open_price: Ok(value.open_price),
                    security_status: Ok(value.security_status),
                    total_volume: Ok(value.total_volume),
                    trade_time: Ok(value.trade_time),
                    x52week_high: Ok(value.x52week_high),
                    x52week_low: Ok(value.x52week_low),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct QuoteMutualFund {
            close_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            n_av: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            net_percent_change:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            security_status: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            total_volume: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            trade_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            x52week_high: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            x52week_low: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for QuoteMutualFund {
            fn default() -> Self {
                Self {
                    close_price: Ok(Default::default()),
                    n_av: Ok(Default::default()),
                    net_change: Ok(Default::default()),
                    net_percent_change: Ok(Default::default()),
                    security_status: Ok(Default::default()),
                    total_volume: Ok(Default::default()),
                    trade_time: Ok(Default::default()),
                    x52week_high: Ok(Default::default()),
                    x52week_low: Ok(Default::default()),
                }
            }
        }

        impl QuoteMutualFund {
            pub fn close_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.close_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for close_price: {e}"));
                self
            }
            pub fn n_av<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.n_av = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for n_av: {e}"));
                self
            }
            pub fn net_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.net_change = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for net_change: {e}"));
                self
            }
            pub fn net_percent_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.net_percent_change = value.try_into().map_err(|e| {
                    format!("error converting supplied value for net_percent_change: {e}")
                });
                self
            }
            pub fn security_status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.security_status = value.try_into().map_err(|e| {
                    format!("error converting supplied value for security_status: {e}")
                });
                self
            }
            pub fn total_volume<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.total_volume = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_volume: {e}"));
                self
            }
            pub fn trade_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.trade_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for trade_time: {e}"));
                self
            }
            pub fn x52week_high<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.x52week_high = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for x52week_high: {e}"));
                self
            }
            pub fn x52week_low<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.x52week_low = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for x52week_low: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<QuoteMutualFund> for super::QuoteMutualFund {
            type Error = super::error::ConversionError;
            fn try_from(
                value: QuoteMutualFund,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    close_price: value.close_price?,
                    n_av: value.n_av?,
                    net_change: value.net_change?,
                    net_percent_change: value.net_percent_change?,
                    security_status: value.security_status?,
                    total_volume: value.total_volume?,
                    trade_time: value.trade_time?,
                    x52week_high: value.x52week_high?,
                    x52week_low: value.x52week_low?,
                })
            }
        }

        impl ::std::convert::From<super::QuoteMutualFund> for QuoteMutualFund {
            fn from(value: super::QuoteMutualFund) -> Self {
                Self {
                    close_price: Ok(value.close_price),
                    n_av: Ok(value.n_av),
                    net_change: Ok(value.net_change),
                    net_percent_change: Ok(value.net_percent_change),
                    security_status: Ok(value.security_status),
                    total_volume: Ok(value.total_volume),
                    trade_time: Ok(value.trade_time),
                    x52week_high: Ok(value.x52week_high),
                    x52week_low: Ok(value.x52week_low),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct QuoteOption {
            ask_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            ask_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            bid_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            bid_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            close_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            delta: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            gamma: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            high_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            implied_yield: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            ind_ask_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            ind_bid_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            ind_quote_time:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            last_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            last_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            low_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mark: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mark_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mark_percent_change:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            money_intrinsic_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            net_percent_change:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            open_interest: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            open_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            quote_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            rho: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            security_status: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            theoretical_option_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            theta: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            time_value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            total_volume: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            trade_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            underlying_price:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            vega: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            volatility: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            x52week_high: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            x52week_low: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for QuoteOption {
            fn default() -> Self {
                Self {
                    ask_price: Ok(Default::default()),
                    ask_size: Ok(Default::default()),
                    bid_price: Ok(Default::default()),
                    bid_size: Ok(Default::default()),
                    close_price: Ok(Default::default()),
                    delta: Ok(Default::default()),
                    gamma: Ok(Default::default()),
                    high_price: Ok(Default::default()),
                    implied_yield: Ok(Default::default()),
                    ind_ask_price: Ok(Default::default()),
                    ind_bid_price: Ok(Default::default()),
                    ind_quote_time: Ok(Default::default()),
                    last_price: Ok(Default::default()),
                    last_size: Ok(Default::default()),
                    low_price: Ok(Default::default()),
                    mark: Ok(Default::default()),
                    mark_change: Ok(Default::default()),
                    mark_percent_change: Ok(Default::default()),
                    money_intrinsic_value: Ok(Default::default()),
                    net_change: Ok(Default::default()),
                    net_percent_change: Ok(Default::default()),
                    open_interest: Ok(Default::default()),
                    open_price: Ok(Default::default()),
                    quote_time: Ok(Default::default()),
                    rho: Ok(Default::default()),
                    security_status: Ok(Default::default()),
                    theoretical_option_value: Ok(Default::default()),
                    theta: Ok(Default::default()),
                    time_value: Ok(Default::default()),
                    total_volume: Ok(Default::default()),
                    trade_time: Ok(Default::default()),
                    underlying_price: Ok(Default::default()),
                    vega: Ok(Default::default()),
                    volatility: Ok(Default::default()),
                    x52week_high: Ok(Default::default()),
                    x52week_low: Ok(Default::default()),
                }
            }
        }

        impl QuoteOption {
            pub fn ask_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_price: {e}"));
                self
            }
            pub fn ask_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_size: {e}"));
                self
            }
            pub fn bid_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_price: {e}"));
                self
            }
            pub fn bid_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_size: {e}"));
                self
            }
            pub fn close_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.close_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for close_price: {e}"));
                self
            }
            pub fn delta<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.delta = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for delta: {e}"));
                self
            }
            pub fn gamma<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.gamma = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for gamma: {e}"));
                self
            }
            pub fn high_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.high_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for high_price: {e}"));
                self
            }
            pub fn implied_yield<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.implied_yield = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for implied_yield: {e}"));
                self
            }
            pub fn ind_ask_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ind_ask_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ind_ask_price: {e}"));
                self
            }
            pub fn ind_bid_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ind_bid_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ind_bid_price: {e}"));
                self
            }
            pub fn ind_quote_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ind_quote_time = value.try_into().map_err(|e| {
                    format!("error converting supplied value for ind_quote_time: {e}")
                });
                self
            }
            pub fn last_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_price: {e}"));
                self
            }
            pub fn last_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_size: {e}"));
                self
            }
            pub fn low_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.low_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for low_price: {e}"));
                self
            }
            pub fn mark<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mark = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mark: {e}"));
                self
            }
            pub fn mark_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mark_change = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mark_change: {e}"));
                self
            }
            pub fn mark_percent_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mark_percent_change = value.try_into().map_err(|e| {
                    format!("error converting supplied value for mark_percent_change: {e}")
                });
                self
            }
            pub fn money_intrinsic_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.money_intrinsic_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for money_intrinsic_value: {e}")
                });
                self
            }
            pub fn net_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.net_change = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for net_change: {e}"));
                self
            }
            pub fn net_percent_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.net_percent_change = value.try_into().map_err(|e| {
                    format!("error converting supplied value for net_percent_change: {e}")
                });
                self
            }
            pub fn open_interest<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.open_interest = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for open_interest: {e}"));
                self
            }
            pub fn open_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.open_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for open_price: {e}"));
                self
            }
            pub fn quote_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.quote_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quote_time: {e}"));
                self
            }
            pub fn rho<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.rho = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rho: {e}"));
                self
            }
            pub fn security_status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.security_status = value.try_into().map_err(|e| {
                    format!("error converting supplied value for security_status: {e}")
                });
                self
            }
            pub fn theoretical_option_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.theoretical_option_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for theoretical_option_value: {e}")
                });
                self
            }
            pub fn theta<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.theta = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for theta: {e}"));
                self
            }
            pub fn time_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.time_value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for time_value: {e}"));
                self
            }
            pub fn total_volume<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.total_volume = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_volume: {e}"));
                self
            }
            pub fn trade_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.trade_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for trade_time: {e}"));
                self
            }
            pub fn underlying_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.underlying_price = value.try_into().map_err(|e| {
                    format!("error converting supplied value for underlying_price: {e}")
                });
                self
            }
            pub fn vega<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.vega = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for vega: {e}"));
                self
            }
            pub fn volatility<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.volatility = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for volatility: {e}"));
                self
            }
            pub fn x52week_high<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.x52week_high = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for x52week_high: {e}"));
                self
            }
            pub fn x52week_low<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.x52week_low = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for x52week_low: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<QuoteOption> for super::QuoteOption {
            type Error = super::error::ConversionError;
            fn try_from(
                value: QuoteOption,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    ask_price: value.ask_price?,
                    ask_size: value.ask_size?,
                    bid_price: value.bid_price?,
                    bid_size: value.bid_size?,
                    close_price: value.close_price?,
                    delta: value.delta?,
                    gamma: value.gamma?,
                    high_price: value.high_price?,
                    implied_yield: value.implied_yield?,
                    ind_ask_price: value.ind_ask_price?,
                    ind_bid_price: value.ind_bid_price?,
                    ind_quote_time: value.ind_quote_time?,
                    last_price: value.last_price?,
                    last_size: value.last_size?,
                    low_price: value.low_price?,
                    mark: value.mark?,
                    mark_change: value.mark_change?,
                    mark_percent_change: value.mark_percent_change?,
                    money_intrinsic_value: value.money_intrinsic_value?,
                    net_change: value.net_change?,
                    net_percent_change: value.net_percent_change?,
                    open_interest: value.open_interest?,
                    open_price: value.open_price?,
                    quote_time: value.quote_time?,
                    rho: value.rho?,
                    security_status: value.security_status?,
                    theoretical_option_value: value.theoretical_option_value?,
                    theta: value.theta?,
                    time_value: value.time_value?,
                    total_volume: value.total_volume?,
                    trade_time: value.trade_time?,
                    underlying_price: value.underlying_price?,
                    vega: value.vega?,
                    volatility: value.volatility?,
                    x52week_high: value.x52week_high?,
                    x52week_low: value.x52week_low?,
                })
            }
        }

        impl ::std::convert::From<super::QuoteOption> for QuoteOption {
            fn from(value: super::QuoteOption) -> Self {
                Self {
                    ask_price: Ok(value.ask_price),
                    ask_size: Ok(value.ask_size),
                    bid_price: Ok(value.bid_price),
                    bid_size: Ok(value.bid_size),
                    close_price: Ok(value.close_price),
                    delta: Ok(value.delta),
                    gamma: Ok(value.gamma),
                    high_price: Ok(value.high_price),
                    implied_yield: Ok(value.implied_yield),
                    ind_ask_price: Ok(value.ind_ask_price),
                    ind_bid_price: Ok(value.ind_bid_price),
                    ind_quote_time: Ok(value.ind_quote_time),
                    last_price: Ok(value.last_price),
                    last_size: Ok(value.last_size),
                    low_price: Ok(value.low_price),
                    mark: Ok(value.mark),
                    mark_change: Ok(value.mark_change),
                    mark_percent_change: Ok(value.mark_percent_change),
                    money_intrinsic_value: Ok(value.money_intrinsic_value),
                    net_change: Ok(value.net_change),
                    net_percent_change: Ok(value.net_percent_change),
                    open_interest: Ok(value.open_interest),
                    open_price: Ok(value.open_price),
                    quote_time: Ok(value.quote_time),
                    rho: Ok(value.rho),
                    security_status: Ok(value.security_status),
                    theoretical_option_value: Ok(value.theoretical_option_value),
                    theta: Ok(value.theta),
                    time_value: Ok(value.time_value),
                    total_volume: Ok(value.total_volume),
                    trade_time: Ok(value.trade_time),
                    underlying_price: Ok(value.underlying_price),
                    vega: Ok(value.vega),
                    volatility: Ok(value.volatility),
                    x52week_high: Ok(value.x52week_high),
                    x52week_low: Ok(value.x52week_low),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct QuoteRequest {
            cusips: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            fields: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            indicative: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            realtime: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            ssids: ::std::result::Result<
                ::std::vec::Vec<::std::num::NonZeroU64>,
                ::std::string::String,
            >,
            symbols: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for QuoteRequest {
            fn default() -> Self {
                Self {
                    cusips: Ok(Default::default()),
                    fields: Ok(Default::default()),
                    indicative: Ok(Default::default()),
                    realtime: Ok(Default::default()),
                    ssids: Ok(Default::default()),
                    symbols: Ok(Default::default()),
                }
            }
        }

        impl QuoteRequest {
            pub fn cusips<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.cusips = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cusips: {e}"));
                self
            }
            pub fn fields<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.fields = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fields: {e}"));
                self
            }
            pub fn indicative<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.indicative = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for indicative: {e}"));
                self
            }
            pub fn realtime<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.realtime = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for realtime: {e}"));
                self
            }
            pub fn ssids<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::num::NonZeroU64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ssids = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ssids: {e}"));
                self
            }
            pub fn symbols<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.symbols = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for symbols: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<QuoteRequest> for super::QuoteRequest {
            type Error = super::error::ConversionError;
            fn try_from(
                value: QuoteRequest,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cusips: value.cusips?,
                    fields: value.fields?,
                    indicative: value.indicative?,
                    realtime: value.realtime?,
                    ssids: value.ssids?,
                    symbols: value.symbols?,
                })
            }
        }

        impl ::std::convert::From<super::QuoteRequest> for QuoteRequest {
            fn from(value: super::QuoteRequest) -> Self {
                Self {
                    cusips: Ok(value.cusips),
                    fields: Ok(value.fields),
                    indicative: Ok(value.indicative),
                    realtime: Ok(value.realtime),
                    ssids: Ok(value.ssids),
                    symbols: Ok(value.symbols),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ReferenceEquity {
            cusip: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            fsi_desc: ::std::result::Result<
                ::std::option::Option<super::ReferenceEquityFsiDesc>,
                ::std::string::String,
            >,
            htb_quantity: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            htb_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            is_hard_to_borrow:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            is_shortable: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            otc_market_tier: ::std::result::Result<
                ::std::option::Option<super::ReferenceEquityOtcMarketTier>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for ReferenceEquity {
            fn default() -> Self {
                Self {
                    cusip: Ok(Default::default()),
                    description: Ok(Default::default()),
                    exchange: Ok(Default::default()),
                    exchange_name: Ok(Default::default()),
                    fsi_desc: Ok(Default::default()),
                    htb_quantity: Ok(Default::default()),
                    htb_rate: Ok(Default::default()),
                    is_hard_to_borrow: Ok(Default::default()),
                    is_shortable: Ok(Default::default()),
                    otc_market_tier: Ok(Default::default()),
                }
            }
        }

        impl ReferenceEquity {
            pub fn cusip<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.cusip = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cusip: {e}"));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {e}"));
                self
            }
            pub fn exchange<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange: {e}"));
                self
            }
            pub fn exchange_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange_name: {e}"));
                self
            }
            pub fn fsi_desc<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ReferenceEquityFsiDesc>>,
                T::Error: ::std::fmt::Display,
            {
                self.fsi_desc = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fsi_desc: {e}"));
                self
            }
            pub fn htb_quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.htb_quantity = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for htb_quantity: {e}"));
                self
            }
            pub fn htb_rate<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.htb_rate = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for htb_rate: {e}"));
                self
            }
            pub fn is_hard_to_borrow<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.is_hard_to_borrow = value.try_into().map_err(|e| {
                    format!("error converting supplied value for is_hard_to_borrow: {e}")
                });
                self
            }
            pub fn is_shortable<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.is_shortable = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for is_shortable: {e}"));
                self
            }
            pub fn otc_market_tier<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<super::ReferenceEquityOtcMarketTier>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.otc_market_tier = value.try_into().map_err(|e| {
                    format!("error converting supplied value for otc_market_tier: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<ReferenceEquity> for super::ReferenceEquity {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ReferenceEquity,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cusip: value.cusip?,
                    description: value.description?,
                    exchange: value.exchange?,
                    exchange_name: value.exchange_name?,
                    fsi_desc: value.fsi_desc?,
                    htb_quantity: value.htb_quantity?,
                    htb_rate: value.htb_rate?,
                    is_hard_to_borrow: value.is_hard_to_borrow?,
                    is_shortable: value.is_shortable?,
                    otc_market_tier: value.otc_market_tier?,
                })
            }
        }

        impl ::std::convert::From<super::ReferenceEquity> for ReferenceEquity {
            fn from(value: super::ReferenceEquity) -> Self {
                Self {
                    cusip: Ok(value.cusip),
                    description: Ok(value.description),
                    exchange: Ok(value.exchange),
                    exchange_name: Ok(value.exchange_name),
                    fsi_desc: Ok(value.fsi_desc),
                    htb_quantity: Ok(value.htb_quantity),
                    htb_rate: Ok(value.htb_rate),
                    is_hard_to_borrow: Ok(value.is_hard_to_borrow),
                    is_shortable: Ok(value.is_shortable),
                    otc_market_tier: Ok(value.otc_market_tier),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ReferenceForex {
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            is_tradable: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            market_maker: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            product: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            trading_hours: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for ReferenceForex {
            fn default() -> Self {
                Self {
                    description: Ok(Default::default()),
                    exchange: Ok(Default::default()),
                    exchange_name: Ok(Default::default()),
                    is_tradable: Ok(Default::default()),
                    market_maker: Ok(Default::default()),
                    product: Ok(Default::default()),
                    trading_hours: Ok(Default::default()),
                }
            }
        }

        impl ReferenceForex {
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {e}"));
                self
            }
            pub fn exchange<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange: {e}"));
                self
            }
            pub fn exchange_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange_name: {e}"));
                self
            }
            pub fn is_tradable<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.is_tradable = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for is_tradable: {e}"));
                self
            }
            pub fn market_maker<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.market_maker = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for market_maker: {e}"));
                self
            }
            pub fn product<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.product = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for product: {e}"));
                self
            }
            pub fn trading_hours<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.trading_hours = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for trading_hours: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ReferenceForex> for super::ReferenceForex {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ReferenceForex,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    description: value.description?,
                    exchange: value.exchange?,
                    exchange_name: value.exchange_name?,
                    is_tradable: value.is_tradable?,
                    market_maker: value.market_maker?,
                    product: value.product?,
                    trading_hours: value.trading_hours?,
                })
            }
        }

        impl ::std::convert::From<super::ReferenceForex> for ReferenceForex {
            fn from(value: super::ReferenceForex) -> Self {
                Self {
                    description: Ok(value.description),
                    exchange: Ok(value.exchange),
                    exchange_name: Ok(value.exchange_name),
                    is_tradable: Ok(value.is_tradable),
                    market_maker: Ok(value.market_maker),
                    product: Ok(value.product),
                    trading_hours: Ok(value.trading_hours),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ReferenceFuture {
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            future_active_symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            future_expiration_date:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            future_is_active:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            future_multiplier:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            future_price_format: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            future_settlement_price:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            future_trading_hours: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            product: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for ReferenceFuture {
            fn default() -> Self {
                Self {
                    description: Ok(Default::default()),
                    exchange: Ok(Default::default()),
                    exchange_name: Ok(Default::default()),
                    future_active_symbol: Ok(Default::default()),
                    future_expiration_date: Ok(Default::default()),
                    future_is_active: Ok(Default::default()),
                    future_multiplier: Ok(Default::default()),
                    future_price_format: Ok(Default::default()),
                    future_settlement_price: Ok(Default::default()),
                    future_trading_hours: Ok(Default::default()),
                    product: Ok(Default::default()),
                }
            }
        }

        impl ReferenceFuture {
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {e}"));
                self
            }
            pub fn exchange<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange: {e}"));
                self
            }
            pub fn exchange_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange_name: {e}"));
                self
            }
            pub fn future_active_symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.future_active_symbol = value.try_into().map_err(|e| {
                    format!("error converting supplied value for future_active_symbol: {e}")
                });
                self
            }
            pub fn future_expiration_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.future_expiration_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for future_expiration_date: {e}")
                });
                self
            }
            pub fn future_is_active<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.future_is_active = value.try_into().map_err(|e| {
                    format!("error converting supplied value for future_is_active: {e}")
                });
                self
            }
            pub fn future_multiplier<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.future_multiplier = value.try_into().map_err(|e| {
                    format!("error converting supplied value for future_multiplier: {e}")
                });
                self
            }
            pub fn future_price_format<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.future_price_format = value.try_into().map_err(|e| {
                    format!("error converting supplied value for future_price_format: {e}")
                });
                self
            }
            pub fn future_settlement_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.future_settlement_price = value.try_into().map_err(|e| {
                    format!("error converting supplied value for future_settlement_price: {e}")
                });
                self
            }
            pub fn future_trading_hours<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.future_trading_hours = value.try_into().map_err(|e| {
                    format!("error converting supplied value for future_trading_hours: {e}")
                });
                self
            }
            pub fn product<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.product = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for product: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ReferenceFuture> for super::ReferenceFuture {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ReferenceFuture,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    description: value.description?,
                    exchange: value.exchange?,
                    exchange_name: value.exchange_name?,
                    future_active_symbol: value.future_active_symbol?,
                    future_expiration_date: value.future_expiration_date?,
                    future_is_active: value.future_is_active?,
                    future_multiplier: value.future_multiplier?,
                    future_price_format: value.future_price_format?,
                    future_settlement_price: value.future_settlement_price?,
                    future_trading_hours: value.future_trading_hours?,
                    product: value.product?,
                })
            }
        }

        impl ::std::convert::From<super::ReferenceFuture> for ReferenceFuture {
            fn from(value: super::ReferenceFuture) -> Self {
                Self {
                    description: Ok(value.description),
                    exchange: Ok(value.exchange),
                    exchange_name: Ok(value.exchange_name),
                    future_active_symbol: Ok(value.future_active_symbol),
                    future_expiration_date: Ok(value.future_expiration_date),
                    future_is_active: Ok(value.future_is_active),
                    future_multiplier: Ok(value.future_multiplier),
                    future_price_format: Ok(value.future_price_format),
                    future_settlement_price: Ok(value.future_settlement_price),
                    future_trading_hours: Ok(value.future_trading_hours),
                    product: Ok(value.product),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ReferenceFutureOption {
            contract_type: ::std::result::Result<
                ::std::option::Option<super::ContractType>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            expiration_date:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            expiration_style: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            multiplier: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            strike_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            underlying: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for ReferenceFutureOption {
            fn default() -> Self {
                Self {
                    contract_type: Ok(Default::default()),
                    description: Ok(Default::default()),
                    exchange: Ok(Default::default()),
                    exchange_name: Ok(Default::default()),
                    expiration_date: Ok(Default::default()),
                    expiration_style: Ok(Default::default()),
                    multiplier: Ok(Default::default()),
                    strike_price: Ok(Default::default()),
                    underlying: Ok(Default::default()),
                }
            }
        }

        impl ReferenceFutureOption {
            pub fn contract_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ContractType>>,
                T::Error: ::std::fmt::Display,
            {
                self.contract_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for contract_type: {e}"));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {e}"));
                self
            }
            pub fn exchange<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange: {e}"));
                self
            }
            pub fn exchange_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange_name: {e}"));
                self
            }
            pub fn expiration_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.expiration_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for expiration_date: {e}")
                });
                self
            }
            pub fn expiration_style<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.expiration_style = value.try_into().map_err(|e| {
                    format!("error converting supplied value for expiration_style: {e}")
                });
                self
            }
            pub fn multiplier<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.multiplier = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for multiplier: {e}"));
                self
            }
            pub fn strike_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.strike_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for strike_price: {e}"));
                self
            }
            pub fn underlying<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.underlying = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for underlying: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ReferenceFutureOption> for super::ReferenceFutureOption {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ReferenceFutureOption,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    contract_type: value.contract_type?,
                    description: value.description?,
                    exchange: value.exchange?,
                    exchange_name: value.exchange_name?,
                    expiration_date: value.expiration_date?,
                    expiration_style: value.expiration_style?,
                    multiplier: value.multiplier?,
                    strike_price: value.strike_price?,
                    underlying: value.underlying?,
                })
            }
        }

        impl ::std::convert::From<super::ReferenceFutureOption> for ReferenceFutureOption {
            fn from(value: super::ReferenceFutureOption) -> Self {
                Self {
                    contract_type: Ok(value.contract_type),
                    description: Ok(value.description),
                    exchange: Ok(value.exchange),
                    exchange_name: Ok(value.exchange_name),
                    expiration_date: Ok(value.expiration_date),
                    expiration_style: Ok(value.expiration_style),
                    multiplier: Ok(value.multiplier),
                    strike_price: Ok(value.strike_price),
                    underlying: Ok(value.underlying),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ReferenceIndex {
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for ReferenceIndex {
            fn default() -> Self {
                Self {
                    description: Ok(Default::default()),
                    exchange: Ok(Default::default()),
                    exchange_name: Ok(Default::default()),
                }
            }
        }

        impl ReferenceIndex {
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {e}"));
                self
            }
            pub fn exchange<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange: {e}"));
                self
            }
            pub fn exchange_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange_name: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ReferenceIndex> for super::ReferenceIndex {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ReferenceIndex,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    description: value.description?,
                    exchange: value.exchange?,
                    exchange_name: value.exchange_name?,
                })
            }
        }

        impl ::std::convert::From<super::ReferenceIndex> for ReferenceIndex {
            fn from(value: super::ReferenceIndex) -> Self {
                Self {
                    description: Ok(value.description),
                    exchange: Ok(value.exchange),
                    exchange_name: Ok(value.exchange_name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ReferenceMutualFund {
            cusip: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange: ::std::result::Result<::std::string::String, ::std::string::String>,
            exchange_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for ReferenceMutualFund {
            fn default() -> Self {
                Self {
                    cusip: Ok(Default::default()),
                    description: Ok(Default::default()),
                    exchange: Ok(super::defaults::reference_mutual_fund_exchange()),
                    exchange_name: Ok(super::defaults::reference_mutual_fund_exchange_name()),
                }
            }
        }

        impl ReferenceMutualFund {
            pub fn cusip<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.cusip = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cusip: {e}"));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {e}"));
                self
            }
            pub fn exchange<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange: {e}"));
                self
            }
            pub fn exchange_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange_name: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ReferenceMutualFund> for super::ReferenceMutualFund {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ReferenceMutualFund,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cusip: value.cusip?,
                    description: value.description?,
                    exchange: value.exchange?,
                    exchange_name: value.exchange_name?,
                })
            }
        }

        impl ::std::convert::From<super::ReferenceMutualFund> for ReferenceMutualFund {
            fn from(value: super::ReferenceMutualFund) -> Self {
                Self {
                    cusip: Ok(value.cusip),
                    description: Ok(value.description),
                    exchange: Ok(value.exchange),
                    exchange_name: Ok(value.exchange_name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ReferenceOption {
            contract_type: ::std::result::Result<
                ::std::option::Option<super::ContractType>,
                ::std::string::String,
            >,
            cusip: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            days_to_expiration:
                ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            deliverables: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange: ::std::result::Result<::std::string::String, ::std::string::String>,
            exchange_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exercise_type: ::std::result::Result<
                ::std::option::Option<super::ExerciseType>,
                ::std::string::String,
            >,
            expiration_day: ::std::result::Result<
                ::std::option::Option<::std::num::NonZeroU32>,
                ::std::string::String,
            >,
            expiration_month: ::std::result::Result<
                ::std::option::Option<::std::num::NonZeroU32>,
                ::std::string::String,
            >,
            expiration_type: ::std::result::Result<
                ::std::option::Option<super::ExpirationType>,
                ::std::string::String,
            >,
            expiration_year:
                ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            is_penny_pilot:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            last_trading_day:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            multiplier: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            settlement_type: ::std::result::Result<
                ::std::option::Option<super::SettlementType>,
                ::std::string::String,
            >,
            strike_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            underlying: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for ReferenceOption {
            fn default() -> Self {
                Self {
                    contract_type: Ok(Default::default()),
                    cusip: Ok(Default::default()),
                    days_to_expiration: Ok(Default::default()),
                    deliverables: Ok(Default::default()),
                    description: Ok(Default::default()),
                    exchange: Ok(super::defaults::reference_option_exchange()),
                    exchange_name: Ok(Default::default()),
                    exercise_type: Ok(Default::default()),
                    expiration_day: Ok(Default::default()),
                    expiration_month: Ok(Default::default()),
                    expiration_type: Ok(Default::default()),
                    expiration_year: Ok(Default::default()),
                    is_penny_pilot: Ok(Default::default()),
                    last_trading_day: Ok(Default::default()),
                    multiplier: Ok(Default::default()),
                    settlement_type: Ok(Default::default()),
                    strike_price: Ok(Default::default()),
                    underlying: Ok(Default::default()),
                }
            }
        }

        impl ReferenceOption {
            pub fn contract_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ContractType>>,
                T::Error: ::std::fmt::Display,
            {
                self.contract_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for contract_type: {e}"));
                self
            }
            pub fn cusip<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.cusip = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cusip: {e}"));
                self
            }
            pub fn days_to_expiration<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.days_to_expiration = value.try_into().map_err(|e| {
                    format!("error converting supplied value for days_to_expiration: {e}")
                });
                self
            }
            pub fn deliverables<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.deliverables = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deliverables: {e}"));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {e}"));
                self
            }
            pub fn exchange<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange: {e}"));
                self
            }
            pub fn exchange_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange_name: {e}"));
                self
            }
            pub fn exercise_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ExerciseType>>,
                T::Error: ::std::fmt::Display,
            {
                self.exercise_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exercise_type: {e}"));
                self
            }
            pub fn expiration_day<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU32>>,
                T::Error: ::std::fmt::Display,
            {
                self.expiration_day = value.try_into().map_err(|e| {
                    format!("error converting supplied value for expiration_day: {e}")
                });
                self
            }
            pub fn expiration_month<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU32>>,
                T::Error: ::std::fmt::Display,
            {
                self.expiration_month = value.try_into().map_err(|e| {
                    format!("error converting supplied value for expiration_month: {e}")
                });
                self
            }
            pub fn expiration_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ExpirationType>>,
                T::Error: ::std::fmt::Display,
            {
                self.expiration_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for expiration_type: {e}")
                });
                self
            }
            pub fn expiration_year<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.expiration_year = value.try_into().map_err(|e| {
                    format!("error converting supplied value for expiration_year: {e}")
                });
                self
            }
            pub fn is_penny_pilot<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.is_penny_pilot = value.try_into().map_err(|e| {
                    format!("error converting supplied value for is_penny_pilot: {e}")
                });
                self
            }
            pub fn last_trading_day<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_trading_day = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_trading_day: {e}")
                });
                self
            }
            pub fn multiplier<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.multiplier = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for multiplier: {e}"));
                self
            }
            pub fn settlement_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::SettlementType>>,
                T::Error: ::std::fmt::Display,
            {
                self.settlement_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for settlement_type: {e}")
                });
                self
            }
            pub fn strike_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.strike_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for strike_price: {e}"));
                self
            }
            pub fn underlying<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.underlying = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for underlying: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ReferenceOption> for super::ReferenceOption {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ReferenceOption,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    contract_type: value.contract_type?,
                    cusip: value.cusip?,
                    days_to_expiration: value.days_to_expiration?,
                    deliverables: value.deliverables?,
                    description: value.description?,
                    exchange: value.exchange?,
                    exchange_name: value.exchange_name?,
                    exercise_type: value.exercise_type?,
                    expiration_day: value.expiration_day?,
                    expiration_month: value.expiration_month?,
                    expiration_type: value.expiration_type?,
                    expiration_year: value.expiration_year?,
                    is_penny_pilot: value.is_penny_pilot?,
                    last_trading_day: value.last_trading_day?,
                    multiplier: value.multiplier?,
                    settlement_type: value.settlement_type?,
                    strike_price: value.strike_price?,
                    underlying: value.underlying?,
                })
            }
        }

        impl ::std::convert::From<super::ReferenceOption> for ReferenceOption {
            fn from(value: super::ReferenceOption) -> Self {
                Self {
                    contract_type: Ok(value.contract_type),
                    cusip: Ok(value.cusip),
                    days_to_expiration: Ok(value.days_to_expiration),
                    deliverables: Ok(value.deliverables),
                    description: Ok(value.description),
                    exchange: Ok(value.exchange),
                    exchange_name: Ok(value.exchange_name),
                    exercise_type: Ok(value.exercise_type),
                    expiration_day: Ok(value.expiration_day),
                    expiration_month: Ok(value.expiration_month),
                    expiration_type: Ok(value.expiration_type),
                    expiration_year: Ok(value.expiration_year),
                    is_penny_pilot: Ok(value.is_penny_pilot),
                    last_trading_day: Ok(value.last_trading_day),
                    multiplier: Ok(value.multiplier),
                    settlement_type: Ok(value.settlement_type),
                    strike_price: Ok(value.strike_price),
                    underlying: Ok(value.underlying),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct RegularMarket {
            regular_market_last_price:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            regular_market_last_size:
                ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            regular_market_net_change:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            regular_market_percent_change:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            regular_market_trade_time:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        }

        impl ::std::default::Default for RegularMarket {
            fn default() -> Self {
                Self {
                    regular_market_last_price: Ok(Default::default()),
                    regular_market_last_size: Ok(Default::default()),
                    regular_market_net_change: Ok(Default::default()),
                    regular_market_percent_change: Ok(Default::default()),
                    regular_market_trade_time: Ok(Default::default()),
                }
            }
        }

        impl RegularMarket {
            pub fn regular_market_last_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.regular_market_last_price = value.try_into().map_err(|e| {
                    format!("error converting supplied value for regular_market_last_price: {e}")
                });
                self
            }
            pub fn regular_market_last_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.regular_market_last_size = value.try_into().map_err(|e| {
                    format!("error converting supplied value for regular_market_last_size: {e}")
                });
                self
            }
            pub fn regular_market_net_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.regular_market_net_change = value.try_into().map_err(|e| {
                    format!("error converting supplied value for regular_market_net_change: {e}")
                });
                self
            }
            pub fn regular_market_percent_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.regular_market_percent_change = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for regular_market_percent_change: {e}"
                    )
                });
                self
            }
            pub fn regular_market_trade_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.regular_market_trade_time = value.try_into().map_err(|e| {
                    format!("error converting supplied value for regular_market_trade_time: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<RegularMarket> for super::RegularMarket {
            type Error = super::error::ConversionError;
            fn try_from(
                value: RegularMarket,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    regular_market_last_price: value.regular_market_last_price?,
                    regular_market_last_size: value.regular_market_last_size?,
                    regular_market_net_change: value.regular_market_net_change?,
                    regular_market_percent_change: value.regular_market_percent_change?,
                    regular_market_trade_time: value.regular_market_trade_time?,
                })
            }
        }

        impl ::std::convert::From<super::RegularMarket> for RegularMarket {
            fn from(value: super::RegularMarket) -> Self {
                Self {
                    regular_market_last_price: Ok(value.regular_market_last_price),
                    regular_market_last_size: Ok(value.regular_market_last_size),
                    regular_market_net_change: Ok(value.regular_market_net_change),
                    regular_market_percent_change: Ok(value.regular_market_percent_change),
                    regular_market_trade_time: Ok(value.regular_market_trade_time),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Screener {
            change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            direction: ::std::result::Result<
                ::std::option::Option<super::ScreenerDirection>,
                ::std::string::String,
            >,
            last: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            total_volume: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        }

        impl ::std::default::Default for Screener {
            fn default() -> Self {
                Self {
                    change: Ok(Default::default()),
                    description: Ok(Default::default()),
                    direction: Ok(Default::default()),
                    last: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    total_volume: Ok(Default::default()),
                }
            }
        }

        impl Screener {
            pub fn change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.change = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for change: {e}"));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {e}"));
                self
            }
            pub fn direction<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ScreenerDirection>>,
                T::Error: ::std::fmt::Display,
            {
                self.direction = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for direction: {e}"));
                self
            }
            pub fn last<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.last = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last: {e}"));
                self
            }
            pub fn symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for symbol: {e}"));
                self
            }
            pub fn total_volume<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.total_volume = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_volume: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Screener> for super::Screener {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Screener,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    change: value.change?,
                    description: value.description?,
                    direction: value.direction?,
                    last: value.last?,
                    symbol: value.symbol?,
                    total_volume: value.total_volume?,
                })
            }
        }

        impl ::std::convert::From<super::Screener> for Screener {
            fn from(value: super::Screener) -> Self {
                Self {
                    change: Ok(value.change),
                    description: Ok(value.description),
                    direction: Ok(value.direction),
                    last: Ok(value.last),
                    symbol: Ok(value.symbol),
                    total_volume: Ok(value.total_volume),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Underlying {
            ask: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            ask_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            bid: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            bid_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            close: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            delayed: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            fifty_two_week_high:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            fifty_two_week_low:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            high_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            last: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            low_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mark: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mark_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mark_percent_change:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            open_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            percent_change:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            quote_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            total_volume: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            trade_time: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        }

        impl ::std::default::Default for Underlying {
            fn default() -> Self {
                Self {
                    ask: Ok(Default::default()),
                    ask_size: Ok(Default::default()),
                    bid: Ok(Default::default()),
                    bid_size: Ok(Default::default()),
                    change: Ok(Default::default()),
                    close: Ok(Default::default()),
                    delayed: Ok(Default::default()),
                    description: Ok(Default::default()),
                    exchange_name: Ok(Default::default()),
                    fifty_two_week_high: Ok(Default::default()),
                    fifty_two_week_low: Ok(Default::default()),
                    high_price: Ok(Default::default()),
                    last: Ok(Default::default()),
                    low_price: Ok(Default::default()),
                    mark: Ok(Default::default()),
                    mark_change: Ok(Default::default()),
                    mark_percent_change: Ok(Default::default()),
                    open_price: Ok(Default::default()),
                    percent_change: Ok(Default::default()),
                    quote_time: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    total_volume: Ok(Default::default()),
                    trade_time: Ok(Default::default()),
                }
            }
        }

        impl Underlying {
            pub fn ask<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask: {e}"));
                self
            }
            pub fn ask_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.ask_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ask_size: {e}"));
                self
            }
            pub fn bid<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid: {e}"));
                self
            }
            pub fn bid_size<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.bid_size = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bid_size: {e}"));
                self
            }
            pub fn change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.change = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for change: {e}"));
                self
            }
            pub fn close<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.close = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for close: {e}"));
                self
            }
            pub fn delayed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.delayed = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for delayed: {e}"));
                self
            }
            pub fn description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.description = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for description: {e}"));
                self
            }
            pub fn exchange_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.exchange_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for exchange_name: {e}"));
                self
            }
            pub fn fifty_two_week_high<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.fifty_two_week_high = value.try_into().map_err(|e| {
                    format!("error converting supplied value for fifty_two_week_high: {e}")
                });
                self
            }
            pub fn fifty_two_week_low<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.fifty_two_week_low = value.try_into().map_err(|e| {
                    format!("error converting supplied value for fifty_two_week_low: {e}")
                });
                self
            }
            pub fn high_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.high_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for high_price: {e}"));
                self
            }
            pub fn last<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.last = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last: {e}"));
                self
            }
            pub fn low_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.low_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for low_price: {e}"));
                self
            }
            pub fn mark<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mark = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mark: {e}"));
                self
            }
            pub fn mark_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mark_change = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for mark_change: {e}"));
                self
            }
            pub fn mark_percent_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mark_percent_change = value.try_into().map_err(|e| {
                    format!("error converting supplied value for mark_percent_change: {e}")
                });
                self
            }
            pub fn open_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.open_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for open_price: {e}"));
                self
            }
            pub fn percent_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.percent_change = value.try_into().map_err(|e| {
                    format!("error converting supplied value for percent_change: {e}")
                });
                self
            }
            pub fn quote_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.quote_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quote_time: {e}"));
                self
            }
            pub fn symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for symbol: {e}"));
                self
            }
            pub fn total_volume<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.total_volume = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_volume: {e}"));
                self
            }
            pub fn trade_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.trade_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for trade_time: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Underlying> for super::Underlying {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Underlying,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    ask: value.ask?,
                    ask_size: value.ask_size?,
                    bid: value.bid?,
                    bid_size: value.bid_size?,
                    change: value.change?,
                    close: value.close?,
                    delayed: value.delayed?,
                    description: value.description?,
                    exchange_name: value.exchange_name?,
                    fifty_two_week_high: value.fifty_two_week_high?,
                    fifty_two_week_low: value.fifty_two_week_low?,
                    high_price: value.high_price?,
                    last: value.last?,
                    low_price: value.low_price?,
                    mark: value.mark?,
                    mark_change: value.mark_change?,
                    mark_percent_change: value.mark_percent_change?,
                    open_price: value.open_price?,
                    percent_change: value.percent_change?,
                    quote_time: value.quote_time?,
                    symbol: value.symbol?,
                    total_volume: value.total_volume?,
                    trade_time: value.trade_time?,
                })
            }
        }

        impl ::std::convert::From<super::Underlying> for Underlying {
            fn from(value: super::Underlying) -> Self {
                Self {
                    ask: Ok(value.ask),
                    ask_size: Ok(value.ask_size),
                    bid: Ok(value.bid),
                    bid_size: Ok(value.bid_size),
                    change: Ok(value.change),
                    close: Ok(value.close),
                    delayed: Ok(value.delayed),
                    description: Ok(value.description),
                    exchange_name: Ok(value.exchange_name),
                    fifty_two_week_high: Ok(value.fifty_two_week_high),
                    fifty_two_week_low: Ok(value.fifty_two_week_low),
                    high_price: Ok(value.high_price),
                    last: Ok(value.last),
                    low_price: Ok(value.low_price),
                    mark: Ok(value.mark),
                    mark_change: Ok(value.mark_change),
                    mark_percent_change: Ok(value.mark_percent_change),
                    open_price: Ok(value.open_price),
                    percent_change: Ok(value.percent_change),
                    quote_time: Ok(value.quote_time),
                    symbol: Ok(value.symbol),
                    total_volume: Ok(value.total_volume),
                    trade_time: Ok(value.trade_time),
                }
            }
        }
    }

    /// Generation of default values for serde.
    pub mod defaults {
        pub(super) fn reference_mutual_fund_exchange() -> ::std::string::String {
            "m".to_string()
        }

        pub(super) fn reference_mutual_fund_exchange_name() -> ::std::string::String {
            "MUTUAL_FUND".to_string()
        }

        pub(super) fn reference_option_exchange() -> ::std::string::String {
            "o".to_string()
        }
    }
}

#[derive(Clone, Debug)]
///Client for Market Data
///
///Trader API - Market data
///
///Version: 1.0.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = ::std::time::Duration::from_secs(15u64);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
}

impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "1.0.0"
    }

    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }

    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn inner(&self) -> &() {
        &()
    }
}

impl ClientHooks<()> for &Client {}
impl Client {
    ///Get Quotes by list of symbols
    ///
    ///Sends a `GET` request to `/quotes`
    ///
    ///Arguments:
    /// - `fields`: Request for subset of data by passing coma separated list of
    ///   root nodes, possible root nodes are quote, fundamental, extended,
    ///   reference, regular. Sending `quote, fundamental` in request will
    ///   return quote and fundamental data in response. Dont send this
    ///   attribute for full response.
    /// - `indicative`: Include indicative symbol quotes for all ETF symbols in
    ///   request. If ETF symbol ABC is in request and indicative=true API will
    ///   return quotes for ABC and its corresponding indicative quote for
    ///   $ABC.IV
    /// - `symbols`: Comma separated list of symbol(s) to look up a quote
    ///```ignore
    /// let response = client.get_quotes()
    ///    .fields(fields)
    ///    .indicative(indicative)
    ///    .symbols(symbols)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_quotes(&self) -> builder::GetQuotes<'_> {
        builder::GetQuotes::new(self)
    }

    ///Get Quote by single symbol
    ///
    ///Sends a `GET` request to `/{symbol_id}/quotes`
    ///
    ///Arguments:
    /// - `symbol_id`: Symbol of instrument
    /// - `fields`: Request for subset of data by passing coma separated list of
    ///   root nodes, possible root nodes are quote, fundamental, extended,
    ///   reference, regular. Sending `quote, fundamental` in request will
    ///   return quote and fundamental data in response. Dont send this
    ///   attribute for full response.
    ///```ignore
    /// let response = client.get_quote()
    ///    .symbol_id(symbol_id)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_quote(&self) -> builder::GetQuote<'_> {
        builder::GetQuote::new(self)
    }

    ///Get option chain for an optionable Symbol
    ///
    ///Get Option Chain including information on options contracts associated
    /// with each expiration.
    ///
    ///Sends a `GET` request to `/chains`
    ///
    ///Arguments:
    /// - `contract_type`: Contract Type
    /// - `days_to_expiration`: Days to expiration to use in calculations.
    ///   Applies only to ANALYTICAL strategy chains (see strategy param)
    /// - `entitlement`: Applicable only if its retail token, entitlement of
    ///   client PP-PayingPro, NP-NonPro and PN-NonPayingPro
    /// - `exp_month`: Expiration month
    /// - `from_date`: From date(pattern: yyyy-MM-dd)
    /// - `include_underlying_quote`: Underlying quotes to be included
    /// - `interest_rate`: Interest rate to use in calculations. Applies only to
    ///   ANALYTICAL strategy chains (see strategy param)
    /// - `interval`: Strike interval for spread strategy chains (see strategy
    ///   param)
    /// - `option_type`: Option Type
    /// - `range`: Range(ITM/NTM/OTM etc.)
    /// - `strategy`: OptionChain strategy. Default is SINGLE. ANALYTICAL allows
    ///   the use of volatility, underlyingPrice, interestRate, and
    ///   daysToExpiration params to calculate theoretical values.
    /// - `strike`: Strike Price
    /// - `strike_count`: The Number of strikes to return above or below the
    ///   at-the-money price
    /// - `symbol`: Enter one symbol
    /// - `to_date`: To date (pattern: yyyy-MM-dd)
    /// - `underlying_price`: Underlying price to use in calculations. Applies
    ///   only to ANALYTICAL strategy chains (see strategy param)
    /// - `volatility`: Volatility to use in calculations.  Applies only to
    ///   ANALYTICAL strategy chains (see strategy param)
    ///```ignore
    /// let response = client.get_chain()
    ///    .contract_type(contract_type)
    ///    .days_to_expiration(days_to_expiration)
    ///    .entitlement(entitlement)
    ///    .exp_month(exp_month)
    ///    .from_date(from_date)
    ///    .include_underlying_quote(include_underlying_quote)
    ///    .interest_rate(interest_rate)
    ///    .interval(interval)
    ///    .option_type(option_type)
    ///    .range(range)
    ///    .strategy(strategy)
    ///    .strike(strike)
    ///    .strike_count(strike_count)
    ///    .symbol(symbol)
    ///    .to_date(to_date)
    ///    .underlying_price(underlying_price)
    ///    .volatility(volatility)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_chain(&self) -> builder::GetChain<'_> {
        builder::GetChain::new(self)
    }

    ///Get option expiration chain for an optionable symbol
    ///
    ///Get Option Expiration (Series) information for an optionable symbol.
    /// Does not include individual options contracts for the underlying.
    ///
    ///Sends a `GET` request to `/expirationchain`
    ///
    ///Arguments:
    /// - `symbol`: Enter one symbol
    ///```ignore
    /// let response = client.get_expiration_chain()
    ///    .symbol(symbol)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_expiration_chain(&self) -> builder::GetExpirationChain<'_> {
        builder::GetExpirationChain::new(self)
    }

    ///Get PriceHistory for a single symbol and date ranges
    ///
    ///Get historical Open, High, Low, Close, and Volume for a given frequency
    /// (i.e. aggregation).  Frequency available is dependent on periodType
    /// selected.  The datetime format is in EPOCH milliseconds.
    ///
    ///Sends a `GET` request to `/pricehistory`
    ///
    ///Arguments:
    /// - `end_date`: The end date, Time   in milliseconds since the UNIX epoch
    ///   eg 1451624400000<br> If not   specified, the endDate will default to
    ///   the market close of previous business day.
    /// - `frequency`: The time frequency duration<br><br> If the frequencyType
    ///   is <br> &#8226; <b>minute</b> - valid values are 1, 5, 10, 15, 30<br>
    ///   &#8226; <b>daily</b> - valid value is 1<br> &#8226; <b>weekly</b> -
    ///   valid value is 1<br> &#8226; <b>monthly</b> - valid value is 1<br><br>
    ///   If frequency  is not specified, default value is <b>1</b><br>
    /// - `frequency_type`: The time frequencyType<br><br> If the periodType is
    ///   <br> &#8226; <b>day</b> - valid value is minute<br> &#8226;
    ///   <b>month</b> - valid values are daily, weekly<br> &#8226; <b>year</b>
    ///   - valid values are daily, weekly, monthly<br> &#8226; <b>ytd</b> -
    ///   valid values are daily, weekly<br><br> If frequencyType  is not
    ///   specified, default value depends on the periodType<br> &#8226;
    ///   <b>day</b> - defaulted to minute.<br> &#8226; <b>month</b> - defaulted
    ///   to weekly.<br> &#8226; <b>year</b> - defaulted to monthly.<br> &#8226;
    ///   <b>ytd</b> - defaulted to weekly.<br>
    /// - `need_extended_hours_data`: Need extended hours data
    /// - `need_previous_close`: Need previous close price/date
    /// - `period`: The number of chart period types.<br><br> If the periodType
    ///   is <br> &#8226; <b>day</b> - valid values are 1, 2, 3, 4, 5, 10<br>
    ///   &#8226; <b>month</b> - valid values are 1, 2, 3, 6<br> &#8226;
    ///   <b>year</b> - valid values are 1, 2, 3, 5, 10, 15, 20<br> &#8226;
    ///   <b>ytd</b> - valid values are 1<br><br> If   the period is not
    ///   specified and the periodType is<br> &#8226; <b>day</b> - default
    ///   period is 10.<br> &#8226; <b>month</b> - default period is 1.<br>
    ///   &#8226; <b>year</b> - default period is 1.<br> &#8226; <b>ytd</b> -
    ///   default period is 1.<br>
    /// - `period_type`: The chart period being requested.
    /// - `start_date`: The start date, Time   in milliseconds since the UNIX
    ///   epoch eg 1451624400000<br>If not   specified startDate will be
    ///   (endDate - period) excluding weekends and holidays.
    /// - `symbol`: The Equity symbol used to look up price history
    ///```ignore
    /// let response = client.get_price_history()
    ///    .end_date(end_date)
    ///    .frequency(frequency)
    ///    .frequency_type(frequency_type)
    ///    .need_extended_hours_data(need_extended_hours_data)
    ///    .need_previous_close(need_previous_close)
    ///    .period(period)
    ///    .period_type(period_type)
    ///    .start_date(start_date)
    ///    .symbol(symbol)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_price_history(&self) -> builder::GetPriceHistory<'_> {
        builder::GetPriceHistory::new(self)
    }

    ///Get Movers for a specific index
    ///
    ///Get a list of top 10 securities movement for a specific index.
    ///
    ///Sends a `GET` request to `/movers/{symbol_id}`
    ///
    ///Arguments:
    /// - `symbol_id`: Index Symbol
    /// - `frequency`: To return movers with the specified directions of up or
    ///   down
    /// - `sort`: Sort by a particular attribute
    ///```ignore
    /// let response = client.get_movers()
    ///    .symbol_id(symbol_id)
    ///    .frequency(frequency)
    ///    .sort(sort)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_movers(&self) -> builder::GetMovers<'_> {
        builder::GetMovers::new(self)
    }

    ///Get Market Hours for different markets
    ///
    ///Get Market Hours for dates in the future across different markets.
    ///
    ///Sends a `GET` request to `/markets`
    ///
    ///Arguments:
    /// - `date`: Valid date range is from currentdate to 1 year from today. It
    ///   will default to current day if not entered. Date format:YYYY-MM-DD
    /// - `markets`: List of markets
    ///```ignore
    /// let response = client.get_market_hours()
    ///    .date(date)
    ///    .markets(markets)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_market_hours(&self) -> builder::GetMarketHours<'_> {
        builder::GetMarketHours::new(self)
    }

    ///Get Market Hours for a single market
    ///
    ///Get Market Hours for dates in the future for a single market.
    ///
    ///Sends a `GET` request to `/markets/{market_id}`
    ///
    ///Arguments:
    /// - `market_id`: market id
    /// - `date`: Valid date range is from currentdate to 1 year from today. It
    ///   will default to current day if not entered. Date format:YYYY-MM-DD
    ///```ignore
    /// let response = client.get_market_hour()
    ///    .market_id(market_id)
    ///    .date(date)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_market_hour(&self) -> builder::GetMarketHour<'_> {
        builder::GetMarketHour::new(self)
    }

    ///Get Instruments by symbols and projections
    ///
    ///Get Instruments details by using different projections.  Get more
    /// specific fundamental instrument data by using fundamental as the
    /// projection.
    ///
    ///Sends a `GET` request to `/instruments`
    ///
    ///Arguments:
    /// - `projection`: search by
    /// - `symbol`: symbol of a security
    ///```ignore
    /// let response = client.get_instruments()
    ///    .projection(projection)
    ///    .symbol(symbol)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_instruments(&self) -> builder::GetInstruments<'_> {
        builder::GetInstruments::new(self)
    }

    ///Get Instrument by specific cusip
    ///
    ///Get basic instrument details by cusip
    ///
    ///Sends a `GET` request to `/instruments/{cusip_id}`
    ///
    ///Arguments:
    /// - `cusip_id`: cusip of a security
    ///```ignore
    /// let response = client.get_instruments_by_cusip()
    ///    .cusip_id(cusip_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_instruments_by_cusip(&self) -> builder::GetInstrumentsByCusip<'_> {
        builder::GetInstrumentsByCusip::new(self)
    }
}

/// Types for composing operation parameters.
#[allow(clippy::all)]
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        ByteStream, ClientHooks, ClientInfo, Error, OperationInfo, RequestBuilderExt,
        ResponseValue, encode_path,
    };
    ///Builder for [`Client::get_quotes`]
    ///
    ///[`Client::get_quotes`]: super::Client::get_quotes
    #[derive(Debug, Clone)]
    pub struct GetQuotes<'a> {
        client: &'a super::Client,
        fields: Result<Option<::std::string::String>, String>,
        indicative: Result<Option<bool>, String>,
        symbols: Result<Option<::std::string::String>, String>,
    }

    impl<'a> GetQuotes<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                fields: Ok(None),
                indicative: Ok(None),
                symbols: Ok(None),
            }
        }

        pub fn fields<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.fields = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for fields failed".to_string()
            });
            self
        }

        pub fn indicative<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.indicative = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for indicative failed".to_string());
            self
        }

        pub fn symbols<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.symbols = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for symbols failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/quotes`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::QuoteResponse>, Error<types::ErrorResponse>> {
            let Self {
                client,
                fields,
                indicative,
                symbols,
            } = self;
            let fields = fields.map_err(Error::InvalidRequest)?;
            let indicative = indicative.map_err(Error::InvalidRequest)?;
            let symbols = symbols.map_err(Error::InvalidRequest)?;
            let url = format!("{}/quotes", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("fields", &fields))
                .query(&progenitor_client::QueryParam::new(
                    "indicative",
                    &indicative,
                ))
                .query(&progenitor_client::QueryParam::new("symbols", &symbols))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_quotes",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_quote`]
    ///
    ///[`Client::get_quote`]: super::Client::get_quote
    #[derive(Debug, Clone)]
    pub struct GetQuote<'a> {
        client: &'a super::Client,
        symbol_id: Result<::std::string::String, String>,
        fields: Result<Option<::std::string::String>, String>,
    }

    impl<'a> GetQuote<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                symbol_id: Err("symbol_id was not initialized".to_string()),
                fields: Ok(None),
            }
        }

        pub fn symbol_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.symbol_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for symbol_id failed".to_string()
            });
            self
        }

        pub fn fields<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.fields = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for fields failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/{symbol_id}/quotes`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::QuoteResponse>, Error<types::ErrorResponse>> {
            let Self {
                client,
                symbol_id,
                fields,
            } = self;
            let symbol_id = symbol_id.map_err(Error::InvalidRequest)?;
            let fields = fields.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/{}/quotes",
                client.baseurl,
                encode_path(&symbol_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("fields", &fields))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_quote",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_chain`]
    ///
    ///[`Client::get_chain`]: super::Client::get_chain
    #[derive(Debug, Clone)]
    pub struct GetChain<'a> {
        client: &'a super::Client,
        contract_type: Result<Option<types::GetChainContractType>, String>,
        days_to_expiration: Result<Option<i32>, String>,
        entitlement: Result<Option<types::GetChainEntitlement>, String>,
        exp_month: Result<Option<types::GetChainExpMonth>, String>,
        from_date: Result<Option<::chrono::naive::NaiveDate>, String>,
        include_underlying_quote: Result<Option<bool>, String>,
        interest_rate: Result<Option<f64>, String>,
        interval: Result<Option<f64>, String>,
        option_type: Result<Option<::std::string::String>, String>,
        range: Result<Option<::std::string::String>, String>,
        strategy: Result<Option<types::GetChainStrategy>, String>,
        strike: Result<Option<f64>, String>,
        strike_count: Result<Option<i64>, String>,
        symbol: Result<::std::string::String, String>,
        to_date: Result<Option<::chrono::naive::NaiveDate>, String>,
        underlying_price: Result<Option<f64>, String>,
        volatility: Result<Option<f64>, String>,
    }

    impl<'a> GetChain<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                contract_type: Ok(None),
                days_to_expiration: Ok(None),
                entitlement: Ok(None),
                exp_month: Ok(None),
                from_date: Ok(None),
                include_underlying_quote: Ok(None),
                interest_rate: Ok(None),
                interval: Ok(None),
                option_type: Ok(None),
                range: Ok(None),
                strategy: Ok(None),
                strike: Ok(None),
                strike_count: Ok(None),
                symbol: Err("symbol was not initialized".to_string()),
                to_date: Ok(None),
                underlying_price: Ok(None),
                volatility: Ok(None),
            }
        }

        pub fn contract_type<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetChainContractType>,
        {
            self.contract_type = value.try_into().map(Some).map_err(|_| {
                "conversion to `GetChainContractType` for contract_type failed".to_string()
            });
            self
        }

        pub fn days_to_expiration<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.days_to_expiration = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i32` for days_to_expiration failed".to_string());
            self
        }

        pub fn entitlement<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetChainEntitlement>,
        {
            self.entitlement = value.try_into().map(Some).map_err(|_| {
                "conversion to `GetChainEntitlement` for entitlement failed".to_string()
            });
            self
        }

        pub fn exp_month<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetChainExpMonth>,
        {
            self.exp_month = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `GetChainExpMonth` for exp_month failed".to_string());
            self
        }

        pub fn from_date<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::chrono::naive::NaiveDate>,
        {
            self.from_date = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: chrono :: naive :: NaiveDate` for from_date failed".to_string()
            });
            self
        }

        pub fn include_underlying_quote<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.include_underlying_quote = value.try_into().map(Some).map_err(|_| {
                "conversion to `bool` for include_underlying_quote failed".to_string()
            });
            self
        }

        pub fn interest_rate<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<f64>,
        {
            self.interest_rate = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `f64` for interest_rate failed".to_string());
            self
        }

        pub fn interval<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<f64>,
        {
            self.interval = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `f64` for interval failed".to_string());
            self
        }

        pub fn option_type<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.option_type = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for option_type failed".to_string()
            });
            self
        }

        pub fn range<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.range = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for range failed".to_string()
            });
            self
        }

        pub fn strategy<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetChainStrategy>,
        {
            self.strategy = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `GetChainStrategy` for strategy failed".to_string());
            self
        }

        pub fn strike<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<f64>,
        {
            self.strike = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `f64` for strike failed".to_string());
            self
        }

        pub fn strike_count<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.strike_count = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for strike_count failed".to_string());
            self
        }

        pub fn symbol<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.symbol = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for symbol failed".to_string()
            });
            self
        }

        pub fn to_date<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::chrono::naive::NaiveDate>,
        {
            self.to_date = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: chrono :: naive :: NaiveDate` for to_date failed".to_string()
            });
            self
        }

        pub fn underlying_price<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<f64>,
        {
            self.underlying_price = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `f64` for underlying_price failed".to_string());
            self
        }

        pub fn volatility<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<f64>,
        {
            self.volatility = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `f64` for volatility failed".to_string());
            self
        }

        ///Sends a `GET` request to `/chains`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::OptionChain>, Error<types::ErrorResponse>> {
            let Self {
                client,
                contract_type,
                days_to_expiration,
                entitlement,
                exp_month,
                from_date,
                include_underlying_quote,
                interest_rate,
                interval,
                option_type,
                range,
                strategy,
                strike,
                strike_count,
                symbol,
                to_date,
                underlying_price,
                volatility,
            } = self;
            let contract_type = contract_type.map_err(Error::InvalidRequest)?;
            let days_to_expiration = days_to_expiration.map_err(Error::InvalidRequest)?;
            let entitlement = entitlement.map_err(Error::InvalidRequest)?;
            let exp_month = exp_month.map_err(Error::InvalidRequest)?;
            let from_date = from_date.map_err(Error::InvalidRequest)?;
            let include_underlying_quote =
                include_underlying_quote.map_err(Error::InvalidRequest)?;
            let interest_rate = interest_rate.map_err(Error::InvalidRequest)?;
            let interval = interval.map_err(Error::InvalidRequest)?;
            let option_type = option_type.map_err(Error::InvalidRequest)?;
            let range = range.map_err(Error::InvalidRequest)?;
            let strategy = strategy.map_err(Error::InvalidRequest)?;
            let strike = strike.map_err(Error::InvalidRequest)?;
            let strike_count = strike_count.map_err(Error::InvalidRequest)?;
            let symbol = symbol.map_err(Error::InvalidRequest)?;
            let to_date = to_date.map_err(Error::InvalidRequest)?;
            let underlying_price = underlying_price.map_err(Error::InvalidRequest)?;
            let volatility = volatility.map_err(Error::InvalidRequest)?;
            let url = format!("{}/chains", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new(
                    "contractType",
                    &contract_type,
                ))
                .query(&progenitor_client::QueryParam::new(
                    "daysToExpiration",
                    &days_to_expiration,
                ))
                .query(&progenitor_client::QueryParam::new(
                    "entitlement",
                    &entitlement,
                ))
                .query(&progenitor_client::QueryParam::new("expMonth", &exp_month))
                .query(&progenitor_client::QueryParam::new("fromDate", &from_date))
                .query(&progenitor_client::QueryParam::new(
                    "includeUnderlyingQuote",
                    &include_underlying_quote,
                ))
                .query(&progenitor_client::QueryParam::new(
                    "interestRate",
                    &interest_rate,
                ))
                .query(&progenitor_client::QueryParam::new("interval", &interval))
                .query(&progenitor_client::QueryParam::new(
                    "optionType",
                    &option_type,
                ))
                .query(&progenitor_client::QueryParam::new("range", &range))
                .query(&progenitor_client::QueryParam::new("strategy", &strategy))
                .query(&progenitor_client::QueryParam::new("strike", &strike))
                .query(&progenitor_client::QueryParam::new(
                    "strikeCount",
                    &strike_count,
                ))
                .query(&progenitor_client::QueryParam::new("symbol", &symbol))
                .query(&progenitor_client::QueryParam::new("toDate", &to_date))
                .query(&progenitor_client::QueryParam::new(
                    "underlyingPrice",
                    &underlying_price,
                ))
                .query(&progenitor_client::QueryParam::new(
                    "volatility",
                    &volatility,
                ))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_chain",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_expiration_chain`]
    ///
    ///[`Client::get_expiration_chain`]: super::Client::get_expiration_chain
    #[derive(Debug, Clone)]
    pub struct GetExpirationChain<'a> {
        client: &'a super::Client,
        symbol: Result<::std::string::String, String>,
    }

    impl<'a> GetExpirationChain<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                symbol: Err("symbol was not initialized".to_string()),
            }
        }

        pub fn symbol<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.symbol = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for symbol failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/expirationchain`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ExpirationChain>, Error<types::ErrorResponse>> {
            let Self { client, symbol } = self;
            let symbol = symbol.map_err(Error::InvalidRequest)?;
            let url = format!("{}/expirationchain", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("symbol", &symbol))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_expiration_chain",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_price_history`]
    ///
    ///[`Client::get_price_history`]: super::Client::get_price_history
    #[derive(Debug, Clone)]
    pub struct GetPriceHistory<'a> {
        client: &'a super::Client,
        end_date: Result<Option<i64>, String>,
        frequency: Result<Option<i32>, String>,
        frequency_type: Result<Option<types::GetPriceHistoryFrequencyType>, String>,
        need_extended_hours_data: Result<Option<bool>, String>,
        need_previous_close: Result<Option<bool>, String>,
        period: Result<Option<i32>, String>,
        period_type: Result<Option<types::GetPriceHistoryPeriodType>, String>,
        start_date: Result<Option<i64>, String>,
        symbol: Result<::std::string::String, String>,
    }

    impl<'a> GetPriceHistory<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                end_date: Ok(None),
                frequency: Ok(None),
                frequency_type: Ok(None),
                need_extended_hours_data: Ok(None),
                need_previous_close: Ok(None),
                period: Ok(None),
                period_type: Ok(None),
                start_date: Ok(None),
                symbol: Err("symbol was not initialized".to_string()),
            }
        }

        pub fn end_date<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.end_date = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for end_date failed".to_string());
            self
        }

        pub fn frequency<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.frequency = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i32` for frequency failed".to_string());
            self
        }

        pub fn frequency_type<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetPriceHistoryFrequencyType>,
        {
            self.frequency_type = value.try_into().map(Some).map_err(|_| {
                "conversion to `GetPriceHistoryFrequencyType` for frequency_type failed".to_string()
            });
            self
        }

        pub fn need_extended_hours_data<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.need_extended_hours_data = value.try_into().map(Some).map_err(|_| {
                "conversion to `bool` for need_extended_hours_data failed".to_string()
            });
            self
        }

        pub fn need_previous_close<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.need_previous_close = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for need_previous_close failed".to_string());
            self
        }

        pub fn period<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i32>,
        {
            self.period = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i32` for period failed".to_string());
            self
        }

        pub fn period_type<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetPriceHistoryPeriodType>,
        {
            self.period_type = value.try_into().map(Some).map_err(|_| {
                "conversion to `GetPriceHistoryPeriodType` for period_type failed".to_string()
            });
            self
        }

        pub fn start_date<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.start_date = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for start_date failed".to_string());
            self
        }

        pub fn symbol<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.symbol = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for symbol failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/pricehistory`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::CandleList>, Error<types::ErrorResponse>> {
            let Self {
                client,
                end_date,
                frequency,
                frequency_type,
                need_extended_hours_data,
                need_previous_close,
                period,
                period_type,
                start_date,
                symbol,
            } = self;
            let end_date = end_date.map_err(Error::InvalidRequest)?;
            let frequency = frequency.map_err(Error::InvalidRequest)?;
            let frequency_type = frequency_type.map_err(Error::InvalidRequest)?;
            let need_extended_hours_data =
                need_extended_hours_data.map_err(Error::InvalidRequest)?;
            let need_previous_close = need_previous_close.map_err(Error::InvalidRequest)?;
            let period = period.map_err(Error::InvalidRequest)?;
            let period_type = period_type.map_err(Error::InvalidRequest)?;
            let start_date = start_date.map_err(Error::InvalidRequest)?;
            let symbol = symbol.map_err(Error::InvalidRequest)?;
            let url = format!("{}/pricehistory", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("endDate", &end_date))
                .query(&progenitor_client::QueryParam::new("frequency", &frequency))
                .query(&progenitor_client::QueryParam::new(
                    "frequencyType",
                    &frequency_type,
                ))
                .query(&progenitor_client::QueryParam::new(
                    "needExtendedHoursData",
                    &need_extended_hours_data,
                ))
                .query(&progenitor_client::QueryParam::new(
                    "needPreviousClose",
                    &need_previous_close,
                ))
                .query(&progenitor_client::QueryParam::new("period", &period))
                .query(&progenitor_client::QueryParam::new(
                    "periodType",
                    &period_type,
                ))
                .query(&progenitor_client::QueryParam::new(
                    "startDate",
                    &start_date,
                ))
                .query(&progenitor_client::QueryParam::new("symbol", &symbol))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_price_history",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_movers`]
    ///
    ///[`Client::get_movers`]: super::Client::get_movers
    #[derive(Debug, Clone)]
    pub struct GetMovers<'a> {
        client: &'a super::Client,
        symbol_id: Result<types::GetMoversSymbolId, String>,
        frequency: Result<Option<types::GetMoversFrequency>, String>,
        sort: Result<Option<types::GetMoversSort>, String>,
    }

    impl<'a> GetMovers<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                symbol_id: Err("symbol_id was not initialized".to_string()),
                frequency: Ok(None),
                sort: Ok(None),
            }
        }

        pub fn symbol_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetMoversSymbolId>,
        {
            self.symbol_id = value
                .try_into()
                .map_err(|_| "conversion to `GetMoversSymbolId` for symbol_id failed".to_string());
            self
        }

        pub fn frequency<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetMoversFrequency>,
        {
            self.frequency = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `GetMoversFrequency` for frequency failed".to_string());
            self
        }

        pub fn sort<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetMoversSort>,
        {
            self.sort = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `GetMoversSort` for sort failed".to_string());
            self
        }

        ///Sends a `GET` request to `/movers/{symbol_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetMoversResponse>, Error<types::ErrorResponse>> {
            let Self {
                client,
                symbol_id,
                frequency,
                sort,
            } = self;
            let symbol_id = symbol_id.map_err(Error::InvalidRequest)?;
            let frequency = frequency.map_err(Error::InvalidRequest)?;
            let sort = sort.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/movers/{}",
                client.baseurl,
                encode_path(&symbol_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("frequency", &frequency))
                .query(&progenitor_client::QueryParam::new("sort", &sort))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_movers",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_market_hours`]
    ///
    ///[`Client::get_market_hours`]: super::Client::get_market_hours
    #[derive(Debug, Clone)]
    pub struct GetMarketHours<'a> {
        client: &'a super::Client,
        date: Result<Option<::chrono::naive::NaiveDate>, String>,
        markets: Result<Vec<types::GetMarketHoursMarketsItem>, String>,
    }

    impl<'a> GetMarketHours<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                date: Ok(None),
                markets: Err("markets was not initialized".to_string()),
            }
        }

        pub fn date<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::chrono::naive::NaiveDate>,
        {
            self.date = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: chrono :: naive :: NaiveDate` for date failed".to_string()
            });
            self
        }

        pub fn markets<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<Vec<types::GetMarketHoursMarketsItem>>,
        {
            self.markets = value.try_into().map_err(|_| {
                "conversion to `Vec < GetMarketHoursMarketsItem >` for markets failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/markets`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::collections::HashMap<::std::string::String, types::Hours>,
                >,
            >,
            Error<types::ErrorResponse>,
        > {
            let Self {
                client,
                date,
                markets,
            } = self;
            let date = date.map_err(Error::InvalidRequest)?;
            let markets = markets.map_err(Error::InvalidRequest)?;
            let url = format!("{}/markets", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("date", &date))
                .query(&progenitor_client::QueryParam::new("markets", &markets))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_market_hours",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_market_hour`]
    ///
    ///[`Client::get_market_hour`]: super::Client::get_market_hour
    #[derive(Debug, Clone)]
    pub struct GetMarketHour<'a> {
        client: &'a super::Client,
        market_id: Result<types::GetMarketHourMarketId, String>,
        date: Result<Option<::chrono::naive::NaiveDate>, String>,
    }

    impl<'a> GetMarketHour<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                market_id: Err("market_id was not initialized".to_string()),
                date: Ok(None),
            }
        }

        pub fn market_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetMarketHourMarketId>,
        {
            self.market_id = value.try_into().map_err(|_| {
                "conversion to `GetMarketHourMarketId` for market_id failed".to_string()
            });
            self
        }

        pub fn date<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::chrono::naive::NaiveDate>,
        {
            self.date = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: chrono :: naive :: NaiveDate` for date failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/markets/{market_id}`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::collections::HashMap<::std::string::String, types::Hours>,
                >,
            >,
            Error<types::ErrorResponse>,
        > {
            let Self {
                client,
                market_id,
                date,
            } = self;
            let market_id = market_id.map_err(Error::InvalidRequest)?;
            let date = date.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/markets/{}",
                client.baseurl,
                encode_path(&market_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("date", &date))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_market_hour",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_instruments`]
    ///
    ///[`Client::get_instruments`]: super::Client::get_instruments
    #[derive(Debug, Clone)]
    pub struct GetInstruments<'a> {
        client: &'a super::Client,
        projection: Result<types::GetInstrumentsProjection, String>,
        symbol: Result<::std::string::String, String>,
    }

    impl<'a> GetInstruments<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                projection: Err("projection was not initialized".to_string()),
                symbol: Err("symbol was not initialized".to_string()),
            }
        }

        pub fn projection<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GetInstrumentsProjection>,
        {
            self.projection = value.try_into().map_err(|_| {
                "conversion to `GetInstrumentsProjection` for projection failed".to_string()
            });
            self
        }

        pub fn symbol<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.symbol = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for symbol failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/instruments`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetInstrumentsResponse>, Error<types::ErrorResponse>>
        {
            let Self {
                client,
                projection,
                symbol,
            } = self;
            let projection = projection.map_err(Error::InvalidRequest)?;
            let symbol = symbol.map_err(Error::InvalidRequest)?;
            let url = format!("{}/instruments", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new(
                    "projection",
                    &projection,
                ))
                .query(&progenitor_client::QueryParam::new("symbol", &symbol))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_instruments",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_instruments_by_cusip`]
    ///
    ///[`Client::get_instruments_by_cusip`]: super::Client::get_instruments_by_cusip
    #[derive(Debug, Clone)]
    pub struct GetInstrumentsByCusip<'a> {
        client: &'a super::Client,
        cusip_id: Result<::std::string::String, String>,
    }

    impl<'a> GetInstrumentsByCusip<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                cusip_id: Err("cusip_id was not initialized".to_string()),
            }
        }

        pub fn cusip_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.cusip_id = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for cusip_id failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/instruments/{cusip_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::InstrumentResponse>, Error<types::ErrorResponse>> {
            let Self { client, cusip_id } = self;
            let cusip_id = cusip_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/instruments/{}",
                client.baseurl,
                encode_path(&cusip_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_instruments_by_cusip",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    pub use self::super::Client;
}
