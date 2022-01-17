use crate::*;
use anyhow::{Result, Error};

#[derive(Debug, Clone)]
pub enum Value {
    Literal(Literal),
    Variable(Var),
    Member(Member),
}

impl Value {
}

impl Parseable for Value {
    fn parse(string: &str) -> Option<Result<Self>> {
        // println!("== Parsing value:\n{:?}", string);
        if let Some(literal_result) = Literal::parse(string) {
            let literal = match literal_result {
                Ok(literal) => literal,
                Err(err) => {
                    return Some(Err(Error::msg(format!(
                        "ERROR: Cannot parse the following literal:\n{}\nbecause of:\n{}",
                        string, err
                    ))))
                }
            };
            Some(Ok(Value::Literal(literal)))
        } else if let Some(var_result) = Var::parse(string) {
            let var = match var_result {
                Ok(var) => var,
                Err(err) => {
                    return Some(Err(Error::msg(format!(
                        "ERROR: Cannot parse the following variable:\n{}\nbecause of:\n{}",
                        string, err
                    ))))
                }
            };
            Some(Ok(Value::Variable(var)))
        } else if let Some(member_result) = Member::parse(string) {
            let member = match member_result {
                Ok(member) => member,
                Err(err) => {
                    return Some(Err(Error::msg(format!(
                        "ERROR: Cannot parse the following member:\n{}\nbecause of:\n{}",
                        string, err
                    ))))
                }
            };
            Some(Ok(Value::Member(member)))
        } else {
            None
        }
    }
}

impl Interpretable for Value {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal> {
        match self {
            Value::Literal(literal) => Ok(literal.clone()),
            Value::Variable(variable) => variable.interpret(scope),
            Value::Member(member) => member.interpret(scope),
        }
    }
}
