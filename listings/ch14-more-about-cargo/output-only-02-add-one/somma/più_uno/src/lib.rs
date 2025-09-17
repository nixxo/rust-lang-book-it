pub fn somma(sinistra: u64, destra: u64) -> u64 {
    sinistra + destra
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn funziona() {
        let risultato = somma(2, 2);
        assert_eq!(risultato, 4);
    }
}
