mod lexer;
mod location;
mod parser;
mod token;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let filepath = "examples/hello.c";

    let lexer = Lexer::from_file(filepath).expect("Failed to create lexer");
    println!("{:#?}", String::from_utf8(lexer.source.to_vec()));
    println!("---------------");

    let mut parser = Parser::new(lexer);
    parser.parse_program();

    // while let Some(token) = lexer.next_token().expect("Failed to get next token") {
    //     println!("{:#?}", token);
    // }
}
