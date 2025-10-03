use gui::Disegna;

struct BoxSelezione {
    larghezza: u32,
    altezza: u32,
    opzioni: Vec<String>,
}

impl Disegna for BoxSelezione {
    fn disegna(&self) {
        // code to actually draw a select box
    }
}

// ANCHOR: here
use gui::{Bottone, Schermo};

fn main() {
    let schermo = Schermo {
        componenti: vec![
            Box::new(BoxSelezione {
                larghezza: 75,
                altezza: 10,
                opzioni: vec![
                    String::from("Sì"),
                    String::from("Forse"),
                    String::from("No"),
                ],
            }),
            Box::new(Bottone {
                larghezza: 50,
                altezza: 10,
                etichetta: String::from("OK"),
            }),
        ],
    };

    schermo.esegui();
}
// ANCHOR_END: here
