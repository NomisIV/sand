use crate::*;

// TODO: Implement this as a trait instead
#[derive(Debug, Clone)]
pub enum Callable {
    Function(Function),
    Intrinsic(Intrinsic),
}

impl Callable {
    pub fn new(intrinsic: Intrinsic) -> Self {
        Self::Intrinsic(intrinsic)
    }

    pub fn get_arguments(&self) -> Vec<Var> {
        match self {
            Callable::Function(function) => function.get_arguments(),
            Callable::Intrinsic(intrinsic) => intrinsic.get_arguments(),
        }
    }
}

impl FromStr for Callable {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing callable:\n{:?}", string);
        Err(SandParseError::Unidentifiable)
            .or(Function::from_str(s).map(|function| Callable::Function(function)))
    }
}

impl Interpretable for Callable {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        match self {
            Callable::Function(function) => function.interpret(scope),
            Callable::Intrinsic(intrinsic) => intrinsic.interpret(scope),
        }
    }
}
