use crate::*;
use anyhow::Error;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Var {
    name: String,
}

impl Var {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl Parseable for Var {
    fn parse(string: &str) -> Option<Result<Self>> {
        let allowed_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVXYZ123456789_-";
        if let Some(_) = string.chars().find(|char| !allowed_chars.contains(*char)) {
            return None;
        }
        let name = string.to_string();
        Some(Ok(Var { name }))
    }
}

impl Interpretable for Var {
    fn interpret(&self, scope: &mut Scope) -> Result<Value> {
        // println!("Interpreting variable:\n{:?}", self);
        scope.get(&self).ok_or(Error::msg(format!("Variable `{}` not in scope", self.name))).map(|value| value.clone())
    }
}
