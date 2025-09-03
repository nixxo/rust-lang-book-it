#[derive(Debug)]
enum StatoUSA {
    Alabama,
    Alaska,
    // --taglio--
}

enum Moneta {
    Penny,
    Nickel,
    Dime,
    Quarter(StatoUSA),
}

// ANCHOR: here
fn valore_in_cent(moneta: Moneta) -> u8 {
    match moneta {
        Moneta::Penny => 1,
        Moneta::Nickel => 5,
        Moneta::Dime => 10,
        Moneta::Quarter(stato) => {
            println!("Quarter statale del {stato:?}!");
            25
        }
    }
}
// ANCHOR_END: here

fn main() {
    valore_in_cent(Moneta::Quarter(StatoUSA::Alaska));
}
