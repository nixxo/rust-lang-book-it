use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Non ci sono abbastanza argomenti");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// ANCHOR: here
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contenuto = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contenuto) {
        println!("{line}");
    }

    Ok(())
}
// ANCHOR_END: here

pub fn cerca<'a>(query: &str, contenuto: &'a str) -> Vec<&'a str> {
    let mut risultato = Vec::new();

    for line in contenuto.lines() {
        if line.contains(query) {
            risultato.push(line);
        }
    }

    risultato
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn un_risultato() {
        let query = "dut";
        let contenuto = "\
Rust:
sicuro, veloce, produttivo.
Scegline tre.";

        assert_eq!(vec!["sicuro, veloce, produttivo."], cerca(query, contenuto));
    }
}
