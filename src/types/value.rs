use crate::*;

#[derive(Debug, Clone)]
pub enum Value {
    Literal(Literal),
    Variable(Var),
    Member(Member),
}

impl Value {}

impl FromStr for Value {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing value:\n{:?}", string);
        Literal::from_str(s)
            .map(|literal| Value::Literal(literal))
            .or(Var::from_str(s).map(|var| Value::Variable(var)))
            .or(Member::from_str(s).map(|member| Value::Member(member)))
            // .map_err(|err| match err {
            //     SandParseError::ParseErr(msg) => SandParseError::ParseErr(format!(
            //         "Cannot parse the following string into a value:\n{}\nbecause of:\n{}",
            //         s, msg
            //     )),
            //     SandParseError::Unidentifiable(_, _) => err,
            // })
    }
}

impl Interpretable for Value {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        match self {
            Value::Literal(literal) => Ok(literal.clone()),
            Value::Variable(variable) => variable.interpret(scope),
            Value::Member(member) => member.interpret(scope),
        }
    }
}
