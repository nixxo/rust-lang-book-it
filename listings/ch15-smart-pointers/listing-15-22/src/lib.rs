pub trait Messaggero {
    fn invia(&self, msg: &str);
}

pub struct TracciaLimiti<'a, T: Messaggero> {
    messaggero: &'a T,
    valore: usize,
    max: usize,
}

impl<'a, T> TracciaLimiti<'a, T>
where
    T: Messaggero,
{
    pub fn new(messaggero: &'a T, max: usize) -> TracciaLimiti<'a, T> {
        TracciaLimiti {
            messaggero,
            valore: 0,
            max,
        }
    }

    pub fn set_valore(&mut self, valore: usize) {
        self.valore = valore;

        let percentuale_di_max = self.valore as f64 / self.max as f64;

        if percentuale_di_max >= 1.0 {
            self.messaggero.invia("Errore: Hai superato la tua quota!");
        } else if percentuale_di_max >= 0.9 {
            self.messaggero
                .invia("Avviso urgente: Hai utilizzato oltre il 90% della tua quota!");
        } else if percentuale_di_max >= 0.75 {
            self.messaggero
                .invia("Avviso: Hai utilizzato oltre il 75% della tua quota!");
        }
    }
}

// ANCHOR: here
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessaggero {
        messaggi_inviati: RefCell<Vec<String>>,
    }

    impl MockMessaggero {
        fn new() -> MockMessaggero {
            MockMessaggero {
                messaggi_inviati: RefCell::new(vec![]),
            }
        }
    }

    impl Messaggero for MockMessaggero {
        fn invia(&self, messaggio: &str) {
            self.messaggi_inviati.borrow_mut().push(String::from(messaggio));
        }
    }

    #[test]
    fn invia_un_messaggio_di_avviso_di_superamento_del_75_percento() {
        // --taglio--
        // ANCHOR_END: here
        let mock_messaggero = MockMessaggero::new();
        let mut traccia_limiti = TracciaLimiti::new(&mock_messaggero, 100);

        traccia_limiti.set_valore(80);
        // ANCHOR: here

        // ANCHOR: here
        assert_eq!(mock_messaggero.messaggi_inviati.borrow().len(), 1);
    }
}
// ANCHOR_END: here
