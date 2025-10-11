struct Point {
    x: i32,
    y: i32,
}

// ANCHOR: here
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("Sull'asse x a {x}"),
        Point { x: 0, y } => println!("Sull'asse y a {y}"),
        Point { x, y } => {
            println!("Su nessun asse: ({x}, {y})");
        }
    }
}
// ANCHOR_END: here
