## Concorrenza a Stato Condiviso

Il passaggio di messaggi è un buon modo per gestire la concorrenza, ma non è
l’unico. Un altro metodo potrebbe essere quello di far accedere più _thread_
agli stessi dati condivisi. Considera ancora una volta questa parte dello slogan
della documentazione del linguaggio Go: “Non comunicare condividendo la
memoria”.

Come funzionerebbe comunicare condividendo la memoria? Inoltre, perché gli
appassionati della tecnica del passaggio di messaggi raccomandano di non
utilizzare la condivisione della memoria?

In un certo senso, i canali in qualsiasi linguaggio di programmazione sono
simili alla proprietà singola, perché una volta che trasferisci un valore lungo
un canale, non dovresti più utilizzarlo. La concorrenza della memoria condivisa
è simile alla proprietà multipla: più _thread_ possono accedere alla stessa
posizione di memoria nello stesso momento. Come hai visto nel Capitolo 15, dove
i puntatori intelligenti rendono possibile la _ownership_ multipla, questo può
aggiungere complessità perché questi diversi proprietari devono essere gestiti.
Il sistema dei _type_ e le regole di _ownership_ di Rust aiutano molto a gestire
correttamente questo aspetto. Per fare un esempio, vediamo i _mutex_, uno dei
_type_ primitivi di concorrenza più comuni per la memoria condivisa.

### Controllare l’Accesso con i _Mutex_

_Mutex_ è l’abbreviazione di _mutual exclusion_ (_mutua esclusione_), ovvero un
_mutex_ permette a un solo _thread_ di accedere ad alcuni dati in un determinato
momento. Per accedere ai dati di un _mutex_, un _thread_ deve prima segnalare
che vuole accedervi chiedendo di acquisire il blocco del _mutex_. Il _blocco_
(_lock_) è una struttura di dati che fa parte del _mutex_ e che tiene traccia di
chi attualmente ha accesso esclusivo ai dati. Per questo motivo, il _mutex_ può
esser visto come un _custode_ di dati a cui garantisce accesso tramite il
sistema di blocco.

I _mutex_ hanno la reputazione di essere difficili da usare perché devi
ricordare due regole:

1. Devi cercare di acquisire il blocco prima di utilizzare i dati.
1. Quando hai finito di utilizzare i dati che il _mutex_ custodisce, devi
   sbloccare i dati in modo che altri _thread_ possano acquisirne il blocco.

Per una metafora del mondo reale di un _mutex_, immagina una tavola rotonda a
una conferenza con un solo microfono. Prima che un relatore possa parlare, deve
chiedere o segnalare che vuole usare il microfono. Quando ottiene il microfono,
può parlare per tutto il tempo che vuole e poi passare il microfono al relatore
successivo che chiede di parlare. Se un relatore dimentica di passare il
microfono quando ha finito, nessun altro potrà parlare. Se chi gestisce la
condivisione del microfono condiviso non fa il suo lavoro correttamente, la
tavola rotonda non funzionerà come previsto!

La gestione dei _mutex_ può essere incredibilmente complicata, per questo molte
persone sono entusiaste dei canali. Tuttavia, grazie al sistema dei _type_ e
alle regole di _ownership_ di Rust, non è possibile sbagliare il blocco e lo
sblocco.

#### L’API di `Mutex<T>`

Come esempio di utilizzo di un _mutex_, iniziamo con l’utilizzo di un _mutex_ in
un contesto a _thread_ singolo, come mostrato nel Listato 16-12.

<Listing number="16-12" file-name="src/main.rs" caption="Uso dell’API di `Mutex<T>` in un contesto a _thread_ singolo per semplicità">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-12/src/main.rs}}
```

</Listing>

Come per molti altri _type_, creiamo un `Mutex<T>` utilizzando la funzione
associata `new`. Per accedere ai dati all’interno del _mutex_, utilizziamo il
metodo `lock` per acquisire il blocco. Questa chiamata bloccherà il _thread_
corrente in modo che non possa svolgere alcuna attività finché non sarà il
nostro turno di avere il blocco.

La chiamata a `lock` fallirebbe se un altro _thread_ che detiene il lock andasse
in _panic_. In questo caso, nessuno sarebbe in grado di ottenere il lock, quindi
abbiamo scelto di usare `unwrap` e di far andare in _panic_ questo _thread_ se
ci troviamo in quella situazione.

Dopo aver acquisito il blocco, possiamo trattare il valore di ritorno, chiamato
`num` in questo caso, come un _reference_ mutabile ai dati all’interno. Il
sistema dei _type_ assicura che acquisiamo un blocco prima di utilizzare il
valore in `m`. Il _type_ di `m` è `Mutex<i32>`, non `i32`, quindi dobbiamo
chiamare `lock` per poter utilizzare il valore `i32`. Non possiamo
dimenticarcene; altrimenti il sistema dei _type_ non ci permetterà di accedere
al valore interno `i32`.

La chiamata a `lock` restituisce un _type_ chiamato `MutexGuard`, incapsulato in
un `LockResult` che abbiamo gestito con la chiamata a `unwrap`. Il _type_
`MutexGuard` implementa `Deref` per puntare ai nostri dati interni; il _type_ ha
anche un’implementazione `Drop` che rilascia automaticamente il blocco quando un
`MutexGuard` esce dallo _scope_, cosa che accade alla fine dello _scope_
interno. Di conseguenza, non rischiamo di dimenticarci di rilasciare il blocco e
di bloccare l’utilizzo del _mutex_ da parte di altri _thread_ perché il rilascio
del blocco avviene automaticamente.

Dopo aver rilasciato il blocco, possiamo stampare il valore del _mutex_ e vedere
che siamo riusciti a cambiare l’interno `i32` in `6`.

#### Condividere Accesso a `Mutex<T>`

Ora proviamo a condividere un valore tra più _thread_ utilizzando `Mutex<T>`.
Avvieremo 10 _thread_ e faremo in modo che ognuno di essi incrementi il valore
di un contatore di 1, in modo che il contatore vada da 0 a 10. L’esempio nel
Listato 16-13 avrà un errore del compilatore, che useremo per imparare di più
sull’uso di `Mutex<T>` e su come Rust ci aiuta a usarlo correttamente.

<Listing number="16-13" file-name="src/main.rs" caption="Dieci _thread_, ognuno dei quali incrementa un contatore custodito da un `Mutex<T>`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-13/src/main.rs}}
```

</Listing>

Creiamo una variabile `contatore` per contenere un `i32` all’interno di un
`Mutex<T>`, come abbiamo fatto nel Listato 16-12. Poi creiamo 10 _thread_
iterando su un intervallo di numeri. Usiamo `thread::spawn` e diamo a tutti i
_thread_ la stessa chiusura: una che sposta il contatore nel _thread_,
acquisisce un blocco sul `Mutex<T>` chiamando il metodo `lock` e poi aggiunge 1
al valore nel mutex. Quando un _thread_ termina l’esecuzione della sua chiusura,
`num` uscirà dallo _scope_ e rilascerà il blocco in modo che un altro _thread_
possa acquisirlo.

Nel _thread_ principale, sugli _handle_ dei _thread_ raccolti in un vettore,
come fatto nel Listato 16-2, chiamiamo `join` su ognuno di essi per assicurarci
che tutti i _thread_ finiscano. A quel punto, il _thread_ principale acquisirà
il blocco e stamperà il risultato di questo programma.

Abbiamo accennato al fatto che questo esempio non sarebbe stato compilato. Ora
scopriamo perché!

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-13/output.txt}}
```

Il messaggio di errore indica che il valore `contatore` è stato spostato
nell’iterazione precedente del ciclo. Rust ci sta dicendo che non possiamo
spostare la _ownership_ del blocco `contatore` in più _thread_. Risolviamo
l’errore del compilatore con il metodo della _ownership_ multipla di cui abbiamo
parlato nel Capitolo 15.

#### _Ownership_ Multipla con _Thread_ Multipli

Nel Capitolo 15, abbiamo dato un valore a più proprietari utilizzando il
puntatore intelligente `Rc<T>` per creare un conteggio di _reference_. Facciamo
lo stesso qui e vediamo cosa succede. Incapsuleremo il `Mutex<T>` in `Rc<T>` nel
Listato 16-14 e cloneremo `Rc<T>` prima di spostare la _ownership_ al _thread_.

<Listing number="16-14" file-name="src/main.rs" caption="Tentativo di utilizzare `Rc<T>` per consentire a più _thread_ di possedere il `Mutex<T>`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-14/src/main.rs}}
```

</Listing>

Ancora una volta, compiliamo e otteniamo... errori diversi! Il compilatore ci
sta insegnando molto.

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-14/output.txt}}
```

Wow, questo messaggio di errore è molto prolisso! Ecco la parte importante su
cui concentrarsi: `` `Rc<Mutex<i32>>` cannot be sent between threads safely ``
(`Rc<Mutex<i32>>` non può essere inviato tra i _thread_ in modo sicuro). Il
compilatore ci dice anche il motivo: `` the trait `Send` is not implemented for
`Rc<Mutex<i32>>` (`` (il _trait_ `Send` non è implementato per
`Rc<Mutex<i32>>`). Parleremo di `Send` nella prossima sezione: è uno dei _trait_
che garantisce che i _type_ che utilizziamo con i _thread_ siano pensati per
l’uso in situazioni concorrenti.

Sfortunatamente, `Rc<T>` non è sicuro da condividere tra i _thread_. Quando
`Rc<T>` gestisce il conteggio dei _reference_, aggiunge al conteggio per ogni
chiamata a `clone` e sottrae dal conteggio quando ogni clone viene rilasciato.
Ma non utilizza alcun _type_ primitivo di concorrenza per assicurarsi che le
modifiche al conteggio non possano essere interrotte da un altro _thread_.
Questo potrebbe portare a conteggi sbagliati; bug che potrebbero a loro volta
portare a perdite di memoria o alla de-allocazione di un valore prima che
abbiamo finito di usarlo. Ciò di cui abbiamo bisogno è un _type_ che sia
esattamente come `Rc<T>`, ma che apporti modifiche al conteggio dei _reference_
in modo sicuro quando usato con i _thread_.

#### Conteggio di _Reference_ Atomico con `Arc<T>`

Fortunatamente, `Arc<T>` _è_ un _type_ come `Rc<T>` che è sicuro da usare in
situazioni di concorrenza (_thread-safe_). La _A_ sta per _atomico_, cioè è un
_type_ contatore di _reference_ _atomico_. Gli _atomici_ sono un ulteriore
_type_ di primitivo di concorrenza che non tratteremo in dettaglio in questa
sede: per maggiori dettagli, consulta la documentazione della libreria standard
per [`std::sync::atomic`][atomic]<!-- ignore -->. A questo punto, ti basterà
sapere che gli atomici funzionano come i _type_ primitivi ma sono sicuri da
condividere tra i _thread_.

Potresti chiederti perché tutti i _type_ primitivi non sono atomici e perché i
_type_ della libreria standard non sono implementati in modo da utilizzare
`Arc<T>` come impostazione predefinita. Il motivo è che la sicurezza dei
_thread_ comporta una penalizzazione delle prestazioni che vorrai scegliere di
usare solo quando ne hai veramente bisogno. Se stai eseguendo operazioni su
valori all’interno di un singolo _thread_, il tuo codice può funzionare più
velocemente se non deve applicare le garanzie che gli atomici forniscono.

Torniamo al nostro esempio: `Arc<T>` e `Rc<T>` hanno la stessa API, quindi
correggiamo il nostro programma cambiando la riga `use`, la chiamata a `new` e
la chiamata a `clone`. Il codice nel Listato 16-15 verrà finalmente compilato ed
eseguito.

<Listing number="16-15" file-name="src/main.rs" caption="Utilizzo di un `Arc<T>` per incapsulare il `Mutex<T>` per poter condividere la _ownership_ tra più _thread_">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-15/src/main.rs}}
```

</Listing>

Questo codice stamperà quanto segue:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Risultato: 10
```

Ce l’abbiamo fatta! Abbiamo contato da 0 a 10, il che può non sembrare molto
impressionante, ma ci ha insegnato molto su `Mutex<T>` e sulla sicurezza dei
_thread_. Puoi anche utilizzare la struttura di questo programma per fare
operazioni più complicate del semplice incremento di un contatore. Utilizzando
questa strategia, puoi dividere un calcolo in parti indipendenti, suddividere
queste parti tra i vari _thread_ e poi utilizzare un `Mutex<T>` per far sì che
ogni _thread_ aggiorni il risultato finale con la sua parte.

Nota che se stai eseguendo semplici operazioni numeriche, esistono _type_ più
semplici di `Mutex<T>` forniti dal modulo [`std::sync::atomic` della libreria
standard][atomic]<!-- ignore -->. Questi _type_ forniscono un accesso sicuro,
concorrente e atomico ai _type_ primitivi. Abbiamo scelto di utilizzare
`Mutex<T>` con un _type_ primitivo per questo esempio, in modo da poterci
concentrare sul funzionamento di `Mutex<T>`.

### Comparazione tra `RefCell<T>`/`Rc<T>` e `Mutex<T>`/`Arc<T>`

Avrai notato che `contatore` è immutabile ma possiamo ottenere un _reference_
mutabile al valore al suo interno; questo significa che `Mutex<T>` fornisce la
mutabilità interna, come fa la famiglia `Cell`. Nello stesso modo in cui abbiamo
usato `RefCell<T>` nel Capitolo 15 per permetterci di mutare i contenuti
all’interno di un `Rc<T>`, usiamo `Mutex<T>` per mutare i contenuti all’interno
di un `Arc<T>`.

Un altro dettaglio da notare è che Rust non può proteggerti da tutti i tipi di
errori logici quando usi `Mutex<T>`. Ricordiamo dal Capitolo 15 che l’uso di
`Rc<T>` comporta il rischio di creare cicli di riferimento, in cui due valori
`Rc<T>` fanno riferimento l’uno all’altro, causando perdite di memoria. Allo
stesso modo, `Mutex<T>` comporta il rischio di creare dei _deadlock_ (_stallo_).
Questi si verificano quando un’operazione deve bloccare due risorse e due
_thread_ hanno acquisito ciascuno uno dei blocchi, facendoli attendere
all’infinito l’un l’altro. Se ti interessano i _deadlock_, prova a creare un
programma Rust che abbia un _deadlock_; quindi ricerca le strategie di
mitigazione degli stalli per i _mutex_ in qualsiasi altro linguaggio e prova a
implementarle in Rust. La documentazione API della libreria standard per
`Mutex<T>` e `MutexGuard` offre informazioni utili.

Concluderemo questo capitolo parlando dei _trait_ `Send` e `Sync` e di come
possiamo utilizzarli con i _type_ personalizzati.

[atomic]: https://doc.rust-lang.org/stable/std/sync/atomic/index.html
