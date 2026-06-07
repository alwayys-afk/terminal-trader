use std::collections::BTreeMap;

fn is_indexable_word(w: &str) -> bool {
    w.len() >= 2 && w.chars().all(|c| c.is_alphabetic())
}

/// Split `desc` into words and add each indexable word to `word_index` under `sym`.
/// Also inserts the symbol → description mapping into `symbol_db`.
pub fn index_symbol(
    sym: &str,
    desc: &str,
    symbol_db: &mut BTreeMap<String, String>,
    word_index: &mut BTreeMap<String, Vec<String>>,
) {
    let sym = sym.to_uppercase();
    for word in desc.split(|c: char| !c.is_alphanumeric()) {
        let w = word.trim().to_lowercase();
        if is_indexable_word(&w) {
            word_index.entry(w).or_default().push(sym.clone());
        }
    }
    symbol_db.insert(sym, desc.to_string());
}

struct StockTsvRow {
    pub symbol: String,
    pub company_name: String,
    pub industry: Option<String>,
    pub market_cap: Option<String>,
}

struct EtfTsvRow {
    pub symbol: String,
    pub name: String,
    pub asset_class: Option<String>,
    pub assets: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TsvRowParseError {
    MissingSymbol,
    MissingName,
    EmptyRequiredField,
}

impl TryFrom<&str> for StockTsvRow {
    type Error = TsvRowParseError;

    fn try_from(line: &str) -> std::result::Result<Self, Self::Error> {
        let mut cols = line.split('\t');
        let symbol = cols
            .next()
            .ok_or(TsvRowParseError::MissingSymbol)?
            .trim()
            .to_string();
        let company_name = cols
            .next()
            .ok_or(TsvRowParseError::MissingName)?
            .trim()
            .to_string();
        let industry = cols.next().map(|s| s.trim().to_string());
        let market_cap = cols.next().map(|s| s.trim().to_string());
        if symbol.is_empty() || company_name.is_empty() {
            return Err(TsvRowParseError::EmptyRequiredField);
        }
        Ok(Self {
            symbol,
            company_name,
            industry,
            market_cap,
        })
    }
}

impl TryFrom<&str> for EtfTsvRow {
    type Error = TsvRowParseError;

    fn try_from(line: &str) -> std::result::Result<Self, Self::Error> {
        let mut cols = line.split('\t');
        let symbol = cols
            .next()
            .ok_or(TsvRowParseError::MissingSymbol)?
            .trim()
            .to_string();
        let name = cols
            .next()
            .ok_or(TsvRowParseError::MissingName)?
            .trim()
            .to_string();
        let asset_class = cols.next().map(|s| s.trim().to_string());
        let assets = cols.next().map(|s| s.trim().to_string());
        if symbol.is_empty() || name.is_empty() {
            return Err(TsvRowParseError::EmptyRequiredField);
        }
        Ok(Self {
            symbol,
            name,
            asset_class,
            assets,
        })
    }
}

trait Indexable: for<'a> TryFrom<&'a str, Error = TsvRowParseError> {
    fn symbol(&self) -> &str;
    fn label(&self) -> String;

    fn add_to_index(
        &self,
        symbol_db: &mut BTreeMap<String, String>,
        word_index: &mut BTreeMap<String, Vec<String>>,
    ) {
        index_symbol(self.symbol(), &self.label(), symbol_db, word_index);
    }
}

impl Indexable for StockTsvRow {
    fn symbol(&self) -> &str {
        &self.symbol
    }
    fn label(&self) -> String {
        let paren = match (&self.industry, &self.market_cap) {
            (Some(ind), Some(cap)) if !ind.is_empty() && !cap.is_empty() => {
                format!(" ({ind} · {cap})")
            }
            (Some(ind), _) if !ind.is_empty() => format!(" ({ind})"),
            (_, Some(cap)) if !cap.is_empty() => format!(" ({cap})"),
            _ => String::new(),
        };
        format!("{}{}", self.company_name, paren)
    }
}

impl Indexable for EtfTsvRow {
    fn symbol(&self) -> &str {
        &self.symbol
    }
    fn label(&self) -> String {
        let paren = match (&self.asset_class, &self.assets) {
            (Some(class), Some(assets)) if !class.is_empty() && !assets.is_empty() => {
                format!(" ({class} · {assets})")
            }
            (Some(class), _) if !class.is_empty() => format!(" ({class})"),
            (_, Some(assets)) if !assets.is_empty() => format!(" ({assets})"),
            _ => String::new(),
        };
        format!("{}{}", self.name, paren)
    }
}

pub fn load_symbols(paths: &[&str]) -> (BTreeMap<String, String>, BTreeMap<String, Vec<String>>) {
    let mut symbol_db: BTreeMap<String, String> = BTreeMap::new();
    let mut word_index: BTreeMap<String, Vec<String>> = BTreeMap::new();
    // can use any tsv or csv, just define the indexable trait for it
    for path in paths {
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                tracing::debug!("load_symbols: skipping {path}: {e}");
                continue;
            }
        };
        for line in content.lines() {
            let sym = line.split('\t').next().unwrap_or("").trim();
            // Skip headers and non-symbol rows.
            if sym.is_empty()
                || !sym
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_alphabetic())
                    .unwrap_or(false)
            {
                tracing::debug!("load_symbols: skipping line with invalid symbol: {line}");
                continue;
            }

            let is_etf = path.to_lowercase().contains("etf");
            if is_etf {
                match EtfTsvRow::try_from(line) {
                    Ok(row) => row.add_to_index(&mut symbol_db, &mut word_index),
                    Err(e) => {
                        tracing::debug!(
                            "load_symbols: failed to parse line in {path}: {line}: {e:?}"
                        );
                        continue;
                    }
                }
            } else {
                match StockTsvRow::try_from(line) {
                    Ok(row) => row.add_to_index(&mut symbol_db, &mut word_index),
                    Err(e) => {
                        tracing::debug!(
                            "load_symbols: failed to parse line in {path}: {line}: {e:?}"
                        );
                        continue;
                    }
                }
            }
        }
    }

    (symbol_db, word_index)
}
