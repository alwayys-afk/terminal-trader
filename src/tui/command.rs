use std::str::FromStr;

use crate::schwab::{ContractSpec, ExpirySpec, PutCall};
use crate::util::{PrefixParseError, prefix_match};

#[derive(Debug, Clone, PartialEq)]
pub enum Verb {
    Buy,
    Sell,
    /// Open the option chain page for a symbol (`chain aapl [filters…]`).
    Chain,
}

impl FromStr for Verb {
    type Err = PrefixParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const VERBS: &[(&str, Verb)] = &[
            ("buy", Verb::Buy),
            ("sell", Verb::Sell),
            ("chain", Verb::Chain),
        ];
        prefix_match(s, VERBS)
    }
}
impl std::fmt::Display for Verb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Verb::Buy => write!(f, "buy"),
            Verb::Sell => write!(f, "sell"),
            Verb::Chain => write!(f, "chain"),
        }
    }
}

/// A filter token on a `chain` command: expiry class, moneyness, or a
/// specific expiration date.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChainFilter {
    Weekly,
    Monthly,
    Atm,
    Itm,
    Otm,
    Date(ExpirySpec),
}

fn parse_chain_filter(tok: &str) -> Result<ChainFilter, PrefixParseError> {
    if let Some(spec) = parse_expiry_date(tok) {
        return Ok(ChainFilter::Date(spec));
    }
    prefix_match(
        &tok.to_lowercase(),
        &[
            ("weekly", ChainFilter::Weekly),
            ("monthly", ChainFilter::Monthly),
            ("atm", ChainFilter::Atm),
            ("itm", ChainFilter::Itm),
            ("otm", ChainFilter::Otm),
        ],
    )
}

#[derive(Debug, Clone, PartialEq)]
pub enum Amount {
    Shares(f64),
    Dollars(f64),
    /// Percentage of held position (e.g. 50% → sell half).
    Percent(f64),
}

impl FromStr for Amount {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = s.strip_prefix('$') {
            stripped
                .parse::<f64>()
                .map(Amount::Dollars)
                .map_err(|_| format!("Invalid dollar amount: {s}"))
        } else if let Some(stripped) = s.strip_suffix('%') {
            let v = stripped
                .parse::<f64>()
                .map_err(|_| format!("Invalid percent: {s}"))?;
            if (0.0..=100.0).contains(&v) {
                Ok(Amount::Percent(v))
            } else {
                Err("Percent must be 0-100".to_string())
            }
        } else {
            s.parse::<f64>()
                .map(Amount::Shares)
                .map_err(|_| format!("Invalid quantity: {s}"))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Modifier {
    /// Midpoint of bid/ask.
    #[default]
    Mid,
    Market,
    Limit(f64),
    /// Limit at the current ask (aggressive buy).
    Ask,
    /// Limit at the current bid (aggressive sell).
    Bid,
}

impl FromStr for Modifier {
    type Err = PrefixParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(price) = s.parse::<f64>() {
            return Ok(Modifier::Limit(price));
        }
        const MODS: &[(&str, Modifier)] = &[
            ("market", Modifier::Market),
            ("mid", Modifier::Mid),
            ("ask", Modifier::Ask),
            ("bid", Modifier::Bid),
        ];
        prefix_match(s, MODS)
    }
}

/// Parse a strike+side token like `215c` / `12.5p` (case-insensitive).
/// `None` means the token isn't contract-shaped, so the command stays in
/// equity form (`215` alone is a share count, never a strike).
fn parse_strike_side(tok: &str) -> Option<(f64, PutCall)> {
    let side = match tok.as_bytes().last()? {
        b'c' | b'C' => PutCall::Call,
        b'p' | b'P' => PutCall::Put,
        _ => return None,
    };
    let strike: f64 = tok[..tok.len() - 1].parse().ok()?;
    (strike > 0.0).then_some((strike, side))
}

const MONTHS: &[(&str, u32)] = &[
    ("january", 1),
    ("february", 2),
    ("march", 3),
    ("april", 4),
    ("may", 5),
    ("june", 6),
    ("july", 7),
    ("august", 8),
    ("september", 9),
    ("october", 10),
    ("november", 11),
    ("december", 12),
];

/// First token of an expiry. Keyword forms may need a follow-up token
/// (`next` → `week`, month name → `monthly`).
enum ExpiryHead {
    Complete(ExpirySpec),
    NeedsWeek,
    NeedsMonthly(u32),
}

/// Parse the head token of an expiration. Date forms are complete on their
/// own; keywords resolve by unique prefix.
fn parse_expiry_head(tok: &str) -> Result<ExpiryHead, PrefixParseError> {
    if let Some(spec) = parse_expiry_date(tok) {
        return Ok(ExpiryHead::Complete(spec));
    }
    #[derive(Clone)]
    enum Kw {
        Lotto,
        Weekly,
        Monthly,
        Next,
        Month(u32),
    }
    let mut options: Vec<(&str, Kw)> = vec![
        ("lotto", Kw::Lotto),
        ("weekly", Kw::Weekly),
        ("monthly", Kw::Monthly),
        ("next", Kw::Next),
    ];
    options.extend(MONTHS.iter().map(|&(name, n)| (name, Kw::Month(n))));
    match prefix_match(&tok.to_lowercase(), &options)? {
        Kw::Lotto => Ok(ExpiryHead::Complete(ExpirySpec::Lotto)),
        Kw::Weekly => Ok(ExpiryHead::Complete(ExpirySpec::Weekly)),
        Kw::Monthly => Ok(ExpiryHead::Complete(ExpirySpec::Monthly)),
        Kw::Next => Ok(ExpiryHead::NeedsWeek),
        Kw::Month(m) => Ok(ExpiryHead::NeedsMonthly(m)),
    }
}

/// Parse explicit date forms: `7/17`, `7/17/27`, `2027-07-17`.
fn parse_expiry_date(tok: &str) -> Option<ExpirySpec> {
    if let Ok(d) = chrono::NaiveDate::parse_from_str(tok, "%Y-%m-%d") {
        return Some(ExpirySpec::Date(d));
    }
    let parts: Vec<&str> = tok.split('/').collect();
    match parts[..] {
        [m, d] => {
            let (month, day) = (m.parse().ok()?, d.parse().ok()?);
            // Validate against a leap year so 2/29 stays parseable; which
            // year it lands in is decided at resolution time.
            chrono::NaiveDate::from_ymd_opt(2024, month, day)?;
            Some(ExpirySpec::MonthDay { month, day })
        }
        [m, d, y] => {
            let (month, day) = (m.parse().ok()?, d.parse().ok()?);
            let year: i32 = y.parse().ok()?;
            let year = if year < 100 { 2000 + year } else { year };
            chrono::NaiveDate::from_ymd_opt(year, month, day).map(ExpirySpec::Date)
        }
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct ParsedCommand {
    pub verb: Verb,
    pub symbol: String,
    /// Present when the command targets an option contract rather than the
    /// underlying itself.
    pub contract: Option<ContractSpec>,
    pub amount: Option<Amount>,
    pub modifier: Modifier,
    /// Filters on a `chain` command (empty for buy/sell).
    pub chain_filters: Vec<ChainFilter>,
}

#[derive(Debug, Clone)]
pub enum ParseResult {
    Incomplete,
    Valid(ParsedCommand),
    Ambiguous(Vec<String>),
    Invalid(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenFocus {
    Verb,
    Symbol,
    /// Option strike+side token (`215c`).
    Contract,
    /// Option expiration token(s) (`7/17`, `lotto`, `next week`, …).
    Expiry,
    Amount,
    Modifier,
}

pub trait ListNav {
    fn item_count(&self) -> usize;
    fn selected_idx(&self) -> usize;
    fn selected_idx_mut(&mut self) -> &mut usize;

    fn select_prev(&mut self) {
        *self.selected_idx_mut() = self.selected_idx().saturating_sub(1);
    }

    fn select_next(&mut self) {
        let count = self.item_count();
        let idx = self.selected_idx_mut();
        if count > 0 && *idx < count - 1 {
            *idx += 1;
        }
    }

    fn select_first(&mut self) {
        *self.selected_idx_mut() = 0;
    }

    fn select_last(&mut self) {
        let count = self.item_count();
        if count > 0 {
            *self.selected_idx_mut() = count - 1;
        }
    }

    fn clamp_selection(&mut self) {
        let count = self.item_count();
        let idx = self.selected_idx_mut();
        if count == 0 {
            *idx = 0;
        } else {
            *idx = (*idx).min(count - 1);
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Autocomplete {
    suggestions: Vec<(String, String)>,
    selected_idx: usize,
}

impl ListNav for Autocomplete {
    fn item_count(&self) -> usize {
        self.suggestions.len()
    }

    fn selected_idx(&self) -> usize {
        self.selected_idx
    }

    fn selected_idx_mut(&mut self) -> &mut usize {
        &mut self.selected_idx
    }
}

impl Autocomplete {
    pub fn suggestions(&self) -> &[(String, String)] {
        &self.suggestions
    }

    pub fn has_suggestions(&self) -> bool {
        !self.suggestions.is_empty()
    }

    pub fn clear(&mut self) {
        self.suggestions.clear();
        self.selected_idx = 0;
    }

    pub fn set_suggestions(&mut self, suggestions: Vec<(String, String)>) {
        self.suggestions = suggestions;
        self.clamp_selection();
    }

    pub fn extend_suggestions(&mut self, new: Vec<(String, String)>) {
        self.suggestions.extend(new);
        self.clamp_selection();
    }

    pub fn selected_item(&self) -> Option<&(String, String)> {
        self.suggestions.get(self.selected_idx)
    }
}

/// What the command box should display below the input line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandBoxDisplay {
    /// Nothing to show yet.
    Nothing,
    /// Show autocomplete suggestions for the current token.
    Suggestions,
    /// Show the quote/order preview for the confirmed symbol.
    QuotePreview,
}

/// Decide what the command box should display given the current state.
pub fn command_box_display(
    input: &str,
    cursor: usize,
    parsed: &ParseResult,
    has_suggestions: bool,
) -> CommandBoxDisplay {
    let focus = token_focus(input, cursor);
    if matches!(
        focus,
        TokenFocus::Contract | TokenFocus::Expiry | TokenFocus::Amount | TokenFocus::Modifier
    ) && matches!(parsed, ParseResult::Valid(_))
    {
        CommandBoxDisplay::QuotePreview
    } else if has_suggestions {
        CommandBoxDisplay::Suggestions
    } else {
        CommandBoxDisplay::Nothing
    }
}

/// Determine which token the cursor is currently in/starting.
pub fn token_focus(input: &str, cursor: usize) -> TokenFocus {
    let clamped = cursor.min(input.len());
    // Snap to nearest valid char boundary if cursor somehow lands mid-character.
    let clamped = if !input.is_char_boundary(clamped) {
        input[..clamped]
            .char_indices()
            .next_back()
            .map(|(i, _)| i)
            .unwrap_or(0)
    } else {
        clamped
    };
    let before_cursor = &input[..clamped];
    let token_count = before_cursor.split_whitespace().count();
    let at_boundary = before_cursor.ends_with(char::is_whitespace) || before_cursor.is_empty();

    let focus_idx = if at_boundary {
        token_count
    } else {
        token_count.saturating_sub(1)
    };

    // Option-form commands shift the token layout: contract and expiration
    // tokens sit between the symbol and the amount.
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let option_mode = tokens.len() > 2 && parse_strike_side(tokens[2]).is_some();
    if !option_mode {
        return match focus_idx {
            0 => TokenFocus::Verb,
            1 => TokenFocus::Symbol,
            2 => TokenFocus::Amount,
            _ => TokenFocus::Modifier,
        };
    }
    // Two-token expirations (`next week`, `july monthly`) push amount out by one.
    let expiry_tokens = match tokens.get(3).map(|t| parse_expiry_head(t)) {
        Some(Ok(ExpiryHead::NeedsWeek | ExpiryHead::NeedsMonthly(_))) => 2,
        _ => 1,
    };
    let amount_idx = 3 + expiry_tokens;
    match focus_idx {
        0 => TokenFocus::Verb,
        1 => TokenFocus::Symbol,
        2 => TokenFocus::Contract,
        i if i < amount_idx => TokenFocus::Expiry,
        i if i == amount_idx => TokenFocus::Amount,
        _ => TokenFocus::Modifier,
    }
}

#[derive(Debug, Clone)]
pub struct TypingState {
    pub input: String,
    /// Cursor position (byte offset into `input`).
    pub cursor: usize,
    pub parsed: ParseResult,
    /// Symbol we last sent FetchQuote for (to avoid duplicate stream subscriptions).
    pub last_fetched_sym: Option<String>,
    /// Symbol we last sent SearchInstruments for (to avoid duplicate searches).
    pub last_searched_sym: Option<String>,
    /// Underlying we last requested an option chain for (option commands).
    pub last_chain_sym: Option<String>,
    /// OSI symbol we last subscribed for the option preview.
    pub last_option_osi: Option<String>,
    pub autocomplete: Autocomplete,
    /// If Some, confirming will replace this order instead of placing a new one.
    pub replace_order_id: Option<i64>,
}

impl TypingState {
    pub fn new(replace_order_id: Option<i64>) -> Self {
        Self {
            input: String::new(),
            cursor: 0,
            parsed: ParseResult::Incomplete,
            last_fetched_sym: None,
            last_searched_sym: None,
            last_chain_sym: None,
            last_option_osi: None,
            autocomplete: Autocomplete::default(),
            replace_order_id,
        }
    }

    pub fn with_input(input: String, replace_order_id: Option<i64>) -> Self {
        let parsed = parse_command(&input);
        let cursor = input.len();
        Self {
            input,
            cursor,
            parsed,
            last_fetched_sym: None,
            last_searched_sym: None,
            last_chain_sym: None,
            last_option_osi: None,
            autocomplete: Autocomplete::default(),
            replace_order_id,
        }
    }

    pub fn set_input(&mut self, input: String) {
        self.cursor = input.len();
        self.parsed = parse_command(&input);
        self.input = input;
    }

    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.cursor, c);
        self.cursor += c.len_utf8();
        self.parsed = parse_command(&self.input);
    }

    /// Delete the character before cursor. Returns false if cursor was at 0.
    pub fn delete_back(&mut self) -> bool {
        if self.cursor == 0 {
            return false;
        }
        let delete_to = self.input[..self.cursor]
            .char_indices()
            .next_back()
            .map(|(i, _)| i)
            .unwrap_or(0);
        self.input.replace_range(delete_to..self.cursor, "");
        self.cursor = delete_to;
        self.parsed = parse_command(&self.input);
        true
    }

    pub fn parsed_symbol(&self) -> Option<String> {
        match &self.parsed {
            ParseResult::Valid(cmd) => Some(cmd.symbol.clone()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum CommandMode {
    #[default]
    Hidden,
    Typing(TypingState),
    Confirming {
        cmd: ParsedCommand,
        price: f64,
        /// Resolved contract for option commands, locked at confirm time so a
        /// chain refresh can't silently retarget a relative expiry.
        osi: Option<crate::schwab::OsiSymbol>,
        /// Original input string from the Typing state, preserved for editing back.
        input: String,
        /// If Some, this will replace an existing order instead of placing a new one.
        replace_order_id: Option<i64>,
    },
    OrderSent(String),
    /// Confirm cancelling the highlighted order.
    ConfirmCancel {
        order_id: i64,
        summary: String,
    },
}

pub fn parse_command(input: &str) -> ParseResult {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    if tokens.is_empty() {
        return ParseResult::Incomplete;
    }

    // Token 0: verb
    let verb = match tokens[0].parse::<Verb>() {
        Ok(v) => v,
        Err(PrefixParseError::Unknown(_)) => {
            return ParseResult::Invalid(format!("Unknown command: {}", tokens[0]));
        }
        Err(PrefixParseError::Ambiguous(cmds)) => return ParseResult::Ambiguous(cmds),
    };

    // Token 1: symbol
    if tokens.len() < 2 {
        return ParseResult::Incomplete;
    }
    let symbol = tokens[1].to_uppercase();

    // `chain <sym> [filters…]` — everything after the symbol is a filter.
    if verb == Verb::Chain {
        let mut chain_filters = Vec::new();
        for tok in &tokens[2..] {
            match parse_chain_filter(tok) {
                Ok(fil) => chain_filters.push(fil),
                Err(PrefixParseError::Unknown(t)) => {
                    return ParseResult::Invalid(format!("Unknown chain filter: {t}"));
                }
                Err(PrefixParseError::Ambiguous(cands)) => return ParseResult::Ambiguous(cands),
            }
        }
        return ParseResult::Valid(ParsedCommand {
            verb,
            symbol,
            contract: None,
            amount: None,
            modifier: Modifier::Mid,
            chain_filters,
        });
    }

    if tokens.len() == 2 {
        return ParseResult::Valid(ParsedCommand {
            verb,
            symbol,
            contract: None,
            amount: None,
            modifier: Modifier::Mid,
            chain_filters: Vec::new(),
        });
    }

    // Token 2 shaped like `215c` switches to option form: contract spec
    // (strike+side, expiry), then the usual amount/modifier.
    let (contract, mut idx) = match parse_strike_side(tokens[2]) {
        None => (None, 2),
        Some((strike, side)) => {
            let Some(head) = tokens.get(3) else {
                return ParseResult::Incomplete;
            };
            let (expiry, consumed) = match parse_expiry_head(head) {
                Ok(ExpiryHead::Complete(e)) => (e, 1),
                Ok(ExpiryHead::NeedsWeek) => match tokens.get(4) {
                    None => return ParseResult::Incomplete,
                    Some(t) if "week".starts_with(&t.to_lowercase()) => (ExpirySpec::NextWeek, 2),
                    Some(t) => {
                        return ParseResult::Invalid(format!("Expected 'week' after 'next': {t}"));
                    }
                },
                Ok(ExpiryHead::NeedsMonthly(m)) => match tokens.get(4) {
                    None => return ParseResult::Incomplete,
                    Some(t) if "monthly".starts_with(&t.to_lowercase()) => {
                        (ExpirySpec::MonthMonthly(m), 2)
                    }
                    Some(t) => {
                        return ParseResult::Invalid(format!(
                            "Expected 'monthly' after month name: {t}"
                        ));
                    }
                },
                Err(PrefixParseError::Unknown(t)) => {
                    return ParseResult::Invalid(format!("Unknown expiration: {t}"));
                }
                Err(PrefixParseError::Ambiguous(cands)) => return ParseResult::Ambiguous(cands),
            };
            (
                Some(ContractSpec {
                    strike,
                    side,
                    expiry,
                }),
                3 + consumed,
            )
        }
    };

    // Next token (optional): amount — shares for equities, contracts for options.
    let amount = if let Some(tok) = tokens.get(idx) {
        idx += 1;
        match tok.parse::<Amount>() {
            Ok(Amount::Percent(_)) if verb != Verb::Sell => {
                return ParseResult::Invalid("Percent amounts are only valid for sell".to_string());
            }
            Ok(a) => Some(a),
            Err(e) => return ParseResult::Invalid(e),
        }
    } else {
        None
    };

    // Final token (optional): modifier
    // "now" crosses the spread: resolve to ask (buys) or bid (sells).
    let modifier = if let Some(tok) = tokens.get(idx) {
        if "now".starts_with(tok) {
            match verb {
                Verb::Buy => Modifier::Ask,
                // Chain commands returned above; only sell remains.
                _ => Modifier::Bid,
            }
        } else {
            match tok.parse::<Modifier>() {
                Ok(m) => m,
                Err(PrefixParseError::Unknown(_)) => {
                    return ParseResult::Invalid(format!("Unknown modifier: {tok}"));
                }
                Err(PrefixParseError::Ambiguous(cmds)) => return ParseResult::Ambiguous(cmds),
            }
        }
    } else {
        Modifier::Mid
    };

    ParseResult::Valid(ParsedCommand {
        verb,
        symbol,
        contract,
        amount,
        modifier,
        chain_filters: Vec::new(),
    })
}
