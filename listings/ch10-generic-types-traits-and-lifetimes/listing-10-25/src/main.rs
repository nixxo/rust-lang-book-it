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
    let my_string = String::from("hello world");

    // prima_parola lavora su slice di `String`s
    let word = prima_parola(&my_string[..]);

    let mia_stringa_literal = "hello world";

    // prima_parola lavora su slice di string literals
    let word = prima_parola(&mia_stringa_literal[..]);

    // Siccome le string literals *sono* già slices di stringa,
    // funziona anche senza la sintassi delle slice!
    let word = prima_parola(mia_stringa_literal);
}
