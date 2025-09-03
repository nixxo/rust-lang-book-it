fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let testo = "hello world wonderful world";

    let mut map = HashMap::new();

    for parola in testo.split_whitespace() {
        let conteggio = map.entry(parola).or_insert(0);
        *conteggio += 1;
    }

    println!("{map:?}");
    // ANCHOR_END: here
}
