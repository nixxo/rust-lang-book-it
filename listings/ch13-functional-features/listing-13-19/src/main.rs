use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::{cerca, cerca_case_insensitive};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problema durante il parsing degli argomenti: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Errore dell'applicazione: {e}");
        process::exit(1);
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignora_case: bool,
}

// ANCHOR: here
impl Config {
    fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        // --taglio--
        // ANCHOR_END: here
        if args.len() < 3 {
            return Err("non ci sono abbastanza argomenti");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignora_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignora_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contenuti = fs::read_to_string(config.file_path)?;

    let risultati = if config.ignora_case {
        cerca_case_insensitive(&config.query, &contenuti)
    } else {
        cerca(&config.query, &contenuti)
    };

    for line in risultati {
        println!("{line}");
    }

    Ok(())
}
