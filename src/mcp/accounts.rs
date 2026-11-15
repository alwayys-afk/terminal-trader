//! Account nicknames for the MCP server.
//!
//! Tool output must never contain an account number or the encrypted account
//! hash, so every account is known to the model only by a label: the
//! nickname set in Schwab, or a generated "Account N". The mapping back to the
//! hash stays in here, and [`AccountDirectory::redact`] scrubs any number or
//! hash that slips into text (for example inside an HTTP error that quotes a
//! request URL).

use crate::schwab::AccountSummary;

/// One account as the server knows it.
#[derive(Debug, Clone)]
pub struct AccountEntry {
    /// What the model sees and passes back.
    pub label: String,
    /// The encrypted hash used for API calls. Never shown.
    pub hash: String,
    /// The account number. Never shown; kept only so it can be redacted.
    number: String,
}

#[derive(Debug, Clone, Default)]
pub struct AccountDirectory {
    entries: Vec<AccountEntry>,
}

impl AccountDirectory {
    /// Build labels for `accounts`. Schwab nicknames are used when set (with
    /// any run of the account number's digits removed); accounts without one
    /// get "Account 1", "Account 2", ... in hash order, so the labels are the
    /// same on every run. Duplicate labels get " (2)", " (3)", ...
    pub fn new(accounts: &[AccountSummary]) -> AccountDirectory {
        let mut sorted: Vec<&AccountSummary> = accounts.iter().collect();
        sorted.sort_by(|a, b| a.hash.cmp(&b.hash));

        let mut generated = 0;
        let mut entries: Vec<AccountEntry> = Vec::with_capacity(sorted.len());
        for acct in sorted {
            let nickname = acct
                .nickname
                .as_deref()
                .map(|n| sanitize_nickname(n, &acct.number))
                .filter(|n| !n.is_empty());
            let base = nickname.unwrap_or_else(|| {
                generated += 1;
                format!("Account {generated}")
            });
            let mut label = base.clone();
            let mut n = 2;
            while entries.iter().any(|e| e.label.eq_ignore_ascii_case(&label)) {
                label = format!("{base} ({n})");
                n += 1;
            }
            entries.push(AccountEntry {
                label,
                hash: acct.hash.clone(),
                number: acct.number.clone(),
            });
        }
        AccountDirectory { entries }
    }

    pub fn entries(&self) -> &[AccountEntry] {
        &self.entries
    }

    /// The accounts a tool call refers to: the one named (case-insensitive,
    /// exact label first, then a unique prefix), or all of them when `name`
    /// is `None`.
    pub fn resolve(&self, name: Option<&str>) -> Result<Vec<&AccountEntry>, String> {
        let Some(name) = name.map(str::trim).filter(|n| !n.is_empty()) else {
            return Ok(self.entries.iter().collect());
        };
        if let Some(e) = self
            .entries
            .iter()
            .find(|e| e.label.eq_ignore_ascii_case(name))
        {
            return Ok(vec![e]);
        }
        let lower = name.to_lowercase();
        let matches: Vec<&AccountEntry> = self
            .entries
            .iter()
            .filter(|e| e.label.to_lowercase().starts_with(&lower))
            .collect();
        match matches.len() {
            1 => Ok(matches),
            _ => Err(format!(
                "No account named {name:?}. Accounts: {}.",
                self.label_list()
            )),
        }
    }

    pub fn label_list(&self) -> String {
        self.entries
            .iter()
            .map(|e| e.label.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// Replace every account hash and number in `text` with the account's
    /// label. Applied to all tool output as a last line of defense.
    pub fn redact(&self, text: &str) -> String {
        let mut out = text.to_string();
        for e in &self.entries {
            for secret in [&e.hash, &e.number] {
                if secret.len() >= 4 && out.contains(secret.as_str()) {
                    out = out.replace(secret.as_str(), &format!("<{}>", e.label));
                }
            }
        }
        out
    }
}

/// Remove any run of three or more digits that also appears in the account
/// number (Schwab's default nicknames can look like "Individual ...123"),
/// then tidy leftover dots and spaces.
fn sanitize_nickname(nickname: &str, number: &str) -> String {
    let mut out = String::with_capacity(nickname.len());
    let chars: Vec<char> = nickname.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_ascii_digit() {
            let start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            let run: String = chars[start..i].iter().collect();
            if run.len() < 3 || !number.contains(&run) {
                out.push_str(&run);
            }
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    let collapsed = out.split_whitespace().collect::<Vec<_>>().join(" ");
    collapsed
        .trim_matches(|c: char| c == '.' || c == '-' || c == '_' || c == '#' || c.is_whitespace())
        .trim_end_matches("...")
        .trim()
        .to_string()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    // Fake account numbers/hashes, shaped like real ones.
    pub const NUM_A: &str = "TESTNUM1234";
    pub const HASH_A: &str = "AAAAHASHAAAAHASHAAAAHASHAAAAHASH";
    const NUM_B: &str = "TESTNUM5678";
    const HASH_B: &str = "BBBBHASHBBBBHASHBBBBHASHBBBBHASH";
    const NUM_C: &str = "TESTNUM9012";
    const HASH_C: &str = "CCCCHASHCCCCHASHCCCCHASHCCCCHASH";

    fn summary(number: &str, hash: &str, nickname: Option<&str>) -> AccountSummary {
        AccountSummary {
            number: number.to_string(),
            hash: hash.to_string(),
            nickname: nickname.map(str::to_string),
        }
    }

    pub fn directory() -> AccountDirectory {
        AccountDirectory::new(&[
            summary(NUM_C, HASH_C, None),
            summary(NUM_A, HASH_A, Some("Roth")),
            summary(NUM_B, HASH_B, None),
        ])
    }

    #[test]
    fn nicknames_win_and_the_rest_get_stable_generated_labels() {
        let dir = directory();
        let labels: Vec<&str> = dir.entries().iter().map(|e| e.label.as_str()).collect();
        // Sorted by hash: A (Roth), B (Account 1), C (Account 2).
        assert_eq!(labels, ["Roth", "Account 1", "Account 2"]);

        // Input order doesn't change the labels.
        let reordered = AccountDirectory::new(&[
            summary(NUM_B, HASH_B, None),
            summary(NUM_C, HASH_C, None),
            summary(NUM_A, HASH_A, Some("Roth")),
        ]);
        let again: Vec<&str> = reordered
            .entries()
            .iter()
            .map(|e| e.label.as_str())
            .collect();
        assert_eq!(labels, again);
    }

    #[test]
    fn resolve_is_case_insensitive_and_accepts_unique_prefixes() {
        let dir = directory();
        assert_eq!(dir.resolve(Some("roth")).unwrap()[0].hash, HASH_A);
        assert_eq!(dir.resolve(Some("ACCOUNT 2")).unwrap()[0].hash, HASH_C);
        assert_eq!(dir.resolve(Some("ro")).unwrap()[0].hash, HASH_A);
        assert_eq!(dir.resolve(None).unwrap().len(), 3);
        assert_eq!(dir.resolve(Some("  ")).unwrap().len(), 3);
        // Ambiguous prefix and unknown names list the labels, nothing else.
        let err = dir.resolve(Some("acc")).unwrap_err();
        assert!(err.contains("Roth, Account 1, Account 2"));
        assert!(!err.contains(HASH_A) && !err.contains(NUM_A));
        assert!(dir.resolve(Some("Joint")).is_err());
    }

    #[test]
    fn duplicate_nicknames_are_disambiguated() {
        let dir = AccountDirectory::new(&[
            summary(NUM_A, HASH_A, Some("Brokerage")),
            summary(NUM_B, HASH_B, Some("brokerage")),
        ]);
        let labels: Vec<&str> = dir.entries().iter().map(|e| e.label.as_str()).collect();
        assert_eq!(labels, ["Brokerage", "brokerage (2)"]);
    }

    #[test]
    fn nickname_digits_from_the_account_number_are_stripped() {
        assert_eq!(sanitize_nickname("Individual ...234", NUM_A), "Individual");
        assert_eq!(sanitize_nickname("Joint 1234", NUM_A), "Joint");
        // Digits that aren't from the account number stay.
        assert_eq!(sanitize_nickname("Roth 2024", NUM_A), "Roth 2024");
        assert_eq!(sanitize_nickname("Fund 42", NUM_A), "Fund 42");
        // A nickname that is only the number becomes empty -> generated label.
        let dir = AccountDirectory::new(&[summary(NUM_A, HASH_A, Some("...1234"))]);
        assert_eq!(dir.entries()[0].label, "Account 1");
    }

    #[test]
    fn redact_replaces_numbers_and_hashes_with_labels() {
        let dir = directory();
        let text = format!(
            "error sending request for url (https://api.example/accounts/{HASH_A}/orders) acct {NUM_B}"
        );
        let out = dir.redact(&text);
        assert!(!out.contains(HASH_A) && !out.contains(NUM_B));
        assert!(out.contains("<Roth>") && out.contains("<Account 1>"));
    }
}
