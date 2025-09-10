## Portare i Percorsi in _Scope_ con la Parola Chiave `use`

Dover scrivere i percorsi (_path_) per chiamare le funzioni può risultare
scomodo e ripetitivo. Nel Listato 7-7, sia che scegliessimo il _path_ assoluto o
relativo per la funzione `aggiungi_in_lista`, ogni volta che volevamo chiamare
`aggiungi_in_lista` dovevamo specificare anche `sala` e `accoglienza`.
Fortunatamente esiste un modo per semplificare questo processo: possiamo creare
un collegamento rapido a un _path_ con la parola chiave `use` una volta, e poi
usare il nome più corto nel resto dello _scope_.

Nel Listato 7-11, portiamo il modulo `crate::sala::accoglienza` nello _scope_
della funzione `mangiare_al_ristorante` così da dover specificare solo
`accoglienza::aggiungi_in_lista` per chiamare la funzione `aggiungi_in_lista` in
`mangiare_al_ristorante`.

<Listing number="7-11" file-name="src/lib.rs" caption="Portare un modulo nello _scope_ con `use`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

</Listing>

Aggiungere `use` e un _path_ in uno _scope_ è simile a creare un collegamento
simbolico nel filesystem. Aggiungendo `use crate::sala::accoglienza` nella
radice (_root_) del _crate_, `accoglienza` è ora un nome valido in quello
_scope_, come se il modulo `accoglienza` fosse stato definito nella radice del
_crate_. I _path_ portati nello _scope_ con `use` rispettano anche la _privacy_,
come qualsiasi altro _path_.

Nota che `use` crea il collegamento rapido solo per lo _scope_ particolare in
cui `use` è dichiarato. Il Listato 7-12 sposta la funzione
`mangiare_al_ristorante` in un nuovo modulo figlio chiamato `cliente`, che ora è
in uno _scope_ diverso rispetto alla dichiarazione `use`, quindi il corpo della
funzione non si compilerà.

<Listing number="7-12" file-name="src/lib.rs" caption="Una dichiarazione `use` si applica solo allo _scope_ in cui si trova.">

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

</Listing>

L'errore del compilatore mostra che il collegamento non si applica più
all'interno del modulo `cliente`:

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-12/output.txt}}
```

Nota che c'è l'avviso che il `use` non è più usato nel suo _scope_! Per
risolvere questo problema, sposta il `use` anche all'interno del modulo
`cliente`, o riferisciti al collegamento dal modulo genitore con
`super::accoglienza` all'interno del modulo figlio `cliente`.

### Creare Percorsi `use` Idiomatici

Nel Listato 7-11, forse ti sarai chiesto perché abbiamo specificato `use
crate::sala::accoglienza` e poi chiamato `accoglienza::aggiungi_in_lista` in
`mangiare_al_ristorante`, invece di specificare il _path_ `use` fino alla
funzione `aggiungi_in_lista` per ottenere lo stesso risultato, come nel Listato
7-13.

<Listing number="7-13" file-name="src/lib.rs" caption="Portare la funzione `aggiungi_in_lista` nello _scope_ con `use`, che non è idiomatico">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

</Listing>

Sebbene sia il Listato 7-11 sia il Listato 7-13 compiano lo stesso compito, il
Listato 7-11 è il modo idiomatico di portare una funzione nello _scope_ con
`use`. Portare il modulo genitore della funzione nello _scope_ con `use`
significa che dobbiamo specificare il modulo genitore quando chiamiamo la
funzione. Specificare il modulo genitore quando si chiama la funzione rende
chiaro che la funzione non è definita localmente riducendo comunque la
ripetizione del _path_ completo. Il codice in Listato 7-13 non chiarisce dove
sia definita `aggiungi_in_lista`.

D'altra parte, quando portiamo `struct`, `enum` e altri elementi con `use`, è
idiomatico specificare il _path_ completo. Il Listato 7-14 mostra il modo
idiomatico per portare, ad esempio, `HashMap` della libreria standard nello
_scope_ di un _crate binario_.

<Listing number="7-14" file-name="src/main.rs" caption="Portare `HashMap` nello _scope_ in modo idiomatico">

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

</Listing>

Non c'è una ragione forte dietro questo uso: è semplicemente la convenzione che
è emersa, e le persone si sono abituate a leggere e scrivere codice Rust in
questo modo.

L'eccezione a questa idioma è se stiamo portando due elementi con lo stesso nome
nello _scope_ con `use`, perché Rust non lo permette. Il Listato 7-15 mostra
come portare due `Result` nello _scope_ che hanno lo stesso nome ma moduli
genitore diversi, e come riferirsi a essi.

<Listing number="7-15" file-name="src/lib.rs" caption="Portare due _type_ con lo stesso nome nello stesso _scope_ richiede l'uso dei loro moduli genitore.">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

</Listing>

Come si vede, usare i moduli genitore distingue i due _type_ `Result`. Se invece
avessimo specificato `use std::fmt::Result` e `use std::io::Result`, avremmo due
_type_ `Result` nello stesso _scope_, e Rust non saprebbe quale intendessimo
quando avremmo usato `Result`.

### Fornire Nuovi Nomi con la Parola Chiave `as`

C'è un'altra soluzione al problema di portare due _type_ con lo stesso nome
nello stesso _scope_ con `use`: dopo il _path_, possiamo specificare `as` e un
nuovo nome locale, o _alias_, per il _type_. Il Listato 7-16 mostra un altro
modo di scrivere il codice del Listato 7-15 rinominando uno dei due _type_
`Result` con `as`.

<Listing number="7-16" file-name="src/lib.rs" caption="Rinominare un _type_ quando viene portato nello _scope_ con la parola chiave `as`">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

</Listing>

Nella seconda dichiarazione `use`, abbiamo scelto il nuovo _alias_ `IoResult`
per il _type_ `std::io::Result`, che non entrerà in conflitto con il `Result` di
`std::fmt` che abbiamo anch'esso portato nello _scope_. Sia il Listato 7-15 che
il Listato 7-16 sono considerati idiomatici, quindi la scelta spetta a te!

### Ri-Esportare Nomi con `pub use`

Quando portiamo un nome nello _scope_ con la parola chiave `use`, il nome è
privato allo _scope_ in cui lo abbiamo importato. Per consentire al codice
esterno a quello _scope_ di riferirsi a quel nome come se fosse stato definito
in quello _scope_, possiamo combinare `pub` e `use`. Questa tecnica si chiama
_ri-esportare_ (_re-exporting_) perché portiamo un elemento nello _scope_ ma lo
rendiamo anche disponibile affinché altri possano portarlo nel loro _scope_.

Il Listato 7-17 mostra il codice del Listato 7-11 con `use` nella radice del
modulo cambiato in `pub use`.

<Listing number="7-17" file-name="src/lib.rs" caption="Rendere un nome disponibile a qualsiasi codice da un nuovo _scope_ con `pub use`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

</Listing>

Prima di questa modifica, il codice esterno avrebbe dovuto chiamare la funzione
`aggiungi_in_lista` usando il _path_
`ristorante::sala::accoglienza::aggiungi_in_lista()`, che avrebbe inoltre
richiesto che il modulo `sala` fosse marcato come `pub`. Ora che questo `pub
use` ha ri-esportato il modulo `accoglienza` dalla radice del modulo, il codice
esterno può invece usare il _path_
`ristorante::accoglienza::aggiungi_in_lista()`.

La ri-esportazione è utile quando la struttura interna del tuo codice è diversa
da come i programmatori che chiamano il tuo codice penserebbero al dominio. Per
esempio, in questa metafora del ristorante, chi gestisce il ristorante pensa in
termini di “sala” e “cucine”. Ma i clienti che visitano un ristorante
probabilmente non penseranno alle parti del ristorante in questi termini. Con
`pub use`, possiamo scrivere il nostro codice con una struttura e però esporre
una struttura diversa. Ciò rende la nostra libreria ben organizzata sia per i
programmatori che lavorano sulla libreria sia per i programmatori che la usano.
Vedremo un altro esempio di `pub use` e di come influisce sulla documentazione
del crate in [“Esportare un API Pubblica Facilmente con `pub
use`”][ch14-pub-use]<!-- ignore --> nel Capitolo 14.

### Usare Pacchetti Esterni

Nel Capitolo 2, abbiamo programmato un progetto del gioco degli indovinelli che
usava un pacchetto esterno chiamato `rand` per ottenere numeri casuali. Per
usare `rand` nel nostro progetto, abbiamo aggiunto questa riga a _Cargo.toml_:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<Listing file-name="Cargo.toml">

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

</Listing>

Aggiungere `rand` come dipendenza in _Cargo.toml_ dice a Cargo di scaricare il
pacchetto `rand` e le sue dipendenze da [crates.io](https://crates.io/) e di
rendere `rand` disponibile al nostro progetto.

Poi, per portare le definizioni di `rand` nello _scope_ del nostro pacchetto,
abbiamo aggiunto una riga `use` che cominciava con il nome del _crate_, `rand`,
e elencava gli elementi che volevamo portare nello _scope_. Ricorda che in
[“Generare un Numero Casuale”][rand]<!-- ignore --> nel Capitolo 2, abbiamo
portato il _trait_ `Rng` nello _scope_ e chiamato la funzione
`rand::thread_rng`:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

Membri della community Rust hanno reso molti pacchetti disponibili su
[crates.io](https://crates.io/), e includere uno di essi nel tuo pacchetto
implica gli stessi passi: elencarlo nel file _Cargo.toml_ del tuo pacchetto e
usare `use` per portare elementi dai loro _crate_ nello _scope_.

Nota che la libreria standard `std` è anch'essa un _crate_ esterno al nostro
pacchetto. Poiché la libreria standard è distribuita con il linguaggio Rust, non
dobbiamo modificare _Cargo.toml_ per includere `std`. Ma dobbiamo comunque
riferirci ad essa con `use` per portare elementi da lì nello _scope_ del nostro
pacchetto. Per esempio, per `HashMap` useremmo questa riga:

```rust
use std::collections::HashMap;
```

Questo è un _path_ assoluto che inizia con `std`, il nome del _crate_ della
libreria standard.

### Usare Percorsi Nidificati per Accorpare Lunghi Elenchi di `use`

Se usiamo più elementi definiti nello stesso _crate_ o nello stesso modulo,
elencare ogni elemento su una sua riga può occupare molto spazio verticale nei
file. Per esempio, queste due dichiarazioni `use` che avevamo nel gioco degli
indovinelli nel Listato 2-4 portano elementi da `std` nello _scope_:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

</Listing>

Invece, possiamo usare _path_ nidificati per portare gli stessi elementi nello
_scope_ in una sola riga. Lo facciamo specificando la parte comune del _path_,
seguita da due due punti, e poi parentesi graffe intorno a una lista delle parti
che differiscono dei _path_, come mostrato nel Listato 7-18.

<Listing number="7-18" file-name="src/main.rs" caption="Specificare un _path_ nidificato per portare più elementi con lo stesso prefisso nello _scope_">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

</Listing>

Nei programmi più grandi, portare molti elementi nello _scope_ dallo stesso
_crate_ o modulo usando _path_ nidificati può ridurre di molto il numero di
dichiarazioni `use` separate necessarie!

Possiamo usare un _path_ nidificato a qualsiasi livello in un _path_, il che è
utile quando si combinano due dichiarazioni `use` che condividono una parte di
_path_. Per esempio, il Listato 7-19 mostra due dichiarazioni `use`: una che
porta `std::io` nello scope e una che porta `std::io::Write` nello scope.

<Listing number="7-19" file-name="src/lib.rs" caption="Due dichiarazioni `use` che condividono parte della _path_">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

</Listing>

La parte comune di questi due _path_ è `std::io`, ed è il _path_ completo della
prima path. Per unire questi due _path_ in una sola dichiarazione `use`,
possiamo usare `self` nel _path_ nidificato, come mostrato nel Listato 7-20.

<Listing number="7-20" file-name="src/lib.rs" caption="Combinare i _path_ nel Listato 7-19 in una sola dichiarazione `use`">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

</Listing>

Questa riga porta `std::io` e `std::io::Write` nello _scope_.

### L'Operatore _Glob_

Se vogliamo portare _tutti_ gli elementi pubblici definiti in un _path_ nello
_scope_, possiamo specificare quel _path_ seguito dall'operatore _glob_ `*`:

```rust
use std::collections::*;
```

Questa dichiarazione `use` porta tutti gli elementi pubblici definiti in
`std::collections` nello _scope_ corrente. Stai attento quando usi l'operatore
_glob_! Il _glob_ può rendere più difficile capire quali nomi sono nello _scope_
e da dove proviene un nome usato nel tuo programma. Inoltre, se la dipendenza
cambia le sue definizioni, ciò che hai importato cambia a sua volta, il che può
portare a errori del compilatore quando aggiorni la dipendenza se la dipendenza
aggiunge una definizione con lo stesso nome di una tua definizione nello stesso
_scope_, per esempio.

L'operatore _glob_ viene spesso usato durante i test per portare tutto ciò che è
sotto test nel modulo `tests`; parleremo di questo in [“Come Scrivere dei
Test”][writing-tests]<!-- ignore --> nel Capitolo 11. L'operatore _glob_ è anche
a volte usato come parte del _pattern_ _prelude_: vedi [la documentazione della
libreria standard][prelude]<!-- ignore --> per ulteriori informazioni su quel
_pattern_.

[ch14-pub-use]: ch14-02-publishing-to-crates-io.html#esportare-un-api-pubblica-facilmente-con-pub-use
[rand]: ch02-00-guessing-game-tutorial.html#generare-un-numero-casuale
[writing-tests]: ch11-01-writing-tests.html#come-scrivere-dei-test
[prelude]: https://doc.rust-lang.org/stable/std/prelude/index.html#other-preludes