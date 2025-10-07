use std::fmt;

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

// ANCHOR: here
struct Punto {
    x: i32,
    y: i32,
}

impl StampaContorno for Punto {}
// ANCHOR_END: here

fn main() {
    let p = Punto { x: 1, y: 3 };
    p.stampa_contorno();
}
