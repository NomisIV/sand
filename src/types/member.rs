use crate::*;

#[derive(Debug, Clone)]
pub struct Member {
    object: Box<Value>,
    field: Var,
}

impl FromStr for Member {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing member:\n{:?}", string);
        if !s.contains('.') {
            return Err(SandParseError::Unidentifiable(s.into(), "member".into()));
        }
        let (value_str, field_str) = s.rsplit_once('.').unwrap();
        let value = Value::from_str(value_str)?;
        let object = Box::new(value);
        let field = Var::from_str(field_str)?;
        Ok(Member { object, field })
    }
}

impl Interpretable for Member {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        // println!("== Interpreting method:\n{:?}", self);
        let object = Object::from_literal(self.object.interpret(scope)?);
        object.interpret(scope)?;
        if let Some(literal) = object.get_member(&self.field) {
            Ok(literal.clone())
        } else {
            return Err(SandInterpretingError::NoMember);
        }
    }
}
