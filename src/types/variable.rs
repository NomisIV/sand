use crate::Parseable;
use anyhow::Result;

#[derive(Debug)]
pub struct Var {
    name: String,
}

impl Parseable for Var {
    fn parse(string: &str) -> Option<Result<Self>> {
        let name = string.to_string();
        Some(Ok(Var { name }))
    }
}
