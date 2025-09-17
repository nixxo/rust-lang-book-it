use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::{cerca, cerca_case_insensitive};

// ANCHOR: here
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problema nella lettura degli argomenti: {err}");
        process::exit(1);
    });

    // --taglio--
    // ANCHOR_END: here

    if let Err(e) = esegui(config) {
        eprintln!("Errore dell'applicazione: {e}");
        process::exit(1);
    }
    // ANCHOR: here
}
// ANCHOR_END: here

pub struct Config {
    pub query: String,
    pub percorso_file: String,
    pub ignora_maiuscole: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("non ci sono abbastanza argomenti");
        }

        let query = args[1].clone();
        let percorso_file = args[2].clone();

        let ignora_maiuscole = env::var("IGNORA_MAIUSCOLE").is_ok();

        Ok(Config {
            query,
            percorso_file,
            ignora_maiuscole,
        })
    }
}

fn esegui(config: Config) -> Result<(), Box<dyn Error>> {
    let contenuto = fs::read_to_string(config.percorso_file)?;

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
