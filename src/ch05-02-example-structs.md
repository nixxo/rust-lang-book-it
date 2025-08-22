## Un esempio di programma che usa _Struct_

Per capire quando potremmo voler usare le _struct_, scriviamo un programma che
calcola l'area di un rettangolo. Partiremo usando variabili singole e poi
riscriveremo il programma un pezzo per volta finché non useremo le _struct_.

Creiamo un nuovo progetto binario con _Cargo_ chiamato _rettangoli_ che prenderà
la larghezza e l'altezza di un rettangolo specificate in pixel e calcolerà
l'area del rettangolo. Il Listato 5-8 mostra un breve programma con un modo per
farlo nel file _src/main.rs_ del nostro progetto.

<Listing number="5-8" file-name="src/main.rs" caption="Calcolo dell'area di un rettangolo specificando in variabili separate larghezza e alatezza">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

</Listing>

Ora esegui questo programma usando `cargo run`:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

Questo codice riesce a calcolare l'area del rettangolo chiamando la funzione
`area` con ogni dimensione, ma possiamo fare di più per rendere il codice chiaro
e leggibile.

Il problema con questo codice è evidente nella firma di `area`:

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

La funzione `area` dovrebbe calcolare l'area di un rettangolo singolo, ma la
funzione che abbiamo scritto ha due parametri, e non è chiaro da nessuna parte
nel nostro programma che i parametri siano correlati. Sarebbe più leggibile e
più gestibile raggruppare larghezza e altezza insieme. Abbiamo già discusso un
modo per farlo nella sezione [“Il _Type_ Tuple”][the-tuple-type]<!-- ignore -->
del Capitolo 3: usando le _tuple_.

### Riscrittura con le _tuple_

Il Listato 5-9 mostra un'altra versione del nostro programma che usa le _tuple_.

<Listing number="5-9" file-name="src/main.rs" caption="Specificare larghezza e altezza di un rettangolo tramite una tuple">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

</Listing>

Da un lato, questo programma è migliore. Le _tuple_ ci permettono di aggiungere
un po' di struttura, e ora stiamo passando un solo argomento. Ma dall'altro,
questa versione è meno chiara: le _tuple_ non nominano i loro elementi, quindi
dobbiamo indicizzare le parti della _tuple_, rendendo il nostro calcolo meno
ovvio.

Confondere larghezza e altezza non avrebbe importanza per il calcolo dell'area,
ma se volessimo disegnare il rettangolo sullo schermo, importerebbe! Dovremmo
tenere a mente che `larghezza` è l'indice della _tuple_ `0` e `altezza` è
l'indice della _tuple_ `1`. Questo sarebbe ancora più difficile da capire e
ricordare per qualcun altro che in futuro leggesse o usasse il nostro codice.
Poiché non abbiamo reso palese il significato dei nostri dati nel codice, è più
facile introdurre errori.

### Riscrittura con le _Struct_: Aggiungere Più Significato

Usiamo la _struct_ per aggiungere significato etichettando i dati. Possiamo
trasformare la _tuple_ che stiamo usando in una _struct_ con un nome per
l'intero e nomi per le parti, come mostrato nel Listato 5-10.

<Listing number="5-10" file-name="src/main.rs" caption="Definizione di una _struct_ `Rettangolo`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

</Listing>

Qui abbiamo definito una _struct_ e l'abbiamo chiamata `Rettangolo`. All'interno
 delle parentesi graffe, abbiamo definito i campi come `larghezza` e `altezza`,
entrambi di _type_ `u32`. Poi, in `main`, abbiamo creato un'istanza particolare
di `Rettangolo` che ha larghezza `30` e altezza `50`.

La nostra funzione `area` è ora definita con un solo parametro, che abbiamo
chiamato `Rettangolo`, il cui _type_ è un _reference_ immutabile a un'istanza
della _struct_ `Rettangolo`. Come menzionato nel Capitolo 4, ci serve solo
prendere in prestito la _struct_ piuttosto che averne la _ownership_. In questo
modo, `main` mantiene la sua _ownership_ e può continuare a usare `rettangolo1`,
che è il motivo per cui usiamo `&` nella firma della funzione e dove chiamiamo
la funzione.

La funzione `area` accede ai campi `larghezza` e `altezza` dell'istanza di
`Rettangolo` (nota che accedere ai campi di un'istanza di _struct_ presa in
prestito non muove i valori dei campi, motivo per cui spesso si vedono
_reference_ di _struct_). La nostra firma della funzione per `area` ora dice
esattamente ciò che intendiamo: calcolare l'area di `Rettangolo`, usando i suoi
campi `larghezza` e `altezza`. Questo comunica che larghezza e altezza sono
correlate tra loro e fornisce nomi descrittivi ai valori invece di usare gli
indici della _tuple_ `0` e `1`. Questo è un vantaggio in termini di chiarezza.

### Aggiungere funzionalità utili con i _trait_ derivati

Sarebbe utile poter stampare un'istanza di `Rettangolo` mentre eseguiamo il
debug del nostro programma e vedere i valori di tutti i suoi campi. Il Listato
5-11 prova a usare la [macro `println!`][println]<!-- ignore --> come l'abbiamo
usata nei capitoli precedenti. Questo però non funzionerà.

<Listing number="5-11" file-name="src/main.rs" caption="Tentativo di stampare un'istanza di `Rettangolo`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

</Listing>

Quando compiliamo questo codice, otteniamo un errore con questo messaggio
principale:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

La macro `println!` può fare molti tipi di formattazione e, come impostazione
predefinita, le parentesi graffe dicono a `println!` di usare una formattazione
conosciuta come `Display`, output pensato per il l'utente finale che utilizzarà
il programma. I _type_ primitivi che abbiamo visto finora implementano `Display`
di default perché c'è un solo modo in cui vorresti mostrare un `1` o qualsiasi
altro _type_ primitivo a un utente. Ma con le _struct_ il modo in cui `println!`
dovrebbe formattare l'output è meno chiaro perché ci sono più possibilità di
visualizzazione: vuoi le virgole o no? Vuoi stampare le parentesi graffe? Devono
essere mostrati tutti i campi? A causa di questa ambiguità, Rust non cerca di
indovinare ciò che vogliamo, e le _struct_ non hanno un'implementazione standard
di `Display` da usare con `println!` e il segnaposto `{}`.

Se continuiamo a leggere gli errori, troveremo questa nota utile:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:12:13}}
```

Proviamolo! La chiamata alla macro `println!` ora assomiglierà a
`println!("rettangolo1 è {rettangolo1:?}");`. Inserire lo specificatore `:?`
all'interno delle parentesi graffe dice a `println!` che vogliamo usare un
formato di output chiamato `Debug`. Il _trait_ `Debug` ci permette di stampare
la nostra _struct_ in un modo utile per gli sviluppatori, così possiamo vedere
il suo valore mentre eseguiamo il debug del nostro codice.

Compila il codice con questa modifica. Accidenti! Otteniamo ancora un errore:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

Ma di nuovo, il compilatore ci dà una nota utile:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:11:12}}
```

Rust _include_ effettivamente funzionalità per stampare informazioni di debug,
ma dobbiamo esplicitamente dichiararlo per rendere disponibile quella
funzionalità alla nostra _struct_. Per farlo, aggiungiamo l'attributo esterno
`#[derive(Debug)]` appena prima della definizione della _struct_, come mostrato
nel Listato 5-12.

<Listing number="5-12" file-name="src/main.rs" caption="Aggiunta dell'attributo per derivare il _trait_ `Debug` e stampare `Rettangolo` usando la formattazione di debug">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

</Listing>

Ora quando eseguiamo il programma, non otterremo errori e vedremo il seguente
output:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

Bene! Non è l'output più bello, ma mostra i valori di tutti i campi per questa
istanza, il che aiuterebbe sicuramente durante lo sviluppo e il debug del
programma. Quando abbiamo _struct_ più grandi, è utile avere un output un po'
più facile da leggere; in quei casi, possiamo usare `{:#?}` invece di `{:?}`
nella stringa di `println!`. In questo esempio, usare lo stile `{:#?}` produrrà
il seguente output:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

Un altro modo per stampare un valore usando il formato `Debug` è usare la [macro
`dbg!`][dbg], che prende _ownership_ di un'espressione (a differenza di
`println!`, che prende un _reference_), stampa file e numero di linea di dove
quella chiamata a `dbg!` si verifica nel codice insieme al valore risultante di
quell'espressione, e restituisce l'_ownership_ del valore.

> Nota: Chiamare la macro _dbg!_ stampa sullo stream di errore standard
> (`stderr`), a differenza di `println!`, che stampa sullo stream di output
> standard (`stdout`). Parleremo meglio di `stderr` e `stdout` nella sezione
> [“Scrivere i Messaggi di Errore su Standard Error invece che su Standard
> Output” del Capitolo 12][err]<!-- ignore -->.

Ecco un esempio in cui siamo interessati al valore che viene assegnato al campo
`larghezza`, così come al valore dell'intera _struct_ in `rettangolo1`:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

Possiamo mettere `dbg!` attorno all'espressione `30 * scala` e, poiché `dbg!`
restituisce l'_ownership_ del valore dell'espressione, il campo `larghezza`
otterrà lo stesso valore come se non avessimo la chiamata a `dbg!` lì. Non
vogliamo che `dbg!` prenda _ownership_ di `rettangolo1`, quindi usiamo un
riferimento a `rettangolo1` nella chiamata successiva. Ecco come appare l'output
di questo esempio:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

Possiamo vedere che il primo frammento di output proviene da _src/main.rs_ riga
10 dove stiamo facendo il debug dell'espressione `30 * scala`, e il suo valore
risultante è `60` (la formattazione `Debug` implementata per gli _integer_ è di
stampare solo il loro valore). La chiamata a `dbg!` alla riga 14 di
_src/main.rs_ stampa il valore di `&rettangolo1`, che è la _struct_
`Rettangolo`. Questo output usa la formattazione `Debug` "pretty" del tipo
`Rettangolo`. La macro `dbg!` può essere davvero utile quando stai cercando di
capire cosa sta facendo il tuo codice!

Oltre al _trait_ `Debug`, Rust fornisce diversi _trait_ che possiamo usare con
l'attributo `derive` che possono aggiungere comportamenti utili ai nostri _type_
personalizzati. Quei _trait_ e i loro comportamenti sono elencati
nell'[Appendice C][app-c]<!-- ignore -->. Tratteremo come implementare questi
_trait_ con un comportamento personalizzato e come creare i propri _trait_ nel
Capitolo 10. Ci sono anche molti attributi oltre a `derive`; per maggiori
informazioni, vedi la sezione [“Attributes” del Rust Reference][attributes].

La nostra funzione `area` è molto specifica: calcola solo l'area dei rettangoli.
Sarebbe utile legare questo comportamento più strettamente alla nostra _struct_
`Rettangolo` perché non funzionerà con altri _type_. Vediamo come possiamo
continuare a riscrivere questo codice trasformando la funzione `area` in un
_metodo_ (_method_) definito sul nostro _type_ `Rettangolo`.

[the-tuple-type]: ch03-02-data-types.html#il-type-tupla
[app-c]: appendix-03-derivable-traits.md
[println]: https://doc.rust-lang.org/stable/std/macro.println.html
[dbg]: https://doc.rust-lang.org/stable/std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.html
[attributes]: https://doc.rust-lang.org/stable/reference/attributes.html
