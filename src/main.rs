mod codegen;
mod lexer;
mod location;
mod parser;
mod token;

use codegen::generate_program;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let filepath = "examples/hello.c";

    let lexer = Lexer::from_file(filepath).expect("Failed to create lexer");

    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().expect("Failed to parse program");

    let output = generate_program(program).expect("Failed to generate program");
    print!("{}", output);

    // while let Some(token) = lexer.next_token().expect("Failed to get next token") {
    // }
}
