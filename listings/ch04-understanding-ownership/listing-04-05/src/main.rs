fn main() {
    let s1 = String::from("ciao");

    let (s2, lung) = calcola_lunghezza(s1);

    println!("La lunghezza di '{s2}' è {lung}.");
}

fn calcola_lunghezza(s: String) -> (String, usize) {
    let lunghezza = s.len(); // len() restituisce la lunghezza di una String

    (s, lunghezza)
}
