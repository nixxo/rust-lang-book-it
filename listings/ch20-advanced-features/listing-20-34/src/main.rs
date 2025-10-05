fn main() {
    let handlers = vec![ritorna_chiusura(), ritorna_chiusura_inizializzata(123)];
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
}

// ANCHOR: here
fn ritorna_chiusura() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn ritorna_chiusura_inizializzata(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}
// ANCHOR_END: here
