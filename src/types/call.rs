use crate::*;

#[derive(Debug, Clone)]
pub struct Call {
    callable: Value,
    parameters: Vec<Value>,
}

impl Call {
    pub fn new(callable: Value, parameters: Vec<Value>) -> Self {
        Self {
            callable,
            parameters,
        }
    }
}

impl FromStr for Call {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing call:\n{:?}", string);
        let (before, after) = s
            .split_once('(')
            .ok_or(SandParseError::Unidentifiable(s.into(), "call".into()))?;
        let callable = Value::from_str(before.trim())?;
        let mut parameters = Vec::new();
        for parameter_str in after
            .trim()
            .strip_suffix(')')
            .ok_or(SandParseError::Unidentifiable(s.into(), "call".into()))?
            .split(',')
        {
            parameters.push(Value::from_str(parameter_str)?);
        }

        Ok(Call {
            callable,
            parameters,
        })
    }
}

impl Interpretable for Call {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        // println!("== Interpreting call:\n{:?}", self);
        let function = &self.callable.interpret(scope)?.as_callable()?;
        let arguments = function.get_arguments();

        if arguments.len() != self.parameters.len() {
            return Err(SandInterpretingError::MismatchedParameters);
        }
        // println!("#### Arguments:\n{:#?}", arguments);

        let mut function_scope = scope.clone();

        for n in 0..arguments.len() {
            function_scope.insert(
                arguments.get(n).unwrap().clone(),
                self.parameters.get(n).unwrap().clone().interpret(scope)?,
            );
        }

        // println!("Scope for calling the function:\n{:#?}", function_scope);

        function.interpret(&mut function_scope)
    }
}
