use crate::interpreter::{InterpretingError, Scope};
use std::fmt;
use std::rc::Rc;

// TODO: Make members not public, or is this a bad idea?
pub struct Intrinsic {
    pub args: Vec<Var>,
    pub fun_interpret: Rc<dyn Fn(&mut Scope) -> Result<Literal, InterpretingError>>,
}

impl fmt::Debug for Intrinsic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Intrinsic")
            .field("args", &self.args)
            .finish()
    }
}

impl PartialEq for Intrinsic {
    fn eq(&self, other: &Self) -> bool {
        self.args == other.args && Rc::ptr_eq(&self.fun_interpret, &other.fun_interpret)
    }
}

#[derive(Debug, PartialEq)]
pub enum Callable {
    Fun(Function),
    Intr(Intrinsic),
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub args: Vec<Var>,
    pub body: Block,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Nope,
    Str(String),
    Num(isize),
    Bool(bool),
    Fun(Callable),
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Lit(Literal),
    Ref(Reference),
    FunCall { fun: Box<Value>, params: Vec<Value> },
}

#[derive(Debug, PartialEq)]
pub struct Var(pub String);

#[derive(Debug, PartialEq)]
pub enum Reference {
    Var(Var),
    Member { val: Box<Value>, field: Var },
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assignment { var: Reference, val: Value },
    Value(Value),
    Include(String),
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}
