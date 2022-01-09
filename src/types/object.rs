use crate::*;

#[derive(Debug, Clone)]
pub struct Obj {
    selff: Box<Value>,
    members: HashMap<Var, Value>,
}

impl Obj {
    pub fn new(value: Value) -> Self {
        Self {
            selff: Box::new(value),
            members: HashMap::new(),
        }
    }

    pub fn from_value(value: Value) -> Self {
        match value {
            Value::Num(num) => num_obj::init(num),
            Value::Obj(obj) => obj,
            value => Obj::new(value),
        }
    }

    pub fn add_member(&mut self, name: Var, value: Value) {
        self.members.insert(name, value);
    }

    pub fn get_member(&self, name: &Var) -> Option<&Value> {
        self.members.get(name)
    }
}

impl Parseable for Obj {
    fn parse(string: &str) -> Option<Result<Self>> {
        let selff = match Value::parse(string) {
            Some(Ok(selff)) => selff,
            Some(Err(err)) => return Some(Err(err)),
            None => return None,
        };
        let selff = Box::new(selff);
        let members = HashMap::new();
        Some(Ok(Obj { selff, members }))
    }
}

impl Interpretable for Obj {
    fn interpret(&self, scope: &mut Scope) -> Result<Value> {
        // println!("== Interpreting object:\n{:?}", self);
        scope.insert(Var::new("self"), *self.selff.clone());
        Ok(Value::Nope)
    }
}
