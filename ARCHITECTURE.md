# Architecture

## Components

```
run()                          Startup: auth, load lot store, spawn DataService + TUI
  |
  +---> DataService            Backend: all I/O, data fetching, lot management
  |       |
  |       +---> SchwabStream   WebSocket: connect, login, subscribe, yield events
  |
  +---> run_tui(AppState)      Frontend: render loop, keyboard input, apply messages
```

**DataService** (`src/schwab/backend.rs`) — the backend. Owns all I/O and data management:
- REST calls (positions, orders, quotes, transactions)
- Lot store (apply transactions, seed from positions, persist to disk)
- Command execution (place/cancel/replace orders, instrument search)
- Delegates streaming to SchwabStream

**SchwabStream** (`src/schwab/schwab_stream.rs`) — the WebSocket layer:
- Connects to Schwab streamer, handles login handshake
- Subscribe/unsubscribe to LEVELONE_EQUITIES and ACCT_ACTIVITY
- Yields typed `StreamEvent`s (equity ticks, account activity)
- Consumes heartbeats and response frames internally

**AppState** (`src/app/mod.rs`) — the frontend state. Pure rendering model:
- Positions, orders, quotes, account info, lot data (read-only)
- UI state: current page, selection indices, filters, command mode
- Symbol DB for autocomplete (loaded from static TSVs at startup)
- `apply(DataMsg)` updates state from backend messages — no logic, just assignment

## Communication

```
DataService ----DataMsg----> AppState.apply()
    ^                            |
    |                            |
    +-------AppCommand-----------+
```

Two `mpsc` channels, no shared state:
- `DataMsg` (backend -> frontend): positions, quotes, orders, lots, errors, status
- `AppCommand` (frontend -> backend): subscribe, search, place/cancel/replace orders

## Startup Sequence

```
1. Authenticate with Schwab API
2. Load lot store from disk (tax_lots.json)
3. Spawn DataService with lot store
4. DataService connects SchwabStream (WebSocket + login)
5. DataService fetches initial data via REST:
   a. Transactions (last 7 days) -> apply to lot store
   b. Positions -> send to TUI, seed lot store for untracked symbols
   c. Orders -> send to TUI
   d. Quotes -> send to TUI, seed local cache
6. Subscribe LEVELONE_EQUITIES for position + order symbols
7. Subscribe ACCT_ACTIVITY for account events
8. Enter event loop
```

## Event Loop (DataService.run_once)

Two event sources via `tokio::select!`:

**Stream events** (from SchwabStream):
- `EquityTicks` — update quotes cache, send `DataMsg::Quotes` to TUI
- `AccountActivity` — refresh orders; on fills, also refresh transactions -> lots -> positions. Update WebSocket subscriptions if symbol set changed.

**TUI commands** (from AppCommand channel):
- `SubscribeToQuote` / `ClearCmdSymbols` — manage temporary quote subscriptions
- `PlaceOrder` / `CancelOrder` / `ReplaceOrder` — execute via REST, send result
- `SearchInstruments` — spawned as background task, sends suggestions + index

## Reconnection

DataService.run() wraps run_once() in a loop. On disconnect or error, it waits 5s
and reconnects. The lot store persists across reconnects (lives on DataService).
SchwabStream is recreated each time (new WebSocket + login).

## Tax Lot Flow

Lot store lives in DataService. TUI gets a read-only copy via `DataMsg::Lots`.

**On startup / fill detected:**
```
1. Fetch transactions (last 7 days)
2. apply_transactions: deduplicate by activity_id, then:
   - Buys (Opening)  -> add_lot with real date/price
   - Sells (Closing) -> reduce_lots (LIFO); warns if no lots tracked for symbol
3. Fetch positions
4. seed_lots_from_positions: for each position where tracked qty < actual qty,
   add a catch-up lot at average cost. This backfills any symbols not covered
   by recent transactions.
5. Save to disk, send DataMsg::Lots to TUI
```

For partially tracked stores (some symbols have lots, others don't):
- Buys for untracked symbols create real lots (accurate date/price)
- Sells for untracked symbols no-op with a warning
- seed_from_positions fills the gaps using position average cost
- Final quantities always match actual positions

## Symbol Tracking (DataService)

DataService maintains three symbol sets for WebSocket subscription management:
- `pos_symbols` — symbols from current positions
- `ord_symbols` — symbols from recent orders (capped at 20)
- `cmd_symbols` — temporary symbols from user's command input

The union of all three is subscribed to LEVELONE_EQUITIES. When the set changes
(e.g. after a fill adds a new position), subscriptions are updated.
