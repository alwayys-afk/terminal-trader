//! Where the app keeps the user's private runtime files.
//!
//! Credentials (`.env`, `schwab_tokens.json`), tax lots, app prefs and
//! `trader.log` all hold real account data, so they live outside the repo:
//! `$TERMINAL_TRADER_DATA_DIR` if set, otherwise
//! `~/Library/Application Support/terminal-trader`. Nothing personal is ever
//! written to the working directory, so nothing personal can be committed.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// Environment variable that overrides the data directory.
pub const DATA_DIR_ENV: &str = "TERMINAL_TRADER_DATA_DIR";

/// Resolve the data directory from an override and a home directory.
/// Pure so it can be tested without touching the process environment.
fn resolve(override_dir: Option<OsString>, home: Option<OsString>) -> PathBuf {
    if let Some(dir) = override_dir.filter(|d| !d.is_empty()) {
        return PathBuf::from(dir);
    }
    // Never fall back to the working directory: that's the repo, and the
    // whole point is to keep these files out of it.
    let home = home
        .filter(|h| !h.is_empty())
        .map(PathBuf::from)
        .expect("HOME is not set; set TERMINAL_TRADER_DATA_DIR to choose a data directory");
    home.join("Library")
        .join("Application Support")
        .join("terminal-trader")
}

/// The data directory (not created; see [`ensure_data_dir`]).
pub fn data_dir() -> PathBuf {
    resolve(std::env::var_os(DATA_DIR_ENV), std::env::var_os("HOME"))
}

/// A file inside the data directory.
pub fn data_path(name: &str) -> PathBuf {
    data_dir().join(name)
}

/// Create the data directory if needed and restrict it to the current user
/// (0700). Call once at startup, before anything reads or writes in it.
pub fn ensure_data_dir() -> std::io::Result<PathBuf> {
    let dir = data_dir();
    std::fs::create_dir_all(&dir)?;
    restrict(&dir)?;
    Ok(dir)
}

#[cfg(unix)]
fn restrict(dir: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700))
}

#[cfg(not(unix))]
fn restrict(_dir: &Path) -> std::io::Result<()> {
    Ok(())
}

/// Load `<data dir>/.env` into the environment (existing variables win).
pub fn load_env() {
    dotenvy::from_path(data_path(".env")).ok();
}

/// The OAuth token file: `$SCHWAB_TOKEN_PATH` if set, else
/// `<data dir>/schwab_tokens.json`.
pub fn token_path() -> String {
    std::env::var("SCHWAB_TOKEN_PATH").unwrap_or_else(|_| {
        data_path("schwab_tokens.json")
            .to_string_lossy()
            .into_owned()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn override_wins() {
        let p = resolve(Some("/tmp/tt-data".into()), Some("/Users/someone".into()));
        assert_eq!(p, PathBuf::from("/tmp/tt-data"));
    }

    #[test]
    fn empty_override_falls_back_to_home() {
        let p = resolve(Some("".into()), Some("/Users/someone".into()));
        assert_eq!(
            p,
            PathBuf::from("/Users/someone/Library/Application Support/terminal-trader")
        );
    }

    #[test]
    fn default_is_under_application_support() {
        let p = resolve(None, Some("/Users/someone".into()));
        assert_eq!(
            p,
            PathBuf::from("/Users/someone/Library/Application Support/terminal-trader")
        );
    }

    #[test]
    fn ensure_creates_dir_with_0700() {
        // Exercise the create + chmod path on a temp dir without touching
        // the process environment.
        let dir = std::env::temp_dir().join(format!("tt-data-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        restrict(&dir).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&dir).unwrap().permissions().mode() & 0o777;
            assert_eq!(mode, 0o700);
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
