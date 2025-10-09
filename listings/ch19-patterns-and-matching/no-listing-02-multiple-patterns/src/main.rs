fn main() {
    // ANCHOR: here
    let x = 1;

    match x {
        1 | 2 => println!("uno o due"),
        3 => println!("tre"),
        _ => println!("altro"),
    }
    // ANCHOR_END: here
}
