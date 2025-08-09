# Programmare un gioco di indovinelli

Cominciamo a programmare in Rust lavorando insieme a un progetto pratico! Questo capitolo ti introduce ad alcuni concetti comuni di Rust mostrandoti come utilizzarli in un programma reale. Imparerai a conoscere `let`, `match`, metodi, funzioni associate, _crates_ esterni e molto altro ancora! Nei capitoli successivi esploreremo queste idee in modo più dettagliato, mentre in questo capitolo ti limiterai a mettere in pratica le nozioni fondamentali.

Implementeremo un classico problema di programmazione per principianti: un gioco di indovinelli. Ecco come funziona: il programma genererà un numero intero casuale compreso tra 1 e 100. Poi chiederà al giocatore di inserire un'ipotesi. Dopo aver inserito un'ipotesi, il programma indicherà se l'ipotesi è troppo bassa o troppo alta. Se l'ipotesi è corretta, il gioco stamperà un messaggio di congratulazioni e terminerà.

## Impostazione di un nuovo progetto

Per creare un nuovo progetto, vai nella cartella _progetti_ che hai creato nel Capitolo 1 e crea un nuovo progetto con Cargo, in questo modo:

```console
$ cargo new gioco_indovinello
$ cd gioco_indovinello
```

Il primo comando, `cargo new`, prende il nome del progetto (`gioco_indovinello`) come primo argomento. Il secondo comando entra nella directory del nuovo progetto.

Diamo un'occhiata al file _Cargo.toml_ appena generato:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial
rm -rf no-listing-01-cargo-new
cargo new no-listing-01-cargo-new --name gioco_indovinello
cd no-listing-01-cargo-new |
cargo run > output.txt 2>&1
cd ../../..
-->

<span class="filename">File: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

Come hai visto nel Capitolo 1, `cargo new` genera per te un programma "Hello, world!". Guarda il file _src/main.rs_:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

Ora compiliamo questo programma "Hello, world!" ed eseguiamolo nello stesso passaggio utilizzando il comando `cargo run`:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

Il comando `run` è utile quando hai bisogno di iterare rapidamente su un progetto, come faremo in questo gioco, testando velocemente ogni modifica prima di passare alla successiva.

Riapri il file _src/main.rs_. In questo file scriverai tutto il codice.

## Elaborazione di un'ipotesi

La prima parte del programma del gioco di indovinelli richiederà l'input dell'utente, lo elaborerà e verificherà che l'input sia nella forma prevista. Per iniziare, permetteremo al giocatore di inserire un'ipotesi. Inserisci il codice del Listato 2-1 in _src/main.rs_.

<Listing number="2-1" file-name="src/main.rs" caption="Codice che riceve l'ipotesi dall'utente e la stampa">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

</Listing>

Questo codice contiene molte informazioni, quindi analizziamolo riga per riga. Per ottenere l'input dell'utente e poi stampare il risultato come output, dobbiamo utilizzare la libreria di input/output `io`. La libreria `io` proviene dalla libreria standard, nota come `std`:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

Come impostazione predefinita, Rust ha un insieme di elementi definiti nella libreria standard che vengono inseriti in ogni programma. Questo insieme è chiamato preludio (_prelude_ d'ora in poi) e puoi vedere tutto ciò che contiene [nella documentazione della libreria standard][prelude].

Se un _type_ che vuoi utilizzare non è presente nel _prelude_, devi inserirlo esplicitamente con un'istruzione `use`. L'utilizzo della libreria `std::io` ti offre una serie di utili funzioni, tra cui la possibilità di ricevere input dall'utente.

Come hai visto nel Capitolo 1, la funzione `main` è il punto di ingresso del programma:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

La sintassi `fn` dichiara una nuova funzione; le parentesi, `()`, indicano che non ci sono parametri; e la parentesi graffa, `{`, inizia il corpo della funzione.

Come hai imparato nel Capitolo 1, `println!` è una _macro_ che stampa una stringa sullo schermo:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

Questo codice stampa un messaggio che introduce il gioco e richiede un input da parte dell'utente.

### Memorizzare i valori con le Variabili

Successivamente, creeremo una _variabile_ per memorizzare l'input dell'utente, in questo modo:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

Ora il programma si fa interessante! In questa piccola riga succedono molte cose. Usiamo l'istruzione `let` per creare la variabile. Ecco un altro esempio:

```rust,ignore
let mele = 5;
```

Questa riga crea una nuova variabile di nome `mele` e la lega al valore 5. In Rust, le variabili sono immutabili (_immutable_ d'ora in poi) come impostazione predefinita, il che significa che una volta assegnato un valore alla variabile, il valore non cambierà. Parleremo di questo concetto in dettaglio nella sezione ["Variabili e mutabilità"][variables-and-mutability]<!-- ignore --> del Capitolo 3. Per rendere mutabile (_mutable_) una variabile, aggiungiamo `mut` prima del nome della variabile:

```rust,ignore
let mele = 5; // immutabile
let mut banane = 5; // mutabile
```

> Nota: la sintassi `//` inizia un commento che continua fino alla fine della riga. Rust ignora tutto ciò che è contenuto nei commenti. Parleremo dei commenti in modo più dettagliato nel [Capitolo 3][comments]<!-- ignore -->.

Torniamo al nostro gioco di indovinelli. Ora sai che `let mut ipotesi` introdurrà una variabile mutabile di nome `ipotesi`. Il segno di uguale (`=`) indica a Rust che vogliamo legare qualcosa alla variabile in quel momento. A destra del segno di uguale c'è il valore a cui `ipotesi` è legata, che è il risultato della chiamata a `String::new`, una funzione che restituisce una nuova istanza di una `String`. [`String`][string]<!-- ignore --> è un _type_ di stringa fornito dalla libreria standard che è un pezzo di testo codificato UTF-8 modificabile in lunghezza.

La sintassi `::` nella riga `::new` indica che `new` è una funzione associata al _type_ `String`. Una _funzione associata_ è una funzione implementata su un _type_, in questo caso `String`. Questa funzione `new` crea una nuova stringa vuota. Troverai una funzione `new` in molti _type_ perché è un nome comune per una funzione che crea un nuovo valore di qualche tipo.

In pratica, la linea `let mut ipotesi = String::new();` ha creato una variabile _mutable_ che è attualmente legata a una nuova istanza vuota di `String`. Wow!

### Ricevere l'input dell'utente

Ricordiamo che abbiamo incluso le funzionalità di input/output della libreria standard con `use std::io;` nella prima riga del programma. Ora chiameremo la funzione `stdin` dal modulo `io`, che ci permetterà di gestire l'input dell'utente:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

Se non avessimo importato il modulo `io` con `use std::io;` all'inizio del programma, potremmo comunque utilizzare la funzione scrivendo questa chiamata di funzione come `std::io::stdin`. La funzione `stdin` restituisce un'istanza di [`std::io::Stdin`][iostdin]<!-- ignore -->, che è un _type_ che rappresenta un _handle_ all' input standard del tuo terminale.

Successivamente, la riga `.read_line(&mut ipotesi)` chiama il metodo [`read_line`][read_line]<!-- ignore --> sull'handle di input standard per ottenere un input dall'utente. Passiamo anche `&mut ipotesi` come argomento a `read_line` per dirgli in quale stringa memorizzare l'input dell'utente. Il compito di `read_line` è quello di prendere tutto ciò che l'utente digita nell'input standard e aggiungerlo a una stringa (senza sovrascriverne il contenuto), quindi passiamo tale stringa come argomento. L'argomento stringa deve essere _mutable_ in modo che il metodo possa cambiare il contenuto della stringa.

Il simbolo `&` indica che questo argomento è un riferimento (_reference_ d'ora in poi), il che ti dà la possibilità di permettere a più parti del codice di accedere a un dato senza doverlo copiare più volte in memoria. I _reference_ sono una funzionalità complessa e uno dei principali vantaggi di Rust è la sicurezza e la facilità con cui è possibile utilizzarli. Non hai bisogno di conoscere molti di questi dettagli per finire questo programma. Per ora, tutto ciò che devi sapere è che, come le variabili, i _reference_ sono immutabili come impostazione predefinita. Di conseguenza, devi scrivere `&mut ipotesi` piuttosto che solo `&ipotesi` per renderli _mutable_ (il Capitolo 4 spiegherà i _reference_ in modo più approfondito)

<!-- Old heading. Do not remove or links may break. -->
<a id="handling-potential-failure-with-the-result-type"></a>

### Gestione dei potenziali errori con `Result`

Stiamo ancora lavorando su questa riga di codice. Ora stiamo discutendo di una terza riga di testo, ma notiamo che fa ancora parte di un'unica riga logica di codice. La prossima parte è questo metodo:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

Avremmo potuto scrivere questo codice come:

```rust,ignore
io::stdin().read_line(&mut ipotesi).expect("Errore di lettura");
```

Tuttavia, una riga lunga può essere difficile da leggere, quindi è meglio dividerla. Spesso è consigliabile andare a capo e aggiungere degli spazi bianchi per aiutare a spezzare le righe lunghe quando chiami un metodo con la sintassi `.nome_metodo()`. Ora vediamo cosa fa questa riga.

Come accennato in precedenza, `read_line` inserisce qualsiasi cosa l'utente inserisca nella stringa che gli passiamo, ma restituisce anche un valore `Result`. [`Result`][result]<!-- ignore --> è una [enumerazione][enums]<!-- ignore -->(_enum_ per brevità), che è un _type_ che può trovarsi in uno dei molteplici stati possibili. Chiamiamo ogni stato possibile una _variante_.

Il [Capitolo 6][enums]<!-- ignore --> tratterà gli _enum_ in modo più dettagliato. Lo scopo di questi _type_ `Result` è quello di fornire informazioni sulla gestione degli errori.

Le varianti di `Result` sono `Ok` e `Err`. La variante `Ok` indica che l'operazione è andata a buon fine e contiene il valore generato con successo. La variante `Err` indica che l'operazione non è andata a buon fine e contiene informazioni su come o perché l'operazione è fallita.

I valori del tipo `Result`, come i valori di qualsiasi _type_, hanno dei metodi definiti su di essi. Un'istanza di `Result` ha un metodo [`expect`][expect]<!-- ignore --> che puoi chiamare. Se questa istanza di `Result` è un valore `Err`, `expect` causerà l'arresto del programma e visualizzerà il messaggio che hai passato come argomento a `expect`. Se il metodo `read_line` restituisce un `Err`, è probabile che sia il risultato di un errore proveniente dal sistema operativo sottostante. Se questa istanza di `Result` è un valore `Ok`, `expect` prenderà il valore di ritorno che `Ok` sta tenendo e ti restituirà solo quel valore in modo che tu possa usarlo. In questo caso, quel valore è il numero di byte nell'input dell'utente.

Se non chiami `expect`, il programma verrà compilato, ma riceverai un avviso:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

Rust avverte che non è stato utilizzato il valore `Result` restituito da `read_line`, indicando che il programma non ha gestito un possibile errore.

Il modo corretto per sopprimere l'avvertimento è quello di scrivere del codice che gestisca questi potenziali errori, ma nel nostro caso non è un grosso problema mandare in crash il programma quando si verifica un problema, quindi possiamo usare `expect`. Imparerai a recuperare dagli errori nel [Capitolo 9][recover]<!-- ignore -->.

### Stampa di valori con i Segnaposto in `println!`

A parte la parentesi graffa di chiusura, c'è un'ultima riga da discutere nel codice:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

Questa riga stampa la stringa che ora contiene l'input dell'utente. La serie di parentesi graffe `{}` è un _segnaposto_: pensa a `{}` come a delle piccole chele di granchio che tengono fermo un valore. Quando stampi il valore di una variabile, il nome della variabile può essere inserito all'interno delle parentesi graffe. Quando devi stampare il risultato della valutazione di un'espressione, inserisci delle parentesi graffe vuote nella stringa di formato, quindi fai seguire alla stringa di formato un elenco separato da virgole di espressioni da stampare in ogni segnaposto vuoto, nello stesso ordine. Stampare una variabile e il risultato di un'espressione in un'unica chiamata a `println!` sarebbe così:

```rust
let x = 5;
let y = 10;

println!("x = {x} e y + 2 = {}", y + 2);
```

Questo codice produrrà `x = 5 e y + 2 = 12`.

### Proviamo la prima parte

Proviamo la prima parte del gioco di indovinelli utilizzando `cargo run`:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling gioco_indovinello v0.1.0 (file:///progetti/gioco_indovinello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/gioco_indovinello`
Indovina il numero!
Inserisci la tua ipotesi.
6
Hai ipotizzato: 6
```

A questo punto, la prima parte del gioco è terminata: stiamo ricevendo input dalla tastiera e poi li stiamo stampando.

## Generare un numero segreto

Ora dobbiamo generare un numero segreto che l'utente cercherà di indovinare. Il numero segreto dovrebbe essere diverso ogni volta, in modo da rendere il gioco divertente più di una volta. Utilizzeremo un numero casuale compreso tra 1 e 100, in modo che il gioco non sia troppo difficile. Rust non include ancora la funzionalità dei numeri casuali nella sua libreria standard, ma il team di Rust fornisce un [_crate_ `rand`][randcrate] con tale funzionalità.

### Utilizzare un _crate_ per ottenere maggiori funzionalità

Ricorda che un _crate_ è una raccolta di file di codice sorgente in Rust. Il progetto che stiamo costruento è un _crate binario_, cioè un eseguibile. Il _crate_ `rand` è un _crate libreria_, che contiene codice destinato a essere utilizzato in altri programmi e non può essere eseguito da solo.

Prima di poter scrivere del codice che utilizzi `rand`, dobbiamo modificare il file _Cargo.toml_ per includere il _crate_ `rand` come dipendenza. Apri il file e aggiungi la seguente riga in fondo, sotto l'intestazione della sezione delle dipendenze `[dependencies]` che Cargo ha creato per te. Assicurati di specificare `rand` esattamente come abbiamo fatto qui, con questo numero di versione, altrimenti gli esempi di codice di questo tutorial potrebbero non funzionare:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">File: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:8:}}
```

Nel file _Cargo.toml_, tutto ciò che segue un'intestazione fa parte di quella sezione che continua fino all'inizio di un'altra sezione. In `[dependencies]` indichi a Cargo quali sono i _crate_ esterni da cui dipende il tuo progetto e quali sono le versioni di tali _crate_ richieste. In questo caso, specifichiamo il _crate_ `rand` con lo specificatore di versione semantica `0.8.5`. Cargo comprende il [Versionamento Semantico][semver]<!-- ignore --> (a volte chiamato _SemVer_ per brevità), che è uno standard per la scrittura dei numeri di versione. Lo specificatore `0.8.5` è in realtà un'abbreviazione di `^0.8.5`, che indica qualsiasi versione che sia almeno 0.8.5 ma inferiore a 0.9.0.

Cargo considera queste versioni con API pubbliche compatibili con la versione 0.8.5 e questa specifica ti garantisce di ottenere l'ultima release della patch che si compila ancora con il codice di questo capitolo. Qualsiasi versione 0.9.0 o superiore non garantisce di avere le stesse API utilizzate negli esempi seguenti.

Ora, senza modificare alcun codice, costruiamo il progetto, come mostrato nel Listato 2-2.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
rm Cargo.lock
cargo clean
cargo build -->

<Listing number="2-2" caption="L'output dall'esecuizione di `cargo build` dopo l'aggiunt a del crate rand come dipendenza">

```console
$ cargo build
  Updating crates.io index
   Locking 15 packages to latest Rust 1.85.0 compatible versions
    Adding rand v0.8.5 (available: v0.9.0)
 Compiling proc-macro2 v1.0.93
 Compiling unicode-ident v1.0.17
 Compiling libc v0.2.170
 Compiling cfg-if v1.0.0
 Compiling byteorder v1.5.0
 Compiling getrandom v0.2.15
 Compiling rand_core v0.6.4
 Compiling quote v1.0.38
 Compiling syn v2.0.98
 Compiling zerocopy-derive v0.7.35
 Compiling zerocopy v0.7.35
 Compiling ppv-lite86 v0.2.20
 Compiling rand_chacha v0.3.1
 Compiling rand v0.8.5
 Compiling gioco_indovinello v0.1.0 (file:///progetti/gioco_indovinello)
  Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.48s
```

</Listing>

Potresti vedere numeri di versione diversi (ma saranno tutti compatibili con il codice, grazie a SemVer!) e righe diverse (a seconda del sistema operativo) e le righe potrebbero essere in un ordine diverso.

Quando includiamo una dipendenza esterna, Cargo recupera le ultime versioni di tutto ciò di cui la dipendenza ha bisogno dal registro, _registry_, che è una copia dei dati di [Crates.io][cratesio]. Crates.io è il sito in cui le persone che fanno parte dell'ecosistema Rust pubblicano i loro progetti Rust open source che possono essere utilizzati da altri.

Dopo aver aggiornato il registro, Cargo controlla la sezione `[dependencies]` e scarica tutti i _crate_ elencati che non sono già stati scaricati. In questo caso, anche se abbiamo elencato solo `rand` come dipendenza, Cargo ha preso anche altri _crate_ da cui `rand` dipende per funzionare. Dopo aver scaricato i _crate_, Rust li compila e poi compila il progetto con le dipendenze disponibili.

Se esegui immediatamente `cargo build` di nuovo senza apportare alcuna modifica, non otterrai alcun risultato a parte la riga `Finished`. Cargo sa che ha già scaricato e compilato le dipendenze e che non hai modificato nulla nel tuo file _Cargo.toml_. Cargo sa anche che non hai modificato nulla del tuo codice, quindi non ricompila nemmeno quello. Non avendo nulla da fare, semplicemente termina l'esecuzione.

Se apri il file _src/main.rs_, apporti una modifica banale e poi salvi e ricostruisci, vedrai solo due righe di output:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling gioco_indovinello v0.1.0 (file:///progetti/gioco_indovinello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```

Queste righe mostrano che Cargo ricompila solo le modifiche, il file _src/main.rs_. Le dipendenze non sono cambiate, quindi Cargo sa di poter riutilizzare ciò che ha già scaricato e compilato in precedenza.

#### Garanzia di build riproducibili con il file _Cargo.lock

Cargo ha un meccanismo che ti garantisce di ricostruire lo stesso artefatto ogni volta che tu o chiunque altro costruisce il tuo codice: Cargo utilizzerà solo le versioni delle dipendenze che hai specificato fino a quando non indicherai il contrario. Per esempio, supponiamo che la prossima settimana esca la versione 0.8.6 del _crate_ `rand`, che contiene un'importante correzione di un bug, ma anche una regressione incompatibile con il tuo codice. Per gestire questo problema, Rust crea il file _Cargo.lock_ la prima volta che esegui `cargo build`, che quindi ora si trova nella directory _gioco_indovinello_.

Quando costruisci un progetto per la prima volta, Cargo calcola tutte le versioni delle dipendenze che soddisfano i criteri e le scrive nel file _Cargo.lock_. Quando costruisci il tuo progetto in futuro, Cargo vedrà che il file _Cargo.lock_ esiste e userà le versioni specificate in esso, invece di fare tutto il lavoro per trovare di nuovo le versioni. In altre parole, il tuo progetto rimarrà alla versione 0.8.5 fino a quando non effettuerai un aggiornamento esplicito, grazie al file _Cargo.lock_. Poiché il file _Cargo.lock_ è importante per la creazione di build riproducibili, spesso viene inserito nel controllo sorgente insieme al resto del codice del progetto.

#### Aggiornare un _crate_ per ottenere una nuova versione

Quando _vuoi_ aggiornare un _crate_, Cargo mette a disposizione il comando `update`, che ignorerà il file _Cargo.lock_ e troverà tutte le ultime versioni che corrispondono alle tue specifiche in _Cargo.toml_. Cargo scriverà quindi queste versioni nel file _Cargo.lock_. In questo caso, Cargo cercherà solo le versioni maggiori di 0.8.5 e minori di 0.9.0. Se il _crate_ `rand` ha rilasciato nuove versioni sia per la versione 0.8 che per la 0.9, vedrai quanto segue se eseguirai `cargo update`:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new 0.8.x version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
     Locking 1 package to latest Rust 1.85.0 compatible version
    Updating rand v0.8.5 -> v0.8.6 (available: v0.9.0)
```

Cargo ignora la versione 0.9.0. A questo punto, noterai anche un cambiamento nel tuo file _Cargo.lock_ che indica che la versione del crate `rand` che stai utilizzando è la 0.8.6. Per utilizzare la versione 0.9.0 di `rand` o qualsiasi altra versione della serie 0.9._x_, dovrai aggiornare il file _Cargo.toml_ in questo modo:

```toml
[dependencies]
rand = "0.9.0"
```

La prossima volta che eseguirai `cargo build`, Cargo aggiornerà il registro dei _crate_ disponibili e rivaluterà i requisiti di `rand` in base alla nuova versione che hai specificato.

C'è molto altro da dire su [Cargo][doccargo]<!-- ignore --> e sul [suo ecosistema][doccratesio]<!-- ignore -->, di cui parleremo nel Capitolo 14, ma per ora questo è tutto ciò che devi sapere. Cargo rende molto facile il riutilizzo delle librerie, per cui i Rustaceani sono in grado di scrivere progetti più piccoli che sono assemblati da una serie di pacchetti.

### Generare un numero casuale

Iniziamo a usare `rand` per generare un numero da indovinare. Il passo successivo è aggiornare _src/main.rs_, come mostrato nel Listato 2-3.

<Listing number="2-3" file-name="src/main.rs" caption="Aggiunta del codice per generare un numero casuale">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

</Listing>

Per prima cosa aggiungiamo la riga `use rand::Rng;`. Il _trait_ `Rng` definisce i metodi che i generatori di numeri casuali implementano e questo _trait_ deve essere campo di applicazione (in _scope_ d'ora in poi), per poter utilizzare tali metodi. Il Capitolo 10 tratterà in dettaglio i _trait_.

Nella prima riga, chiamiamo la funzione `rand::thread_rng` che ci fornisce il particolare generatore di numeri casuali che utilizzeremo: un generatore locale che si appoggia al sistema operativo. Poi chiamiamo il metodo `gen_range` sul generatore di numeri casuali. Questo metodo è definito dal _trait_ `Rng` che abbiamo portato in _scope_ con l'istruzione `use rand::Rng;`. Il metodo `gen_range` prende un'espressione di intervallo come argomento e genera un numero casuale nell'intervallo. Il tipo di espressione di intervallo che stiamo usando qui ha la forma `inizio..=fine` ed è inclusivo dei limiti inferiore e superiore, quindi dobbiamo specificare `1..=100` per richiedere un numero compreso tra 1 e 100.

> Nota: non sarai sempre a conoscenza di quali _trait_ utilizzare e quali metodi e funzioni chiamare di un _crate_, quindi ogni _crate_ ha una documentazione con le istruzioni per utilizzarlo. Un'altra caratteristica interessante di Cargo è che eseguendo il comando `cargo doc --open`, la documentazione fornita da tutte le tue dipendenze viene creata localmente e aperta nel browser. Se sei interessato ad altre funzionalità del _crate_ `rand`, ad esempio, esegui `cargo doc --open` e clicca su `rand` nella barra laterale a sinistra.

La seconda nuova riga stampa il numero segreto. Questo è utile durante lo sviluppo del programma per poterlo testare, ma lo elimineremo dalla versione finale. Non è un grande gioco se il programma stampa la risposta non appena inizia!

Prova a eseguire il programma alcune volte:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling gioco_indovinello v0.1.0 (file:///progetti/gioco_indovinello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/gioco_indovinello`
Indovina il numero!
Il numero segreto è: 7
Inserisci la tua ipotesi.
4
Hai ipotizzato: 4

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/gioco_indovinello`
Indovina il numero!
Il numero segreto è: 83
Inserisci la tua ipotesi.
5
Hai ipotizzato: 5
```

Dovresti ottenere diversi numeri casuali, tutti compresi tra 1 e 100. Ottimo lavoro!

## Confrontare l'ipotesi con il numero segreto

Ora che abbiamo l'input dell'utente e un numero casuale, possiamo confrontarli. Questo passo è mostrato nel Listato 2-4. Nota che questo codice non è compilabile per il momento, come spiegheremo.

<Listing number="2-4" file-name="src/main.rs" caption="Gestione Handling the possible return values of comparing two numbers">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

</Listing>

Per prima cosa aggiungiamo un'altra istruzione `use`, che porta un _type_ chiamato `std::cmp::Ordering` dalla libreria standard. Il _type_ `Ordering` è un altro _enum_ e ha le varianti `Less`, `Greater` e `Equal`. Questi sono i tre risultati possibili quando si confrontano due valori.

Poi aggiungiamo cinque nuove righe in basso che utilizzano il _type_ `Ordering`. Il metodo `cmp` confronta due valori e può essere richiamato su qualsiasi cosa possa essere confrontata. Come paramentro prende un _reference_ a qualsiasi cosa si voglia confrontare: in questo caso sta confrontando `ipotesi` con `numero_segreto`. Poi restituisce una variante dell'_enum_ `Ordering` che abbiamo portato nello _scope_ con l'istruzione `use`. Utilizziamo un'espressione [`match`][match]<!-- ignore --> per decidere cosa fare successivamente in base a quale variante di `Ordering` è stata restituita dalla chiamata a `cmp` con i valori in `ipotesi` e `numero_segreto`.

Un'espressione `match` è composta da due _rami_. Da una parte un _pattern_ su cui fare il contronto, dall'altra il codice da eseguire se il valore dato a `match` corrisponde al _pattern_. Rust prende il valore dato a `match` e lo contronta con il _ pattern_ dei vari rami, eseguendo poi il codice se corrispondono. I _pattern_ e il costrutto `match` sono potenti caratteristiche di Rust: ti permettono di esprimere una varietà di situazioni in cui il tuo codice potrebbe imbattersi e ti assicurano di gestirle tutte. Queste caratteristiche saranno trattate in dettaglio nel Capitolo 6 e nel Capitolo 19, rispettivamente.

Facciamo un esempio con l'espressione `match` che utilizziamo qui. Supponiamo che l'utente abbia ipotizzato 50 e che il numero segreto generato in modo casuale questa volta sia 38.

Quando il codice confronta 50 con 38, il metodo `cmp` restituirà `Ordering::Greater` perché 50 è maggiore di 38. L'espressione `match` ottiene il valore `Ordering::Greater` e inizia a controllare il _pattern_ di ciascun ramo. Esamina il _pattern_ del primo ramo, `Ordering::Less`, e vede che il valore `Ordering::Greater` non corrisponde a `Ordering::Less`, quindi ignora il codice in quel ramo e passa al ramo successivo. Il modello del ramo successivo è `Ordering::Greater`, che _corrisponde_ a `Ordering::Greater`! Il codice associato in quel ramo verrà eseguito e stamperà `Troppo grande!` sullo schermo. L'espressione `match`
termina dopo la prima corrispondenza riuscita, quindi non esaminerà l'ultimo ramo in questo scenario.

Tuttavia, il codice del Listato 2-4 non viene compilato. Proviamo:

<!--
The error numbers in this output should be that of the code **WITHOUT** the
anchor or snip comments
-->

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

Il messaggio di errore afferma che ci sono _mismatched types_ (_type_ non corrispondenti). Rust ha un sistema di _type_ forte e statico. Tuttavia, ha anche l'inferenza del _type_. Quando abbiamo scritto `let mut ipotesi = String::new()`, Rust è stato in grado di dedurre che `ipotesi` doveva essere un `String` e non ci ha fatto scrivere il _type_. Il `numero_segreto`, d'altra parte, è un _type_ numerico. Alcuni _type_ numerici di Rust possono avere un valore compreso tra 1 e 100: `i32`, un numero a 32 bit; `u32`, un numero a 32 bit senza segno; `i64`, un numero a 64 bit; e altri ancora. Se non diversamente specificato, Rust imposta come predefinito un `i32`, che è il _type_ di `numero_segreto` a meno che non si aggiungano informazioni sul _type_ altrove che indurrebbero Rust a dedurre un _type_ numerico differente. Il motivo dell'errore è che Rust non può confrontare una _type_ stringa e un _type_ numerico.

In definitiva, vogliamo convertire la `String` che il programma legge come input in un _type_  numerico in modo da poterlo confrontare numericamente con il numero segreto. Lo facciamo aggiungendo questa riga al corpo della funzione `main`:

<span class="filename">File: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

La riga è:

```rust,ignore
let ipotesi: u32 = ipotesi.trim().parse().expect("Inserisci un numero!");
```

Creiamo una variabile di nome `ipotesi`. Ma aspetta, il programma non ha già una variabile di nome `ipotesi`? Sì, ma Rust ci permette di _mettere in ombra_ il valore precedente di `ipotesi` con uno nuovo. Lo _Shadowing_ ci permette di riutilizzare il nome della variabile `ipotesi` invece di costringerci a creare due variabili uniche, come `ipotesi_str` e `ipotesi`, per esempio. Ne parleremo in modo più dettagliato nel [Capitolo 3][shadowing]<!-- ignore -->, ma per ora, sappi che questa funzione è spesso usata quando vuoi convertire un valore da un _type_ ad un altro.

Leghiamo questa nuova variabile all'espressione `ipotesi.trim().parse()`. L'`ipotesi` nell'espressione si riferisce alla variabile `ipotesi` originale che contiene l'input come stringa. Il metodo `trim` su un'istatnza di `String` elimina ogni spazio bianco ad inizio e fine, cosa da fare prima di convertire la stringa in `u32`, che può contenere solo dati numerici. L'utente deve premere <kbd>invio</kbd> per confermare l'input da terminale e questo aggiunge un carattere _nuova_linea_ (_newline_ d'ora in poi) alla stringa letta da `read_line`. Per esempio, se l'utente digita <kbd>5</kbd> e poi preme <kbd>invio</kbd>, `ipotesi` conterrà: `5\n`. Il carattere `\n` rappresenta _newline_. (Su Windows, premere <kbd>invio</kbd> aggiunge anche il carattere di _ritorno_a_capo_ oltre a _newline_, risultando in `\r\n`.) Il metodo `trim` elimina sia `\n` che `\r\n`, restituendo quindi solo `5`.

Il metodo [`parse` sulle stringhe][parse]<!-- ignore --> converte una stringa in un altro _type_. In questo caso, lo usiamo per convertire una stringa in un numero. Dobbiamo indicare a Rust il tipo esatto di numero che vogliamo usando `let ipotesi: u32`. I due punti (`:`) dopo `ipotesi` dicono a Rust che annoteremo il tipo di variabile. Rust ha alcuni _type_ numerici incorporati; `u32` visto qui è un intero a 32 bit senza segno. È una buona scelta predefinita per un piccolo numero positivo. Imparerai a conoscere altri _type_ numerici [Capitolo 3][integers]<!-- ignore -->.

Inoltre, l'annotazione `u32` in questo programma di esempio e il confronto con `numero_segreto` significa che Rust dedurrà che anche `numero_segreto` dovrebbe essere un `u32`. Quindi ora il confronto sarà tra due valori con lo stesso _type_!

Il metodo `parse` funziona solo su caratteri che possono essere convertiti logicamente in numeri e quindi può facilmente causare errori. Se, ad esempio, la stringa contenesse `A👍%`, non ci sarebbe modo di convertirla in un numero. Poiché potrebbe fallire, il metodo `parse` restituisce un _type_ `Result`, proprio come fa il metodo `read_line` (discusso in precedenza in [“Gestione dei potenziali errori con `Result`”](#gestione-dei-potenziali-errori-con-result)<!-- ignore-->). Tratteremo questo `Result` allo stesso modo utilizzando nuovamente il metodo `expect`. Se `parse` restituisce una variante `Err` perché non è riuscito a creare un numero dalla stringa, la chiamata `expect` causerà il crash del gioco e stamperà il messaggio che gli abbiamo fornito. Se `parse` riesce a convertire la stringa in un numero, restituirà la variante
`Ok` di `Result` e `expect` restituirà il numero che vogliamo dal valore `Ok`.

Ora eseguiamo il programma:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
touch src/main.rs
cargo run
  76
-->

```console
$ cargo run
   Compiling gioco_indovinello v0.1.0 (file:///progetti/gioco_indovinello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/gioco_indovinello`
Indovina il numero!
Il numero segreto è: 58
Inserisci la tua ipotesi.
  76
Hai ipotizzato: 76
Troppo grande!
```

Bene! Anche se sono stati aggiunti degli spazi prima del numero, il programma ha capito che l'utente aveva ipotizzato 76. Esegui il programma alcune volte per verificare il diverso comportamento con diversi tipi di input: ipotizzare il numero corretto, ipotizzare un numero troppo alto e ipotizzare un numero troppo basso.

Ora la maggior parte del gioco funziona, ma l'utente può fare una sola ipotesi. Cambiamo questa situazione aggiungendo un ciclo!

## Consentire più ipotesi con la ripetizione

La parola chiave `loop` (ndt: _ripetere_) crea un ciclo infinito. Aggiungeremo un ciclo per dare agli utenti più possibilità di indovinare il numero:

<span class="filename">File: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

Come puoi vedere, abbiamo spostato tutto ciò che va dalla richiesta di indovinare in poi all'interno di un ciclo. Assicurati di aggiungere degli spazi ad inizio riga per indentare correttamente il codice all'interno del ciclo ed esegui di nuovo il programma. Il programma ora chiederà sempre un'altra ipotesi, il che introduce un nuovo problema: come fa l'utente a smettere di giocare?

L'utente può sempre interrompere il programma utilizzando la scorciatoia da tastiera <kbd>ctrl</kbd>-<kbd>c</kbd>. Ma c'è un altro modo per sfuggire a questo mostro insaziabile, come accennato nella discussione su `parse` in ["Confrontare l'ipotesi con il numero segreto"](#confrontare-l-ipotesi-con-il-numero-segreto)<!-- ignore -->: se l'utente inserisce una risposta non numerica, il programma si blocca. Possiamo approfittarne per consentire all'utente di uscire, come mostrato qui:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
touch src/main.rs
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling gioco_indovinello v0.1.0 (file:///progetti/gioco_indovinello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/gioco_indovinello`
Indovina il numero!
Il numero segreto è: 59
Inserisci la tua ipotesi.
45
Hai ipotizzato: 45
Troppo piccolo!
Inserisci la tua ipotesi.
60
Hai ipotizzato: 60
Troppo grande!
Inserisci la tua ipotesi.
59
Hai ipotizzato: 59
Hai indovinato!
Inserisci la tua ipotesi.
esci

thread 'main' panicked at src/main.rs:28:47:
Inserisci un numero!: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Digitando `esci` chiude il gioco, ma come noterai, anche l'inserimento di qualsiasi altro input che non sia un numero. Questo è a dir poco subottimale: vogliamo che il gioco si fermi anche quando viene indovinato il numero corretto.

### Uscita dopo un'ipotesi corretta

Programmiamo il gioco in modo che esca quando l'utente vince, aggiungendo un'istruzione `break` (ndt: _uscita_):

<span class="filename">File: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

L'aggiunta della riga `break` dopo `Hai indovinato!` fa sì che il programma esca dal ciclo quando l'utente indovina correttamente il numero segreto. Uscire dal ciclo significa anche uscire dal programma, perché il ciclo è l'ultima parte di `main`.

### Gestione degli input non validi

Per perfezionare ulteriormente il comportamento del gioco, invece di mandare in crash il programma quando l'utente non inserisce un numero valido, facciamo in modo che il gioco ignori un valore non numerico in modo che l'utente possa continuare a indovinare. Possiamo farlo modificando la riga in cui `ipotesi` viene convertito da `String` in `u32`, come mostrato nel Listato 2-5.

<Listing number="2-5" file-name="src/main.rs" caption="Ignorare un valore non numerico e continuare a chiedere un'ipotesi anziché terminare il programma">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

</Listing>

Passiamo da una chiamata `expect` a un'espressione `match` per passare dal _crash_ su un errore alla gestione di quell'errore. Ricorda che `parse` restituisce un _type_ `Result` e `Result` è un _enum_ che ha le varianti `Ok` e `Err`. Stiamo usando un'espressione `match` qui, come abbiamo fatto con il risultato `Ordering` del metodo `cmp`.

Se `parse` riesce a trasformare la stringa in un numero, restituirà un valore `Ok` che contiene il numero risultante. Questo valore `Ok` corrisponderà allo schema del primo ramo e l'espressione `match` restituirà il valore `num` che `parse` ha prodotto e messo all'interno del valore `Ok`. Quel numero finirà proprio dove vogliamo nella nuova variabile `ipotesi` che stiamo creando.

Se `parse` non riesce a trasformare la stringa in un numero, restituirà un valore `Err` che contiene ulteriori informazioni sull'errore. Il valore `Err` non corrisponde allo schema `Ok(num)` del primo ramo `match`, ma corrisponde allo schema `Err(_)` del secondo ramo. Il trattino basso, `_`, è un valore _piglia-tutto_; in questo esempio, stiamo dicendo che va bene qualsiasi valore di `Err`, indipendentemente dalle informazioni che contene. Quindi il programma eseguirà il codice del secondo ramo, `continue`, che dice al programma di passare alla successiva iterazione del `loop` e di chiedere un'altra ipotesi. Quindi, in effetti, il programma ignora tutti gli errori che `parse` potrebbe incontrare!

Ora tutto il programma dovrebbe funzionare come previsto. Proviamo:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling gioco_indovinello v0.1.0 (file:///progetti/gioco_indovinello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/gioco_indovinello`
Indovina il numero!
Il numero segreto è: 61
Inserisci la tua ipotesi.
10
Hai ipotizzato: 10
Troppo piccolo!
Inserisci la tua ipotesi.
99
Hai ipotizzato: 99
Troppo grande!
Inserisci la tua ipotesi.
foo
Inserisci la tua ipotesi.
61
Hai ipotizzato: 61
Hai vinto!
```

Perfetto! Con un'ultima piccola modifica, finiremo il gioco di indovinelli. Ricorda che il programma continua a stampare il numero segreto. Questo funziona bene per testare il funzionamento, ma rovina il gioco. Eliminiamo il `println!` che produce il numero segreto. Il Listato 2-6 mostra il codice finale.

<Listing number="2-6" file-name="src/main.rs" caption="Codice finale del gioco completo">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

</Listing>

A questo punto, hai costruito con successo il gioco dell'indovinello: complimenti!

## Riassunto

Questo progetto è stato un modo pratico per introdurti a molti nuovi concetti di Rust: `let`, `match`, le funzioni, l'uso di _crate_ esterni e altro ancora. Nei prossimi capitoli imparerai a conoscere questi concetti in modo più dettagliato. Il Capitolo 3 tratta i concetti che la maggior parte dei linguaggi di programmazione possiede, come le variabili, i tipi di dati e le funzioni, e mostra come utilizzarli in Rust. Il Capitolo 4 esplora il controllo esclusivo, una caratteristica che rende Rust diverso dagli altri linguaggi. Il Capitolo 5 parla delle strutture e della sintassi dei metodi, mentre il Capitolo 6 spiega come funzionano gli _enum_.

[prelude]: https://doc.rust-lang.org/stable/std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#variables-and-mutability
[comments]: ch03-04-comments.html
[string]: https://doc.rust-lang.org/stable/std/string/struct.String.html
[iostdin]: https://doc.rust-lang.org/stable/std/io/struct.Stdin.html
[read_line]: https://doc.rust-lang.org/stable/std/io/struct.Stdin.html#method.read_line
[result]: https://doc.rust-lang.org/stable/std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: https://doc.rust-lang.org/stable/std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: https://semver.org/lang/it/
[cratesio]: https://crates.io/
[doccargo]: https://doc.rust-lang.org/cargo/
[doccratesio]: https://doc.rust-lang.org/cargo/reference/publishing.html
[match]: ch06-02-match.html
[shadowing]: ch03-01-variables-and-mutability.html#shadowing
[parse]: https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse
[integers]: ch03-02-data-types.html#integer-types
