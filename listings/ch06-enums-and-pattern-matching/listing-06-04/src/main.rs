// ANCHOR: here
#[derive(Debug)] // so we can inspect the state in a minute
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
// ANCHOR_END: here

fn main() {}
