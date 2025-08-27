mod cucine {
    pub enum Antipasti {
        Brodo,
        Insalata,
    }
}

pub fn mangiare_al_ristorante() {
    let ordine1 = cucine::Antipasti::Brodo;
    let ordine2 = cucine::Antipasti::Insalata;
}
