#[derive(Debug)]
struct Rettangolo {
    larghezza: u32,
    altezza: u32,
}

impl Rettangolo {
    fn può_contenere(&self, altro: &Rettangolo) -> bool {
        self.larghezza > altro.larghezza && self.altezza > altro.altezza
    }
}
