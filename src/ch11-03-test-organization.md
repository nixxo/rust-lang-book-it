## Organizzazione dei Test

Come accennato all'inizio del capitolo, i test sono una disciplina complessa e
diverse persone tendono ad utilizzare una terminologia e un'organizzazione
diverse. La comunità di Rust pensa ai test in termini di due categorie
principali: i _test per unit_ (_unit test_) e i _test di integrazione_
(_integration test_). Gli _unit test_ sono piccoli e più mirati, testano un
singolo modulo alla volta e possono testare interfacce private. Gli _integration
test_ sono invece esterni alla tua libreria e utilizzano il tuo codice nello
stesso modo in cui lo farebbe qualsiasi altro codice esterno, utilizzando solo
l'interfaccia pubblica e potenzialmente utilizzando più moduli per test.

Scrivere entrambi i tipi di test è importante per garantire che i pezzi della
tua libreria facciano ciò che ti aspetti, sia separatamente che quando integrate
in altro codice.

### _Unit Test_

Lo scopo degli _unit test_ è quello di testare ogni unità di codice in modo
isolato dal resto del codice per individuare rapidamente i punti in cui il
codice funziona e non funziona come previsto. Gli _unit test_ vengono inseriti
nella directory _src_ in ogni file con il codice che stanno testando. La
convenzione è quella di creare un modulo chiamato `tests` in ogni file per
contenere le funzioni di test e di annotare il modulo con `cfg(test)`.

#### Il Modulo di Test e `#[cfg(test)]`

L'annotazione `#[cfg(test)]` sul modulo `tests` dice a Rust di compilare ed
eseguire il codice di test solo quando si esegue `cargo test`, non quando si
esegue `cargo build`. In questo modo si risparmia tempo di compilazione quando
si vuole costruire solo la libreria e si risparmia spazio nell'artefatto
compilato risultante perché i test non sono inclusi. Vedrai che, poiché i test
di integrazione si trovano in una directory diversa, non hanno bisogno
dell'annotazione `#[cfg(test)]`. Tuttavia, poiché gli _unit test_ si trovano
negli stessi file del codice, dovrai specificare `#[cfg(test)]` per evitare che
siano inclusi nel risultato compilato.

Ricordi che quando abbiamo generato il nuovo progetto `addizione` nella prima
sezione di questo capitolo, Cargo ha generato questo codice per noi:

<span class="filename">File: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

Nel modulo `tests` generato automaticamente, l'attributo `cfg` sta per
_configuration_ (_configurazione_) e indica a Rust che il seguente elemento deve
essere incluso solo in presenza di una determinata opzione di configurazione. In
questo caso, l'opzione di configurazione è `test`, che viene fornita da Rust per
la compilazione e l'esecuzione dei test. Utilizzando l'attributo `cfg`, Cargo
compila il nostro codice di test solo se effettivamente eseguiamo i test con
`cargo test`. Questo include qualsiasi funzione ausiliarie che potrebbero essere
presenti in questo modulo, oltre alle funzioni annotate con `#[test]`.

#### Testing di Funzioni Private

All'interno della comunità dei tester si discute se le funzioni private debbano
essere testate direttamente o meno, e altri linguaggi rendono difficile o
impossibile testare le funzioni private. Indipendentemente dall'ideologia di
testing a cui aderisci, le regole sulla privacy di Rust ti permettono di testare
le funzioni private. Considera il codice nel Listato 11-12 con la funzione
privata `addizione_privata`.

<Listing number="11-12" file-name="src/lib.rs" caption="Test di una funzione privata">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-12/src/lib.rs}}
```

</Listing>

Nota che la funzione `addizione_privata` non è contrassegnata come `pub`. I test
sono solo codice Rust e il modulo `tests` è solo un altro modulo. Come abbiamo
discusso in ["Percorsi per Fare Riferimento a un Elemento nell'Albero dei
Moduli"][paths]<!-- ignore -->, gli elementi dei moduli figli possono utilizzare
gli elementi dei loro moduli antenati. In questo test, portiamo tutti gli
elementi dei moduli genitore del modulo `tests` nello _scope_ con `use
super::*`, e poi il test può chiamare `addizione_privata`. Se non pensi che le
funzioni private debbano essere testate, non c'è nulla in Rust che ti costringa
a farlo.

### Test di Integrazione

In Rust, i test di integrazione sono esterni alla tua libreria. Utilizzano la
libreria nello stesso modo in cui lo farebbe qualsiasi altro codice, il che
significa che possono chiamare solo le funzioni che fanno parte dell'API
pubblica della libreria. Il loro scopo è quello di verificare se molte parti
della libreria funzionano correttamente insieme. Unità di codice che funzionano
correttamente da sole potrebbero avere problemi quando vengono integrate, quindi
creare test che verifichino queste funzionalità del codice integrato è
importante. Per creare i test di integrazione, hai bisogno innanzitutto di una
cartella _tests_.

#### La Cartella _tests_

Creiamo una cartella _tests_ all'inizio della nostra cartella di progetto,
accanto a _src_. Cargo sa che deve cercare i file di test di integrazione in
questa cartella. Possiamo quindi creare tutti i file di test che vogliamo e
Cargo compilerà ciascuno di essi come _crate_ separati.

Creiamo un test di integrazione. Con il codice del Listato 11-12 ancora nel file
_src/lib.rs_, crea una cartella _tests_ e un nuovo file chiamato
_tests/test_integrazione.rs_. La struttura delle cartelle del tuo progetto
dovrebbe essere simile a questa:

```text
addizione
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── test_integrazione.rs
```

Inserisci il codice del Listato 11-13 nel file _tests/test_integrazione.rs_.

<Listing number="11-13" file-name="tests/test_integrazione.rs" caption="Un test di integrazione di una funzione nel crate `addizione`">

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-13/tests/test_integrazione.rs}}
```

</Listing>

Ogni file della cartella _tests_ è un _crate_ separato, quindi dobbiamo portare
la nostra libreria nello _scope_ di ogni _crate_ di test. Per questo motivo
aggiungiamo `use addizione::aggiungi_due;` all'inizio del codice, che non era
necessario negli _unit test_ usati finora.

Non abbiamo bisogno di annotare alcun codice in _tests/test_integrazione.rs_ con
`#[cfg(test)]`. Cargo tratta la cartella _tests_ in modo speciale e compila i
file in questa directory solo quando eseguiamo `cargo test`. Esegui ora `cargo
test`:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-13/output.txt}}
```

Le tre sezioni di output comprendono gli _unit test_, i test di integrazione e i
test di documentazione. Nota che se un test di una sezione fallisce, le sezioni
successive non verranno eseguite. Ad esempio, se un _unit test_ fallisce, non ci
sarà alcun output per i test di integrazione e di documentazione perché questi
test verranno eseguiti solo se tutti gli _unit test_ passano.

La prima sezione per gli _unit test_ è la stessa che abbiamo visto finora: una
riga per ogni _unit test_ (una denominata `privata` che abbiamo aggiunto nel
Listato 11-12) e poi una riga di riepilogo per i _unit test_.

La sezione dei test di integrazione inizia con la riga `Running
test/test_integrazione.rs`. Poi, c'è una riga per ogni funzione di test in quel
test di integrazione e una riga di riepilogo dei risultati del test di
integrazione appena prima dell'inizio della sezione `Doc-tests addizione`.

Ogni file di test di integrazione ha una propria sezione, quindi se aggiungiamo
altri file nella cartella _tests_, ci saranno più sezioni di test di
integrazione.

Possiamo comunque eseguire una particolare funzione di test di integrazione
specificando il nome della funzione di test come argomento di `cargo test`. Per
eseguire tutti i test in un particolare file di test di integrazione, usa
l'argomento `--test` di `cargo test` seguito dal nome del file:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-05-single-integration/output.txt}}
```

Questo comando esegue solo i test presenti nel file _tests/test_integrazione.rs_.

#### Sottomoduli nei Test di Integrazione

Man mano che aggiungi altri test di integrazione, potresti voler creare altri
file nella cartella _tests_ per organizzarli; ad esempio, puoi raggruppare le
funzioni di test in base alla funzionalità che stanno testando. Come già detto,
ogni file nella cartella _tests_ viene compilato come un proprio crate separato,
il che è utile per creare _scope_ separati per imitare più da vicino il modo in
cui gli utenti finali utilizzeranno il tuo _crate_. Tuttavia, questo significa
che i file nella cartella _tests_ non condividono lo stesso comportamento dei
file in _src_, come hai appreso nel Capitolo 7 su come separare il codice in
moduli e file.

Il diverso comportamento dei file della cartella _tests_ si nota soprattutto quando hai una serie di funzioni comuni da utilizzare in più file di test di integrazione e cerchi di seguire i passi della sezione ["Separare i Moduli in File Diversi"][separating-modules-into-files]<!-- ignore --> del Capitolo 7 per metterle in un modulo comune. Ad esempio, se creiamo _tests/comune.rs_ e vi inseriamo una funzione chiamata `inizializzazione` a cui aggiungere del codice che vogliamo chiamare da più funzioni di test in più file di test:

<span class="filename">File: tests/comune.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/tests/comune.rs}}
```

Quando eseguiamo nuovamente i test, vedremo una nuova sezione nell'output del
test per il file _comune.rs_, anche se questo file non contiene alcuna funzione
di test né abbiamo chiamato la funzione `inizializzazione` da nessuna parte:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/output.txt}}
```

Il fatto che `comune` appaia nei risultati dei test con `running 0 tests`
(_eseguiti 0 test_) non è quello che volevamo. Volevamo solo condividere un po'
di codice con gli altri file dei test di integrazione. Per evitare che `comune`
appaia nell'output dei test, invece di creare _tests/comune.rs_, creeremo
_tests/comune/mod.rs_. La cartella del progetto ora ha questo aspetto:

```text
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── comune
    │   └── mod.rs
    └── test_integrazione.rs
```

Questa è la vecchia convenzione di denominazione comunque compresa da Rust, di
cui abbiamo parlato in ["Percorsi di File Alternativi"][alt-paths]<!-- ignore
--> nel Capitolo 7. Nominare il file in questo modo indica a Rust di non
trattare il modulo `comune` come un file di test di integrazione. Quando
spostiamo il codice della funzione `inizializzazione` in _tests/comune/mod.rs_ e
cancelliamo il file _tests/comune.rs_, la sezione nell'output del test non
apparirà più. I file nelle sottocartelle della cartella _tests_ non vengono
compilati come _crate_ separati né hanno sezioni nell'output del test.

Dopo aver creato _tests/comune/mod.rs_, possiamo utilizzarlo da qualsiasi file
di test di integrazione come modulo. Ecco un esempio di chiamata della funzione
`inizializzazione` dal test `aggiungere_due` in _tests/test_integrazione.rs_:

<span class="filename">File: tests/test_integrazione.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-13-fix-shared-test-code-problem/tests/test_integrazione.rs}}
```

Nota come la dichiarazione `mod comune;` è uguale alla dichiarazione che abbiamo
mostrato nel listato 7-21. Quindi, nella funzione di test, possiamo chiamare la
funzione `comune::inizializzazione()`.

#### Test di Integrazione per i _Crate_ Binari

Se il nostro progetto è un _crate_ binario che contiene solo un file
_src/main.rs_ e non ha un file _src/lib.rs_, non possiamo creare test di
integrazione nella cartella _tests_ e testare le funzioni definite nel file
_src/main.rs_ con una dichiarazione `use`. Solo i _crate_ libreria espongono
funzioni che altri _crate_ possono utilizzare; i _crate_ binari sono pensati per
essere eseguiti da soli.

Per questo è buona pratica per i progetti Rust che forniscono un binario avere
un file _src/main.rs_ semplice che si limita a richiamare la logica che risiede
nel file _src/lib.rs_. Utilizzando questa struttura, i test di integrazione
_possono_ testare il _crate_ libreria con `use` per rendere disponibile le
funzionalità che ci interessa testare. Se la funzionalità passa il test, anche
la piccola quantità di codice nel file _src/main.rs_ funzionerà e quella piccola
quantità di codice non dovrà essere testata.

## Riepilogo

Le funzioni di testing di Rust forniscono un modo per specificare come il codice
debba funzionare e ci si  assicuri che continui a funzionare come ci si aspetta,
anche quando si apportano delle modifiche. Gli _unit test_ usano e testano le
diverse parti di una libreria separatamente e possono testare i dettagli privati
dell'implementazione. I test di integrazione verificano che molte parti della
libreria funzionino insieme correttamente e utilizzano l'API pubblica della
libreria per testare il codice nello stesso modo in cui lo utilizzerà il codice
esterno. Anche se il sistema dei _type_ e le regole di _ownership_ di Rust
aiutano a prevenire alcuni tipi di bug, i test sono comunque importanti per
ridurre i bug logici che hanno a che fare con il modo in cui ci si aspetta che
il codice si comporti.

Combiniamo le conoscenze apprese in questo capitolo e nei capitoli precedenti
per lavorare a un progetto!

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[separating-modules-into-files]: ch07-05-separating-modules-into-different-files.html
[alt-paths]: ch07-05-separating-modules-into-different-files.html#percorsi-di-file-alternativi
