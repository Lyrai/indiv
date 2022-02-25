use std::ops::{Add, Mul, Neg};
use crate::lexer::Token;
use crate::parser::Node;

pub struct Interpreter;

struct Residue(i32, i32);

impl Residue {
    fn to_equiv(self) -> Self {
        if self.0 < 0 {
            Residue(self.0 + self.1, self.1)
        } else {
            self
        }
    }
}

impl Add<Residue> for Residue {
    type Output = Self;

    fn add(self, rhs: Residue) -> Self::Output {
        if self.1 != rhs.1 {
            panic!("Adding different classes")
        }

        Residue((self.0 + rhs.0) % self.1, self.1).to_equiv()
    }
}

impl Neg for Residue {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Residue(-self.0, self.1)
    }
}

impl Mul<Residue> for Residue {
    type Output = Self;

    fn mul(self, rhs: Residue) -> Self::Output {
        if self.1 != rhs.1 {
            panic!("Multiplying different classes")
        }

        Residue((self.0 * rhs.0) % self.1, self.1).to_equiv()
    }
}

impl Interpreter {
    pub fn interpret(root: &Node) -> i32 {
        Self::interpret_internal(root, 0).to_equiv().0
    }

    fn interpret_internal(node: &Node, modulus: i32) -> Residue {
        let result = match node {
            Node::Number(num) => Residue(*num as i32, modulus),
            Node::BinOp { token, left, right } => {
                match token {
                    Token::Plus => Self::interpret_internal(left, modulus) + Self::interpret_internal(right, modulus),
                    Token::Minus => Self::interpret_internal(left, modulus) + -Self::interpret_internal(right, modulus),
                    Token::Multiply => Self::interpret_internal(left, modulus) * Self::interpret_internal(right, modulus),
                    _ => panic!("Invalid token in node")
                }
            }
            Node::UnOp { token, child } => {
                match token {
                    Token::Minus => -Self::interpret_internal(child, modulus),
                    Token::Inverse => inverse(Self::interpret_internal(child, modulus).0, modulus),
                    _ => panic!("Invalid token in node")
                }
            },
            Node::Mod { modulus, expr}  => Self::interpret_internal(expr, *modulus as i32),
            _ => panic!("Unexpected node")
        };

        result
    }
}

fn inverse(num: i32, modulus: i32) -> Residue {
    let (d, x, _) = euclid(num, modulus);
    if d != 1 {
        panic!("Inverse does not exist");
    }
    Residue(x, modulus)
}

fn euclid(num: i32, modulus: i32) -> (i32, i32, i32) {
    if num == 0 {
        return (modulus, 0, 1);
    }

    let (d, x1, y1) = euclid(modulus % num, num);
    let x = y1 - (modulus / num) * x1;
    let y = x1;
    (d, x, y)
}