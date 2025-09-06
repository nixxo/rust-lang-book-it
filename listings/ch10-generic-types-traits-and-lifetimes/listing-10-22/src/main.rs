// ANCHOR: here
fn main() {
    let stringa1 = String::from("una stringa bella lunga");

    {
        let stringa2 = String::from("xyz");
        let risultato = piu_lunga(stringa1.as_str(), stringa2.as_str());
        println!("La stringa più lunga è {risultato}");
    }
}
// ANCHOR_END: here

fn piu_lunga<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
