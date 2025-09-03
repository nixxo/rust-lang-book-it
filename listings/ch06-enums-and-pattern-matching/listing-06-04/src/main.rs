// ANCHOR: here
#[derive(Debug)] // così possiamo vederne i valori tra un po'
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
// ANCHOR_END: here

fn main() {}
