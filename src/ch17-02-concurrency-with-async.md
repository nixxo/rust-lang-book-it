## Applicare la Concorrenza con _Async_

In questa sezione, vedremo come usare _async_ per affrontare alcune sfide di
concorrenza che abbiamo già visto con i _thread_ nel Capitolo 16. Dato che
abbiamo già parlato dei concetti chiave, ci concentreremo sulle differenze tra
_thread_ e _future_.

In molti casi, le API per lavorare con la concorrenza usando _async_ sono molto
simili a quelle per usare i _thread_. In altri casi, finiscono per essere
piuttosto diverse. Anche quando le API sembrano simili tra _thread_ e _async_,
spesso hanno comportamenti diversi e quasi sempre caratteristiche di prestazioni
differenti.

### Creare un Nuovo _Task_ con `spawn_task`

La prima operazione che abbiamo affrontato in [“Creare un Nuovo _Thread_ con
`spawn`”][thread-spawn]<!-- ignore --> era contare su due _thread_ separati.
Facciamo la stessa cosa usando _async_. Il _crate_ `trpl` fornisce una funzione
`spawn_task` che sembra molto simile all’API `thread::spawn`, e una funzione
`sleep` che è una versione _async_ dell’API `thread::sleep`. Possiamo usarle
insieme per implementare l’esempio di conteggio, come mostrato nel Listato 17-6.

<Listing number="17-6" file-name="src/main.rs" caption="Creare un nuovo _task_ per stampare una cosa mentre il _task_ principale ne stampa un’altra">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-06/src/main.rs:all}}
```

</Listing>

Come punto di partenza, impostiamo la nostra funzione `main` con `trpl::run` in
modo che la nostra funzione di livello superiore possa essere _async_.

> Nota: Da questo punto in poi nel capitolo, ogni esempio includerà lo stesso
> esatto codice di incapsulamento con `trpl::run` in `main`, quindi spesso lo
> salteremo proprio come facciamo con `main`. Non dimenticare di includerlo nel
> tuo codice!

Poi scriviamo due loop all’interno di quel blocco, ciascuno contenente una
chiamata a `trpl::sleep`, che aspetta mezzo secondo (500 millisecondi) prima di
inviare il prossimo messaggio. Mettiamo un loop nel corpo di un
`trpl::spawn_task` e l’altro è un ciclo `for` nel _task_ principale. Aggiungiamo
anche un `await` dopo le chiamate `sleep`.

Questo codice si comporta in modo simile all’implementazione basata su _thread_,
inclusa la possibilità che tu possa vedere i messaggi apparire in un ordine
diverso nel tuo terminale quando lo esegui:

```text
ciao numero 1 dal secondo task!
ciao numero 1 dal primo task!
ciao numero 2 dal primo task!
ciao numero 2 dal secondo task!
ciao numero 3 dal primo task!
ciao numero 3 dal secondo task!
ciao numero 4 dal primo task!
ciao numero 4 dal secondo task!
ciao numero 5 dal primo task!
```

Questa versione si ferma non appena il ciclo `for` nel corpo del blocco _async_
principale finisce, perché il _task_ avviato da `spawn_task` viene chiuso quando
la funzione `main` termina. Se vuoi che si esegua fino al completamento del
_task_, dovrai usare un _join handle_ per aspettare che il primo _task_ si
completi. Con i _thread_, abbiamo usato il metodo `join` per "bloccare" fino a
quando il _thread_ avesse finito di eseguirsi. Nel Listato 17-7, possiamo usare
`await` per fare la stessa cosa, perché l’_handle_ del _task_ stesso è un
_future_. Il suo _type_ `Output` è un `Result`, quindi dopo averlo atteso
(_await_), dobbiamo anche esporlo (_unwrap_).

<Listing number="17-7" file-name="src/main.rs" caption="Usare `await` con un _join handle_ per eseguire un _task_ fino al completamento">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-07/src/main.rs:handle}}
```

</Listing>

Questa versione aggiornata si esegue fino a quando _entrambi_ i loop finiscono.


```text
ciao numero 1 dal secondo task!
ciao numero 1 dal primo task!
ciao numero 2 dal primo task!
ciao numero 2 dal secondo task!
ciao numero 3 dal primo task!
ciao numero 3 dal secondo task!
ciao numero 4 dal primo task!
ciao numero 4 dal secondo task!
ciao numero 5 dal primo task!
ciao numero 6 dal primo task!
ciao numero 7 dal primo task!
ciao numero 8 dal primo task!
ciao numero 9 dal primo task!
```

Finora, sembra che _async_ e _thread_ ci diano gli stessi risultati di base,
solo con una sintassi diversa: usando `await` invece di chiamare `join`
sull’_handle_, e aspettando le chiamate `sleep`.

La differenza più grande è che non abbiamo dovuto avviare un altro _thread_ del
sistema operativo per farlo. In realtà, non dobbiamo nemmeno avviare un _task_
qui. Poiché i blocchi _async_ si compilano in _future_ anonime, possiamo mettere
ogni loop in un blocco _async_ e far eseguire al _runtime_ entrambe fino al
completamento usando la funzione `trpl::join`.

Nella sezione [“Attendere Che Tutti i _Thread_ Finiscano”][join-handles]<!--
ignore -->, abbiamo mostrato come usare il metodo `join` sul _type_ `JoinHandle`
restituito quando si chiama `std::thread::spawn`. La funzione `trpl::join` è
simile, ma per le _future_. Quando gli dai due _future_, produce una singola
nuova _future_ il cui output è una tupla che contiene l’output di ciascuna
_future_ che hai passato una volta che _entrambe_ si completano. Quindi, nel
Listato 17-8, usiamo `trpl::join` per aspettare che sia `fut1` che `fut2`
finiscano. Non aspettiamo `fut1` e `fut2` ma invece la nuova _future_ prodotta
da `trpl::join`. Ignoriamo l’output, perché è solo una tupla che contiene due
valori unitari.

<Listing number="17-8" file-name="src/main.rs" caption="Usare `trpl::join` per aspettare due _future_ anonime">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-08/src/main.rs:join}}
```

</Listing>

Quando lo eseguiamo, vediamo entrambe le _future_ eseguirsi fino al
completamento:


```text
ciao numero 1 dal primo task!
ciao numero 1 dal secondo task!
ciao numero 2 dal primo task!
ciao numero 2 dal secondo task!
ciao numero 3 dal primo task!
ciao numero 3 dal secondo task!
ciao numero 4 dal primo task!
ciao numero 4 dal secondo task!
ciao numero 5 dal primo task!
ciao numero 6 dal primo task!
ciao numero 7 dal primo task!
ciao numero 8 dal primo task!
ciao numero 9 dal primo task!
```

Ora, vedrai lo stesso ordine ogni volta, il che è molto diverso da quello che
abbiamo visto con i _thread_. Questo perché la funzione `trpl::join` è _equa_,
il che significa che controlla ciascuna _future_ con la stessa frequenza,
alternando tra loro, e non lascia che una “corra avanti” se l’altra è pronta.
Con i _thread_, il sistema operativo decide quale _thread_ controllare e per
quanto tempo farlo eseguire. Con _async_ Rust, il _runtime_ decide quale _task_
controllare. (Nella pratica, i dettagli si complicano perché un _runtime_
_async_ potrebbe in realtà usare i _thread_ del sistema operativo come parte
della gestione della concorrenza, quindi garantire l’equità può essere più
lavoro per un _runtime_, ma è comunque possibile!) I _runtime_ non devono
garantire l’equità per qualsiasi operazione data, e spesso offrono diverse API
per farti scegliere se vuoi l’equità o meno.

Prova alcune di queste varianti sull’attesa dei _future_ e vedi cosa fanno:

- Rimuovi il blocco _async_ da uno o entrambi i loop.
- Aspetta ogni blocco _async_ immediatamente dopo averlo definito.
- Incapsula solo il primo loop in un blocco _async_ e aspetta il _future_
  risultante dopo il corpo del secondo loop.

Per una sfida extra, cerca di capire quale sarà l’output in ciascun caso _prima_
di eseguire il codice!

### Conteggiare su Due _Task_ Usando il Passaggio di Messaggi

Condividere dati tra _future_ sarà familiare: useremo di nuovo il passaggio di
messaggi, ma questa volta con le versioni _async_ dei _type_ e delle funzioni.
Prenderemo una strada leggermente diversa rispetto a quella che abbiamo preso in
[“Usare il Passaggio di Messaggi per Trasferire Dati tra
_Thread_”][message-passing-threads]<!-- ignore --> per illustrare alcune delle
differenze chiave tra concorrenza basata su _thread_ e concorrenza basata su
_future_. Nel Listato 17-9, inizieremo con un singolo blocco _async_, _non_
creando un _task_ separato come avevamo creato un _thread_ separato.

<Listing number="17-9" file-name="src/main.rs" caption="Creare un canale _async_ e assegnare le due metà a `tx` e `rx`">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-09/src/main.rs:channel}}
```

</Listing>

Qui, usiamo `trpl::channel`, una versione _async_ dell’API del canale
multi-produttore, singolo-consumatore che abbiamo usato con i _thread_ nel
Capitolo 16. La versione _async_ dell’API è solo un po' diversa dalla versione
basata su _thread_: usa un ricevitore `rx` mutabile piuttosto che immutabile, e
il suo metodo `recv` produce una _future_ che dobbiamo aspettare piuttosto che
produrre il valore direttamente. Ora possiamo inviare messaggi dal mittente al
ricevitore. Nota che non dobbiamo avviare un _thread_ separato o nemmeno un
_task_; dobbiamo solo aspettare la chiamata `rx.recv`.

Il metodo sincrono `Receiver::recv` in `std::mpsc::channel` blocca fino a quando
non riceve un messaggio. Il metodo `trpl::Receiver::recv` non lo fa, perché è
_async_. Invece di bloccare, restituisce il controllo al _runtime_ fino a quando
non viene ricevuto un messaggio o la estremità di invio del canale si chiude. Al
contrario, non aspettiamo la chiamata `send`, perché non blocca. Non ne ha
bisogno, perché il canale in cui lo stiamo inviando è senza vincoli.

> Nota: Poiché tutto questo codice _async_ si esegue in un blocco _async_ in una
> chiamata `trpl::run`, tutto al suo interno può evitare di bloccare. Tuttavia,
> il codice _fuori_ da esso si bloccherà sulla funzione `run` che restituisce.
> Questo è proprio lo scopo della funzione `trpl::run`: ci permette di
> _scegliere_ dove bloccare su un insieme di codice _async_, e quindi dove
> passare tra codice sincrono e asincrono. In molti _runtime_ asincroni, `run` è
> effettivamente chiamato `block_on` proprio per questo motivo.

Nota due cose in questo esempio. Prima di tutto, il messaggio arriverà subito.
Secondo, anche se usiamo una _future_ qui, non c’è ancora concorrenza. Tutto
nell’elenco accade in sequenza, proprio come farebbe se non ci fossero _future_
coinvolte.

Affrontiamo la prima parte inviando una serie di messaggi e “dormendo” tra di
loro, come mostrato nel Listato 17-10.

<!-- Non possiamo testare questo codice perché non si ferma mai! -->

<Listing number="17-10" file-name="src/main.rs" caption="Inviare e ricevere più messaggi sul canale _async_ e dormire con un `await` tra ogni messaggio">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-10/src/main.rs:many-messages}}
```

</Listing>

Oltre ad inviare i messaggi, dobbiamo riceverli. In questo caso, poiché sappiamo
quanti messaggi stanno arrivando, potremmo farlo manualmente chiamando
`rx.recv().await` quattro volte. Nel mondo reale, tuttavia, di solito stiamo
aspettando un numero _sconosciuto_ di messaggi, quindi dobbiamo continuare ad
aspettare fino a quando non determiniamo che non ci sono più messaggi.

Nel Listato 16-10, abbiamo usato un ciclo `for` per elaborare tutti gli elementi
ricevuti da un canale sincrono. Rust non ha ancora un modo per scrivere un ciclo
`for` su una serie _asincrona_ di elementi, quindi dobbiamo usare un ciclo che
non abbiamo visto prima: il ciclo condizionale `while let`. Questo è la versione
ciclo della costruzione `if let` che abbiamo visto nella sezione [“Controllare
il Flusso con `if let` e `let else`”][if-let]<!-- ignore -->. Il ciclo
continuerà ad eseguirsi finché il _pattern_ specificato continua a corrispondere
al valore.

La chiamata `rx.recv` produce una _future_, che aspettiamo. Il _runtime_ metterà
in pausa la _future_ fino a quando non sarà pronta. Una volta che arriva un
messaggio, la _future_ si risolverà in `Some(messaggio)` tutte le volte che
arriva un messaggio. Quando il canale si chiude, indipendentemente dal fatto che
siano arrivati _alcuni_ messaggi, la _future_ si risolverà invece in `None` per
indicare che non ci sono più valori e quindi dobbiamo smettere di aspettare.

Il ciclo `while let` mette insieme tutto questo. Se il risultato della chiamata
`rx.recv().await` è `Some(messaggio)`, otteniamo accesso al messaggio e possiamo
usarlo nel corpo del ciclo, proprio come potremmo fare con `if let`. Se il
risultato è `None`, il ciclo termina. Ogni volta che il ciclo si completa,
raggiunge di nuovo il punto di attesa, quindi il _runtime_ lo mette di nuovo in
pausa fino a quando non arriva un altro messaggio.

Il codice invia e riceve ora tutti i messaggi con successo. Purtroppo, ci sono
ancora un paio di problemi. Innanzitutto, i messaggi non arrivano a intervalli
di mezzo secondo. Arrivano tutti insieme, 2 secondi (2.000 millisecondi) dopo
aver avviato il programma. In secondo luogo, questo programma non si arresta
mai! Invece, aspetta per sempre nuovi messaggi. Dovrai interromperlo usando
<kbd>ctrl</kbd>-<kbd>C</kbd>.

Iniziamo esaminando perché i messaggi arrivano tutti insieme dopo il ritardo
cumulativo, piuttosto che arrivare con ritardi tra ciascuno. All’interno di un
dato blocco _async_, l’ordine in cui compaiono le parole chiave `await` nel
codice è anche l’ordine in cui vengono eseguite quando il programma si avvia.

C’è un singolo blocco _async_ nel Listato 17-10, quindi tutto in esso si esegue
linearmente. Non c’è ancora concorrenza. Tutti i `tx.send` accadono, intercalati
con tutte le chiamate `trpl::sleep` e i loro punti di attesa associati. Solo
allora il ciclo `while let` può passare in rassegna alcuni dei punti di attesa
sulle chiamate `recv`.

Per ottenere il comportamento che vogliamo, dove il ritardo accade tra ogni
messaggio, dobbiamo mettere le operazioni `tx` e `rx` nei loro blocchi _async_
separati, come mostrato nel Listato 17-11. In questo modo il _runtime_ può
eseguire ciascuno di essi separatamente usando `trpl::join`, proprio come
nell’esempio del conteggio. Ancora una volta, aspettiamo il risultato della
chiamata a `trpl::join`, non le _future_ singole. Se avessimo aspettato le
_future_ singole in sequenza, saremmo tornati a un flusso sequenziale, proprio
quello che stiamo cercando di _non_ fare.

<!-- Non possiamo testare questo codice perché non si ferma mai! -->

<Listing number="17-11" file-name="src/main.rs" caption="Separare `send` e `recv` nei loro blocchi `async` e aspettare le _future_ per quei blocchi">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-11/src/main.rs:futures}}
```

</Listing>

Con il codice aggiornato nel Listato 17-11, i messaggi vengono stampati a
intervalli di 500 millisecondi, piuttosto che tutti insieme dopo 2 secondi.

Il programma non si arresta comunque, perché il ciclo `while let` interagisce
con `trpl::join`:

- La _future_ restituita da `trpl::join` si completa solo una volta che
  _entrambe_ le _future_ passate ad esso si sono completate.
- La _future_ `tx` si completa una volta che ha finito di dormire dopo aver
  inviato l’ultimo messaggio in `vals`.
- La _future_ `rx` non si completerà fino a quando il ciclo `while let` non
  termina.
- Il ciclo `while let` non terminerà fino a quando l’attesa di `rx.recv` produce
  `None`.
- L’attesa di `rx.recv` restituirà `None` solo una volta che l’altra estremità
  del canale è chiusa.
- Il canale si chiuderà solo se chiamiamo `rx.close` o quando l’estremità invio,
  `tx`, viene eliminata.
- Non chiamiamo `rx.close` da nessuna parte, e `tx` non verrà eliminato fino a
  quando il blocco _async_ più esterno passato a `trpl::run` non termina.
- Il blocco non può terminare perché è bloccato su `trpl::join` in attesa di
  completamento, il che ci riporta all’inizio di questo elenco.

Potremmo chiudere manualmente `rx` chiamando `rx.close` da qualche parte, ma non
ha molto senso. Fermarsi dopo aver gestito un numero arbitrario di messaggi
farebbe chiudere il programma, ma potremmo perdere messaggi. Abbiamo bisogno di
un altro modo per assicurarci che `tx` venga eliminato _prima_ della fine della
funzione.

Al momento, il blocco _async_ in cui inviamo i messaggi prende in prestito solo
`tx` perché inviare un messaggio non richiede la _ownership_, ma se potessimo
spostare `tx` in quel blocco _async_, verrebbe eliminato una volta che quel
blocco termina. Nella sezione del Capitolo 13 [“Catturare i _Reference_ o
Trasferire la _Ownership_”][capture-or-move]<!-- ignore -->, hai imparato come
usare la parola chiave `move` con le chiusure, e, come discusso nella sezione
del Capitolo 16 [“Usare le Chiusure `move` con i `Thread`”][move-threads]<!--
ignore -->, spesso dobbiamo spostare i dati nelle chiusure quando lavoriamo con
i _thread_. Le stesse dinamiche di base si applicano ai blocchi _async_, quindi
la parola chiave `move` funziona con i blocchi _async_ proprio come fa con le
chiusure.

Nel Listato 17-12, cambiamo il blocco usato per inviare messaggi da `async` a
`async move`. Quando eseguiamo _questa_ versione del codice, si chiude
correttamente dopo che l’ultimo messaggio è stato inviato e ricevuto.

<Listing number="17-12" file-name="src/main.rs" caption="Una revisione del codice nel Listato 17-11 che si chiude correttamente al completamento">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-12/src/main.rs:with-move}}
```

</Listing>

Questo canale _async_ è anche un canale multi-produttore, quindi possiamo
chiamare `clone` su `tx` se vogliamo inviare messaggi da più _future_, come
mostrato nel Listato 17-13.

<Listing number="17-13" file-name="src/main.rs" caption="Usare più produttori con blocchi _async_">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-13/src/main.rs:here}}
```

</Listing>

Prima di tutto, cloniamo `tx`, creando `tx1` fuori dal primo blocco _async_.
Spostiamo `tx1` in quel blocco proprio come abbiamo fatto prima con `tx`. Poi,
in seguito, spostiamo l’originale `tx` in un _nuovo_ blocco _async_, dove
inviamo più messaggi con un ritardo leggermente minore. Abbiamo messo questo
nuovo blocco _async_ dopo il blocco _async_ per ricevere messaggi, ma andrebbe
bene anche se messo prima. La chiave è l’ordine in cui le _future_ vengono
attese, non quello in cui vengono create.

Entrambi i blocchi _async_ per inviare messaggi devono essere blocchi `async
move` in modo che sia `tx` che `tx1` vengano eliminati quando quei blocchi
finiscono. Altrimenti, finiremo di nuovo nello stesso ciclo infinito da cui
siamo partiti. Infine, passiamo da `trpl::join` a `trpl::join3` per gestire la
_future_ aggiuntiva.

Ora vediamo tutti i messaggi da entrambe le _future_ di invio, e poiché le
_future_ di invio usano ritardi leggermente diversi dopo l’invio, i messaggi
vengono anche ricevuti a quegli intervalli diversi.

```text
ricevuto 'ciao'
ricevuto 'altri'
ricevuto 'dalla'
ricevuto 'future'
ricevuto 'messaggi'
ricevuto '!!!'
ricevuto 'per'
ricevuto 'te'
```

Questo è un buon inizio, ma ci limita a solo una manciata di _future_: due con
`join`, o tre con `join3`. Vediamo come potremmo lavorare con più _future_.

[thread-spawn]: ch16-01-threads.html#creare-un-nuovo-thread-con-spawn
[join-handles]: ch16-01-threads.html#attendere-che-tutti-i-thread-finiscano
[message-passing-threads]: ch16-02-message-passing.html
[if-let]: ch06-03-if-let.html
[capture-or-move]: ch13-01-closures.html#catturare-i-reference-o-trasferire-la-ownership
[move-threads]: ch16-01-threads.html#usare-le-chiusure-move-con-i-thread
