fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut punteggi = HashMap::new();
    punteggi.insert(String::from("Blu"), 10);

    punteggi.entry(String::from("Gialla")).or_insert(50);
    punteggi.entry(String::from("Blu")).or_insert(50);

    println!("{punteggi:?}");
    // ANCHOR_END: here
}
