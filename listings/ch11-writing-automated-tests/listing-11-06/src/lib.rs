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

// ANCHOR: here
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grande_contiene_piccolo() {
        let grande = Rettangolo {
            larghezza: 8,
            altezza: 7,
        };
        let piccolo = Rettangolo {
            larghezza: 5,
            altezza: 1,
        };

        assert!(grande.può_contenere(&piccolo));
    }
}
// ANCHOR_END: here
