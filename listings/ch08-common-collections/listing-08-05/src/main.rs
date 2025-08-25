fn main() {
    // ANCHOR: here
    let v = vec![1, 2, 3, 4, 5];

    let non_esiste = &v[100];
    let non_esiste = v.get(100);
    // ANCHOR_END: here
}
