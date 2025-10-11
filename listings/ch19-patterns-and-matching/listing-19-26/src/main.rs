fn main() {
    // ANCHOR: here
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("Il numero {x} è pari"),
        Some(x) => println!("Il numero {x} è dispari"),
        None => (),
    }
    // ANCHOR_END: here
}
