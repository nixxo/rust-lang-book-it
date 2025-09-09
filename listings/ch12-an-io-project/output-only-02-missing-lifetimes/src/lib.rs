pub fn cerca(query: &str, contenuto: &str) -> Vec<&str> {
    vec![]
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
