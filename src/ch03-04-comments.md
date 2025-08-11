## Commenti

Tutti i programmatori si sforzano di rendere il loro codice facile da capire, ma
a volte è necessario fornire ulteriori spiegazioni. In questi casi, i
programmatori lasciano dei _commenti_ nel loro codice sorgente che il
compilatore ignorerà ma che chi legge il codice sorgente potrebbe trovare utili.

Ecco un semplice commento:

```rust
// hello, world
```

In Rust, lo stile idiomatico di commento inizia un commento con due barre
oblique, _slash_ in inglese, e il commento continua fino alla fine della riga.
Per i commenti che si estendono oltre una singola riga, dovrai includere `//` su
ogni riga, come in questo caso:

```rust
// Stiamo facendo qualcosa di complicato, tanto da aver bisogno di
// più righe di commento per farlo! Speriamo che questo commento possa
// spiegare cosa sta succedendo.
```

I commenti possono essere inseriti anche alla fine delle righe contenenti
codice:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

Ma più spesso li vedrai utilizzati in questo formato, con il commento su una
riga separata sopra il codice che sta annotando:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

Rust ha anche un altro tipo di commento, i commenti alla documentazione, di cui
parleremo nella sezione ["Pubblicazione di un Crate su
Crates.io"][pubblicazione]<!-- ignore --> del Capitolo 14.

[pubblicazione]: ch14-02-publishing-to-crates-io.html
