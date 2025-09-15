fn stampa_e_ritorna_10(a: i32) -> i32 {
    println!("Ho ricevuto il valore {a}");
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn questo_test_passerà() {
        let valore = stampa_e_ritorna_10(4);
        assert_eq!(valore, 10);
    }

    #[test]
    fn questo_test_fallirà() {
        let valore = stampa_e_ritorna_10(8);
        assert_eq!(valore, 5);
    }
}
