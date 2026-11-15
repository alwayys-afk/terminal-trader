# terminal-trader

_Because trading is a terminal condition._

A fast, keyboard-driven terminal (TUI) trading client for the
[Charles Schwab](https://www.schwab.com) brokerage API, written in Rust.

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
![Built with Rust](https://img.shields.io/badge/built%20with-Rust-orange.svg)

## Features

- **Live streaming quotes** — positions and working orders update in real time
  over the Schwab WebSocket streamer (no polling).
- **Positions view** — day and open P&L, computed client-side to work around
  quirks in the API's reported values.
- **Fast order entry** — a vim-style command bar (`:`) to place, cancel, and
  replace equity and single-leg option orders.
- **Tax-lot tracking** — local FIFO/LIFO lot accounting with realized and
  unrealized P&L, since the API doesn't report lot-level dispositions. Cost
  basis can be seeded by importing a Schwab CSV transaction export.
- **Account P&L** — aggregate short-/long-term realized P&L broken out by year.
- **Multi-account** — switch between accounts; configurable per-account
  default lot method.
- **MCP server** — `terminal-trader mcp` gives Claude (or any MCP client)
  read-only access to your accounts, quotes and option chains. See
  [MCP server](#mcp-server).

## Architecture

A two-task design (backend I/O service + TUI render loop) communicating over
channels, with a thin wrapper around two generated API clients. See
[ARCHITECTURE.md](ARCHITECTURE.md) for the full breakdown.

```
terminal-trader/        # workspace root + TUI application crate
  schwab-trader/        # generated Rust client — Schwab Trader API
  schwab-marketdata/    # generated Rust client — Schwab Market Data API
```

## Getting started

### Prerequisites

- Rust 1.89+ (2024 edition)
- A [Schwab developer](https://developer.schwab.com) account with an app that
  has the **Trader API** and **Market Data API** products enabled, and a
  callback (redirect) URL set to `https://127.0.0.1:8182`. Creating and
  configuring the app is the fiddliest part of setup; the
  [schwab-py getting-started guide](https://schwab-py.readthedocs.io/en/latest/getting-started.html) walks through the Schwab-side steps pretty well.

### Setup

```sh
git clone https://github.com/alwayys-afk/terminal-trader.git
cd terminal-trader
git config core.hooksPath .githooks   # pre-push check for account data
mkdir -p "$HOME/Library/Application Support/terminal-trader"
cp .env.example "$HOME/Library/Application Support/terminal-trader/.env"   # then fill in SCHWAB_API_KEY and SCHWAB_APP_SECRET
```

All private runtime files live in a data directory **outside the repo**:
`~/Library/Application Support/terminal-trader` (override with `TERMINAL_TRADER_DATA_DIR`). That's
`.env`, `schwab_tokens.json`, `tax_lots_<account hash>.json`, `app_prefs.json`
and `trader.log`. The app creates the directory (mode 0700) on first run. Keeping
them out of the working tree means there's nothing personal in the repo to
commit by accident.

### Run

```sh
cargo run --release
```

### Logging in to Schwab

On first launch (and whenever the saved login has expired) the app opens your
browser at Schwab's login page and waits for Schwab to send you back. It
receives that redirect itself on `https://127.0.0.1:8182`, so there's nothing
to copy or paste.

- **The browser will warn that the connection isn't private.** The local
  listener uses a self-signed certificate that the app generates and keeps in
  the data directory (`callback_cert.pem` / `callback_key.pem`). Click
  **Advanced → Proceed to 127.0.0.1**. Because the same certificate is reused,
  some browsers remember the exception.
- The app waits up to 5 minutes for the login. If port 8182 is already in use,
  it falls back to asking you to paste the address the browser ended up on.
- Tokens are saved in `schwab_tokens.json` in the data directory and refreshed
  automatically. **Schwab's refresh token lasts 7 days**, so expect to log in
  in the browser about once a week. If it expires while the app is running, the
  status line says so; quit and restart to log in again.
- Several processes can share the token file: refreshes take a lock
  (`schwab_tokens.json.lock`) and saves are atomic, so two copies of the app
  never refresh over each other.

## MCP server

`terminal-trader mcp` runs a local [MCP](https://modelcontextprotocol.io)
server over stdio, so an AI assistant can look at your accounts and help you
think through trades, especially option pricing. It's **read-only**: no tool
can place, change or cancel an order.

| Tool | Returns |
| --- | --- |
| `auth_status` | Whether there's a usable login, and when it expires |
| `login` | Runs the browser login (below) |
| `list_accounts` | Account nicknames |
| `get_positions` | Positions with day and open P&L |
| `get_balances` | Liquidation value, cash, buying power, margin figures |
| `get_orders` | Orders, filtered by status and date (last 7 days by default) |
| `get_quotes` | Quotes for up to 50 symbols |
| `get_lots` | Open tax lots with short/long-term status, days to long-term and unrealized gain |
| `get_expirations` | Listed option expirations |
| `get_option_chain` | A filtered chain (expiration range, strikes around the money, calls/puts) with IV, delta, gamma, theta, vega and Schwab's theoretical value |

Output is compact CSV, since all of it lands in the model's context. Chains
default to 45 days and 8 strikes each side of the money and are capped at 120
rows (a `TRUNCATED` line says how many more matched).

**Register it** with Claude Code (build first with `cargo build --release`):

```sh
claude mcp add terminal-trader -- /path/to/terminal-trader/target/release/terminal-trader mcp
```

or in Claude Desktop's `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "terminal-trader": {
      "command": "/path/to/terminal-trader/target/release/terminal-trader",
      "args": ["mcp"]
    }
  }
}
```

It uses the same `.env`, token file and lot files as the app, so set up and log
in as above first.

**Logging in.** The server never prompts on its own; tools say when a login is
needed, and the `login` tool runs the same browser login as the app (callback
on `https://127.0.0.1:8182`, same certificate warning, 5-minute window). If the
client supports MCP URL elicitation, it shows you the link and the tool waits
for you to finish. Otherwise the tool opens your browser, returns the link
right away, and the assistant calls `login` with `wait=true` once you say
you're done. The app and the server share the token file safely, but only one
login can listen on port 8182 at a time.

**Privacy.** Accounts appear only by their Schwab nickname (or "Account 1",
"Account 2", ... when none is set). Account numbers and hashes stay inside the
server, and every tool result is scrubbed for them before it's returned. Your
positions, balances and orders do go to the AI client you connect, so use one
you trust with that. Logs go to `trader.log`; stdout carries only the protocol.

Tax lots come from the lot files the app keeps; they're as current as the last
time the app synced (shown in the output). Options aren't tracked as lots.

Planned: setup help (`setup_status`, prefs), pricing tools (price history,
what-if chains, a multi-leg P&L and greeks calculator), and, off by default,
order preview and placement with per-order limits.

## Contributing

Contributions are welcome: issues, ideas, and PRs.

Just keep in mind this software places real orders against real money, so the
bar is correctness over volume:

- Understand what you're changing and why. If you can't explain the diff and
  how you tested it, it's not ready.
- Test against a real or paper account first, and note what you tested in the PR.
- Keep changes small and easy to review.

A Claude-written PR description is fine (they can be pretty good, and save you the
busywork of code references and formatting). But add a short **TL;DR in your own
words below it** with everything you'd put in the PR if you'd written it yourself:
why you made the change, what you changed and why, how you tested it, and anything
you hit along the way. Grammar doesn't matter; it just has to show you actually
drove the change instead of rubber-stamping it. (This very section ends with a
human TL;DR — that's the idea.)

Not sure if an idea fits? Open an issue first.

> **TL;DR:** Contributions are welcome and appreciated, but don't slop it up.
> This is real money, and I'm already good enough at losing it with functional
> trading software. I don't need slop helping me with that. Understand and test
> what you're changing, keep PRs small and easily reviewable, and write a TL;DR
> with your PR.

## Disclaimer

> ⚠️ **Unofficial and unaffiliated.** This project is not produced, endorsed, or
> supported by Charles Schwab. It places **real orders against real money**.
> There is no warranty of any kind. Use it at your own risk and review every
> order before you submit it.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. This applies to the application and to the generated
`schwab-trader` / `schwab-marketdata` SDK crates, which are permissively
licensed so they can be reused on their own.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
