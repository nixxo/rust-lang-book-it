#[derive(Debug)]
struct Rettangolo {
    larghezza: u32,
    altezza: u32,
}

// ANCHOR: here
// --taglio--
impl Rettangolo {
    fn puo_contenere(&self, altro: &Rettangolo) -> bool {
        self.larghezza < altro.larghezza && self.altezza > altro.altezza
    }
}
// ANCHOR_END: here

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

        assert!(grande.puo_contenere(&piccolo));
    }

    #[test]
    fn piccolo_non_contiene_grande() {
        let grande = Rettangolo {
            larghezza: 8,
            altezza: 7,
        };
        let piccolo = Rettangolo {
            larghezza: 5,
            altezza: 1,
        };

        assert!(!piccolo.puo_contenere(&grande));
    }
}
