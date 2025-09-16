use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::cerca;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problema nella lettura degli argomenti: {err}");
        process::exit(1);
    });

    if let Err(e) = esegui(config) {
        println!("Errore dell'applicazione: {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    percorso_file: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("non ci sono abbastanza argomenti");
        }

        let query = args[1].clone();
        let percorso_file = args[2].clone();

        Ok(Config { query, percorso_file })
    }
}

fn esegui(config: Config) -> Result<(), Box<dyn Error>> {
    let contenuto = fs::read_to_string(config.percorso_file)?;

    for line in cerca(&config.query, &contenuto) {
        println!("{line}");
    }

    Ok(())
}
