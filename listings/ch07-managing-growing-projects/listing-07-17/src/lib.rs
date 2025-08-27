mod sala {
    pub mod accoglienza {
        pub fn aggiungi_in_lista() {}
    }
}

pub use crate::sala::accoglienza;

pub fn mangiare_al_ristorante() {
    accoglienza::aggiungi_in_lista();
}
