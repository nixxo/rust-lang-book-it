fn main() {
    let stringa1 = String::from("abcd");
    let stringa2 = "efghijklmnopqrstuvwxyz";

    let risultato = più_lunga(stringa1.as_str(), stringa2);
    println!("La stringa più lunga è {risultato}");
}

// ANCHOR: here
fn più_lunga<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
// ANCHOR_END: here
