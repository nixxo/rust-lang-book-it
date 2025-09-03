fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let nome_campo = String::from("Colore preferito");
    let valore_campo = String::from("Blu");

    let mut map = HashMap::new();
    map.insert(nome_campo, valore_campo);
    // nome_campo e valore_campo non sono validi a questo punto,
    // prova a usarli e vedi quale errore di compilazione ottieni!
    // ANCHOR_END: here
}
