fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() aggiunge un letterale a una String

    println!("{s}");        // verrà stampato `hello, world!`
    // ANCHOR_END: here
}
