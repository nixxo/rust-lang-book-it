use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// ANCHOR: here
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a conteggio rc iniziale = {}", Rc::strong_count(&a));
    println!("a prossimo item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a conteggio rc dopo creazione b = {}", Rc::strong_count(&a));
    println!("b conteggio rc iniziale = {}", Rc::strong_count(&b));
    println!("b prossimo item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b conteggio rc dopo modifica a = {}", Rc::strong_count(&b));
    println!("a conteggio rc dopo modifica a = {}", Rc::strong_count(&a));

    // Togli il commento alla prossima riga per vedere che abbiamo un ciclo;
    // causerà un overflow dello stack.
    // println!("a prossimo item = {:?}", a.tail());
}
// ANCHOR_END: here
