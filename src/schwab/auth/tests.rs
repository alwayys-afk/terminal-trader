//! Auth tests. Everything runs against temp dirs and a local mock token
//! endpoint; nothing touches the real data dir or calls Schwab.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use chrono::Utc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use super::*;

/// A plain-HTTP stand-in for Schwab's token endpoint. Answers every request
/// with `status` and `body` after `delay`, and counts requests.
struct MockTokenServer {
    url: String,
    hits: Arc<AtomicUsize>,
}

impl MockTokenServer {
    async fn start(status: u16, body: &'static str, delay: Duration) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}/v1/oauth/token", listener.local_addr().unwrap());
        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);
        tokio::spawn(async move {
            loop {
                let Ok((mut stream, _)) = listener.accept().await else {
                    return;
                };
                let counter = Arc::clone(&counter);
                tokio::spawn(async move {
                    read_request(&mut stream).await;
                    counter.fetch_add(1, Ordering::SeqCst);
                    tokio::time::sleep(delay).await;
                    let response = format!(
                        "HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                        body.len()
                    );
                    stream.write_all(response.as_bytes()).await.ok();
                    stream.shutdown().await.ok();
                });
            }
        });
        Self { url, hits }
    }

    fn hits(&self) -> usize {
        self.hits.load(Ordering::SeqCst)
    }
}

/// Read a request head and its Content-Length body.
async fn read_request(stream: &mut tokio::net::TcpStream) {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 1024];
    loop {
        let n = stream.read(&mut chunk).await.unwrap_or(0);
        if n == 0 {
            return;
        }
        buf.extend_from_slice(&chunk[..n]);
        let text = String::from_utf8_lossy(&buf);
        if let Some(end) = text.find("\r\n\r\n") {
            let len = text[..end]
                .lines()
                .find_map(|l| {
                    let (k, v) = l.split_once(':')?;
                    k.eq_ignore_ascii_case("content-length")
                        .then(|| v.trim().parse::<usize>().ok())?
                })
                .unwrap_or(0);
            if buf.len() >= end + 4 + len {
                return;
            }
        }
    }
}

const REFRESHED: &str =
    r#"{"access_token":"fresh-access","expires_in":1800,"token_type":"Bearer","scope":"api"}"#;
const LOGGED_IN: &str = r#"{"access_token":"new-access","refresh_token":"new-refresh","expires_in":1800,"token_type":"Bearer"}"#;

fn auth_for(path: &std::path::Path, token_url: &str) -> SchwabAuth {
    SchwabAuth::new("test-key".into(), "test-secret".into(), path).with_token_url(token_url)
}

fn write_token(path: &std::path::Path, access_expired: bool, refresh_expired: bool) {
    let now = Utc::now();
    let token = TokenData {
        access_token: "old-access".into(),
        refresh_token: "old-refresh".into(),
        expires_at: if access_expired {
            now - chrono::Duration::minutes(5)
        } else {
            now + chrono::Duration::minutes(20)
        },
        refresh_expires_at: if refresh_expired {
            now - chrono::Duration::hours(1)
        } else {
            now + chrono::Duration::days(3)
        },
    };
    std::fs::write(path, serde_json::to_string(&token).unwrap()).unwrap();
}

fn free_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

fn insecure_client() -> reqwest::Client {
    reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
}

// ── get_valid_token ──────────────────────────────────────────────────────────

#[tokio::test]
async fn missing_token_requires_login() {
    let dir = tempfile::tempdir().unwrap();
    let auth = auth_for(&dir.path().join("t.json"), "http://127.0.0.1:9/unused");
    assert!(matches!(
        auth.get_valid_token().await,
        Err(AuthError::LoginRequired { .. })
    ));
}

#[tokio::test]
async fn expired_refresh_token_requires_login_without_calling_schwab() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("t.json");
    write_token(&path, true, true);
    let mock = MockTokenServer::start(200, REFRESHED, Duration::ZERO).await;
    let err = auth_for(&path, &mock.url)
        .get_valid_token()
        .await
        .unwrap_err();
    assert!(matches!(err, AuthError::LoginRequired { .. }), "{err}");
    assert_eq!(mock.hits(), 0);
}

#[tokio::test]
async fn valid_token_is_returned_without_refresh() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("t.json");
    write_token(&path, false, false);
    let mock = MockTokenServer::start(200, REFRESHED, Duration::ZERO).await;
    let token = auth_for(&path, &mock.url).get_valid_token().await.unwrap();
    assert_eq!(token.access_token, "old-access");
    assert_eq!(mock.hits(), 0);
}

#[tokio::test]
async fn expired_access_token_is_refreshed_and_saved() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("t.json");
    write_token(&path, true, false);
    let mock = MockTokenServer::start(200, REFRESHED, Duration::ZERO).await;
    let token = auth_for(&path, &mock.url).get_valid_token().await.unwrap();
    assert_eq!(token.access_token, "fresh-access");
    assert_eq!(token.refresh_token, "old-refresh", "refresh token kept");

    let saved: TokenData = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(saved.access_token, "fresh-access");
}

#[tokio::test]
async fn rejected_refresh_requires_login_and_keeps_the_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("t.json");
    write_token(&path, true, false);
    let before = std::fs::read_to_string(&path).unwrap();
    let mock = MockTokenServer::start(
        400,
        r#"{"error":"invalid_grant","error_description":"bad refresh"}"#,
        Duration::ZERO,
    )
    .await;
    let err = auth_for(&path, &mock.url)
        .get_valid_token()
        .await
        .unwrap_err();
    match err {
        AuthError::LoginRequired { reason } => {
            assert!(reason.contains("invalid_grant"), "{reason}")
        }
        other => panic!("expected LoginRequired, got {other}"),
    }
    assert_eq!(std::fs::read_to_string(&path).unwrap(), before);
}

#[tokio::test]
async fn server_error_is_transient_and_keeps_the_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("t.json");
    write_token(&path, true, false);
    let before = std::fs::read_to_string(&path).unwrap();
    let mock = MockTokenServer::start(503, "{}", Duration::ZERO).await;
    let err = auth_for(&path, &mock.url)
        .get_valid_token()
        .await
        .unwrap_err();
    assert!(matches!(err, AuthError::Transient(_)), "{err}");
    assert_eq!(std::fs::read_to_string(&path).unwrap(), before);
}

#[tokio::test]
async fn unreachable_endpoint_is_transient() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("t.json");
    write_token(&path, true, false);
    let url = format!("http://127.0.0.1:{}/token", free_port());
    let err = auth_for(&path, &url).get_valid_token().await.unwrap_err();
    assert!(matches!(err, AuthError::Transient(_)), "{err}");
}

#[tokio::test]
async fn two_instances_share_one_refresh() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("t.json");
    write_token(&path, true, false);
    // Slow response so both callers are in flight at once.
    let mock = MockTokenServer::start(200, REFRESHED, Duration::from_millis(300)).await;
    let a = auth_for(&path, &mock.url);
    let b = auth_for(&path, &mock.url);

    let (ta, tb) = tokio::join!(a.get_valid_token(), b.get_valid_token());
    assert_eq!(ta.unwrap().access_token, "fresh-access");
    assert_eq!(tb.unwrap().access_token, "fresh-access");
    assert_eq!(
        mock.hits(),
        1,
        "the second caller reused the first one's refresh"
    );
}

#[test]
fn login_required_survives_anyhow_context() {
    let err =
        anyhow::Error::from(AuthError::LoginRequired { reason: "x".into() }).context("auth failed");
    assert!(matches!(
        err.downcast_ref::<AuthError>(),
        Some(AuthError::LoginRequired { .. })
    ));
}

// ── Callback listener and login ──────────────────────────────────────────────

#[tokio::test]
async fn callback_server_ignores_noise_and_returns_the_code() {
    let dir = tempfile::tempdir().unwrap();
    let tls = cert::server_config(dir.path()).unwrap();
    let server = CallbackServer::bind(0, tls).await.unwrap();
    let port = server.local_addr().unwrap().port();

    let wait = tokio::spawn(async move {
        server
            .wait_for_code("good-state", Duration::from_secs(10))
            .await
    });

    let client = insecure_client();
    let base = format!("https://127.0.0.1:{port}");
    let favicon = client
        .get(format!("{base}/favicon.ico"))
        .send()
        .await
        .unwrap();
    assert_eq!(favicon.status(), 404);
    let stale = client
        .get(format!("{base}/?code=nope&state=bad-state"))
        .send()
        .await
        .unwrap();
    assert_eq!(stale.status(), 400);
    let ok = client
        .get(format!("{base}/?code=C0.abc%40def&state=good-state"))
        .send()
        .await
        .unwrap();
    assert_eq!(ok.status(), 200);
    assert!(ok.text().await.unwrap().contains("close this tab"));

    assert_eq!(wait.await.unwrap().unwrap(), "C0.abc@def");
}

#[tokio::test]
async fn callback_server_reports_denied_login() {
    let dir = tempfile::tempdir().unwrap();
    let tls = cert::server_config(dir.path()).unwrap();
    let server = CallbackServer::bind(0, tls).await.unwrap();
    let port = server.local_addr().unwrap().port();
    let wait =
        tokio::spawn(async move { server.wait_for_code("s", Duration::from_secs(10)).await });

    insecure_client()
        .get(format!(
            "https://127.0.0.1:{port}/?error=access_denied&state=s"
        ))
        .send()
        .await
        .unwrap();
    assert!(matches!(
        wait.await.unwrap(),
        Err(AuthError::LoginDenied(reason)) if reason == "access_denied"
    ));
}

#[tokio::test]
async fn callback_server_times_out() {
    let dir = tempfile::tempdir().unwrap();
    let tls = cert::server_config(dir.path()).unwrap();
    let server = CallbackServer::bind(0, tls).await.unwrap();
    let err = server
        .wait_for_code("s", Duration::from_millis(100))
        .await
        .unwrap_err();
    assert!(matches!(err, AuthError::LoginTimedOut(_)));
}

#[tokio::test]
async fn busy_port_is_reported() {
    let dir = tempfile::tempdir().unwrap();
    let tls = cert::server_config(dir.path()).unwrap();
    let held = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = held.local_addr().unwrap().port();
    let err = CallbackServer::bind(port, tls).await.err().unwrap();
    assert!(matches!(err, AuthError::PortInUse { port: p } if p == port));
}

#[tokio::test]
async fn login_end_to_end_saves_the_token() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("t.json");
    let mock = MockTokenServer::start(200, LOGGED_IN, Duration::ZERO).await;
    let auth = auth_for(&path, &mock.url);
    let opts = LoginOptions {
        port: free_port(),
        timeout: Duration::from_secs(10),
        cert_dir: dir.path().to_path_buf(),
    };

    let (url_tx, url_rx) = tokio::sync::oneshot::channel::<String>();
    let login = {
        let auth = auth.clone();
        let opts = opts.clone();
        tokio::spawn(async move {
            auth.login_with(&opts, |url| {
                url_tx.send(url.to_string()).ok();
            })
            .await
        })
    };

    let url = url_rx.await.unwrap();
    assert!(url.contains("redirect_uri=https://127.0.0.1:8182"), "{url}");
    let state = callback::query_param(url.split_once('?').unwrap().1, "state").unwrap();

    let resp = insecure_client()
        .get(format!(
            "https://127.0.0.1:{}/?code=the-code&state={state}",
            opts.port
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    let token = login.await.unwrap().unwrap();
    assert_eq!(token.access_token, "new-access");
    assert_eq!(token.refresh_token, "new-refresh");
    assert_eq!(mock.hits(), 1);
    // The saved file is what get_valid_token now returns.
    assert_eq!(
        auth.get_valid_token().await.unwrap().access_token,
        "new-access"
    );
}

#[tokio::test]
async fn pasted_redirect_url_checks_state() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("t.json");
    let mock = MockTokenServer::start(200, LOGGED_IN, Duration::ZERO).await;
    let auth = auth_for(&path, &mock.url);

    let err = auth
        .login_from_redirect_url("https://127.0.0.1:8182/?code=c&state=other", "mine")
        .await
        .unwrap_err();
    assert!(matches!(err, AuthError::LoginDenied(_)));
    assert_eq!(mock.hits(), 0);

    let token = auth
        .login_from_redirect_url("https://127.0.0.1:8182/?code=c&state=mine", "mine")
        .await
        .unwrap();
    assert_eq!(token.access_token, "new-access");
}
