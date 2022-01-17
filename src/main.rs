use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use structopt::StructOpt;

mod objects;
mod types;

use objects::*;
use types::*;

// TODO: Use FromStr instead
trait Parseable {
    fn parse(string: &str) -> Option<Result<Self>>
    where
        Self: Sized;
}

type Scope = HashMap<Var, Literal>;

pub trait Interpretable {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal>;
}

// trait Compileable {
//     fn compile(&self, scope: &mut Scope, buffer: impl Write) -> Result<()>;
// }

#[derive(StructOpt)]
enum Cmd {
    Parse,
    Run,
}

#[derive(StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    subcommand: Cmd,

    #[structopt(parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let file_contents = fs::read_to_string(opt.file).unwrap();

    match opt.subcommand {
        Cmd::Parse => {
            println!("==== File:\n{}", file_contents);
            let parse_result = Block::from_str(&file_contents.trim());
            if let Ok(tokens) = parse_result {
                println!("==== Tokens:\n{:#?}", tokens);

                let mut scope: Scope = HashMap::new();
                scope.insert(Var::new("main"), Literal::Object(main_obj::init()));
            } else {
                eprintln!("ERROR: File is not a block");
            }
        }
        Cmd::Run => {
            let parse_result = Block::from_str(&file_contents.trim());
            if let Ok(tokens) = parse_result {
                let mut scope: Scope = HashMap::new();
                scope.insert(Var::new("main"), Literal::Object(main_obj::init()));

                if let Err(err) = tokens.interpret(&mut scope) {
                    eprintln!("{}", err)
                }
            } else {
                eprintln!("ERROR: File is not a block");
            }
        }
    }
}
