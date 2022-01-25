use std::fmt;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

mod interpreter;
mod intrinsics;
mod parser;
mod tokenizer;
mod types;

use interpreter::Interpret;
use interpreter::InterpretingError;
use intrinsics::*;
use parser::parse_tokens;
use parser::ParseError;
use tokenizer::tokenize_str;
use tokenizer::Token;
use tokenizer::TokenError;
use types::TypeError;

// TODO: Implement the compiler (llvm?)
// TODO: Implement typechecking
// TODO: Implement a language server

#[derive(Clone, PartialEq)]
pub struct FilePos {
    file: PathBuf,
    row: usize,
    col: usize,
}

impl fmt::Display for FilePos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file.display(), self.row, self.col)
    }
}

impl fmt::Debug for FilePos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file.display(), self.row, self.col)
    }
}

impl FilePos {
    pub fn new(file: &PathBuf, row: usize, col: usize) -> Self {
        Self {
            file: file.clone(),
            row,
            col,
        }
    }

    pub fn internal() -> Self {
        Self {
            file: PathBuf::from("internal"),
            row: 0,
            col: 0,
        }
    }

    pub fn temp() -> Self {
        Self {
            file: PathBuf::from("undefined"),
            row: 0,
            col: 0,
        }
    }
}

impl From<&Vec<Token>> for FilePos {
    fn from(tokens: &Vec<Token>) -> Self {
        tokens.get(0).unwrap().pos.clone()
    }
}

pub struct SandError {
    pos: FilePos,
    msg: String,
}

impl fmt::Display for SandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.pos, self.msg)
    }
}

impl fmt::Debug for SandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.pos, self.msg)
    }
}

impl From<TokenError> for SandError {
    fn from(token_error: TokenError) -> Self {
        Self {
            pos: token_error.pos,
            msg: format!("TOKEN_ERROR: {}", token_error.msg),
        }
    }
}

impl From<ParseError> for SandError {
    fn from(parse_error: ParseError) -> Self {
        Self {
            pos: parse_error.pos,
            msg: format!("PARSE_ERROR: {}", parse_error.msg),
        }
    }
}

impl From<InterpretingError> for SandError {
    fn from(parse_error: InterpretingError) -> Self {
        Self {
            pos: parse_error.pos,
            msg: format!("INTERPRETING_ERROR: {}", parse_error.msg),
        }
    }
}

impl From<TypeError> for SandError {
    fn from(parse_error: TypeError) -> Self {
        Self {
            pos: parse_error.pos,
            msg: format!("TYPE_ERROR: {}", parse_error.msg),
        }
    }
}

#[derive(StructOpt)]
enum Cmd {
    Tokenize,
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

    let file_contents = fs::read_to_string(&opt.file).unwrap();

    match opt.subcommand {
        Cmd::Tokenize => {
            println!("==== File:\n{}", file_contents);
            match tokenize_str(&file_contents, &opt.file, 1, 1) {
                Ok(tokens) => {
                    println!("==== Tokens:\n{:#?}", tokens);
                }
                Err(err) => {
                    eprintln!("{}", SandError::from(err))
                }
            }
        }
        Cmd::Parse => {
            println!("==== File:\n{}", file_contents);
            match tokenize_str(&file_contents, &opt.file, 1, 1) {
                Ok(tokens) => match parse_tokens(tokens) {
                    Ok(tree) => {
                        println!("==== Tree:\n{:#?}", tree);
                    }
                    Err(err) => {
                        eprintln!("{}", SandError::from(err))
                    }
                },
                Err(err) => {
                    eprintln!("{}", SandError::from(err))
                }
            }
        }
        Cmd::Run => match tokenize_str(&file_contents, &opt.file, 1, 1) {
            Ok(tokens) => match parse_tokens(tokens) {
                Ok(tree) => match tree.interpret(&mut init_scope()) {
                    Ok(result) => println!("Program exited with {:?}", result),
                    Err(err) => eprintln!("{}", SandError::from(err)),
                },
                Err(err) => {
                    eprintln!("{}", SandError::from(err))
                }
            },
            Err(err) => {
                eprintln!("{}", SandError::from(err))
            }
        },
    }
}
