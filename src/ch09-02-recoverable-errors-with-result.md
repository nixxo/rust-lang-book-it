## Errori Recuperabili con `Result`

La maggior parte degli errori non è abbastanza grave da richiedere l'arresto completo del programma.
A volte, quando una funzione fallisce, è per un motivo facilmente interpretabile
e a cui è possibile rispondere. Ad esempio, se si tenta di aprire un file e l'operazione fallisce
perché il file non esiste, potrebbe essere opportuno crearlo anziché
terminare il processo.

Ricorda da ["Gestione dei potenziali errori con `Result`”][handle_failure]<!--
ignore --> nel Capitolo 2 che l'enum `Result` è definito come avente due
varianti, `Ok` ed `Err`, come segue:

```rust
enum Result<T, E> {
Ok(T),
Err(E),
}
```

`T` ed `E` sono parametri di tipo generico: parleremo dei generici più in dettaglio
nel Capitolo 10. Quello che devi sapere ora è che `T` rappresenta
il tipo di valore che verrà restituito in caso di successo nella variante `Ok`,
ed `E` rappresenta il tipo di errore che verrà restituito in caso di fallimento nella variante `Err`. Poiché `Result` ha questi parametri di tipo generico, possiamo utilizzare il tipo `Result` e le funzioni definite su di esso in
molte situazioni diverse in cui il valore di successo e il valore di errore che vogliamo
restituire potrebbero differire.

Chiamiamo una funzione che restituisca un valore `Result` perché la funzione potrebbe
fallire. Nel Listato 9-3 proviamo ad aprire un file.

<Listing number="9-3" file-name="src/main.rs" caption="Apertura di un file">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

</Listing>

Il tipo restituito da `File::open` è `Result<T, E>`. Il parametro generico `T`
è stato compilato dall'implementazione di `File::open` con il tipo del
valore di successo, `std::fs::File`, che è un handle di file. Il tipo di `E` utilizzato nel
valore di errore è `std::io::Error`. Questo tipo di return significa che la chiamata a
`File::open` potrebbe avere esito positivo e restituire un handle di file da cui è possibile leggere o
scrivere. La chiamata di funzione potrebbe anche fallire: ad esempio, il file potrebbe non
esistere o potremmo non avere i permessi per accedervi. La funzione `File::open`
deve avere un modo per indicarci se è riuscita o meno e allo stesso tempo
fornirci l'handle di file o le informazioni sull'errore. Queste
informazioni sono esattamente ciò che l'enum `Result` trasmette.

Nel caso in cui `File::open` abbia esito positivo, il valore nella variabile
`greeting_file_result` sarà un'istanza di `Ok` che contiene un handle di file.
In caso di errore, il valore in `greeting_file_result` sarà un'
istanza di `Err` che contiene maggiori informazioni sul tipo di errore che
si è verificato.

Dobbiamo aggiungere al codice del Listato 9-3 azioni diverse a seconda
del valore restituito da `File::open`. Il Listato 9-4 mostra un modo per gestire il risultato
`Result` utilizzando uno strumento di base, l'espressione `match` di cui abbiamo parlato nel
Capitolo 6.

<Listing number="9-4" file-name="src/main.rs" caption="Utilizzo di un'espressione `match` per gestire le varianti di `Result` che potrebbero essere restituite">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

</Listing>

Si noti che, come l'enum `Option`, l'enum `Result` e le sue varianti sono state
introdotte nell'ambito dal preludio, quindi non è necessario specificare `Result::`
prima delle varianti `Ok` ed `Err` nel gruppo `match`.

Quando il risultato è `Ok`, questo codice restituirà il valore `file` interno
dalla variante `Ok`, e quindi assegneremo quel valore di handle di file alla variabile
`greeting_file`. Dopo `match`, possiamo utilizzare l'handle di file per la lettura o
la scrittura.

L'altro braccio di `match` gestisce il caso in cui otteniamo un valore `Err` da
`File::open`. In questo esempio, abbiamo scelto di chiamare la macro `panic!`. Se
non c'è alcun file denominato _hello.txt_ nella nostra directory corrente ed eseguiamo questo
codice, vedremo il seguente output dalla macro `panic!`:

```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

Come al solito, questo output ci dice esattamente cosa è andato storto.

### Corrispondenza in Caso di Errori Diversi

Il codice nel Listato 9-4 genererà un `panic!` indipendentemente dal motivo per cui `File::open` ha fallito.
Tuttavia, vogliamo intraprendere azioni diverse per diversi motivi di errore. Se
`File::open` ha fallito perché il file non esiste, vogliamo crearlo
e restituire l'handle al nuovo file. Se `File::open` ha fallito per qualsiasi altro
motivo, ad esempio perché non avevamo l'autorizzazione per aprire il file, vogliamo comunque
che il codice generi un `panic!` come nel Listato 9-4. Per questo,
aggiungiamo un'espressione `match` interna, mostrata nel Listato 9-5.

<Listing number="9-5" file-name="src/main.rs" caption="Gestione di diversi tipi di errori in modi diversi">

<!-- ignora questo test perché altrimenti crea hello.txt che causa il fallimento di altri
test lol -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

Il tipo di valore restituito da `File::open` all'interno della variante `Err` è
`io::Error`, una struttura fornita dalla libreria standard. Questa struttura
ha un metodo `kind` che possiamo chiamare per ottenere un valore `io::ErrorKind`. L'enum
`io::ErrorKind` è fornito dalla libreria standard e ha varianti
che rappresentano i diversi tipi di errori che potrebbero verificarsi da un'operazione `io`.
La variante che vogliamo utilizzare è `ErrorKind::NotFound`, che indica
che il file che stiamo cercando di aprire non esiste ancora. Quindi eseguiamo una corrispondenza su
`greeting_file_result`, ma abbiamo anche una corrispondenza interna su `error.kind()`.

La condizione che vogliamo verificare nella corrispondenza interna è se il valore restituito
da `error.kind()` è la variante `NotFound` dell'enum `ErrorKind`. In tal caso,
proviamo a creare il file con `File::create`. Tuttavia, poiché `File::create`
potrebbe anche fallire, abbiamo bisogno di un secondo caso nell'espressione `match` interna. Quando il
file non può essere creato, viene visualizzato un messaggio di errore diverso. Il secondo ramo dell'espressione `match` esterna rimane invariato, quindi il programma va in panico in caso di qualsiasi errore diverso
dall'errore di file mancante.

> #### Alternative all'utilizzo di `match` con `Result<T, E>`
>
> Un sacco di `match`! L'espressione `match` è molto utile, ma anche molto
> primitiva. Nel Capitolo 13, imparerai a conoscere le chiusure, che vengono utilizzate
> con molti dei metodi definiti in `Result<T, E>`. Questi metodi possono essere più
> concisi rispetto all'utilizzo di `match` quando si gestiscono i valori `Result<T, E>` nel codice. >
> Ad esempio, ecco un altro modo per scrivere la stessa logica mostrata nel Listato
> 9-5, questa volta utilizzando le chiusure e il metodo `unwrap_or_else`:
>
> <!-- IMPOSSIBILE ESTRARRE VEDI https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
> let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
> if error.kind() == ErrorKind::NotFound {
> File::create("hello.txt").unwrap_or_else(|error| {
> panic!("Problema durante la creazione del file: {error:?}");
> })
> } else {
> panic!("Problema durante l'apertura del file: {error:?}");
> }
> });
> }
> ```
>
> Sebbene questo codice abbia lo stesso comportamento del Listato 9-5, non contiene
> alcuna espressione `match` ed è più chiaro da leggere. Torna a questo esempio
> dopo aver letto il Capitolo 13 e cerca il metodo `unwrap_or_else` nella
> documentazione della libreria standard. Molti altri di questi metodi possono sostituire enormi 
> espressioni annidate di `match` quando si hanno errori.

#### Scorciatoie per il Panico in Caso di Errore: `unwrap` e `expect`

L'uso di `match` funziona abbastanza bene, ma può essere un po' prolisso e non sempre
comunica bene l'intento. Il tipo `Result<T, E>` ha molti metodi helper
definiti per svolgere varie attività più specifiche. Il metodo `unwrap` è un
metodo di scelta rapida implementato proprio come l'espressione `match` che abbiamo scritto nel
Listato 9-4. Se il valore `Result` è la variante `Ok`, `unwrap` restituirà
il valore all'interno di `Ok`. Se `Result` è la variante `Err`, `unwrap` richiamerà
la macro `panic!` per noi. Ecco un esempio di `unwrap` in azione:

<Listing file-name="src/main.rs">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

</Listing>

Se eseguiamo questo codice senza un file _hello.txt_, vedremo un messaggio di errore dalla
chiamata `panic!` effettuata dal metodo `unwrap`:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-04-unwrap
cargo run
copia e incolla il testo rilevante
-->

```text
thread 'main' panicked at src/main.rs:4:49:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

Analogamente, il metodo `expect` ci consente anche di scegliere il messaggio di errore `panic!`.
Usare `expect` invece di `unwrap` e fornire messaggi di errore efficaci può trasmettere
le proprie intenzioni e facilitare l'individuazione della fonte di un errore. La sintassi di
`expect` è la seguente:

<Listing file-name="src/main.rs">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

</Listing>

Usiamo `expect` allo stesso modo di `unwrap`: per restituire l'handle del file o chiamare
la macro `panic!`. Il messaggio di errore utilizzato da `expect` nella sua chiamata a `panic!`
sarà il parametro che passeremo a `expect`, anziché il messaggio predefinito
`panic!` utilizzato da `unwrap`. Ecco come appare:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-05-expect
cargo run
copia e incolla il testo rilevante
-->

```text
thread 'main' panicked at src/main.rs:5:10:
hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

Nel codice ottimizzato per il rilascio, la maggior parte dei Rustacean sceglie `expect` invece di
`unwrap` e fornisce più contesto sul motivo per cui ci si aspetta che l'operazione
riesca sempre. In questo modo, se le tue ipotesi dovessero rivelarsi sbagliate, avrai più
informazioni da utilizzare per il debug.

### Propagazione degli Errori

Quando l'implementazione di una funzione richiama qualcosa che potrebbe non funzionare, invece di
gestire l'errore all'interno della funzione stessa, è possibile restituirlo al
codice chiamante in modo che possa decidere cosa fare. Questa operazione è nota come _propagazione_
dell'errore e conferisce maggiore controllo al codice chiamante, dove potrebbero esserci più
informazioni o logica che determinano come gestire l'errore rispetto a quelle
disponibili nel contesto del codice.

Ad esempio, il Listato 9-6 mostra una funzione che legge un nome utente da un file. Se
il file non esiste o non può essere letto, questa funzione restituirà gli errori
al codice che ha chiamato la funzione.

<Listing number="9-6" file-name="src/main.rs" caption="Una funzione che restituisce errori al codice chiamante utilizzando `match`">

<!-- Non si usa deliberatamente rustdoc_include qui; la funzione `main` nel
file va in panico. Vogliamo includerlo a scopo di sperimentazione per i lettori, ma
non vogliamo includerlo per scopi di test di rustdoc. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

</Listing>

Questa funzione può essere scritta in un modo molto più breve, ma inizieremo
eseguendo gran parte del processo manualmente per esplorare la gestione degli errori; alla fine,
mostreremo il modo più breve. Diamo un'occhiata al tipo di ritorno della funzione
prima: `Result<String, io::Error>`. Ciò significa che la funzione restituisce un
valore del tipo `Result<T, E>`, dove il parametro generico `T` è stato
riempito con il tipo concreto `String` e il tipo generico `E` è stato
riempito con il tipo concreto `io::Error`.

Se questa funzione ha esito positivo senza problemi, il codice che la chiama
riceverà un valore `Ok` che contiene una `String`, ovvero il `nome utente` che
questa funzione ha letto dal file. Se questa funzione riscontra problemi, il
codice chiamante riceverà un valore `Err` che contiene un'istanza di `io::Error`
che contiene maggiori informazioni sulla causa del problema. Abbiamo scelto `io::Error` come tipo di ritorno di questa funzione perché è il
tipo del valore di errore restituito da entrambe le operazioni che stiamo chiamando nel
corpo di questa funzione che potrebbero fallire: la funzione `File::open` e il metodo
`read_to_string`.

Il corpo della funzione inizia con la chiamata alla funzione `File::open`. Quindi
gestiamo il valore `Result` con un `match` simile al `match` nel Listato 9-4. Se `File::open` ha esito positivo, l'handle del file nella variabile pattern `file`
diventa il valore nella variabile mutabile `username_file` e la funzione
continua. Nel caso `Err`, invece di chiamare `panic!`, utilizziamo la parola chiave `return`
per uscire completamente dalla funzione e passare il valore di errore
da `File::open`, ora nella variabile pattern `e`, al codice chiamante come
valore di errore di questa funzione.

Quindi, se abbiamo un handle di file in `username_file`, la funzione crea una
nuova `String` nella variabile `username` e chiama il metodo `read_to_string` sull'handle di file in `username_file` per leggere il contenuto del file in
`username`. Anche il metodo `read_to_string` restituisce un `Result` perché
potrebbe fallire, anche se `File::open` ha avuto esito positivo. Abbiamo quindi bisogno di un altro `match` per
gestire quel `Result`: se `read_to_string` ha esito positivo, la nostra funzione ha
avuto successo e restituiamo il nome utente dal file che ora si trova in `username`
avvolto in un `Ok`. Se `read_to_string` fallisce, restituiamo il valore di errore nello
stesso modo in cui abbiamo restituito il valore di errore nel `match` che gestiva il
valore di ritorno di `File::open`. Tuttavia, non è necessario specificare esplicitamente
`return`, perché questa è l'ultima espressione nella funzione.

Il codice chiamante gestirà quindi l'ottenimento di un valore `Ok`
che contiene un nome utente o di un valore `Err` che contiene un `io::Error`. Spetta
al codice chiamante decidere cosa fare con questi valori. Se il codice chiamante
riceve un valore `Err`, potrebbe chiamare `panic!` e mandare in crash il programma, utilizzare un
nome utente predefinito o cercare il nome utente da una posizione diversa da un file, ad esempio. Non abbiamo informazioni sufficienti su cosa stia effettivamente
cercando di fare il codice chiamante, quindi propaghiamo tutte le informazioni di successo o errore verso l'alto affinché
possa gestirle in modo appropriato.

Questo schema di propagazione degli errori è così comune in Rust che Rust fornisce l'operatore `?` con punto interrogativo per semplificare la procedura.

#### Una Scorciatoia per la Propagazione degli Errori: l'Operatore `?`

Il Listato 9-7 mostra un'implementazione di `read_username_from_file` che ha la
stessa funzionalità del Listato 9-6, ma questa implementazione utilizza l'operatore `?`.

<Listing number="9-7" file-name="src/main.rs" caption="Una funzione che restituisce errori al codice chiamante utilizzando l'operatore `?`">

<!-- Abbiamo deliberatamente evitato di usare rustdoc_include qui; la funzione `main` nel
file genera un errore. Vogliamo includerla a scopo di sperimentazione per i lettori, ma
non vogliamo includerla per testare rustdoc. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

</Listing>

Il `?` inserito dopo un valore `Result` è definito per funzionare quasi allo stesso modo
delle espressioni `match` che abbiamo definito per gestire i valori `Result` nel Listato
9-6. Se il valore di `Result` è `Ok`, il valore all'interno di `Ok` verrà
restituito da questa espressione e il programma continuerà. Se il valore
è `Err`, `Err` verrà restituito dall'intera funzione come se avessimo
utilizzato la parola chiave `return`, quindi il valore di errore viene propagato al codice chiamante.

C'è una differenza tra ciò che fa l'espressione `match` del Listato 9-6
e ciò che fa l'operatore `?`: i valori di errore che hanno l'operatore `?` chiamato
su di essi passano attraverso la funzione `from`, definita nel trait `From` nella
libreria standard, che viene utilizzata per convertire i valori da un tipo all'altro.
Quando l'operatore `?` chiama la funzione `from`, il tipo di errore ricevuto viene
convertito nel tipo di errore definito nel tipo di ritorno della funzione
corrente. Questo è utile quando una funzione restituisce un tipo di errore per rappresentare
tutti i modi in cui una funzione potrebbe fallire, anche se alcune parti potrebbero fallire per molti motivi
diversi.

Ad esempio, potremmo modificare la funzione `read_username_from_file` nel Listato
9-7 per restituire un tipo di errore personalizzato denominato `OurError` da noi definito. Se definiamo anche
`impl From<io::Error> per OurError` per costruire un'istanza di
`OurError` da un `io::Error`, l'operatore `?` chiamato nel corpo di
`read_username_from_file` chiamerà `from` e convertirà i tipi di errore senza
bisogno di aggiungere altro codice alla funzione.

Nel contesto del Listato 9-7, `?` alla fine della chiamata `File::open`
restituirà il valore all'interno di un `Ok` alla variabile `username_file`. Se si verifica un errore,
l'operatore `?` restituirà l'intera funzione in anticipo e assegnerà
un valore `Err` al codice chiamante. Lo stesso vale per `?` alla
fine della chiamata `read_to_string`.

L'operatore `?` elimina gran parte del codice standard e semplifica l'implementazione di questa funzione. Potremmo anche accorciare ulteriormente questo codice concatenando
le chiamate ai metodi subito dopo `?`, come mostrato nel Listato 9-8.

<Listing number="9-8" file-name="src/main.rs" caption="Concatenamento delle chiamate ai metodi dopo l'operatore `?`">

<!-- Qui non si usa deliberatamente rustdoc_include; la funzione `main` nel
file genera un errore. Vogliamo includerla a scopo di sperimentazione per i lettori, ma
non vogliamo includerla per i test di rustdoc. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

</Listing>

Abbiamo spostato la creazione della nuova `String` in `username` all'inizio della
funzione; questa parte non è cambiata. Invece di creare una variabile
`username_file`, abbiamo concatenato la chiamata a `read_to_string` direttamente al
risultato di `File::open("hello.txt")?`. Abbiamo ancora un `?` alla fine della
chiamata a `read_to_string` e continuiamo a restituire un valore `Ok` contenente `username`
quando sia `File::open` che `read_to_string` hanno esito positivo, anziché restituire
errori. La funzionalità è ancora la stessa dei Listati 9-6 e 9-7;
Questo è solo un modo diverso e più ergonomico di scriverlo.

Il Listato 9-9 mostra un modo per renderlo ancora più breve usando `fs::read_to_string`.

<Listing number="9-9" file-name="src/main.rs" caption="Usare `fs::read_to_string` invece di aprire e poi leggere il file">

<!-- Non usiamo deliberatamente rustdoc_include qui; la funzione `main` nel
file va in panico. Vogliamo includerla a scopo di sperimentazione del lettore, ma
non vogliamo includerla a scopo di test di rustdoc. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

</Listing>

Leggere un file in una stringa è un'operazione abbastanza comune, quindi la libreria standard
fornisce la comoda funzione `fs::read_to_string` che apre il
file, crea una nuova `String`, ne legge il contenuto, lo inserisce
in quella `String` e la restituisce. Naturalmente, usare `fs::read_to_string`
non ci dà l'opportunità di spiegare tutta la gestione degli errori, quindi l'abbiamo fatto
prima nel modo più lungo.

#### Dove Può Essere Utilizzato l'Operatore `?`

L'operatore `?` può essere utilizzato solo in funzioni il cui tipo di ritorno è compatibile
con il valore su cui viene utilizzato `?`. Questo perché l'operatore `?` è definito
per eseguire una restituzione anticipata di un valore dalla funzione, allo stesso modo
dell'espressione `match` che abbiamo definito nel Listato 9-6. Nel Listato 9-6, la funzione
`match` utilizzava un valore `Result` e il ramo di ritorno anticipato restituiva un valore
`Err(e)`. Il tipo di ritorno della funzione deve essere `Result` in modo che
sia compatibile con questo `return`.

Nel Listato 9-10, esaminiamo l'errore che otterremo se utilizziamo l'operatore `?`
in una funzione `main` con un tipo di ritorno incompatibile con il tipo del
valore su cui utilizziamo `?`.

<Listing number="9-10" file-name="src/main.rs" caption="Il tentativo di utilizzare `?` nella funzione `main` che restituisce `()` non verrà compilato.">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

</Listing>

Questo codice apre un file, che potrebbe non funzionare. L'operatore `?` segue il valore `Result`
restituito da `File::open`, ma questa funzione `main` ha come tipo di ritorno ``()`, non `Result`. Quando compiliamo questo codice, otteniamo il seguente messaggio di errore:

```console
{{#include ../listings/ch09-error-handling/listing-09-10/output.txt}}
```

Questo errore indica che possiamo utilizzare l'operatore `?` solo in una
funzione che restituisce `Result`, `Option` o un altro tipo che implementa
`FromResidual`.

Per correggere l'errore, hai due possibilità. Una è modificare il tipo di ritorno
della funzione in modo che sia compatibile con il valore su cui stai utilizzando l'operatore `?`,
a condizione che non ci siano restrizioni che lo impediscano. L'altra possibilità è
utilizzare un `match` o uno dei metodi `Result<T, E>` per gestire `Result<T, E>`
nel modo più appropriato.

Il messaggio di errore indica anche che `?` può essere utilizzato anche con i valori `Option<T>`. Come per l'utilizzo di `?` su `Result`, è possibile utilizzare `?` solo su `Option` in una
funzione che restituisce `Option`. Il comportamento dell'operatore `?` quando viene chiamato
su `Option<T>` è simile al suo comportamento quando viene chiamato su `Result<T, E>`:
se il valore è `None`, `None` verrà restituito in anticipo dalla funzione in quel
punto. Se il valore è `Some`, il valore all'interno di `Some` è il
valore risultante dell'espressione e la funzione continua. Il Listato 9-11 contiene
un esempio di una funzione che trova l'ultimo carattere della prima riga del
testo dato.

<Listing number="9-11" caption="Utilizzo dell'operatore `?` su un valore `Option<T>`">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-11/src/main.rs:here}}
```

</Listing>

Questa funzione restituisce `Option<char>` perché è possibile che ci sia un
carattere, ma è anche possibile che non ci sia. Questo codice accetta l'argomento `text` della porzione di stringa e chiama il metodo `lines` su di esso, che restituisce
un iteratore sulle righe della stringa. Poiché questa funzione vuole
esaminare la prima riga, chiama `next` sull'iteratore per ottenere il primo valore
dall'iteratore. Se `text` è una stringa vuota, questa chiamata a `next`
restituirà `None`, nel qual caso usiamo `?` per fermarci e restituire `None` da
`last_char_of_first_line`. Se `text` non è una stringa vuota, `next`
restituirà un valore `Some` contenente una porzione di stringa della prima riga di `text`.

`?` estrae la porzione di stringa e possiamo chiamare `chars` su quella porzione di stringa
per ottenere un iteratore dei suoi caratteri. Siamo interessati all'ultimo carattere in
questa prima riga, quindi chiamiamo `last` per restituire l'ultimo elemento nell'iteratore.
Questa è un'`Option` perché è possibile che la prima riga sia una stringa
vuota; ad esempio, se `text` inizia con una riga vuota ma ha caratteri su
altre righe, come in `"\nhi"`. Tuttavia, se c'è un ultimo carattere sulla prima
riga, verrà restituito nella variante `Some`. L'operatore `?` al centro
ci fornisce un modo conciso per esprimere questa logica, permettendoci di implementare la
funzione in una sola riga. Se non potessimo usare l'operatore `?` su `Option`, dovremmo
implementare questa logica utilizzando più chiamate di metodo o un'espressione `match`.

Si noti che è possibile utilizzare l'operatore `?` su un `Result` in una funzione che restituisce
`Result`, e si può utilizzare l'operatore `?` su un `Option` in una funzione che
restituisce `Option`, ma non è possibile combinare le due operazioni. L'operatore `?` non
convertirà automaticamente un `Result` in un `Option` o viceversa; in questi casi,
è possibile utilizzare metodi come il metodo `ok` su `Result` o il metodo `ok_or` su
`Option` per eseguire la conversione in modo esplicito.

Finora, tutte le funzioni `main` che abbiamo utilizzato restituiscono `()`. La funzione `main` è
speciale perché è il punto di ingresso e di uscita di un programma eseguibile,
e ci sono delle restrizioni sul tipo di ritorno che può essere usato affinché il programma
si comporti come previsto.

Fortunatamente, `main` può anche restituire `Result<(), E>`. Il Listato 9-12 contiene il codice
del Listato 9-10, ma abbiamo modificato il tipo di ritorno di `main` in
`Result<(), Box<dyn Error>>` e aggiunto un valore di ritorno `Ok(())` alla fine. Questo
codice ora verrà compilato.

<Listing number="9-12" file-name="src/main.rs" caption="Modificando `main` in modo che restituisca `Result<(), E>` è possibile utilizzare l'operatore `?` sui valori `Result`.">

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

</Listing>

Il tipo `Box<dyn Error>` è un _oggetto trait_, di cui parleremo in ["Utilizzo
di oggetti trait che consentono valori di tipi diversi"][trait-objects]<!--
ignore --> nel Capitolo 18. Per ora, puoi leggere `Box<dyn Error>` come "qualsiasi
tipo di errore". L'utilizzo di `?` su un valore `Result` in una funzione `main` con il
tipo di errore `Box<dyn Error>` è consentito perché consente la
restituzione anticipata di qualsiasi valore `Err`. Anche se il corpo di questa funzione `main` restituirà sempre e solo errori di tipo `std::io::Error`, specificando `Box<dyn Error>`, questa
firma continuerà a essere corretta anche se al corpo di `main` viene aggiunto altro codice che restituisce altri
errori.

Quando una funzione `main` restituisce `Result<(), E>`, l'eseguibile uscirà con
un valore di `0` se `main` restituisce `Ok(())` e uscirà con un valore diverso da zero se
`main` restituisce un valore `Err`. Gli eseguibili scritti in C restituiscono interi quando
escono: i programmi che escono correttamente restituiscono l'intero `0`, e i programmi
che generano un errore restituiscono un intero diverso da `0`. Anche Rust restituisce interi dagli
eseguibili per essere compatibile con questa convenzione.

La funzione `main` può restituire qualsiasi tipo che implementi [il
trait `std::process::Termination`][termination]<!-- ignore -->, che contiene
una funzione `report` che restituisce un `ExitCode`. Consulta la documentazione della libreria standard
per maggiori informazioni sull'implementazione del tratto `Termination` per
i tuoi tipi.

Ora che abbiamo discusso i dettagli della chiamata a `panic!` o della restituzione di `Result`,
torniamo all'argomento di come decidere quale sia appropriato da utilizzare in quali
casi.

[handle_failure]: ch02-00-guessing-game-tutorial.html#gestione-dei-potenziali-errori-con-result
[trait-objects]: ch18-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
[termination]: https://doc.rust-lang.org/stable/std/process/trait.Termination.html
