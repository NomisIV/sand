use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

use crate::*;

#[derive(Clone)]
pub struct Intrinsic {
    arguments: Vec<Var>,
    function: Rc<dyn Fn(&mut Scope) -> Value>,
}

impl Debug for Intrinsic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Intrinsic")
            .field("arguments", &self.arguments)
            .field("function", &"rust code lol")
            .finish()
    }
}

impl Intrinsic {
    pub fn new(arguments: Vec<Var>, function: Rc<dyn Fn(&mut Scope) -> Value>) -> Self {
        Self {
            arguments,
            function,
        }
    }
}

impl Callable for Intrinsic {
    fn get_arguments(&self) -> Vec<Var> {
        self.arguments.clone()
    }
}

impl Interpretable for Intrinsic {
    fn interpret(&self, scope: &mut Scope) -> Result<Value> {
        // println!("== Interpreting intrinsic:\n{:?}", self);
        let function = self.function.deref();
        Ok(function(scope))
    }
}
