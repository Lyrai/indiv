use crate::lexer::Token;
use crate::parser::Node;
use crate::except::{Except, Unite};
use std::rc::Rc;
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;

pub struct Interpreter;

/*
fv
w = w
wa = fv(w) u fv(a)
^x.t = fv(t) \ x

bv
w = 0
wa = bv(w) u bv(a)
^x.t = x u bv(t)
*/


impl Interpreter {
    pub fn interpret(root: Rc<Mutex<Node>>) -> String {
        let mut redex = Self::find_redex(root.clone());
        while let Some(node) = redex {
            Self::normalize(node.clone());
            redex = Self::find_redex(root.clone());
        }

        root.lock().unwrap().deref().to_string()
    }

    pub fn free_vars(root: &Rc<Mutex<Node>>) -> Vec<char> {
        match root.lock().unwrap().deref() {
            Node::Application {left, right} => Self::free_vars(left).unite(&Self::free_vars(right)),
            Node::Lambda {var, term} => Self::free_vars(term).except(&Self::free_vars(var)),
            Node::Var(c) => vec![*c]
        }
    }

    pub fn bound_vars(root: &Rc<Mutex<Node>>) -> Vec<char> {
        match root.lock().unwrap().deref() {
            Node::Application {left, right} => Self::bound_vars(left).unite(&Self::bound_vars(right)),
            Node::Lambda {var, term} => Self::free_vars(var).unite(&Self::bound_vars(term)),
            Node::Var(_) => vec![]
        }
    }

    fn interpret_internal(node: &Node) -> Node {
        let result = match node {
            _ => panic!("Unexpected node")
        };

        result
    }

    fn find_redex(root: Rc<Mutex<Node>>) -> Option<Rc<Mutex<Node>>> {
        match root.clone().lock().unwrap().deref() {
            Node::Application {left, right} => {
                if let &Node::Lambda {..} = left.lock().unwrap().deref() {
                    return Some(root);
                }

                if let r @ Some(_) = Self::find_redex(left.clone()) {
                    r
                } else {
                    Self::find_redex(right.clone())
                }
            }
            Node::Lambda { var: _, term} => Self::find_redex(term.clone()),
            Node::Var(_) => None
        }
    }

    fn normalize(node: Rc<Mutex<Node>>) {
        let mut term;
        let mut subs;
        let mut var;
        if let Node::Application {left, right} = node.lock().unwrap().deref() {
            if let Node::Lambda {var: v, term: t} = left.lock().unwrap().deref() {
                term = t.clone();
                if let Node::Var(v) = v.lock().unwrap().deref() {
                    var = *v;
                } else {
                    unreachable!();
                }
            } else {
                unreachable!();
            }

            subs = right.clone();
        } else {
            unreachable!();
        }
        let subs = Self::substitute(term, subs, var, 0);
        let mut n = node.lock().unwrap();
        let n = n.deref_mut();
        *n = subs;
    }

    fn substitute(term: Rc<Mutex<Node>>, subs: Rc<Mutex<Node>>, var: char, depth: u8) -> Node {
        for _ in 0..depth {
            print!(" ");
        }
        print!("[{}/{}]{} {{", subs.lock().unwrap().deref().to_string(), var, term.lock().unwrap().deref().to_string());
        let result = match term.lock().unwrap().deref() {
            //Rule 3
            Node::Application {left, right} => {
                print!(" Rule 3\n");
                let left = Self::substitute(left.clone(), subs.clone(), var, depth + 1);
                let right = Self::substitute(right.clone(), subs, var, depth + 1);
                Node::Application {
                    left: Rc::new(Mutex::new(left)),
                    right: Rc::new(Mutex::new(right))
                }

            }
            //Rules 1 & 2
            v @ Node::Var(c) => {
                if *c == var {
                    //Rule 1
                    print!(" Rule 1");
                    subs.lock().unwrap().deref().clone()
                } else {
                    //Rule 2
                    print!(" Rule 2");
                    v.clone()
                }
            }
            n @ Node::Lambda {var: v, term} => {
                let v = if let Node::Var(v) = v.lock().unwrap().deref() {
                    *v
                } else {
                    unreachable!()
                };
                if var == v {
                    //Rule 4
                    print!(" Rule 4");
                    n.clone()
                } else if !Self::free_vars(term).contains(&var) {
                    //Rule 5
                    print!("Rule 5");
                    n.clone()
                } else if !Self::free_vars(&subs).contains(&v) {
                    //Rule 6
                    print!(" Rule 6\n");
                    let new_term = Self::substitute(term.clone(), subs, var, depth + 1);
                    Node::Lambda {
                        var: Rc::new(Mutex::new(Node::make_var(v))),
                        term: Rc::new(Mutex::new(new_term))
                    }
                } else {
                    //Rule 7
                    let app = Node::Application {
                        left: subs.clone(),
                        right: term.clone()
                    };
                    let app = Rc::new(Mutex::new(app));
                    let free_vars = Self::free_vars(&app);
                    let mut subs_var = None;
                    for c in 'a'..'z' {
                        if !free_vars.contains(&c) {
                            subs_var = Some(c);
                            break;
                        }
                    }
                    if subs_var.is_none() {
                        panic!("Cannot find z for rule 7")
                    }
                    let subs_var = Node::make_var(subs_var.unwrap());
                    let subs_var = Rc::new(Mutex::new(subs_var));
                    print!(" Rule 7\n");
                    let term_subs = Self::substitute(term.clone(), subs_var.clone(), v, depth + 1);
                    let term_subs = Self::substitute(Rc::new(Mutex::new(term_subs)), subs, var, depth + 1);

                    Node::Lambda {
                        var: subs_var,
                        term: Rc::new(Mutex::new(term_subs))
                    }
                }
            }
        };

        for _ in 1..(depth + 1) {
            print!(" ");
        }
        println!("}} => {};", result.to_string());
        result
    }
}