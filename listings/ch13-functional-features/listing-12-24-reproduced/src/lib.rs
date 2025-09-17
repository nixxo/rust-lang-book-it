pub fn cerca<'a>(query: &str, contenuto: &'a str) -> Vec<&'a str> {
    let mut risultato = Vec::new();

    for line in contenuto.lines() {
        if line.contains(query) {
            risultato.push(line);
        }
    }

    risultato
}

pub fn cerca_case_insensitive<'a>(
    query: &str,
    contenuto: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut risultato = Vec::new();

    for line in contenuto.lines() {
        if line.to_lowercase().contains(&query) {
            risultato.push(line);
        }
    }

    risultato
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "dut";
        let contenuto = "\
Rust:
sicuro, veloce, produttivo.
Scegline tre.
Duttilità.";

        assert_eq!(vec!["sicuro, veloce, produttivo."], cerca(query, contenuto));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contenuto = "\
Rust:
sicuro, veloce, produttivo.
Scegline tre.
Una frusta.";

        assert_eq!(
            vec!["Rust:", "Una frusta."],
            cerca_case_insensitive(query, contenuto)
        );
    }
}
