mod sala {
    pub mod accoglienza {
        pub fn aggiungi_in_lista() {}
    }
}

use crate::sala::accoglienza::aggiungi_in_lista;

pub fn mangiare_al_ristorante() {
    aggiungi_in_lista();
}
