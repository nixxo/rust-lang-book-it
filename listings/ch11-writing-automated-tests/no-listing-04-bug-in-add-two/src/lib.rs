// ANCHOR: here
pub fn aggiungi_due(a: u64) -> u64 {
    a + 3
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggiungere_due() {
        let risultato = aggiungi_due(2);
        assert_eq!(risultato, 4);
    }
}
