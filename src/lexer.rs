pub struct TokenStream {
    position: usize,
    line: Vec<char>
}
#[derive(Debug)]
pub enum Token {
    Number(u32),
    Plus,
    Minus,
    Multiply,
    Inverse,
    Lpar,
    Rpar,
    Mod
}

impl TokenStream {
    pub fn new(line: String) -> Self {
        TokenStream {
            position: 0,
            line: line
                .chars()
                .filter(|&x| !x.is_whitespace())
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

        if symbol.is_digit(10) {
            let mut num = 0;
            let mut count = 0;
            while let Some(n) = symbol.to_digit(10) {
                num = num * 10 + n;
                self.position += 1;
                count += 1;
                if self.position == self.line.len() {
                    break;
                }

                symbol = self.line[self.position];
            }

            self.position -= count;
            return (Some(Token::Number(num)), count)
        };

        match symbol {
            '+' => (Some(Token::Plus), 1),
            '-' => (Some(Token::Minus), 1),
            '*' => (Some(Token::Multiply), 1),
            '(' => (Some(Token::Lpar), 1),
            ')' => (Some(Token::Rpar), 1),
            'i' => {
                let rest = self.line[self.position..self.position + 7].to_vec();
                let check = "inverse".chars().collect::<Vec<char>>();
                if check == rest {
                    (Some(Token::Inverse), 7)
                } else {
                    (None, 0)
                }
            }
            'm' => {
                let rest = self.line[self.position..self.position + 3].to_vec();
                let check = "mod".chars();

                if check.eq(rest) {
                    (Some(Token::Mod), 3)
                } else {
                    (None, 0)
                }
            }
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