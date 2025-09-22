enum Lista {
    Cons(i32, Box<Lista>),
    Nil,
}

use crate::Lista::{Cons, Nil};

fn main() {
    let lista = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
