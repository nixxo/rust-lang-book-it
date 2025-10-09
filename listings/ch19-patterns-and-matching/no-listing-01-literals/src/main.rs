fn main() {
    // ANCHOR: here
    let x = 1;

    match x {
        1 => println!("uno"),
        2 => println!("due"),
        3 => println!("tre"),
        _ => println!("altro"),
    }
    // ANCHOR_END: here
}
