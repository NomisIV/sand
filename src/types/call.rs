use crate::*;
use anyhow::{Error, Result};

#[derive(Debug, Clone)]
pub struct Call {
    function: Value,
    parameters: Vec<Value>,
}

impl Call {
    pub fn new(function: Value, parameters: Vec<Value>) -> Self {
        Self {
            function,
            parameters,
        }
    }
}

pub trait Callable: Interpretable {
    fn get_arguments(&self) -> Vec<Var>;
}

impl Parseable for Call {
    fn parse(string: &str) -> Option<Result<Self>> {
        let (before, after) = string.split_once('(')?;
        let function = match Value::parse(before.trim()) {
            Some(Ok(function)) => function,
            Some(Err(err)) => return Some(Err(err)),
            None => {
                return Some(Err(Error::msg(format!(
                    "ERROR: Cannot parse the following value:\n{}",
                    before.trim()
                ))))
            }
        };
        let mut parameters = Vec::new();
        for parameter_str in after.trim().strip_suffix(')')?.split(',') {
            match Value::parse(parameter_str) {
                Some(Ok(parameter)) => parameters.push(parameter),
                Some(Err(err)) => return Some(Err(err)),
                None => {
                    return Some(Err(Error::msg(format!(
                        "ERROR: Cannot parse the following value:\n{}",
                        parameter_str
                    ))))
                }
            };
        }

        Some(Ok(Call {
            function,
            parameters,
        }))
    }
}

impl Interpretable for Call {
    fn interpret(&self, scope: &mut Scope) -> Result<Value> {
        // println!("== Interpreting call:\n{:?}", self);
        let function: Box<dyn Callable> = match &self.function {
            Value::Fun(function) => Box::new(function.clone()),
            Value::Met(method) => match method.interpret(scope) {
                Ok(Value::Fun(function)) => Box::new(function.clone()),
                Ok(Value::Intrinsic(intrinsic)) => Box::new(intrinsic.clone()),
                Ok(value) => {
                    return Err(Error::msg(format!(
                        "ERROR: Value is not a function:\n{:?}",
                        value
                    )))
                }
                Err(err) => return Err(err),
            },
            Value::Var(variable) => match scope.get(&variable) {
                Some(Value::Fun(function)) => Box::new(function.clone()),
                Some(Value::Intrinsic(intrinsic)) => Box::new(intrinsic.clone()),
                Some(value) => {
                    return Err(Error::msg(format!(
                        "ERROR: Variable is not a function:\n{:?}",
                        value
                    )))
                }
                None => return Err(Error::msg(format!("ERROR: Function is not in scope"))),
            },
            // Value::Var(variable) => Box::new(variable.interpret(scope)),
            value => {
                return Err(Error::msg(format!(
                    "ERROR: Value is not a function:\n{:?}",
                    value
                )))
            }
        };
        let arguments = function.get_arguments();

        if arguments.len() != self.parameters.len() {
            return Err(Error::msg("ERROR: Mismatched number of parameters"));
        }
        // println!("#### Arguments:\n{:#?}", arguments);

        let mut function_scope = scope.clone();

        for n in 0..arguments.len() {
            function_scope.insert(
                arguments.get(n).unwrap().clone(),
                self.parameters.get(n).unwrap().clone(),
            );
        }

        // println!("Scope for calling the function:\n{:#?}", function_scope);

        function.interpret(&mut function_scope)
    }
}
