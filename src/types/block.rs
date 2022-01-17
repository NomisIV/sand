use crate::*;

#[derive(Debug, Clone)]
pub struct Block {
    statements: Vec<Statement>,
}

impl FromStr for Block {
    type Err = SandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing block:\n{:?}", string);
        if !s.starts_with('{') || !s.ends_with('}') {
            return Err(SandParseError::Unidentifiable(s.into(), "block".into()));
        }

        let mut curly_lvl: usize = 0;
        let mut statements = Vec::new();
        let mut line = String::new();
        let chars = s
            .strip_prefix("{")
            .unwrap()
            .strip_suffix("}")
            .unwrap()
            .chars();

        for char in chars {
            line.push(char);
            match char {
                ';' => {
                    if curly_lvl == 0 {
                        let statement_str = line.trim().strip_suffix(';').unwrap();
                        let statement = Statement::from_str(statement_str)?;
                        statements.push(statement);
                        line = String::new();
                    }
                }
                '{' => curly_lvl += 1,
                '}' => curly_lvl -= 1,
                _ => (),
            }
        }

        Ok(Self { statements })
    }
}

impl Interpretable for Block {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        // println!("== Interpreting block:\n{:?}", self);
        for statement in &self.statements {
            statement.interpret(scope)?;
        }
        // TODO: Body implement a way to return values
        Ok(Literal::Nope)
    }
}
