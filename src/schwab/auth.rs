use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::{info, warn};

const TOKEN_URL: &str = "https://api.schwabapi.com/v1/oauth/token";
const AUTH_URL: &str = "https://api.schwabapi.com/v1/oauth/authorize";

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
}

#[derive(Clone)]
pub struct SchwabAuth {
    api_key: String,
    app_secret: String,
    token_path: PathBuf,
    http: reqwest::Client,
}

impl SchwabAuth {
    pub fn new(api_key: String, app_secret: String, token_path: impl Into<PathBuf>) -> Self {
        Self {
            api_key,
            app_secret,
            token_path: token_path.into(),
            http: reqwest::Client::new(),
        }
    }

    /// Load token from disk, refreshing if expired. Triggers full OAuth flow if needed.
    pub async fn get_valid_token(&self) -> Result<TokenData> {
        if let Ok(token) = self.load_token() {
            if token.refresh_token_expired() {
                info!("refresh token expired — re-authenticating");
                return self.full_auth_flow().await;
            }
            if token.access_token_expired() {
                match self.refresh_access_token(&token).await {
                    Ok(new_token) => return Ok(new_token),
                    Err(e) => {
                        warn!("refresh failed ({e:#}), falling back to full re-auth");
                        return self.full_auth_flow().await;
                    }
                }
            }
            return Ok(token);
        }

        self.full_auth_flow().await
    }

    fn load_token(&self) -> Result<TokenData> {
        let data = std::fs::read_to_string(&self.token_path).context("token file not found")?;
        serde_json::from_str(&data).context("failed to parse token file")
    }

    fn save_token(&self, token: &TokenData) -> Result<()> {
        let data = serde_json::to_string_pretty(token)?;
        std::fs::write(&self.token_path, data)?;
        Ok(())
    }

    async fn refresh_access_token(&self, token: &TokenData) -> Result<TokenData> {
        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", token.refresh_token.as_str()),
        ];

        let response: serde_json::Value = self
            .http
            .post(TOKEN_URL)
            .basic_auth(&self.api_key, Some(&self.app_secret))
            .form(&params)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        let new_token = self.parse_token_response(
            response,
            Some(token.refresh_token.clone()),
            Some(token.refresh_expires_at),
        )?;
        info!(
            refresh_expires_at = %new_token.refresh_expires_at,
            "refresh token updated"
        );
        self.save_token(&new_token)?;
        Ok(new_token)
    }

    async fn full_auth_flow(&self) -> Result<TokenData> {
        use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
        use tokio::io::{AsyncBufReadExt, BufReader};

        // Build auth URL
        let state: String = {
            let mut bytes = [0u8; 16];
            getrandom::getrandom(&mut bytes).ok();
            URL_SAFE_NO_PAD.encode(bytes)
        };

        let auth_url = format!(
            "{}?response_type=code&client_id={}&redirect_uri=https://127.0.0.1:8182&state={}",
            AUTH_URL, self.api_key, state
        );

        println!("Opening browser for Schwab authentication...");
        println!("If the browser doesn't open, visit:\n  {}", auth_url);
        open::that(&auth_url).ok();

        println!("\nAfter authorizing, paste the full redirect URL here:");
        print!("> ");
        let stdin = tokio::io::stdin();
        let mut reader = BufReader::new(stdin);
        let mut redirect_url = String::new();
        reader.read_line(&mut redirect_url).await?;
        let redirect_url = redirect_url.trim();

        // Extract code from redirect URL
        let code = extract_query_param(redirect_url, "code")
            .context("could not find 'code' in redirect URL")?;

        let params = [
            ("grant_type", "authorization_code"),
            ("code", &code),
            ("redirect_uri", "https://127.0.0.1:8182"),
        ];

        let response: serde_json::Value = self
            .http
            .post(TOKEN_URL)
            .basic_auth(&self.api_key, Some(&self.app_secret))
            .form(&params)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        let token = self.parse_token_response(response, None, None)?;
        self.save_token(&token)?;
        Ok(token)
    }

    fn parse_token_response(
        &self,
        response: serde_json::Value,
        existing_refresh: Option<String>,
        existing_refresh_expiry: Option<DateTime<Utc>>,
    ) -> Result<TokenData> {
        let expires_in = response["expires_in"].as_i64();
        let scope = response["scope"].as_str();
        info!(?expires_in, ?scope, "token endpoint response");

        let access_token = response["access_token"]
            .as_str()
            .context("missing access_token")?
            .to_string();

        let expires_in = response["expires_in"].as_i64().unwrap_or(1800);
        let expires_at = Utc::now() + chrono::Duration::seconds(expires_in);

        let (refresh_token, refresh_expires_at) =
            if let Some(rt) = response["refresh_token"].as_str() {
                // Schwab doesn't return a refresh token expiry field.
                // Keep the existing expiry during refreshes; only assume
                // 7 days on the initial auth flow.
                let refresh_expires_at =
                    existing_refresh_expiry.unwrap_or(Utc::now() + chrono::Duration::days(7));
                (rt.to_string(), refresh_expires_at)
            } else {
                (
                    existing_refresh.context("no refresh token available")?,
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
}

fn extract_query_param(url: &str, key: &str) -> Option<String> {
    let query = url.split('?').nth(1)?;
    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        if parts.next()? == key {
            return Some(
                urlencoding::decode(parts.next().unwrap_or(""))
                    .unwrap_or_default()
                    .into_owned(),
            );
        }
    }
    None
}
