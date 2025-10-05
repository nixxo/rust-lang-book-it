use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
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

impl OutlinePrint for Punto {}
// ANCHOR_END: here

fn main() {
    let p = Punto { x: 1, y: 3 };
    p.outline_print();
}
