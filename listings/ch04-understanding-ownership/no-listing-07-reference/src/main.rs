// ANCHOR: all
fn main() {
    // ANCHOR: here
    let s1 = String::from("ciao");

    let lung = calcola_lunghezza(&s1);
    // ANCHOR_END: here

    println!("La lunghezza di '{s1}' è {lung}.");
}

fn calcola_lunghezza(s: &String) -> usize {
    s.len()
}
// ANCHOR_END: all
