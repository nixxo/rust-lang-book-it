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

    if let Err(e) = run(config) {
        println!("Errore nell'applicazione: {e}");
        process::exit(1);
    }
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

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contenuto = fs::read_to_string(config.file_path)?;

    for line in cerca(&config.query, &contenuto) {
        println!("{line}");
    }

    Ok(())
}
