use crate::*;

#[derive(Debug, Clone)]
pub struct Assignment {
    var: Var,
    value: Value,
}

impl FromStr for Assignment {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing assignment:\n{:?}", string);
        let (before, after) = s.split_once('=').ok_or(SandParseError::Unidentifiable(
            s.into(),
            "assignment".into(),
        ))?;
        let var = Var::from_str(
            before
                .strip_prefix("let")
                .ok_or(SandParseError::Unidentifiable(
                    s.into(),
                    "assignment".into(),
                ))?
                .trim(),
        )?;
        let value = Value::from_str(after.trim())?;
        Ok(Assignment { var, value })
    }
}

impl Interpretable for Assignment {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        // println!("== Interpreting assignment:\n{:?}", self);
        scope.insert(
            self.var.clone(),
            self.value.clone().interpret(&mut scope.clone())?,
        );
        Ok(Literal::Nope)
    }
}
