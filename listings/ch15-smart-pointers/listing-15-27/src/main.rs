// ANCHOR: here
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    valore: i32,
    figlio: RefCell<Vec<Rc<Node>>>,
}
// ANCHOR_END: here

// ANCHOR: there
fn main() {
    let foglia = Rc::new(Node {
        valore: 3,
        figlio: RefCell::new(vec![]),
    });

    let ramo = Rc::new(Node {
        valore: 5,
        figlio: RefCell::new(vec![Rc::clone(&foglia)]),
    });
}
// ANCHOR_END: there
