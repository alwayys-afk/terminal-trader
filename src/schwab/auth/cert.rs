//! The self-signed certificate for the local OAuth callback listener.
//!
//! Schwab only redirects to an `https://` callback, so the listener on
//! 127.0.0.1 needs TLS. The cert is generated once and kept in the data
//! directory, so a browser's "proceed anyway" exception keeps working on later
//! logins. It is regenerated if either file is missing or unusable.

use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result};
use chrono::Datelike;
use tokio_rustls::rustls::{
    self, ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
};
use tracing::{info, warn};

use super::store::{create_private, restrict};

pub(super) const CERT_FILE: &str = "callback_cert.pem";
pub(super) const KEY_FILE: &str = "callback_key.pem";

/// Build the listener's TLS config from the cert in `dir`, creating or
/// replacing it if needed.
pub(super) fn server_config(dir: &Path) -> Result<Arc<ServerConfig>> {
    match load_config(dir) {
        Ok(config) => Ok(config),
        Err(e) => {
            if dir.join(CERT_FILE).exists() || dir.join(KEY_FILE).exists() {
                warn!("callback certificate unusable ({e:#}); generating a new one");
            } else {
                info!("generating the local callback certificate");
            }
            generate(dir)?;
            load_config(dir).context("freshly generated callback certificate didn't load")
        }
    }
}

fn load_config(dir: &Path) -> Result<Arc<ServerConfig>> {
    let certs = CertificateDer::pem_file_iter(dir.join(CERT_FILE))
        .context("reading callback certificate")?
        .collect::<Result<Vec<_>, _>>()
        .context("parsing callback certificate")?;
    anyhow::ensure!(!certs.is_empty(), "callback certificate file is empty");
    let key = PrivateKeyDer::from_pem_file(dir.join(KEY_FILE)).context("reading callback key")?;

    let provider = Arc::new(rustls::crypto::aws_lc_rs::default_provider());
    let config = ServerConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()?
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .context("callback certificate and key don't match")?;
    Ok(Arc::new(config))
}

fn generate(dir: &Path) -> Result<()> {
    let mut params =
        rcgen::CertificateParams::new(vec!["127.0.0.1".to_string(), "localhost".to_string()])?;
    params.distinguished_name.push(
        rcgen::DnType::CommonName,
        "terminal-trader local OAuth callback",
    );
    let year = chrono::Utc::now().year();
    params.not_before = rcgen::date_time_ymd(year - 1, 1, 1);
    params.not_after = rcgen::date_time_ymd(year + 10, 1, 1);

    let key_pair = rcgen::KeyPair::generate()?;
    let cert = params.self_signed(&key_pair)?;

    write_replacing(&dir.join(KEY_FILE), key_pair.serialize_pem().as_bytes())?;
    write_replacing(&dir.join(CERT_FILE), cert.pem().as_bytes())?;
    Ok(())
}

/// Replace `path` with `data`, readable only by the current user.
fn write_replacing(path: &Path, data: &[u8]) -> Result<()> {
    use std::io::Write;
    match std::fs::remove_file(path) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e).with_context(|| format!("removing old {}", path.display())),
    }
    let mut file = create_private(path).with_context(|| format!("creating {}", path.display()))?;
    file.write_all(data)?;
    restrict(path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_once_then_reused() {
        let dir = tempfile::tempdir().unwrap();
        server_config(dir.path()).unwrap();
        let first = std::fs::read(dir.path().join(CERT_FILE)).unwrap();

        server_config(dir.path()).unwrap();
        let second = std::fs::read(dir.path().join(CERT_FILE)).unwrap();
        assert_eq!(first, second, "existing cert is reused");

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(dir.path().join(KEY_FILE))
                .unwrap()
                .permissions()
                .mode()
                & 0o777;
            assert_eq!(mode, 0o600);
        }
    }

    #[test]
    fn regenerated_when_corrupt() {
        let dir = tempfile::tempdir().unwrap();
        server_config(dir.path()).unwrap();
        let original = std::fs::read(dir.path().join(CERT_FILE)).unwrap();

        std::fs::write(dir.path().join(CERT_FILE), "garbage").unwrap();
        server_config(dir.path()).unwrap();
        let replaced = std::fs::read(dir.path().join(CERT_FILE)).unwrap();
        assert_ne!(original, replaced);
        assert!(
            String::from_utf8(replaced)
                .unwrap()
                .contains("BEGIN CERTIFICATE")
        );
    }

    #[test]
    fn regenerated_when_key_missing() {
        let dir = tempfile::tempdir().unwrap();
        server_config(dir.path()).unwrap();
        std::fs::remove_file(dir.path().join(KEY_FILE)).unwrap();
        server_config(dir.path()).unwrap();
        assert!(dir.path().join(KEY_FILE).exists());
    }
}
