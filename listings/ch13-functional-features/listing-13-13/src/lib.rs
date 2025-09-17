#[cfg(test)]
mod tests {
    // ANCHOR: here
    #[test]
    fn somma_con_iteratore() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let totale: i32 = v1_iter.sum();

        assert_eq!(totale, 6);
    }
    // ANCHOR_END: here
}
