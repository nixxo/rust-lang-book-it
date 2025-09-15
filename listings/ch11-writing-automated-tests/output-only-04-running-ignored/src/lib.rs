pub fn aggiungi_due(a: u64) -> u64 {
    a + 2
}

// ANCHOR: here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn somma_due_e_due() {
        let risultato = aggiungi_due(2);
        assert_eq!(risultato, 4);
    }

    #[test]
    #[ignore]
    fn test_impegnativo() {
        // codice che richiede un'ora per completarsi
    }
}
// ANCHOR_END: here
