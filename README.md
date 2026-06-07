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

- Rust 1.85+ (2024 edition)
- A [Schwab developer](https://developer.schwab.com) account with an app that
  has the **Trader API** and **Market Data API** products enabled, and a
  callback (redirect) URL set to `https://127.0.0.1:8182`. Creating and
  configuring the app is the fiddliest part of setup; the
  [schwab-py getting-started guide](https://schwab-py.readthedocs.io/en/latest/getting-started.html) walks through the Schwab-side steps pretty well.

### Setup

```sh
git clone https://github.com/alwayys-afk/terminal-trader.git
cd terminal-trader
cp .env.example .env       # then fill in SCHWAB_API_KEY and SCHWAB_APP_SECRET
```

### Run

```sh
cargo run --release
```

On first launch the app opens your browser to complete the Schwab OAuth flow,
then caches the tokens locally (in `schwab_tokens.json` by default) and refreshes
them automatically on subsequent runs.

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
