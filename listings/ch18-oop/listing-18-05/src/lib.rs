pub trait Disegna {
    fn disegna(&self);
}

pub struct Schermo {
    pub componenti: Vec<Box<dyn Disegna>>,
}

// ANCHOR: here
impl Schermo {
    pub fn esegui(&self) {
        for componente in self.componenti.iter() {
            componente.disegna();
        }
    }
}
// ANCHOR_END: here
