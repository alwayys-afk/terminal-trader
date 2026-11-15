//! End-to-end smoke test of `terminal-trader mcp` over stdio. Needs no
//! Schwab account: it runs against an empty data directory with no
//! credentials and checks the protocol, the tool list, and that stdout carries
//! nothing but JSON-RPC.

use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::mpsc;
use std::time::Duration;

use serde_json::{Value, json};

const EXPECTED_TOOLS: &[&str] = &[
    "auth_status",
    "login",
    "list_accounts",
    "get_positions",
    "get_balances",
    "get_orders",
    "get_quotes",
    "get_lots",
    "get_expirations",
    "get_option_chain",
];

/// A running `terminal-trader mcp` with line-based JSON-RPC helpers.
struct McpProcess {
    child: Child,
    stdin: ChildStdin,
    lines: mpsc::Receiver<String>,
    next_id: i64,
}

impl McpProcess {
    /// Spawn with `TERMINAL_TRADER_DATA_DIR` set to `data_dir`, or with the
    /// user's real data directory when `None`.
    fn spawn(data_dir: Option<&std::path::Path>) -> McpProcess {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_terminal-trader"));
        cmd.arg("mcp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        if let Some(dir) = data_dir {
            cmd.env("TERMINAL_TRADER_DATA_DIR", dir)
                .env_remove("SCHWAB_API_KEY")
                .env_remove("SCHWAB_APP_SECRET")
                .env_remove("SCHWAB_TOKEN_PATH");
        }
        let mut child = cmd.spawn().expect("spawn terminal-trader mcp");
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let (tx, lines) = mpsc::channel::<String>();
        std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines() {
                let Ok(line) = line else { break };
                if tx.send(line).is_err() {
                    break;
                }
            }
        });
        let mut p = McpProcess {
            child,
            stdin,
            lines,
            next_id: 1,
        };
        let init = p.request(
            "initialize",
            json!({
                "protocolVersion": "2025-06-18",
                "capabilities": {},
                "clientInfo": {"name": "smoke-test", "version": "0"}
            }),
        );
        assert_eq!(init["result"]["serverInfo"]["name"], "terminal-trader");
        p.send(json!({"jsonrpc": "2.0", "method": "notifications/initialized"}));
        p
    }

    fn send(&mut self, msg: Value) {
        writeln!(self.stdin, "{msg}").unwrap();
        self.stdin.flush().unwrap();
    }

    /// Send a request and return its response. Every stdout line on the way
    /// must be JSON-RPC.
    fn request(&mut self, method: &str, params: Value) -> Value {
        let id = self.next_id;
        self.next_id += 1;
        self.send(json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}));
        loop {
            let line = self
                .lines
                .recv_timeout(Duration::from_secs(60))
                .expect("timed out waiting for the server");
            let msg: Value = serde_json::from_str(&line)
                .unwrap_or_else(|e| panic!("stdout line isn't JSON ({e}): {line:?}"));
            assert_eq!(msg["jsonrpc"], "2.0", "not JSON-RPC: {line}");
            if msg["id"] == json!(id) {
                return msg;
            }
        }
    }

    /// Call a tool; returns (is_error, text).
    fn call(&mut self, name: &str, args: Value) -> (bool, String) {
        let resp = self.request("tools/call", json!({"name": name, "arguments": args}));
        let result = &resp["result"];
        assert!(result.is_object(), "{name}: no result: {resp}");
        let text = result["content"][0]["text"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        (result["isError"] == true, text)
    }
}

impl Drop for McpProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
fn stdio_server_initializes_and_lists_tools() {
    let data_dir = tempfile::tempdir().unwrap();
    let mut mcp = McpProcess::spawn(Some(data_dir.path()));

    let list = mcp.request("tools/list", json!({}));
    let tools = list["result"]["tools"].as_array().expect("tools array");
    let mut names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    names.sort();
    let mut expected = EXPECTED_TOOLS.to_vec();
    expected.sort();
    assert_eq!(names, expected);
    for t in tools {
        assert!(
            t["description"].as_str().is_some_and(|d| d.len() > 20),
            "tool {} needs a description",
            t["name"]
        );
        assert_eq!(t["inputSchema"]["type"], "object");
    }
    // Read-only: no tool that could trade.
    for banned in ["place", "cancel", "replace", "preview"] {
        assert!(
            !names.iter().any(|n| n.contains(banned)),
            "unexpected {banned} tool in {names:?}"
        );
    }

    // Without credentials the tools explain the setup, as a tool error.
    let (is_error, text) = mcp.call("get_positions", json!({}));
    assert!(is_error);
    assert!(text.contains("SCHWAB_API_KEY"), "{text}");

    let (is_error, text) = mcp.call(
        "get_option_chain",
        json!({"symbol": "SPY", "side": "sideways"}),
    );
    assert!(is_error);
    assert!(text.contains("side"), "{text}");
}

/// Every tool against the real account, through the real binary. Asserts
/// that no account number or hash appears in any output. Prints only sizes,
/// never the output itself.
///
///   cargo test --test mcp_stdio -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn live_tools_never_leak_account_numbers() {
    use terminal_trader::schwab::{SchwabAuth, SchwabClient};

    terminal_trader::data_dir::load_env();
    let (Ok(key), Ok(secret)) = (
        std::env::var("SCHWAB_API_KEY"),
        std::env::var("SCHWAB_APP_SECRET"),
    ) else {
        eprintln!("SKIP: Schwab credentials not set");
        return;
    };
    let client = match SchwabClient::new(SchwabAuth::new(
        key,
        secret,
        terminal_trader::data_dir::token_path(),
    ))
    .await
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("SKIP: no usable login: {e}");
            return;
        }
    };
    let accounts = client.get_accounts().await.expect("get_accounts");
    let secrets: Vec<String> = accounts
        .iter()
        .flat_map(|a| [a.number.clone(), a.hash.clone()])
        .collect();

    let mut mcp = McpProcess::spawn(None);
    let (err, status) = mcp.call("auth_status", json!({}));
    assert!(!err && status.contains("logged_in=true"), "{status}");

    let (err, list) = mcp.call("list_accounts", json!({}));
    assert!(!err, "list_accounts failed");
    let labels: Vec<String> = list.lines().skip(1).map(str::to_string).collect();
    assert_eq!(labels.len(), accounts.len());
    let first = labels[0].clone();

    let calls: Vec<(&str, Value)> = vec![
        ("get_positions", json!({})),
        ("get_balances", json!({})),
        (
            "get_orders",
            json!({"status": "all", "from_date": (chrono::Local::now().date_naive() - chrono::Duration::days(59)).to_string()}),
        ),
        ("get_lots", json!({})),
        ("get_positions", json!({"account": first.to_lowercase()})),
        ("get_quotes", json!({"symbols": ["SPY", "$SPX", "AAPL"]})),
        ("get_expirations", json!({"symbol": "SPY", "max_dte": 60})),
        (
            "get_option_chain",
            json!({"symbol": "SPY", "max_dte": 10, "strike_count": 3}),
        ),
        (
            "get_option_chain",
            json!({"symbol": "SPY", "side": "put", "max_rows": 5, "strike_count": 0}),
        ),
        ("get_positions", json!({"account": "no such account"})),
    ];
    for (name, args) in calls {
        let (is_error, text) = mcp.call(name, args.clone());
        println!(
            "{name} {args}: error={is_error} {} bytes, {} lines",
            text.len(),
            text.lines().count()
        );
        for s in &secrets {
            assert!(
                !text.contains(s.as_str()),
                "{name} leaked an account number or hash"
            );
        }
        if name == "get_positions" && args["account"] == "no such account" {
            assert!(is_error);
            continue;
        }
        assert!(
            !is_error,
            "{name} failed: {}",
            text.lines().next().unwrap_or("")
        );
    }

    let (_, chain) = mcp.call(
        "get_option_chain",
        json!({"symbol": "SPY", "side": "put", "max_rows": 5, "strike_count": 0}),
    );
    assert!(
        chain.contains("TRUNCATED"),
        "a full put chain is cut at 5 rows"
    );
    let header = chain.lines().find(|l| l.starts_with("exp,")).unwrap();
    assert!(header.contains("gamma") && header.contains("vega") && header.contains("theo"));
}
