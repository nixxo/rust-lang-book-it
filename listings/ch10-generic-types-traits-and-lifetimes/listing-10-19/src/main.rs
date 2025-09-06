fn main() {
    let stringa1 = String::from("abcd");
    let stringa2 = "xyz";

    let risultato = piu_lunga(stringa1.as_str(), stringa2);
    println!("La stringa più lunga è {}", risultato);
}
