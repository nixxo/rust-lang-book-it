fn main() {
    // ANCHOR: here
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Ottenuto: {val}");
    }
    // ANCHOR_END: here
}
