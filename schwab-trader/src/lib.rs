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

    ///`Account`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "securitiesAccount": {
    ///      "$ref": "#/components/schemas/SecuritiesAccount"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Account {
        #[serde(
            rename = "securitiesAccount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub securities_account: ::std::option::Option<SecuritiesAccount>,
    }

    impl ::std::default::Default for Account {
        fn default() -> Self {
            Self {
                securities_account: Default::default(),
            }
        }
    }

    impl Account {
        pub fn builder() -> builder::Account {
            Default::default()
        }
    }

    ///`AccountApiOptionDeliverable`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "apiCurrencyType": {
    ///      "type": "string",
    ///      "enum": [
    ///        "USD",
    ///        "CAD",
    ///        "EUR",
    ///        "JPY"
    ///      ]
    ///    },
    ///    "assetType": {
    ///      "$ref": "#/components/schemas/assetType"
    ///    },
    ///    "deliverableUnits": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "symbol": {
    ///      "type": "string",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AccountApiOptionDeliverable {
        #[serde(
            rename = "apiCurrencyType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub api_currency_type: ::std::option::Option<AccountApiOptionDeliverableApiCurrencyType>,
        #[serde(
            rename = "assetType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub asset_type: ::std::option::Option<AssetType>,
        #[serde(
            rename = "deliverableUnits",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub deliverable_units: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for AccountApiOptionDeliverable {
        fn default() -> Self {
            Self {
                api_currency_type: Default::default(),
                asset_type: Default::default(),
                deliverable_units: Default::default(),
                symbol: Default::default(),
            }
        }
    }

    impl AccountApiOptionDeliverable {
        pub fn builder() -> builder::AccountApiOptionDeliverable {
            Default::default()
        }
    }

    ///`AccountApiOptionDeliverableApiCurrencyType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "USD",
    ///    "CAD",
    ///    "EUR",
    ///    "JPY"
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
    pub enum AccountApiOptionDeliverableApiCurrencyType {
        #[serde(rename = "USD")]
        Usd,
        #[serde(rename = "CAD")]
        Cad,
        #[serde(rename = "EUR")]
        Eur,
        #[serde(rename = "JPY")]
        Jpy,
    }

    impl ::std::fmt::Display for AccountApiOptionDeliverableApiCurrencyType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Usd => f.write_str("USD"),
                Self::Cad => f.write_str("CAD"),
                Self::Eur => f.write_str("EUR"),
                Self::Jpy => f.write_str("JPY"),
            }
        }
    }

    impl ::std::str::FromStr for AccountApiOptionDeliverableApiCurrencyType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "USD" => Ok(Self::Usd),
                "CAD" => Ok(Self::Cad),
                "EUR" => Ok(Self::Eur),
                "JPY" => Ok(Self::Jpy),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AccountApiOptionDeliverableApiCurrencyType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String>
        for AccountApiOptionDeliverableApiCurrencyType
    {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AccountApiOptionDeliverableApiCurrencyType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AccountCashEquivalent`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AccountsBaseInstrument"
    ///    }
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "SWEEP_VEHICLE",
    ///        "SAVINGS",
    ///        "MONEY_MARKET_FUND",
    ///        "UNKNOWN"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AccountCashEquivalent {
        #[serde(rename = "assetType")]
        pub asset_type: AccountCashEquivalentAssetType,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "instrumentId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub instrument_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::serde_json::Value::is_null")]
        pub name: ::serde_json::Value,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<AccountCashEquivalentType>,
    }

    impl AccountCashEquivalent {
        pub fn builder() -> builder::AccountCashEquivalent {
            Default::default()
        }
    }

    ///`AccountCashEquivalentAssetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EQUITY",
    ///    "OPTION",
    ///    "INDEX",
    ///    "MUTUAL_FUND",
    ///    "CASH_EQUIVALENT",
    ///    "FIXED_INCOME",
    ///    "CURRENCY",
    ///    "COLLECTIVE_INVESTMENT"
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
    pub enum AccountCashEquivalentAssetType {
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "CASH_EQUIVALENT")]
        CashEquivalent,
        #[serde(rename = "FIXED_INCOME")]
        FixedIncome,
        #[serde(rename = "CURRENCY")]
        Currency,
        #[serde(rename = "COLLECTIVE_INVESTMENT")]
        CollectiveInvestment,
    }

    impl ::std::fmt::Display for AccountCashEquivalentAssetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Equity => f.write_str("EQUITY"),
                Self::Option => f.write_str("OPTION"),
                Self::Index => f.write_str("INDEX"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::CashEquivalent => f.write_str("CASH_EQUIVALENT"),
                Self::FixedIncome => f.write_str("FIXED_INCOME"),
                Self::Currency => f.write_str("CURRENCY"),
                Self::CollectiveInvestment => f.write_str("COLLECTIVE_INVESTMENT"),
            }
        }
    }

    impl ::std::str::FromStr for AccountCashEquivalentAssetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EQUITY" => Ok(Self::Equity),
                "OPTION" => Ok(Self::Option),
                "INDEX" => Ok(Self::Index),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "CASH_EQUIVALENT" => Ok(Self::CashEquivalent),
                "FIXED_INCOME" => Ok(Self::FixedIncome),
                "CURRENCY" => Ok(Self::Currency),
                "COLLECTIVE_INVESTMENT" => Ok(Self::CollectiveInvestment),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AccountCashEquivalentAssetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AccountCashEquivalentAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AccountCashEquivalentAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AccountCashEquivalentType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "SWEEP_VEHICLE",
    ///    "SAVINGS",
    ///    "MONEY_MARKET_FUND",
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
    pub enum AccountCashEquivalentType {
        #[serde(rename = "SWEEP_VEHICLE")]
        SweepVehicle,
        #[serde(rename = "SAVINGS")]
        Savings,
        #[serde(rename = "MONEY_MARKET_FUND")]
        MoneyMarketFund,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for AccountCashEquivalentType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::SweepVehicle => f.write_str("SWEEP_VEHICLE"),
                Self::Savings => f.write_str("SAVINGS"),
                Self::MoneyMarketFund => f.write_str("MONEY_MARKET_FUND"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for AccountCashEquivalentType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "SWEEP_VEHICLE" => Ok(Self::SweepVehicle),
                "SAVINGS" => Ok(Self::Savings),
                "MONEY_MARKET_FUND" => Ok(Self::MoneyMarketFund),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AccountCashEquivalentType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AccountCashEquivalentType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AccountCashEquivalentType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AccountEquity`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AccountsBaseInstrument"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct AccountEquity(pub AccountsBaseInstrument);
    impl ::std::ops::Deref for AccountEquity {
        type Target = AccountsBaseInstrument;
        fn deref(&self) -> &AccountsBaseInstrument {
            &self.0
        }
    }

    impl ::std::convert::From<AccountEquity> for AccountsBaseInstrument {
        fn from(value: AccountEquity) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<AccountsBaseInstrument> for AccountEquity {
        fn from(value: AccountsBaseInstrument) -> Self {
            Self(value)
        }
    }

    ///`AccountFixedIncome`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AccountsBaseInstrument"
    ///    }
    ///  ],
    ///  "properties": {
    ///    "factor": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "maturityDate": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "variableRate": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AccountFixedIncome {
        #[serde(rename = "assetType")]
        pub asset_type: AccountFixedIncomeAssetType,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub factor: ::std::option::Option<f64>,
        #[serde(
            rename = "instrumentId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub instrument_id: ::std::option::Option<i64>,
        #[serde(
            rename = "maturityDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub maturity_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::serde_json::Value::is_null")]
        pub name: ::serde_json::Value,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "variableRate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub variable_rate: ::std::option::Option<f64>,
    }

    impl AccountFixedIncome {
        pub fn builder() -> builder::AccountFixedIncome {
            Default::default()
        }
    }

    ///`AccountFixedIncomeAssetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EQUITY",
    ///    "OPTION",
    ///    "INDEX",
    ///    "MUTUAL_FUND",
    ///    "CASH_EQUIVALENT",
    ///    "FIXED_INCOME",
    ///    "CURRENCY",
    ///    "COLLECTIVE_INVESTMENT"
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
    pub enum AccountFixedIncomeAssetType {
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "CASH_EQUIVALENT")]
        CashEquivalent,
        #[serde(rename = "FIXED_INCOME")]
        FixedIncome,
        #[serde(rename = "CURRENCY")]
        Currency,
        #[serde(rename = "COLLECTIVE_INVESTMENT")]
        CollectiveInvestment,
    }

    impl ::std::fmt::Display for AccountFixedIncomeAssetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Equity => f.write_str("EQUITY"),
                Self::Option => f.write_str("OPTION"),
                Self::Index => f.write_str("INDEX"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::CashEquivalent => f.write_str("CASH_EQUIVALENT"),
                Self::FixedIncome => f.write_str("FIXED_INCOME"),
                Self::Currency => f.write_str("CURRENCY"),
                Self::CollectiveInvestment => f.write_str("COLLECTIVE_INVESTMENT"),
            }
        }
    }

    impl ::std::str::FromStr for AccountFixedIncomeAssetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EQUITY" => Ok(Self::Equity),
                "OPTION" => Ok(Self::Option),
                "INDEX" => Ok(Self::Index),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "CASH_EQUIVALENT" => Ok(Self::CashEquivalent),
                "FIXED_INCOME" => Ok(Self::FixedIncome),
                "CURRENCY" => Ok(Self::Currency),
                "COLLECTIVE_INVESTMENT" => Ok(Self::CollectiveInvestment),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AccountFixedIncomeAssetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AccountFixedIncomeAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AccountFixedIncomeAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AccountMutualFund`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AccountsBaseInstrument"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct AccountMutualFund(pub AccountsBaseInstrument);
    impl ::std::ops::Deref for AccountMutualFund {
        type Target = AccountsBaseInstrument;
        fn deref(&self) -> &AccountsBaseInstrument {
            &self.0
        }
    }

    impl ::std::convert::From<AccountMutualFund> for AccountsBaseInstrument {
        fn from(value: AccountMutualFund) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<AccountsBaseInstrument> for AccountMutualFund {
        fn from(value: AccountsBaseInstrument) -> Self {
            Self(value)
        }
    }

    ///`AccountNumberHash`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "accountNumber": {
    ///      "type": "string"
    ///    },
    ///    "hashValue": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AccountNumberHash {
        #[serde(
            rename = "accountNumber",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub account_number: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "hashValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub hash_value: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for AccountNumberHash {
        fn default() -> Self {
            Self {
                account_number: Default::default(),
                hash_value: Default::default(),
            }
        }
    }

    impl AccountNumberHash {
        pub fn builder() -> builder::AccountNumberHash {
            Default::default()
        }
    }

    ///`AccountOption`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AccountsBaseInstrument"
    ///    }
    ///  ],
    ///  "properties": {
    ///    "optionDeliverables": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AccountAPIOptionDeliverable"
    ///      }
    ///    },
    ///    "optionMultiplier": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "putCall": {
    ///      "type": "string",
    ///      "enum": [
    ///        "PUT",
    ///        "CALL",
    ///        "UNKNOWN"
    ///      ]
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "VANILLA",
    ///        "BINARY",
    ///        "BARRIER",
    ///        "UNKNOWN"
    ///      ]
    ///    },
    ///    "underlyingSymbol": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AccountOption {
        #[serde(rename = "assetType")]
        pub asset_type: AccountOptionAssetType,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "instrumentId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub instrument_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::serde_json::Value::is_null")]
        pub name: ::serde_json::Value,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(
            rename = "optionDeliverables",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub option_deliverables: ::std::vec::Vec<AccountApiOptionDeliverable>,
        #[serde(
            rename = "optionMultiplier",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub option_multiplier: ::std::option::Option<i32>,
        #[serde(
            rename = "putCall",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub put_call: ::std::option::Option<AccountOptionPutCall>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<AccountOptionType>,
        #[serde(
            rename = "underlyingSymbol",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub underlying_symbol: ::std::option::Option<::std::string::String>,
    }

    impl AccountOption {
        pub fn builder() -> builder::AccountOption {
            Default::default()
        }
    }

    ///`AccountOptionAssetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EQUITY",
    ///    "OPTION",
    ///    "INDEX",
    ///    "MUTUAL_FUND",
    ///    "CASH_EQUIVALENT",
    ///    "FIXED_INCOME",
    ///    "CURRENCY",
    ///    "COLLECTIVE_INVESTMENT"
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
    pub enum AccountOptionAssetType {
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "CASH_EQUIVALENT")]
        CashEquivalent,
        #[serde(rename = "FIXED_INCOME")]
        FixedIncome,
        #[serde(rename = "CURRENCY")]
        Currency,
        #[serde(rename = "COLLECTIVE_INVESTMENT")]
        CollectiveInvestment,
    }

    impl ::std::fmt::Display for AccountOptionAssetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Equity => f.write_str("EQUITY"),
                Self::Option => f.write_str("OPTION"),
                Self::Index => f.write_str("INDEX"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::CashEquivalent => f.write_str("CASH_EQUIVALENT"),
                Self::FixedIncome => f.write_str("FIXED_INCOME"),
                Self::Currency => f.write_str("CURRENCY"),
                Self::CollectiveInvestment => f.write_str("COLLECTIVE_INVESTMENT"),
            }
        }
    }

    impl ::std::str::FromStr for AccountOptionAssetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EQUITY" => Ok(Self::Equity),
                "OPTION" => Ok(Self::Option),
                "INDEX" => Ok(Self::Index),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "CASH_EQUIVALENT" => Ok(Self::CashEquivalent),
                "FIXED_INCOME" => Ok(Self::FixedIncome),
                "CURRENCY" => Ok(Self::Currency),
                "COLLECTIVE_INVESTMENT" => Ok(Self::CollectiveInvestment),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AccountOptionAssetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AccountOptionAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AccountOptionAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AccountOptionPutCall`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "PUT",
    ///    "CALL",
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
    pub enum AccountOptionPutCall {
        #[serde(rename = "PUT")]
        Put,
        #[serde(rename = "CALL")]
        Call,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for AccountOptionPutCall {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Put => f.write_str("PUT"),
                Self::Call => f.write_str("CALL"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for AccountOptionPutCall {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "PUT" => Ok(Self::Put),
                "CALL" => Ok(Self::Call),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AccountOptionPutCall {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AccountOptionPutCall {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AccountOptionPutCall {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AccountOptionType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "VANILLA",
    ///    "BINARY",
    ///    "BARRIER",
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
    pub enum AccountOptionType {
        #[serde(rename = "VANILLA")]
        Vanilla,
        #[serde(rename = "BINARY")]
        Binary,
        #[serde(rename = "BARRIER")]
        Barrier,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for AccountOptionType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Vanilla => f.write_str("VANILLA"),
                Self::Binary => f.write_str("BINARY"),
                Self::Barrier => f.write_str("BARRIER"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for AccountOptionType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "VANILLA" => Ok(Self::Vanilla),
                "BINARY" => Ok(Self::Binary),
                "BARRIER" => Ok(Self::Barrier),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AccountOptionType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AccountOptionType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AccountOptionType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AccountsBaseInstrument`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "assetType",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "assetType": {
    ///      "type": "string",
    ///      "enum": [
    ///        "EQUITY",
    ///        "OPTION",
    ///        "INDEX",
    ///        "MUTUAL_FUND",
    ///        "CASH_EQUIVALENT",
    ///        "FIXED_INCOME",
    ///        "CURRENCY",
    ///        "COLLECTIVE_INVESTMENT"
    ///      ]
    ///    },
    ///    "cusip": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "instrumentId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "netChange": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct AccountsBaseInstrument {
        #[serde(rename = "assetType")]
        pub asset_type: AccountsBaseInstrumentAssetType,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "instrumentId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub instrument_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::serde_json::Value::is_null")]
        pub name: ::serde_json::Value,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
    }

    impl AccountsBaseInstrument {
        pub fn builder() -> builder::AccountsBaseInstrument {
            Default::default()
        }
    }

    ///`AccountsBaseInstrumentAssetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EQUITY",
    ///    "OPTION",
    ///    "INDEX",
    ///    "MUTUAL_FUND",
    ///    "CASH_EQUIVALENT",
    ///    "FIXED_INCOME",
    ///    "CURRENCY",
    ///    "COLLECTIVE_INVESTMENT"
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
    pub enum AccountsBaseInstrumentAssetType {
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "CASH_EQUIVALENT")]
        CashEquivalent,
        #[serde(rename = "FIXED_INCOME")]
        FixedIncome,
        #[serde(rename = "CURRENCY")]
        Currency,
        #[serde(rename = "COLLECTIVE_INVESTMENT")]
        CollectiveInvestment,
    }

    impl ::std::fmt::Display for AccountsBaseInstrumentAssetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Equity => f.write_str("EQUITY"),
                Self::Option => f.write_str("OPTION"),
                Self::Index => f.write_str("INDEX"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::CashEquivalent => f.write_str("CASH_EQUIVALENT"),
                Self::FixedIncome => f.write_str("FIXED_INCOME"),
                Self::Currency => f.write_str("CURRENCY"),
                Self::CollectiveInvestment => f.write_str("COLLECTIVE_INVESTMENT"),
            }
        }
    }

    impl ::std::str::FromStr for AccountsBaseInstrumentAssetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EQUITY" => Ok(Self::Equity),
                "OPTION" => Ok(Self::Option),
                "INDEX" => Ok(Self::Index),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "CASH_EQUIVALENT" => Ok(Self::CashEquivalent),
                "FIXED_INCOME" => Ok(Self::FixedIncome),
                "CURRENCY" => Ok(Self::Currency),
                "COLLECTIVE_INVESTMENT" => Ok(Self::CollectiveInvestment),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AccountsBaseInstrumentAssetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AccountsBaseInstrumentAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AccountsBaseInstrumentAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AccountsInstrument`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/AccountCashEquivalent"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AccountEquity"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AccountFixedIncome"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AccountMutualFund"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/AccountOption"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum AccountsInstrument {
        CashEquivalent(AccountCashEquivalent),
        Equity(AccountEquity),
        FixedIncome(AccountFixedIncome),
        MutualFund(AccountMutualFund),
        Option(AccountOption),
    }

    impl ::std::convert::From<AccountCashEquivalent> for AccountsInstrument {
        fn from(value: AccountCashEquivalent) -> Self {
            Self::CashEquivalent(value)
        }
    }

    impl ::std::convert::From<AccountEquity> for AccountsInstrument {
        fn from(value: AccountEquity) -> Self {
            Self::Equity(value)
        }
    }

    impl ::std::convert::From<AccountFixedIncome> for AccountsInstrument {
        fn from(value: AccountFixedIncome) -> Self {
            Self::FixedIncome(value)
        }
    }

    impl ::std::convert::From<AccountMutualFund> for AccountsInstrument {
        fn from(value: AccountMutualFund) -> Self {
            Self::MutualFund(value)
        }
    }

    impl ::std::convert::From<AccountOption> for AccountsInstrument {
        fn from(value: AccountOption) -> Self {
            Self::Option(value)
        }
    }

    ///`AmountIndicator`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "DOLLARS",
    ///    "SHARES",
    ///    "ALL_SHARES",
    ///    "PERCENTAGE",
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
    pub enum AmountIndicator {
        #[serde(rename = "DOLLARS")]
        Dollars,
        #[serde(rename = "SHARES")]
        Shares,
        #[serde(rename = "ALL_SHARES")]
        AllShares,
        #[serde(rename = "PERCENTAGE")]
        Percentage,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for AmountIndicator {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Dollars => f.write_str("DOLLARS"),
                Self::Shares => f.write_str("SHARES"),
                Self::AllShares => f.write_str("ALL_SHARES"),
                Self::Percentage => f.write_str("PERCENTAGE"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for AmountIndicator {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "DOLLARS" => Ok(Self::Dollars),
                "SHARES" => Ok(Self::Shares),
                "ALL_SHARES" => Ok(Self::AllShares),
                "PERCENTAGE" => Ok(Self::Percentage),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AmountIndicator {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AmountIndicator {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AmountIndicator {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`ApiOrderStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "AWAITING_PARENT_ORDER",
    ///    "AWAITING_CONDITION",
    ///    "AWAITING_STOP_CONDITION",
    ///    "AWAITING_MANUAL_REVIEW",
    ///    "ACCEPTED",
    ///    "AWAITING_UR_OUT",
    ///    "PENDING_ACTIVATION",
    ///    "QUEUED",
    ///    "WORKING",
    ///    "REJECTED",
    ///    "PENDING_CANCEL",
    ///    "CANCELED",
    ///    "PENDING_REPLACE",
    ///    "REPLACED",
    ///    "FILLED",
    ///    "EXPIRED",
    ///    "NEW",
    ///    "AWAITING_RELEASE_TIME",
    ///    "PENDING_ACKNOWLEDGEMENT",
    ///    "PENDING_RECALL",
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
    pub enum ApiOrderStatus {
        #[serde(rename = "AWAITING_PARENT_ORDER")]
        AwaitingParentOrder,
        #[serde(rename = "AWAITING_CONDITION")]
        AwaitingCondition,
        #[serde(rename = "AWAITING_STOP_CONDITION")]
        AwaitingStopCondition,
        #[serde(rename = "AWAITING_MANUAL_REVIEW")]
        AwaitingManualReview,
        #[serde(rename = "ACCEPTED")]
        Accepted,
        #[serde(rename = "AWAITING_UR_OUT")]
        AwaitingUrOut,
        #[serde(rename = "PENDING_ACTIVATION")]
        PendingActivation,
        #[serde(rename = "QUEUED")]
        Queued,
        #[serde(rename = "WORKING")]
        Working,
        #[serde(rename = "REJECTED")]
        Rejected,
        #[serde(rename = "PENDING_CANCEL")]
        PendingCancel,
        #[serde(rename = "CANCELED")]
        Canceled,
        #[serde(rename = "PENDING_REPLACE")]
        PendingReplace,
        #[serde(rename = "REPLACED")]
        Replaced,
        #[serde(rename = "FILLED")]
        Filled,
        #[serde(rename = "EXPIRED")]
        Expired,
        #[serde(rename = "NEW")]
        New,
        #[serde(rename = "AWAITING_RELEASE_TIME")]
        AwaitingReleaseTime,
        #[serde(rename = "PENDING_ACKNOWLEDGEMENT")]
        PendingAcknowledgement,
        #[serde(rename = "PENDING_RECALL")]
        PendingRecall,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for ApiOrderStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AwaitingParentOrder => f.write_str("AWAITING_PARENT_ORDER"),
                Self::AwaitingCondition => f.write_str("AWAITING_CONDITION"),
                Self::AwaitingStopCondition => f.write_str("AWAITING_STOP_CONDITION"),
                Self::AwaitingManualReview => f.write_str("AWAITING_MANUAL_REVIEW"),
                Self::Accepted => f.write_str("ACCEPTED"),
                Self::AwaitingUrOut => f.write_str("AWAITING_UR_OUT"),
                Self::PendingActivation => f.write_str("PENDING_ACTIVATION"),
                Self::Queued => f.write_str("QUEUED"),
                Self::Working => f.write_str("WORKING"),
                Self::Rejected => f.write_str("REJECTED"),
                Self::PendingCancel => f.write_str("PENDING_CANCEL"),
                Self::Canceled => f.write_str("CANCELED"),
                Self::PendingReplace => f.write_str("PENDING_REPLACE"),
                Self::Replaced => f.write_str("REPLACED"),
                Self::Filled => f.write_str("FILLED"),
                Self::Expired => f.write_str("EXPIRED"),
                Self::New => f.write_str("NEW"),
                Self::AwaitingReleaseTime => f.write_str("AWAITING_RELEASE_TIME"),
                Self::PendingAcknowledgement => f.write_str("PENDING_ACKNOWLEDGEMENT"),
                Self::PendingRecall => f.write_str("PENDING_RECALL"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for ApiOrderStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "AWAITING_PARENT_ORDER" => Ok(Self::AwaitingParentOrder),
                "AWAITING_CONDITION" => Ok(Self::AwaitingCondition),
                "AWAITING_STOP_CONDITION" => Ok(Self::AwaitingStopCondition),
                "AWAITING_MANUAL_REVIEW" => Ok(Self::AwaitingManualReview),
                "ACCEPTED" => Ok(Self::Accepted),
                "AWAITING_UR_OUT" => Ok(Self::AwaitingUrOut),
                "PENDING_ACTIVATION" => Ok(Self::PendingActivation),
                "QUEUED" => Ok(Self::Queued),
                "WORKING" => Ok(Self::Working),
                "REJECTED" => Ok(Self::Rejected),
                "PENDING_CANCEL" => Ok(Self::PendingCancel),
                "CANCELED" => Ok(Self::Canceled),
                "PENDING_REPLACE" => Ok(Self::PendingReplace),
                "REPLACED" => Ok(Self::Replaced),
                "FILLED" => Ok(Self::Filled),
                "EXPIRED" => Ok(Self::Expired),
                "NEW" => Ok(Self::New),
                "AWAITING_RELEASE_TIME" => Ok(Self::AwaitingReleaseTime),
                "PENDING_ACKNOWLEDGEMENT" => Ok(Self::PendingAcknowledgement),
                "PENDING_RECALL" => Ok(Self::PendingRecall),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ApiOrderStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ApiOrderStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ApiOrderStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`ApiRuleAction`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ACCEPT",
    ///    "ALERT",
    ///    "REJECT",
    ///    "REVIEW",
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
    pub enum ApiRuleAction {
        #[serde(rename = "ACCEPT")]
        Accept,
        #[serde(rename = "ALERT")]
        Alert,
        #[serde(rename = "REJECT")]
        Reject,
        #[serde(rename = "REVIEW")]
        Review,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for ApiRuleAction {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Accept => f.write_str("ACCEPT"),
                Self::Alert => f.write_str("ALERT"),
                Self::Reject => f.write_str("REJECT"),
                Self::Review => f.write_str("REVIEW"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for ApiRuleAction {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ACCEPT" => Ok(Self::Accept),
                "ALERT" => Ok(Self::Alert),
                "REJECT" => Ok(Self::Reject),
                "REVIEW" => Ok(Self::Review),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ApiRuleAction {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ApiRuleAction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ApiRuleAction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`AssetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EQUITY",
    ///    "MUTUAL_FUND",
    ///    "OPTION",
    ///    "FUTURE",
    ///    "FOREX",
    ///    "INDEX",
    ///    "CASH_EQUIVALENT",
    ///    "FIXED_INCOME",
    ///    "PRODUCT",
    ///    "CURRENCY",
    ///    "COLLECTIVE_INVESTMENT"
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
    pub enum AssetType {
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "FUTURE")]
        Future,
        #[serde(rename = "FOREX")]
        Forex,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "CASH_EQUIVALENT")]
        CashEquivalent,
        #[serde(rename = "FIXED_INCOME")]
        FixedIncome,
        #[serde(rename = "PRODUCT")]
        Product,
        #[serde(rename = "CURRENCY")]
        Currency,
        #[serde(rename = "COLLECTIVE_INVESTMENT")]
        CollectiveInvestment,
    }

    impl ::std::fmt::Display for AssetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Equity => f.write_str("EQUITY"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::Option => f.write_str("OPTION"),
                Self::Future => f.write_str("FUTURE"),
                Self::Forex => f.write_str("FOREX"),
                Self::Index => f.write_str("INDEX"),
                Self::CashEquivalent => f.write_str("CASH_EQUIVALENT"),
                Self::FixedIncome => f.write_str("FIXED_INCOME"),
                Self::Product => f.write_str("PRODUCT"),
                Self::Currency => f.write_str("CURRENCY"),
                Self::CollectiveInvestment => f.write_str("COLLECTIVE_INVESTMENT"),
            }
        }
    }

    impl ::std::str::FromStr for AssetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EQUITY" => Ok(Self::Equity),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "OPTION" => Ok(Self::Option),
                "FUTURE" => Ok(Self::Future),
                "FOREX" => Ok(Self::Forex),
                "INDEX" => Ok(Self::Index),
                "CASH_EQUIVALENT" => Ok(Self::CashEquivalent),
                "FIXED_INCOME" => Ok(Self::FixedIncome),
                "PRODUCT" => Ok(Self::Product),
                "CURRENCY" => Ok(Self::Currency),
                "COLLECTIVE_INVESTMENT" => Ok(Self::CollectiveInvestment),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for AssetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for AssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for AssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`CashAccount`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SecuritiesAccountBase"
    ///    }
    ///  ],
    ///  "properties": {
    ///    "currentBalances": {
    ///      "$ref": "#/components/schemas/CashBalance"
    ///    },
    ///    "initialBalances": {
    ///      "$ref": "#/components/schemas/CashInitialBalance"
    ///    },
    ///    "projectedBalances": {
    ///      "$ref": "#/components/schemas/CashBalance"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CashAccount {
        #[serde(
            rename = "accountNumber",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub account_number: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "currentBalances",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub current_balances: ::std::option::Option<CashBalance>,
        #[serde(
            rename = "initialBalances",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub initial_balances: ::std::option::Option<CashInitialBalance>,
        #[serde(rename = "isClosingOnlyRestricted", default)]
        pub is_closing_only_restricted: bool,
        #[serde(rename = "isDayTrader", default)]
        pub is_day_trader: bool,
        #[serde(rename = "pfcbFlag", default)]
        pub pfcb_flag: bool,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub positions: ::std::vec::Vec<Position>,
        #[serde(
            rename = "projectedBalances",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub projected_balances: ::std::option::Option<CashBalance>,
        #[serde(
            rename = "roundTrips",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub round_trips: ::std::option::Option<i32>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<CashAccountType>,
    }

    impl ::std::default::Default for CashAccount {
        fn default() -> Self {
            Self {
                account_number: Default::default(),
                current_balances: Default::default(),
                initial_balances: Default::default(),
                is_closing_only_restricted: Default::default(),
                is_day_trader: Default::default(),
                pfcb_flag: Default::default(),
                positions: Default::default(),
                projected_balances: Default::default(),
                round_trips: Default::default(),
                type_: Default::default(),
            }
        }
    }

    impl CashAccount {
        pub fn builder() -> builder::CashAccount {
            Default::default()
        }
    }

    ///`CashAccountType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "CASH",
    ///    "MARGIN"
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
    pub enum CashAccountType {
        #[serde(rename = "CASH")]
        Cash,
        #[serde(rename = "MARGIN")]
        Margin,
    }

    impl ::std::fmt::Display for CashAccountType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Cash => f.write_str("CASH"),
                Self::Margin => f.write_str("MARGIN"),
            }
        }
    }

    impl ::std::str::FromStr for CashAccountType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "CASH" => Ok(Self::Cash),
                "MARGIN" => Ok(Self::Margin),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for CashAccountType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for CashAccountType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CashAccountType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`CashBalance`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "cashAvailableForTrading": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "cashAvailableForWithdrawal": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "cashCall": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "cashDebitCallValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "longNonMarginableMarketValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "totalCash": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "unsettledCash": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CashBalance {
        #[serde(
            rename = "cashAvailableForTrading",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cash_available_for_trading: ::std::option::Option<f64>,
        #[serde(
            rename = "cashAvailableForWithdrawal",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cash_available_for_withdrawal: ::std::option::Option<f64>,
        #[serde(
            rename = "cashCall",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cash_call: ::std::option::Option<f64>,
        #[serde(
            rename = "cashDebitCallValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cash_debit_call_value: ::std::option::Option<f64>,
        #[serde(
            rename = "longNonMarginableMarketValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub long_non_marginable_market_value: ::std::option::Option<f64>,
        #[serde(
            rename = "totalCash",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_cash: ::std::option::Option<f64>,
        #[serde(
            rename = "unsettledCash",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub unsettled_cash: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for CashBalance {
        fn default() -> Self {
            Self {
                cash_available_for_trading: Default::default(),
                cash_available_for_withdrawal: Default::default(),
                cash_call: Default::default(),
                cash_debit_call_value: Default::default(),
                long_non_marginable_market_value: Default::default(),
                total_cash: Default::default(),
                unsettled_cash: Default::default(),
            }
        }
    }

    impl CashBalance {
        pub fn builder() -> builder::CashBalance {
            Default::default()
        }
    }

    ///`CashInitialBalance`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "accountValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "accruedInterest": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "bondValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "cashAvailableForTrading": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "cashAvailableForWithdrawal": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "cashBalance": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "cashDebitCallValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "cashReceipts": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "isInCall": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "liquidationValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "longOptionMarketValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "longStockValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "moneyMarketFund": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "mutualFundValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "pendingDeposits": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "shortOptionMarketValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "shortStockValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "unsettledCash": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CashInitialBalance {
        #[serde(
            rename = "accountValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub account_value: ::std::option::Option<f64>,
        #[serde(
            rename = "accruedInterest",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub accrued_interest: ::std::option::Option<f64>,
        #[serde(
            rename = "bondValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bond_value: ::std::option::Option<f64>,
        #[serde(
            rename = "cashAvailableForTrading",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cash_available_for_trading: ::std::option::Option<f64>,
        #[serde(
            rename = "cashAvailableForWithdrawal",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cash_available_for_withdrawal: ::std::option::Option<f64>,
        #[serde(
            rename = "cashBalance",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cash_balance: ::std::option::Option<f64>,
        #[serde(
            rename = "cashDebitCallValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cash_debit_call_value: ::std::option::Option<f64>,
        #[serde(
            rename = "cashReceipts",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cash_receipts: ::std::option::Option<f64>,
        #[serde(
            rename = "isInCall",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_in_call: ::std::option::Option<bool>,
        #[serde(
            rename = "liquidationValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub liquidation_value: ::std::option::Option<f64>,
        #[serde(
            rename = "longOptionMarketValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub long_option_market_value: ::std::option::Option<f64>,
        #[serde(
            rename = "longStockValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub long_stock_value: ::std::option::Option<f64>,
        #[serde(
            rename = "moneyMarketFund",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub money_market_fund: ::std::option::Option<f64>,
        #[serde(
            rename = "mutualFundValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mutual_fund_value: ::std::option::Option<f64>,
        #[serde(
            rename = "pendingDeposits",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub pending_deposits: ::std::option::Option<f64>,
        #[serde(
            rename = "shortOptionMarketValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short_option_market_value: ::std::option::Option<f64>,
        #[serde(
            rename = "shortStockValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short_stock_value: ::std::option::Option<f64>,
        #[serde(
            rename = "unsettledCash",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub unsettled_cash: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for CashInitialBalance {
        fn default() -> Self {
            Self {
                account_value: Default::default(),
                accrued_interest: Default::default(),
                bond_value: Default::default(),
                cash_available_for_trading: Default::default(),
                cash_available_for_withdrawal: Default::default(),
                cash_balance: Default::default(),
                cash_debit_call_value: Default::default(),
                cash_receipts: Default::default(),
                is_in_call: Default::default(),
                liquidation_value: Default::default(),
                long_option_market_value: Default::default(),
                long_stock_value: Default::default(),
                money_market_fund: Default::default(),
                mutual_fund_value: Default::default(),
                pending_deposits: Default::default(),
                short_option_market_value: Default::default(),
                short_stock_value: Default::default(),
                unsettled_cash: Default::default(),
            }
        }
    }

    impl CashInitialBalance {
        pub fn builder() -> builder::CashInitialBalance {
            Default::default()
        }
    }

    ///`CollectiveInvestment`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/TransactionBaseInstrument"
    ///    }
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "UNIT_INVESTMENT_TRUST",
    ///        "EXCHANGE_TRADED_FUND",
    ///        "CLOSED_END_FUND",
    ///        "INDEX",
    ///        "UNITS"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CollectiveInvestment {
        #[serde(rename = "assetType")]
        pub asset_type: CollectiveInvestmentAssetType,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "instrumentId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub instrument_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::serde_json::Value::is_null")]
        pub name: ::serde_json::Value,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<CollectiveInvestmentType>,
    }

    impl CollectiveInvestment {
        pub fn builder() -> builder::CollectiveInvestment {
            Default::default()
        }
    }

    ///`CollectiveInvestmentAssetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EQUITY",
    ///    "OPTION",
    ///    "INDEX",
    ///    "MUTUAL_FUND",
    ///    "CASH_EQUIVALENT",
    ///    "FIXED_INCOME",
    ///    "CURRENCY",
    ///    "COLLECTIVE_INVESTMENT"
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
    pub enum CollectiveInvestmentAssetType {
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "CASH_EQUIVALENT")]
        CashEquivalent,
        #[serde(rename = "FIXED_INCOME")]
        FixedIncome,
        #[serde(rename = "CURRENCY")]
        Currency,
        #[serde(rename = "COLLECTIVE_INVESTMENT")]
        CollectiveInvestment,
    }

    impl ::std::fmt::Display for CollectiveInvestmentAssetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Equity => f.write_str("EQUITY"),
                Self::Option => f.write_str("OPTION"),
                Self::Index => f.write_str("INDEX"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::CashEquivalent => f.write_str("CASH_EQUIVALENT"),
                Self::FixedIncome => f.write_str("FIXED_INCOME"),
                Self::Currency => f.write_str("CURRENCY"),
                Self::CollectiveInvestment => f.write_str("COLLECTIVE_INVESTMENT"),
            }
        }
    }

    impl ::std::str::FromStr for CollectiveInvestmentAssetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EQUITY" => Ok(Self::Equity),
                "OPTION" => Ok(Self::Option),
                "INDEX" => Ok(Self::Index),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "CASH_EQUIVALENT" => Ok(Self::CashEquivalent),
                "FIXED_INCOME" => Ok(Self::FixedIncome),
                "CURRENCY" => Ok(Self::Currency),
                "COLLECTIVE_INVESTMENT" => Ok(Self::CollectiveInvestment),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for CollectiveInvestmentAssetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for CollectiveInvestmentAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CollectiveInvestmentAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`CollectiveInvestmentType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "UNIT_INVESTMENT_TRUST",
    ///    "EXCHANGE_TRADED_FUND",
    ///    "CLOSED_END_FUND",
    ///    "INDEX",
    ///    "UNITS"
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
    pub enum CollectiveInvestmentType {
        #[serde(rename = "UNIT_INVESTMENT_TRUST")]
        UnitInvestmentTrust,
        #[serde(rename = "EXCHANGE_TRADED_FUND")]
        ExchangeTradedFund,
        #[serde(rename = "CLOSED_END_FUND")]
        ClosedEndFund,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "UNITS")]
        Units,
    }

    impl ::std::fmt::Display for CollectiveInvestmentType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::UnitInvestmentTrust => f.write_str("UNIT_INVESTMENT_TRUST"),
                Self::ExchangeTradedFund => f.write_str("EXCHANGE_TRADED_FUND"),
                Self::ClosedEndFund => f.write_str("CLOSED_END_FUND"),
                Self::Index => f.write_str("INDEX"),
                Self::Units => f.write_str("UNITS"),
            }
        }
    }

    impl ::std::str::FromStr for CollectiveInvestmentType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "UNIT_INVESTMENT_TRUST" => Ok(Self::UnitInvestmentTrust),
                "EXCHANGE_TRADED_FUND" => Ok(Self::ExchangeTradedFund),
                "CLOSED_END_FUND" => Ok(Self::ClosedEndFund),
                "INDEX" => Ok(Self::Index),
                "UNITS" => Ok(Self::Units),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for CollectiveInvestmentType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for CollectiveInvestmentType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for CollectiveInvestmentType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`Commission`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "commissionLegs": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CommissionLeg"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Commission {
        #[serde(
            rename = "commissionLegs",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub commission_legs: ::std::vec::Vec<CommissionLeg>,
    }

    impl ::std::default::Default for Commission {
        fn default() -> Self {
            Self {
                commission_legs: Default::default(),
            }
        }
    }

    impl Commission {
        pub fn builder() -> builder::Commission {
            Default::default()
        }
    }

    ///`CommissionAndFee`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "commission": {
    ///      "$ref": "#/components/schemas/Commission"
    ///    },
    ///    "fee": {
    ///      "$ref": "#/components/schemas/Fees"
    ///    },
    ///    "trueCommission": {
    ///      "$ref": "#/components/schemas/Commission"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CommissionAndFee {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub commission: ::std::option::Option<Commission>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub fee: ::std::option::Option<Fees>,
        #[serde(
            rename = "trueCommission",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub true_commission: ::std::option::Option<Commission>,
    }

    impl ::std::default::Default for CommissionAndFee {
        fn default() -> Self {
            Self {
                commission: Default::default(),
                fee: Default::default(),
                true_commission: Default::default(),
            }
        }
    }

    impl CommissionAndFee {
        pub fn builder() -> builder::CommissionAndFee {
            Default::default()
        }
    }

    ///`CommissionLeg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "commissionValues": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CommissionValue"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CommissionLeg {
        #[serde(
            rename = "commissionValues",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub commission_values: ::std::vec::Vec<CommissionValue>,
    }

    impl ::std::default::Default for CommissionLeg {
        fn default() -> Self {
            Self {
                commission_values: Default::default(),
            }
        }
    }

    impl CommissionLeg {
        pub fn builder() -> builder::CommissionLeg {
            Default::default()
        }
    }

    ///`CommissionValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "type": {
    ///      "$ref": "#/components/schemas/FeeType"
    ///    },
    ///    "value": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct CommissionValue {
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<FeeType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for CommissionValue {
        fn default() -> Self {
            Self {
                type_: Default::default(),
                value: Default::default(),
            }
        }
    }

    impl CommissionValue {
        pub fn builder() -> builder::CommissionValue {
            Default::default()
        }
    }

    ///`ComplexOrderStrategyType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NONE",
    ///    "COVERED",
    ///    "VERTICAL",
    ///    "BACK_RATIO",
    ///    "CALENDAR",
    ///    "DIAGONAL",
    ///    "STRADDLE",
    ///    "STRANGLE",
    ///    "COLLAR_SYNTHETIC",
    ///    "BUTTERFLY",
    ///    "CONDOR",
    ///    "IRON_CONDOR",
    ///    "VERTICAL_ROLL",
    ///    "COLLAR_WITH_STOCK",
    ///    "DOUBLE_DIAGONAL",
    ///    "UNBALANCED_BUTTERFLY",
    ///    "UNBALANCED_CONDOR",
    ///    "UNBALANCED_IRON_CONDOR",
    ///    "UNBALANCED_VERTICAL_ROLL",
    ///    "MUTUAL_FUND_SWAP",
    ///    "CUSTOM"
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
    pub enum ComplexOrderStrategyType {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "COVERED")]
        Covered,
        #[serde(rename = "VERTICAL")]
        Vertical,
        #[serde(rename = "BACK_RATIO")]
        BackRatio,
        #[serde(rename = "CALENDAR")]
        Calendar,
        #[serde(rename = "DIAGONAL")]
        Diagonal,
        #[serde(rename = "STRADDLE")]
        Straddle,
        #[serde(rename = "STRANGLE")]
        Strangle,
        #[serde(rename = "COLLAR_SYNTHETIC")]
        CollarSynthetic,
        #[serde(rename = "BUTTERFLY")]
        Butterfly,
        #[serde(rename = "CONDOR")]
        Condor,
        #[serde(rename = "IRON_CONDOR")]
        IronCondor,
        #[serde(rename = "VERTICAL_ROLL")]
        VerticalRoll,
        #[serde(rename = "COLLAR_WITH_STOCK")]
        CollarWithStock,
        #[serde(rename = "DOUBLE_DIAGONAL")]
        DoubleDiagonal,
        #[serde(rename = "UNBALANCED_BUTTERFLY")]
        UnbalancedButterfly,
        #[serde(rename = "UNBALANCED_CONDOR")]
        UnbalancedCondor,
        #[serde(rename = "UNBALANCED_IRON_CONDOR")]
        UnbalancedIronCondor,
        #[serde(rename = "UNBALANCED_VERTICAL_ROLL")]
        UnbalancedVerticalRoll,
        #[serde(rename = "MUTUAL_FUND_SWAP")]
        MutualFundSwap,
        #[serde(rename = "CUSTOM")]
        Custom,
    }

    impl ::std::fmt::Display for ComplexOrderStrategyType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("NONE"),
                Self::Covered => f.write_str("COVERED"),
                Self::Vertical => f.write_str("VERTICAL"),
                Self::BackRatio => f.write_str("BACK_RATIO"),
                Self::Calendar => f.write_str("CALENDAR"),
                Self::Diagonal => f.write_str("DIAGONAL"),
                Self::Straddle => f.write_str("STRADDLE"),
                Self::Strangle => f.write_str("STRANGLE"),
                Self::CollarSynthetic => f.write_str("COLLAR_SYNTHETIC"),
                Self::Butterfly => f.write_str("BUTTERFLY"),
                Self::Condor => f.write_str("CONDOR"),
                Self::IronCondor => f.write_str("IRON_CONDOR"),
                Self::VerticalRoll => f.write_str("VERTICAL_ROLL"),
                Self::CollarWithStock => f.write_str("COLLAR_WITH_STOCK"),
                Self::DoubleDiagonal => f.write_str("DOUBLE_DIAGONAL"),
                Self::UnbalancedButterfly => f.write_str("UNBALANCED_BUTTERFLY"),
                Self::UnbalancedCondor => f.write_str("UNBALANCED_CONDOR"),
                Self::UnbalancedIronCondor => f.write_str("UNBALANCED_IRON_CONDOR"),
                Self::UnbalancedVerticalRoll => f.write_str("UNBALANCED_VERTICAL_ROLL"),
                Self::MutualFundSwap => f.write_str("MUTUAL_FUND_SWAP"),
                Self::Custom => f.write_str("CUSTOM"),
            }
        }
    }

    impl ::std::str::FromStr for ComplexOrderStrategyType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "COVERED" => Ok(Self::Covered),
                "VERTICAL" => Ok(Self::Vertical),
                "BACK_RATIO" => Ok(Self::BackRatio),
                "CALENDAR" => Ok(Self::Calendar),
                "DIAGONAL" => Ok(Self::Diagonal),
                "STRADDLE" => Ok(Self::Straddle),
                "STRANGLE" => Ok(Self::Strangle),
                "COLLAR_SYNTHETIC" => Ok(Self::CollarSynthetic),
                "BUTTERFLY" => Ok(Self::Butterfly),
                "CONDOR" => Ok(Self::Condor),
                "IRON_CONDOR" => Ok(Self::IronCondor),
                "VERTICAL_ROLL" => Ok(Self::VerticalRoll),
                "COLLAR_WITH_STOCK" => Ok(Self::CollarWithStock),
                "DOUBLE_DIAGONAL" => Ok(Self::DoubleDiagonal),
                "UNBALANCED_BUTTERFLY" => Ok(Self::UnbalancedButterfly),
                "UNBALANCED_CONDOR" => Ok(Self::UnbalancedCondor),
                "UNBALANCED_IRON_CONDOR" => Ok(Self::UnbalancedIronCondor),
                "UNBALANCED_VERTICAL_ROLL" => Ok(Self::UnbalancedVerticalRoll),
                "MUTUAL_FUND_SWAP" => Ok(Self::MutualFundSwap),
                "CUSTOM" => Ok(Self::Custom),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ComplexOrderStrategyType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ComplexOrderStrategyType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ComplexOrderStrategyType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`Currency`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/TransactionBaseInstrument"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct Currency(pub TransactionBaseInstrument);
    impl ::std::ops::Deref for Currency {
        type Target = TransactionBaseInstrument;
        fn deref(&self) -> &TransactionBaseInstrument {
            &self.0
        }
    }

    impl ::std::convert::From<Currency> for TransactionBaseInstrument {
        fn from(value: Currency) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<TransactionBaseInstrument> for Currency {
        fn from(value: TransactionBaseInstrument) -> Self {
            Self(value)
        }
    }

    ///`DateParam`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "date": {
    ///      "description": "Valid ISO-8601 format is :<br>
    /// <code>yyyy-MM-dd'T'HH:mm:ss.SSSZ</code>",
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct DateParam {
        ///Valid ISO-8601 format is :<br>
        /// <code>yyyy-MM-dd'T'HH:mm:ss.SSSZ</code>
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub date: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for DateParam {
        fn default() -> Self {
            Self {
                date: Default::default(),
            }
        }
    }

    impl DateParam {
        pub fn builder() -> builder::DateParam {
            Default::default()
        }
    }

    ///`Duration`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "DAY",
    ///    "GOOD_TILL_CANCEL",
    ///    "FILL_OR_KILL",
    ///    "IMMEDIATE_OR_CANCEL",
    ///    "END_OF_WEEK",
    ///    "END_OF_MONTH",
    ///    "NEXT_END_OF_MONTH",
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
    pub enum Duration {
        #[serde(rename = "DAY")]
        Day,
        #[serde(rename = "GOOD_TILL_CANCEL")]
        GoodTillCancel,
        #[serde(rename = "FILL_OR_KILL")]
        FillOrKill,
        #[serde(rename = "IMMEDIATE_OR_CANCEL")]
        ImmediateOrCancel,
        #[serde(rename = "END_OF_WEEK")]
        EndOfWeek,
        #[serde(rename = "END_OF_MONTH")]
        EndOfMonth,
        #[serde(rename = "NEXT_END_OF_MONTH")]
        NextEndOfMonth,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for Duration {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Day => f.write_str("DAY"),
                Self::GoodTillCancel => f.write_str("GOOD_TILL_CANCEL"),
                Self::FillOrKill => f.write_str("FILL_OR_KILL"),
                Self::ImmediateOrCancel => f.write_str("IMMEDIATE_OR_CANCEL"),
                Self::EndOfWeek => f.write_str("END_OF_WEEK"),
                Self::EndOfMonth => f.write_str("END_OF_MONTH"),
                Self::NextEndOfMonth => f.write_str("NEXT_END_OF_MONTH"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for Duration {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "DAY" => Ok(Self::Day),
                "GOOD_TILL_CANCEL" => Ok(Self::GoodTillCancel),
                "FILL_OR_KILL" => Ok(Self::FillOrKill),
                "IMMEDIATE_OR_CANCEL" => Ok(Self::ImmediateOrCancel),
                "END_OF_WEEK" => Ok(Self::EndOfWeek),
                "END_OF_MONTH" => Ok(Self::EndOfMonth),
                "NEXT_END_OF_MONTH" => Ok(Self::NextEndOfMonth),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for Duration {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for Duration {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for Duration {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`ExecutionLeg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "instrumentId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "legId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "mismarkedQuantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "price": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "quantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "time": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ExecutionLeg {
        #[serde(
            rename = "instrumentId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub instrument_id: ::std::option::Option<i64>,
        #[serde(
            rename = "legId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub leg_id: ::std::option::Option<i64>,
        #[serde(
            rename = "mismarkedQuantity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mismarked_quantity: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quantity: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    }

    impl ::std::default::Default for ExecutionLeg {
        fn default() -> Self {
            Self {
                instrument_id: Default::default(),
                leg_id: Default::default(),
                mismarked_quantity: Default::default(),
                price: Default::default(),
                quantity: Default::default(),
                time: Default::default(),
            }
        }
    }

    impl ExecutionLeg {
        pub fn builder() -> builder::ExecutionLeg {
            Default::default()
        }
    }

    ///`FeeLeg`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "feeValues": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/FeeValue"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FeeLeg {
        #[serde(
            rename = "feeValues",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub fee_values: ::std::vec::Vec<FeeValue>,
    }

    impl ::std::default::Default for FeeLeg {
        fn default() -> Self {
            Self {
                fee_values: Default::default(),
            }
        }
    }

    impl FeeLeg {
        pub fn builder() -> builder::FeeLeg {
            Default::default()
        }
    }

    ///`FeeType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "COMMISSION",
    ///    "SEC_FEE",
    ///    "STR_FEE",
    ///    "R_FEE",
    ///    "CDSC_FEE",
    ///    "OPT_REG_FEE",
    ///    "ADDITIONAL_FEE",
    ///    "MISCELLANEOUS_FEE",
    ///    "FTT",
    ///    "FUTURES_CLEARING_FEE",
    ///    "FUTURES_DESK_OFFICE_FEE",
    ///    "FUTURES_EXCHANGE_FEE",
    ///    "FUTURES_GLOBEX_FEE",
    ///    "FUTURES_NFA_FEE",
    ///    "FUTURES_PIT_BROKERAGE_FEE",
    ///    "FUTURES_TRANSACTION_FEE",
    ///    "LOW_PROCEEDS_COMMISSION",
    ///    "BASE_CHARGE",
    ///    "GENERAL_CHARGE",
    ///    "GST_FEE",
    ///    "TAF_FEE",
    ///    "INDEX_OPTION_FEE",
    ///    "TEFRA_TAX",
    ///    "STATE_TAX",
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
    pub enum FeeType {
        #[serde(rename = "COMMISSION")]
        Commission,
        #[serde(rename = "SEC_FEE")]
        SecFee,
        #[serde(rename = "STR_FEE")]
        StrFee,
        #[serde(rename = "R_FEE")]
        RFee,
        #[serde(rename = "CDSC_FEE")]
        CdscFee,
        #[serde(rename = "OPT_REG_FEE")]
        OptRegFee,
        #[serde(rename = "ADDITIONAL_FEE")]
        AdditionalFee,
        #[serde(rename = "MISCELLANEOUS_FEE")]
        MiscellaneousFee,
        #[serde(rename = "FTT")]
        Ftt,
        #[serde(rename = "FUTURES_CLEARING_FEE")]
        FuturesClearingFee,
        #[serde(rename = "FUTURES_DESK_OFFICE_FEE")]
        FuturesDeskOfficeFee,
        #[serde(rename = "FUTURES_EXCHANGE_FEE")]
        FuturesExchangeFee,
        #[serde(rename = "FUTURES_GLOBEX_FEE")]
        FuturesGlobexFee,
        #[serde(rename = "FUTURES_NFA_FEE")]
        FuturesNfaFee,
        #[serde(rename = "FUTURES_PIT_BROKERAGE_FEE")]
        FuturesPitBrokerageFee,
        #[serde(rename = "FUTURES_TRANSACTION_FEE")]
        FuturesTransactionFee,
        #[serde(rename = "LOW_PROCEEDS_COMMISSION")]
        LowProceedsCommission,
        #[serde(rename = "BASE_CHARGE")]
        BaseCharge,
        #[serde(rename = "GENERAL_CHARGE")]
        GeneralCharge,
        #[serde(rename = "GST_FEE")]
        GstFee,
        #[serde(rename = "TAF_FEE")]
        TafFee,
        #[serde(rename = "INDEX_OPTION_FEE")]
        IndexOptionFee,
        #[serde(rename = "TEFRA_TAX")]
        TefraTax,
        #[serde(rename = "STATE_TAX")]
        StateTax,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for FeeType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Commission => f.write_str("COMMISSION"),
                Self::SecFee => f.write_str("SEC_FEE"),
                Self::StrFee => f.write_str("STR_FEE"),
                Self::RFee => f.write_str("R_FEE"),
                Self::CdscFee => f.write_str("CDSC_FEE"),
                Self::OptRegFee => f.write_str("OPT_REG_FEE"),
                Self::AdditionalFee => f.write_str("ADDITIONAL_FEE"),
                Self::MiscellaneousFee => f.write_str("MISCELLANEOUS_FEE"),
                Self::Ftt => f.write_str("FTT"),
                Self::FuturesClearingFee => f.write_str("FUTURES_CLEARING_FEE"),
                Self::FuturesDeskOfficeFee => f.write_str("FUTURES_DESK_OFFICE_FEE"),
                Self::FuturesExchangeFee => f.write_str("FUTURES_EXCHANGE_FEE"),
                Self::FuturesGlobexFee => f.write_str("FUTURES_GLOBEX_FEE"),
                Self::FuturesNfaFee => f.write_str("FUTURES_NFA_FEE"),
                Self::FuturesPitBrokerageFee => f.write_str("FUTURES_PIT_BROKERAGE_FEE"),
                Self::FuturesTransactionFee => f.write_str("FUTURES_TRANSACTION_FEE"),
                Self::LowProceedsCommission => f.write_str("LOW_PROCEEDS_COMMISSION"),
                Self::BaseCharge => f.write_str("BASE_CHARGE"),
                Self::GeneralCharge => f.write_str("GENERAL_CHARGE"),
                Self::GstFee => f.write_str("GST_FEE"),
                Self::TafFee => f.write_str("TAF_FEE"),
                Self::IndexOptionFee => f.write_str("INDEX_OPTION_FEE"),
                Self::TefraTax => f.write_str("TEFRA_TAX"),
                Self::StateTax => f.write_str("STATE_TAX"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for FeeType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "COMMISSION" => Ok(Self::Commission),
                "SEC_FEE" => Ok(Self::SecFee),
                "STR_FEE" => Ok(Self::StrFee),
                "R_FEE" => Ok(Self::RFee),
                "CDSC_FEE" => Ok(Self::CdscFee),
                "OPT_REG_FEE" => Ok(Self::OptRegFee),
                "ADDITIONAL_FEE" => Ok(Self::AdditionalFee),
                "MISCELLANEOUS_FEE" => Ok(Self::MiscellaneousFee),
                "FTT" => Ok(Self::Ftt),
                "FUTURES_CLEARING_FEE" => Ok(Self::FuturesClearingFee),
                "FUTURES_DESK_OFFICE_FEE" => Ok(Self::FuturesDeskOfficeFee),
                "FUTURES_EXCHANGE_FEE" => Ok(Self::FuturesExchangeFee),
                "FUTURES_GLOBEX_FEE" => Ok(Self::FuturesGlobexFee),
                "FUTURES_NFA_FEE" => Ok(Self::FuturesNfaFee),
                "FUTURES_PIT_BROKERAGE_FEE" => Ok(Self::FuturesPitBrokerageFee),
                "FUTURES_TRANSACTION_FEE" => Ok(Self::FuturesTransactionFee),
                "LOW_PROCEEDS_COMMISSION" => Ok(Self::LowProceedsCommission),
                "BASE_CHARGE" => Ok(Self::BaseCharge),
                "GENERAL_CHARGE" => Ok(Self::GeneralCharge),
                "GST_FEE" => Ok(Self::GstFee),
                "TAF_FEE" => Ok(Self::TafFee),
                "INDEX_OPTION_FEE" => Ok(Self::IndexOptionFee),
                "TEFRA_TAX" => Ok(Self::TefraTax),
                "STATE_TAX" => Ok(Self::StateTax),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for FeeType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for FeeType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for FeeType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`FeeValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "type": {
    ///      "$ref": "#/components/schemas/FeeType"
    ///    },
    ///    "value": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct FeeValue {
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<FeeType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub value: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for FeeValue {
        fn default() -> Self {
            Self {
                type_: Default::default(),
                value: Default::default(),
            }
        }
    }

    impl FeeValue {
        pub fn builder() -> builder::FeeValue {
            Default::default()
        }
    }

    ///`Fees`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "feeLegs": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/FeeLeg"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Fees {
        #[serde(
            rename = "feeLegs",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub fee_legs: ::std::vec::Vec<FeeLeg>,
    }

    impl ::std::default::Default for Fees {
        fn default() -> Self {
            Self {
                fee_legs: Default::default(),
            }
        }
    }

    impl Fees {
        pub fn builder() -> builder::Fees {
            Default::default()
        }
    }

    ///`Forex`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/TransactionBaseInstrument"
    ///    }
    ///  ],
    ///  "properties": {
    ///    "baseCurrency": {
    ///      "$ref": "#/components/schemas/Currency"
    ///    },
    ///    "counterCurrency": {
    ///      "$ref": "#/components/schemas/Currency"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "STANDARD",
    ///        "NBBO",
    ///        "UNKNOWN"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Forex {
        #[serde(rename = "assetType")]
        pub asset_type: ForexAssetType,
        #[serde(
            rename = "baseCurrency",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub base_currency: ::std::option::Option<Currency>,
        #[serde(
            rename = "counterCurrency",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub counter_currency: ::std::option::Option<Currency>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "instrumentId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub instrument_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::serde_json::Value::is_null")]
        pub name: ::serde_json::Value,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<ForexType>,
    }

    impl Forex {
        pub fn builder() -> builder::Forex {
            Default::default()
        }
    }

    ///`ForexAssetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EQUITY",
    ///    "OPTION",
    ///    "INDEX",
    ///    "MUTUAL_FUND",
    ///    "CASH_EQUIVALENT",
    ///    "FIXED_INCOME",
    ///    "CURRENCY",
    ///    "COLLECTIVE_INVESTMENT"
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
    pub enum ForexAssetType {
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "CASH_EQUIVALENT")]
        CashEquivalent,
        #[serde(rename = "FIXED_INCOME")]
        FixedIncome,
        #[serde(rename = "CURRENCY")]
        Currency,
        #[serde(rename = "COLLECTIVE_INVESTMENT")]
        CollectiveInvestment,
    }

    impl ::std::fmt::Display for ForexAssetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Equity => f.write_str("EQUITY"),
                Self::Option => f.write_str("OPTION"),
                Self::Index => f.write_str("INDEX"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::CashEquivalent => f.write_str("CASH_EQUIVALENT"),
                Self::FixedIncome => f.write_str("FIXED_INCOME"),
                Self::Currency => f.write_str("CURRENCY"),
                Self::CollectiveInvestment => f.write_str("COLLECTIVE_INVESTMENT"),
            }
        }
    }

    impl ::std::str::FromStr for ForexAssetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EQUITY" => Ok(Self::Equity),
                "OPTION" => Ok(Self::Option),
                "INDEX" => Ok(Self::Index),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "CASH_EQUIVALENT" => Ok(Self::CashEquivalent),
                "FIXED_INCOME" => Ok(Self::FixedIncome),
                "CURRENCY" => Ok(Self::Currency),
                "COLLECTIVE_INVESTMENT" => Ok(Self::CollectiveInvestment),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ForexAssetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ForexAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ForexAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`ForexType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "STANDARD",
    ///    "NBBO",
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
    pub enum ForexType {
        #[serde(rename = "STANDARD")]
        Standard,
        #[serde(rename = "NBBO")]
        Nbbo,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for ForexType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Standard => f.write_str("STANDARD"),
                Self::Nbbo => f.write_str("NBBO"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for ForexType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "STANDARD" => Ok(Self::Standard),
                "NBBO" => Ok(Self::Nbbo),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ForexType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ForexType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ForexType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`Future`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "properties": {
    ///    "activeContract": {
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "expirationDate": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "firstNoticeDate": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "lastTradingDate": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "multiplier": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "STANDARD",
    ///        "UNKNOWN"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Future {
        #[serde(rename = "activeContract", default)]
        pub active_contract: bool,
        #[serde(
            rename = "expirationDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expiration_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "firstNoticeDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub first_notice_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "lastTradingDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_trading_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub multiplier: ::std::option::Option<f64>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<FutureType>,
    }

    impl ::std::default::Default for Future {
        fn default() -> Self {
            Self {
                active_contract: Default::default(),
                expiration_date: Default::default(),
                first_notice_date: Default::default(),
                last_trading_date: Default::default(),
                multiplier: Default::default(),
                type_: Default::default(),
            }
        }
    }

    impl Future {
        pub fn builder() -> builder::Future {
            Default::default()
        }
    }

    ///`FutureType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "STANDARD",
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
    pub enum FutureType {
        #[serde(rename = "STANDARD")]
        Standard,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for FutureType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Standard => f.write_str("STANDARD"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for FutureType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "STANDARD" => Ok(Self::Standard),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for FutureType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for FutureType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for FutureType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`Index`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "properties": {
    ///    "activeContract": {
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "BROAD_BASED",
    ///        "NARROW_BASED",
    ///        "UNKNOWN"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Index {
        #[serde(rename = "activeContract", default)]
        pub active_contract: bool,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<IndexType>,
    }

    impl ::std::default::Default for Index {
        fn default() -> Self {
            Self {
                active_contract: Default::default(),
                type_: Default::default(),
            }
        }
    }

    impl Index {
        pub fn builder() -> builder::Index {
            Default::default()
        }
    }

    ///`IndexType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "BROAD_BASED",
    ///    "NARROW_BASED",
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
    pub enum IndexType {
        #[serde(rename = "BROAD_BASED")]
        BroadBased,
        #[serde(rename = "NARROW_BASED")]
        NarrowBased,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for IndexType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::BroadBased => f.write_str("BROAD_BASED"),
                Self::NarrowBased => f.write_str("NARROW_BASED"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for IndexType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "BROAD_BASED" => Ok(Self::BroadBased),
                "NARROW_BASED" => Ok(Self::NarrowBased),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for IndexType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for IndexType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for IndexType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`Instruction`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "BUY",
    ///    "SELL",
    ///    "BUY_TO_COVER",
    ///    "SELL_SHORT",
    ///    "BUY_TO_OPEN",
    ///    "BUY_TO_CLOSE",
    ///    "SELL_TO_OPEN",
    ///    "SELL_TO_CLOSE",
    ///    "EXCHANGE",
    ///    "SELL_SHORT_EXEMPT"
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
    pub enum Instruction {
        #[serde(rename = "BUY")]
        Buy,
        #[serde(rename = "SELL")]
        Sell,
        #[serde(rename = "BUY_TO_COVER")]
        BuyToCover,
        #[serde(rename = "SELL_SHORT")]
        SellShort,
        #[serde(rename = "BUY_TO_OPEN")]
        BuyToOpen,
        #[serde(rename = "BUY_TO_CLOSE")]
        BuyToClose,
        #[serde(rename = "SELL_TO_OPEN")]
        SellToOpen,
        #[serde(rename = "SELL_TO_CLOSE")]
        SellToClose,
        #[serde(rename = "EXCHANGE")]
        Exchange,
        #[serde(rename = "SELL_SHORT_EXEMPT")]
        SellShortExempt,
    }

    impl ::std::fmt::Display for Instruction {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Buy => f.write_str("BUY"),
                Self::Sell => f.write_str("SELL"),
                Self::BuyToCover => f.write_str("BUY_TO_COVER"),
                Self::SellShort => f.write_str("SELL_SHORT"),
                Self::BuyToOpen => f.write_str("BUY_TO_OPEN"),
                Self::BuyToClose => f.write_str("BUY_TO_CLOSE"),
                Self::SellToOpen => f.write_str("SELL_TO_OPEN"),
                Self::SellToClose => f.write_str("SELL_TO_CLOSE"),
                Self::Exchange => f.write_str("EXCHANGE"),
                Self::SellShortExempt => f.write_str("SELL_SHORT_EXEMPT"),
            }
        }
    }

    impl ::std::str::FromStr for Instruction {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "BUY" => Ok(Self::Buy),
                "SELL" => Ok(Self::Sell),
                "BUY_TO_COVER" => Ok(Self::BuyToCover),
                "SELL_SHORT" => Ok(Self::SellShort),
                "BUY_TO_OPEN" => Ok(Self::BuyToOpen),
                "BUY_TO_CLOSE" => Ok(Self::BuyToClose),
                "SELL_TO_OPEN" => Ok(Self::SellToOpen),
                "SELL_TO_CLOSE" => Ok(Self::SellToClose),
                "EXCHANGE" => Ok(Self::Exchange),
                "SELL_SHORT_EXEMPT" => Ok(Self::SellShortExempt),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for Instruction {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for Instruction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for Instruction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`MarginAccount`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/SecuritiesAccountBase"
    ///    }
    ///  ],
    ///  "properties": {
    ///    "currentBalances": {
    ///      "$ref": "#/components/schemas/MarginBalance"
    ///    },
    ///    "initialBalances": {
    ///      "$ref": "#/components/schemas/MarginInitialBalance"
    ///    },
    ///    "projectedBalances": {
    ///      "$ref": "#/components/schemas/MarginBalance"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MarginAccount {
        #[serde(
            rename = "accountNumber",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub account_number: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "currentBalances",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub current_balances: ::std::option::Option<MarginBalance>,
        #[serde(
            rename = "initialBalances",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub initial_balances: ::std::option::Option<MarginInitialBalance>,
        #[serde(rename = "isClosingOnlyRestricted", default)]
        pub is_closing_only_restricted: bool,
        #[serde(rename = "isDayTrader", default)]
        pub is_day_trader: bool,
        #[serde(rename = "pfcbFlag", default)]
        pub pfcb_flag: bool,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub positions: ::std::vec::Vec<Position>,
        #[serde(
            rename = "projectedBalances",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub projected_balances: ::std::option::Option<MarginBalance>,
        #[serde(
            rename = "roundTrips",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub round_trips: ::std::option::Option<i32>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<MarginAccountType>,
    }

    impl ::std::default::Default for MarginAccount {
        fn default() -> Self {
            Self {
                account_number: Default::default(),
                current_balances: Default::default(),
                initial_balances: Default::default(),
                is_closing_only_restricted: Default::default(),
                is_day_trader: Default::default(),
                pfcb_flag: Default::default(),
                positions: Default::default(),
                projected_balances: Default::default(),
                round_trips: Default::default(),
                type_: Default::default(),
            }
        }
    }

    impl MarginAccount {
        pub fn builder() -> builder::MarginAccount {
            Default::default()
        }
    }

    ///`MarginAccountType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "CASH",
    ///    "MARGIN"
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
    pub enum MarginAccountType {
        #[serde(rename = "CASH")]
        Cash,
        #[serde(rename = "MARGIN")]
        Margin,
    }

    impl ::std::fmt::Display for MarginAccountType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Cash => f.write_str("CASH"),
                Self::Margin => f.write_str("MARGIN"),
            }
        }
    }

    impl ::std::str::FromStr for MarginAccountType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "CASH" => Ok(Self::Cash),
                "MARGIN" => Ok(Self::Margin),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for MarginAccountType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for MarginAccountType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MarginAccountType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`MarginBalance`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "availableFunds": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "availableFundsNonMarginableTrade": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "buyingPower": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "buyingPowerNonMarginableTrade": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "dayTradingBuyingPower": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "dayTradingBuyingPowerCall": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "equity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "equityPercentage": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "isInCall": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "longMarginValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "maintenanceCall": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "maintenanceRequirement": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "marginBalance": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "optionBuyingPower": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "regTCall": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "shortBalance": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "shortMarginValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "sma": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "stockBuyingPower": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MarginBalance {
        #[serde(
            rename = "availableFunds",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub available_funds: ::std::option::Option<f64>,
        #[serde(
            rename = "availableFundsNonMarginableTrade",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub available_funds_non_marginable_trade: ::std::option::Option<f64>,
        #[serde(
            rename = "buyingPower",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub buying_power: ::std::option::Option<f64>,
        #[serde(
            rename = "buyingPowerNonMarginableTrade",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub buying_power_non_marginable_trade: ::std::option::Option<f64>,
        #[serde(
            rename = "cashBalance",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cash_balance: ::std::option::Option<f64>,
        #[serde(
            rename = "dayTradingBuyingPower",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub day_trading_buying_power: ::std::option::Option<f64>,
        #[serde(
            rename = "dayTradingBuyingPowerCall",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub day_trading_buying_power_call: ::std::option::Option<f64>,
        #[serde(
            rename = "liquidationValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub liquidation_value: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub equity: ::std::option::Option<f64>,
        #[serde(
            rename = "equityPercentage",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub equity_percentage: ::std::option::Option<f64>,
        #[serde(
            rename = "isInCall",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_in_call: ::std::option::Option<bool>,
        #[serde(
            rename = "longMarginValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub long_margin_value: ::std::option::Option<f64>,
        #[serde(
            rename = "maintenanceCall",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub maintenance_call: ::std::option::Option<f64>,
        #[serde(
            rename = "maintenanceRequirement",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub maintenance_requirement: ::std::option::Option<f64>,
        #[serde(
            rename = "marginBalance",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub margin_balance: ::std::option::Option<f64>,
        #[serde(
            rename = "optionBuyingPower",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub option_buying_power: ::std::option::Option<f64>,
        #[serde(
            rename = "regTCall",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub reg_t_call: ::std::option::Option<f64>,
        #[serde(
            rename = "shortBalance",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short_balance: ::std::option::Option<f64>,
        #[serde(
            rename = "shortMarginValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short_margin_value: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sma: ::std::option::Option<f64>,
        #[serde(
            rename = "stockBuyingPower",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub stock_buying_power: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for MarginBalance {
        fn default() -> Self {
            Self {
                available_funds: Default::default(),
                available_funds_non_marginable_trade: Default::default(),
                buying_power: Default::default(),
                buying_power_non_marginable_trade: Default::default(),
                cash_balance: Default::default(),
                day_trading_buying_power: Default::default(),
                day_trading_buying_power_call: Default::default(),
                liquidation_value: Default::default(),
                equity: Default::default(),
                equity_percentage: Default::default(),
                is_in_call: Default::default(),
                long_margin_value: Default::default(),
                maintenance_call: Default::default(),
                maintenance_requirement: Default::default(),
                margin_balance: Default::default(),
                option_buying_power: Default::default(),
                reg_t_call: Default::default(),
                short_balance: Default::default(),
                short_margin_value: Default::default(),
                sma: Default::default(),
                stock_buying_power: Default::default(),
            }
        }
    }

    impl MarginBalance {
        pub fn builder() -> builder::MarginBalance {
            Default::default()
        }
    }

    ///`MarginInitialBalance`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "accountValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "accruedInterest": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "availableFundsNonMarginableTrade": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "bondValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "buyingPower": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "cashAvailableForTrading": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "cashBalance": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "cashReceipts": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "dayTradingBuyingPower": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "dayTradingBuyingPowerCall": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "dayTradingEquityCall": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "equity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "equityPercentage": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "isInCall": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "liquidationValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "longMarginValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "longOptionMarketValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "longStockValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "maintenanceCall": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "maintenanceRequirement": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "margin": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "marginBalance": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "marginEquity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "moneyMarketFund": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "mutualFundValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "pendingDeposits": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "regTCall": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "shortBalance": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "shortMarginValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "shortOptionMarketValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "shortStockValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "totalCash": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "unsettledCash": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct MarginInitialBalance {
        #[serde(
            rename = "accountValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub account_value: ::std::option::Option<f64>,
        #[serde(
            rename = "accruedInterest",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub accrued_interest: ::std::option::Option<f64>,
        #[serde(
            rename = "availableFundsNonMarginableTrade",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub available_funds_non_marginable_trade: ::std::option::Option<f64>,
        #[serde(
            rename = "bondValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bond_value: ::std::option::Option<f64>,
        #[serde(
            rename = "buyingPower",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub buying_power: ::std::option::Option<f64>,
        #[serde(
            rename = "cashAvailableForTrading",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cash_available_for_trading: ::std::option::Option<f64>,
        #[serde(
            rename = "cashBalance",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cash_balance: ::std::option::Option<f64>,
        #[serde(
            rename = "cashReceipts",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cash_receipts: ::std::option::Option<f64>,
        #[serde(
            rename = "dayTradingBuyingPower",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub day_trading_buying_power: ::std::option::Option<f64>,
        #[serde(
            rename = "dayTradingBuyingPowerCall",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub day_trading_buying_power_call: ::std::option::Option<f64>,
        #[serde(
            rename = "dayTradingEquityCall",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub day_trading_equity_call: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub equity: ::std::option::Option<f64>,
        #[serde(
            rename = "equityPercentage",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub equity_percentage: ::std::option::Option<f64>,
        #[serde(
            rename = "isInCall",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub is_in_call: ::std::option::Option<bool>,
        #[serde(
            rename = "liquidationValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub liquidation_value: ::std::option::Option<f64>,
        #[serde(
            rename = "longMarginValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub long_margin_value: ::std::option::Option<f64>,
        #[serde(
            rename = "longOptionMarketValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub long_option_market_value: ::std::option::Option<f64>,
        #[serde(
            rename = "longStockValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub long_stock_value: ::std::option::Option<f64>,
        #[serde(
            rename = "maintenanceCall",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub maintenance_call: ::std::option::Option<f64>,
        #[serde(
            rename = "maintenanceRequirement",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub maintenance_requirement: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub margin: ::std::option::Option<f64>,
        #[serde(
            rename = "marginBalance",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub margin_balance: ::std::option::Option<f64>,
        #[serde(
            rename = "marginEquity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub margin_equity: ::std::option::Option<f64>,
        #[serde(
            rename = "moneyMarketFund",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub money_market_fund: ::std::option::Option<f64>,
        #[serde(
            rename = "mutualFundValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mutual_fund_value: ::std::option::Option<f64>,
        #[serde(
            rename = "pendingDeposits",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub pending_deposits: ::std::option::Option<f64>,
        #[serde(
            rename = "regTCall",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub reg_t_call: ::std::option::Option<f64>,
        #[serde(
            rename = "shortBalance",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short_balance: ::std::option::Option<f64>,
        #[serde(
            rename = "shortMarginValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short_margin_value: ::std::option::Option<f64>,
        #[serde(
            rename = "shortOptionMarketValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short_option_market_value: ::std::option::Option<f64>,
        #[serde(
            rename = "shortStockValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short_stock_value: ::std::option::Option<f64>,
        #[serde(
            rename = "totalCash",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub total_cash: ::std::option::Option<f64>,
        #[serde(
            rename = "unsettledCash",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub unsettled_cash: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for MarginInitialBalance {
        fn default() -> Self {
            Self {
                account_value: Default::default(),
                accrued_interest: Default::default(),
                available_funds_non_marginable_trade: Default::default(),
                bond_value: Default::default(),
                buying_power: Default::default(),
                cash_available_for_trading: Default::default(),
                cash_balance: Default::default(),
                cash_receipts: Default::default(),
                day_trading_buying_power: Default::default(),
                day_trading_buying_power_call: Default::default(),
                day_trading_equity_call: Default::default(),
                equity: Default::default(),
                equity_percentage: Default::default(),
                is_in_call: Default::default(),
                liquidation_value: Default::default(),
                long_margin_value: Default::default(),
                long_option_market_value: Default::default(),
                long_stock_value: Default::default(),
                maintenance_call: Default::default(),
                maintenance_requirement: Default::default(),
                margin: Default::default(),
                margin_balance: Default::default(),
                margin_equity: Default::default(),
                money_market_fund: Default::default(),
                mutual_fund_value: Default::default(),
                pending_deposits: Default::default(),
                reg_t_call: Default::default(),
                short_balance: Default::default(),
                short_margin_value: Default::default(),
                short_option_market_value: Default::default(),
                short_stock_value: Default::default(),
                total_cash: Default::default(),
                unsettled_cash: Default::default(),
            }
        }
    }

    impl MarginInitialBalance {
        pub fn builder() -> builder::MarginInitialBalance {
            Default::default()
        }
    }

    ///`Offer`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "level2Permissions": {
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "mktDataPermission": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Offer {
        #[serde(rename = "level2Permissions", default)]
        pub level2_permissions: bool,
        #[serde(
            rename = "mktDataPermission",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mkt_data_permission: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for Offer {
        fn default() -> Self {
            Self {
                level2_permissions: Default::default(),
                mkt_data_permission: Default::default(),
            }
        }
    }

    impl Offer {
        pub fn builder() -> builder::Offer {
            Default::default()
        }
    }

    ///`Order`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "accountNumber": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "activationPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "cancelTime": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "cancelable": {
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "childOrderStrategies": {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    "closeTime": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "complexOrderStrategyType": {
    ///      "$ref": "#/components/schemas/complexOrderStrategyType"
    ///    },
    ///    "destinationLinkName": {
    ///      "type": "string"
    ///    },
    ///    "duration": {
    ///      "$ref": "#/components/schemas/duration"
    ///    },
    ///    "editable": {
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "enteredTime": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "filledQuantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "orderActivityCollection": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OrderActivity"
    ///      }
    ///    },
    ///    "orderId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "orderLegCollection": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OrderLegCollection"
    ///      }
    ///    },
    ///    "orderStrategyType": {
    ///      "$ref": "#/components/schemas/orderStrategyType"
    ///    },
    ///    "orderType": {
    ///      "$ref": "#/components/schemas/orderType"
    ///    },
    ///    "price": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "priceLinkBasis": {
    ///      "$ref": "#/components/schemas/priceLinkBasis"
    ///    },
    ///    "priceLinkType": {
    ///      "$ref": "#/components/schemas/priceLinkType"
    ///    },
    ///    "quantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "releaseTime": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "remainingQuantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "replacingOrderCollection": {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    "requestedDestination": {
    ///      "$ref": "#/components/schemas/requestedDestination"
    ///    },
    ///    "session": {
    ///      "$ref": "#/components/schemas/session"
    ///    },
    ///    "specialInstruction": {
    ///      "$ref": "#/components/schemas/specialInstruction"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/status"
    ///    },
    ///    "statusDescription": {
    ///      "type": "string"
    ///    },
    ///    "stopPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "stopPriceLinkBasis": {
    ///      "$ref": "#/components/schemas/stopPriceLinkBasis"
    ///    },
    ///    "stopPriceLinkType": {
    ///      "$ref": "#/components/schemas/stopPriceLinkType"
    ///    },
    ///    "stopPriceOffset": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "stopType": {
    ///      "$ref": "#/components/schemas/stopType"
    ///    },
    ///    "tag": {
    ///      "type": "string"
    ///    },
    ///    "taxLotMethod": {
    ///      "$ref": "#/components/schemas/taxLotMethod"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Order {
        #[serde(
            rename = "accountNumber",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub account_number: ::std::option::Option<i64>,
        #[serde(
            rename = "activationPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub activation_price: ::std::option::Option<f64>,
        #[serde(
            rename = "cancelTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cancel_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default)]
        pub cancelable: bool,
        #[serde(
            rename = "childOrderStrategies",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub child_order_strategies: ::std::vec::Vec<Order>,
        #[serde(
            rename = "closeTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub close_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "complexOrderStrategyType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub complex_order_strategy_type: ::std::option::Option<ComplexOrderStrategyType>,
        #[serde(
            rename = "destinationLinkName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub destination_link_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub duration: ::std::option::Option<Duration>,
        #[serde(default)]
        pub editable: bool,
        #[serde(
            rename = "enteredTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub entered_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "filledQuantity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub filled_quantity: ::std::option::Option<f64>,
        #[serde(
            rename = "orderActivityCollection",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub order_activity_collection: ::std::vec::Vec<OrderActivity>,
        #[serde(
            rename = "orderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_id: ::std::option::Option<i64>,
        #[serde(
            rename = "orderLegCollection",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub order_leg_collection: ::std::vec::Vec<OrderLegCollection>,
        #[serde(
            rename = "orderStrategyType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_strategy_type: ::std::option::Option<OrderStrategyType>,
        #[serde(
            rename = "orderType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_type: ::std::option::Option<OrderType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price: ::std::option::Option<f64>,
        #[serde(
            rename = "priceLinkBasis",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub price_link_basis: ::std::option::Option<PriceLinkBasis>,
        #[serde(
            rename = "priceLinkType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub price_link_type: ::std::option::Option<PriceLinkType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quantity: ::std::option::Option<f64>,
        #[serde(
            rename = "releaseTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub release_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "remainingQuantity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub remaining_quantity: ::std::option::Option<f64>,
        #[serde(
            rename = "replacingOrderCollection",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub replacing_order_collection: ::std::vec::Vec<Order>,
        #[serde(
            rename = "requestedDestination",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub requested_destination: ::std::option::Option<RequestedDestination>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub session: ::std::option::Option<Session>,
        #[serde(
            rename = "specialInstruction",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub special_instruction: ::std::option::Option<SpecialInstruction>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<Status>,
        #[serde(
            rename = "statusDescription",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub status_description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "stopPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub stop_price: ::std::option::Option<f64>,
        #[serde(
            rename = "stopPriceLinkBasis",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub stop_price_link_basis: ::std::option::Option<StopPriceLinkBasis>,
        #[serde(
            rename = "stopPriceLinkType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub stop_price_link_type: ::std::option::Option<StopPriceLinkType>,
        #[serde(
            rename = "stopPriceOffset",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub stop_price_offset: ::std::option::Option<f64>,
        #[serde(
            rename = "stopType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub stop_type: ::std::option::Option<StopType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub tag: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "taxLotMethod",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tax_lot_method: ::std::option::Option<TaxLotMethod>,
    }

    impl ::std::default::Default for Order {
        fn default() -> Self {
            Self {
                account_number: Default::default(),
                activation_price: Default::default(),
                cancel_time: Default::default(),
                cancelable: Default::default(),
                child_order_strategies: Default::default(),
                close_time: Default::default(),
                complex_order_strategy_type: Default::default(),
                destination_link_name: Default::default(),
                duration: Default::default(),
                editable: Default::default(),
                entered_time: Default::default(),
                filled_quantity: Default::default(),
                order_activity_collection: Default::default(),
                order_id: Default::default(),
                order_leg_collection: Default::default(),
                order_strategy_type: Default::default(),
                order_type: Default::default(),
                price: Default::default(),
                price_link_basis: Default::default(),
                price_link_type: Default::default(),
                quantity: Default::default(),
                release_time: Default::default(),
                remaining_quantity: Default::default(),
                replacing_order_collection: Default::default(),
                requested_destination: Default::default(),
                session: Default::default(),
                special_instruction: Default::default(),
                status: Default::default(),
                status_description: Default::default(),
                stop_price: Default::default(),
                stop_price_link_basis: Default::default(),
                stop_price_link_type: Default::default(),
                stop_price_offset: Default::default(),
                stop_type: Default::default(),
                tag: Default::default(),
                tax_lot_method: Default::default(),
            }
        }
    }

    impl Order {
        pub fn builder() -> builder::Order {
            Default::default()
        }
    }

    ///`OrderActivity`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "activityType": {
    ///      "type": "string",
    ///      "enum": [
    ///        "EXECUTION",
    ///        "ORDER_ACTION"
    ///      ]
    ///    },
    ///    "executionLegs": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ExecutionLeg"
    ///      }
    ///    },
    ///    "executionType": {
    ///      "type": "string",
    ///      "enum": [
    ///        "FILL"
    ///      ]
    ///    },
    ///    "orderRemainingQuantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "quantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrderActivity {
        #[serde(
            rename = "activityType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub activity_type: ::std::option::Option<OrderActivityActivityType>,
        #[serde(
            rename = "executionLegs",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub execution_legs: ::std::vec::Vec<ExecutionLeg>,
        #[serde(
            rename = "executionType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub execution_type: ::std::option::Option<OrderActivityExecutionType>,
        #[serde(
            rename = "orderRemainingQuantity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_remaining_quantity: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quantity: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for OrderActivity {
        fn default() -> Self {
            Self {
                activity_type: Default::default(),
                execution_legs: Default::default(),
                execution_type: Default::default(),
                order_remaining_quantity: Default::default(),
                quantity: Default::default(),
            }
        }
    }

    impl OrderActivity {
        pub fn builder() -> builder::OrderActivity {
            Default::default()
        }
    }

    ///`OrderActivityActivityType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EXECUTION",
    ///    "ORDER_ACTION"
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
    pub enum OrderActivityActivityType {
        #[serde(rename = "EXECUTION")]
        Execution,
        #[serde(rename = "ORDER_ACTION")]
        OrderAction,
    }

    impl ::std::fmt::Display for OrderActivityActivityType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Execution => f.write_str("EXECUTION"),
                Self::OrderAction => f.write_str("ORDER_ACTION"),
            }
        }
    }

    impl ::std::str::FromStr for OrderActivityActivityType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EXECUTION" => Ok(Self::Execution),
                "ORDER_ACTION" => Ok(Self::OrderAction),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for OrderActivityActivityType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for OrderActivityActivityType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for OrderActivityActivityType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`OrderActivityExecutionType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "FILL"
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
    pub enum OrderActivityExecutionType {
        #[serde(rename = "FILL")]
        Fill,
        /// Returned by the API for canceled/replaced order activity (not in published spec).
        #[serde(rename = "CANCELED")]
        Canceled,
        #[serde(rename = "EXPIRED")]
        Expired,
        #[serde(rename = "REPLACED")]
        Replaced,
    }

    impl ::std::fmt::Display for OrderActivityExecutionType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Fill => f.write_str("FILL"),
                Self::Canceled => f.write_str("CANCELED"),
                Self::Expired => f.write_str("EXPIRED"),
                Self::Replaced => f.write_str("REPLACED"),
            }
        }
    }

    impl ::std::str::FromStr for OrderActivityExecutionType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "FILL" => Ok(Self::Fill),
                "CANCELED" => Ok(Self::Canceled),
                "EXPIRED" => Ok(Self::Expired),
                "REPLACED" => Ok(Self::Replaced),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for OrderActivityExecutionType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for OrderActivityExecutionType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for OrderActivityExecutionType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`OrderBalance`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "orderValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "projectedAvailableFund": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "projectedBuyingPower": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "projectedCommission": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrderBalance {
        #[serde(
            rename = "orderValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_value: ::std::option::Option<f64>,
        #[serde(
            rename = "projectedAvailableFund",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub projected_available_fund: ::std::option::Option<f64>,
        #[serde(
            rename = "projectedBuyingPower",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub projected_buying_power: ::std::option::Option<f64>,
        #[serde(
            rename = "projectedCommission",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub projected_commission: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for OrderBalance {
        fn default() -> Self {
            Self {
                order_value: Default::default(),
                projected_available_fund: Default::default(),
                projected_buying_power: Default::default(),
                projected_commission: Default::default(),
            }
        }
    }

    impl OrderBalance {
        pub fn builder() -> builder::OrderBalance {
            Default::default()
        }
    }

    ///`OrderLeg`
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
    ///    "assetType": {
    ///      "$ref": "#/components/schemas/assetType"
    ///    },
    ///    "bidPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "finalSymbol": {
    ///      "type": "string"
    ///    },
    ///    "instruction": {
    ///      "$ref": "#/components/schemas/instruction"
    ///    },
    ///    "lastPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "legId": {
    ///      "type": "number",
    ///      "format": "long"
    ///    },
    ///    "markPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "projectedCommission": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "quantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrderLeg {
        #[serde(
            rename = "askPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub ask_price: ::std::option::Option<f64>,
        #[serde(
            rename = "assetType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub asset_type: ::std::option::Option<AssetType>,
        #[serde(
            rename = "bidPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub bid_price: ::std::option::Option<f64>,
        #[serde(
            rename = "finalSymbol",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub final_symbol: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub instruction: ::std::option::Option<Instruction>,
        #[serde(
            rename = "lastPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_price: ::std::option::Option<f64>,
        #[serde(
            rename = "legId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub leg_id: ::std::option::Option<f64>,
        #[serde(
            rename = "markPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub mark_price: ::std::option::Option<f64>,
        #[serde(
            rename = "projectedCommission",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub projected_commission: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quantity: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for OrderLeg {
        fn default() -> Self {
            Self {
                ask_price: Default::default(),
                asset_type: Default::default(),
                bid_price: Default::default(),
                final_symbol: Default::default(),
                instruction: Default::default(),
                last_price: Default::default(),
                leg_id: Default::default(),
                mark_price: Default::default(),
                projected_commission: Default::default(),
                quantity: Default::default(),
            }
        }
    }

    impl OrderLeg {
        pub fn builder() -> builder::OrderLeg {
            Default::default()
        }
    }

    ///`OrderLegCollection`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "divCapGains": {
    ///      "type": "string",
    ///      "enum": [
    ///        "REINVEST",
    ///        "PAYOUT"
    ///      ]
    ///    },
    ///    "instruction": {
    ///      "$ref": "#/components/schemas/instruction"
    ///    },
    ///    "instrument": {
    ///      "$ref": "#/components/schemas/AccountsInstrument"
    ///    },
    ///    "legId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "orderLegType": {
    ///      "type": "string",
    ///      "enum": [
    ///        "EQUITY",
    ///        "OPTION",
    ///        "INDEX",
    ///        "MUTUAL_FUND",
    ///        "CASH_EQUIVALENT",
    ///        "FIXED_INCOME",
    ///        "CURRENCY",
    ///        "COLLECTIVE_INVESTMENT"
    ///      ]
    ///    },
    ///    "positionEffect": {
    ///      "type": "string",
    ///      "enum": [
    ///        "OPENING",
    ///        "CLOSING",
    ///        "AUTOMATIC"
    ///      ]
    ///    },
    ///    "quantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "quantityType": {
    ///      "type": "string",
    ///      "enum": [
    ///        "ALL_SHARES",
    ///        "DOLLARS",
    ///        "SHARES"
    ///      ]
    ///    },
    ///    "toSymbol": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrderLegCollection {
        #[serde(
            rename = "divCapGains",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub div_cap_gains: ::std::option::Option<OrderLegCollectionDivCapGains>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub instruction: ::std::option::Option<Instruction>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub instrument: ::std::option::Option<AccountsInstrument>,
        #[serde(
            rename = "legId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub leg_id: ::std::option::Option<i64>,
        #[serde(
            rename = "orderLegType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_leg_type: ::std::option::Option<OrderLegCollectionOrderLegType>,
        #[serde(
            rename = "positionEffect",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub position_effect: ::std::option::Option<OrderLegCollectionPositionEffect>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quantity: ::std::option::Option<f64>,
        #[serde(
            rename = "quantityType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub quantity_type: ::std::option::Option<OrderLegCollectionQuantityType>,
        #[serde(
            rename = "toSymbol",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub to_symbol: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for OrderLegCollection {
        fn default() -> Self {
            Self {
                div_cap_gains: Default::default(),
                instruction: Default::default(),
                instrument: Default::default(),
                leg_id: Default::default(),
                order_leg_type: Default::default(),
                position_effect: Default::default(),
                quantity: Default::default(),
                quantity_type: Default::default(),
                to_symbol: Default::default(),
            }
        }
    }

    impl OrderLegCollection {
        pub fn builder() -> builder::OrderLegCollection {
            Default::default()
        }
    }

    ///`OrderLegCollectionDivCapGains`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "REINVEST",
    ///    "PAYOUT"
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
    pub enum OrderLegCollectionDivCapGains {
        #[serde(rename = "REINVEST")]
        Reinvest,
        #[serde(rename = "PAYOUT")]
        Payout,
    }

    impl ::std::fmt::Display for OrderLegCollectionDivCapGains {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Reinvest => f.write_str("REINVEST"),
                Self::Payout => f.write_str("PAYOUT"),
            }
        }
    }

    impl ::std::str::FromStr for OrderLegCollectionDivCapGains {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "REINVEST" => Ok(Self::Reinvest),
                "PAYOUT" => Ok(Self::Payout),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for OrderLegCollectionDivCapGains {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for OrderLegCollectionDivCapGains {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for OrderLegCollectionDivCapGains {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`OrderLegCollectionOrderLegType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EQUITY",
    ///    "OPTION",
    ///    "INDEX",
    ///    "MUTUAL_FUND",
    ///    "CASH_EQUIVALENT",
    ///    "FIXED_INCOME",
    ///    "CURRENCY",
    ///    "COLLECTIVE_INVESTMENT"
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
    pub enum OrderLegCollectionOrderLegType {
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "CASH_EQUIVALENT")]
        CashEquivalent,
        #[serde(rename = "FIXED_INCOME")]
        FixedIncome,
        #[serde(rename = "CURRENCY")]
        Currency,
        #[serde(rename = "COLLECTIVE_INVESTMENT")]
        CollectiveInvestment,
    }

    impl ::std::fmt::Display for OrderLegCollectionOrderLegType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Equity => f.write_str("EQUITY"),
                Self::Option => f.write_str("OPTION"),
                Self::Index => f.write_str("INDEX"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::CashEquivalent => f.write_str("CASH_EQUIVALENT"),
                Self::FixedIncome => f.write_str("FIXED_INCOME"),
                Self::Currency => f.write_str("CURRENCY"),
                Self::CollectiveInvestment => f.write_str("COLLECTIVE_INVESTMENT"),
            }
        }
    }

    impl ::std::str::FromStr for OrderLegCollectionOrderLegType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EQUITY" => Ok(Self::Equity),
                "OPTION" => Ok(Self::Option),
                "INDEX" => Ok(Self::Index),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "CASH_EQUIVALENT" => Ok(Self::CashEquivalent),
                "FIXED_INCOME" => Ok(Self::FixedIncome),
                "CURRENCY" => Ok(Self::Currency),
                "COLLECTIVE_INVESTMENT" => Ok(Self::CollectiveInvestment),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for OrderLegCollectionOrderLegType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for OrderLegCollectionOrderLegType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for OrderLegCollectionOrderLegType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`OrderLegCollectionPositionEffect`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "OPENING",
    ///    "CLOSING",
    ///    "AUTOMATIC"
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
    pub enum OrderLegCollectionPositionEffect {
        #[serde(rename = "OPENING")]
        Opening,
        #[serde(rename = "CLOSING")]
        Closing,
        #[serde(rename = "AUTOMATIC")]
        Automatic,
    }

    impl ::std::fmt::Display for OrderLegCollectionPositionEffect {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Opening => f.write_str("OPENING"),
                Self::Closing => f.write_str("CLOSING"),
                Self::Automatic => f.write_str("AUTOMATIC"),
            }
        }
    }

    impl ::std::str::FromStr for OrderLegCollectionPositionEffect {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "OPENING" => Ok(Self::Opening),
                "CLOSING" => Ok(Self::Closing),
                "AUTOMATIC" => Ok(Self::Automatic),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for OrderLegCollectionPositionEffect {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for OrderLegCollectionPositionEffect {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for OrderLegCollectionPositionEffect {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`OrderLegCollectionQuantityType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ALL_SHARES",
    ///    "DOLLARS",
    ///    "SHARES"
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
    pub enum OrderLegCollectionQuantityType {
        #[serde(rename = "ALL_SHARES")]
        AllShares,
        #[serde(rename = "DOLLARS")]
        Dollars,
        #[serde(rename = "SHARES")]
        Shares,
    }

    impl ::std::fmt::Display for OrderLegCollectionQuantityType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AllShares => f.write_str("ALL_SHARES"),
                Self::Dollars => f.write_str("DOLLARS"),
                Self::Shares => f.write_str("SHARES"),
            }
        }
    }

    impl ::std::str::FromStr for OrderLegCollectionQuantityType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ALL_SHARES" => Ok(Self::AllShares),
                "DOLLARS" => Ok(Self::Dollars),
                "SHARES" => Ok(Self::Shares),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for OrderLegCollectionQuantityType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for OrderLegCollectionQuantityType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for OrderLegCollectionQuantityType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`OrderRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "accountNumber": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "activationPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "cancelTime": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "cancelable": {
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "childOrderStrategies": {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    "closeTime": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "complexOrderStrategyType": {
    ///      "$ref": "#/components/schemas/complexOrderStrategyType"
    ///    },
    ///    "destinationLinkName": {
    ///      "type": "string"
    ///    },
    ///    "duration": {
    ///      "$ref": "#/components/schemas/duration"
    ///    },
    ///    "editable": {
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "enteredTime": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "filledQuantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "orderActivityCollection": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OrderActivity"
    ///      }
    ///    },
    ///    "orderId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "orderLegCollection": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OrderLegCollection"
    ///      }
    ///    },
    ///    "orderStrategyType": {
    ///      "$ref": "#/components/schemas/orderStrategyType"
    ///    },
    ///    "orderType": {
    ///      "$ref": "#/components/schemas/orderTypeRequest"
    ///    },
    ///    "price": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "priceLinkBasis": {
    ///      "$ref": "#/components/schemas/priceLinkBasis"
    ///    },
    ///    "priceLinkType": {
    ///      "$ref": "#/components/schemas/priceLinkType"
    ///    },
    ///    "quantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "releaseTime": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "remainingQuantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "replacingOrderCollection": {
    ///      "type": "array",
    ///      "items": {}
    ///    },
    ///    "session": {
    ///      "$ref": "#/components/schemas/session"
    ///    },
    ///    "specialInstruction": {
    ///      "$ref": "#/components/schemas/specialInstruction"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/status"
    ///    },
    ///    "statusDescription": {
    ///      "type": "string"
    ///    },
    ///    "stopPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "stopPriceLinkBasis": {
    ///      "$ref": "#/components/schemas/stopPriceLinkBasis"
    ///    },
    ///    "stopPriceLinkType": {
    ///      "$ref": "#/components/schemas/stopPriceLinkType"
    ///    },
    ///    "stopPriceOffset": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "stopType": {
    ///      "$ref": "#/components/schemas/stopType"
    ///    },
    ///    "taxLotMethod": {
    ///      "$ref": "#/components/schemas/taxLotMethod"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrderRequest {
        #[serde(
            rename = "accountNumber",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub account_number: ::std::option::Option<i64>,
        #[serde(
            rename = "activationPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub activation_price: ::std::option::Option<f64>,
        #[serde(
            rename = "cancelTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cancel_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default)]
        pub cancelable: bool,
        #[serde(
            rename = "childOrderStrategies",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub child_order_strategies: ::std::vec::Vec<OrderRequest>,
        #[serde(
            rename = "closeTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub close_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "complexOrderStrategyType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub complex_order_strategy_type: ::std::option::Option<ComplexOrderStrategyType>,
        #[serde(
            rename = "destinationLinkName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub destination_link_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub duration: ::std::option::Option<Duration>,
        #[serde(default)]
        pub editable: bool,
        #[serde(
            rename = "enteredTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub entered_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "filledQuantity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub filled_quantity: ::std::option::Option<f64>,
        #[serde(
            rename = "orderActivityCollection",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub order_activity_collection: ::std::vec::Vec<OrderActivity>,
        #[serde(
            rename = "orderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_id: ::std::option::Option<i64>,
        #[serde(
            rename = "orderLegCollection",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub order_leg_collection: ::std::vec::Vec<OrderLegCollection>,
        #[serde(
            rename = "orderStrategyType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_strategy_type: ::std::option::Option<OrderStrategyType>,
        #[serde(
            rename = "orderType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_type: ::std::option::Option<OrderTypeRequest>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price: ::std::option::Option<f64>,
        #[serde(
            rename = "priceLinkBasis",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub price_link_basis: ::std::option::Option<PriceLinkBasis>,
        #[serde(
            rename = "priceLinkType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub price_link_type: ::std::option::Option<PriceLinkType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quantity: ::std::option::Option<f64>,
        #[serde(
            rename = "releaseTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub release_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "remainingQuantity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub remaining_quantity: ::std::option::Option<f64>,
        #[serde(
            rename = "replacingOrderCollection",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub replacing_order_collection: ::std::vec::Vec<OrderRequest>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub session: ::std::option::Option<Session>,
        #[serde(
            rename = "specialInstruction",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub special_instruction: ::std::option::Option<SpecialInstruction>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<Status>,
        #[serde(
            rename = "statusDescription",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub status_description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "stopPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub stop_price: ::std::option::Option<f64>,
        #[serde(
            rename = "stopPriceLinkBasis",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub stop_price_link_basis: ::std::option::Option<StopPriceLinkBasis>,
        #[serde(
            rename = "stopPriceLinkType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub stop_price_link_type: ::std::option::Option<StopPriceLinkType>,
        #[serde(
            rename = "stopPriceOffset",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub stop_price_offset: ::std::option::Option<f64>,
        #[serde(
            rename = "stopType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub stop_type: ::std::option::Option<StopType>,
        #[serde(
            rename = "taxLotMethod",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tax_lot_method: ::std::option::Option<TaxLotMethod>,
    }

    impl ::std::default::Default for OrderRequest {
        fn default() -> Self {
            Self {
                account_number: Default::default(),
                activation_price: Default::default(),
                cancel_time: Default::default(),
                cancelable: Default::default(),
                child_order_strategies: Default::default(),
                close_time: Default::default(),
                complex_order_strategy_type: Default::default(),
                destination_link_name: Default::default(),
                duration: Default::default(),
                editable: Default::default(),
                entered_time: Default::default(),
                filled_quantity: Default::default(),
                order_activity_collection: Default::default(),
                order_id: Default::default(),
                order_leg_collection: Default::default(),
                order_strategy_type: Default::default(),
                order_type: Default::default(),
                price: Default::default(),
                price_link_basis: Default::default(),
                price_link_type: Default::default(),
                quantity: Default::default(),
                release_time: Default::default(),
                remaining_quantity: Default::default(),
                replacing_order_collection: Default::default(),
                session: Default::default(),
                special_instruction: Default::default(),
                status: Default::default(),
                status_description: Default::default(),
                stop_price: Default::default(),
                stop_price_link_basis: Default::default(),
                stop_price_link_type: Default::default(),
                stop_price_offset: Default::default(),
                stop_type: Default::default(),
                tax_lot_method: Default::default(),
            }
        }
    }

    impl OrderRequest {
        pub fn builder() -> builder::OrderRequest {
            Default::default()
        }
    }

    ///`OrderStrategy`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "accountNumber": {
    ///      "type": "string"
    ///    },
    ///    "advancedOrderType": {
    ///      "type": "string",
    ///      "enum": [
    ///        "NONE",
    ///        "OTO",
    ///        "OCO",
    ///        "OTOCO",
    ///        "OT2OCO",
    ///        "OT3OCO",
    ///        "BLAST_ALL",
    ///        "OTA",
    ///        "PAIR"
    ///      ]
    ///    },
    ///    "allOrNone": {
    ///      "type": "boolean"
    ///    },
    ///    "amountIndicator": {
    ///      "$ref": "#/components/schemas/amountIndicator"
    ///    },
    ///    "closeTime": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "discretionary": {
    ///      "type": "boolean"
    ///    },
    ///    "duration": {
    ///      "$ref": "#/components/schemas/duration"
    ///    },
    ///    "enteredTime": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "filledQuantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "orderBalance": {
    ///      "$ref": "#/components/schemas/OrderBalance"
    ///    },
    ///    "orderLegs": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OrderLeg"
    ///      }
    ///    },
    ///    "orderStrategyType": {
    ///      "$ref": "#/components/schemas/orderStrategyType"
    ///    },
    ///    "orderType": {
    ///      "$ref": "#/components/schemas/orderType"
    ///    },
    ///    "orderValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "orderVersion": {
    ///      "type": "number"
    ///    },
    ///    "price": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "quantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "remainingQuantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "sellNonMarginableFirst": {
    ///      "type": "boolean"
    ///    },
    ///    "session": {
    ///      "$ref": "#/components/schemas/session"
    ///    },
    ///    "settlementInstruction": {
    ///      "$ref": "#/components/schemas/settlementInstruction"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/apiOrderStatus"
    ///    },
    ///    "strategy": {
    ///      "$ref": "#/components/schemas/complexOrderStrategyType"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrderStrategy {
        #[serde(
            rename = "accountNumber",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub account_number: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "advancedOrderType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub advanced_order_type: ::std::option::Option<OrderStrategyAdvancedOrderType>,
        #[serde(
            rename = "allOrNone",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub all_or_none: ::std::option::Option<bool>,
        #[serde(
            rename = "amountIndicator",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub amount_indicator: ::std::option::Option<AmountIndicator>,
        #[serde(
            rename = "closeTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub close_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub discretionary: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub duration: ::std::option::Option<Duration>,
        #[serde(
            rename = "enteredTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub entered_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "filledQuantity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub filled_quantity: ::std::option::Option<f64>,
        #[serde(
            rename = "orderBalance",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_balance: ::std::option::Option<OrderBalance>,
        #[serde(
            rename = "orderLegs",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub order_legs: ::std::vec::Vec<OrderLeg>,
        #[serde(
            rename = "orderStrategyType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_strategy_type: ::std::option::Option<OrderStrategyType>,
        #[serde(
            rename = "orderType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_type: ::std::option::Option<OrderType>,
        #[serde(
            rename = "orderValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_value: ::std::option::Option<f64>,
        #[serde(
            rename = "orderVersion",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_version: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub quantity: ::std::option::Option<f64>,
        #[serde(
            rename = "remainingQuantity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub remaining_quantity: ::std::option::Option<f64>,
        #[serde(
            rename = "sellNonMarginableFirst",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sell_non_marginable_first: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub session: ::std::option::Option<Session>,
        #[serde(
            rename = "settlementInstruction",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub settlement_instruction: ::std::option::Option<SettlementInstruction>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<ApiOrderStatus>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub strategy: ::std::option::Option<ComplexOrderStrategyType>,
    }

    impl ::std::default::Default for OrderStrategy {
        fn default() -> Self {
            Self {
                account_number: Default::default(),
                advanced_order_type: Default::default(),
                all_or_none: Default::default(),
                amount_indicator: Default::default(),
                close_time: Default::default(),
                discretionary: Default::default(),
                duration: Default::default(),
                entered_time: Default::default(),
                filled_quantity: Default::default(),
                order_balance: Default::default(),
                order_legs: Default::default(),
                order_strategy_type: Default::default(),
                order_type: Default::default(),
                order_value: Default::default(),
                order_version: Default::default(),
                price: Default::default(),
                quantity: Default::default(),
                remaining_quantity: Default::default(),
                sell_non_marginable_first: Default::default(),
                session: Default::default(),
                settlement_instruction: Default::default(),
                status: Default::default(),
                strategy: Default::default(),
            }
        }
    }

    impl OrderStrategy {
        pub fn builder() -> builder::OrderStrategy {
            Default::default()
        }
    }

    ///`OrderStrategyAdvancedOrderType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NONE",
    ///    "OTO",
    ///    "OCO",
    ///    "OTOCO",
    ///    "OT2OCO",
    ///    "OT3OCO",
    ///    "BLAST_ALL",
    ///    "OTA",
    ///    "PAIR"
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
    pub enum OrderStrategyAdvancedOrderType {
        #[serde(rename = "NONE")]
        None,
        #[serde(rename = "OTO")]
        Oto,
        #[serde(rename = "OCO")]
        Oco,
        #[serde(rename = "OTOCO")]
        Otoco,
        #[serde(rename = "OT2OCO")]
        Ot2oco,
        #[serde(rename = "OT3OCO")]
        Ot3oco,
        #[serde(rename = "BLAST_ALL")]
        BlastAll,
        #[serde(rename = "OTA")]
        Ota,
        #[serde(rename = "PAIR")]
        Pair,
    }

    impl ::std::fmt::Display for OrderStrategyAdvancedOrderType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::None => f.write_str("NONE"),
                Self::Oto => f.write_str("OTO"),
                Self::Oco => f.write_str("OCO"),
                Self::Otoco => f.write_str("OTOCO"),
                Self::Ot2oco => f.write_str("OT2OCO"),
                Self::Ot3oco => f.write_str("OT3OCO"),
                Self::BlastAll => f.write_str("BLAST_ALL"),
                Self::Ota => f.write_str("OTA"),
                Self::Pair => f.write_str("PAIR"),
            }
        }
    }

    impl ::std::str::FromStr for OrderStrategyAdvancedOrderType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NONE" => Ok(Self::None),
                "OTO" => Ok(Self::Oto),
                "OCO" => Ok(Self::Oco),
                "OTOCO" => Ok(Self::Otoco),
                "OT2OCO" => Ok(Self::Ot2oco),
                "OT3OCO" => Ok(Self::Ot3oco),
                "BLAST_ALL" => Ok(Self::BlastAll),
                "OTA" => Ok(Self::Ota),
                "PAIR" => Ok(Self::Pair),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for OrderStrategyAdvancedOrderType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for OrderStrategyAdvancedOrderType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for OrderStrategyAdvancedOrderType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`OrderStrategyType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "SINGLE",
    ///    "CANCEL",
    ///    "RECALL",
    ///    "PAIR",
    ///    "FLATTEN",
    ///    "TWO_DAY_SWAP",
    ///    "BLAST_ALL",
    ///    "OCO",
    ///    "TRIGGER"
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
    pub enum OrderStrategyType {
        #[serde(rename = "SINGLE")]
        Single,
        #[serde(rename = "CANCEL")]
        Cancel,
        #[serde(rename = "RECALL")]
        Recall,
        #[serde(rename = "PAIR")]
        Pair,
        #[serde(rename = "FLATTEN")]
        Flatten,
        #[serde(rename = "TWO_DAY_SWAP")]
        TwoDaySwap,
        #[serde(rename = "BLAST_ALL")]
        BlastAll,
        #[serde(rename = "OCO")]
        Oco,
        #[serde(rename = "TRIGGER")]
        Trigger,
    }

    impl ::std::fmt::Display for OrderStrategyType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Single => f.write_str("SINGLE"),
                Self::Cancel => f.write_str("CANCEL"),
                Self::Recall => f.write_str("RECALL"),
                Self::Pair => f.write_str("PAIR"),
                Self::Flatten => f.write_str("FLATTEN"),
                Self::TwoDaySwap => f.write_str("TWO_DAY_SWAP"),
                Self::BlastAll => f.write_str("BLAST_ALL"),
                Self::Oco => f.write_str("OCO"),
                Self::Trigger => f.write_str("TRIGGER"),
            }
        }
    }

    impl ::std::str::FromStr for OrderStrategyType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "SINGLE" => Ok(Self::Single),
                "CANCEL" => Ok(Self::Cancel),
                "RECALL" => Ok(Self::Recall),
                "PAIR" => Ok(Self::Pair),
                "FLATTEN" => Ok(Self::Flatten),
                "TWO_DAY_SWAP" => Ok(Self::TwoDaySwap),
                "BLAST_ALL" => Ok(Self::BlastAll),
                "OCO" => Ok(Self::Oco),
                "TRIGGER" => Ok(Self::Trigger),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for OrderStrategyType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for OrderStrategyType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for OrderStrategyType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`OrderType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "MARKET",
    ///    "LIMIT",
    ///    "STOP",
    ///    "STOP_LIMIT",
    ///    "TRAILING_STOP",
    ///    "CABINET",
    ///    "NON_MARKETABLE",
    ///    "MARKET_ON_CLOSE",
    ///    "EXERCISE",
    ///    "TRAILING_STOP_LIMIT",
    ///    "NET_DEBIT",
    ///    "NET_CREDIT",
    ///    "NET_ZERO",
    ///    "LIMIT_ON_CLOSE",
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
    pub enum OrderType {
        #[serde(rename = "MARKET")]
        Market,
        #[serde(rename = "LIMIT")]
        Limit,
        #[serde(rename = "STOP")]
        Stop,
        #[serde(rename = "STOP_LIMIT")]
        StopLimit,
        #[serde(rename = "TRAILING_STOP")]
        TrailingStop,
        #[serde(rename = "CABINET")]
        Cabinet,
        #[serde(rename = "NON_MARKETABLE")]
        NonMarketable,
        #[serde(rename = "MARKET_ON_CLOSE")]
        MarketOnClose,
        #[serde(rename = "EXERCISE")]
        Exercise,
        #[serde(rename = "TRAILING_STOP_LIMIT")]
        TrailingStopLimit,
        #[serde(rename = "NET_DEBIT")]
        NetDebit,
        #[serde(rename = "NET_CREDIT")]
        NetCredit,
        #[serde(rename = "NET_ZERO")]
        NetZero,
        #[serde(rename = "LIMIT_ON_CLOSE")]
        LimitOnClose,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for OrderType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Market => f.write_str("MARKET"),
                Self::Limit => f.write_str("LIMIT"),
                Self::Stop => f.write_str("STOP"),
                Self::StopLimit => f.write_str("STOP_LIMIT"),
                Self::TrailingStop => f.write_str("TRAILING_STOP"),
                Self::Cabinet => f.write_str("CABINET"),
                Self::NonMarketable => f.write_str("NON_MARKETABLE"),
                Self::MarketOnClose => f.write_str("MARKET_ON_CLOSE"),
                Self::Exercise => f.write_str("EXERCISE"),
                Self::TrailingStopLimit => f.write_str("TRAILING_STOP_LIMIT"),
                Self::NetDebit => f.write_str("NET_DEBIT"),
                Self::NetCredit => f.write_str("NET_CREDIT"),
                Self::NetZero => f.write_str("NET_ZERO"),
                Self::LimitOnClose => f.write_str("LIMIT_ON_CLOSE"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for OrderType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "MARKET" => Ok(Self::Market),
                "LIMIT" => Ok(Self::Limit),
                "STOP" => Ok(Self::Stop),
                "STOP_LIMIT" => Ok(Self::StopLimit),
                "TRAILING_STOP" => Ok(Self::TrailingStop),
                "CABINET" => Ok(Self::Cabinet),
                "NON_MARKETABLE" => Ok(Self::NonMarketable),
                "MARKET_ON_CLOSE" => Ok(Self::MarketOnClose),
                "EXERCISE" => Ok(Self::Exercise),
                "TRAILING_STOP_LIMIT" => Ok(Self::TrailingStopLimit),
                "NET_DEBIT" => Ok(Self::NetDebit),
                "NET_CREDIT" => Ok(Self::NetCredit),
                "NET_ZERO" => Ok(Self::NetZero),
                "LIMIT_ON_CLOSE" => Ok(Self::LimitOnClose),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for OrderType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for OrderType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for OrderType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///Same as orderType, but does not have UNKNOWN since this type is not
    /// allowed as an input
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "description": "Same as orderType, but does not have UNKNOWN since this
    /// type is not allowed as an input",
    ///  "type": "string",
    ///  "enum": [
    ///    "MARKET",
    ///    "LIMIT",
    ///    "STOP",
    ///    "STOP_LIMIT",
    ///    "TRAILING_STOP",
    ///    "CABINET",
    ///    "NON_MARKETABLE",
    ///    "MARKET_ON_CLOSE",
    ///    "EXERCISE",
    ///    "TRAILING_STOP_LIMIT",
    ///    "NET_DEBIT",
    ///    "NET_CREDIT",
    ///    "NET_ZERO",
    ///    "LIMIT_ON_CLOSE"
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
    pub enum OrderTypeRequest {
        #[serde(rename = "MARKET")]
        Market,
        #[serde(rename = "LIMIT")]
        Limit,
        #[serde(rename = "STOP")]
        Stop,
        #[serde(rename = "STOP_LIMIT")]
        StopLimit,
        #[serde(rename = "TRAILING_STOP")]
        TrailingStop,
        #[serde(rename = "CABINET")]
        Cabinet,
        #[serde(rename = "NON_MARKETABLE")]
        NonMarketable,
        #[serde(rename = "MARKET_ON_CLOSE")]
        MarketOnClose,
        #[serde(rename = "EXERCISE")]
        Exercise,
        #[serde(rename = "TRAILING_STOP_LIMIT")]
        TrailingStopLimit,
        #[serde(rename = "NET_DEBIT")]
        NetDebit,
        #[serde(rename = "NET_CREDIT")]
        NetCredit,
        #[serde(rename = "NET_ZERO")]
        NetZero,
        #[serde(rename = "LIMIT_ON_CLOSE")]
        LimitOnClose,
    }

    impl ::std::fmt::Display for OrderTypeRequest {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Market => f.write_str("MARKET"),
                Self::Limit => f.write_str("LIMIT"),
                Self::Stop => f.write_str("STOP"),
                Self::StopLimit => f.write_str("STOP_LIMIT"),
                Self::TrailingStop => f.write_str("TRAILING_STOP"),
                Self::Cabinet => f.write_str("CABINET"),
                Self::NonMarketable => f.write_str("NON_MARKETABLE"),
                Self::MarketOnClose => f.write_str("MARKET_ON_CLOSE"),
                Self::Exercise => f.write_str("EXERCISE"),
                Self::TrailingStopLimit => f.write_str("TRAILING_STOP_LIMIT"),
                Self::NetDebit => f.write_str("NET_DEBIT"),
                Self::NetCredit => f.write_str("NET_CREDIT"),
                Self::NetZero => f.write_str("NET_ZERO"),
                Self::LimitOnClose => f.write_str("LIMIT_ON_CLOSE"),
            }
        }
    }

    impl ::std::str::FromStr for OrderTypeRequest {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "MARKET" => Ok(Self::Market),
                "LIMIT" => Ok(Self::Limit),
                "STOP" => Ok(Self::Stop),
                "STOP_LIMIT" => Ok(Self::StopLimit),
                "TRAILING_STOP" => Ok(Self::TrailingStop),
                "CABINET" => Ok(Self::Cabinet),
                "NON_MARKETABLE" => Ok(Self::NonMarketable),
                "MARKET_ON_CLOSE" => Ok(Self::MarketOnClose),
                "EXERCISE" => Ok(Self::Exercise),
                "TRAILING_STOP_LIMIT" => Ok(Self::TrailingStopLimit),
                "NET_DEBIT" => Ok(Self::NetDebit),
                "NET_CREDIT" => Ok(Self::NetCredit),
                "NET_ZERO" => Ok(Self::NetZero),
                "LIMIT_ON_CLOSE" => Ok(Self::LimitOnClose),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for OrderTypeRequest {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for OrderTypeRequest {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for OrderTypeRequest {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`OrderValidationDetail`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "activityMessage": {
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "originalSeverity": {
    ///      "$ref": "#/components/schemas/APIRuleAction"
    ///    },
    ///    "overrideName": {
    ///      "type": "string"
    ///    },
    ///    "overrideSeverity": {
    ///      "$ref": "#/components/schemas/APIRuleAction"
    ///    },
    ///    "validationRuleName": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrderValidationDetail {
        #[serde(
            rename = "activityMessage",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub activity_message: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "originalSeverity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub original_severity: ::std::option::Option<ApiRuleAction>,
        #[serde(
            rename = "overrideName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub override_name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "overrideSeverity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub override_severity: ::std::option::Option<ApiRuleAction>,
        #[serde(
            rename = "validationRuleName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub validation_rule_name: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for OrderValidationDetail {
        fn default() -> Self {
            Self {
                activity_message: Default::default(),
                message: Default::default(),
                original_severity: Default::default(),
                override_name: Default::default(),
                override_severity: Default::default(),
                validation_rule_name: Default::default(),
            }
        }
    }

    impl OrderValidationDetail {
        pub fn builder() -> builder::OrderValidationDetail {
            Default::default()
        }
    }

    ///`OrderValidationResult`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "accepts": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OrderValidationDetail"
    ///      }
    ///    },
    ///    "alerts": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OrderValidationDetail"
    ///      }
    ///    },
    ///    "rejects": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OrderValidationDetail"
    ///      }
    ///    },
    ///    "reviews": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OrderValidationDetail"
    ///      }
    ///    },
    ///    "warns": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OrderValidationDetail"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct OrderValidationResult {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub accepts: ::std::vec::Vec<OrderValidationDetail>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub alerts: ::std::vec::Vec<OrderValidationDetail>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub rejects: ::std::vec::Vec<OrderValidationDetail>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub reviews: ::std::vec::Vec<OrderValidationDetail>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub warns: ::std::vec::Vec<OrderValidationDetail>,
    }

    impl ::std::default::Default for OrderValidationResult {
        fn default() -> Self {
            Self {
                accepts: Default::default(),
                alerts: Default::default(),
                rejects: Default::default(),
                reviews: Default::default(),
                warns: Default::default(),
            }
        }
    }

    impl OrderValidationResult {
        pub fn builder() -> builder::OrderValidationResult {
            Default::default()
        }
    }

    ///`Position`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "agedQuantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "averageLongPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "averagePrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "averageShortPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "currentDayCost": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "currentDayProfitLoss": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "currentDayProfitLossPercentage": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "instrument": {
    ///      "$ref": "#/components/schemas/AccountsInstrument"
    ///    },
    ///    "longOpenProfitLoss": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "longQuantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "maintenanceRequirement": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "marketValue": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "previousSessionLongQuantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "previousSessionShortQuantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "settledLongQuantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "settledShortQuantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "shortOpenProfitLoss": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "shortQuantity": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "taxLotAverageLongPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "taxLotAverageShortPrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Position {
        #[serde(
            rename = "agedQuantity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub aged_quantity: ::std::option::Option<f64>,
        #[serde(
            rename = "averageLongPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub average_long_price: ::std::option::Option<f64>,
        #[serde(
            rename = "averagePrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub average_price: ::std::option::Option<f64>,
        #[serde(
            rename = "averageShortPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub average_short_price: ::std::option::Option<f64>,
        #[serde(
            rename = "currentDayCost",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub current_day_cost: ::std::option::Option<f64>,
        #[serde(
            rename = "currentDayProfitLoss",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub current_day_profit_loss: ::std::option::Option<f64>,
        #[serde(
            rename = "currentDayProfitLossPercentage",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub current_day_profit_loss_percentage: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub instrument: ::std::option::Option<AccountsInstrument>,
        #[serde(
            rename = "longOpenProfitLoss",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub long_open_profit_loss: ::std::option::Option<f64>,
        #[serde(
            rename = "longQuantity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub long_quantity: ::std::option::Option<f64>,
        #[serde(
            rename = "maintenanceRequirement",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub maintenance_requirement: ::std::option::Option<f64>,
        #[serde(
            rename = "marketValue",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub market_value: ::std::option::Option<f64>,
        #[serde(
            rename = "previousSessionLongQuantity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub previous_session_long_quantity: ::std::option::Option<f64>,
        #[serde(
            rename = "previousSessionShortQuantity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub previous_session_short_quantity: ::std::option::Option<f64>,
        #[serde(
            rename = "settledLongQuantity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub settled_long_quantity: ::std::option::Option<f64>,
        #[serde(
            rename = "settledShortQuantity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub settled_short_quantity: ::std::option::Option<f64>,
        #[serde(
            rename = "shortOpenProfitLoss",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short_open_profit_loss: ::std::option::Option<f64>,
        #[serde(
            rename = "shortQuantity",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub short_quantity: ::std::option::Option<f64>,
        #[serde(
            rename = "taxLotAverageLongPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tax_lot_average_long_price: ::std::option::Option<f64>,
        #[serde(
            rename = "taxLotAverageShortPrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub tax_lot_average_short_price: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for Position {
        fn default() -> Self {
            Self {
                aged_quantity: Default::default(),
                average_long_price: Default::default(),
                average_price: Default::default(),
                average_short_price: Default::default(),
                current_day_cost: Default::default(),
                current_day_profit_loss: Default::default(),
                current_day_profit_loss_percentage: Default::default(),
                instrument: Default::default(),
                long_open_profit_loss: Default::default(),
                long_quantity: Default::default(),
                maintenance_requirement: Default::default(),
                market_value: Default::default(),
                previous_session_long_quantity: Default::default(),
                previous_session_short_quantity: Default::default(),
                settled_long_quantity: Default::default(),
                settled_short_quantity: Default::default(),
                short_open_profit_loss: Default::default(),
                short_quantity: Default::default(),
                tax_lot_average_long_price: Default::default(),
                tax_lot_average_short_price: Default::default(),
            }
        }
    }

    impl Position {
        pub fn builder() -> builder::Position {
            Default::default()
        }
    }

    ///`PreviewOrder`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "commissionAndFee": {
    ///      "$ref": "#/components/schemas/CommissionAndFee"
    ///    },
    ///    "orderId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "orderStrategy": {
    ///      "$ref": "#/components/schemas/OrderStrategy"
    ///    },
    ///    "orderValidationResult": {
    ///      "$ref": "#/components/schemas/OrderValidationResult"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct PreviewOrder {
        #[serde(
            rename = "commissionAndFee",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub commission_and_fee: ::std::option::Option<CommissionAndFee>,
        #[serde(
            rename = "orderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_id: ::std::option::Option<i64>,
        #[serde(
            rename = "orderStrategy",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_strategy: ::std::option::Option<OrderStrategy>,
        #[serde(
            rename = "orderValidationResult",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_validation_result: ::std::option::Option<OrderValidationResult>,
    }

    impl ::std::default::Default for PreviewOrder {
        fn default() -> Self {
            Self {
                commission_and_fee: Default::default(),
                order_id: Default::default(),
                order_strategy: Default::default(),
                order_validation_result: Default::default(),
            }
        }
    }

    impl PreviewOrder {
        pub fn builder() -> builder::PreviewOrder {
            Default::default()
        }
    }

    ///`PriceLinkBasis`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "MANUAL",
    ///    "BASE",
    ///    "TRIGGER",
    ///    "LAST",
    ///    "BID",
    ///    "ASK",
    ///    "ASK_BID",
    ///    "MARK",
    ///    "AVERAGE"
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
    pub enum PriceLinkBasis {
        #[serde(rename = "MANUAL")]
        Manual,
        #[serde(rename = "BASE")]
        Base,
        #[serde(rename = "TRIGGER")]
        Trigger,
        #[serde(rename = "LAST")]
        Last,
        #[serde(rename = "BID")]
        Bid,
        #[serde(rename = "ASK")]
        Ask,
        #[serde(rename = "ASK_BID")]
        AskBid,
        #[serde(rename = "MARK")]
        Mark,
        #[serde(rename = "AVERAGE")]
        Average,
    }

    impl ::std::fmt::Display for PriceLinkBasis {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Manual => f.write_str("MANUAL"),
                Self::Base => f.write_str("BASE"),
                Self::Trigger => f.write_str("TRIGGER"),
                Self::Last => f.write_str("LAST"),
                Self::Bid => f.write_str("BID"),
                Self::Ask => f.write_str("ASK"),
                Self::AskBid => f.write_str("ASK_BID"),
                Self::Mark => f.write_str("MARK"),
                Self::Average => f.write_str("AVERAGE"),
            }
        }
    }

    impl ::std::str::FromStr for PriceLinkBasis {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "MANUAL" => Ok(Self::Manual),
                "BASE" => Ok(Self::Base),
                "TRIGGER" => Ok(Self::Trigger),
                "LAST" => Ok(Self::Last),
                "BID" => Ok(Self::Bid),
                "ASK" => Ok(Self::Ask),
                "ASK_BID" => Ok(Self::AskBid),
                "MARK" => Ok(Self::Mark),
                "AVERAGE" => Ok(Self::Average),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PriceLinkBasis {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PriceLinkBasis {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PriceLinkBasis {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`PriceLinkType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "VALUE",
    ///    "PERCENT",
    ///    "TICK"
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
    pub enum PriceLinkType {
        #[serde(rename = "VALUE")]
        Value,
        #[serde(rename = "PERCENT")]
        Percent,
        #[serde(rename = "TICK")]
        Tick,
    }

    impl ::std::fmt::Display for PriceLinkType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Value => f.write_str("VALUE"),
                Self::Percent => f.write_str("PERCENT"),
                Self::Tick => f.write_str("TICK"),
            }
        }
    }

    impl ::std::str::FromStr for PriceLinkType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "VALUE" => Ok(Self::Value),
                "PERCENT" => Ok(Self::Percent),
                "TICK" => Ok(Self::Tick),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PriceLinkType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PriceLinkType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PriceLinkType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`Product`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/TransactionBaseInstrument"
    ///    }
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "TBD",
    ///        "UNKNOWN"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Product {
        #[serde(rename = "assetType")]
        pub asset_type: ProductAssetType,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "instrumentId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub instrument_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::serde_json::Value::is_null")]
        pub name: ::serde_json::Value,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<ProductType>,
    }

    impl Product {
        pub fn builder() -> builder::Product {
            Default::default()
        }
    }

    ///`ProductAssetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EQUITY",
    ///    "OPTION",
    ///    "INDEX",
    ///    "MUTUAL_FUND",
    ///    "CASH_EQUIVALENT",
    ///    "FIXED_INCOME",
    ///    "CURRENCY",
    ///    "COLLECTIVE_INVESTMENT"
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
    pub enum ProductAssetType {
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "CASH_EQUIVALENT")]
        CashEquivalent,
        #[serde(rename = "FIXED_INCOME")]
        FixedIncome,
        #[serde(rename = "CURRENCY")]
        Currency,
        #[serde(rename = "COLLECTIVE_INVESTMENT")]
        CollectiveInvestment,
    }

    impl ::std::fmt::Display for ProductAssetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Equity => f.write_str("EQUITY"),
                Self::Option => f.write_str("OPTION"),
                Self::Index => f.write_str("INDEX"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::CashEquivalent => f.write_str("CASH_EQUIVALENT"),
                Self::FixedIncome => f.write_str("FIXED_INCOME"),
                Self::Currency => f.write_str("CURRENCY"),
                Self::CollectiveInvestment => f.write_str("COLLECTIVE_INVESTMENT"),
            }
        }
    }

    impl ::std::str::FromStr for ProductAssetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EQUITY" => Ok(Self::Equity),
                "OPTION" => Ok(Self::Option),
                "INDEX" => Ok(Self::Index),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "CASH_EQUIVALENT" => Ok(Self::CashEquivalent),
                "FIXED_INCOME" => Ok(Self::FixedIncome),
                "CURRENCY" => Ok(Self::Currency),
                "COLLECTIVE_INVESTMENT" => Ok(Self::CollectiveInvestment),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ProductAssetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ProductAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ProductAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`ProductType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "TBD",
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
    pub enum ProductType {
        #[serde(rename = "TBD")]
        Tbd,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for ProductType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Tbd => f.write_str("TBD"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for ProductType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "TBD" => Ok(Self::Tbd),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ProductType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ProductType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ProductType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`RequestedDestination`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "INET",
    ///    "ECN_ARCA",
    ///    "CBOE",
    ///    "AMEX",
    ///    "PHLX",
    ///    "ISE",
    ///    "BOX",
    ///    "NYSE",
    ///    "NASDAQ",
    ///    "BATS",
    ///    "C2",
    ///    "AUTO"
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
    pub enum RequestedDestination {
        #[serde(rename = "INET")]
        Inet,
        #[serde(rename = "ECN_ARCA")]
        EcnArca,
        #[serde(rename = "CBOE")]
        Cboe,
        #[serde(rename = "AMEX")]
        Amex,
        #[serde(rename = "PHLX")]
        Phlx,
        #[serde(rename = "ISE")]
        Ise,
        #[serde(rename = "BOX")]
        Box,
        #[serde(rename = "NYSE")]
        Nyse,
        #[serde(rename = "NASDAQ")]
        Nasdaq,
        #[serde(rename = "BATS")]
        Bats,
        C2,
        #[serde(rename = "AUTO")]
        Auto,
    }

    impl ::std::fmt::Display for RequestedDestination {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Inet => f.write_str("INET"),
                Self::EcnArca => f.write_str("ECN_ARCA"),
                Self::Cboe => f.write_str("CBOE"),
                Self::Amex => f.write_str("AMEX"),
                Self::Phlx => f.write_str("PHLX"),
                Self::Ise => f.write_str("ISE"),
                Self::Box => f.write_str("BOX"),
                Self::Nyse => f.write_str("NYSE"),
                Self::Nasdaq => f.write_str("NASDAQ"),
                Self::Bats => f.write_str("BATS"),
                Self::C2 => f.write_str("C2"),
                Self::Auto => f.write_str("AUTO"),
            }
        }
    }

    impl ::std::str::FromStr for RequestedDestination {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "INET" => Ok(Self::Inet),
                "ECN_ARCA" => Ok(Self::EcnArca),
                "CBOE" => Ok(Self::Cboe),
                "AMEX" => Ok(Self::Amex),
                "PHLX" => Ok(Self::Phlx),
                "ISE" => Ok(Self::Ise),
                "BOX" => Ok(Self::Box),
                "NYSE" => Ok(Self::Nyse),
                "NASDAQ" => Ok(Self::Nasdaq),
                "BATS" => Ok(Self::Bats),
                "C2" => Ok(Self::C2),
                "AUTO" => Ok(Self::Auto),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for RequestedDestination {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for RequestedDestination {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RequestedDestination {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`SecuritiesAccount`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/MarginAccount"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/CashAccount"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum SecuritiesAccount {
        MarginAccount(MarginAccount),
        CashAccount(CashAccount),
    }

    impl ::std::convert::From<MarginAccount> for SecuritiesAccount {
        fn from(value: MarginAccount) -> Self {
            Self::MarginAccount(value)
        }
    }

    impl ::std::convert::From<CashAccount> for SecuritiesAccount {
        fn from(value: CashAccount) -> Self {
            Self::CashAccount(value)
        }
    }

    ///`SecuritiesAccountBase`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "accountNumber": {
    ///      "type": "string"
    ///    },
    ///    "isClosingOnlyRestricted": {
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "isDayTrader": {
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "pfcbFlag": {
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "positions": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Position"
    ///      }
    ///    },
    ///    "roundTrips": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "CASH",
    ///        "MARGIN"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct SecuritiesAccountBase {
        #[serde(
            rename = "accountNumber",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub account_number: ::std::option::Option<::std::string::String>,
        #[serde(rename = "isClosingOnlyRestricted", default)]
        pub is_closing_only_restricted: bool,
        #[serde(rename = "isDayTrader", default)]
        pub is_day_trader: bool,
        #[serde(rename = "pfcbFlag", default)]
        pub pfcb_flag: bool,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub positions: ::std::vec::Vec<Position>,
        #[serde(
            rename = "roundTrips",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub round_trips: ::std::option::Option<i32>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<SecuritiesAccountBaseType>,
    }

    impl ::std::default::Default for SecuritiesAccountBase {
        fn default() -> Self {
            Self {
                account_number: Default::default(),
                is_closing_only_restricted: Default::default(),
                is_day_trader: Default::default(),
                pfcb_flag: Default::default(),
                positions: Default::default(),
                round_trips: Default::default(),
                type_: Default::default(),
            }
        }
    }

    impl SecuritiesAccountBase {
        pub fn builder() -> builder::SecuritiesAccountBase {
            Default::default()
        }
    }

    ///`SecuritiesAccountBaseType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "CASH",
    ///    "MARGIN"
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
    pub enum SecuritiesAccountBaseType {
        #[serde(rename = "CASH")]
        Cash,
        #[serde(rename = "MARGIN")]
        Margin,
    }

    impl ::std::fmt::Display for SecuritiesAccountBaseType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Cash => f.write_str("CASH"),
                Self::Margin => f.write_str("MARGIN"),
            }
        }
    }

    impl ::std::str::FromStr for SecuritiesAccountBaseType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "CASH" => Ok(Self::Cash),
                "MARGIN" => Ok(Self::Margin),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for SecuritiesAccountBaseType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for SecuritiesAccountBaseType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SecuritiesAccountBaseType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`ServiceError`
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
    ///        "type": "string"
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct ServiceError {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub errors: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for ServiceError {
        fn default() -> Self {
            Self {
                errors: Default::default(),
                message: Default::default(),
            }
        }
    }

    impl ServiceError {
        pub fn builder() -> builder::ServiceError {
            Default::default()
        }
    }

    ///`Session`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NORMAL",
    ///    "AM",
    ///    "PM",
    ///    "SEAMLESS"
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
    pub enum Session {
        #[serde(rename = "NORMAL")]
        Normal,
        #[serde(rename = "AM")]
        Am,
        #[serde(rename = "PM")]
        Pm,
        #[serde(rename = "SEAMLESS")]
        Seamless,
        /// Extended trading hours (not in the published spec but returned by the API).
        #[serde(rename = "EXTO")]
        Exto,
    }

    impl ::std::fmt::Display for Session {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Normal => f.write_str("NORMAL"),
                Self::Am => f.write_str("AM"),
                Self::Pm => f.write_str("PM"),
                Self::Seamless => f.write_str("SEAMLESS"),
                Self::Exto => f.write_str("EXTO"),
            }
        }
    }

    impl ::std::str::FromStr for Session {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NORMAL" => Ok(Self::Normal),
                "AM" => Ok(Self::Am),
                "PM" => Ok(Self::Pm),
                "SEAMLESS" => Ok(Self::Seamless),
                "EXTO" => Ok(Self::Exto),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for Session {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for Session {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for Session {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`SettlementInstruction`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "REGULAR",
    ///    "CASH",
    ///    "NEXT_DAY",
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
    pub enum SettlementInstruction {
        #[serde(rename = "REGULAR")]
        Regular,
        #[serde(rename = "CASH")]
        Cash,
        #[serde(rename = "NEXT_DAY")]
        NextDay,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for SettlementInstruction {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Regular => f.write_str("REGULAR"),
                Self::Cash => f.write_str("CASH"),
                Self::NextDay => f.write_str("NEXT_DAY"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for SettlementInstruction {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "REGULAR" => Ok(Self::Regular),
                "CASH" => Ok(Self::Cash),
                "NEXT_DAY" => Ok(Self::NextDay),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for SettlementInstruction {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for SettlementInstruction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SettlementInstruction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`SpecialInstruction`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ALL_OR_NONE",
    ///    "DO_NOT_REDUCE",
    ///    "ALL_OR_NONE_DO_NOT_REDUCE"
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
    pub enum SpecialInstruction {
        #[serde(rename = "ALL_OR_NONE")]
        AllOrNone,
        #[serde(rename = "DO_NOT_REDUCE")]
        DoNotReduce,
        #[serde(rename = "ALL_OR_NONE_DO_NOT_REDUCE")]
        AllOrNoneDoNotReduce,
    }

    impl ::std::fmt::Display for SpecialInstruction {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AllOrNone => f.write_str("ALL_OR_NONE"),
                Self::DoNotReduce => f.write_str("DO_NOT_REDUCE"),
                Self::AllOrNoneDoNotReduce => f.write_str("ALL_OR_NONE_DO_NOT_REDUCE"),
            }
        }
    }

    impl ::std::str::FromStr for SpecialInstruction {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ALL_OR_NONE" => Ok(Self::AllOrNone),
                "DO_NOT_REDUCE" => Ok(Self::DoNotReduce),
                "ALL_OR_NONE_DO_NOT_REDUCE" => Ok(Self::AllOrNoneDoNotReduce),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for SpecialInstruction {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for SpecialInstruction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for SpecialInstruction {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`Status`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "AWAITING_PARENT_ORDER",
    ///    "AWAITING_CONDITION",
    ///    "AWAITING_STOP_CONDITION",
    ///    "AWAITING_MANUAL_REVIEW",
    ///    "ACCEPTED",
    ///    "AWAITING_UR_OUT",
    ///    "PENDING_ACTIVATION",
    ///    "QUEUED",
    ///    "WORKING",
    ///    "REJECTED",
    ///    "PENDING_CANCEL",
    ///    "CANCELED",
    ///    "PENDING_REPLACE",
    ///    "REPLACED",
    ///    "FILLED",
    ///    "EXPIRED",
    ///    "NEW",
    ///    "AWAITING_RELEASE_TIME",
    ///    "PENDING_ACKNOWLEDGEMENT",
    ///    "PENDING_RECALL",
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
    pub enum Status {
        #[serde(rename = "AWAITING_PARENT_ORDER")]
        AwaitingParentOrder,
        #[serde(rename = "AWAITING_CONDITION")]
        AwaitingCondition,
        #[serde(rename = "AWAITING_STOP_CONDITION")]
        AwaitingStopCondition,
        #[serde(rename = "AWAITING_MANUAL_REVIEW")]
        AwaitingManualReview,
        #[serde(rename = "ACCEPTED")]
        Accepted,
        #[serde(rename = "AWAITING_UR_OUT")]
        AwaitingUrOut,
        #[serde(rename = "PENDING_ACTIVATION")]
        PendingActivation,
        #[serde(rename = "QUEUED")]
        Queued,
        #[serde(rename = "WORKING")]
        Working,
        #[serde(rename = "REJECTED")]
        Rejected,
        #[serde(rename = "PENDING_CANCEL")]
        PendingCancel,
        #[serde(rename = "CANCELED")]
        Canceled,
        #[serde(rename = "PENDING_REPLACE")]
        PendingReplace,
        #[serde(rename = "REPLACED")]
        Replaced,
        #[serde(rename = "FILLED")]
        Filled,
        #[serde(rename = "EXPIRED")]
        Expired,
        #[serde(rename = "NEW")]
        New,
        #[serde(rename = "AWAITING_RELEASE_TIME")]
        AwaitingReleaseTime,
        #[serde(rename = "PENDING_ACKNOWLEDGEMENT")]
        PendingAcknowledgement,
        #[serde(rename = "PENDING_RECALL")]
        PendingRecall,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for Status {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AwaitingParentOrder => f.write_str("AWAITING_PARENT_ORDER"),
                Self::AwaitingCondition => f.write_str("AWAITING_CONDITION"),
                Self::AwaitingStopCondition => f.write_str("AWAITING_STOP_CONDITION"),
                Self::AwaitingManualReview => f.write_str("AWAITING_MANUAL_REVIEW"),
                Self::Accepted => f.write_str("ACCEPTED"),
                Self::AwaitingUrOut => f.write_str("AWAITING_UR_OUT"),
                Self::PendingActivation => f.write_str("PENDING_ACTIVATION"),
                Self::Queued => f.write_str("QUEUED"),
                Self::Working => f.write_str("WORKING"),
                Self::Rejected => f.write_str("REJECTED"),
                Self::PendingCancel => f.write_str("PENDING_CANCEL"),
                Self::Canceled => f.write_str("CANCELED"),
                Self::PendingReplace => f.write_str("PENDING_REPLACE"),
                Self::Replaced => f.write_str("REPLACED"),
                Self::Filled => f.write_str("FILLED"),
                Self::Expired => f.write_str("EXPIRED"),
                Self::New => f.write_str("NEW"),
                Self::AwaitingReleaseTime => f.write_str("AWAITING_RELEASE_TIME"),
                Self::PendingAcknowledgement => f.write_str("PENDING_ACKNOWLEDGEMENT"),
                Self::PendingRecall => f.write_str("PENDING_RECALL"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for Status {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "AWAITING_PARENT_ORDER" => Ok(Self::AwaitingParentOrder),
                "AWAITING_CONDITION" => Ok(Self::AwaitingCondition),
                "AWAITING_STOP_CONDITION" => Ok(Self::AwaitingStopCondition),
                "AWAITING_MANUAL_REVIEW" => Ok(Self::AwaitingManualReview),
                "ACCEPTED" => Ok(Self::Accepted),
                "AWAITING_UR_OUT" => Ok(Self::AwaitingUrOut),
                "PENDING_ACTIVATION" => Ok(Self::PendingActivation),
                "QUEUED" => Ok(Self::Queued),
                "WORKING" => Ok(Self::Working),
                "REJECTED" => Ok(Self::Rejected),
                "PENDING_CANCEL" => Ok(Self::PendingCancel),
                "CANCELED" => Ok(Self::Canceled),
                "PENDING_REPLACE" => Ok(Self::PendingReplace),
                "REPLACED" => Ok(Self::Replaced),
                "FILLED" => Ok(Self::Filled),
                "EXPIRED" => Ok(Self::Expired),
                "NEW" => Ok(Self::New),
                "AWAITING_RELEASE_TIME" => Ok(Self::AwaitingReleaseTime),
                "PENDING_ACKNOWLEDGEMENT" => Ok(Self::PendingAcknowledgement),
                "PENDING_RECALL" => Ok(Self::PendingRecall),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for Status {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for Status {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for Status {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`StopPriceLinkBasis`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "MANUAL",
    ///    "BASE",
    ///    "TRIGGER",
    ///    "LAST",
    ///    "BID",
    ///    "ASK",
    ///    "ASK_BID",
    ///    "MARK",
    ///    "AVERAGE"
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
    pub enum StopPriceLinkBasis {
        #[serde(rename = "MANUAL")]
        Manual,
        #[serde(rename = "BASE")]
        Base,
        #[serde(rename = "TRIGGER")]
        Trigger,
        #[serde(rename = "LAST")]
        Last,
        #[serde(rename = "BID")]
        Bid,
        #[serde(rename = "ASK")]
        Ask,
        #[serde(rename = "ASK_BID")]
        AskBid,
        #[serde(rename = "MARK")]
        Mark,
        #[serde(rename = "AVERAGE")]
        Average,
    }

    impl ::std::fmt::Display for StopPriceLinkBasis {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Manual => f.write_str("MANUAL"),
                Self::Base => f.write_str("BASE"),
                Self::Trigger => f.write_str("TRIGGER"),
                Self::Last => f.write_str("LAST"),
                Self::Bid => f.write_str("BID"),
                Self::Ask => f.write_str("ASK"),
                Self::AskBid => f.write_str("ASK_BID"),
                Self::Mark => f.write_str("MARK"),
                Self::Average => f.write_str("AVERAGE"),
            }
        }
    }

    impl ::std::str::FromStr for StopPriceLinkBasis {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "MANUAL" => Ok(Self::Manual),
                "BASE" => Ok(Self::Base),
                "TRIGGER" => Ok(Self::Trigger),
                "LAST" => Ok(Self::Last),
                "BID" => Ok(Self::Bid),
                "ASK" => Ok(Self::Ask),
                "ASK_BID" => Ok(Self::AskBid),
                "MARK" => Ok(Self::Mark),
                "AVERAGE" => Ok(Self::Average),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for StopPriceLinkBasis {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for StopPriceLinkBasis {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for StopPriceLinkBasis {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`StopPriceLinkType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "VALUE",
    ///    "PERCENT",
    ///    "TICK"
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
    pub enum StopPriceLinkType {
        #[serde(rename = "VALUE")]
        Value,
        #[serde(rename = "PERCENT")]
        Percent,
        #[serde(rename = "TICK")]
        Tick,
    }

    impl ::std::fmt::Display for StopPriceLinkType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Value => f.write_str("VALUE"),
                Self::Percent => f.write_str("PERCENT"),
                Self::Tick => f.write_str("TICK"),
            }
        }
    }

    impl ::std::str::FromStr for StopPriceLinkType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "VALUE" => Ok(Self::Value),
                "PERCENT" => Ok(Self::Percent),
                "TICK" => Ok(Self::Tick),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for StopPriceLinkType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for StopPriceLinkType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for StopPriceLinkType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`StopPriceOffset`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "number",
    ///  "format": "double"
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(transparent)]
    pub struct StopPriceOffset(pub f64);
    impl ::std::ops::Deref for StopPriceOffset {
        type Target = f64;
        fn deref(&self) -> &f64 {
            &self.0
        }
    }

    impl ::std::convert::From<StopPriceOffset> for f64 {
        fn from(value: StopPriceOffset) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<f64> for StopPriceOffset {
        fn from(value: f64) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for StopPriceOffset {
        type Err = <f64 as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for StopPriceOffset {
        type Error = <f64 as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for StopPriceOffset {
        type Error = <f64 as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for StopPriceOffset {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    ///`StopType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "STANDARD",
    ///    "BID",
    ///    "ASK",
    ///    "LAST",
    ///    "MARK"
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
    pub enum StopType {
        #[serde(rename = "STANDARD")]
        Standard,
        #[serde(rename = "BID")]
        Bid,
        #[serde(rename = "ASK")]
        Ask,
        #[serde(rename = "LAST")]
        Last,
        #[serde(rename = "MARK")]
        Mark,
    }

    impl ::std::fmt::Display for StopType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Standard => f.write_str("STANDARD"),
                Self::Bid => f.write_str("BID"),
                Self::Ask => f.write_str("ASK"),
                Self::Last => f.write_str("LAST"),
                Self::Mark => f.write_str("MARK"),
            }
        }
    }

    impl ::std::str::FromStr for StopType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "STANDARD" => Ok(Self::Standard),
                "BID" => Ok(Self::Bid),
                "ASK" => Ok(Self::Ask),
                "LAST" => Ok(Self::Last),
                "MARK" => Ok(Self::Mark),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for StopType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for StopType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for StopType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`StreamerInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "schwabClientChannel": {
    ///      "type": "string"
    ///    },
    ///    "schwabClientCorrelId": {
    ///      "type": "string"
    ///    },
    ///    "schwabClientCustomerId": {
    ///      "type": "string"
    ///    },
    ///    "schwabClientFunctionId": {
    ///      "type": "string"
    ///    },
    ///    "streamerSocketUrl": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct StreamerInfo {
        #[serde(
            rename = "schwabClientChannel",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub schwab_client_channel: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "schwabClientCorrelId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub schwab_client_correl_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "schwabClientCustomerId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub schwab_client_customer_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "schwabClientFunctionId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub schwab_client_function_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "streamerSocketUrl",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub streamer_socket_url: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for StreamerInfo {
        fn default() -> Self {
            Self {
                schwab_client_channel: Default::default(),
                schwab_client_correl_id: Default::default(),
                schwab_client_customer_id: Default::default(),
                schwab_client_function_id: Default::default(),
                streamer_socket_url: Default::default(),
            }
        }
    }

    impl StreamerInfo {
        pub fn builder() -> builder::StreamerInfo {
            Default::default()
        }
    }

    ///`TaxLotMethod`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "FIFO",
    ///    "LIFO",
    ///    "HIGH_COST",
    ///    "LOW_COST",
    ///    "AVERAGE_COST",
    ///    "SPECIFIC_LOT",
    ///    "LOSS_HARVESTER"
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
    pub enum TaxLotMethod {
        #[serde(rename = "FIFO")]
        Fifo,
        #[serde(rename = "LIFO")]
        Lifo,
        #[serde(rename = "HIGH_COST")]
        HighCost,
        #[serde(rename = "LOW_COST")]
        LowCost,
        #[serde(rename = "AVERAGE_COST")]
        AverageCost,
        #[serde(rename = "SPECIFIC_LOT")]
        SpecificLot,
        #[serde(rename = "LOSS_HARVESTER")]
        LossHarvester,
    }

    impl ::std::fmt::Display for TaxLotMethod {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Fifo => f.write_str("FIFO"),
                Self::Lifo => f.write_str("LIFO"),
                Self::HighCost => f.write_str("HIGH_COST"),
                Self::LowCost => f.write_str("LOW_COST"),
                Self::AverageCost => f.write_str("AVERAGE_COST"),
                Self::SpecificLot => f.write_str("SPECIFIC_LOT"),
                Self::LossHarvester => f.write_str("LOSS_HARVESTER"),
            }
        }
    }

    impl ::std::str::FromStr for TaxLotMethod {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "FIFO" => Ok(Self::Fifo),
                "LIFO" => Ok(Self::Lifo),
                "HIGH_COST" => Ok(Self::HighCost),
                "LOW_COST" => Ok(Self::LowCost),
                "AVERAGE_COST" => Ok(Self::AverageCost),
                "SPECIFIC_LOT" => Ok(Self::SpecificLot),
                "LOSS_HARVESTER" => Ok(Self::LossHarvester),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TaxLotMethod {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TaxLotMethod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TaxLotMethod {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`Transaction`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "accountNumber": {
    ///      "type": "string"
    ///    },
    ///    "activityId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "activityType": {
    ///      "type": "string",
    ///      "enum": [
    ///        "ACTIVITY_CORRECTION",
    ///        "EXECUTION",
    ///        "ORDER_ACTION",
    ///        "TRANSFER",
    ///        "UNKNOWN"
    ///      ]
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "netAmount": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "orderId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "positionId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "settlementDate": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "status": {
    ///      "type": "string",
    ///      "enum": [
    ///        "VALID",
    ///        "INVALID",
    ///        "PENDING",
    ///        "UNKNOWN"
    ///      ]
    ///    },
    ///    "subAccount": {
    ///      "type": "string",
    ///      "enum": [
    ///        "CASH",
    ///        "MARGIN",
    ///        "SHORT",
    ///        "DIV",
    ///        "INCOME",
    ///        "UNKNOWN"
    ///      ]
    ///    },
    ///    "time": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "tradeDate": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "transferItems": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/TransferItem"
    ///      }
    ///    },
    ///    "type": {
    ///      "$ref": "#/components/schemas/TransactionType"
    ///    },
    ///    "user": {
    ///      "$ref": "#/components/schemas/UserDetails"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct Transaction {
        #[serde(
            rename = "accountNumber",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub account_number: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "activityId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub activity_id: ::std::option::Option<i64>,
        #[serde(
            rename = "activityType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub activity_type: ::std::option::Option<TransactionActivityType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "netAmount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_amount: ::std::option::Option<f64>,
        #[serde(
            rename = "orderId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub order_id: ::std::option::Option<i64>,
        #[serde(
            rename = "positionId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub position_id: ::std::option::Option<i64>,
        #[serde(
            rename = "settlementDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub settlement_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub status: ::std::option::Option<TransactionStatus>,
        #[serde(
            rename = "subAccount",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub sub_account: ::std::option::Option<TransactionSubAccount>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "tradeDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub trade_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "transferItems",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub transfer_items: ::std::vec::Vec<TransferItem>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<TransactionType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub user: ::std::option::Option<UserDetails>,
    }

    impl ::std::default::Default for Transaction {
        fn default() -> Self {
            Self {
                account_number: Default::default(),
                activity_id: Default::default(),
                activity_type: Default::default(),
                description: Default::default(),
                net_amount: Default::default(),
                order_id: Default::default(),
                position_id: Default::default(),
                settlement_date: Default::default(),
                status: Default::default(),
                sub_account: Default::default(),
                time: Default::default(),
                trade_date: Default::default(),
                transfer_items: Default::default(),
                type_: Default::default(),
                user: Default::default(),
            }
        }
    }

    impl Transaction {
        pub fn builder() -> builder::Transaction {
            Default::default()
        }
    }

    ///`TransactionActivityType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ACTIVITY_CORRECTION",
    ///    "EXECUTION",
    ///    "ORDER_ACTION",
    ///    "TRANSFER",
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
    pub enum TransactionActivityType {
        #[serde(rename = "ACTIVITY_CORRECTION")]
        ActivityCorrection,
        #[serde(rename = "EXECUTION")]
        Execution,
        #[serde(rename = "ORDER_ACTION")]
        OrderAction,
        #[serde(rename = "TRANSFER")]
        Transfer,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for TransactionActivityType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::ActivityCorrection => f.write_str("ACTIVITY_CORRECTION"),
                Self::Execution => f.write_str("EXECUTION"),
                Self::OrderAction => f.write_str("ORDER_ACTION"),
                Self::Transfer => f.write_str("TRANSFER"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for TransactionActivityType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ACTIVITY_CORRECTION" => Ok(Self::ActivityCorrection),
                "EXECUTION" => Ok(Self::Execution),
                "ORDER_ACTION" => Ok(Self::OrderAction),
                "TRANSFER" => Ok(Self::Transfer),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TransactionActivityType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TransactionActivityType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TransactionActivityType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`TransactionApiOptionDeliverable`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "assetType": {
    ///      "$ref": "#/components/schemas/assetType"
    ///    },
    ///    "deliverable": {},
    ///    "deliverableNumber": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "deliverableUnits": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "rootSymbol": {
    ///      "type": "string"
    ///    },
    ///    "strikePercent": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TransactionApiOptionDeliverable {
        #[serde(
            rename = "assetType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub asset_type: ::std::option::Option<AssetType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deliverable: ::std::option::Option<::std::boxed::Box<TransactionInstrument>>,
        #[serde(
            rename = "deliverableNumber",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub deliverable_number: ::std::option::Option<i64>,
        #[serde(
            rename = "deliverableUnits",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub deliverable_units: ::std::option::Option<f64>,
        #[serde(
            rename = "rootSymbol",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub root_symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "strikePercent",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub strike_percent: ::std::option::Option<i64>,
    }

    impl ::std::default::Default for TransactionApiOptionDeliverable {
        fn default() -> Self {
            Self {
                asset_type: Default::default(),
                deliverable: Default::default(),
                deliverable_number: Default::default(),
                deliverable_units: Default::default(),
                root_symbol: Default::default(),
                strike_percent: Default::default(),
            }
        }
    }

    impl TransactionApiOptionDeliverable {
        pub fn builder() -> builder::TransactionApiOptionDeliverable {
            Default::default()
        }
    }

    ///`TransactionBaseInstrument`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "assetType",
    ///    "name"
    ///  ],
    ///  "properties": {
    ///    "assetType": {
    ///      "type": "string",
    ///      "enum": [
    ///        "EQUITY",
    ///        "OPTION",
    ///        "INDEX",
    ///        "MUTUAL_FUND",
    ///        "CASH_EQUIVALENT",
    ///        "FIXED_INCOME",
    ///        "CURRENCY",
    ///        "COLLECTIVE_INVESTMENT"
    ///      ]
    ///    },
    ///    "cusip": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "instrumentId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "netChange": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "symbol": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TransactionBaseInstrument {
        #[serde(rename = "assetType")]
        pub asset_type: TransactionBaseInstrumentAssetType,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "instrumentId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub instrument_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::serde_json::Value::is_null")]
        pub name: ::serde_json::Value,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
    }

    impl TransactionBaseInstrument {
        pub fn builder() -> builder::TransactionBaseInstrument {
            Default::default()
        }
    }

    ///`TransactionBaseInstrumentAssetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EQUITY",
    ///    "OPTION",
    ///    "INDEX",
    ///    "MUTUAL_FUND",
    ///    "CASH_EQUIVALENT",
    ///    "FIXED_INCOME",
    ///    "CURRENCY",
    ///    "COLLECTIVE_INVESTMENT"
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
    pub enum TransactionBaseInstrumentAssetType {
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "CASH_EQUIVALENT")]
        CashEquivalent,
        #[serde(rename = "FIXED_INCOME")]
        FixedIncome,
        #[serde(rename = "CURRENCY")]
        Currency,
        #[serde(rename = "COLLECTIVE_INVESTMENT")]
        CollectiveInvestment,
    }

    impl ::std::fmt::Display for TransactionBaseInstrumentAssetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Equity => f.write_str("EQUITY"),
                Self::Option => f.write_str("OPTION"),
                Self::Index => f.write_str("INDEX"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::CashEquivalent => f.write_str("CASH_EQUIVALENT"),
                Self::FixedIncome => f.write_str("FIXED_INCOME"),
                Self::Currency => f.write_str("CURRENCY"),
                Self::CollectiveInvestment => f.write_str("COLLECTIVE_INVESTMENT"),
            }
        }
    }

    impl ::std::str::FromStr for TransactionBaseInstrumentAssetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EQUITY" => Ok(Self::Equity),
                "OPTION" => Ok(Self::Option),
                "INDEX" => Ok(Self::Index),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "CASH_EQUIVALENT" => Ok(Self::CashEquivalent),
                "FIXED_INCOME" => Ok(Self::FixedIncome),
                "CURRENCY" => Ok(Self::Currency),
                "COLLECTIVE_INVESTMENT" => Ok(Self::CollectiveInvestment),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TransactionBaseInstrumentAssetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TransactionBaseInstrumentAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TransactionBaseInstrumentAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`TransactionCashEquivalent`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/TransactionBaseInstrument"
    ///    }
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "SWEEP_VEHICLE",
    ///        "SAVINGS",
    ///        "MONEY_MARKET_FUND",
    ///        "UNKNOWN"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TransactionCashEquivalent {
        #[serde(rename = "assetType")]
        pub asset_type: TransactionCashEquivalentAssetType,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "instrumentId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub instrument_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::serde_json::Value::is_null")]
        pub name: ::serde_json::Value,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<TransactionCashEquivalentType>,
    }

    impl TransactionCashEquivalent {
        pub fn builder() -> builder::TransactionCashEquivalent {
            Default::default()
        }
    }

    ///`TransactionCashEquivalentAssetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EQUITY",
    ///    "OPTION",
    ///    "INDEX",
    ///    "MUTUAL_FUND",
    ///    "CASH_EQUIVALENT",
    ///    "FIXED_INCOME",
    ///    "CURRENCY",
    ///    "COLLECTIVE_INVESTMENT"
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
    pub enum TransactionCashEquivalentAssetType {
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "CASH_EQUIVALENT")]
        CashEquivalent,
        #[serde(rename = "FIXED_INCOME")]
        FixedIncome,
        #[serde(rename = "CURRENCY")]
        Currency,
        #[serde(rename = "COLLECTIVE_INVESTMENT")]
        CollectiveInvestment,
    }

    impl ::std::fmt::Display for TransactionCashEquivalentAssetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Equity => f.write_str("EQUITY"),
                Self::Option => f.write_str("OPTION"),
                Self::Index => f.write_str("INDEX"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::CashEquivalent => f.write_str("CASH_EQUIVALENT"),
                Self::FixedIncome => f.write_str("FIXED_INCOME"),
                Self::Currency => f.write_str("CURRENCY"),
                Self::CollectiveInvestment => f.write_str("COLLECTIVE_INVESTMENT"),
            }
        }
    }

    impl ::std::str::FromStr for TransactionCashEquivalentAssetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EQUITY" => Ok(Self::Equity),
                "OPTION" => Ok(Self::Option),
                "INDEX" => Ok(Self::Index),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "CASH_EQUIVALENT" => Ok(Self::CashEquivalent),
                "FIXED_INCOME" => Ok(Self::FixedIncome),
                "CURRENCY" => Ok(Self::Currency),
                "COLLECTIVE_INVESTMENT" => Ok(Self::CollectiveInvestment),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TransactionCashEquivalentAssetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TransactionCashEquivalentAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TransactionCashEquivalentAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`TransactionCashEquivalentType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "SWEEP_VEHICLE",
    ///    "SAVINGS",
    ///    "MONEY_MARKET_FUND",
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
    pub enum TransactionCashEquivalentType {
        #[serde(rename = "SWEEP_VEHICLE")]
        SweepVehicle,
        #[serde(rename = "SAVINGS")]
        Savings,
        #[serde(rename = "MONEY_MARKET_FUND")]
        MoneyMarketFund,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for TransactionCashEquivalentType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::SweepVehicle => f.write_str("SWEEP_VEHICLE"),
                Self::Savings => f.write_str("SAVINGS"),
                Self::MoneyMarketFund => f.write_str("MONEY_MARKET_FUND"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for TransactionCashEquivalentType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "SWEEP_VEHICLE" => Ok(Self::SweepVehicle),
                "SAVINGS" => Ok(Self::Savings),
                "MONEY_MARKET_FUND" => Ok(Self::MoneyMarketFund),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TransactionCashEquivalentType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TransactionCashEquivalentType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TransactionCashEquivalentType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`TransactionEquity`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/TransactionBaseInstrument"
    ///    }
    ///  ],
    ///  "properties": {
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "COMMON_STOCK",
    ///        "PREFERRED_STOCK",
    ///        "DEPOSITORY_RECEIPT",
    ///        "PREFERRED_DEPOSITORY_RECEIPT",
    ///        "RESTRICTED_STOCK",
    ///        "COMPONENT_UNIT",
    ///        "RIGHT",
    ///        "WARRANT",
    ///        "CONVERTIBLE_PREFERRED_STOCK",
    ///        "CONVERTIBLE_STOCK",
    ///        "LIMITED_PARTNERSHIP",
    ///        "WHEN_ISSUED",
    ///        "UNKNOWN"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TransactionEquity {
        #[serde(rename = "assetType")]
        pub asset_type: TransactionEquityAssetType,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "instrumentId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub instrument_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::serde_json::Value::is_null")]
        pub name: ::serde_json::Value,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<TransactionEquityType>,
    }

    impl TransactionEquity {
        pub fn builder() -> builder::TransactionEquity {
            Default::default()
        }
    }

    ///`TransactionEquityAssetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EQUITY",
    ///    "OPTION",
    ///    "INDEX",
    ///    "MUTUAL_FUND",
    ///    "CASH_EQUIVALENT",
    ///    "FIXED_INCOME",
    ///    "CURRENCY",
    ///    "COLLECTIVE_INVESTMENT"
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
    pub enum TransactionEquityAssetType {
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "CASH_EQUIVALENT")]
        CashEquivalent,
        #[serde(rename = "FIXED_INCOME")]
        FixedIncome,
        #[serde(rename = "CURRENCY")]
        Currency,
        #[serde(rename = "COLLECTIVE_INVESTMENT")]
        CollectiveInvestment,
    }

    impl ::std::fmt::Display for TransactionEquityAssetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Equity => f.write_str("EQUITY"),
                Self::Option => f.write_str("OPTION"),
                Self::Index => f.write_str("INDEX"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::CashEquivalent => f.write_str("CASH_EQUIVALENT"),
                Self::FixedIncome => f.write_str("FIXED_INCOME"),
                Self::Currency => f.write_str("CURRENCY"),
                Self::CollectiveInvestment => f.write_str("COLLECTIVE_INVESTMENT"),
            }
        }
    }

    impl ::std::str::FromStr for TransactionEquityAssetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EQUITY" => Ok(Self::Equity),
                "OPTION" => Ok(Self::Option),
                "INDEX" => Ok(Self::Index),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "CASH_EQUIVALENT" => Ok(Self::CashEquivalent),
                "FIXED_INCOME" => Ok(Self::FixedIncome),
                "CURRENCY" => Ok(Self::Currency),
                "COLLECTIVE_INVESTMENT" => Ok(Self::CollectiveInvestment),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TransactionEquityAssetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TransactionEquityAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TransactionEquityAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`TransactionEquityType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "COMMON_STOCK",
    ///    "PREFERRED_STOCK",
    ///    "DEPOSITORY_RECEIPT",
    ///    "PREFERRED_DEPOSITORY_RECEIPT",
    ///    "RESTRICTED_STOCK",
    ///    "COMPONENT_UNIT",
    ///    "RIGHT",
    ///    "WARRANT",
    ///    "CONVERTIBLE_PREFERRED_STOCK",
    ///    "CONVERTIBLE_STOCK",
    ///    "LIMITED_PARTNERSHIP",
    ///    "WHEN_ISSUED",
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
    pub enum TransactionEquityType {
        #[serde(rename = "COMMON_STOCK")]
        CommonStock,
        #[serde(rename = "PREFERRED_STOCK")]
        PreferredStock,
        #[serde(rename = "DEPOSITORY_RECEIPT")]
        DepositoryReceipt,
        #[serde(rename = "PREFERRED_DEPOSITORY_RECEIPT")]
        PreferredDepositoryReceipt,
        #[serde(rename = "RESTRICTED_STOCK")]
        RestrictedStock,
        #[serde(rename = "COMPONENT_UNIT")]
        ComponentUnit,
        #[serde(rename = "RIGHT")]
        Right,
        #[serde(rename = "WARRANT")]
        Warrant,
        #[serde(rename = "CONVERTIBLE_PREFERRED_STOCK")]
        ConvertiblePreferredStock,
        #[serde(rename = "CONVERTIBLE_STOCK")]
        ConvertibleStock,
        #[serde(rename = "LIMITED_PARTNERSHIP")]
        LimitedPartnership,
        #[serde(rename = "WHEN_ISSUED")]
        WhenIssued,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for TransactionEquityType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::CommonStock => f.write_str("COMMON_STOCK"),
                Self::PreferredStock => f.write_str("PREFERRED_STOCK"),
                Self::DepositoryReceipt => f.write_str("DEPOSITORY_RECEIPT"),
                Self::PreferredDepositoryReceipt => f.write_str("PREFERRED_DEPOSITORY_RECEIPT"),
                Self::RestrictedStock => f.write_str("RESTRICTED_STOCK"),
                Self::ComponentUnit => f.write_str("COMPONENT_UNIT"),
                Self::Right => f.write_str("RIGHT"),
                Self::Warrant => f.write_str("WARRANT"),
                Self::ConvertiblePreferredStock => f.write_str("CONVERTIBLE_PREFERRED_STOCK"),
                Self::ConvertibleStock => f.write_str("CONVERTIBLE_STOCK"),
                Self::LimitedPartnership => f.write_str("LIMITED_PARTNERSHIP"),
                Self::WhenIssued => f.write_str("WHEN_ISSUED"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for TransactionEquityType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "COMMON_STOCK" => Ok(Self::CommonStock),
                "PREFERRED_STOCK" => Ok(Self::PreferredStock),
                "DEPOSITORY_RECEIPT" => Ok(Self::DepositoryReceipt),
                "PREFERRED_DEPOSITORY_RECEIPT" => Ok(Self::PreferredDepositoryReceipt),
                "RESTRICTED_STOCK" => Ok(Self::RestrictedStock),
                "COMPONENT_UNIT" => Ok(Self::ComponentUnit),
                "RIGHT" => Ok(Self::Right),
                "WARRANT" => Ok(Self::Warrant),
                "CONVERTIBLE_PREFERRED_STOCK" => Ok(Self::ConvertiblePreferredStock),
                "CONVERTIBLE_STOCK" => Ok(Self::ConvertibleStock),
                "LIMITED_PARTNERSHIP" => Ok(Self::LimitedPartnership),
                "WHEN_ISSUED" => Ok(Self::WhenIssued),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TransactionEquityType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TransactionEquityType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TransactionEquityType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`TransactionFixedIncome`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/TransactionBaseInstrument"
    ///    }
    ///  ],
    ///  "properties": {
    ///    "factor": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "maturityDate": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "multiplier": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "BOND_UNIT",
    ///        "CERTIFICATE_OF_DEPOSIT",
    ///        "CONVERTIBLE_BOND",
    ///        "COLLATERALIZED_MORTGAGE_OBLIGATION",
    ///        "CORPORATE_BOND",
    ///        "GOVERNMENT_MORTGAGE",
    ///        "GNMA_BONDS",
    ///        "MUNICIPAL_ASSESSMENT_DISTRICT",
    ///        "MUNICIPAL_BOND",
    ///        "OTHER_GOVERNMENT",
    ///        "SHORT_TERM_PAPER",
    ///        "US_TREASURY_BOND",
    ///        "US_TREASURY_BILL",
    ///        "US_TREASURY_NOTE",
    ///        "US_TREASURY_ZERO_COUPON",
    ///        "AGENCY_BOND",
    ///        "WHEN_AS_AND_IF_ISSUED_BOND",
    ///        "ASSET_BACKED_SECURITY",
    ///        "UNKNOWN"
    ///      ]
    ///    },
    ///    "variableRate": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TransactionFixedIncome {
        #[serde(rename = "assetType")]
        pub asset_type: TransactionFixedIncomeAssetType,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub factor: ::std::option::Option<f64>,
        #[serde(
            rename = "instrumentId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub instrument_id: ::std::option::Option<i64>,
        #[serde(
            rename = "maturityDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub maturity_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub multiplier: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::serde_json::Value::is_null")]
        pub name: ::serde_json::Value,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<TransactionFixedIncomeType>,
        #[serde(
            rename = "variableRate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub variable_rate: ::std::option::Option<f64>,
    }

    impl TransactionFixedIncome {
        pub fn builder() -> builder::TransactionFixedIncome {
            Default::default()
        }
    }

    ///`TransactionFixedIncomeAssetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EQUITY",
    ///    "OPTION",
    ///    "INDEX",
    ///    "MUTUAL_FUND",
    ///    "CASH_EQUIVALENT",
    ///    "FIXED_INCOME",
    ///    "CURRENCY",
    ///    "COLLECTIVE_INVESTMENT"
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
    pub enum TransactionFixedIncomeAssetType {
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "CASH_EQUIVALENT")]
        CashEquivalent,
        #[serde(rename = "FIXED_INCOME")]
        FixedIncome,
        #[serde(rename = "CURRENCY")]
        Currency,
        #[serde(rename = "COLLECTIVE_INVESTMENT")]
        CollectiveInvestment,
    }

    impl ::std::fmt::Display for TransactionFixedIncomeAssetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Equity => f.write_str("EQUITY"),
                Self::Option => f.write_str("OPTION"),
                Self::Index => f.write_str("INDEX"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::CashEquivalent => f.write_str("CASH_EQUIVALENT"),
                Self::FixedIncome => f.write_str("FIXED_INCOME"),
                Self::Currency => f.write_str("CURRENCY"),
                Self::CollectiveInvestment => f.write_str("COLLECTIVE_INVESTMENT"),
            }
        }
    }

    impl ::std::str::FromStr for TransactionFixedIncomeAssetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EQUITY" => Ok(Self::Equity),
                "OPTION" => Ok(Self::Option),
                "INDEX" => Ok(Self::Index),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "CASH_EQUIVALENT" => Ok(Self::CashEquivalent),
                "FIXED_INCOME" => Ok(Self::FixedIncome),
                "CURRENCY" => Ok(Self::Currency),
                "COLLECTIVE_INVESTMENT" => Ok(Self::CollectiveInvestment),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TransactionFixedIncomeAssetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TransactionFixedIncomeAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TransactionFixedIncomeAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`TransactionFixedIncomeType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "BOND_UNIT",
    ///    "CERTIFICATE_OF_DEPOSIT",
    ///    "CONVERTIBLE_BOND",
    ///    "COLLATERALIZED_MORTGAGE_OBLIGATION",
    ///    "CORPORATE_BOND",
    ///    "GOVERNMENT_MORTGAGE",
    ///    "GNMA_BONDS",
    ///    "MUNICIPAL_ASSESSMENT_DISTRICT",
    ///    "MUNICIPAL_BOND",
    ///    "OTHER_GOVERNMENT",
    ///    "SHORT_TERM_PAPER",
    ///    "US_TREASURY_BOND",
    ///    "US_TREASURY_BILL",
    ///    "US_TREASURY_NOTE",
    ///    "US_TREASURY_ZERO_COUPON",
    ///    "AGENCY_BOND",
    ///    "WHEN_AS_AND_IF_ISSUED_BOND",
    ///    "ASSET_BACKED_SECURITY",
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
    pub enum TransactionFixedIncomeType {
        #[serde(rename = "BOND_UNIT")]
        BondUnit,
        #[serde(rename = "CERTIFICATE_OF_DEPOSIT")]
        CertificateOfDeposit,
        #[serde(rename = "CONVERTIBLE_BOND")]
        ConvertibleBond,
        #[serde(rename = "COLLATERALIZED_MORTGAGE_OBLIGATION")]
        CollateralizedMortgageObligation,
        #[serde(rename = "CORPORATE_BOND")]
        CorporateBond,
        #[serde(rename = "GOVERNMENT_MORTGAGE")]
        GovernmentMortgage,
        #[serde(rename = "GNMA_BONDS")]
        GnmaBonds,
        #[serde(rename = "MUNICIPAL_ASSESSMENT_DISTRICT")]
        MunicipalAssessmentDistrict,
        #[serde(rename = "MUNICIPAL_BOND")]
        MunicipalBond,
        #[serde(rename = "OTHER_GOVERNMENT")]
        OtherGovernment,
        #[serde(rename = "SHORT_TERM_PAPER")]
        ShortTermPaper,
        #[serde(rename = "US_TREASURY_BOND")]
        UsTreasuryBond,
        #[serde(rename = "US_TREASURY_BILL")]
        UsTreasuryBill,
        #[serde(rename = "US_TREASURY_NOTE")]
        UsTreasuryNote,
        #[serde(rename = "US_TREASURY_ZERO_COUPON")]
        UsTreasuryZeroCoupon,
        #[serde(rename = "AGENCY_BOND")]
        AgencyBond,
        #[serde(rename = "WHEN_AS_AND_IF_ISSUED_BOND")]
        WhenAsAndIfIssuedBond,
        #[serde(rename = "ASSET_BACKED_SECURITY")]
        AssetBackedSecurity,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for TransactionFixedIncomeType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::BondUnit => f.write_str("BOND_UNIT"),
                Self::CertificateOfDeposit => f.write_str("CERTIFICATE_OF_DEPOSIT"),
                Self::ConvertibleBond => f.write_str("CONVERTIBLE_BOND"),
                Self::CollateralizedMortgageObligation => {
                    f.write_str("COLLATERALIZED_MORTGAGE_OBLIGATION")
                }
                Self::CorporateBond => f.write_str("CORPORATE_BOND"),
                Self::GovernmentMortgage => f.write_str("GOVERNMENT_MORTGAGE"),
                Self::GnmaBonds => f.write_str("GNMA_BONDS"),
                Self::MunicipalAssessmentDistrict => f.write_str("MUNICIPAL_ASSESSMENT_DISTRICT"),
                Self::MunicipalBond => f.write_str("MUNICIPAL_BOND"),
                Self::OtherGovernment => f.write_str("OTHER_GOVERNMENT"),
                Self::ShortTermPaper => f.write_str("SHORT_TERM_PAPER"),
                Self::UsTreasuryBond => f.write_str("US_TREASURY_BOND"),
                Self::UsTreasuryBill => f.write_str("US_TREASURY_BILL"),
                Self::UsTreasuryNote => f.write_str("US_TREASURY_NOTE"),
                Self::UsTreasuryZeroCoupon => f.write_str("US_TREASURY_ZERO_COUPON"),
                Self::AgencyBond => f.write_str("AGENCY_BOND"),
                Self::WhenAsAndIfIssuedBond => f.write_str("WHEN_AS_AND_IF_ISSUED_BOND"),
                Self::AssetBackedSecurity => f.write_str("ASSET_BACKED_SECURITY"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for TransactionFixedIncomeType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "BOND_UNIT" => Ok(Self::BondUnit),
                "CERTIFICATE_OF_DEPOSIT" => Ok(Self::CertificateOfDeposit),
                "CONVERTIBLE_BOND" => Ok(Self::ConvertibleBond),
                "COLLATERALIZED_MORTGAGE_OBLIGATION" => Ok(Self::CollateralizedMortgageObligation),
                "CORPORATE_BOND" => Ok(Self::CorporateBond),
                "GOVERNMENT_MORTGAGE" => Ok(Self::GovernmentMortgage),
                "GNMA_BONDS" => Ok(Self::GnmaBonds),
                "MUNICIPAL_ASSESSMENT_DISTRICT" => Ok(Self::MunicipalAssessmentDistrict),
                "MUNICIPAL_BOND" => Ok(Self::MunicipalBond),
                "OTHER_GOVERNMENT" => Ok(Self::OtherGovernment),
                "SHORT_TERM_PAPER" => Ok(Self::ShortTermPaper),
                "US_TREASURY_BOND" => Ok(Self::UsTreasuryBond),
                "US_TREASURY_BILL" => Ok(Self::UsTreasuryBill),
                "US_TREASURY_NOTE" => Ok(Self::UsTreasuryNote),
                "US_TREASURY_ZERO_COUPON" => Ok(Self::UsTreasuryZeroCoupon),
                "AGENCY_BOND" => Ok(Self::AgencyBond),
                "WHEN_AS_AND_IF_ISSUED_BOND" => Ok(Self::WhenAsAndIfIssuedBond),
                "ASSET_BACKED_SECURITY" => Ok(Self::AssetBackedSecurity),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TransactionFixedIncomeType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TransactionFixedIncomeType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TransactionFixedIncomeType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`TransactionInstrument`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "oneOf": [
    ///    {
    ///      "$ref": "#/components/schemas/TransactionCashEquivalent"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/CollectiveInvestment"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Currency"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/TransactionEquity"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/TransactionFixedIncome"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Forex"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Future"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Index"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/TransactionMutualFund"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/TransactionOption"
    ///    },
    ///    {
    ///      "$ref": "#/components/schemas/Product"
    ///    }
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    #[serde(untagged)]
    pub enum TransactionInstrument {
        TransactionCashEquivalent(TransactionCashEquivalent),
        CollectiveInvestment(CollectiveInvestment),
        Currency(Currency),
        TransactionEquity(TransactionEquity),
        TransactionFixedIncome(TransactionFixedIncome),
        Forex(Forex),
        Future(Future),
        Index(Index),
        TransactionMutualFund(TransactionMutualFund),
        TransactionOption(TransactionOption),
        Product(Product),
    }

    impl ::std::convert::From<TransactionCashEquivalent> for TransactionInstrument {
        fn from(value: TransactionCashEquivalent) -> Self {
            Self::TransactionCashEquivalent(value)
        }
    }

    impl ::std::convert::From<CollectiveInvestment> for TransactionInstrument {
        fn from(value: CollectiveInvestment) -> Self {
            Self::CollectiveInvestment(value)
        }
    }

    impl ::std::convert::From<Currency> for TransactionInstrument {
        fn from(value: Currency) -> Self {
            Self::Currency(value)
        }
    }

    impl ::std::convert::From<TransactionEquity> for TransactionInstrument {
        fn from(value: TransactionEquity) -> Self {
            Self::TransactionEquity(value)
        }
    }

    impl ::std::convert::From<TransactionFixedIncome> for TransactionInstrument {
        fn from(value: TransactionFixedIncome) -> Self {
            Self::TransactionFixedIncome(value)
        }
    }

    impl ::std::convert::From<Forex> for TransactionInstrument {
        fn from(value: Forex) -> Self {
            Self::Forex(value)
        }
    }

    impl ::std::convert::From<Future> for TransactionInstrument {
        fn from(value: Future) -> Self {
            Self::Future(value)
        }
    }

    impl ::std::convert::From<Index> for TransactionInstrument {
        fn from(value: Index) -> Self {
            Self::Index(value)
        }
    }

    impl ::std::convert::From<TransactionMutualFund> for TransactionInstrument {
        fn from(value: TransactionMutualFund) -> Self {
            Self::TransactionMutualFund(value)
        }
    }

    impl ::std::convert::From<TransactionOption> for TransactionInstrument {
        fn from(value: TransactionOption) -> Self {
            Self::TransactionOption(value)
        }
    }

    impl ::std::convert::From<Product> for TransactionInstrument {
        fn from(value: Product) -> Self {
            Self::Product(value)
        }
    }

    ///`TransactionMutualFund`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/TransactionBaseInstrument"
    ///    }
    ///  ],
    ///  "properties": {
    ///    "exchangeCutoffTime": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "fundFamilyName": {
    ///      "type": "string"
    ///    },
    ///    "fundFamilySymbol": {
    ///      "type": "string"
    ///    },
    ///    "fundGroup": {
    ///      "type": "string"
    ///    },
    ///    "purchaseCutoffTime": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "redemptionCutoffTime": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "NOT_APPLICABLE",
    ///        "OPEN_END_NON_TAXABLE",
    ///        "OPEN_END_TAXABLE",
    ///        "NO_LOAD_NON_TAXABLE",
    ///        "NO_LOAD_TAXABLE",
    ///        "UNKNOWN"
    ///      ]
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TransactionMutualFund {
        #[serde(rename = "assetType")]
        pub asset_type: TransactionMutualFundAssetType,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "exchangeCutoffTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub exchange_cutoff_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "fundFamilyName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub fund_family_name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "fundFamilySymbol",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub fund_family_symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "fundGroup",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub fund_group: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "instrumentId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub instrument_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::serde_json::Value::is_null")]
        pub name: ::serde_json::Value,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(
            rename = "purchaseCutoffTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub purchase_cutoff_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "redemptionCutoffTime",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub redemption_cutoff_time:
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<TransactionMutualFundType>,
    }

    impl TransactionMutualFund {
        pub fn builder() -> builder::TransactionMutualFund {
            Default::default()
        }
    }

    ///`TransactionMutualFundAssetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EQUITY",
    ///    "OPTION",
    ///    "INDEX",
    ///    "MUTUAL_FUND",
    ///    "CASH_EQUIVALENT",
    ///    "FIXED_INCOME",
    ///    "CURRENCY",
    ///    "COLLECTIVE_INVESTMENT"
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
    pub enum TransactionMutualFundAssetType {
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "CASH_EQUIVALENT")]
        CashEquivalent,
        #[serde(rename = "FIXED_INCOME")]
        FixedIncome,
        #[serde(rename = "CURRENCY")]
        Currency,
        #[serde(rename = "COLLECTIVE_INVESTMENT")]
        CollectiveInvestment,
    }

    impl ::std::fmt::Display for TransactionMutualFundAssetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Equity => f.write_str("EQUITY"),
                Self::Option => f.write_str("OPTION"),
                Self::Index => f.write_str("INDEX"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::CashEquivalent => f.write_str("CASH_EQUIVALENT"),
                Self::FixedIncome => f.write_str("FIXED_INCOME"),
                Self::Currency => f.write_str("CURRENCY"),
                Self::CollectiveInvestment => f.write_str("COLLECTIVE_INVESTMENT"),
            }
        }
    }

    impl ::std::str::FromStr for TransactionMutualFundAssetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EQUITY" => Ok(Self::Equity),
                "OPTION" => Ok(Self::Option),
                "INDEX" => Ok(Self::Index),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "CASH_EQUIVALENT" => Ok(Self::CashEquivalent),
                "FIXED_INCOME" => Ok(Self::FixedIncome),
                "CURRENCY" => Ok(Self::Currency),
                "COLLECTIVE_INVESTMENT" => Ok(Self::CollectiveInvestment),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TransactionMutualFundAssetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TransactionMutualFundAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TransactionMutualFundAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`TransactionMutualFundType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "NOT_APPLICABLE",
    ///    "OPEN_END_NON_TAXABLE",
    ///    "OPEN_END_TAXABLE",
    ///    "NO_LOAD_NON_TAXABLE",
    ///    "NO_LOAD_TAXABLE",
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
    pub enum TransactionMutualFundType {
        #[serde(rename = "NOT_APPLICABLE")]
        NotApplicable,
        #[serde(rename = "OPEN_END_NON_TAXABLE")]
        OpenEndNonTaxable,
        #[serde(rename = "OPEN_END_TAXABLE")]
        OpenEndTaxable,
        #[serde(rename = "NO_LOAD_NON_TAXABLE")]
        NoLoadNonTaxable,
        #[serde(rename = "NO_LOAD_TAXABLE")]
        NoLoadTaxable,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for TransactionMutualFundType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::NotApplicable => f.write_str("NOT_APPLICABLE"),
                Self::OpenEndNonTaxable => f.write_str("OPEN_END_NON_TAXABLE"),
                Self::OpenEndTaxable => f.write_str("OPEN_END_TAXABLE"),
                Self::NoLoadNonTaxable => f.write_str("NO_LOAD_NON_TAXABLE"),
                Self::NoLoadTaxable => f.write_str("NO_LOAD_TAXABLE"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for TransactionMutualFundType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "NOT_APPLICABLE" => Ok(Self::NotApplicable),
                "OPEN_END_NON_TAXABLE" => Ok(Self::OpenEndNonTaxable),
                "OPEN_END_TAXABLE" => Ok(Self::OpenEndTaxable),
                "NO_LOAD_NON_TAXABLE" => Ok(Self::NoLoadNonTaxable),
                "NO_LOAD_TAXABLE" => Ok(Self::NoLoadTaxable),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TransactionMutualFundType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TransactionMutualFundType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TransactionMutualFundType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`TransactionOption`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "allOf": [
    ///    {
    ///      "$ref": "#/components/schemas/TransactionBaseInstrument"
    ///    }
    ///  ],
    ///  "properties": {
    ///    "deliverable": {},
    ///    "expirationDate": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "optionDeliverables": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/TransactionAPIOptionDeliverable"
    ///      }
    ///    },
    ///    "optionPremiumMultiplier": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "putCall": {
    ///      "type": "string",
    ///      "enum": [
    ///        "PUT",
    ///        "CALL",
    ///        "UNKNOWN"
    ///      ]
    ///    },
    ///    "strikePrice": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "VANILLA",
    ///        "BINARY",
    ///        "BARRIER",
    ///        "UNKNOWN"
    ///      ]
    ///    },
    ///    "underlyingCusip": {
    ///      "type": "string"
    ///    },
    ///    "underlyingSymbol": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TransactionOption {
        #[serde(rename = "assetType")]
        pub asset_type: TransactionOptionAssetType,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cusip: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deliverable: ::std::option::Option<::std::boxed::Box<TransactionInstrument>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub description: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "expirationDate",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub expiration_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(
            rename = "instrumentId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub instrument_id: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "::serde_json::Value::is_null")]
        pub name: ::serde_json::Value,
        #[serde(
            rename = "netChange",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub net_change: ::std::option::Option<f64>,
        #[serde(
            rename = "optionDeliverables",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub option_deliverables: ::std::vec::Vec<TransactionApiOptionDeliverable>,
        #[serde(
            rename = "optionPremiumMultiplier",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub option_premium_multiplier: ::std::option::Option<i64>,
        #[serde(
            rename = "putCall",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub put_call: ::std::option::Option<TransactionOptionPutCall>,
        #[serde(
            rename = "strikePrice",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub strike_price: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub symbol: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<TransactionOptionType>,
        #[serde(
            rename = "underlyingCusip",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub underlying_cusip: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "underlyingSymbol",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub underlying_symbol: ::std::option::Option<::std::string::String>,
    }

    impl TransactionOption {
        pub fn builder() -> builder::TransactionOption {
            Default::default()
        }
    }

    ///`TransactionOptionAssetType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "EQUITY",
    ///    "OPTION",
    ///    "INDEX",
    ///    "MUTUAL_FUND",
    ///    "CASH_EQUIVALENT",
    ///    "FIXED_INCOME",
    ///    "CURRENCY",
    ///    "COLLECTIVE_INVESTMENT"
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
    pub enum TransactionOptionAssetType {
        #[serde(rename = "EQUITY")]
        Equity,
        #[serde(rename = "OPTION")]
        Option,
        #[serde(rename = "INDEX")]
        Index,
        #[serde(rename = "MUTUAL_FUND")]
        MutualFund,
        #[serde(rename = "CASH_EQUIVALENT")]
        CashEquivalent,
        #[serde(rename = "FIXED_INCOME")]
        FixedIncome,
        #[serde(rename = "CURRENCY")]
        Currency,
        #[serde(rename = "COLLECTIVE_INVESTMENT")]
        CollectiveInvestment,
    }

    impl ::std::fmt::Display for TransactionOptionAssetType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Equity => f.write_str("EQUITY"),
                Self::Option => f.write_str("OPTION"),
                Self::Index => f.write_str("INDEX"),
                Self::MutualFund => f.write_str("MUTUAL_FUND"),
                Self::CashEquivalent => f.write_str("CASH_EQUIVALENT"),
                Self::FixedIncome => f.write_str("FIXED_INCOME"),
                Self::Currency => f.write_str("CURRENCY"),
                Self::CollectiveInvestment => f.write_str("COLLECTIVE_INVESTMENT"),
            }
        }
    }

    impl ::std::str::FromStr for TransactionOptionAssetType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "EQUITY" => Ok(Self::Equity),
                "OPTION" => Ok(Self::Option),
                "INDEX" => Ok(Self::Index),
                "MUTUAL_FUND" => Ok(Self::MutualFund),
                "CASH_EQUIVALENT" => Ok(Self::CashEquivalent),
                "FIXED_INCOME" => Ok(Self::FixedIncome),
                "CURRENCY" => Ok(Self::Currency),
                "COLLECTIVE_INVESTMENT" => Ok(Self::CollectiveInvestment),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TransactionOptionAssetType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TransactionOptionAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TransactionOptionAssetType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`TransactionOptionPutCall`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "PUT",
    ///    "CALL",
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
    pub enum TransactionOptionPutCall {
        #[serde(rename = "PUT")]
        Put,
        #[serde(rename = "CALL")]
        Call,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for TransactionOptionPutCall {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Put => f.write_str("PUT"),
                Self::Call => f.write_str("CALL"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for TransactionOptionPutCall {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "PUT" => Ok(Self::Put),
                "CALL" => Ok(Self::Call),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TransactionOptionPutCall {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TransactionOptionPutCall {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TransactionOptionPutCall {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`TransactionOptionType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "VANILLA",
    ///    "BINARY",
    ///    "BARRIER",
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
    pub enum TransactionOptionType {
        #[serde(rename = "VANILLA")]
        Vanilla,
        #[serde(rename = "BINARY")]
        Binary,
        #[serde(rename = "BARRIER")]
        Barrier,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for TransactionOptionType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Vanilla => f.write_str("VANILLA"),
                Self::Binary => f.write_str("BINARY"),
                Self::Barrier => f.write_str("BARRIER"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for TransactionOptionType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "VANILLA" => Ok(Self::Vanilla),
                "BINARY" => Ok(Self::Binary),
                "BARRIER" => Ok(Self::Barrier),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TransactionOptionType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TransactionOptionType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TransactionOptionType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`TransactionStatus`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "VALID",
    ///    "INVALID",
    ///    "PENDING",
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
    pub enum TransactionStatus {
        #[serde(rename = "VALID")]
        Valid,
        #[serde(rename = "INVALID")]
        Invalid,
        #[serde(rename = "PENDING")]
        Pending,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for TransactionStatus {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Valid => f.write_str("VALID"),
                Self::Invalid => f.write_str("INVALID"),
                Self::Pending => f.write_str("PENDING"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for TransactionStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "VALID" => Ok(Self::Valid),
                "INVALID" => Ok(Self::Invalid),
                "PENDING" => Ok(Self::Pending),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TransactionStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TransactionStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TransactionStatus {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`TransactionSubAccount`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "CASH",
    ///    "MARGIN",
    ///    "SHORT",
    ///    "DIV",
    ///    "INCOME",
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
    pub enum TransactionSubAccount {
        #[serde(rename = "CASH")]
        Cash,
        #[serde(rename = "MARGIN")]
        Margin,
        #[serde(rename = "SHORT")]
        Short,
        #[serde(rename = "DIV")]
        Div,
        #[serde(rename = "INCOME")]
        Income,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for TransactionSubAccount {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Cash => f.write_str("CASH"),
                Self::Margin => f.write_str("MARGIN"),
                Self::Short => f.write_str("SHORT"),
                Self::Div => f.write_str("DIV"),
                Self::Income => f.write_str("INCOME"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for TransactionSubAccount {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "CASH" => Ok(Self::Cash),
                "MARGIN" => Ok(Self::Margin),
                "SHORT" => Ok(Self::Short),
                "DIV" => Ok(Self::Div),
                "INCOME" => Ok(Self::Income),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TransactionSubAccount {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TransactionSubAccount {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TransactionSubAccount {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`TransactionType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "TRADE",
    ///    "RECEIVE_AND_DELIVER",
    ///    "DIVIDEND_OR_INTEREST",
    ///    "ACH_RECEIPT",
    ///    "ACH_DISBURSEMENT",
    ///    "CASH_RECEIPT",
    ///    "CASH_DISBURSEMENT",
    ///    "ELECTRONIC_FUND",
    ///    "WIRE_OUT",
    ///    "WIRE_IN",
    ///    "JOURNAL",
    ///    "MEMORANDUM",
    ///    "MARGIN_CALL",
    ///    "MONEY_MARKET",
    ///    "SMA_ADJUSTMENT"
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
    pub enum TransactionType {
        #[serde(rename = "TRADE")]
        Trade,
        #[serde(rename = "RECEIVE_AND_DELIVER")]
        ReceiveAndDeliver,
        #[serde(rename = "DIVIDEND_OR_INTEREST")]
        DividendOrInterest,
        #[serde(rename = "ACH_RECEIPT")]
        AchReceipt,
        #[serde(rename = "ACH_DISBURSEMENT")]
        AchDisbursement,
        #[serde(rename = "CASH_RECEIPT")]
        CashReceipt,
        #[serde(rename = "CASH_DISBURSEMENT")]
        CashDisbursement,
        #[serde(rename = "ELECTRONIC_FUND")]
        ElectronicFund,
        #[serde(rename = "WIRE_OUT")]
        WireOut,
        #[serde(rename = "WIRE_IN")]
        WireIn,
        #[serde(rename = "JOURNAL")]
        Journal,
        #[serde(rename = "MEMORANDUM")]
        Memorandum,
        #[serde(rename = "MARGIN_CALL")]
        MarginCall,
        #[serde(rename = "MONEY_MARKET")]
        MoneyMarket,
        #[serde(rename = "SMA_ADJUSTMENT")]
        SmaAdjustment,
    }

    impl ::std::fmt::Display for TransactionType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Trade => f.write_str("TRADE"),
                Self::ReceiveAndDeliver => f.write_str("RECEIVE_AND_DELIVER"),
                Self::DividendOrInterest => f.write_str("DIVIDEND_OR_INTEREST"),
                Self::AchReceipt => f.write_str("ACH_RECEIPT"),
                Self::AchDisbursement => f.write_str("ACH_DISBURSEMENT"),
                Self::CashReceipt => f.write_str("CASH_RECEIPT"),
                Self::CashDisbursement => f.write_str("CASH_DISBURSEMENT"),
                Self::ElectronicFund => f.write_str("ELECTRONIC_FUND"),
                Self::WireOut => f.write_str("WIRE_OUT"),
                Self::WireIn => f.write_str("WIRE_IN"),
                Self::Journal => f.write_str("JOURNAL"),
                Self::Memorandum => f.write_str("MEMORANDUM"),
                Self::MarginCall => f.write_str("MARGIN_CALL"),
                Self::MoneyMarket => f.write_str("MONEY_MARKET"),
                Self::SmaAdjustment => f.write_str("SMA_ADJUSTMENT"),
            }
        }
    }

    impl ::std::str::FromStr for TransactionType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "TRADE" => Ok(Self::Trade),
                "RECEIVE_AND_DELIVER" => Ok(Self::ReceiveAndDeliver),
                "DIVIDEND_OR_INTEREST" => Ok(Self::DividendOrInterest),
                "ACH_RECEIPT" => Ok(Self::AchReceipt),
                "ACH_DISBURSEMENT" => Ok(Self::AchDisbursement),
                "CASH_RECEIPT" => Ok(Self::CashReceipt),
                "CASH_DISBURSEMENT" => Ok(Self::CashDisbursement),
                "ELECTRONIC_FUND" => Ok(Self::ElectronicFund),
                "WIRE_OUT" => Ok(Self::WireOut),
                "WIRE_IN" => Ok(Self::WireIn),
                "JOURNAL" => Ok(Self::Journal),
                "MEMORANDUM" => Ok(Self::Memorandum),
                "MARGIN_CALL" => Ok(Self::MarginCall),
                "MONEY_MARKET" => Ok(Self::MoneyMarket),
                "SMA_ADJUSTMENT" => Ok(Self::SmaAdjustment),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TransactionType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TransactionType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TransactionType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`TransferItem`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "amount": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "cost": {
    ///      "type": "number",
    ///      "format": "double"
    ///    },
    ///    "feeType": {
    ///      "type": "string",
    ///      "enum": [
    ///        "COMMISSION",
    ///        "SEC_FEE",
    ///        "STR_FEE",
    ///        "R_FEE",
    ///        "CDSC_FEE",
    ///        "OPT_REG_FEE",
    ///        "ADDITIONAL_FEE",
    ///        "MISCELLANEOUS_FEE",
    ///        "FUTURES_EXCHANGE_FEE",
    ///        "LOW_PROCEEDS_COMMISSION",
    ///        "BASE_CHARGE",
    ///        "GENERAL_CHARGE",
    ///        "GST_FEE",
    ///        "TAF_FEE",
    ///        "INDEX_OPTION_FEE",
    ///        "UNKNOWN"
    ///      ]
    ///    },
    ///    "instrument": {
    ///      "$ref": "#/components/schemas/TransactionInstrument"
    ///    },
    ///    "positionEffect": {
    ///      "type": "string",
    ///      "enum": [
    ///        "OPENING",
    ///        "CLOSING",
    ///        "AUTOMATIC",
    ///        "UNKNOWN"
    ///      ]
    ///    },
    ///    "price": {
    ///      "type": "number",
    ///      "format": "double"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct TransferItem {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub amount: ::std::option::Option<f64>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub cost: ::std::option::Option<f64>,
        #[serde(
            rename = "feeType",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub fee_type: ::std::option::Option<TransferItemFeeType>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub instrument: ::std::option::Option<TransactionInstrument>,
        #[serde(
            rename = "positionEffect",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub position_effect: ::std::option::Option<TransferItemPositionEffect>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub price: ::std::option::Option<f64>,
    }

    impl ::std::default::Default for TransferItem {
        fn default() -> Self {
            Self {
                amount: Default::default(),
                cost: Default::default(),
                fee_type: Default::default(),
                instrument: Default::default(),
                position_effect: Default::default(),
                price: Default::default(),
            }
        }
    }

    impl TransferItem {
        pub fn builder() -> builder::TransferItem {
            Default::default()
        }
    }

    ///`TransferItemFeeType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "COMMISSION",
    ///    "SEC_FEE",
    ///    "STR_FEE",
    ///    "R_FEE",
    ///    "CDSC_FEE",
    ///    "OPT_REG_FEE",
    ///    "ADDITIONAL_FEE",
    ///    "MISCELLANEOUS_FEE",
    ///    "FUTURES_EXCHANGE_FEE",
    ///    "LOW_PROCEEDS_COMMISSION",
    ///    "BASE_CHARGE",
    ///    "GENERAL_CHARGE",
    ///    "GST_FEE",
    ///    "TAF_FEE",
    ///    "INDEX_OPTION_FEE",
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
    pub enum TransferItemFeeType {
        #[serde(rename = "COMMISSION")]
        Commission,
        #[serde(rename = "SEC_FEE")]
        SecFee,
        #[serde(rename = "STR_FEE")]
        StrFee,
        #[serde(rename = "R_FEE")]
        RFee,
        #[serde(rename = "CDSC_FEE")]
        CdscFee,
        #[serde(rename = "OPT_REG_FEE")]
        OptRegFee,
        #[serde(rename = "ADDITIONAL_FEE")]
        AdditionalFee,
        #[serde(rename = "MISCELLANEOUS_FEE")]
        MiscellaneousFee,
        #[serde(rename = "FUTURES_EXCHANGE_FEE")]
        FuturesExchangeFee,
        #[serde(rename = "LOW_PROCEEDS_COMMISSION")]
        LowProceedsCommission,
        #[serde(rename = "BASE_CHARGE")]
        BaseCharge,
        #[serde(rename = "GENERAL_CHARGE")]
        GeneralCharge,
        #[serde(rename = "GST_FEE")]
        GstFee,
        #[serde(rename = "TAF_FEE")]
        TafFee,
        #[serde(rename = "INDEX_OPTION_FEE")]
        IndexOptionFee,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for TransferItemFeeType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Commission => f.write_str("COMMISSION"),
                Self::SecFee => f.write_str("SEC_FEE"),
                Self::StrFee => f.write_str("STR_FEE"),
                Self::RFee => f.write_str("R_FEE"),
                Self::CdscFee => f.write_str("CDSC_FEE"),
                Self::OptRegFee => f.write_str("OPT_REG_FEE"),
                Self::AdditionalFee => f.write_str("ADDITIONAL_FEE"),
                Self::MiscellaneousFee => f.write_str("MISCELLANEOUS_FEE"),
                Self::FuturesExchangeFee => f.write_str("FUTURES_EXCHANGE_FEE"),
                Self::LowProceedsCommission => f.write_str("LOW_PROCEEDS_COMMISSION"),
                Self::BaseCharge => f.write_str("BASE_CHARGE"),
                Self::GeneralCharge => f.write_str("GENERAL_CHARGE"),
                Self::GstFee => f.write_str("GST_FEE"),
                Self::TafFee => f.write_str("TAF_FEE"),
                Self::IndexOptionFee => f.write_str("INDEX_OPTION_FEE"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for TransferItemFeeType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "COMMISSION" => Ok(Self::Commission),
                "SEC_FEE" => Ok(Self::SecFee),
                "STR_FEE" => Ok(Self::StrFee),
                "R_FEE" => Ok(Self::RFee),
                "CDSC_FEE" => Ok(Self::CdscFee),
                "OPT_REG_FEE" => Ok(Self::OptRegFee),
                "ADDITIONAL_FEE" => Ok(Self::AdditionalFee),
                "MISCELLANEOUS_FEE" => Ok(Self::MiscellaneousFee),
                "FUTURES_EXCHANGE_FEE" => Ok(Self::FuturesExchangeFee),
                "LOW_PROCEEDS_COMMISSION" => Ok(Self::LowProceedsCommission),
                "BASE_CHARGE" => Ok(Self::BaseCharge),
                "GENERAL_CHARGE" => Ok(Self::GeneralCharge),
                "GST_FEE" => Ok(Self::GstFee),
                "TAF_FEE" => Ok(Self::TafFee),
                "INDEX_OPTION_FEE" => Ok(Self::IndexOptionFee),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TransferItemFeeType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TransferItemFeeType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TransferItemFeeType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`TransferItemPositionEffect`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "OPENING",
    ///    "CLOSING",
    ///    "AUTOMATIC",
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
    pub enum TransferItemPositionEffect {
        #[serde(rename = "OPENING")]
        Opening,
        #[serde(rename = "CLOSING")]
        Closing,
        #[serde(rename = "AUTOMATIC")]
        Automatic,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for TransferItemPositionEffect {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Opening => f.write_str("OPENING"),
                Self::Closing => f.write_str("CLOSING"),
                Self::Automatic => f.write_str("AUTOMATIC"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for TransferItemPositionEffect {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "OPENING" => Ok(Self::Opening),
                "CLOSING" => Ok(Self::Closing),
                "AUTOMATIC" => Ok(Self::Automatic),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for TransferItemPositionEffect {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for TransferItemPositionEffect {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for TransferItemPositionEffect {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`UserDetails`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "brokerRepCode": {
    ///      "type": "string"
    ///    },
    ///    "cdDomainId": {
    ///      "type": "string"
    ///    },
    ///    "firstName": {
    ///      "type": "string"
    ///    },
    ///    "lastName": {
    ///      "type": "string"
    ///    },
    ///    "login": {
    ///      "type": "string"
    ///    },
    ///    "systemUserName": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string",
    ///      "enum": [
    ///        "ADVISOR_USER",
    ///        "BROKER_USER",
    ///        "CLIENT_USER",
    ///        "SYSTEM_USER",
    ///        "UNKNOWN"
    ///      ]
    ///    },
    ///    "userId": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserDetails {
        #[serde(
            rename = "brokerRepCode",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub broker_rep_code: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "cdDomainId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub cd_domain_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "firstName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub first_name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "lastName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub last_name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub login: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "systemUserName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub system_user_name: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<UserDetailsType>,
        #[serde(
            rename = "userId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub user_id: ::std::option::Option<i64>,
    }

    impl ::std::default::Default for UserDetails {
        fn default() -> Self {
            Self {
                broker_rep_code: Default::default(),
                cd_domain_id: Default::default(),
                first_name: Default::default(),
                last_name: Default::default(),
                login: Default::default(),
                system_user_name: Default::default(),
                type_: Default::default(),
                user_id: Default::default(),
            }
        }
    }

    impl UserDetails {
        pub fn builder() -> builder::UserDetails {
            Default::default()
        }
    }

    ///`UserDetailsType`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ADVISOR_USER",
    ///    "BROKER_USER",
    ///    "CLIENT_USER",
    ///    "SYSTEM_USER",
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
    pub enum UserDetailsType {
        #[serde(rename = "ADVISOR_USER")]
        AdvisorUser,
        #[serde(rename = "BROKER_USER")]
        BrokerUser,
        #[serde(rename = "CLIENT_USER")]
        ClientUser,
        #[serde(rename = "SYSTEM_USER")]
        SystemUser,
        #[serde(rename = "UNKNOWN")]
        Unknown,
    }

    impl ::std::fmt::Display for UserDetailsType {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::AdvisorUser => f.write_str("ADVISOR_USER"),
                Self::BrokerUser => f.write_str("BROKER_USER"),
                Self::ClientUser => f.write_str("CLIENT_USER"),
                Self::SystemUser => f.write_str("SYSTEM_USER"),
                Self::Unknown => f.write_str("UNKNOWN"),
            }
        }
    }

    impl ::std::str::FromStr for UserDetailsType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "ADVISOR_USER" => Ok(Self::AdvisorUser),
                "BROKER_USER" => Ok(Self::BrokerUser),
                "CLIENT_USER" => Ok(Self::ClientUser),
                "SYSTEM_USER" => Ok(Self::SystemUser),
                "UNKNOWN" => Ok(Self::Unknown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for UserDetailsType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for UserDetailsType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for UserDetailsType {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///`UserPreference`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "accounts": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UserPreferenceAccount"
    ///      }
    ///    },
    ///    "offers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Offer"
    ///      }
    ///    },
    ///    "streamerInfo": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/StreamerInfo"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserPreference {
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub accounts: ::std::vec::Vec<UserPreferenceAccount>,
        #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
        pub offers: ::std::vec::Vec<Offer>,
        #[serde(
            rename = "streamerInfo",
            default,
            skip_serializing_if = "::std::vec::Vec::is_empty"
        )]
        pub streamer_info: ::std::vec::Vec<StreamerInfo>,
    }

    impl ::std::default::Default for UserPreference {
        fn default() -> Self {
            Self {
                accounts: Default::default(),
                offers: Default::default(),
                streamer_info: Default::default(),
            }
        }
    }

    impl UserPreference {
        pub fn builder() -> builder::UserPreference {
            Default::default()
        }
    }

    ///`UserPreferenceAccount`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "accountColor": {
    ///      "description": "Green | Blue",
    ///      "type": "string"
    ///    },
    ///    "accountNumber": {
    ///      "type": "string"
    ///    },
    ///    "autoPositionEffect": {
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "displayAcctId": {
    ///      "type": "string"
    ///    },
    ///    "nickName": {
    ///      "type": "string"
    ///    },
    ///    "primaryAccount": {
    ///      "default": false,
    ///      "type": "boolean"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
    pub struct UserPreferenceAccount {
        ///Green | Blue
        #[serde(
            rename = "accountColor",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub account_color: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "accountNumber",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub account_number: ::std::option::Option<::std::string::String>,
        #[serde(rename = "autoPositionEffect", default)]
        pub auto_position_effect: bool,
        #[serde(
            rename = "displayAcctId",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub display_acct_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "nickName",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub nick_name: ::std::option::Option<::std::string::String>,
        #[serde(rename = "primaryAccount", default)]
        pub primary_account: bool,
        #[serde(
            rename = "type",
            default,
            skip_serializing_if = "::std::option::Option::is_none"
        )]
        pub type_: ::std::option::Option<::std::string::String>,
    }

    impl ::std::default::Default for UserPreferenceAccount {
        fn default() -> Self {
            Self {
                account_color: Default::default(),
                account_number: Default::default(),
                auto_position_effect: Default::default(),
                display_acct_id: Default::default(),
                nick_name: Default::default(),
                primary_account: Default::default(),
                type_: Default::default(),
            }
        }
    }

    impl UserPreferenceAccount {
        pub fn builder() -> builder::UserPreferenceAccount {
            Default::default()
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct Account {
            securities_account: ::std::result::Result<
                ::std::option::Option<super::SecuritiesAccount>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for Account {
            fn default() -> Self {
                Self {
                    securities_account: Ok(Default::default()),
                }
            }
        }

        impl Account {
            pub fn securities_account<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::SecuritiesAccount>>,
                T::Error: ::std::fmt::Display,
            {
                self.securities_account = value.try_into().map_err(|e| {
                    format!("error converting supplied value for securities_account: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<Account> for super::Account {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Account,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    securities_account: value.securities_account?,
                })
            }
        }

        impl ::std::convert::From<super::Account> for Account {
            fn from(value: super::Account) -> Self {
                Self {
                    securities_account: Ok(value.securities_account),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AccountApiOptionDeliverable {
            api_currency_type: ::std::result::Result<
                ::std::option::Option<super::AccountApiOptionDeliverableApiCurrencyType>,
                ::std::string::String,
            >,
            asset_type: ::std::result::Result<
                ::std::option::Option<super::AssetType>,
                ::std::string::String,
            >,
            deliverable_units:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for AccountApiOptionDeliverable {
            fn default() -> Self {
                Self {
                    api_currency_type: Ok(Default::default()),
                    asset_type: Ok(Default::default()),
                    deliverable_units: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                }
            }
        }

        impl AccountApiOptionDeliverable {
            pub fn api_currency_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<super::AccountApiOptionDeliverableApiCurrencyType>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.api_currency_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for api_currency_type: {e}")
                });
                self
            }
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::AssetType>>,
                T::Error: ::std::fmt::Display,
            {
                self.asset_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for asset_type: {e}"));
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

        impl ::std::convert::TryFrom<AccountApiOptionDeliverable> for super::AccountApiOptionDeliverable {
            type Error = super::error::ConversionError;
            fn try_from(
                value: AccountApiOptionDeliverable,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    api_currency_type: value.api_currency_type?,
                    asset_type: value.asset_type?,
                    deliverable_units: value.deliverable_units?,
                    symbol: value.symbol?,
                })
            }
        }

        impl ::std::convert::From<super::AccountApiOptionDeliverable> for AccountApiOptionDeliverable {
            fn from(value: super::AccountApiOptionDeliverable) -> Self {
                Self {
                    api_currency_type: Ok(value.api_currency_type),
                    asset_type: Ok(value.asset_type),
                    deliverable_units: Ok(value.deliverable_units),
                    symbol: Ok(value.symbol),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AccountCashEquivalent {
            asset_type:
                ::std::result::Result<super::AccountCashEquivalentAssetType, ::std::string::String>,
            cusip: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            name: ::std::result::Result<::serde_json::Value, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                ::std::option::Option<super::AccountCashEquivalentType>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for AccountCashEquivalent {
            fn default() -> Self {
                Self {
                    asset_type: Err("no value supplied for asset_type".to_string()),
                    cusip: Ok(Default::default()),
                    description: Ok(Default::default()),
                    instrument_id: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    net_change: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl AccountCashEquivalent {
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::AccountCashEquivalentAssetType>,
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
            pub fn instrument_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.instrument_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
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
                T: ::std::convert::TryInto<::std::option::Option<super::AccountCashEquivalentType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<AccountCashEquivalent> for super::AccountCashEquivalent {
            type Error = super::error::ConversionError;
            fn try_from(
                value: AccountCashEquivalent,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_type: value.asset_type?,
                    cusip: value.cusip?,
                    description: value.description?,
                    instrument_id: value.instrument_id?,
                    name: value.name?,
                    net_change: value.net_change?,
                    symbol: value.symbol?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::AccountCashEquivalent> for AccountCashEquivalent {
            fn from(value: super::AccountCashEquivalent) -> Self {
                Self {
                    asset_type: Ok(value.asset_type),
                    cusip: Ok(value.cusip),
                    description: Ok(value.description),
                    instrument_id: Ok(value.instrument_id),
                    name: Ok(value.name),
                    net_change: Ok(value.net_change),
                    symbol: Ok(value.symbol),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AccountFixedIncome {
            asset_type:
                ::std::result::Result<super::AccountFixedIncomeAssetType, ::std::string::String>,
            cusip: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            factor: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            maturity_date: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            name: ::std::result::Result<::serde_json::Value, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            variable_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for AccountFixedIncome {
            fn default() -> Self {
                Self {
                    asset_type: Err("no value supplied for asset_type".to_string()),
                    cusip: Ok(Default::default()),
                    description: Ok(Default::default()),
                    factor: Ok(Default::default()),
                    instrument_id: Ok(Default::default()),
                    maturity_date: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    net_change: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    variable_rate: Ok(Default::default()),
                }
            }
        }

        impl AccountFixedIncome {
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::AccountFixedIncomeAssetType>,
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
            pub fn factor<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.factor = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for factor: {e}"));
                self
            }
            pub fn instrument_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.instrument_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
                self
            }
            pub fn maturity_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.maturity_date = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for maturity_date: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
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
            pub fn variable_rate<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.variable_rate = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for variable_rate: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<AccountFixedIncome> for super::AccountFixedIncome {
            type Error = super::error::ConversionError;
            fn try_from(
                value: AccountFixedIncome,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_type: value.asset_type?,
                    cusip: value.cusip?,
                    description: value.description?,
                    factor: value.factor?,
                    instrument_id: value.instrument_id?,
                    maturity_date: value.maturity_date?,
                    name: value.name?,
                    net_change: value.net_change?,
                    symbol: value.symbol?,
                    variable_rate: value.variable_rate?,
                })
            }
        }

        impl ::std::convert::From<super::AccountFixedIncome> for AccountFixedIncome {
            fn from(value: super::AccountFixedIncome) -> Self {
                Self {
                    asset_type: Ok(value.asset_type),
                    cusip: Ok(value.cusip),
                    description: Ok(value.description),
                    factor: Ok(value.factor),
                    instrument_id: Ok(value.instrument_id),
                    maturity_date: Ok(value.maturity_date),
                    name: Ok(value.name),
                    net_change: Ok(value.net_change),
                    symbol: Ok(value.symbol),
                    variable_rate: Ok(value.variable_rate),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AccountNumberHash {
            account_number: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            hash_value: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for AccountNumberHash {
            fn default() -> Self {
                Self {
                    account_number: Ok(Default::default()),
                    hash_value: Ok(Default::default()),
                }
            }
        }

        impl AccountNumberHash {
            pub fn account_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.account_number = value.try_into().map_err(|e| {
                    format!("error converting supplied value for account_number: {e}")
                });
                self
            }
            pub fn hash_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.hash_value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for hash_value: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<AccountNumberHash> for super::AccountNumberHash {
            type Error = super::error::ConversionError;
            fn try_from(
                value: AccountNumberHash,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    account_number: value.account_number?,
                    hash_value: value.hash_value?,
                })
            }
        }

        impl ::std::convert::From<super::AccountNumberHash> for AccountNumberHash {
            fn from(value: super::AccountNumberHash) -> Self {
                Self {
                    account_number: Ok(value.account_number),
                    hash_value: Ok(value.hash_value),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AccountOption {
            asset_type: ::std::result::Result<super::AccountOptionAssetType, ::std::string::String>,
            cusip: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            name: ::std::result::Result<::serde_json::Value, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            option_deliverables: ::std::result::Result<
                ::std::vec::Vec<super::AccountApiOptionDeliverable>,
                ::std::string::String,
            >,
            option_multiplier:
                ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            put_call: ::std::result::Result<
                ::std::option::Option<super::AccountOptionPutCall>,
                ::std::string::String,
            >,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                ::std::option::Option<super::AccountOptionType>,
                ::std::string::String,
            >,
            underlying_symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for AccountOption {
            fn default() -> Self {
                Self {
                    asset_type: Err("no value supplied for asset_type".to_string()),
                    cusip: Ok(Default::default()),
                    description: Ok(Default::default()),
                    instrument_id: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    net_change: Ok(Default::default()),
                    option_deliverables: Ok(Default::default()),
                    option_multiplier: Ok(Default::default()),
                    put_call: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    type_: Ok(Default::default()),
                    underlying_symbol: Ok(Default::default()),
                }
            }
        }

        impl AccountOption {
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::AccountOptionAssetType>,
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
            pub fn instrument_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.instrument_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
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
            pub fn option_deliverables<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::AccountApiOptionDeliverable>>,
                T::Error: ::std::fmt::Display,
            {
                self.option_deliverables = value.try_into().map_err(|e| {
                    format!("error converting supplied value for option_deliverables: {e}")
                });
                self
            }
            pub fn option_multiplier<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.option_multiplier = value.try_into().map_err(|e| {
                    format!("error converting supplied value for option_multiplier: {e}")
                });
                self
            }
            pub fn put_call<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::AccountOptionPutCall>>,
                T::Error: ::std::fmt::Display,
            {
                self.put_call = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for put_call: {e}"));
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
                T: ::std::convert::TryInto<::std::option::Option<super::AccountOptionType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
            pub fn underlying_symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.underlying_symbol = value.try_into().map_err(|e| {
                    format!("error converting supplied value for underlying_symbol: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<AccountOption> for super::AccountOption {
            type Error = super::error::ConversionError;
            fn try_from(
                value: AccountOption,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_type: value.asset_type?,
                    cusip: value.cusip?,
                    description: value.description?,
                    instrument_id: value.instrument_id?,
                    name: value.name?,
                    net_change: value.net_change?,
                    option_deliverables: value.option_deliverables?,
                    option_multiplier: value.option_multiplier?,
                    put_call: value.put_call?,
                    symbol: value.symbol?,
                    type_: value.type_?,
                    underlying_symbol: value.underlying_symbol?,
                })
            }
        }

        impl ::std::convert::From<super::AccountOption> for AccountOption {
            fn from(value: super::AccountOption) -> Self {
                Self {
                    asset_type: Ok(value.asset_type),
                    cusip: Ok(value.cusip),
                    description: Ok(value.description),
                    instrument_id: Ok(value.instrument_id),
                    name: Ok(value.name),
                    net_change: Ok(value.net_change),
                    option_deliverables: Ok(value.option_deliverables),
                    option_multiplier: Ok(value.option_multiplier),
                    put_call: Ok(value.put_call),
                    symbol: Ok(value.symbol),
                    type_: Ok(value.type_),
                    underlying_symbol: Ok(value.underlying_symbol),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AccountsBaseInstrument {
            asset_type: ::std::result::Result<
                super::AccountsBaseInstrumentAssetType,
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
            instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            name: ::std::result::Result<::serde_json::Value, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for AccountsBaseInstrument {
            fn default() -> Self {
                Self {
                    asset_type: Err("no value supplied for asset_type".to_string()),
                    cusip: Ok(Default::default()),
                    description: Ok(Default::default()),
                    instrument_id: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    net_change: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                }
            }
        }

        impl AccountsBaseInstrument {
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::AccountsBaseInstrumentAssetType>,
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
            pub fn instrument_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.instrument_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
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

        impl ::std::convert::TryFrom<AccountsBaseInstrument> for super::AccountsBaseInstrument {
            type Error = super::error::ConversionError;
            fn try_from(
                value: AccountsBaseInstrument,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_type: value.asset_type?,
                    cusip: value.cusip?,
                    description: value.description?,
                    instrument_id: value.instrument_id?,
                    name: value.name?,
                    net_change: value.net_change?,
                    symbol: value.symbol?,
                })
            }
        }

        impl ::std::convert::From<super::AccountsBaseInstrument> for AccountsBaseInstrument {
            fn from(value: super::AccountsBaseInstrument) -> Self {
                Self {
                    asset_type: Ok(value.asset_type),
                    cusip: Ok(value.cusip),
                    description: Ok(value.description),
                    instrument_id: Ok(value.instrument_id),
                    name: Ok(value.name),
                    net_change: Ok(value.net_change),
                    symbol: Ok(value.symbol),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CashAccount {
            account_number: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            current_balances: ::std::result::Result<
                ::std::option::Option<super::CashBalance>,
                ::std::string::String,
            >,
            initial_balances: ::std::result::Result<
                ::std::option::Option<super::CashInitialBalance>,
                ::std::string::String,
            >,
            is_closing_only_restricted: ::std::result::Result<bool, ::std::string::String>,
            is_day_trader: ::std::result::Result<bool, ::std::string::String>,
            pfcb_flag: ::std::result::Result<bool, ::std::string::String>,
            positions:
                ::std::result::Result<::std::vec::Vec<super::Position>, ::std::string::String>,
            projected_balances: ::std::result::Result<
                ::std::option::Option<super::CashBalance>,
                ::std::string::String,
            >,
            round_trips: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            type_: ::std::result::Result<
                ::std::option::Option<super::CashAccountType>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for CashAccount {
            fn default() -> Self {
                Self {
                    account_number: Ok(Default::default()),
                    current_balances: Ok(Default::default()),
                    initial_balances: Ok(Default::default()),
                    is_closing_only_restricted: Ok(Default::default()),
                    is_day_trader: Ok(Default::default()),
                    pfcb_flag: Ok(Default::default()),
                    positions: Ok(Default::default()),
                    projected_balances: Ok(Default::default()),
                    round_trips: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl CashAccount {
            pub fn account_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.account_number = value.try_into().map_err(|e| {
                    format!("error converting supplied value for account_number: {e}")
                });
                self
            }
            pub fn current_balances<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::CashBalance>>,
                T::Error: ::std::fmt::Display,
            {
                self.current_balances = value.try_into().map_err(|e| {
                    format!("error converting supplied value for current_balances: {e}")
                });
                self
            }
            pub fn initial_balances<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::CashInitialBalance>>,
                T::Error: ::std::fmt::Display,
            {
                self.initial_balances = value.try_into().map_err(|e| {
                    format!("error converting supplied value for initial_balances: {e}")
                });
                self
            }
            pub fn is_closing_only_restricted<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.is_closing_only_restricted = value.try_into().map_err(|e| {
                    format!("error converting supplied value for is_closing_only_restricted: {e}")
                });
                self
            }
            pub fn is_day_trader<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.is_day_trader = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for is_day_trader: {e}"));
                self
            }
            pub fn pfcb_flag<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.pfcb_flag = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pfcb_flag: {e}"));
                self
            }
            pub fn positions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::Position>>,
                T::Error: ::std::fmt::Display,
            {
                self.positions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for positions: {e}"));
                self
            }
            pub fn projected_balances<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::CashBalance>>,
                T::Error: ::std::fmt::Display,
            {
                self.projected_balances = value.try_into().map_err(|e| {
                    format!("error converting supplied value for projected_balances: {e}")
                });
                self
            }
            pub fn round_trips<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.round_trips = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for round_trips: {e}"));
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::CashAccountType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<CashAccount> for super::CashAccount {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CashAccount,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    account_number: value.account_number?,
                    current_balances: value.current_balances?,
                    initial_balances: value.initial_balances?,
                    is_closing_only_restricted: value.is_closing_only_restricted?,
                    is_day_trader: value.is_day_trader?,
                    pfcb_flag: value.pfcb_flag?,
                    positions: value.positions?,
                    projected_balances: value.projected_balances?,
                    round_trips: value.round_trips?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::CashAccount> for CashAccount {
            fn from(value: super::CashAccount) -> Self {
                Self {
                    account_number: Ok(value.account_number),
                    current_balances: Ok(value.current_balances),
                    initial_balances: Ok(value.initial_balances),
                    is_closing_only_restricted: Ok(value.is_closing_only_restricted),
                    is_day_trader: Ok(value.is_day_trader),
                    pfcb_flag: Ok(value.pfcb_flag),
                    positions: Ok(value.positions),
                    projected_balances: Ok(value.projected_balances),
                    round_trips: Ok(value.round_trips),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CashBalance {
            cash_available_for_trading:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            cash_available_for_withdrawal:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            cash_call: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            cash_debit_call_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            long_non_marginable_market_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            total_cash: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            unsettled_cash:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for CashBalance {
            fn default() -> Self {
                Self {
                    cash_available_for_trading: Ok(Default::default()),
                    cash_available_for_withdrawal: Ok(Default::default()),
                    cash_call: Ok(Default::default()),
                    cash_debit_call_value: Ok(Default::default()),
                    long_non_marginable_market_value: Ok(Default::default()),
                    total_cash: Ok(Default::default()),
                    unsettled_cash: Ok(Default::default()),
                }
            }
        }

        impl CashBalance {
            pub fn cash_available_for_trading<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.cash_available_for_trading = value.try_into().map_err(|e| {
                    format!("error converting supplied value for cash_available_for_trading: {e}")
                });
                self
            }
            pub fn cash_available_for_withdrawal<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.cash_available_for_withdrawal = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for cash_available_for_withdrawal: {e}"
                    )
                });
                self
            }
            pub fn cash_call<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.cash_call = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cash_call: {e}"));
                self
            }
            pub fn cash_debit_call_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.cash_debit_call_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for cash_debit_call_value: {e}")
                });
                self
            }
            pub fn long_non_marginable_market_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.long_non_marginable_market_value = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for long_non_marginable_market_value: {e}"
                    )
                });
                self
            }
            pub fn total_cash<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.total_cash = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_cash: {e}"));
                self
            }
            pub fn unsettled_cash<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.unsettled_cash = value.try_into().map_err(|e| {
                    format!("error converting supplied value for unsettled_cash: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<CashBalance> for super::CashBalance {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CashBalance,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    cash_available_for_trading: value.cash_available_for_trading?,
                    cash_available_for_withdrawal: value.cash_available_for_withdrawal?,
                    cash_call: value.cash_call?,
                    cash_debit_call_value: value.cash_debit_call_value?,
                    long_non_marginable_market_value: value.long_non_marginable_market_value?,
                    total_cash: value.total_cash?,
                    unsettled_cash: value.unsettled_cash?,
                })
            }
        }

        impl ::std::convert::From<super::CashBalance> for CashBalance {
            fn from(value: super::CashBalance) -> Self {
                Self {
                    cash_available_for_trading: Ok(value.cash_available_for_trading),
                    cash_available_for_withdrawal: Ok(value.cash_available_for_withdrawal),
                    cash_call: Ok(value.cash_call),
                    cash_debit_call_value: Ok(value.cash_debit_call_value),
                    long_non_marginable_market_value: Ok(value.long_non_marginable_market_value),
                    total_cash: Ok(value.total_cash),
                    unsettled_cash: Ok(value.unsettled_cash),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CashInitialBalance {
            account_value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            accrued_interest:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            bond_value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            cash_available_for_trading:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            cash_available_for_withdrawal:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            cash_balance: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            cash_debit_call_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            cash_receipts: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            is_in_call: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            liquidation_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            long_option_market_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            long_stock_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            money_market_fund:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mutual_fund_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            pending_deposits:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            short_option_market_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            short_stock_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            unsettled_cash:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for CashInitialBalance {
            fn default() -> Self {
                Self {
                    account_value: Ok(Default::default()),
                    accrued_interest: Ok(Default::default()),
                    bond_value: Ok(Default::default()),
                    cash_available_for_trading: Ok(Default::default()),
                    cash_available_for_withdrawal: Ok(Default::default()),
                    cash_balance: Ok(Default::default()),
                    cash_debit_call_value: Ok(Default::default()),
                    cash_receipts: Ok(Default::default()),
                    is_in_call: Ok(Default::default()),
                    liquidation_value: Ok(Default::default()),
                    long_option_market_value: Ok(Default::default()),
                    long_stock_value: Ok(Default::default()),
                    money_market_fund: Ok(Default::default()),
                    mutual_fund_value: Ok(Default::default()),
                    pending_deposits: Ok(Default::default()),
                    short_option_market_value: Ok(Default::default()),
                    short_stock_value: Ok(Default::default()),
                    unsettled_cash: Ok(Default::default()),
                }
            }
        }

        impl CashInitialBalance {
            pub fn account_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.account_value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for account_value: {e}"));
                self
            }
            pub fn accrued_interest<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.accrued_interest = value.try_into().map_err(|e| {
                    format!("error converting supplied value for accrued_interest: {e}")
                });
                self
            }
            pub fn bond_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.bond_value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bond_value: {e}"));
                self
            }
            pub fn cash_available_for_trading<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.cash_available_for_trading = value.try_into().map_err(|e| {
                    format!("error converting supplied value for cash_available_for_trading: {e}")
                });
                self
            }
            pub fn cash_available_for_withdrawal<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.cash_available_for_withdrawal = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for cash_available_for_withdrawal: {e}"
                    )
                });
                self
            }
            pub fn cash_balance<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.cash_balance = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cash_balance: {e}"));
                self
            }
            pub fn cash_debit_call_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.cash_debit_call_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for cash_debit_call_value: {e}")
                });
                self
            }
            pub fn cash_receipts<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.cash_receipts = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cash_receipts: {e}"));
                self
            }
            pub fn is_in_call<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.is_in_call = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for is_in_call: {e}"));
                self
            }
            pub fn liquidation_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.liquidation_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for liquidation_value: {e}")
                });
                self
            }
            pub fn long_option_market_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.long_option_market_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for long_option_market_value: {e}")
                });
                self
            }
            pub fn long_stock_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.long_stock_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for long_stock_value: {e}")
                });
                self
            }
            pub fn money_market_fund<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.money_market_fund = value.try_into().map_err(|e| {
                    format!("error converting supplied value for money_market_fund: {e}")
                });
                self
            }
            pub fn mutual_fund_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mutual_fund_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for mutual_fund_value: {e}")
                });
                self
            }
            pub fn pending_deposits<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.pending_deposits = value.try_into().map_err(|e| {
                    format!("error converting supplied value for pending_deposits: {e}")
                });
                self
            }
            pub fn short_option_market_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.short_option_market_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for short_option_market_value: {e}")
                });
                self
            }
            pub fn short_stock_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.short_stock_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for short_stock_value: {e}")
                });
                self
            }
            pub fn unsettled_cash<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.unsettled_cash = value.try_into().map_err(|e| {
                    format!("error converting supplied value for unsettled_cash: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<CashInitialBalance> for super::CashInitialBalance {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CashInitialBalance,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    account_value: value.account_value?,
                    accrued_interest: value.accrued_interest?,
                    bond_value: value.bond_value?,
                    cash_available_for_trading: value.cash_available_for_trading?,
                    cash_available_for_withdrawal: value.cash_available_for_withdrawal?,
                    cash_balance: value.cash_balance?,
                    cash_debit_call_value: value.cash_debit_call_value?,
                    cash_receipts: value.cash_receipts?,
                    is_in_call: value.is_in_call?,
                    liquidation_value: value.liquidation_value?,
                    long_option_market_value: value.long_option_market_value?,
                    long_stock_value: value.long_stock_value?,
                    money_market_fund: value.money_market_fund?,
                    mutual_fund_value: value.mutual_fund_value?,
                    pending_deposits: value.pending_deposits?,
                    short_option_market_value: value.short_option_market_value?,
                    short_stock_value: value.short_stock_value?,
                    unsettled_cash: value.unsettled_cash?,
                })
            }
        }

        impl ::std::convert::From<super::CashInitialBalance> for CashInitialBalance {
            fn from(value: super::CashInitialBalance) -> Self {
                Self {
                    account_value: Ok(value.account_value),
                    accrued_interest: Ok(value.accrued_interest),
                    bond_value: Ok(value.bond_value),
                    cash_available_for_trading: Ok(value.cash_available_for_trading),
                    cash_available_for_withdrawal: Ok(value.cash_available_for_withdrawal),
                    cash_balance: Ok(value.cash_balance),
                    cash_debit_call_value: Ok(value.cash_debit_call_value),
                    cash_receipts: Ok(value.cash_receipts),
                    is_in_call: Ok(value.is_in_call),
                    liquidation_value: Ok(value.liquidation_value),
                    long_option_market_value: Ok(value.long_option_market_value),
                    long_stock_value: Ok(value.long_stock_value),
                    money_market_fund: Ok(value.money_market_fund),
                    mutual_fund_value: Ok(value.mutual_fund_value),
                    pending_deposits: Ok(value.pending_deposits),
                    short_option_market_value: Ok(value.short_option_market_value),
                    short_stock_value: Ok(value.short_stock_value),
                    unsettled_cash: Ok(value.unsettled_cash),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CollectiveInvestment {
            asset_type:
                ::std::result::Result<super::CollectiveInvestmentAssetType, ::std::string::String>,
            cusip: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            name: ::std::result::Result<::serde_json::Value, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                ::std::option::Option<super::CollectiveInvestmentType>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for CollectiveInvestment {
            fn default() -> Self {
                Self {
                    asset_type: Err("no value supplied for asset_type".to_string()),
                    cusip: Ok(Default::default()),
                    description: Ok(Default::default()),
                    instrument_id: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    net_change: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl CollectiveInvestment {
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::CollectiveInvestmentAssetType>,
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
            pub fn instrument_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.instrument_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
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
                T: ::std::convert::TryInto<::std::option::Option<super::CollectiveInvestmentType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<CollectiveInvestment> for super::CollectiveInvestment {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CollectiveInvestment,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_type: value.asset_type?,
                    cusip: value.cusip?,
                    description: value.description?,
                    instrument_id: value.instrument_id?,
                    name: value.name?,
                    net_change: value.net_change?,
                    symbol: value.symbol?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::CollectiveInvestment> for CollectiveInvestment {
            fn from(value: super::CollectiveInvestment) -> Self {
                Self {
                    asset_type: Ok(value.asset_type),
                    cusip: Ok(value.cusip),
                    description: Ok(value.description),
                    instrument_id: Ok(value.instrument_id),
                    name: Ok(value.name),
                    net_change: Ok(value.net_change),
                    symbol: Ok(value.symbol),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Commission {
            commission_legs:
                ::std::result::Result<::std::vec::Vec<super::CommissionLeg>, ::std::string::String>,
        }

        impl ::std::default::Default for Commission {
            fn default() -> Self {
                Self {
                    commission_legs: Ok(Default::default()),
                }
            }
        }

        impl Commission {
            pub fn commission_legs<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::CommissionLeg>>,
                T::Error: ::std::fmt::Display,
            {
                self.commission_legs = value.try_into().map_err(|e| {
                    format!("error converting supplied value for commission_legs: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<Commission> for super::Commission {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Commission,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    commission_legs: value.commission_legs?,
                })
            }
        }

        impl ::std::convert::From<super::Commission> for Commission {
            fn from(value: super::Commission) -> Self {
                Self {
                    commission_legs: Ok(value.commission_legs),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CommissionAndFee {
            commission: ::std::result::Result<
                ::std::option::Option<super::Commission>,
                ::std::string::String,
            >,
            fee: ::std::result::Result<::std::option::Option<super::Fees>, ::std::string::String>,
            true_commission: ::std::result::Result<
                ::std::option::Option<super::Commission>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for CommissionAndFee {
            fn default() -> Self {
                Self {
                    commission: Ok(Default::default()),
                    fee: Ok(Default::default()),
                    true_commission: Ok(Default::default()),
                }
            }
        }

        impl CommissionAndFee {
            pub fn commission<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Commission>>,
                T::Error: ::std::fmt::Display,
            {
                self.commission = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for commission: {e}"));
                self
            }
            pub fn fee<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Fees>>,
                T::Error: ::std::fmt::Display,
            {
                self.fee = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fee: {e}"));
                self
            }
            pub fn true_commission<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Commission>>,
                T::Error: ::std::fmt::Display,
            {
                self.true_commission = value.try_into().map_err(|e| {
                    format!("error converting supplied value for true_commission: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<CommissionAndFee> for super::CommissionAndFee {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CommissionAndFee,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    commission: value.commission?,
                    fee: value.fee?,
                    true_commission: value.true_commission?,
                })
            }
        }

        impl ::std::convert::From<super::CommissionAndFee> for CommissionAndFee {
            fn from(value: super::CommissionAndFee) -> Self {
                Self {
                    commission: Ok(value.commission),
                    fee: Ok(value.fee),
                    true_commission: Ok(value.true_commission),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CommissionLeg {
            commission_values: ::std::result::Result<
                ::std::vec::Vec<super::CommissionValue>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for CommissionLeg {
            fn default() -> Self {
                Self {
                    commission_values: Ok(Default::default()),
                }
            }
        }

        impl CommissionLeg {
            pub fn commission_values<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::CommissionValue>>,
                T::Error: ::std::fmt::Display,
            {
                self.commission_values = value.try_into().map_err(|e| {
                    format!("error converting supplied value for commission_values: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<CommissionLeg> for super::CommissionLeg {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CommissionLeg,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    commission_values: value.commission_values?,
                })
            }
        }

        impl ::std::convert::From<super::CommissionLeg> for CommissionLeg {
            fn from(value: super::CommissionLeg) -> Self {
                Self {
                    commission_values: Ok(value.commission_values),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CommissionValue {
            type_:
                ::std::result::Result<::std::option::Option<super::FeeType>, ::std::string::String>,
            value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for CommissionValue {
            fn default() -> Self {
                Self {
                    type_: Ok(Default::default()),
                    value: Ok(Default::default()),
                }
            }
        }

        impl CommissionValue {
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::FeeType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
            pub fn value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for value: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<CommissionValue> for super::CommissionValue {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CommissionValue,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    type_: value.type_?,
                    value: value.value?,
                })
            }
        }

        impl ::std::convert::From<super::CommissionValue> for CommissionValue {
            fn from(value: super::CommissionValue) -> Self {
                Self {
                    type_: Ok(value.type_),
                    value: Ok(value.value),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DateParam {
            date: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for DateParam {
            fn default() -> Self {
                Self {
                    date: Ok(Default::default()),
                }
            }
        }

        impl DateParam {
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
        }

        impl ::std::convert::TryFrom<DateParam> for super::DateParam {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DateParam,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self { date: value.date? })
            }
        }

        impl ::std::convert::From<super::DateParam> for DateParam {
            fn from(value: super::DateParam) -> Self {
                Self {
                    date: Ok(value.date),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ExecutionLeg {
            instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            leg_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            mismarked_quantity:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            quantity: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for ExecutionLeg {
            fn default() -> Self {
                Self {
                    instrument_id: Ok(Default::default()),
                    leg_id: Ok(Default::default()),
                    mismarked_quantity: Ok(Default::default()),
                    price: Ok(Default::default()),
                    quantity: Ok(Default::default()),
                    time: Ok(Default::default()),
                }
            }
        }

        impl ExecutionLeg {
            pub fn instrument_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.instrument_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
                self
            }
            pub fn leg_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.leg_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for leg_id: {e}"));
                self
            }
            pub fn mismarked_quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mismarked_quantity = value.try_into().map_err(|e| {
                    format!("error converting supplied value for mismarked_quantity: {e}")
                });
                self
            }
            pub fn price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for price: {e}"));
                self
            }
            pub fn quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.quantity = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quantity: {e}"));
                self
            }
            pub fn time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for time: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ExecutionLeg> for super::ExecutionLeg {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ExecutionLeg,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    instrument_id: value.instrument_id?,
                    leg_id: value.leg_id?,
                    mismarked_quantity: value.mismarked_quantity?,
                    price: value.price?,
                    quantity: value.quantity?,
                    time: value.time?,
                })
            }
        }

        impl ::std::convert::From<super::ExecutionLeg> for ExecutionLeg {
            fn from(value: super::ExecutionLeg) -> Self {
                Self {
                    instrument_id: Ok(value.instrument_id),
                    leg_id: Ok(value.leg_id),
                    mismarked_quantity: Ok(value.mismarked_quantity),
                    price: Ok(value.price),
                    quantity: Ok(value.quantity),
                    time: Ok(value.time),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FeeLeg {
            fee_values:
                ::std::result::Result<::std::vec::Vec<super::FeeValue>, ::std::string::String>,
        }

        impl ::std::default::Default for FeeLeg {
            fn default() -> Self {
                Self {
                    fee_values: Ok(Default::default()),
                }
            }
        }

        impl FeeLeg {
            pub fn fee_values<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::FeeValue>>,
                T::Error: ::std::fmt::Display,
            {
                self.fee_values = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fee_values: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<FeeLeg> for super::FeeLeg {
            type Error = super::error::ConversionError;
            fn try_from(
                value: FeeLeg,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    fee_values: value.fee_values?,
                })
            }
        }

        impl ::std::convert::From<super::FeeLeg> for FeeLeg {
            fn from(value: super::FeeLeg) -> Self {
                Self {
                    fee_values: Ok(value.fee_values),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FeeValue {
            type_:
                ::std::result::Result<::std::option::Option<super::FeeType>, ::std::string::String>,
            value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for FeeValue {
            fn default() -> Self {
                Self {
                    type_: Ok(Default::default()),
                    value: Ok(Default::default()),
                }
            }
        }

        impl FeeValue {
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::FeeType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
            pub fn value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for value: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<FeeValue> for super::FeeValue {
            type Error = super::error::ConversionError;
            fn try_from(
                value: FeeValue,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    type_: value.type_?,
                    value: value.value?,
                })
            }
        }

        impl ::std::convert::From<super::FeeValue> for FeeValue {
            fn from(value: super::FeeValue) -> Self {
                Self {
                    type_: Ok(value.type_),
                    value: Ok(value.value),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Fees {
            fee_legs: ::std::result::Result<::std::vec::Vec<super::FeeLeg>, ::std::string::String>,
        }

        impl ::std::default::Default for Fees {
            fn default() -> Self {
                Self {
                    fee_legs: Ok(Default::default()),
                }
            }
        }

        impl Fees {
            pub fn fee_legs<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::FeeLeg>>,
                T::Error: ::std::fmt::Display,
            {
                self.fee_legs = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fee_legs: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Fees> for super::Fees {
            type Error = super::error::ConversionError;
            fn try_from(value: Fees) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    fee_legs: value.fee_legs?,
                })
            }
        }

        impl ::std::convert::From<super::Fees> for Fees {
            fn from(value: super::Fees) -> Self {
                Self {
                    fee_legs: Ok(value.fee_legs),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Forex {
            asset_type: ::std::result::Result<super::ForexAssetType, ::std::string::String>,
            base_currency: ::std::result::Result<
                ::std::option::Option<super::Currency>,
                ::std::string::String,
            >,
            counter_currency: ::std::result::Result<
                ::std::option::Option<super::Currency>,
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
            instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            name: ::std::result::Result<::serde_json::Value, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                ::std::option::Option<super::ForexType>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for Forex {
            fn default() -> Self {
                Self {
                    asset_type: Err("no value supplied for asset_type".to_string()),
                    base_currency: Ok(Default::default()),
                    counter_currency: Ok(Default::default()),
                    cusip: Ok(Default::default()),
                    description: Ok(Default::default()),
                    instrument_id: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    net_change: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl Forex {
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::ForexAssetType>,
                T::Error: ::std::fmt::Display,
            {
                self.asset_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for asset_type: {e}"));
                self
            }
            pub fn base_currency<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Currency>>,
                T::Error: ::std::fmt::Display,
            {
                self.base_currency = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for base_currency: {e}"));
                self
            }
            pub fn counter_currency<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Currency>>,
                T::Error: ::std::fmt::Display,
            {
                self.counter_currency = value.try_into().map_err(|e| {
                    format!("error converting supplied value for counter_currency: {e}")
                });
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
            pub fn instrument_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.instrument_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
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
                T: ::std::convert::TryInto<::std::option::Option<super::ForexType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Forex> for super::Forex {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Forex,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_type: value.asset_type?,
                    base_currency: value.base_currency?,
                    counter_currency: value.counter_currency?,
                    cusip: value.cusip?,
                    description: value.description?,
                    instrument_id: value.instrument_id?,
                    name: value.name?,
                    net_change: value.net_change?,
                    symbol: value.symbol?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::Forex> for Forex {
            fn from(value: super::Forex) -> Self {
                Self {
                    asset_type: Ok(value.asset_type),
                    base_currency: Ok(value.base_currency),
                    counter_currency: Ok(value.counter_currency),
                    cusip: Ok(value.cusip),
                    description: Ok(value.description),
                    instrument_id: Ok(value.instrument_id),
                    name: Ok(value.name),
                    net_change: Ok(value.net_change),
                    symbol: Ok(value.symbol),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Future {
            active_contract: ::std::result::Result<bool, ::std::string::String>,
            expiration_date: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            first_notice_date: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            last_trading_date: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            multiplier: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            type_: ::std::result::Result<
                ::std::option::Option<super::FutureType>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for Future {
            fn default() -> Self {
                Self {
                    active_contract: Ok(Default::default()),
                    expiration_date: Ok(Default::default()),
                    first_notice_date: Ok(Default::default()),
                    last_trading_date: Ok(Default::default()),
                    multiplier: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl Future {
            pub fn active_contract<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.active_contract = value.try_into().map_err(|e| {
                    format!("error converting supplied value for active_contract: {e}")
                });
                self
            }
            pub fn expiration_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.expiration_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for expiration_date: {e}")
                });
                self
            }
            pub fn first_notice_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.first_notice_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for first_notice_date: {e}")
                });
                self
            }
            pub fn last_trading_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.last_trading_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for last_trading_date: {e}")
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
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::FutureType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Future> for super::Future {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Future,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    active_contract: value.active_contract?,
                    expiration_date: value.expiration_date?,
                    first_notice_date: value.first_notice_date?,
                    last_trading_date: value.last_trading_date?,
                    multiplier: value.multiplier?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::Future> for Future {
            fn from(value: super::Future) -> Self {
                Self {
                    active_contract: Ok(value.active_contract),
                    expiration_date: Ok(value.expiration_date),
                    first_notice_date: Ok(value.first_notice_date),
                    last_trading_date: Ok(value.last_trading_date),
                    multiplier: Ok(value.multiplier),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Index {
            active_contract: ::std::result::Result<bool, ::std::string::String>,
            type_: ::std::result::Result<
                ::std::option::Option<super::IndexType>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for Index {
            fn default() -> Self {
                Self {
                    active_contract: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl Index {
            pub fn active_contract<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.active_contract = value.try_into().map_err(|e| {
                    format!("error converting supplied value for active_contract: {e}")
                });
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::IndexType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Index> for super::Index {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Index,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    active_contract: value.active_contract?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::Index> for Index {
            fn from(value: super::Index) -> Self {
                Self {
                    active_contract: Ok(value.active_contract),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MarginAccount {
            account_number: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            current_balances: ::std::result::Result<
                ::std::option::Option<super::MarginBalance>,
                ::std::string::String,
            >,
            initial_balances: ::std::result::Result<
                ::std::option::Option<super::MarginInitialBalance>,
                ::std::string::String,
            >,
            is_closing_only_restricted: ::std::result::Result<bool, ::std::string::String>,
            is_day_trader: ::std::result::Result<bool, ::std::string::String>,
            pfcb_flag: ::std::result::Result<bool, ::std::string::String>,
            positions:
                ::std::result::Result<::std::vec::Vec<super::Position>, ::std::string::String>,
            projected_balances: ::std::result::Result<
                ::std::option::Option<super::MarginBalance>,
                ::std::string::String,
            >,
            round_trips: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            type_: ::std::result::Result<
                ::std::option::Option<super::MarginAccountType>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for MarginAccount {
            fn default() -> Self {
                Self {
                    account_number: Ok(Default::default()),
                    current_balances: Ok(Default::default()),
                    initial_balances: Ok(Default::default()),
                    is_closing_only_restricted: Ok(Default::default()),
                    is_day_trader: Ok(Default::default()),
                    pfcb_flag: Ok(Default::default()),
                    positions: Ok(Default::default()),
                    projected_balances: Ok(Default::default()),
                    round_trips: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl MarginAccount {
            pub fn account_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.account_number = value.try_into().map_err(|e| {
                    format!("error converting supplied value for account_number: {e}")
                });
                self
            }
            pub fn current_balances<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::MarginBalance>>,
                T::Error: ::std::fmt::Display,
            {
                self.current_balances = value.try_into().map_err(|e| {
                    format!("error converting supplied value for current_balances: {e}")
                });
                self
            }
            pub fn initial_balances<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::MarginInitialBalance>>,
                T::Error: ::std::fmt::Display,
            {
                self.initial_balances = value.try_into().map_err(|e| {
                    format!("error converting supplied value for initial_balances: {e}")
                });
                self
            }
            pub fn is_closing_only_restricted<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.is_closing_only_restricted = value.try_into().map_err(|e| {
                    format!("error converting supplied value for is_closing_only_restricted: {e}")
                });
                self
            }
            pub fn is_day_trader<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.is_day_trader = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for is_day_trader: {e}"));
                self
            }
            pub fn pfcb_flag<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.pfcb_flag = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pfcb_flag: {e}"));
                self
            }
            pub fn positions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::Position>>,
                T::Error: ::std::fmt::Display,
            {
                self.positions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for positions: {e}"));
                self
            }
            pub fn projected_balances<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::MarginBalance>>,
                T::Error: ::std::fmt::Display,
            {
                self.projected_balances = value.try_into().map_err(|e| {
                    format!("error converting supplied value for projected_balances: {e}")
                });
                self
            }
            pub fn round_trips<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.round_trips = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for round_trips: {e}"));
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::MarginAccountType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<MarginAccount> for super::MarginAccount {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MarginAccount,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    account_number: value.account_number?,
                    current_balances: value.current_balances?,
                    initial_balances: value.initial_balances?,
                    is_closing_only_restricted: value.is_closing_only_restricted?,
                    is_day_trader: value.is_day_trader?,
                    pfcb_flag: value.pfcb_flag?,
                    positions: value.positions?,
                    projected_balances: value.projected_balances?,
                    round_trips: value.round_trips?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::MarginAccount> for MarginAccount {
            fn from(value: super::MarginAccount) -> Self {
                Self {
                    account_number: Ok(value.account_number),
                    current_balances: Ok(value.current_balances),
                    initial_balances: Ok(value.initial_balances),
                    is_closing_only_restricted: Ok(value.is_closing_only_restricted),
                    is_day_trader: Ok(value.is_day_trader),
                    pfcb_flag: Ok(value.pfcb_flag),
                    positions: Ok(value.positions),
                    projected_balances: Ok(value.projected_balances),
                    round_trips: Ok(value.round_trips),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MarginBalance {
            available_funds:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            available_funds_non_marginable_trade:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            buying_power: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            buying_power_non_marginable_trade:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            cash_balance: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            day_trading_buying_power:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            day_trading_buying_power_call:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            liquidation_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            equity: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            equity_percentage:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            is_in_call: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            long_margin_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            maintenance_call:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            maintenance_requirement:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            margin_balance:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            option_buying_power:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            reg_t_call: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            short_balance: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            short_margin_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            sma: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            stock_buying_power:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for MarginBalance {
            fn default() -> Self {
                Self {
                    available_funds: Ok(Default::default()),
                    available_funds_non_marginable_trade: Ok(Default::default()),
                    buying_power: Ok(Default::default()),
                    buying_power_non_marginable_trade: Ok(Default::default()),
                    cash_balance: Ok(Default::default()),
                    day_trading_buying_power: Ok(Default::default()),
                    day_trading_buying_power_call: Ok(Default::default()),
                    liquidation_value: Ok(Default::default()),
                    equity: Ok(Default::default()),
                    equity_percentage: Ok(Default::default()),
                    is_in_call: Ok(Default::default()),
                    long_margin_value: Ok(Default::default()),
                    maintenance_call: Ok(Default::default()),
                    maintenance_requirement: Ok(Default::default()),
                    margin_balance: Ok(Default::default()),
                    option_buying_power: Ok(Default::default()),
                    reg_t_call: Ok(Default::default()),
                    short_balance: Ok(Default::default()),
                    short_margin_value: Ok(Default::default()),
                    sma: Ok(Default::default()),
                    stock_buying_power: Ok(Default::default()),
                }
            }
        }

        impl MarginBalance {
            pub fn available_funds<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.available_funds = value.try_into().map_err(|e| {
                    format!("error converting supplied value for available_funds: {e}")
                });
                self
            }
            pub fn available_funds_non_marginable_trade<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self . available_funds_non_marginable_trade = value . try_into () . map_err (| e | format ! ("error converting supplied value for available_funds_non_marginable_trade: {e}")) ;
                self
            }
            pub fn buying_power<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.buying_power = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for buying_power: {e}"));
                self
            }
            pub fn buying_power_non_marginable_trade<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.buying_power_non_marginable_trade = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for buying_power_non_marginable_trade: {e}"
                    )
                });
                self
            }
            pub fn cash_balance<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.cash_balance = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cash_balance: {e}"));
                self
            }
            pub fn day_trading_buying_power<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.day_trading_buying_power = value.try_into().map_err(|e| {
                    format!("error converting supplied value for day_trading_buying_power: {e}")
                });
                self
            }
            pub fn day_trading_buying_power_call<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.day_trading_buying_power_call = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for day_trading_buying_power_call: {e}"
                    )
                });
                self
            }
            pub fn liquidation_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.liquidation_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for liquidation_value: {e}")
                });
                self
            }
            pub fn equity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.equity = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for equity: {e}"));
                self
            }
            pub fn equity_percentage<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.equity_percentage = value.try_into().map_err(|e| {
                    format!("error converting supplied value for equity_percentage: {e}")
                });
                self
            }
            pub fn is_in_call<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.is_in_call = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for is_in_call: {e}"));
                self
            }
            pub fn long_margin_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.long_margin_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for long_margin_value: {e}")
                });
                self
            }
            pub fn maintenance_call<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.maintenance_call = value.try_into().map_err(|e| {
                    format!("error converting supplied value for maintenance_call: {e}")
                });
                self
            }
            pub fn maintenance_requirement<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.maintenance_requirement = value.try_into().map_err(|e| {
                    format!("error converting supplied value for maintenance_requirement: {e}")
                });
                self
            }
            pub fn margin_balance<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.margin_balance = value.try_into().map_err(|e| {
                    format!("error converting supplied value for margin_balance: {e}")
                });
                self
            }
            pub fn option_buying_power<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.option_buying_power = value.try_into().map_err(|e| {
                    format!("error converting supplied value for option_buying_power: {e}")
                });
                self
            }
            pub fn reg_t_call<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.reg_t_call = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for reg_t_call: {e}"));
                self
            }
            pub fn short_balance<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.short_balance = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for short_balance: {e}"));
                self
            }
            pub fn short_margin_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.short_margin_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for short_margin_value: {e}")
                });
                self
            }
            pub fn sma<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.sma = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sma: {e}"));
                self
            }
            pub fn stock_buying_power<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.stock_buying_power = value.try_into().map_err(|e| {
                    format!("error converting supplied value for stock_buying_power: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<MarginBalance> for super::MarginBalance {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MarginBalance,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    available_funds: value.available_funds?,
                    available_funds_non_marginable_trade: value
                        .available_funds_non_marginable_trade?,
                    buying_power: value.buying_power?,
                    buying_power_non_marginable_trade: value.buying_power_non_marginable_trade?,
                    cash_balance: value.cash_balance?,
                    day_trading_buying_power: value.day_trading_buying_power?,
                    day_trading_buying_power_call: value.day_trading_buying_power_call?,
                    liquidation_value: value.liquidation_value?,
                    equity: value.equity?,
                    equity_percentage: value.equity_percentage?,
                    is_in_call: value.is_in_call?,
                    long_margin_value: value.long_margin_value?,
                    maintenance_call: value.maintenance_call?,
                    maintenance_requirement: value.maintenance_requirement?,
                    margin_balance: value.margin_balance?,
                    option_buying_power: value.option_buying_power?,
                    reg_t_call: value.reg_t_call?,
                    short_balance: value.short_balance?,
                    short_margin_value: value.short_margin_value?,
                    sma: value.sma?,
                    stock_buying_power: value.stock_buying_power?,
                })
            }
        }

        impl ::std::convert::From<super::MarginBalance> for MarginBalance {
            fn from(value: super::MarginBalance) -> Self {
                Self {
                    available_funds: Ok(value.available_funds),
                    available_funds_non_marginable_trade: Ok(
                        value.available_funds_non_marginable_trade
                    ),
                    buying_power: Ok(value.buying_power),
                    buying_power_non_marginable_trade: Ok(value.buying_power_non_marginable_trade),
                    cash_balance: Ok(value.cash_balance),
                    day_trading_buying_power: Ok(value.day_trading_buying_power),
                    day_trading_buying_power_call: Ok(value.day_trading_buying_power_call),
                    liquidation_value: Ok(value.liquidation_value),
                    equity: Ok(value.equity),
                    equity_percentage: Ok(value.equity_percentage),
                    is_in_call: Ok(value.is_in_call),
                    long_margin_value: Ok(value.long_margin_value),
                    maintenance_call: Ok(value.maintenance_call),
                    maintenance_requirement: Ok(value.maintenance_requirement),
                    margin_balance: Ok(value.margin_balance),
                    option_buying_power: Ok(value.option_buying_power),
                    reg_t_call: Ok(value.reg_t_call),
                    short_balance: Ok(value.short_balance),
                    short_margin_value: Ok(value.short_margin_value),
                    sma: Ok(value.sma),
                    stock_buying_power: Ok(value.stock_buying_power),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MarginInitialBalance {
            account_value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            accrued_interest:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            available_funds_non_marginable_trade:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            bond_value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            buying_power: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            cash_available_for_trading:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            cash_balance: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            cash_receipts: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            day_trading_buying_power:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            day_trading_buying_power_call:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            day_trading_equity_call:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            equity: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            equity_percentage:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            is_in_call: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            liquidation_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            long_margin_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            long_option_market_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            long_stock_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            maintenance_call:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            maintenance_requirement:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            margin: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            margin_balance:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            margin_equity: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            money_market_fund:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mutual_fund_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            pending_deposits:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            reg_t_call: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            short_balance: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            short_margin_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            short_option_market_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            short_stock_value:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            total_cash: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            unsettled_cash:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for MarginInitialBalance {
            fn default() -> Self {
                Self {
                    account_value: Ok(Default::default()),
                    accrued_interest: Ok(Default::default()),
                    available_funds_non_marginable_trade: Ok(Default::default()),
                    bond_value: Ok(Default::default()),
                    buying_power: Ok(Default::default()),
                    cash_available_for_trading: Ok(Default::default()),
                    cash_balance: Ok(Default::default()),
                    cash_receipts: Ok(Default::default()),
                    day_trading_buying_power: Ok(Default::default()),
                    day_trading_buying_power_call: Ok(Default::default()),
                    day_trading_equity_call: Ok(Default::default()),
                    equity: Ok(Default::default()),
                    equity_percentage: Ok(Default::default()),
                    is_in_call: Ok(Default::default()),
                    liquidation_value: Ok(Default::default()),
                    long_margin_value: Ok(Default::default()),
                    long_option_market_value: Ok(Default::default()),
                    long_stock_value: Ok(Default::default()),
                    maintenance_call: Ok(Default::default()),
                    maintenance_requirement: Ok(Default::default()),
                    margin: Ok(Default::default()),
                    margin_balance: Ok(Default::default()),
                    margin_equity: Ok(Default::default()),
                    money_market_fund: Ok(Default::default()),
                    mutual_fund_value: Ok(Default::default()),
                    pending_deposits: Ok(Default::default()),
                    reg_t_call: Ok(Default::default()),
                    short_balance: Ok(Default::default()),
                    short_margin_value: Ok(Default::default()),
                    short_option_market_value: Ok(Default::default()),
                    short_stock_value: Ok(Default::default()),
                    total_cash: Ok(Default::default()),
                    unsettled_cash: Ok(Default::default()),
                }
            }
        }

        impl MarginInitialBalance {
            pub fn account_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.account_value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for account_value: {e}"));
                self
            }
            pub fn accrued_interest<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.accrued_interest = value.try_into().map_err(|e| {
                    format!("error converting supplied value for accrued_interest: {e}")
                });
                self
            }
            pub fn available_funds_non_marginable_trade<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self . available_funds_non_marginable_trade = value . try_into () . map_err (| e | format ! ("error converting supplied value for available_funds_non_marginable_trade: {e}")) ;
                self
            }
            pub fn bond_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.bond_value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for bond_value: {e}"));
                self
            }
            pub fn buying_power<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.buying_power = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for buying_power: {e}"));
                self
            }
            pub fn cash_available_for_trading<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.cash_available_for_trading = value.try_into().map_err(|e| {
                    format!("error converting supplied value for cash_available_for_trading: {e}")
                });
                self
            }
            pub fn cash_balance<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.cash_balance = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cash_balance: {e}"));
                self
            }
            pub fn cash_receipts<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.cash_receipts = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cash_receipts: {e}"));
                self
            }
            pub fn day_trading_buying_power<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.day_trading_buying_power = value.try_into().map_err(|e| {
                    format!("error converting supplied value for day_trading_buying_power: {e}")
                });
                self
            }
            pub fn day_trading_buying_power_call<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.day_trading_buying_power_call = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for day_trading_buying_power_call: {e}"
                    )
                });
                self
            }
            pub fn day_trading_equity_call<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.day_trading_equity_call = value.try_into().map_err(|e| {
                    format!("error converting supplied value for day_trading_equity_call: {e}")
                });
                self
            }
            pub fn equity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.equity = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for equity: {e}"));
                self
            }
            pub fn equity_percentage<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.equity_percentage = value.try_into().map_err(|e| {
                    format!("error converting supplied value for equity_percentage: {e}")
                });
                self
            }
            pub fn is_in_call<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.is_in_call = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for is_in_call: {e}"));
                self
            }
            pub fn liquidation_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.liquidation_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for liquidation_value: {e}")
                });
                self
            }
            pub fn long_margin_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.long_margin_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for long_margin_value: {e}")
                });
                self
            }
            pub fn long_option_market_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.long_option_market_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for long_option_market_value: {e}")
                });
                self
            }
            pub fn long_stock_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.long_stock_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for long_stock_value: {e}")
                });
                self
            }
            pub fn maintenance_call<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.maintenance_call = value.try_into().map_err(|e| {
                    format!("error converting supplied value for maintenance_call: {e}")
                });
                self
            }
            pub fn maintenance_requirement<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.maintenance_requirement = value.try_into().map_err(|e| {
                    format!("error converting supplied value for maintenance_requirement: {e}")
                });
                self
            }
            pub fn margin<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.margin = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for margin: {e}"));
                self
            }
            pub fn margin_balance<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.margin_balance = value.try_into().map_err(|e| {
                    format!("error converting supplied value for margin_balance: {e}")
                });
                self
            }
            pub fn margin_equity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.margin_equity = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for margin_equity: {e}"));
                self
            }
            pub fn money_market_fund<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.money_market_fund = value.try_into().map_err(|e| {
                    format!("error converting supplied value for money_market_fund: {e}")
                });
                self
            }
            pub fn mutual_fund_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.mutual_fund_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for mutual_fund_value: {e}")
                });
                self
            }
            pub fn pending_deposits<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.pending_deposits = value.try_into().map_err(|e| {
                    format!("error converting supplied value for pending_deposits: {e}")
                });
                self
            }
            pub fn reg_t_call<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.reg_t_call = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for reg_t_call: {e}"));
                self
            }
            pub fn short_balance<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.short_balance = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for short_balance: {e}"));
                self
            }
            pub fn short_margin_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.short_margin_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for short_margin_value: {e}")
                });
                self
            }
            pub fn short_option_market_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.short_option_market_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for short_option_market_value: {e}")
                });
                self
            }
            pub fn short_stock_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.short_stock_value = value.try_into().map_err(|e| {
                    format!("error converting supplied value for short_stock_value: {e}")
                });
                self
            }
            pub fn total_cash<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.total_cash = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for total_cash: {e}"));
                self
            }
            pub fn unsettled_cash<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.unsettled_cash = value.try_into().map_err(|e| {
                    format!("error converting supplied value for unsettled_cash: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<MarginInitialBalance> for super::MarginInitialBalance {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MarginInitialBalance,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    account_value: value.account_value?,
                    accrued_interest: value.accrued_interest?,
                    available_funds_non_marginable_trade: value
                        .available_funds_non_marginable_trade?,
                    bond_value: value.bond_value?,
                    buying_power: value.buying_power?,
                    cash_available_for_trading: value.cash_available_for_trading?,
                    cash_balance: value.cash_balance?,
                    cash_receipts: value.cash_receipts?,
                    day_trading_buying_power: value.day_trading_buying_power?,
                    day_trading_buying_power_call: value.day_trading_buying_power_call?,
                    day_trading_equity_call: value.day_trading_equity_call?,
                    equity: value.equity?,
                    equity_percentage: value.equity_percentage?,
                    is_in_call: value.is_in_call?,
                    liquidation_value: value.liquidation_value?,
                    long_margin_value: value.long_margin_value?,
                    long_option_market_value: value.long_option_market_value?,
                    long_stock_value: value.long_stock_value?,
                    maintenance_call: value.maintenance_call?,
                    maintenance_requirement: value.maintenance_requirement?,
                    margin: value.margin?,
                    margin_balance: value.margin_balance?,
                    margin_equity: value.margin_equity?,
                    money_market_fund: value.money_market_fund?,
                    mutual_fund_value: value.mutual_fund_value?,
                    pending_deposits: value.pending_deposits?,
                    reg_t_call: value.reg_t_call?,
                    short_balance: value.short_balance?,
                    short_margin_value: value.short_margin_value?,
                    short_option_market_value: value.short_option_market_value?,
                    short_stock_value: value.short_stock_value?,
                    total_cash: value.total_cash?,
                    unsettled_cash: value.unsettled_cash?,
                })
            }
        }

        impl ::std::convert::From<super::MarginInitialBalance> for MarginInitialBalance {
            fn from(value: super::MarginInitialBalance) -> Self {
                Self {
                    account_value: Ok(value.account_value),
                    accrued_interest: Ok(value.accrued_interest),
                    available_funds_non_marginable_trade: Ok(
                        value.available_funds_non_marginable_trade
                    ),
                    bond_value: Ok(value.bond_value),
                    buying_power: Ok(value.buying_power),
                    cash_available_for_trading: Ok(value.cash_available_for_trading),
                    cash_balance: Ok(value.cash_balance),
                    cash_receipts: Ok(value.cash_receipts),
                    day_trading_buying_power: Ok(value.day_trading_buying_power),
                    day_trading_buying_power_call: Ok(value.day_trading_buying_power_call),
                    day_trading_equity_call: Ok(value.day_trading_equity_call),
                    equity: Ok(value.equity),
                    equity_percentage: Ok(value.equity_percentage),
                    is_in_call: Ok(value.is_in_call),
                    liquidation_value: Ok(value.liquidation_value),
                    long_margin_value: Ok(value.long_margin_value),
                    long_option_market_value: Ok(value.long_option_market_value),
                    long_stock_value: Ok(value.long_stock_value),
                    maintenance_call: Ok(value.maintenance_call),
                    maintenance_requirement: Ok(value.maintenance_requirement),
                    margin: Ok(value.margin),
                    margin_balance: Ok(value.margin_balance),
                    margin_equity: Ok(value.margin_equity),
                    money_market_fund: Ok(value.money_market_fund),
                    mutual_fund_value: Ok(value.mutual_fund_value),
                    pending_deposits: Ok(value.pending_deposits),
                    reg_t_call: Ok(value.reg_t_call),
                    short_balance: Ok(value.short_balance),
                    short_margin_value: Ok(value.short_margin_value),
                    short_option_market_value: Ok(value.short_option_market_value),
                    short_stock_value: Ok(value.short_stock_value),
                    total_cash: Ok(value.total_cash),
                    unsettled_cash: Ok(value.unsettled_cash),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Offer {
            level2_permissions: ::std::result::Result<bool, ::std::string::String>,
            mkt_data_permission: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for Offer {
            fn default() -> Self {
                Self {
                    level2_permissions: Ok(Default::default()),
                    mkt_data_permission: Ok(Default::default()),
                }
            }
        }

        impl Offer {
            pub fn level2_permissions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.level2_permissions = value.try_into().map_err(|e| {
                    format!("error converting supplied value for level2_permissions: {e}")
                });
                self
            }
            pub fn mkt_data_permission<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.mkt_data_permission = value.try_into().map_err(|e| {
                    format!("error converting supplied value for mkt_data_permission: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<Offer> for super::Offer {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Offer,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    level2_permissions: value.level2_permissions?,
                    mkt_data_permission: value.mkt_data_permission?,
                })
            }
        }

        impl ::std::convert::From<super::Offer> for Offer {
            fn from(value: super::Offer) -> Self {
                Self {
                    level2_permissions: Ok(value.level2_permissions),
                    mkt_data_permission: Ok(value.mkt_data_permission),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Order {
            account_number:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            activation_price:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            cancel_time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            cancelable: ::std::result::Result<bool, ::std::string::String>,
            child_order_strategies:
                ::std::result::Result<::std::vec::Vec<super::Order>, ::std::string::String>,
            close_time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            complex_order_strategy_type: ::std::result::Result<
                ::std::option::Option<super::ComplexOrderStrategyType>,
                ::std::string::String,
            >,
            destination_link_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            duration: ::std::result::Result<
                ::std::option::Option<super::Duration>,
                ::std::string::String,
            >,
            editable: ::std::result::Result<bool, ::std::string::String>,
            entered_time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            filled_quantity:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            order_activity_collection:
                ::std::result::Result<::std::vec::Vec<super::OrderActivity>, ::std::string::String>,
            order_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            order_leg_collection: ::std::result::Result<
                ::std::vec::Vec<super::OrderLegCollection>,
                ::std::string::String,
            >,
            order_strategy_type: ::std::result::Result<
                ::std::option::Option<super::OrderStrategyType>,
                ::std::string::String,
            >,
            order_type: ::std::result::Result<
                ::std::option::Option<super::OrderType>,
                ::std::string::String,
            >,
            price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            price_link_basis: ::std::result::Result<
                ::std::option::Option<super::PriceLinkBasis>,
                ::std::string::String,
            >,
            price_link_type: ::std::result::Result<
                ::std::option::Option<super::PriceLinkType>,
                ::std::string::String,
            >,
            quantity: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            release_time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            remaining_quantity:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            replacing_order_collection:
                ::std::result::Result<::std::vec::Vec<super::Order>, ::std::string::String>,
            requested_destination: ::std::result::Result<
                ::std::option::Option<super::RequestedDestination>,
                ::std::string::String,
            >,
            session:
                ::std::result::Result<::std::option::Option<super::Session>, ::std::string::String>,
            special_instruction: ::std::result::Result<
                ::std::option::Option<super::SpecialInstruction>,
                ::std::string::String,
            >,
            status:
                ::std::result::Result<::std::option::Option<super::Status>, ::std::string::String>,
            status_description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            stop_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            stop_price_link_basis: ::std::result::Result<
                ::std::option::Option<super::StopPriceLinkBasis>,
                ::std::string::String,
            >,
            stop_price_link_type: ::std::result::Result<
                ::std::option::Option<super::StopPriceLinkType>,
                ::std::string::String,
            >,
            stop_price_offset:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            stop_type: ::std::result::Result<
                ::std::option::Option<super::StopType>,
                ::std::string::String,
            >,
            tag: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            tax_lot_method: ::std::result::Result<
                ::std::option::Option<super::TaxLotMethod>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for Order {
            fn default() -> Self {
                Self {
                    account_number: Ok(Default::default()),
                    activation_price: Ok(Default::default()),
                    cancel_time: Ok(Default::default()),
                    cancelable: Ok(Default::default()),
                    child_order_strategies: Ok(Default::default()),
                    close_time: Ok(Default::default()),
                    complex_order_strategy_type: Ok(Default::default()),
                    destination_link_name: Ok(Default::default()),
                    duration: Ok(Default::default()),
                    editable: Ok(Default::default()),
                    entered_time: Ok(Default::default()),
                    filled_quantity: Ok(Default::default()),
                    order_activity_collection: Ok(Default::default()),
                    order_id: Ok(Default::default()),
                    order_leg_collection: Ok(Default::default()),
                    order_strategy_type: Ok(Default::default()),
                    order_type: Ok(Default::default()),
                    price: Ok(Default::default()),
                    price_link_basis: Ok(Default::default()),
                    price_link_type: Ok(Default::default()),
                    quantity: Ok(Default::default()),
                    release_time: Ok(Default::default()),
                    remaining_quantity: Ok(Default::default()),
                    replacing_order_collection: Ok(Default::default()),
                    requested_destination: Ok(Default::default()),
                    session: Ok(Default::default()),
                    special_instruction: Ok(Default::default()),
                    status: Ok(Default::default()),
                    status_description: Ok(Default::default()),
                    stop_price: Ok(Default::default()),
                    stop_price_link_basis: Ok(Default::default()),
                    stop_price_link_type: Ok(Default::default()),
                    stop_price_offset: Ok(Default::default()),
                    stop_type: Ok(Default::default()),
                    tag: Ok(Default::default()),
                    tax_lot_method: Ok(Default::default()),
                }
            }
        }

        impl Order {
            pub fn account_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.account_number = value.try_into().map_err(|e| {
                    format!("error converting supplied value for account_number: {e}")
                });
                self
            }
            pub fn activation_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.activation_price = value.try_into().map_err(|e| {
                    format!("error converting supplied value for activation_price: {e}")
                });
                self
            }
            pub fn cancel_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.cancel_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cancel_time: {e}"));
                self
            }
            pub fn cancelable<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.cancelable = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cancelable: {e}"));
                self
            }
            pub fn child_order_strategies<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::Order>>,
                T::Error: ::std::fmt::Display,
            {
                self.child_order_strategies = value.try_into().map_err(|e| {
                    format!("error converting supplied value for child_order_strategies: {e}")
                });
                self
            }
            pub fn close_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.close_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for close_time: {e}"));
                self
            }
            pub fn complex_order_strategy_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ComplexOrderStrategyType>>,
                T::Error: ::std::fmt::Display,
            {
                self.complex_order_strategy_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for complex_order_strategy_type: {e}")
                });
                self
            }
            pub fn destination_link_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.destination_link_name = value.try_into().map_err(|e| {
                    format!("error converting supplied value for destination_link_name: {e}")
                });
                self
            }
            pub fn duration<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Duration>>,
                T::Error: ::std::fmt::Display,
            {
                self.duration = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for duration: {e}"));
                self
            }
            pub fn editable<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.editable = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for editable: {e}"));
                self
            }
            pub fn entered_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.entered_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for entered_time: {e}"));
                self
            }
            pub fn filled_quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.filled_quantity = value.try_into().map_err(|e| {
                    format!("error converting supplied value for filled_quantity: {e}")
                });
                self
            }
            pub fn order_activity_collection<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::OrderActivity>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_activity_collection = value.try_into().map_err(|e| {
                    format!("error converting supplied value for order_activity_collection: {e}")
                });
                self
            }
            pub fn order_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for order_id: {e}"));
                self
            }
            pub fn order_leg_collection<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::OrderLegCollection>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_leg_collection = value.try_into().map_err(|e| {
                    format!("error converting supplied value for order_leg_collection: {e}")
                });
                self
            }
            pub fn order_strategy_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::OrderStrategyType>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_strategy_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for order_strategy_type: {e}")
                });
                self
            }
            pub fn order_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::OrderType>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for order_type: {e}"));
                self
            }
            pub fn price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for price: {e}"));
                self
            }
            pub fn price_link_basis<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::PriceLinkBasis>>,
                T::Error: ::std::fmt::Display,
            {
                self.price_link_basis = value.try_into().map_err(|e| {
                    format!("error converting supplied value for price_link_basis: {e}")
                });
                self
            }
            pub fn price_link_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::PriceLinkType>>,
                T::Error: ::std::fmt::Display,
            {
                self.price_link_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for price_link_type: {e}")
                });
                self
            }
            pub fn quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.quantity = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quantity: {e}"));
                self
            }
            pub fn release_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.release_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for release_time: {e}"));
                self
            }
            pub fn remaining_quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.remaining_quantity = value.try_into().map_err(|e| {
                    format!("error converting supplied value for remaining_quantity: {e}")
                });
                self
            }
            pub fn replacing_order_collection<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::Order>>,
                T::Error: ::std::fmt::Display,
            {
                self.replacing_order_collection = value.try_into().map_err(|e| {
                    format!("error converting supplied value for replacing_order_collection: {e}")
                });
                self
            }
            pub fn requested_destination<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::RequestedDestination>>,
                T::Error: ::std::fmt::Display,
            {
                self.requested_destination = value.try_into().map_err(|e| {
                    format!("error converting supplied value for requested_destination: {e}")
                });
                self
            }
            pub fn session<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Session>>,
                T::Error: ::std::fmt::Display,
            {
                self.session = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for session: {e}"));
                self
            }
            pub fn special_instruction<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::SpecialInstruction>>,
                T::Error: ::std::fmt::Display,
            {
                self.special_instruction = value.try_into().map_err(|e| {
                    format!("error converting supplied value for special_instruction: {e}")
                });
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Status>>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {e}"));
                self
            }
            pub fn status_description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.status_description = value.try_into().map_err(|e| {
                    format!("error converting supplied value for status_description: {e}")
                });
                self
            }
            pub fn stop_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.stop_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for stop_price: {e}"));
                self
            }
            pub fn stop_price_link_basis<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::StopPriceLinkBasis>>,
                T::Error: ::std::fmt::Display,
            {
                self.stop_price_link_basis = value.try_into().map_err(|e| {
                    format!("error converting supplied value for stop_price_link_basis: {e}")
                });
                self
            }
            pub fn stop_price_link_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::StopPriceLinkType>>,
                T::Error: ::std::fmt::Display,
            {
                self.stop_price_link_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for stop_price_link_type: {e}")
                });
                self
            }
            pub fn stop_price_offset<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.stop_price_offset = value.try_into().map_err(|e| {
                    format!("error converting supplied value for stop_price_offset: {e}")
                });
                self
            }
            pub fn stop_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::StopType>>,
                T::Error: ::std::fmt::Display,
            {
                self.stop_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for stop_type: {e}"));
                self
            }
            pub fn tag<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.tag = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tag: {e}"));
                self
            }
            pub fn tax_lot_method<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::TaxLotMethod>>,
                T::Error: ::std::fmt::Display,
            {
                self.tax_lot_method = value.try_into().map_err(|e| {
                    format!("error converting supplied value for tax_lot_method: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<Order> for super::Order {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Order,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    account_number: value.account_number?,
                    activation_price: value.activation_price?,
                    cancel_time: value.cancel_time?,
                    cancelable: value.cancelable?,
                    child_order_strategies: value.child_order_strategies?,
                    close_time: value.close_time?,
                    complex_order_strategy_type: value.complex_order_strategy_type?,
                    destination_link_name: value.destination_link_name?,
                    duration: value.duration?,
                    editable: value.editable?,
                    entered_time: value.entered_time?,
                    filled_quantity: value.filled_quantity?,
                    order_activity_collection: value.order_activity_collection?,
                    order_id: value.order_id?,
                    order_leg_collection: value.order_leg_collection?,
                    order_strategy_type: value.order_strategy_type?,
                    order_type: value.order_type?,
                    price: value.price?,
                    price_link_basis: value.price_link_basis?,
                    price_link_type: value.price_link_type?,
                    quantity: value.quantity?,
                    release_time: value.release_time?,
                    remaining_quantity: value.remaining_quantity?,
                    replacing_order_collection: value.replacing_order_collection?,
                    requested_destination: value.requested_destination?,
                    session: value.session?,
                    special_instruction: value.special_instruction?,
                    status: value.status?,
                    status_description: value.status_description?,
                    stop_price: value.stop_price?,
                    stop_price_link_basis: value.stop_price_link_basis?,
                    stop_price_link_type: value.stop_price_link_type?,
                    stop_price_offset: value.stop_price_offset?,
                    stop_type: value.stop_type?,
                    tag: value.tag?,
                    tax_lot_method: value.tax_lot_method?,
                })
            }
        }

        impl ::std::convert::From<super::Order> for Order {
            fn from(value: super::Order) -> Self {
                Self {
                    account_number: Ok(value.account_number),
                    activation_price: Ok(value.activation_price),
                    cancel_time: Ok(value.cancel_time),
                    cancelable: Ok(value.cancelable),
                    child_order_strategies: Ok(value.child_order_strategies),
                    close_time: Ok(value.close_time),
                    complex_order_strategy_type: Ok(value.complex_order_strategy_type),
                    destination_link_name: Ok(value.destination_link_name),
                    duration: Ok(value.duration),
                    editable: Ok(value.editable),
                    entered_time: Ok(value.entered_time),
                    filled_quantity: Ok(value.filled_quantity),
                    order_activity_collection: Ok(value.order_activity_collection),
                    order_id: Ok(value.order_id),
                    order_leg_collection: Ok(value.order_leg_collection),
                    order_strategy_type: Ok(value.order_strategy_type),
                    order_type: Ok(value.order_type),
                    price: Ok(value.price),
                    price_link_basis: Ok(value.price_link_basis),
                    price_link_type: Ok(value.price_link_type),
                    quantity: Ok(value.quantity),
                    release_time: Ok(value.release_time),
                    remaining_quantity: Ok(value.remaining_quantity),
                    replacing_order_collection: Ok(value.replacing_order_collection),
                    requested_destination: Ok(value.requested_destination),
                    session: Ok(value.session),
                    special_instruction: Ok(value.special_instruction),
                    status: Ok(value.status),
                    status_description: Ok(value.status_description),
                    stop_price: Ok(value.stop_price),
                    stop_price_link_basis: Ok(value.stop_price_link_basis),
                    stop_price_link_type: Ok(value.stop_price_link_type),
                    stop_price_offset: Ok(value.stop_price_offset),
                    stop_type: Ok(value.stop_type),
                    tag: Ok(value.tag),
                    tax_lot_method: Ok(value.tax_lot_method),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OrderActivity {
            activity_type: ::std::result::Result<
                ::std::option::Option<super::OrderActivityActivityType>,
                ::std::string::String,
            >,
            execution_legs:
                ::std::result::Result<::std::vec::Vec<super::ExecutionLeg>, ::std::string::String>,
            execution_type: ::std::result::Result<
                ::std::option::Option<super::OrderActivityExecutionType>,
                ::std::string::String,
            >,
            order_remaining_quantity:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            quantity: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for OrderActivity {
            fn default() -> Self {
                Self {
                    activity_type: Ok(Default::default()),
                    execution_legs: Ok(Default::default()),
                    execution_type: Ok(Default::default()),
                    order_remaining_quantity: Ok(Default::default()),
                    quantity: Ok(Default::default()),
                }
            }
        }

        impl OrderActivity {
            pub fn activity_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::OrderActivityActivityType>>,
                T::Error: ::std::fmt::Display,
            {
                self.activity_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for activity_type: {e}"));
                self
            }
            pub fn execution_legs<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::ExecutionLeg>>,
                T::Error: ::std::fmt::Display,
            {
                self.execution_legs = value.try_into().map_err(|e| {
                    format!("error converting supplied value for execution_legs: {e}")
                });
                self
            }
            pub fn execution_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<super::OrderActivityExecutionType>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.execution_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for execution_type: {e}")
                });
                self
            }
            pub fn order_remaining_quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_remaining_quantity = value.try_into().map_err(|e| {
                    format!("error converting supplied value for order_remaining_quantity: {e}")
                });
                self
            }
            pub fn quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.quantity = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quantity: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<OrderActivity> for super::OrderActivity {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OrderActivity,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    activity_type: value.activity_type?,
                    execution_legs: value.execution_legs?,
                    execution_type: value.execution_type?,
                    order_remaining_quantity: value.order_remaining_quantity?,
                    quantity: value.quantity?,
                })
            }
        }

        impl ::std::convert::From<super::OrderActivity> for OrderActivity {
            fn from(value: super::OrderActivity) -> Self {
                Self {
                    activity_type: Ok(value.activity_type),
                    execution_legs: Ok(value.execution_legs),
                    execution_type: Ok(value.execution_type),
                    order_remaining_quantity: Ok(value.order_remaining_quantity),
                    quantity: Ok(value.quantity),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OrderBalance {
            order_value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            projected_available_fund:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            projected_buying_power:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            projected_commission:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for OrderBalance {
            fn default() -> Self {
                Self {
                    order_value: Ok(Default::default()),
                    projected_available_fund: Ok(Default::default()),
                    projected_buying_power: Ok(Default::default()),
                    projected_commission: Ok(Default::default()),
                }
            }
        }

        impl OrderBalance {
            pub fn order_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for order_value: {e}"));
                self
            }
            pub fn projected_available_fund<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.projected_available_fund = value.try_into().map_err(|e| {
                    format!("error converting supplied value for projected_available_fund: {e}")
                });
                self
            }
            pub fn projected_buying_power<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.projected_buying_power = value.try_into().map_err(|e| {
                    format!("error converting supplied value for projected_buying_power: {e}")
                });
                self
            }
            pub fn projected_commission<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.projected_commission = value.try_into().map_err(|e| {
                    format!("error converting supplied value for projected_commission: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<OrderBalance> for super::OrderBalance {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OrderBalance,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    order_value: value.order_value?,
                    projected_available_fund: value.projected_available_fund?,
                    projected_buying_power: value.projected_buying_power?,
                    projected_commission: value.projected_commission?,
                })
            }
        }

        impl ::std::convert::From<super::OrderBalance> for OrderBalance {
            fn from(value: super::OrderBalance) -> Self {
                Self {
                    order_value: Ok(value.order_value),
                    projected_available_fund: Ok(value.projected_available_fund),
                    projected_buying_power: Ok(value.projected_buying_power),
                    projected_commission: Ok(value.projected_commission),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OrderLeg {
            ask_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            asset_type: ::std::result::Result<
                ::std::option::Option<super::AssetType>,
                ::std::string::String,
            >,
            bid_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            final_symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            instruction: ::std::result::Result<
                ::std::option::Option<super::Instruction>,
                ::std::string::String,
            >,
            last_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            leg_id: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            mark_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            projected_commission:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            quantity: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for OrderLeg {
            fn default() -> Self {
                Self {
                    ask_price: Ok(Default::default()),
                    asset_type: Ok(Default::default()),
                    bid_price: Ok(Default::default()),
                    final_symbol: Ok(Default::default()),
                    instruction: Ok(Default::default()),
                    last_price: Ok(Default::default()),
                    leg_id: Ok(Default::default()),
                    mark_price: Ok(Default::default()),
                    projected_commission: Ok(Default::default()),
                    quantity: Ok(Default::default()),
                }
            }
        }

        impl OrderLeg {
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
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::AssetType>>,
                T::Error: ::std::fmt::Display,
            {
                self.asset_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for asset_type: {e}"));
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
            pub fn final_symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.final_symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for final_symbol: {e}"));
                self
            }
            pub fn instruction<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Instruction>>,
                T::Error: ::std::fmt::Display,
            {
                self.instruction = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instruction: {e}"));
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
            pub fn leg_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.leg_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for leg_id: {e}"));
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
            pub fn projected_commission<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.projected_commission = value.try_into().map_err(|e| {
                    format!("error converting supplied value for projected_commission: {e}")
                });
                self
            }
            pub fn quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.quantity = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quantity: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<OrderLeg> for super::OrderLeg {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OrderLeg,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    ask_price: value.ask_price?,
                    asset_type: value.asset_type?,
                    bid_price: value.bid_price?,
                    final_symbol: value.final_symbol?,
                    instruction: value.instruction?,
                    last_price: value.last_price?,
                    leg_id: value.leg_id?,
                    mark_price: value.mark_price?,
                    projected_commission: value.projected_commission?,
                    quantity: value.quantity?,
                })
            }
        }

        impl ::std::convert::From<super::OrderLeg> for OrderLeg {
            fn from(value: super::OrderLeg) -> Self {
                Self {
                    ask_price: Ok(value.ask_price),
                    asset_type: Ok(value.asset_type),
                    bid_price: Ok(value.bid_price),
                    final_symbol: Ok(value.final_symbol),
                    instruction: Ok(value.instruction),
                    last_price: Ok(value.last_price),
                    leg_id: Ok(value.leg_id),
                    mark_price: Ok(value.mark_price),
                    projected_commission: Ok(value.projected_commission),
                    quantity: Ok(value.quantity),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OrderLegCollection {
            div_cap_gains: ::std::result::Result<
                ::std::option::Option<super::OrderLegCollectionDivCapGains>,
                ::std::string::String,
            >,
            instruction: ::std::result::Result<
                ::std::option::Option<super::Instruction>,
                ::std::string::String,
            >,
            instrument: ::std::result::Result<
                ::std::option::Option<super::AccountsInstrument>,
                ::std::string::String,
            >,
            leg_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            order_leg_type: ::std::result::Result<
                ::std::option::Option<super::OrderLegCollectionOrderLegType>,
                ::std::string::String,
            >,
            position_effect: ::std::result::Result<
                ::std::option::Option<super::OrderLegCollectionPositionEffect>,
                ::std::string::String,
            >,
            quantity: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            quantity_type: ::std::result::Result<
                ::std::option::Option<super::OrderLegCollectionQuantityType>,
                ::std::string::String,
            >,
            to_symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for OrderLegCollection {
            fn default() -> Self {
                Self {
                    div_cap_gains: Ok(Default::default()),
                    instruction: Ok(Default::default()),
                    instrument: Ok(Default::default()),
                    leg_id: Ok(Default::default()),
                    order_leg_type: Ok(Default::default()),
                    position_effect: Ok(Default::default()),
                    quantity: Ok(Default::default()),
                    quantity_type: Ok(Default::default()),
                    to_symbol: Ok(Default::default()),
                }
            }
        }

        impl OrderLegCollection {
            pub fn div_cap_gains<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<super::OrderLegCollectionDivCapGains>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.div_cap_gains = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for div_cap_gains: {e}"));
                self
            }
            pub fn instruction<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Instruction>>,
                T::Error: ::std::fmt::Display,
            {
                self.instruction = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instruction: {e}"));
                self
            }
            pub fn instrument<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::AccountsInstrument>>,
                T::Error: ::std::fmt::Display,
            {
                self.instrument = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instrument: {e}"));
                self
            }
            pub fn leg_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.leg_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for leg_id: {e}"));
                self
            }
            pub fn order_leg_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<super::OrderLegCollectionOrderLegType>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.order_leg_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for order_leg_type: {e}")
                });
                self
            }
            pub fn position_effect<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<super::OrderLegCollectionPositionEffect>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.position_effect = value.try_into().map_err(|e| {
                    format!("error converting supplied value for position_effect: {e}")
                });
                self
            }
            pub fn quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.quantity = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quantity: {e}"));
                self
            }
            pub fn quantity_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<super::OrderLegCollectionQuantityType>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.quantity_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quantity_type: {e}"));
                self
            }
            pub fn to_symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.to_symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for to_symbol: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<OrderLegCollection> for super::OrderLegCollection {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OrderLegCollection,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    div_cap_gains: value.div_cap_gains?,
                    instruction: value.instruction?,
                    instrument: value.instrument?,
                    leg_id: value.leg_id?,
                    order_leg_type: value.order_leg_type?,
                    position_effect: value.position_effect?,
                    quantity: value.quantity?,
                    quantity_type: value.quantity_type?,
                    to_symbol: value.to_symbol?,
                })
            }
        }

        impl ::std::convert::From<super::OrderLegCollection> for OrderLegCollection {
            fn from(value: super::OrderLegCollection) -> Self {
                Self {
                    div_cap_gains: Ok(value.div_cap_gains),
                    instruction: Ok(value.instruction),
                    instrument: Ok(value.instrument),
                    leg_id: Ok(value.leg_id),
                    order_leg_type: Ok(value.order_leg_type),
                    position_effect: Ok(value.position_effect),
                    quantity: Ok(value.quantity),
                    quantity_type: Ok(value.quantity_type),
                    to_symbol: Ok(value.to_symbol),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OrderRequest {
            account_number:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            activation_price:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            cancel_time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            cancelable: ::std::result::Result<bool, ::std::string::String>,
            child_order_strategies:
                ::std::result::Result<::std::vec::Vec<super::OrderRequest>, ::std::string::String>,
            close_time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            complex_order_strategy_type: ::std::result::Result<
                ::std::option::Option<super::ComplexOrderStrategyType>,
                ::std::string::String,
            >,
            destination_link_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            duration: ::std::result::Result<
                ::std::option::Option<super::Duration>,
                ::std::string::String,
            >,
            editable: ::std::result::Result<bool, ::std::string::String>,
            entered_time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            filled_quantity:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            order_activity_collection:
                ::std::result::Result<::std::vec::Vec<super::OrderActivity>, ::std::string::String>,
            order_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            order_leg_collection: ::std::result::Result<
                ::std::vec::Vec<super::OrderLegCollection>,
                ::std::string::String,
            >,
            order_strategy_type: ::std::result::Result<
                ::std::option::Option<super::OrderStrategyType>,
                ::std::string::String,
            >,
            order_type: ::std::result::Result<
                ::std::option::Option<super::OrderTypeRequest>,
                ::std::string::String,
            >,
            price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            price_link_basis: ::std::result::Result<
                ::std::option::Option<super::PriceLinkBasis>,
                ::std::string::String,
            >,
            price_link_type: ::std::result::Result<
                ::std::option::Option<super::PriceLinkType>,
                ::std::string::String,
            >,
            quantity: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            release_time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            remaining_quantity:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            replacing_order_collection:
                ::std::result::Result<::std::vec::Vec<super::OrderRequest>, ::std::string::String>,
            session:
                ::std::result::Result<::std::option::Option<super::Session>, ::std::string::String>,
            special_instruction: ::std::result::Result<
                ::std::option::Option<super::SpecialInstruction>,
                ::std::string::String,
            >,
            status:
                ::std::result::Result<::std::option::Option<super::Status>, ::std::string::String>,
            status_description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            stop_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            stop_price_link_basis: ::std::result::Result<
                ::std::option::Option<super::StopPriceLinkBasis>,
                ::std::string::String,
            >,
            stop_price_link_type: ::std::result::Result<
                ::std::option::Option<super::StopPriceLinkType>,
                ::std::string::String,
            >,
            stop_price_offset:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            stop_type: ::std::result::Result<
                ::std::option::Option<super::StopType>,
                ::std::string::String,
            >,
            tax_lot_method: ::std::result::Result<
                ::std::option::Option<super::TaxLotMethod>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for OrderRequest {
            fn default() -> Self {
                Self {
                    account_number: Ok(Default::default()),
                    activation_price: Ok(Default::default()),
                    cancel_time: Ok(Default::default()),
                    cancelable: Ok(Default::default()),
                    child_order_strategies: Ok(Default::default()),
                    close_time: Ok(Default::default()),
                    complex_order_strategy_type: Ok(Default::default()),
                    destination_link_name: Ok(Default::default()),
                    duration: Ok(Default::default()),
                    editable: Ok(Default::default()),
                    entered_time: Ok(Default::default()),
                    filled_quantity: Ok(Default::default()),
                    order_activity_collection: Ok(Default::default()),
                    order_id: Ok(Default::default()),
                    order_leg_collection: Ok(Default::default()),
                    order_strategy_type: Ok(Default::default()),
                    order_type: Ok(Default::default()),
                    price: Ok(Default::default()),
                    price_link_basis: Ok(Default::default()),
                    price_link_type: Ok(Default::default()),
                    quantity: Ok(Default::default()),
                    release_time: Ok(Default::default()),
                    remaining_quantity: Ok(Default::default()),
                    replacing_order_collection: Ok(Default::default()),
                    session: Ok(Default::default()),
                    special_instruction: Ok(Default::default()),
                    status: Ok(Default::default()),
                    status_description: Ok(Default::default()),
                    stop_price: Ok(Default::default()),
                    stop_price_link_basis: Ok(Default::default()),
                    stop_price_link_type: Ok(Default::default()),
                    stop_price_offset: Ok(Default::default()),
                    stop_type: Ok(Default::default()),
                    tax_lot_method: Ok(Default::default()),
                }
            }
        }

        impl OrderRequest {
            pub fn account_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.account_number = value.try_into().map_err(|e| {
                    format!("error converting supplied value for account_number: {e}")
                });
                self
            }
            pub fn activation_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.activation_price = value.try_into().map_err(|e| {
                    format!("error converting supplied value for activation_price: {e}")
                });
                self
            }
            pub fn cancel_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.cancel_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cancel_time: {e}"));
                self
            }
            pub fn cancelable<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.cancelable = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cancelable: {e}"));
                self
            }
            pub fn child_order_strategies<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::OrderRequest>>,
                T::Error: ::std::fmt::Display,
            {
                self.child_order_strategies = value.try_into().map_err(|e| {
                    format!("error converting supplied value for child_order_strategies: {e}")
                });
                self
            }
            pub fn close_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.close_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for close_time: {e}"));
                self
            }
            pub fn complex_order_strategy_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ComplexOrderStrategyType>>,
                T::Error: ::std::fmt::Display,
            {
                self.complex_order_strategy_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for complex_order_strategy_type: {e}")
                });
                self
            }
            pub fn destination_link_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.destination_link_name = value.try_into().map_err(|e| {
                    format!("error converting supplied value for destination_link_name: {e}")
                });
                self
            }
            pub fn duration<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Duration>>,
                T::Error: ::std::fmt::Display,
            {
                self.duration = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for duration: {e}"));
                self
            }
            pub fn editable<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.editable = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for editable: {e}"));
                self
            }
            pub fn entered_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.entered_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for entered_time: {e}"));
                self
            }
            pub fn filled_quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.filled_quantity = value.try_into().map_err(|e| {
                    format!("error converting supplied value for filled_quantity: {e}")
                });
                self
            }
            pub fn order_activity_collection<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::OrderActivity>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_activity_collection = value.try_into().map_err(|e| {
                    format!("error converting supplied value for order_activity_collection: {e}")
                });
                self
            }
            pub fn order_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for order_id: {e}"));
                self
            }
            pub fn order_leg_collection<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::OrderLegCollection>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_leg_collection = value.try_into().map_err(|e| {
                    format!("error converting supplied value for order_leg_collection: {e}")
                });
                self
            }
            pub fn order_strategy_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::OrderStrategyType>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_strategy_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for order_strategy_type: {e}")
                });
                self
            }
            pub fn order_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::OrderTypeRequest>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for order_type: {e}"));
                self
            }
            pub fn price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for price: {e}"));
                self
            }
            pub fn price_link_basis<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::PriceLinkBasis>>,
                T::Error: ::std::fmt::Display,
            {
                self.price_link_basis = value.try_into().map_err(|e| {
                    format!("error converting supplied value for price_link_basis: {e}")
                });
                self
            }
            pub fn price_link_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::PriceLinkType>>,
                T::Error: ::std::fmt::Display,
            {
                self.price_link_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for price_link_type: {e}")
                });
                self
            }
            pub fn quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.quantity = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quantity: {e}"));
                self
            }
            pub fn release_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.release_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for release_time: {e}"));
                self
            }
            pub fn remaining_quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.remaining_quantity = value.try_into().map_err(|e| {
                    format!("error converting supplied value for remaining_quantity: {e}")
                });
                self
            }
            pub fn replacing_order_collection<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::OrderRequest>>,
                T::Error: ::std::fmt::Display,
            {
                self.replacing_order_collection = value.try_into().map_err(|e| {
                    format!("error converting supplied value for replacing_order_collection: {e}")
                });
                self
            }
            pub fn session<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Session>>,
                T::Error: ::std::fmt::Display,
            {
                self.session = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for session: {e}"));
                self
            }
            pub fn special_instruction<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::SpecialInstruction>>,
                T::Error: ::std::fmt::Display,
            {
                self.special_instruction = value.try_into().map_err(|e| {
                    format!("error converting supplied value for special_instruction: {e}")
                });
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Status>>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {e}"));
                self
            }
            pub fn status_description<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.status_description = value.try_into().map_err(|e| {
                    format!("error converting supplied value for status_description: {e}")
                });
                self
            }
            pub fn stop_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.stop_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for stop_price: {e}"));
                self
            }
            pub fn stop_price_link_basis<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::StopPriceLinkBasis>>,
                T::Error: ::std::fmt::Display,
            {
                self.stop_price_link_basis = value.try_into().map_err(|e| {
                    format!("error converting supplied value for stop_price_link_basis: {e}")
                });
                self
            }
            pub fn stop_price_link_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::StopPriceLinkType>>,
                T::Error: ::std::fmt::Display,
            {
                self.stop_price_link_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for stop_price_link_type: {e}")
                });
                self
            }
            pub fn stop_price_offset<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.stop_price_offset = value.try_into().map_err(|e| {
                    format!("error converting supplied value for stop_price_offset: {e}")
                });
                self
            }
            pub fn stop_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::StopType>>,
                T::Error: ::std::fmt::Display,
            {
                self.stop_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for stop_type: {e}"));
                self
            }
            pub fn tax_lot_method<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::TaxLotMethod>>,
                T::Error: ::std::fmt::Display,
            {
                self.tax_lot_method = value.try_into().map_err(|e| {
                    format!("error converting supplied value for tax_lot_method: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<OrderRequest> for super::OrderRequest {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OrderRequest,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    account_number: value.account_number?,
                    activation_price: value.activation_price?,
                    cancel_time: value.cancel_time?,
                    cancelable: value.cancelable?,
                    child_order_strategies: value.child_order_strategies?,
                    close_time: value.close_time?,
                    complex_order_strategy_type: value.complex_order_strategy_type?,
                    destination_link_name: value.destination_link_name?,
                    duration: value.duration?,
                    editable: value.editable?,
                    entered_time: value.entered_time?,
                    filled_quantity: value.filled_quantity?,
                    order_activity_collection: value.order_activity_collection?,
                    order_id: value.order_id?,
                    order_leg_collection: value.order_leg_collection?,
                    order_strategy_type: value.order_strategy_type?,
                    order_type: value.order_type?,
                    price: value.price?,
                    price_link_basis: value.price_link_basis?,
                    price_link_type: value.price_link_type?,
                    quantity: value.quantity?,
                    release_time: value.release_time?,
                    remaining_quantity: value.remaining_quantity?,
                    replacing_order_collection: value.replacing_order_collection?,
                    session: value.session?,
                    special_instruction: value.special_instruction?,
                    status: value.status?,
                    status_description: value.status_description?,
                    stop_price: value.stop_price?,
                    stop_price_link_basis: value.stop_price_link_basis?,
                    stop_price_link_type: value.stop_price_link_type?,
                    stop_price_offset: value.stop_price_offset?,
                    stop_type: value.stop_type?,
                    tax_lot_method: value.tax_lot_method?,
                })
            }
        }

        impl ::std::convert::From<super::OrderRequest> for OrderRequest {
            fn from(value: super::OrderRequest) -> Self {
                Self {
                    account_number: Ok(value.account_number),
                    activation_price: Ok(value.activation_price),
                    cancel_time: Ok(value.cancel_time),
                    cancelable: Ok(value.cancelable),
                    child_order_strategies: Ok(value.child_order_strategies),
                    close_time: Ok(value.close_time),
                    complex_order_strategy_type: Ok(value.complex_order_strategy_type),
                    destination_link_name: Ok(value.destination_link_name),
                    duration: Ok(value.duration),
                    editable: Ok(value.editable),
                    entered_time: Ok(value.entered_time),
                    filled_quantity: Ok(value.filled_quantity),
                    order_activity_collection: Ok(value.order_activity_collection),
                    order_id: Ok(value.order_id),
                    order_leg_collection: Ok(value.order_leg_collection),
                    order_strategy_type: Ok(value.order_strategy_type),
                    order_type: Ok(value.order_type),
                    price: Ok(value.price),
                    price_link_basis: Ok(value.price_link_basis),
                    price_link_type: Ok(value.price_link_type),
                    quantity: Ok(value.quantity),
                    release_time: Ok(value.release_time),
                    remaining_quantity: Ok(value.remaining_quantity),
                    replacing_order_collection: Ok(value.replacing_order_collection),
                    session: Ok(value.session),
                    special_instruction: Ok(value.special_instruction),
                    status: Ok(value.status),
                    status_description: Ok(value.status_description),
                    stop_price: Ok(value.stop_price),
                    stop_price_link_basis: Ok(value.stop_price_link_basis),
                    stop_price_link_type: Ok(value.stop_price_link_type),
                    stop_price_offset: Ok(value.stop_price_offset),
                    stop_type: Ok(value.stop_type),
                    tax_lot_method: Ok(value.tax_lot_method),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OrderStrategy {
            account_number: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            advanced_order_type: ::std::result::Result<
                ::std::option::Option<super::OrderStrategyAdvancedOrderType>,
                ::std::string::String,
            >,
            all_or_none: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            amount_indicator: ::std::result::Result<
                ::std::option::Option<super::AmountIndicator>,
                ::std::string::String,
            >,
            close_time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            discretionary:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            duration: ::std::result::Result<
                ::std::option::Option<super::Duration>,
                ::std::string::String,
            >,
            entered_time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            filled_quantity:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            order_balance: ::std::result::Result<
                ::std::option::Option<super::OrderBalance>,
                ::std::string::String,
            >,
            order_legs:
                ::std::result::Result<::std::vec::Vec<super::OrderLeg>, ::std::string::String>,
            order_strategy_type: ::std::result::Result<
                ::std::option::Option<super::OrderStrategyType>,
                ::std::string::String,
            >,
            order_type: ::std::result::Result<
                ::std::option::Option<super::OrderType>,
                ::std::string::String,
            >,
            order_value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            order_version: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            quantity: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            remaining_quantity:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            sell_non_marginable_first:
                ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
            session:
                ::std::result::Result<::std::option::Option<super::Session>, ::std::string::String>,
            settlement_instruction: ::std::result::Result<
                ::std::option::Option<super::SettlementInstruction>,
                ::std::string::String,
            >,
            status: ::std::result::Result<
                ::std::option::Option<super::ApiOrderStatus>,
                ::std::string::String,
            >,
            strategy: ::std::result::Result<
                ::std::option::Option<super::ComplexOrderStrategyType>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for OrderStrategy {
            fn default() -> Self {
                Self {
                    account_number: Ok(Default::default()),
                    advanced_order_type: Ok(Default::default()),
                    all_or_none: Ok(Default::default()),
                    amount_indicator: Ok(Default::default()),
                    close_time: Ok(Default::default()),
                    discretionary: Ok(Default::default()),
                    duration: Ok(Default::default()),
                    entered_time: Ok(Default::default()),
                    filled_quantity: Ok(Default::default()),
                    order_balance: Ok(Default::default()),
                    order_legs: Ok(Default::default()),
                    order_strategy_type: Ok(Default::default()),
                    order_type: Ok(Default::default()),
                    order_value: Ok(Default::default()),
                    order_version: Ok(Default::default()),
                    price: Ok(Default::default()),
                    quantity: Ok(Default::default()),
                    remaining_quantity: Ok(Default::default()),
                    sell_non_marginable_first: Ok(Default::default()),
                    session: Ok(Default::default()),
                    settlement_instruction: Ok(Default::default()),
                    status: Ok(Default::default()),
                    strategy: Ok(Default::default()),
                }
            }
        }

        impl OrderStrategy {
            pub fn account_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.account_number = value.try_into().map_err(|e| {
                    format!("error converting supplied value for account_number: {e}")
                });
                self
            }
            pub fn advanced_order_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<super::OrderStrategyAdvancedOrderType>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.advanced_order_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for advanced_order_type: {e}")
                });
                self
            }
            pub fn all_or_none<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.all_or_none = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for all_or_none: {e}"));
                self
            }
            pub fn amount_indicator<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::AmountIndicator>>,
                T::Error: ::std::fmt::Display,
            {
                self.amount_indicator = value.try_into().map_err(|e| {
                    format!("error converting supplied value for amount_indicator: {e}")
                });
                self
            }
            pub fn close_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.close_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for close_time: {e}"));
                self
            }
            pub fn discretionary<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.discretionary = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for discretionary: {e}"));
                self
            }
            pub fn duration<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Duration>>,
                T::Error: ::std::fmt::Display,
            {
                self.duration = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for duration: {e}"));
                self
            }
            pub fn entered_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.entered_time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for entered_time: {e}"));
                self
            }
            pub fn filled_quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.filled_quantity = value.try_into().map_err(|e| {
                    format!("error converting supplied value for filled_quantity: {e}")
                });
                self
            }
            pub fn order_balance<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::OrderBalance>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_balance = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for order_balance: {e}"));
                self
            }
            pub fn order_legs<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::OrderLeg>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_legs = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for order_legs: {e}"));
                self
            }
            pub fn order_strategy_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::OrderStrategyType>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_strategy_type = value.try_into().map_err(|e| {
                    format!("error converting supplied value for order_strategy_type: {e}")
                });
                self
            }
            pub fn order_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::OrderType>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for order_type: {e}"));
                self
            }
            pub fn order_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for order_value: {e}"));
                self
            }
            pub fn order_version<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_version = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for order_version: {e}"));
                self
            }
            pub fn price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for price: {e}"));
                self
            }
            pub fn quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.quantity = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for quantity: {e}"));
                self
            }
            pub fn remaining_quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.remaining_quantity = value.try_into().map_err(|e| {
                    format!("error converting supplied value for remaining_quantity: {e}")
                });
                self
            }
            pub fn sell_non_marginable_first<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<bool>>,
                T::Error: ::std::fmt::Display,
            {
                self.sell_non_marginable_first = value.try_into().map_err(|e| {
                    format!("error converting supplied value for sell_non_marginable_first: {e}")
                });
                self
            }
            pub fn session<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::Session>>,
                T::Error: ::std::fmt::Display,
            {
                self.session = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for session: {e}"));
                self
            }
            pub fn settlement_instruction<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::SettlementInstruction>>,
                T::Error: ::std::fmt::Display,
            {
                self.settlement_instruction = value.try_into().map_err(|e| {
                    format!("error converting supplied value for settlement_instruction: {e}")
                });
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiOrderStatus>>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {e}"));
                self
            }
            pub fn strategy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ComplexOrderStrategyType>>,
                T::Error: ::std::fmt::Display,
            {
                self.strategy = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for strategy: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<OrderStrategy> for super::OrderStrategy {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OrderStrategy,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    account_number: value.account_number?,
                    advanced_order_type: value.advanced_order_type?,
                    all_or_none: value.all_or_none?,
                    amount_indicator: value.amount_indicator?,
                    close_time: value.close_time?,
                    discretionary: value.discretionary?,
                    duration: value.duration?,
                    entered_time: value.entered_time?,
                    filled_quantity: value.filled_quantity?,
                    order_balance: value.order_balance?,
                    order_legs: value.order_legs?,
                    order_strategy_type: value.order_strategy_type?,
                    order_type: value.order_type?,
                    order_value: value.order_value?,
                    order_version: value.order_version?,
                    price: value.price?,
                    quantity: value.quantity?,
                    remaining_quantity: value.remaining_quantity?,
                    sell_non_marginable_first: value.sell_non_marginable_first?,
                    session: value.session?,
                    settlement_instruction: value.settlement_instruction?,
                    status: value.status?,
                    strategy: value.strategy?,
                })
            }
        }

        impl ::std::convert::From<super::OrderStrategy> for OrderStrategy {
            fn from(value: super::OrderStrategy) -> Self {
                Self {
                    account_number: Ok(value.account_number),
                    advanced_order_type: Ok(value.advanced_order_type),
                    all_or_none: Ok(value.all_or_none),
                    amount_indicator: Ok(value.amount_indicator),
                    close_time: Ok(value.close_time),
                    discretionary: Ok(value.discretionary),
                    duration: Ok(value.duration),
                    entered_time: Ok(value.entered_time),
                    filled_quantity: Ok(value.filled_quantity),
                    order_balance: Ok(value.order_balance),
                    order_legs: Ok(value.order_legs),
                    order_strategy_type: Ok(value.order_strategy_type),
                    order_type: Ok(value.order_type),
                    order_value: Ok(value.order_value),
                    order_version: Ok(value.order_version),
                    price: Ok(value.price),
                    quantity: Ok(value.quantity),
                    remaining_quantity: Ok(value.remaining_quantity),
                    sell_non_marginable_first: Ok(value.sell_non_marginable_first),
                    session: Ok(value.session),
                    settlement_instruction: Ok(value.settlement_instruction),
                    status: Ok(value.status),
                    strategy: Ok(value.strategy),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OrderValidationDetail {
            activity_message: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            message: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            original_severity: ::std::result::Result<
                ::std::option::Option<super::ApiRuleAction>,
                ::std::string::String,
            >,
            override_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            override_severity: ::std::result::Result<
                ::std::option::Option<super::ApiRuleAction>,
                ::std::string::String,
            >,
            validation_rule_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for OrderValidationDetail {
            fn default() -> Self {
                Self {
                    activity_message: Ok(Default::default()),
                    message: Ok(Default::default()),
                    original_severity: Ok(Default::default()),
                    override_name: Ok(Default::default()),
                    override_severity: Ok(Default::default()),
                    validation_rule_name: Ok(Default::default()),
                }
            }
        }

        impl OrderValidationDetail {
            pub fn activity_message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.activity_message = value.try_into().map_err(|e| {
                    format!("error converting supplied value for activity_message: {e}")
                });
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
            pub fn original_severity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiRuleAction>>,
                T::Error: ::std::fmt::Display,
            {
                self.original_severity = value.try_into().map_err(|e| {
                    format!("error converting supplied value for original_severity: {e}")
                });
                self
            }
            pub fn override_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.override_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for override_name: {e}"));
                self
            }
            pub fn override_severity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ApiRuleAction>>,
                T::Error: ::std::fmt::Display,
            {
                self.override_severity = value.try_into().map_err(|e| {
                    format!("error converting supplied value for override_severity: {e}")
                });
                self
            }
            pub fn validation_rule_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.validation_rule_name = value.try_into().map_err(|e| {
                    format!("error converting supplied value for validation_rule_name: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<OrderValidationDetail> for super::OrderValidationDetail {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OrderValidationDetail,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    activity_message: value.activity_message?,
                    message: value.message?,
                    original_severity: value.original_severity?,
                    override_name: value.override_name?,
                    override_severity: value.override_severity?,
                    validation_rule_name: value.validation_rule_name?,
                })
            }
        }

        impl ::std::convert::From<super::OrderValidationDetail> for OrderValidationDetail {
            fn from(value: super::OrderValidationDetail) -> Self {
                Self {
                    activity_message: Ok(value.activity_message),
                    message: Ok(value.message),
                    original_severity: Ok(value.original_severity),
                    override_name: Ok(value.override_name),
                    override_severity: Ok(value.override_severity),
                    validation_rule_name: Ok(value.validation_rule_name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OrderValidationResult {
            accepts: ::std::result::Result<
                ::std::vec::Vec<super::OrderValidationDetail>,
                ::std::string::String,
            >,
            alerts: ::std::result::Result<
                ::std::vec::Vec<super::OrderValidationDetail>,
                ::std::string::String,
            >,
            rejects: ::std::result::Result<
                ::std::vec::Vec<super::OrderValidationDetail>,
                ::std::string::String,
            >,
            reviews: ::std::result::Result<
                ::std::vec::Vec<super::OrderValidationDetail>,
                ::std::string::String,
            >,
            warns: ::std::result::Result<
                ::std::vec::Vec<super::OrderValidationDetail>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for OrderValidationResult {
            fn default() -> Self {
                Self {
                    accepts: Ok(Default::default()),
                    alerts: Ok(Default::default()),
                    rejects: Ok(Default::default()),
                    reviews: Ok(Default::default()),
                    warns: Ok(Default::default()),
                }
            }
        }

        impl OrderValidationResult {
            pub fn accepts<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::OrderValidationDetail>>,
                T::Error: ::std::fmt::Display,
            {
                self.accepts = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for accepts: {e}"));
                self
            }
            pub fn alerts<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::OrderValidationDetail>>,
                T::Error: ::std::fmt::Display,
            {
                self.alerts = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for alerts: {e}"));
                self
            }
            pub fn rejects<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::OrderValidationDetail>>,
                T::Error: ::std::fmt::Display,
            {
                self.rejects = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rejects: {e}"));
                self
            }
            pub fn reviews<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::OrderValidationDetail>>,
                T::Error: ::std::fmt::Display,
            {
                self.reviews = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for reviews: {e}"));
                self
            }
            pub fn warns<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::OrderValidationDetail>>,
                T::Error: ::std::fmt::Display,
            {
                self.warns = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for warns: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<OrderValidationResult> for super::OrderValidationResult {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OrderValidationResult,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    accepts: value.accepts?,
                    alerts: value.alerts?,
                    rejects: value.rejects?,
                    reviews: value.reviews?,
                    warns: value.warns?,
                })
            }
        }

        impl ::std::convert::From<super::OrderValidationResult> for OrderValidationResult {
            fn from(value: super::OrderValidationResult) -> Self {
                Self {
                    accepts: Ok(value.accepts),
                    alerts: Ok(value.alerts),
                    rejects: Ok(value.rejects),
                    reviews: Ok(value.reviews),
                    warns: Ok(value.warns),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Position {
            aged_quantity: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            average_long_price:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            average_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            average_short_price:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            current_day_cost:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            current_day_profit_loss:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            current_day_profit_loss_percentage:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            instrument: ::std::result::Result<
                ::std::option::Option<super::AccountsInstrument>,
                ::std::string::String,
            >,
            long_open_profit_loss:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            long_quantity: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            maintenance_requirement:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            market_value: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            previous_session_long_quantity:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            previous_session_short_quantity:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            settled_long_quantity:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            settled_short_quantity:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            short_open_profit_loss:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            short_quantity:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            tax_lot_average_long_price:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            tax_lot_average_short_price:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for Position {
            fn default() -> Self {
                Self {
                    aged_quantity: Ok(Default::default()),
                    average_long_price: Ok(Default::default()),
                    average_price: Ok(Default::default()),
                    average_short_price: Ok(Default::default()),
                    current_day_cost: Ok(Default::default()),
                    current_day_profit_loss: Ok(Default::default()),
                    current_day_profit_loss_percentage: Ok(Default::default()),
                    instrument: Ok(Default::default()),
                    long_open_profit_loss: Ok(Default::default()),
                    long_quantity: Ok(Default::default()),
                    maintenance_requirement: Ok(Default::default()),
                    market_value: Ok(Default::default()),
                    previous_session_long_quantity: Ok(Default::default()),
                    previous_session_short_quantity: Ok(Default::default()),
                    settled_long_quantity: Ok(Default::default()),
                    settled_short_quantity: Ok(Default::default()),
                    short_open_profit_loss: Ok(Default::default()),
                    short_quantity: Ok(Default::default()),
                    tax_lot_average_long_price: Ok(Default::default()),
                    tax_lot_average_short_price: Ok(Default::default()),
                }
            }
        }

        impl Position {
            pub fn aged_quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.aged_quantity = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for aged_quantity: {e}"));
                self
            }
            pub fn average_long_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.average_long_price = value.try_into().map_err(|e| {
                    format!("error converting supplied value for average_long_price: {e}")
                });
                self
            }
            pub fn average_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.average_price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for average_price: {e}"));
                self
            }
            pub fn average_short_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.average_short_price = value.try_into().map_err(|e| {
                    format!("error converting supplied value for average_short_price: {e}")
                });
                self
            }
            pub fn current_day_cost<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.current_day_cost = value.try_into().map_err(|e| {
                    format!("error converting supplied value for current_day_cost: {e}")
                });
                self
            }
            pub fn current_day_profit_loss<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.current_day_profit_loss = value.try_into().map_err(|e| {
                    format!("error converting supplied value for current_day_profit_loss: {e}")
                });
                self
            }
            pub fn current_day_profit_loss_percentage<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self . current_day_profit_loss_percentage = value . try_into () . map_err (| e | format ! ("error converting supplied value for current_day_profit_loss_percentage: {e}")) ;
                self
            }
            pub fn instrument<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::AccountsInstrument>>,
                T::Error: ::std::fmt::Display,
            {
                self.instrument = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instrument: {e}"));
                self
            }
            pub fn long_open_profit_loss<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.long_open_profit_loss = value.try_into().map_err(|e| {
                    format!("error converting supplied value for long_open_profit_loss: {e}")
                });
                self
            }
            pub fn long_quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.long_quantity = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for long_quantity: {e}"));
                self
            }
            pub fn maintenance_requirement<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.maintenance_requirement = value.try_into().map_err(|e| {
                    format!("error converting supplied value for maintenance_requirement: {e}")
                });
                self
            }
            pub fn market_value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.market_value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for market_value: {e}"));
                self
            }
            pub fn previous_session_long_quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.previous_session_long_quantity = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for previous_session_long_quantity: {e}"
                    )
                });
                self
            }
            pub fn previous_session_short_quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.previous_session_short_quantity = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for previous_session_short_quantity: {e}"
                    )
                });
                self
            }
            pub fn settled_long_quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.settled_long_quantity = value.try_into().map_err(|e| {
                    format!("error converting supplied value for settled_long_quantity: {e}")
                });
                self
            }
            pub fn settled_short_quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.settled_short_quantity = value.try_into().map_err(|e| {
                    format!("error converting supplied value for settled_short_quantity: {e}")
                });
                self
            }
            pub fn short_open_profit_loss<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.short_open_profit_loss = value.try_into().map_err(|e| {
                    format!("error converting supplied value for short_open_profit_loss: {e}")
                });
                self
            }
            pub fn short_quantity<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.short_quantity = value.try_into().map_err(|e| {
                    format!("error converting supplied value for short_quantity: {e}")
                });
                self
            }
            pub fn tax_lot_average_long_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.tax_lot_average_long_price = value.try_into().map_err(|e| {
                    format!("error converting supplied value for tax_lot_average_long_price: {e}")
                });
                self
            }
            pub fn tax_lot_average_short_price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.tax_lot_average_short_price = value.try_into().map_err(|e| {
                    format!("error converting supplied value for tax_lot_average_short_price: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<Position> for super::Position {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Position,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    aged_quantity: value.aged_quantity?,
                    average_long_price: value.average_long_price?,
                    average_price: value.average_price?,
                    average_short_price: value.average_short_price?,
                    current_day_cost: value.current_day_cost?,
                    current_day_profit_loss: value.current_day_profit_loss?,
                    current_day_profit_loss_percentage: value.current_day_profit_loss_percentage?,
                    instrument: value.instrument?,
                    long_open_profit_loss: value.long_open_profit_loss?,
                    long_quantity: value.long_quantity?,
                    maintenance_requirement: value.maintenance_requirement?,
                    market_value: value.market_value?,
                    previous_session_long_quantity: value.previous_session_long_quantity?,
                    previous_session_short_quantity: value.previous_session_short_quantity?,
                    settled_long_quantity: value.settled_long_quantity?,
                    settled_short_quantity: value.settled_short_quantity?,
                    short_open_profit_loss: value.short_open_profit_loss?,
                    short_quantity: value.short_quantity?,
                    tax_lot_average_long_price: value.tax_lot_average_long_price?,
                    tax_lot_average_short_price: value.tax_lot_average_short_price?,
                })
            }
        }

        impl ::std::convert::From<super::Position> for Position {
            fn from(value: super::Position) -> Self {
                Self {
                    aged_quantity: Ok(value.aged_quantity),
                    average_long_price: Ok(value.average_long_price),
                    average_price: Ok(value.average_price),
                    average_short_price: Ok(value.average_short_price),
                    current_day_cost: Ok(value.current_day_cost),
                    current_day_profit_loss: Ok(value.current_day_profit_loss),
                    current_day_profit_loss_percentage: Ok(value.current_day_profit_loss_percentage),
                    instrument: Ok(value.instrument),
                    long_open_profit_loss: Ok(value.long_open_profit_loss),
                    long_quantity: Ok(value.long_quantity),
                    maintenance_requirement: Ok(value.maintenance_requirement),
                    market_value: Ok(value.market_value),
                    previous_session_long_quantity: Ok(value.previous_session_long_quantity),
                    previous_session_short_quantity: Ok(value.previous_session_short_quantity),
                    settled_long_quantity: Ok(value.settled_long_quantity),
                    settled_short_quantity: Ok(value.settled_short_quantity),
                    short_open_profit_loss: Ok(value.short_open_profit_loss),
                    short_quantity: Ok(value.short_quantity),
                    tax_lot_average_long_price: Ok(value.tax_lot_average_long_price),
                    tax_lot_average_short_price: Ok(value.tax_lot_average_short_price),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct PreviewOrder {
            commission_and_fee: ::std::result::Result<
                ::std::option::Option<super::CommissionAndFee>,
                ::std::string::String,
            >,
            order_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            order_strategy: ::std::result::Result<
                ::std::option::Option<super::OrderStrategy>,
                ::std::string::String,
            >,
            order_validation_result: ::std::result::Result<
                ::std::option::Option<super::OrderValidationResult>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for PreviewOrder {
            fn default() -> Self {
                Self {
                    commission_and_fee: Ok(Default::default()),
                    order_id: Ok(Default::default()),
                    order_strategy: Ok(Default::default()),
                    order_validation_result: Ok(Default::default()),
                }
            }
        }

        impl PreviewOrder {
            pub fn commission_and_fee<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::CommissionAndFee>>,
                T::Error: ::std::fmt::Display,
            {
                self.commission_and_fee = value.try_into().map_err(|e| {
                    format!("error converting supplied value for commission_and_fee: {e}")
                });
                self
            }
            pub fn order_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for order_id: {e}"));
                self
            }
            pub fn order_strategy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::OrderStrategy>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_strategy = value.try_into().map_err(|e| {
                    format!("error converting supplied value for order_strategy: {e}")
                });
                self
            }
            pub fn order_validation_result<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::OrderValidationResult>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_validation_result = value.try_into().map_err(|e| {
                    format!("error converting supplied value for order_validation_result: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<PreviewOrder> for super::PreviewOrder {
            type Error = super::error::ConversionError;
            fn try_from(
                value: PreviewOrder,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    commission_and_fee: value.commission_and_fee?,
                    order_id: value.order_id?,
                    order_strategy: value.order_strategy?,
                    order_validation_result: value.order_validation_result?,
                })
            }
        }

        impl ::std::convert::From<super::PreviewOrder> for PreviewOrder {
            fn from(value: super::PreviewOrder) -> Self {
                Self {
                    commission_and_fee: Ok(value.commission_and_fee),
                    order_id: Ok(value.order_id),
                    order_strategy: Ok(value.order_strategy),
                    order_validation_result: Ok(value.order_validation_result),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Product {
            asset_type: ::std::result::Result<super::ProductAssetType, ::std::string::String>,
            cusip: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            name: ::std::result::Result<::serde_json::Value, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                ::std::option::Option<super::ProductType>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for Product {
            fn default() -> Self {
                Self {
                    asset_type: Err("no value supplied for asset_type".to_string()),
                    cusip: Ok(Default::default()),
                    description: Ok(Default::default()),
                    instrument_id: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    net_change: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl Product {
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::ProductAssetType>,
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
            pub fn instrument_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.instrument_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
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
                T: ::std::convert::TryInto<::std::option::Option<super::ProductType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Product> for super::Product {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Product,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_type: value.asset_type?,
                    cusip: value.cusip?,
                    description: value.description?,
                    instrument_id: value.instrument_id?,
                    name: value.name?,
                    net_change: value.net_change?,
                    symbol: value.symbol?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::Product> for Product {
            fn from(value: super::Product) -> Self {
                Self {
                    asset_type: Ok(value.asset_type),
                    cusip: Ok(value.cusip),
                    description: Ok(value.description),
                    instrument_id: Ok(value.instrument_id),
                    name: Ok(value.name),
                    net_change: Ok(value.net_change),
                    symbol: Ok(value.symbol),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct SecuritiesAccountBase {
            account_number: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            is_closing_only_restricted: ::std::result::Result<bool, ::std::string::String>,
            is_day_trader: ::std::result::Result<bool, ::std::string::String>,
            pfcb_flag: ::std::result::Result<bool, ::std::string::String>,
            positions:
                ::std::result::Result<::std::vec::Vec<super::Position>, ::std::string::String>,
            round_trips: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            type_: ::std::result::Result<
                ::std::option::Option<super::SecuritiesAccountBaseType>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for SecuritiesAccountBase {
            fn default() -> Self {
                Self {
                    account_number: Ok(Default::default()),
                    is_closing_only_restricted: Ok(Default::default()),
                    is_day_trader: Ok(Default::default()),
                    pfcb_flag: Ok(Default::default()),
                    positions: Ok(Default::default()),
                    round_trips: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl SecuritiesAccountBase {
            pub fn account_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.account_number = value.try_into().map_err(|e| {
                    format!("error converting supplied value for account_number: {e}")
                });
                self
            }
            pub fn is_closing_only_restricted<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.is_closing_only_restricted = value.try_into().map_err(|e| {
                    format!("error converting supplied value for is_closing_only_restricted: {e}")
                });
                self
            }
            pub fn is_day_trader<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.is_day_trader = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for is_day_trader: {e}"));
                self
            }
            pub fn pfcb_flag<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.pfcb_flag = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pfcb_flag: {e}"));
                self
            }
            pub fn positions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::Position>>,
                T::Error: ::std::fmt::Display,
            {
                self.positions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for positions: {e}"));
                self
            }
            pub fn round_trips<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.round_trips = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for round_trips: {e}"));
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::SecuritiesAccountBaseType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<SecuritiesAccountBase> for super::SecuritiesAccountBase {
            type Error = super::error::ConversionError;
            fn try_from(
                value: SecuritiesAccountBase,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    account_number: value.account_number?,
                    is_closing_only_restricted: value.is_closing_only_restricted?,
                    is_day_trader: value.is_day_trader?,
                    pfcb_flag: value.pfcb_flag?,
                    positions: value.positions?,
                    round_trips: value.round_trips?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::SecuritiesAccountBase> for SecuritiesAccountBase {
            fn from(value: super::SecuritiesAccountBase) -> Self {
                Self {
                    account_number: Ok(value.account_number),
                    is_closing_only_restricted: Ok(value.is_closing_only_restricted),
                    is_day_trader: Ok(value.is_day_trader),
                    pfcb_flag: Ok(value.pfcb_flag),
                    positions: Ok(value.positions),
                    round_trips: Ok(value.round_trips),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ServiceError {
            errors: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            message: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for ServiceError {
            fn default() -> Self {
                Self {
                    errors: Ok(Default::default()),
                    message: Ok(Default::default()),
                }
            }
        }

        impl ServiceError {
            pub fn errors<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.errors = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for errors: {e}"));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<ServiceError> for super::ServiceError {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ServiceError,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    errors: value.errors?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::ServiceError> for ServiceError {
            fn from(value: super::ServiceError) -> Self {
                Self {
                    errors: Ok(value.errors),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct StreamerInfo {
            schwab_client_channel: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            schwab_client_correl_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            schwab_client_customer_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            schwab_client_function_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            streamer_socket_url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for StreamerInfo {
            fn default() -> Self {
                Self {
                    schwab_client_channel: Ok(Default::default()),
                    schwab_client_correl_id: Ok(Default::default()),
                    schwab_client_customer_id: Ok(Default::default()),
                    schwab_client_function_id: Ok(Default::default()),
                    streamer_socket_url: Ok(Default::default()),
                }
            }
        }

        impl StreamerInfo {
            pub fn schwab_client_channel<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.schwab_client_channel = value.try_into().map_err(|e| {
                    format!("error converting supplied value for schwab_client_channel: {e}")
                });
                self
            }
            pub fn schwab_client_correl_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.schwab_client_correl_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for schwab_client_correl_id: {e}")
                });
                self
            }
            pub fn schwab_client_customer_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.schwab_client_customer_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for schwab_client_customer_id: {e}")
                });
                self
            }
            pub fn schwab_client_function_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.schwab_client_function_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for schwab_client_function_id: {e}")
                });
                self
            }
            pub fn streamer_socket_url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.streamer_socket_url = value.try_into().map_err(|e| {
                    format!("error converting supplied value for streamer_socket_url: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<StreamerInfo> for super::StreamerInfo {
            type Error = super::error::ConversionError;
            fn try_from(
                value: StreamerInfo,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    schwab_client_channel: value.schwab_client_channel?,
                    schwab_client_correl_id: value.schwab_client_correl_id?,
                    schwab_client_customer_id: value.schwab_client_customer_id?,
                    schwab_client_function_id: value.schwab_client_function_id?,
                    streamer_socket_url: value.streamer_socket_url?,
                })
            }
        }

        impl ::std::convert::From<super::StreamerInfo> for StreamerInfo {
            fn from(value: super::StreamerInfo) -> Self {
                Self {
                    schwab_client_channel: Ok(value.schwab_client_channel),
                    schwab_client_correl_id: Ok(value.schwab_client_correl_id),
                    schwab_client_customer_id: Ok(value.schwab_client_customer_id),
                    schwab_client_function_id: Ok(value.schwab_client_function_id),
                    streamer_socket_url: Ok(value.streamer_socket_url),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Transaction {
            account_number: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            activity_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            activity_type: ::std::result::Result<
                ::std::option::Option<super::TransactionActivityType>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            net_amount: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            order_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            position_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            settlement_date: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            status: ::std::result::Result<
                ::std::option::Option<super::TransactionStatus>,
                ::std::string::String,
            >,
            sub_account: ::std::result::Result<
                ::std::option::Option<super::TransactionSubAccount>,
                ::std::string::String,
            >,
            time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            trade_date: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            transfer_items:
                ::std::result::Result<::std::vec::Vec<super::TransferItem>, ::std::string::String>,
            type_: ::std::result::Result<
                ::std::option::Option<super::TransactionType>,
                ::std::string::String,
            >,
            user: ::std::result::Result<
                ::std::option::Option<super::UserDetails>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for Transaction {
            fn default() -> Self {
                Self {
                    account_number: Ok(Default::default()),
                    activity_id: Ok(Default::default()),
                    activity_type: Ok(Default::default()),
                    description: Ok(Default::default()),
                    net_amount: Ok(Default::default()),
                    order_id: Ok(Default::default()),
                    position_id: Ok(Default::default()),
                    settlement_date: Ok(Default::default()),
                    status: Ok(Default::default()),
                    sub_account: Ok(Default::default()),
                    time: Ok(Default::default()),
                    trade_date: Ok(Default::default()),
                    transfer_items: Ok(Default::default()),
                    type_: Ok(Default::default()),
                    user: Ok(Default::default()),
                }
            }
        }

        impl Transaction {
            pub fn account_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.account_number = value.try_into().map_err(|e| {
                    format!("error converting supplied value for account_number: {e}")
                });
                self
            }
            pub fn activity_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.activity_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for activity_id: {e}"));
                self
            }
            pub fn activity_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::TransactionActivityType>>,
                T::Error: ::std::fmt::Display,
            {
                self.activity_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for activity_type: {e}"));
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
            pub fn net_amount<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.net_amount = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for net_amount: {e}"));
                self
            }
            pub fn order_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.order_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for order_id: {e}"));
                self
            }
            pub fn position_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.position_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for position_id: {e}"));
                self
            }
            pub fn settlement_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.settlement_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for settlement_date: {e}")
                });
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::TransactionStatus>>,
                T::Error: ::std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {e}"));
                self
            }
            pub fn sub_account<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::TransactionSubAccount>>,
                T::Error: ::std::fmt::Display,
            {
                self.sub_account = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sub_account: {e}"));
                self
            }
            pub fn time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.time = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for time: {e}"));
                self
            }
            pub fn trade_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.trade_date = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for trade_date: {e}"));
                self
            }
            pub fn transfer_items<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::TransferItem>>,
                T::Error: ::std::fmt::Display,
            {
                self.transfer_items = value.try_into().map_err(|e| {
                    format!("error converting supplied value for transfer_items: {e}")
                });
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::TransactionType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
            pub fn user<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::UserDetails>>,
                T::Error: ::std::fmt::Display,
            {
                self.user = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<Transaction> for super::Transaction {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Transaction,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    account_number: value.account_number?,
                    activity_id: value.activity_id?,
                    activity_type: value.activity_type?,
                    description: value.description?,
                    net_amount: value.net_amount?,
                    order_id: value.order_id?,
                    position_id: value.position_id?,
                    settlement_date: value.settlement_date?,
                    status: value.status?,
                    sub_account: value.sub_account?,
                    time: value.time?,
                    trade_date: value.trade_date?,
                    transfer_items: value.transfer_items?,
                    type_: value.type_?,
                    user: value.user?,
                })
            }
        }

        impl ::std::convert::From<super::Transaction> for Transaction {
            fn from(value: super::Transaction) -> Self {
                Self {
                    account_number: Ok(value.account_number),
                    activity_id: Ok(value.activity_id),
                    activity_type: Ok(value.activity_type),
                    description: Ok(value.description),
                    net_amount: Ok(value.net_amount),
                    order_id: Ok(value.order_id),
                    position_id: Ok(value.position_id),
                    settlement_date: Ok(value.settlement_date),
                    status: Ok(value.status),
                    sub_account: Ok(value.sub_account),
                    time: Ok(value.time),
                    trade_date: Ok(value.trade_date),
                    transfer_items: Ok(value.transfer_items),
                    type_: Ok(value.type_),
                    user: Ok(value.user),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct TransactionApiOptionDeliverable {
            asset_type: ::std::result::Result<
                ::std::option::Option<super::AssetType>,
                ::std::string::String,
            >,
            deliverable: ::std::result::Result<
                ::std::option::Option<::std::boxed::Box<super::TransactionInstrument>>,
                ::std::string::String,
            >,
            deliverable_number:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            deliverable_units:
                ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            root_symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            strike_percent:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        }

        impl ::std::default::Default for TransactionApiOptionDeliverable {
            fn default() -> Self {
                Self {
                    asset_type: Ok(Default::default()),
                    deliverable: Ok(Default::default()),
                    deliverable_number: Ok(Default::default()),
                    deliverable_units: Ok(Default::default()),
                    root_symbol: Ok(Default::default()),
                    strike_percent: Ok(Default::default()),
                }
            }
        }

        impl TransactionApiOptionDeliverable {
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::AssetType>>,
                T::Error: ::std::fmt::Display,
            {
                self.asset_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for asset_type: {e}"));
                self
            }
            pub fn deliverable<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::std::boxed::Box<super::TransactionInstrument>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.deliverable = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deliverable: {e}"));
                self
            }
            pub fn deliverable_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.deliverable_number = value.try_into().map_err(|e| {
                    format!("error converting supplied value for deliverable_number: {e}")
                });
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
            pub fn root_symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.root_symbol = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for root_symbol: {e}"));
                self
            }
            pub fn strike_percent<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.strike_percent = value.try_into().map_err(|e| {
                    format!("error converting supplied value for strike_percent: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<TransactionApiOptionDeliverable>
            for super::TransactionApiOptionDeliverable
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: TransactionApiOptionDeliverable,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_type: value.asset_type?,
                    deliverable: value.deliverable?,
                    deliverable_number: value.deliverable_number?,
                    deliverable_units: value.deliverable_units?,
                    root_symbol: value.root_symbol?,
                    strike_percent: value.strike_percent?,
                })
            }
        }

        impl ::std::convert::From<super::TransactionApiOptionDeliverable>
            for TransactionApiOptionDeliverable
        {
            fn from(value: super::TransactionApiOptionDeliverable) -> Self {
                Self {
                    asset_type: Ok(value.asset_type),
                    deliverable: Ok(value.deliverable),
                    deliverable_number: Ok(value.deliverable_number),
                    deliverable_units: Ok(value.deliverable_units),
                    root_symbol: Ok(value.root_symbol),
                    strike_percent: Ok(value.strike_percent),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct TransactionBaseInstrument {
            asset_type: ::std::result::Result<
                super::TransactionBaseInstrumentAssetType,
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
            instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            name: ::std::result::Result<::serde_json::Value, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for TransactionBaseInstrument {
            fn default() -> Self {
                Self {
                    asset_type: Err("no value supplied for asset_type".to_string()),
                    cusip: Ok(Default::default()),
                    description: Ok(Default::default()),
                    instrument_id: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    net_change: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                }
            }
        }

        impl TransactionBaseInstrument {
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TransactionBaseInstrumentAssetType>,
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
            pub fn instrument_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.instrument_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
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

        impl ::std::convert::TryFrom<TransactionBaseInstrument> for super::TransactionBaseInstrument {
            type Error = super::error::ConversionError;
            fn try_from(
                value: TransactionBaseInstrument,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_type: value.asset_type?,
                    cusip: value.cusip?,
                    description: value.description?,
                    instrument_id: value.instrument_id?,
                    name: value.name?,
                    net_change: value.net_change?,
                    symbol: value.symbol?,
                })
            }
        }

        impl ::std::convert::From<super::TransactionBaseInstrument> for TransactionBaseInstrument {
            fn from(value: super::TransactionBaseInstrument) -> Self {
                Self {
                    asset_type: Ok(value.asset_type),
                    cusip: Ok(value.cusip),
                    description: Ok(value.description),
                    instrument_id: Ok(value.instrument_id),
                    name: Ok(value.name),
                    net_change: Ok(value.net_change),
                    symbol: Ok(value.symbol),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct TransactionCashEquivalent {
            asset_type: ::std::result::Result<
                super::TransactionCashEquivalentAssetType,
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
            instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            name: ::std::result::Result<::serde_json::Value, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                ::std::option::Option<super::TransactionCashEquivalentType>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for TransactionCashEquivalent {
            fn default() -> Self {
                Self {
                    asset_type: Err("no value supplied for asset_type".to_string()),
                    cusip: Ok(Default::default()),
                    description: Ok(Default::default()),
                    instrument_id: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    net_change: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl TransactionCashEquivalent {
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TransactionCashEquivalentAssetType>,
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
            pub fn instrument_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.instrument_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
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
                T: ::std::convert::TryInto<
                        ::std::option::Option<super::TransactionCashEquivalentType>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<TransactionCashEquivalent> for super::TransactionCashEquivalent {
            type Error = super::error::ConversionError;
            fn try_from(
                value: TransactionCashEquivalent,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_type: value.asset_type?,
                    cusip: value.cusip?,
                    description: value.description?,
                    instrument_id: value.instrument_id?,
                    name: value.name?,
                    net_change: value.net_change?,
                    symbol: value.symbol?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::TransactionCashEquivalent> for TransactionCashEquivalent {
            fn from(value: super::TransactionCashEquivalent) -> Self {
                Self {
                    asset_type: Ok(value.asset_type),
                    cusip: Ok(value.cusip),
                    description: Ok(value.description),
                    instrument_id: Ok(value.instrument_id),
                    name: Ok(value.name),
                    net_change: Ok(value.net_change),
                    symbol: Ok(value.symbol),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct TransactionEquity {
            asset_type:
                ::std::result::Result<super::TransactionEquityAssetType, ::std::string::String>,
            cusip: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            name: ::std::result::Result<::serde_json::Value, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                ::std::option::Option<super::TransactionEquityType>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for TransactionEquity {
            fn default() -> Self {
                Self {
                    asset_type: Err("no value supplied for asset_type".to_string()),
                    cusip: Ok(Default::default()),
                    description: Ok(Default::default()),
                    instrument_id: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    net_change: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl TransactionEquity {
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TransactionEquityAssetType>,
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
            pub fn instrument_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.instrument_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
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
                T: ::std::convert::TryInto<::std::option::Option<super::TransactionEquityType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<TransactionEquity> for super::TransactionEquity {
            type Error = super::error::ConversionError;
            fn try_from(
                value: TransactionEquity,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_type: value.asset_type?,
                    cusip: value.cusip?,
                    description: value.description?,
                    instrument_id: value.instrument_id?,
                    name: value.name?,
                    net_change: value.net_change?,
                    symbol: value.symbol?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::TransactionEquity> for TransactionEquity {
            fn from(value: super::TransactionEquity) -> Self {
                Self {
                    asset_type: Ok(value.asset_type),
                    cusip: Ok(value.cusip),
                    description: Ok(value.description),
                    instrument_id: Ok(value.instrument_id),
                    name: Ok(value.name),
                    net_change: Ok(value.net_change),
                    symbol: Ok(value.symbol),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct TransactionFixedIncome {
            asset_type: ::std::result::Result<
                super::TransactionFixedIncomeAssetType,
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
            factor: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            maturity_date: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            multiplier: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            name: ::std::result::Result<::serde_json::Value, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                ::std::option::Option<super::TransactionFixedIncomeType>,
                ::std::string::String,
            >,
            variable_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for TransactionFixedIncome {
            fn default() -> Self {
                Self {
                    asset_type: Err("no value supplied for asset_type".to_string()),
                    cusip: Ok(Default::default()),
                    description: Ok(Default::default()),
                    factor: Ok(Default::default()),
                    instrument_id: Ok(Default::default()),
                    maturity_date: Ok(Default::default()),
                    multiplier: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    net_change: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    type_: Ok(Default::default()),
                    variable_rate: Ok(Default::default()),
                }
            }
        }

        impl TransactionFixedIncome {
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TransactionFixedIncomeAssetType>,
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
            pub fn factor<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.factor = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for factor: {e}"));
                self
            }
            pub fn instrument_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.instrument_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
                self
            }
            pub fn maturity_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.maturity_date = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for maturity_date: {e}"));
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
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
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
                T: ::std::convert::TryInto<
                        ::std::option::Option<super::TransactionFixedIncomeType>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
            pub fn variable_rate<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.variable_rate = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for variable_rate: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<TransactionFixedIncome> for super::TransactionFixedIncome {
            type Error = super::error::ConversionError;
            fn try_from(
                value: TransactionFixedIncome,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_type: value.asset_type?,
                    cusip: value.cusip?,
                    description: value.description?,
                    factor: value.factor?,
                    instrument_id: value.instrument_id?,
                    maturity_date: value.maturity_date?,
                    multiplier: value.multiplier?,
                    name: value.name?,
                    net_change: value.net_change?,
                    symbol: value.symbol?,
                    type_: value.type_?,
                    variable_rate: value.variable_rate?,
                })
            }
        }

        impl ::std::convert::From<super::TransactionFixedIncome> for TransactionFixedIncome {
            fn from(value: super::TransactionFixedIncome) -> Self {
                Self {
                    asset_type: Ok(value.asset_type),
                    cusip: Ok(value.cusip),
                    description: Ok(value.description),
                    factor: Ok(value.factor),
                    instrument_id: Ok(value.instrument_id),
                    maturity_date: Ok(value.maturity_date),
                    multiplier: Ok(value.multiplier),
                    name: Ok(value.name),
                    net_change: Ok(value.net_change),
                    symbol: Ok(value.symbol),
                    type_: Ok(value.type_),
                    variable_rate: Ok(value.variable_rate),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct TransactionMutualFund {
            asset_type:
                ::std::result::Result<super::TransactionMutualFundAssetType, ::std::string::String>,
            cusip: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            exchange_cutoff_time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            fund_family_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            fund_family_symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            fund_group: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            name: ::std::result::Result<::serde_json::Value, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            purchase_cutoff_time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            redemption_cutoff_time: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                ::std::option::Option<super::TransactionMutualFundType>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for TransactionMutualFund {
            fn default() -> Self {
                Self {
                    asset_type: Err("no value supplied for asset_type".to_string()),
                    cusip: Ok(Default::default()),
                    description: Ok(Default::default()),
                    exchange_cutoff_time: Ok(Default::default()),
                    fund_family_name: Ok(Default::default()),
                    fund_family_symbol: Ok(Default::default()),
                    fund_group: Ok(Default::default()),
                    instrument_id: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    net_change: Ok(Default::default()),
                    purchase_cutoff_time: Ok(Default::default()),
                    redemption_cutoff_time: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl TransactionMutualFund {
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TransactionMutualFundAssetType>,
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
            pub fn exchange_cutoff_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.exchange_cutoff_time = value.try_into().map_err(|e| {
                    format!("error converting supplied value for exchange_cutoff_time: {e}")
                });
                self
            }
            pub fn fund_family_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.fund_family_name = value.try_into().map_err(|e| {
                    format!("error converting supplied value for fund_family_name: {e}")
                });
                self
            }
            pub fn fund_family_symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.fund_family_symbol = value.try_into().map_err(|e| {
                    format!("error converting supplied value for fund_family_symbol: {e}")
                });
                self
            }
            pub fn fund_group<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.fund_group = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fund_group: {e}"));
                self
            }
            pub fn instrument_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.instrument_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
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
            pub fn purchase_cutoff_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.purchase_cutoff_time = value.try_into().map_err(|e| {
                    format!("error converting supplied value for purchase_cutoff_time: {e}")
                });
                self
            }
            pub fn redemption_cutoff_time<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.redemption_cutoff_time = value.try_into().map_err(|e| {
                    format!("error converting supplied value for redemption_cutoff_time: {e}")
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
                T: ::std::convert::TryInto<::std::option::Option<super::TransactionMutualFundType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<TransactionMutualFund> for super::TransactionMutualFund {
            type Error = super::error::ConversionError;
            fn try_from(
                value: TransactionMutualFund,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_type: value.asset_type?,
                    cusip: value.cusip?,
                    description: value.description?,
                    exchange_cutoff_time: value.exchange_cutoff_time?,
                    fund_family_name: value.fund_family_name?,
                    fund_family_symbol: value.fund_family_symbol?,
                    fund_group: value.fund_group?,
                    instrument_id: value.instrument_id?,
                    name: value.name?,
                    net_change: value.net_change?,
                    purchase_cutoff_time: value.purchase_cutoff_time?,
                    redemption_cutoff_time: value.redemption_cutoff_time?,
                    symbol: value.symbol?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::TransactionMutualFund> for TransactionMutualFund {
            fn from(value: super::TransactionMutualFund) -> Self {
                Self {
                    asset_type: Ok(value.asset_type),
                    cusip: Ok(value.cusip),
                    description: Ok(value.description),
                    exchange_cutoff_time: Ok(value.exchange_cutoff_time),
                    fund_family_name: Ok(value.fund_family_name),
                    fund_family_symbol: Ok(value.fund_family_symbol),
                    fund_group: Ok(value.fund_group),
                    instrument_id: Ok(value.instrument_id),
                    name: Ok(value.name),
                    net_change: Ok(value.net_change),
                    purchase_cutoff_time: Ok(value.purchase_cutoff_time),
                    redemption_cutoff_time: Ok(value.redemption_cutoff_time),
                    symbol: Ok(value.symbol),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct TransactionOption {
            asset_type:
                ::std::result::Result<super::TransactionOptionAssetType, ::std::string::String>,
            cusip: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            deliverable: ::std::result::Result<
                ::std::option::Option<::std::boxed::Box<super::TransactionInstrument>>,
                ::std::string::String,
            >,
            description: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            expiration_date: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            instrument_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            name: ::std::result::Result<::serde_json::Value, ::std::string::String>,
            net_change: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            option_deliverables: ::std::result::Result<
                ::std::vec::Vec<super::TransactionApiOptionDeliverable>,
                ::std::string::String,
            >,
            option_premium_multiplier:
                ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
            put_call: ::std::result::Result<
                ::std::option::Option<super::TransactionOptionPutCall>,
                ::std::string::String,
            >,
            strike_price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                ::std::option::Option<super::TransactionOptionType>,
                ::std::string::String,
            >,
            underlying_cusip: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            underlying_symbol: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for TransactionOption {
            fn default() -> Self {
                Self {
                    asset_type: Err("no value supplied for asset_type".to_string()),
                    cusip: Ok(Default::default()),
                    deliverable: Ok(Default::default()),
                    description: Ok(Default::default()),
                    expiration_date: Ok(Default::default()),
                    instrument_id: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    net_change: Ok(Default::default()),
                    option_deliverables: Ok(Default::default()),
                    option_premium_multiplier: Ok(Default::default()),
                    put_call: Ok(Default::default()),
                    strike_price: Ok(Default::default()),
                    symbol: Ok(Default::default()),
                    type_: Ok(Default::default()),
                    underlying_cusip: Ok(Default::default()),
                    underlying_symbol: Ok(Default::default()),
                }
            }
        }

        impl TransactionOption {
            pub fn asset_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TransactionOptionAssetType>,
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
            pub fn deliverable<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::std::boxed::Box<super::TransactionInstrument>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.deliverable = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deliverable: {e}"));
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
            pub fn expiration_date<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.expiration_date = value.try_into().map_err(|e| {
                    format!("error converting supplied value for expiration_date: {e}")
                });
                self
            }
            pub fn instrument_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.instrument_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instrument_id: {e}"));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {e}"));
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
            pub fn option_deliverables<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::TransactionApiOptionDeliverable>>,
                T::Error: ::std::fmt::Display,
            {
                self.option_deliverables = value.try_into().map_err(|e| {
                    format!("error converting supplied value for option_deliverables: {e}")
                });
                self
            }
            pub fn option_premium_multiplier<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.option_premium_multiplier = value.try_into().map_err(|e| {
                    format!("error converting supplied value for option_premium_multiplier: {e}")
                });
                self
            }
            pub fn put_call<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::TransactionOptionPutCall>>,
                T::Error: ::std::fmt::Display,
            {
                self.put_call = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for put_call: {e}"));
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
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::TransactionOptionType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
            pub fn underlying_cusip<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.underlying_cusip = value.try_into().map_err(|e| {
                    format!("error converting supplied value for underlying_cusip: {e}")
                });
                self
            }
            pub fn underlying_symbol<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.underlying_symbol = value.try_into().map_err(|e| {
                    format!("error converting supplied value for underlying_symbol: {e}")
                });
                self
            }
        }

        impl ::std::convert::TryFrom<TransactionOption> for super::TransactionOption {
            type Error = super::error::ConversionError;
            fn try_from(
                value: TransactionOption,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    asset_type: value.asset_type?,
                    cusip: value.cusip?,
                    deliverable: value.deliverable?,
                    description: value.description?,
                    expiration_date: value.expiration_date?,
                    instrument_id: value.instrument_id?,
                    name: value.name?,
                    net_change: value.net_change?,
                    option_deliverables: value.option_deliverables?,
                    option_premium_multiplier: value.option_premium_multiplier?,
                    put_call: value.put_call?,
                    strike_price: value.strike_price?,
                    symbol: value.symbol?,
                    type_: value.type_?,
                    underlying_cusip: value.underlying_cusip?,
                    underlying_symbol: value.underlying_symbol?,
                })
            }
        }

        impl ::std::convert::From<super::TransactionOption> for TransactionOption {
            fn from(value: super::TransactionOption) -> Self {
                Self {
                    asset_type: Ok(value.asset_type),
                    cusip: Ok(value.cusip),
                    deliverable: Ok(value.deliverable),
                    description: Ok(value.description),
                    expiration_date: Ok(value.expiration_date),
                    instrument_id: Ok(value.instrument_id),
                    name: Ok(value.name),
                    net_change: Ok(value.net_change),
                    option_deliverables: Ok(value.option_deliverables),
                    option_premium_multiplier: Ok(value.option_premium_multiplier),
                    put_call: Ok(value.put_call),
                    strike_price: Ok(value.strike_price),
                    symbol: Ok(value.symbol),
                    type_: Ok(value.type_),
                    underlying_cusip: Ok(value.underlying_cusip),
                    underlying_symbol: Ok(value.underlying_symbol),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct TransferItem {
            amount: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            cost: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
            fee_type: ::std::result::Result<
                ::std::option::Option<super::TransferItemFeeType>,
                ::std::string::String,
            >,
            instrument: ::std::result::Result<
                ::std::option::Option<super::TransactionInstrument>,
                ::std::string::String,
            >,
            position_effect: ::std::result::Result<
                ::std::option::Option<super::TransferItemPositionEffect>,
                ::std::string::String,
            >,
            price: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        }

        impl ::std::default::Default for TransferItem {
            fn default() -> Self {
                Self {
                    amount: Ok(Default::default()),
                    cost: Ok(Default::default()),
                    fee_type: Ok(Default::default()),
                    instrument: Ok(Default::default()),
                    position_effect: Ok(Default::default()),
                    price: Ok(Default::default()),
                }
            }
        }

        impl TransferItem {
            pub fn amount<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.amount = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for amount: {e}"));
                self
            }
            pub fn cost<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.cost = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cost: {e}"));
                self
            }
            pub fn fee_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::TransferItemFeeType>>,
                T::Error: ::std::fmt::Display,
            {
                self.fee_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for fee_type: {e}"));
                self
            }
            pub fn instrument<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::TransactionInstrument>>,
                T::Error: ::std::fmt::Display,
            {
                self.instrument = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for instrument: {e}"));
                self
            }
            pub fn position_effect<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                        ::std::option::Option<super::TransferItemPositionEffect>,
                    >,
                T::Error: ::std::fmt::Display,
            {
                self.position_effect = value.try_into().map_err(|e| {
                    format!("error converting supplied value for position_effect: {e}")
                });
                self
            }
            pub fn price<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<f64>>,
                T::Error: ::std::fmt::Display,
            {
                self.price = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for price: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<TransferItem> for super::TransferItem {
            type Error = super::error::ConversionError;
            fn try_from(
                value: TransferItem,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    amount: value.amount?,
                    cost: value.cost?,
                    fee_type: value.fee_type?,
                    instrument: value.instrument?,
                    position_effect: value.position_effect?,
                    price: value.price?,
                })
            }
        }

        impl ::std::convert::From<super::TransferItem> for TransferItem {
            fn from(value: super::TransferItem) -> Self {
                Self {
                    amount: Ok(value.amount),
                    cost: Ok(value.cost),
                    fee_type: Ok(value.fee_type),
                    instrument: Ok(value.instrument),
                    position_effect: Ok(value.position_effect),
                    price: Ok(value.price),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UserDetails {
            broker_rep_code: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            cd_domain_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            first_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            last_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            login: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            system_user_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            type_: ::std::result::Result<
                ::std::option::Option<super::UserDetailsType>,
                ::std::string::String,
            >,
            user_id: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        }

        impl ::std::default::Default for UserDetails {
            fn default() -> Self {
                Self {
                    broker_rep_code: Ok(Default::default()),
                    cd_domain_id: Ok(Default::default()),
                    first_name: Ok(Default::default()),
                    last_name: Ok(Default::default()),
                    login: Ok(Default::default()),
                    system_user_name: Ok(Default::default()),
                    type_: Ok(Default::default()),
                    user_id: Ok(Default::default()),
                }
            }
        }

        impl UserDetails {
            pub fn broker_rep_code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.broker_rep_code = value.try_into().map_err(|e| {
                    format!("error converting supplied value for broker_rep_code: {e}")
                });
                self
            }
            pub fn cd_domain_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.cd_domain_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for cd_domain_id: {e}"));
                self
            }
            pub fn first_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.first_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for first_name: {e}"));
                self
            }
            pub fn last_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.last_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_name: {e}"));
                self
            }
            pub fn login<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.login = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for login: {e}"));
                self
            }
            pub fn system_user_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.system_user_name = value.try_into().map_err(|e| {
                    format!("error converting supplied value for system_user_name: {e}")
                });
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::UserDetailsType>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
            pub fn user_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i64>>,
                T::Error: ::std::fmt::Display,
            {
                self.user_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user_id: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<UserDetails> for super::UserDetails {
            type Error = super::error::ConversionError;
            fn try_from(
                value: UserDetails,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    broker_rep_code: value.broker_rep_code?,
                    cd_domain_id: value.cd_domain_id?,
                    first_name: value.first_name?,
                    last_name: value.last_name?,
                    login: value.login?,
                    system_user_name: value.system_user_name?,
                    type_: value.type_?,
                    user_id: value.user_id?,
                })
            }
        }

        impl ::std::convert::From<super::UserDetails> for UserDetails {
            fn from(value: super::UserDetails) -> Self {
                Self {
                    broker_rep_code: Ok(value.broker_rep_code),
                    cd_domain_id: Ok(value.cd_domain_id),
                    first_name: Ok(value.first_name),
                    last_name: Ok(value.last_name),
                    login: Ok(value.login),
                    system_user_name: Ok(value.system_user_name),
                    type_: Ok(value.type_),
                    user_id: Ok(value.user_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UserPreference {
            accounts: ::std::result::Result<
                ::std::vec::Vec<super::UserPreferenceAccount>,
                ::std::string::String,
            >,
            offers: ::std::result::Result<::std::vec::Vec<super::Offer>, ::std::string::String>,
            streamer_info:
                ::std::result::Result<::std::vec::Vec<super::StreamerInfo>, ::std::string::String>,
        }

        impl ::std::default::Default for UserPreference {
            fn default() -> Self {
                Self {
                    accounts: Ok(Default::default()),
                    offers: Ok(Default::default()),
                    streamer_info: Ok(Default::default()),
                }
            }
        }

        impl UserPreference {
            pub fn accounts<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::UserPreferenceAccount>>,
                T::Error: ::std::fmt::Display,
            {
                self.accounts = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for accounts: {e}"));
                self
            }
            pub fn offers<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::Offer>>,
                T::Error: ::std::fmt::Display,
            {
                self.offers = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for offers: {e}"));
                self
            }
            pub fn streamer_info<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::StreamerInfo>>,
                T::Error: ::std::fmt::Display,
            {
                self.streamer_info = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for streamer_info: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<UserPreference> for super::UserPreference {
            type Error = super::error::ConversionError;
            fn try_from(
                value: UserPreference,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    accounts: value.accounts?,
                    offers: value.offers?,
                    streamer_info: value.streamer_info?,
                })
            }
        }

        impl ::std::convert::From<super::UserPreference> for UserPreference {
            fn from(value: super::UserPreference) -> Self {
                Self {
                    accounts: Ok(value.accounts),
                    offers: Ok(value.offers),
                    streamer_info: Ok(value.streamer_info),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct UserPreferenceAccount {
            account_color: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            account_number: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            auto_position_effect: ::std::result::Result<bool, ::std::string::String>,
            display_acct_id: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            nick_name: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            primary_account: ::std::result::Result<bool, ::std::string::String>,
            type_: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for UserPreferenceAccount {
            fn default() -> Self {
                Self {
                    account_color: Ok(Default::default()),
                    account_number: Ok(Default::default()),
                    auto_position_effect: Ok(Default::default()),
                    display_acct_id: Ok(Default::default()),
                    nick_name: Ok(Default::default()),
                    primary_account: Ok(Default::default()),
                    type_: Ok(Default::default()),
                }
            }
        }

        impl UserPreferenceAccount {
            pub fn account_color<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.account_color = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for account_color: {e}"));
                self
            }
            pub fn account_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.account_number = value.try_into().map_err(|e| {
                    format!("error converting supplied value for account_number: {e}")
                });
                self
            }
            pub fn auto_position_effect<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.auto_position_effect = value.try_into().map_err(|e| {
                    format!("error converting supplied value for auto_position_effect: {e}")
                });
                self
            }
            pub fn display_acct_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.display_acct_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for display_acct_id: {e}")
                });
                self
            }
            pub fn nick_name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.nick_name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for nick_name: {e}"));
                self
            }
            pub fn primary_account<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.primary_account = value.try_into().map_err(|e| {
                    format!("error converting supplied value for primary_account: {e}")
                });
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {e}"));
                self
            }
        }

        impl ::std::convert::TryFrom<UserPreferenceAccount> for super::UserPreferenceAccount {
            type Error = super::error::ConversionError;
            fn try_from(
                value: UserPreferenceAccount,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    account_color: value.account_color?,
                    account_number: value.account_number?,
                    auto_position_effect: value.auto_position_effect?,
                    display_acct_id: value.display_acct_id?,
                    nick_name: value.nick_name?,
                    primary_account: value.primary_account?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::UserPreferenceAccount> for UserPreferenceAccount {
            fn from(value: super::UserPreferenceAccount) -> Self {
                Self {
                    account_color: Ok(value.account_color),
                    account_number: Ok(value.account_number),
                    auto_position_effect: Ok(value.auto_position_effect),
                    display_acct_id: Ok(value.display_acct_id),
                    nick_name: Ok(value.nick_name),
                    primary_account: Ok(value.primary_account),
                    type_: Ok(value.type_),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
///Client for Trader API - Account Access and User Preferences
///
///Schwab Trader API access to Account, Order entry and User Preferences
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
    ///Get list of account numbers and their encrypted values
    ///
    ///Account numbers in plain text cannot be used outside of headers or
    /// request/response bodies. As the first step consumers must invoke this
    /// service to retrieve the list of plain text/encrypted value pairs, and
    /// use encrypted account values for all subsequent calls for any
    /// accountNumber request.
    ///
    ///Sends a `GET` request to `/accounts/accountNumbers`
    ///
    ///```ignore
    /// let response = client.get_account_numbers()
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_account_numbers(&self) -> builder::GetAccountNumbers<'_> {
        builder::GetAccountNumbers::new(self)
    }

    ///Get linked account(s) balances and positions for the logged in user
    ///
    ///All the linked account information for the user logged in. The
    ///balances on these accounts are displayed by default however the
    /// positions on these accounts will be displayed based on the
    /// "positions" flag.
    ///
    ///Sends a `GET` request to `/accounts`
    ///
    ///Arguments:
    /// - `fields`: This allows one to determine which fields they want
    ///   returned. Possible value in this String can be:
    ///<br><code>positions</code><br> Example:<br><code>fields=positions</code>
    ///```ignore
    /// let response = client.get_accounts()
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_accounts(&self) -> builder::GetAccounts<'_> {
        builder::GetAccounts::new(self)
    }

    ///Get a specific account balance and positions for the logged in user
    ///
    ///Specific account information with balances and positions.
    ///The balance information on these accounts is displayed by default but
    ///Positions will be returned based on the "positions" flag.
    ///
    ///Sends a `GET` request to `/accounts/{accountNumber}`
    ///
    ///Arguments:
    /// - `account_number`: The encrypted ID of the account
    /// - `fields`: This allows one to determine
    ///which fields they want returned. Possible values in this String can be:
    ///<br><code>positions</code><br> Example:<br><code>fields=positions</code>
    ///```ignore
    /// let response = client.get_account()
    ///    .account_number(account_number)
    ///    .fields(fields)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_account(&self) -> builder::GetAccount<'_> {
        builder::GetAccount::new(self)
    }

    ///Get all orders for a specific account
    ///
    ///All orders for a specific account. Orders retrieved can be filtered
    /// based on input parameters below. Maximum date range is 1 year.
    ///
    ///Sends a `GET` request to `/accounts/{accountNumber}/orders`
    ///
    ///Arguments:
    /// - `account_number`: The encrypted ID of the account
    /// - `from_entered_time`: Specifies that no orders entered before this time
    ///   should be returned.
    ///Valid ISO-8601 formats are :<br> <code>yyyy-MM-dd'T'HH:mm:ss.SSSZ</code>
    /// Example fromEnteredTime is '2024-03-29T00:00:00.000Z'.
    /// 'toEnteredTime' must also be set.
    /// - `max_results`: The max number of orders to retrieve. Default is 3000.
    /// - `status`: Specifies that only orders of this status should be
    ///   returned.
    /// - `to_entered_time`: Specifies that no orders entered after this time
    ///   should be returned.Valid
    ///ISO-8601 formats are :<br> <code>yyyy-MM-dd'T'HH:mm:ss.SSSZ</code>.
    /// Example toEnteredTime is '2024-04-28T23:59:59.000Z'.
    /// 'fromEnteredTime' must also be set.
    ///```ignore
    /// let response = client.get_orders_by_path_param()
    ///    .account_number(account_number)
    ///    .from_entered_time(from_entered_time)
    ///    .max_results(max_results)
    ///    .status(status)
    ///    .to_entered_time(to_entered_time)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_orders_by_path_param(&self) -> builder::GetOrdersByPathParam<'_> {
        builder::GetOrdersByPathParam::new(self)
    }

    ///Place order for a specific account
    ///
    ///Place an order for a specific account.
    ///
    ///Sends a `POST` request to `/accounts/{accountNumber}/orders`
    ///
    ///Arguments:
    /// - `account_number`: The encrypted ID of the account
    /// - `body`: The new Order Object.
    ///```ignore
    /// let response = client.place_order()
    ///    .account_number(account_number)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn place_order(&self) -> builder::PlaceOrder<'_> {
        builder::PlaceOrder::new(self)
    }

    ///Get a specific order by its ID, for a specific account
    ///
    ///Get a specific order by its ID, for a specific account
    ///
    ///Sends a `GET` request to `/accounts/{accountNumber}/orders/{orderId}`
    ///
    ///Arguments:
    /// - `account_number`: The encrypted ID of the account
    /// - `order_id`: The ID of the order being retrieved.
    ///```ignore
    /// let response = client.get_order()
    ///    .account_number(account_number)
    ///    .order_id(order_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_order(&self) -> builder::GetOrder<'_> {
        builder::GetOrder::new(self)
    }

    ///Replace order for a specific account
    ///
    ///Replace an existing order for an account. The existing order will be
    /// replaced by the new               order. Once replaced, the old order
    /// will be canceled and a new order will be created.
    ///
    ///Sends a `PUT` request to `/accounts/{accountNumber}/orders/{orderId}`
    ///
    ///Arguments:
    /// - `account_number`: The encrypted ID of the account
    /// - `order_id`: The ID of the order being retrieved.
    /// - `body`: The Order Object.
    ///```ignore
    /// let response = client.replace_order()
    ///    .account_number(account_number)
    ///    .order_id(order_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn replace_order(&self) -> builder::ReplaceOrder<'_> {
        builder::ReplaceOrder::new(self)
    }

    ///Cancel an order for a specific account
    ///
    ///Cancel a specific order for a specific account<br>
    ///
    ///Sends a `DELETE` request to `/accounts/{accountNumber}/orders/{orderId}`
    ///
    ///Arguments:
    /// - `account_number`: The encrypted ID of the account
    /// - `order_id`: The ID of the order being cancelled
    ///```ignore
    /// let response = client.cancel_order()
    ///    .account_number(account_number)
    ///    .order_id(order_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn cancel_order(&self) -> builder::CancelOrder<'_> {
        builder::CancelOrder::new(self)
    }

    ///Get all orders for all accounts
    ///
    ///Get all orders for all accounts<br>
    ///
    ///Sends a `GET` request to `/orders`
    ///
    ///Arguments:
    /// - `from_entered_time`: Specifies that no orders entered before this time
    ///   should be returned. Valid ISO-8601 formats are-
    ///yyyy-MM-dd'T'HH:mm:ss.SSSZ Date must be within 60 days from today's
    /// date. 'toEnteredTime' must also be set.
    /// - `max_results`: The max number of orders to retrieve. Default is 3000.
    /// - `status`: Specifies that only orders of this status should be
    ///   returned.
    /// - `to_entered_time`: Specifies that no orders entered after this time
    ///   should be returned.Valid ISO-8601 formats are -
    ///yyyy-MM-dd'T'HH:mm:ss.SSSZ. 'fromEnteredTime' must also be set.
    ///```ignore
    /// let response = client.get_orders_by_query_param()
    ///    .from_entered_time(from_entered_time)
    ///    .max_results(max_results)
    ///    .status(status)
    ///    .to_entered_time(to_entered_time)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_orders_by_query_param(&self) -> builder::GetOrdersByQueryParam<'_> {
        builder::GetOrdersByQueryParam::new(self)
    }

    ///Preview order for a specific account
    ///
    ///Preview an order for a specific account.
    ///
    ///Sends a `POST` request to `/accounts/{accountNumber}/previewOrder`
    ///
    ///Arguments:
    /// - `account_number`: The encrypted ID of the account
    /// - `body`: The Order Object.
    ///```ignore
    /// let response = client.preview_order()
    ///    .account_number(account_number)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn preview_order(&self) -> builder::PreviewOrder<'_> {
        builder::PreviewOrder::new(self)
    }

    ///Get all transactions information for a specific account
    ///
    ///All transactions for a specific account. Maximum number of transactions
    /// in response is 3000. Maximum date range is 1 year.
    ///
    ///Sends a `GET` request to `/accounts/{accountNumber}/transactions`
    ///
    ///Arguments:
    /// - `account_number`: The encrypted ID of the account
    /// - `end_date`: Specifies that no transactions entered after this time
    ///   should be returned.Valid
    ///ISO-8601 formats are :<br> <code>yyyy-MM-dd'T'HH:mm:ss.SSSZ</code>.
    /// Example start date is '2024-05-10T21:10:42.000Z'. The 'startDate'
    /// must also be set.
    /// - `start_date`: Specifies that no transactions entered before this time
    ///   should be returned.
    ///Valid ISO-8601 formats are :<br> <code>yyyy-MM-dd'T'HH:mm:ss.SSSZ</code>
    /// .  Example start date is '2024-03-28T21:10:42.000Z'. The 'endDate' must
    /// also be set.
    /// - `symbol`: It filters all the transaction activities based on the
    ///   symbol specified. <u>NOTE:</u> If there is any special character in
    ///   the symbol, please send th encoded value.
    /// - `types`: Specifies that only transactions of this status should be
    ///   returned.
    ///```ignore
    /// let response = client.get_transactions_by_path_param()
    ///    .account_number(account_number)
    ///    .end_date(end_date)
    ///    .start_date(start_date)
    ///    .symbol(symbol)
    ///    .types(types)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_transactions_by_path_param(&self) -> builder::GetTransactionsByPathParam<'_> {
        builder::GetTransactionsByPathParam::new(self)
    }

    ///Get specific transaction information for a specific account
    ///
    ///Get specific transaction information for a specific account
    ///
    ///Sends a `GET` request to
    /// `/accounts/{accountNumber}/transactions/{transactionId}`
    ///
    ///Arguments:
    /// - `account_number`: The encrypted ID of the account
    /// - `transaction_id`: The ID of the transaction being retrieved.
    ///```ignore
    /// let response = client.get_transactions_by_id()
    ///    .account_number(account_number)
    ///    .transaction_id(transaction_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_transactions_by_id(&self) -> builder::GetTransactionsById<'_> {
        builder::GetTransactionsById::new(self)
    }

    ///Get user preference information for the logged in user
    ///
    ///Get user preference information for the logged in user.
    ///
    ///Sends a `GET` request to `/userPreference`
    ///
    ///```ignore
    /// let response = client.get_user_preference()
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_user_preference(&self) -> builder::GetUserPreference<'_> {
        builder::GetUserPreference::new(self)
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
    ///Builder for [`Client::get_account_numbers`]
    ///
    ///[`Client::get_account_numbers`]: super::Client::get_account_numbers
    #[derive(Debug, Clone)]
    pub struct GetAccountNumbers<'a> {
        client: &'a super::Client,
    }

    impl<'a> GetAccountNumbers<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        ///Sends a `GET` request to `/accounts/accountNumbers`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<::std::vec::Vec<types::AccountNumberHash>>,
            Error<types::ServiceError>,
        > {
            let Self { client } = self;
            let url = format!("{}/accounts/accountNumbers", client.baseurl,);
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
                operation_id: "get_account_numbers",
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
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                503u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_accounts`]
    ///
    ///[`Client::get_accounts`]: super::Client::get_accounts
    #[derive(Debug, Clone)]
    pub struct GetAccounts<'a> {
        client: &'a super::Client,
        fields: Result<Option<::std::string::String>, String>,
    }

    impl<'a> GetAccounts<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                fields: Ok(None),
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

        ///Sends a `GET` request to `/accounts`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::Account>>, Error<types::ServiceError>>
        {
            let Self { client, fields } = self;
            let fields = fields.map_err(Error::InvalidRequest)?;
            let url = format!("{}/accounts", client.baseurl,);
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
                operation_id: "get_accounts",
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
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                503u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_account`]
    ///
    ///[`Client::get_account`]: super::Client::get_account
    #[derive(Debug, Clone)]
    pub struct GetAccount<'a> {
        client: &'a super::Client,
        account_number: Result<::std::string::String, String>,
        fields: Result<Option<::std::string::String>, String>,
    }

    impl<'a> GetAccount<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                account_number: Err("account_number was not initialized".to_string()),
                fields: Ok(None),
            }
        }

        pub fn account_number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.account_number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for account_number failed".to_string()
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

        ///Sends a `GET` request to `/accounts/{accountNumber}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::Account>, Error<types::ServiceError>> {
            let Self {
                client,
                account_number,
                fields,
            } = self;
            let account_number = account_number.map_err(Error::InvalidRequest)?;
            let fields = fields.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/accounts/{}",
                client.baseurl,
                encode_path(&account_number.to_string()),
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
                operation_id: "get_account",
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
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                503u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_orders_by_path_param`]
    ///
    ///[`Client::get_orders_by_path_param`]: super::Client::get_orders_by_path_param
    #[derive(Debug, Clone)]
    pub struct GetOrdersByPathParam<'a> {
        client: &'a super::Client,
        account_number: Result<::std::string::String, String>,
        from_entered_time: Result<::std::string::String, String>,
        max_results: Result<Option<i64>, String>,
        status: Result<Option<types::ApiOrderStatus>, String>,
        to_entered_time: Result<::std::string::String, String>,
    }

    impl<'a> GetOrdersByPathParam<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                account_number: Err("account_number was not initialized".to_string()),
                from_entered_time: Err("from_entered_time was not initialized".to_string()),
                max_results: Ok(None),
                status: Ok(None),
                to_entered_time: Err("to_entered_time was not initialized".to_string()),
            }
        }

        pub fn account_number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.account_number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for account_number failed".to_string()
            });
            self
        }

        pub fn from_entered_time<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.from_entered_time = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for from_entered_time failed"
                    .to_string()
            });
            self
        }

        pub fn max_results<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.max_results = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for max_results failed".to_string());
            self
        }

        pub fn status<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiOrderStatus>,
        {
            self.status = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `ApiOrderStatus` for status failed".to_string());
            self
        }

        pub fn to_entered_time<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.to_entered_time = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for to_entered_time failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/accounts/{accountNumber}/orders`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::Order>>, Error<types::ServiceError>>
        {
            let Self {
                client,
                account_number,
                from_entered_time,
                max_results,
                status,
                to_entered_time,
            } = self;
            let account_number = account_number.map_err(Error::InvalidRequest)?;
            let from_entered_time = from_entered_time.map_err(Error::InvalidRequest)?;
            let max_results = max_results.map_err(Error::InvalidRequest)?;
            let status = status.map_err(Error::InvalidRequest)?;
            let to_entered_time = to_entered_time.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/accounts/{}/orders",
                client.baseurl,
                encode_path(&account_number.to_string()),
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
                .query(&progenitor_client::QueryParam::new(
                    "fromEnteredTime",
                    &from_entered_time,
                ))
                .query(&progenitor_client::QueryParam::new(
                    "maxResults",
                    &max_results,
                ))
                .query(&progenitor_client::QueryParam::new("status", &status))
                .query(&progenitor_client::QueryParam::new(
                    "toEnteredTime",
                    &to_entered_time,
                ))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_orders_by_path_param",
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
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                503u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::place_order`]
    ///
    ///[`Client::place_order`]: super::Client::place_order
    #[derive(Debug, Clone)]
    pub struct PlaceOrder<'a> {
        client: &'a super::Client,
        account_number: Result<::std::string::String, String>,
        body: Result<types::builder::OrderRequest, String>,
    }

    impl<'a> PlaceOrder<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                account_number: Err("account_number was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn account_number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.account_number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for account_number failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::OrderRequest>,
            <V as std::convert::TryInto<types::OrderRequest>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `OrderRequest` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::OrderRequest) -> types::builder::OrderRequest,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/accounts/{accountNumber}/orders`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::ServiceError>> {
            let Self {
                client,
                account_number,
                body,
            } = self;
            let account_number = account_number.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::OrderRequest::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/accounts/{}/orders",
                client.baseurl,
                encode_path(&account_number.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "place_order",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                201u16 => Ok(ResponseValue::empty(response)),
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                503u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_order`]
    ///
    ///[`Client::get_order`]: super::Client::get_order
    #[derive(Debug, Clone)]
    pub struct GetOrder<'a> {
        client: &'a super::Client,
        account_number: Result<::std::string::String, String>,
        order_id: Result<i64, String>,
    }

    impl<'a> GetOrder<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                account_number: Err("account_number was not initialized".to_string()),
                order_id: Err("order_id was not initialized".to_string()),
            }
        }

        pub fn account_number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.account_number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for account_number failed".to_string()
            });
            self
        }

        pub fn order_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.order_id = value
                .try_into()
                .map_err(|_| "conversion to `i64` for order_id failed".to_string());
            self
        }

        ///Sends a `GET` request to
        /// `/accounts/{accountNumber}/orders/{orderId}`
        pub async fn send(self) -> Result<ResponseValue<types::Order>, Error<types::ServiceError>> {
            let Self {
                client,
                account_number,
                order_id,
            } = self;
            let account_number = account_number.map_err(Error::InvalidRequest)?;
            let order_id = order_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/accounts/{}/orders/{}",
                client.baseurl,
                encode_path(&account_number.to_string()),
                encode_path(&order_id.to_string()),
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
                operation_id: "get_order",
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
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                503u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::replace_order`]
    ///
    ///[`Client::replace_order`]: super::Client::replace_order
    #[derive(Debug, Clone)]
    pub struct ReplaceOrder<'a> {
        client: &'a super::Client,
        account_number: Result<::std::string::String, String>,
        order_id: Result<i64, String>,
        body: Result<types::builder::OrderRequest, String>,
    }

    impl<'a> ReplaceOrder<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                account_number: Err("account_number was not initialized".to_string()),
                order_id: Err("order_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn account_number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.account_number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for account_number failed".to_string()
            });
            self
        }

        pub fn order_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.order_id = value
                .try_into()
                .map_err(|_| "conversion to `i64` for order_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::OrderRequest>,
            <V as std::convert::TryInto<types::OrderRequest>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `OrderRequest` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::OrderRequest) -> types::builder::OrderRequest,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `PUT` request to
        /// `/accounts/{accountNumber}/orders/{orderId}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::ServiceError>> {
            let Self {
                client,
                account_number,
                order_id,
                body,
            } = self;
            let account_number = account_number.map_err(Error::InvalidRequest)?;
            let order_id = order_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::OrderRequest::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/accounts/{}/orders/{}",
                client.baseurl,
                encode_path(&account_number.to_string()),
                encode_path(&order_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .put(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "replace_order",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                201u16 => Ok(ResponseValue::empty(response)),
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                503u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::cancel_order`]
    ///
    ///[`Client::cancel_order`]: super::Client::cancel_order
    #[derive(Debug, Clone)]
    pub struct CancelOrder<'a> {
        client: &'a super::Client,
        account_number: Result<::std::string::String, String>,
        order_id: Result<i64, String>,
    }

    impl<'a> CancelOrder<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                account_number: Err("account_number was not initialized".to_string()),
                order_id: Err("order_id was not initialized".to_string()),
            }
        }

        pub fn account_number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.account_number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for account_number failed".to_string()
            });
            self
        }

        pub fn order_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.order_id = value
                .try_into()
                .map_err(|_| "conversion to `i64` for order_id failed".to_string());
            self
        }

        ///Sends a `DELETE` request to
        /// `/accounts/{accountNumber}/orders/{orderId}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::ServiceError>> {
            let Self {
                client,
                account_number,
                order_id,
            } = self;
            let account_number = account_number.map_err(Error::InvalidRequest)?;
            let order_id = order_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/accounts/{}/orders/{}",
                client.baseurl,
                encode_path(&account_number.to_string()),
                encode_path(&order_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "cancel_order",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200u16 => Ok(ResponseValue::empty(response)),
                400u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                401u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                503u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_orders_by_query_param`]
    ///
    ///[`Client::get_orders_by_query_param`]: super::Client::get_orders_by_query_param
    #[derive(Debug, Clone)]
    pub struct GetOrdersByQueryParam<'a> {
        client: &'a super::Client,
        from_entered_time: Result<::std::string::String, String>,
        max_results: Result<Option<i64>, String>,
        status: Result<Option<types::ApiOrderStatus>, String>,
        to_entered_time: Result<::std::string::String, String>,
    }

    impl<'a> GetOrdersByQueryParam<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                from_entered_time: Err("from_entered_time was not initialized".to_string()),
                max_results: Ok(None),
                status: Ok(None),
                to_entered_time: Err("to_entered_time was not initialized".to_string()),
            }
        }

        pub fn from_entered_time<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.from_entered_time = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for from_entered_time failed"
                    .to_string()
            });
            self
        }

        pub fn max_results<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.max_results = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for max_results failed".to_string());
            self
        }

        pub fn status<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiOrderStatus>,
        {
            self.status = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `ApiOrderStatus` for status failed".to_string());
            self
        }

        pub fn to_entered_time<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.to_entered_time = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for to_entered_time failed".to_string()
            });
            self
        }

        ///Sends a `GET` request to `/orders`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::Order>>, Error<types::ServiceError>>
        {
            let Self {
                client,
                from_entered_time,
                max_results,
                status,
                to_entered_time,
            } = self;
            let from_entered_time = from_entered_time.map_err(Error::InvalidRequest)?;
            let max_results = max_results.map_err(Error::InvalidRequest)?;
            let status = status.map_err(Error::InvalidRequest)?;
            let to_entered_time = to_entered_time.map_err(Error::InvalidRequest)?;
            let url = format!("{}/orders", client.baseurl,);
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
                    "fromEnteredTime",
                    &from_entered_time,
                ))
                .query(&progenitor_client::QueryParam::new(
                    "maxResults",
                    &max_results,
                ))
                .query(&progenitor_client::QueryParam::new("status", &status))
                .query(&progenitor_client::QueryParam::new(
                    "toEnteredTime",
                    &to_entered_time,
                ))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_orders_by_query_param",
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
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                503u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::preview_order`]
    ///
    ///[`Client::preview_order`]: super::Client::preview_order
    #[derive(Debug, Clone)]
    pub struct PreviewOrder<'a> {
        client: &'a super::Client,
        account_number: Result<::std::string::String, String>,
        body: Result<types::builder::PreviewOrder, String>,
    }

    impl<'a> PreviewOrder<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                account_number: Err("account_number was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn account_number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.account_number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for account_number failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::PreviewOrder>,
            <V as std::convert::TryInto<types::PreviewOrder>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `PreviewOrder` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::PreviewOrder) -> types::builder::PreviewOrder,
        {
            self.body = self.body.map(f);
            self
        }

        ///Sends a `POST` request to `/accounts/{accountNumber}/previewOrder`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::PreviewOrder>, Error<types::ServiceError>> {
            let Self {
                client,
                account_number,
                body,
            } = self;
            let account_number = account_number.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::PreviewOrder::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/accounts/{}/previewOrder",
                client.baseurl,
                encode_path(&account_number.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "preview_order",
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
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                503u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_transactions_by_path_param`]
    ///
    ///[`Client::get_transactions_by_path_param`]: super::Client::get_transactions_by_path_param
    #[derive(Debug, Clone)]
    pub struct GetTransactionsByPathParam<'a> {
        client: &'a super::Client,
        account_number: Result<::std::string::String, String>,
        end_date: Result<::std::string::String, String>,
        start_date: Result<::std::string::String, String>,
        symbol: Result<Option<::std::string::String>, String>,
        types: Result<types::TransactionType, String>,
    }

    impl<'a> GetTransactionsByPathParam<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                account_number: Err("account_number was not initialized".to_string()),
                end_date: Err("end_date was not initialized".to_string()),
                start_date: Err("start_date was not initialized".to_string()),
                symbol: Ok(None),
                types: Err("types was not initialized".to_string()),
            }
        }

        pub fn account_number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.account_number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for account_number failed".to_string()
            });
            self
        }

        pub fn end_date<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.end_date = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for end_date failed".to_string()
            });
            self
        }

        pub fn start_date<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.start_date = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for start_date failed".to_string()
            });
            self
        }

        pub fn symbol<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.symbol = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for symbol failed".to_string()
            });
            self
        }

        pub fn types<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TransactionType>,
        {
            self.types = value
                .try_into()
                .map_err(|_| "conversion to `TransactionType` for types failed".to_string());
            self
        }

        ///Sends a `GET` request to `/accounts/{accountNumber}/transactions`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::Transaction>>, Error<types::ServiceError>>
        {
            let Self {
                client,
                account_number,
                end_date,
                start_date,
                symbol,
                types,
            } = self;
            let account_number = account_number.map_err(Error::InvalidRequest)?;
            let end_date = end_date.map_err(Error::InvalidRequest)?;
            let start_date = start_date.map_err(Error::InvalidRequest)?;
            let symbol = symbol.map_err(Error::InvalidRequest)?;
            let types = types.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/accounts/{}/transactions",
                client.baseurl,
                encode_path(&account_number.to_string()),
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
                .query(&progenitor_client::QueryParam::new("endDate", &end_date))
                .query(&progenitor_client::QueryParam::new(
                    "startDate",
                    &start_date,
                ))
                .query(&progenitor_client::QueryParam::new("symbol", &symbol))
                .query(&progenitor_client::QueryParam::new("types", &types))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_transactions_by_path_param",
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
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                503u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_transactions_by_id`]
    ///
    ///[`Client::get_transactions_by_id`]: super::Client::get_transactions_by_id
    #[derive(Debug, Clone)]
    pub struct GetTransactionsById<'a> {
        client: &'a super::Client,
        account_number: Result<::std::string::String, String>,
        transaction_id: Result<i64, String>,
    }

    impl<'a> GetTransactionsById<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                account_number: Err("account_number was not initialized".to_string()),
                transaction_id: Err("transaction_id was not initialized".to_string()),
            }
        }

        pub fn account_number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.account_number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for account_number failed".to_string()
            });
            self
        }

        pub fn transaction_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.transaction_id = value
                .try_into()
                .map_err(|_| "conversion to `i64` for transaction_id failed".to_string());
            self
        }

        ///Sends a `GET` request to
        /// `/accounts/{accountNumber}/transactions/{transactionId}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::Transaction>>, Error<types::ServiceError>>
        {
            let Self {
                client,
                account_number,
                transaction_id,
            } = self;
            let account_number = account_number.map_err(Error::InvalidRequest)?;
            let transaction_id = transaction_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/accounts/{}/transactions/{}",
                client.baseurl,
                encode_path(&account_number.to_string()),
                encode_path(&transaction_id.to_string()),
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
                operation_id: "get_transactions_by_id",
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
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                503u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    ///Builder for [`Client::get_user_preference`]
    ///
    ///[`Client::get_user_preference`]: super::Client::get_user_preference
    #[derive(Debug, Clone)]
    pub struct GetUserPreference<'a> {
        client: &'a super::Client,
    }

    impl<'a> GetUserPreference<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        ///Sends a `GET` request to `/userPreference`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::UserPreference>, Error<types::ServiceError>> {
            let Self { client } = self;
            let url = format!("{}/userPreference", client.baseurl,);
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
                operation_id: "get_user_preference",
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
                403u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                404u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                503u16 => Err(Error::ErrorResponse(
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
