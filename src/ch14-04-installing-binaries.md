## Installazione di Binari con `cargo install`

Il comando `cargo install` ti permette di installare e utilizzare localmente i
_crate_ binari. Questo non è destinato a sostituire la gestione dei pacchetti di
sistema, ma è un modo comodo per gli sviluppatori di Rust di installare gli
strumenti che altri hanno condiviso su [crates.io](https://crates.io/)<!--
ignore -->. Nota che puoi installare solo i pacchetti che hanno dei target
binari. Un _target binario_ è il programma eseguibile che viene creato se il
_crate_ ha un file _src/main.rs_ o un altro file specificato come binario, in
contrapposizione a un _target_ libreria che non è eseguibile da solo ma è adatto
per essere incluso in altri programmi. Di solito, i _crate_ hanno informazioni
nel file _README_ sul fatto che un _crate_ è una libreria, è un binario o
entrambi.

Tutti i file binari installati con `cargo install` sono memorizzati nella
cartella _bin_ della radice dell’installazione. Se hai installato Rust
utilizzando `rustup` e non hai configurazioni personalizzate, questa cartella
sarà *$HOME/.cargo/bin*. Assicurati che questa cartella sia presente nella tua
`$PATH` per poter eseguire i programmi che hai installato con `cargo install`.

Ad esempio, nel Capitolo 12 abbiamo accennato all’esistenza di
un’implementazione Rust dello strumento `grep` chiamata `ripgrep` per la ricerca
di file. Per installare `ripgrep`, possiamo usare il seguente comando:

<!-- manual-regeneration
cargo install something you don't have, copy relevant output below
-->

```console
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v14.1.1
  Downloaded 1 crate (213.6 KB) in 0.40s
  Installing ripgrep v14.1.1
--taglio--
   Compiling grep v0.3.2
    Finished `release` profile [optimized + debuginfo] target(s) in 6.73s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v14.1.1` (executable `rg`)
```

La penultima riga dell’output mostra la posizione e il nome del binario
installato, che nel caso di `ripgrep` è `rg`. Se la directory di installazione è
presente nel tuo `$PATH`, come detto in precedenza, puoi eseguire `rg --help` e
iniziare a usare uno strumento più veloce e più ruspante per la ricerca dei
file!
