fn main() {
    // ANCHOR: here
    let x = 5;

    match x {
        1..=5 => println!("da uno a cinque"),
        _ => println!("altro"),
    }
    // ANCHOR_END: here
}
