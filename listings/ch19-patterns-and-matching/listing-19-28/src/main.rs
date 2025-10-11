fn main() {
    // ANCHOR: here
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("si"),
        _ => println!("no"),
    }
    // ANCHOR_END: here
}
