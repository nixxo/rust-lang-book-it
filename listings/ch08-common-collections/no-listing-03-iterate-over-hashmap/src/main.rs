fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut punteggi = HashMap::new();

    punteggi.insert(String::from("Blu"), 10);
    punteggi.insert(String::from("Gialla"), 50);

    for (chiave, valore) in &punteggi {
        println!("{chiave}: {valore}");
    }
    // ANCHOR_END: here
}
