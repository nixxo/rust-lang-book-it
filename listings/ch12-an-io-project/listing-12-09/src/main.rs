use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Cerco {}", config.query);
    println!("Nel file {}", config.percorso_file);

    let contenuto = fs::read_to_string(config.percorso_file)
        .expect("Dovrebbe essere stato possibile leggere il file");

    println!("Con il testo:\n{contenuto}");
}

struct Config {
    query: String,
    percorso_file: String,
}

// ANCHOR: here
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Non ci sono abbastanza argomenti");
        }

        let query = args[1].clone();
        let percorso_file = args[2].clone();

        Ok(Config { query, percorso_file })
    }
}
// ANCHOR_END: here
