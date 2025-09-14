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

    pub fn setta_valore(&mut self, valore: usize) {
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

// ANCHOR: here
#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessaggiatore {
        messaggi_inviati: Vec<String>,
    }

    impl MockMessaggiatore {
        fn new() -> MockMessaggiatore {
            MockMessaggiatore {
                messaggi_inviati: vec![],
            }
        }
    }

    impl Messaggiatore for MockMessaggiatore {
        fn invia(&self, messaggio: &str) {
            self.messaggi_inviati.push(String::from(messaggio));
        }
    }

    #[test]
    fn invia_un_messaggio_di_avviso_di_superamento_del_75_percento() {
        let mock_messaggiatore = MockMessaggiatore::new();
        let mut traccia_limiti = TracciaLimiti::new(&mock_messaggiatore, 100);

        traccia_limiti.setta_valore(80);

        assert_eq!(mock_messaggiatore.messaggi_inviati.len(), 1);
    }
}
// ANCHOR_END: here
