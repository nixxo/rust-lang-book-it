// ANCHOR: here
use std::env;
use std::fs;

fn main() {
    // --taglio--
    // ANCHOR_END: here
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let percorso_file = &args[2];

    println!("Cerco {query}");
    // ANCHOR: here
    println!("Nel file {percorso_file}");

    let contenuto = fs::read_to_string(percorso_file)
        .expect("Dovrebbe essere stato possibile leggere il file");

    println!("Con il testo:\n{contenuto}");
}
// ANCHOR_END: here
