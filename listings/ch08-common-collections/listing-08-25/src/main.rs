fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let testo = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in testo.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
    // ANCHOR_END: here
}
