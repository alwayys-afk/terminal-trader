//! Schwab OAuth: loading, refreshing and saving the token, and the browser
//! login.
//!
//! [`SchwabAuth::get_valid_token`] never prompts. When the user has to log in
//! again it returns [`AuthError::LoginRequired`], and the caller decides how to
//! run [`SchwabAuth::login`]: the TUI does it at startup, before it takes over
//! the terminal. The login serves the OAuth redirect itself on
//! `https://127.0.0.1:8182` with a self-signed certificate, so nothing has to
//! be pasted.

mod callback;
mod cert;
mod store;

use std::path::PathBuf;
use std::time::Duration;

use anyhow::Context;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

pub use callback::CallbackServer;
use store::{LoadError, TokenStore};

const TOKEN_URL: &str = "https://api.schwabapi.com/v1/oauth/token";
const AUTH_URL: &str = "https://api.schwabapi.com/v1/oauth/authorize";

/// The callback URL registered with the Schwab app. Must match exactly.
pub const REDIRECT_URI: &str = "https://127.0.0.1:8182";
/// The port in [`REDIRECT_URI`].
pub const CALLBACK_PORT: u16 = 8182;
/// How long a login waits for the browser redirect by default.
pub const DEFAULT_LOGIN_TIMEOUT: Duration = Duration::from_secs(5 * 60);
/// Refresh the access token when it expires within this many seconds.
pub(crate) const REFRESH_BUFFER_SECS: i64 = 60;
/// Timeout for each call to Schwab's token endpoint.
const TOKEN_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    /// No usable saved login: the user has to log in in a browser.
    #[error("Schwab login required: {reason}")]
    LoginRequired { reason: String },
    /// Schwab's token endpoint couldn't be reached or had a server error.
    /// The saved token is left as it was; retry later.
    #[error("couldn't refresh the Schwab login right now: {0}")]
    Transient(String),
    #[error(
        "port {port} is already in use, so the login callback can't be received \
         (is another terminal-trader login running?)"
    )]
    PortInUse { port: u16 },
    #[error("timed out after {} seconds waiting for the Schwab login in the browser", .0.as_secs())]
    LoginTimedOut(Duration),
    /// Schwab redirected back with an error, or rejected the authorization code.
    #[error("Schwab login failed: {0}")]
    LoginDenied(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub refresh_expires_at: DateTime<Utc>,
}

impl TokenData {
    pub fn access_token_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    pub fn refresh_token_expired(&self) -> bool {
        Utc::now() >= self.refresh_expires_at
    }

    /// True when the access token is expired or expires within the refresh
    /// buffer.
    fn access_token_due(&self) -> bool {
        Utc::now() + chrono::Duration::seconds(REFRESH_BUFFER_SECS) >= self.expires_at
    }
}

/// Settings for [`SchwabAuth::login_with`].
#[derive(Debug, Clone)]
pub struct LoginOptions {
    /// Port for the callback listener. Schwab only redirects to
    /// [`CALLBACK_PORT`]; other values are for tests.
    pub port: u16,
    pub timeout: Duration,
    /// Where the listener's self-signed certificate is kept.
    pub cert_dir: PathBuf,
}

impl Default for LoginOptions {
    fn default() -> Self {
        Self {
            port: CALLBACK_PORT,
            timeout: DEFAULT_LOGIN_TIMEOUT,
            cert_dir: crate::data_dir::data_dir(),
        }
    }
}

/// How a call to the token endpoint failed.
enum TokenCallError {
    /// Network error, timeout, 429 or 5xx.
    Transient(String),
    /// Schwab refused the request (4xx); the message carries its error code.
    Rejected(String),
    Other(anyhow::Error),
}

#[derive(Clone)]
pub struct SchwabAuth {
    api_key: String,
    app_secret: String,
    store: TokenStore,
    token_url: String,
    http: reqwest::Client,
}

impl SchwabAuth {
    pub fn new(api_key: String, app_secret: String, token_path: impl Into<PathBuf>) -> Self {
        let http = reqwest::Client::builder()
            .timeout(TOKEN_REQUEST_TIMEOUT)
            .build()
            .unwrap_or_default();
        Self {
            api_key,
            app_secret,
            store: TokenStore::new(token_path.into()),
            token_url: TOKEN_URL.to_string(),
            http,
        }
    }

    /// Point token requests at a test server instead of Schwab.
    #[cfg(test)]
    fn with_token_url(mut self, url: impl Into<String>) -> Self {
        self.token_url = url.into();
        self
    }

    /// The saved token, refreshed if the access token is due. Never prompts:
    /// returns [`AuthError::LoginRequired`] when a browser login is needed.
    ///
    /// Safe to call from several processes sharing the token file: the
    /// refresh happens under a file lock, and a process that waited on the
    /// lock uses the token the other one saved.
    pub async fn get_valid_token(&self) -> Result<TokenData, AuthError> {
        let token = self.load_usable()?;
        if !token.access_token_due() {
            return Ok(token);
        }

        let _lock = self.store.lock().await.context("locking the token file")?;
        // Another process may have refreshed while we waited for the lock.
        let token = self.load_usable()?;
        if !token.access_token_due() {
            info!("using the access token another process just refreshed");
            return Ok(token);
        }
        self.refresh_access_token(&token).await
    }

    /// Run the browser login: start the HTTPS callback listener, hand the
    /// authorize URL to `on_url` (to print or open), wait for Schwab's
    /// redirect, exchange the code and save the token.
    pub async fn login(&self, on_url: impl FnOnce(&str)) -> Result<TokenData, AuthError> {
        self.login_with(&LoginOptions::default(), on_url).await
    }

    pub async fn login_with(
        &self,
        opts: &LoginOptions,
        on_url: impl FnOnce(&str),
    ) -> Result<TokenData, AuthError> {
        let tls = cert::server_config(&opts.cert_dir)?;
        let server = CallbackServer::bind(opts.port, tls).await?;
        let state = new_state();
        on_url(&self.authorize_url(&state));
        let code = server.wait_for_code(&state, opts.timeout).await?;
        drop(server);
        self.complete_login(&code).await
    }

    /// A fresh random `state` for a login.
    pub fn new_login_state(&self) -> String {
        new_state()
    }

    /// The Schwab authorize URL for a login with this `state`. Same shape as
    /// the URL the app has always used (redirect URI unencoded).
    pub fn authorize_url(&self, state: &str) -> String {
        format!(
            "{AUTH_URL}?response_type=code&client_id={}&redirect_uri={REDIRECT_URI}&state={state}",
            self.api_key,
        )
    }

    /// Finish a login from a redirect URL the user pasted (the fallback when
    /// the callback port can't be used). Checks `state`.
    pub async fn login_from_redirect_url(
        &self,
        redirect_url: &str,
        state: &str,
    ) -> Result<TokenData, AuthError> {
        let query = redirect_url.split_once('?').map(|(_, q)| q).unwrap_or("");
        let path_and_query = format!("/?{query}");
        match callback::classify(&path_and_query, state) {
            callback::Outcome::Code(code) => self.complete_login(&code).await,
            callback::Outcome::Denied(reason) => Err(AuthError::LoginDenied(reason)),
            callback::Outcome::StateMismatch => Err(AuthError::LoginDenied(
                "that redirect URL is from a different login attempt".to_string(),
            )),
            callback::Outcome::Ignored => Err(AuthError::LoginDenied(
                "no authorization code in that URL".to_string(),
            )),
        }
    }

    /// Exchange an authorization code and save the token.
    async fn complete_login(&self, code: &str) -> Result<TokenData, AuthError> {
        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", REDIRECT_URI),
        ];
        let response = self
            .call_token_endpoint(&params)
            .await
            .map_err(|e| match e {
                TokenCallError::Rejected(msg) => AuthError::LoginDenied(format!(
                    "Schwab rejected the authorization code ({msg})"
                )),
                TokenCallError::Transient(msg) => AuthError::Transient(msg),
                TokenCallError::Other(e) => AuthError::Other(e),
            })?;
        let token = parse_token_response(&response, None, None)?;

        // Lock only around the save, never across the browser wait.
        let _lock = self.store.lock().await.context("locking the token file")?;
        self.store.save(&token).context("saving the token file")?;
        info!(refresh_expires_at = %token.refresh_expires_at, "logged in to Schwab");
        Ok(token)
    }

    /// Load the saved token, or say why a login is needed.
    fn load_usable(&self) -> Result<TokenData, AuthError> {
        let token = self.store.load().map_err(|e| match e {
            LoadError::Missing => AuthError::LoginRequired {
                reason: "no saved login".to_string(),
            },
            LoadError::Unreadable(err) => AuthError::LoginRequired {
                reason: format!("the saved token file can't be read ({err})"),
            },
            LoadError::Io(err) => AuthError::Other(
                anyhow::Error::new(err).context(format!("reading {}", self.store.path().display())),
            ),
        })?;
        if token.refresh_token_expired() {
            return Err(AuthError::LoginRequired {
                reason: format!(
                    "the refresh token expired at {} (Schwab requires a new login every 7 days)",
                    token.refresh_expires_at.format("%Y-%m-%d %H:%M UTC")
                ),
            });
        }
        Ok(token)
    }

    /// Refresh the access token and save it. Callers hold the token lock.
    async fn refresh_access_token(&self, token: &TokenData) -> Result<TokenData, AuthError> {
        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", token.refresh_token.as_str()),
        ];
        let response = self
            .call_token_endpoint(&params)
            .await
            .map_err(|e| match e {
                TokenCallError::Rejected(msg) => {
                    warn!("Schwab rejected the refresh token: {msg}");
                    AuthError::LoginRequired {
                        reason: format!("Schwab rejected the saved login ({msg})"),
                    }
                }
                TokenCallError::Transient(msg) => {
                    warn!("token refresh failed, will retry: {msg}");
                    AuthError::Transient(msg)
                }
                TokenCallError::Other(e) => AuthError::Other(e),
            })?;

        let new_token = parse_token_response(
            &response,
            Some(&token.refresh_token),
            Some(token.refresh_expires_at),
        )?;
        self.store
            .save(&new_token)
            .context("saving the token file")?;
        info!(
            refresh_expires_at = %new_token.refresh_expires_at,
            "access token refreshed"
        );
        Ok(new_token)
    }

    async fn call_token_endpoint(
        &self,
        params: &[(&str, &str)],
    ) -> Result<serde_json::Value, TokenCallError> {
        let response = self
            .http
            .post(&self.token_url)
            .basic_auth(&self.api_key, Some(&self.app_secret))
            .form(params)
            .send()
            .await
            .map_err(|e| TokenCallError::Transient(e.without_url().to_string()))?;

        let status = response.status();
        if status.is_success() {
            return response.json().await.map_err(|e| {
                TokenCallError::Other(
                    anyhow::Error::new(e.without_url()).context("parsing token response"),
                )
            });
        }

        let body: serde_json::Value = response.json().await.unwrap_or_default();
        let detail = token_error_detail(status, &body);
        if status.is_server_error() || status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            Err(TokenCallError::Transient(detail))
        } else {
            Err(TokenCallError::Rejected(detail))
        }
    }
}

/// "HTTP 400 invalid_grant: ..." from an OAuth error body. Only the standard
/// `error` / `error_description` fields are used; they hold no secrets.
fn token_error_detail(status: reqwest::StatusCode, body: &serde_json::Value) -> String {
    let mut detail = format!("HTTP {}", status.as_u16());
    if let Some(error) = body["error"].as_str() {
        detail.push(' ');
        detail.push_str(error);
    }
    if let Some(desc) = body["error_description"].as_str() {
        let desc: String = desc.chars().take(200).collect();
        detail.push_str(": ");
        detail.push_str(&desc);
    }
    detail
}

fn new_state() -> String {
    let mut bytes = [0u8; 16];
    getrandom::getrandom(&mut bytes).expect("OS random number generator unavailable");
    URL_SAFE_NO_PAD.encode(bytes)
}

fn parse_token_response(
    response: &serde_json::Value,
    existing_refresh: Option<&str>,
    existing_refresh_expiry: Option<DateTime<Utc>>,
) -> Result<TokenData, AuthError> {
    let expires_in = response["expires_in"].as_i64();
    let scope = response["scope"].as_str();
    info!(?expires_in, ?scope, "token endpoint response");

    let access_token = response["access_token"]
        .as_str()
        .context("token response is missing access_token")?
        .to_string();

    let expires_in = expires_in.unwrap_or(1800);
    let expires_at = Utc::now() + chrono::Duration::seconds(expires_in);

    let (refresh_token, refresh_expires_at) = if let Some(rt) = response["refresh_token"].as_str() {
        // Schwab doesn't return a refresh token expiry field.
        // Keep the existing expiry during refreshes; only assume
        // 7 days on the initial auth flow.
        let refresh_expires_at =
            existing_refresh_expiry.unwrap_or(Utc::now() + chrono::Duration::days(7));
        (rt.to_string(), refresh_expires_at)
    } else {
        (
            existing_refresh
                .context("no refresh token available")?
                .to_string(),
            existing_refresh_expiry.context("no refresh expiry available")?,
        )
    };

    Ok(TokenData {
        access_token,
        refresh_token,
        expires_at,
        refresh_expires_at,
    })
}

#[cfg(test)]
mod tests;
