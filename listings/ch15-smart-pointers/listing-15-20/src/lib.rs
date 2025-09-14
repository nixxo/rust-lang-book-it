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
