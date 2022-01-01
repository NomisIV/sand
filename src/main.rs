use anyhow::Result;
use std::env;
use std::fmt::Write;
use std::fs;

mod types;

use types::*;

trait Parseable {
    fn parse(string: &str) -> Option<Result<Self>>
    where
        Self: Sized;
}

trait Interpretable {
    fn interpret(&self) -> Result<()>;
}

trait Compileable {
    fn compile(&self, buffer: impl Write) -> Result<()>;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(args.get(1).unwrap()).unwrap();
    println!("==== File:\n{}", file);
    let parse_result = Block::parse(&file.trim());
    if let Some(Ok(tokens)) = parse_result {
        println!("==== Tokens:\n{:#?}", tokens);
    } else if let Some(Err(err)) = parse_result {
        eprintln!("ERROR: {}", err.to_string());
    } else {
        eprintln!("ERROR: File is not a block");
    }
}
