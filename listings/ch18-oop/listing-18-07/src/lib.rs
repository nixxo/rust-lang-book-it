pub trait Disegna {
    fn disegna(&self);
}

pub struct Schermo {
    pub componenti: Vec<Box<dyn Disegna>>,
}

impl Schermo {
    pub fn esegui(&self) {
        for componente in self.componenti.iter() {
            componente.disegna();
        }
    }
}

// ANCHOR: here
pub struct Bottone {
    pub larghezza: u32,
    pub altezza: u32,
    pub etichetta: String,
}

impl Disegna for Bottone {
    fn disegna(&self) {
        // codice per disegnare il bottone
    }
}
// ANCHOR_END: here
