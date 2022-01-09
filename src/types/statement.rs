use crate::*;
use anyhow::{Error, Result};

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(Assignment),
    Call(Call),
}

impl Parseable for Statement {
    fn parse(string: &str) -> Option<Result<Self>> {
        if let Some(assignment_result) = Assignment::parse(string) {
            let assignment = match assignment_result {
                Ok(assignment) => assignment,
                Err(err) => {
                    return Some(Err(Error::msg(format!(
                        "ERROR: Cannot parse the following assignment:\n{}\nbecause of:\n{}",
                        string, err
                    ))))
                }
            };
            return Some(Ok(Statement::Assignment(assignment)));
        } else if let Some(call_result) = Call::parse(string) {
            let call = match call_result {
                Ok(call) => call,
                Err(err) => {
                    return Some(Err(Error::msg(format!(
                        "ERROR: Cannot parse the following call:\n{}\nbecause of:\n{}",
                        string, err
                    ))))
                }
            };
            return Some(Ok(Statement::Call(call)));
        } else {
            return None;
        }
    }
}

impl Interpretable for Statement {
    fn interpret(&self, scope: &mut Scope) -> Result<Value> {
        // println!("== Interpreting statement:\n{:?}", self);
        match self {
            Self::Assignment(assignment) => assignment.interpret(scope),
            Self::Call(call) => call.interpret(scope),
        }
    }
}
