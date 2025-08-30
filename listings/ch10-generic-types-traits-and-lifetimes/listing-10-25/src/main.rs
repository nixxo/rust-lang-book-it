// ANCHOR: here
fn prima_parola(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// ANCHOR_END: here

fn main() {
    let mia_string = String::from("hello world");

    // prima_parola lavora su slice di `String`s
    let parola = prima_parola(&mia_string[..]);

    let mia_stringa_literal = "hello world";

    // prima_parola lavora su slice di string literals
    let parola_literal = prima_parola(&mia_stringa_literal[..]);

    // Siccome le string literals *sono* già slices di stringa,
    // funziona anche senza la sintassi delle slice!
    let parola = prima_parola(mia_stringa_literal);
}
