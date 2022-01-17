use crate::*;
use anyhow::{Error, Result};

#[derive(Debug, Clone)]
pub struct Member {
    object: Box<Value>,
    field: Var,
}

impl Parseable for Member {
    fn parse(string: &str) -> Option<Result<Self>> {
        // println!("== Parsing member:\n{:?}", string);
        if !string.contains('.') {
            return None;
        }
        let (value_str, field_str) = string.rsplit_once('.')?;
        let value = match Value::parse(value_str) {
            Some(Ok(value)) => value,
            Some(Err(err)) => return Some(Err(err)),
            None => {
                // return Some(Err(Error::msg(format!(
                //     "ERROR: Cannot parse the following object:\n{}",
                //     object_str
                // ))));
                return None;
            }
        };
        let object = Box::new(value);
        let field = match Var::parse(field_str) {
            Some(Ok(field)) => field,
            Some(Err(err)) => return Some(Err(err)),
            None => {
                // return Some(Err(Error::msg(format!(
                //     "ERROR: Cannot parse the following variable:\n{}",
                //     function_str
                // ))));
                return None;
            }
        };
        Some(Ok(Member { object, field }))
    }
}

impl Interpretable for Member {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal> {
        // println!("== Interpreting method:\n{:?}", self);
        let object = Object::from_literal(self.object.interpret(scope)?);
        object.interpret(scope)?;
        if let Some(literal) = object.get_member(&self.field) {
            Ok(literal.clone())
        } else {
            return Err(Error::msg(format!(
                "ERROR: Value `{:?}` does not have the field `{:?}`",
                object,
                self.field.get_name()
            )));
        }
    }
}
