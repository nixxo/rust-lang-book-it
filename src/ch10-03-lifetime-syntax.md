## Validazione dei Riferimenti con i Lifetime

I lifetime (tempo di vita) sono un altro "tipo" di generico che abbiamo già utilizzato. Invece di garantire che un tipo abbia il comportamento desiderato, i lifetime assicurano che i riferimenti siano validi finché ne abbiamo bisogno.

Un dettaglio che non abbiamo discusso nella sezione ["_Reference_ e _Borrowing_”][references-and-borrowing]<!-- ignore --> del Capitolo 4 è
che ogni riferimento in Rust ha un _lifetime_, che è l'ambito per il quale quel riferimento è valido. Il più delle volte, i lifetime sono impliciti e inferiti,
proprio come il più delle volte i tipi sono inferiti. Siamo tenuti ad
annotare i tipi solo quando sono possibili più tipi. Allo stesso modo, dobbiamo
annotare i lifetime quando i lifetime dei riferimenti potrebbero essere correlati in diversi
modi. Rust ci richiede di annotare le relazioni utilizzando parametri di lifetime generici per garantire che i riferimenti effettivamente utilizzati in fase di esecuzione siano
sicuramente validi.

Annotare i lifetime non è un concetto presente nella maggior parte degli altri linguaggi di programmazione, quindi questo vi sembrerà poco familiare. Sebbene non tratteremo i lifetime nella loro
interezza in questo capitolo, discuteremo i modi più comuni in cui potreste incontrare la
sintassi dei lifetime in modo che possiate familiarizzare con il concetto.

### Prevenire i Riferimenti Sospesi con i Lifetime

Lo scopo principale dei lifetime è prevenire i _riferimenti sospesi (dangling references)_, che causano
il riferimento di un programma a dati diversi da quelli a cui dovrebbe fare riferimento.
Si consideri il programma nel Listato 10-16, che ha un ambito esterno e uno interno.

<Listing number="10-16" caption="Tentativo di utilizzare un riferimento il cui valore è uscito dall'ambito">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/src/main.rs}}
```

</Listing>

> Nota: gli esempi nei Listati 10-16, 10-17 e 10-23 dichiarano variabili
> senza assegnare loro un valore iniziale, quindi il nome della variabile esiste esterna all'ambito. 
>A prima vista, questo potrebbe sembrare in conflitto con il fatto che Rust non abbia
> valori null. Tuttavia, se proviamo a utilizzare una variabile prima di assegnarle un valore,
> otterremo un errore in fase di compilazione, il che dimostra che Rust in effetti non ammette
> valori null.

L'ambito esterno dichiara una variabile denominata `r` senza valore iniziale, mentre
l'ambito interno dichiara una variabile denominata `x` con valore iniziale `5`. Nell'ambito interno, proviamo a impostare il valore di `r` come riferimento a `x`. Quindi
l'ambito interno termina e proviamo a stampare il valore in `r`. Questo codice non verrà compilato
perché il valore a cui fa riferimento `r` è uscito dall'ambito prima
che proviamo ad utilizzarlo. Ecco il messaggio di errore:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/output.txt}}
```

Il messaggio di errore indica che la variabile `x` "non dura abbastanza a lungo". Il
motivo è che `x` sarà fuori dall'ambito quando l'ambito interno termina alla riga 7.
Ma `r` è ancora valido per l'ambito esterno; Poiché il suo scope è più ampio, diciamo
che "vive più a lungo". Se Rust permettesse a questo codice di funzionare, `r` farebbe
riferimento alla memoria deallocata quando `x` è uscita dallo scope, e
qualsiasi cosa provassimo a fare con `r` non funzionerebbe correttamente. Quindi, come fa Rust
a determinare che questo codice non è valido? Utilizza un borrow checker (verificatore di prestiti).

### Il Borrow Checker

Il compilatore Rust ha un _borrow checker_ che confronta gli scope per determinare
se tutti i prestiti sono validi. Il Listato 10-17 mostra lo stesso codice del Listato
10-16 ma con annotazioni che mostrano la durata delle variabili.

<Listing number="10-17" caption="Annotazioni dei lifetimes di `r` e `x`, denominati rispettivamente `'a` e `'b`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/src/main.rs}}
```

</Listing>

Qui abbiamo annotato il lifetime di `r` con `'a` e il lifetime di `x`
con `'b`. Come potete vedere, il blocco `'b` interno è molto più piccolo del blocco `'a` esterno. In fase di compilazione, Rust confronta la dimensione dei due
lifetimes e vede che `r` ha un lifetime di `'a` ma che si riferisce alla memoria
con un lifetime di `'b`. Il programma viene rifiutato perché `'b` è più breve di
`'a`: il soggetto del riferimento non ha la stessa durata del riferimento.

Il Listato 10-18 corregge il codice in modo che non abbia un riferimento sospeso e
si compili senza errori.

<Listing number="10-18" caption="Un riferimento valido perché i dati hanno una durata maggiore del riferimento">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-18/src/main.rs}}
```

</Listing>

Qui, `x` ha la durata `'b`, che in questo caso è maggiore di `'a`. Questo
significa che `r` può fare riferimento a `x` perché Rust sa che il riferimento in `r` sarà
sempre valido finché `x` è valido.

Ora che sai cosa sono i lifetimes (tempi di vita) dei riferimenti e come Rust analizza
i lifetimes per garantire che i riferimenti siano sempre validi, esploriamo i lifetimes generici
dei parametri e dei valori di ritorno nel contesto delle funzioni.

### Lifetime Generica nelle Funzioni

Scriveremo una funzione che restituisce la più lunga tra due porzioni di stringa. Questa
funzione prenderà due porzioni di stringa e ne restituirà una singola. Dopo
aver implementato la funzione `piulunga`, il codice nel Listato 10-19 dovrebbe
stampare `La stringa più lunga è abcd`.

<Listing number="10-19" file-name="src/main.rs" caption="Una funzione `main` che chiama la funzione `piulunga` per trovare la più lunga tra due string slice">

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-19/src/main.rs}}
```

</Listing>

Si noti che vogliamo che la funzione accetti string slice, che sono riferimenti,
piuttosto che stringhe, perché non vogliamo che la funzione `piulunga` prenda
la proprietà dei suoi parametri. Fare riferimento a ["String _Slice_ come Parametri”][string-slices-as-parameters]<!-- ignore --> nel Capitolo 4 per ulteriori
discussioni sul motivo per cui i parametri che utilizziamo nel Listato 10-19 sono quelli che
desideriamo.

Se proviamo a implementare la funzione `piulunga` come mostrato nel Listato 10-20,
non verrà compilata.

<Listing number="10-20" file-name="src/main.rs" caption="Un'implementazione della funzione `piulunga` che restituisce la più lunga tra due stringhe ma non viene ancora compilata">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/src/main.rs:here}}
```

</Listing>

Invece, otteniamo il seguente errore che parla di durate:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/output.txt}}
```

Il testo di aiuto rivela che il tipo restituito necessita di un parametro di durata generico
perché Rust non riesce a stabilire se il riferimento restituito si riferisce a a
`x` o `y`. In realtà, non lo sappiamo anche perché il blocco `if` nel corpo
di questa funzione restituisce un riferimento a `x` e il blocco `else` restituisce un
riferimento a `y`!

Quando definiamo questa funzione, non conosciamo i valori concreti che
verranno passati a questa funzione, quindi non sappiamo se verrà eseguito il caso `if` o il caso
`else`. Non conosciamo nemmeno la durata concreta dei
riferimenti che verranno passati, quindi non possiamo esaminare gli ambiti come abbiamo fatto nei
Listati 10-17 e 10-18 per determinare se il riferimento restituito sarà
sempre valido. Nemmeno il borrow checker può determinarlo, perché
non sa come la durata di `x` e `y` si relaziona alla durata del
valore di ritorno. Per correggere questo errore, aggiungeremo parametri di durata generici che
definiscono la relazione tra i riferimenti in modo che il borrow checker possa
eseguire la sua analisi.

### Sintassi dell'Annotazione di Durata

Le annotazioni di durata non modificano la durata di vita di alcun riferimento. Piuttosto,
descrivono le relazioni tra le durate di vita di più riferimenti
senza influenzare le durate. Proprio come le funzioni possono accettare qualsiasi tipo
quando la firma specifica un parametro di tipo generico, le funzioni possono accettare
riferimenti con qualsiasi durata specificando un parametro di durata generico.

Le annotazioni di durata hanno una sintassi leggermente insolita: i nomi dei parametri di durata
devono iniziare con un apostrofo (`'`) e sono solitamente tutti in minuscolo
e molto brevi, come i tipi generici. La maggior parte delle persone usa il nome `'a` per la prima
annotazione di durata. Posizioniamo le annotazioni dei parametri di durata dopo `&` di un
riferimento, utilizzando uno spazio per separare l'annotazione dal tipo del riferimento.

Ecco alcuni esempi: un riferimento a un `i32` senza un parametro di durata, un
riferimento a un `i32` che ha un parametro di durata denominato `'a` e un riferimento
mutabile a un `i32` che ha anch'esso il parametro di durata `'a`.

```rust,ignore
&i32 // un riferimento
&'a i32 // un riferimento con una durata esplicita
&'a mut i32 // un riferimento mutabile con una durata esplicita
```

Un'annotazione di durata di per sé non ha molto significato perché le
annotazioni servono a indicare a Rust come i parametri di durata generici di più
riferimenti si relazionano tra loro. Esaminiamo come le annotazioni di durata
si relazionano tra loro nel contesto della funzione `piulunga`.

### Annotazioni di Durata nelle Firme delle Funzioni

Per utilizzare le annotazioni di durata nelle firme delle funzioni, dobbiamo dichiarare i
parametri generici _lifetime_ all'interno di parentesi angolari tra il nome della funzione
e l'elenco dei parametri, proprio come abbiamo fatto con i parametri generici _type_.

Vogliamo che la firma esprima il seguente vincolo: il riferimento restituito
sarà valido finché entrambi i parametri saranno validi. Questa è la
relazione tra i lifetimes dei parametri e il valore restituito. Chiameremo il lifetime `'a` e lo aggiungeremo a ciascun riferimento, come mostrato nel Listato
10-21.

<Listing number="10-21" file-name="src/main.rs" caption="La definizione di funzione `piulunga` che specifica che tutti i riferimenti nella firma devono avere la stessa durata `'a`">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/src/main.rs:here}}
```

</Listing>

Questo codice dovrebbe compilarsi e produrre il risultato desiderato quando lo utilizziamo con la
funzione `main` nel Listato 10-19.

La firma della funzione ora indica a Rust che per un certo lifetime `'a`, la funzione
accetta due parametri, entrambi slice di stringa che durano almeno quanto
il lifetime `'a`. La firma della funzione indica anche a Rust che la slice di stringa
restituita dalla funzione avrà una durata massima pari al lifetime `'a`. In pratica, significa che la durata del riferimento restituito dalla funzione `piulunga`
è minore o uguale alla minore tra le durate dei valori
a cui fanno riferimento gli argomenti della funzione. Queste relazioni sono ciò che vogliamo che Rust
utilizzi quando analizza questo codice.

Ricorda, quando specifichiamo i parametri di durata nella firma di questa funzione,
non stiamo modificando le durate dei valori passati o restituiti. Piuttosto,
stiamo specificando che il borrow checker deve rifiutare qualsiasi valore che non
rispetta questi vincoli. Si noti che la funzione `piulunga` non ha bisogno
di sapere esattamente quanto dureranno `x` e `y`, ma solo che può essere sostituito un ambito `'a` che soddisfi questa firma.

Quando si annotano le durate nelle funzioni, le annotazioni vanno nella firma della funzione,
non nel corpo della funzione. Le annotazioni di durata diventano parte del
contratto della funzione, proprio come i tipi nella firma. Avere
le firme delle funzioni che contengono il contratto di durata significa che l'analisi effettuata dal compilatore Rust
può essere più semplice. Se c'è un problema con il modo in cui una funzione è
annotata o con il modo in cui viene chiamata, gli errori del compilatore possono indicare la parte del
nostro codice e i vincoli in modo più preciso. Se, invece, il compilatore Rust
facesse più inferenze su ciò che intendevamo che fossero le relazioni tra le durate,
il compilatore potrebbe essere in grado di indicare solo un utilizzo del nostro codice molto
lontano dalla causa del problema.

Quando passiamo riferimenti concreti a `piulunga`, la durata concreta che viene
sostituita per `'a` è la parte dell'ambito di `x` che si sovrappone all'ambito di `y`. In altre parole, la durata generica `'a` otterrà la durata concreta
uguale al minore tra le durate di `x` e `y`. Poiché
abbiamo annotato il riferimento restituito con lo stesso parametro di durata `'a`,
il riferimento restituito sarà valido anche per la lunghezza del minore tra
la durata di `x` e `y`.

Osserviamo come le annotazioni di durata limitano la funzione `piulunga`
passando riferimenti con durate concrete diverse. Il Listato 10-22 è
un esempio semplice.

<Listing number="10-22" file-name="src/main.rs" caption="Utilizzo della funzione `piulunga` con riferimenti a valori `String` con durate concrete diverse">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-22/src/main.rs:here}}
```

</Listing>

In questo esempio, `string1` è valido fino alla fine dell'ambito esterno, `string2`
è valido fino alla fine dell'ambito interno e `result` fa riferimento a qualcosa
che è valido fino alla fine dell'ambito interno. Esegui questo codice e vedrai
che verrà approvato dal borrow checker; compilerà e stamperà `La stringa più lunga
è lunga la stringa è lunga`.

Proviamo ora un esempio che mostra che il lifetime del riferimento in
`result` deve essere il lifetime più breve tra i due argomenti. Sposteremo la
dichiarazione della variabile `result` al di fuori dell'ambito interno, ma lasceremo l'
assegnazione del valore alla variabile `result` all'interno dello scope con
`string2`. Quindi sposteremo `println!` che utilizza `result` al di fuori dello scope
interno, dopo che quest'ultimo è terminato. Il codice nel Listato 10-23
non verrà compilato.

<Listing number="10-23" file-name="src/main.rs" caption="Tentativo di utilizzare `result` dopo che `string2` è uscito dallo scope">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/src/main.rs:here}}
```

</Listing>

Quando proviamo a compilare questo codice, otteniamo questo errore:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/output.txt}}
```

L'errore indica che, affinché `result` sia valido per l'istruzione `println!`,
`string2` dovrebbe essere valido fino alla fine dell'ambito esterno. Rust lo sa
perché abbiamo annotato i lifetimes dei parametri della funzione e dei valori restituiti
utilizzando lo stesso parametro `'a`.

Come esseri umani, possiamo guardare questo codice e vedere che `string1` è più lungo di
`string2` e, pertanto, `result` conterrà un riferimento a `string1`.
Poiché `string1` non è ancora uscito dall'ambito, un riferimento a `string1` sarà
ancora valido per l'istruzione `println!`. Tuttavia, il compilatore non può verificare
che il riferimento sia valido in questo caso. Abbiamo detto a Rust che il lifetime del
riferimento restituito dalla funzione `piulunga` è uguale al più piccolo tra
i lifetimes dei riferimenti passati. Pertanto, il borrow checker
non consente il codice nel Listato 10-23 in quanto potrebbe contenere un riferimento non valido.

Provate a progettare altri esperimenti che varino i valori e i lifetimes dei
riferimenti passati alla funzione `piulunga` e il modo in cui il riferimento restituito
viene utilizzato. Formulate ipotesi sul fatto che i vostri esperimenti supereranno o meno il borrow cewcker
prima di compilare; poi verificate se avete ragione!

### Pensare in Termini di lifetimes

Il modo in cui è necessario specificare i parametri di lifetime dipende da cosa sta facendo la vostra
funzione. Ad esempio, se modificassimo l'implementazione della
funzione `piulunga` in modo che restituisca sempre il primo parametro anziché la porzione di stringa più lunga, non avremmo bisogno di specificare un lifetime per il parametro `y`. Il
codice seguente verrà compilato:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-08-only-one-reference-with-lifetime/src/main.rs:here}}
```

</Listing>

Abbiamo specificato un parametro di durata `'a` per il parametro `x` e il tipo di ritorno,
ma non per il parametro `y`, perché il lifetime di `y` non ha
alcuna relazione con il lifetime di `x` o con il valore di ritorno.

Quando si restituisce un riferimento da una funzione, il parametro di durata per il
tipo di ritorno deve corrispondere al parametro di durata per uno dei parametri. Se
il riferimento restituito _non_ fa riferimento a uno dei parametri, deve fare riferimento
a un valore creato all'interno di questa funzione. Tuttavia, questo sarebbe un riferimento sospeso
perché il valore uscirà dallo scope alla fine della funzione.
Si consideri questo tentativo di implementazione della funzione `piulunga` che non verrà compilato:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/src/main.rs:here}}
```

</Listing>

Qui, anche se abbiamo specificato un parametro di durata `'a` per il tipo
di ritorno, questa implementazione non verrà compilata perché il valore di ritorno
lifetime non è affatto correlato al valore di durata dei parametri. Ecco il
messaggio di errore che riceviamo:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/output.txt}}
```

Il problema è che `result` esce dall'ambito e viene ripulito alla fine
della funzione `piulunga`. Stiamo anche cercando di restituire un riferimento a `result`
dalla funzione. Non c'è modo di specificare parametri di durata che
modifichino il riferimento sospeso, e Rust non ci permette di creare un riferimento sospeso. In questo caso, la soluzione migliore sarebbe restituire un tipo di dati di proprietà
piuttosto che un riferimento, in modo che la funzione chiamante sia responsabile
della pulizia del valore.

In definitiva, la sintassi di durata riguarda il collegamento dei lifetimes di vari
parametri e valori di ritorno delle funzioni. Una volta connessi, Rust ha
informazioni sufficienti per consentire operazioni che proteggono la memoria e impedire operazioni che
creerebbero puntatori sospesi o comunque violerebbero la sicurezza della memoria.

### Annotazioni di Durata nelle Definizioni delle Struct

Finora, tutte le strutture che abbiamo definito contengono tipi di proprietà. Possiamo definire strutture
per contenere riferimenti, ma in tal caso dovremmo aggiungere un'annotazione di durata
su ogni riferimento nella definizione della struttura. Il Listato 10-24 ha una struttura denominata
`ParteImportante` che contiene una slice di stringa.

<Listing number="10-24" file-name="src/main.rs" caption="Una struttura che contiene un riferimento, che richiede un'annotazione di durata">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/src/main.rs}}
```

</Listing>

Questa struttura ha il singolo campo `parte` che contiene una porzione di stringa, che è un
riferimento. Come per i tipi di dati generici, dichiariamo il nome del parametro generico
lifetime tra parentesi angolari dopo il nome della struttura, in modo da poter
utilizzare il parametro lifetime nel corpo della definizione della struttura. Questa
annotazione significa che un'istanza di `ParteImportante` non può sopravvivere al riferimento
che contiene nel suo campo `parte`.

La funzione `main` qui crea un'istanza della struttura `ParteImportante`
che contiene un riferimento alla prima frase della `String` di proprietà della
variabile `romanzo`. I dati in `romanzo` esistono prima che l'istanza di `ParteImportante`
venga creata. Inoltre, `romanzo` non esce dallo scope finché
anche `ParteImportante` non esce dallo scope, quindi il riferimento nell'istanza
di `ParteImportante` è valido.

### Elisione del lifetime

Hai imparato che ogni riferimento ha un lifetime e che è necessario specificare
parametri di lifetime per funzioni o strutture che utilizzano riferimenti. Tuttavia, avevamo
una funzione nel Listato 4-9, mostrata di nuovo nel Listato 10-25, che veniva compilata
senza annotazioni di lifetime.

<Listing number="10-25" file-name="src/lib.rs" caption="Una funzione che abbiamo definito nel Listato 4-9 che è stata compilata senza annotazioni di durata, anche se il parametro e il tipo restituito sono riferimenti">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-25/src/main.rs:here}}
```

</Listing>

Il motivo per cui questa funzione viene compilata senza annotazioni di durata è storico:
nelle prime versioni (precedenti alla 1.0) di Rust, questo codice non sarebbe stato compilato perché
ogni riferimento necessitava di un lifetime esplicito. A quel tempo, la firma della funzione
sarebbe stata scritta in questo modo:

```rust,ignore
fn prima_parola<'a>(s: &'a str) -> &'a str {
```

Dopo aver scritto molto codice Rust, il team Rust ha scoperto che i programmatori Rust
inserivano le stesse annotazioni di durata più e più volte in particolari
situazioni. Queste situazioni erano prevedibili e seguivano alcuni schemi
deterministici. Gli sviluppatori hanno programmato questi schemi nel codice del compilatore in modo che
il borrow checker potesse dedurre i lifetimes in queste situazioni e non avesse
bisogno di annotazioni esplicite.

Questo pezzo di storia di Rust è rilevante perché è possibile che emergano e vengano aggiunti al compilatore altri schemi
deterministici. In futuro,
potrebbero essere necessarie ancora meno annotazioni di durata.

Gli schemi programmati nell'analisi dei riferimenti di Rust sono chiamati
_lifetime elision rules_ (regole di elisione del tempo di vita). Queste non sono regole che i programmatori devono seguire; Sono
un insieme di casi particolari che il compilatore prenderà in considerazione e, se il codice
si adatta a questi casi, non è necessario scrivere i lifetimes in modo esplicito.

"L'elisione è la caduta della vocale finale di una parola, che viene segnalata da un apostrofo, quando la parola è seguita da un'altra che inizia per vocale".

Le regole di elisione non forniscono un'inferenza completa. Se persiste un'ambiguità
sui lifetimes dei riferimenti dopo che Rust ha applicato le regole,
il compilatore non indovinerà quale dovrebbe essere il lifetime dei riferimenti rimanenti.
Invece di indovinare, il compilatore genererà un errore che è possibile risolvere
aggiungendo le annotazioni sui lifetimes.

I lifetimes dei parametri di funzione o metodo sono chiamati _lifetimes di input_, e
i lifetimes dei valori di ritorno sono chiamati _lifetimes di output_.

Il compilatore utilizza tre regole per calcolare i lifetimes dei riferimenti
quando non ci sono annotazioni esplicite. La prima regola si applica ai lifetimes
di input, mentre la seconda e la terza regola si applicano ai lifetimes di output. Se il compilatore
arriva alla fine delle tre regole e ci sono ancora riferimenti
per i quali non riesce a calcolare i lifetimes, il compilatore si interromperà con un errore.
Queste regole si applicano sia alle definizioni `fn` che ai blocchi `impl`.

La prima regola è che il compilatore assegna un parametro di lifetime a ogni
parametro che è un riferimento. In altre parole, una funzione con un parametro
riceve un parametro di lifetime: `fn foo<'a>(x: &'a i32)`; una funzione con due
parametri riceve due parametri di lifetime separati: `fn foo<'a, 'b>(x: &'a i32,
y: &'b i32)`; e così via.

La seconda regola è che, se c'è esattamente un parametro di lifetime in input, quel
lifetime viene assegnato a tutti i parametri di lifetime in output: `fn foo<'a>(x: &'a i32)
-> &'a i32`.

La terza regola è che, se ci sono più parametri di durata in input, ma
uno di questi è `&self` o `&mut self` perché si tratta di un metodo, la durata di
`self` viene assegnata a tutti i parametri di durata in output. Questa terza regola rende
i metodi molto più facili da leggere e scrivere perché sono necessari meno simboli.

Facciamo finta di essere il compilatore. Applicheremo queste regole per calcolare
le durate dei riferimenti nella firma della funzione `prima_parola` nel
Listato 10-25. La firma inizia senza alcuna durata associata ai
riferimenti:

```rust,ignore
fn prima_parola(s: &str) -> &str {
```

Quindi il compilatore applica la prima regola, che specifica che ogni parametro
abbia un proprio tempo di vita. Lo chiameremo `'a` come al solito, quindi ora la firma è
questa:

```rust,ignore
fn prima_parola<'a>(s: &'a str) -> &str {
```

La seconda regola si applica perché esiste esattamente un tempo di vita in input. La seconda
regola specifica che il tempo di vita di un parametro in input viene assegnato al tempo di vita in output, quindi la firma è ora questa:

```rust,ignore
fn prima_parola<'a>(s: &'a str) -> &'a str {
```

Ora tutti i riferimenti in questa firma di funzione hanno un tempo di vita e il
compilatore può continuare la sua analisi senza che il programmatore debba annotare
i lifetimes in questa firma di funzione.

Diamo un'occhiata a un altro esempio, questa volta utilizzando la funzione `piulunga` che non aveva
parametri di durata quando abbiamo iniziato a lavorarci nel Listato 10-20:

```rust,ignore
fn piulunga(x: &str, y: &str) -> &str {
```

Applichiamo la prima regola: ogni parametro ha la propria durata. Questa volta
abbiamo due parametri invece di uno, quindi abbiamo due durate:

```rust,ignore
fn piulunga<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

Si può notare che la seconda regola non si applica perché c'è più di una
durata di input. Nemmeno la terza regola si applica, perché `piulunga` è una
funzione e non un metodo, quindi nessuno dei parametri è `self`. Dopo
aver elaborato tutte e tre le regole, non abbiamo ancora capito qual è la durata
di ritorno del tipo. Ecco perché abbiamo ricevuto un errore durante la compilazione del codice nel
Listato 10-20: il compilatore ha elaborato le regole di elisione dei lifetimes, ma non è comunque riuscito a
calcolare tutti i lifetimes dei riferimenti nella firma.

Poiché la terza regola si applica solo alle firme dei metodi, esamineremo
i lifetimes in quel contesto per capire perché la terza regola ci impedisce di
annotare i lifetimes nelle firme dei metodi molto spesso.

### Annotazioni dei Lifetimes nelle Definizioni dei Metodi

Quando implementiamo metodi su una struttura con lifetimes, utilizziamo la stessa sintassi
dei parametri di tipo generico, come mostrato nel Listato 10-11. Il punto in cui dichiariamo e
utilizziamo i parametri dei lifetimes dipende dal fatto che siano correlati ai campi
della struttura o ai parametri del metodo e ai valori di ritorno.

I nomi dei lifetimes per i campi della struttura devono sempre essere dichiarati dopo la parola chiave `impl`
e quindi utilizzati dopo il nome della struttura, poiché tali lifetimes fanno parte
del tipo della struttura.

Nelle firme dei metodi all'interno del blocco `impl`, i riferimenti potrebbero essere legati al
lifetime dei riferimenti nei campi della struttura, oppure potrebbero essere indipendenti.
Inoltre, le regole di elisione del lifetime spesso fanno sì che le annotazioni del lifetime
non siano necessarie nelle firme dei metodi. Diamo un'occhiata ad alcuni esempi utilizzando la
struttura denominata `ParteImportante` che abbiamo definito nel Listato 10-24.

Per prima cosa useremo un metodo chiamato `livello` il cui unico parametro è un riferimento a
`self` e il cui valore di ritorno è un `i32`, che non è un riferimento a nulla:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:1st}}
```

La dichiarazione del parametro lifetime dopo `impl` e il suo utilizzo dopo il nome del tipo
sono obbligatori, ma non siamo tenuti ad annotare il lifetime del riferimento
a `self` a causa della prima regola di elisione.

Ecco un esempio in cui si applica la terza regola di elisione del lifetime:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:3rd}}
```

Ci sono due lifetimes in input, quindi Rust applica la prima regola di elisione del lifetime
e assegna sia a `&self` che a `annuncio` i propri lifetimes. Quindi, poiché
uno dei parametri è `&self`, il tipo restituito ottiene il lifetime di `&self`,
e tutti i lifetimes sono stati considerati.

### Il Lifetime Statico

Un lifetime speciale di cui dobbiamo discutere è `'static`, che indica che il
riferimento interessato _può_ vivere per l'intera durata del programma. Tutti
i letterali stringa hanno il lifetime `'static`, che possiamo annotare come segue:

```rust
let s: &'static str = "Ho un lifetime statico.";
```

Il testo di questa stringa è memorizzato direttamente nel binario del programma, che è
sempre disponibile. Pertanto, il lifetime di tutti i letterali stringa è `'static`.

Potresti trovare suggerimenti nei messaggi di errore per utilizzare il lifetime `'static`. Ma
prima di specificare `'static` come lifetime per un riferimento, valuta
se il riferimento che hai effettivamente vive per l'intera durata del tuo
programma o meno, e se lo desideri. Nella maggior parte dei casi, un messaggio di errore
che suggerisce il lifetime ``static` deriva dal tentativo di creare un riferimento
sospeso o da una mancata corrispondenza dei lifetimes disponibili. In questi casi, la soluzione
è risolvere questi problemi, non specificare il lifetime ``static`.

## Parametri di Tipo Generico, Limiti del Tratto e Lifetimes Insieme

Esaminiamo brevemente la sintassi per specificare parametri di tipo generico, limiti di tratto e lifetimes, tutto in un'unica funzione!

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-11-generics-traits-and-lifetimes/src/main.rs:here}}
```

Questa è la funzione `piulunga` del Listato 10-21 che restituisce il più lungo tra
due sezioni di stringa. Ma ora ha un parametro aggiuntivo denominato `ann` di tipo generico `T`, che può essere compilato da qualsiasi tipo che implementi il ​​tratto `Display`
come specificato dalla clausola `where`. Questo parametro aggiuntivo verrà stampato
utilizzando `{}`, motivo per cui il limite del tratto `Display` è necessario. Poiché
i tempi di vita sono un tipo generico, le dichiarazioni del parametro di durata
`'a` e del parametro di tipo generico `T` vanno nella stessa lista all'interno delle parentesi angolari
dopo il nome della funzione.

## Riepilogo

Abbiamo trattato molti argomenti in questo capitolo! Ora che conosci i parametri di tipo generico, i tratti, i limiti dei tratti e i parametri di durata generici, sei
pronto a scrivere codice senza ripetizioni che funzioni in molte situazioni diverse.
I parametri di tipo generico consentono di applicare il codice a tipi diversi. I tratti e
i limiti dei tratti garantiscono che, anche se i tipi sono generici, abbiano il
comportamento di cui il codice ha bisogno. Hai imparato come usare le annotazioni a vita per garantire
che questo codice flessibile non abbia riferimenti sospesi. E tutta questa
analisi avviene in fase di compilazione, il che non influisce sulle prestazioni in fase di esecuzione!

Che ci crediate o no, c'è molto altro da imparare sugli argomenti trattati in
questo capitolo: il Capitolo 18 tratta degli oggetti Tratto, che rappresentano un altro modo per utilizzare
i tratti. Esistono anche scenari più complessi che coinvolgono le annotazioni lifetimes,
che ti serviranno solo in scenari molto avanzati; per questi, dovresti leggere
il [Riferimento Rust][reference]. Ma ora imparerai come scrivere test in
Rust in modo da poterti assicurare che il tuo codice funzioni a dovere.

[references-and-borrowing]: ch04-02-references-and-borrowing.html#reference-e-borrowing
[string-slices-as-parameters]: ch04-03-slices.html#string-slice-come-parametri
[reference]: https://doc.rust-lang.org/stable/reference/index.html
