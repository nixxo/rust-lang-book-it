fn main() {
    // ANCHOR: here
    let x = 'c';

    match x {
        'a'..='j' => println!("lettere ASCII iniziali"),
        'k'..='z' => println!("lettere ASCII finali"),
        _ => println!("altro"),
    }
    // ANCHOR_END: here
}
