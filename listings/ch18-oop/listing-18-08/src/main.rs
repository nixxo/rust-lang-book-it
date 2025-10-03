// ANCHOR: here
use gui::Disegna;

struct BoxSelezione {
    larghezza: u32,
    altezza: u32,
    opzioni: Vec<String>,
}

impl Disegna for BoxSelezione {
    fn disegna(&self) {
        // codice per disegnare il box di selezione
    }
}
// ANCHOR_END: here

fn main() {}
