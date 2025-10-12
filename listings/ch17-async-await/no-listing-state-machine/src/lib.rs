extern crate trpl; // necessario per test mdbook

// ANCHOR: enum
enum TitoloPaginaFuture<'a> {
    Iniziale { url: &'a str },
    PrendiPuntoAttesa { url: &'a str },
    TestoPuntoAttesa { risposta: trpl::Response },
}
// ANCHOR_END: enum
