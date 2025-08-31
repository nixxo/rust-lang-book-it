fn stampa_e_ritorna_10(a: i32) -> i32 {
    println!("Ho ricevuto il valore {a}");
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let valore = stampa_e_ritorna_10(4);
        assert_eq!(valore, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let valore = stampa_e_ritorna_10(8);
        assert_eq!(valore, 5);
    }
}
