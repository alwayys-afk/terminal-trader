use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    // Handle --seed before setting up TUI logging.
    // Usage: --seed <csv> [--lot-method <fifo|lifo|hifo>]
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 && args[1] == "--seed" {
        use terminal_trader::schwab::LotSelectionMethod;
        let csv_path = args[2].clone();
        let lot_method = match args.iter().position(|a| a == "--lot-method") {
            Some(i) => match args.get(i + 1) {
                Some(m) => Some(m.parse::<LotSelectionMethod>().map_err(|_| {
                    anyhow::anyhow!("--lot-method expects fifo|lifo|hifo, got {m:?}")
                })?),
                None => anyhow::bail!("--lot-method expects fifo|lifo|hifo, got (missing)"),
            },
            None => None,
        };
        let lot_store_path = terminal_trader::app::seed::pick_seed_lot_store_path().await?;
        return terminal_trader::app::seed::seed_from_csv(&csv_path, &lot_store_path, lot_method);
    }

    // Log to file so it doesn't interfere with the TUI.
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("trader.log")?;
    tracing_subscriber::fmt()
        .with_writer(log_file)
        .with_ansi(false)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("terminal_trader=info")),
        )
        .init();

    terminal_trader::app::run().await
}
