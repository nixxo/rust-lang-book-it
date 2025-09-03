// ANCHOR: here
mod sala {
    pub mod accoglienza {
        pub fn aggiungi_in_lista() {}
    }
}

// --taglio--
// ANCHOR_END: here
pub fn mangiare_al_ristorante() {
    // Path assoluta
    crate::sala::accoglienza::aggiungi_in_lista();

    // Path relativa
    sala::accoglienza::aggiungi_in_lista();
}
