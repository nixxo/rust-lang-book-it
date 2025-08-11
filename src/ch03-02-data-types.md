## Datatype - Tipi di dato

Ogni valore in Rust è di un dato con un certo _type_, il che dice a Rust che
tipo di dati vengono specificati in modo che sappia come lavorare con quei dati.
Esamineremo due sottoinsiemi di tipi di dati: scalare e composto. Tieni presente
che Rust è un linguaggio _tipizzato staticamente_, il che significa che deve
conoscere il _type_ di tutte le variabili in fase di compilazione. Il
compilatore di solito può dedurre quale _type_ vogliamo utilizzare in base al
valore e al modo in cui lo utilizziamo. Nei casi in cui sono possibili
molteplici _type_, come quando abbiamo convertito uno `String` in un _type_
numerico usando `parse` nella sezione ["Confrontare l'ipotesi con il numero
segreto"][confrontare-lipotesi-con-il-numero-segreto]<!-- ignore --> del
Capitolo 2, dobbiamo aggiungere un'annotazione, specificando il _type_ in questo
modo:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/src/main.rs:main}}
```

Se non aggiungiamo l'annotazione del _type_ `: u32` mostrata nel codice
precedente, Rust visualizzerà il seguente errore, il che significa che il
compilatore ha bisogno di ulteriori informazioni per sapere quale _type_
vogliamo utilizzare:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

Potrai vedere annotazioni di _type_ diverso per altri tipi di dati.

### Type Scalare

Un _type scalare_ rappresenta un singolo valore. Rust ha quattro _type_ scalari
primari: numeri interi, numeri in virgola mobile, booleani e caratteri. Potresti
riconoscerli da altri linguaggi di programmazione. Andiamo a vedere come
funzionano in Rust.

#### Type Intero
Un intero, _integer_ d'ora in poi, è un numero senza una componente frazionaria.
Nel Capitolo 2 abbiamo utilizzato un tipo _integer_, il _type_ `u32`. Questa
dichiarazione del _type_ indica che il valore a cui è associato deve essere un
_integer_ senza segno (i _type integer_ con segno iniziano con `i` invece che
con `u`) che occupa 32 bit di spazio. La Tabella 3-1 mostra i _type integer_
incorporati in Rust. Possiamo usare una qualsiasi di queste varianti per
dichiarare il _type_ di un valore intero.


<span class="caption">Tabella 3-1: Type Integer in Rust</span>
| Lunghezza | Con Segno | Senza Segno |
| --------- | --------- | ----------- |
| 8-bit     | `i8`      | `u8`        |
| 16-bit    | `i16`     | `u16`       |
| 32-bit    | `i32`     | `u32`       |
| 64-bit    | `i64`     | `u64`       |
| 128-bit   | `i128`    | `u128`      |
| in base all'architettura | `isize` | `usize`  |

Ogni variante può essere con segno o senza e ha una dimensione esplicita. _Con
segno_ e _senza segno_ si riferisce alla possibilità che il numero sia negativo,
in altre parole, se il numero deve avere un segno con sé (con segno) o se sarà
sempre e solo positivo e potrà quindi essere rappresentato senza segno (senza
segno). È come scrivere numeri su carta: quando il segno conta, un numero viene
indicato con il segno più o con il segno meno; tuttavia, quando è lecito
ritenere che il numero sia positivo, viene visualizzato senza segno. I numeri
con segno vengono memorizzati utilizzando la rappresentazione del [complemento a
due][complemento-a-due]<!-- ignore -->.

Ogni variante con segnoa può memorizzare numeri da -(2<sup>n - 1</sup>) a
2<sup>n - 1</sup> - 1 inclusi, dove _n_ è il numero di bit che la variante
utilizza. Quindi un `i8` può memorizzare numeri da -(2<sup>7</sup>) a
2<sup>7</sup> - 1, il che equivale a -128 a 127. Le varianti senza segno possono
memorizzare numeri da 0 a 2<sup>n</sup> - 1, quindi un `u8` può memorizzare
numeri da 0 a 2<sup>8</sup> - 1, il che equivale a 0 a 255.

Inoltre, i _type_ `isize` e `usize` dipendono dall'architettura del computer su
cui viene eseguito il programma: 64 bit se si tratta di un'architettura a 64 bit
e 32 bit se si tratta di un'architettura a 32 bit.

Puoi scrivere i letterali _integer_ in una qualsiasi delle forme mostrate nella
Tabella 3-2. Nota che i letterali numerici che possono essere di più _type_
numerici permettono, tramite un suffisso, di specificarne il _type_ in questo
modo `57u8`. I letterali numerici possono anche usare `_` come separatore visivo
per rendere il numero più facile da leggere, come ad esempio `1_000`, che avrà
lo stesso valore che avrebbe se avessi specificato `1000`.

<span class="caption">Tabella 3-2: letterali _Integer_ in Rust</span>
| Letterali numerici | Esempio       |
| ------------------ | ------------- |
| Decimale           | `98_222`      |
| Esadecimale        | `0xff`        |
| Ottale             | `0o77`        |
| Binario            | `0b1111_0000` |
| Byte (solo `u8`)   | `b'A'`        |

Come si fa a sapere quale _type_ numerico utilizzare? Se non sei sicuro, le
impostazioni predefinite di Rust sono in genere un buon punto di partenza: il
_type integer_ di default è `i32`. La situazione principale in cui puoi usare
`isize` o `usize` è quando indicizzi qualche tipo di collezione.

> ##### Integer Overflow
>
> Supponiamo di avere una variabile di _type_ `u8` che può contenere valori
> compresi tra 0 e  255. Se provi a cambiare la variabile con un valore al di
> fuori di questo intervallo, ad esempio 256, si verificherà un _integer
> overflow_, che può portare a uno dei due comportamenti seguenti. Quando stai
> compilando in modalità debug, Rust include controlli per l'integer overflow
> che fanno sì che il tuo programma vada in _panico_ (_panic_ d'ora in poi) in
> fase di esecuzione se si verifica questo comportamento. Rust usa il termine
> _panic_ quando un programma termina con un errore; parleremo in modo più
> approfondito di _panic_ nella sezione ["Errori irrecuperabili con
> `panic!`"][errori-irrecuperabili-con-panic]<!-- ignore --> nel Capitolo 9.
> Quando si compila in modalità release con il flag `--release`, Rust _non_
> include i controlli per l'overflow degli integer che causano il _panic_.
> Invece, se si verifica l'overflow, Rust esegue l'_avvolgimento del complemento
> a due_. In pratica, i valori maggiori del valore massimo che il _type_ può
> contenere si "avvolgono" fino al minimo dei valori che il _type_ può
> contenere. Nel caso di un `u8`, il valore 256 diventa 0, il valore 257 diventa
> 1 e così via. Il programma non andrà in _panic_, ma la variabile avrà un
> valore che probabilmente non è quello che ci si aspettava che avesse.
> Affidarsi all'_avvolgimento del complemento a due_ degli _integer_ è
> considerato un errore. Per gestire esplicitamente la possibilità di overflow,
> puoi utilizzare queste famiglie di metodi forniti dalla libreria standard per
> i _type_ numerici primitivi:
> - Racchiudere tutte le modalità con i metodi `wrapping_*`, come ad esempio
>   `wrapping_add`.
> - Restituire il valore `None` se c'è overflow con i metodi `checked_*`.
> - Restituire il valore e un booleano che indica se c'è stato overflow con i
>   metodi `overflowing_*`.
> - Saturare i valori minimi o massimi del valore con i metodi `saturating_*`.

#### Type a Virgola Mobile

Rust ha anche due _type_ primitivi per i _numeri in virgola mobile_, abbreviato
_float_ in i nglese, che sono numeri con punti decimali. I _type_ in virgola
mobile di Rust sono `f32` e `f64`, rispettivamente di 32 e 64 bit. Il tipo
predefinito è `f64` perché sulle CPU moderne ha più o meno la stessa velocità di
`f32` ma è in grado di garantire una maggiore precisione. Tutti i _type_ in
virgola mobile sono _con segno_.

Ecco un esempio che mostra i numeri in virgola mobile in azione:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

I numeri in virgola mobile sono rappresentati secondo lo standard
[IEEE-754][ieee-754]<!-- ignore -->.

#### Operazioni numeriche

Rust supporta le operazioni matematiche di base che ti aspetteresti per tutte le
tipologie di numero: addizione, sottrazione, moltiplicazione, divisione e resto.
La divisione degli interi tronca verso lo zero al numero intero più vicino. Il
codice seguente mostra come utilizzare ogni operazione numerica in una
dichiarazione `let`:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

Ogni espressione in queste dichiarazioni utilizza un operatore matematico e
valuta un singolo valore, che viene poi legato a una variabile. [Appendice
B][appendice_b]<!-- ignora --> contiene un elenco di tutti gli operatori che
Rust mette a disposizione.

#### Il Type Booleano

As in most other programming languages, a Boolean type in Rust has two possible
values: `true` and `false`. Booleans are one byte in size. The Boolean type in
Rust is specified using `bool`. For example:
Come nella maggior parte degli altri linguaggi di programmazione, un _type_ booleano in Rust ha due valori possibili: _vero_ o _falso_ (`true` e `false` rispettivamente d'ora in poi). I booleani hanno la dimensione di un byte. Il _type_ booleano in Rust viene specificato con `bool`. Ad esempio:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

Il modo principale per utilizzare i valori booleani è attraverso i condizionali, come ad esempio un'espressione `if`. Tratteremo il funzionamento delle espressioni `if` in Rust nella sezione ["Flusso di controllo"][control-flow]<!-- ignore -->.

#### Il Type Carattere

Il _type_ carattere (`char` d'ota in poi) di Rust è il tipo alfabetico più primitivo del linguaggio. Ecco alcuni esempi di dichiarazione di valori `char`:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

Nota che specifichiamo i letterali `char` con le singole virgolette, al
contrario dei letterali stringa, che utilizzano le virgolette doppie. Il tipo
`char` di Rust ha la dimensione di quattro byte e rappresenta un valore scalare
Unicode, il che significa che può rappresentare molte altre cose oltre
all'ASCII. Le lettere accentate, i caratteri cinesi, giapponesi e coreani, le
emoji e gli spazi a larghezza zero sono tutti valori `char` validi in Rust. I
valori scalari Unicode vanno da `U+0000` a `U+D7FF` e da `U+E000` a `U+10FFFF`
inclusi. Tuttavia, un "carattere" non è un concetto vero e proprio in Unicode,
quindi la tua concettualizzazione umano di cosa sia un "carattere" potrebbe non
corrispondere a cosa sia un `char` in Rust. Discuteremo questo argomento in
dettaglio in ["Memorizzazione del testo codificato UTF-8 con le
stringhe"][strings]<!-- ignore --> nel Capitolo 8.

### I Type Composti

I _type composti_ possono raggruppare più valori in un unico _type_. Rust ha due _type_ composti primitivi: le tuple e gli array.

#### Il Type Tupla

Una _tupla_ è un modo generale per raggruppare una serie di valori di tipo
diverso in un unico _type_ composto. Le _tuple_ hanno una lunghezza fissa: una
volta dichiarate, non possono crescere o diminuire di dimensione. Creiamo una
tupla scrivendo un elenco di valori separati da virgole all'interno di
parentesi. Ogni posizione nella tupla ha un _type_ e i _type_ dei diversi valori
nella tupla non devono essere necessariamente gli stessi. In questo esempio
abbiamo aggiunto annotazioni del _type_ opzionali:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

La variabile `tup` si lega all'intera tupla perché una tupla è considerata un
singolo elemento composto. Per ottenere i singoli valori di una tupla, possiamo
fare _pattern matching_ per destrutturare il valore di una tupla, in questo
modo:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

Questo programma crea prima una tupla e la associa alla variabile `tup`. Quindi
utilizza un _pattern_ con `let` per prendere `tup` e trasformarlo in tre
variabili separate, `x`, `y` e `z`. Questa operazione è chiamata
_destrutturazione_ perché spezza la singola tupla in tre parti. Infine, il
programma stampa il valore di `y`, che è `6,4`.

Possiamo anche accedere direttamente a un elemento tupla utilizzando un punto
(`.`) seguito dall’indice del valore a cui vogliamo accedere. Ad esempio:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

Questo programma crea la tupla `x` e poi accede a ogni elemento della tupla
utilizzando i rispettivi indici. Come nella maggior parte dei linguaggi di
programmazione, il primo indice di una tupla è 0. La tupla senza valori ha un
nome speciale, _unit_. Questo valore e il suo _type_ corrispondente sono
entrambi scritti `()` e rappresentano un valore vuoto o un _type_ di ritorno
vuoto. Le espressioni restituiscono implicitamente il valore _unit_ se non
restituiscono nessun altro valore. La _tuple_ senza valori ha un nome speciale,
unità. Questo valore e il tipo cor- rispondente sono entrambi scritti () e
rappresentano un valore vuoto o un tipo restituito vuoto. Le espressioni
restituiscono implicitamente il valore unitario se non restituiscono nessun
altro valore.

#### Il Type Array

Un altro modo per avere una collezione di valori multipli è un _array_. A
differenza di una tupla, ogni elemento di un array deve avere lo stesso _type_.
A differenza degli array in altri linguaggi, gli array in Rust hanno una
lunghezza fissa.

Scriviamo i valori di un array come un elenco separato da virgole all'interno di
parentesi quadre:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

Gli array sono utili quando vuoi che i tuoi dati siano allocati sullo _stack_,
come gli altri _type_ che abbiamo visto finora, piuttosto che sull'_heap_
(parleremo dello _stack_ e dell'_heap_ in modo più approfondito nel [Capitolo
4][stack-and-heap]<!-- ignore -->) o quando vuoi assicurarti di avere sempre un
numero fisso di elementi. Un array, però, non è flessibile come il _type
vettore_ (_vector_ d'ora in poi). Un _vector_ è un _type_ simile, che consente
la collezione di dati, fornito dalla libreria standard ma che _è_ autorizzato a
crescere o a ridursi di dimensioni perché il suo contenuto risiede sull'_heap_.
Se non sei sicuro se usare un array o un _vector_, è probabile che tu debba
usare un _vector_. Il [Capitolo 8][vectors]<!-- ignore --> tratta i _vector_ in
modo più dettagliato.

Tuttavia, gli array sono più utili quando sai che il numero di elementi non
dovrà cambiare. Ad esempio, se dovessi utilizzare i nomi dei mesi in un
programma, probabilmente utilizzeresti un array piuttosto che un _vector_ perché
sai che conterrà sempre 12 elementi:

```rust
let mesi = ["Gennaio", "Febbraio", "Marzo", "Aprile", "Maggio", "Giugno",
            "Luglio", "Agosto", "Settembre", "Ottobre", "Novembre", "Dicembre"];
```

Il _type_ array si scrive utilizzando le parentesi quadre con il _type_ di ogni
elemento, il punto e virgola e il numero di elementi dell'array, in questo modo:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Here, `i32` is the type of each element. After the semicolon, the number `5`
indicates the array contains five elements.

In questo caso, `i32` è il _type_ di ogni elemento. Dopo il punto e virgola, il
numero `5` indica che l'array contiene cinque elementi. Puoi anche inizializzare
un array in modo che contenga lo stesso valore per ogni elemento specificando il
valore iniziale, seguito da un punto e virgola, e poi la lunghezza dell'array
tra parentesi quadre, come mostrato qui:

```rust
let a = [3; 5];
```

L'array chiamato `a` conterrà `5` elementi che saranno tutti impostati
inizialmente al valore `3`. Questo equivale a scrivere `let a = [3, 3, 3, 3,
3];` ma in modo più conciso.

##### Accessing Array Elements

Un array è un singolo blocco di memoria di dimensione fissa e nota che può
essere allocato nello _stack_. Puoi accedere agli elementi di un array
utilizzando l’indicizzazione, in questo modo:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

In questo esempio, la variabile denominata `primo` otterrà il valore `1` perché è il valore all'indice `[0]` dell'array. La variabile denominata `secondo` otterrà il valore `2` dall'indice `[1]` dell'array.

##### Accesso all'elemento dell'array non valido

Let’s see what happens if you try to access an element of an array that is past
the end of the array. Say you run this code, similar to the guessing game in
Chapter 2, to get an array index from the user:
Vediamo cosa succede se cerchi di accedere a un elemento di un array che si trova oltre la fine dell'array stesso. Supponiamo di eseguire questo codice, simile al gioco di indovinelli del Capitolo 2, per ottenere un indice dell'array dall'utente:

<span class="filename">File: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

Se esegui questo codice utilizzando `cargo run` e inserisci `0`, `1`, `2`, `3` o
`4`, il programma stamperà il valore corrispondente a quell'indice nell'array.
Se invece inserisci un numero oltre la fine dell'array, come ad esempio `10`,
vedrai un risultato come questo:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at src/main.rs:19:20:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Il programma ha generato un errore _durante l'esecuzione_, _at runtime_ in
inglese, nel momento in cui ha utilizzato un valore non valido nell'operazione
di indicizzazione. Il programma è uscito con un messaggio di errore e non ha
eseguito l'istruzione finale `println!`. Quando si tenta di accedere a un
elemento utilizzando l'indicizzazione, Rust controlla che l'indice specificato
sia inferiore alla lunghezza dell'array. Se l'indice è maggiore o uguale alla
lunghezza, Rust va in _panic_. Questo controllo deve avvenire in _runtime_,
soprattutto in questo caso, perché il compilatore non può sapere quale valore
inserirà l'utente quando eseguirà il codice in seguito.

Questo è un esempio dei principi di sicurezza della memoria di Rust in azione.
In molti linguaggi di basso livello, questo tipo di controllo non viene fatto e
quando si fornisce un indice errato, si può accedere a una memoria non valida.
Rust ti protegge da questo tipo di errore uscendo immediatamente invece di
consentire l'accesso alla memoria e continuare. Il [Capitolo
9][errori-irrecuperabili-con-panic] tratta di altri aspetti della gestione degli
errori di Rust e di come puoi scrivere codice leggibile e sicuro che non va in
_panic_ né consente l'accesso non valido alla memoria.

[confrontare-lipotesi-con-il-numero-segreto]: ch02-00-guessing-game-tutorial.html#confrontare-lipotesi-con-il-numero-segreto
[complemento-a-due]: https://it.wikipedia.org/wiki/Complemento_a_due
[ieee-754]: https://it.wikipedia.org/wiki/IEEE_754
[control-flow]: ch03-05-control-flow.html#control-flow
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[stack-and-heap]: ch04-01-what-is-ownership.html#the-stack-and-the-heap
[vectors]: ch08-01-vectors.html
[errori-irrecuperabili-con-panic]: ch09-01-unrecoverable-errors-with-panic.html
[appendice_b]: appendix-02-operators.md
