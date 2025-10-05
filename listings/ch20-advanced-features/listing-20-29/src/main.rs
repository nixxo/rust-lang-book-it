fn main() {
    // ANCHOR: here
    let lista_di_numeri = vec![1, 2, 3];
    let lista_di_stringhe: Vec<String> =
        lista_di_numeri.iter().map(|i| i.to_string()).collect();
    // ANCHOR_END: here
}
