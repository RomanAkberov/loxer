pub mod ast;
pub mod parser;
pub mod scanner;

use parser::Parser;
use scanner::{Scanner, TokenType};
use std::io::{stdin, BufRead, BufReader};

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> DynResult<()> {
    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => Err("Usage loxer [script]".into()),
    }
}

fn run_prompt() -> DynResult<()> {
    let mut stdin = BufReader::new(stdin().lock());
    let mut source = String::new();
    loop {
        println!(">");
        source.clear();
        stdin.read_line(&mut source)?;
        run(&source)?;
    }
}

fn run_file(path: &str) -> DynResult<()> {
    let source = std::fs::read_to_string(path)?;
    run(&source)
}

fn run(input: &str) -> DynResult<()> {
    for expression in Parser::new(
        input,
        Scanner::new(input).filter(|token| token.ty != TokenType::Comment),
    ) {
        println!("{:?}", expression);
        // let lexeme = &input[token.start as usize..token.end as usize];
        // match token.ty {
        //     TokenType::Unknown => {
        //         eprintln!("Unknown token {} at line {}.", lexeme, token.line);
        //     }
        //     TokenType::String if !lexeme.ends_with('"') => {
        //         eprintln!(
        //             "Unterminated string {} at line {}.",
        //             lexeme.replace('\n', "\\n"),
        //             token.line
        //         );
        //     }
        //     _ => {
        //         println!("'{}' :: {:?}", lexeme, token.ty,);
        //     }
        // }
    }
    Ok(())
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}
