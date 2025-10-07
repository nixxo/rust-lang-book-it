struct Contatore {
    conteggio: u32,
}

impl Contatore {
    fn new() -> Contatore {
        Contatore { conteggio: 0 }
    }
}

// ANCHOR: ch19
impl Iterator for Contatore {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --taglio--
        // ANCHOR_END: ch19
        if self.conteggio < 5 {
            self.conteggio += 1;
            Some(self.conteggio)
        } else {
            None
        }
    }
}
