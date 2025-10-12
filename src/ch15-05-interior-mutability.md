## `RefCell<T>` e il Modello di Mutabilità Interna

_Interior mutability_ è un modello di design in Rust che consente di mutare i
dati anche in presenza di _reference_ immutabili a tali dati; normalmente,
questa azione non è consentita dalle regole di prestito. Per mutare i dati, il
modello utilizza codice `unsafe` all’interno di una struttura dati per
modificare le normali regole di Rust che governano la mutabilità e il prestito.
Il codice `unsafe` indica al compilatore che stiamo controllando le regole
manualmente invece di affidarci al compilatore affinché le controlli per noi;
approfondiremo il codice `unsafe` nel Capitolo 20.

Possiamo utilizzare _type_ che utilizzano il modello di mutabilità interna solo
quando possiamo garantire che le regole di prestito vengano rispettate durante
l’esecuzione, anche se il compilatore non può garantirlo. Il codice `unsafe`
coinvolto viene quindi racchiuso in un’API sicura e il _type_ esterno rimane
immutabile.

Esploriamo questo concetto esaminando il _type_ `RefCell<T>` che segue il
modello di mutabilità interna.

### Applicare le Regole di Prestito in Fase di Esecuzione

A differenza di `Rc<T>`, il _type_ `RefCell<T>` rappresenta la _ownership_
singola sui dati che contiene. Quindi, cosa rende `RefCell<T>` diverso da un
_type_ come `Box<T>`? Ricorda le regole di prestito apprese nel Capitolo 4:

- In qualsiasi momento, puoi avere _o_ un _reference_ mutabile _o_ un numero
  qualsiasi di _reference_ immutabili (ma non entrambi).
- I _reference_ devono essere sempre validi.

Con i _reference_ e `Box<T>`, le invarianti delle regole di prestito vengono
applicate in fase di compilazione. Con `RefCell<T>`, queste invarianti vengono
applicate in fase di esecuzione. Con i _reference_, se si violano queste regole,
si otterrà un errore di compilazione. Con `RefCell<T>`, se si violano queste
regole, il programma andrà in _panic_ e si chiuderà.

I vantaggi del controllo delle regole di prestito in fase di compilazione sono
che gli errori vengono rilevati durante il processo di sviluppo e non vi è alcun
impatto sulle prestazioni in fase di esecuzione perché tutta l’analisi viene
completata in anticipo. Per questi motivi, il controllo delle regole di prestito
in fase di compilazione è la scelta migliore nella maggior parte dei casi, ed è
per questo che questa è la scelta predefinita di Rust.

Il vantaggio del controllo delle regole di prestito in fase di esecuzione è che
vengono consentiti determinati scenari di sicurezza della memoria, laddove
sarebbero stati non consentiti dai controlli in fase di compilazione. L’analisi
statica, come quella effettuata dal compilatore Rust, è intrinsecamente
conservativa. Alcune proprietà del codice sono impossibili da rilevare
analizzando il codice: l’esempio più famoso è il _problema della terminazione_
(_Halting Problem_), che esula dall’ambito di questo libro ma è un argomento
interessante da approfondire se vuoi.

Poiché alcune analisi sono impossibili, se il compilatore Rust non può essere
sicuro che il codice sia conforme alle regole di _ownership_, potrebbe rifiutare
di compilare un programma corretto; in questo modo, è conservativo. Se Rust
accettasse un programma errato, gli utenti non potrebbero fidarsi delle garanzie
fornite da Rust. Tuttavia, se Rust rifiuta di compilare un programma corretto,
il programmatore non sarà certo contento, anche se non è nulla di catastrofico.
Il _type_ `RefCell<T>` è utile quando si è certi che il codice segua le regole
di prestito, ma il compilatore non è in grado di comprenderlo e garantirlo.

Simile a `Rc<T>`, `RefCell<T>` è utilizzabile solo in scenari a _thread_ singolo
e genererà un errore in fase di compilazione se si tenta di utilizzarlo in un
contesto multi-_thread_. Parleremo di come ottenere la funzionalità di
`RefCell<T>` in un programma multi-_thread_ nel Capitolo 16.

Ecco un riepilogo delle ragioni per scegliere `Box<T>`, `Rc<T>` o `RefCell<T>`:

- `Rc<T>` consente più proprietari degli stessi dati; `Box<T>` e `RefCell<T>`
  hanno proprietari singoli.
- `Box<T>` consente prestiti immutabili o mutabili controllati in fase di
  compilazione; `Rc<T>` consente solo prestiti immutabili controllati in fase di
  compilazione; `RefCell<T>` consente prestiti immutabili o mutabili controllati
  in fase di esecuzione.
- Poiché `RefCell<T>` consente prestiti mutabili controllati in fase di
  esecuzione, è possibile modificare il valore all’interno di `RefCell<T>` anche
  quando `RefCell<T>` è immutabile.

Mutare il valore all’interno di un valore immutabile è il modello di _Interior
Mutability_ . Esaminiamo una situazione in cui la mutabilità interna è utile e
vediamo come sia possibile.

### Usare la Mutabilità Interna

Una conseguenza delle regole di prestito è che quando si ha un valore
immutabile, non è possibile prenderlo in prestito mutabilmente. Ad esempio,
questo codice non verrà compilato:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/src/main.rs}}
```

Se provassi a compilare questo codice, otterresti il seguente errore:

```console
{{#include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/output.txt}}
```

Tuttavia, ci sono situazioni in cui sarebbe utile che un valore mutasse se
stesso nei suoi metodi, ma apparisse immutabile ad altro codice. Il codice
esterno ai metodi del valore non sarebbe in grado di mutare il valore. Usare
`RefCell<T>` è un modo per ottenere la possibilità di avere una mutabilità
interna, senza però aggirare completamente le regole di prestito: il controllore
di prestito nel compilatore consente questa mutabilità interna e le regole di
prestito vengono invece verificate durante l’esecuzione. Se si violano le
regole, si otterrà un `panic!` invece di un errore del compilatore.

Esaminiamo un esempio pratico in cui possiamo usare `RefCell<T>` per mutare un
valore immutabile e vediamo perché è utile.

#### Testare con gli Oggetti _Mock_

A volte, durante i test, un programmatore usa un _type_ al posto di un altro,
per osservare un comportamento particolare e verificare che sia implementato
correttamente. Questo _type_ segnaposto è chiamato _test double_ (_doppione di
test_). Pensalo come ad una controfigura nel cinema, dove una persona interviene
e sostituisce un attore per girare una scena particolarmente difficile. I _test
double_ sostituiscono altri _type_ durante l’esecuzione dei test. Gli _oggetti
mock_ sono _type_ specifici di _test double_ che registrano ciò che accade
durante un test, in modo da poter verificare che sono state eseguite le azioni
corrette.

Rust non ha oggetti nello stesso senso in cui li hanno altri linguaggi, e Rust
non ha funzionalità di _oggetti mock_ integrate nella libreria standard come
altri linguaggi. Tuttavia, è sicuramente possibile creare una _struct_ che
svolgerà le stesse funzioni di un _oggetto mock_.

Ecco lo scenario che testeremo: creeremo una libreria che tiene traccia di un
valore rispetto a un valore massimo e invia messaggi in base a quanto il valore
corrente è vicino al valore massimo. Questa libreria potrebbe essere utilizzata,
ad esempio, per tenere traccia della quota di un utente per il numero di
chiamate API che gli è consentito effettuare.

La nostra libreria fornirà solo la funzionalità di tracciare quanto un valore è
vicino al massimo e quali messaggi dovrebbero essere inviati e in quali momenti.
Le applicazioni che utilizzano la nostra libreria dovranno fornire il meccanismo
per l’invio dei messaggi: l’applicazione potrebbe mostrare il messaggio
direttamente all’utente, inviare un’email, inviare un messaggio di testo o fare
altro. La libreria non ha bisogno di conoscere questo dettaglio. Tutto ciò di
cui ha bisogno è qualcosa che implementi un _trait_ che forniremo chiamato
`Messaggero`. Il Listato 15-20 mostra il codice della libreria.

<Listing number="15-20" file-name="src/lib.rs" caption="Una libreria per tenere traccia di quanto un valore sia vicino a un valore massimo e avvisare quando il valore raggiunge determinati livelli">

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-20/src/lib.rs}}
```

</Listing>

Una parte importante di questo codice è che il _trait_ `Messaggero` ha un metodo
chiamato `invia` che accetta un _reference_ immutabile a `self` e il testo del
messaggio. Questo _trait_ è l’interfaccia che il nostro _oggetto mock_ deve
implementare in modo che il _mock_ possa essere utilizzato allo stesso modo di
un oggetto reale. L’altra parte importante è che vogliamo testare il
comportamento del metodo `setta_valore` su `TracciaLimiti`. Possiamo modificare
ciò che passiamo per il parametro `valore`, ma `setta_valore` non restituisce
nulla su cui fare asserzioni. Vogliamo poter dire che se creiamo un
`TracciaLimiti` con qualcosa che implementa il _trait_ `Messaggero` e un valore
specifico per `max`, al messaggero viene detto di inviare i messaggi appropriati
quando passiamo numeri diversi per `valore`.

Abbiamo bisogno di un oggetto _mock_ che, invece di inviare un’email o un
messaggio di testo quando chiamiamo `invia`, tenga traccia solo dei messaggi che
gli viene detto di inviare. Possiamo creare una nuova istanza dell’oggetto
_mock_, creare un `TracciaLimiti` che utilizzi l’oggetto _mock_, chiamare il
metodo `setta_valore` su `TracciaLimiti` e quindi verificare che l’oggetto
_mock_ contenga i messaggi che ci aspettiamo. Il Listato 15-21 mostra un
tentativo di implementare un oggetto _mock_ per fare proprio questo, ma il
controllo dei prestiti non lo consente.

<Listing number="15-21" file-name="src/lib.rs" caption="Tentativo di implementare un `MockMessaggero` non consentito dal controllo dei prestiti">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-21/src/lib.rs:here}}
```

</Listing>

Questo codice di test definisce una _struct_ `MockMessaggero` che ha un campo
`messaggi_inviati` con un `Vec` di valori `String` per tenere traccia dei
messaggi che gli viene chiesto di inviare. Definiamo anche una funzione
associata `new` per semplificare la creazione di nuovi valori `MockMessaggero`
che iniziano con un elenco vuoto di messaggi. Implementiamo quindi il _trait_
`Messaggero` per `MockMessaggero` in modo da poter assegnare un `MockMessaggero`
a un `TracciaLimiti`. Nella definizione del metodo `invia`, prendiamo il
messaggio passato come parametro e lo memorizziamo nella lista `MockMessaggero`
di `messaggi_inviati`.

Nel test, stiamo testando cosa succede quando a `TracciaLimiti` viene chiesto di
impostare `valore` a un valore superiore al 75% del valore `max`. Per prima cosa
creiamo un nuovo `MockMessaggero`, che inizierà con una lista vuota di messaggi.
Quindi creiamo un nuovo `TracciaLimiti` e gli diamo un _reference_ al nuovo
`MockMessaggero` e un valore `max` di `100`. Chiamiamo il metodo `setta_valore`
su `TracciaLimiti` con un valore di `80`, che è superiore al 75% di 100. Quindi
verifichiamo che la lista di messaggi di cui `MockMessaggero` sta tenendo
traccia dovrebbe ora contenere un messaggio.

Tuttavia, c’è un problema con questo test, come mostrato qui:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-21/output.txt}}
```

Non possiamo modificare `MockMessaggero` per tenere traccia dei messaggi perché
il metodo `invia` accetta un _reference_ immutabile a `self`. Inoltre, non
possiamo accettare il suggerimento dal testo di errore di utilizzare `&mut self`
sia nel metodo `impl` che nella definizione del _trait_. Non vogliamo modificare
il _trait_ `Messaggero` solo per il funzionamento del test. Dobbiamo invece
trovare un modo per far funzionare il nostro codice di test correttamente con il
nostro design esistente.

Questa è una situazione in cui la mutabilità interna può essere d’aiuto!
Memorizzeremo `messaggi_inviati` all’interno di un `RefCell<T>`, e poi il metodo
`invia` sarà in grado di modificare `messaggi_inviati` per memorizzare i
messaggi che abbiamo visto. Il Listato 15-22 mostra come fare.

<Listing number="15-22" file-name="src/lib.rs" caption="Usare `RefCell<T>` per modificare un valore interno mentre il valore esterno è considerato immutabile">

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-22/src/lib.rs:here}}
```

</Listing>

Il campo `messaggi_inviati` è ora di _type_ `RefCell<Vec<String>>` invece di
`Vec<String>`. Nella funzione `new`, creiamo una nuova istanza di
`RefCell<Vec<String>>` incapsulando il vettore vuoto.

Per l’implementazione del metodo `send`, il primo parametro è ancora un prestito
immutabile di `self`, che corrisponde alla definizione del _trait_. Chiamiamo
`borrow_mut` su `RefCell<Vec<String>>` in `self.messaggi_inviati` per ottenere
un _reference_ mutabile al valore all’interno di `RefCell<Vec<String>>`, che è
il vettore. Quindi possiamo chiamare `push` sul _reference_ mutabile al vettore
per tenere traccia dei messaggi inviati durante il test.

L’ultima modifica che dobbiamo apportare riguarda l’asserzione: per vedere
quanti elementi ci sono nel vettore interno, chiamiamo `borrow` su
`RefCell<Vec<String>>` per ottenere un _reference_ immutabile al vettore.

Ora che hai visto come usare `RefCell<T>`, approfondiamo il suo funzionamento!

#### Tracciare i Prestiti in Fase di Esecuzione

Quando creiamo _reference_ immutabili e mutabili, utilizziamo rispettivamente la
sintassi `&` e `&mut`. Con `RefCell<T>`, utilizziamo i metodi `borrow` e
`borrow_mut`, che fanno parte dell’API sicura di `RefCell<T>`. Il metodo
`borrow` restituisce il _type_ di puntatore intelligente `Ref<T>`, mentre
`borrow_mut` restituisce il _type_ di puntatore intelligente `RefMut<T>`.
Entrambi i _type_ implementano `Deref`, quindi possiamo trattarli come normali
_reference_.

`RefCell<T>` tiene traccia di quanti puntatori intelligenti `Ref<T>` e
`RefMut<T>` sono attualmente attivi. Ogni volta che chiamiamo `borrow`,
`RefCell<T>` aumenta il conteggio dei prestiti immutabili attivi. Quando un
valore `Ref<T>` esce dallo _scope_, il conteggio dei prestiti immutabili
diminuisce di 1. Proprio come per le regole di prestito in fase di compilazione,
`RefCell<T>` ci consente di avere molti prestiti immutabili o un prestito
mutabile in qualsiasi momento.

Se proviamo a violare queste regole, anziché ottenere un errore di compilazione
come accadrebbe con i _reference_, l’implementazione di `RefCell<T>` andrà in
_panic_ in fase di esecuzione. Il Listato 15-23 mostra una modifica
dell’implementazione di `invia` nel Listato 15-22. Stiamo deliberatamente
cercando di creare due prestiti mutabili attivi nello stesso _scope_ per
dimostrare che `RefCell<T>` ci impedisce di farlo in fase di esecuzione.

<Listing number="15-23" file-name="src/lib.rs" caption="Creazione di due _reference_ mutabili nello stesso _scope_ per verificare che `RefCell<T>` generi un _panic_">

```rust,ignore,panics
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-23/src/lib.rs:here}}
```

</Listing>

Creiamo una variabile `borrow_uno` per il puntatore intelligente `RefMut<T>`
restituito da `borrow_mut`. Quindi creiamo un altro prestito mutabile allo
stesso modo nella variabile `borrow_due`. Questo crea due _reference_ mutabili
nello stesso _scope_, cosa non consentita. Quando eseguiamo i test per la nostra
libreria, il codice nel Listato 15-23 verrà compilato senza errori, ma il test
fallirà:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-23/output.txt}}
```

Nota che il codice è andato in _panic_ con il messaggio `already borrowed:
BorrowMutError`. Ecco come `RefCell<T>` gestisce le violazioni delle regole di
prestito in fase di esecuzione.

Scegliere di rilevare gli errori di prestito durante l’esecuzione anziché in
fase di compilazione, come abbiamo fatto qui, significa che potenzialmente si
troverebbero errori nel codice in una fase successiva del processo di sviluppo:
probabilmente non prima del rilascio del codice in produzione. Inoltre, il
codice subirebbe una piccola penalizzazione delle prestazioni durante
l’esecuzione a causa del monitoraggio dei prestiti durante l’esecuzione anziché
in fase di compilazione. Tuttavia, l’utilizzo di `RefCell<T>` consente di
scrivere un oggetto fittizio in grado di modificarsi per tenere traccia dei
messaggi visualizzati durante l’utilizzo in un contesto in cui sono consentiti
solo valori immutabili. È possibile utilizzare `RefCell<T>` nonostante i suoi
compromessi per ottenere più funzionalità rispetto a quelle fornite dai
_reference_ standard.

### Consentire più Proprietari di Dati Mutabili

Un modo comune per utilizzare `RefCell<T>` è in combinazione con `Rc<T>`.
Ricorda che `Rc<T>` consente di avere più proprietari di alcuni dati, ma
fornisce solo un accesso immutabile a tali dati. Se hai un `Rc<T>` che contiene
un `RefCell<T>`, puoi ottenere un valore che può avere più proprietari _e_ che
puoi mutare!

Ad esempio, ricorda l’esempio della _cons list_ nel Listato 15-18, dove abbiamo
utilizzato `Rc<T>` per consentire a più liste di condividere la proprietà di
un’altra lista. Poiché `Rc<T>` contiene solo valori immutabili, non possiamo
modificare nessuno dei valori nell’elenco una volta creato. Aggiungiamo
`RefCell<T>` per la sua capacità di modificare i valori negli elenchi. Il
Listato 15-24 mostra che utilizzando `RefCell<T>` nella definizione di `Cons`,
possiamo modificare il valore memorizzato in tutte le liste.

<Listing number="15-24" file-name="src/main.rs" caption="Utilizzo di `Rc<RefCell<i32>>` per creare una `Lista` che possiamo modificare">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-24/src/main.rs}}
```

</Listing>

Creiamo un valore che è un’istanza di `Rc<RefCell<i32>>` e lo memorizziamo in
una variabile denominata `valore` in modo da potervi accedere direttamente in
seguito. Quindi creiamo una `Lista` in `a` con una variante `Cons` che contiene
`valore`. Dobbiamo clonare `valore` in modo che sia `a` che `valore` abbiano la
_ownership_ del valore `5` interno, anziché trasferire la proprietà da `valore`
ad `a` o far sì che `a` prenda il prestito da `valore`.

Racchiudiamo la lista `a` in un `Rc<T>` in modo che quando creiamo le liste `b`
e `c`, possano entrambe fare riferimento ad `a`, come abbiamo fatto nel Listato
15-18.

Dopo aver creato le liste in `a`, `b` e `c`, vogliamo aggiungere 10 al valore in
`valore`. Lo facciamo chiamando `borrow_mut` su `valore`, che utilizza la
funzione di de-referenziazione automatica di cui abbiamo parlato in [“Dov’è
l’operatore `->`?”][wheres-the---operator]<!-- ignore --> nel Capitolo 5 per
de-referenziare `Rc<T>` al valore interno `RefCell<T>`. Il metodo `borrow_mut`
restituisce un puntatore intelligente `RefMut<T>`, su cui utilizziamo
l’operatore di de-referenziazione e modifichiamo il valore interno.

Quando stampiamo `a`, `b` e `c`, possiamo vedere che hanno tutti il valore
modificato di `15` anziché `5`:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-24/output.txt}}
```

Questa tecnica è davvero interessante! Utilizzando `RefCell<T>`, abbiamo un
valore `Lista` esternamente immutabile. Ma possiamo usare i metodi su
`RefCell<T>` che forniscono l’accesso alla sua mutabilità interna, così da poter
modificare i nostri dati quando necessario. I controlli durante l’esecuzione
delle regole di prestito ci proteggono dalle _data race_, e a volte vale la pena
sacrificare un po' di prestazioni per questa flessibilità nelle nostre strutture
dati. Nota che `RefCell<T>` non funziona per il codice _multi-thread_!
`Mutex<T>` è la versione di `RefCell<T>` che funzioni ai ambito _multi-thread_ e
ne parleremo nel Capitolo 16.

[wheres-the---operator]: ch05-03-method-syntax.html#dovè-loperatore--
