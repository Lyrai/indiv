use crate::TokenStream;
use crate::lexer::Token;
use crate::parser::Node::UnOp;

pub struct Parser {
    stream: TokenStream
}

#[derive(Debug)]
pub enum Node {
    Number(u32),
    BinOp {
        token: Token,
        left: Box<Node>,
        right: Box<Node>
    },
    UnOp {
        token: Token,
        child: Box<Node>
    },
    Mod {
        modulus: u32,
        expr: Box<Node>
    },
    None
}

/*
expr => factor ((+|-) factor)* mod num
factor => term (* term)*
term => (inverse|-)? term | primary
primary => num | (expr)
 */

impl Parser {
    pub fn new(lexer: TokenStream) -> Self {
        Parser {
            stream: lexer
        }
    }

    pub fn parse(&mut self) -> Node {
        let node = self.expression();
        if let Some(Token::Mod) = self.match_token() {
            self.consume_token();
            if let Some(Token::Number(num)) = self.match_token() {
                self.consume_token();
                Node::Mod {
                    modulus: num,
                    expr: Box::new(node)
                }
            } else {
                panic!("Unexpected token");
            }
        } else {
            panic!("Mod required");
        }
    }

    fn expression(&mut self) -> Node {
        let left = self.factor();
        let token = match self.match_token() {
            Some(Token::Plus | Token::Minus) => self.consume_token().unwrap(),
            _ => return left
        };

        let right = self.factor();
        Node::BinOp {
            token,
            left: Box::new(left),
            right: Box::new(right)
        }
    }

    fn factor(&mut self) -> Node {
        let left = self.term();
        let token = match self.match_token() {
            Some(Token::Multiply) => self.consume_token().unwrap(),
            _ => return left
        };
        let right = self.term();
        Node::BinOp {
            token,
            left: Box::new(left),
            right: Box::new(right)
        }
    }

    fn term(&mut self) -> Node {
        match self.match_token() {
            None => panic!("Unexpected end of file"),
            Some(Token::Inverse | Token::Minus) => UnOp {
                token: self.consume_token().unwrap(),
                child: Box::new(self.term()),
            },
            Some(Token::Number(_)) => self.primary(),
            Some(Token::Lpar) => {
                self.consume_token();
                let node = self.expression();
                if let Some(Token::Rpar) = self.match_token() {
                    self.consume_token();
                } else {
                    panic!("Unexpected token");
                }

                node
            }
            Some(t) => panic!("Unexpected token {:?}", t)
        }
    }

    fn primary(&mut self) -> Node {
        match self.match_token() {
            None => panic!("Unexpected end of file"),
            Some(Token::Number(num)) => {
                self.consume_token();
                Node::Number(num)
            }
            //Some(Token::Lpar) => self.expression(),
            Some(t) => panic!("Unexpected token {:?}", t)
        }
    }

    fn match_token(&mut self) -> Option<Token> {
        self.stream.peek()
    }

    fn consume_token(&mut self) -> Option<Token> {
        self.stream.next()
    }
}