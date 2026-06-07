//! Small shared helpers used across the TUI and domain layers.

#[derive(Debug, Clone)]
pub enum PrefixParseError {
    Unknown(String),
    Ambiguous(Vec<String>),
}

/// Resolve `input` against `options` by unique-prefix match: an input that is a
/// prefix of exactly one option name selects it. Zero matches is `Unknown`;
/// more than one is `Ambiguous` (carrying the candidate names). An empty input
/// matches every option, so it surfaces as `Ambiguous` rather than silently
/// picking the first.
pub fn prefix_match<T: Clone>(input: &str, options: &[(&str, T)]) -> Result<T, PrefixParseError> {
    let matches: Vec<_> = options
        .iter()
        .filter(|(name, _)| name.starts_with(input))
        .collect();
    match matches.len() {
        0 => Err(PrefixParseError::Unknown(input.to_string())),
        1 => Ok(matches[0].1.clone()),
        _ => Err(PrefixParseError::Ambiguous(
            matches.iter().map(|(name, _)| name.to_string()).collect(),
        )),
    }
}
