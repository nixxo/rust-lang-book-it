mod sala_pranzo {
    pub mod accoglienza {
        pub fn aggiungi_in_lista() {}
    }
}

use crate::sala_pranzo::accoglienza::aggiungi_in_lista;

pub fn mangiare_al_ristorante() {
    aggiungi_in_lista();
}
