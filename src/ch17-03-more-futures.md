## Lavorare con un Numero Qualsiasi di _Future_

Quando siamo passati dall’usare due _future_ a tre nella sezione precedente,
abbiamo dovuto passare da `join` a `join3`. Sarebbe fastidioso dover chiamare
una funzione diversa ogni volta che cambiamo il numero di _future_ che vogliamo
unire. Per fortuna, abbiamo una forma macro di `join` a cui possiamo passare un
numero arbitrario di argomenti. Gestisce anche l’attesa delle _future_ stessa.
Quindi, potremmo riscrivere il codice del Listato 17-13 per usare `join!` invece
di `join3`, come nel Listato 17-14.

<Listing number="17-14" file-name="src/main.rs" caption="Usare `join!` per aspettare più _future_">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-14/src/main.rs:here}}
```

</Listing>

Questo è sicuramente un miglioramento rispetto allo scambio tra `join` e `join3`
e `join4` e così via! Tuttavia, anche questa macro funziona solo quando
conosciamo il numero di _future_ in anticipo. Nel mondo reale di Rust, tuttavia,
mettere le _future_ in una collezione e poi aspettare che alcune o tutte le
_future_ si completino è il modello più comune.

Per controllare tutte le _future_ in una qualche collezione, dovremo iterare e
unire su _tutte_ loro. La funzione `trpl::join_all` accetta qualsiasi _type_ che
implementa il _trait_ `Iterator`, che hai imparato nel Capitolo 13 in [“Il
_Trait_ `Iterator` e il Metodo `next`”][iterator-trait]<!-- ignore -->, quindi
sembra proprio la cosa giusta. Proviamo a mettere le nostre _future_ in un
vettore e sostituire `join!` con `join_all` come mostrato nel Listato 17-15.

<Listing number="17-15" caption="Memorizzare _future_ anonime in un vettore e chiamare `join_all`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-15/src/main.rs:here}}
```

</Listing>

Purtroppo, questo codice non si compila. Invece, otteniamo questo errore:

```console
{{#include ../listings/ch17-async-await/listing-17-15/output.txt:2:23}}
```

Questo potrebbe essere sorprendente. Dopotutto, nessuno dei blocchi _async_
restituisce nulla, quindi ciascuno produce un `Future<Output = ()>`. Ricorda che
`Future` è un _trait_, e che il compilatore crea una _enum_ univoca per ogni
blocco _async_. Non puoi mettere due _struct_ scritte a mano diverse in un
`Vec`, e la stessa regola si applica alle _enum_ diverse generate dal
compilatore.

Per farlo funzionare, dobbiamo usare gli oggetti _trait_, proprio come abbiamo
fatto in [“Restituire Errori dalla Funzione `esegui`”][dyn]<!-- ignore --> nel
Capitolo 12. (Parleremo degli oggetti _trait_ in dettaglio nel Capitolo 18.)
Usare oggetti _trait_ ci permette di trattare ciascuna delle _future_ anonime
prodotte da questi _type_ come fossero il medesimo _type_, perché tutti
implementano il _trait_ `Future`.

> Nota: In [“Utilizzare un’_Enum_ per Memorizzare Più _Type_”][enum-alt]<!--ignore -->
> nel Capitolo 8, abbiamo discusso un altro modo per includere più _type_ in un
> `Vec`: usando una _enum_ per rappresentare ciascun _type_ che può apparire nel
> vettore. Non possiamo farlo qui, però. Per prima cosa, non abbiamo modo di
> nominare i diversi _type_, perché sono anonimi. Inoltre, il motivo per cui
> abbiamo aggiunto un vettore e `join_all` in primo luogo era per poter lavorare
> con una collezione dinamica di _future_ dove ci importa solo che abbiano lo
> stesso tipo di output.

Iniziamo incapsulando ciascuna _future_ nel `vec!` in una `Box::new`, come
mostrato nel Listato 17-16.

<Listing number="17-16" file-name="src/main.rs" caption="Usare `Box::new` per allineare i _type_ delle _future_ in un `Vec`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-16/src/main.rs:here}}
```

</Listing>

Purtroppo, questo codice non si compila ancora. In realtà, otteniamo lo stesso
errore di base che abbiamo ottenuto prima sia per la seconda che la terza
chiamata a `Box::new` , oltre a nuovi errori che fanno riferimento al _trait_
`Unpin`. Torneremo sugli errori `Unpin` tra un momento. Prima, correggiamo gli
errori di _type_ sulle chiamate `Box::new` annotando esplicitamente il _type_
della variabile `future` come nel Listato 17-17.

<Listing number="17-17" file-name="src/main.rs" caption="Correggere il resto degli errori di _type_ non corrispondente usando una dichiarazione di _type_ esplicita">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-17/src/main.rs:here}}
```

</Listing>

Questa dichiarazione di _type_ è un po' complicata, quindi descriviamola pezzo
per pezzo:

1. Il _type_ più interno è la _future_ stessa. Annotiamo esplicitamente che
   l’output della _future_ è il _type_ unitario `()` scrivendo `Future<Output =
   ()>`.
1. Quindi annotiamo il _trait_ con `dyn` per marcarlo come dinamico.
1. L’intero _reference_ al _trait_ è incapsulato in una `Box`.
1. Infine, dichiariamo esplicitamente che `future` è un `Vec` che contiene
   questi elementi.

Questo ha già fatto una grande differenza. Ora, quando eseguiamo la
compilazione, otteniamo solo gli errori che menzionano `Unpin`. Anche se ce ne
sono tre, i loro contenuti sono molto simili.

```console
{{#include ../listings/ch17-async-await/listing-17-17/output.txt:2:56}}
```

Questo è un sacco da digerire, quindi facciamolo a pezzi. La prima parte del
messaggio ci dice che il primo blocco _async_ (`src/main.rs:8:23: 20:10`) non
implementa il _trait_ `Unpin` e suggerisce di usare `pin!` o `Box::pin` per
risolverlo. Più avanti nel capitolo, approfondiremo alcuni dettagli su `Pin` e
`Unpin`. Per il momento, però, possiamo semplicemente seguire il consiglio del
compilatore per sbloccarci. Nel Listato 17-18, iniziamo importando `Pin` da
`std::pin`. Quindi aggiorniamo l’annotazione di _type_ per `future`, con un
`Pin` che incapsula ogni `Box`. Infine, usiamo `Box::pin` per sistemare le
stesse _future_.

<Listing number="17-18" file-name="src/main.rs" caption="Usare `Pin` e `Box::pin` per far sì che il _type_ `Vec` superi il controllo">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-18/src/main.rs:here}}
```

</Listing>

Se compiliamo ed eseguiamo questo, otteniamo finalmente l’output che speravamo:

```text
ricevuto 'ciao'
ricevuto 'altri'
ricevuto 'dalla'
ricevuto 'messaggi'
ricevuto 'future'
ricevuto 'per'
ricevuto '!!!'
ricevuto 'te'
```

Bene!

C’è ancora un po' da fare qui. Per prima cosa, usare `Pin<Box<T>>` aggiunge una
piccola quantità di _overhead_ perché mettiamo queste _future_ nell’_heap_ con
`Box`, e lo stiamo facendo solo per far sì che i _type_ si allineino. In realtà,
non abbiamo bisogno dell’allocazione nell’_heap_: queste _future_ sono locali a
questa particolare funzione. Come notato prima, `Pin` è esso stesso un _type_ di
incapsulamento, quindi possiamo ottenere il beneficio di avere un singolo _type_
nel `Vec`, la ragione per cui abbiamo usato `Box`, senza fare un’allocazione
nell’_heap_. Possiamo perciò usare `Pin` direttamente con ciascuna _future_,
usando la macro `std::pin::pin`.

Tuttavia, dobbiamo ancora essere espliciti sul _type_ del _reference_ fissato;
altrimenti, Rust non saprà di interpretare questi come oggetti _trait_ dinamici,
che è ciò di cui abbiamo bisogno che siano nel `Vec`. Aggiungiamo `pin` alla
nostra lista di importazioni da `std::pin` e quindi possiamo usare `pin!` con
ciascuna _future_ quando la definiamo per poi definire `future` come un `Vec`
che contiene _reference_ mutabili fissati ai _type_ _future_ dinamici, come nel
Listato 17-19.

<Listing number="17-19" file-name="src/main.rs" caption="Usare `Pin` direttamente con la macro `pin!` per evitare allocazioni nell’_heap_ non necessarie">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-19/src/main.rs:here}}
```

</Listing>

Siamo arrivati fin qui ignorando il fatto che potremmo avere _type_ `Output`
diversi. Ad esempio, nel Listato 17-20, la _future_ anonima per `a` implementa
`Future<Output = u32>`, la _future_ anonima per `b` implementa `Future<Output =
&str>`, e la _future_ anonima per `c` implementa `Future<Output = bool>`.

<Listing number="17-20" file-name="src/main.rs" caption="Tre _future_ con _type_ distinti">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-20/src/main.rs:here}}
```

</Listing>

Possiamo usare `trpl::join!` per aspettarle, perché ci permette di passare più
_type_ di _future_ e produce una tupla di quei _type_. _Non_ possiamo usare
`trpl::join_all`, perché richiede che tutte le _future_ passate abbiano lo
stesso _type_. Ricorda, quell’errore è quello che ci ha fatto iniziare questa
avventura con `Pin`!

Questo è un compromesso fondamentale: possiamo gestire un numero dinamico di
_future_ con `join_all`, purché abbiano tutte lo stesso _type_, oppure possiamo
gestire un numero fisso di _future_ con le funzioni `join` o la macro `join!`,
anche se hanno _type_ diversi. Questo è lo stesso scenario che affronteremmo
lavorando con qualsiasi altro _type_ in Rust. Le _future_ non sono speciali,
anche se abbiamo una bella sintassi per lavorare con loro, e questo è un bene.

### Competizione tra _Future_

Quando “uniamo” le _future_ con la famiglia di funzioni e macro `join`,
richiediamo che _tutte_ finiscano prima di andare avanti. A volte, però, abbiamo
bisogno che solo _alcune_ _future_ di un insieme finiscano prima di proseguire,
un po' come mettere in competizione una _future_ contro un’altra.

Nel Listato 17-21, utilizziamo di nuovo `trpl::race` per eseguire due _future_,
`lenta` e `veloce`, l’una contro l’altra.

<Listing number="17-21" file-name="src/main.rs" caption="Utilizzo di `race` per ottenere il risultato di quale _future_ finisce prima">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-21/src/main.rs:here}}
```

</Listing>

Ogni _future_ stampa un messaggio quando inizia l’esecuzione, si ferma per un
certo periodo di tempo chiamando e aspettando `sleep`, e poi stampa un altro
messaggio quando finisce. Poi passiamo sia `lenta` che `veloce` a `trpl::race` e
aspettiamo che una di esse finisca. (Il risultato qui non è troppo sorprendente:
`veloce` vince.) A differenza di quando abbiamo usato `race` ne [“Il Nostro
Primo Programma _Async_”][async-program]<!-- ignore -->, qui ignoriamo
semplicemente l’istanza `Either` che restituisce, perché tutto il comportamento
interessante avviene nel corpo dei blocchi _async_.

Nota che se inverti l’ordine degli argomenti a `race`, l’ordine dei messaggi
“iniziati” cambia, anche se la _future_ `veloce` si conclude sempre per prima.
Questo perché l’implementazione di questa particolare funzione `race` non è
equa. Esegue sempre le _future_ passate come argomenti nell’ordine in cui sono
passate. Altre implementazioni _sono_ eque e sceglieranno casualmente quale
_future_ eseguire per prima. Indipendentemente dal fatto che l’implementazione
di _race_ che stiamo usando sia equa, però, _una_ delle _future_ eseguirà fino
al primo `await` nel suo corpo prima che un’altra attività possa iniziare.

Ricorda da [“Il Nostro Primo Programma _Async_”][async-program]<!-- ignore -->
che ad ogni punto di attesa, Rust dà a un _runtime_ la possibilità di mettere in
pausa l’attività e passare a un’altra se la _future_ in attesa non è pronta.
Anche l’inverso è vero: Rust _mette in pausa_ solo i blocchi _async_ e
restituisce il controllo a un _runtime_ in un punto di attesa. Tutto ciò che si
trova tra i punti di attesa è sincrono.

Questo significa che se fai un sacco di lavoro in un blocco _async_ senza un
punto di attesa, quella _future_ bloccherà qualsiasi altra _future_ dal fare
progressi. A volte potresti sentire questo comportamento riferito come una
_future_ che _affama_ (_starving_) altre _future_. In alcuni casi, potrebbe non
essere un grosso problema. Tuttavia, se stai facendo qualche tipo di
elaborazione dispendiosa o lavoro a lungo termine, o se hai una _future_ che
continuerà a fare un particolare compito indefinitamente, dovrai pensare a
quando e dove restituire il controllo al _runtime_.

Allo stesso modo, se hai operazioni bloccanti a lungo termine, l’_async_ può
essere uno strumento utile per fornire modi affinché diverse parti del programma
si relazionino tra loro.

Ma _come_ restituiresti il controllo al _runtime_ in quei casi?

### Restituire il Controllo al _Runtime_

Simuliamo un’operazione a lungo termine. Il Listato 17-22 introduce una funzione
`lenta`.

<Listing number="17-22" file-name="src/main.rs" caption="Utilizzo di `thread::sleep` per simulare operazioni lente">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-22/src/main.rs:slow}}
```

</Listing>

Questo codice utilizza `std::thread::sleep` invece di `trpl::sleep` in modo che
chiamare `lenta` blocchi il _thread_ corrente per un certo numero di
millisecondi. Possiamo usare `lenta` per rappresentare operazioni del mondo
reale che sono sia a lungo termine che bloccanti.

Nel Listato 17-23, utilizziamo `lenta` per emulare questo tipo di lavoro legato
alla CPU in un paio di _future_.

<Listing number="17-23" file-name="src/main.rs" caption="Due _future_ che utilizzano la funzione `lenta` per simulare operazioni di lunga durata">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-23/src/main.rs:slow-futures}}
```

</Listing>

Per cominciare, ogni future restituisce il controllo al _runtime_ _dopo_ aver
eseguito alcune operazioni lente. Se esegui questo codice, vedrai questo output:

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

Come nel nostro esempio precedente, `race` termina non appena `a` è completata.
Non c’è “intreccio” tra le due _future_, però. La _future_ `a` fa tutto il suo
lavoro fino a quando la chiamata a `trpl::sleep` è in attesa, poi la _future_
`b` fa tutto il suo lavoro fino a quando la sua chiamata a `trpl::sleep` è in
attesa, e infine la _future_ `a` finisce. Per consentire a entrambe le _future_
lente di fare progressi, abbiamo bisogno di punti di attesa in modo da poter
restituire il controllo al _runtime_ di tanto in tanto per consentire anche
all’altra di proseguire!

Possiamo già vedere questo tipo di passaggio avvenire nel Listato 17-23: se
rimuovessimo `trpl::sleep` alla fine della _future_ `a`, essa completerebbe la
propria esecuzione senza che la _future_ `b` nemmeno cominciasse. Proviamo a
utilizzare la funzione `sleep` come punto di partenza per consentire alle
operazioni di alternarsi nel fare progressi, come mostrato nel Listato 17-24.

<Listing number="17-24" file-name="src/main.rs" caption="Utilizzo di `sleep` per consentire alle operazioni di alternarsi nel fare progressi">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-24/src/main.rs:here}}
```

</Listing>

Nel Listato 17-24, aggiungiamo chiamate a `trpl::sleep` con punti di attesa tra
ogni chiamata a `lenta`. Ora il lavoro delle due _future_ è intervallato:

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

La _future_ `a` continua a lavorare per un po' prima di restituire il controllo
a `b`, perché chiama `lenta` prima di chiamare `trpl::sleep`, ma dopo ciò le
_future_ si alternano ogni volta che una di esse incontra un punto di attesa. In
questo caso, abbiamo fatto ciò dopo ogni chiamata a `lenta`, ma potremmo
suddividere il lavoro in qualsiasi modo abbia più senso per noi.

Ma non vogliamo davvero _dormire_ qui, però: vogliamo eseguire le nostre
operazioni il più velocemente possibile e restituire il controllo al _runtime_
quando possibile. Possiamo farlo direttamente, utilizzando la funzione
`yield_now`. Nel Listato 17-25, sostituiamo tutte quelle chiamate a `sleep` con
`yield_now`.

<Listing number="17-25" file-name="src/main.rs" caption="Utilizzo di `yield_now` per consentire alle operazioni di alternarsi nel fare progressi">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-25/src/main.rs:yields}}
```

</Listing>

Questo codice è sia più chiaro riguardo all’intento reale sia può essere
significativamente più veloce rispetto all’uso di `sleep`, perché i timer come
quello usato da `sleep` hanno spesso limiti su quanto possono essere granulari.
La versione di `sleep` che stiamo usando, ad esempio, dormirà sempre per almeno
un millisecondo, anche se le passiamo una `Duration` di un nanosecondo. Ancora
una volta, i computer moderni sono _veloci_: possono fare molto in un
millisecondo!

Puoi vedere questo di persona impostando un piccolo _benchmark_, come quello nel
Listato 17-26. (Questo non è un modo particolarmente rigoroso per fare test di
prestazioni, ma è sufficiente a mostrare la differenza qui.)

<Listing number="17-26" file-name="src/main.rs" caption="Confronto delle prestazioni di `sleep` e `yield_now`">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-26/src/main.rs:here}}
```

</Listing>

Qui, saltiamo tutte le stampe di stato, passiamo una `Duration` di un
nanosecondo a `trpl::sleep`, e lasciamo che ogni _future_ giri da sola, senza
alternarci tra le _future_. Poi eseguiamo per 1.000 iterazioni e vediamo quanto
tempo impiega la _future_ che utilizza `trpl::sleep` rispetto alla _future_ che
utilizza `trpl::yield_now`.

```console
versione 'sleep' finita dopo 1.1282331 secondi.
versione 'yield' finita dopo 0.000536924 secondi.
```

La versione con `yield_now` è _di gran lunga_ più veloce!

Questo significa che l’_async_ può essere utile anche per compiti legati al
calcolo, a seconda di cosa sta facendo il tuo programma, perché fornisce uno
strumento utile per strutturare le relazioni tra le diverse parti del programma.
Questa è una forma di _multitasking cooperativo_, in cui ogni _future_ ha il
potere di determinare quando restituisce il controllo tramite i punti di attesa.
Ogni _future_ ha quindi anche la responsabilità di evitare di bloccarsi troppo a
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

Possiamo anche comporre le _future_ insieme per creare nuovi schemi. Ad esempio,
possiamo costruire una funzione `timeout` con i blocchi _async_ che abbiamo già.
Quando abbiamo finito, il risultato sarà un altro blocco di costruzione che
potremmo usare per creare ancora più astrazioni _async_.

Il Listato 17-27 mostra come ci aspettiamo che funzioni questo `timeout` con una
_future_ lenta.

<Listing number="17-27" file-name="src/main.rs" caption="Utilizzo del nostro immaginato `timeout` per eseguire un’operazione lenta con un limite di tempo">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-27/src/main.rs:here}}
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

Il Listato 17-28 mostra questa dichiarazione.

<!-- Non testato, perché scritto intenzionalmente per non compoilarsi. -->

<Listing number="17-28" file-name="src/main.rs" caption="Definire la firma di `timeout`">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-28/src/main.rs:declaration}}
```

</Listing>

Questo soddisfa i nostri obiettivi per i _type_. Ora pensiamo al _comportamento_
di cui abbiamo bisogno: vogliamo far competere la _future_ passata contro la
durata fornita. Possiamo usare `trpl::sleep` per creare una _future_ che duri
quanto richiesto e usare `trpl::race` per eseguirla contro la _future_ che il
chiamante passa.

Sappiamo anche che `race` non è equa, processando gli argomenti nell’ordine in
cui sono passati. Pertanto, passiamo `future_da_testare` a `race` per prima in
modo che abbia la possibilità di completare anche se `tempo_massimo` è una
durata molto breve. Se `future_da_testare` finisce prima, `race` restituirà
`Left` con l’output da `future_da_testare`. Se il timer finisce prima, `race`
restituirà `Right` con l’output del timer di `()`.

Nel Listato 17-29, facciamo il _match_ sul risultato dell’attesa di
`trpl::race`.

<Listing number="17-29" file-name="src/main.rs" caption="Definire `timeout` con `race` e `sleep`">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-29/src/main.rs:implementation}}
```

</Listing>

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
ripetizioni, e a loro volta usarli con operazioni come chiamate di rete (uno
degli esempi dall’inizio del capitolo).

Nella pratica, di solito lavorerai direttamente con `async` e `await`, e
secondariamente con funzioni e macro come `join`, `join_all`, `race`, e così
via. Avrai bisogno di ricorrere a `pin` solo di tanto in tanto per utilizzare le
_future_ con quelle API.

Abbiamo ora visto diversi modi per lavorare con più _future_ contemporaneamente.
Prossimamente, vedremo come possiamo lavorare con più _future_ in una sequenza
nel tempo con gli _stream_. Ecco un paio di altre cose che potresti voler
considerare prima, però:

- Abbiamo usato un `Vec` con `join_all` per attendere che tutte le _future_ in
  un gruppo finissero. Come potresti usare un `Vec` per elaborare un gruppo di
  _future_ in sequenza invece? Quali sono i compromessi nel farlo?
- Dai un’occhiata al _type_ `futures::stream::FuturesUnordered` dal _crate_
  `futures`. Come sarebbe diverso usarlo rispetto a un `Vec`? (Non preoccuparti
  del fatto che provenga dalla parte `stream` del _crate_; funziona benissimo
  con qualsiasi collezione di _future_.)

[dyn]: ch12-03-improving-error-handling-and-modularity.html#restituire-errori-dalla-funzione-esegui
[enum-alt]: ch08-01-vectors.html#utilizzare-unenum-per-memorizzare-più-type
[async-program]: ch17-01-futures-and-syntax.html#il-nostro-primo-programma-async
[iterator-trait]: ch13-02-iterators.html#il-trait-iterator-e-il-metodo-next
