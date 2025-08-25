fn main() {
    // ANCHOR: here
    let mut v = vec![1, 2, 3, 4, 5];

    let primo = &v[0];

    v.push(6);

    println!("Il primo elemento è: {primo}");
    // ANCHOR_END: here
}
