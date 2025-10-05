trait Animale {
    fn nomignolo() -> String;
}

struct Cane;

impl Cane {
    fn nomignolo() -> String {
        String::from("Rex")
    }
}

impl Animale for Cane {
    fn nomignolo() -> String {
        String::from("cucciolo")
    }
}

// ANCHOR: here
fn main() {
    println!("Un piccolo di cane è detto {}", <Cane as Animale>::nomignolo());
}
// ANCHOR_END: here
