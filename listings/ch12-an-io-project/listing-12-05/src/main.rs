use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, percorso_file) = leggi_config(&args);

    // --taglio--
    // ANCHOR_END: here

    println!("Cerco {query}");
    println!("Nel file {percorso_file}");

    let contenuto = fs::read_to_string(percorso_file)
        .expect("Dovrebbe essere stato possibile leggere il file");

    println!("Con il testo:\n{contenuto}");
    // ANCHOR: here
}

fn leggi_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let percorso_file = &args[2];

    (query, percorso_file)
}
// ANCHOR_END: here
