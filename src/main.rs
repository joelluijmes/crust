use std::{env, process::exit};

mod codegen;
mod lexer;
mod location;
mod parser;
mod token;

use codegen::generate_program;
use lexer::Lexer;
use parser::Parser;

use crate::parser::ParserError;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Missing input source file");
        exit(1);
    }

    let filepath = &args[1];
    let lexer = Lexer::from_file(filepath).expect("Failed to create lexer");

    let mut parser = Parser::new(lexer);
    match parser.parse_program() {
        Ok(program) => {
            let output = generate_program(program).expect("Failed to generate program");
            print!("{}", output);
        }

        Err(err) => {
            display_parser_error(&err, parser.lexer());
        }
    }
}

fn display_parser_error(err: &ParserError, lexer: &mut Lexer) {
    eprintln!(
        "Error occurred during parsing program\n  kind: {:?}\n  at: {:?}\n",
        err.kind, err.loc
    );

    // Seek to start of the line
    let idx = err.loc.idx - err.loc.col;
    lexer.seek(idx);

    let line = String::from_utf8(lexer.read_line()).expect("Invalid ASCII character");
    eprintln!("  {line}\n  {:>col$}", '^', col = err.loc.col + 1);
}
