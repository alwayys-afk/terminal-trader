# Options Compatibility Audit (Epic #11, Phase 1)

Audit of every code path for equity-only assumptions, with the fix/defer
decision for each. Scope for this phase: single-leg calls/puts, chain view,
LEVELONE_OPTIONS streaming. Options are **excluded from tax-lot tracking** for
now (options P&L is a future phase of #11).

An option position is identified by `AssetType::Option` and keyed by its OSI
symbol — underlying padded to 6 chars, `YYMMDD`, `C`/`P`, strike ×1000 in 8
digits (e.g. `AAPL  260619C00150000`). Two properties drive most findings:
the symbol **contains spaces**, and premiums carry a **×100 contract
multiplier**.

## Findings

### F1. `Position` conversion: `current_price` is 100× for options — FIX
`Position::try_from` (`src/schwab/models.rs`) computes
`current_price = market_value / quantity`. For options, `market_value`
already includes the ×100 multiplier, so `current_price` comes out as 100×
the per-contract premium. `pos_metrics` (`src/app/mod.rs`) then mixes scales:
with a live quote it shows the true premium; without one it falls back to the
100× `current_price`. Fix: divide by the multiplier for option positions so
`current_price` is always the premium.

### F2. Day-P&L multiplier handling — ALREADY DONE (verify)
`pos_metrics`/`compute_day_pnl` already apply ×100 for option positions when
a streamed quote exists. Correct as long as F1 makes the no-quote fallback
consistent. `day_pnl_pct` comes from the quote's `net_percent_change`
(premium-relative), same convention as equities.

### F3. OSI symbols render raw in every table — FIX
Positions table (`Symbol` column is 10 wide), orders table, order dialogs,
and status strings all print `order.symbol`/`pos.symbol` verbatim. A raw OSI
symbol is 21 chars with embedded spaces. Fix: shared compact formatter
(e.g. `AAPL 7/17 215C`, year included when not the current year) used by all
display paths; widen the symbol columns.

### F4. Command grammar cannot express option symbols — FIX (design)
The command parser (`src/tui/command.rs`) tokenizes on whitespace, so an OSI
symbol can never round-trip through it. All prefill paths
(`open_position_order` for b/s/f on a position, `open_order_dialog` for
e/o/r on an order) build `"{verb} {symbol} {qty}"` strings that mis-parse for
options today. Fix: contract-spec tokens (`215c` strike+side, plus an
expiration token: explicit date, `lotto`, `weekly`, `monthly`,
`next week`, `<month> monthly`); prefill emits the compact spec; the OSI
symbol is derived deterministically from the resolved spec.

### F5. Lot store ingests option trades with 100×-off economics — FIX (exclude)
`TaxLotStore::apply_transaction` (`src/app/lots.rs`) accepts any Trade
transaction's transfer items; `transaction_instrument_symbol` happily returns
OSI symbols for `TransactionOption` items. Option lots would carry contract
quantities and per-contract prices — proceeds/realized P&L off by 100×, and
sales joined to `order_methods` that don't apply to options. Decision:
**exclude** option transfer items from lot ingestion (and from the sell-leg
tax-method machinery). The account-P&L panel, lot detail page, and operations
timeline all read the lot store, so exclusion keeps them consistent.
Position-based seeding no longer exists, so ingestion is the only entry point.

### F6. Backend hardcodes `InstrumentSpec::Equity` on order placement — FIX
`AppCommand::PlaceOrder`/`ReplaceOrder` handling in `src/schwab/backend.rs`
builds `InstrumentSpec::Equity { symbol }` unconditionally, and maps
buy/sell to equity `Instruction::Buy`/`Sell`. Options need
`InstrumentSpec::Option { osi_symbol }` and to-open/to-close instructions
(buy → `BuyToClose` when covering a short, else `BuyToOpen`; sell →
`SellToClose` when long, else `SellToOpen`). `client.place_order` already
handles the rest (leg type, position effect, NORMAL session).

### F7. Tax-lot method must not be sent on option orders — FIX
Sells auto-attach a `tax_lot_method` and may split into LT/ST legs
(`compute_sell_legs`). With options excluded from lots this must short-circuit:
no method, no splitting, single leg.

### F8. Streamer subscribes everything to LEVELONE_EQUITIES — FIX (port)
`DataService` keeps one symbol set; option symbols from positions/orders would
be subscribed to the equities feed, which ignores them → no live option
quotes. Fix: port the `options-streaming` archive branch (feed enum,
`LevelOneOptionTick`, per-feed symbol partitioning, option tick → quote cache
merge). Extend it to command symbols (detect OSI format) so the chain view and
order previews get live option ticks too; the branch left cmd symbols
equity-only.

### F9. Quote plumbing is already option-safe — NO CHANGE
`get_quotes` parses `OptionResponse` into `Quote`, and the shared quote cache
is keyed by symbol string, OSI or not. Mid/bid/ask price modifiers work off
`Quote` fields, so option limit prices need no special handling (options tick
in $0.01 for premiums; the existing `round_price` to cents is valid).

### F10. Autocomplete / instrument search are underlying-oriented — NO CHANGE
`SearchInstruments` and the TSV symbol DB only know underlyings. That's the
desired UX: the user types the underlying and a contract spec; nothing
searches OSI strings. Guard: don't fire instrument search for tokens that
parse as contract specs.

### F11. Percent amounts and dollar amounts for options — FIX (small)
`Amount::Dollars` sizing divides by the per-contract price; for options the
true notional is ×100, so `$500` would buy 100× too many contracts.
`Amount::Percent` works off `held_qty` (contracts) and is fine. Fix: use
premium ×100 in contract-count math and in the `(~$…)` estimate lines.

### F12. Order-symbol subscription cap and account activity — NO CHANGE
`fetch_orders` symbol collection and the ACCT_ACTIVITY refresh path are
symbol-agnostic; with F8's feed partitioning they route correctly. Fill
detection re-fetches positions/orders the same way for options.

### F13. `Order` conversion reads only the first leg — DEFER (Phase 3)
`Order::try_from` takes the first order leg. Fine for single-leg equity and
option orders; multi-leg strategies (Phase 3, needs #22) will need a legs
model.

### F14. Chain data has no staleness marker — DEFER
`OptionChainSnapshot` is a point-in-time REST snapshot. The chain view
overlays live streamed ticks for subscribed contracts; a fetched-at timestamp
in the view is enough for now.

## Decisions
- Options excluded from tax lots, account P&L panel, and lot detail (F5, F7).
- Chain view + typed grammar both supported; both resolve to a contract spec,
  and the OSI symbol is constructed from the spec (no chain round-trip needed
  for explicit strike+date entry).
- Relative expiration tokens (`lotto`, `weekly`, `monthly`, `next week`,
  `<month> monthly`) resolve against a cached chain fetch for the underlying.
- Naked `SellToOpen` is allowed but the confirm screen labels it OPEN so
  selling without a position is never silent (F6).
