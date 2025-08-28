## Memorizzazione di Elenchi di Valori con Vettori

Il primo tipo di collezione che esamineremo è `Vec<T>`, nota anche come
_vector_. I vettori consentono di memorizzare più di un valore in un'unica
struttura dati che mette tutti i valori uno accanto all'altro in memoria. I
vettori possono memorizzare solo valori dello stesso _type_. Sono utili quando
si ha un elenco di elementi, come le righe di testo in un file o i prezzi degli
articoli in un carrello.

### Creazione di un Nuovo Vettore

Per creare un nuovo vettore vuoto, chiamiamo la funzione `Vec::new`, come
mostrato nel Listato 8-1.

<Listing number="8-1" caption="Creazione di un nuovo vettore vuoto per contenere valori di _type_ `i32`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-01/src/main.rs:here}}
```

</Listing>

Nota che qui abbiamo aggiunto un'annotazione di _type_. Poiché non stiamo
inserendo alcun valore in questo vettore, Rust non sa che tipo di elementi
intendiamo memorizzare. Questo è un punto importante. I vettori vengono
implementati utilizzando i _type_ generici; spiegheremo come utilizzare i _type_
generici quando si creano _type_ propri nel Capitolo 10. Per ora, sappiamo che
il _type_ `Vec<T>` fornito dalla libreria standard può contenere qualsiasi
_type_. Quando creiamo un vettore per contenere uno specifico _type_, possiamo
specificarlo tra parentesi angolari. Nel Listato 8-1, abbiamo detto a Rust che
`Vec<T>` in `v` conterrà elementi di _type_ `i32`.

Più spesso, si crea un `Vec<T>` con valori iniziali e Rust dedurrà il _type_ dai
valori che si desidera memorizzare, quindi raramente è necessario eseguire
questa annotazione di _type_. Rust fornisce opportunamente la macro `vec!`, che
creerà un nuovo vettore contenente i valori che gli vengono assegnati. Il
Listato 8-2 crea un nuovo `Vec<i32>` che contiene i valori `1`, `2` e `3`. Il
_type_ intero è `i32` perché è il _type_ intero predefinito, come discusso nella
sezione ["Tipi di dati”][data-types]<!-- ignore --> del Capitolo 3.

<Listing number="8-2" caption="Creazione di un nuovo vettore contenente valori">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-02/src/main.rs:here}}
```

</Listing>

Poiché abbiamo assegnato valori iniziali `i32`, Rust può dedurre che il _type_
di `v` è `Vec<i32>` e l'annotazione di _type_ non è necessaria. Successivamente,
vedremo come modificare un vettore.

### Aggiornamento di un Vettore

Per creare un vettore e aggiungervi elementi, possiamo usare il metodo `push`,
come mostrato nel Listato 8-3.

<Listing number="8-3" caption="Utilizzo del metodo `push` per aggiungere valori a un vettore">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-03/src/main.rs:here}}
```

</Listing>

Come per qualsiasi variabile, se vogliamo poterne modificare il valore, dobbiamo
renderla mutabile usando la parola chiave `mut`, come discusso nel Capitolo 3. I
numeri che inseriamo al suo interno sono tutti di _type_ `i32`, e Rust lo deduce
dai dati, quindi non abbiamo bisogno dell'annotazione `Vec<i32>`.

### Lettura degli Elementi dei Vettori

Esistono due modi per fare riferimento a un valore memorizzato in un vettore:
tramite indicizzazione o utilizzando il metodo `get`. Negli esempi seguenti,
abbiamo annotato i _type_ dei valori restituiti da queste funzioni per maggiore
chiarezza.

Il Listato 8-4 mostra entrambi i metodi per accedere a un valore in un vettore,
con la sintassi di indicizzazione e il metodo `get`.

<Listing number="8-4" caption="Utilizzo della sintassi di indicizzazione e utilizzo del metodo `get` per accedere a un elemento in un vettore">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-04/src/main.rs:here}}
```

</Listing>

Sono da notare alcuni dettagli. Utilizziamo il valore di indice `2` per ottenere
il terzo elemento poiché i vettori sono indicizzati per numero, a partire da
zero. Utilizzando `&` e `[]` otteniamo un _reference_ all'elemento a
quell'indice. Quando utilizziamo il metodo `get` con l'indice passato come
argomento, otteniamo un `Option<&T>` che possiamo utilizzare con `match`.

Rust fornisce questi due modi per ottenere un _reference_ ad un elemento, in
modo da poter scegliere come il programma si comporta quando si tenta di
utilizzare un valore di indice al di fuori dell'intervallo di elementi
esistenti. Ad esempio, vediamo cosa succede quando abbiamo un vettore di cinque
elementi e poi proviamo ad accedere a un elemento all'indice 100 con ciascuna
tecnica, come mostrato nel Listato 8-5.

<Listing number="8-5" caption="Tentativo di accesso all'elemento all'indice 100 in un vettore contenente cinque elementi">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-05/src/main.rs:here}}
```

</Listing>

Quando eseguiamo questo codice, il primo metodo `[]` causerà un crash del
programma perché fa riferimento ad un elemento inesistente. Questo metodo è
ideale quando si desidera che il programma si blocchi in caso di tentativo di
accesso a un elemento oltre la fine del vettore.

Quando al metodo `get` viene passato un indice esterno al vettore, restituisce
`None` senza crash. Si consiglia di utilizzare questo metodo se l'accesso a un
elemento oltre l'intervallo del vettore può verificarsi occasionalmente in
circostanze normali. Il codice avrà quindi una logica per gestire sia
`Some(&element)` che `None`, come discusso nel Capitolo 6. Ad esempio, l'indice
potrebbe provenire da un utente che inserisce un numero. Se inserisce
accidentalmente un numero troppo grande e il programma ottiene un valore `None`,
è possibile indicare all'utente quanti elementi sono presenti nel vettore
corrente e dargli un'altra possibilità di inserire un valore valido. Sarebbe più
intuitivo che mandare in crash il programma a causa di un errore di battitura!

Quando il programma ha un _reference_ valido, il sistema applica le regole di
_ownership_ e _borrowing_ (trattate nel Capitolo 4) per garantire che questo
_reference_ e qualsiasi altro _reference_ al contenuto del vettore rimangano
validi. Ricordati la regola che stabilisce che non è possibile avere _reference_
mutabili e immutabili nello stesso _scope_. Questa regola si applica al Listato
8-6, dove manteniamo un _reference_ immutabile al primo elemento di un vettore e
proviamo ad aggiungere un elemento alla fine. Questo programma non funzionerà se
proviamo a fare _reference_ a quell'elemento anche più avanti nella funzione.

<Listing number="8-6" caption="Tentativo di aggiungere un elemento a un vettore in compresenza di un _reference_ all'oggetto">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-06/src/main.rs:here}}
```

</Listing>

La compilazione di questo codice genererà questo errore:

```console
{{#include ../listings/ch08-common-collections/listing-08-06/output.txt}}
```

Il codice nel Listato 8-6 potrebbe sembrare funzionante: perché un _reference_
al primo elemento dovrebbe preoccuparsi delle modifiche alla fine del vettore?
Questo errore è dovuto al funzionamento dei vettori: poiché i vettori
posizionano i valori uno accanto all'altro in memoria, se non c'è abbastanza
spazio per posizionare tutti gli elementi uno accanto all'altro dove il vettore
è attualmente memorizzato l'aggiunta di un nuovo elemento alla fine del vettore
potrebbe richiedere l'allocazione di nuova memoria e la copia dei vecchi
elementi nel nuovo spazio. In tal caso, il _reference_ al primo elemento
punterebbe alla memoria deallocata. Le regole di _borrowing_ impediscono ai
programmi di finire in questa situazione.

> Nota: per maggiori dettagli sull'implementazione del _type_ `Vec<T>`, vedere
> ["Il Rustonomicon”][nomicon].

### Iterazione sui Valori di un Vettore

Per accedere a ogni elemento di un vettore a turno, dovremmo iterare su tutti
gli elementi anziché utilizzare gli indici per accedervi uno alla volta. Il
Listato 8-7 mostra come utilizzare un ciclo `for` per ottenere _reference_
immutabili a ciascun elemento in un vettore di valori `i32` e stamparli.

<Listing number="8-7" caption="Stampa di ogni elemento in un vettore iterando sugli elementi utilizzando un ciclo `for`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-07/src/main.rs:here}}
```

</Listing>

Possiamo anche iterare su _reference_ mutabili a ciascun elemento in un vettore
mutabile per apportare modifiche a tutti gli elementi. Il ciclo `for` nel
Listato 8-8 aggiungerà `50` a ciascun elemento.

<Listing number="8-8" caption="Iterare su _reference_ mutabili a elementi in un vettore">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-08/src/main.rs:here}}
```

</Listing>

Per modificare il valore a cui punta il _reference_ mutabile, dobbiamo usare
l'operatore di dereferenziazione `*` per arrivare al valore in `i` prima di
poter usare l'operatore `+=`. Approfondiremo l'operatore di dereferenziazione
nella sezione [“Seguire il _Reference_ al Valore”][deref]<!-- ignore --> del
Capitolo 15.

L'iterazione su un vettore, sia immutabile che mutabile, è sicura grazie alle
regole di _ownership_ e _borrowing_. Se tentassimo di inserire o rimuovere
elementi nei corpi del ciclo `for` nei Listati 8-7 e 8-8, otterremmo un errore
del compilatore simile a quello ottenuto con il codice nel Listato 8-6. Il
_reference_ al vettore contenuto nel ciclo `for` impedisce la modifica
simultanea dell'intero vettore.

### Utilizzo di un'Enum per Memorizzare Più _Type_

I vettori possono memorizzare solo valori dello stesso _type_. Questo può essere
scomodo; ci sono sicuramente casi d'uso in cui è necessario memorizzare un
elenco di elementi di _type_ diverso. Fortunatamente, le varianti di un'_enum_
sono definite sotto lo stesso _type_ di _enum_, quindi quando abbiamo bisogno di
un _type_ per rappresentare elementi di tipi diversi, possiamo definire e
utilizzare un'_enum_!

Ad esempio, supponiamo di voler ottenere valori da una riga di un foglio di
calcolo in cui alcune colonne della riga contengono numeri interi, alcuni numeri
in virgola mobile e alcune stringhe. Possiamo definire un _enum_ le cui varianti
conterranno i diversi tipi di valore, e tutte le varianti dell'_enum_ saranno
considerate dello stesso _type_: quello dell'_enum_. Possiamo adesso creare un
vettore per contenere quell'_enum_ e quindi, in definitiva, contenere _type_
"diversi". Lo abbiamo dimostrato nel Listato 8-9.

<Listing number="8-9" caption="Definizione di un `enum` per memorizzare valori di _type_ diversi in un vettore">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-09/src/main.rs:here}}
```

</Listing>

Rust deve sapere quali _type_ saranno presenti nel vettore in fase di
compilazione, in modo da sapere esattamente quanta memoria sull'_heap_ sarà
necessaria per memorizzare ogni elemento. Dobbiamo anche essere espliciti sui
_type_ consentiti in questo vettore. Se Rust permettesse a un vettore di
contenere qualsiasi _type_, ci sarebbe la possibilità che uno o più _type_
causassero errori nelle operazioni eseguite sugli elementi del vettore.
L'utilizzo di un _enum_ assieme ad un'espressione `match` significa che Rust
garantirà in fase di compilazione che ogni possibile caso venga gestito, come
discusso nel Capitolo 6.

Se non si conosce l'insieme esaustivo di _type_ che un programma avrà una volta
in esecuzione da memorizzare in un vettore, la tecnica dell'_enum_ non
funzionerà. In alternativa, è possibile utilizzare un oggetto _trait_, che
tratteremo nel Capitolo 18.

Ora che abbiamo discusso alcuni dei modi più comuni per utilizzare i vettori,
assicuratevi di consultare [la documentazione dell'API][vec-api]<!-- ignore -->
per tutti i numerosi metodi utili definiti su `Vec<T>` dalla libreria standard.
Ad esempio, oltre a `push`, un metodo `pop` rimuove e restituisce l'ultimo
elemento.

### L'Eliminazione di un Vettore comporta l'Eliminazione dei suoi Elementi

Come qualsiasi altra `struct`, un vettore viene liberato quando esce dallo
_scope_, come annotato nel Listato 8-10.

<Listing number="8-10" caption="Mostra dove vengono rilasciati il ​​vettore e i suoi elementi">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-10/src/main.rs:here}}
```

</Listing>

Quando il vettore viene rilasciato, anche tutto il suo contenuto viene
rilasciato, il che significa che gli interi in esso contenuti verranno ripuliti.
Il _borrow checker_ (_controllo dei prestiti_) garantisce che qualsiasi
_reference_ al contenuto di un vettore venga utilizzato solo finché il vettore
stesso è valido.

Passiamo al tipo di collezione successivo: `String`!

[data-types]: ch03-02-data-types.html#datatype---tipi-di-dato
[nomicon]: https://doc.rust-lang.org/stable/nomicon/vec/vec.html
[vec-api]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
[deref]: ch15-02-deref.html#seguire-il-reference-al-valore
