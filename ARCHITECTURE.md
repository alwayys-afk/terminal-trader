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

## Data Directory

`src/data_dir.rs` is the one place that decides where private runtime files
live: `$TERMINAL_TRADER_DATA_DIR`, else `~/Library/Application Support/terminal-trader`
(created 0700 at startup). `.env`, the OAuth token file, per-account tax lots,
`app_prefs.json` and `trader.log` are all resolved through it; nothing is read
from or written to the working directory except the bundled `stocks.tsv` /
`etfs.tsv` symbol lists.

## Authentication

`src/schwab/auth.rs` (`SchwabAuth`) owns the OAuth token file and never
touches stdin or stdout, so the TUI and the MCP server share it.

- `get_valid_token()` returns the saved token, refreshing the access token
  when it's within 60s of expiring. It never prompts: with no saved login, an
  expired refresh token (Schwab's last 7 days) or a refresh Schwab rejects, it
  returns `AuthError::LoginRequired`. Network errors and 5xx/429 responses
  return `AuthError::Transient` and leave the token file alone.
- Refreshes run under a cross-process lock on `<token file>.lock`. After
  taking it, the token is re-read; if another process already refreshed, that
  token is used instead of calling Schwab again. Saves write a 0600 temp file
  and rename it into place.
- `login(on_url)` runs the browser login: it starts an HTTPS listener on
  `127.0.0.1:8182` (`auth/callback.rs`) with a self-signed certificate kept in
  the data dir (`auth/cert.rs`, regenerated if missing or broken), hands the
  authorize URL to `on_url`, waits (5 min default) for the redirect with the
  matching `state`, exchanges the code and saves the token. Unrelated requests
  (favicon) are ignored; a mismatched `state` gets an error page and the
  listener keeps waiting. The token lock is only taken around the save, never
  across the browser wait.
- The terminal side lives in `src/app/login.rs`: at startup (and for `--seed`)
  `ensure_logged_in` prints the URL, opens the browser and explains the
  certificate warning, before the TUI takes over the terminal. If port 8182 is
  busy it falls back to pasting the redirect URL.
- Mid-session, `DataService` treats `LoginRequired` as "tell the user to
  restart and log in" and re-checks the token file every 60s (another process
  may have logged in meanwhile); it never prompts.

## MCP Server

`terminal-trader mcp` (`src/mcp/`) is a separate entry point: `main.rs`
dispatches to it before anything touches the terminal, sets up file logging,
and serves MCP on stdio with `rmcp`. It never starts the TUI or `DataService`;
each tool call makes its REST calls directly through a lazily built
`SchwabClient` (awaiting inline is fine here, there's no event loop to block).

- `server.rs`: the tools (read-only; there is no order tool). Auth errors
  become a tool error telling the model to call `login`.
- `login.rs`: runs `SchwabAuth::login` in a background task so `login` can
  return the URL immediately. URL elicitation is used when the client
  advertises it: a server-to-client `elicitation/create` on older protocols,
  an `InputRequiredResult` (MRTR) round trip on 2026-07-28.
- `accounts.rs`: nickname labels for accounts and the redaction pass that
  scrubs account numbers and hashes from every result.
- `chain.rs` / `format.rs`: filters and compact CSV output.

stdout is the protocol, so nothing reachable from the server may print;
`mcp::tests::mcp_code_paths_never_print_to_stdout` guards the files involved,
and the browser opener runs with its output discarded.

## Startup Sequence

```
1. Authenticate with Schwab API (browser login first if there's no usable saved login)
2. Load lot store from disk (<data dir>/tax_lots_<account hash>.json)
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
