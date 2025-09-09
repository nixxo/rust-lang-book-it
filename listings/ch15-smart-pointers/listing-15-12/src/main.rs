use std::ops::Deref;

impl<T> Deref for MioBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct MioBox<T>(T);

impl<T> MioBox<T> {
    fn new(x: T) -> MioBox<T> {
        MioBox(x)
    }
}

fn hello(nome: &str) {
    println!("Hello, {nome}!");
}

// ANCHOR: here
fn main() {
    let m = MioBox::new(String::from("Rust"));
    hello(&m);
}
// ANCHOR_END: here
