When running stuff in rust it's possible that you might get 500+ lines of dependency compilation stuff so be wary of that and use tools like grep or tail to make sure we don't read in that whole thing.

Try to avoid looking things up on the internet, the api code should have what we need.

Favor using the generated api sdk instead of reinventing the wheel. Whenever we use a new endpoint write a test for it and run it to make sure it returns what we think it will. It's fine to test on the account with small amounts. Like one share of $HOG. If it doesn't, update the sdk code to be correct and update the SCHWAB_API_ISSUES doc.

Never await blocking operations (REST API calls, file I/O) inline in the main `select!` event loop in `backend.rs`. Spawn them as background tasks and send results back through the `BackgroundResult` channel or the TUI `DataMsg` channel, so WebSocket ticks keep flowing.

This is a live **public** repo. Never publish PII (account numbers/hashes, OAuth tokens, real balances, real holdings, dollar amounts, raw API responses, account CSV exports) to the remote, an issue, or a (draft) PR. The obvious secret files are already gitignored; the real risk is account data that ends up somewhere gitignore doesn't catch: pasted into an issue/PR, hardcoded into code, committed as a new file, or left unscrubbed in a test fixture.

Private runtime files (`.env`, tokens, tax lots, prefs, `trader.log`) live in the data directory, `~/Library/Application Support/terminal-trader` (or `$TERMINAL_TRADER_DATA_DIR`), never in the repo; any new file holding account data goes through `src/data_dir.rs`. `.githooks/pre-push` blocks pushes that add private files, dollar amounts of four figures or more, 64-hex hashes or 8+ digit numbers (generated SDKs excepted; mark a safe line with `pii-ok`). Enable it in every clone with `git config core.hooksPath .githooks`; bypass only with `git push --no-verify` when sure.
