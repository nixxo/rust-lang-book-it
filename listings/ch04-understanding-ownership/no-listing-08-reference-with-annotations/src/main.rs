fn main() {
    let s1 = String::from("ciao");

    let lung = calcola_lunghezza(&s1);

    println!("La lunghezza di '{s1}' è {lung}.");
}

// ANCHOR: here
fn calcola_lunghezza(s: &String) -> usize { // `s` è un reference a una String
    s.len()
} // Qui, `s` esce dallo scope. Ma siccome `s` non ha ownership di quello
  // a cui fa riferimento, i valori di String non vengono cancellati
// ANCHOR_END: here
