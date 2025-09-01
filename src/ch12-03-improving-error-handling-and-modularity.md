## Refactoring per Migliorare la Modularità e la Gestione degli Errori

Per migliorare il nostro programma, risolveremo quattro problemi che riguardano
la struttura del programma e la gestione di potenziali errori. Innanzitutto, la nostra funzione `main`
ora esegue due attività: analizza gli argomenti e legge i file. Man mano che il nostro
programma cresce, il numero di attività separate gestite dalla funzione `main`
aumenterà. Man mano che una funzione acquisisce responsabilità, diventa più difficile
esaminare, testare e modificare senza danneggiare una delle sue
parti. È meglio separare le funzionalità in modo che ogni funzione sia responsabile di
un'attività.

Questo problema si collega anche al secondo problema: sebbene `query` e `file_path`
siano variabili di configurazione del nostro programma, variabili come `contents` vengono utilizzate
per eseguire la struttura logica del programma. Più `main` diventa lungo, più variabili
dovremo includere nello scope; più variabili abbiamo nello scope, più difficile
sarà tenere traccia dello scopo di ciascuna. È meglio raggruppare le
variabili di configurazione in un'unica struttura per chiarirne lo scopo.

Il terzo problema è che abbiamo usato `expect` per visualizzare un messaggio di errore quando
la lettura del file fallisce, ma il messaggio di errore visualizza solo `Avrei dovuto
essere in grado di leggere il file`. La lettura di un file può fallire in diversi modi: ad esempio, il file potrebbe essere mancante o potremmo non avere i permessi per aprirlo.
Al momento, indipendentemente dalla situazione, visualizzeremo lo stesso messaggio di errore per
tutto, il che non fornirebbe alcuna informazione all'utente!

In quarto luogo, usiamo `expect` per gestire un errore e, se l'utente esegue il nostro programma
senza specificare argomenti sufficienti, riceverà un errore `index out of bounds`
da Rust che non spiega chiaramente il problema. Sarebbe meglio se tutto il
codice di gestione degli errori fosse in un unico posto, in modo che i futuri manutentori abbiano un solo posto
in cui consultare il codice se la struttura di gestione degli errori dovesse cambiare. Avere tutto il codice di gestione degli errori in un unico posto garantirà anche la stampa di messaggi
comprensibili per i nostri utenti finali.

Affrontiamo questi quattro problemi riorganizzando il nostro progetto.

### Separazione delle Attività per i Progetti Binari

Il problema organizzativo di allocare la responsabilità di più attività alla
funzione `main` è comune a molti progetti binari. Di conseguenza, molti programmatori Rust
trovano utile suddividere le attività di un programma binario
quando la funzione `main` inizia a diventare più grande. Questo processo prevede i
seguenti passaggi:

- Suddividere il programma in un file _main.rs_ e un file _lib.rs_ e spostare
la logica del programma in _lib.rs_.
- Finché la logica di analisi della riga di comando è piccola, può rimanere nella
funzione `main`.
- Quando la logica di analisi della riga di comando inizia a complicarsi, estrarla
dalla funzione `main` in altre funzioni o tipi.

Le responsabilità che rimangono nella funzione `main` dopo questo processo
dovrebbero essere limitate a quanto segue:

- Chiamare la logica di analisi della riga di comando con i valori degli argomenti
- Impostare qualsiasi altra configurazione
- Chiamare una funzione `run` in _lib.rs_
- Gestire l'errore se `run` restituisce un errore

Questo schema riguarda la separazione delle attività: _main.rs_ gestisce l'esecuzione
del programma e _lib.rs_ gestisce tutta la logica dell'attività in corso. Poiché
non è possibile testare direttamente la funzione `main`, questa struttura consente di testare tutta
la logica del programma spostandola fuori dalla funzione `main`. Il codice che
rimane nella funzione `main` sarà sufficientemente piccolo da poterne verificare la correttezza
leggendolo. Rielaboriamo il nostro programma seguendo questo processo.

#### Estrazione del Parser degli Argomenti

Estrarremo la funzionalità per l'analisi degli argomenti in una funzione che
chiamerà `main`. Il Listato 12-5 mostra il nuovo avvio della funzione `main` che
chiama una nuova funzione `parse_config`, che definiremo in _src/main.rs_.

<Listing number="12-5" file-name="src/main.rs" caption="Estrazione di una funzione `parse_config` da `main`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

</Listing>

Stiamo ancora raccogliendo gli argomenti della riga di comando in un vettore, ma invece di
assegnare il valore dell'argomento all'indice 1 alla variabile `query` e il
valore dell'argomento all'indice 2 alla variabile `file_path` all'interno della funzione `main`,
passiamo l'intero vettore alla funzione `parse_config`. La funzione
`parse_config` contiene quindi la logica che determina quale argomento
andare in quale variabile e restituisce i valori a `main`. Creiamo ancora
le variabili `query` e `file_path` in `main`, ma `main` non ha più la
responsabilità di determinare come gli argomenti e le variabili della riga di comando
corrispondono.

Questa rielaborazione potrebbe sembrare eccessiva per il nostro piccolo programma, ma stiamo eseguendo il refactoring
in piccoli passaggi incrementali. Dopo aver apportato questa modifica, esegui nuovamente il programma per
verificare che l'analisi degli argomenti funzioni ancora. È consigliabile controllare spesso i progressi
per aiutare a identificare la causa dei problemi quando si verificano.

#### Raggruppamento dei Valori di Configurazione

Possiamo fare un altro piccolo passo per migliorare ulteriormente la funzione `parse_config`.
Al momento, restituiamo una tupla, ma poi la suddividiamo immediatamente
in singole parti. Questo è un segno che forse non abbiamo ancora
l'astrazione giusta.

Un altro indicatore che mostra che c'è margine di miglioramento è la parte `config`
di `parse_config`, che implica che i due valori restituiti sono correlati e
fanno entrambi parte di un unico valore di configurazione. Al momento non stiamo evidenziando questo
significato nella struttura dei dati se non raggruppando i due valori in
una tupla; inseriremo invece i due valori in un'unica struttura e daremo a ciascuno dei
campi della struttura un nome significativo. In questo modo, sarà più facile per i futuri manutentori di questo codice comprendere come i diversi valori si relazionano tra loro
e qual è il loro scopo.

Il Listato 12-6 mostra i miglioramenti alla funzione `parse_config`.

<Listing number="12-6" file-name="src/main.rs" caption="Refactoring di `parse_config` per restituire un'istanza di una struttura `Config`">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

</Listing>

Abbiamo aggiunto una struttura denominata `Config` definita per avere campi denominati `query` e
`file_path`. La firma di `parse_config` ora indica che restituisce un
valore `Config`. Nel corpo di `parse_config`, dove prima restituivamo
slice di stringa che fanno riferimento a valori `String` in `args`, ora definiamo `Config`
in modo che contenga valori `String` di proprietà. La variabile `args` in `main` è proprietaria dei
valori degli argomenti e consente solo alla funzione `parse_config` di prenderli in prestito,
il che significa che violeremmo le regole di Rust sui prestiti se `Config` tentasse di prendere
la proprietà dei valori in `args`.

Ci sono diversi modi per gestire i dati `String`; il modo più semplice,
anche se un po' inefficiente, è chiamare il metodo `clone` sui valori.
Questo creerà una copia completa dei dati per l'istanza di `Config`, che ne diverrà proprietaria,
il che richiede più tempo e memoria rispetto alla memorizzazione di un riferimento ai dati stringa.
Tuttavia, clonare i dati rende anche il nostro codice molto semplice perché
non dobbiamo gestire la lifetime dei riferimenti; in questo caso,
rinunciare a un po' di prestazioni per guadagnare semplicità è un compromesso che vale la pena accettare.

> ### I Compromessi dell'Utilizzo di `Clone`
>
> Molti utenti di Rust tendono a evitare di usare `clone` per risolvere
> problemi di proprietà a causa del suo costo di esecuzione. Nel
> [Capitolo 13][ch13]<!-- ignore -->, imparerai come utilizzare metodi più efficienti
> in questo tipo di situazioni. Ma per ora, va bene copiare alcune
> stringhe per continuare, perché queste copie verranno eseguite solo
> una volta e il percorso del file e la stringa di query sono molto piccoli. È meglio avere
> un programma funzionante ma un po' inefficiente che cercare di iperottimizzare il codice
> al primo tentativo. Man mano che acquisirai esperienza con Rust, sarà
> più facile iniziare con la soluzione più efficiente, ma per ora è
> perfettamente accettabile chiamare `clone`.

Abbiamo aggiornato `main` in modo che inserisca l'istanza di `Config` restituita da
`parse_config` in una variabile denominata `config`, e abbiamo aggiornato il codice che
in precedenza utilizzava le variabili separate `query` e `file_path`, in modo che ora utilizzi
i campi della struttura `Config`.

Ora il nostro codice comunica più chiaramente da che `query` e `file_path` sono correlati e
che il loro scopo è configurare il funzionamento del programma. Qualsiasi codice che
utilizza questi valori sa come trovarli nell'istanza di `config` nei campi
denominati appositamente per il loro scopo.

#### Creazione di un Costruttore per `Config`

Finora, abbiamo estratto la logica responsabile dell'analisi degli argomenti della riga di comando
da `main` e l'abbiamo inserita nella funzione `parse_config`. In questo modo
abbiamo potuto verificare che i valori `query` e `file_path` erano correlati e che
questa relazione doveva essere comunicata nel nostro codice. Abbiamo quindi aggiunto una struttura `Config` per
denominare lo scopo correlato di `query` e `file_path` e per poter restituire i nomi dei
valori come nomi di campo della struttura dalla funzione `parse_config`.

Ora che lo scopo della funzione `parse_config` è creare un'istanza di `Config`,
possiamo modificare `parse_config` da una semplice funzione a una funzione
chiamata `new` associata alla struttura `Config`. Questa modifica
renderà il codice più idiomatico. Possiamo creare istanze di tipi nella
libreria standard, come `String`, chiamando `String::new`. Allo stesso modo,
modificando `parse_config` in una funzione `new` associata a `Config`, saremo
in grado di creare istanze di `Config` chiamando `Config::new`. Il Listato 12-7
mostra le modifiche che dobbiamo apportare.

<Numero di inserzione="12-7" nome-file="src/main.rs" didascalia="Modifica di `parse_config` in `Config::new`">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

</Listing>

We’ve updated `main` where we were calling `parse_config` to instead call
`Config::new`. We’ve changed the name of `parse_config` to `new` and moved it
within an `impl` block, which associates the `new` function with `Config`. Try
compiling this code again to make sure it works.

### Correggere la Gestione degli Errori

Ora correggeremo la nostra gestione degli errori. Ricorda che il tentativo di accedere
ai valori nel vettore `args` all'indice 1 o all'indice 2 causerà il
panico del programma se il vettore contiene meno di tre elementi. Prova a eseguire il programma
senza argomenti; apparirà così:

```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

La riga `index out of bounds: the len is 1 but the index is 1` è un messaggio di errore
destinato ai programmatori. Non aiuterà i nostri utenti finali a capire cosa
dovrebbero fare. Correggiamolo ora.

#### Miglioramento del messaggio di errore

Nel Listato 12-8, aggiungiamo un controllo nella funzione `new` che verificherà che la
slice sia sufficientemente lunga prima di accedere agli indici 1 e 2. Se la
slice non è sufficientemente lunga, il programma va in panico e visualizza un messaggio di errore più chiaro.

<Listing number="12-8" file-name="src/main.rs" caption="Aggiunta di un controllo per il numero di argomenti">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

</Listing>

Questo codice è simile alla [funzione `Ipotesi::new` che abbiamo scritto nel Listato
9-13][ch9-custom-types]<!-- ignore -->, dove abbiamo chiamato `panic!` quando l'argomento
`valore` era fuori dall'intervallo di valori validi. Invece di controllare
un intervallo di valori, qui controlliamo che la lunghezza di `args` sia almeno
`3` e che il resto della funzione possa funzionare presupponendo che questa
condizione sia stata soddisfatta. Se `args` ha meno di tre elementi, questa condizione
sarà `true` e chiameremo la macro `panic!` per terminare immediatamente il programma.

Con queste poche righe di codice in `new`, eseguiamo di nuovo il programma senza
argomenti per vedere come appare ora l'errore:

```console
{{#include ../listings/ch12-an-io-project/listing-12-08/output.txt}}
```

Questo output è migliore: ora abbiamo un messaggio di errore ragionevole. Tuttavia, abbiamo anche
informazioni estranee che non vogliamo fornire ai nostri utenti. Forse la
tecnica che abbiamo usato nel Listato 9-13 non è la migliore da usare in questo contesto: una chiamata a
`panic!` è più appropriata per un problema di programmazione che per un problema di utilizzo,
[come discusso nel Capitolo 9][ch9-error-guidelines]<!-- ignore -->. Invece,
utilizzeremo l'altra tecnica che hai imparato nel Capitolo 9: [restituire un
`Result`][ch9-result]<!-- ignore --> che indica un successo o un errore.

<!-- Old headings. Do not remove or links may break. -->

<a id="returning-a-result-from-new-instead-of-calling-panic"></a>

#### Restituire un `Result` Invece di Chiamare `panic!`

Possiamo invece restituire un valore `Result` che conterrà un'istanza di `Config` nel
caso di successo e descriverà il problema nel caso di errore. Cambieremo anche
il nome della funzione da `new` a `build` perché molti
programmatori si aspettano che le funzioni `new` non falliscano mai. Quando `Config::build`
comunica con `main`, possiamo usare il tipo `Result` per segnalare che si è verificato un
problema. Possiamo quindi modificare `main` per convertire una variante `Err` in un errore più pratico per i nostri utenti, senza il testo su `thread 'main'` e `RUST_BACKTRACE` causata da una chiamata a `panic!`.

Il Listato 12-9 mostra le modifiche che dobbiamo apportare al valore di ritorno della
funzione che stiamo chiamando `Config::build` e al corpo della funzione necessaria
per restituire un `Result`. Si noti che questa funzione non verrà compilata finché non aggiorneremo anche `main`,
cosa che faremo nel prossimo listato.

<Listing number="12-9" file-name="src/main.rs" caption="Restituzione di un `Result` da `Config::build`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

</Listing>

La nostra funzione `build` restituisce un `Result` con un'istanza di `Config` in caso di successo
e una stringa letterale in caso di errore. I nostri valori di errore saranno sempre
stringhe letterali con durata `'static`.

Abbiamo apportato due modifiche al corpo della funzione: invece di chiamare `panic!`
quando l'utente non passa abbastanza argomenti, ora restituiamo un valore `Err` e
abbiamo racchiuso il valore restituito da `Config` in un `Ok`. Queste modifiche rendono la
funzione conforme alla sua nuova firma di tipo.

Restituire un valore `Err` da `Config::build` consente alla funzione `main` di
gestire il valore `Result` restituito dalla funzione `build` e di uscire dal
processo in modo più pulito in caso di errore.

<!-- Old headings. Do not remove or links may break. -->

<a id="calling-confignew-and-handling-errors"></a>

#### Chiamata di `Config::build` e Gestione degli Errori

Per gestire il caso di errore e visualizzare un messaggio intuitivo, dobbiamo aggiornare
`main` per gestire il `Result` restituito da `Config::build`, come mostrato nel
Listato 12-10. Ci assumeremo anche la responsabilità di uscire dallo strumento da riga di comando
con un codice di errore diverso da zero ma senza `panic!`, e lo implementeremo manualmente. Uno stato di uscita diverso da zero è una convenzione per segnalare al processo che
chiamante che il programma è uscito con uno stato di errore.

<Listing number="12-10" file-name="src/main.rs" caption="Exiting with an error code if building a `Config` fails">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

</Listing>

In questo listato, abbiamo utilizzato un metodo che non abbiamo ancora trattato in dettaglio:
`unwrap_or_else`, definito su `Result<T, E>` dalla libreria standard.
L'utilizzo di `unwrap_or_else` ci consente di definire una gestione degli errori personalizzata, non di tipo `panic!`.
Se `Result` è un valore `Ok`, il comportamento di questo metodo è simile
a `unwrap`: restituisce il valore interno che `Ok` sta racchiudendo. Tuttavia, se il
valore è un valore `Err`, questo metodo richiama il codice nella _closure_, che è
una funzione anonima che definiamo e passiamo come argomento a `unwrap_or_else`.
Tratteremo le closure più in dettaglio nel [Capitolo 13][cap13]<!-- ignore -->. Per
ora, è sufficiente sapere che `unwrap_or_else` passerà il valore interno di
`Err`, che in questo caso è la stringa statica `"Non ci sono abbastanza argomenti"`
che abbiamo aggiunto nel Listato 12-9, alla nostra chiusura nell'argomento `err` che
appare tra i tubi verticali. Il codice nella chiusura può quindi utilizzare il
valore `err` durante l'esecuzione.

Abbiamo aggiunto una nuova riga `use` per portare `process` dalla libreria standard nell'ambito. Il codice nella chiusura che verrà eseguito in caso di errore è composto da sole due
righe: stampiamo il valore `err` e poi chiamiamo `process::exit`. La funzione
`process::exit` interromperà immediatamente il programma e restituirà il
numero che è stato passato come codice di stato di uscita. Questo è simile alla
gestione basata su `panic!` che abbiamo usato nel Listato 12-8, ma non otteniamo più tutto l'output
extra. Proviamolo:

```console
{{#include ../listings/ch12-an-io-project/listing-12-10/output.txt}}
```

Ottimo! Questo output è molto più intuitivo per i nostri utenti.

<!-- Old headings. Do not remove or links may break. -->

<a id="extracting-logic-from-main"></a>

### Estrazione della Logica dalla Funzione `main`

Ora che abbiamo completato il refactoring dell'analisi della configurazione, passiamo alla
logica del programma. Come affermato in [“Separation of Concerns for Binary
Projects”](#separation-of-concerns-for-binary-projects)<!-- ignore -->,
estrarremo una funzione denominata `run` che conterrà tutta la logica attualmente presente nella
funzione `main` che non è coinvolta nell'impostazione della configurazione o nella gestione
degli errori. Al termine, la funzione `main` sarà concisa e facile da verificare
tramite ispezione, e saremo in grado di scrivere test per tutta la restante logica.

Il Listato 12-11 mostra il piccolo miglioramento incrementale dell'estrazione di una
funzione `run`.

<Listing number="12-11" file-name="src/main.rs" caption="Estrazione di una funzione `run` contenente il resto della logica del programma">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

</Listing>

La funzione `run` ora contiene tutta la logica rimanente di `main`, a partire
dalla lettura del file. La funzione `run` accetta l'istanza `Config` come
argomento.

#### Restituzione di Errori dalla Funzione `run`

Con la logica del programma rimanente separata nella funzione `run`, possiamo
migliorare la gestione degli errori, come abbiamo fatto con `Config::build` nel Listato 12-9. Invece di lasciare che il programma vada in panico chiamando `expect`, la funzione `run`
restituirà `Result<T, E>` quando qualcosa va storto. Questo ci permetterà
di consolidare ulteriormente la logica di gestione degli errori in `main` in modo
intuitivo. Il Listato 12-12 mostra le modifiche che dobbiamo apportare alla
firma e al corpo di `run`.

<Listing number="12-12" file-name="src/main.rs" caption="Modifica della funzione `run` per restituire `Result`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

</Listing>

Abbiamo apportato tre modifiche significative. Innanzitutto, abbiamo cambiato il tipo di ritorno della
funzione `run` in `Result<(), Box<dyn Error>>`. Questa funzione in precedenza
restituiva il tipo unitario, `()`, e lo manteniamo come valore restituito nel caso
`Ok`.

Per il tipo di errore, abbiamo utilizzato l'oggetto _trait_ `Box<dyn Error>` (e abbiamo
portato `std::error::Error` nell'ambito con un'istruzione `use` all'inizio).
Tratteremo gli oggetti trait nel [Capitolo 18][cap18]<!-- ignore -->. Per ora,
sappi solo che `Box<dyn Error>` significa che la funzione restituirà un tipo che
implementa il tratto `Error`, ma non dobbiamo specificare di quale tipo specifico
sarà il valore restituito. Questo ci offre la flessibilità di restituire valori di errore che
possono essere di tipo diverso in diversi casi di errore. La parola chiave `dyn` è l'abbreviazione
di _dynamic_.

In secondo luogo, abbiamo rimosso la chiamata a `expect` a favore dell'operatore `?`, come abbiamo
illustrato nel [Capitolo 9][ch9-question-mark]<!-- ignore -->. Invece di
`panic!` in caso di errore, `?` restituirà il valore di errore dalla funzione corrente
affinché il chiamante possa gestirlo.

In terzo luogo, la funzione `run` ora restituisce un valore `Ok` in caso di successo.
Abbiamo dichiarato il tipo di successo della funzione `run` come `()` nella firma,
il che significa che dobbiamo racchiudere il valore del tipo unit nel valore `Ok`. Questa
sintassi `Ok(())` potrebbe sembrare un po' strana a prima vista, ma usare `()` in questo modo è
il modo idiomatico per indicare che stiamo chiamando `run` solo per i suoi effetti collaterali; non restituisce un valore di cui abbiamo bisogno.

Quando esegui questo codice, verrà compilato ma verrà visualizzato un avviso:

```console
{{#include ../listings/ch12-an-io-project/listing-12-12/output.txt}}
```

Rust ci dice che il nostro codice ha ignorato il valore `Result` e il valore `Result`
potrebbe indicare che si è verificato un errore. Ma non stiamo verificando se
si è verificato un errore e il compilatore ci ricorda che probabilmente intendevamo
inserire del codice di gestione degli errori! Risolviamo subito il problema.

#### Gestione degli Errori Restituiti da `run` in `main`

Verificheremo la presenza di errori e li gestiremo utilizzando una tecnica simile a quella utilizzata
con `Config::build` nel Listato 12-10, ma con una leggera differenza:

<span class="filename">File: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

Utilizziamo `if let` anziché `unwrap_or_else` per verificare se `run` restituisce un
valore `Err` e per chiamare `process::exit(1)` in tal caso. La funzione `run`
non restituisce un valore che vogliamo `unwrap` nello stesso modo in cui
`Config::build` restituisce l'istanza di `Config`. Poiché `run` restituisce `()` in
caso di successo, ci interessa solo rilevare un errore, quindi non abbiamo bisogno di `unwrap_or_else` per restituire il valore unwrap, che sarebbe solo `()`.

I corpi delle funzioni `if let` e `unwrap_or_else` sono gli stessi in
entrambi i casi: stampiamo l'errore ed usciamo.

### Suddivisione del Codice in un Library Crate

Il nostro progetto `minigrep` sembra funzionare bene finora! Ora suddivideremo il file
_src/main.rs_ e inseriremo del codice nel file _src/lib.rs_. In questo modo,
possiamo testare il codice e avere un file _src/main.rs_ con meno responsabilità.

Definiamo il codice responsabile della ricerca del testo in _src/lib.rs_ anziché
in _src/main.rs_, il che permetterà a noi (o a chiunque altro utilizzi la nostra
libreria `minigrep`) di chiamare la funzione di ricerca da più contesti rispetto al nostro
binario `minigrep`.

Per prima cosa, definiamo la firma della funzione `cerca` in _src/lib.rs_ come mostrato nel
Listato 12-13, con un corpo che richiama la macro `unimplemented!`. Spiegheremo
la firma più dettagliatamente quando completeremo l'implementazione.

<Listing number="12-13" file-name="src/lib.rs" caption="Definizione della funzione `cerca` in *src/lib.rs*">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs}}
```

</Listing>

Abbiamo utilizzato la parola chiave `pub` nella definizione della funzione per designare `cerca`
come parte dell'API pubblica del nostro contenitore di librerie. Ora abbiamo un Crate di libreria che
possiamo utilizzare dal nostro binario e che possiamo testare!

Ora dobbiamo inserire il codice definito in _src/lib.rs_ nell'ambito del
contenitore binario in _src/main.rs_ e chiamarlo, come mostrato nel Listato 12-14.

<Listing number="12-14" file-name="src/main.rs" caption="Utilizzo della funzione `cerca` del crate di libreria `minigrep` in *src/main.rs*">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

</Listing>

Aggiungiamo una riga `use minigrep::cerca` per portare la funzione `cerca` dal crate di libreria
nello scope del contenitore binario. Quindi, nella funzione `run`,
anziché stampare il contenuto del file, chiamiamo la funzione `cerca`
e passiamo il valore `config.query` e `contents` come argomenti. Quindi,
`run` utilizzerà un ciclo `for` per stampare ogni riga restituita da `cerca` che
corrisponde alla query. Questo è anche un buon momento per rimuovere le chiamate `println!` nella
funzione `main` che visualizzava la query e il percorso del file, in modo che il nostro
programma stampi solo i risultati della ricerca (se non si verificano errori).

Si noti che la funzione di ricerca raccoglierà tutti i risultati in un vettore
che restituirà prima di qualsiasi stampa. Questa implementazione potrebbe essere lenta
nel visualizzare i risultati quando si cercano file di grandi dimensioni, perché i risultati non vengono stampati man mano che
vengono trovati; discuteremo un possibile modo per risolvere questo problema utilizzando gli iteratori nel
Capitolo 13.

Uffa! È stato un duro lavoro, ma ci siamo preparati per il successo
futuro. Ora è molto più facile gestire gli errori e abbiamo reso il codice più
modulare. Quasi tutto il nostro lavoro sarà svolto in _src/lib.rs_ da ora in poi.

Sfruttiamo questa nuova modularità facendo qualcosa che
sarebbe stato difficile con il vecchio codice, ma è facile con il nuovo:
scriveremo dei test!

[ch13]: ch13-00-functional-features.html
[ch9-custom-types]: ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation
[ch9-error-guidelines]: ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling
[ch9-result]: ch09-02-recoverable-errors-with-result.html
[ch18]: ch18-00-oop.html
[ch9-question-mark]: ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
