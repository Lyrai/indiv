use std::iter::FromIterator;

pub struct TokenStream {
    position: usize,
    line: Vec<char>
}
#[derive(Debug)]
pub enum Token {
    Lpar,
    Rpar,
    Var(char),
    Lambda,
    Dot,
    Eof
}

impl TokenStream {
    pub fn new(line: String) -> Self {
        TokenStream {
            position: 0,
            line: line
                .chars()
                .collect()
        }
    }
}

impl TokenStream {
    pub fn peek(&mut self) -> Option<Token> {
        self.peek_internal().0
    }

    fn peek_internal(&mut self) -> (Option<Token>, usize) {
        if self.position == self.line.len() {
            return (None, 0);
        }

        let mut symbol = self.line[self.position];

        while symbol.is_whitespace() {
            if self.position + 1 == self.line.len() {
                return (None, 0);
            }
            self.position += 1;
            symbol = self.line[self.position];
        }

        match symbol {
            '(' => (Some(Token::Lpar), 1),
            ')' => (Some(Token::Rpar), 1),
            '^' => (Some(Token::Lambda), 1),
            t if t.is_alphabetic() => {
                (Some(Token::Var(t)), 1)
            }
            '.' => (Some(Token::Dot), 1),
            _ if self.position == self.line.len() => (Some(Token::Eof), 0),
            _ => (None, 0)
        }
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let (result, count) = self.peek_internal();
        self.position += count;
        result
    }
}