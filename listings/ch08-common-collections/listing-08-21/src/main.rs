fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut punteggi = HashMap::new();

    punteggi.insert(String::from("Blu"), 10);
    punteggi.insert(String::from("Gialla"), 50);

    let nome_squadra = String::from("Blu");
    let punteggio = punteggi.get(&nome_squadra).copied().unwrap_or(0);
    // ANCHOR_END: here
}
