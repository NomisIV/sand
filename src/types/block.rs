use crate::Parseable;
use anyhow::{Error, Result};

use super::statement::Statement;

#[derive(Debug)]
pub struct Block {
    statements: Vec<Statement>,
}

impl Parseable for Block {
    fn parse(string: &str) -> Option<Result<Self>> {
        if !string.starts_with('{') || !string.ends_with('}') {
            return None;
        }

        let mut curly_lvl: usize = 0;
        let mut statements = Vec::new();
        let mut line = String::new();
        let chars = string
            .strip_prefix("{")
            .unwrap()
            .strip_suffix("}")
            .unwrap()
            .chars();

        for char in chars {
            line.push(char);
            match char {
                ';' => {
                    if curly_lvl == 0 {
                        let statement =
                            match Statement::parse(line.trim().strip_suffix(';').unwrap()) {
                                Some(Ok(statement)) => statement,
                                Some(Err(err)) => return Some(Err(err)),
                                None => {
                                    return Some(Err(Error::msg(format!(
                                        "ERROR: Cannot parse the following statement:\n{}",
                                        line.trim()
                                    ))))
                                }
                            };
                        statements.push(statement);
                        line = String::new();
                    }
                }
                '{' => curly_lvl += 1,
                '}' => curly_lvl -= 1,
                _ => (),
            }
        }

        Some(Ok(Self { statements }))
    }
}
