fn main() {
    let reference_a_nulla = pendente();
}

// ANCHOR: here
fn pendente() -> &String { // pendente ritorna un reference a String

    let s = String::from("ciao"); // `s` è una String nuova

    &s // ritorniamo un reference alla String `s`
} // Qui `s` esce dallo scope e viene cancellata, così come la memora assegnatale.
  // Pericolo!
  // ANCHOR_END: here
