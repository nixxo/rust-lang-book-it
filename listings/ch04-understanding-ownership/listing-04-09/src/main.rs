// ANCHOR: here
fn prima_parola(s: &str) -> &str {
    // ANCHOR_END: here
    let bytes = s.as_bytes();

    for (i, &lettera) in bytes.iter().enumerate() {
        if lettera == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: usage
fn main() {
    let mia_stringa = String::from("hello world");

    // `prima_parola` funziona con slice di `String`, parziali o intere.
    let parola = prima_parola(&mia_stringa[0..6]);
    let parola = prima_parola(&mia_stringa[..]);
    // `prima_parola` funziona anche con reference a `String`, che corrisponde
    // a una slice intera di `String`.
    let parola = prima_parola(&mia_stringa);

    let mia_stringa_literal = "hello world";

    // `prima_parola` funziona con slice di lettrali di stringhe,
    // parziali o intere.
    let parola = prima_parola(&mia_stringa_literal[0..6]);
    let parola = prima_parola(&mia_stringa_literal[..]);

    // E siccome i letterali di stringa *sono* già degli slice,
    // funziona pure così, senza usare la sintassi degli slice!
    let parola = prima_parola(mia_stringa_literal);
}
// ANCHOR_END: usage
