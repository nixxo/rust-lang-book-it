## Accettazione degli Argomenti dalla Riga di Comando

Creiamo un nuovo progetto con, come sempre, `cargo new`. Chiameremo il nostro progetto
`minigrep` per distinguerlo dallo strumento `grep` che potresti già avere
sul tuo sistema.

```console
$ cargo new minigrep
Created binary (application) `minigrep` project
$ cd minigrep
```

Il primo compito è fare in modo che `minigrep` accetti i suoi due argomenti della riga di comando: il
percorso del file e una stringa da cercare. Cioè, vogliamo essere in grado di eseguire il nostro
programma con `cargo run`, due trattini per indicare che i seguenti argomenti sono
per il nostro programma e non per `cargo`, una stringa da cercare e un percorso a
un file in cui cercare, in questo modo:

```console
$ cargo run -- searchstring example-filename.txt
```

Al momento, il programma generato da `cargo new` non può elaborare gli argomenti che
gli forniamo. Alcune librerie esistenti su [crates.io](https://crates.io/) possono aiutare
a scrivere un programma che accetti argomenti da riga di comando, ma poiché stiamo
apprendo solo ora questo concetto, implementiamo questa funzionalità da soli.

### Lettura dei Valori degli Argomenti

Per consentire a `minigrep` di leggere i valori degli argomenti da riga di comando che gli passiamo,
avremo bisogno della funzione `std::env::args` fornita nella libreria standard di Rust. Questa funzione restituisce un iteratore degli argomenti della riga di comando passati
a `minigrep`. Tratteremo gli iteratori in dettaglio nel [Capitolo 13][ch13]<!-- ignore -->. Per ora, è necessario conoscere solo due dettagli sugli iteratori: gli iteratori
producono una serie di valori e possiamo chiamare il metodo `collect` su un iteratore
per trasformarlo in una raccolta, come un vettore, che contiene tutti gli elementi
prodotti dall'iteratore.

Il codice nel Listato 12-1 consente al programma `minigrep` di leggere qualsiasi argomento della riga di comando
passato e quindi raccogliere i valori in un vettore.

<Listing number="12-1" file-name="src/main.rs" caption="Raccolta degli argomenti della riga di comando in un vettore e loro stampa">

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-01/src/main.rs}}
```

</Listing>

Per prima cosa, portiamo il modulo `std::env` nello scope con un'istruzione `use` in modo da
poter utilizzare la sua funzione `args`. Si noti che la funzione `std::env::args` è
annidata in due livelli di moduli. Come discusso nel [Capitolo
7][ch7-idiomatic-use]<!-- ignore -->, nei casi in cui la funzione desiderata è
annidata in più di un modulo, abbiamo scelto di portare nello scope
il modulo padre anziché la funzione. In questo modo, possiamo facilmente utilizzare altre funzioni
da `std::env`. È anche meno ambiguo rispetto all'aggiunta di `use std::env::args` e
quindi alla chiamata della funzione con solo `args`, perché `args` potrebbe essere facilmente
confuso con una funzione definita nel modulo corrente.

> ### La funzione `args` e un Unicode non Valido
>
> Si noti che `std::env::args` andrà in panico se un argomento contiene Unicode non valido. Se il programma deve accettare argomenti contenenti Unicode non valido, utilizzare invece `std::env::args_os`. Questa funzione restituisce un iteratore
> che produce valori `OsString` invece di valori `String`. Abbiamo scelto di
> utilizzare `std::env::args` qui per semplicità perché i valori `OsString` variano a seconda della
> piattaforma e sono più complessi da gestire rispetto ai valori `String`.

Sulla prima riga di `main`, chiamiamo `env::args` e utilizziamo immediatamente
`collect` per trasformare l'iteratore in un vettore contenente tutti i valori prodotti
dall'iteratore. Possiamo usare la funzione `collect` per creare molti tipi di
collezioni, quindi annotiamo esplicitamente il tipo di `args` per specificare che
vogliamo un vettore di stringhe. Sebbene sia molto raro dover annotare i tipi in
Rust, `collect` è una funzione che spesso occorre annotare perché Rust
non è in grado di dedurre il tipo di collezione desiderata.

Infine, stampiamo il vettore usando la macro di debug. Proviamo a eseguire il codice
prima senza argomenti e poi con due argomenti:

```console
{{#include ../listings/ch12-an-io-project/listing-12-01/output.txt}}
```

```console
{{#include ../listings/ch12-an-io-project/output-only-01-with-args/output.txt}}
```

Nota che il primo valore nel vettore è `"target/debug/minigrep"`, che
è il nome del nostro binario. Questo corrisponde al comportamento dell'elenco degli argomenti in
C, consentendo ai programmi di utilizzare il nome con cui sono stati invocati durante l'esecuzione.
Spesso è comodo avere accesso al nome del programma nel caso in cui si voglia
visualizzarlo nei messaggi o modificarne il comportamento in base all'alias della riga di comando utilizzato per invocarlo. Ma ai fini di questo
capitolo, lo ignoreremo e salveremo solo i due argomenti di cui abbiamo bisogno.

### Salvataggio dei Valori degli Argomenti nelle Variabili

Il programma è attualmente in grado di accedere ai valori specificati come argomenti della riga di comando.
Ora dobbiamo salvare i valori dei due argomenti nelle variabili in modo da
poterli utilizzare nel resto del programma. Lo facciamo nel Listato
12-2.

<Listing number="12-2" file-name="src/main.rs" caption="Creazione di variabili per contenere l'argomento query e l'argomento percorso file">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-02/src/main.rs}}
```

</Listing>

Come abbiamo visto quando abbiamo stampato il vettore, il nome del programma occupa il primo
valore nel vettore in `args[0]`, quindi stiamo iniziando gli argomenti dall'indice 1. Il
primo argomento preso da `minigrep` è la stringa che stiamo cercando, quindi inseriamo un
riferimento al primo argomento nella variabile `query`. Il secondo argomento
sarà il percorso del file, quindi inseriamo un riferimento al secondo argomento nella
variabile `file_path`.

Stampiamo temporaneamente i valori di queste variabili per dimostrare che il codice
funziona come previsto. Eseguiamo di nuovo questo programma con gli argomenti `test`
e `sample.txt`:

```console
{{#include ../listings/ch12-an-io-project/listing-12-02/output.txt}}
```

Ottimo, il programma funziona! I valori degli argomenti di cui abbiamo bisogno vengono
salvati nelle variabili corrette. In seguito aggiungeremo una gestione degli errori per gestire
alcune potenziali situazioni errate, come quando l'utente non fornisce
argomenti; per ora ignoreremo questa situazione e lavoreremo invece sull'aggiunta di funzionalità di lettura dei file.

[ch13]: ch13-00-functional-features.html
[ch7-idiomatic-use]: ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creare-percorsi-use-idiomatici
