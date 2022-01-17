use crate::*;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Function {
    arguments: Vec<Var>,
    body: Block,
}

impl Function {
    pub fn get_arguments(&self) -> Vec<Var> {
        self.arguments.clone()
    }
}

impl Parseable for Function {
    fn parse(string: &str) -> Option<Result<Self>> {
        // println!("== Parsing function:\n{:?}", string);
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
                    // return Some(Err(Error::msg(format!(
                    //     "ERROR: Cannot parse the following variable:\n{}",
                    //     argument_str
                    // ))))
                    return None;
                }
            }
        }
        let body = match Block::parse(after.trim()) {
            Some(Ok(block)) => block,
            Some(Err(err)) => return Some(Err(err)),
            None => {
                // return Some(Err(Error::msg(format!(
                //     "ERROR: Cannot parse the following block:\n{}",
                //     after.trim()
                // ))))
                return None;
            }
        };

        Some(Ok(Function { arguments, body }))
    }
}

impl Interpretable for Function {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal> {
        // println!("Interpreting function:\n{:?}", self);
        self.body.interpret(scope)
    }
}
