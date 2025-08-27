mod sala_pranzo {
    pub mod accoglienza {
        pub fn aggiungi_in_lista() {}
    }
}

use crate::sala_pranzo::accoglienza;

mod cliente {
    pub fn mangiare_al_ristorante() {
        accoglienza::aggiungi_in_lista();
    }
}
