mod cucine {
    pub enum Antipasti {
        Zuppa,
        Insalata,
    }
}

pub fn mangiare_al_ristorante() {
    let ordine1 = cucine::Antipasti::Zuppa;
    let ordine2 = cucine::Antipasti::Insalata;
}
