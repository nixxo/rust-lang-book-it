## _Future_, _Task_ e _Thread_

Come abbiamo visto nel [Capitolo 16][ch16]<!-- ignore -->, i _thread_ offrono un
approccio alla concorrenza. Abbiamo visto un altro approccio in questo capitolo:
utilizzare _async_ con _future_ e _stream_. Se ti stai chiedendo quando
scegliere un metodo rispetto all’altro, la risposta è: dipende! E in molti casi,
la scelta non è tra _thread_ __o__ _async_, ma piuttosto tra _thread_ __e__
_async_.

Molti sistemi operativi hanno fornito modelli di concorrenza basati su _thread_
per decenni, e molti linguaggi di programmazione li supportano di conseguenza.
Tuttavia, questi modelli non sono privi di compromessi. Su molti sistemi
operativi, utilizzano una buona quantità di memoria per ogni _thread_. I
_thread_ sono anche un’opzione solo quando il tuo sistema operativo e hardware
li supportano. A differenza dei computer desktop e smartphone moderni, alcuni
sistemi _embedded_ non hanno affatto un OS, quindi non hanno nemmeno _thread_.

Il modello _async_ fornisce un insieme di compromessi diverso, e alla fine
complementare. Nel modello _async_, le operazioni concorrenti non richiedono i
propri _thread_. Invece, possono essere eseguite su _task_, come quando abbiamo
utilizzato `trpl::spawn_task` per avviare un lavoro da una funzione sincrona
nella sezione degli _stream_. Un _task_ è simile a un _thread_, ma invece di
essere gestito dal sistema operativo, è gestito da codice a livello di libreria:
il _runtime_.

C’è un motivo per cui le API per generare _thread_ e _task_ sono così simili. I
_thread_ agiscono come un confine per insiemi di operazioni sincrone; la
concorrenza è possibile _tra_ i _thread_. I _task_ agiscono come un confine per
insiemi di operazioni _asincrone_; la concorrenza è possibile sia _tra_ che
_all’interno_ dei _task_, perché un _task_ può passare tra _future_ nel suo
corpo. Infine, le _future_ sono l’unità di concorrenza più granulare di Rust, e
ogni _future_ può rappresentare un albero di altre _future_. Il _runtime_, e
nello specifico il suo esecutore, gestisce i _task_, e i _task_ gestiscono le
_future_. In questo senso, i _task_ sono simili a _thread_ leggeri gestiti dal
_runtime_ con capacità aggiuntive che derivano dal fatto di essere gestiti da un
_runtime_ anziché dal sistema operativo.

Questo non significa che i _task_ _async_ siano sempre migliori dei _thread_ (o
viceversa). La concorrenza con i _thread_ è in alcuni modi un modello di
programmazione più semplice rispetto alla concorrenza con `async`. Questo può
essere un punto di forza o una debolezza. I _thread_ sono in un certo senso
“esegui e dimenticatene”; non hanno un equivalente nativo a una _future_, quindi
semplicemente eseguono fino al completamento senza essere interrotti, tranne che
dal sistema operativo stesso.

E se non bastasse, i _thread_ e i _task_ spesso funzionano molto bene insieme,
perché i _task_ possono (almeno in alcuni _runtime_) essere spostati tra i
_thread_. Infatti, dietro le quinte, il _runtime_ che abbiamo utilizzato,
comprese le funzioni `spawn_blocking` e `spawn_task`, è multi-_thread_ per
impostazione predefinita! Molti _runtime_ utilizzano un approccio chiamato _work
stealing_ per spostare in modo trasparente i _task_ tra i _thread_, in base a
come i _thread_ vengono attualmente utilizzati, per migliorare le prestazioni
complessive del sistema. Questo approccio richiede effettivamente sia _thread_
che _task_, e quindi _future_.

Quando si pensa a quale metodo utilizzare, considera queste regole pratiche:

- Se il lavoro è _molto parallelizzabile_ (limitato dalla potenza di calcolo),
  come l’elaborazione di un insieme di dati in cui ogni parte può essere
  elaborata separatamente, i _thread_ sono una scelta migliore.
- Se il lavoro è _molto concorrente_ (limitato da I/O), come gestire messaggi
  provenienti da diverse fonti che possono arrivare a intervalli o tassi
  diversi, _async_ è una scelta migliore.

E se hai bisogno sia di parallelismo che di concorrenza, non devi scegliere tra
_thread_ e _async_. Puoi usarli insieme liberamente, lasciando a ciascuno il
compito che svolge meglio. Ad esempio, il Listato 17-42 mostra un esempio
piuttosto comune di questo tipo di mix nel codice Rust reale.

<Listing number="17-25" file-name="src/main.rs" caption="Invio di messaggi con codice bloccante in un _thread_ e attesa dei messaggi in un blocco _async_">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-25/src/main.rs:all}}
```

</Listing>

Iniziamo creando un canale _async_, quindi avviamo un _thread_ che prende
possesso della estremità del mittente del canale usando la parola chiave `move`.
All’interno del _thread_, inviamo i numeri da 1 a 10, dormendo per un secondo
tra ciascuno. Infine, eseguiamo una _future_ creata con un blocco _async_
passato a `trpl::block_on`, proprio come abbiamo fatto in tutto il capitolo. In
quella _future_, attendiamo quei messaggi, proprio come negli altri esempi di
invio messaggi che abbiamo visto.

Per tornare allo scenario con cui abbiamo aperto il capitolo, immagina di
eseguire un insieme di _task_ di codifica video utilizzando un _thread_ dedicato
(perché la codifica video è vincolata al calcolo) ma notificando l’interfaccia
utente che quelle operazioni sono terminate con un canale _async_. Ci sono
innumerevoli esempi di queste combinazioni in casi d’uso reali.

## Riepilogo

Questa non è l’ultima volta che vedrai la concorrenza in questo libro. Il
progetto nel [Capitolo 21][ch21]<!-- ignore -->  applicherà questi concetti in
una situazione più realistica rispetto agli esempi più semplici discussi qui e
confronterà la risoluzione dei problemi con i _thread_ rispetto ai _task_ e
_future_ in modo più diretto.

Indipendentemente da quale di questi approcci scegli, Rust ti offre gli
strumenti necessari per scrivere codice concorrente sicuro e veloce, sia per un
server web ad alta capacità che per un sistema operativo _embedded_.

Nel prossimo capitolo, parleremo di modi idiomatici per modellare problemi e
strutturare soluzioni man mano che i tuoi programmi Rust crescono. Inoltre,
discuteremo di come gli idiomi di Rust si relazionano a quelli con cui potresti
avere familiarità provenienti dalla programmazione orientata agli oggetti.

[ch16]: ch16-00-concurrency.html
[ch21]: ch21-00-final-project-a-web-server.html
