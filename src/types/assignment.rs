use crate::*;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Assignment {
    var: Var,
    value: Value,
}

impl Parseable for Assignment {
    fn parse(string: &str) -> Option<Result<Self>> {
        let (before, after) = string.split_once('=')?;
        let var = Var::parse(before.strip_prefix("let")?.trim())?.ok()?;
        let value = Value::parse(after.trim())?.ok()?;
        Some(Ok(Assignment { var, value }))
    }
}

impl Interpretable for Assignment {
    fn interpret(&self, scope: &mut Scope) -> Result<Value> {
        // println!("== Interpreting assignment:\n{:?}", self);
        scope.insert(self.var.clone(), self.value.clone());
        Ok(Value::Nope)
    }
}
