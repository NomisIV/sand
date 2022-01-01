use super::Value;
use crate::Parseable;
use anyhow::Result;

#[derive(Debug)]
pub struct Assignment {
    name: String, // TODO: Use variable instead
    value: Value,
}

impl Parseable for Assignment {
    fn parse(string: &str) -> Option<Result<Self>> {
        let (before, after) = string.split_once('=')?;
        let name = before.strip_prefix("let")?.trim().to_string();
        let value = Value::parse(after.trim())?.ok()?;
        Some(Ok(Assignment { name, value }))
    }
}
