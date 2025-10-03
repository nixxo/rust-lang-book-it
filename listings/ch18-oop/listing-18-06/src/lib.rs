pub trait Disegna {
    fn disegna(&self);
}

// ANCHOR: here
pub struct Schermo<T: Disegna> {
    pub componenti: Vec<T>,
}

impl<T> Screen<T>
where
    T: Disegna,
{
    pub fn esegui(&self) {
        for componente in self.componenti.iter() {
            componente.disegna();
        }
    }
}
// ANCHOR_END: here
