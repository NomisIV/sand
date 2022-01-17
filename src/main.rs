use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;
use structopt::StructOpt;

mod objects;
mod types;

use objects::*;
use types::*;

// TODO: Add locations for all tokens, and include them in the error
#[derive(Debug)]
pub enum SandParseError {
    ParseErr(String),
    Unidentifiable(String, String),
}

impl fmt::Display for SandParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseErr(msg) => write!(f, "{}", msg),
            Self::Unidentifiable(string, r#type) => write!(
                f,
                "Cannot parse the following string into a {type}:\n{string}",
                string = string,
                r#type = r#type
            ),
        }
    }
}

#[derive(Debug)]
pub enum SandInterpretingError {
    NotInScope,
    MismatchedParameters,
    NoMember,
    BadValue,
}

impl fmt::Display for SandInterpretingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotInScope => write!(f, "Value not in scope"),
            Self::MismatchedParameters => write!(f, "Mismatched parameters"),
            Self::NoMember => write!(f, "Object has no such member"),
            Self::BadValue => write!(f, "The value is of the wrong type"),
        }
    }
}

type Scope = HashMap<Var, Literal>;

pub trait Interpretable {
    fn interpret(&self, scope: &mut Scope) -> Result<Literal, SandInterpretingError>;
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
            match Block::from_str(&file_contents.trim()) {
                Ok(tree) => {
                    println!("==== Tree:\n{:#?}", tree);
                }
                Err(err) => {
                    eprintln!("ERROR: {}", err)
                }
            }
        }
        Cmd::Run => match Block::from_str(&file_contents.trim()) {
            Ok(tree) => {
                let mut scope: Scope = HashMap::new();
                scope.insert(Var::new("main"), Literal::Object(main_obj::init()));

                match tree.interpret(&mut scope) {
                    Ok(_) => (),
                    Err(err) => eprintln!("ERROR: {}", err),
                }
            }
            Err(err) => eprintln!("ERROR: {}", err),
        },
    }
}
