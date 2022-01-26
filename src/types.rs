use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;

use crate::interpreter::{InterpretingError, Scope};
use crate::FilePos;

#[derive(Debug)]
pub struct TypeError {
    pub msg: String,
    pub pos: FilePos,
}

impl TypeError {
    fn new(msg: &str, pos: &FilePos) -> Self {
        Self {
            msg: msg.to_string(),
            pos: pos.clone(),
        }
    }
}

// TODO: Add a position field for every type
// TODO: Make members not public, or is this a bad idea?
#[derive(Clone)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
    Fun(Function),
    Intr(Intrinsic),
}

impl Callable {
    pub fn get_args(&self) -> Vec<Var> {
        match self {
            Self::Fun(fun) => fun.args.clone(),
            Self::Intr(intr) => intr.args.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub args: Vec<Var>,
    pub body: Statements,
    pub pos: FilePos,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Nope,
    Str(String),
    Char(char),
    Num(f64),
    Bool(bool),
    List(Vec<Literal>),
    Fun(Callable),
    Set(HashMap<String, Literal>),
}

impl Literal {
    pub fn as_nope(self) -> Result<(), TypeError> {
        match self {
            Self::Nope => Ok(()),
            _ => Err(TypeError::new("Literal is not a Nope", &FilePos::temp())),
        }
    }

    pub fn as_str(self) -> Result<String, TypeError> {
        match self {
            Self::Str(str) => Ok(str),
            _ => Err(TypeError::new("Literal is not a string", &FilePos::temp())),
        }
    }

    pub fn as_num(self) -> Result<f64, TypeError> {
        match self {
            Self::Num(num) => Ok(num),
            _ => Err(TypeError::new("Literal is not a number", &FilePos::temp())),
        }
    }

    pub fn as_int(self) -> Result<isize, TypeError> {
        match self {
            Self::Num(num) => {
                if num.fract() == 0.0 {
                    Ok(num as isize)
                } else {
                    Err(TypeError::new("Number is not an integer", &FilePos::temp()))
                }
            }
            _ => Err(TypeError::new("Literal is not a number", &FilePos::temp())),
        }
    }

    pub fn as_bool(self) -> Result<bool, TypeError> {
        match self {
            Self::Bool(bool) => Ok(bool),
            _ => Err(TypeError::new("Literal is not a boolean", &FilePos::temp())),
        }
    }

    pub fn as_list(self) -> Result<Vec<Literal>, TypeError> {
        match self {
            Self::List(list) => Ok(list),
            _ => Err(TypeError::new("Literal is not a boolean", &FilePos::temp())),
        }
    }

    pub fn as_fun(self) -> Result<Callable, TypeError> {
        match self {
            Self::Fun(fun) => Ok(fun),
            _ => Err(TypeError::new(
                "Literal is not a function",
                &FilePos::temp(),
            )),
        }
    }

    pub fn as_set(self) -> Result<HashMap<String, Literal>, TypeError> {
        match self {
            Self::Set(set) => Ok(set),
            _ => Err(TypeError::new("Literal is not a set", &FilePos::temp())),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Lit(Literal),
    Ref(Reference),
    FunCall {
        fun: Box<Value>,
        params: Vec<Value>,
        pos: FilePos,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var {
    pub name: String,
    pub pos: FilePos,
}

impl Var {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            pos: FilePos::internal(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Reference {
    Var(Var),
    Member {
        set: Box<Value>,
        field: Var,
        pos: FilePos,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Assignment {
        var: Reference,
        val: Value,
        pos: FilePos,
    },
    Value(Value),
    Include(PathBuf),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statements(pub Vec<Statement>);
