//! The OAuth token file: atomic saves and a cross-process lock.
//!
//! The TUI and (later) the MCP server share one token file. A refresh takes
//! the `<token file>.lock` sidecar lock, re-reads the file (another process
//! may have refreshed already), and only then calls Schwab. Saves write a
//! temp file next to the real one and rename it over, so a reader never sees
//! a half-written file.

use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use super::TokenData;

/// Why a saved token couldn't be loaded.
#[derive(Debug)]
pub(super) enum LoadError {
    Missing,
    Unreadable(String),
    Io(io::Error),
}

#[derive(Debug, Clone)]
pub(super) struct TokenStore {
    path: PathBuf,
}

/// Held while refreshing or saving. Dropping it closes the lock file, which
/// releases the lock.
pub(super) struct TokenLock {
    _file: File,
}

impl TokenStore {
    pub(super) fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub(super) fn path(&self) -> &Path {
        &self.path
    }

    pub(super) fn load(&self) -> Result<TokenData, LoadError> {
        let data = match std::fs::read_to_string(&self.path) {
            Ok(d) => d,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Err(LoadError::Missing),
            Err(e) => return Err(LoadError::Io(e)),
        };
        serde_json::from_str(&data).map_err(|e| LoadError::Unreadable(e.to_string()))
    }

    /// Write the token atomically (temp file in the same directory, 0600,
    /// fsync, rename). Callers hold the [`TokenLock`].
    pub(super) fn save(&self, token: &TokenData) -> io::Result<()> {
        let data = serde_json::to_vec_pretty(token).map_err(io::Error::other)?;
        let tmp = self.temp_path();
        let result = (|| {
            let mut file = create_private(&tmp)?;
            file.write_all(&data)?;
            file.sync_all()?;
            std::fs::rename(&tmp, &self.path)
        })();
        if result.is_err() {
            let _ = std::fs::remove_file(&tmp);
        }
        result
    }

    /// Take the cross-process lock. Blocks (off the async runtime) until any
    /// other holder releases it.
    pub(super) async fn lock(&self) -> io::Result<TokenLock> {
        let lock_path = self.lock_path();
        tokio::task::spawn_blocking(move || {
            let file = OpenOptions::new()
                .create(true)
                .truncate(false)
                .write(true)
                .open(&lock_path)?;
            restrict(&lock_path)?;
            file.lock()?;
            Ok(TokenLock { _file: file })
        })
        .await
        .map_err(io::Error::other)?
    }

    fn lock_path(&self) -> PathBuf {
        with_suffix(&self.path, ".lock")
    }

    fn temp_path(&self) -> PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        with_suffix(&self.path, &format!(".tmp-{}-{n}", std::process::id()))
    }
}

fn with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut s: OsString = path.as_os_str().to_owned();
    s.push(suffix);
    PathBuf::from(s)
}

/// Create a new file readable only by the current user.
pub(super) fn create_private(path: &Path) -> io::Result<File> {
    let mut opts = OpenOptions::new();
    opts.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    opts.open(path)
}

#[cfg(unix)]
pub(super) fn restrict(path: &Path) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))
}

#[cfg(not(unix))]
pub(super) fn restrict(_path: &Path) -> io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn token(access: &str) -> TokenData {
        TokenData {
            access_token: access.to_string(),
            refresh_token: "refresh".to_string(),
            expires_at: Utc::now(),
            refresh_expires_at: Utc::now(),
        }
    }

    #[test]
    fn save_is_atomic_and_private() {
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::new(dir.path().join("tokens.json"));
        store.save(&token("one")).unwrap();
        store.save(&token("two")).unwrap();

        assert_eq!(store.load().unwrap().access_token, "two");
        let names: Vec<String> = std::fs::read_dir(dir.path())
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(names, vec!["tokens.json".to_string()], "no temp files left");

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(store.path())
                .unwrap()
                .permissions()
                .mode()
                & 0o777;
            assert_eq!(mode, 0o600);
        }
    }

    #[test]
    fn load_distinguishes_missing_and_unreadable() {
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::new(dir.path().join("tokens.json"));
        assert!(matches!(store.load(), Err(LoadError::Missing)));
        std::fs::write(store.path(), "not json").unwrap();
        assert!(matches!(store.load(), Err(LoadError::Unreadable(_))));
    }

    #[tokio::test]
    async fn lock_excludes_a_second_holder() {
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::new(dir.path().join("tokens.json"));
        let held = store.lock().await.unwrap();

        // A second open of the lock file (as another process would do) can't
        // take it while the first is held.
        let other = OpenOptions::new()
            .write(true)
            .open(store.lock_path())
            .unwrap();
        assert!(other.try_lock().is_err());
        drop(held);
        assert!(other.try_lock().is_ok());
    }
}
