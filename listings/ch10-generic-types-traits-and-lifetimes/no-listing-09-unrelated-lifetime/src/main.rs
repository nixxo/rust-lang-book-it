fn main() {
    let stringa1 = String::from("abcd");
    let stringa2 = "xyz";

    let risultato = più_lunga(stringa1.as_str(), stringa2);
    println!("La stringa più lunga è {risultato}");
}

// ANCHOR: here
fn più_lunga<'a>(x: &str, y: &str) -> &'a str {
    let risultato = String::from("una stringa bella lunga");
    risultato.as_str()
}
// ANCHOR_END: here
