## Trattare i puntatori intelligenti come riferimenti regolari con `Deref`

<!-- Old link, do not remove -->

<a id="treating-smart-pointers-like-regular-references-with-the-deref-trait"></a>

L'implementazione del trait `Deref` consente di personalizzare il comportamento dell'operatore di _dereference_ `*` (da non confondere con l'operatore di moltiplicazione o glob). Implementando `Deref` in modo tale che un puntatore intelligente possa essere
trattato come un riferimento normale, è possibile scrivere codice che opera sui
riferimenti e utilizzarlo anche con i puntatori intelligenti.

Vediamo prima come funziona l'operatore di dereference con i riferimenti normali.
Poi proveremo a definire un tipo personalizzato che si comporti come `Box<T>` e vedremo perché
l'operatore di dereference non funziona come un riferimento sul nostro nuovo
tipo definito. Esploreremo come l'implementazione del trait `Deref` consenta ai
puntatori intelligenti di funzionare in modo simile ai riferimenti. Infine, esamineremo la funzionalità di Rust _deref coercion_ e come ci consenta di lavorare sia con i riferimenti
che con i puntatori intelligenti.

<!-- Old links, do not remove -->
<a id="following-the-pointer-to-the-value-with-the-dereference-operator"></a>
<a id="following-the-pointer-to-the-value"></a>

### Seguire il _Reference_ al Valore

Un riferimento regolare è un tipo di puntatore, un modo per pensare a un puntatore è
immaginare una freccia che punta verso un valore memorizzato altrove. Nel Listato 15-6, creiamo un
riferimento a un valore `i32` e poi utilizziamo l'operatore di de-referenziazione per seguire il
riferimento al valore.

<Listing number="15-6" file-name="src/main.rs" caption="Utilizzo dell'operatore di de-referenziazione per seguire un riferimento a un valore `i32`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-06/src/main.rs}}
```

</Listing>

La variabile `x` contiene un valore `i32` `5`. Impostiamo `y` uguale a un riferimento a
`x`. Possiamo affermare che `x` è uguale a `5`. Tuttavia, se vogliamo fare un'asserzione sul valore in `y`, dobbiamo usare `*y` per seguire il riferimento
al valore a cui punta (da qui _dereference_) in modo che il compilatore possa confrontare
il valore effettivo. Una volta de-referenziato `y`, abbiamo accesso al valore intero
a cui punta `y`, che possiamo confrontare con `5`.

Se provassimo a scrivere `assert_eq!(5, y);`, otterremmo questo
errore di compilazione:

```console
{{#include ../listings/ch15-smart-pointers/output-only-01-comparing-to-reference/output.txt}}
```

Il confronto tra un numero e un riferimento a un numero non è consentito perché
sono diversi. Dobbiamo usare l'operatore di de-referenziazione per seguire il
riferimento al valore a cui punta.

### Utilizzo di `Box<T>` come riferimento

Possiamo riscrivere il codice nel Listato 15-6 per utilizzare `Box<T>` invece di
un riferimento; l'operatore di de-referenziazione utilizzato su `Box<T>` nel
Listato 15-7 funziona allo stesso modo dell'operatore di de-referenziazione
utilizzato sul riferimento nel Listato 15-6.

<Listing number="15-7" file-name="src/main.rs" caption="Utilizzo dell'operatore di de-referenziazione su un `Box<i32>`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-07/src/main.rs}}
```

</Listing>

La differenza principale tra il Listato 15-7 e il Listato 15-6 è che qui impostiamo
`y` come un'istanza di un box che punta a un valore copiato di `x` anziché come
un riferimento che punta al valore di `x`. Nell'ultima asserzione, possiamo usare
l'operatore di de-referenziazione per seguire il puntatore del box nello stesso modo in cui facevamo
quando `y` era un riferimento. Successivamente, esploreremo le peculiarità di `Box<T>`
che ci consentono di utilizzare l'operatore di de-referenziazione definendo il nostro tipo di box.

### Definizione del nostro puntatore intelligente

Creiamo un tipo wrapper simile al tipo `Box<T>` fornito dalla
libreria standard per sperimentare come i tipi di puntatore intelligente si comportino diversamente dai
riferimenti di default. Poi vedremo come aggiungere la possibilità di utilizzare l'operatore
di de-referenziazione.

> Nota: c'è una grande differenza tra il tipo `MioBox<T>` che stiamo per
> creare e il vero `Box<T>`: la nostra versione non memorizzerà i dati nell'heap.
> Ci stiamo concentrando su `Deref`, quindi dove vengono effettivamente memorizzati i dati
> è meno importante del comportamento "simile" a un puntatore.

Il tipo `Box<T>` è in definitiva definito come una struttura tupla con un elemento, quindi
il Listato 15-8 definisce un tipo `MioBox<T>` allo stesso modo. Definiremo anche una
funzione `new` che corrisponda alla funzione `new` definita su `Box<T>`.

<Listing number="15-8" file-name="src/main.rs" caption="Definizione di un tipo `MioBox<T>`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-08/src/main.rs:here}}
```

</Listing>

Definiamo una struttura denominata `MioBox` e dichiariamo un parametro generico `T` perché
vogliamo che il nostro tipo contenga valori di qualsiasi tipo. Il tipo `MioBox` è una struttura tupla
con un elemento di tipo `T`. La funzione `MioBox::new` accetta un parametro di
tipo `T` e restituisce un'istanza di `MioBox` che contiene il valore passato.

Proviamo ad aggiungere la funzione `main` del Listato 15-7 al Listato 15-8 e
modificarla in modo che utilizzi il tipo `MioBox<T>` che abbiamo definito invece di `Box<T>`. Il
codice nel Listato 15-9 non verrà compilato perché Rust non sa come de-referenziare
`MioBox`.

<Listing number="15-9" file-name="src/main.rs" caption="Tentativo di utilizzare `MioBox<T>` nello stesso modo in cui abbiamo utilizzato i riferimenti e `Box<T>`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-09/src/main.rs:here}}
```

</Listing>

Ecco l'errore di compilazione risultante:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-09/output.txt}}
```

Il nostro tipo `MioBox<T>` non può essere de-referenziato perché non abbiamo implementato tale
possibilità sul nostro tipo. Per abilitare la de-referenziazione con l'operatore `*`,
implementiamo la caratteristica `Deref`.

<!-- Old link, do not remove -->

<a id="treating-a-type-like-a-reference-by-implementing-the-deref-trait"></a>

### Implementazione del tratto `Deref`

Come discusso in [“Implementazione di un _Trait_ su un _Type_”][impl-trait]<!-- ignore --> nel
Capitolo 10, per implementare un tratto dobbiamo fornire le implementazioni per
i metodi richiesti dal tratto. Il tratto `Deref`, fornito dalla libreria standard,
richiede l'implementazione di un metodo chiamato `deref` che prende in prestito `self` e
restituisce un riferimento ai dati interni. Il Listato 15-10 contiene un'implementazione
di `Deref` da aggiungere alla definizione di `MioBox<T>`.

<Listing number="15-10" file-name="src/main.rs" caption="Implementazione di `Deref` su `MioBox<T>`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-10/src/main.rs:here}}
```

</Listing>

La sintassi `type Target = T;` definisce un tipo associato che il tratto `Deref`
può utilizzare. I tipi associati rappresentano un modo leggermente diverso di dichiarare un
parametro generico, ma per ora non è necessario preoccuparsene; li tratteremo
più dettagliatamente nel Capitolo 20.

Compiliamo il corpo del metodo `deref` con `&self.0` in modo che `deref` restituisca un
riferimento al valore a cui vogliamo accedere con l'operatore `*`; richiamare da
[“_Struct_ Tupla Senza Campi Denominati per creare _Type_ Diversi”][tuple-structs]<!-- ignore --> Nel Capitolo 5, `.0` accede al primo
valore in una struttura di tupla. La funzione `main` nel Listato 15-10 che chiama `*` sul
valore `MioBox<T>` ora compila e le asserzioni passano!

Senza il tratto `Deref`, il compilatore può de-referenziare solo i riferimenti `&`.
Il metodo `deref` consente al compilatore di accettare un valore di qualsiasi tipo
che implementi `Deref` e chiamare il metodo `deref` per ottenere un riferimento `&` che
sa come de-referenziare.

Quando abbiamo inserito `*y` nel Listato 15-10, dietro le quinte Rust ha effettivamente eseguito questo
codice:

```rust,ignore
*(y.deref())
```

Rust sostituisce l'operatore `*` con una chiamata al metodo `deref` e poi un
semplice de-referenziamento, così non dobbiamo pensare se sia necessario o meno
chiamare il metodo `deref`. Questa funzionalità di Rust ci permette di scrivere codice che funziona
identicamente, indipendentemente dal fatto che abbiamo un riferimento normale o un tipo che implementa
`Deref`.

Il motivo per cui il metodo `deref` restituisce un riferimento a un valore, e il fatto che il
semplice de-referenziamento al di fuori delle parentesi in `*(y.deref())` sia ancora necessario,
ha a che fare con il sistema di ownership. Se il metodo `deref` restituisse il valore
direttamente invece di un riferimento al valore, il valore verrebbe spostato fuori da
`self`. Non vogliamo assumere la proprietà del valore interno di `MioBox<T>` in
questo caso, né nella maggior parte dei casi in cui utilizziamo l'operatore di de-referenziazione.

Nota che l'operatore `*` viene sostituito con una chiamata al metodo `deref` e
poi con una chiamata all'operatore `*` una sola volta, ogni volta che utilizziamo `*` nel nostro codice.
Poiché la sostituzione dell'operatore `*` non è ricorsiva all'infinito,
otteniamo dati di tipo `i32`, che corrispondono al `5` in `assert_eq!` nel
Listato 15-9.

### De-referenziazione Forzata Implicita in Funzioni e Metodi

_Deref coercion_ converte un riferimento a un tipo che implementa il tratto `Deref`
in un riferimento a un altro tipo. Ad esempio, la coercizione di deref può convertire
`&String` in `&str` perché `String` implementa il tratto `Deref` in modo tale da
restituire `&str`. La coercizione di deref è una funzionalità che Rust applica agli argomenti di
funzioni e metodi e funziona solo sui tipi che implementano il trait `Deref`. Avviene automaticamente quando passiamo un riferimento al valore di un tipo specifico
come argomento a una funzione o a un metodo che non corrisponde al tipo di parametro
nella definizione della funzione o del metodo. Una sequenza di chiamate al metodo `deref`
converte il tipo fornito nel tipo richiesto dal parametro.

La Deref coercion è stata aggiunta a Rust in modo che i programmatori che scrivono chiamate di funzioni e
metodi non debbano aggiungere così tanti riferimenti e de-referenziazioni espliciti
con `&` e `*`. La funzionalità di deref coercion ci consente anche di scrivere più codice
che può funzionare sia per riferimenti che per puntatori intelligenti.

Per vedere la deref coercion in azione, utilizziamo il tipo `MioBox<T>` definito nel
Listato 15-8 e l'implementazione di `Deref` aggiunta nel Listato
15-10. Il Listato 15-11 mostra la definizione di una funzione che ha un parametro di tipo `slice` stringa.

<Listing number="15-11" file-name="src/main.rs" caption="Una funzione `hello` che ha il parametro `name` di tipo `&str`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-11/src/main.rs:here}}
```

</Listing>

Possiamo chiamare la funzione `hello` con un parametro di tipo `slice` stringa come argomento, ad esempio
`hello("Rust");`. La coercizione di deref consente di chiamare `hello`
con un riferimento a un valore di tipo `MioBox<String>`, come mostrato nel Listato 15-12.

<Listing number="15-12" file-name="src/main.rs" caption="Chiamata di `hello` con un riferimento a un valore `MioBox<String>`, che funziona grazie alla deref coercion">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-12/src/main.rs:here}}
```

</Listing>

Qui chiamiamo la funzione `hello` con l'argomento `&m`, che è un
riferimento a un valore `MioBox<String>`. Poiché abbiamo implementato il tratto `Deref`
su `MioBox<T>` nel Listato 15-10, Rust può trasformare `&MioBox<String>` in `&String`
chiamando `deref`. La libreria standard fornisce un'implementazione di `Deref`
su `String` che restituisce una porzione di stringa, ed è presente nella documentazione API
per `Deref`. Rust chiama nuovamente `deref` per trasformare `&String` in `&str`, che
corrisponde alla definizione della funzione `hello`.

Se Rust non implementasse la deref , dovremmo scrivere il codice nel
Listato 15-13 invece del codice nel Listato 15-12 per chiamare `hello` con un valore
di tipo `&MioBox<String>`.

<Listing number="15-13" file-name="src/main.rs" caption="Il codice che dovremmo scrivere se Rust non avesse la coercizione di de-referenziazione">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-13/src/main.rs:here}}
```

</Listing>

`(*m)` de-referenzia `MioBox<String>` in una `String`. Quindi `&` e
`[..]` prendono una porzione di stringa di `String` che è uguale all'intera stringa per
corrispondere alla firma di `hello`. Questo codice senza deref coercion con tutti questi simboli coinvolti è più difficile da
leggere, scrivere e comprendere. La deref 
consente a Rust di gestire automaticamente queste conversioni.

Quando il tratto `Deref` è definito per i tipi coinvolti, Rust analizzerà i
tipi e utilizzerà `Deref::deref` tutte le volte necessarie per ottenere un riferimento che
corrisponda al tipo del parametro. Il numero di volte in cui `Deref::deref` deve essere
inserito viene risolto in fase di compilazione, quindi non ci sono penalità in fase di esecuzione per aver sfruttato
la deref coercion!

### Come la Deref Coercion Interagisce con la Mutabilità

Analogamente a come si usa il tratto `Deref` per sovrascrivere l'operatore `*` sui
riferimenti immutabili, è possibile usare il tratto `DerefMut` per sovrascrivere l'operatore `*`
sui riferimenti mutabili.

Rust esegue la deref coercion quando trova tipi e implementazioni di traits in tre
casi:

1. Da `&T` a `&U` quando `T: Deref<Target=U>`
2. Da `&mut T` a `&mut U` quando `T: DerefMut<Target=U>`
3. Da `&mut T` a `&U` quando `T: Deref<Target=U>`

I primi due casi sono gli stessi, tranne per il fatto che il secondo implementa la mutabilità. Il primo caso afferma che se si ha un `&T` e `T` implementa `Deref` a
un tipo `U`, è possibile ottenere un `&U` in modo trasparente. Il secondo caso afferma che la
stessa deref coercion avviene per i riferimenti mutabili.

Il terzo caso è più complicato: Rust convertirà anche un riferimento mutabile a uno
immutabile. Ma il contrario _non_ è possibile: i riferimenti immutabili non convertiranno mai in riferimenti mutabili. A causa delle regole di borrowing, se si ha
un riferimento mutabile, quel riferimento mutabile deve essere l'unico riferimento a quei
dati (altrimenti, il programma non verrebbe compilato). La conversione di un riferimento mutabile
in un riferimento immutabile non violerà mai le regole di borrowing.
La conversione di un riferimento immutabile in un riferimento mutabile richiederebbe che il
riferimento immutabile iniziale sia l'unico riferimento immutabile a quei dati, ma
le regole di borrowing non lo garantiscono. Pertanto, Rust non può dare per scontato
che sia possibile convertire un riferimento immutabile in un riferimento mutabile.

[impl-trait]: ch10-02-traits.html#implementazione-di-un-trait-su-un-type
[tuple-structs]: ch05-01-defining-structs.html#struct-tupla-senza-campi-denominati-per-creare-type-diversi