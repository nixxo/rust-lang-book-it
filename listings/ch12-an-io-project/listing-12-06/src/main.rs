use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = leggi_config(&args);

    println!("Cerco {}", config.query);
    println!("Nel file {}", config.percorso_file);

    let contenuto = fs::read_to_string(config.percorso_file)
        .expect("Dovrebbe essere stato possibile leggere il file");

    // --taglio--
    // ANCHOR_END: here

    println!("Con il testo:\n{contenuto}");
    // ANCHOR: here
}

struct Config {
    query: String,
    percorso_file: String,
}

fn leggi_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let percorso_file = args[2].clone();

    Config { query, percorso_file }
}
// ANCHOR_END: here
