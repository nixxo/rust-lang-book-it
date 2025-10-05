## Trasferire Dati tra _Thread_ Usando il Passaggio di Messaggi

Un approccio sempre più diffuso per garantire una concomitanza sicura è il
_passaggio di messaggi_ (_message passing_), in cui i _thread_ o gli attori
comunicano inviandosi messaggi contenenti dati. Ecco l’idea in uno slogan tratto
dalla [documentazione del linguaggio
Go](https://golang.org/doc/effective_go.html#concurrency): “Non comunicare
condividendo la memoria; condividi invece la memoria comunicando.”

Per realizzare la concomitanza tramite invio di messaggi, la libreria standard
di Rust fornisce un’implementazione dei _canali_. Un _canale_ è un concetto
generale di programmazione con cui i dati vengono inviati da un _thread_
all’altro.

Puoi immaginare un canale nella programmazione come un canale d’acqua
direzionale, come un ruscello o un fiume. Se metti una paperella di gomma in un
fiume, questa viaggerà a valle fino alla fine del corso d’acqua.

Un canale ha due metà: un trasmettitore e un ricevitore. La metà del
trasmettitore è il punto a monte in cui metti la paperella di gomma nel fiume,
mentre la metà del ricevitore è il punto in cui la paperella di gomma finisce a
valle. Una parte del tuo codice chiama i metodi del trasmettitore con i dati che
vuoi inviare, mentre un’altra parte controlla la ricezione dei messaggi in
arrivo. Un canale si dice _chiuso_ se una delle due metà del trasmettitore o del
ricevitore viene abbandonata.

Qui lavoreremo su un programma che ha un _thread_ che genera valori e li invia
attraverso un canale, e un altro _thread_ che riceve i valori e li stampa. Per
illustrare la funzione, invieremo semplici valori tra i _thread_ utilizzando un
canale. Una volta che avrai acquisito familiarità con la tecnica, potrai
utilizzare i canali per qualsiasi _thread_ che abbia bisogno di comunicare tra
loro, come ad esempio un sistema di chat o un sistema in cui molti _thread_
eseguono parti di un calcolo e inviano le parti a un _thread_ che aggrega i
risultati.

Iniziamo nel Listato 16-6 creando semplicemente un canale senza fargli fare
nulla. Nota che questo non verrà ancora compilato perché Rust non può dire che
tipo di valori vogliamo inviare attraverso il canale.

<Listing number="16-6" file-name="src/main.rs" caption="Creare un canale e assegnare le due metà a `tx` e `rx`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-06/src/main.rs}}
```

</Listing>

We create a new channel using the `mpsc::channel` function; `mpsc` stands for
_multiple producer, single consumer_. In short, the way Rust’s standard library
implements channels means a channel can have multiple _sending_ ends that
produce values but only one _receiving_ end that consumes those values. Imagine
multiple streams flowing together into one big river: everything sent down any
of the streams will end up in one river at the end. We’ll start with a single
producer for now, but we’ll add multiple producers when we get this example
working.

Creiamo un nuovo canale utilizzando la funzione `mpsc::channel`; `mpsc` sta per
_multiple producer, single consumer_. In breve, il modo in cui la libreria
standard di Rust implementa i canali significa che un canale può avere più punti
di _invio_ (_produttori_) che producono valori, ma un solo punto di _ricezione_
(_consumatore_) che li riceve. Immagina più ruscelli che confluiscono in un
unico grande fiume: tutto ciò che viene inviato lungo uno qualsiasi dei ruscelli
finirà in un unico fiume alla fine. Inizieremo con un singolo produttore per
ora, ma aggiungeremo più produttori quando questo esempio funzionerà.

La funzione `mpsc::channel` restituisce una tupla, in cui il primo elemento è
l’estremità di invio - il trasmettitore - e il secondo elemento è l’estremità di
ricezione - il ricevitore. Le abbreviazioni `tx` e `rx` sono tradizionalmente
utilizzate in molti campi per indicare rispettivamente il _trasmettitore_ e il
_ricevitore_, quindi chiamiamo le nostre variabili in questo modo per indicare
ciascuna estremità. Stiamo utilizzando un’istruzione `let` con un _pattern_ che
destruttura la tupla; parleremo più approfonditamente dell’uso dei _pattern_
nelle istruzioni `let` e della destrutturazione nel Capitolo 19. Per ora, sappi
che l’utilizzo di un’istruzione `let` in questo modo è un approccio conveniente
per estrarre i pezzi della tupla restituita da `mpsc::channel`.

Spostiamo l’estremità di trasmissione in un _thread_ generato e facciamogli
inviare una stringa in modo che il _thread_ generato comunichi con il _thread_
principale, come mostrato nel Listato 16-7. Questo è come mettere una paperella
di gomma nel fiume a monte o inviare un messaggio di chat da un _thread_
all’altro.

<Listing number="16-7" file-name="src/main.rs" caption="Spostamento di `tx` in un _thread_ generato e invio di `“ciao”`">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-07/src/main.rs}}
```

</Listing>

Anche in questo caso, usiamo `thread::spawn` per generare un nuovo _thread_ e
poi usiamo `move` per spostare `tx` nella chiusura in modo che il _thread_
generato possieda `tx`. Il _thread_ generato deve possedere il trasmettitore per
poter inviare messaggi attraverso il canale.

Il trasmettitore ha un metodo `send` (_invio_) che accetta il valore che
vogliamo inviare. Il metodo `send` restituisce un _type_ `Result<T, E>`, quindi
se il ricevitore è già stato abbandonato e non c’è nessun posto dove inviare un
valore, l’operazione di invio restituirà un errore. In questo esempio, chiamiamo
`unwrap` per andare in _panic_ in caso di errore. Ma in un’applicazione reale,
lo gestiremmo in modo corretto: torna al Capitolo 9 per rivedere le strategie
per una corretta gestione degli errori.

Nel Listato 16-8, otterremo il valore dal ricevitore nel _thread_ principale. È
come recuperare la paperella di gomma dall’acqua alla fine del fiume o ricevere
un messaggio di chat.

<Listing number="16-8" file-name="src/main.rs" caption="Ricevere il valore `“ciao”` nel _thread_ principale e stamparlo">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-08/src/main.rs}}
```

</Listing>

Il ricevitore ha due metodi utili: `recv` e `try_recv`. Utilizzeremo `recv`,
abbreviazione di _receive_ (_ricevi_), che bloccherà l’esecuzione del _thread_
principale e aspetterà che un valore venga ricevuto dal canale. Una volta
ricevuto un valore, `recv` lo restituirà in un `Result<T, E>`. Quando il
trasmettitore si chiude, `recv` restituirà un errore per segnalare che non
arriveranno altri valori.

Il metodo `try_recv` invece non aspetterà, ma restituisce immediatamente un
`Result<T, E>`: un valore `Ok` che contiene un messaggio se è disponibile e un
valore `Err` se non ci sono messaggi questa volta. L’uso di `try_recv` è utile
se questo _thread_ ha altro lavoro da fare mentre aspetta i messaggi: potremmo
scrivere un ciclo che chiama `try_recv` di tanto in tanto, gestisce un messaggio
se è disponibile e altrimenti svolge altro lavoro per un po' di tempo fino a
quando non viene controllato di nuovo.

In questo esempio abbiamo usato `recv` per semplicità; non abbiamo altro lavoro
da fare per il _thread_ principale oltre all’attesa dei messaggi, quindi
bloccare il _thread_ principale è appropriato.

Quando eseguiamo il codice nel Listato 16-8, vedremo il valore stampato dal
_thread_ principale:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Ricevuto: ciao
```

Perfetto!

### Trasferire _Ownership_ Attraverso i Canali

Le regole di _ownership_ giocano un ruolo fondamentale nell’invio dei messaggi
perché ti aiutano a scrivere codice sicuro e concorrente. Prevenire gli errori
nella programmazione concorrente è il vantaggio di pensare alla _ownership_ in
tutti i tuoi programmi Rust. Facciamo un esperimento per mostrare come i canali
e la _ownership_ lavorino insieme per prevenire i problemi: proveremo a usare un
valore `val` nel _thread_ generato _dopo_ che lo abbiamo inviato nel canale.
Prova a compilare il codice nel Listato 16-9 per vedere perché questo codice non
è consentito.

<Listing number="16-9" file-name="src/main.rs" caption="Tentativo di utilizzare `val` dopo averlo inviato nel canale">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-09/src/main.rs}}
```

</Listing>

In questo caso, cerchiamo di stampare `val` dopo averlo inviato nel canale
tramite `tx.send`. Consentire questa operazione sarebbe una cattiva idea: una
volta che il valore è stato inviato a un altro _thread_, questo _thread_
potrebbe modificarlo o liberarne la memoria prima che noi cerchiamo di
utilizzarlo di nuovo. Potenzialmente, le modifiche dell’altro _thread_
potrebbero causare errori o risultati inaspettati a causa di dati incoerenti o
inesistenti. Tuttavia, Rust ci dà un errore se proviamo a compilare il codice
del Listato 16-9:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-09/output.txt}}
```

Il nostro errore di concorrenza ha causato un errore in fase di compilazione. La
funzione `send` prende _ownership_ del suo parametro e quando il valore viene
inviato, è il destinatario ne prende la _ownership_. Questo ci impedisce di
utilizzare accidentalmente il valore dopo averlo inviato; il sistema di
_ownership_ controlla che tutto sia a posto.

### Inviare Più Valori

Il codice del Listato 16-8 è stato compilato ed eseguito, ma non mostrava
chiaramente che due _thread_ separati stavano parlando tra loro attraverso il
canale.

Nel Listato 16-10 abbiamo apportato alcune modifiche che dimostreranno che il
codice del Listato 16-8 è in esecuzione simultanea: il _thread_ generato ora
invierà più messaggi e farà una pausa di un secondo tra un messaggio e l’altro.

<Listing number="16-10" file-name="src/main.rs" caption="Invio di più messaggi e pausa tra uno e l’altro">

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-10/src/main.rs}}
```

</Listing>

Questa volta, il _thread_ generato ha un vettore di stringhe che vogliamo
inviare al _thread_ principale. Le iteriamo, inviandole singolarmente, e
facciamo una pausa tra una e l’altra chiamando la funzione `thread::sleep` con
un valore `Duration` di 1 secondo.

Nel _thread_ principale, non chiamiamo più esplicitamente la funzione `recv`, ma
trattiamo `rx` come un iteratore. Per ogni valore ricevuto, lo stampiamo. Quando
il canale viene chiuso perché i messaggi inviati finiscono, l’iterazione
termina.

Quando esegui il codice del Listato 16-10, dovresti vedere il seguente output
con una pausa di 1 secondo tra una riga e l’altra:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Ricevuto: ciao
Ricevuto: dal
Ricevuto: thread
Ricevuto: !!!
```

Poiché non abbiamo alcun codice che mette in pausa o ritarda il ciclo `for` nel
_thread_ principale, possiamo dire che il _thread_ principale sta effettivamente
aspettando di ricevere i valori dal _thread_ generato.

### Creare più Produttori

Prima abbiamo detto che `mpsc` è l’acronimo di _multiple producer, single
consumer_. Mettiamo in pratica `mpsc` ed espandiamo il codice del Listato 16-10
per creare _thread_ multipli che tutti inviano i valori allo stesso ricevitore.
Possiamo farlo clonando il trasmettitore, come mostrato nel Listato 16-11.

<Listing number="16-11" file-name="src/main.rs" caption="Invio di più messaggi da più produttori">

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-11/src/main.rs:here}}
```

</Listing>

Questa volta, prima di creare il primo _thread_ generato, chiamiamo `clone` sul
trasmettitore. In questo modo avremo un nuovo trasmettitore da passare al primo
_thread_ generato. Passiamo poi il trasmettitore originale a un secondo _thread_
generato. In questo modo avremo due _thread_, ognuno dei quali invierà messaggi
diversi all’unico ricevitore.

Quando esegui il codice, l’output dovrebbe essere simile a questo:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Ricevuto: ciao
Ricevuto: altri
Ricevuto: dal
Ricevuto: messaggi
Ricevuto: thread
Ricevuto: per
Ricevuto: !!!
Ricevuto: te
```

Potresti vedere i valori in un altro ordine, a seconda del tuo sistema. Questo è
ciò che rende la concorrenza interessante e difficile. Se sperimenti con
`thread::sleep`, dandogli vari valori nei diversi _thread_, ogni esecuzione sarà
più non deterministica e creerà ogni volta un output diverso.

Ora che abbiamo visto come funzionano i canali, analizziamo un altro metodo di
concorrenza.
