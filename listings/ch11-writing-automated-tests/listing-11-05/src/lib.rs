#[derive(Debug)]
struct Rettangolo {
    larghezza: u32,
    altezza: u32,
}

impl Rettangolo {
    fn puo_contenere(&self, altro: &Rettangolo) -> bool {
        self.larghezza > altro.larghezza && self.altezza > altro.altezza
    }
}
