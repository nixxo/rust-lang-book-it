// ANCHOR: here
enum Moneta {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn valore_in_cent(moneta: Moneta) -> u8 {
    match moneta {
        Moneta::Penny => 1,
        Moneta::Nickel => 5,
        Moneta::Dime => 10,
        Moneta::Quarter => 25,
    }
}
// ANCHOR_END: here

fn main() {}
