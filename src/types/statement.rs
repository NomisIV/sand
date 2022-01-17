use crate::*;

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(Assignment),
    Call(Call),
}

impl FromStr for Statement {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing statement:\n{:?}", string);
        Assignment::from_str(s)
            .map(|assignment| Statement::Assignment(assignment))
            .or(Call::from_str(s).map(|call| Statement::Call(call)))
    }
}

impl Interpretable for Statement {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        // println!("== Interpreting statement:\n{:?}", self);
        match self {
            Self::Assignment(assignment) => assignment.interpret(scope),
            Self::Call(call) => call.interpret(scope),
        }
    }
}
