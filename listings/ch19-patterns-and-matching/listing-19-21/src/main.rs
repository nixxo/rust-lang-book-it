fn main() {
    // ANCHOR: here
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("trovata una stringa");
    }

    println!("{s:?}");
    // ANCHOR_END: here
}
