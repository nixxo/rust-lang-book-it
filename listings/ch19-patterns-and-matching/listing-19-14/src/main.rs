struct Punto {
    x: i32,
    y: i32,
}

// ANCHOR: here
fn main() {
    let p = Punto { x: 0, y: 7 };

    match p {
        Punto { x, y: 0 } => println!("Sull'asse x a {x}"),
        Punto { x: 0, y } => println!("Sull'asse y a {y}"),
        Punto { x, y } => println!("Su nessun asse: ({x}, {y})"),
    }
}
// ANCHOR_END: here
