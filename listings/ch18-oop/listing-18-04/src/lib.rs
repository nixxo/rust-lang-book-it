pub trait Disegna {
    fn disegna(&self);
}

// ANCHOR: here
pub struct Schermo {
    pub componenti: Vec<Box<dyn Disegna>>,
}
// ANCHOR_END: here
