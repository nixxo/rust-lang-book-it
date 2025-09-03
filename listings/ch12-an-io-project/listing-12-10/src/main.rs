use std::env;
use std::fs;
// ANCHOR: here
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problema nella lettura degli argomenti: {err}");
        process::exit(1);
    });

    // --taglio--
    // ANCHOR_END: here

    println!("Cerco {}", config.query);
    println!("Nel file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Dovrebbe essere stato possibile leggere il file");

    println!("Con il testo:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Non ci sono abbastanza argomenti");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
