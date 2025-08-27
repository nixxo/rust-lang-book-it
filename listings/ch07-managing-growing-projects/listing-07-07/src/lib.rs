// ANCHOR: here
mod sala_pranzo {
    pub mod accoglienza {
        pub fn aggiungi_in_lista() {}
    }
}

// -- snip --
// ANCHOR_END: here
pub fn mangiare_al_ristorante() {
    // Path assoluta
    crate::sala_pranzo::accoglienza::aggiungi_in_lista();

    // Path relativa
    sala_pranzo::accoglienza::aggiungi_in_lista();
}
