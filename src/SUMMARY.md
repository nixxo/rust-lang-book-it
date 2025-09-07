# Il Linguaggio di Programmazione Rust

[Il Linguaggio di Programmazione Rust](title-page.md)
[Prefazione](foreword.md)
[Introduzione](ch00-00-introduction.md)

## Primi Passi

- [Primi Passi](ch01-00-getting-started.md)
  - [Installazione](ch01-01-installation.md)
  - [Hello, World!](ch01-02-hello-world.md)
  - [Hello, Cargo!](ch01-03-hello-cargo.md)

- [Programmare un gioco di indovinelli](ch02-00-guessing-game-tutorial.md)

- [Concetti comuni di programmazione](ch03-00-common-programming-concepts.md)
  - [Variabili e mutabilità](ch03-01-variables-and-mutability.md)
  - [Datatype - Tipi di dato](ch03-02-data-types.md)
  - [Funzioni](ch03-03-how-functions-work.md)
  - [Commenti](ch03-04-comments.md)
  - [Controllo del flusso](ch03-05-control-flow.md)

- [Capire la Ownership](ch04-00-understanding-ownership.md)
  - [Cos'è la Ownership?](ch04-01-what-is-ownership.md)
  - [Reference e Borrowing](ch04-02-references-and-borrowing.md)
  - [Il Type Slice](ch04-03-slices.md)

- [Utilizzare le Struct per Strutturare Dati Correlati](ch05-00-structs.md)
  - [Definire e Istanziare le Struct](ch05-01-defining-structs.md)
  - [Un esempio di programma che usa Struct](ch05-02-example-structs.md)
  - [Sintassi dei Metodi](ch05-03-method-syntax.md)

- [Enumerazioni e Corrispondenza dei Pattern](ch06-00-enums.md)
  - [Definire un Enum](ch06-01-defining-an-enum.md)
  - [Controllo del Flusso col costrutto Match](ch06-02-match.md)
  - [Controllo di flusso conciso con if let e let else](ch06-03-if-let.md)

## Letteratura di Base Rust

- [Gestione di Progetti in Crescita con Pacchetti, _Crates_, e Moduli](ch07-00-managing-growing-projects-with-packages-crates-and-modules.md)
  - [Pacchetti e Crate](ch07-01-packages-and-crates.md)
  - [Definire Moduli per Controllare _Scope_ e Privacy](ch07-02-defining-modules-to-control-scope-and-privacy.md)
  - [Percorsi per Fare Riferimento a un Elemento nell'Albero dei Moduli](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.md)
  - [Portare i Percorsi in _Scope_ con la Parola Chiave `use`](ch07-04-bringing-paths-into-scope-with-the-use-keyword.md)
  - [Separare i Moduli in File Diversi](ch07-05-separating-modules-into-different-files.md)

- [Collezioni Comuni](ch08-00-common-collections.md)
  - [Memorizzazione di Elenchi di Valori con Vettori](ch08-01-vectors.md)
  - [Memorizzazione di Testo Codificato UTF-8 con Stringhe](ch08-02-strings.md)
  - [Memorizzazione di Chiavi con Valori Associati in Mappe Hash](ch08-03-hash-maps.md)

- [Gestione degli Errori](ch09-00-error-handling.md)
  - [Errori Irreversibili con `panic!`](ch09-01-unrecoverable-errors-with-panic.md)
  - [Errori Reversibili con `Result`](ch09-02-recoverable-errors-with-result.md)
  - [`panic!` o non `panic!`](ch09-03-to-panic-or-not-to-panic.md)

- [Generic Types, Traits, and Lifetimes](ch10-00-generics.md)
  - [Generic Data Types](ch10-01-syntax.md)
  - [Traits: Defining Shared Behavior](ch10-02-traits.md)
  - [Validating References with Lifetimes](ch10-03-lifetime-syntax.md)

- [Scrivere Test Automatizzati](ch11-00-testing.md)
  - [How to Write Tests](ch11-01-writing-tests.md)
  - [Controlling How Tests Are Run](ch11-02-running-tests.md)
  - [Test Organization](ch11-03-test-organization.md)

- [An I/O Project: Building a Command Line Program](ch12-00-an-io-project.md)
  - [Accepting Command Line Arguments](ch12-01-accepting-command-line-arguments.md)
  - [Reading a File](ch12-02-reading-a-file.md)
  - [Refactoring to Improve Modularity and Error Handling](ch12-03-improving-error-handling-and-modularity.md)
  - [Developing the Library’s Functionality with Test Driven Development](ch12-04-testing-the-librarys-functionality.md)
  - [Working with Environment Variables](ch12-05-working-with-environment-variables.md)
  - [Writing Error Messages to Standard Error Instead of Standard Output](ch12-06-writing-to-stderr-instead-of-stdout.md)

## Thinking in Rust

- [Functional Language Features: Iterators and Closures](ch13-00-functional-features.md)
  - [Closures: Anonymous Functions that Capture Their Environment](ch13-01-closures.md)
  - [Processing a Series of Items with Iterators](ch13-02-iterators.md)
  - [Improving Our I/O Project](ch13-03-improving-our-io-project.md)
  - [Comparing Performance: Loops vs. Iterators](ch13-04-performance.md)

- [More about Cargo and Crates.io](ch14-00-more-about-cargo.md)
  - [Customizing Builds with Release Profiles](ch14-01-release-profiles.md)
  - [Publishing a Crate to Crates.io](ch14-02-publishing-to-crates-io.md)
  - [Cargo Workspaces](ch14-03-cargo-workspaces.md)
  - [Installing Binaries from Crates.io with `cargo install`](ch14-04-installing-binaries.md)
  - [Extending Cargo with Custom Commands](ch14-05-extending-cargo.md)

- [Smart Pointers](ch15-00-smart-pointers.md)
  - [Using `Box<T>` to Point to Data on the Heap](ch15-01-box.md)
  - [Treating Smart Pointers Like Regular References with `Deref`](ch15-02-deref.md)
  - [Running Code on Cleanup with the `Drop` Trait](ch15-03-drop.md)
  - [`Rc<T>`, the Reference Counted Smart Pointer](ch15-04-rc.md)
  - [`RefCell<T>` and the Interior Mutability Pattern](ch15-05-interior-mutability.md)
  - [Reference Cycles Can Leak Memory](ch15-06-reference-cycles.md)

- [Fearless Concurrency](ch16-00-concurrency.md)
  - [Using Threads to Run Code Simultaneously](ch16-01-threads.md)
  - [Using Message Passing to Transfer Data Between Threads](ch16-02-message-passing.md)
  - [Shared-State Concurrency](ch16-03-shared-state.md)
  - [Extensible Concurrency with the `Send` and `Sync` Traits](ch16-04-extensible-concurrency-sync-and-send.md)

- [Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams](ch17-00-async-await.md)
  - [Futures and the Async Syntax](ch17-01-futures-and-syntax.md)
  - [Applying Concurrency with Async](ch17-02-concurrency-with-async.md)
  - [Working With Any Number of Futures](ch17-03-more-futures.md)
  - [Streams: Futures in Sequence](ch17-04-streams.md)
  - [A Closer Look at the Traits for Async](ch17-05-traits-for-async.md)
  - [Futures, Tasks, and Threads](ch17-06-futures-tasks-threads.md)

- [Object Oriented Programming Features of Rust](ch18-00-oop.md)
  - [Characteristics of Object-Oriented Languages](ch18-01-what-is-oo.md)
  - [Using Trait Objects That Allow for Values of Different Types](ch18-02-trait-objects.md)
  - [Implementing an Object-Oriented Design Pattern](ch18-03-oo-design-patterns.md)

## Advanced Topics

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
