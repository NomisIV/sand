use super::Block;
use super::Var;
use crate::Parseable;
use anyhow::Error;
use anyhow::Result;

#[derive(Debug)]
pub struct Fun {
    arguments: Vec<Var>,
    body: Block,
}

impl Parseable for Fun {
    fn parse(string: &str) -> Option<Result<Self>> {
        let (before, after) = string.split_once(')')?;
        let mut arguments = Vec::new();
        for argument_str in before.trim().strip_prefix('(')?.split(',') {
            if argument_str.is_empty() {
                continue;
            }
            match Var::parse(argument_str) {
                Some(Ok(argument)) => arguments.push(argument),
                Some(Err(err)) => return Some(Err(err)),
                None => {
                    return Some(Err(Error::msg(format!(
                        "ERROR: Cannot parse the following variable:\n{}",
                        argument_str
                    ))))
                }
            }
        }
        let body = match Block::parse(after.trim()) {
            Some(Ok(block)) => block,
            Some(Err(err)) => return Some(Err(err)),
            None => {
                return Some(Err(Error::msg(format!(
                    "ERROR: Cannot parse the following block:\n{}",
                    after.trim()
                ))))
            }
        };

        Some(Ok(Fun { arguments, body }))
    }
}
