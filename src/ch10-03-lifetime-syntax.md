## Validazione dei _Reference_ con le _Lifetime_

Le _lifetime_ (_longevità_) sono un'altra tipologia di generico che abbiamo già
utilizzato. Invece di garantire che un _type_ abbia il comportamento desiderato,
le _lifetime_ assicurano che i _reference_ siano validi finché ne abbiamo
bisogno.

Un dettaglio che non abbiamo discusso nella sezione ["_Reference_ e
_Borrowing_”][references-and-borrowing]<!-- ignore --> del Capitolo 4 è che ogni
_reference_ in Rust ha una certa longevità, _lifetime_, che è lo _scope_ per il
quale quel _reference_ è valido. Il più delle volte, la _lifetime_ è implicita e
inferita, proprio come il più delle volte i _type_ sono inferiti. Siamo tenuti
ad annotare il _type_ solo quando sono possibili più _type_. Allo stesso modo,
dobbiamo annotare la longevità quando la _lifetime_ dei _reference_ potrebbe
essere correlata in diversi modi. Rust ci richiede di annotare le relazioni
utilizzando parametri di _lifetime_ generici per garantire che i _reference_
utilizzati in fase di esecuzione siano e rimangano sicuramente validi.

Annotare la _lifetime_ non è un concetto presente nella maggior parte degli
altri linguaggi di programmazione, quindi questo vi sembrerà poco familiare.
Sebbene non tratteremo la _lifetime_ nella sua interezza in questo capitolo,
discuteremo i modi più comuni in cui potresti incontrare la sintassi di
_lifetime_ in modo che tu possa familiarizzare con il concetto.

### Prevenire i Riferimenti Pendenti con la _Lifetime_

Lo scopo principale della _lifetime_ è prevenire i _riferimenti
pendenti_(_dangling references_), che causano il fatto che un _reference_ faccia
riferimento a dati a cui non dovrebbe far riferimento. Considera il programma
nel Listato 10-16, che ha uno _scope_ esterno e uno interno.

<Listing number="10-16" caption="Tentativo di utilizzare un _reference_ il cui valore è uscito dallo _scope_">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/src/main.rs}}
```

</Listing>

> Nota: gli esempi nei Listati 10-16, 10-17 e 10-23 dichiarano variabili senza
> assegnare loro un valore iniziale, quindi il nome della variabile esiste nello
> _scope_ esterno. A prima vista, questo potrebbe sembrare in conflitto con il
> fatto che Rust non abbia valori _null_. Tuttavia, se proviamo a utilizzare una
> variabile prima di assegnarle un valore, otterremo un errore in fase di
> compilazione, il che dimostra che Rust in effetti non ammette valori _null_.

Lo _scope_ esterno dichiara una variabile denominata `r` senza valore iniziale,
mentre lo _scope_ interno dichiara una variabile denominata `x` con valore
iniziale `5`. Nello _scope_ interno, proviamo a impostare il valore di `r` come
_reference_ a `x`. Quindi lo _scope_ interno termina e proviamo a stampare il
valore in `r`. Questo codice non verrà compilato perché il valore a cui fa
riferimento `r` è uscito dallo _scope_ prima che proviamo ad utilizzarlo. Ecco
il messaggio di errore:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/output.txt}}
```

Il messaggio di errore indica che la variabile `x` "non dura abbastanza a
lungo". Il motivo è che `x` sarà fuori dallo _scope_ quando lo _scope_ interno
termina alla riga 7. Ma `r` è ancora valido per lo _scope_ esterno; Poiché il
suo _scope_ è più ampio, diciamo che "vive più a lungo". Se Rust permettesse a
questo codice di funzionare, `r` farebbe _reference_ alla memoria deallocata
quando `x` è uscita dallo _scope_, e qualsiasi cosa provassimo a fare con `r`
non funzionerebbe correttamente. Quindi, come fa Rust a determinare che questo
codice non è valido? Utilizza un _borrow checker_.

### Il _Borrow Checker_

Il compilatore Rust ha un _borrow checker_ (_controllore dei prestiti_) che
confronta gli _scope_ per determinare se tutti i prestiti sono validi. Il
Listato 10-17 mostra lo stesso codice del Listato 10-16 ma con annotazioni che
mostrano la longevità delle variabili.

<Listing number="10-17" caption="Annotazioni delle _lifetime_ di `r` e `x`, denominate rispettivamente `'a` e `'b`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/src/main.rs}}
```

</Listing>

Qui abbiamo annotato la _lifetime_ di `r` con `'a` e la _lifetime_ di `x` con
`'b`. Come potete vedere, il blocco `'b` interno è molto più piccolo del blocco
`'a` esterno. In fase di compilazione, Rust confronta la dimensione dei due
_lifetime_ e vede che `r` ha una _lifetime_ di `'a` ma che si riferisce alla
memoria con una _lifetime_ di `'b`. Il programma viene rifiutato perché `'b` è
più breve di `'a`: il soggetto del _reference_ non ha la stessa longevità del
_reference_ stesso.

Il Listato 10-18 corregge il codice in modo che non abbia un _reference_
pendente e si compili senza errori.

<Listing number="10-18" caption="Un _reference_ valido perché i dati hanno una longevità maggiore del _reference_">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-18/src/main.rs}}
```

</Listing>

Qui, `x` ha la longevità `'b`, che in questo caso è maggiore di `'a`. Questo
significa che `r` può fare riferimento a `x` perché Rust sa che il _reference_
in `r` sarà sempre valido finché `x` è valido.

Ora che sai cosa sono le _lifetime_ dei _reference_ e come Rust analizza la
longevità per garantire che i _reference_ siano sempre validi, esploriamo le
_lifetime_ generiche dei parametri e dei valori di ritorno nel contesto delle
funzioni.

### _Lifetime_ Generica nelle Funzioni

Scriveremo una funzione che restituisce la più lunga tra due _slice_ di stringa.
Questa funzione prenderà due _slice_ e ne restituirà una singola. Dopo aver
implementato la funzione `piu_lunga`, il codice nel Listato 10-19 dovrebbe
stampare `La stringa più lunga è abcd`.

<Listing number="10-19" file-name="src/main.rs" caption="Una funzione `main` che chiama la funzione `piu_lunga` per trovare la più lunga tra due _slice_">

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-19/src/main.rs}}
```

</Listing>

Nota che vogliamo che la funzione accetti _slice_, che sono _reference_,
piuttosto che stringhe, perché non vogliamo che la funzione `piu_lunga` prenda
possesso dei suoi parametri. Fai riferimento a ["String _Slice_ come
Parametri”][string-slices-as-parameters]<!-- ignore --> nel Capitolo 4 per una
disamina più approfondita sul motivo per cui i parametri che utilizziamo nel
Listato 10-19 sono quelli che desideriamo.

Se proviamo a implementare la funzione `piu_lunga` come mostrato nel Listato
10-20, non verrà compilata.

<Listing number="10-20" file-name="src/main.rs" caption="Un'implementazione della funzione `piu_lunga` che restituisce la più lunga tra due stringhe ma non viene ancora compilata">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/src/main.rs:here}}
```

</Listing>

Invece, otteniamo il seguente errore che parla di _lifetime_:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/output.txt}}
```

Il testo di aiuto rivela che il _type_ restituito necessita di un parametro di
_lifetime_ generico perché Rust non riesce a stabilire se il _reference_
restituito si riferisce a `x` o `y`. In realtà, non lo sappiamo anche perché il
blocco `if` nel corpo di questa funzione restituisce un _reference_ a `x` e il
blocco `else` restituisce un _reference_ a `y`!

Quando definiamo questa funzione, non conosciamo i valori concreti che verranno
passati a questa funzione, quindi non sappiamo se verrà eseguito il caso `if` o
il caso `else`. Non conosciamo nemmeno la longevità concreta dei riferimenti che
verranno passati, quindi non possiamo esaminare gli _scope__ come abbiamo fatto
nei Listati 10-17 e 10-18 per determinare se il _reference_ restituito sarà
sempre valido. Nemmeno il _borrow checker_ può determinarlo, perché non sa come
la longevità di `x` e `y` si relaziona alla longevità del valore di ritorno. Per
correggere questo errore, aggiungeremo parametri di _lifetime_ generici che
definiscono la relazione tra i _reference_ in modo che il _borrow checker_ possa
eseguire la sua analisi.

### Sintassi dell'Annotazione di _Lifetime_

Le annotazioni di _lifetime_ non modificano la longevità di alcun riferimento.
Piuttosto, descrivono le relazioni tra le longevità di più riferimenti senza
influenzare la propria. Proprio come le funzioni possono accettare qualsiasi
_type_ quando la firma specifica un parametro di _type_ generico, le funzioni
possono accettare _reference_ con qualsiasi longevità specificando un parametro
di _lifetime_ generico.

Le annotazioni di longevità hanno una sintassi leggermente insolita: i nomi dei
parametri di _lifetime_ devono iniziare con un apostrofo (`'`) e sono
solitamente tutti in minuscolo e molto brevi, come i _type_ generici. La maggior
parte delle persone usa il nome `'a` per la prima annotazione di _lifetime_.
Posizioniamo le annotazioni dei parametri di _lifetime_ dopo la `&` di un
_reference_, utilizzando uno spazio per separare l'annotazione dal _type_ del
_reference_.

Ecco alcuni esempi: un _reference_ a un `i32` senza un parametro di longevità,
un _reference_ a un `i32` che ha un parametro di longevità denominato `'a` e un
_reference_ mutabile a un `i32` che ha anch'esso il parametro di longevità `'a`.

```rust,ignore
&i32        // _reference_ senza parametro di longevità
&'a i32     // _reference_ con annotazione della longevità
&'a mut i32 // _reference_ mutabile con annotazione della longevità
```

Un'annotazione di longevità di per sé non ha molto significato perché le
annotazioni servono a indicare a Rust come i parametri di _lifetime_ generici di
più riferimenti si relazionano tra loro. Esaminiamo come le annotazioni di
longevità si relazionano tra loro nel contesto della funzione `piu_lunga`.

### Annotazioni di _Lifetime_ nelle Firme delle Funzioni

Per utilizzare le annotazioni di longevità nelle firme delle funzioni, dobbiamo
dichiarare i parametri _lifetime_ generici  all'interno di parentesi angolari
tra il nome della funzione e l'elenco dei parametri, proprio come abbiamo fatto
con i parametri _type_ generici.

Vogliamo che la firma esprima il seguente vincolo: il _reference_ restituito
sarà valido finché entrambi i parametri saranno validi. Questa è la relazione
tra le _lifetime_ dei parametri e il valore restituito. Chiameremo la _lifetime_
`'a` e lo aggiungeremo a ciascun _reference_, come mostrato nel Listato 10-21.

<Listing number="10-21" file-name="src/main.rs" caption="La definizione di funzione `piu_lunga` che specifica che tutti i _reference_ nella firma devono avere la stessa _lifetime_ `'a`">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/src/main.rs:here}}
```

</Listing>

Questo codice dovrebbe compilarsi e produrre il risultato desiderato quando lo
utilizziamo con la funzione `main` nel Listato 10-19.

La firma della funzione ora indica a Rust che per un certo _lifetime_ `'a`, la
funzione accetta due parametri, entrambi _slice_ di stringa che durano almeno
quanto la _lifetime_ `'a`. La firma della funzione indica anche a Rust che la
_slice_ di stringa ritornata dalla funzione avrà una longevitò massima pari al
_lifetime_ `'a`. In pratica, significa che la longevità del _reference_
ritornato dalla funzione `piu_lunga` è minore o uguale alla minore tra le
longevità dei valori a cui fanno riferimento gli argomenti della funzione.
Queste relazioni sono ciò che vogliamo che Rust utilizzi quando analizza questo
codice.

Ricorda, quando specifichiamo i parametri di longevità nella firma di questa
funzione, non stiamo modificando le longevità dei valori passati o ritornati.
Piuttosto, stiamo specificando che il _borrow checker_ deve rifiutare qualsiasi
valore che non rispetta questi vincoli. Nota che la funzione `piu_lunga` non ha
bisogno di sapere esattamente quanto dureranno `x` e `y`, ma solo esiste uno
_scope_ che può essere sostituito ad `'a` che soddisfi questa firma.

Quando si annotano le longevità nelle funzioni, le annotazioni vanno nella firma
della funzione, non nel corpo della funzione. Le annotazioni di _lifetime_
diventano parte del contratto della funzione, proprio come i _type_ nella firma.
Avere le firme delle funzioni che contengono il contratto di longevità significa
che l'analisi effettuata dal compilatore Rust può essere più semplice. Se c'è un
problema con il modo in cui una funzione è annotata o con il modo in cui viene
chiamata, gli errori del compilatore possono indicare la parte del nostro codice
e i vincoli in modo più preciso. Se, invece, il compilatore Rust facesse più
inferenze su ciò che intendevamo che fossero le relazioni tra le longevità, il
compilatore potrebbe essere in grado di indicare solo un utilizzo del nostro
codice molto lontano dalla causa del problema.

Quando passiamo _reference_ concreti a `piu_lunga`, la longevità concreta che
viene sostituita per `'a` è la parte dello _scope_ di `x` che si sovrappone allo
_scope_ di `y`. In altre parole, la longevità generica `'a` otterrà la longevità
concreta uguale alla minore tra le longevità di `x` e `y`. Poiché abbiamo
annotato il _reference_ ritornato con lo stesso parametro di longevità `'a`, il
_reference_ ritornato sarà valido anche per la lunghezza della minore tra la
longevità di `x` e `y`.

Osserviamo come le annotazioni di longevità limitano la funzione `piu_lunga` dal
ricevere _reference_ con longevità concrete diverse. Il Listato 10-22 è un
esempio semplice.

<Listing number="10-22" file-name="src/main.rs" caption="Utilizzo della funzione `piu_lunga` con _reference_ a valori `String` con longevità concrete diverse">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-22/src/main.rs:here}}
```

</Listing>

In questo esempio, `stringa1` è valida fino alla fine dello _scope_ esterno,
`stringa2` è valido fino alla fine dello _scope_ interno e `risultato` fa
riferimento a qualcosa che è valido fino alla fine dello _scope_ interno. Esegui
questo codice e vedrai che verrà approvato dal _borrow checker_; compilerà e
stamperà `La stringa più lunga è una stringa bella lunga`.

Proviamo ora un esempio che mostra che la _lifetime_ del _reference_ in
`risultato` deve essere la _lifetime_ più breve tra i due argomenti. Sposteremo
la dichiarazione della variabile `risultato` al di fuori dello _scope_ interno,
ma lasceremo l' assegnazione del valore alla variabile `risultato` all'interno
dello _scope_ con `stringa2`. Quindi sposteremo `println!` che utilizza
`risultato` al di fuori dello _scope_ interno, dopo che quest'ultimo è
terminato. Il codice nel Listato 10-23 non verrà compilato.

<Listing number="10-23" file-name="src/main.rs" caption="Tentativo di utilizzare `risultato` dopo che `stringa2` è uscita dallo _scope_">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/src/main.rs:here}}
```

</Listing>

Quando proviamo a compilare questo codice, otteniamo questo errore:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/output.txt}}
```

L'errore indica che, affinché `risultato` sia valido per l'istruzione
`println!`, `stringa2` dovrebbe essere valido fino alla fine dello _scope_
esterno. Rust lo sa perché abbiamo annotato le _lifetime_ dei parametri della
funzione e dei valori ritornati utilizzando lo stesso parametro `'a`.

Come esseri umani, possiamo guardare questo codice e vedere che `stringa1` è più
lungo di `stringa2` e, pertanto, `risultato` conterrà un _reference_ a
`stringa1`. Poiché `stringa1` non è ancora uscito dallo _scope_, un _reference_
a `stringa1` sarà ancora valido per l'istruzione `println!`. Tuttavia, il
compilatore non può verificare che il _reference_ sia valido in questo caso.
Abbiamo detto a Rust che la _lifetime_ del _reference_ ritornato dalla funzione
`piu_lunga` è uguale al più piccolo tra le _lifetime_ dei riferimenti passati.
Pertanto, il _borrow checker_ non consente il codice nel Listato 10-23 in quanto
potrebbe contenere un _reference_ non valido.

Prova a progettare altri esperimenti che varino i valori e le _lifetime_ dei
_reference_ passati alla funzione `piu_lunga` e il modo in cui il _reference_
ritornato viene utilizzato. Formula ipotesi sul fatto che questi esperimenti
supereranno o meno il _borrow cewcker_ prima di compilare; poi verifica se avevi
ragione!

### Pensare in Termini di _Lifetime_

Il modo in cui è necessario specificare i parametri di longevità dipende da cosa
sta facendo la vostra funzione. Ad esempio, se modificassimo l'implementazione
della funzione `piu_lunga` in modo che restituisca sempre il primo parametro
anziché la _slice_ di stringa più lunga, non avremmo bisogno di specificare un
_lifetime_ per il parametro `y`. Il codice seguente verrà compilato:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-08-only-one-reference-with-lifetime/src/main.rs:here}}
```

</Listing>

Abbiamo specificato un parametro di longevità `'a` per il parametro `x` e il
_type_ di ritorno, ma non per il parametro `y`, perché la _lifetime_ di `y` non
ha alcuna relazione con la _lifetime_ di `x` o con il valore di ritorno.

Quando si restituisce un _reference_ da una funzione, il parametro di longevità
per il _type_ di ritorno deve corrispondere al parametro di longevità per uno
dei parametri. Se il _reference_ ritornato _non_ fa _reference_ a uno dei
parametri, deve fare _reference_ a un valore creato all'interno di questa
funzione. Tuttavia, questo sarebbe un _reference_ pendente perché il valore
uscirà dallo _scope_ al termine della funzione. Considera questo tentativo di
implementazione della funzione `piu_lunga` che non verrà compilato:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/src/main.rs:here}}
```

</Listing>

Qui, anche se abbiamo specificato un parametro di longevità `'a` per il _type_
di ritorno, questa implementazione non verrà compilata perché la _lifetime_ del
 valore di ritorno non è affatto correlata alla _lifetime_ dei parametri. Ecco
il messaggio di errore che riceviamo:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/output.txt}}
```

Il problema è che `risultato` esce dallo _scope_ e viene ripulito alla fine
della funzione `piu_lunga`. Stiamo anche cercando di ritornare un _reference_ a
`risultato` dalla funzione. Non c'è modo di specificare parametri di longevità
che modifichino il _reference_ pendente, e Rust non ci permette di creare un
_reference_ pendente. In questo caso, la soluzione migliore sarebbe restituire
un _type_ piuttosto che un _reference_, in modo che la funzione chiamante sia
responsabile della pulizia del valore.

In definitiva, la sintassi di longevità serve a collegare le _lifetime_ dei vari
parametri e valori di ritorno delle funzioni. Una volta connessi, Rust ha
informazioni sufficienti per consentire operazioni che proteggono la memoria e
impedire operazioni che creerebbero _reference_ pendenti o comunque violerebbero
la sicurezza della memoria.

### Annotazioni di _Lifetime_ nelle Definizioni delle _Struct_

Finora, tutte le _struct_ che abbiamo definito contenevano _type_ con
_ownership_. Possiamo definire _struct_ che contengano _reference_, ma in tal
caso dovremmo aggiungere un'annotazione di longevità su ogni _reference_ nella
definizione della _struct_. Il Listato 10-24 ha una _struct_ denominata
`ParteImportante` che contiene una _slice_ di stringa.

<Listing number="10-24" file-name="src/main.rs" caption="Una _struct_ che contiene un _reference_, che richiede un'annotazione di longevità">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/src/main.rs}}
```

</Listing>

Questa _struct_ ha il singolo campo `parte` che contiene una _slice_ di stringa,
che è un _reference_. Come per i _type_ generici, dichiariamo il nome del
parametro _lifetime_ generico tra parentesi angolari dopo il nome della
_struct_, in modo da poter utilizzare il parametro _lifetime_ nel corpo della
definizione della _struct_. Questa annotazione significa che un'istanza di
`ParteImportante` avrà una longevità non superiore a quella del _reference_ che
contiene nel suo campo `parte`.

La funzione `main` qui crea un'istanza della _struct_ `ParteImportante` che
contiene un _reference_ alla prima frase della `String` di proprietà della
variabile `romanzo`. I dati in `romanzo` esistono prima che l'istanza di
`ParteImportante` venga creata. Inoltre, `romanzo` non esce dallo _scope_ finché
anche `ParteImportante` non esce dallo _scope_, quindi il _reference_
nell'istanza di `ParteImportante` è valido.

### Elisione della _Lifetime_

Hai imparato che ogni _reference_ ha una _lifetime_ e che è necessario
specificare parametri di longevità per funzioni o _struct_ che utilizzano
_reference_. Tuttavia, avevamo una funzione nel Listato 4-9, mostrata di nuovo
nel Listato 10-25, che veniva compilata senza annotazioni di longevità.

<Listing number="10-25" file-name="src/lib.rs" caption="Una funzione che abbiamo definito nel Listato 4-9 che è stata compilata senza annotazioni di longevità, anche se il parametro e il _type_ ritornato sono _reference_">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-25/src/main.rs:here}}
```

</Listing>

Il motivo per cui questa funzione viene compilata senza annotazioni di longevità
è storico: nelle prime versioni (precedenti alla 1.0) di Rust, questo codice non
sarebbe stato compilato perché ogni _reference_ necessitava di una _lifetime_
esplicita. A quel tempo, la firma della funzione sarebbe stata scritta in questo
modo:

```rust,ignore
fn prima_parola<'a>(s: &'a str) -> &'a str {
```

Dopo aver scritto molto codice Rust, il team Rust ha scoperto che i
programmatori Rust inserivano le stesse annotazioni di longevità più e più volte
in particolari situazioni. Queste situazioni erano prevedibili e seguivano
alcuni schemi deterministici. Gli sviluppatori hanno programmato questi schemi
nel codice del compilatore in modo che il _borrow checker_ potesse dedurre le
_lifetime_ in queste situazioni e non avesse bisogno di annotazioni esplicite.

Questo pezzo di storia di Rust è rilevante perché è possibile che emergano e
vengano aggiunti al compilatore altri schemi deterministici. In futuro,
potrebbero essere necessarie ancora meno annotazioni di longevità.

Gli schemi programmati nell'analisi dei riferimenti di Rust sono chiamati
_regole di elisione della longevità_ (_lifetime elision rules_). Queste non sono
regole che i programmatori devono seguire; sono un insieme di casi particolari
che il compilatore prenderà in considerazione e, se il codice si adatta a questi
casi, non sarà necessario esplicitare le _lifetime_.

Le regole di elisione non forniscono un'inferenza completa. Se persiste
un'ambiguità sui _lifetime_ dei _reference_ dopo che Rust ha applicato le
regole, il compilatore non inferirà quale dovrebbe essere la longevità dei
_reference_ rimanenti. E quindi, invece di indovinare, il compilatore genererà
un errore indicando dove è necessario aggiungere le annotazioni di longevità.

Le lifetime dei parametri di funzione o metodo sono chiamate _lifetime_ _di
input_, e le _lifetime_ dei valori di ritorno sono chiamate _lifetime_ _di
output_.

Il compilatore utilizza tre regole per calcolare le _lifetime_ dei _reefrence_
quando non ci sono annotazioni esplicite. La prima regola si applica ai
_lifetime_ di input, mentre la seconda e la terza regola si applicano ai
_lifetime_ di output. Se il compilatore arriva alla fine delle tre regole e ci
sono ancora _reference_ per i quali non riesce a calcolare la longevità, il
compilatore si interromperà con un errore. Queste regole si applicano sia alle
definizioni `fn` che ai blocchi `impl`.

La prima regola è che il compilatore assegna un parametro di _lifetime_ a ogni
parametro che è un _reference_. In altre parole, una funzione con un parametro
riceve un parametro di _lifetime_: `fn foo<'a>(x: &'a i32)`; una funzione con
due parametri riceve due parametri di _lifetime_ separati: `fn foo<'a, 'b>(x:
&'a i32, y: &'b i32)`; e così via.

La seconda regola è che, se c'è esattamente un parametro di _lifetime_ in input,
quel _lifetime_ viene assegnato a tutti i parametri di _lifetime_ in output: `fn
foo<'a>(x: &'a i32) -> &'a i32`.

La terza regola è che, se ci sono più parametri di _lifetime_ in input, ma uno
di questi è `&self` o `&mut self` perché si tratta di un metodo, la _lifetime_
di `self` viene assegnata a tutti i parametri di _lifetime_ in output. Questa
terza regola rende i metodi molto più facili da leggere e scrivere perché sono
necessari meno simboli.

Facciamo finta di essere il compilatore. Applicheremo queste regole per
calcolare le longevità dei _reference_ nella firma della funzione `prima_parola`
nel Listato 10-25. La firma inizia senza alcuna _lifetime_ associata ai
_reefrence_:

```rust,ignore
fn prima_parola(s: &str) -> &str {
```

Quindi il compilatore applica la prima regola, che specifica che ogni parametro
abbia una propria longevità. La chiameremo `'a` come al solito, quindi ora la
firma è questa:

```rust,ignore
fn prima_parola<'a>(s: &'a str) -> &str {
```

La seconda regola si applica perché esiste esattamente un singolo parametro di
longevità in input. La seconda regola specifica che la longevità di un parametro
in input viene assegnata alala longevità in output, quindi la firma è ora
questa:

```rust,ignore
fn prima_parola<'a>(s: &'a str) -> &'a str {
```

Ora tutti i _reference_ in questa firma di funzione hanno una longevità e il
compilatore può continuare la sua analisi senza che il programmatore debba
annotare le _lifetime_ in questa firma di funzione.

Diamo un'occhiata a un altro esempio, questa volta utilizzando la funzione
`piu_lunga` che non aveva parametri di longevità quando abbiamo iniziato a
lavorarci nel Listato 10-20:

```rust,ignore
fn piu_lunga(x: &str, y: &str) -> &str {
```

Applichiamo la prima regola: ogni parametro ha la propria longevità. Questa
volta abbiamo due parametri invece di uno, quindi abbiamo due _lifetime_:

```rust,ignore
fn piu_lunga<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

Si può notare che la seconda regola non si applica perché c'è più di una
_lifetime_ di input. Nemmeno la terza regola si applica, perché `piu_lunga` è
una funzione e non un metodo, quindi nessuno dei parametri è `self`. Dopo aver
elaborato tutte e tre le regole, non abbiamo ancora capito qual è la longevità
di ritorno del _type_. Ecco perché abbiamo ricevuto un errore durante la
compilazione del codice nel Listato 10-20: il compilatore ha elaborato le regole
di elisione della _lifetime_, ma non è comunque riuscito a calcolare tutte le
_lifetime_ dei _reference_ nella firma.

Poiché la terza regola si applica solo alle firme dei metodi, esamineremo le
_lifetime_ in quel contesto per capire perché la terza regola ci consente di non
dover  annotare la longevità nelle firme dei metodi nella maggior parte dei
casi.

### Annotazione dei _Lifetime_ nelle Definizioni dei Metodi

Quando implementiamo metodi su una _struct_ con _lifetime_, utilizziamo la
stessa sintassi dei parametri di _type_ generico, come mostrato nel Listato
10-11. Il punto in cui dichiariamo e utilizziamo i parametri di longevità
dipende dal fatto che siano correlati ai campi della _struct_ o ai parametri del
metodo e ai valori di ritorno.

I nomi delle _lifetime_ per i campi della _struct_ devono sempre essere
dichiarati dopo la parola chiave `impl` e quindi utilizzati dopo il nome della
_struct_, poiché tali _lifetime_ fanno parte del _type_ della _struct_.

Nelle firme dei metodi all'interno del blocco `impl`, i _reference_ potrebbero
essere legati alla longevità dei _reference_ nei campi della _struct_, oppure
potrebbero essere indipendenti. Inoltre, le regole di elisione della _lifetime_
spesso fanno sì che le annotazioni della longevità non siano necessarie nelle
firme dei metodi. Diamo un'occhiata ad alcuni esempi utilizzando la _struct_
denominata `ParteImportante` che abbiamo definito nel Listato 10-24.

Per prima cosa useremo un metodo chiamato `livello` il cui unico parametro è un
_reference_ a `self` e il cui valore di ritorno è un `i32`, che non è un
_reference_ ad alcunché:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:1st}}
```

La dichiarazione del parametro _lifetime_ dopo `impl` e il suo utilizzo dopo il
nome del _type_ sono obbligatori, ma non siamo tenuti ad annotare la longevità
del _reference_ a `self` a causa della prima regola di elisione.

Ecco un esempio in cui si applica la terza regola di elisione della _lifetime_:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:3rd}}
```

Ci sono due _lifetime_ in input, quindi Rust applica la prima regola di elisione
della _lifetime_ e assegna sia a `&self` che a `annuncio` i corrispettivi
_lifetime_. Quindi, poiché uno dei parametri è `&self`, al _type_ di ritorno
viene assegnata la _lifetime_ di `&self`, e tutte le _lifetime_ sono state
considerate.

### La _Lifetime_ Statica

Una _lifetime_ speciale di cui dobbiamo discutere è `'static`, che indica che la
longevità del _reference_ interessato corrisponde a quella del programma. Tutti
i letterali stringa hanno la _lifetime_ `'static`, che possiamo annotare come
segue:

```rust
let s: &'static str = "Ho una lifetime statica.";
```

Il testo di questa stringa è memorizzato direttamente nel binario del programma,
che è sempre disponibile. Pertanto, la longevità di tutti i letterali stringa è
`'static`.

Potresti trovare suggerimenti nei messaggi di errore del compilatore di
utilizzare la _lifetime_ `'static`. Ma prima di specificare `'static` come
_lifetime_ per un _reference_, valuta se quel _reference_ ha effettivamente
necessità di rimanere valido per l'intera durata dell'esecuzione del tuo
programma. Nella maggior parte dei casi, un messaggio di errore che suggerisce
la _lifetime_ ``static` deriva dal tentativo di creare un _reference_ pendente o
da una mancata corrispondenza delle longevità disponibili. In questi casi, la
soluzione è risolvere questi problemi, non specificare la _lifetime_ ``static`.

## Parametri di _Type_ Generico, Vincoli del _Trait_ e _Lifetime_ Insieme

Esaminiamo brevemente la sintassi per specificare parametri di _type_ generico,
vincoli di _trait_ e _lifetime_, tutto in un'unica funzione!

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-11-generics-traits-and-lifetimes/src/main.rs:here}}
```

Questa è la funzione `piu_lunga` del Listato 10-21 che restituisce la più lunga
tra due _slice_. Ma ora ha un parametro aggiuntivo denominato `ann` di _type_
generico `T`, che può ricevere qualsiasi _type_ che implementi il _trait_
`Display` come specificato dalla clausola `where`. Questo parametro aggiuntivo
verrà stampato utilizzando `{}`, motivo per cui il vincolo del _trait_ `Display`
è necessario. Poiché le _lifetime_ sono un _type_ generico, le dichiarazioni del
parametro di longevità `'a` e del parametro di _type_ generico `T` vanno nella
stessa lista all'interno delle parentesi angolari dopo il nome della funzione.

## Riepilogo

Abbiamo trattato molti argomenti in questo capitolo! Ora che conosci i parametri
di _type_ generico, i _trait_, i vincoli dei _trait_ e i parametri di _lifetime_
generici, sei pronto a scrivere codice senza ripetizioni che funzioni in molte
situazioni diverse. I parametri di _type_ generico consentono di applicare il
codice a _type_ diversi. I _trait_ e i limiti dei _trait_ garantiscono che,
anche se i _type_ sono generici, abbiano il comportamento di cui il codice ha
bisogno. Hai imparato come usare le annotazioni di longevità per garantire che
questo codice flessibile non abbia _reference_ pendenti. E tutta questa analisi
avviene in fase di compilazione, il che non influisce sulle prestazioni in fase
di esecuzione!

Che ci crediate o no, c'è molto altro da imparare sugli argomenti trattati in
questo capitolo: il Capitolo 18 tratta degli oggetti _trait_, che rappresentano
un altro modo di utilizzare i _trait_. Esistono anche scenari più complessi che
coinvolgono le annotazioni _lifetime_, che ti serviranno solo in scenari molto
avanzati; se ti può interessare dovresti leggere [The Rust Reference][reference]
(in inglese). Ma ora imparerai come scrivere test in Rust in modo da poterti
assicurare che il tuo codice funzioni a dovere.

[references-and-borrowing]:
    ch04-02-references-and-borrowing.html#reference-e-borrowing
[string-slices-as-parameters]: ch04-03-slices.html#string-slice-come-parametri
[reference]: https://doc.rust-lang.org/stable/reference/index.html
