use chrono::Datelike;
/// Integration tests against the real Schwab API.
///
/// Run all:
///   cargo test -- --ignored --nocapture
///
/// Run one:
///   cargo test test_get_streamer_info -- --ignored --nocapture
///   cargo test test_acct_activity_stream -- --ignored --nocapture
///
/// Prerequisites:
///   - A `.env` file (or environment) with SCHWAB_API_KEY and SCHWAB_APP_SECRET
///   - A valid token file (default: schwab_tokens.json) from a prior auth flow
///
/// If credentials are missing the tests print a message and pass vacuously.
use terminal_trader::schwab::{InstrumentSpec, SchwabAuth, SchwabClient};

// ── Setup helper ──────────────────────────────────────────────────────────────

/// Returns `(client, account_hash)` if credentials and a token file are
/// present, or `None` to signal that the test should be skipped.
async fn setup() -> Option<(SchwabClient, String)> {
    dotenvy::dotenv().ok();

    let api_key = match std::env::var("SCHWAB_API_KEY") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("SKIP: SCHWAB_API_KEY not set");
            return None;
        }
    };
    let app_secret = match std::env::var("SCHWAB_APP_SECRET") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("SKIP: SCHWAB_APP_SECRET not set");
            return None;
        }
    };
    let token_path =
        std::env::var("SCHWAB_TOKEN_PATH").unwrap_or_else(|_| "schwab_tokens.json".to_string());

    if !std::path::Path::new(&token_path).exists() {
        eprintln!("SKIP: token file not found at {token_path} — run the app first to authenticate");
        return None;
    }

    let auth = SchwabAuth::new(api_key, app_secret, token_path);
    let client = match SchwabClient::new(auth).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("SKIP: failed to build client: {e}");
            return None;
        }
    };

    let accounts = match client.get_account_hashes().await {
        Ok(a) => a,
        Err(e) => {
            eprintln!("SKIP: get_account_hashes failed: {e}");
            return None;
        }
    };

    let (_, hash) = accounts.into_iter().next()?;
    Some((client, hash))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[tokio::test]
#[ignore]
async fn test_get_account_hashes() {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("SCHWAB_API_KEY").expect("SCHWAB_API_KEY not set");
    let app_secret = std::env::var("SCHWAB_APP_SECRET").expect("SCHWAB_APP_SECRET not set");
    let token_path =
        std::env::var("SCHWAB_TOKEN_PATH").unwrap_or_else(|_| "schwab_tokens.json".to_string());

    if !std::path::Path::new(&token_path).exists() {
        eprintln!("SKIP: token file not found");
        return;
    }

    let auth = SchwabAuth::new(api_key, app_secret, token_path);
    let client = SchwabClient::new(auth)
        .await
        .expect("failed to build client");

    let accounts = client
        .get_account_hashes()
        .await
        .expect("get_account_hashes failed");

    assert!(!accounts.is_empty(), "expected at least one account");

    for (num, hash) in &accounts {
        assert!(!num.is_empty(), "account number should not be empty");
        assert!(!hash.is_empty(), "account hash should not be empty");
        println!("account: {num}  hash: {}...", &hash[..8.min(hash.len())]);
    }
}

#[tokio::test]
#[ignore]
async fn test_get_positions() {
    let Some((client, hash)) = setup().await else {
        return;
    };

    let (positions, account_info) = client
        .get_positions(&hash)
        .await
        .expect("get_positions failed");

    println!("got {} position(s)", positions.len());
    println!("cash_balance: {:.2}", account_info.cash_balance);
    println!("buying_power:   {:.2}", account_info.buying_power);

    for pos in &positions {
        assert!(
            !pos.symbol.is_empty(),
            "position symbol should not be empty"
        );
        assert!(
            pos.quantity != 0.0,
            "position quantity should be non-zero (symbol: {})",
            pos.symbol
        );
        assert!(
            pos.market_value.is_finite(),
            "market_value should be finite (symbol: {})",
            pos.symbol
        );
        println!(
            "  {} qty={} mkt_val={:.2}",
            pos.symbol, pos.quantity, pos.market_value
        );
    }
}

#[tokio::test]
#[ignore]
async fn test_get_orders() {
    let Some((client, hash)) = setup().await else {
        return;
    };

    let orders = client.get_orders(&hash).await.expect("get_orders failed");

    println!("got {} order(s)", orders.len());

    for order in &orders {
        assert!(order.order_id > 0, "order_id should be positive");
        assert!(!order.symbol.is_empty(), "order symbol should not be empty");
        assert!(
            order.quantity > 0.0,
            "order quantity should be positive (id: {})",
            order.order_id
        );
        assert!(
            order.filled_quantity >= 0.0,
            "filled_quantity should be non-negative (id: {})",
            order.order_id
        );
        assert!(
            order.filled_quantity <= order.quantity,
            "filled_quantity should not exceed quantity (id: {})",
            order.order_id
        );
        println!(
            "  {} {:?} {:?} qty={} filled={} status={:?}",
            order.symbol,
            order.instruction,
            order.order_type,
            order.quantity,
            order.filled_quantity,
            order.status,
        );
    }
}

#[tokio::test]
#[ignore]
async fn test_get_quotes() {
    let Some((client, _hash)) = setup().await else {
        return;
    };

    let symbols = &["AAPL", "SPY"];
    let quotes = client.get_quotes(symbols).await.expect("get_quotes failed");

    for sym in symbols {
        let quote = quotes
            .get(*sym)
            .unwrap_or_else(|| panic!("expected a quote for {sym}"));

        assert!(
            quote.last_price > 0.0,
            "last_price should be positive for {sym}"
        );
        assert!(
            quote.bid_price >= 0.0,
            "bid_price should be non-negative for {sym}"
        );
        assert!(
            quote.ask_price >= 0.0,
            "ask_price should be non-negative for {sym}"
        );
        println!(
            "  {} last={:.2} bid={:.2} ask={:.2}",
            sym, quote.last_price, quote.bid_price, quote.ask_price
        );
    }
}

#[tokio::test]
#[ignore]
async fn test_get_streamer_info() {
    let Some((client, _hash)) = setup().await else {
        return;
    };

    let info = client
        .get_streamer_info()
        .await
        .expect("get_streamer_info failed");

    let ws_url = info.streamer_socket_url.expect("missing streamerSocketUrl");
    let customer_id = info
        .schwab_client_customer_id
        .expect("missing schwabClientCustomerId");
    let correl_id = info
        .schwab_client_correl_id
        .expect("missing schwabClientCorrelId");
    let channel = info
        .schwab_client_channel
        .expect("missing schwabClientChannel");
    let function_id = info
        .schwab_client_function_id
        .expect("missing schwabClientFunctionId");

    assert!(
        ws_url.starts_with("wss://"),
        "ws_url should be a wss:// address, got: {ws_url}"
    );
    assert!(!customer_id.is_empty(), "customer_id should not be empty");
    assert!(!correl_id.is_empty(), "correl_id should not be empty");
    assert!(!channel.is_empty(), "channel should not be empty");
    assert!(!function_id.is_empty(), "function_id should not be empty");

    println!("ws_url:      {ws_url}");
    println!("customer_id: {customer_id}");
    println!("correl_id:   {correl_id}");
    println!("channel:     {channel}");
    println!("function_id: {function_id}");
}

#[tokio::test]
#[ignore]
async fn test_search_instruments() {
    let Some((client, _hash)) = setup().await else {
        return;
    };

    use schwab_marketdata::types::GetInstrumentsProjection;

    let results = client
        .search_instruments("AAPL", GetInstrumentsProjection::SymbolSearch)
        .await
        .expect("search_instruments failed");

    assert!(!results.is_empty(), "expected at least one result for AAPL");

    let (sym, desc) = &results[0];
    println!("first result: sym={sym} desc={desc}");
    assert!(
        sym.contains("AAPL"),
        "expected first result symbol to contain AAPL, got {sym}"
    );
    assert!(!desc.is_empty(), "expected non-empty description");

    println!("got {} result(s) total", results.len());
    for (s, d) in &results {
        println!("  {s}: {d}");
    }
}

#[tokio::test]
#[ignore]
async fn test_get_transactions() {
    let Some((client, hash)) = setup().await else {
        return;
    };

    let transactions = client
        .get_transactions_by_type(
            &hash,
            "2025-03-06T00:00:00.000Z".parse().unwrap(),
            "2026-03-05T00:00:00.000Z".parse().unwrap(),
            schwab_trader::types::TransactionType::Trade,
        )
        .await
        .expect("get_transactions failed");

    println!("got {} transaction(s)", transactions.len());

    for txn in &transactions {
        let desc = txn.description.as_deref().unwrap_or("");
        let trade_date = txn.trade_date.map(|d| d.to_string()).unwrap_or_default();
        let net = txn.net_amount.unwrap_or(0.0);
        let txn_type = txn
            .type_
            .as_ref()
            .map(|t| format!("{:?}", t))
            .unwrap_or_default();
        let activity = txn
            .activity_type
            .as_ref()
            .map(|t| format!("{:?}", t))
            .unwrap_or_default();

        println!(
            "  type={txn_type} activity={activity} date={trade_date} net={net:.2} desc={desc}"
        );

        for item in &txn.transfer_items {
            let sym = item
                .instrument
                .as_ref()
                .and_then(terminal_trader::schwab::transaction_instrument_symbol);
            println!(
                "    item: sym={:?} amount={:?} price={:?} cost={:?} effect={:?} instrument={:?}",
                sym, item.amount, item.price, item.cost, item.position_effect, item.instrument
            );
        }
    }
}

/// Test whether the API allows querying historical date ranges (e.g. 2024)
/// while we're in 2025+. If it works, we can reconstruct lots from the full
/// transaction history by walking back year-by-year.
///
///   cargo test test_get_transactions_historical -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn test_get_transactions_historical() {
    let Some((client, hash)) = setup().await else {
        return;
    };

    // Try a full year in the past: 2024-01-01 to 2024-12-31
    let start: chrono::DateTime<chrono::Utc> = "2024-01-01T00:00:00.000Z".parse().unwrap();
    let end: chrono::DateTime<chrono::Utc> = "2024-12-31T23:59:59.000Z".parse().unwrap();

    match client
        .get_transactions_by_type(
            &hash,
            start,
            end,
            schwab_trader::types::TransactionType::Trade,
        )
        .await
    {
        Ok(transactions) => {
            println!(
                "SUCCESS: got {} transaction(s) for 2024",
                transactions.len()
            );
            for txn in &transactions {
                let desc = txn.description.as_deref().unwrap_or("");
                let trade_date = txn.trade_date.map(|d| d.to_string()).unwrap_or_default();
                let net = txn.net_amount.unwrap_or(0.0);
                println!("  date={trade_date} net={net:.2} desc={desc}");
            }
        }
        Err(e) => {
            println!("FAILED to fetch 2024 transactions: {e}");
            println!("The API may not support historical date ranges outside the current year.");
        }
    }

    // Also try an even older range: 2023
    let start2: chrono::DateTime<chrono::Utc> = "2023-01-01T00:00:00.000Z".parse().unwrap();
    let end2: chrono::DateTime<chrono::Utc> = "2023-12-31T23:59:59.000Z".parse().unwrap();

    match client
        .get_transactions_by_type(
            &hash,
            start2,
            end2,
            schwab_trader::types::TransactionType::Trade,
        )
        .await
    {
        Ok(transactions) => {
            println!(
                "SUCCESS: got {} transaction(s) for 2023",
                transactions.len()
            );
        }
        Err(e) => {
            println!("FAILED to fetch 2023 transactions: {e}");
        }
    }
}

/// Test that applying all transactions from the API to an empty lot store
/// produces lots. The exact assertions depend on the account, so we just
/// verify the pipeline works end-to-end and print the results.
#[tokio::test]
#[ignore]
async fn test_tax_lots_from_transactions() {
    use terminal_trader::app::lots::TaxLotStore;

    let Some((client, hash)) = setup().await else {
        return;
    };

    let mut all_txns = client
        .get_transactions_by_type(
            &hash,
            "2025-03-06T00:00:00.000Z".parse().unwrap(),
            "2026-03-05T00:00:00.000Z".parse().unwrap(),
            schwab_trader::types::TransactionType::Trade,
        )
        .await
        .expect("get_transactions failed");
    all_txns.sort_by_key(|t| t.trade_date);

    println!("total transactions: {}", all_txns.len());

    let mut store = TaxLotStore::default();
    for txn in &all_txns {
        store.apply_transaction(txn);
    }

    println!("\nResulting lots:");
    for (symbol, lots) in &store.lots {
        let total_qty: f64 = lots.iter().map(|l| l.quantity).sum();
        println!(
            "  {symbol}: {total_qty:.0} shares across {} lot(s)",
            lots.len()
        );
        for lot in lots {
            println!(
                "    {} qty={} cost={:.2}",
                lot.open_date, lot.quantity, lot.cost_per_share
            );
        }
    }

    // Basic sanity: if there were any TRADE transactions, we should have lots
    // (unless every buy was fully sold off).
    let trade_count = all_txns
        .iter()
        .filter(|t| t.type_.as_ref() == Some(&schwab_trader::types::TransactionType::Trade))
        .count();
    println!(
        "\ntrade transactions: {trade_count}, resulting symbols: {}",
        store.lots.len()
    );
}

/// Pull ~2 years of all transaction types and look for corporate actions
/// (mergers, splits, reorganizations, etc.)
///
///   cargo test test_all_transaction_types -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn test_all_transaction_types() {
    let Some((client, hash)) = setup().await else {
        return;
    };

    // Schwab API limits to 1-year ranges, so we split into two windows
    let windows = [
        ("2024-03-10T00:00:00.000Z", "2025-03-10T00:00:00.000Z"),
        ("2025-03-10T00:00:00.000Z", "2026-03-10T00:00:00.000Z"),
    ];

    let types = [
        "TRADE",
        "RECEIVE_AND_DELIVER",
        "DIVIDEND_OR_INTEREST",
        "JOURNAL",
        "MEMORANDUM",
        "ACH_RECEIPT",
        "ACH_DISBURSEMENT",
        "CASH_RECEIPT",
        "CASH_DISBURSEMENT",
        "ELECTRONIC_FUND",
        "WIRE_OUT",
        "WIRE_IN",
        "MARGIN_CALL",
        "MONEY_MARKET",
        "SMA_ADJUSTMENT",
    ];

    let trader = client.trader().await;

    let mut all_txns = Vec::new();

    for txn_type in &types {
        for (start, end) in &windows {
            let result = trader
                .get_transactions_by_path_param()
                .account_number(&hash)
                .start_date(*start)
                .end_date(*end)
                .types(*txn_type)
                .send()
                .await;

            match result {
                Ok(resp) => {
                    let txns = resp.into_inner();
                    if !txns.is_empty() {
                        println!(
                            "[{} | {}..{}] {} txn(s)",
                            txn_type,
                            &start[..10],
                            &end[..10],
                            txns.len()
                        );
                    }
                    all_txns.extend(txns);
                }
                Err(e) => {
                    println!(
                        "[{} | {}..{}] ERROR: {}",
                        txn_type,
                        &start[..10],
                        &end[..10],
                        e
                    );
                }
            }
        }
    }

    println!("\n=== Total: {} transactions ===\n", all_txns.len());

    // Sort by trade date
    all_txns.sort_by_key(|t| t.trade_date);

    // Print all non-TRADE transactions (these are the interesting ones)
    println!("=== NON-TRADE TRANSACTIONS ===");
    for txn in &all_txns {
        let txn_type = txn
            .type_
            .as_ref()
            .map(|t| format!("{:?}", t))
            .unwrap_or_default();

        if txn_type == "Trade" {
            continue; // skip regular trades, we want corporate actions
        }

        let desc = txn.description.as_deref().unwrap_or("");
        let trade_date = txn.trade_date.map(|d| d.to_string()).unwrap_or_default();
        let net = txn.net_amount.unwrap_or(0.0);
        let activity = txn
            .activity_type
            .as_ref()
            .map(|t| format!("{:?}", t))
            .unwrap_or_default();

        println!(
            "  type={txn_type} activity={activity} date={trade_date} net={net:.2} desc=\"{desc}\""
        );

        for item in &txn.transfer_items {
            let sym = item.instrument.as_ref().and_then(|i| match i {
                schwab_trader::types::TransactionInstrument::TransactionEquity(e) => {
                    e.symbol.as_deref()
                }
                _ => None,
            });
            println!(
                "    item: sym={:?} amount={:?} price={:?} cost={:?} effect={:?}",
                sym, item.amount, item.price, item.cost, item.position_effect
            );
        }
    }

    // Now specifically look for merger/split/reorg keywords
    let keywords = [
        "merger",
        "split",
        "reorg",
        "spinoff",
        "spin-off",
        "exchange",
        "convert",
        "tender",
        "acquisition",
        "mandatory",
        "voluntary",
        "reverse",
        "stock dividend",
        "name change",
    ];

    println!("\n=== CORPORATE ACTION CANDIDATES (keyword match) ===");
    let mut found = 0;
    for txn in &all_txns {
        let desc = txn.description.as_deref().unwrap_or("").to_lowercase();
        let matched: Vec<&&str> = keywords.iter().filter(|k| desc.contains(**k)).collect();
        if !matched.is_empty() {
            found += 1;
            let txn_type = txn
                .type_
                .as_ref()
                .map(|t| format!("{:?}", t))
                .unwrap_or_default();
            let trade_date = txn.trade_date.map(|d| d.to_string()).unwrap_or_default();
            let net = txn.net_amount.unwrap_or(0.0);
            println!(
                "  MATCH {:?}: type={txn_type} date={trade_date} net={net:.2} desc=\"{}\"",
                matched,
                txn.description.as_deref().unwrap_or("")
            );
            for item in &txn.transfer_items {
                let sym = item.instrument.as_ref().and_then(|i| match i {
                    schwab_trader::types::TransactionInstrument::TransactionEquity(e) => {
                        e.symbol.as_deref()
                    }
                    _ => None,
                });
                println!(
                    "    item: sym={:?} amount={:?} price={:?} cost={:?} effect={:?}",
                    sym, item.amount, item.price, item.cost, item.position_effect
                );
            }
        }
    }
    println!("\nFound {} corporate action candidates", found);

    // Also look for RECEIVE_AND_DELIVER specifically — that's where mergers/splits live
    println!("\n=== ALL RECEIVE_AND_DELIVER TRANSACTIONS ===");
    for txn in &all_txns {
        if txn.type_.as_ref() == Some(&schwab_trader::types::TransactionType::ReceiveAndDeliver) {
            let desc = txn.description.as_deref().unwrap_or("");
            let trade_date = txn.trade_date.map(|d| d.to_string()).unwrap_or_default();
            let net = txn.net_amount.unwrap_or(0.0);
            let activity = txn
                .activity_type
                .as_ref()
                .map(|t| format!("{:?}", t))
                .unwrap_or_default();
            println!("  activity={activity} date={trade_date} net={net:.2} desc=\"{desc}\"");
            for item in &txn.transfer_items {
                println!("    item: {:?}", item);
            }
        }
    }
}

/// Place a market buy for 1 share of NVDA (Session::Normal / Duration::Day).
/// Because this runs after hours the order will be queued, not filled immediately.
/// After placing we fetch the order list and print it so we can inspect the result.
///
///   cargo test test_place_order_nvda -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn test_place_order_nvda() {
    use terminal_trader::schwab::Instruction;

    let Some((client, hash)) = setup().await else {
        return;
    };

    // Fetch live quote so we can place a competitive limit order.
    let quotes = client
        .get_quotes(&["HOG"])
        .await
        .expect("get_quotes failed");
    let quote = quotes.get("HOG").expect("no HOG quote returned");
    // Use ask price (or last if ask is zero after hours) so the limit is fillable.
    let limit = if quote.ask_price > 0.0 {
        quote.ask_price
    } else {
        quote.last_price
    };
    println!(
        "HOG last={:.2} ask={:.2} — placing limit at {limit:.2} (session=SEAMLESS, duration=GTC)",
        quote.last_price, quote.ask_price
    );

    let result = client
        .place_order(
            &hash,
            InstrumentSpec::Equity {
                symbol: "HOG".to_string(),
            },
            Instruction::Buy,
            1.0,
            Some(limit),
            None,
        )
        .await;

    match &result {
        Ok(()) => println!("place_order returned Ok(())"),
        Err(e) => {
            eprintln!("place_order failed: {e:#}");
            panic!("place_order returned an error: {e}");
        }
    }

    // Give the API a moment to record the order, then pull the list.
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let orders = client
        .get_orders(&hash)
        .await
        .expect("get_orders failed after placing order");

    println!("get_orders returned {} order(s):", orders.len());
    for o in &orders {
        println!(
            "  id={} sym={} {:?} {:?} qty={} filled={} status={:?} session={:?} duration={:?}",
            o.order_id,
            o.symbol,
            o.instruction,
            o.order_type,
            o.quantity,
            o.filled_quantity,
            o.status,
            o.session,
            o.duration,
        );
    }

    let hog_order = orders.iter().find(|o| o.symbol == "HOG");
    assert!(
        hog_order.is_some(),
        "expected to find a HOG order in the list after placing one"
    );
    let o = hog_order.unwrap();
    assert_eq!(o.quantity, 1.0, "expected quantity 1");
    println!(
        "Assertion passed — HOG order found with id={} status={:?}",
        o.order_id, o.status
    );
}

/// Buy 1 share of HOG then immediately sell it, verifying both fill.
///
///   cargo test test_buy_and_sell_hog -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn test_buy_and_sell_hog() {
    use std::collections::HashSet;
    use terminal_trader::schwab::Instruction;

    let Some((client, hash)) = setup().await else {
        return;
    };

    // ── 1. Quote ──
    let quotes = client
        .get_quotes(&["HOG"])
        .await
        .expect("get_quotes failed");
    let q = quotes.get("HOG").expect("no HOG quote returned");
    // Use ask for buy, bid for sell; fall back to last +/- 0.10 if zero (pre-market).
    let ask = if q.ask_price > 0.0 {
        q.ask_price
    } else {
        q.last_price + 0.10
    };
    let bid = if q.bid_price > 0.0 {
        q.bid_price
    } else {
        q.last_price - 0.10
    };
    println!(
        "HOG: last={:.2}  bid={:.2}  ask={:.2}",
        q.last_price, bid, ask
    );

    // ── 2. Snapshot existing order IDs ──
    let ids_before_buy: HashSet<i64> = client
        .get_orders(&hash)
        .await
        .expect("get_orders (before buy) failed")
        .into_iter()
        .map(|o| o.order_id)
        .collect();

    // ── 3. Place BUY at ask ──
    println!("Placing BUY 1 HOG limit @ {ask:.2} (Seamless/GTC)...");
    client
        .place_order(
            &hash,
            InstrumentSpec::Equity {
                symbol: "HOG".to_string(),
            },
            Instruction::Buy,
            1.0,
            Some(ask),
            None,
        )
        .await
        .expect("place_order (buy) failed");

    // ── 4. Wait for buy fill ──
    let buy_id = poll_for_fill(&client, &hash, &ids_before_buy, 60)
        .await
        .expect("BUY HOG did not fill within 60s");
    println!("BUY filled! order_id={buy_id}");

    // ── 5. Re-quote for a fresh bid ──
    let quotes2 = client
        .get_quotes(&["HOG"])
        .await
        .expect("get_quotes (pre-sell) failed");
    let q2 = quotes2.get("HOG").expect("no HOG quote (pre-sell)");
    let bid2 = if q2.bid_price > 0.0 {
        q2.bid_price
    } else {
        q2.last_price - 0.10
    };
    println!(
        "HOG re-quote: bid={:.2} — placing SELL 1 HOG limit @ {bid2:.2}...",
        bid2
    );

    // ── 6. Snapshot existing order IDs (now includes the buy) ──
    let ids_before_sell: HashSet<i64> = client
        .get_orders(&hash)
        .await
        .expect("get_orders (before sell) failed")
        .into_iter()
        .map(|o| o.order_id)
        .collect();

    // ── 7. Place SELL at bid ──
    client
        .place_order(
            &hash,
            InstrumentSpec::Equity {
                symbol: "HOG".to_string(),
            },
            Instruction::Sell,
            1.0,
            Some(bid2),
            None,
        )
        .await
        .expect("place_order (sell) failed");

    // ── 8. Wait for sell fill ──
    let sell_id = poll_for_fill(&client, &hash, &ids_before_sell, 60)
        .await
        .expect("SELL HOG did not fill within 60s");
    println!("SELL filled! order_id={sell_id}");

    println!("Round-trip complete — buy={buy_id}  sell={sell_id}");

    // Sanity: the two orders are different.
    assert_ne!(buy_id, sell_id, "buy and sell should be separate orders");
}

/// Poll `get_orders` every 2 s until a new order (not in `known_ids`) reaches
/// `Filled` status.  Returns the order_id on success, None on timeout.
async fn poll_for_fill(
    client: &terminal_trader::schwab::SchwabClient,
    hash: &str,
    known_ids: &std::collections::HashSet<i64>,
    timeout_secs: u64,
) -> Option<i64> {
    use terminal_trader::schwab::OrderStatus;

    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(timeout_secs);
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let orders = match client.get_orders(hash).await {
            Ok(o) => o,
            Err(e) => {
                eprintln!("  poll get_orders error: {e}");
                continue;
            }
        };

        // Print status of any new orders so we can see progress.
        for o in orders.iter().filter(|o| !known_ids.contains(&o.order_id)) {
            println!(
                "  order {} {:?} {:?} filled={}/{}",
                o.order_id, o.instruction, o.status, o.filled_quantity, o.quantity
            );
            if matches!(o.status, OrderStatus::Filled) {
                return Some(o.order_id);
            }
        }

        if std::time::Instant::now() >= deadline {
            return None;
        }
    }
}

/// Connect to the Schwab streamer and dump raw ACCT_ACTIVITY messages for 60 seconds.
/// Place, fill, or cancel an order while this runs to capture real payloads.
///
///   cargo test test_acct_activity_stream -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn test_acct_activity_stream() {
    use futures::{SinkExt, StreamExt};
    use terminal_trader::schwab::streamer_types::{
        AcctActivityTick, ServiceName, StreamerInbound, StreamerRequest,
    };
    use tokio::time::{Duration, timeout};
    use tokio_tungstenite::{connect_async, tungstenite::Message};

    let Some((client, _hash)) = setup().await else {
        return;
    };

    let info = client
        .get_streamer_info()
        .await
        .expect("get_streamer_info failed");

    let ws_url = info.streamer_socket_url.expect("missing streamerSocketUrl");
    let customer_id = info
        .schwab_client_customer_id
        .expect("missing schwabClientCustomerId");
    let correl_id = info
        .schwab_client_correl_id
        .expect("missing schwabClientCorrelId");
    let channel = info.schwab_client_channel.unwrap_or_default();
    let function_id = info.schwab_client_function_id.unwrap_or_default();

    println!("Connecting to WebSocket: {ws_url}");
    let (mut ws, _) = connect_async(&ws_url)
        .await
        .expect("WebSocket connect failed");
    println!("WebSocket connected");

    // LOGIN
    let login = StreamerRequest::login(
        &customer_id,
        &correl_id,
        &client.access_token().await.expect("access_token"),
        &channel,
        &function_id,
    );
    ws.send(Message::text(serde_json::to_string(&login).unwrap()))
        .await
        .expect("LOGIN send failed");
    println!("LOGIN sent");

    if let Some(Ok(msg)) = ws.next().await {
        let parsed: StreamerInbound =
            serde_json::from_str(&msg.to_text().unwrap_or_default()).unwrap();
        println!("LOGIN response: {parsed:?}");
    }

    // Subscribe to ACCT_ACTIVITY
    let subs = StreamerRequest::subs_acct_activity(&customer_id, &correl_id);
    ws.send(Message::text(serde_json::to_string(&subs).unwrap()))
        .await
        .expect("SUBS send failed");
    println!("ACCT_ACTIVITY SUBS sent");

    println!("Listening for 60s — place/fill/cancel an order to capture payloads...");

    let listen = async {
        while let Some(msg) = ws.next().await {
            match msg {
                Ok(Message::Text(t)) => match serde_json::from_str::<StreamerInbound>(&t) {
                    Ok(StreamerInbound::Data { data }) => {
                        for frame in &data {
                            if frame.service == ServiceName::AcctActivity {
                                for item in &frame.content {
                                    match serde_json::from_value::<AcctActivityTick>(item.clone()) {
                                        Ok(tick) => println!(
                                            "ACCT_ACTIVITY: account={:?} type={:?} data={:?}",
                                            tick.account, tick.message_type, tick.message_data
                                        ),
                                        Err(e) => {
                                            println!("ACCT_ACTIVITY parse error: {e} raw={item}")
                                        }
                                    }
                                }
                            } else {
                                println!(
                                    "DATA ({:?}): {} item(s)",
                                    frame.service,
                                    frame.content.len()
                                );
                            }
                        }
                    }
                    Ok(StreamerInbound::Response { response }) => {
                        for r in &response {
                            println!(
                                "RESPONSE: {} {} code={} msg={:?}",
                                r.service, r.command, r.content.code, r.content.msg
                            );
                        }
                    }
                    Ok(StreamerInbound::Notify { notify }) => {
                        for n in &notify {
                            println!("HEARTBEAT: {}", n.heartbeat);
                        }
                    }
                    Err(e) => println!("PARSE ERROR: {e}\n  raw: {t}"),
                },
                Ok(Message::Ping(_)) => {}
                Ok(Message::Close(frame)) => {
                    println!("WS closed: {frame:?}");
                    break;
                }
                Ok(other) => println!("OTHER: {other:?}"),
                Err(e) => {
                    eprintln!("WS error: {e}");
                    break;
                }
            }
        }
    };

    timeout(Duration::from_secs(60), listen).await.ok();
    println!("Done.");
}

/// Fetch all FIG transactions and position to analyze wash sale behavior.
///
///   cargo test test_fig_wash_sales -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn test_fig_wash_sales() {
    use terminal_trader::schwab::transaction_instrument_symbol;

    let Some((client, hash)) = setup().await else {
        return;
    };

    // Fetch FIG position
    let (positions, _) = client.get_positions(&hash).await.expect("get_positions");
    let fig_pos = positions.iter().find(|p| p.symbol == "FIG");
    println!("=== FIG POSITION ===");
    match fig_pos {
        Some(p) => println!(
            "  qty={} avg_cost=${:.4} market_value=${:.2} current_price=${:.2}",
            p.quantity, p.average_cost, p.market_value, p.current_price
        ),
        None => println!("  (no FIG position found)"),
    }

    // Fetch all TRADE transactions for FIG over ~1 year
    let all_txns_result = client
        .get_transactions_by_type(
            &hash,
            "2025-03-11T00:00:00.000Z".parse().unwrap(),
            "2026-03-11T00:00:00.000Z".parse().unwrap(),
            schwab_trader::types::TransactionType::Trade,
        )
        .await;
    let mut all_txns = match all_txns_result {
        Ok(txns) => txns,
        Err(e) => {
            eprintln!("fetch error: {e}");
            return;
        }
    };
    all_txns.sort_by_key(|t| t.trade_date);

    // Filter to FIG
    let fig_txns: Vec<_> = all_txns
        .iter()
        .filter(|t| {
            t.transfer_items.iter().any(|item| {
                item.instrument
                    .as_ref()
                    .and_then(|i| transaction_instrument_symbol(i))
                    == Some("FIG")
            })
        })
        .collect();

    println!("\n=== FIG TRANSACTIONS ({}) ===", fig_txns.len());
    for txn in &fig_txns {
        let date = txn
            .trade_date
            .map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_default();
        let desc = txn.description.as_deref().unwrap_or("");
        let net = txn.net_amount.unwrap_or(0.0);
        let activity_id = txn.activity_id.unwrap_or(0);

        for item in &txn.transfer_items {
            let sym = item
                .instrument
                .as_ref()
                .and_then(|i| transaction_instrument_symbol(i));
            if sym == Some("FIG") {
                let effect = item
                    .position_effect
                    .as_ref()
                    .map(|e| format!("{:?}", e))
                    .unwrap_or_default();
                println!(
                    "  {} {} amt={:?} price={:?} cost={:?} net={:.2} id={} desc=\"{}\"",
                    date, effect, item.amount, item.price, item.cost, net, activity_id, desc
                );
            }
        }
    }

    // Now replay through our lot store to see what we compute
    println!("\n=== LOT STORE REPLAY ===");
    let mut store = terminal_trader::app::lots::TaxLotStore::default();
    for txn in &all_txns {
        store.apply_transaction(txn);
    }

    let lots = store.lots_for("FIG");
    let total_qty: f64 = lots.iter().map(|l| l.quantity).sum();
    let total_cost: f64 = lots.iter().map(|l| l.cost_per_share * l.remaining).sum();
    let avg_cost = if total_qty > 0.0 {
        total_cost / total_qty
    } else {
        0.0
    };
    println!(
        "  lots: {} open, total_qty={}, avg_cost=${:.4}",
        lots.len(),
        total_qty,
        avg_cost
    );
    for lot in lots {
        println!(
            "    {} qty={} cost=${:.4}",
            lot.open_date, lot.quantity, lot.cost_per_share
        );
    }

    let sales = store.sales_for("FIG");
    println!("  sales: {}", sales.len());
    for s in sales {
        println!(
            "    {} qty={} price=${:.4} pnl=${:.2}",
            s.sale_date, s.quantity, s.sale_price, s.realized_pnl
        );
    }

    // Compare
    if let Some(p) = fig_pos {
        println!("\n=== COMPARISON ===");
        println!("  Broker qty={} vs Lot qty={}", p.quantity, total_qty);
        println!(
            "  Broker avg=${:.4} vs Lot avg=${:.4} (diff=${:.4})",
            p.average_cost,
            avg_cost,
            (p.average_cost - avg_cost).abs()
        );
    }
}

/// Inspect transfer_items on every ReceiveAndDeliver transaction.
/// Answers: do any transactions have >1 transfer item?
///
///   cargo test test_rd_transfer_item_counts -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn test_rd_transfer_item_counts() {
    let Some((client, hash)) = setup().await else {
        return;
    };

    let windows = [
        ("2024-03-10T00:00:00.000Z", "2025-03-10T00:00:00.000Z"),
        ("2025-03-10T00:00:00.000Z", "2026-03-10T00:00:00.000Z"),
    ];

    let trader = client.trader().await;
    let mut all_rd = Vec::new();

    for (start, end) in &windows {
        match trader
            .get_transactions_by_path_param()
            .account_number(&hash)
            .start_date(*start)
            .end_date(*end)
            .types("RECEIVE_AND_DELIVER")
            .send()
            .await
        {
            Ok(resp) => all_rd.extend(resp.into_inner()),
            Err(e) => println!("ERROR fetching {start}..{end}: {e}"),
        }
    }

    println!("Total ReceiveAndDeliver transactions: {}", all_rd.len());

    let mut multi_item_count = 0;

    for txn in &all_rd {
        let n = txn.transfer_items.len();
        let desc = txn.description.as_deref().unwrap_or("");
        let date = txn.trade_date.map(|d| d.to_string()).unwrap_or_default();

        let sym = txn.transfer_items.first().and_then(|item| {
            item.instrument
                .as_ref()
                .and_then(|inst| terminal_trader::schwab::transaction_instrument_symbol(inst))
        });
        let effect = txn
            .transfer_items
            .first()
            .and_then(|item| item.position_effect.as_ref());
        let amount = txn.transfer_items.first().and_then(|item| item.amount);
        println!(
            "[{n} items] date={date} sym={sym:?} amount={amount:?} effect={effect:?} desc=\"{desc}\""
        );

        if n != 1 {
            multi_item_count += 1;
            for (i, item) in txn.transfer_items.iter().enumerate() {
                let s = item
                    .instrument
                    .as_ref()
                    .and_then(|inst| terminal_trader::schwab::transaction_instrument_symbol(inst));
                println!(
                    "  item[{i}]: sym={s:?} amount={:?} price={:?} cost={:?} effect={:?}",
                    item.amount, item.price, item.cost, item.position_effect
                );
            }
        }
    }

    println!("\n--- Summary ---");
    println!("Total R&D txns: {}", all_rd.len());
    println!("Txns with != 1 transfer item: {multi_item_count}");

    if multi_item_count == 0 {
        println!("All R&D transactions have exactly 1 transfer item.");
    }
}

/// Check previous_session_long_quantity for LWLG and RKT to see if we can
/// use it to detect same-day positions.
///
///   cargo test test_previous_session_qty -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn test_previous_session_qty() {
    let Some((client, hash)) = setup().await else {
        return;
    };

    let trader = client.trader().await;
    let account = trader
        .get_account()
        .account_number(&hash)
        .fields("positions")
        .send()
        .await
        .expect("get_account failed")
        .into_inner();

    let positions = match account.securities_account {
        Some(schwab_trader::types::SecuritiesAccount::MarginAccount(a)) => a.positions,
        Some(schwab_trader::types::SecuritiesAccount::CashAccount(a)) => a.positions,
        None => vec![],
    };

    println!("Found {} raw positions", positions.len());

    for pos in &positions {
        let sym = pos.instrument.as_ref().and_then(|i| match i {
            schwab_trader::types::AccountsInstrument::Equity(e) => e.0.symbol.as_deref(),
            schwab_trader::types::AccountsInstrument::Option(o) => o.symbol.as_deref(),
            _ => None,
        });
        let sym = sym.unwrap_or("???");

        let long_qty = pos.long_quantity.unwrap_or(0.0);
        let prev_long = pos.previous_session_long_quantity.unwrap_or(0.0);
        let short_qty = pos.short_quantity.unwrap_or(0.0);
        let prev_short = pos.previous_session_short_quantity.unwrap_or(0.0);
        let avg_price = pos.average_price.unwrap_or(0.0);
        let new_today = long_qty - prev_long;

        println!("=== {sym} ===");
        println!("  long_quantity:                  {long_qty}");
        println!("  previous_session_long_quantity: {prev_long}");
        println!("  short_quantity:                 {short_qty}");
        println!("  previous_session_short_quantity:{prev_short}");
        println!("  average_price:                  {avg_price:.4}");
        println!("  new_today (long):               {new_today}");
    }
}

/// Dump what /userPreference returns about each account so we can see what
/// display fields (nickname / displayAcctId / accountColor / etc.) are
/// actually populated. Used to design the account-picker label.
///
///   cargo test test_user_preference_accounts -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn test_user_preference_accounts() {
    let Some((client, _hash)) = setup().await else {
        return;
    };

    let trader = client.trader().await;
    let prefs = trader
        .get_user_preference()
        .send()
        .await
        .expect("GET /userPreference failed")
        .into_inner();

    println!(
        "\n=== /userPreference accounts ({}) ===",
        prefs.accounts.len()
    );
    for (i, a) in prefs.accounts.iter().enumerate() {
        println!("--- account {i} ---");
        println!("  accountNumber:      {:?}", a.account_number);
        println!("  nickName:           {:?}", a.nick_name);
        println!("  displayAcctId:      {:?}", a.display_acct_id);
        println!("  accountColor:       {:?}", a.account_color);
        println!("  type:               {:?}", a.type_);
        println!("  primaryAccount:     {}", a.primary_account);
        println!("  autoPositionEffect: {}", a.auto_position_effect);
    }
}

/// Verify the joined `get_accounts()` returns one row per account with
/// nicknames pulled from /userPreference. Used to sanity-check the picker
/// will have the right labels.
///
///   cargo test test_get_accounts_joined -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn test_get_accounts_joined() {
    let Some((client, _hash)) = setup().await else {
        return;
    };

    let accounts = client.get_accounts().await.expect("get_accounts failed");
    println!("\n=== get_accounts() returned {} ===", accounts.len());
    for a in &accounts {
        let nick = a.nickname.as_deref().unwrap_or("(no nickname)");
        println!(
            "  number={}  nickname={nick}  hash={}...",
            a.number,
            &a.hash[..8.min(a.hash.len())]
        );
    }
}

/// Fetch the AAPL option chain and assert it flattens into expirations with
/// both calls and puts. Prints a sample contract per expiration so we can
/// confirm the `ExpiryKind` (weekly/monthly) classification against live data.
///
///   cargo test test_get_chain_returns_calls_and_puts -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn test_get_chain_returns_calls_and_puts() {
    let Some((client, _hash)) = setup().await else {
        return;
    };

    let snapshot = client
        .get_option_chain("AAPL")
        .await
        .expect("get_option_chain failed");

    println!(
        "\n=== AAPL chain: underlying={:.2}  expirations={}  contracts={} ===",
        snapshot.underlying_price,
        snapshot.expirations.len(),
        snapshot.contract_count(),
    );

    assert!(
        snapshot.underlying_price > 0.0,
        "expected a positive underlying price (include_underlying_quote)"
    );
    assert!(
        !snapshot.expirations.is_empty(),
        "expected at least one expiration in the chain"
    );

    // Print the first few expirations and a sample call so we can eyeball the
    // strike sorting and expiry classification.
    for (date, slice) in snapshot.expirations.iter().take(5) {
        let sample = slice.calls.first().or_else(|| slice.puts.first());
        let kind = sample.map(|c| c.expiry_kind);
        println!(
            "  {date}  calls={:<3} puts={:<3} kind={:?}",
            slice.calls.len(),
            slice.puts.len(),
            kind,
        );
        if let Some(c) = slice.calls.first() {
            println!(
                "      sample call: {} strike={:.2} bid={:.2} ask={:.2} mark={:.2} delta={:.3} oi={} dte={}",
                c.osi_symbol,
                c.strike,
                c.bid,
                c.ask,
                c.mark,
                c.delta,
                c.open_interest,
                c.days_to_exp,
            );
        }
    }

    // At least one expiration should have both sides populated.
    let has_both = snapshot
        .expirations
        .values()
        .any(|s| !s.calls.is_empty() && !s.puts.is_empty());
    assert!(
        has_both,
        "expected at least one expiration with both calls and puts"
    );

    // Calls within an expiration must be sorted ascending by strike.
    for slice in snapshot.expirations.values() {
        let sorted = slice.calls.windows(2).all(|w| w[0].strike <= w[1].strike);
        assert!(sorted, "calls should be sorted ascending by strike");
    }
}

/// Place a 1-contract limit BUY_TO_OPEN on a deep out-of-the-money AAPL call at
/// $0.01 (never marketable), confirm it lands in the order list as an OPTION,
/// then cancel it. Tiny notional; the order is canceled immediately regardless.
///
///   cargo test test_place_option_order_far_otm_call -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn test_place_option_order_far_otm_call() {
    use std::collections::HashSet;
    use terminal_trader::schwab::Instruction;

    let Some((client, _default_hash)) = setup().await else {
        return;
    };

    // Options trading is only enabled on the account nicknamed "options", so
    // place against that one rather than the default first account.
    let accounts = client.get_accounts().await.expect("get_accounts failed");
    let Some(opt_account) = accounts.iter().find(|a| {
        a.nickname
            .as_deref()
            .is_some_and(|n| n.eq_ignore_ascii_case("options"))
    }) else {
        eprintln!("SKIP: no account nicknamed 'options' found");
        return;
    };
    let hash = opt_account.hash.clone();
    println!("Using options-enabled account (nickname='options')");

    let snapshot = client
        .get_option_chain("AAPL")
        .await
        .expect("get_option_chain failed");
    let underlying = snapshot.underlying_price;
    assert!(underlying > 0.0, "no underlying price");

    // Pick the first expiration at least a week out (avoids same-day/expired
    // contracts), then the cheapest strike at least 50% above spot — deep OTM,
    // so a $0.01 buy limit cannot fill.
    let target_strike_floor = underlying * 1.5;
    let mut chosen = None;
    for slice in snapshot.expirations.values() {
        if !slice.calls.iter().any(|c| c.days_to_exp >= 7) {
            continue;
        }
        if let Some(c) = slice
            .calls
            .iter()
            .find(|c| c.strike >= target_strike_floor)
            .or_else(|| slice.calls.last())
        {
            chosen = Some(c.clone());
            break;
        }
    }
    let contract = chosen.expect("no call contract found at least a week out");
    println!(
        "Chosen deep-OTM call: {} strike={:.2} (spot {:.2}) ask={:.2}",
        contract.osi_symbol, contract.strike, underlying, contract.ask,
    );

    // Snapshot existing order ids so we can identify the one we place.
    let ids_before: HashSet<i64> = client
        .get_orders(&hash)
        .await
        .expect("get_orders (before) failed")
        .into_iter()
        .map(|o| o.order_id)
        .collect();

    println!(
        "Placing BUY_TO_OPEN 1 {} limit @ 0.01...",
        contract.osi_symbol
    );
    client
        .place_order(
            &hash,
            InstrumentSpec::Option {
                osi_symbol: contract.osi_symbol.clone(),
            },
            Instruction::BuyToOpen,
            1.0,
            Some(0.01),
            None,
        )
        .await
        .expect("place option order failed");

    // Give the API a moment, then find the new order.
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let orders = client
        .get_orders(&hash)
        .await
        .expect("get_orders (after) failed");

    let placed = orders
        .iter()
        .find(|o| !ids_before.contains(&o.order_id) && o.symbol == contract.osi_symbol)
        .or_else(|| orders.iter().find(|o| o.symbol == contract.osi_symbol));
    let placed = placed.expect("placed option order not found in order list");
    println!(
        "Found order id={} sym={} qty={} status={:?}",
        placed.order_id, placed.symbol, placed.quantity, placed.status,
    );
    assert_eq!(placed.quantity, 1.0, "expected quantity 1");

    // Clean up. A deep-OTM $0.01 buy rests (Working/PendingActivation) and is
    // cancelable. Only cancel when actually cancelable so a terminal state
    // (e.g. a rejection from an account lacking options permissions) doesn't
    // turn cleanup into a spurious failure.
    if placed.cancelable {
        println!("Canceling resting order {}...", placed.order_id);
        client
            .cancel_order(&hash, placed.order_id)
            .await
            .expect("cancel_order failed");
        println!("Canceled. Option order place + cancel round-trip complete.");
    } else {
        println!(
            "Order is terminal ({:?}, not cancelable) — the request was structurally \
             accepted as an OPTION order, but it did not rest (check that the account \
             has options trading permissions). Nothing to cancel.",
            placed.status,
        );
    }
}

/// Re-verify the documented limitation (see SCHWAB_API_ISSUES.md / #75): the
/// transactions API never reports which tax lot or lot-selection method a sale
/// used. Pulls real SELL (Closing) trades and dumps them, then scans every
/// rendered field for any lot/method leakage (e.g. in a description string).
///
///   cargo test test_transactions_omit_lot_method -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn test_transactions_omit_lot_method() {
    use schwab_trader::types as tr;

    let Some((client, hash)) = setup().await else {
        return;
    };

    let txns = client
        .get_transactions_by_type(
            &hash,
            chrono::Utc::now() - chrono::Duration::days(365),
            chrono::Utc::now(),
            tr::TransactionType::Trade,
        )
        .await
        .expect("get_transactions failed");

    // A sell = a trade whose security transfer item closes a position.
    let mut sells: Vec<&tr::Transaction> = txns
        .iter()
        .filter(|t| {
            t.transfer_items.iter().any(|ti| {
                ti.position_effect == Some(tr::TransferItemPositionEffect::Closing)
                    && ti
                        .instrument
                        .as_ref()
                        .and_then(terminal_trader::schwab::transaction_instrument_symbol)
                        .is_some()
            })
        })
        .collect();
    sells.sort_by_key(|t| t.trade_date);

    println!(
        "Found {} SELL (closing) trade transaction(s) in the last 365 days.",
        sells.len()
    );
    if sells.is_empty() {
        println!("No sells to inspect — cannot confirm against live data this run.");
        return;
    }

    // Tokens that would indicate the API is exposing lot/method info anywhere.
    const NEEDLES: &[&str] = &[
        "lot",
        "fifo",
        "lifo",
        "hifo",
        "method",
        "high cost",
        "low cost",
        "average cost",
        "specific",
        "loss harvest",
    ];

    let mut leaked = false;
    for (i, t) in sells.iter().rev().take(3).enumerate() {
        println!("\n──────── SELL transaction #{} ────────", i + 1);
        let dump = format!("{t:#?}");
        println!("{dump}");
        let hay = dump.to_lowercase();
        for needle in NEEDLES {
            if hay.contains(needle) {
                println!(">>> POSSIBLE LEAK: rendered transaction contains {needle:?}");
                leaked = true;
            }
        }
    }

    println!("\n=== CONCLUSION ===");
    if leaked {
        println!(
            "A lot/method-related token appeared in a transaction — the #75 finding may \
             no longer hold; inspect the dump above."
        );
    } else {
        println!(
            "No lot or lot-selection-method information present in any sell transaction. \
             The #75 limitation still holds: the transactions API does not report which \
             lot/method a sale used."
        );
    }
    assert!(
        !leaked,
        "transactions API appears to expose lot/method info — revisit SCHWAB_API_ISSUES.md"
    );
}

/// Probe whether the ORDERS endpoint echoes back `tax_lot_method` on an order
/// placed with an explicit method. Places a SELL_TO_CLOSE on one option we
/// already hold at an impossible limit ($99 — far above market, cannot fill),
/// reads the raw order back, reports its `tax_lot_method`, then cancels and
/// confirms the cancellation. No execution occurs.
///
///   cargo test test_order_echoes_tax_lot_method -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn test_order_echoes_tax_lot_method() {
    use schwab_trader::types as tr;
    use std::collections::HashSet;

    let Some((client, hash)) = setup().await else {
        return;
    };

    // Find an option position we own (OSI symbols contain a space) to place a
    // non-filling closing sell against.
    let (positions, _acct) = client
        .get_positions(&hash)
        .await
        .expect("get_positions failed");
    let Some(pos) = positions
        .iter()
        .find(|p| p.quantity >= 1.0 && p.symbol.contains(' '))
    else {
        eprintln!("SKIP: no option position with qty>=1 to test a closing sell against");
        return;
    };
    let osi = pos.symbol.clone();
    let chosen = tr::TaxLotMethod::Fifo;
    println!(
        "Testing tax_lot_method echo with SELL_TO_CLOSE 1 {osi} @ $99 (cannot fill), \
         method={chosen:?}"
    );

    let ids_before: HashSet<i64> = client
        .get_orders(&hash)
        .await
        .expect("get_orders before")
        .into_iter()
        .map(|o| o.order_id)
        .collect();

    let leg = tr::OrderLegCollection {
        instruction: Some(tr::Instruction::SellToClose),
        instrument: Some(tr::AccountsInstrument::Option(tr::AccountOption {
            asset_type: tr::AccountOptionAssetType::Option,
            symbol: Some(osi.clone()),
            cusip: None,
            description: None,
            instrument_id: None,
            name: serde_json::Value::Null,
            net_change: None,
            option_deliverables: Vec::new(),
            option_multiplier: None,
            put_call: None,
            type_: None,
            underlying_symbol: None,
        })),
        quantity: Some(1.0),
        order_leg_type: Some(tr::OrderLegCollectionOrderLegType::Option),
        position_effect: Some(tr::OrderLegCollectionPositionEffect::Closing),
        ..Default::default()
    };
    let order = tr::OrderRequest {
        order_type: Some(tr::OrderTypeRequest::Limit),
        order_strategy_type: Some(tr::OrderStrategyType::Single),
        session: Some(tr::Session::Normal),
        duration: Some(tr::Duration::GoodTillCancel),
        price: Some(99.0),
        order_leg_collection: vec![leg],
        tax_lot_method: Some(chosen),
        ..Default::default()
    };

    let trader = client.trader().await;
    if let Err(e) = trader
        .place_order()
        .account_number(&hash)
        .body(order)
        .send()
        .await
    {
        eprintln!("place_order with tax_lot_method was REJECTED: {e:#}");
        eprintln!(
            "=> Inconclusive for the echo question — retry with an equity sell during \
             market hours."
        );
        return;
    }
    println!("Order accepted (tax_lot_method not rejected). Reading it back...");

    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let from = (chrono::Utc::now() - chrono::Duration::days(1))
        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
        .to_string();
    let to = chrono::Utc::now()
        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
        .to_string();
    let raw = trader
        .get_orders_by_path_param()
        .account_number(&hash)
        .from_entered_time(from)
        .to_entered_time(to)
        .send()
        .await
        .expect("raw get_orders failed")
        .into_inner();

    // The order we just placed is the newest one not present before.
    let placed = raw
        .iter()
        .filter(|o| o.order_id.is_some_and(|id| !ids_before.contains(&id)))
        .max_by_key(|o| o.order_id)
        .expect("placed order not found in raw order list");
    let order_id = placed.order_id.expect("placed order missing id");

    println!("\n=== ORDERS endpoint ===");
    println!("order_id={order_id} status={:?}", placed.status);
    match placed.tax_lot_method {
        Some(m) => println!(">>> ECHOED: orders endpoint returned tax_lot_method = {m:?}"),
        None => println!(">>> NOT echoed: tax_lot_method is None on the read-back order"),
    }

    // Clean up: cancel and confirm.
    if placed.cancelable {
        client
            .cancel_order(&hash, order_id)
            .await
            .expect("cancel_order failed");
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        let after = client
            .get_orders(&hash)
            .await
            .expect("get_orders after cancel");
        let still_live = after.iter().any(|o| {
            o.order_id == order_id
                && !matches!(o.status, terminal_trader::schwab::OrderStatus::Canceled)
        });
        assert!(
            !still_live,
            "order {order_id} was not canceled — manual cleanup needed!"
        );
        println!("Canceled and confirmed. No position was touched.");
    } else {
        panic!(
            "order {order_id} is not cancelable (status {:?}) — manual cleanup may be needed",
            placed.status
        );
    }
}

/// Live equity confirmation that an EQUITY sell placed with an explicit
/// `tax_lot_method` echoes back on the orders endpoint (via our `Order` model).
/// Buys 1 HOG to acquire a share, sells it with FIFO, then reads the sell order
/// back and asserts `tax_lot_method == Fifo`. Round-trips the share (no leftover
/// position). Needs market/extended hours so the orders fill.
///
///   cargo test test_equity_sell_tax_lot_method_echo -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn test_equity_sell_tax_lot_method_echo() {
    use schwab_trader::types as tr;
    use std::collections::HashSet;
    use terminal_trader::schwab::{Instruction, LotSelectionMethod};

    let Some((client, hash)) = setup().await else {
        return;
    };

    let quotes = client
        .get_quotes(&["HOG"])
        .await
        .expect("get_quotes failed");
    let q = quotes.get("HOG").expect("no HOG quote");
    // Cross the spread on both sides to maximize fill odds in extended hours.
    let buy_limit = if q.ask_price > 0.0 {
        q.ask_price + 0.05
    } else {
        q.last_price + 0.15
    };

    let ids_before_buy: HashSet<i64> = client
        .get_orders(&hash)
        .await
        .expect("get_orders (before buy) failed")
        .into_iter()
        .map(|o| o.order_id)
        .collect();

    println!("Placing BUY 1 HOG limit @ {buy_limit:.2}...");
    client
        .place_order(
            &hash,
            InstrumentSpec::Equity {
                symbol: "HOG".to_string(),
            },
            Instruction::Buy,
            1.0,
            Some(buy_limit),
            None,
        )
        .await
        .expect("place_order (buy) failed");
    let buy_id = poll_for_fill(&client, &hash, &ids_before_buy, 90)
        .await
        .expect("BUY HOG did not fill within 90s (market closed?)");
    println!("BUY filled order_id={buy_id}");

    // Re-quote, then SELL 1 HOG with an explicit FIFO tax lot method.
    let quotes2 = client.get_quotes(&["HOG"]).await.expect("requote failed");
    let q2 = quotes2.get("HOG").expect("no HOG requote");
    let sell_limit = if q2.bid_price > 0.0 {
        q2.bid_price - 0.05
    } else {
        q2.last_price - 0.15
    };

    let ids_before_sell: HashSet<i64> = client
        .get_orders(&hash)
        .await
        .expect("get_orders (before sell) failed")
        .into_iter()
        .map(|o| o.order_id)
        .collect();

    println!("Placing SELL 1 HOG limit @ {sell_limit:.2} with tax_lot_method=FIFO...");
    client
        .place_order(
            &hash,
            InstrumentSpec::Equity {
                symbol: "HOG".to_string(),
            },
            Instruction::Sell,
            1.0,
            Some(sell_limit),
            Some(tr::TaxLotMethod::Fifo),
        )
        .await
        .expect("place_order (sell w/ tax_lot_method) failed");
    let sell_id = poll_for_fill(&client, &hash, &ids_before_sell, 90)
        .await
        .expect("SELL HOG did not fill within 90s");
    println!("SELL filled order_id={sell_id}");

    // Read the sell order back and confirm the method echoes through our model.
    let orders = client
        .get_orders(&hash)
        .await
        .expect("get_orders (after sell) failed");
    let sell_order = orders
        .iter()
        .find(|o| o.order_id == sell_id)
        .expect("sell order not found");
    println!(
        "\n=== EQUITY ORDERS endpoint ===\norder_id={sell_id} tax_lot_method={:?}",
        sell_order.tax_lot_method
    );
    assert_eq!(
        sell_order.tax_lot_method,
        Some(LotSelectionMethod::Fifo),
        "equity sell order should echo back tax_lot_method = FIFO"
    );
    println!(">>> CONFIRMED: equity sell echoes tax_lot_method = FIFO. Share round-tripped.");
}
