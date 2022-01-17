use crate::*;

#[derive(Debug, Clone)]
pub struct Function {
    arguments: Vec<Var>,
    body: Block,
}

impl Function {
    pub fn get_arguments(&self) -> Vec<Var> {
        self.arguments.clone()
    }
}

impl FromStr for Function {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing function:\n{:?}", string);
        let (before, after) = s
            .split_once(')')
            .ok_or(SandParseError::Unidentifiable(s.into(), "function".into()))?;
        let mut arguments = Vec::new();
        for argument_str in before
            .trim()
            .strip_prefix('(')
            .ok_or(SandParseError::Unidentifiable(s.into(), "function".into()))?
            .split(',')
        {
            if argument_str.is_empty() {
                continue; // TODO: This might cause bugs with empty arguments
            }

            arguments.push(Var::from_str(argument_str)?);
        }
        let body = Block::from_str(after.trim())?;

        Ok(Function { arguments, body })
    }
}

impl Interpretable for Function {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        // println!("Interpreting function:\n{:?}", self);
        self.body.interpret(scope)
    }
}
