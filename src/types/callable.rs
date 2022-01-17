use crate::*;
use anyhow::Result;

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

impl Parseable for Callable {
    fn parse(string: &str) -> Option<Result<Self>> {
        // println!("== Parsing callable:\n{:?}", string);
        Function::parse(string)
            .map(|function_result| function_result.map(|function| Callable::Function(function)))
    }
}

impl Interpretable for Callable {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal> {
        match self {
            Callable::Function(function) => function.interpret(scope),
            Callable::Intrinsic(intrinsic) => intrinsic.interpret(scope),
        }
    }
}
