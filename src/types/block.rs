use crate::*;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Block {
    statements: Vec<Statement>,
}

impl Parseable for Block {
    fn parse(string: &str) -> Option<Result<Self>> {
        // println!("== Parsing block:\n{:?}", string);
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
                                    // return Some(Err(Error::msg(format!(
                                    //     "ERROR: Cannot parse the following statement:\n{}",
                                    //     line.trim()
                                    // ))))
                                    return None;
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

impl Interpretable for Block {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal> {
        // println!("== Interpreting block:\n{:?}", self);
        for statement in &self.statements {
            statement.interpret(scope)?;
        }
        // TODO: Body implement a way to return values
        Ok(Literal::Nope)
    }
}
