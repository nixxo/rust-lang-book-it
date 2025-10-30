## Arresto Ordinato e Pulizia

Il codice nel Listato 21-20 risponde alle richieste in modo asincrono attraverso
l’uso di un _thread_ _pool_, come volevamo. Riceviamo alcuni avvisi sui campi
`workers`, `id` e `thread` che non stiamo utilizzando in modo diretto, il che ci
ricorda che non stiamo ripulendo alcunché. Quando utilizziamo il metodo meno
elegante <kbd>ctrl</kbd>-<kbd>C</kbd> per arrestare il _thread_ principale,
anche tutti gli altri _thread_ vengono immediatamente arrestati, anche se sono
nel mezzo dell’elaborazione di una richiesta.

A seguire, implementeremo il _trait_ `Drop` per chiamare `join` su ciascuno dei
_thread_ nel gruppo in modo che possano completare le richieste su cui stanno
lavorando prima della chiusura. Quindi implementeremo un modo per comunicare ai
_thread_ che devono smettere di accettare nuove richieste e chiudersi. Per
vedere questo codice in azione, modificheremo il nostro server in modo che
accetti solo due richieste prima di chiudere correttamente il suo gruppo di
_thread_.

Una cosa da notare mentre procediamo: nulla di tutto ciò influisce sulle parti
del codice che gestiscono l’esecuzione delle chiusure, quindi tutto qui sarebbe
esattamente lo stesso se utilizzassimo un _thread_ _pool_ per un _runtime_
asincrono.

### Implementare  il _Trait_ `Drop` su `ThreadPool`

Iniziamo con l’implementazione di `Drop` sul nostro _thread_ _pool_. Quando il
gruppo viene eliminato, tutti i nostri _thread_ dovrebbero unirsi per
assicurarsi di completare il loro lavoro. Il Listato 21-22 mostra un primo
tentativo di implementazione di `Drop`; questo codice non funziona ancora
perfettamente.

<Listing number="21-22" file-name=“src/lib.rs” caption="Unire ogni _thread_ quando il _thread_ _pool_ esce dallo _scope_">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-22/src/lib.rs:here}}
```

</Listing>

Per prima cosa, eseguiamo un ciclo su ciascuno dei _thread_ del gruppo
`workers`. Usiamo `&mut` per questo perché `self` è un _reference_ mutabile e
abbiamo anche bisogno di poter mutare `worker`. Per ogni `worker`, stampiamo un
messaggio che dice che questa particolare istanza di `Worker` si sta spegnendo,
quindi chiamiamo `join` sul _thread_ di quell’istanza di `Worker`. Se la
chiamata a `join` fallisce, utilizziamo `unwrap` per far andare Rust in _panic_
e procedere a uno spegnimento non ordinato.

Ecco l’errore che otteniamo quando compiliamo questo codice:

```console
{{#include ../listings/ch21-web-server/listing-21-22/output.txt}}
```

L’errore ci dice che non possiamo chiamare `join` perché abbiamo solo un
prestito mutabile di ogni `worker` e `join` assume la _ownership_ del suo
argomento. Per risolvere questo problema, dobbiamo spostare il _thread_ fuori
dall’istanza `Worker` che possiede `thread` in modo che `join` possa consumare
il _thread_. Un modo per farlo è quello di adottare lo stesso approccio che
abbiamo usato nel Listato 18-15. Se `Worker` contenesse un
`Option<thread::JoinHandle<()>>`, potremmo chiamare il metodo `take` su `Option`
per spostare il valore fuori dalla variante `Some` e lasciare una variante
`None` al suo posto. In altre parole, un `Worker` in esecuzione avrebbe una
variante `Some` in `thread` e, quando volessimo ripulire un `Worker`,
sostituiremmo `Some` con `None` in modo che il `Worker` non abbia più un
_thread_ da eseguire.

Tuttavia, l’_unico_ caso in cui ciò si verificherebbe sarebbe quando si elimina
`Worker`. In cambio, dovremmo gestire un `Option<thread::JoinHandle<()>>`
ovunque accedessimo a `worker.thread`. Il Rust idiomatico usa abbastanza spesso
`Option`, ma quando ti ritrovi a incapsulare qualcosa che sai sarà sempre
presente in un `Option` come scappatoia, è una buona idea cercare approcci
alternativi per rendere il tuo codice più pulito e meno soggetto a errori.

In questo caso, esiste un’alternativa migliore: il metodo `Vec::drain`. Accetta
un parametro di intervallo per specificare quali elementi rimuovere dal vettore
e restituisce un iteratore di tali elementi. Passando la sintassi
dell’intervallo `..` si rimuoveranno *tutti* i valori dal vettore.

Quindi, dobbiamo aggiornare l’implementazione `drop` di `ThreadPool` in questo
modo:

<Listing file-name=“src/lib.rs”>

```rust
{{#rustdoc_include ../listings/ch21-web-server/no-listing-04-update-drop-definition/src/lib.rs:here}}
```

</Listing>

Questo risolve l’errore del compilatore e non richiede altre modifiche al nostro
codice. Nota che, poiché _drop_ può essere chiamato in caso di _panic_, anche
_unwrap_ potrebbe andare in _panic_ e causare un doppio _panic_, che blocca
immediatamente il programma e interrompe qualsiasi operazione di pulizia in
corso. Questo va bene per un programma di esempio, ma non è consigliabile per il
codice di produzione.

### Segnalare ai _Thread_ di Interrompere l’Attesa di Lavori

Con tutte le modifiche apportate, il nostro codice viene compilato senza alcun
avviso. Tuttavia, la cattiva notizia è che questo codice non funziona ancora nel
modo desiderato. La chiave è la logica nelle chiusure eseguite dai _thread_
delle istanze `Worker`: al momento, chiamiamo `join`, ma questo non chiude i
_thread_, perché il `loop` che eseguono cerca continuamente lavori. Se proviamo
a cancellare il nostro `ThreadPool` con la nostra attuale implementazione di
`drop`, il _thread_ principale rimarrà bloccato per sempre, in attesa che il
primo _thread_ finisca.

Per risolvere questo problema, dovremo modificare l’implementazione di `drop` in
`ThreadPool` e poi modificare il ciclo `Worker`.

Per prima cosa, modificheremo l’implementazione di `drop` in `ThreadPool` per
eliminare esplicitamente il `mittente` prima di attendere il completamento dei
_thread_. Il Listato 21-23 mostra le modifiche apportate a `ThreadPool` per
eliminare esplicitamente il `mittente`. A differenza del _thread_, qui abbiamo
_bisogno_ di usare un `Option` per poter spostare `mittente` fuori da
`ThreadPool` con `Option::take`.

<Listing number="21-23" file-name=“src/lib.rs” caption="Eliminazione esplicita di `mittente` prima di unire i _thread_ `Worker`">

```rust,noplayground,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-23/src/lib.rs:here}}
```

</Listing>

L’eliminazione di `mittente` chiude il canale, indicando che non verranno più
inviati messaggi. Quando ciò accade, tutte le chiamate a `recv` che le istanze
`Worker` eseguono nel ciclo infinito restituiranno un errore. Nel Listato 21-24,
modifichiamo il ciclo `Worker` per uscire correttamente dal ciclo in tal caso,
il che significa che i _thread_ termineranno quando l’implementazione `drop` di
`ThreadPool` chiamerà `join` su di essi.

<Listing number="21-24" file-name=“src/lib.rs” caption="Uscita esplicita dal ciclo quando `recv` restituisce un errore">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-24/src/lib.rs:here}}
```

</Listing>

Per vedere questo codice in azione, modifichiamo `main` in modo che accetti solo
due richieste prima di chiudere correttamente il server, come mostrato nel
Listato 21-25.

<Listing number="21-25" file-name=“src/main.rs” caption="Chiusura del server dopo aver servito due richieste uscendo dal ciclo">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/listing-21-25/src/main.rs:here}}
```

</Listing>

Non vorresti che un server web reale si spegnesse dopo aver servito solo due
richieste. Questo codice dimostra semplicemente che lo spegnimento avviene
ordinatamente e la pulizia funziona correttamente.

Il metodo `take` è definito nel _trait_ `Iterator` e limita l’iterazione al
massimo ai primi due elementi. Il `ThreadPool` uscirà dallo _scope_ alla fine di
`main` e verrà eseguita l’implementazione `drop`.

Avvia il server con `cargo run` ed effettua tre richieste. La terza richiesta
dovrebbe generare un errore e nel terminale dovresti vedere un output simile a
questo:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-25
cargo run
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
third request will error because server will have shut down
copy output below
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling ciao v0.1.0 (file:///progetti/ciao)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.52s
     Running `target/debug/ciao`
Worker 0 ha un lavoro; in esecuzione.
Spegnimento.
Spegnimento worker 0
Worker 3 ha un lavoro; in esecuzione.
Worker 1 disconnesso; spegnimento.
Worker 2 disconnesso; spegnimento.
Worker 3 disconnesso; spegnimento.
Worker 0 disconnesso; spegnimento.
Spegnimento worker 1
Spegnimento worker 2
Spegnimento worker 3
```

Potresti vedere un ordine diverso degli ID `Worker` e dei messaggi stampati.
Possiamo vedere come funziona questo codice dai messaggi: le istanze `Worker` 0
e 3 hanno ricevuto le prime due richieste. Il server ha smesso di accettare
connessioni dopo la seconda connessione e l’implementazione `Drop` su
`ThreadPool` inizia l’esecuzione prima ancora che `Worker 3` inizi il suo
lavoro. L’eliminazione di `mittente` disconnette tutte le istanze `Worker` e
dice loro di chiudersi. Ciascuna istanza `Worker` stampa un messaggio quando si
disconnette, quindi il _thread_ _pool_ chiama `join` per attendere che ogni
_thread_ `Worker` finisca.

Nota un aspetto interessante di questa particolare esecuzione: il `ThreadPool`
ha eliminato il `mittente` e, prima che qualsiasi `Worker` ricevesse un errore,
abbiamo provato a unire `Worker 0`. `Worker 0` non aveva ancora ricevuto un
errore da `recv`, quindi il _thread_ principale si è bloccato, in attesa che
`Worker 0` terminasse. Nel frattempo, `Worker 3` ha ricevuto un lavoro e poi
tutti i _thread_ hanno ricevuto un errore. Quando `Worker 0` ha terminato, il
_thread_ principale ha atteso che le restanti istanze `Worker` terminassero. A
quel punto, tutte erano uscite dai loro cicli e si erano fermate.

Congratulazioni! Abbiamo completato il nostro progetto: ora abbiamo un server
web di base che utilizza un _thread_ _pool_ per rispondere in modo asincrono.
Siamo in grado di eseguire un arresto ordinato del server, che pulisce tutti i
_thread_ nel _pool_.

Ecco il codice completo come riferimento:

<Listing file-name=“src/main.rs”>

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/main.rs}}
```

</Listing>

<Listing file-name=“src/lib.rs”>

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/lib.rs}}
```

</Listing>

Potremmo fare di più qui! Se vuoi continuare a migliorare questo progetto, ecco
alcune idee:

- Aggiungi altra documentazione a `ThreadPool` e ai suoi metodi pubblici.
- Aggiungi dei test delle funzionalità della libreria.
- Modifica le chiamate a `unwrap` per una gestione degli errori più robusta.
- Utilizza `ThreadPool` per eseguire alcune attività diverse dalla gestione
  delle richieste web.
- Trova un _crate_ che gestisce _thread_ _pool_ su
  [crates.io](https://crates.io/) e implementa un server web simile utilizzando
  il _crate_. Quindi, confronta la sua API e la sua robustezza con il _thread_
  _pool_ che abbiamo implementato.

## Riepilogo

Complimenti! Sei arrivato alla fine del libro! Ti ringraziamo per averci
accompagnato in questo viaggio alla scoperta di Rust. Ora sei pronto per
implementare i tuoi progetti Rust e aiutare gli altri nei loro. Ricorda che
esiste una comunità accogliente di altri Rustacean che saranno felici di
aiutarti con qualsiasi sfida incontrerai nel tuo viaggio con Rust.
