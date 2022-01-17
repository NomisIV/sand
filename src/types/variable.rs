use crate::*;

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

impl FromStr for Var {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing variable:\n{:?}", string);
        let allowed_chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVXYZ123456789_-";
        if let Some(_) = s.chars().find(|char| !allowed_chars.contains(*char)) {
            return Err(SandParseError::Unidentifiable(s.into(), "variable".into()));
        }
        let name = s.to_string();
        Ok(Var { name })
    }
}

impl Interpretable for Var {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        // println!("Interpreting variable:\n{:?}", self);
        scope
            .get(&self)
            .ok_or(SandInterpretingError::NotInScope)
            .map(|value| value.clone())
    }
}
