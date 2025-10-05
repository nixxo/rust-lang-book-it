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

// ANCHOR: here
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a conteggio rc iniziale = {}", Rc::strong_count(&a));
    println!("a prossimo elemento = {:?}", a.coda());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a conteggio rc dopo creazione b = {}", Rc::strong_count(&a));
    println!("b conteggio rc iniziale = {}", Rc::strong_count(&b));
    println!("b prossimo elemento = {:?}", b.coda());

    if let Some(link) = a.coda() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b conteggio rc dopo modifica a = {}", Rc::strong_count(&b));
    println!("a conteggio rc dopo modifica a = {}", Rc::strong_count(&a));

    // Togli il commento alla prossima riga per vedere che
    // abbiamo un ciclo; causerà un overflow dello stack.
    // println!("a prossimo elemento = {:?}", a.coda());
}
// ANCHOR_END: here
