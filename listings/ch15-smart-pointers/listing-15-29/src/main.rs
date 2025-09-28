use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    valore: i32,
    padre: RefCell<Weak<Node>>,
    figlio: RefCell<Vec<Rc<Node>>>,
}

// ANCHOR: here
fn main() {
    let foglia = Rc::new(Node {
        valore: 3,
        padre: RefCell::new(Weak::new()),
        figlio: RefCell::new(vec![]),
    });

    println!(
        "foglia strong = {}, debole = {}",
        Rc::strong_count(&foglia),
        Rc::weak_count(&foglia),
    );

    {
        let ramo = Rc::new(Node {
            valore: 5,
            padre: RefCell::new(Weak::new()),
            figlio: RefCell::new(vec![Rc::clone(&foglia)]),
        });

        *foglia.padre.borrow_mut() = Rc::downgrade(&ramo);

        println!(
            "ramo strong = {}, weak = {}",
            Rc::strong_count(&ramo),
            Rc::weak_count(&ramo),
        );

        println!(
            "foglia strong = {}, weak = {}",
            Rc::strong_count(&foglia),
            Rc::weak_count(&foglia),
        );
    }

    println!("foglia padre = {:?}", foglia.padre.borrow().upgrade());
    println!(
        "foglia strong = {}, weak = {}",
        Rc::strong_count(&foglia),
        Rc::weak_count(&foglia),
    );
}
// ANCHOR_END: here
