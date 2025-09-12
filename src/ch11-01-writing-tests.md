## Come Scrivere dei Test

I test sono funzioni di Rust che verificano che il codice non di test funzioni
nel modo previsto. I corpi delle funzioni di test eseguono tipicamente queste
tre azioni:

- Impostare i dati o lo stato necessari.
- Eseguire il codice che si desidera testare.
- Verificare che i risultati sono quelli attesi.

Vediamo le caratteristiche che Rust fornisce specificamente per scrivere test
che eseguono queste azioni, che includono l'attributo `test`, alcune macro e
l'attributo `should_panic`.

### Anatomia di Una Funzione di Test

Nella sua forma più semplice, un test in Rust è una funzione annotata con
l'attributo `test`. Gli attributi sono metadati relativi a pezzi di codice Rust;
un esempio è l'attributo `derive` che abbiamo usato con le _struct_ nel Capitolo
5. Per trasformare una funzione in una funzione di test, aggiungi `#[test]`
nella riga prima di `fn`. Quando esegui i tuoi test con il comando `cargo test`,
Rust costruisce un eseguibile di test che esegue le funzioni annotate e riporta
se ogni funzione di test passa o fallisce.

Ogni volta che creiamo un nuovo progetto di libreria con Cargo, viene generato
automaticamente un modulo di test con una funzione di test al suo interno.
Questo modulo ti fornisce un modello per scrivere i tuoi test, in modo da non
dover cercare la struttura e la sintassi esatta ogni volta che inizi un nuovo
progetto. Puoi aggiungere tutte le funzioni di test e tutti i moduli di test che
vuoi!

Esploreremo alcuni aspetti del funzionamento dei test sperimentando il modello
di test prima di testare effettivamente il codice. Poi scriveremo alcuni test
reali che richiamano il codice che abbiamo scritto e verificano che il suo
comportamento sia corretto.

Creiamo un nuovo progetto di libreria chiamato `addizione` che aggiungerà due
numeri:

```console
$ cargo new addizione --lib
     Created library `addizione` project
$ cd addizione
```

The contents of the _src/lib.rs_ file in your `adder` library should look like
Listing 11-1.

<Listing number="11-1" file-name="src/lib.rs" caption="Il codice generato automaticamente da `cargo new`">

<!-- manual-regeneration
cd listings/ch11-writing-automated-tests
rm -rf listing-11-01
cargo new listing-11-01 --lib --name adder
cd listing-11-01
echo "$ cargo test" > output.txt
RUSTFLAGS="-A unused_variables -A dead_code" RUST_TEST_THREADS=1 cargo test >> output.txt 2>&1
git diff output.txt # commit any relevant changes; discard irrelevant ones
cd ../../..
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

</Listing>

Il file inizia con un esempio di funzione `add` (_aggiungi_), in modo da avere
qualcosa da testare.

Per ora, concentriamoci solo sulla funzione `it_works` (_funziona_). Nota
l'annotazione `#[test]`: questo attributo indica che si tratta di una funzione
di test, in modo che il test runner sappia che deve trattare questa funzione
come un test. Potremmo anche avere funzioni non di test nel modulo `tests` per
aiutare a configurare scenari comuni o eseguire operazioni comuni, quindi
dobbiamo sempre indicare quali funzioni sono di test.

Il corpo della funzione di esempio utilizza la macro `assert_eq!` per verificare
che `result`, che contiene il risultato della chiamata a `add` con 2 e 2, sia
uguale a 4. Questa asserzione serve come esempio del formato di un test tipico.
Eseguiamola per vedere se il test passa.

Il comando `cargo test` esegue tutti i test del nostro progetto, come mostrato
nel listato 11-2.

<Listing number="11-2" caption="L'output dell'esecuzione del test generato automaticamente">

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

</Listing>

Cargo ha compilato ed eseguito il test. Vediamo la riga `running 1 test`. La
riga successiva mostra il nome della funzione di test generata, chiamata
`tests::it_works`, e che il risultato dell'esecuzione di quel test è `ok`. Il
riepilogo complessivo `test result: ok.` significa che tutti i test sono
passati, e la parte che recita `1 passed; 0 failed` tiene il conto del numero di
test che sono passati o falliti.

È possibile contrassegnare un test come da ignorare in modo che non venga
eseguito in una particolare istanza; ne parleremo nella sezione ["Ignorare
alcuni test se non specificamente richiesti"][ignoring]<!-- ignore --> più
avanti in questo capitolo. Poiché non l'abbiamo fatto qui, il riepilogo mostra
`0 ignored`. Possiamo anche passare un argomento al comando `cargo test` per
eseguire solo i test il cui nome corrisponda a una stringa; questo si chiama
_filtraggio_ e lo tratteremo nella sezione ["Eseguire un sottoinsieme di test in
base al nome"][subset]<!-- ignore -->. In questo caso non abbiamo filtrato i
test in esecuzione, quindi la fine del riepilogo mostra `0 filtered out`.

La statistica `0 measured` è per i test di benchmark che misurano le
prestazioni. I test di benchmark, al momento, sono disponibili solo nelle
nightly di Rust. Per saperne di più, consulta [la documentazione sui test di
benchmark][bench].

La parte successiva dell'output di test che inizia con `Doc-tests addizione` è
per i risultati di qualsiasi test sulla documentazione. Non abbiamo ancora test
sulla documentazione, ma Rust può compilare qualsiasi esempio di codice che
appare nella nostra documentazione. Questa funzione aiuta a mantenere
sincronizzata la documetazione e il codice! Parleremo di come scrivere test
sulla documentazione nella sezione ["Commenti di documentazione come
test"][doc-comments]<!-- ignore --> del Capitolo 14. Per ora, ignoreremo
l'output `Doc-tests`.

Iniziamo a personalizzare il test in base alle nostre esigenze. Per prima cosa,
cambiamo il nome della funzione `it_works`, ad esempio `esplorazione`, in questo
modo:

<span class="filename">File: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```

Esegui nuovamente `cargo test`. L'output ora mostra `esplorazione` invece di
`it_works`:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

Now we’ll add another test, but this time we’ll make a test that fails! Tests
fail when something in the test function panics. Each test is run in a new
thread, and when the main thread sees that a test thread has died, the test is
marked as failed. In Chapter 9, we talked about how the simplest way to panic
is to call the `panic!` macro. Enter the new test as a function named
`another`, so your _src/lib.rs_ file looks like Listing 11-3.
Ora aggiungeremo un altro test, ma questa volta faremo un test che fallisce! I test falliscono quando qualcosa nella funzione di test va in panico. Ogni test viene eseguito in un nuovo _thread_ e quando il _thread_ principale vede che un _thread_ di test fallisce, il test viene contrassegnato come fallito. Nel Capitolo 9 abbiamo parlato di come il modo più semplice per mandare in panico un programma sia quello di chiamare la macro `panic!`. Inserisci il nuovo test come una funzione di nome `un_altra`, in modo che il tuo file _src/lib.rs_ assuma l'aspetto del Listato 11-3.

<Listing number="11-3" file-name="src/lib.rs" caption="Aggiungere un secondo test che fallisce perché chiamiamo la macro `panic!`">

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs}}
```

</Listing>

Esegui di nuovo i test utilizzando `cargo test`. L'output dovrebbe assomigliare
al Listato 11-4, che mostra che il nostro test `esplorazione` è passato e
`un_altra` è fallito.

<Listing number="11-4" caption="Risultati dei test quando uno viene superato e uno fallisce">

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

</Listing>

<!-- manual-regeneration
rg panicked listings/ch11-writing-automated-tests/listing-11-03/output.txt
check the line number of the panic matches the line number in the following paragraph
 -->

Al posto di `ok`, la riga `test tests::un_altra` mostra `FAILED` (_fallito_).
Tra i singoli risultati e il riepilogo compaiono due nuove sezioni: la prima
mostra il motivo dettagliato del fallimento di ogni test. In questo caso,
otteniamo il dettaglio che `tests::un_altra` ha fallito perché è andato in
panico con il messaggio `Fai fallire questo test` alla riga 17 del file
_src/lib.rs_. La sezione successiva elenca solo i nomi di tutti i test falliti,
il che è utile quando ci sono molti test e molti output dettagliati di test
falliti. Possiamo usare il nome di un test fallito per eseguire solo quel test
per eseguire più facilmente il debug; parleremo più diffusamente dei modi per
eseguire i test nella sezione ["Controllare come vengono eseguiti i
test"][controlling-how-tests-are-run]<!-- ignore -->.

Alla fine viene visualizzata la riga di riepilogo: in generale, il risultato del
nostro test è `FAILED`. Abbiamo avuto un test superato e un test fallito.

Ora che hai visto come appaiono i risultati dei test in diversi scenari, vediamo
alcune macro diverse da `panic!` che sono utili nei test.

## Verifica dei Risultati Con la Macro `assert!`

La macro `assert!`, fornita dalla libreria standard, è utile quando vuoi
assicurarti che una condizione in un test risulti essere vera, `true`. Diamo
alla macro `assert!` un argomento che valuta in un booleano. Se il valore è
`true`, non succede nulla e il test passa. Se il valore è `false`, la macro
`assert!` chiama `panic!` per far fallire il test. L'uso della macro `assert!`
ci aiuta a verificare che il nostro codice funzioni nel modo in cui intendiamo.

Nel Listato 5-15 del Capitolo 5 abbiamo utilizzato una _struct_ `Rettangolo` e
un metodo `può_contenere`, che sono ripetuti qui nel Listato 11-5. Inseriamo
questo codice nel file _src/lib.rs_ e scriviamo alcuni test utilizzando la macro
`assert!`.

<Listing number="11-5" file-name="src/lib.rs" caption="La _struct_ `Rettangolo` e il suo metodo `può_contenere` del Capitolo 5">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs}}
```

</Listing>

Il metodo `può_contenere` restituisce un booleano, il che significa che è un
caso d'uso perfetto per la macro `assert!`. Nel Listato 11-6, scriviamo un test
che utilizza il metodo `può_contenere` creando un'istanza di `Rettangolo` che ha
una larghezza di 8 e un'altezza di 7 e affermando che può contenere un'altra
istanza di `Rettangolo` che ha una larghezza di 5 e un'altezza di 1.

<Listing number="11-6" file-name="src/lib.rs" caption="Un test per `può_contenere` che verifica se un rettangolo più grande può effettivamente contenere un rettangolo più piccolo">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

</Listing>

Nota la riga `use super::*;` all'interno del modulo `tests`. Il modulo `tests` è
un modulo normale che segue le solite regole di visibilità che abbiamo trattato
nel Capitolo 7 nella sezione ["Percorsi per Fare Riferimento a un Elemento
nell'Albero dei Moduli"][paths-for-referring-to-an-item-in-the-module-tree]<!--
ignore -->. Poiché il modulo `tests` è un modulo interno, dobbiamo portare il
codice in fase di test nel modulo esterno nello _scope_ del modulo interno.
Usiamo un _glob_ in questo caso, in modo che tutto ciò che definiamo nel modulo
esterno sia disponibile per questo modulo `tests`.

Abbiamo chiamato il nostro test `grande_contiene_piccolo` e abbiamo creato le
due istanze di `Rettangolo` di cui abbiamo bisogno. Poi abbiamo chiamato la
macro `assert!` e le abbiamo passato il risultato della chiamata
`grande.può_contenere(&piccolo)`. Questa espressione dovrebbe restituire `true`,
quindi il nostro test dovrebbe passare. Scopriamolo!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

Passa davvero! Aggiungiamo un altro test, questa volta per verificare che un
rettangolo più piccolo non può contenere un rettangolo più grande:

<span class="filename">File: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

Poiché il risultato corretto della funzione `può_contenere` in questo caso è
`false`, dobbiamo negare questo risultato prima di passarlo alla macro
`assert!`. Di conseguenza, il nostro test passerà se `può_contenere` restituisce
`false`:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

Due test superati! Ora vediamo cosa succede ai risultati dei nostri test quando
introduciamo un bug nel nostro codice. Cambieremo l'implementazione del metodo
`può_contenere` sostituendo il segno maggiore con il segno minore quando
confronta le larghezze:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

L'esecuzione dei test produce ora il seguente risultato:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```
Poiché `grande.larghezza` è `8` e `piccolo.larghezza` è `5`, il confronto delle
larghezze in `può_contenere` ora restituisce `false`: 8 non è inferiore a 5.

### Testare l'Eguaglianza Con le Macro `assert_eq!` e `assert_ne!`

Un modo comune per verificare le funzionalità è quello di testare l'uguaglianza
tra il risultato del codice in esame e il valore che ti aspetti che il codice
restituisca. Potresti farlo utilizzando la macro `assert!` e passandole
un'espressione con l'operatore `==`. Tuttavia, questo è un test così comune che
la libreria standard fornisce una coppia di macro, `assert_eq!` e `assert_ne!`,
per eseguire questo test in modo più conveniente. Queste macro confrontano due
argomenti per l'uguaglianza o l'ineguaglianza, rispettivamente. Inoltre,
stampano i due valori se l'asserzione fallisce, il che rende più facile vedere
il _perché_ il test è fallito; al contrario, la macro `assert!` indica solo che
ha ottenuto un valore `false` per l'espressione `==`, senza stampare i valori
che hanno portato al valore `false`.

Nel Listato 11-7, scriviamo una funzione chiamata `aggiungi_due` che aggiunge
`2` al suo parametro, poi testiamo questa funzione usando la macro `assert_eq!`

<Listing number="11-7" file-name="src/lib.rs" caption="Test della funzione `aggiungi_due` utilizzando la macro `assert_eq!`">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

</Listing>

Controlliamo che passi!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

Abbiamo creato una variabile chiamata `risultato` che contiene il risultato
della chiamata a `aggiungi_due(2)`. Poi passiamo `risultato` e `4` come
argomenti alla macro `assert_eq!`. La riga di output per questo test è `test
tests::aggiungere_due ... ok`, e il testo `ok` indica che il nostro test è
passato!

Introduciamo un bug nel nostro codice per vedere come appare `assert_eq!` quando fallisce. Cambia l'implementazione della funzione `aggiungi_due` per aggiungere invece `3`:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

Esegui nuovamente i test:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

Il nostro test ha rilevato il bug! Il test `tests::aggiungere_due` è fallito e
il messaggio ci dice che l'asserzione fallita era `left == right`
(_sinistra_/_destra_) e quali sono i valori `left` e `right`. Questo messaggio
ci aiuta a iniziare il debug: l'argomento `left`, dove avevamo il risultato
della chiamata a `aggiungi_due(2)`, era `5` ma l'argomento `right` era `4`. Puoi
immaginare che questo sia particolarmente utile quando abbiamo molti test in
corso.

Nota che in alcuni linguaggi e framework di test, i parametri delle funzioni di
asserzione di uguaglianza sono chiamati `expected` (_atteso_) e `actual`
(_effettivo_) e l'ordine in cui specifichiamo gli argomenti è importante.
Tuttavia, in Rust, sono chiamati `left` e `right` e l'ordine in cui
specifichiamo il valore che ci aspettiamo e il valore che il codice produce non
ha importanza. Potremmo scrivere l'asserzione in questo test come `assert_eq!(4,
risultato)`, che risulterebbe nello stesso messaggio di fallimento che mostra
``assertion `left == right` failed``.

La macro `assert_ne!` ha successo se i due valori che le diamo non sono uguali e
fallisce se sono uguali. Questa macro è molto utile nei casi in cui non siamo
sicuri di quale sarà il valore, ma sappiamo quale valore sicuramente non
dovrebbe essere. Ad esempio, se stiamo testando una funzione che ha la garanzia
di cambiare il suo input in qualche modo, ma il modo in cui l'input viene
cambiato dipende dal giorno della settimana in cui eseguiamo i test, la cosa
migliore da asserire potrebbe essere che l'output della funzione non è uguale
all'input.

Sotto la superficie, le macro `assert_eq!` e `assert_ne!` utilizzano
rispettivamente gli operatori `==` e `!=`. Quando le asserzioni falliscono,
queste macro stampano i loro argomenti utilizzando la formattazione di debug, il
che significa che i valori confrontati devono implementare i _trait_ `PartialEq`
e `Debug`. Tutti i _type_ primitivi e la maggior parte dei _type_ della libreria
standard implementano questi _trait_. Per le _struct_ e le _enum_ che definisci
tu stesso, dovrai implementare `PartialEq` per asserire l'uguaglianza di tali
_type_. Dovrai anche implementare `Debug` per stampare i valori quando
l'asserzione fallisce. Poiché entrambi i _trait_ sono derivabili, come
menzionato nel Listato 5-12 nel Capitolo 5, questo è solitamente semplice come
aggiungere l'annotazione `#[derive(PartialEq, Debug)]` alla definizione della
_struct_ o dell'_enum_. Vedi l'Appendice C, [_Trait_
derivabili”]][derivable-traits]<!-- ignore -->, per ulteriori dettagli su questi
e altri _trait_ derivabili.

### Aggiunta di Messaggi di Errore Personalizzati

Puoi anche aggiungere un messaggio personalizzato da stampare insieme al
messaggio di fallimento come argomenti opzionali alle macro `assert!`,
`assert_eq!` e `assert_ne!`. Qualsiasi argomento specificato dopo gli argomenti
obbligatori viene passato alla macro `format!` (di cui si parla in
["Concatenazione con l'Operatore `+` o la Macro `format!`"][format-macro]<!--
ignore --> nel Capitolo 8), quindi puoi passare una stringa di formato che
contenga dei segnaposto `{}` e dei valori da inserire in quei segnaposto. I
messaggi personalizzati sono utili per documentare il significato di
un'asserzione; quando un test fallisce, avrai un'idea più precisa del problema
del codice.

Ad esempio, supponiamo di avere una funzione che saluta le persone per nome e
vogliamo verificare che il nome che passiamo nella funzione appaia nell'output:

<span class="filename">File: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```

I requisiti per questo programma non sono ancora stati concordati e siamo
abbastanza sicuri che il testo `Ciao` all'inizio del saluto cambierà. Abbiamo
deciso di non dover aggiornare il test quando i requisiti cambiano, quindi
invece di verificare l'esatta uguaglianza con il valore restituito dalla
funzione `saluto`, asseriremo che l'output debba contenere il testo del
parametro di input.

Ora introduciamo un bug in questo codice cambiando `saluto` per non includere
`nome` e vedere come si presenta il fallimento del test:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

L'esecuzione di questo test produce il seguente risultato:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```

Questo risultato indica solo che l'asserzione è fallita e su quale riga si trova
l'asserzione. Un messaggio di fallimento più utile stamperebbe il valore della
funzione `saluto`. Aggiungiamo un messaggio di fallimento personalizzato
composto da una stringa di formato con un segnaposto riempito con il valore
effettivo ottenuto dalla funzione `saluto`:

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

Ora, quando eseguiamo il test, otterremo un messaggio di errore più informativo:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

Possiamo vedere il valore effettivamente ottenuto nell'output del test, il che
ci aiuterebbe a fare il debug di ciò che è accaduto invece di ciò che ci
aspettavamo che accadesse.

### Verifica Dei Casi di Panico Con `should_panic`

Oltre a verificare i valori di ritorno, è importante controllare che il nostro
codice gestisca le condizioni di errore come ci aspettiamo. Ad esempio,
considera il _type_ `Ipotesi` che abbiamo creato nel Capitolo 9, Listato 9-13.
Altro codice che utilizza `Ipotesi` dipende dalla garanzia che le istanze di
`Ipotesi` conterranno solo valori compresi tra 1 e 100. Possiamo scrivere un
test che garantisca che il tentativo di creare un'istanza di `Ipotesi` con un
valore al di fuori di questo intervallo vada in panico.

Per farlo, aggiungiamo l'attributo `should_panic` alla nostra funzione di test.
Il test passa se il codice all'interno della funzione va in panico; il test
fallisce se il codice all'interno della funzione non va in panico.

Il Listato 11-8 mostra un test che verifica che le condizioni di errore di
`Ipotesi::new` si verifichino quando ce lo aspettiamo.

<Listing number="11-8" file-name="src/lib.rs" caption="Test che una condizione generi un `panic!`">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

</Listing>

Inseriamo l'attributo `#[should_panic]` dopo l'attributo `#[test]` e prima della
funzione di test a cui si applica. Vediamo il risultato quando questo test viene
superato:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

Non male! Ora introduciamo un bug nel nostro codice rimuovendo la condizione per
cui la funzione `new` va in panico se il valore è superiore a 100:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

Quando eseguiamo il test nel Listato 11-8, questo fallirà:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

In questo caso non riceviamo un messaggio molto utile, ma se guardiamo la
funzione di test, vediamo che è annotata con `#[should_panic]`. Il fallimento
ottenuto significa che il codice della funzione di test non ha causato un
panico.

I test che utilizzano `should_panic` possono essere imprecisi. Un test
`should_panic` passerebbe anche se il test va in panico per un motivo diverso da
quello atteso. Per rendere i test `should_panic` più precisi, possiamo
aggiungere un parametro opzionale `expected` all'attributo `should_panic`.
L'infrastruttura di test si assicurerà che il messaggio di fallimento contenga
il testo fornito. Per esempio, considera il codice modificato per `Ipotesi` nel
Listato 11-9 dove la funzione `new` va in panico con messaggi diversi a seconda
che il valore sia troppo piccolo o troppo grande.

<Listing number="11-9" file-name="src/lib.rs" caption="Test per un `panic!` con un messaggio di panico contenente una sotto-stringa specificata">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

</Listing>

Questo test passerà perché il valore che abbiamo inserito nel parametro
`expected` dell'attributo `should_panic` è una sotto-stringa del messaggio con
cui la funzione `Ipotesi::new` va in panico. Avremmo potuto specificare l'intero
messaggio di panico che ci aspettiamo, che in questo caso sarebbe stato
`L'ipotesi deve essere minore o uguale a 100, valore fornito 200.`. Quello che
scegli di specificare dipende da quanto il messaggio di panico è unico o
dinamico e da quanto preciso vuoi che sia il tuo test. In questo caso, una
sotto-stringa del messaggio di panico è sufficiente per garantire che il codice
nella funzione di test esegua la parte con `else if valore > 100`.

Per vedere cosa succede quando un test `should_panic` con un messaggio
`expected` fallisce, introduciamo nuovamente un bug nel nostro codice scambiando
i corpi dei blocchi `if valore < 1` e `else if valore > 100`:

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

Questa volta il test `should_panic` fallirà:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

Il messaggio di fallimento indica che questo test è andato in panico come ci
aspettavamo, ma il messaggio di panico non includeva la stringa prevista `minore
o uguale a 100`. Il messaggio di panico che abbiamo ottenuto in questo caso è
stato `L'ipotesi deve essere maggiore di zero, valore fornito 200.`. Ora
possiamo iniziare a capire dove si trova il nostro bug!

### Utilizzo di `Result<T, E>` nei Test

I test che abbiamo fatto finora vanno tutti nel panico quando falliscono.
Possiamo anche scrivere test che utilizzano `Result<T, E>`! Ecco il test del
Listato 11-1, riscritto per utilizzare `Result<T, E>` e restituire un `Err`
invece di andare in panico:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs:here}}
```

La funzione `it_works` ora ha il _type_ di ritorno `Result<(), String>`. Nel
corpo della funzione, invece di richiamare la macro `assert_eq!`, restituiamo
`Ok())` quando il test passa e un `Err` con una `String` all'interno quando il
test fallisce.

Scrivere i test in modo che restituiscano un `Result<T, E>` ti permette di usare
l'operatore punto interrogativo nel corpo dei test, il che può essere un modo
comodo per scrivere test che dovrebbero fallire se qualsiasi operazione al loro
interno restituisce una variante `Err`.

Non puoi usare l'annotazione `#[should_panic]` nei test che usano `Result<T,
E>`. Per verificare che un'operazione restituisce una variante `Err`, _non_
usare l'operatore punto interrogativo sul valore `Result<T, E>`, ma usa
`assert!(valore.is_err())`.

Ora che conosci diversi modi per scrivere i test, vediamo cosa succede quando li
eseguiamo ed esploriamo le diverse opzioni che possiamo utilizzare con `cargo
test`

[format-macro]: ch08-02-strings.html#concatenazione-con-loperatore--o-la-macro-format
[bench]: https://doc.rust-lang.org/stable/unstable-book/library-features/test.html
[ignoring]: ch11-02-running-tests.html#ignorare-alcuni-test-se-non-specificamente-richiesti
[subset]: ch11-02-running-tests.html#eseguire-un-sottoinsieme-di-test-in-base-al-nome
[controlling-how-tests-are-run]: ch11-02-running-tests.html#controllare-come-vengono-eseguiti-i-test
[derivable-traits]: appendix-03-derivable-traits.html
[doc-comments]: ch14-02-publishing-to-crates-io.html#commenti-di-documentazione-come-test
[paths-for-referring-to-an-item-in-the-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
