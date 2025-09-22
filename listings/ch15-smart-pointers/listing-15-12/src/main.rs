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

fn ciao(nome: &str) {
    println!("Ciao, {nome}!");
}

// ANCHOR: here
fn main() {
    let m = MioBox::new(String::from("Rust"));
    ciao(&m);
}
// ANCHOR_END: here
