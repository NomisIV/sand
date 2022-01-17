use crate::*;
use anyhow::{Error, Result};

#[derive(Debug, Clone)]
pub struct Call {
    callable: Value,
    parameters: Vec<Value>,
}

impl Call {
    pub fn new(callable: Value, parameters: Vec<Value>) -> Self {
        Self {
            callable,
            parameters,
        }
    }
}

impl Parseable for Call {
    fn parse(string: &str) -> Option<Result<Self>> {
        // println!("== Parsing call:\n{:?}", string);
        let (before, after) = string.split_once('(')?;
        let callable = match Value::parse(before.trim()) {
            Some(Ok(function)) => function,
            Some(Err(err)) => return Some(Err(err)),
            None => {
                // return Some(Err(Error::msg(format!(
                //     "ERROR: Cannot parse the following function:\n{}",
                //     before.trim()
                // ))))
                return None;
            }
        };
        let mut parameters = Vec::new();
        for parameter_str in after.trim().strip_suffix(')')?.split(',') {
            match Value::parse(parameter_str) {
                Some(Ok(parameter)) => parameters.push(parameter),
                Some(Err(err)) => return Some(Err(err)),
                None => {
                    // return Some(Err(Error::msg(format!(
                    //     "ERROR: Cannot parse the following value:\n{}",
                    //     parameter_str
                    // ))))
                    return None;
                }
            };
        }

        Some(Ok(Call {
            callable,
            parameters,
        }))
    }
}

impl Interpretable for Call {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal> {
        // println!("== Interpreting call:\n{:?}", self);
        let function = &self.callable.interpret(scope)?.as_callable()?;
        let arguments = function.get_arguments();

        if arguments.len() != self.parameters.len() {
            return Err(Error::msg("ERROR: Mismatched number of parameters"));
        }
        // println!("#### Arguments:\n{:#?}", arguments);

        let mut function_scope = scope.clone();

        for n in 0..arguments.len() {
            function_scope.insert(
                arguments.get(n).unwrap().clone(),
                self.parameters.get(n).unwrap().clone().interpret(scope)?,
            );
        }

        // println!("Scope for calling the function:\n{:#?}", function_scope);

        function.interpret(&mut function_scope)
    }
}
