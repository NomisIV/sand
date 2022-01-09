use crate::*;
use anyhow::{Error, Result};

// TODO: Generalize this to be a member instead of only methods,
// implement it kind of like variables
#[derive(Debug, Clone)]
pub struct Met {
    object: Box<Value>,
    function: Var,
}

impl Parseable for Met {
    fn parse(string: &str) -> Option<Result<Self>> {
        let (object_str, function_str) = string.rsplit_once('.')?;
        let object = match Value::parse(object_str) {
            Some(Ok(object)) => object,
            Some(Err(err)) => return Some(Err(err)),
            None => {
                return Some(Err(Error::msg(format!(
                    "ERROR: Cannot parse the following object:\n{}",
                    object_str
                ))));
            }
        };
        let object = Box::new(object);
        let function = match Var::parse(function_str) {
            Some(Ok(function)) => function,
            Some(Err(err)) => return Some(Err(err)),
            None => {
                return Some(Err(Error::msg(format!(
                    "ERROR: Cannot parse the following variable:\n{}",
                    function_str
                ))));
            }
        };
        Some(Ok(Met { object, function }))
    }
}

impl Interpretable for Met {
    fn interpret(&self, scope: &mut Scope) -> Result<Value> {
        // println!("== Interpreting method:\n{:?}", self);
        let object = match &*self.object {
            Value::Obj(obj) => obj.clone(),
            Value::Var(var) => match scope.get(var) {
                Some(Value::Obj(obj)) => obj.clone(),
                Some(value) => Obj::from_value(value.clone()),
                None => return Err(Error::msg(format!("ERROR: Variable `{}` is not in scope", var.get_name())))
            },
            value => Obj::from_value(value.clone()),
        };
        object.interpret(scope)?;
        if let Some(value) = object.get_member(&self.function) {
            Ok(value.clone())
        } else {
            return Err(Error::msg(format!(
                "ERROR: Object `{:?}` does not have the function `{:?}`",
                object, self.function.get_name()
            )));
        }
    }
}
