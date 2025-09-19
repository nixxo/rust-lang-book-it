extern crate trpl; // necessario per test mdbook

// ANCHOR: enum
enum TitoloPaginaFuture<'a> {
    Initial { url: &'a str },
    GetAwaitPoint { url: &'a str },
    TextAwaitPoint { risposta: trpl::Response },
}
// ANCHOR_END: enum
