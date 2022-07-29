pub mod ast;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod span;
pub mod value;
pub mod vm;

use std::io::{stdin, BufRead, BufReader};

use interpreter::eval;
use parser::Parser;
use scanner::{Scanner, Token};
use span::Span;

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
    let lines = count_lines(input);
    for result in Parser::new(
        input,
        Scanner::new(input).filter(|token| token.value != Token::Comment),
    ) {
        match result.value {
            Ok(expression) => match eval(expression) {
                Ok(value) => println!("{:?}", value),
                Err(error) => {
                    println_span(input, &lines, result.span);
                    println!("{:?}", error);
                }
            },
            Err(error) => {
                println_span(input, &lines, result.span);
                println!("{:?}", error);
            }
        }
        // let lexeme = &input[token.start as usize..token.end as usize];
        // match token.tt {
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
        //         println!("'{}' :: {:?}", lexeme, token.tt,);
        //     }
        // }
    }
    Ok(())
}

fn println_span(input: &str, lines: &[i32], span: Span) {
    let low = lines.binary_search(&span.start).unwrap_or_else(|x| x) - 1;
    let high = lines.binary_search(&(span.end - 1)).unwrap_or_else(|x| x) - 1;
    let line_start = (lines[low] + 1) as usize;
    let line_end = (lines[high + 1]) as usize;
    println!(
        "lines {:?}, span {:?}, low: {}, high: {}, range: {:?}",
        lines,
        span,
        low,
        high,
        line_start..line_end
    );
    println!("{}", &input[line_start..line_end]);
    for _ in line_start..span.start as usize {
        print!(" ");
    }
    for _ in span.start..span.end {
        print!("^");
    }
    for _ in span.end as usize..line_end {
        print!(" ");
    }
    println!();
}

pub fn count_lines(input: &str) -> Vec<i32> {
    let mut lines = vec![-1];
    for (index, ch) in input.char_indices() {
        if ch == '\n' {
            lines.push(index as i32);
        }
    }
    lines
}
