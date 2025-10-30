## Da Server Single-_Thread_ a Server Multi-_Thread_

Al momento, il server elaborerà ogni richiesta a turno, il che significa che non
elaborerà una seconda connessione fino a quando la prima connessione non avrà
finito di essere elaborata. Se il server riceve sempre più richieste, questa
esecuzione seriale sarebbe sempre meno ottimale. Se il server riceve una
richiesta che richiede molto tempo per essere elaborata, le richieste successive
dovranno aspettare fino a quando la richiesta lunga non sarà finita, anche se le
nuove richieste potrebbero essere elaborate rapidamente. Dovremo risolvere
questo problema, ma prima osserviamo il problema in azione.

### Simulare una Richiesta Lenta

Vedremo come una richiesta che impiega molto tempo a essere processata possa
influenzare le altre richieste fatte alla nostra implementazione attuale del
server. Il Listato 21-10 implementa la gestione di una richiesta ad _/attesa_
con una risposta lenta simulata, che farà attendere il server per cinque secondi
prima di rispondere.

<Listing number="21-10" file-name="src/main.rs" caption="Simulare una richiesta lenta aspettando per 5 secondi">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-10/src/main.rs:here}}
```

</Listing>

Siamo passati da `if` a `match` ora che abbiamo tre casi. Dobbiamo
esplicitamente fare _match_ su una _slice_ di `request_line` per trovare
corrispondenza con dei valori letterali stringa; `match` non fa riferimenti e
de-referenziamenti automatici, come fa il metodo di uguaglianza.

Il primo ramo è uguale al blocco `if` del Listato 21-9. Il secondo ramo fa
_match_ di una richiesta ad _/attesa_. Quando viene ricevuta quella richiesta,
il server attende per cinque secondi prima di inviare la pagina HTML di
successo. Il terzo ramo è lo stesso del blocco `else` del Listato 21-9.

Puoi vedere quanto sia primitivo il nostro server: librerie reali gestirebbero
il riconoscimento di richieste multiple in un modo molto meno verboso!

Avvia il server usando `cargo run`. Poi, apri due finestre del browser: una per
_http://127.0.0.1:7878_ e l’altra per _http://127.0.0.1:7878/attesa_. Se
inserisci l’URI _/_ alcune volte, come prima, vedrai che risponde rapidamente.
Ma se inserisci _/attesa_ e poi carichi _/_, vedrai che _/_ aspetta fino a
quando `sleep` non ha completato l’attesa per i suoi cinque secondi completi
prima di caricarsi.

Ci sono molteplici tecniche che potremmo usare per evitare che le richieste si
accumulino dietro a una richiesta lenta, inclusa l’uso di _async_ come abbiamo
fatto nel Capitolo 17; quella che implementeremo è un _thread_ _pool_.

### Migliorare la Produttività con un _Thread_ _Pool_

Un _thread_ _pool_ è un gruppo di _thread_ generati che sono pronti e in attesa
di gestire un compito. Quando il programma riceve un nuovo compito, assegna uno
dei _thread_ nel gruppo al compito, e quel _thread_ elaborerà il compito. I
_thread_ rimanenti nel gruppo sono disponibili per gestire qualsiasi altro
compito che arrivi mentre il primo _thread_ sta elaborando. Quando il primo
_thread_ ha finito di elaborare il suo compito, viene restituito al gruppo di
_thread_ inattivi, pronto per gestire un nuovo compito. Un _thread_ _pool_ ti
permette quindi di elaborare connessioni concorrentemente, aumentando la
produttività del tuo server.

Limiteremo il numero di _thread_ nel gruppo a un numero piccolo per proteggerci
da attacchi _DoS_; se il programma creasse un nuovo _thread_ per ogni richiesta
in arrivo, qualcuno che fa 10 milioni di richieste al nostro server potrebbe
causare grossi problemi utilizzando tutte le risorse del nostro server e
bloccando l’elaborazione delle richieste fino a fermarla.

Quindi, invece di generare _thread_ illimitati, avremo un numero fisso di
_thread_ in attesa nel gruppo. Le richieste in arrivo vengono mandate al gruppo
per l’elaborazione. Il gruppo manterrà una coda di richieste in arrivo. Ogni
_thread_ del gruppo prenderà una richiesta da questa coda, la gestirà e poi
chiederà un’altra richiesta dalla coda. Con questo modello, possiamo elaborare
fino a _`N`_ richieste simultaneamente, dove _`N`_ è il numero di _thread_. Se
ogni _thread_ sta rispondendo a una richiesta a lungo termine, le richieste
successive possono ancora accumularsi nella coda, ma abbiamo aumentato il numero
di richieste a lungo termine che possiamo gestire prima di raggiungere quel
punto.

Questa tecnica è solo una delle molte maniere per migliorare la produttività di
un server web. Altre opzioni che potresti esplorare sono il modello _fork/join_,
il modello _I/O_ _async_ _a singolo_ _thread_, e il modello _I/O_ _async_
_multi_-_thread_. Se sei interessato a questo argomento, puoi leggere ed
informarti su queste ed altre soluzioni e provare a implementarle; con un
linguaggio di basso livello come Rust, tutte queste opzioni sono possibili.

Prima di iniziare a implementare un _pool_ di _thread_, parliamo prima di come
dovrebbe essere usato un _pool_. Quando stai cercando di progettare codice,
scrivere prima l’interfaccia client può aiutare a guidare il tuo design. Scrivi
l’API del codice così che sia strutturata nel modo in cui vuoi chiamarla; poi
implementa la funzionalità dentro questa struttura invece di implementare prima
la funzionalità e poi progettare l’API pubblica.

In modo simile a come abbiamo usato lo sviluppo guidato dai test nel progetto
del Capitolo 12, qui invece useremo lo sviluppo guidato dal compilatore.
Scriveremo il codice che chiama le funzioni che vogliamo, e poi guarderemo agli
errori dal compilatore per determinare cosa dovremmo cambiare dopo per far
funzionare il codice. Prima di farlo, tuttavia, esploreremo la tecnica che non
useremo come punto di partenza.

#### Generare un _Thread_ per Ogni Richiesta

Per prima cosa, esploriamo come potrebbe apparire il nostro codice se creasse un
nuovo _thread_ per ogni connessione. Come detto, questa non è la nostra
soluzione finale a causa dei problemi legati al numero illimitato di _thread_
che potrebbero essere creati, ma è un punto di partenza per avere un server
multi-_thread_ funzionante. Poi, aggiungeremo il _thread_ _pool_ come
miglioramento, e confrontare le due soluzioni sarà più facile.

Il Listato 21-11 mostra le modifiche da fare a `main` per generare un nuovo
_thread_ per gestire ogni _stream_ nel ciclo `for`.

<Listing number="21-11" file-name="src/main.rs" caption="Generare un nuovo _thread_ per ogni _stream_">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-11/src/main.rs:here}}
```

</Listing>

Come hai imparato nel Capitolo 16, `thread::spawn` creerà un nuovo _thread_ e
poi eseguirà il codice della chiusura nel nuovo _thread_. Se esegui questo
codice e carichi _/attesa_ nel tuo browser, poi _/_ in altre due schede del
browser, vedrai infatti che le richieste a _/_ non devono aspettare che
_/attesa_ finisca. Tuttavia, come abbiamo menzionato, questo alla fine
sovraccaricherà il sistema perché staresti creando nuovi _thread_ senza alcun
limite.

Potresti anche ricordare dal Capitolo 17 che questa è esattamente la situazione
in cui _async_ e _await_ eccellono! Tieni a mente questo mentre costruiamo il
_thread_ _pool_ e pensa a come le cose sarebbero diverse o uguali con _async_.

#### Creare un Numero Finito di _Thread_

Vogliamo che il nostro _thread_ _pool_ funzioni in un modo simile e familiare in
modo che passare dai _thread_ a un _thread_ _pool_ non richieda grandi
cambiamenti al codice che usa la nostra API. Il Listato 21-12 mostra
l’interfaccia ipotetica per una struct `ThreadPool` che vogliamo usare invece di
`thread::spawn`.

<Listing number="21-12" file-name="src/main.rs" caption="La nostra interfaccia ideale per `ThreadPool`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-12/src/main.rs:here}}
```

</Listing>

Usiamo `ThreadPool::new` per creare un nuovo gruppo di _thread_ con un numero
configurabile di _thread_, in questo caso 4. Poi, nel ciclo `for`,
`pool.execute` ha un interfaccia simile a `thread::spawn` nel senso che prende
una chiusura che il gruppo dovrebbe eseguire per ogni _stream_. Dobbiamo
implementare `pool.execute` in modo che prenda la chiusura e la dia a un
_thread_ nel gruppo per eseguirla. Questo codice non si compilerà ancora, ma lo
proveremo in modo che il compilatore ci guidi su come sistemarlo.

#### Costruire `ThreadPool` Usando lo Sviluppo Guidato dal Compilatore

Apporta le modifiche riportate nel Listato 21-12 a _src/main.rs_, quindi
utilizziamo gli errori del compilatore da `cargo check` per guidare il nostro
sviluppo. Ecco il primo errore che otteniamo:

```console
{{#include ../listings/ch21-web-server/listing-21-12/output.txt}}
```

Ottimo! Questo errore ci dice che abbiamo bisogno di un _type_ o modulo
`ThreadPool`, quindi ne creeremo uno adesso. La nostra implementazione di
`ThreadPool` sarà indipendente dal tipo di lavoro svolto dal nostro server web.
Quindi, trasformiamo il _crate_ `ciao` da un _crate_ binario a un _crate_
libreria per contenere la nostra implementazione di `ThreadPool`. Dopo essere
passati a un _crate_ libreria, potremmo anche utilizzare la libreria del _thread
pool_ separata per qualsiasi lavoro che vogliamo svolgere utilizzando un gruppo
di _thread_, non solo per eseguire richieste web.

Creiamo un file _src/lib.rs_ che contenga quanto segue, ovvero la definizione
più semplice di una _struct_ `ThreadPool` che possiamo avere per ora:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/lib.rs}}
```

</Listing>

Quindi, modifichiamo il file _main.rs_ per portare `ThreadPool` nello _scope_
dal _crate_ della libreria aggiungendo il seguente codice all’inizio di
_src/main.rs_:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/main.rs:here}}
```

</Listing>

Questo codice ancora non funzionerà, ma controlliamolo di nuovo per ottenere il
prossimo errore che dobbiamo risolvere:

```console
{{#include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/output.txt}}
```

Questo errore indica che ora dobbiamo creare una funzione associata denominata
`new` per `ThreadPool`. Sappiamo anche che `new` deve avere un parametro che può
accettare `4` come argomento e dovrebbe restituire un’istanza `ThreadPool`.
Implementiamo la funzione `new` più semplice che abbia queste caratteristiche:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/src/lib.rs}}
```

</Listing>

Abbiamo scelto `usize` come _type_ del parametro `dimensione` perché sappiamo
che un numero negativo di _thread_ non ha alcun senso. Sappiamo anche che
useremo questo `4` come numero di elementi in una collezione di _thread_, che è
lo scopo del _type_ `usize`, come discusso nella sezione [“Il _Type_
Intero”][integer-types]<!-- ignore -->del Capitolo 3.

Controlliamo nuovamente il codice:

```console
{{#include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/output.txt}}
```

Ora l’errore si verifica perché non abbiamo un metodo `execute` su `ThreadPool`.
Ricorda dalla sezione [“Creare un Numero Finito di
_Thread_”](#creare-un-numero-finito-di-thread) che abbiamo deciso che il nostro
_thread_ _pool_ dovrebbe avere un’interfaccia simile a `thread::spawn`. In
aggiunta, implementeremo la funzione `execute` in modo che prenda la chiusura
che riceve e la dia a un thread inattivo nel gruppo per eseguirla.

Definiremo il metodo `execute` su `ThreadPool` in modo che accetti una chiusura
come parametro. Ricorda dalla sezione [“Restituire i Valori Catturati dalle
Chiusure”][moving-out-of-closures]<!-- ignore --> del Capitolo 13 che possiamo
accettare chiusure come parametri con tre diversi _trait_: `Fn`, `FnMut` e
`FnOnce`. Dobbiamo decidere quale tipo di chiusura utilizzare in questo caso.
Sappiamo che finiremo per fare qualcosa di simile all’implementazione della
libreria standard `thread::spawn`, quindi possiamo guardare quali sono i vincoli
definiti nella firma di `thread::spawn` sul suo parametro. La documentazione ci
mostra quanto segue:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

Il parametro di _type_ `F` è quello che ci interessa in questo caso; il
parametro di _type_ `T` è relativo al valore di ritorno, e non ci interessa.
Possiamo vedere che `spawn` utilizza `FnOnce` come vincolo di _trait_ su `F`.
Probabilmente è quello che vogliamo anche noi, perché alla fine passeremo
l’argomento che otteniamo in `execute` a `spawn`. Possiamo essere ulteriormente
sicuri che `FnOnce` sia il _trait_ che vogliamo utilizzare perché il _thread_
per l’esecuzione di una richiesta eseguirà solo una volta la chiusura di quella
richiesta, il che corrisponde a `Once` in `FnOnce`.

Il parametro di _type_ `F` ha anche il vincolo di _trait_ `Send` e il vincolo di
_lifetime_ `'static`, che sono utili nella nostra situazione: abbiamo bisogno di
`Send` per trasferire la chiusura da un _thread_ all’altro e di `'static` perché
non sappiamo quanto tempo impiegherà il _thread_ per eseguire quanto richiesto.
Creiamo un metodo `execute` su `ThreadPool` che prenderà un parametro generico
di _type_ `F` con questi vincoli:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-03-define-execute/src/lib.rs:here}}
```

</Listing>

Continuiamo a usare `()` dopo `FnOnce` perché questo `FnOnce` rappresenta una
chiusura che non accetta parametri e restituisce il _type_ unitario `()`.
Proprio come nelle definizioni delle funzioni, il _type_ di ritorno può essere
omesso dalla firma, ma anche se non abbiamo parametri, abbiamo comunque bisogno
delle parentesi.

Ancora una volta, questa è l’implementazione più semplice del metodo `execute`:
non fa nulla, ma stiamo solo cercando di far compilare il nostro codice.
Controlliamo di nuovo:

```console
{{#include ../listings/ch21-web-server/no-listing-03-define-execute/output.txt}}
```

Si compila! Ma nota che se provi `cargo run` e fai una richiesta nel browser,
vedrai gli errori nel browser che abbiamo visto all’inizio del capitolo. La
nostra libreria non sta ancora chiamando la chiusura passata a `execute`!

> Nota: un detto che potreste sentire riguardo ai linguaggi con compilatori
> rigorosi, come Haskell e Rust, è “Se il codice si compila, funziona”. Ma
> questo detto non è universalmente vero. Il nostro progetto si compila, ma non
> fa assolutamente nulla! Se stessimo realizzando un progetto reale e completo,
> questo sarebbe un buon momento per iniziare a scrivere dei test unitari per
> verificare che il codice si compili _e_ abbia il comportamento che
> desideriamo.

Una considerazione: cosa cambierebbe qui se eseguissimo una _future_ invece di
una chiusura?

#### Validare il Numero di _Thread_ in `new`

Non stiamo facendo nulla con i parametri di `new` e `execute`. Implementiamo il
corpo di queste funzioni con il comportamento desiderato. Per iniziare, pensiamo
a `new`. In precedenza abbiamo scelto un _type_ senza segno per il parametro
`dimensione` perché un gruppo con un numero negativo di _thread_ non ha senso.
Tuttavia, anche un gruppo con zero _thread_ non ha senso, ma zero è un `usize`
perfettamente valido. Aggiungeremo del codice per verificare che `dimensione`
sia maggiore di zero prima di restituire un’istanza `ThreadPool` e faremo andare
in _panic_ il programma se riceve uno zero utilizzando la macro `assert!`, come
mostrato nel Listato 21-13.

<Listing number="21-13" file-name=“src/lib.rs” caption="Implementazione di `ThreadPool::new` per generare un errore se `dimensione` è zero">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-13/src/lib.rs:here}}
```

</Listing>

Abbiamo anche aggiunto della documentazione per il nostro `ThreadPool` con
commenti di documentazione. Nota che abbiamo seguito le buone pratiche di
documentazione aggiungendo una sezione che evidenzia le situazioni in cui la
nostra funzione può generare un _panic_, come discusso nel Capitolo 14. Prova ad
eseguire `cargo doc --open` e clicca sulla _struct_ `ThreadPool` per vedere come
appare la documentazione generata per `new`!

Invece di aggiungere la macro `assert!` come abbiamo fatto qui, potremmo
cambiare `new` in `build` e restituire un `Result` come abbiamo fatto con
`Config::build` nel progetto I/O nel Listato 12-9. Ma in questo caso abbiamo
deciso che cercare di creare un _thread_ _pool_ senza alcun _thread_ dovrebbe
essere un errore irrecuperabile. Se ti senti ambizioso, prova a scrivere una
funzione chiamata `build` con la seguente firma per confrontarla con la funzione
`new`:

```rust,ignore
pub fn build(size: usize) -> Result<ThreadPool, ErroreCreazionePool>
```

#### Creare Spazio per Memorizzare i _Thread_

Ora che abbiamo un modo per sapere che abbiamo un numero valido di _thread_ da
memorizzare nel gruppo, possiamo creare quei _thread_ e memorizzarli nella
_struct_ `ThreadPool` prima di restituire la _struct_. Ma come si “memorizza” un
_thread_? Diamo un’altra occhiata alla firma di `thread::spawn`:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

La funzione `spawn` restituisce un `JoinHandle<T>`, dove `T` è il _type_ che la
chiusura restituisce. Proviamo anche noi a usare `JoinHandle` e vediamo cosa
succede. Nel nostro caso, le chiusure che stiamo passando al _thread_ _pool_
gestiranno la connessione e non restituiranno nulla, quindi `T` sarà il _type_
unitario `()`.

Il codice nel Listato 21-14 verrà compilato, ma non creerà ancora alcun
_thread_. Abbiamo modificato la definizione di `ThreadPool` per contenere un
vettore di istanze `thread::JoinHandle<()>`, inizializzato il vettore con una
capacità di `dimensione`, impostato un ciclo `for` che eseguirà del codice per
creare i _thread_ e restituito un’istanza `ThreadPool` che li contiene.

<Listing number="21-14" file-name=“src/lib.rs” caption="Creazione di un vettore per `ThreadPool` per contenere i _thread_">

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-14/src/lib.rs:here}}
```

</Listing>

Abbiamo portato `std::thread` nello _scope_ della libreria _crate_ perché stiamo
utilizzando `thread::JoinHandle` come _type_ degli elementi nel vettore in
`ThreadPool`.

Una volta ricevuta una dimensione valida, il nostro `ThreadPool` crea un nuovo
vettore in grado di contenere `dimensione` elementi. La funzione `with_capacity`
svolge lo stesso compito di `Vec::new`, ma con un’importante differenza:
pre-alloca lo spazio nel vettore. Poiché sappiamo che dobbiamo memorizzare
`dimensione` elementi nel vettore, eseguire questa allocazione in anticipo è
leggermente più efficiente rispetto all’utilizzo di `Vec::new`, che ridimensiona
se stesso man mano che vengono inseriti gli elementi.

Quando esegui nuovamente `cargo check`, dovrebbe avere esito positivo.

#### Inviare Codice da `ThreadPool` a un _Thread_

Abbiamo lasciato un commento nel ciclo `for` nel Listato 21-14 riguardo alla
creazione di _thread_. Qui vedremo come creare effettivamente i _thread_. La
libreria standard fornisce `thread::spawn` come metodo per creare _thread_, e
`thread::spawn` si aspetta di ricevere del codice che il _thread_ deve eseguire
non appena viene creato. Tuttavia, nel nostro caso, vogliamo creare i _thread_ e
farli _attendere_ il codice che invieremo in seguito. L’implementazione dei
_thread_ della libreria standard non include alcun modo per farlo; dobbiamo
implementarlo manualmente.

Implementeremo questo comportamento introducendo una nuova struttura dati
intermedia tra `ThreadPool` e i _thread_ che gestirà questo nuovo comportamento.
Chiameremo questa struttura dati _Worker_, che è un termine comune nelle
implementazioni di _pooling_. Il `Worker` raccoglie il codice che deve essere
eseguito ed esegue il codice nel suo _thread_.

Pensate alle persone che lavorano in cucina in un ristorante: i lavoratori
aspettano che arrivino le ordinazioni dai clienti, quindi sono responsabili di
prendere quelle ordinazioni e soddisfarle.

Invece di memorizzare un vettore di istanze `JoinHandle<()>` nel _thread_
_pool_, memorizzeremo le istanze della _struct_ `Worker`. Ogni `Worker`
memorizzerà una singola istanza `JoinHandle<()>`. Quindi, implementeremo un
metodo su `Worker` che prenderà una chiusura di codice da eseguire e la invierà
al _thread_ già in esecuzione per l’esecuzione. Assegneremo anche a ciascun
`Worker` un `id` in modo da poter distinguere tra le diverse istanze di `Worker`
nel gruppo quando facciamo _logging_ o _debugging_.

Ecco il nuovo processo che avverrà quando creeremo un `ThreadPool`.
Implementeremo il codice che invia la chiusura al _thread_ dopo aver impostato
`Worker` in questo modo:

1. Definire una struttura `Worker` che contenga un `id` e un `JoinHandle<()>`.
1. Modificare `ThreadPool` in modo che contenga un vettore di istanze `Worker`.
1. Definire una funzione `Worker::new` che accetta un numero `id` e restituisce
   un’istanza `Worker` che contiene l’`id` e un _thread_ generato con una
   chiusura vuota.
1. In `ThreadPool::new`, utilizzare il contatore del ciclo `for` per generare un
   `id`, creare un nuovo `Worker` con quell’`id` e memorizzare il `Worker` nel
   vettore.

Se sei pronto per una sfida, prova a implementare queste modifiche da solo prima
di guardare il codice nel Listato 21-15.

Pronto? Ecco il Listato 21-15 con un modo per apportare le modifiche precedenti.

<Listing number="21-15" file-name=“src/lib.rs” caption="Modifica di `ThreadPool` per contenere istanze `Worker` invece di contenere direttamente i _thread_">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-15/src/lib.rs:here}}
```

</Listing>

Abbiamo cambiato il nome del campo su `ThreadPool` da `threads` a `workers`
perché ora contiene istanze `Worker` invece di istanze `JoinHandle<()>`. Usiamo
il contatore nel ciclo `for` come argomento per `Worker::new` e memorizziamo
ogni nuovo `Worker` nel vettore chiamato `workers`.

Il codice esterno (come il nostro server in _src/main.rs_) non ha bisogno di
conoscere i dettagli di implementazione relativi all’uso di una _struct_
`Worker` all’interno di `ThreadPool`, quindi rendiamo privata la _struct_
`Worker` e la sua funzione `new`. La funzione `Worker::new` utilizza l’`id` che
le forniamo e memorizza un’istanza `JoinHandle<()>` creata generando un nuovo
_thread_ utilizzando una chiusura vuota.

> Nota: se il sistema operativo non è in grado di creare un _thread_ perché non
> ci sono risorse di sistema sufficienti, `thread::spawn` andrà in _panic_. Ciò
> causerà il _panic_ dell’intero server, anche se la creazione di alcuni
> _thread_ potrebbe avere esito positivo. Per semplicità, questo comportamento
> va bene, ma in un’implementazione di _thread_ _pool_ di produzione,
> probabilmente si preferirà utilizzare
>[`std::thread::Builder`][builder]<!-- > ignore --> e il suo
>[`spawn`][builder-spawn]<!-- ignore --> che restituisce invece `Result`.

Questo codice verrà compilato e memorizzerà il numero di istanze `Worker` che
abbiamo specificato come argomento di `ThreadPool::new`. Ma non stiamo ancora
elaborando la chiusura che otteniamo in `execute`. Vediamo come farlo.

#### Inviare Richieste ai _Thread_ Tramite Canali

Il prossimo problema che affronteremo è che le chiusure fornite a
`thread::spawn` non fanno assolutamente nulla. Attualmente, otteniamo la
chiusura che vogliamo eseguire nel metodo `execute`. Ma dobbiamo fornire a
`thread::spawn` una chiusura da eseguire quando creiamo ogni `Worker` durante la
creazione del `ThreadPool`.

Vogliamo che le _struct_ `Worker` che abbiamo appena creato recuperino il codice
da eseguire da una coda contenuta nel `ThreadPool` e lo inviino al proprio
_thread_ per l’esecuzione.

I canali che abbiamo imparato a conoscere nel Capitolo 16, un modo semplice per
comunicare tra due _thread_, sarebbero perfetti per questo caso d’uso. Useremo
un canale da far funzionare come coda di lavori, e `execute` invierà un lavoro
dal `ThreadPool` alle istanze `Worker`, che invieranno il lavoro al proprio
_thread_. Ecco il piano:

1. Il `ThreadPool` creerà un canale e diverrà l’estremità mittente.
1. Ogni `Worker` diverrà il ricevitore.
1. Creeremo una nuova _struct_ `Job` che conterrà le chiusure che vogliamo
   inviare lungo il canale.
1. Il metodo `execute` invierà il lavoro che vuole eseguire tramite il mittente.
1. Nel suo _thread_, il `Worker` eseguirà un ciclo sul suo ricevitore ed
   eseguirà le chiusure di tutti i lavori che riceve.

Iniziamo creando un canale in `ThreadPool::new` e conservando il mittente
nell’istanza `ThreadPool`, come mostrato nel Listato 21-16. La struttura `Job`
per ora non contiene nulla, ma sarà il _type_ di elemento che invieremo nel
canale.

<Listing number="21-16" file-name=“src/lib.rs” caption="Modifica di `ThreadPool` per memorizzare il mittente di un canale che trasmette istanze `Job`">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-16/src/lib.rs:here}}
```

</Listing>

In `ThreadPool::new`, creiamo il nostro nuovo canale e facciamo in modo che il
gruppo mantenga l’estremità del mittente. Questo verrà compilato con successo.

Proviamo a passare un ricevitore del canale a ciascun `Worker` mentre il
_thread_ _pool_ crea il canale. Sappiamo che vogliamo utilizzare il ricevitore
nel _thread_ che le istanze `Worker` generano, quindi faremo riferimento al
parametro `ricevitore` nella chiusura. Il codice nel Listato 21-17 non è ancora
completamente compilabile.

<Listing number="21-17" file-name=“src/lib.rs” caption="Passaggio del ricevitore a ciascun `Worker`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-17/src/lib.rs:here}}
```

</Listing>

Abbiamo apportato alcune piccole e semplici modifiche: passiamo il ricevitore a
`Worker::new`, per poi usarlo all’interno della chiusura.

Quando proviamo a controllare questo codice, otteniamo questo errore:

```console
{{#include ../listings/ch21-web-server/listing-21-17/output.txt}}
```

Il codice sta cercando di passare `ricevitore` a più istanze `Worker`. Questo
non funzionerà, come ricorderai dal Capitolo 16: l’implementazione del canale
che Rust fornisce è multi-_produttore_, singolo _consumatore_. Ciò significa che
non possiamo semplicemente clonare l’estremità di ricezione del canale per
correggere questo codice. Inoltre, non vogliamo inviare un messaggio più volte a
più ricevitori; vogliamo un unico elenco di messaggi con più istanze `Worker` in
modo che ogni messaggio venga elaborato una sola volta.

Inoltre, rimuovere un lavoro dalla coda del canale comporta la mutazione del
`ricevitore`, quindi i _thread_ hanno bisogno di un modo sicuro per condividere
e modificare `ricevitore`; altrimenti, potremmo ottenere condizioni di
competizione (come descritto nel capitolo 16).

Ricorda i puntatori intelligenti _thread-safe_ discussi nel Capitolo 16: per
condividere la _ownership_ tra più _thread_ e consentire ai _thread_ di
modificare il valore, dobbiamo utilizzare `Arc<Mutex<T>>`. Il _type_ `Arc`
consentirà a più istanze `Worker` di possedere il ricevitore, mentre `Mutex`
garantirà che solo un `Worker` alla volta riceva un lavoro dal ricevitore. Il
Listato 21-18 mostra le modifiche che dobbiamo apportare.

<Listing number="21-18" file-name=“src/lib.rs” caption="Condivisione del ricevitore tra le istanze `Worker` utilizzando `Arc` e `Mutex`">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-18/src/lib.rs:here}}
```

</Listing>

In `ThreadPool::new`, inseriamo il ricevitore in un `Arc` e in un `Mutex`. Per
ogni nuovo `Worker`, cloniamo l’`Arc` per aumentare il conteggio dei _reference_
in modo che le istanze `Worker` possano condividere la _ownership_ del
ricevitore.

Con queste modifiche, il codice viene compilato! Ci siamo quasi!

#### Implementare il Metodo `execute`

Implementiamo infine il metodo `execute` su `ThreadPool`. Modificheremo anche
`Job` da una _struct_ a un _alias_ di _type_ per un oggetto _trait_ che contiene
il _type_ della chiusura che `execute` riceve. Come discusso nella sezione
[“Sinonimi e _Alias_ di _Type_”][type-aliases]<!-- ignore --> nel Capitolo 20,
gli _alias_ di _type_ ci consentono di abbreviare i _type_ lunghi per
facilitarne l’uso. Guarda il Listato 21-19.

<Listing number="21-19" file-name=“src/lib.rs” caption="Creazione di un _alias_ di _type_ `Job` per una `Box` che contiene ogni chiusura e quindi invio del lavoro al canale">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-19/src/lib.rs:here}}
```

</Listing>

Dopo aver creato una nuova istanza `Job` utilizzando la chiusura ottenuta in
`execute`, inviamo quel lavoro tramite l’estremità mittente del canale.
Chiamiamo `unwrap` su `send` nel caso in cui l’invio fallisca. Ciò potrebbe
accadere se, ad esempio, interrompiamo l’esecuzione di tutti i nostri _thread_,
il che significa che l’estremità ricevente ha smesso di ricevere nuovi messaggi.
Al momento, non possiamo interrompere l’esecuzione dei nostri _thread_: i nostri
_thread_ continuano a essere eseguiti finché esiste il _pool_. Il motivo per cui
utilizziamo `unwrap` è che sappiamo che il caso di errore non si verificherà, ma
il compilatore non lo sa.

Ma non abbiamo ancora finito! Nel `Worker`, la nostra chiusura passata a
`thread::spawn` continua a _fare riferimento_ solo all’estremità ricevente del
canale. Invece, abbiamo bisogno che la chiusura continui a girare all’infinito,
chiedendo all’estremità ricevente del canale un lavoro ed eseguendolo quando lo
riceve. Apportiamo la modifica mostrata nel Listato 21-20 a `Worker::new`.

<Listing number="21-20" file-name=“src/lib.rs” caption="Ricezione ed esecuzione dei lavori nel _thread_ dell’istanza `Worker`">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-20/src/lib.rs:here}}
```

</Listing>

Qui, chiamiamo prima `lock` sul `ricevitore` per acquisire il _mutex_, quindi
chiamiamo `unwrap` per generare un _panic_ in caso di errori. L’acquisizione di
un blocco potrebbe non riuscire se il _mutex_ è in uno stato _poisoned_
(_avvelenato_), cosa che può accadere se un altro _thread_ va in _panic_ mentre
mantiene il blocco invece di rilasciarlo. In questa situazione, chiamare
`unwrap` per far andare in _panic_ questo _thread_ è l’azione corretta da
intraprendere. Potresti anche cambiare questo `unwrap` in un `expect` con un
messaggio di errore che sia significativo per te.

Se otteniamo il blocco sul _mutex_, chiamiamo `recv` per ricevere un `Job` dal
canale. Un ultimo `unwrap` supera anche qui eventuali errori, che potrebbero
verificarsi se il _thread_ che detiene il mittente si è arrestato, in modo
simile a come il metodo `send` restituisce `Err` se il ricevente si arresta.

La chiamata a `recv` blocca, quindi se non ci sono ancora lavori, il _thread_
corrente attenderà fino a quando non sarà disponibile un lavoro. Il `Mutex<T>`
assicura che solo un _thread_ `Worker` alla volta tenti di richiedere un lavoro.

Il nostro gruppo di _thread_ è ora in uno stato funzionante! Esegui `cargo run`
e fai alcune richieste:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-20
cargo run
make some requests to 127.0.0.1:7878
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling ciao v0.1.0 (file:///progetti/ciao)
warning: field `workers` is never read    
 --> src/lib.rs:7:5
  |
6 | pub struct ThreadPool {
  |            ---------- field in this struct
7 |     workers: Vec<Worker>,
  |     ^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: fields `id` and `thread` are never read
  --> src/lib.rs:48:5
   |
47 | struct Worker {
   |        ------ fields in this struct
48 |     id: usize,
   |     ^^
49 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^

warning: `ciao` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.57s
     Running `target/debug/ciao`
Worker 1 ha un lavoro; in esecuzione.
Worker 1 ha un lavoro; in esecuzione.
Worker 0 ha un lavoro; in esecuzione.
Worker 3 ha un lavoro; in esecuzione.
Worker 2 ha un lavoro; in esecuzione.
Worker 1 ha un lavoro; in esecuzione.
```

Successo! Ora abbiamo un _thread_ _pool_ che esegue le connessioni in modo
asincrono. Non vengono mai creati più di quattro _thread_, quindi il nostro
sistema non andrà in sovraccarico se il server riceve molte richieste. Se
effettuiamo una richiesta a _/attesa_, il server sarà in grado di servire altre
richieste facendo in modo che un altro _thread_ le esegua.

> Nota: se apri _/attesa_ in più finestre del browser contemporaneamente,
> potrebbero caricarsi una alla volta a intervalli di cinque secondi. Alcuni
> browser web eseguono più istanze della stessa richiesta in sequenza per motivi
> di _cache_. Questa limitazione non è causata dal nostro server web.

Questo è un buon momento per fare una pausa e considerare come il codice nei
Listati 21-18, 21-19 e 21-20 sarebbe diverso se utilizzassimo le _future_ invece
di una chiusura per il lavoro da svolgere. Quali _type_ cambierebbero? In che
modo le firme dei metodi sarebbero diverse, se lo fossero? Quali parti del
codice rimarrebbero invariate?

Dopo aver appreso il funzionamento del ciclo `while let` nei capitoli 17 e 19,
potresti chiederti perché non abbiamo scritto il codice del _thread_ `Worker`
come mostrato nel Listato 21-21.

<Listing number="21-21" file-name=“src/lib.rs” caption="Un’implementazione alternativa di `Worker::new` utilizzando `while let`">

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-21/src/lib.rs:here}}
```

</Listing>

Questo codice viene compilato ed eseguito, ma non produce il comportamento
concorrente desiderato: una richiesta lenta continuerà a causare l’attesa delle
altre richieste per essere elaborate. Il motivo è piuttosto sottile: la _struct_
`Mutex` non ha un metodo pubblico `unlock` perché la _ownership_ del blocco si
basa sulla longevità di `MutexGuard<T>` all’interno del
`LockResult<MutexGuard<T>>` che il metodo `lock` restituisce. In fase di
compilazione, il controllo dei prestiti può quindi applicare la regola secondo
cui non è possibile accedere a una risorsa protetta da un `Mutex` a meno che non
si detenga il blocco. Tuttavia, questa implementazione può anche comportare il
mantenimento del blocco più a lungo del previsto se non si presta attenzione
alla _lifetime_ del `MutexGuard<T>`.

Il codice nel Listato 21-20 che utilizza `let job =
ricevitore.lock().unwrap().recv().unwrap();` funziona perché con `let`,
qualsiasi valore temporaneo utilizzato nell’espressione a destra del segno di
uguale viene immediatamente eliminato al termine dell’istruzione `let`.
Tuttavia, `while let` (e `if let` e `match`) non elimina i valori temporanei
fino alla fine del blocco associato. Nel Listato 21-21, il blocco rimane attivo
per tutta la durata della chiamata a `job()`, il che significa che altre istanze
`Worker` non possono ricevere lavori.

[type-aliases]: ch20-03-advanced-types.html#sinonimi-e-alias-di-type
[integer-types]: ch03-02-data-types.html#il-type-intero
[moving-out-of-closures]: ch13-01-closures.html#restituire-i-valori-catturati-dalle-chiusure
[builder]: https://doc.rust-lang.org/stable/std/thread/struct.Builder.html
[builder-spawn]: https://doc.rust-lang.org/stable/std/thread/struct.Builder.html#method.spawn
