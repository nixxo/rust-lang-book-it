use std::env;
use std::fs;
use std::process;
// ANCHOR: here
use std::error::Error;

// --taglio--

// ANCHOR_END: here

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problema nella lettura degli argomenti: {err}");
        process::exit(1);
    });

    println!("Cerco {}", config.query);
    println!("Nel file {}", config.file_path);

    run(config);
}

// ANCHOR: here
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contenuto = fs::read_to_string(config.file_path)?;

    println!("Con il testo:\n{contenuto}");

    Ok(())
}
// ANCHOR_END: here

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
