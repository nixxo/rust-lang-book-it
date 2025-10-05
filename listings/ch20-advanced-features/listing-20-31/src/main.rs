fn main() {
    // ANCHOR: here
    enum Stato {
        Valore(u32),
        Stop,
    }

    let lista_stati: Vec<Stato> = (0u32..20).map(Stato::Valore).collect();
    // ANCHOR_END: here
}
