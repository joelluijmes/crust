mod lexer;
mod location;
mod token;

use lexer::Lexer;

fn main() {
    let filepath = "examples/hello.c";

    let mut lexer = Lexer::from_file(filepath).expect("Failed to create lexer");
    println!("{:#?}", String::from_utf8(lexer.source.to_vec()));
    println!("---------------");

    while let Some(token) = lexer.next_token().expect("Failed to get next token") {
        println!("{:#?}", token);
    }
}
