enum Lista {
    Cons(i32, Lista),
    Nil,
}

// ANCHOR: here
// --taglio--

use crate::Lista::{Cons, Nil};

fn main() {
    let lista = Cons(1, Cons(2, Cons(3, Nil)));
}
// ANCHOR_END: here
