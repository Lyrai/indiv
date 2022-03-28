use crate::interpreter::Interpreter;
use crate::lexer::TokenStream;
use crate::parser::Parser;

mod lexer;
mod parser;
mod interpreter;
mod except;

#[macro_use]
mod macro_decl;

fn interpret(line: String) {
    let tokens = TokenStream::new(line);
    let mut parser = Parser::new(tokens);
    let node = parser.parse();
    let result = Interpreter::interpret(node);
    println!("Result is {}", result);
}

fn main() {
    loop {
        let stdin = std::io::stdin();
        let mut str = String::new();
        let _ = stdin.read_line(&mut str);
        let str = str.trim_end().to_owned();
        if str.eq_ignore_ascii_case("quit") {
            break;
        }
        interpret(str);
    }
}
