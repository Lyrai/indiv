use crate::parser::Node;
use crate::except::{Except, Unite};
use std::rc::Rc;
use std::ops::{Deref, DerefMut};
use std::cell::RefCell;

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
    pub fn interpret(root: Rc<RefCell<Node>>) -> String {
        let mut redex = Self::find_redex(root.clone());
        print!("{} => ", root.borrow().to_string());
        while let Some(node) = redex {
            Self::prepare(node.clone());
            while let Some(subs) = Self::find_substitution(node.clone()) {
                print!("{} => ", root.borrow().to_string());
                Self::substitute(subs);
            }
            print!("{} => ", root.borrow().to_string());
            redex = Self::find_redex(root.clone());
        }

        root.borrow().to_string()
    }

    pub fn free_vars(root: &Rc<RefCell<Node>>) -> Vec<char> {
        match root.borrow().deref() {
            Node::Application {left, right} => Self::free_vars(left).unite(&Self::free_vars(right)),
            Node::Lambda {var, term} => Self::free_vars(term).except(&Self::free_vars(var)),
            Node::Var(c) => vec![*c],
            _ => panic!("free_vars on subs")
        }
    }

    fn find_redex(root: Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        match root.clone().borrow().deref() {
            Node::Application {left, right} => {
                if let &Node::Lambda {..} = (left).borrow().deref() {
                    return Some(root);
                }

                if let r @ Some(_) = Self::find_redex(left.clone()) {
                    r
                } else {
                    Self::find_redex(right.clone())
                }
            }
            Node::Lambda { var: _, term} => Self::find_redex(term.clone()),
            Node::Var(_) => None,
            _ => panic!("find_redex on subs")
        }
    }

    fn prepare(node: Rc<RefCell<Node>>) {
        let term;
        let subs;
        let var;
        if let Node::Application {left, right} = node.borrow().deref() {
            if let Node::Lambda {var: v, term: t} = left.borrow().deref() {
                term = t.clone();
                if let Node::Var(v) = v.borrow().deref() {
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

        let mut n = node.borrow_mut();
        *n = Node::Substitution {
            var,
            subs,
            term
        };
    }

    fn substitute(node: Rc<RefCell<Node>>) {
        let s = if let Node::Substitution {var, subs, term} = node.borrow().deref() {
            match term.borrow().deref() {
                Node::Application {left, right} => {
                    //Rule 3
                    let left = Node::Substitution {
                        var: *var,
                        subs: subs.clone(),
                        term: left.clone()
                    };
                    let right = Node::Substitution {
                        var: *var,
                        subs: subs.clone(),
                        term: right.clone()
                    };
                    Node::make_application(left, right)
                }
                v @ Node::Var(c) => {
                    if c == var {
                        //Rule 1
                        (subs).borrow().clone()
                    } else {
                        //Rule 2
                        v.clone()
                    }
                }
                n @ Node::Lambda {var: v, term} => {
                    let v = if let Node::Var(v) = v.borrow().deref() {
                        *v
                    } else {
                        unreachable!()
                    };

                    if *var == v {
                        //Rule 4
                        n.clone()
                    } else if !Interpreter::free_vars(term).contains(&var) {
                        //Rule 5
                        n.clone()
                    } else if !Interpreter::free_vars(&subs).contains(&v) {
                        //Rule 6
                        let new_term = Node::Substitution {
                            var: *var,
                            subs: subs.clone(),
                            term: term.clone()
                        };
                        Node::Lambda {
                            var: Rc::new(RefCell::new(Node::make_var(v))),
                            term: Rc::new(RefCell::new(new_term))
                        }
                    } else {
                        //Rule 7
                        let app = Node::Application {
                            left: subs.clone(),
                            right: term.clone()
                        };
                        let app = Rc::new(RefCell::new(app));
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
                        let subs_var = Rc::new(RefCell::new(subs_var));
                        let term_subs = Node::Substitution {
                            var: v,
                            subs: subs_var.clone(),
                            term: term.clone()
                        };
                        let term_subs = Node::Substitution {
                            var: *var,
                            subs: subs.clone(),
                            term: Rc::new(RefCell::new(term_subs))
                        };

                        Node::Lambda {
                            var: subs_var,
                            term: Rc::new(RefCell::new(term_subs))
                        }
                    }
                }
                _ => unreachable!()
            }
        } else {
            unreachable!()
        };

        let mut n = node.borrow_mut();
        *n = s;
    }

    fn find_substitution(root: Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
        match root.borrow().deref() {
            Node::Substitution {..} => Some(root.clone()),
            Node::Var(_) => None,
            Node::Application {left, right} => {
                let left = Self::find_substitution(left.clone());
                if let Some(n) = left {
                    Some(n)
                } else {
                    Self::find_substitution(right.clone())
                }
            }
            Node::Lambda {var: _, term} => {
                Self::find_substitution(term.clone())
            }
        }
    }
}