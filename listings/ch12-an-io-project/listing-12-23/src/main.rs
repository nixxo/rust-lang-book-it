use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::{cerca, cerca_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problema nella lettura degli argomenti: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Errore dell'applicazione: {e}");
        process::exit(1);
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignora_maiuscole: bool,
}

// ANCHOR: here
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("non ci sono abbastanza argomenti");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignora_maiuscole = env::var("IGNORA_MAIUSCOLE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignora_maiuscole,
        })
    }
}
// ANCHOR_END: here

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contenuto = fs::read_to_string(config.file_path)?;

    let risultato = if config.ignora_maiuscole {
        cerca_case_insensitive(&config.query, &contenuto)
    } else {
        cerca(&config.query, &contenuto)
    };

    for line in risultato {
        println!("{line}");
    }

    Ok(())
}
