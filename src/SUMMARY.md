# Il Linguaggio di Programmazione Rust

[Il Linguaggio di Programmazione Rust](title-page.md)
[Prefazione](foreword.md)
[Introduzione](ch00-00-introduction.md)

## Primi Passi

- [Primi Passi](ch01-00-getting-started.md)
  - [Installazione](ch01-01-installation.md)
  - [Hello, World!](ch01-02-hello-world.md)
  - [Hello, Cargo!](ch01-03-hello-cargo.md)

- [Programmare un Gioco di Indovinelli](ch02-00-guessing-game-tutorial.md)

- [Concetti Comuni di Programmazione](ch03-00-common-programming-concepts.md)
  - [Variabili e Mutabilità](ch03-01-variables-and-mutability.md)
  - [Tipi di Dato](ch03-02-data-types.md)
  - [Funzioni](ch03-03-how-functions-work.md)
  - [Commenti](ch03-04-comments.md)
  - [Controllare il Flusso](ch03-05-control-flow.md)

- [Capire la _Ownership_](ch04-00-understanding-ownership.md)
  - [Cos’è la _Ownership_?](ch04-01-what-is-ownership.md)
  - [_Reference_ e _Borrowing_](ch04-02-references-and-borrowing.md)
  - [Il _Type_ Slice](ch04-03-slices.md)

- [Utilizzare le _Struct_ per Strutturare Dati Correlati](ch05-00-structs.md)
  - [Definire e Istanziare le _Struct_](ch05-01-defining-structs.md)
  - [Un Esempio di Programma Che Usa _Struct_](ch05-02-example-structs.md)
  - [Metodi](ch05-03-method-syntax.md)

- [Enumerazioni e Corrispondenza dei _Pattern_](ch06-00-enums.md)
  - [Definire un’_Enum_](ch06-01-defining-an-enum.md)
  - [Controllare il Flusso col Costrutto `match`](ch06-02-match.md)
  - [Controllare il Flusso con `if let` e `let else`](ch06-03-if-let.md)

## Letteratura di Base Rust

- [Pacchetti, _Crate_, e Moduli](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
  - [Pacchetti e _Crate_](ch07-01-packages-and-crates.md)
  - [Controllare _Scope_ e Privacy con i Moduli](ch07-02-defining-modules-to-control-scope-and-privacy.md)
  - [Percorsi per Fare Riferimento a un Elemento nell’Albero dei Moduli](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
  - [Portare i Percorsi in _Scope_ con la Parola Chiave `use`](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
  - [Separare i Moduli in File Diversi](ch07-05-separating-modules-into-different-files.md)

- [Collezioni Comuni](ch08-00-common-collections.md)
  - [Memorizzare Elenchi di Valori con Vettori](ch08-01-vectors.md)
  - [Memorizzare Testo Codificato UTF-8 con Stringhe](ch08-02-strings.md)
  - [Memorizzare Chiavi con Valori Associati in Mappe _Hash_](ch08-03-hash-maps.md)

- [Gestione degli Errori](ch09-00-error-handling.md)
  - [Errori Irreversibili con `panic!`](ch09-01-unrecoverable-errors-with-panic.md)
  - [Errori Reversibili con `Result`](ch09-02-recoverable-errors-with-result.md)
  - [`panic!` o non `panic!`](ch09-03-to-panic-or-not-to-panic.md)

- [_Type_ Generici, _Trait_ e _Lifetime_](ch10-00-generics.md)
  - [Tipi di Dati Generici](ch10-01-syntax.md)
  - [Definire il Comportamento Condiviso con i _Trait_](ch10-02-traits.md)
  - [Validare i _Reference_ con la _Lifetime_](ch10-03-lifetime-syntax.md)

- [Scrivere Test Automatizzati](ch11-00-testing.md)
  - [Come Scrivere dei Test](ch11-01-writing-tests.md)
  - [Controllare Come Vengono Eseguiti i Test](ch11-02-running-tests.md)
  - [Organizzare i Test](ch11-03-test-organization.md)

- [Un progetto I/O: Creare un Programma da Riga di Comando](ch12-00-an-io-project.md)
  - [Ricevere Argomenti dalla Riga di Comando](ch12-01-accepting-command-line-arguments.md)
  - [Leggere un File](ch12-02-reading-a-file.md)
  - [_Refactoring_ per Migliorare Modularità e Gestione degli Errori](ch12-03-improving-error-handling-and-modularity.md)
  - [Aggiungere Funzionalità con il Test-Driven Development](ch12-04-testing-the-librarys-functionality.md)
  - [Lavorare con le Variabili d’Ambiente](ch12-05-working-with-environment-variables.md)
  - [Scrivere Messaggi di Errore su _Standard Error_](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Pensare in Rust

- [Caratteristiche dei Linguaggi Funzionali: Iteratori e Chiusure](ch13-00-functional-features.md)
  - [Chiusure](ch13-01-closures.md)
  - [Elaborare una Serie di Elementi con Iteratori](ch13-02-iterators.md)
  - [Migliorare il Nostro Progetto I/O](ch13-03-improving-our-io-project.md)
  - [Prestazioni di Cicli e Iteratori](ch13-04-performance.md)

- [Maggiori informazioni su Cargo e Crates.io](ch14-00-more-about-cargo.md)
  - [Personalizzare le Build con i Profili di Rilascio](ch14-01-release-profiles.md)
  - [Pubblicare un _Crate_ su Crates.io](ch14-02-publishing-to-crates-io.md)
  - [Spazi di Lavoro Cargo](ch14-03-cargo-workspaces.md)
  - [Installazione di Binari con `cargo install`](ch14-04-installing-binaries.md)
  - [Estendere Cargo con Comandi Personalizzati](ch14-05-extending-cargo.md)

- [Puntatori Intelligenti](ch15-00-smart-pointers.md)
  - [Utilizzare `Box<T>` per Puntare ai Dati nell’Heap](ch15-01-box.md)
  - [Trattare i Puntatori Intelligenti Come Normali _Reference_ con `Deref`](ch15-02-deref.md)
  - [Eseguire del Codice Durante la Pulizia con il _Trait_ `Drop`](ch15-03-drop.md)
  - [`Rc<T>`, il Puntatore Intelligente con Conteggio dei _Reference_](ch15-04-rc.md)
  - [`RefCell<T>` e il Modello di Mutabilità Interna](ch15-05-interior-mutability.md)
  - [Cicli di Riferimento Possono Causare Perdite di Memoria](ch15-06-reference-cycles.md)

- [Concorrenza Senza Paura](ch16-00-concurrency.md)
  - [Usare i _Thread_ Per Eseguire Codice Simultaneamente](ch16-01-threads.md)
  - [Trasferire Dati tra _Thread_ Usando il Passaggio di Messaggi](ch16-02-message-passing.md)
  - [Concorrenza a Stato Condiviso](ch16-03-shared-state.md)
  - [Concorrenza Estensibile con i _Trait_ `Send` e `Sync`](ch16-04-extensible-concurrency-sync-and-send.md)

- [Fondamenti di Programmazione Asincrona: _Async_, _Await_, _Future_ e _Stream_](ch17-00-async-await.md)
  - [_Future_ e la Sintassi _Async_](ch17-01-futures-and-syntax.md)
  - [Applicare la Concorrenza con _Async_](ch17-02-concurrency-with-async.md)
  - [Lavorare con un Numero Qualsiasi di _Future_](ch17-03-more-futures.md)
  - [_Stream_: _Future_ in Sequenza](ch17-04-streams.md)
  - [Uno Sguardo Più Da Vicino ai _Trait_ per _Async_](ch17-05-traits-for-async.md)
  - [_Future_, _Task_ e _Thread_](ch17-06-futures-tasks-threads.md)

- [Object Oriented Programming Features of Rust](ch18-00-oop.md)
  - [Characteristics of Object-Oriented Languages](ch18-01-what-is-oo.md)
  - [Using Trait Objects That Allow for Values of Different Types](ch18-02-trait-objects.md)
  - [Implementing an Object-Oriented Design Pattern](ch18-03-oo-design-patterns.md)

## Tematiche Avanzate

- [Patterns and Matching](ch19-00-patterns.md)
  - [All the Places Patterns Can Be Used](ch19-01-all-the-places-for-patterns.md)
  - [Refutability: Whether a Pattern Might Fail to Match](ch19-02-refutability.md)
  - [Pattern Syntax](ch19-03-pattern-syntax.md)

- [Advanced Features](ch20-00-advanced-features.md)
  - [Unsafe Rust](ch20-01-unsafe-rust.md)
  - [Advanced Traits](ch20-02-advanced-traits.md)
  - [Advanced Types](ch20-03-advanced-types.md)
  - [Advanced Functions and Closures](ch20-04-advanced-functions-and-closures.md)
  - [Macros](ch20-05-macros.md)

- [Final Project: Building a Multithreaded Web Server](ch21-00-final-project-a-web-server.md)
  - [Building a Single-Threaded Web Server](ch21-01-single-threaded.md)
  - [Turning Our Single-Threaded Server into a Multithreaded Server](ch21-02-multithreaded.md)
  - [Graceful Shutdown and Cleanup](ch21-03-graceful-shutdown-and-cleanup.md)

- [Appendice](appendix-00.md)
  - [A - Parole Chiave](appendix-01-keywords.md)
  - [B - Operators and Symbols](appendix-02-operators.md)
  - [C - Derivable Traits](appendix-03-derivable-traits.md)
  - [D - Utili Strumenti di sviluppo](appendix-04-useful-development-tools.md)
  - [E - Editions](appendix-05-editions.md)
  - [F - Traduzioni del Libro](appendix-06-translation.md)
  - [G - How Rust is Made and “Nightly Rust”](appendix-07-nightly-rust.md)
  - [H - Note di Traduzione](appendix-08-note-di-traduzione.md)
