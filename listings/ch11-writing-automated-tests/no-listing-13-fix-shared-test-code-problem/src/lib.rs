pub fn aggiungi_due(a: u64) -> u64 {
    addizione_privata(a, 2)
}

fn addizione_privata(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn privata() {
        let result = addizione_privata(2, 2);
        assert_eq!(result, 4);
    }
}
