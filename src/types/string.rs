use crate::Parseable;
use anyhow::Result;

#[derive(Debug)]
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
