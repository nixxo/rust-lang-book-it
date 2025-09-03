use std::env;
use std::fs;

// ANCHOR: here
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    // ANCHOR_END: here

    println!("Cerco {}", config.query);
    println!("Nel file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Dovrebbe essere stato possibile leggere il file");

    println!("Con il testo:\n{contents}");
    // ANCHOR: here

    // --taglio--
}

// --taglio--

// ANCHOR_END: here
struct Config {
    query: String,
    file_path: String,
}

// ANCHOR: here
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
// ANCHOR_END: here
