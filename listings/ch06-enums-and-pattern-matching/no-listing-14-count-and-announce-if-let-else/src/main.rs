#[derive(Debug)]
enum StatoUSA {
    Alabama,
    Alaska,
    // --snip--
}

enum Moneta {
    Penny,
    Nickel,
    Dime,
    Quarter(StatoUSA),
}

fn main() {
    let moneta = Moneta::Penny;
    // ANCHOR: here
    let mut conteggio = 0;
    if let Moneta::Quarter(stato) = moneta {
        println!("Quarter statale del {stato:?}!");
    } else {
        conteggio += 1;
    }
    // ANCHOR_END: here
}
