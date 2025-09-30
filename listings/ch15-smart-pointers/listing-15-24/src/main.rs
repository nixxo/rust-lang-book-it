#[derive(Debug)]
enum Lista {
    Cons(Rc<RefCell<i32>>, Rc<Lista>),
    Nil,
}

use crate::Lista::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let valore = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&valore), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *valore.borrow_mut() += 10;

    println!("a dopo = {a:?}");
    println!("b dopo = {b:?}");
    println!("c dopo = {c:?}");
}
