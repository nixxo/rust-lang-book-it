pub fn più_uno(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn funziona() {
        assert_eq!(3, più_uno(2));
    }
}
