## Estendere Cargo con Comandi Personalizzati

Cargo è stato progettato in modo che tu possa estenderlo con nuovi sotto-comandi
senza doverlo modificare. Se un binario nella tua `$PATH` si chiama
`cargo-qualcosa`, puoi eseguirlo come se fosse un sotto-comando di Cargo
eseguendo `cargo qualcosa`. I comandi personalizzati come questo sono anche
elencati quando esegui `cargo --list`. La possibilità di usare `cargo install`
per installare le estensioni e poi eseguirle proprio come gli strumenti
integrati di Cargo è un vantaggio super conveniente del design di Cargo!

## Riepilogo

La condivisione di codice con Cargo e [crates.io](https://crates.io/)<!-- ignore
--> è parte di ciò che rende l’ecosistema Rust utile per molti compiti diversi.
La libreria standard di Rust è piccola e stabile, ma i _crate_ sono facili da
condividere, usare e migliorare con una tempistica diversa da quella del
linguaggio. Non essere timido nel condividere il codice che ti è utile su
[crates.io](https://crates.io/)<!-- ignore -->; è probabile che sia utile anche
a qualcun altro!
