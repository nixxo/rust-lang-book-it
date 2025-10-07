trait StampaContorno: fmt::Display {
    fn stampa_contorno(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Punto {
    x: i32,
    y: i32,
}

impl StampaContorno for Punto {}

// ANCHOR: here
use std::fmt;

impl fmt::Display for Punto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
// ANCHOR_END: here

fn main() {
    let p = Punto { x: 1, y: 3 };
    p.stampa_contorno();
}
