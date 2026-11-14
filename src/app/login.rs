//! The terminal side of the Schwab login, run at startup before the TUI takes
//! over the terminal (and by `--seed`). Printing and reading stdin is fine
//! here; it is not fine anywhere in `schwab::auth`.

use anyhow::{Context, Result};
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::schwab::{AuthError, CALLBACK_PORT, DEFAULT_LOGIN_TIMEOUT, REDIRECT_URI, SchwabAuth};

/// Make sure there is a usable saved login, running the browser login if
/// there isn't one.
pub async fn ensure_logged_in(auth: &SchwabAuth) -> Result<()> {
    match auth.get_valid_token().await {
        Ok(_) => Ok(()),
        Err(AuthError::LoginRequired { reason }) => {
            println!("Schwab login needed: {reason}.");
            browser_login(auth).await
        }
        Err(e) => Err(e).context("checking the saved Schwab login"),
    }
}

async fn browser_login(auth: &SchwabAuth) -> Result<()> {
    let result = auth
        .login(|url| {
            println!("Opening your browser to log in to Schwab.");
            println!("If it doesn't open, visit:\n  {url}");
            println!(
                "After you log in, your browser will warn that the connection to 127.0.0.1 isn't \
                 private. That's terminal-trader's own local certificate: click Advanced, then \
                 Proceed to 127.0.0.1."
            );
            println!(
                "Waiting up to {} minutes for the login to finish...",
                DEFAULT_LOGIN_TIMEOUT.as_secs() / 60
            );
            open::that(url).ok();
        })
        .await;

    match result {
        Ok(_) => {
            println!("Logged in.");
            Ok(())
        }
        Err(AuthError::PortInUse { .. }) => {
            println!(
                "Port {CALLBACK_PORT} is busy, so the login can't be received automatically. \
                 Falling back to pasting the redirect URL."
            );
            paste_login(auth).await
        }
        Err(e) => Err(e.into()),
    }
}

/// The old flow: the user copies the URL the browser was redirected to.
async fn paste_login(auth: &SchwabAuth) -> Result<()> {
    let state = auth.new_login_state();
    let url = auth.authorize_url(&state);
    println!("Visit:\n  {url}");
    open::that(&url).ok();
    println!(
        "After logging in, copy the full address the browser ends up on \
         (it starts with {REDIRECT_URI}/?code=) and paste it here:"
    );
    let mut line = String::new();
    BufReader::new(tokio::io::stdin())
        .read_line(&mut line)
        .await
        .context("reading the redirect URL")?;
    auth.login_from_redirect_url(line.trim(), &state).await?;
    println!("Logged in.");
    Ok(())
}
