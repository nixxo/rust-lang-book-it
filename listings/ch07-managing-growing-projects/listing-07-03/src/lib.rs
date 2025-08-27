mod sala_pranzo {
    mod accoglienza {
        fn aggiungi_in_lista() {}
    }
}

pub fn mangiare_al_ristorante() {
    // Path assoluta
    crate::sala_pranzo::accoglienza::aggiungi_in_lista();

    // Path relativa
    sala_pranzo::accoglienza::aggiungi_in_lista();
}
