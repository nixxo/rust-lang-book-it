fn main() {
    let ipotesi = "3";
    // ANCHOR: here
    let ipotesi = match ipotesi.trim().parse() {
        Ok(_) => 5,
        Err(_) => "ciao",
    };
    // ANCHOR_END: here
}
