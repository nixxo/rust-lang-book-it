## Uno Sguardo Più Da Vicino ai _Trait_ per _Async_

Nel corso del capitolo, abbiamo utilizzato i _trait_ `Future`, `Stream` e
`StreamExt` in vari modi. Finora, però, abbiamo evitato di addentrarci troppo
nei dettagli di come funzionano o di come interagiscono, il che va bene per la
maggior parte delle volte che li userai nel tuo lavoro quotidiano con Rust. A
volte, però, ti capiterà di incontrare situazioni in cui avrai bisogno di
comprendere queste cose più in dettaglio. In questa sezione, ci addentreremo il
giusto in questi dettagli per aiutarti in quegli scenari, lasciando comunque il
vero e proprio _approfondimento completo_ alla documentazione specifica di
quello che ti interessa.

### Il _Trait_ `Future`

Iniziamo a dare un’occhiata più da vicino a come funziona il _trait_ `Future`.
Ecco come Rust lo definisce:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

Quella definizione di _trait_ include alcuni nuovi _type_ e anche una sintassi
che non abbiamo visto prima d’ora, quindi esaminiamola un pezzo per volta.

Per prima cosa, il _type_ associato `Output` di `Future` dice in cosa si risolve
la _future_. Questo è analogo al _type_ associato `Item` per il _trait_
`Iterator`. In secondo luogo, `Future` ha il metodo `poll`, che prende un
_reference_ speciale `Pin` per il suo parametro `self` e un _reference_ mutabile
a un _type_ `Context`, e restituisce un `Poll<Self::Output>`. Parleremo più
avanti di `Pin` e `Context`. Per ora, concentriamoci su cosa restituisce il
metodo, il _type_ `Poll`:

```rust
pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

Questo _type_ `Poll` è simile a un `Option`. Ha una variante che ha un valore,
`Ready(T)`, e una che non ce l’ha, `Pending` (_in attesa_). Tuttavia, `Poll`
significa qualcosa di molto diverso da `Option`! La variante `Pending` indica
che la _future_ ha ancora lavoro da fare, quindi il chiamante dovrà controllare
di nuovo più tardi. La variante `Ready` indica che la `Future` ha finito il suo
lavoro e il valore `T` è disponibile.

> Nota: È raro dover chiamare `poll` direttamente, ma se devi, tieni a mente che
> con la maggior parte delle _future_, il chiamante non dovrebbe chiamare `poll`
> di nuovo dopo che la _future_ ha restituito `Ready`. Molte _future_ andranno
> in _panic_ se interrogate di nuovo dopo essere diventate pronte. Le _future_
> che possono essere interrogate di nuovo lo diranno esplicitamente nella loro
> documentazione. Questo è simile a come si comporta `Iterator::next`.

Quando vedi codice che usa `await`, Rust lo compila dietro le quinte in codice
che chiama `poll`. Se guardi indietro al Listato 17-4, dove abbiamo stampato il
titolo della pagina per un singolo URL, Rust lo compila in qualcosa di simile
(anche se non esattamente) a questo:

```rust,ignore
match titolo_pagina(url).poll() {
    Ready(valore) => match titolo_pagina {
        Some(titolo) => println!("Il titolo per {url} era {titolo}"),
        None => println!("{url} non aveva titolo"),
    }
    Pending => {
        // Ma cosa mettiamo qui?
    }
}
```

Cosa dovremmo fare quando la _future_ è ancora `Pending`? Abbiamo bisogno di un
modo per riprovare, e riprovare, e riprovare, fino a quando la _future_ è
finalmente pronta. In altre parole, abbiamo bisogno di un ciclo:

```rust,ignore
let mut titolo_pagina_fut = titolo_pagina(url);
loop {
    match titolo_pagina_fut.poll() {
        Ready(valore) => match titolo_pagina {
            Some(titolo) => println!("Il titolo per {url} era {titolo}"),
            None => println!("{url} non aveva titolo"),
        }
        Pending => {
            // continua
        }
    }
}
```

Se Rust lo compilasse esattamente in quel codice, però, ogni `await` sarebbe
bloccante, esattamente l’opposto di ciò che volevamo! Invece, Rust si assicura
che il ciclo possa cedere il controllo a qualcosa che può mettere in pausa il
lavoro su questa _future_ per lavorare su altre _future_ e poi controllare di
nuovo questo più tardi. Come abbiamo visto, quel qualcosa è un _runtime_
_async_, e questo lavoro di pianificazione e coordinamento è uno dei suoi
compiti principali.

Nella sezione [“Inviare Dati Tra Due _Task_ Usando il Passaggio di
Messaggi”][message-passing]<!-- ignore -->, abbiamo descritto l’attesa su
`rx.recv`. La chiamata `recv` restituisce una _future_, e attendere la _future_
la richiama. Abbiamo notato che un _runtime_ metterà in pausa la _future_ fino a
quando non è pronta con `Some(messaggio)` o `None` quando il canale si chiude.
Ora che comprendi meglio il _trait_ `Future`, e specificamente `Future::poll`,
possiamo vedere come funziona. Il _runtime_ sa che la _future_ non è pronta
quando restituisce `Poll::Pending`. Al contrario, il _runtime_ sa che la
_future_ _è_ pronta e la avanza quando `poll` restituisce
`Poll::Ready(Some(messaggio))` o `Poll::Ready(None)`.

I dettagli esatti di come un _runtime_ faccia ciò vanno oltre lo scopo di questo
libro, ma la chiave è vedere i meccanismi di base delle _future_: un _runtime_
_interroga_ ogni _future_ di cui è responsabile, rimettendo la _future_ a
dormire quando non è ancora pronta.

### Il _Type_ `Pin` e il _Trait_ `Unpin`

Nel Listato 17-13 abbiamo usato la macro `trpl::join!` per unire e aspettare tre
_future_. È tuttavia comune avere una collezione come un vettore che contiene un
certo numero di _future_ non conoscibile se non durante l’esecuzione del
programma. Apportiamo delle modifiche al Listato 17-13 per mettere le tre
_future_ in un vettore e chiamare la funzione `trpl::join_all` al posto della
macro, cosa che per ora non si compilerà.

<Listing number="17-23" file-name="src/main.rs" caption="Attesa di _future_ in una collezione">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-23/src/main.rs:here}}
```

</Listing>

Incapsuliamo ciascuna _future_ in una `Box` rendendole oggetti _trait_, proprio
come abbiamo fatto nella sezione [“Restituire Errori dalla Funzione
`esegui`”][dyn]<!-- ignore --> del Capitolo 12. (Parleremo degli oggetti _trait_
in dettaglio nel Capitolo 18.) Usare oggetti _trait_ ci permette di trattare
ciascuna delle _future_ anonime prodotte da questi _type_ come fossero il
medesimo _type_, perché tutti implementano il _trait_ `Future`.

Questo potrebbe essere sorprendente. Dopotutto, nessuno dei blocchi _async_
restituisce nulla, quindi ciascuno produce un `Future<Output = ()>`. Ricorda che
`Future` è un _trait_, e che il compilatore crea una _enum_ univoca per ogni
blocco _async_ anche se hanno _type_ di output identici. Non puoi mettere due
_struct_ scritte a mano diverse in un `Vec`, e la stessa regola si applica alle
_enum_ diverse generate dal compilatore.

Passiamo quindi la collezione di _future_ alla funzione `trpl::join_all` e
aspettiamo il risultato. Tuttavia, questa modifica non viene compilata; ecco la
parte rilevante dei messaggi di errore:

```text
{{#include ../listings/ch17-async-await/listing-17-23/output.txt:40:47}}
```

La nota in questo messaggio di errore ci dice non solo che dobbiamo usare la
macro `pin!` per fissare i valori, il che significa incapsularli nel _type_
`Pin` che garantisce il fatto che questi valori non vengano spostati nella
memoria. Il messaggio di errore dice che il _pinning_ è richiesto perché `dyn
Future<Output = ()>` deve implementare il _trait_ `Unpin`, cosa che al momento
non fa.

La funzione `trpl::join_all` restituisce una _struct_ chiamata `JoinAll`. Quella
_struct_ è generica su un _type_ `F`, che è vincolato a implementare il _trait_
`Future`. Attendere direttamente una _future_ con `await` blocca implicitamente
la _future_. Ecco perché non abbiamo bisogno di usare `pin!` ovunque vogliamo
attendere le _future_.

Tuttavia, qui non stiamo attendendo direttamente una _future_. Invece,
costruiamo un nuova _future_, JoinAll, passando una collezione di _future_ alla
funzione `join_all`. La firma per `join_all` richiede che i _type_ degli
elementi nella collezione implementino tutti il _trait_ `Future`, e `Box<T>`
implementa `Future` solo se il `T` che incapsula è una _future_ che implementa
il _trait_ `Unpin`.

Sono un sacco di informazioni da assorbire! Per capire davvero, approfondiamo un
po' di più come funziona effettivamente il _trait_ `Future`, in particolare
riguardo al _pinning_. Guarda di nuovo la definizione del _trait_ `Future`:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;

    // Metodo richiesto
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

Il parametro `cx` e il suo _type_ `Context` sono la chiave per capire come un
_runtime_ sa effettivamente quando controllare una data _future_ pur rimanendo
_lazy_. Ancora una volta, i dettagli di come ciò funzioni vanno oltre lo scopo
di questo capitolo, e generalmente devi pensare a questo solo quando scrivi
un’implementazione personalizzata di `Future`. Ci concentreremo invece sul
_type_ per `self`, poiché è la prima volta che vediamo un metodo in cui `self`
ha un’annotazione di _type_. Un’annotazione di _type_ per `self` funziona come
le annotazioni di _type_ per altri parametri di funzione ma con due differenze
chiave:

- Indica a Rust quale _type_ deve essere `self` affinché il metodo possa essere
  chiamato.
- Non può essere semplicemente qualsiasi _type_. È limitato al _type_ su cui il
  metodo è implementato, a un _reference_ o a un puntatore intelligente a quel
  _type_, o a un `Pin` che incapsula un _reference_ a quel _type_.

Vedremo di più su questa sintassi nel [Capitolo 18][ch-18]<!-- ignore -->. Per
ora, è sufficiente sapere che se vogliamo interrogare una _future_ per
controllare se è `Pending` o `Ready(Output)`, abbiamo bisogno di un _reference_
mutabile al _type_ incapsulato in `Pin`.

`Pin` è un _wrapper_ per _type_ simili a puntatori come `&`, `&mut`, `Box` e
`Rc`. (Tecnicamente, `Pin` funziona con _type_ che implementano i _trait_
`Deref` o `DerefMut`, ma questo è effettivamente equivalente a lavorare solo con
_reference_ e puntatori intelligenti.) `Pin` non è un puntatore e non ha alcun
comportamento proprio, come invece `Rc` e `Arc` fanno con il conteggio dei
_reference_; è puramente uno strumento che il compilatore può utilizzare per
imporre vincoli sull’uso dei puntatori.

Ricordando che `await` è implementato in termini di chiamate a `poll`, iniziamo
a capire il messaggio di errore che abbiamo visto in precedenza, ma quello era
in termini di `Unpin`, non di `Pin`. Quindi, come si relazionano esattamente
`Pin` e `Unpin`, e perché il `Future` ha bisogno che `self` sia in un _type_
`Pin` per chiamare `poll`?

Come menzionato in precedenza nel capitolo, una serie di punti di attesa in una
_future_ viene compilata in una macchina a stati, e il compilatore si assicura
che quella macchina a stati segua tutte le normali regole di sicurezza di Rust,
inclusi il prestito e la _ownership_. Per far funzionare tutto ciò, Rust guarda
quali dati sono necessari tra un punto di attesa e l’altro, o tra il punto di
attesa e la fine del blocco _async_. Crea quindi una variante corrispondente
nella macchina a stati compilata. Ogni variante ottiene l’accesso di cui ha
bisogno ai dati che verranno utilizzati in quella sezione del codice sorgente,
sia prendendo possesso di quei dati sia ottenendo un _reference_ mutabile o
immutabile ad essi.

Finora, tutto bene: se commettiamo errori riguardo alla _ownership_ o ai
riferimenti in un dato blocco _async_, il _borrow checker_ ce lo dirà. Quando
vogliamo spostare la _future_ che corrisponde a quel blocco, come spostarla in
un `Vec` da passare a `join_all`, le cose diventano più complicate.

Quando spostiamo una _future_, sia mettendola in una struttura dati da
utilizzare come iteratore con `join_all` o restituendola da una funzione,
significa effettivamente spostare la macchina a stati che Rust crea per noi. E a
differenza della maggior parte degli altri _type_ in Rust, le _future_ che Rust
crea per i blocchi _async_ possono finire con riferimenti a se stesse nei campi
di una data variante, come mostrato nell’illustrazione semplificata nella Figura
17-4.

<figure>

<img src="img/trpl17-04.svg" class="center" alt="Una tabella a colonna singola e
tre righe che rappresenta una `future`, fut1, che ha valori di dati 0 e 1 nelle
prime due righe e una freccia che punta dalla terza riga di nuovo alla seconda
riga, rappresentando un riferimento interno all’interno della `future`." />

<figcaption>Figura 17-4: Un tipo di dato auto-referenziale</figcaption>

</figure>

Per impostazione predefinita, però, qualsiasi oggetto che ha un riferimento a se
stesso è insicuro da spostare, perché i riferimenti puntano sempre all’indirizzo
di memoria effettivo di ciò a cui si riferiscono (vedi Figura 17-5). Se sposti
la struttura dati stessa, quei riferimenti interni rimarranno puntati alla
vecchia posizione. Tuttavia, quella posizione di memoria è ora non valida. Per
un verso, il suo valore non verrà aggiornato quando apporti modifiche alla
struttura dati. Per un altro e più importante motivo, il computer è ora libero
di riutilizzare quella memoria per altri scopi! Potresti finire per leggere dati
completamente non correlati in seguito.

<figure>

<img src="img/trpl17-05.svg" class="center" alt="Due tabelle, che raffigurano
due `future`, fut1 e fut2, ciascuna delle quali ha una colonna e tre righe,
rappresentando il risultato di aver spostato una `future` da fut1 a fut2. La
prima, fut1, è grigia, con un punto interrogativo in ciascun indice,
rappresentando una memoria sconosciuta. La seconda, fut2, ha 0 e 1 nella prima e
nella seconda riga e una freccia che punta dalla sua terza riga di nuovo alla
seconda riga di fut1, rappresentando un puntatore che fa riferimento alla
vecchia posizione in memoria della `future` prima che fosse spostata." />

<figcaption>Figura 17-5: Il risultato non sicuro di spostare un tipo di dato
auto-referenziale</figcaption>

</figure>

Teoricamente, il compilatore Rust potrebbe cercare di aggiornare ogni
riferimento a un oggetto ogni volta che viene spostato, ma ciò potrebbe
aggiungere un notevole sovraccarico di prestazioni, specialmente se un’intera
rete di riferimenti deve essere aggiornata. Se potessimo invece assicurarci che
la struttura dati in questione _non si muova in memoria_, non dovremmo
aggiornare alcun riferimento. Questo è esattamente a ciò che serve il _borrow
checker_ di Rust: nel codice sicuro, impedisce di spostare qualsiasi elemento
con un riferimento attivo.

`Pin` si basa su questo per darci la garanzia esatta di cui abbiamo bisogno.
Quando fissiamo un valore incapsulando un puntatore a quel valore in `Pin`, non
può più muoversi. Quindi, se hai `Pin<Box<QualcheType>>`, in realtà fissi il
valore `QualcheType`, _non_ il puntatore `Box`. La Figura 17-6 illustra questo
processo.

<figure>

<img src="img/trpl17-06.svg" class="center" alt="Tre scatole disposte
affiancate. La prima è etichettata “Pin”, la seconda “b1”, e la terza “pinned”.
All’interno di “pinned” c’è una tabella etichettata “fut”, con una singola
colonna; rappresenta una `future` con celle per ciascuna parte della struttura
dati. La sua prima cella ha il valore “0”, la sua seconda cella ha una freccia
che esce da essa e punta alla quarta e ultima cella, che ha il valore “1”, e la
terza cella ha linee tratteggiate e un’ellissi per indicare che potrebbero
esserci altre parti nella struttura dati. Insieme, la tabella “fut” rappresenta
una `future` che è auto-referenziale. Una freccia esce dalla scatola etichettata
“Pin”, passa attraverso la scatola etichettata “b1” e termina all’interno della
scatola “pinned” nella tabella “fut”." />

<figcaption>Figura 17-6: _Pinning_ di una `Box` che punta a un _type_ di
_future_ auto-referenziale</figcaption>

</figure>

In effetti, il puntatore `Box` può ancora muoversi liberamente. Ricorda: ci
interessa assicurarci che i dati a cui si fa riferimento rimangano al loro
posto. Se un puntatore si muove, _ma i dati a cui punta_ sono nello stesso
posto, come nella Figura 17-7, non c’è alcun problema potenziale. (Come
esercizio indipendente, dai un’occhiata alla documentazione per i _type_ così
come a quella del modulo `std::pin` e prova a capire come faresti questo con un
`Pin` che incapsula una `Box`.) La chiave è che il _type_ auto-referenziale
stesso non può muoversi, perché è ancora fissato.

<figure>

<img src="img/trpl17-07.svg" class="center" alt="Quattro scatole disposte in tre
colonne approssimative, identiche al diagramma precedente con una modifica alla
seconda colonna. Ora ci sono due scatole nella seconda colonna, etichettate “b1”
e “b2”, “b1” è grigia, e la freccia da “Pin” passa attraverso “b2” invece di
“b1”, indicando che il puntatore si è spostato da “b1” a “b2”, ma i dati in
“pinned” non si sono mossi." />

<figcaption>Figura 17-7: Spostare una `Box` che punta a un _type_ di _future_
auto-referenziale</figcaption>

</figure>

Tuttavia, la maggior parte dei _type_ è perfettamente sicura da spostare, anche
se sono incapsulati da `Pin`. Dobbiamo pensare al _pinning_ solo quando gli
elementi hanno _reference_ interni. I valori primitivi come numeri e booleani
sono sicuri perché ovviamente non hanno _reference_ interni. Né la maggior parte
dei _type_ con cui normalmente lavori in Rust. Puoi spostare un `Vec`, ad
esempio, senza preoccuparti. Dato ciò che abbiamo visto finora, se hai un
`Pin<Vec<String>>`, dovresti fare tutto tramite le API sicure ma restrittive
fornite da `Pin`, anche se un `Vec<String>` è sempre sicuro da spostare se non
ci sono altri riferimenti ad esso. Abbiamo bisogno di un modo per dire al
compilatore che va bene spostare gli elementi in casi come questo, ed è qui che
entra in gioco `Unpin`.

`Unpin` è un _trait_ marcatore, simile ai _trait_ `Send` e `Sync` che abbiamo
visto nel Capitolo 16, e quindi non ha funzionalità propria. I _trait_ marcatori
esistono solo per dire al compilatore che è sicuro utilizzare il _type_ che
implementa un dato _trait_ in un contesto particolare. `Unpin` informa il
compilatore che un dato _type_ _non_ ha bisogno di verificare alcuna garanzia
sul fatto che il valore in questione possa essere spostato in sicurezza.

Proprio come per `Send` e `Sync`, il compilatore implementa automaticamente
`Unpin` per tutti i _type_ per i quali può dimostrare che è sicuro. Un caso
speciale, di nuovo simile a `Send` e `Sync`, è dove `Unpin` _non_ è implementato
per un _type_. La notazione per questo è <code>impl !Unpin for
<em>QualcheType</em></code>, dove <code><em>QualcheType</em></code> è il nome di
un _type_ che _deve_ mantenere quelle garanzie per essere sicuro ogni volta che
un puntatore a quel _type_ viene utilizzato in un `Pin`.

In altre parole, ci sono due cose da tenere a mente riguardo alla relazione tra
`Pin` e `Unpin`. Prima di tutto, `Unpin` è il caso “normale”, e `!Unpin` è il
caso speciale. In secondo luogo, se un _type_ implementa `Unpin` o `!Unpin`
_importa solo_ quando stai usando un puntatore fissato a quel _type_ come
<code>Pin<&mut <em>QualcheType</em>></code>.

Per andare nel concreto, pensa a una `String`: ha una lunghezza e i caratteri
Unicode che la compongono. Possiamo incapsulare una `String` in `Pin`, come
visto nella Figura 17-8. Tuttavia, `String` implementa automaticamente `Unpin`,
così come la maggior parte degli altri _type_ in Rust.

<figure>

<img src="img/trpl17-08.svg" class="center" alt="Un contenitore etichettato
“Pin” sulla sinistra con una feccia che parte da esso e punta ad un contenitore
etichettato “String” sulla destra. Il contenitore “String” contiene il dato
5usize, che rappresenta la lunghezza della stringa, e le lettere “h”, “e”, “l”,
“l” e “o” rappresentanti il caratteri della stringa “hello” memorizzata in
questa istanza di String. Un rettangolo punteggiato circonda il contenitore
“String” e la sua etichetta, ma non il contenitore “Pin”." />

<figcaption>Figura 17-8: _Pinning_ di una `String`; la linea tratteggiata indica
che la `String` implementa il _trait_ `Unpin` e quindi non è
fissata</figcaption>

</figure>

Di conseguenza, possiamo fare cose che sarebbero illegali se `String`
implementasse `!Unpin`, come sostituire una stringa con un’altra nella stessa
posizione in memoria, come nella Figura 17-9. Questo non viola il contratto di
`Pin`, perché `String` non ha riferimenti interni che la rendano insicura da
spostare. È proprio per questo che implementa `Unpin` piuttosto che `!Unpin`.

<figure>

<img src="img/trpl17-09.svg" class="center" alt="La medesima stringa “hello”
dell'esempio precedente, ora etichettata “s1” e sbiadita. Il contenitore “Pin”
dell'esempio precedente ora punta ad una differente istanza di String,
etichettata “s2”, valida, con lunghezza 7usize, e contenente i caratteri della
stringa “goodbye”. s2 è circondata da un rettangolo puntinato perché, anch'essa,
implementa il trait Unpin." />

<figcaption>Figura 17-9: Sostituzione della `String` con un’altra `String`
completamente diversa in memoria</figcaption>

</figure>

Ora sappiamo abbastanza per comprendere gli errori segnalati per quella chiamata
a `join_all` dal Listato 17-23. Inizialmente abbiamo cercato di spostare le
_future_ prodotte dai blocchi _async_ in un `Vec<Box<dyn Future<Output = ()>>>`,
ma come abbiamo visto, quelle _future_ possono avere riferimenti interni, quindi
non implementano automaticamente `Unpin`. Una volta _fissate_, e poi possiamo
passare il risultante _type_ `Pin` nel `Vec`, certi che i dati sottostanti nelle
_future_ _non_ verranno spostati. Il Listato 17-24 mostra come sistemare il
codice chiamando la macro `pin!` dove ognuna delle tre future è definita e
sistemare il _type_ dell’oggetto _trait_.

<Listing number="17-24" caption="Usare `pin!` per consentire alle _future_ di essere spostate nel vettore">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-24/src/main.rs:here}}
```

</Listing>

Questo esempio ora si compila ed esegue, e possiamo aggiungere o togliere
_future_ dal vettore durante l’esecuzione aspettandole tutte.

`Pin` e `Unpin` sono principalmente importanti per costruire librerie di basso
livello, o quando stai costruendo un _runtime_ stesso, piuttosto che per il
codice Rust quotidiano. Tuttavia, quando vedi questi _trait_ nei messaggi di
errore, ora avrai un’idea migliore di come correggere il tuo codice!

> Nota: Questa combinazione di `Pin` e `Unpin` rende possibile implementare in
> modo sicuro un’intera classe di _type_ complessi in Rust che altrimenti
> risulterebbero difficili a causa della loro auto-referenzialità. I _type_ che
> richiedono `Pin` si presentano più comunemente nella programmazione asincrona
> di Rust, ma di tanto in tanto potresti vederli anche in altri contesti.
>
> I dettagli specifici su come funzionano `Pin` e `Unpin`, e le regole che
> devono rispettare, sono trattati ampiamente nella documentazione API per
> `std::pin`, quindi se sei interessato a saperne di più, quello è un ottimo
> punto di partenza.
>
> Se vuoi capire come funzionano le cose sotto il cofano in modo ancora più
> dettagliato, consulta i capitoli su [gestione dell’esecuzione][under-the-hood]
> e [pinning][pinning] di [_Asynchronous Programming in Rust_][async-book].

### Il _Trait_ `Stream`

Ora che hai una comprensione più profonda dei _trait_ `Future`, `Pin` e `Unpin`,
possiamo rivolgere la nostra attenzione al _trait_ `Stream`. Come hai appreso in
precedenza nel capitolo, gli _stream_ sono simili agli iteratori asincroni. A
differenza di `Iterator` e `Future`, tuttavia, `Stream` non ha una definizione
nella libreria standard al momento della scrittura, ma c’è _una_ definizione
molto comune dal _crate_ `futures` utilizzata in tutto l’ecosistema.

Rivediamo le definizioni dei _trait_ `Iterator` e `Future` prima di vedere come
un _trait_ `Stream` potrebbe unirli. Da `Iterator`, abbiamo l’idea di una
sequenza: il suo metodo `next` fornisce un `Option<Self::Item>`. Da `Future`,
abbiamo l’idea di prontezza nel tempo: il suo metodo `poll` fornisce un
`Poll<Self::Output>`. Per rappresentare una sequenza di elementi che diventano
pronti nel tempo, definiamo un _trait_ `Stream` che mette insieme queste
caratteristiche:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

trait Stream {
    type Item;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>
    ) -> Poll<Option<Self::Item>>;
}
```

Il _trait_ `Stream` definisce un _type_ associato chiamato `Item` per il _type_
degli elementi prodotti dallo _stream_. Questo è simile a `Iterator`, dove
possono esserci da zero a molti elementi, e a differenza di `Future`, dove c’è
sempre un singolo `Output`, anche se è il _type_ unitario `()`.

`Stream` definisce anche un metodo per ottenere quegli elementi. Lo chiamiamo
`poll_next`, per chiarire che interroga nello stesso modo in cui fa
`Future::poll` e produce una sequenza di elementi nello stesso modo in cui fa
`Iterator::next`. Il suo _type_ di ritorno combina `Poll` con `Option`. Il
_type_ esterno è `Poll`, perché deve essere controllato per prontezza, proprio
come una _future_. Il _type_ interno è `Option`, perché deve segnalare se ci
sono altri messaggi, proprio come fa un iteratore.

Qualcosa di molto simile a questa definizione diverrà probabilmente parte della
libreria standard di Rust in futuro. Nel frattempo, fa parte dell’arsenale della
maggior parte dei _runtime_, quindi puoi fare affidamento su di essa, e tutto
ciò che copriremo successivamente dovrebbe generalmente applicarsi!

Nell’esempio che abbiamo visto nella sezione [“_Stream_: _Future_ in
Sequenza”][streams]<!-- ignore -->, però, non abbiamo usato `poll_next` _o_
`Stream`, ma invece abbiamo usato `next` e `StreamExt`. Potremmo lavorare
direttamente in termini dell’API `poll_next` scrivendo a mano le nostre macchine
a stati `Stream`, ovviamente, proprio come potremmo lavorare con le _future_
direttamente tramite il loro metodo `poll`. Tuttavia, usare `await` è molto più
piacevole, e il _trait_ `StreamExt` fornisce il metodo `next` in modo da poter
fare proprio questo:

```rust
{{#rustdoc_include ../listings/ch17-async-await/no-listing-stream-ext/src/lib.rs:here}}
```

<!--
TODO: update this if/when tokio/etc. update their MSRV and switch to using async functions
in traits, since the lack thereof is the reason they do not yet have this.
-->

> Nota: La definizione effettiva che abbiamo utilizzato in precedenza nel
> capitolo appare leggermente diversa da questa, perché supporta versioni di
> Rust che non supportavano ancora l’uso di funzioni _async_ nei _trait_. Di
> conseguenza, appare in questo modo:
>
> ```rust,ignore
> fn next(&mut self) -> Next<'_, Self> where Self: Unpin;
> ```
>
> Quel _type_ `Next` è una `struct` che implementa `Future` e ci consente di
> nominare la _lifetime_ del _reference_ a `self` con `Next<'_, Self>`, in modo
> che `await` possa funzionare con questo metodo.

Il _trait_ `StreamExt` è anche la casa di tutti i metodi interessanti
disponibili per l’uso con gli _stream_. `StreamExt` è implementato
automaticamente per ogni _type_ che implementa `Stream`, ma questi _trait_ sono
definiti separatamente per consentire alla comunità di aggiungere API extra
senza influenzare il _trait_ fondamentale.

Nella versione di `StreamExt` utilizzata nel _crate_ `trpl`, il _trait_ non solo
definisce il metodo `next`, ma fornisce anche un’implementazione predefinita di
`next` che gestisce correttamente i dettagli della chiamata a
`Stream::poll_next`. Questo significa che anche quando hai bisogno di scrivere
il tuo _type_ di _stream_, devi _solo_ implementare `Stream`, e poi chiunque
utilizzi il tuo _type_ può utilizzare `StreamExt` e i suoi metodi con esso
automaticamente.

Questo è tutto ciò che tratteremo per i dettagli di basso livello su questi
_trait_. Per concludere, vedremo come _future_ (inclusi gli _stream_), _task_ e
_thread_ si integrano tutti insieme!

[message-passing]: ch17-02-concurrency-with-async.html#inviare-dati-tra-due-task-usando-il-passaggio-di-messaggi
[ch-18]: ch18-00-oop.html
[async-book]: https://rust-lang.github.io/async-book/
[under-the-hood]: https://rust-lang.github.io/async-book/02_execution/01_chapter.html
[pinning]: https://rust-lang.github.io/async-book/part-reference/pinning.html
[first-async]: ch17-01-futures-and-syntax.html#our-first-async-program
[any-number-futures]: ch17-03-more-futures.html#working-with-any-number-of-futures
[streams]: ch17-04-streams.html
