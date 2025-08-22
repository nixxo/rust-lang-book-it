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
    match moneta {
        Moneta::Quarter(stato) => println!("Quarter statale del {stato:?}!"),
        _ => conteggio += 1,
    }
    // ANCHOR_END: here
}
