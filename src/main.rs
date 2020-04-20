mod lexer;
mod parser;
mod types;
mod engines;

use std::fs::read_to_string;

fn main() -> Result<(), std::io::Error> {
    let source = read_to_string("./simple_test.f")?;
    println!("{}", source);
    println!("======");
    let tokens = lexer::Lexer::lex(source);
    let parser = parser::Parser::new(tokens);
    let code = parser.parse();
    let mut interpreter = engines::interpreter::Interpreter::new();
    for node in code {
        interpreter.apply(node);
    }
    Ok(())
}
