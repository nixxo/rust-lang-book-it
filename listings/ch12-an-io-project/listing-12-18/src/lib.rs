// ANCHOR: here
pub fn cerca<'a>(query: &str, contenuto: &'a str) -> Vec<&'a str> {
    for line in contenuto.lines() {
        if line.contains(query) {
            // facciamo qualcosa con la riga
        }
    }
}
// ANCHOR_END: here

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
