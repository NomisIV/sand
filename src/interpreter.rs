use std::collections::HashMap;
use std::fs;

use crate::intrinsics::init_scope;
use crate::parser::parse_file;
use crate::tokenizer::tokenize_str;
use crate::types::*;
use crate::FilePos;
use crate::SandError;

pub type Scope = HashMap<String, Literal>;

#[derive(Debug)]
pub struct InterpretingError {
    pub msg: String,
    pub pos: FilePos,
}

impl InterpretingError {
    pub fn new(msg: &str, pos: &FilePos) -> Self {
        Self {
            msg: msg.to_string(),
            pos: pos.clone(),
        }
    }
}

pub trait Interpret {
    fn interpret(self, scope: &mut Scope) -> Result<Literal, InterpretingError>;
}

impl Interpret for Callable {
    fn interpret(self, scope: &mut Scope) -> Result<Literal, InterpretingError> {
        match self {
            Self::Fun(fun) => fun.body.interpret(scope),
            Self::Intr(intr) => (intr.fun_interpret)(scope),
        }
    }
}

impl Interpret for Reference {
    fn interpret(self, scope: &mut Scope) -> Result<Literal, InterpretingError> {
        match self {
            Self::Var(var) => scope
                .get(&var.name)
                .ok_or(InterpretingError::new("Variable not in scope", &var.pos))
                .map(|val| val.clone()),
            Self::Member { set, field, .. } => {
                // let mut set_scope = scope.clone();
                let set_literal = set.interpret(scope)?;
                let set_set = match &set_literal {
                    Literal::Nope => scope.get("Nope").unwrap().clone().as_set().unwrap(),
                    Literal::Str(str) => {
                        scope.insert("self".to_string(), Literal::Str(str.clone()));
                        scope.get("Str").unwrap().clone().as_set().unwrap()
                    }
                    Literal::Char(char) => {
                        scope.insert("self".to_string(), Literal::Char(char.clone()));
                        scope.get("Char").unwrap().clone().as_set().unwrap()
                    }
                    Literal::Num(num) => {
                        scope.insert("self".to_string(), Literal::Num(num.clone()));
                        scope.get("Num").unwrap().clone().as_set().unwrap()
                    }
                    Literal::Bool(bool) => {
                        scope.insert("self".to_string(), Literal::Bool(bool.clone()));
                        scope.get("Bool").unwrap().clone().as_set().unwrap()
                    }
                    Literal::List(list) => {
                        scope.insert("self".to_string(), Literal::List(list.clone()));
                        scope.get("List").unwrap().clone().as_set().unwrap()
                    }
                    Literal::Fun(fun) => {
                        scope.insert("self".to_string(), Literal::Fun(fun.clone()));
                        scope.get("Fun").unwrap().clone().as_set().unwrap()
                    }
                    Literal::Set(set) => set.clone(),
                };
                Ok(set_set
                    .get(&field.name)
                    .ok_or(InterpretingError::new("Set has no such member", &field.pos))
                    .map(|val| val.clone())?)
            }
        }
    }
}

impl Interpret for Value {
    fn interpret(self, scope: &mut Scope) -> Result<Literal, InterpretingError> {
        match self {
            Self::Lit(lit) => Ok(lit),
            Self::Ref(r#ref) => r#ref.interpret(scope),
            Self::FunCall { fun, params, .. } => {
                let fun = fun.interpret(scope)?.as_fun().unwrap();
                let args = fun.get_args();
                if args.len() != params.len() {
                    return Err(InterpretingError::new("Mismatched arity", &FilePos::temp()));
                }
                let mut fun_scope = scope.clone();
                for n in 0..args.len() {
                    let var = args.get(n).unwrap().name.clone();
                    let val = params.get(n).unwrap().clone().interpret(scope)?;
                    fun_scope.insert(var, val);
                }
                fun.interpret(&mut fun_scope)
            }
        }
    }
}

impl Interpret for Statement {
    fn interpret(self, scope: &mut Scope) -> Result<Literal, InterpretingError> {
        match self {
            Self::Assignment { var, val, .. } => {
                match var {
                    Reference::Var(var) => {
                        scope.insert(var.name.clone(), val.interpret(&mut scope.clone())?);
                        assert!(scope.contains_key(&var.name));
                        Ok(Literal::Nope)
                    }
                    Reference::Member { set, field, pos } => {
                        if let Value::Ref(Reference::Var(var)) = *set {
                            let mut parent =
                                scope.get(&var.name).unwrap().clone().as_set().unwrap(); // TODO: Handle unwraps
                            parent.insert(field.name, val.interpret(scope)?);
                            scope.insert(var.name, Literal::Set(parent));
                            Ok(Literal::Nope)
                        } else {
                            return Err(InterpretingError::new(
                                "Complex referencing is not supported yet",
                                &pos,
                            ));
                        }
                    }
                }
            }
            Self::Value(val) => val.interpret(scope),
            Self::Include(file) => {
                let str = fs::read_to_string(&file).map_err(|err| {
                    InterpretingError::new(
                        &format!("Cannot include `{}` because:\n{}", file.display(), err),
                        &FilePos::temp(),
                    )
                })?;
                let tokens = tokenize_str(&str, FilePos::new(&file, 1, 1)).map_err(|err| {
                    InterpretingError::new(
                        &format!(
                            "Cannot tokenize `{}` because:\n{}",
                            file.display(),
                            SandError::from(err)
                        ),
                        &FilePos::temp(),
                    )
                })?;
                let tree = parse_file(tokens).map_err(|err| {
                    InterpretingError::new(
                        &format!(
                            "Cannot parse `{}` because:\n{}",
                            file.display(),
                            SandError::from(err)
                        ),
                        &FilePos::temp(),
                    )
                })?;
                tree.interpret(scope)
            }
        }
    }
}

impl Interpret for Statements {
    fn interpret(self, scope: &mut Scope) -> Result<Literal, InterpretingError> {
        if self.0.len() > 1 {
            for i in 0..self.0.len() - 1 {
                self.0.get(i).unwrap().clone().interpret(scope)?;
            }
        }
        self.0.last().unwrap().clone().interpret(scope)
    }
}

pub fn interpret_file(tree: Statements, args: Vec<String>) -> Result<i32, InterpretingError> {
    let mut scope = init_scope();
    tree.interpret(&mut scope)?;
    let call = Value::FunCall {
        fun: Box::new(Value::Lit(scope.get("main").unwrap().clone())),
        params: vec![Value::Lit(Literal::Str("ur mom gae lol".to_string()))], // TODO
        pos: FilePos::internal(),
    };
    let exit_code = call.interpret(&mut scope)?.as_int().unwrap();
    Ok(exit_code as i32)
}

#[cfg(test)]
mod tests {
    use crate::intrinsics::init_scope;

    use super::*;

    #[test]
    fn interpret_statement_assignment() {
        let str = r#"let foo = "Hello World!"; foo"#;
        let tokens = tokenize_str(str, FilePos::internal()).unwrap();
        let tree = parse_file(tokens).unwrap();
        let mut scope = init_scope();
        tree.interpret(&mut scope).unwrap();
        assert!(scope.contains_key("foo"));
        assert!(scope.get("foo").unwrap() == &Literal::Str("Hello World!".to_string()))
    }

    // TODO: This is not testable until I can define sets
    // #[test]
    // fn interpret_statement_assignment_complex() {
    //     let str = r#"let Foo.bar = "Hello World!";"#;
    //     let tokens = tokenize_str(str, FilePos::internal()).unwrap();
    //     let tree = parse_file(tokens).unwrap();
    //     let mut scope = init_scope();
    //     tree.interpret(&mut scope).unwrap();
    //     assert!(scope.contains_key("Foo"));
    //     let foo = scope.get("Foo").unwrap().clone().as_set().unwrap();
    //     assert!(foo.contains_key("bar"));
    //     assert!(foo.get("bar").unwrap() == &Literal::Str("Hello World!".to_string()))
    // }

    // TODO: This one is harder to test, since it depends on external files
    // #[test]
    // fn interpret_statement_include() {
    //     let str = r#"include "std.sand""#;
    //     let tokens = tokenize_str(str, &PathBuf::new(), 1, 1).unwrap();
    //     let tree = parse_file(tokens).unwrap();
    //     assert!(tree.interpret(&mut init_scope()).unwrap() == Literal::Str("Hello World!".to_string()))
    // }

    #[test]
    fn interpret_value_lit() {
        let str = r#""Hello World!""#;
        let tokens = tokenize_str(str, FilePos::internal()).unwrap();
        let tree = parse_file(tokens).unwrap();
        assert!(
            tree.interpret(&mut init_scope()).unwrap() == Literal::Str("Hello World!".to_string())
        )
    }

    #[test]
    fn interpret_value_ref() {
        let str = r#"let foo = "hello"; foo"#;
        let tokens = tokenize_str(str, FilePos::internal()).unwrap();
        let tree = parse_file(tokens).unwrap();
        assert!(tree.interpret(&mut init_scope()).unwrap() == Literal::Str("hello".to_string()))
    }

    #[test]
    fn interpret_value_ref_complex() {
        let str = r#"1.add(1)"#;
        let tokens = tokenize_str(str, FilePos::internal()).unwrap();
        let tree = parse_file(tokens).unwrap();
        assert!(tree.interpret(&mut init_scope()).unwrap() == Literal::Num(2.0))
    }

    #[test]
    fn interpret_value_funcall() {
        let str = r#"(var) { var } ("foo")"#;
        let tokens = tokenize_str(str, FilePos::internal()).unwrap();
        let tree = parse_file(tokens).unwrap();
        assert!(tree.interpret(&mut init_scope()).unwrap() == Literal::Str("foo".to_string()))
    }
}
