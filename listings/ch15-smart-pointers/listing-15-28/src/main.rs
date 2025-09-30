// ANCHOR: here
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Nodo {
    valore: i32,
    genitore: RefCell<Weak<Nodo>>,
    figli: RefCell<Vec<Rc<Nodo>>>,
}
// ANCHOR_END: here

// ANCHOR: there
fn main() {
    let foglia = Rc::new(Nodo {
        valore: 3,
        genitore: RefCell::new(Weak::new()),
        figli: RefCell::new(vec![]),
    });

    println!("genitore `foglia` = {:?}", foglia.genitore.borrow().upgrade());

    let ramo = Rc::new(Nodo {
        valore: 5,
        genitore: RefCell::new(Weak::new()),
        figli: RefCell::new(vec![Rc::clone(&foglia)]),
    });

    *foglia.genitore.borrow_mut() = Rc::downgrade(&ramo);

    println!("genitore `foglia` = {:?}", foglia.genitore.borrow().upgrade());
}
// ANCHOR_END: there
