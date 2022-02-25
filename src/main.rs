use crate::interpreter::Interpreter;
use crate::lexer::TokenStream;
use crate::parser::Parser;

mod lexer;
mod parser;
mod interpreter;

fn interpret(line: String) {
    let tokens = TokenStream::new(line);
    let mut parser = Parser::new(tokens);
    let node = parser.parse();
    let result = Interpreter::interpret(&node);
    println!("{}", result);
}

fn main() {
    interpret("-10 - 21 mod 55".to_owned());
}
