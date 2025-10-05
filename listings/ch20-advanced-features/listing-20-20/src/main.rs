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

fn main() {
    println!("Un piccolo di cane è detto {}", Cane::nomignolo());
}
