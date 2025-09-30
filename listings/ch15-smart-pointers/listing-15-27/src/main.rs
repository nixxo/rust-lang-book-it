// ANCHOR: here
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Nodo {
    valore: i32,
    figli: RefCell<Vec<Rc<Nodo>>>,
}
// ANCHOR_END: here

// ANCHOR: there
fn main() {
    let foglia = Rc::new(Nodo {
        valore: 3,
        figli: RefCell::new(vec![]),
    });

    let ramo = Rc::new(Nodo {
        valore: 5,
        figli: RefCell::new(vec![Rc::clone(&foglia)]),
    });
}
// ANCHOR_END: there
