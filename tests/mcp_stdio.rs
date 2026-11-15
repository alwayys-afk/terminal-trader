//! End-to-end smoke test of `terminal-trader mcp` over stdio. Needs no
//! Schwab account: it runs against an empty data directory with no
//! credentials and checks the protocol, the tool list, and that stdout carries
//! nothing but JSON-RPC.

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
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

#[test]
fn stdio_server_initializes_and_lists_tools() {
    let data_dir = tempfile::tempdir().unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_terminal-trader"))
        .arg("mcp")
        .env("TERMINAL_TRADER_DATA_DIR", data_dir.path())
        .env_remove("SCHWAB_API_KEY")
        .env_remove("SCHWAB_APP_SECRET")
        .env_remove("SCHWAB_TOKEN_PATH")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn terminal-trader mcp");

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();

    // Read stdout lines on a thread so a hung server fails the test instead
    // of hanging it.
    let (tx, rx) = mpsc::channel::<String>();
    std::thread::spawn(move || {
        for line in BufReader::new(stdout).lines() {
            let Ok(line) = line else { break };
            if tx.send(line).is_err() {
                break;
            }
        }
    });
    let mut next_response = |id: i64| -> Value {
        loop {
            let line = rx
                .recv_timeout(Duration::from_secs(20))
                .expect("timed out waiting for the server");
            let msg: Value = serde_json::from_str(&line)
                .unwrap_or_else(|e| panic!("stdout line isn't JSON ({e}): {line:?}"));
            assert_eq!(msg["jsonrpc"], "2.0", "not JSON-RPC: {line}");
            if msg["id"] == json!(id) {
                return msg;
            }
        }
    };
    let mut send = |msg: Value| {
        writeln!(stdin, "{msg}").unwrap();
        stdin.flush().unwrap();
    };

    send(json!({
        "jsonrpc": "2.0", "id": 1, "method": "initialize",
        "params": {
            "protocolVersion": "2025-06-18",
            "capabilities": {},
            "clientInfo": {"name": "smoke-test", "version": "0"}
        }
    }));
    let init = next_response(1);
    assert_eq!(init["result"]["serverInfo"]["name"], "terminal-trader");
    assert!(init["result"]["capabilities"]["tools"].is_object());
    assert!(
        init["result"]["instructions"]
            .as_str()
            .unwrap()
            .contains("nickname")
    );

    send(json!({"jsonrpc": "2.0", "method": "notifications/initialized"}));
    send(json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list"}));
    let list = next_response(2);
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
    // Read-only: nothing that sounds like trading.
    assert!(
        !names
            .iter()
            .any(|n| n.contains("order") && *n != "get_orders"),
        "unexpected order tool in {names:?}"
    );

    // Without credentials the tools explain the setup, as a tool error.
    send(json!({
        "jsonrpc": "2.0", "id": 3, "method": "tools/call",
        "params": {"name": "get_positions", "arguments": {}}
    }));
    let call = next_response(3);
    assert_eq!(call["result"]["isError"], true);
    let text = call["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("SCHWAB_API_KEY"), "{text}");

    send(json!({
        "jsonrpc": "2.0", "id": 4, "method": "tools/call",
        "params": {"name": "get_option_chain", "arguments": {"symbol": "SPY", "side": "sideways"}}
    }));
    let bad = next_response(4);
    assert_eq!(bad["result"]["isError"], true);

    drop(send);
    let _ = child.kill();
    let _ = child.wait();
}
