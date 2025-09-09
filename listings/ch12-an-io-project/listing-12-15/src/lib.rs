pub fn cerca<'a>(query: &str, contenuto: &'a str) -> Vec<&'a str> {
    unimplemented!();
}

// ANCHOR: here
// --taglio--

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
// ANCHOR_END: here
