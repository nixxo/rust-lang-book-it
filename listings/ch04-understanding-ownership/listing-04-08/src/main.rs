fn prima_parola(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &lettera) in bytes.iter().enumerate() {
        if lettera == b' ' {
            return i;
        }
    }

    s.len()
}

// ANCHOR: here
fn main() {
    let mut s = String::from("hello world");

    let parola = prima_parola(&s); // `parola` riceverà il valore 5

    s.clear(); // questo svuota la String, rendendola uguale a ""

    // `parola` mantiene ancora il valore di 5, ma `s` non contiene più quello a cui
    // quel 5 si riferisce, quindi `parola` è adesso considerabile come non valida!
}
// ANCHOR_END: here
