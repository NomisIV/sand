use anyhow::Result;
use std::collections::HashMap;
use std::env;
use std::fmt::Write;
use std::fs;
use std::rc::Rc;

mod objects;
mod types;

use objects::*;
use types::*;

trait Parseable {
    fn parse(string: &str) -> Option<Result<Self>>
    where
        Self: Sized;
}

type Scope = HashMap<Var, Value>;

pub trait Interpretable {
    fn interpret(&self, scope: &mut Scope) -> Result<Value>;
}

trait Compileable {
    fn compile(&self, scope: &mut Scope, buffer: impl Write) -> Result<()>;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(args.get(1).unwrap()).unwrap();
    // println!("==== File:\n{}", file);
    let parse_result = Block::parse(&file.trim());
    if let Some(Ok(tokens)) = parse_result {
        // println!("==== Tokens:\n{:#?}", tokens);

        let mut scope: Scope = HashMap::new();
        scope.insert(Var::new("main"), Value::Obj(main_obj::init()));

        if let Err(err) = tokens.interpret(&mut scope) {
            eprintln!("{}", err)
        }
    } else if let Some(Err(err)) = parse_result {
        eprintln!("ERROR: {}", err.to_string());
    } else {
        eprintln!("ERROR: File is not a block");
    }
}
