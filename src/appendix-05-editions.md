## Appendice E - Edizioni

Nel Capitolo 1, hai visto visto che il comando `cargo new` aggiunge un po' di
metadata al tuo file _Cargo.toml_ riguardo a un’edizione. Questa appendice parla
di cosa significhi!

Il linguaggio Rust e il suo compilatore hanno un ciclo di rilascio di sei
settimane, il che significa che gli utenti ricevono un flusso continuo di nuove
funzionalità. Altri linguaggi di programmazione rilasciano cambiamenti più
grandi meno frequentemente; Rust rilascia aggiornamenti più piccoli più
frequentemente. Dopo un po', tutti questi piccoli cambiamenti si sommano. Ma da
una versione all’altra, può essere difficile guardare indietro e dire: “Wow, tra
Rust 1.10 e Rust 1.31, Rust è cambiato molto!”

Ogni circa tre anni, il team di Rust produce una nuova _edizione_ di Rust. Ogni
edizione raccoglie insieme le funzionalità introdotte in un pacchetto chiaro con
documentazione e strumenti completamente aggiornati. Le nuove edizioni vengono
rilasciate come parte del consueto processo di rilascio ogni sei settimane.

Le edizioni servono a scopi diversi per persone diverse:

- Per gli utenti attivi di Rust, una nuova edizione raccoglie i cambiamenti
  incrementali in un pacchetto facile da comprendere.
- Per chi non usa Rust, una nuova edizione segnala che sono stati introdotti
  miglioramenti importanti, che potrebbero far valere la pena di dare un’altra
  occhiata a Rust.
- Per chi sviluppa Rust, una nuova edizione fornisce un punto di aggregazione
  per l’intero progetto.

Al momento della scrittura, sono disponibili quattro edizioni di Rust: Rust
2015, Rust 2018, Rust 2021 e Rust 2024. Questo libro è scritto usando le
idiomatiche dell’edizione Rust 2024.

La chiave `edition` in _Cargo.toml_ indica quale edizione il compilatore deve
usare per il vostro codice. Se la chiave non esiste, Rust usa `2015` come valore
di edizione per ragioni di retro-compatibilità.

Ogni progetto può optare per un’edizione diversa dalla predefinita 2015. Le
edizioni possono contenere cambiamenti incompatibili, come l’inclusione di una
nuova parola chiave che confligge con identificatori nel codice. Tuttavia, a
meno che non si scelga di adottare questi cambiamenti, il vostro codice
continuerà a compilarsi anche aggiornando la versione del compilatore Rust.

Tutte le versioni di compilatore Rust supportano qualsiasi edizione esistita
prima del rilascio di quel compilatore, e possono collegare _crate_ di tutte le
edizioni supportate insieme. I cambiamenti di edizione influenzano solo il modo
in cui il compilatore analizza inizialmente il codice. Pertanto, se state usando
Rust 2015 e una delle vostre dipendenze usa Rust 2018, il vostro progetto si
compilerà e potrà usare quella dipendenza. Anche la situazione opposta, in cui
il vostro progetto usa Rust 2018 e una dipendenza usa Rust 2015, funziona
altrettanto bene.

Per essere chiari: la maggior parte delle funzionalità sarà disponibile in tutte
le edizioni. Gli sviluppatori che usano qualsiasi edizione di Rust vedranno
continuare a migliorare il linguaggio con nuovi rilasci stabili. Tuttavia, in
alcuni casi, principalmente quando vengono aggiunte nuove parole chiave, alcune
funzionalità nuove potrebbero essere disponibili solo nelle edizioni successive.
Sarà necessario cambiare edizione se si vuole usufruire di tali funzionalità.

Per ulteriori dettagli, la [_Guida alle edizioni_][edition-guide] è un libro
completo sulle edizioni che elenca le differenze tra le edizioni e spiega come
aggiornare automaticamente il vostro codice a una nuova edizione tramite `cargo
fix`.

[edition-guide]: https://doc.rust-lang.org/stable/edition-guide
