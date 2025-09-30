use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Nodo {
    valore: i32,
    genitore: RefCell<Weak<Nodo>>,
    figli: RefCell<Vec<Rc<Nodo>>>,
}

// ANCHOR: here
fn main() {
    let foglia = Rc::new(Nodo {
        valore: 3,
        genitore: RefCell::new(Weak::new()),
        figli: RefCell::new(vec![]),
    });

    println!(
        "foglia forte = {}, debole = {}",
        Rc::strong_count(&foglia),
        Rc::weak_count(&foglia),
    );

    {
        let ramo = Rc::new(Nodo {
            valore: 5,
            genitore: RefCell::new(Weak::new()),
            figli: RefCell::new(vec![Rc::clone(&foglia)]),
        });

        *foglia.genitore.borrow_mut() = Rc::downgrade(&ramo);

        println!(
            "ramo forte = {}, debole = {}",
            Rc::strong_count(&ramo),
            Rc::weak_count(&ramo),
        );

        println!(
            "foglia forte = {}, debole = {}",
            Rc::strong_count(&foglia),
            Rc::weak_count(&foglia),
        );
    }

    println!("genitore `foglia` = {:?}", foglia.genitore.borrow().upgrade());
    println!(
        "foglia forte = {}, debole = {}",
        Rc::strong_count(&foglia),
        Rc::weak_count(&foglia),
    );
}
// ANCHOR_END: here
