fn prima_parola(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &lettera) in bytes.iter().enumerate() {
        if lettera == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: here
fn main() {
    let mut s = String::from("hello world");

    let parola = prima_parola(&s);

    s.clear(); // errore!

    println!("la prima parola è: {parola}");
}
// ANCHOR_END: here
