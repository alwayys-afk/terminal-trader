//! The browser login as the MCP server runs it.
//!
//! A login runs in the background: the callback listener is started, the
//! browser opened, and the tool can return the URL right away while the
//! listener keeps waiting (up to [`DEFAULT_LOGIN_TIMEOUT`]). A later `login`
//! call with `wait` (or an elicitation round trip) waits for it to finish.

use std::process::Stdio;
use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::sync::{Mutex, oneshot, watch};
use tracing::{info, warn};

use crate::schwab::{AuthError, DEFAULT_LOGIN_TIMEOUT, SchwabAuth};

/// How a finished login ended. `Err` holds a message safe to show.
pub type LoginOutcome = Result<(), String>;

#[derive(Clone)]
pub struct PendingLogin {
    /// Opaque id, used as the MRTR request state and the elicitation id.
    pub id: String,
    pub url: String,
    started: Instant,
    done: watch::Receiver<Option<LoginOutcome>>,
}

impl PendingLogin {
    pub fn remaining(&self) -> Duration {
        DEFAULT_LOGIN_TIMEOUT.saturating_sub(self.started.elapsed())
    }

    pub fn outcome(&self) -> Option<LoginOutcome> {
        self.done.borrow().clone()
    }

    /// Wait up to `limit` for the login to finish. `None` if it is still
    /// running when the wait ends.
    pub async fn wait(&self, limit: Duration) -> Option<LoginOutcome> {
        let mut rx = self.done.clone();
        match tokio::time::timeout(limit, rx.wait_for(Option::is_some)).await {
            Ok(Ok(v)) => v.clone(),
            // Sender dropped without a result: the task panicked.
            Ok(Err(_)) => Some(Err("the login task stopped unexpectedly".to_string())),
            Err(_) => None,
        }
    }
}

/// Whether [`LoginManager::start`] began a new login or found one running.
pub struct Started {
    pub login: PendingLogin,
    pub is_new: bool,
}

#[derive(Default, Clone)]
pub struct LoginManager {
    current: Arc<Mutex<Option<PendingLogin>>>,
}

impl LoginManager {
    /// The login in progress (or the last one, finished).
    pub async fn current(&self) -> Option<PendingLogin> {
        self.current.lock().await.clone()
    }

    /// Start a browser login, or return the one already waiting for the
    /// browser. Opens the browser for a new login.
    pub async fn start(&self, auth: &SchwabAuth) -> Result<Started, String> {
        let mut current = self.current.lock().await;
        if let Some(p) = current.as_ref()
            && p.outcome().is_none()
            && !p.remaining().is_zero()
        {
            return Ok(Started {
                login: p.clone(),
                is_new: false,
            });
        }

        let (url_tx, url_rx) = oneshot::channel::<String>();
        let (done_tx, done_rx) = watch::channel::<Option<LoginOutcome>>(None);
        let auth = auth.clone();
        tokio::spawn(async move {
            let result = auth
                .login(|url| {
                    let _ = url_tx.send(url.to_string());
                })
                .await;
            let outcome = match result {
                Ok(_) => {
                    info!("MCP login finished");
                    Ok(())
                }
                Err(e) => {
                    warn!("MCP login failed: {e}");
                    Err(login_error_message(&e))
                }
            };
            let _ = done_tx.send(Some(outcome));
        });

        let url = match url_rx.await {
            Ok(url) => url,
            // The listener failed before producing a URL (e.g. port in use).
            Err(_) => {
                let mut rx = done_rx;
                let outcome = rx
                    .wait_for(Option::is_some)
                    .await
                    .ok()
                    .and_then(|v| v.clone());
                return Err(match outcome {
                    Some(Err(msg)) => msg,
                    _ => "the login could not start".to_string(),
                });
            }
        };

        open_browser(&url);
        let login = PendingLogin {
            id: new_id(),
            url,
            started: Instant::now(),
            done: done_rx,
        };
        *current = Some(login.clone());
        Ok(Started {
            login,
            is_new: true,
        })
    }
}

fn login_error_message(e: &AuthError) -> String {
    match e {
        AuthError::PortInUse { port } => format!(
            "port {port} is busy, so the login callback can't be received. Another \
             terminal-trader login (the TUI or another MCP session) is probably running; \
             finish or close it and try again"
        ),
        other => other.to_string(),
    }
}

fn new_id() -> String {
    let mut bytes = [0u8; 8];
    getrandom::getrandom(&mut bytes).expect("OS random number generator unavailable");
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Open `url` in the default browser with the opener's output discarded:
/// stdout carries the MCP protocol, and some openers print to it.
fn open_browser(url: &str) {
    for mut cmd in open::commands(url) {
        cmd.stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        match cmd.spawn() {
            Ok(mut child) => {
                // Reap it so it doesn't linger as a zombie.
                std::thread::spawn(move || child.wait());
                return;
            }
            Err(e) => warn!("couldn't run browser opener: {e}"),
        }
    }
}
