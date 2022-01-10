use anyhow::Result;
use std::collections::HashMap;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use structopt::StructOpt;

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

#[derive(StructOpt)]
enum Cmd {
    Parse {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
    Run {
        #[structopt(parse(from_os_str))]
        file: PathBuf,
    },
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Cmd,
}

fn main() {
    let opt = Opt::from_args();

    match opt.subcommand {
        Cmd::Parse { file } => {
            let file_contents = fs::read_to_string(file).unwrap();
            println!("==== File:\n{}", file_contents);
            let parse_result = Block::parse(&file_contents.trim());
            if let Some(Ok(tokens)) = parse_result {
                println!("==== Tokens:\n{:#?}", tokens);

                let mut scope: Scope = HashMap::new();
                scope.insert(Var::new("main"), Value::Obj(main_obj::init()));
            } else if let Some(Err(err)) = parse_result {
                eprintln!("ERROR: {}", err.to_string());
            } else {
                eprintln!("ERROR: File is not a block");
            }
        }
        Cmd::Run { file } => {
            let file_contents = fs::read_to_string(file).unwrap();
            let parse_result = Block::parse(&file_contents.trim());
            if let Some(Ok(tokens)) = parse_result {
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
    }
}
