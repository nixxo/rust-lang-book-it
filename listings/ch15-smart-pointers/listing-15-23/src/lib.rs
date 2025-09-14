pub trait Messaggiatore {
    fn invia(&self, msg: &str);
}

pub struct TracciaLimiti<'a, T: Messaggiatore> {
    messaggiatore: &'a T,
    valore: usize,
    max: usize,
}

impl<'a, T> TracciaLimiti<'a, T>
where
    T: Messaggiatore,
{
    pub fn new(messaggiatore: &'a T, max: usize) -> TracciaLimiti<'a, T> {
        TracciaLimiti {
            messaggiatore,
            valore: 0,
            max,
        }
    }

    pub fn set_valore(&mut self, valore: usize) {
        self.valore = valore;

        let percentuale_di_max = self.valore as f64 / self.max as f64;

        if percentuale_di_max >= 1.0 {
            self.messaggiatore.invia("Errore: Hai superato la tua quota!");
        } else if percentuale_di_max >= 0.9 {
            self.messaggiatore
                .invia("Avviso urgente: Hai utilizzato oltre il 90% della tua quota!");
        } else if percentuale_di_max >= 0.75 {
            self.messaggiatore
                .invia("Avviso: Hai utilizzato oltre il 75% della tua quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessaggiatore {
        messaggi_inviati: RefCell<Vec<String>>,
    }

    impl MockMessaggiatore {
        fn new() -> MockMessaggiatore {
            MockMessaggiatore {
                messaggi_inviati: RefCell::new(vec![]),
            }
        }
    }

    // ANCHOR: here
    impl Messaggiatore for MockMessaggiatore {
        fn invia(&self, messaggio: &str) {
            let mut un_borrow = self.messaggi_inviati.borrow_mut();
            un_borrow.push(String::from(messaggio));
        }
    }
    // ANCHOR_END: here

    #[test]
    fn invia_un_messaggio_di_avviso_di_superamento_del_75_percento() {
        let mock_messaggiatore = MockMessaggiatore::new();
        let mut traccia_limiti = TracciaLimiti::new(&mock_messaggiatore, 100);

        traccia_limiti.set_valore(80);

        assert_eq!(mock_messaggiatore.messaggi_inviati.borrow().len(), 1);
    }
}
