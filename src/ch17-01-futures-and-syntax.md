## _Future_ e la Sintassi _Async_

Gli elementi chiave della programmazione asincrona in Rust sono le _future_ e le
parole chiave `async` e `await` di Rust.

Una _future_ è un valore che potrebbe non essere pronto ora, ma lo diventerà in
qualche momento in futuro. (Questo stesso concetto compare in molti linguaggi, a
volte sotto altri nomi come _task_ o _promise_.) Rust fornisce un _trait_
`Future` come blocco costruttivo in modo che diverse operazioni _async_ possano
essere implementate con strutture dati diverse ma con un’interfaccia comune. In
Rust, le _future_ sono _type_ che implementano il _trait_ `Future`. Ogni
_future_ contiene le proprie informazioni sui progressi fatti e su cosa
significa essere "pronti".

Puoi applicare la parola chiave `async` a blocchi e funzioni per specificare che
possono essere interrotti e ripresi. All’interno di un blocco _async_ o di una
funzione _async_, puoi usare la parola chiave `await` per _attendere una future_
(cioè, aspettare che sia pronta). Ogni punto in cui attendi una _future_
all’interno di un blocco o funzione _async_ è un potenziale punto in cui quel
blocco o funzione _async_ può mettersi in pausa e riprendere. Il processo di
verifica con una _future_ per vedere se il suo valore è già disponibile è
chiamato _polling_.

Alcuni altri linguaggi, come C# e JavaScript, usano parole chiave `async` e
`await` per la programmazione asincrona. Se hai familiarità con questi
linguaggi, potresti notare alcune differenze significative nel modo in cui Rust
fa le cose, incluso come gestisce la sintassi. E questo è per una buona ragione,
come vedremo!

Quando scriviamo codice _async_ in Rust, usiamo la maggior parte delle volte le
parole chiave `async` e `await`. Rust le compila in codice equivalente usando il
_trait_ `Future`, proprio come compila i cicli `for` in codice equivalente
usando il _trait_ `Iterator`. Poiché Rust fornisce il _trait_ `Future`, puoi
anche implementarlo per i _type_ da te definiti quando ne hai bisogno. Molte
delle funzioni che vedremo in questo capitolo restituiscono _type_ con le
proprie implementazioni di `Future`. Torneremo alla definizione del _trait_ alla
fine del capitolo e approfondiremo come funziona, ma questi dettagli sono
sufficienti per procedere.

Tutto questo potrebbe sembrare un po' astratto, quindi scriviamo il nostro primo
programma _async_: un piccolo _web scraper_ (_estrattore info da pagine web_).
Passeremo due URL dalla riga di comando, li recupereremo contemporaneamente e
restituiremo il risultato di quello che finisce per primo. Questo esempio avrà
parecchia nuova sintassi, ma non preoccuparti, spiegheremo tutto ciò che serve
sapere man mano che procediamo.

## Il Nostro Primo Programma _Async_

Per mantenere l’attenzione di questo capitolo sull’apprendimento di _async_
piuttosto che sulla gestione di parti dell’ecosistema, abbiamo creato il _crate_
`trpl` (`trpl` è abbreviazione di "**T**he **R**ust **P**rogramming
**L**anguage"). Riesporta tutti i _type_, i _trait_ e le funzioni di cui avrai
bisogno, principalmente dai _crate_ [`futures`][futures-crate]<!-- ignore --> e
[`tokio`][tokio]<!-- ignore -->. Il _crate_ `futures` è la sede ufficiale per la
sperimentazione Rust del codice _async_, ed è in realtà dove il _trait_ `Future`
è stato originariamente progettato. _Tokio_ è il _runtime_ _async_ più
utilizzato in Rust oggi, specialmente per applicazioni web. Ci sono altri ottimi
_runtime_ là fuori, e potrebbero essere più adatti ai tuoi scopi. Usiamo il
_crate_ `tokio` come base per `trpl` perché è ben testato e ampiamente
utilizzato.

In alcuni casi, `trpl` rinomina o incapsula le API originali per mantenerti
concentrato sui dettagli rilevanti per questo capitolo. Se vuoi capire cosa fa
il _crate_, ti incoraggiamo a controllare [il suo codice
sorgente][crate-source]<!-- ignore -->. Sarai in grado di vedere da quale
_crate_ proviene ogni riesportazione, e abbiamo lasciato commenti esaurienti che
spiegano cosa fa il _crate_.

Crea un nuovo progetto binario chiamato `hello-async` e aggiungi il _crate_
`trpl` come dipendenza:

```console
$ cargo new hello-async
$ cd hello-async
$ cargo add trpl
```

Ora possiamo usare i vari pezzi forniti da `trpl` per scrivere il nostro primo
programma _async_. Costruiremo un piccolo strumento da riga di comando che
recupera due pagine web, estrae l’elemento `<title>` da ciascuna e stampa il
titolo della pagina che completa per prima l’intero processo.

### Definire la Funzione `titolo_pagina`

Iniziamo scrivendo una funzione che prende un URL di una pagina come parametro,
la scarica e restituisce il testo dell’elemento del titolo (vedi Listato 17-1).

<Listing number="17-1" file-name="src/main.rs" caption="Definizione di una funzione asincrona per ottenere l’elemento del titolo da una pagina HTML">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-01/src/main.rs:all}}
```

</Listing>

Per prima cosa, definiamo una funzione chiamata `titolo_pagina` e la
contrassegniamo con la parola chiave `async`. Poi usiamo la funzione `trpl::get`
per recuperare l’URL passato e aggiungiamo la parola chiave `await` per
aspettare la risposta. Per ottenere il testo della risposta, chiamiamo il suo
metodo `text` e di nuovo aspettiamo con la parola chiave `await`. Entrambi
questi passaggi sono asincroni.

Per la funzione `get`, dobbiamo aspettare che il server invii la prima parte
della sua risposta, che includerà intestazioni HTTP, cookie e così via, e può
essere consegnata separatamente dal corpo della risposta. Soprattutto se il
corpo è molto grande, può volerci del tempo perché arrivi tutto. Poiché dobbiamo
aspettare l’_intera_ risposta, anche il metodo `text` è asincrono.

Dobbiamo esplicitamente attendere entrambi queste _future_, perché le _future_
in Rust sono _lazy_ (_pigre_): non fanno nulla finché non le chiedi di farlo con
la parola chiave `await`. (In effetti, Rust mostrerà un avviso del compilatore
se non usi una _future_.) Questo potrebbe ricordarti la discussione del Capitolo
13 sugli iteratori nella sezione [Elaborare una Serie di Elementi con
Iteratori][iterators-lazy]<!-- ignore -->. Gli iteratori non fanno nulla a meno
che non chiami il loro metodo `next`, sia direttamente che usando cicli `for` o
metodi come `map` che usano `next` sotto il cofano. Allo stesso modo, le
_future_ non fanno nulla a meno che tu non le chieda esplicitamente di farlo.
Questa _pigrizia_ permette a Rust di evitare di eseguire codice asincrono finché
non è effettivamente necessario.

> Nota: Questo è diverso dal comportamento che abbiamo visto nel capitolo
> precedente quando abbiamo usato `thread::spawn` in [Creare un Nuovo _Thread_
> con `spawn`][thread-spawn]<!--ignore-->, dove la chiusura passata a un altro
> _thread_ veniva eseguita immediatamente. È anche diverso da come molti altri
> linguaggi gestiscono l’asincronia. Ma è importante per Rust poter fornire le
> sue garanzie di prestazioni, proprio come accade con gli iteratori.

Una volta che abbiamo `testo_risposta`, possiamo analizzarlo in un’istanza del
_type_ `Html` usando `Html::parse`. Invece di una stringa grezza, ora abbiamo un
tipo di dato che possiamo usare per lavorare con l’HTML come una struttura dati
più funzionale. In particolare, possiamo usare il metodo `select_first` per
trovare la prima istanza di un dato selettore CSS. Passando la stringa
`"title"`, otterremo il primo elemento `<title>` nel documento, se presente.
Poiché potrebbe non esserci alcun elemento corrispondente, `select_first`
restituisce un `Option<ElementRef>`. Infine, usiamo il metodo `Option::map`, che
ci permette di lavorare sull’elemento nell’`Option` se è presente, e non fare
nulla se non lo è. (Potremmo anche usare un’espressione `match`, ma `map` è più
idiomatico.) Nel corpo della funzione che forniamo a `map`, chiamiamo
`inner_html` su `titolo` per ottenere il suo contenuto, che è una `String`. Alla
fine dei conti, abbiamo un `Option<String>`.

Nota che la parola chiave `await` di Rust va _dopo_ l’espressione che stai
attendendo, non prima. Cioè, è una parola chiave _post-fissa_. Questo potrebbe
differire da ciò a cui sei abituato se hai usato `async` in altri linguaggi, ma
in Rust rende le catene di metodi molto più gradevoli da gestire. Di
conseguenza, possiamo modificare il corpo di `titolo_pagina` per concatenare le
chiamate di funzione `trpl::get` e `text` con `await` in mezzo, come mostrato
nel Listato 17-2.

<Listing number="17-2" file-name="src/main.rs" caption="Concatenazione con la parola chiave `await`">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-02/src/main.rs:chaining}}
```

</Listing>

Con questo, abbiamo scritto con successo la nostra prima funzione asincrona!
Prima di aggiungere del codice in `main` per chiamarla, parliamo un po' di più
di cosa abbiamo scritto e cosa significa.

Quando Rust vede un blocco contrassegnato con la parola chiave `async`, lo
compila in un _type_ anonimo e univoco che implementa il _trait_ `Future`.
Quando Rust vede una funzione contrassegnata con `async`, la compila in una
funzione non asincrona il cui corpo è un blocco asincrono. Il _type_ di ritorno
di una funzione asincrona è il _type_ anonimo che il compilatore crea per quel
blocco asincrono.

Quindi, scrivere `async fn` è equivalente a scrivere una funzione che
restituisce una _future_ del _type_ di ritorno. Per il compilatore, una
definizione di funzione come `async fn titolo_pagina` nel Listato 17-1 è
equivalente a una funzione non asincrona definita in questo modo:

```rust
# extern crate trpl; // necessario per test mdbook
use std::future::Future;
use trpl::Html;

fn titolo_pagina(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let testo_risposta = trpl::get(url).await.testo_risposta().await;
        Html::parse(&testo_risposta)
            .select_first("title")
            .map(|titolo| titolo.inner_html())
    }
}
```

Analizziamo ogni parte della versione trasformata:

- Usa la sintassi `impl Trait` che abbiamo discusso nel Capitolo 10 nella
  sezione [“Usare i _Trait_ come Parametri”][impl-trait]<!-- ignore -->.
- Il _trait_ restituito è una `Future` con un _type_ associato di `Output`. Nota
  che il _type_ `Output` è `Option<String>`, che è lo stesso _type_ di ritorno
  della versione `async fn` di `titolo_pagina`.
- Tutto il codice chiamato nel corpo della funzione originale è racchiuso in un
  blocco `async move`. Ricorda che i blocchi sono espressioni. Questo intero
  blocco è l’espressione restituita dalla funzione.
- Questo blocco asincrono produce un valore di _type_ `Option<String>`, come
  appena descritto. Quel valore corrisponde al _type_ `Output` nel _type_ di
  ritorno. È proprio come altri blocchi che hai visto.
- Il nuovo corpo della funzione è un blocco `async move` per come usa il
  parametro `url`. (Confronteremo molto più approfonditamente `async` e `async
  move` più avanti in questo capitolo).

Ora possiamo chiamare `titolo_pagina` in `main`.

## Determinare il Titolo di una Singola Pagina

Per iniziare, prenderemo il titolo di una singola pagina. Nel Listato 17-3,
seguiamo lo stesso schema che abbiamo usato nel Capitolo 12 per [Ricevere
Argomenti dalla Riga di Comando][cli-args]<!-- ignore -->. Poi passiamo il primo
URL a `titolo_pagina` e attendiamo il risultato. Poiché il valore prodotto dalla
_future_ è un `Option<String>`, usiamo un’espressione `match` per stampare
messaggi diversi a seconda che la pagina abbia o meno un `<title>`.

<Listing number="17-3" file-name="src/main.rs" caption="Chiamare la funzione `titolo_pagina` da `main` con un argomento fornito dall’utente">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-03/src/main.rs:main}}
```

</Listing>

Purtroppo, questo codice non si compila. L’unico posto in cui possiamo usare la
parola chiave `await` è in funzioni o blocchi _async_, e Rust non ci permette di
contrassegnare la funzione `main` speciale come `async`.

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-03
cargo build
copy just the compiler error
-->

```console
error[E0752]: `main` function is not allowed to be `async`
 --> src/main.rs:6:1
  |
6 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

Il motivo per cui `main` non può essere contrassegnata `async` è che il codice
_async_ ha bisogno di un _runtime_: un _crate_ Rust che gestisce i dettagli
dell’esecuzione del codice asincrono. La funzione `main` di un programma può
_inizializzare_ un _runtime_, ma non è un _runtime_ _in sé_. (Vedremo più avanti
perché è così.) Ogni programma Rust che esegue codice asincrono ha almeno un
punto in cui configura un _runtime_ ed esegue le _future_.

La maggior parte dei linguaggi che supportano _async_ includono un _runtime_, ma
Rust no. Invece, ci sono molti _runtime_ asincroni disponibili, ognuno dei quali
fa compromessi diversi adatti al caso d’uso che intende coprire. Ad esempio, un
server web che gestisce grandi quantità di dati eseguito su CPU multi-core e una
grande quantità di RAM ha esigenze molto diverse da un micro-controllore con un
singolo core, poca RAM e nessuna capacità di allocazione nell’_heap_. I _crate_
che forniscono questi _runtime_ spesso forniscono anche versioni _async_ di
funzionalità comuni come I/O su file o di rete.

Qui, e nel resto di questo capitolo, useremo la funzione `run` del _crate_
`trpl`, che prende una _future_ come argomento e la esegue fino al
completamento. Dietro le quinte, chiamare `run` configura un _runtime_ usato per
eseguire la _future_ passata. Una volta che la _future_ è completata, `run`
restituisce qualsiasi valore che la _future_ ha prodotto.

Potremmo passare direttamente la _future_ restituita da `titolo_pagina` a `run`,
e una volta completata, potremmo fare il _match_ sul risultante
`Option<String>`, come abbiamo provato a fare nel Listato 17-3. Tuttavia, per la
maggior parte degli esempi in questo capitolo (e per la maggior parte del codice
_async_ nel mondo reale), faremo più di una singola chiamata di funzione
_async_, quindi invece passeremo un blocco `async` ed esplicitamente attendiamo
il risultato della chiamata `titolo_pagina`, come nel Listato 17-4.

<Listing number="17-4" caption="Eseguire ed attendere un blocco _async_ con `trpl::run`" file-name="src/main.rs">

<!-- should_panic,noplayground because mdbook test does not pass args -->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-04/src/main.rs:run}}
```

</Listing>

Quando eseguiamo questo codice, otteniamo il comportamento che avevamo
inizialmente previsto:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-04
cargo build # skip all the build noise
cargo run https://www.rust-lang.org
# copy the output here
-->

```console
$ cargo run -- https://www.rust-lang.org
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/async_await 'https://www.rust-lang.org'`
Il titolo per https://www.rust-lang.org era
            Rust Programming Language
```

Bene! Finalmente abbiamo del codice _async_ funzionante! Ma prima di aggiungere
il codice per mettere a gara i due siti l’uno contro l’altro, dedichiamo
brevemente la nostra attenzione a come funzionano le _future_.

Ogni _punto di attesa_ (_await point_) - cioè, ogni punto in cui il codice usa
la parola chiave `await` - rappresenta un punto in cui il controllo viene
restituito al _runtime_. Perché la cosa funzioni, Rust deve tenere traccia dello
stato nel blocco _async_ in modo che il _runtime_ possa avviare altro lavoro e
poi tornare quando è pronto per provare a far avanzare il primo. Questa è in
pratica una _macchina a stati finiti_[^msf], come se avessi scritto un _enum_
come questo per salvare lo stato corrente ad ogni punto di attesa:

```rust
{{#rustdoc_include ../listings/ch17-async-await/no-listing-state-machine/src/lib.rs:enum}}
```

Scrivere il codice per passare manualmente tra ogni stato sarebbe laborioso e
soggetto a errori, soprattutto quando è necessario aggiungere più funzionalità e
più stati al codice in seguito. Fortunatamente, il compilatore Rust crea e
gestisce automaticamente le strutture dati della macchina a stati per il codice
_async_. Tutte le normali regole di prestito e _ownership_ intorno alle
strutture dati si applicano ancora, e felicemente, il compilatore gestisce anche
la verifica di quelle per noi e fornisce messaggi di errore utili. Ne
esamineremo alcuni più avanti in questo capitolo.

Alla fine, qualcosa deve eseguire questa macchina a stati, e quella cosa è un
_runtime_. (Questo è il motivo per cui potresti imbatterti in riferimenti a
_executor_ quando cerchi informazioni sui _runtime_: un _executor_ è la parte di
un _runtime_ responsabile dell’esecuzione del codice _async_.)

Ora puoi vedere perché il compilatore ci ha impedito di rendere `main` stesso
una funzione _async_ nel Listato 17-3. Se `main` fosse una funzione _async_,
qualcos’altro dovrebbe gestire la macchina a stati per qualsiasi _future_ che
`main` restituisse, ma `main` è il punto di partenza del programma! Invece,
abbiamo chiamato la funzione `trpl::run` in `main` per configurare un _runtime_
ed eseguire la _future_ restituita dal blocco `async` fino al suo completamento.

> Nota: Alcuni _runtime_ forniscono macro in modo che tu _possa_ scrivere una
> funzione `main` _async_. Quelle macro riscrivono `async fn main() { ... }` per
> essere un normale `fn main`, che fa la stessa cosa che abbiamo fatto a mano
> nel Listato 17-4: chiamare una funzione che esegue una _future_ fino al
> completamento proprio come fa `trpl::run`.

Ora mettiamo insieme questi pezzi e vediamo come possiamo scrivere codice
concorrente.

### Mettere a Gara i Due URL l’Uno Contro l’Altro

Nel Listato 17-5, chiamiamo `titolo_pagina` con due URL diversi passati dalla
riga di comando e li mettiamo a gara.

<Listing number="17-5" caption="Creazione di due _future_ con chiamata a `titolo_pagina` per farle competere tra loro" file-name="src/main.rs">

<!-- should_panic,noplayground because mdbook does not pass args -->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-05/src/main.rs:all}}
```

</Listing>

Iniziamo chiamando `titolo_pagina` per ciascuno degli URL forniti dall’utente.
Salviamo le _future_ risultanti come `titolo_fut_1` e `titolo_fut_2`. Ricorda,
queste non fanno ancora nulla, perché le _future_ sono _lazy_ e non le abbiamo
ancora messe in coda. Poi passiamo le _future_ a `trpl::race`, che restituisce
un valore per indicare quale delle _future_ a esso passate finisce per prima.

> Nota: Sotto il cofano, `race` è costruito su una funzione più generale,
> `select`, che incontrerai più spesso nel codice Rust reale. Una funzione
> `select` può fare molte cose che la funzione `trpl::race` non può, ma ha anche
> alcune complessità aggiuntive che possiamo tralasciare per ora.

Può legittimamente “vincere” una qualsiasi delle _future_, quindi non ha senso
restituire un `Result`. Invece, `race` restituisce un _type_ che non abbiamo
ancora visto, `trpl::Either`. Il _type_ `Either` è in qualche modo simile a un
`Result` in quanto ha due casi. A differenza di `Result`, però, non c’è alcuna
nozione di successo o fallimento incorporata in `Either`. Invece, usa `Left` e
`Right` per indicare “l’uno o l’altro”:

```rust
enum Either<A, B> {
    Left(A),
    Right(B),
}
```

La funzione `race` restituisce `Left` con l’output dalla prima _future_ che
finisce, o `Right` con l’output della seconda _future_ se quella finisce per
prima. Questo corrisponde all’ordine in cui appaiono gli argomenti quando si
chiama la funzione: il primo argomento è a sinistra del secondo argomento.

Aggiorniamo anche `titolo_pagina` per restituire lo stesso URL passato. In modo
che, se la pagina che restituisce per prima non ha un `<title>` che possiamo
risolvere, possiamo comunque stampare un messaggio significativo. Con queste
informazioni disponibili, concludiamo aggiornando l’output di `println!` per
indicare sia quale URL ha finito per primo, sia qual è il `<title>`, se
presente, per la pagina web a quell’URL.

Hai costruito ora un piccolo _web scraper_ funzionante! Scegli un paio di URL ed
esegui lo strumento da riga di comando. Potresti scoprire che alcuni siti sono
costantemente più veloci di altri, mentre in altri casi il sito più veloce varia
da un’esecuzione all’altra. Cosa più importante, hai imparato le basi del lavoro
con le _future_, quindi ora possiamo approfondire cosa possiamo fare con
_async_.

[impl-trait]: ch10-02-traits.html#usare-i-trait-come-parametri
[iterators-lazy]: ch13-02-iterators.html
[thread-spawn]: ch16-01-threads.html#creare-un-nuovo-thread-con-spawn
[cli-args]: ch12-01-accepting-command-line-arguments.html
[crate-source]: https://github.com/rust-lang/book/tree/main/packages/trpl
[futures-crate]: https://crates.io/crates/futures
[tokio]: https://tokio.rs
[^msf]: [Macchina a Stati Finiti su wikipedia](https://it.wikipedia.org/wiki/Automa_a_stati_finiti)
