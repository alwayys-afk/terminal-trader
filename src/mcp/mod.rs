//! `terminal-trader mcp`: a local MCP server over stdio that gives an AI
//! client read-only access to the user's Schwab accounts.
//!
//! stdout carries the MCP protocol and nothing else: logs go to `trader.log`
//! in the data directory, the browser is opened with its output discarded,
//! and nothing on this path may `print!`. Accounts are shown by nickname only
//! (see [`accounts`]).

pub mod accounts;
pub mod chain;
pub mod format;
pub mod login;
pub mod server;

use anyhow::Result;
use rmcp::ServiceExt;
use tracing::info;

use crate::schwab::SchwabAuth;
pub use server::TraderServer;

/// The Schwab auth for the configured app, or a message saying what is
/// missing. Never fails outright, so the server can still start and report it.
fn auth_from_env() -> Result<SchwabAuth, String> {
    let key = std::env::var("SCHWAB_API_KEY")
        .ok()
        .filter(|v| !v.is_empty());
    let secret = std::env::var("SCHWAB_APP_SECRET")
        .ok()
        .filter(|v| !v.is_empty());
    match (key, secret) {
        (Some(key), Some(secret)) => {
            Ok(SchwabAuth::new(key, secret, crate::data_dir::token_path()))
        }
        _ => Err(
            "terminal-trader isn't set up: SCHWAB_API_KEY and SCHWAB_APP_SECRET must be \
                  set in the .env file in terminal-trader's data directory (see the README). \
                  Ask the user to set them up; never ask them to paste the app secret here."
                .to_string(),
        ),
    }
}

/// Serve MCP on stdin/stdout until the client disconnects. Expects the data
/// directory, `.env` and file logging to be set up already.
pub async fn serve_stdio() -> Result<()> {
    info!("MCP server starting");
    let server = TraderServer::new(auth_from_env());
    let running = server.serve(rmcp::transport::stdio()).await?;
    let reason = running.waiting().await?;
    info!(?reason, "MCP server stopped");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::accounts::tests::{HASH_A, NUM_A, directory};
    use super::format;
    use crate::schwab::{AccountBalances, AssetType, Position};

    /// Every formatter's output, with fixture data that smuggles the fake
    /// account number and hash into free-text fields, must come out clean
    /// after the server's redaction pass.
    #[test]
    fn tool_output_never_contains_account_numbers_or_hashes() {
        let dir = directory();
        let label = &dir.resolve(Some("Roth")).unwrap()[0].label;
        let position = Position {
            symbol: format!("LEAK{NUM_A}"),
            quantity: 1.0,
            average_cost: 1.0,
            current_price: 1.0,
            market_value: 1.0,
            day_pnl: 0.0,
            day_pnl_pct: 0.0,
            open_pnl: 0.0,
            asset_type: AssetType::Equity,
            previous_session_qty: 1.0,
            multiplier: 1.0,
        };
        let outputs = [
            format::positions(label, &[position]),
            format::balances(label, &AccountBalances::default()),
            format::orders(label, &[], "2026-11-01..2026-11-15"),
            format::lots(label, &[], Some(HASH_A)),
            // An HTTP error quoting the request URL.
            format!(
                "Schwab request failed: error sending request for url \
                 (https://api.schwabapi.com/trader/v1/accounts/{HASH_A}?fields=positions)"
            ),
        ];
        for text in outputs {
            let clean = dir.redact(&text);
            assert!(!clean.contains(NUM_A), "account number leaked: {clean}");
            assert!(!clean.contains(HASH_A), "account hash leaked: {clean}");
        }
    }

    /// stdout is the MCP protocol. Nothing the server can reach may print to
    /// it (the TUI's login and `--seed` print, but the server never calls
    /// them).
    #[test]
    fn mcp_code_paths_never_print_to_stdout() {
        let root = env!("CARGO_MANIFEST_DIR");
        let files = [
            "src/mcp/mod.rs",
            "src/mcp/accounts.rs",
            "src/mcp/chain.rs",
            "src/mcp/format.rs",
            "src/mcp/login.rs",
            "src/mcp/server.rs",
            "src/schwab/auth.rs",
            "src/schwab/auth/callback.rs",
            "src/schwab/auth/cert.rs",
            "src/schwab/auth/store.rs",
            "src/schwab/client.rs",
            "src/schwab/models.rs",
            "src/schwab/options.rs",
            "src/app/lots.rs",
            "src/data_dir.rs",
        ];
        let banned = ["print!(", "println!(", "dbg!(", "stdout()", "open::that"];
        for file in files {
            let src = std::fs::read_to_string(format!("{root}/{file}")).unwrap();
            // Tests may print; only check the code before the top-level
            // test module.
            let code_part = src.split("\n#[cfg(test)]\n").next().unwrap_or("");
            for (n, line) in code_part.lines().enumerate() {
                let code = line.split("//").next().unwrap_or("");
                for b in banned {
                    // rmcp's stdio transport is the one allowed stdout user.
                    if b == "stdout()" && code.contains("rmcp::transport::stdio()") {
                        continue;
                    }
                    assert!(
                        !code.contains(b) || code.contains("eprint"),
                        "{file}:{} uses {b}: {line}",
                        n + 1
                    );
                }
            }
        }
    }
}
