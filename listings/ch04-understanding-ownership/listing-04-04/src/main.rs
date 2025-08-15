fn main() {
    let s1 = cede_ownership();         // `cede_ownership` sposta il proprio
                                       // valore di ritorno in `s1`

    let s2 = String::from("hello");    // `s2` entra in scope

    let s3 = prende_e_restituisce(s2); // `s2` viene spostata in
                                       // `prende_e_restituisce`, che a sua
                                       // volta sposta il proprio valore
                                       // di ritorno in `s3`

} // Qui, `s3` esce dallo scope e viene cancellata con `drop`. `s2` era stata spostata
  // e quindi non succede nulla. `s1` viene cancellata con `drop` anch'essa.

fn cede_ownership() -> String {   // `cede_ownership` spostera il proprio valore di
                                  // ritorno alla funzione che l'ha chiamata

    let una_stringa = String::from("yours"); // `una_stringa` entra in scope

    una_stringa                     // `una_stringa` viene ritornata e spostata
                                    // alla funzione chiamante
}

// Questa funzione prende una String e ritorna una String.
fn prende_e_restituisce(altra_stringa: String) -> String {
                    // `altra_stringa` entra in scope

    altra_stringa   // `altra_stringa` viene ritornata
                    // e spostata alla funzione chiamante
}
