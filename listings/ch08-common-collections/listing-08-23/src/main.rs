fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut punteggi = HashMap::new();

    punteggi.insert(String::from("Blu"), 10);
    punteggi.insert(String::from("Blu"), 25);

    println!("{punteggi:?}");
    // ANCHOR_END: here
}
