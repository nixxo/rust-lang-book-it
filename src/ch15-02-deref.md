## Trattare i Puntatori Intelligenti Come Normali _Reference_

L’implementazione del _trait_ `Deref` consente di personalizzare il
comportamento dell’operatore di de-referenziazione (_dereference operator_) `*`
(da non confondere con l’operatore di moltiplicazione o glob). Implementando
`Deref` in modo tale che un puntatore intelligente possa essere trattato come un
normale _reference_, è possibile scrivere codice che opera sui _reference_ e
utilizzarlo anche con i puntatori intelligenti.

Vediamo prima come funziona l’operatore di de-referenziazione con i normali
_reference_. Poi proveremo a definire un _type_ personalizzato che si comporti
come `Box<T>` e vedremo perché l’operatore di de-referenziazione non funziona
come un _reference_ sul nostro nuovo _type_ che abbiamo definito. Esploreremo
come l’implementazione del _trait_ `Deref` consenta ai puntatori intelligenti di
funzionare in modo simile ai _reference_. Infine, esamineremo la funzionalità di
Rust di _de-referenziazione forzata_ (_deref coercion_) e come ci consenta di
lavorare sia con i _reference_ che con i puntatori intelligenti.

### Seguire il _Reference_ al Valore

Un normale _reference_ è un tipo di puntatore, un modo per pensare a un
puntatore è immaginare una freccia che punta verso un valore memorizzato
altrove. Nel Listato 15-6, creiamo un _reference_ a un valore `i32` e poi
utilizziamo l’operatore di de-referenziazione per seguire il riferimento al
valore.

<Listing number="15-6" file-name="src/main.rs" caption="Utilizzo dell’operatore di de-referenziazione per seguire un riferimento a un valore `i32`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-06/src/main.rs}}
```

</Listing>

La variabile `x` contiene un valore `i32` `5`. Impostiamo `y` uguale a un
_reference_ a `x`. Possiamo asserire che `x` è uguale a `5`. Tuttavia, se
vogliamo fare un’asserzione sul valore in `y`, dobbiamo usare `*y` per seguire
il _reference_ al valore a cui punta (da qui _de-referenziazione_) in modo che
il compilatore possa confrontare il valore effettivo. Una volta de-referenziato
`y`, abbiamo accesso al valore intero a cui punta `y`, che possiamo confrontare
con `5`.

Se provassimo a scrivere `assert_eq!(5, y);`, otterremmo questo errore di
compilazione:

```console
{{#include ../listings/ch15-smart-pointers/output-only-01-comparing-to-reference/output.txt}}
```

Il confronto tra un numero e un _reference_ a un numero non è consentito perché
sono diversi. Dobbiamo usare l’operatore di de-referenziazione per seguire il
_reference_ al valore a cui punta.

### Utilizzare `Box<T>` Come _Reference_

Possiamo riscrivere il codice nel Listato 15-6 per utilizzare `Box<T>` invece di
un _reference_; l’operatore di de-referenziazione utilizzato su `Box<T>` nel
Listato 15-7 funziona allo stesso modo dell’operatore di de-referenziazione
utilizzato sul _reference_ nel Listato 15-6.

<Listing number="15-7" file-name="src/main.rs" caption="Utilizzare l’operatore di de-referenziazione su una `Box<i32>`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-07/src/main.rs}}
```

</Listing>

La differenza principale tra il Listato 15-7 e il Listato 15-6 è che qui
impostiamo `y` come un’istanza di una _box_ che punta a un valore copiato di `x`
anziché come un _reference_ che punta al valore di `x`. Nell’ultima asserzione,
possiamo usare l’operatore di de-referenziazione per seguire il puntatore della
_box_ nello stesso modo in cui facevamo quando `y` era un _reference_.
Successivamente, esploreremo le peculiarità di `Box<T>` che ci consentono di
utilizzare l’operatore di de-referenziazione definendo un nostro _type_ di
_box_.

### Definire il Nostro Puntatore Intelligente

Creiamo un _type_ _incapsulatore_, simile al _type_ `Box<T>` fornito dalla
libreria standard, per sperimentare come i tipi di puntatore intelligente si
comportino diversamente dai normali _reference_. Poi vedremo come aggiungere la
possibilità di utilizzare l’operatore di de-referenziazione.

> Nota: c’è una grande differenza tra il _type_ `MioBox<T>` che stiamo per
> creare e il vero `Box<T>`: la nostra versione non memorizzerà i dati
> nell’_heap_. Ci stiamo concentrando su `Deref`, quindi dove vengono
> effettivamente memorizzati i dati è meno importante del comportamento “simile”
> a un puntatore.

Il _type_ `Box<T>` è essenzialmente definito come una _struct_ tupla con un
elemento, quindi il Listato 15-8 definisce un _type_ `MioBox<T>` allo stesso
modo. Definiremo anche una funzione `new` che corrisponda alla funzione `new`
definita su `Box<T>`.

<Listing number="15-8" file-name="src/main.rs" caption="Definizione di un _type_ `MioBox<T>`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-08/src/main.rs:here}}
```

</Listing>

Definiamo una _struct_ denominata `MioBox` e dichiariamo un parametro generico
`T` perché vogliamo che il nostro _type_ contenga valori di qualsiasi _type_. Il
_type_ `MioBox` è una _struct_ tupla con un elemento di _type_ `T`. La funzione
`MioBox::new` accetta un parametro di _type_ `T` e restituisce un’istanza di
`MioBox` che contiene il valore passato.

Proviamo ad aggiungere la funzione `main` del Listato 15-7 al Listato 15-8 e
modificarla in modo che utilizzi il _type_ `MioBox<T>` che abbiamo definito
invece di `Box<T>`. Il codice nel Listato 15-9 non verrà compilato perché Rust
non sa come de-referenziare `MioBox`.

<Listing number="15-9" file-name="src/main.rs" caption="Tentativo di utilizzare `MioBox<T>` nello stesso modo in cui abbiamo utilizzato i _reference_ e `Box<T>`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-09/src/main.rs:here}}
```

</Listing>

Ecco l’errore di compilazione risultante:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-09/output.txt}}
```

Il nostro _type_ `MioBox<T>` non può essere de-referenziato perché non abbiamo
implementato tale possibilità sul nostro _type_. Per abilitare la
de-referenziazione con l’operatore `*`, implementiamo il _trait_ `Deref`.

### Implementare il _Trait_ `Deref`

Come discusso in [“Implementare un _Trait_ su un _Type_”][impl-trait]<!-- ignore
--> nel Capitolo 10, per implementare un _trait_ dobbiamo fornire le
implementazioni per i metodi richiesti dal _trait_. Il _trait_ `Deref`, fornito
dalla libreria standard, richiede l’implementazione di un metodo chiamato
`deref` che prende in prestito `self` e restituisce un _reference_ ai dati
interni. Il Listato 15-10 contiene un’implementazione di `Deref` da aggiungere
alla definizione di `MioBox<T>`.

<Listing number="15-10" file-name="src/main.rs" caption="Implementazione di `Deref` su `MioBox<T>`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-10/src/main.rs:here}}
```

</Listing>

La sintassi `type Target = T;` definisce un _type_ associato che il _trait_
`Deref` può utilizzare. I _type_ associati rappresentano un modo leggermente
diverso di dichiarare un parametro generico, ma per ora non è necessario
preoccuparsene; li tratteremo più dettagliatamente nel Capitolo 20.

Nel corpo del metodo `deref` inseriamo `&self.0` in modo che `deref` restituisca
un _reference_ al valore a cui vogliamo accedere con l’operatore `*`; come detto
in [“Creare _Type_ Diversi con _Struct_ Tupla ”][tuple-structs]<!-- ignore -->
nel Capitolo 5, `.0` accede al primo valore in una _struct_ tupla. La funzione
`main` nel Listato 15-10 che chiama `*` sul valore `MioBox<T>` ora si compila e
le asserzioni vengono verificate!

Senza il _trait_ `Deref`, il compilatore può de-referenziare solo i _reference_
`&`. Il metodo `deref` consente al compilatore di accettare un valore di
qualsiasi _type_ che implementi `Deref` e chiamare il metodo `deref` per
ottenere un _reference_ `&` che sa come de-referenziare.

Quando abbiamo inserito `*y` nel Listato 15-10, dietro le quinte Rust ha
effettivamente eseguito questo codice:

```rust,ignore
*(y.deref())
```

Rust sostituisce l’operatore `*` con una chiamata al metodo `deref` e poi un
semplice de-referenziamento, così non dobbiamo pensare se sia necessario o meno
chiamare il metodo `deref`. Questa funzionalità di Rust ci permette di scrivere
codice che funziona nello stesso modo indipendentemente dal fatto che abbiamo un
normale _reference_ o un _type_ che implementa `Deref`.

Il motivo per cui il metodo `deref` restituisce un _reference_ a un valore, e il
fatto che il semplice de-referenziamento al di fuori delle parentesi in
`*(y.deref())` sia ancora necessario, ha a che fare con il sistema di
_ownership_. Se il metodo `deref` restituisse il valore direttamente invece di
un _reference_ al valore, il valore verrebbe spostato fuori da `self`. Non
vogliamo prendere la _ownership_ del valore interno di `MioBox<T>` in questo
caso, né nella maggior parte dei casi in cui utilizziamo l’operatore di
de-referenziazione.

Nota che l’operatore `*` viene sostituito con una chiamata al metodo `deref` e
poi con una chiamata all’operatore `*` una sola volta, ogni volta che
utilizziamo `*` nel nostro codice. Poiché la sostituzione dell’operatore `*` non
è ricorsiva all’infinito, otteniamo dati di _type_ `i32`, che corrispondono al
`5` in `assert_eq!` nel Listato 15-9.

### Usare la De-Referenziazione Forzata in Funzioni e Metodi

La _deref coercion_ converte un _reference_ a un _type_ che implementa il
_trait_ `Deref` in un _reference_ a un altro _type_. Ad esempio, la
de-referenziazione forzata può convertire `&String` in `&str` perché `String`
implementa il _trait_ `Deref` in modo tale da restituire `&str`. La
de-referenziazione forzata è una funzionalità che Rust applica agli argomenti di
funzioni e metodi e funziona solo sui _type_ che implementano il _trait_
`Deref`. Avviene automaticamente quando passiamo un _reference_ al valore di un
_type_ specifico come argomento a una funzione o a un metodo che non corrisponde
al _type_ di parametro nella definizione della funzione o del metodo. Una
sequenza di chiamate al metodo `deref` converte il _type_ fornito nel _type_
richiesto dal parametro.

La _deref coercion_ è stata aggiunta a Rust in modo che i programmatori che
scrivono chiamate di funzioni e metodi non debbano esplicitare troppo spesso i
_reference_ o i _dereference_ con `&` e `*`. La funzionalità di
de-referenziazione forzata ci consente anche di scrivere più codice che può
funzionare sia per _reference_ che per puntatori intelligenti.

Per vedere la _deref coercion_ in azione, utilizziamo il _type_ `MioBox<T>`
definito nel Listato 15-8 e l’implementazione di `Deref` aggiunta nel Listato
15-10. Il Listato 15-11 mostra la definizione di una funzione che ha un
parametro di _type_ _slice_ stringa.

<Listing number="15-11" file-name="src/main.rs" caption="Una funzione `ciao` che ha il parametro `nome` di _type_ `&str`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-11/src/main.rs:here}}
```

</Listing>

Possiamo chiamare la funzione `ciao` con un parametro di _type_ _slice_ stringa
come argomento, ad esempio `ciao("Rust");`. La _deref coercion_ consente di
chiamare `ciao` con un _reference_ a un valore di _type_ `MioBox<String>`, come
mostrato nel Listato 15-12.

<Listing number="15-12" file-name="src/main.rs" caption="Chiamata di `ciao` con un _reference_ a un valore `MioBox<String>`, che funziona grazie alla _deref coercion_">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-12/src/main.rs:here}}
```

</Listing>

Qui chiamiamo la funzione `ciao` con l’argomento `&m`, che è un _reference_ a un
valore `MioBox<String>`. Poiché abbiamo implementato il _trait_ `Deref` su
`MioBox<T>` nel Listato 15-10, Rust può trasformare `&MioBox<String>` in
`&String` chiamando `deref`. La libreria standard fornisce un’implementazione di
`Deref` su `String` che restituisce una _slice_ di stringa, ed è presente nella
documentazione API per `Deref`. Rust chiama nuovamente `deref` per trasformare
`&String` in `&str`, che corrisponde alla definizione della funzione `ciao`.

Se Rust non implementasse la de-referenziazione forzata, dovremmo scrivere il
codice nel Listato 15-13 invece del codice nel Listato 15-12 per chiamare `ciao`
con un valore di tipo `&MioBox<String>`.

<Listing number="15-13" file-name="src/main.rs" caption="Il codice che dovremmo scrivere se Rust non avesse la de-referenziazione forzata">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-13/src/main.rs:here}}
```

</Listing>

`(*m)` de-referenzia `MioBox<String>` in una `String`. Quindi `&` e `[..]`
prendono una _slice_ di `String` che è uguale all’intera stringa per
corrispondere alla firma di `ciao`. Questo codice senza _deref coercion_ con
tutti questi simboli coinvolti è più difficile da leggere, scrivere e
comprendere. La _deref_ consente a Rust di gestire automaticamente queste
conversioni.

Quando il _trait_ `Deref` è definito per i _type_ coinvolti, Rust analizzerà i
_type_ e utilizzerà `Deref::deref` tutte le volte necessarie per ottenere un
_reference_ che corrisponda al _type_ del parametro. Il numero di volte in cui
`Deref::deref` deve essere inserito viene risolto in fase di compilazione,
quindi non ci sono penalità prestazionali in fase di esecuzione per aver
sfruttato la _deref coercion_!

### Gestire la De-referenziazione Forzata con _Reference_ Mutabili

Analogamente a come si usa il _trait_ `Deref` per sovrascrivere l’operatore `*`
sui _reference_ immutabili, è possibile usare il _trait_ `DerefMut` per
sovrascrivere l’operatore `*` sui _reference_ mutabili.

Rust esegue la _deref coercion_ quando trova _type_ e implementazioni di _trait_ in tre
casi:

1. Da `&T` a `&U` quando `T: Deref<Target=U>`
2. Da `&mut T` a `&mut U` quando `T: DerefMut<Target=U>`
3. Da `&mut T` a `&U` quando `T: Deref<Target=U>`

I primi due casi sono gli stessi, tranne per il fatto che il secondo implementa
la mutabilità. Il primo caso afferma che se si ha un `&T` e `T` implementa
`Deref` a un _type_ `U`, è possibile ottenere un `&U` in modo trasparente. Il
secondo caso afferma che la stessa de-referenziazione forzata avviene per i
reference mutabili.

Il terzo caso è più complicato: Rust convertirà anche un _reference_ mutabile a
uno immutabile. Ma il contrario _non_ è possibile: i _reference_ immutabili non
verranno mai convertirti in _reference_ mutabili. A causa delle regole di
prestito, se si ha un _reference_ mutabile, quel _reference_ mutabile deve
essere l’unico _reference_ a quei dati (altrimenti, il programma non verrebbe
compilato). La conversione di un _reference_ mutabile in un _reference_
immutabile non violerà mai le regole di prestito. La conversione di un
_reference_ immutabile in un _reference_ mutabile richiederebbe che il
_reference_ immutabile iniziale sia l’unico _reference_ immutabile a quei dati,
ma le regole di prestito non lo garantiscono. Pertanto, Rust non può dare per
scontato che sia possibile convertire un _reference_ immutabile in un
_reference_ mutabile.

[impl-trait]: ch10-02-traits.html#implementare-un-trait-su-un-type
[tuple-structs]: ch05-01-defining-structs.html#creare-type-diversi-con-struct-tupla 