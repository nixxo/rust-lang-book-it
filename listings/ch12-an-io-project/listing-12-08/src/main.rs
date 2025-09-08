use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Cerco {}", config.query);
    println!("Nel file {}", config.file_path);

    let contenuto = fs::read_to_string(config.file_path)
        .expect("Dovrebbe essere stato possibile leggere il file");

    println!("Con il testo:\n{contenuto}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // ANCHOR: here
    // --taglio--
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("non ci sono abbastanza argomenti");
        }
        // --taglio--
        // ANCHOR_END: here

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
