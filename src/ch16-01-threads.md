## Usare i Thread Per Eseguire Codice Simultaneamente

Nella maggior parte dei sistemi operativi attuali, il codice di un programma
viene eseguito in un _processo_ e il sistema operativo gestisce più processi
contemporaneamente. All'interno di un programma, è possibile avere anche parti
indipendenti che vengono eseguite simultaneamente. Le funzioni che eseguono
queste parti indipendenti sono chiamate _thread_. Ad esempio, un server web
potrebbe avere più _thread_ in modo da poter rispondere a più richieste
contemporaneamente.

Suddividere i calcoli del tuo programma in più _thread_ per eseguire più
attività contemporaneamente può migliorare le prestazioni, ma aggiunge anche
complessità. Poiché i _thread_ possono essere eseguiti simultaneamente, non c'è
alcuna garanzia intrinseca sull'ordine di esecuzione dei _thread_ del tuo
codice. Questo può portare a problemi, come ad esempio:

- Competizione (_race condition_), in cui i _thread_ accedono ai dati o alle
  risorse in un ordine incoerente
- Stallo (_deadlock_), in cui due _thread_ sono in attesa l'uno dell'altro,
  impedendo a entrambi di continuare
- Bug che si verificano solo in determinate situazioni e sono difficili da
  riprodurre e risolvere in modo affidabile

Rust attempts to mitigate the negative effects of using threads, but programming
in a multithreaded context still takes careful thought and requires a code
structure that is different from that in programs running in a single thread.

Rust cerca di mitigare gli effetti negativi dell'uso dei _thread_, ma
programmare in un contesto multi-_thread_ richiede comunque un'attenta
riflessione e una struttura del codice diversa da quella dei programmi eseguiti
in un singolo _thread_.

I linguaggi di programmazione implementano i _thread_ in diversi modi e molti
sistemi operativi forniscono un'API che il linguaggio di programmazione può
richiamare per creare nuovi _thread_. La libreria standard di Rust utilizza un
modello _1:1_ di implementazione dei _thread_, in base al quale un programma
utilizza un _thread_ del sistema operativo per ogni _thread_ del linguaggio.
Esistono dei _crate_ che implementano altri modelli di _threading_ che fanno dei
compromessi diversi rispetto al modello 1:1. (Anche il sistema _async_ di Rust,
che vedremo nel prossimo capitolo, fornisce anche un ulteriore approccio alla
concorrenza.)

### Creare un Nuovo _Thread_ con `spawn`

Per creare un nuovo _thread_, chiamiamo la funzione `thread::spawn` e le
passiamo una chiusura (abbiamo parlato delle chiusure nel [Capitolo
13][closures]) contenente il codice che vogliamo eseguire nel nuovo _thread_.
L'esempio nel Listato 16-1 stampa del testo da un _thread_ principale e altro
testo da un nuovo _thread_.

<Listing number="16-1" file-name="src/main.rs" caption="Creazione di un nuovo _thread_ per stampare una cosa mentre il _thread_ principale stampa qualcos'altro">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-01/src/main.rs}}
```

</Listing>

Note that when the main thread of a Rust program completes, all spawned threads
are shut down, whether or not they have finished running. The output from this
program might be a little different every time, but it will look similar to the
following:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
ciao numero 1 dal thread principale!
ciao numero 1 dal thread generato!
ciao numero 2 dal thread principale!
ciao numero 2 dal thread generato!
ciao numero 3 dal thread principale!
ciao numero 3 dal thread generato!
ciao numero 4 dal thread principale!
ciao numero 4 dal thread generato!
ciao numero 5 dal thread generato!
```

Le chiamate a `thread::sleep` costringono un _thread_ a interrompere la sua
esecuzione per un breve periodo, consentendo a un altro _thread_ di funzionare.
I _thread_ probabilmente si alterneranno, ma questo non è garantito: dipende da
come il sistema operativo pianifica i _thread_. In questa esecuzione, il
_thread_ principale ha stampato per primo, anche se l'istruzione di stampa del
_thread_ generato (_spawned_) appare per prima nel codice. E anche se abbiamo
detto al _thread_ generato di stampare finché `i` non è `9`, è arrivato solo a
`5` prima che il _thread_ principale si spegnesse.

Se esegui questo codice e vedi solo l'output del _thread_ principale o non vedi
alcuna sovrapposizione, prova ad aumentare i numeri negli intervalli per creare
più opportunità per il sistema operativo di passare da un _thread_ all'altro.

### Attendere Che Tutti i _Thread_ Finiscano Usando `join`

Il codice nel listato 16-1 non solo arresta il _thread_ geenrato prematuramente
nella maggior parte dei casi a causa della fine del _thread_ principale, ma
poiché non c'è alcuna garanzia sull'ordine di esecuzione dei _thread_, non
possiamo nemmeno garantire che il _thread_ generato venga eseguito!

Possiamo risolvere il problema del _thread_ generato che non viene eseguito o
che termina prematuramente salvando il valore di ritorno di `thread::spawn` in
una variabile. Il _type_ di ritorno di `thread::spawn` è `JoinHandle<T>`. Un
`JoinHandle<T>` è un valore posseduto che, quando chiamiamo il metodo `join` su
di esso, aspetterà che il suo _thread_ finisca. Il Listato 16-2 mostra come
utilizzare il `JoinHandle<T>` del _thread_ creato nel Listato 16-1 e come
chiamare `join` per assicurarsi che il _thread_ generato finisca prima che
`main` esca.

<Listing number="16-2" file-name="src/main.rs" caption="Salvare un `JoinHandle<T>` da `thread::spawn` per garantire che il _thread_ venga eseguito fino al completamento">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-02/src/main.rs}}
```

</Listing>

La chiamata a `join` sull'_handle_ blocca il _thread_ attualmente in esecuzione
fino a quando il _thread_ rappresentato dall'_handle_ non termina. _Bloccare_ un
_thread_ significa che a quel _thread_ viene impedito di eseguire lavori o di
uscire. Poiché abbiamo inserito la chiamata a `join` dopo il ciclo `for` del
_thread_ principale, l'esecuzione del listato 16-2 dovrebbe produrre un
risultato simile a questo:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
ciao numero 1 dal thread principale!
ciao numero 1 dal thread generato!
ciao numero 2 dal thread principale!
ciao numero 2 dal thread generato!
ciao numero 3 dal thread principale!
ciao numero 3 dal thread generato!
ciao numero 4 dal thread principale!
ciao numero 4 dal thread generato!
ciao numero 5 dal thread generato!
ciao numero 6 dal thread generato!
ciao numero 7 dal thread generato!
ciao numero 8 dal thread generato!
ciao numero 9 dal thread generato!
```

I due _thread_ continuano ad alternarsi, ma il _thread_ principale attende a
causa della chiamata a `handle.join()` e non termina finché il _thread_ generato
non è terminato.

Ma vediamo cosa succede se spostiamo `handle.join()` prima del ciclo `for` di
`main`, in questo modo:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/no-listing-01-join-too-early/src/main.rs}}
```

</Listing>

Il _thread_ principale aspetterà che il _thread_ generato finisca e poi eseguirà
il suo ciclo `for`, quindi l'output non sarà più alternato, come mostrato qui:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
ciao numero 1 dal thread generato!
ciao numero 2 dal thread generato!
ciao numero 3 dal thread generato!
ciao numero 4 dal thread generato!
ciao numero 5 dal thread generato!
ciao numero 6 dal thread generato!
ciao numero 7 dal thread generato!
ciao numero 8 dal thread generato!
ciao numero 9 dal thread generato!
ciao numero 1 dal thread principale!
ciao numero 2 dal thread principale!
ciao numero 3 dal thread principale!
ciao numero 4 dal thread principale!
```

Piccoli dettagli, come il punto in cui viene chiamato `join`, possono
influenzare l'esecuzione simultanea o meno dei _thread_.

### Usare le Chiusure `move` con i `Thread`

Spesso useremo la parola chiave `move` con le chiusure passate a `thread::spawn`
perché la chiusura prenderà la _ownership_ dei valori che utilizza
dall'ambiente, trasferendo così la _ownership_ di quei valori da un _thread_
all'altro. In ["Cattura di _Riference_ o Trasferimento di
_Ownership_"][capture]<!-- ignore --> del Capitolo 13, abbiamo parlato di `move`
nel contesto delle chiusure. Ora ci concentreremo maggiormente sull'interazione
tra `move` e `thread::spawn`.

Nota nel Listato 16-1 che la chiusura che passiamo a `thread::spawn` non accetta
argomenti: non stiamo utilizzando alcun dato del _thread_ principale nel codice
del _thread_ generato. Per utilizzare i dati del _thread_ principale nel
_thread_ generato, la chiusura del _thread_ generato deve catturare i valori di
cui ha bisogno. Il listato 16-3 mostra un tentativo di creare un vettore nel
_thread_ principale e di utilizzarlo nel _thread_ generato. Tuttavia, questo non
funziona ancora, come vedrai tra poco.

<Listing number="16-3" file-name="src/main.rs" caption="Tentativo di utilizzare un vettore creato dal _thread_ principale in un altro _thread_">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-03/src/main.rs}}
```

</Listing>

La chiusura utilizza `v`, quindi catturerà `v` e lo renderà parte dell'ambiente
della chiusura. Poiché `thread::spawn` esegue questa chiusura in un nuovo
_thread_, dovremmo essere in grado di accedere a `v` all'interno di questo nuovo
_thread_. Ma quando compiliamo questo esempio, otteniamo il seguente errore:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-03/output.txt}}
```

Rust _inserisce_ come catturare `v` e, poiché `println!` ha bisogno solo di un
_reference_ a `v`, la chiusura cerca di prendere in prestito `v`. Tuttavia, c'è
un problema: Rust non può sapere per quanto tempo verrà eseguito il _thread_
generato, quindi non sa se il _reference_ a `v` sarà sempre valido.

Il listato 16-4 mostra uno scenario in cui è più probabile che un _reference_ a
`v` non sia valido.

<Listing number="16-4" file-name="src/main.rs" caption="Un _thread_ con una chiusura che tenta di catturare un _reference_ a `v` da un _thread_ principale che libera `v`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-04/src/main.rs}}
```

</Listing>

Se Rust ci permettesse di eseguire questo codice, è possibile che il _thread_
generato venga immediatamente messo in background senza essere eseguito affatto.
Il _thread_ generato ha un _reference_ a `v` al suo interno, ma il _thread_
principale libera immediatamente `v`, utilizzando la funzione `drop` di cui
abbiamo parlato nel Capitolo 15. Poi, quando il _thread_ generato inizia viene
eseguito, `v` non è più valido, quindi anche il _reference_ ad esso non è
valido. Oh no!

Per risolvere l'errore del compilatore nel listato 16-3, possiamo utilizzare i
consigli del messaggio di errore:

<!-- manual-regeneration
after automatic regeneration, look at listings/ch16-fearless-concurrency/listing-16-03/output.txt and copy the relevant part
-->

```text
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++
```

Aggiungendo la parola chiave `move` prima della chiusura, obblighiamo la
chiusura a prendere _ownership_ dei valori che sta utilizzando, invece di
permettere a Rust di dedurre che deve prendere in prestito i valori. La modifica
al listato 16-3 mostrata nel listato 16-5 verrà compilata ed eseguita come
previsto.

<Listing number="16-5" file-name="src/main.rs" caption="Usare la parola chiave `move` per forzare una chiusura a prendere _ownership_ dei valori che utilizza">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-05/src/main.rs}}
```

</Listing>

Potremmo essere tentati di fare la stessa cosa per correggere il codice del
Listato 16-4 in cui il _thread_ principale chiamava `drop` utilizzando una
chiusura `move`. Tuttavia, questa correzione non funzionerà perché ciò che il
Listato 16-4 sta cercando di fare è vietato per un motivo diverso. Se
aggiungessimo `move` alla chiusura, sposteremmo `v` nell'ambiente della chiusura
e non potremmo più chiamare `drop` su di essa nel _thread_ principale.
Otterremmo invece questo errore del compilatore:

```console
{{#include ../listings/ch16-fearless-concurrency/output-only-01-move-drop/output.txt}}
```

Le regole di _ownership_ di Rust ci hanno salvato ancora una volta! Abbiamo
ricevuto un errore dal codice del Listato 16-3 perché Rust era conservativo e
prendeva in prestito solo `v` per il _thread_, il che significava che il
_thread_ principale poteva teoricamente invalidare il _reference_ del _thread_
generato. Dicendo a Rust di spostare la _ownership_ di `v` al _thread_ generato,
garantiamo a Rust che il _thread_ principale non userà più `v`. Se modifichiamo
il Listato 16-4 nello stesso modo, violeremo le regole di _ownership_ quando
cercheremo di usare `v` nel _thread_ principale. La parola chiave `move`
sovrascrive il comportamento conservativo di Rust di prendere in prestito; non
ci permette di violare le regole di _ownership_.

Ora che abbiamo analizzato cosa sono i _thread_ e i metodi forniti dall'API dei
_thread_, vediamo alcune situazioni in cui possiamo utilizzarli.

[closures]: ch13-01-closures.html
[capture]: ch13-01-closures.html#cattura-di-reference-o-trasferimento-di-ownership