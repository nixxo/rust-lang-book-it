mod sala;

pub use crate::sala::accoglienza;

pub fn mangiare_al_ristorante() {
    accoglienza::aggiungi_in_lista();
}
