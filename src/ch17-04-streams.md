<!-- Old headings. Do not remove or links may break. -->
<a id="streams"></a>

## _Stream_: _Future_ in Sequenza

Fino ad ora in questo capitolo, ci siamo principalmente concentrati su _future_
singole. L'unica grande eccezione è stata il canale _async_ che abbiamo usato.
Ricorda come abbiamo utilizzato il ricevitore per il nostro canale _async_ in
precedenza in questo capitolo nella sezione [“Conteggiare su Due _Task_ Usando
il Passaggio di Messaggi”][17-02-messages]<!-- ignore -->. Il metodo _async_
`recv` produce una sequenza di elementi nel tempo. Questo è un esempio di un
modello molto più generale noto come _stream_.

Abbiamo visto una sequenza di elementi nel Capitolo 13, quando abbiamo esaminato
il _trait_ `Iterator` nella sezione [“Il _Trait_ `Iterator` e il Metodo
`next`”][iterator-trait]<!-- ignore -->, ma ci sono due differenze tra gli
iteratori e il ricevitore del canale _async_. La prima differenza è il tempo:
gli iteratori sono sincroni, mentre il ricevitore del canale è asincrono. La
seconda è l'API. Quando lavoriamo direttamente con `Iterator`, chiamiamo il suo
metodo sincrono `next`. Con lo _stream_ `trpl::Receiver`, in particolare,
abbiamo chiamato un metodo asincrono `recv`. A parte questo, le API si
somigliano molto, e questa somiglianza non è una coincidenza. Uno _stream_ è
come una forma asincrona di iterazione. Mentre il `trpl::Receiver` aspetta
specificamente di ricevere messaggi, però, l'API dello _stream_ di uso generale
è molto più ampia: fornisce il prossimo elemento come fa `Iterator`, ma in modo
asincrono.

La somiglianza tra iteratori e _stream_ in Rust significa che possiamo
effettivamente creare uno _stream_ da qualsiasi iteratore. Come con un
iteratore, possiamo lavorare con uno _stream_ chiamando il suo metodo `next` e
poi aspettare l'output, come nel Listato 17-30.

<Listing number="17-30" caption="Creare uno _stream_ da un iteratore e stampare i suoi valori" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-30/src/main.rs:stream}}
```

</Listing>

Iniziamo con un array di numeri, che convertiamo in un iteratore e poi chiamiamo
`map` su di esso per raddoppiare tutti i valori. Poi convertiamo l'iteratore in
uno _stream_ usando la funzione `trpl::stream_from_iter`. Successivamente,
iteriamo sugli elementi nello _stream_ man mano che arrivano con il ciclo `while
let`.

Sfortunatamente, quando proviamo a eseguire il codice, non si compila, ma invece
riporta che non c'è alcun metodo `next` disponibile:

```console
{{#include ../listings/ch17-async-await/listing-17-30/output.txt:2:18}}
```

Come spiega questo output, la ragione dell'errore del compilatore è che abbiamo
bisogno del _trait_ giusto in _scope_ per poter utilizzare il metodo `next`.
Dato il nostro discorso finora, potresti ragionevolmente aspettarti che quel
_trait_ sia `Stream`, ma in realtà è `StreamExt`. Abbreviazione di _estensione_,
`Ext` è un modello comune nella comunità Rust per estendere un _trait_ con un
altro.

Spiegheremo i _trait_ `Stream` e `StreamExt` in modo un po' più dettagliato alla
fine del capitolo, ma per ora tutto ciò che devi sapere è che il _trait_
`Stream` definisce un'interfaccia a basso livello che combina efficacemente i
_trait_ `Iterator` e `Future`. `StreamExt` fornisce un insieme di API di livello
superiore costruite sulla base di `Stream`, inclusi il metodo `next` e altri
metodi utili simili a quelli forniti dal _trait_ `Iterator`. `Stream` e
`StreamExt` non fanno ancora parte della libreria standard di Rust, ma la
maggior parte dei _crate_ dell'ecosistema utilizza la stessa definizione.

La soluzione all'errore del compilatore è aggiungere una dichiarazione `use` per
`trpl::StreamExt`, come nel Listato 17-31.

<Listing number="17-31" caption="Utilizzare con successo un iteratore come base per uno _stream_" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-31/src/main.rs:all}}
```

</Listing>

Con tutti questi pezzi messi insieme, questo codice funziona come vogliamo!
Inoltre, ora che abbiamo `StreamExt` in _scope_, possiamo utilizzare tutti i
suoi metodi utili, proprio come con gli iteratori. Ad esempio, nel Listato
17-32, utilizziamo il metodo `filter` per filtrare tutto tranne i multipli di
tre e cinque.

<Listing number="17-32" caption="Filtrare uno _stream_ con il metodo `StreamExt::filter`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-32/src/main.rs:all}}
```

</Listing>

Certo, questo non è molto interessante, dato che potremmo fare lo stesso con
normali iteratori e senza alcun _async_. Vediamo cosa possiamo fare che _è_
unico per gli _stream_.

### Combinare _Stream_

Molti concetti sono naturalmente rappresentati come _stream_: elementi che
diventano disponibili in una coda, porzioni di dati che vengono estratti
incrementalmente dal filesystem quando l'intero set di dati è troppo grande per
la memoria del computer, o dati che arrivano attraverso la rete nel tempo.
Poiché gli _stream_ sono _future_, possiamo usarli con qualsiasi altro tipo di
_future_ e combinarli in modi interessanti. Ad esempio, possiamo raggruppare
eventi per evitare di attivare troppe chiamate di rete, impostare _timeout_ su
sequenze di operazioni a lungo termine, o limitare gli eventi dell'interfaccia
utente per evitare di fare lavoro inutile.

Iniziamo costruendo un piccolo _stream_ di messaggi come sostituto di uno
_stream_ di dati che potremmo vedere da un WebSocket o un altro protocollo di
comunicazione in tempo reale, come mostrato nel Listato 17-33.

<Listing number="17-33" caption="Utilizza il ricevitore `rx` come `ReceiverStream`" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-33/src/main.rs:all}}
```

</Listing>

Per prima cosa, creiamo una funzione chiamata `ricevi_messaggi` che restituisce
`impl Stream<Item = String>`. Per la sua implementazione, creiamo un canale
_async_, iteriamo sulle prime 10 lettere dell'alfabeto inglese e le inviamo
attraverso il canale.

Utilizziamo anche un nuovo _type_: `ReceiverStream`, che converte il ricevitore
`rx` da `trpl::channel` in uno `Stream` con un metodo `next`. Tornando a `main`,
utilizziamo un ciclo `while let` per stampare tutti i messaggi dallo _stream_.

Quando eseguiamo questo codice, otteniamo esattamente i risultati che ci
aspetteremmo:

```console
$ cargo run
Messaggio: 'a'
Messaggio: 'b'
Messaggio: 'c'
Messaggio: 'd'
Messaggio: 'e'
Messaggio: 'f'
Messaggio: 'g'
Messaggio: 'h'
Messaggio: 'i'
Messaggio: 'j'
```

Ancora una volta, potremmo fare questo con l'API `Receiver` regolare o anche con
l'API `Iterator` regolare, quindi aggiungiamo una funzionalità che richiede
_stream_: aggiungere un _timeout_ che si applica a ogni elemento nello _stream_
e un ritardo sugli elementi che emettiamo, come mostrato nel Listato 17-34.

<Listing number="17-34" caption="Utilizzo del metodo `StreamExt::timeout` per impostare un limite di tempo sugli elementi in uno _stream_" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-34/src/main.rs:timeout}}
```

</Listing>

Iniziamo aggiungendo un _timeout_ allo _stream_ con il metodo `timeout`, che
proviene dal _trait_ `StreamExt`. Poi aggiorniamo il corpo del ciclo `while
let`, perché ora lo _stream_ restituisce un `Result`. La variante `Ok` indica
che un messaggio è arrivato in tempo; la variante `Err` indica che il _timeout_
è scaduto prima che arrivasse un messaggio. Facciamo il `match` su quel
risultato e stampiamo il messaggio quando lo riceviamo con successo o stampiamo
una notifica riguardo al _timeout_. Infine, nota che fissiamo i messaggi con
`pin!` dopo aver applicato il _timeout_, perché _timeout_ produce uno _stream_
che deve essere fissato per essere letto.

Tuttavia, poiché non ci sono ritardi tra i messaggi, questo _timeout_ non cambia
il comportamento del programma. Ora aggiungiamo un ritardo variabile ai messaggi
che inviamo, come mostrato nel Listato 17-35.

<Listing number="17-35" caption="Invio di messaggi attraverso `tx` con un ritardo _async_ senza rendere `ricevi_messaggi` una funzione _async_" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-35/src/main.rs:messages}}
```

</Listing>

In `ricevi_messaggi`, utilizziamo il metodo `enumerate` dell'iteratore con
l'array `messaggi` in modo da poter ottenere l'indice di ogni elemento che
stiamo inviando insieme all'elemento stesso. Poi applichiamo un ritardo di 100
millisecondi agli elementi con indice pari e un ritardo di 300 millisecondi agli
elementi con indice dispari per simulare i diversi ritardi che potremmo vedere
da uno _stream_ di messaggi nel mondo reale. Poiché il nostro _timeout_ è di 200
millisecondi, questo dovrebbe influenzare metà dei messaggi.

Per “dormire” tra i messaggi nella funzione `ricevi_messaggi` senza bloccare,
dobbiamo usare _async_. Tuttavia, non possiamo rendere `ricevi_messaggi` stessa
una funzione _async_, perché altrimenti restituiremmo un `Future<Output =
Stream<Item = String>>` invece di uno `Stream<Item = String>`. Il chiamante
dovrebbe attendere `ricevi_messaggi` stessa per accedere allo _stream_. Ma
ricorda: tutto in una data _future_ avviene linearmente; la concorrenza avviene
_tra_ le _future_. Attendere `ricevi_messaggi` richiederebbe che inviasse tutti
i messaggi, incluso il ritardo tra ogni messaggio, prima di restituire il
ricevitore dello _stream_. Di conseguenza, il _timeout_ sarebbe inutile. Non ci
sarebbero ritardi nello _stream_ stesso; si verificherebbero tutti ancor prima
che lo _stream_ fosse disponibile.

Invece, lasciamo `ricevi_messaggi` come una funzione regolare che restituisce
uno _stream_ e invece creiamo un _task_ per gestire le chiamate _async_ a
`sleep`.

> Nota: Chiamare `spawn_task` in questo modo funziona perché abbiamo già
> impostato il nostro _runtime_; se non lo avessimo fatto, causerebbe un
> _panic_. Altre implementazioni scelgono compromessi diversi: potrebbero
> avviare un nuovo _runtime_ e evitare il _panic_, ma avrebbero po' di
> _overhead_ extra, oppure potrebbero semplicemente non fornire un modo autonomo
> per avviare un _task_ senza un riferimento a un _runtime_. Assicurati di
> sapere quale compromesso ha scelto il tuo _runtime_ e scrivi il tuo codice di
> conseguenza!

Ora il nostro codice ha un risultato molto più interessante. Tra ogni coppia di
messaggi, appare un errore `Problema: Elapsed(())`.

```console
$cargo run
Messaggio: 'a'
Problema: Elapsed(())
Messaggio: 'b'
Messaggio: 'c'
Problema: Elapsed(())
Messaggio: 'd'
Messaggio: 'e'
Problema: Elapsed(())
Messaggio: 'f'
Messaggio: 'g'
Problema: Elapsed(())
Messaggio: 'h'
Messaggio: 'i'
Problema: Elapsed(())
Messaggio: 'j'
```

Il _timeout_ non impedisce ai messaggi di arrivare alla fine. Riceviamo ancora
tutti i messaggi originali, perché il nostro canale è _illimitato_: può
contenere quanti più messaggi possiamo inserire in memoria. Se il messaggio non
arriva prima del _timeout_, il nostro gestore di _stream_ ne terrà conto, ma
quando interroga di nuovo lo _stream_, il messaggio potrebbe ora essere
arrivato.

Puoi ottenere un comportamento diverso se necessario utilizzando altri tipi di
canali o altri tipi di _stream_ in modo più generale. Vediamo uno di questi in
pratica combinando uno _stream_ di intervalli di tempo con questo _stream_ di
messaggi.

### Unire _Stream_

Per prima cosa, creiamo un altro _stream_, che invierà un elemento ogni
millisecondo se lo lasciamo girare direttamente. Per semplicità, possiamo usare
la funzione `sleep` per inviare un messaggio con un ritardo e combinarlo con lo
stesso approccio che abbiamo usato in `ricevi_messaggi` per creare uno _stream_
da un canale. La differenza è che questa volta, stiamo per restituire il
conteggio degli intervalli che sono trascorsi, quindi il _type_ di ritorno sarà
`impl Stream<Item = u32>`, e possiamo chiamare la funzione `ricevi_intervalli`
(vedi Listato 17-36).

<Listing number="17-36" caption="Creare uno _stream_ con un contatore che verrà inviato una volta ogni millisecondo" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-36/src/main.rs:intervals}}
```

</Listing>

Iniziamo definendo un `conteggio` nel _task_. (Potremmo definirlo anche al di
fuori del _task_, ma il codice risulterà più chiaro se limitiamo ogni variabile
allo _scope_ che la riguarda.) Poi creiamo un ciclo infinito. Ogni iterazione
del ciclo “dorme” asincronamente per un millisecondo, incrementa il conteggio e
poi lo invia attraverso il canale. Poiché tutto questo è avvolto nell'attività
creata da `spawn_task`, tutto, incluso il ciclo infinito, verrà pulito insieme
al _runtime_.

Questo tipo di ciclo infinito, che termina solo quando l'intero _runtime_ viene
distrutto, è abbastanza comune in Rust _async_: molti programmi devono
continuare a girare indefinitamente. Con _async_, questo non blocca nulla,
purché ci sia almeno un punto di attesa in ogni iterazione del ciclo.

Ora, tornando al blocco _async_ della nostra funzione principale, possiamo
tentare di unire gli _stream_ `messaggi` e `intervalli`, come mostrato nel
Listato 17-37.

<Listing number="17-37" caption="Tentativo di unire gli _stream_ `messaggi` e `intervalli`" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-37/src/main.rs:main}}
```

</Listing>

Iniziamo chiamando `ricevi_intervalli`. Poi uniamo gli _stream_ `messaggi` e
`intervalli` con il metodo `merge`, che combina più _stream_ in uno _stream_
unico che produce elementi da qualsiasi degli _stream_ sorgente non appena gli
elementi sono disponibili, senza imporre alcun ordinamento particolare. Infine,
iteriamo su quello _stream_ unico invece che su `messaggi`.

A questo punto, né `messaggi` né `intervalli` devono essere fissati o mutabili,
perché entrambi finiranno nello _stream_ unico `uniti`. Tuttavia, questa
chiamata a `merge` non si compila! (Neanche la chiamata a `next` nel ciclo
`while let`, ma ci torneremo.) Questo perché i due _stream_ hanno _type_
diversi. Lo _stream_ `messaggi` ha il _type_ `Timeout<impl Stream<Item =
String>>`, dove `Timeout` è il _type_ che implementa `Stream` per una chiamata
di `timeout`. Lo _stream_ `intervalli` ha il _type_ `impl Stream<Item = u32>`.
Per unire questi due _stream_, dobbiamo trasformare uno di essi per farlo
corrispondere all'altro. Rielaboreremo lo _stream_ degli intervalli, perché
`messaggi` è già nel formato di base che vogliamo e deve gestire gli errori di
_timeout_ (vedi Listato 17-38).

<!-- Non possiamo testare direttamente questo, perché non si ferma mai. -->

<Listing number="17-38" caption="Allineare il _type_ dello _stream_ `intervalli` con il _type_ dello _stream_ `messaggi`" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-38/src/main.rs:main}}
```

</Listing>

Per prima cosa, possiamo usare il metodo `map` per trasformare gli `intervalli`
in una stringa. In secondo luogo, dobbiamo abbinare il `Timeout` da `messaggi`.
Poiché non vogliamo effettivamente un _timeout_ per `intervalli`, però, possiamo
semplicemente creare un _timeout_ che sia più lungo delle altre durate che
stiamo usando. Qui, creiamo un _timeout_ di 10 secondi con
`Duration::from_secs(10)`. Infine, dobbiamo rendere `stream` mutabile, in modo
che le chiamate `next` del ciclo `while let` possano iterare attraverso lo
_stream_, e fissarlo in modo che sia sicuro farlo. Questo ci porta _quasi_ dove
dobbiamo essere. Tutto è dello stesso _type_. Se lo esegui in questo momento,
però, ci saranno due problemi. Primo, non si fermerà mai! Dovrai fermarlo con
<kbd>ctrl</kbd>-<kbd>c</kbd>. Secondo, i messaggi dall'alfabeto inglese saranno
sepolti in mezzo a tutti i messaggi del contatore degli intervalli:

```text
--taglio--
Intervallo: 43
Intervallo: 44
Intervallo: 45
Messaggio: 'a'
Intervallo: 46
Intervallo: 47
Intervallo: 48
--taglio--
```

Il Listato 17-39 mostra un modo per risolvere questi ultimi due problemi.

<Listing number="17-39" caption="Utilizzo di `throttle` e `take` per gestire gli _stream_ uniti" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-39/src/main.rs:throttle}}
```

</Listing>

Per prima cosa, utilizziamo il metodo `throttle` sullo _stream_ `intervalli` in
modo che non sovraccarichi lo _stream_ `messaggi`. _Throttling_ è un modo per
limitare la frequenza con cui una funzione verrà chiamata, o, in questo caso,
quanto spesso lo _stream_ verrà interrogato. Una volta ogni 100 millisecondi
dovrebbe andare bene, perché è più o meno quanto spesso arrivano i nostri
messaggi.

Per limitare il numero di elementi che accetteremo da uno _stream_, applichiamo
il metodo `take` allo _stream_ `uniti`, perché vogliamo limitare l'output
finale, non solo uno _stream_ o l'altro.

Ora, quando eseguiamo il programma, si ferma dopo aver estratto 20 elementi
dallo _stream_, e gli intervalli non sovraccaricano i messaggi. Non otteniamo
`Interval: 100` o `Interval: 200` e così via, ma invece otteniamo `Interval: 1`,
`Interval: 2`, e così via, anche se abbiamo uno _stream_ sorgente che _può_
produrre un evento ogni millisecondo. Questo perché la chiamata a `throttle`
produce un nuovo _stream_ che avvolge lo _stream_ originale, in modo che lo
_stream_ originale venga interrogato solo alla velocità di throttle, non alla
sua "velocità nativa". Non abbiamo un sacco di messaggi di intervallo non
gestiti che scegliamo di ignorare. Invece, non produciamo mai quei messaggi di
intervallo in primo luogo! Questa è l'innata "pigrizia" dei _future_ di Rust che
entra in gioco, permettendoci di scegliere le nostre caratteristiche di
prestazione.

```console
{{#include ../listings/ch17-async-await/listing-17-39/output.txt}}
```

C'è un'ultima cosa che dobbiamo gestire: gli errori! Con entrambi questi
_stream_ basati su canali, le chiamate a `send` potrebbero fallire quando
l'altra estremità del canale si chiude, e questo è solo una questione di come il
_runtime_ esegue le _future_ che compongono lo _stream_. Fino ad ora, abbiamo
ignorato questa possibilità chiamando `unwrap`, ma in un'applicazione ben
progettata, dovremmo gestire esplicitamente l'errore, almeno terminando il ciclo
in modo da non provare a inviare ulteriori messaggi. Il listato 17-40 mostra una
semplice strategia per gli errori: stampare il problema e poi uscire dai cicli
con `break`.

<Listing number="17-40" caption="Gestione degli errori e chiusura dei cicli">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-40/src/main.rs:errors}}
```

</Listing>

Come al solito, il modo corretto per gestire un errore di invio di un messaggio
varierà; assicurati solo di avere una strategia.

Ora che abbiamo visto un sacco di _async_ nella pratica, facciamo un passo
indietro e approfondiamo alcuni dettagli su come Rust usa `Future`, `Stream` e
gli altri _trait_ chiave per far funzionare l'async.

[17-02-messages]: ch17-02-concurrency-with-async.html#conteggiare-su-due-task-usando-il-passaggio-di-messaggi
[iterator-trait]: ch13-02-iterators.html#il-trait-iterator-e-il-metodo-next
