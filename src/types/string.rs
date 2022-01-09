use std::fmt::Display;

use crate::Parseable;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Str {
    str: String,
}

impl Parseable for Str {
    fn parse(string: &str) -> Option<Result<Self>> {
        let str = string
            .trim()
            .strip_prefix('"')?
            .strip_suffix('"')?
            .to_string();
        Some(Ok(Str { str }))
    }
}

impl Display for Str {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str)
    }
}
