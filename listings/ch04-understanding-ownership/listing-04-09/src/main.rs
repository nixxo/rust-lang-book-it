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

    let mia_stringa_letterale = "hello world";

    // `prima_parola` funziona con slice di letterali di stringa,
    // parziali o intere.
    let parola = prima_parola(&mia_stringa_letterale[0..6]);
    let parola = prima_parola(&mia_stringa_letterale[..]);

    // E siccome i letterali di stringa *sono* già delle slice,
    // funziona pure così, senza usare la sintassi delle slice!
    let parola = prima_parola(mia_stringa_letterale);
}
// ANCHOR_END: usage
