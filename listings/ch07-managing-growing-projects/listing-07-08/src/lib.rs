fn servi_ordine() {}

mod cucine {
    fn correzione_ordine() {
        cucina_ordine();
        super::servi_ordine();
    }

    fn cucina_ordine() {}
}
