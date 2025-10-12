// ANCHOR: here
use crate::Lista::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum Lista {
    Cons(i32, RefCell<Rc<Lista>>),
    Nil,
}

impl Lista {
    fn coda(&self) -> Option<&RefCell<Rc<Lista>>> {
        match self {
            Cons(_, elemento) => Some(elemento),
            Nil => None,
        }
    }
}
// ANCHOR_END: here

fn main() {}
