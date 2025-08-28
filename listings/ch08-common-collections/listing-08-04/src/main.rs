fn main() {
    // ANCHOR: here
    let v = vec![1, 2, 3, 4, 5];

    let terzo: &i32 = &v[2];
    println!("Il terzo elemento è {terzo}");

    let terzo: Option<&i32> = v.get(2);
    match terzo {
        Some(terzo) => println!("Il terzo elemento è {terzo}"),
        None => println!("Non c'è un terzo elemento."),
    }
    // ANCHOR_END: here
}
