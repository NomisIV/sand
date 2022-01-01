use crate::Parseable;
use anyhow::Result;

#[derive(Debug)]
pub struct Num {
    num: f64,
}

impl Parseable for Num {
    fn parse(string: &str) -> Option<Result<Self>> {
        let num = string.parse().ok()?;
        Some(Ok(Num { num }))
    }
}
