use crate::*;

// TODO: Objects are basically just sets, so rename objects to sets instead
#[derive(Debug, Clone)]
pub struct Object {
    members: HashMap<Var, Literal>,
}

impl Object {
    pub fn new() -> Self {
        Self {
            members: HashMap::new(),
        }
    }

    pub fn from_literal(literal: Literal) -> Self {
        match literal {
            // Literal::Nope => (),
            // Literal::String(string) => (),
            Literal::Number(number) => num_obj::init(number),
            // Literal::Boolean(bool) => (),
            // Literal::Callable(callable) => (),
            Literal::Object(object) => object,
            _ => {
                let mut object = Object::new();
                object.add_member(Var::new("self"), literal);
                object
            }
        }
    }

    pub fn add_member(&mut self, name: Var, value: Literal) {
        self.members.insert(name, value);
    }

    pub fn get_member(&self, name: &Var) -> Option<&Literal> {
        self.members.get(name)
    }
}

impl FromStr for Object {
    type Err = SandParseError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        // println!("== Parsing object:\n{:?}", string);
        unimplemented!()
    }
}

impl Interpretable for Object {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError> {
        for (var, value) in self.members.iter() {
            scope.insert(var.clone(), value.clone());
        }
        Ok(Literal::Object(self.clone()))
    }
}
