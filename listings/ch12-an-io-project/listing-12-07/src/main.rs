use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    // ANCHOR_END: here

    println!("Cerco {}", config.query);
    println!("Nel file {}", config.percorso_file);

    let contenuto = fs::read_to_string(config.percorso_file)
        .expect("Dovrebbe essere stato possibile leggere il file");

    println!("Con il testo:\n{contenuto}");
    // ANCHOR: here

    // --taglio--
}

// --taglio--

// ANCHOR_END: here
struct Config {
    query: String,
    percorso_file: String,
}

// ANCHOR: here
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let percorso_file = args[2].clone();

        Config { query, percorso_file }
    }
}
// ANCHOR_END: here
