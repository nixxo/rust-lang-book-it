fn main() {
    // ANCHOR: here
    let v = vec!['a', 'b', 'c'];

    for (indice, valore) in v.iter().enumerate() {
        println!("{valore} è all'indice {indice}");
    }
    // ANCHOR_END: here
}
