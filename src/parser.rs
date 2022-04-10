use crate::TokenStream;
use crate::lexer::Token;
use std::rc::Rc;
use std::ops::Deref;
use std::cell::RefCell;

pub struct Parser {
    stream: TokenStream
}

#[derive(Debug, Clone)]
pub enum Node {
    Application {
        left: Rc<RefCell<Node>>,
        right: Rc<RefCell<Node>>
    },
    Lambda {
        var: Rc<RefCell<Node>>,
        term: Rc<RefCell<Node>>
    },
    Substitution {
        var: char,
        subs: Rc<RefCell<Node>>,
        term: Rc<RefCell<Node>>
    },
    Var(char)
}

impl ToString for Node {
    fn to_string(&self) -> String {
        match self {
            Node::Application { left, right } => {
                if let n @ Node::Application {..} = right.borrow().deref() {
                    format!("{}({})", left.borrow().to_string(), n.to_string())
                } else {
                    format!("{}{}", left.borrow().to_string(), right.borrow().to_string())
                }
            },
            Node::Lambda { var, term } => format!("(^{}.{})", var.borrow().to_string(), term.borrow().to_string()),
            Node::Var(c) => format!("{}", c),
            Node::Substitution {var, subs, term} => format!("([{}/{}]{})", subs.borrow().to_string(), var, term.borrow().to_string())
        }
    }
}

impl Node {
    pub fn make_application(lhs: Self, rhs: Self) -> Self {
        Node::Application {
            left: Rc::new(RefCell::new(lhs)),
            right: Rc::new(RefCell::new(rhs))
        }
    }

    pub fn make_lambda(var: Self, term: Self) -> Self {
        Node::Lambda {
            var: Rc::new(RefCell::new(var)),
            term: Rc::new(RefCell::new(term))
        }
    }

    pub fn make_var(var: char) -> Self {
        Node::Var(var)
    }
}

/*
term => application | (term)
application => lambda (atom)* | var (atom)*
atom => lambda | var
lambda => ^var.term
var => a..zA..Z
 */

impl Parser {
    pub fn new(lexer: TokenStream) -> Self {
        Parser {
            stream: lexer
        }
    }

    pub fn parse(&mut self) -> Rc<RefCell<Node>> {
        let node = self.application();
        if let Some(Token::Eof) = self.match_token() {
            panic!("Unexpected end of file")
        }
        Rc::new(RefCell::new(node))
    }

    fn term(&mut self) -> Node {
        match self.match_token() {
            None => panic!("Unexpected end of file"),
            Some(Token::Lambda | Token::Var(_)) => self.application(),
            Some(Token::Lpar) => {
                self.consume_token();
                let result = self.application();
                let token = self.consume_token();
                if let Some(Token::Rpar) = token {
                    result
                } else {
                    panic!("Unexpected token {:?}", token)
                }
            }
            _ => panic!("Unexpected token")
        }
    }

    fn application(&mut self) -> Node {
        let mut lhs = self.atom().unwrap();
        loop {
            if let Some(Token::Eof) = self.match_token() {
                break;
            }
            let rhs = self.atom();
            if rhs.is_none() {
                break;
            }

            lhs = Node::make_application(lhs, rhs.unwrap());
        }

        lhs
    }

    fn atom(&mut self) -> Option<Node> {
        match self.match_token() {
            Some(Token::Var(c)) => { self.consume_token(); Some(Node::Var(c)) }
            Some(Token::Lambda) => { self.consume_token(); Some(self.lambda()) }
            Some(Token::Lpar) => Some(self.term()),
            _ => None
        }
    }

    fn lambda(&mut self) -> Node {
        if let Some(Token::Var(c)) = self.consume_token() {
            if let Some(Token::Dot) = self.consume_token() {
                Node::make_lambda(Node::make_var(c), self.term())
            } else {
                panic!("No dot in lambda")
            }
        } else {
            panic!("No variable in lambda")
        }
    }

    fn match_token(&mut self) -> Option<Token> {
        self.stream.peek()
    }

    fn consume_token(&mut self) -> Option<Token> {
        self.stream.next()
    }
}