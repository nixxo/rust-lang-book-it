### Restituire il Controllo al _Runtime_

Ricorda da [“Il Nostro Primo Programma _Async_”][async-program]<!-- ignore -->
che ad ogni punto di attesa, Rust dà a un _runtime_ la possibilità di mettere in
pausa il compito e passare a un altro se la _future_ in attesa non è pronta.
Anche l’inverso è vero: Rust _mette in pausa_ solo i blocchi _async_ e
restituisce il controllo a un _runtime_ in un punto di attesa. Tutto ciò che si
trova tra i punti di attesa è sincrono.

Questo significa che se fai un sacco di lavoro in un blocco _async_ senza un
punto di attesa, quella _future_ bloccherà qualsiasi altra _future_ dal fare
progressi. A volte potresti sentire menzionato questo comportamento come ad una
_future_ che _affama_ (_starving_) altre _future_. In alcuni casi, potrebbe non
essere un grosso problema. Tuttavia, se stai facendo qualche tipo di
elaborazione dispendiosa o lavoro a lungo termine, o se hai una _future_ che
continuerà a fare un particolare compito indefinitamente, dovrai pensare a
quando e dove restituire il controllo al _runtime_.

Simuliamo un’operazione a lungo termine per illustrare il problema
dell’_affamamento_ e come risolverlo. Il Listato 17-14 introduce una funzione
`lenta`.

<Listing number="17-14" file-name="src/main.rs" caption="Utilizzo di `thread::sleep` per simulare operazioni lente">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-14/src/main.rs:slow}}
```

</Listing>

Questo codice utilizza `std::thread::sleep` invece di `trpl::sleep` in modo che
chiamare `lenta` blocchi il _thread_ corrente per un certo numero di
millisecondi. Possiamo usare `lenta` per rappresentare operazioni del mondo
reale che sono sia a lungo termine che bloccanti.

Nel Listato 17-15, utilizziamo `lenta` per emulare questo tipo di lavoro legato
alla CPU in un paio di _future_.

<Listing number="17-15" file-name="src/main.rs" caption="Chiamate a `lenta` per simulare operazioni che vanno a rilento">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-15/src/main.rs:slow-futures}}
```

</Listing>

Ogni future restituisce il controllo al _runtime_ solo _dopo_ aver eseguito
alcune operazioni lente. Se esegui questo codice, vedrai questo output:

```text
'a' iniziata.
'a' eseguita per 30ms
'a' eseguita per 10ms
'a' eseguita per 20ms
'b' iniziata.
'b' eseguita per 75ms
'b' eseguita per 10ms
'b' eseguita per 15ms
'b' eseguita per 350ms
'a' finita.
```

Come per il Listato 17-5 dove abbiamo usato `trpl::select` per mettere a gara
due _future_ che elaboravano un URL, `select` termina non appena `a` è
completata. Non c’è “intreccio” tra le due _future_, però. La _future_ `a` fa
tutto il suo lavoro fino a quando la chiamata a `trpl::sleep` viene attesa con
`await`, poi la _future_ `b` fa tutto il suo lavoro fino a quando la sua
chiamata a `trpl::sleep` viene attesa, e infine la _future_ `a` finisce. Per
consentire a entrambe le _future_ lente di fare progressi, abbiamo bisogno di
punti di attesa in modo da poter restituire il controllo al _runtime_ di tanto
in tanto per consentire anche all’altra di proseguire!

Possiamo già vedere questo tipo di passaggio avvenire nel Listato 17-15: se
rimuovessimo `trpl::sleep` alla fine della _future_ `a`, essa completerebbe la
propria esecuzione senza che la _future_ `b` nemmeno cominciasse. Proviamo a
utilizzare la funzione `trpl::sleep` come punto di partenza per consentire alle
operazioni di alternarsi nel fare progressi, come mostrato nel Listato 17-16.

<Listing number="17-16" file-name="src/main.rs" caption="Utilizzo di `trpl::sleep` per consentire alle operazioni di alternarsi nel fare progressi">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-16/src/main.rs:here}}
```

</Listing>

Abbiamo aggiunto chiamate a `trpl::sleep` con punti di attesa tra ogni chiamata
a `lenta`. Ora il lavoro delle due _future_ è intervallato:

```text
'a' iniziata.
'a' eseguita per 30ms
'b' iniziata.
'b' eseguita per 75ms
'a' eseguita per 10ms
'b' eseguita per 10ms
'a' eseguita per 20ms
'b' eseguita per 15ms
'a' finita.
```

La _future_ `a` continua a lavorare per un po’ prima di restituire il controllo
a `b`, perché chiama `lenta` prima di chiamare `trpl::sleep`, ma dopo ciò le
_future_ si alternano ogni volta che una di esse incontra un punto di attesa. In
questo caso, abbiamo fatto ciò dopo ogni chiamata a `lenta`, ma potremmo
suddividere il lavoro in qualsiasi modo abbia più senso per noi.

Ma non vogliamo davvero _dormire_ qui, però: vogliamo eseguire le nostre
operazioni il più velocemente possibile e restituire il controllo al _runtime_
quando possibile. Possiamo farlo direttamente, utilizzando la funzione
`trpl::yield_now`. Nel Listato 17-17, sostituiamo tutte quelle chiamate a
`trpl::sleep` con `trpl::yield_now`.

<Listing number="17-17" file-name="src/main.rs" caption="Utilizzo di `trpl::yield_now` per consentire alle operazioni di alternarsi nel fare progressi">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-17/src/main.rs:yields}}
```

</Listing>

Questo codice è sia più chiaro riguardo all’intento reale sia può essere
significativamente più veloce rispetto all’uso di `sleep`, perché i timer come
quello usato da `sleep` hanno spesso limiti su quanto possono essere granulari.
La versione di `sleep` che stiamo usando, ad esempio, dormirà sempre per almeno
un millisecondo, anche se le passiamo una `Duration` di un nanosecondo. Ancora
una volta, i computer moderni sono _veloci_: possono fare molto in un
millisecondo!

Questo significa che l’_async_ può essere utile anche per compiti legati al
calcolo, a seconda di cosa sta facendo il tuo programma, perché fornisce uno
strumento utile per strutturare le relazioni tra le diverse parti del programma
(ma con un costo prestazionale per la macchina a stati _async_). Questa è una
forma di _multitasking cooperativo_, in cui ogni _future_ ha il potere di
determinare quando restituisce il controllo tramite i punti di attesa. Ogni
_future_ ha quindi anche la responsabilità di evitare di bloccarsi troppo a
lungo. In alcuni sistemi operativi _embedded_ basati su Rust, questo è l’_unico_
tipo di multi-tasking!

Nel codice reale, di solito non lavorerai direttamente alternando chiamate di
funzione con punti di attesa su ogni singola riga, ovviamente. Anche se
restituire il controllo in questo modo è relativamente poco costoso, non è
gratuito. In molti casi, cercare di suddividere un compito legato al calcolo
potrebbe renderlo significativamente più lento, quindi a volte è meglio per le
prestazioni _complessive_ lasciare che un’operazione si blocchi brevemente.
Misura sempre per vedere quali sono i veri colli di bottiglia delle prestazioni
del tuo codice. Tuttavia, la dinamica sottostante è importante da tenere a
mente, se _stai_ vedendo molto lavoro avvenire in serie che ti aspettavi
avvenisse in parallelo!

###  Costruire le Nostre Astrazioni _Async_

Possiamo anche comporre le _future_ insieme per creare nuovi modelli. Ad
esempio, possiamo costruire una funzione `timeout` con i blocchi _async_ che
abbiamo già. Quando abbiamo finito, il risultato sarà un altro blocco di
costruzione che potremmo usare per creare ancora più astrazioni _async_.

Il Listato 17-18 mostra come ci aspettiamo che funzioni questo `timeout` con una
_future_ `lento`.

<Listing number="17-18" file-name="src/main.rs" caption="Utilizzo del nostro `timeout` per eseguire un’operazione lenta con un limite di tempo">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-18/src/main.rs:here}}
```

</Listing>

Implementiamolo! Per cominciare, pensiamo all’API per `timeout`:

- Deve essere essa stessa una funzione _async_ in modo da poterla attendere.
- Il suo primo parametro dovrebbe essere una _future_ da eseguire. Possiamo
  renderla generica per consentirle di funzionare con qualsiasi _future_.
- Il suo secondo parametro sarà il tempo massimo da attendere. Se usiamo una
  `Duration`, sarà facile passarla a `trpl::sleep`.
- Dovrebbe restituire un `Result`. Se la _future_ completa con successo, il
  `Result` sarà `Ok` con il valore prodotto dalla _future_. Se il _timeout_
  scade prima, il `Result` sarà `Err` con la durata che il _timeout_ ha atteso.

Il Listato 17-19 mostra questa dichiarazione.

<!-- Non testato, perché scritto intenzionalmente per non compoilarsi. -->

<Listing number="17-19" file-name="src/main.rs" caption="Definizione della firma di `timeout`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-19/src/main.rs:declaration}}
```

</Listing>

Questo soddisfa i nostri obiettivi per i _type_. Ora pensiamo al _comportamento_
di cui abbiamo bisogno: vogliamo far competere la _future_ passata contro la
durata fornita. Possiamo usare `trpl::sleep` per creare una _future_ che duri
quanto richiesto e usare `trpl::select` per eseguirla contro la _future_ che il
chiamante passa.

Nel Listato 17-20, implementiamo `timeout` facendo il _match_ sul risultato
dell’attesa di `trpl::select`.

<Listing number="17-20" file-name="src/main.rs" caption="Definizione di `timeout` con `select` e `sleep`">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-20/src/main.rs:implementation}}
```

</Listing>

L’implementazione di `trpl::select` non è equa: processa gli argomenti sempre
nell’ordine in cui sono passati (altre implementazioni di `select` scelgono a
caso quale argomento processare per primo). Pertanto, passiamo
`future_da_testare` a `select` per prima in modo che abbia la possibilità di
completare anche se `tempo_massimo` è una durata molto breve. Se
`future_da_testare` finisce prima, `select` restituirà `Left` con l’output da
`future_da_testare`. Se il timer finisce prima, `select` restituirà `Right` con
l’output del timer di `()`

Se `future_da_testare` ha successo e otteniamo un `Left(output)`, restituiamo
`Ok(output)`. Se invece il _timer_ finisce prima e otteniamo un `Right(())`,
ignoriamo il `()` con `_` e restituiamo `Err(tempo_massimo)`.

Con questo, abbiamo un `timeout` funzionante combinando più blocchi _async_. Se
eseguiamo il nostro codice, stamperà la modalità di errore dopo il timeout:

```text
Fallito dopo 2 secondi
```

Poiché le _future_ si compongono con altre _future_, puoi costruire strumenti
davvero potenti utilizzando blocchi di costruzione _async_ più piccoli. Ad
esempio, puoi utilizzare questo stesso approccio per combinare _timeout_ con
ripetizioni, e a loro volta usarli con operazioni come chiamate di rete (come
quelli nel Listato 17-5).

Nella pratica, di solito lavorerai direttamente con `async` e `await`, e
secondariamente con funzioni come `select` e macro come `join!` per controllare
come vengano eseguite le varie _future_.

Abbiamo ora visto diversi modi per lavorare con più _future_ contemporaneamente.
Prossimamente, vedremo come possiamo lavorare con più _future_ in una sequenza
nel tempo con gli _stream_.

[async-program]: ch17-01-futures-and-syntax.html#il-nostro-primo-programma-async
