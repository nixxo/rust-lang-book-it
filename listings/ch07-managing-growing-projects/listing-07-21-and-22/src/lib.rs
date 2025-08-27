mod sala_pranzo;

pub use crate::sala_pranzo::accoglienza;

pub fn mangiare_al_ristorante() {
    accoglienza::aggiungi_in_lista();
}
