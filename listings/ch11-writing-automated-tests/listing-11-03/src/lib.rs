pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn esplorazione() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn un_altra() {
        panic!("Fai fallire questo test");
    }
}
