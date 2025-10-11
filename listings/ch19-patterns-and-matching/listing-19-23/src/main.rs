fn main() {
    // ANCHOR: here
    struct Punto {
        x: i32,
        y: i32,
        z: i32,
    }

    let origine = Punto { x: 0, y: 0, z: 0 };

    match origine {
        Punto { x, .. } => println!("x è {x}"),
    }
    // ANCHOR_END: here
}
