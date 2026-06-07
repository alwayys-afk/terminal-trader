use std::str::FromStr;

use crate::util::{PrefixParseError, prefix_match};

#[derive(Debug, Clone, PartialEq)]
pub enum Verb {
    Buy,
    Sell,
}

impl FromStr for Verb {
    type Err = PrefixParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const VERBS: &[(&str, Verb)] = &[("buy", Verb::Buy), ("sell", Verb::Sell)];
        prefix_match(s, VERBS)
    }
}
impl std::fmt::Display for Verb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Verb::Buy => write!(f, "buy"),
            Verb::Sell => write!(f, "sell"),
        }
    }
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

#[derive(Debug, Clone)]
pub struct ParsedCommand {
    pub verb: Verb,
    pub symbol: String,
    pub amount: Option<Amount>,
    pub modifier: Modifier,
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
    if matches!(focus, TokenFocus::Amount | TokenFocus::Modifier)
        && matches!(parsed, ParseResult::Valid(_))
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
    match focus_idx {
        0 => TokenFocus::Verb,
        1 => TokenFocus::Symbol,
        2 => TokenFocus::Amount,
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

    if tokens.len() == 2 {
        return ParseResult::Valid(ParsedCommand {
            verb,
            symbol,
            amount: None,
            modifier: Modifier::Mid,
        });
    }

    // Token 2 (optional): amount
    let amount = if tokens.len() >= 3 {
        match tokens[2].parse::<Amount>() {
            Ok(Amount::Percent(_)) if verb != Verb::Sell => {
                return ParseResult::Invalid("Percent amounts are only valid for sell".to_string());
            }
            Ok(a) => Some(a),
            Err(e) => return ParseResult::Invalid(e),
        }
    } else {
        None
    };

    // Token 3 (optional): modifier
    // "now" crosses the spread: resolve to ask (buys) or bid (sells).
    let modifier = if tokens.len() >= 4 {
        if "now".starts_with(tokens[3]) {
            match verb {
                Verb::Buy => Modifier::Ask,
                Verb::Sell => Modifier::Bid,
            }
        } else {
            match tokens[3].parse::<Modifier>() {
                Ok(m) => m,
                Err(PrefixParseError::Unknown(_)) => {
                    return ParseResult::Invalid(format!("Unknown modifier: {}", tokens[3]));
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
        amount,
        modifier,
    })
}
