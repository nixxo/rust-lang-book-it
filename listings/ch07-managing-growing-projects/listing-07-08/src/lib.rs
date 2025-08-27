fn consegna_ordine() {}

mod cucine {
    fn correzione_ordine() {
        cucina_ordine();
        super::consegna_ordine();
    }

    fn cucina_ordine() {}
}
