fn main() {
    let stringa1 = String::from("abcd");
    let stringa2 = "xyz";

    let result = Piu_lunga_con_annuncio(
        stringa1.as_str(),
        stringa2,
        "Oggi è il compleanno di qualcuno!",
    );
    println!("La stringa più lunga è {result}");
}

// ANCHOR: here
use std::fmt::Display;

fn Piu_lunga_con_annuncio<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Annuncio! {ann}");
    if x.len() > y.len() { x } else { y }
}
// ANCHOR_END: here
