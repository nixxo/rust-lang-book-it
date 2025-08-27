mod sala {
    pub mod accoglienza {
        pub fn aggiungi_in_lista() {}
    }
}

use crate::sala::accoglienza;

mod cliente {
    pub fn mangiare_al_ristorante() {
        accoglienza::aggiungi_in_lista();
    }
}
