use config::*;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::path::Path;

pub struct Quote {
    pub symbol: String,
}

impl Display for Quote {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Symbol: {}", self.symbol)
    }
}
pub fn read_quotes() -> Vec<Quote> {
    if !Path::new("stocks.toml").exists() {
        return vec![];
    }

    let mut settings = Config::default();
    settings
        .merge(config::File::with_name("stocks.toml"))
        .unwrap();

    let quotes = settings.try_into::<HashMap<String, Value>>().unwrap();

    return quotes
        .iter()
        .map(|(k, _v)| Quote {
            symbol: k.to_string(),
        })
        .collect();
}
