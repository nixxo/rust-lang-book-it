// ANCHOR: here
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    valore: i32,
    padre: RefCell<Weak<Node>>,
    figlio: RefCell<Vec<Rc<Node>>>,
}
// ANCHOR_END: here

// ANCHOR: there
fn main() {
    let foglia = Rc::new(Node {
        valore: 3,
        padre: RefCell::new(Weak::new()),
        figlio: RefCell::new(vec![]),
    });

    println!("foglia padre = {:?}", foglia.padre.borrow().upgrade());

    let ramo = Rc::new(Node {
        valore: 5,
        padre: RefCell::new(Weak::new()),
        figlio: RefCell::new(vec![Rc::clone(&foglia)]),
    });

    *foglia.padre.borrow_mut() = Rc::downgrade(&ramo);

    println!("foglia padre = {:?}", foglia.padre.borrow().upgrade());
}
// ANCHOR_END: there
