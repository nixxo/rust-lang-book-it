fn main() {
    let stringa1 = String::from("abcd");
    let stringa2 = "xyz";

    let risultato = piu_lunga(stringa1.as_str(), stringa2);
    println!("La stringa più lunga è {}", risultato);
}

// ANCHOR: here
fn piu_lunga<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// ANCHOR_END: here
