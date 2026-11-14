//! The local HTTPS listener that receives Schwab's OAuth redirect.
//!
//! Schwab sends the browser to `https://127.0.0.1:8182/?code=...&state=...`.
//! This listener answers on loopback only, ignores unrelated requests (the
//! browser's favicon fetch, speculative connections), rejects a mismatched
//! `state` and keeps waiting, and returns the decoded `code`. It never touches
//! stdin or stdout, so the MCP server can reuse it.

use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::Duration;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_rustls::{TlsAcceptor, rustls::ServerConfig};
use tracing::debug;

use super::AuthError;

/// Per-connection limit on the TLS handshake plus reading the request head.
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(15);
/// Largest request head we'll read.
const MAX_HEAD_BYTES: usize = 16 * 1024;

/// What one request on the callback port turned out to be.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Outcome {
    /// Not the OAuth redirect (favicon, wrong path, no code/error params).
    Ignored,
    /// The redirect, but for a different login attempt.
    StateMismatch,
    /// Schwab redirected with `error=` (e.g. the user declined).
    Denied(String),
    /// The authorization code, URL-decoded.
    Code(String),
}

pub struct CallbackServer {
    listener: TcpListener,
    acceptor: TlsAcceptor,
}

impl CallbackServer {
    /// Listen on `127.0.0.1:<port>` (port 0 picks a free one, for tests).
    pub async fn bind(port: u16, tls: Arc<ServerConfig>) -> Result<Self, AuthError> {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, port))
            .await
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::AddrInUse {
                    AuthError::PortInUse { port }
                } else {
                    AuthError::Other(anyhow::Error::new(e).context("binding callback listener"))
                }
            })?;
        Ok(Self {
            listener,
            acceptor: TlsAcceptor::from(tls),
        })
    }

    pub fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.listener.local_addr()
    }

    /// Serve requests until the redirect for `expected_state` arrives (or
    /// `timeout` passes). Returns the authorization code.
    pub async fn wait_for_code(
        &self,
        expected_state: &str,
        timeout: Duration,
    ) -> Result<String, AuthError> {
        let expected: Arc<str> = Arc::from(expected_state);
        let (tx, mut rx) = mpsc::channel::<Outcome>(8);
        let serve = async {
            loop {
                tokio::select! {
                    accepted = self.listener.accept() => {
                        let Ok((stream, peer)) = accepted else { continue };
                        // Each connection gets its own task: browsers open
                        // speculative connections that may never send a
                        // request, and those mustn't block the real one.
                        let acceptor = self.acceptor.clone();
                        let expected = Arc::clone(&expected);
                        let tx = tx.clone();
                        tokio::spawn(async move {
                            let handled = tokio::time::timeout(
                                CONNECTION_TIMEOUT,
                                handle_connection(acceptor, stream, &expected),
                            )
                            .await;
                            match handled {
                                Ok(Ok(outcome)) => {
                                    let _ = tx.send(outcome).await;
                                }
                                // TLS handshake refused (browser cert warning
                                // before the user clicks through), idle
                                // connection, or a malformed request.
                                Ok(Err(e)) => debug!(%peer, "callback connection: {e}"),
                                Err(_) => debug!(%peer, "callback connection timed out"),
                            }
                        });
                    }
                    Some(outcome) = rx.recv() => match outcome {
                        Outcome::Code(code) => return Ok(code),
                        Outcome::Denied(reason) => return Err(AuthError::LoginDenied(reason)),
                        Outcome::Ignored | Outcome::StateMismatch => {}
                    },
                }
            }
        };
        tokio::time::timeout(timeout, serve)
            .await
            .map_err(|_| AuthError::LoginTimedOut(timeout))?
    }
}

async fn handle_connection(
    acceptor: TlsAcceptor,
    stream: TcpStream,
    expected_state: &str,
) -> std::io::Result<Outcome> {
    let mut tls = acceptor.accept(stream).await?;
    let head = read_head(&mut tls).await?;
    let outcome = match request_target(&head) {
        Some(target) => classify(target, expected_state),
        None => Outcome::Ignored,
    };
    let (status, body) = match &outcome {
        Outcome::Code(_) => (
            "200 OK",
            page(
                "Logged in to Schwab",
                "terminal-trader has your login. You can close this tab.",
            ),
        ),
        Outcome::Denied(reason) => (
            "200 OK",
            page(
                "Schwab login failed",
                &format!(
                    "Schwab said: {}. Go back to terminal-trader and try again.",
                    html_escape(reason)
                ),
            ),
        ),
        Outcome::StateMismatch => (
            "400 Bad Request",
            page(
                "Stale login link",
                "This redirect is from a different login attempt. Use the link from the latest login.",
            ),
        ),
        Outcome::Ignored => ("404 Not Found", page("Not found", "")),
    };
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    tls.write_all(response.as_bytes()).await?;
    tls.shutdown().await.ok();
    Ok(outcome)
}

/// Read up to the blank line that ends the request head.
async fn read_head<S: AsyncReadExt + Unpin>(stream: &mut S) -> std::io::Result<String> {
    let mut buf = Vec::with_capacity(1024);
    let mut chunk = [0u8; 1024];
    loop {
        let n = stream.read(&mut chunk).await?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&chunk[..n]);
        if buf.windows(4).any(|w| w == b"\r\n\r\n") || buf.len() >= MAX_HEAD_BYTES {
            break;
        }
    }
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

/// The request target of a `GET` request line, e.g. `/?code=...`.
pub(super) fn request_target(head: &str) -> Option<&str> {
    let mut parts = head.lines().next()?.split_whitespace();
    if parts.next()? != "GET" {
        return None;
    }
    parts.next()
}

/// Decide what a request target means for the login waiting on
/// `expected_state`.
pub(super) fn classify(target: &str, expected_state: &str) -> Outcome {
    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    if path != "/" {
        return Outcome::Ignored;
    }
    let code = query_param(query, "code");
    let error = query_param(query, "error");
    if code.is_none() && error.is_none() {
        return Outcome::Ignored;
    }
    if query_param(query, "state").as_deref() != Some(expected_state) {
        return Outcome::StateMismatch;
    }
    if let Some(error) = error {
        let reason = match query_param(query, "error_description") {
            Some(desc) if !desc.is_empty() => format!("{error} ({desc})"),
            _ => error,
        };
        return Outcome::Denied(reason);
    }
    match code {
        Some(code) if !code.is_empty() => Outcome::Code(code),
        _ => Outcome::Ignored,
    }
}

/// Look up and URL-decode one query parameter (`+` means space).
pub(super) fn query_param(query: &str, key: &str) -> Option<String> {
    query.split('&').find_map(|pair| {
        let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
        (k == key).then(|| {
            urlencoding::decode(&v.replace('+', " "))
                .map(|s| s.into_owned())
                .unwrap_or_else(|_| v.to_string())
        })
    })
}

fn page(title: &str, message: &str) -> String {
    format!(
        "<!doctype html><html><head><meta charset=\"utf-8\"><title>{title}</title></head>\
         <body style=\"font-family: system-ui, sans-serif; margin: 3em;\">\
         <h2>{title}</h2><p>{message}</p></body></html>"
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_target_parses_get_only() {
        assert_eq!(
            request_target("GET /?code=a&state=b HTTP/1.1\r\nHost: x\r\n\r\n"),
            Some("/?code=a&state=b")
        );
        assert_eq!(request_target("POST / HTTP/1.1\r\n\r\n"), None);
        assert_eq!(request_target(""), None);
    }

    #[test]
    fn query_params_are_decoded() {
        assert_eq!(
            query_param("code=C0.abc%40&state=s", "code").as_deref(),
            Some("C0.abc@")
        );
        assert_eq!(
            query_param("error_description=access+denied", "error_description").as_deref(),
            Some("access denied")
        );
        assert_eq!(query_param("code=x", "state"), None);
    }

    #[test]
    fn classify_redirects() {
        assert_eq!(
            classify("/?code=abc%40def&state=S1", "S1"),
            Outcome::Code("abc@def".to_string())
        );
        assert_eq!(
            classify("/?code=abc&state=OTHER", "S1"),
            Outcome::StateMismatch
        );
        assert_eq!(classify("/?code=abc", "S1"), Outcome::StateMismatch);
        assert_eq!(
            classify(
                "/?error=access_denied&error_description=user+said+no&state=S1",
                "S1"
            ),
            Outcome::Denied("access_denied (user said no)".to_string())
        );
        assert_eq!(
            classify("/?error=access_denied&state=nope", "S1"),
            Outcome::StateMismatch
        );
        assert_eq!(classify("/favicon.ico", "S1"), Outcome::Ignored);
        assert_eq!(classify("/", "S1"), Outcome::Ignored);
        assert_eq!(classify("/other?code=x&state=S1", "S1"), Outcome::Ignored);
    }
}
