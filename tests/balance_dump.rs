/// Quick test to dump all balance fields from the Schwab API.
///
/// Run with:
///   cargo test --test balance_dump -- --ignored --nocapture
use terminal_trader::schwab::{SchwabAuth, SchwabClient};

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

/// Dump cash-relevant balances for EVERY account, labeled by nickname.
/// Run with:
///   cargo test --test balance_dump test_dump_all_accounts -- --ignored --nocapture
#[tokio::test]
#[ignore]
async fn test_dump_all_accounts() {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("SCHWAB_API_KEY").expect("SCHWAB_API_KEY");
    let app_secret = std::env::var("SCHWAB_APP_SECRET").expect("SCHWAB_APP_SECRET");
    let token_path =
        std::env::var("SCHWAB_TOKEN_PATH").unwrap_or_else(|_| "schwab_tokens.json".to_string());
    let auth = SchwabAuth::new(api_key, app_secret, token_path);
    let client = SchwabClient::new(auth).await.expect("client");

    let accounts = client.get_accounts().await.expect("get_accounts");
    let token = client.access_token().await.expect("access_token");
    let http = reqwest::Client::new();

    for acct in &accounts {
        let nick = acct.nickname.as_deref().unwrap_or("(no nickname)");
        let url = format!(
            "https://api.schwabapi.com/trader/v1/accounts/{}?fields=positions",
            acct.hash
        );
        let raw: serde_json::Value = http
            .get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .expect("req")
            .json()
            .await
            .expect("json");
        let cb = raw
            .pointer("/securitiesAccount/currentBalances")
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        let get = |k: &str| cb.get(k).and_then(|v| v.as_f64());
        println!("=== {nick} ({}) ===", acct.number);
        println!("  cashBalance:        {:?}", get("cashBalance"));
        println!("  marginBalance:      {:?}", get("marginBalance"));
        println!("  shortBalance:       {:?}", get("shortBalance"));
        println!("  availableFunds:     {:?}", get("availableFunds"));
        println!("  buyingPower:        {:?}", get("buyingPower"));
        println!("  equity:             {:?}", get("equity"));
        println!("  liquidationValue:   {:?}", get("liquidationValue"));
        println!("  longOptionMktValue: {:?}", get("longOptionMarketValue"));
        println!("  longMarketValue:    {:?}", get("longMarketValue"));
        println!();
    }
}

#[tokio::test]
#[ignore]
async fn test_dump_balances() {
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

    // Also do a raw HTTP call to see the full JSON
    let token = client.access_token().await.expect("access_token failed");
    let raw_url = format!(
        "https://api.schwabapi.com/trader/v1/accounts/{}?fields=positions",
        hash
    );
    let http = reqwest::Client::new();
    let raw_resp = http
        .get(&raw_url)
        .bearer_auth(&token)
        .send()
        .await
        .expect("raw request failed");
    let raw_json: serde_json::Value = raw_resp.json().await.expect("json parse failed");

    // Print just the balances from raw JSON
    if let Some(acct) = raw_json.get("securitiesAccount") {
        println!("=== RAW JSON currentBalances ===");
        if let Some(cb) = acct.get("currentBalances") {
            println!("{}", serde_json::to_string_pretty(cb).unwrap());
        }
        println!("\n=== RAW JSON initialBalances ===");
        if let Some(ib) = acct.get("initialBalances") {
            println!("{}", serde_json::to_string_pretty(ib).unwrap());
        }
        println!("\n=== RAW JSON projectedBalances ===");
        if let Some(pb) = acct.get("projectedBalances") {
            println!("{}", serde_json::to_string_pretty(pb).unwrap());
        }
    }

    println!("\n========================================\n");

    match account.securities_account {
        Some(schwab_trader::types::SecuritiesAccount::MarginAccount(a)) => {
            println!("=== Account type: MARGIN ===\n");

            if let Some(ref bal) = a.current_balances {
                println!("--- currentBalances (MarginBalance) ---");
                println!(
                    "  available_funds:                    {:?}",
                    bal.available_funds
                );
                println!(
                    "  available_funds_non_marginable:     {:?}",
                    bal.available_funds_non_marginable_trade
                );
                println!(
                    "  buying_power:                       {:?}",
                    bal.buying_power
                );
                println!(
                    "  buying_power_non_marginable_trade:  {:?}",
                    bal.buying_power_non_marginable_trade
                );
                println!(
                    "  cash_balance:                       {:?}",
                    bal.cash_balance
                );
                println!(
                    "  liquidation_value:                  {:?}",
                    bal.liquidation_value
                );
                println!(
                    "  day_trading_buying_power:           {:?}",
                    bal.day_trading_buying_power
                );
                println!("  equity:                             {:?}", bal.equity);
                println!(
                    "  equity_percentage:                  {:?}",
                    bal.equity_percentage
                );
                println!(
                    "  long_margin_value:                  {:?}",
                    bal.long_margin_value
                );
                println!(
                    "  maintenance_call:                   {:?}",
                    bal.maintenance_call
                );
                println!(
                    "  maintenance_requirement:            {:?}",
                    bal.maintenance_requirement
                );
                println!(
                    "  margin_balance:                     {:?}",
                    bal.margin_balance
                );
                println!(
                    "  option_buying_power:                {:?}",
                    bal.option_buying_power
                );
                println!("  reg_t_call:                         {:?}", bal.reg_t_call);
                println!(
                    "  short_balance:                      {:?}",
                    bal.short_balance
                );
                println!(
                    "  short_margin_value:                 {:?}",
                    bal.short_margin_value
                );
                println!("  sma:                                {:?}", bal.sma);
                println!(
                    "  stock_buying_power:                 {:?}",
                    bal.stock_buying_power
                );
            } else {
                println!("currentBalances: None");
            }

            if let Some(ref bal) = a.initial_balances {
                println!("\n--- initialBalances (MarginInitialBalance) ---");
                println!(
                    "  account_value:                      {:?}",
                    bal.account_value
                );
                println!(
                    "  accrued_interest:                   {:?}",
                    bal.accrued_interest
                );
                println!(
                    "  available_funds_non_marginable:     {:?}",
                    bal.available_funds_non_marginable_trade
                );
                println!("  bond_value:                         {:?}", bal.bond_value);
                println!(
                    "  buying_power:                       {:?}",
                    bal.buying_power
                );
                println!(
                    "  cash_available_for_trading:         {:?}",
                    bal.cash_available_for_trading
                );
                println!(
                    "  cash_balance:                       {:?}",
                    bal.cash_balance
                );
                println!(
                    "  cash_receipts:                      {:?}",
                    bal.cash_receipts
                );
                println!(
                    "  day_trading_buying_power:           {:?}",
                    bal.day_trading_buying_power
                );
                println!("  equity:                             {:?}", bal.equity);
                println!(
                    "  equity_percentage:                  {:?}",
                    bal.equity_percentage
                );
                println!(
                    "  liquidation_value:                  {:?}",
                    bal.liquidation_value
                );
                println!(
                    "  long_margin_value:                  {:?}",
                    bal.long_margin_value
                );
                println!(
                    "  long_option_market_value:           {:?}",
                    bal.long_option_market_value
                );
                println!(
                    "  long_stock_value:                   {:?}",
                    bal.long_stock_value
                );
                println!(
                    "  maintenance_call:                   {:?}",
                    bal.maintenance_call
                );
                println!(
                    "  maintenance_requirement:            {:?}",
                    bal.maintenance_requirement
                );
                println!("  margin:                             {:?}", bal.margin);
                println!(
                    "  margin_balance:                     {:?}",
                    bal.margin_balance
                );
                println!(
                    "  money_market_fund:                  {:?}",
                    bal.money_market_fund
                );
                println!(
                    "  mutual_fund_value:                  {:?}",
                    bal.mutual_fund_value
                );
                println!(
                    "  pending_deposits:                   {:?}",
                    bal.pending_deposits
                );
                println!("  reg_t_call:                         {:?}", bal.reg_t_call);
                println!(
                    "  short_balance:                      {:?}",
                    bal.short_balance
                );
                println!(
                    "  short_margin_value:                 {:?}",
                    bal.short_margin_value
                );
                println!(
                    "  short_option_market_value:          {:?}",
                    bal.short_option_market_value
                );
                println!(
                    "  short_stock_value:                  {:?}",
                    bal.short_stock_value
                );
                println!("  total_cash:                         {:?}", bal.total_cash);
                println!(
                    "  unsettled_cash:                     {:?}",
                    bal.unsettled_cash
                );
            } else {
                println!("initialBalances: None");
            }

            if let Some(ref bal) = a.projected_balances {
                println!("\n--- projectedBalances (MarginBalance) ---");
                println!(
                    "  available_funds:                    {:?}",
                    bal.available_funds
                );
                println!(
                    "  available_funds_non_marginable:     {:?}",
                    bal.available_funds_non_marginable_trade
                );
                println!(
                    "  buying_power:                       {:?}",
                    bal.buying_power
                );
                println!(
                    "  buying_power_non_marginable_trade:  {:?}",
                    bal.buying_power_non_marginable_trade
                );
                println!(
                    "  margin_balance:                     {:?}",
                    bal.margin_balance
                );
                println!(
                    "  stock_buying_power:                 {:?}",
                    bal.stock_buying_power
                );
            } else {
                println!("projectedBalances: None");
            }
        }
        Some(schwab_trader::types::SecuritiesAccount::CashAccount(a)) => {
            println!("=== Account type: CASH ===");
            println!("currentBalances: {:?}", a.current_balances);
            println!("initialBalances: {:?}", a.initial_balances);
            println!("projectedBalances: {:?}", a.projected_balances);
        }
        None => {
            println!("No securities_account returned");
        }
    }
}
