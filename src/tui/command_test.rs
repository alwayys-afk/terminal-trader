use super::command::*;

// ── ListNav ─────────────────────────────────────────────────────────

struct TestList {
    items: Vec<i32>,
    selected_idx: usize,
}

impl TestList {
    fn new(n: usize) -> Self {
        Self {
            items: (0..n as i32).collect(),
            selected_idx: 0,
        }
    }
}

impl ListNav for TestList {
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

#[test]
fn listnav_select_next_at_last_stays() {
    let mut list = TestList::new(3);
    list.selected_idx = 2;
    list.select_next();
    assert_eq!(list.selected_idx(), 2);
}

#[test]
fn listnav_select_next_empty_stays_zero() {
    let mut list = TestList::new(0);
    list.select_next();
    assert_eq!(list.selected_idx(), 0);
}

#[test]
fn listnav_select_prev_at_zero_stays() {
    let mut list = TestList::new(3);
    list.select_prev();
    assert_eq!(list.selected_idx(), 0);
}

#[test]
fn listnav_select_last_empty_stays_zero() {
    let mut list = TestList::new(0);
    list.select_last();
    assert_eq!(list.selected_idx(), 0);
}

#[test]
fn listnav_clamp_after_shrink() {
    let mut list = TestList::new(5);
    list.selected_idx = 4;
    list.items.truncate(2);
    list.clamp_selection();
    assert_eq!(list.selected_idx(), 1);
}

#[test]
fn listnav_clamp_empty_resets_to_zero() {
    let mut list = TestList::new(3);
    list.selected_idx = 2;
    list.items.clear();
    list.clamp_selection();
    assert_eq!(list.selected_idx(), 0);
}

#[test]
fn autocomplete_set_suggestions_clamps_idx() {
    let mut ac = Autocomplete::default();
    ac.set_suggestions(vec![
        ("A".into(), "a".into()),
        ("B".into(), "b".into()),
        ("C".into(), "c".into()),
    ]);
    ac.select_next();
    ac.select_next(); // idx = 2
    // Shrink: idx should clamp from 2 to 1
    ac.set_suggestions(vec![("A".into(), "a".into()), ("B".into(), "b".into())]);
    assert_eq!(ac.selected_idx(), 1);
}

#[test]
fn autocomplete_clear_resets_idx() {
    let mut ac = Autocomplete::default();
    ac.set_suggestions(vec![("A".into(), "a".into()), ("B".into(), "b".into())]);
    ac.select_next();
    ac.clear();
    assert_eq!(ac.selected_idx(), 0);
    assert!(ac.selected_item().is_none());
}

// ── command_box_display ──────────────────────────────────────────────

#[test]
fn display_nothing_when_no_suggestions_and_before_amount() {
    let parsed = parse_command("buy AA");
    assert_eq!(
        command_box_display("buy AA", 6, &parsed, false),
        CommandBoxDisplay::Nothing,
    );
}

#[test]
fn display_suggestions_when_typing_symbol() {
    let parsed = parse_command("buy AA");
    assert_eq!(
        command_box_display("buy AA", 6, &parsed, true),
        CommandBoxDisplay::Suggestions,
    );
}

#[test]
fn display_preview_after_symbol_confirmed() {
    // User typed "buy AAPL " — cursor past symbol, focus is Amount
    let input = "buy AAPL ";
    let parsed = parse_command(input);
    assert_eq!(
        command_box_display(input, input.len(), &parsed, false),
        CommandBoxDisplay::QuotePreview,
    );
}

#[test]
fn display_preview_with_modifier() {
    let input = "buy AAPL 10 market";
    let parsed = parse_command(input);
    assert_eq!(
        command_box_display(input, input.len(), &parsed, false),
        CommandBoxDisplay::QuotePreview,
    );
}

#[test]
fn display_preview_beats_suggestions_when_past_symbol() {
    // Even if suggestions exist, once we're past the symbol we show preview
    let input = "buy AAPL 10";
    let parsed = parse_command(input);
    assert_eq!(
        command_box_display(input, input.len(), &parsed, true),
        CommandBoxDisplay::QuotePreview,
    );
}

#[test]
fn display_suggestions_when_cursor_on_symbol_despite_trailing_tokens() {
    // Cursor moved back to the symbol token
    let input = "buy AAPL 10";
    let parsed = parse_command(input);
    assert_eq!(
        command_box_display(input, 6, &parsed, true),
        CommandBoxDisplay::Suggestions,
    );
}

#[test]
fn display_nothing_on_invalid_parse_past_symbol() {
    // Invalid amount — parse fails, so no preview even though focus is Amount
    let input = "buy AAPL abc";
    let parsed = parse_command(input);
    assert!(matches!(parsed, ParseResult::Invalid(_)));
    assert_eq!(
        command_box_display(input, input.len(), &parsed, false),
        CommandBoxDisplay::Nothing,
    );
}

// ── token_focus ──────────────────────────────────────────────────────

#[test]
fn focus_empty_input() {
    assert_eq!(token_focus("", 0), TokenFocus::Verb);
}

#[test]
fn focus_typing_verb() {
    assert_eq!(token_focus("bu", 2), TokenFocus::Verb);
}

#[test]
fn focus_after_verb_space() {
    assert_eq!(token_focus("buy ", 4), TokenFocus::Symbol);
}

#[test]
fn focus_after_symbol_space() {
    assert_eq!(token_focus("buy AAPL ", 9), TokenFocus::Amount);
}

#[test]
fn focus_typing_amount() {
    assert_eq!(token_focus("buy AAPL 10", 11), TokenFocus::Amount);
}

#[test]
fn focus_after_amount_space() {
    assert_eq!(token_focus("buy AAPL 10 ", 12), TokenFocus::Modifier);
}

#[test]
fn focus_typing_modifier() {
    assert_eq!(token_focus("buy AAPL 10 mar", 15), TokenFocus::Modifier);
}

#[test]
fn focus_backspace_removes_trailing_space() {
    // "buy AAPL " -> "buy AAPL" should snap back to Symbol
    assert_eq!(token_focus("buy AAPL", 8), TokenFocus::Symbol);
}

#[test]
fn focus_cursor_mid_input() {
    // Cursor in the middle of "buy" even though more text follows
    assert_eq!(token_focus("buy AAPL 10", 2), TokenFocus::Verb);
}

#[test]
fn focus_cursor_at_space_between_tokens() {
    // Cursor right at the space after "buy"
    assert_eq!(token_focus("buy AAPL 10", 4), TokenFocus::Symbol);
}

// ── parse_command ────────────────────────────────────────────────────

#[test]
fn parse_empty() {
    assert!(matches!(parse_command(""), ParseResult::Incomplete));
}

#[test]
fn parse_verb_only() {
    assert!(matches!(parse_command("buy"), ParseResult::Incomplete));
}

#[test]
fn parse_verb_prefix() {
    // "s" only matches sell
    let result = parse_command("s AAPL");
    assert!(matches!(result, ParseResult::Valid(ref cmd) if cmd.verb == Verb::Sell));
}

#[test]
fn parse_verb_symbol() {
    let result = parse_command("buy AAPL");
    let ParseResult::Valid(cmd) = result else {
        panic!("expected Valid");
    };
    assert_eq!(cmd.verb, Verb::Buy);
    assert_eq!(cmd.symbol, "AAPL");
    assert!(cmd.amount.is_none());
    assert_eq!(cmd.modifier, Modifier::Mid);
}

#[test]
fn parse_symbol_uppercased() {
    let ParseResult::Valid(cmd) = parse_command("buy aapl") else {
        panic!("expected Valid");
    };
    assert_eq!(cmd.symbol, "AAPL");
}

#[test]
fn parse_shares_amount() {
    let ParseResult::Valid(cmd) = parse_command("buy AAPL 10") else {
        panic!("expected Valid");
    };
    assert_eq!(cmd.amount, Some(Amount::Shares(10.0)));
}

#[test]
fn parse_dollar_amount() {
    let ParseResult::Valid(cmd) = parse_command("buy AAPL $500") else {
        panic!("expected Valid");
    };
    assert_eq!(cmd.amount, Some(Amount::Dollars(500.0)));
}

#[test]
fn parse_percent_sell() {
    let ParseResult::Valid(cmd) = parse_command("sell AAPL 50%") else {
        panic!("expected Valid");
    };
    assert_eq!(cmd.amount, Some(Amount::Percent(50.0)));
}

#[test]
fn parse_percent_buy_invalid() {
    assert!(matches!(
        parse_command("buy AAPL 50%"),
        ParseResult::Invalid(_)
    ));
}

#[test]
fn parse_modifier_market() {
    let ParseResult::Valid(cmd) = parse_command("buy AAPL 10 market") else {
        panic!("expected Valid");
    };
    assert_eq!(cmd.modifier, Modifier::Market);
}

#[test]
fn parse_modifier_market_prefix() {
    let ParseResult::Valid(cmd) = parse_command("buy AAPL 10 ma") else {
        panic!("expected Valid");
    };
    assert_eq!(cmd.modifier, Modifier::Market);
}

#[test]
fn parse_modifier_mid() {
    let ParseResult::Valid(cmd) = parse_command("buy AAPL 10 mid") else {
        panic!("expected Valid");
    };
    assert_eq!(cmd.modifier, Modifier::Mid);
}

#[test]
fn parse_modifier_m_ambiguous() {
    assert!(matches!(
        parse_command("buy AAPL 10 m"),
        ParseResult::Ambiguous(_)
    ));
}

#[test]
fn parse_modifier_limit_price() {
    let ParseResult::Valid(cmd) = parse_command("buy AAPL 10 150.50") else {
        panic!("expected Valid");
    };
    assert_eq!(cmd.modifier, Modifier::Limit(150.50));
}

#[test]
fn parse_modifier_ask() {
    let ParseResult::Valid(cmd) = parse_command("sell AAPL 10 ask") else {
        panic!("expected Valid");
    };
    assert_eq!(cmd.modifier, Modifier::Ask);
}

#[test]
fn parse_modifier_bid() {
    let ParseResult::Valid(cmd) = parse_command("sell AAPL 10 bid") else {
        panic!("expected Valid");
    };
    assert_eq!(cmd.modifier, Modifier::Bid);
}

#[test]
fn parse_modifier_now_buy_resolves_to_ask() {
    let ParseResult::Valid(cmd) = parse_command("buy AAPL 10 now") else {
        panic!("expected Valid");
    };
    assert_eq!(cmd.modifier, Modifier::Ask);
}

#[test]
fn parse_modifier_now_sell_resolves_to_bid() {
    let ParseResult::Valid(cmd) = parse_command("sell AAPL 10 n") else {
        panic!("expected Valid");
    };
    assert_eq!(cmd.modifier, Modifier::Bid);
}

#[test]
fn parse_unknown_verb() {
    assert!(matches!(
        parse_command("foobar AAPL"),
        ParseResult::Invalid(_)
    ));
}

#[test]
fn parse_unknown_modifier() {
    assert!(matches!(
        parse_command("buy AAPL 10 foobar"),
        ParseResult::Invalid(_)
    ));
}

#[test]
fn parse_invalid_amount() {
    assert!(matches!(
        parse_command("buy AAPL abc"),
        ParseResult::Invalid(_)
    ));
}
