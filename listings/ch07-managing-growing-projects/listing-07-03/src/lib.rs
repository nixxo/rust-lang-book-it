mod sala {
    mod accoglienza {
        fn aggiungi_in_lista() {}
    }
}

pub fn mangiare_al_ristorante() {
    // Path assoluta
    crate::sala::accoglienza::aggiungi_in_lista();

    // Path relativa
    sala::accoglienza::aggiungi_in_lista();
}
