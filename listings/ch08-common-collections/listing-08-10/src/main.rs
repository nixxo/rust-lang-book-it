fn main() {
    // ANCHOR: here
    {
        let v = vec![1, 2, 3, 4];

        // esegui operazioni su `v`
    } // <- qui `v` esce dallo scope e la memoria viene liberata
    // ANCHOR_END: here
}
