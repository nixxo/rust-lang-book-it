enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

// ANCHOR: here
// --taglio--

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("conteggio dopo la creazione di a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("conteggio dopo la creazione di b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("conteggio dopo la creazione di c = {}", Rc::strong_count(&a));
    }
    println!("conteggio dopo che c è uscita dallo scope c = {}", Rc::strong_count(&a));
}
// ANCHOR_END: here
