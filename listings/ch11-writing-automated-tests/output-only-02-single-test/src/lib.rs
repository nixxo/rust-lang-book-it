pub fn aggiungi_due(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn somma_due_e_due() {
        let risultato = aggiungi_due(2);
        assert_eq!(risultato, 4);
    }

    #[test]
    fn somma_due_e_tre() {
        let risultato = aggiungi_due(3);
        assert_eq!(risultato, 5);
    }

    #[test]
    fn cento() {
        let risultato = aggiungi_due(100);
        assert_eq!(risultato, 102);
    }
}
