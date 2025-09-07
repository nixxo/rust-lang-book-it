## Utilizzo di `Box<T>` per Puntare ai Dati sull'Heap

Il puntatore intelligente più semplice è un box, il cui tipo è scritto
`Box<T>`. I _box_ consentono di memorizzare i dati sull'heap anziché sullo stack.
Ciò che rimane sullo stack è il puntatore ai dati dell'heap. Fare riferimento al Capitolo 4
per esaminare la differenza tra stack e heap.

I box non hanno un overhead di prestazioni, a parte il fatto che memorizzano i dati
sull'heap anziché sullo stack. Ma non hanno nemmeno molte funzionalità extra. Li userete più spesso in queste situazioni:

- Quando avete un tipo la cui dimensione non può essere conosciuta in fase di compilazione e volete
utilizzare un valore di quel tipo in un contesto che richiede una dimensione esatta
- Quando avete una grande quantità di dati e volete trasferire la proprietà ma
assicuratevi che i dati non vengano copiati quando lo fate
- Quando volete possedere un valore e vi interessa solo che sia un tipo che
implementa una caratteristica particolare piuttosto che essere di un tipo specifico

Descriveremo la prima situazione in [“Abilitare i tipi ricorsivi con
i box”](#enabling-recursive-types-with-boxes)<!-- ignore -->. Nel secondo
caso, il trasferimento della ownership di una grande quantità di dati può richiedere molto tempo
perché i dati vengono copiati nello stack. Per migliorare le prestazioni in questa
situazione, possiamo memorizzare la grande quantità di dati sull'heap in un box. Quindi,
solo la piccola quantità di dati del puntatore viene copiata sullo stack, mentre
i dati a cui fa riferimento rimangono in un unico punto dell'heap. Il terzo caso è noto come
_oggetto tratto (trait object)_, e [“Utilizzo di oggetti tratto che consentono valori di
tipi diversi,”][trait-objects]<!-- ignore --> nel Capitolo 18 è dedicato a questo argomento.
Quindi, ciò che imparerete qui lo applicherete di nuovo in quella sezione!

### Utilizzo di `Box<T>` per Memorizzare Dati sull'Heap

Prima di discutere il caso d'uso di archiviazione nell'heap per `Box<T>`, tratteremo la
sintassi e come interagire con i valori memorizzati all'interno di un `Box<T>`.

Il Listato 15-1 mostra come utilizzare un box per memorizzare un valore `i32` sull'heap.

<Listing number="15-1" file-name="src/main.rs" caption="Memorizzazione di un valore `i32` sull'heap tramite un box">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-01/src/main.rs}}
```

</Listing>

Definiamo la variabile `b` come avente il valore di un `Box` che punta al
valore `5`, allocato sull'heap. Questo programma stamperà `b = 5`; in
questo caso, possiamo accedere ai dati nel box in modo simile a come faremmo se questi
dati fossero sullo stack. Proprio come qualsiasi valore posseduto, quando un box esce
dall'ambito, come accade a `b` alla fine di `main`, verrà deallocato. La deallocazione avviene sia per il box (memorizzato sullo stack) sia per i dati a cui punta (memorizzati sull'heap).

Mettere un singolo valore sull'heap non è molto utile, quindi i box non verranno utilizzati molto spesso da soli in questo modo. Avere valori come un singolo `i32` sullo
stack, dove vengono memorizzati di default, è più appropriato nella maggior parte
delle situazioni. Diamo un'occhiata a un caso in cui i box ci consentono di definire tipi che
non saremmo autorizzati a definire se non avessimo box.

### Abilitare i Tipi Ricorsivi con i Box

Un valore di un _tipo ricorsivo (recursive type)_ può avere un altro valore dello stesso tipo come parte di
sé. I tipi ricorsivi pongono un problema perché Rust deve sapere in fase di compilazione
quanto spazio occupa un tipo. Tuttavia, l'annidamento dei valori dei tipi ricorsivi potrebbe teoricamente continuare all'infinito, quindi Rust non può sapere di quanto spazio
ha bisogno il valore. Poiché le boxes hanno dimensioni note, possiamo abilitare i tipi ricorsivi
inserendo una box nella definizione del tipo ricorsivo.

Come esempio di tipo ricorsivo, esploriamo la _cons list_. Questo è un tipo di dato
comunemente presente nei linguaggi di programmazione funzionale. Il tipo di lista dei contro
che definiremo è semplice, fatta eccezione per la ricorsione; pertanto, i
concetti nell'esempio con cui lavoreremo saranno utili ogni volta che ci si troverà
in situazioni più complesse che coinvolgono i tipi ricorsivi.

#### Ulteriori Informazioni Sulla Cons List

Una _Cons List_ è una struttura dati derivata dal linguaggio di programmazione Lisp
e dai suoi dialetti, è composta da coppie annidate ed è la versione Lisp di una
lista concatenata. Il suo nome deriva dalla funzione `cons` (abbreviazione di _construct function_) in Lisp, che costruisce una nuova coppia a partire dai suoi due argomenti.
Chiamando `cons` su una coppia composta da un valore e un'altra coppia, possiamo
costruire cons list composte da coppie ricorsive.

Ad esempio, ecco una rappresentazione in pseudocodice di una cons list contenente la
lista `1, 2, 3` con ciascuna coppia tra parentesi:

```text
(1, (2, (3, Nil)))
```

Ogni elemento in una cons list contiene due elementi: il valore dell'elemento corrente
e l'elemento successivo. L'ultimo elemento della lista contiene solo un valore chiamato `Nil`
senza un elemento successivo. Una cons list viene prodotta chiamando ricorsivamente la funzione `cons`. Il nome canonico per indicare il caso base della ricorsione è `Nil`.
Si noti che questo non è lo stesso del concetto di "null" o "nil" discusso nel
Capitolo 6, che indica un valore non valido o assente.

La cons list non è una struttura dati comunemente utilizzata in Rust. Nella maggior parte dei casi
quando si ha una lista di elementi in Rust, `Vec<T>` è una scelta migliore.
Altri tipi di dati ricorsivi più complessi _sono_ utili in varie situazioni,
ma iniziando con la cons list in questo capitolo, possiamo esplorare come i box
ci consentano di definire un tipo di dati ricorsivo senza troppe distrazioni.

Il Listato 15-2 contiene una definizione enum per una cons list. Si noti che questo codice
non verrà ancora compilato perché il tipo `List` non ha una dimensione nota, che
dimostreremo.

<Listing number="15-2" file-name="src/main.rs" caption="Il primo tentativo di definire un enum per rappresentare una struttura dati di tipo cons list di valori `i32`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-02/src/main.rs:here}}
```

</Listing>

> Nota: stiamo implementando un cons list che contiene solo valori `i32` per
> gli scopi di questo esempio. Avremmo potuto implementarlo utilizzando i generici, come
> discusso nel Capitolo 10, per definire un tipo di cons list in grado di memorizzare valori di
> qualsiasi tipo.

L'utilizzo del tipo `List` per memorizzare l'elenco `1, 2, 3` sarebbe simile al codice nel
Listato 15-3.

<Listing number="15-3" file-name="src/main.rs" caption="Utilizzo dell'enum `List` per memorizzare la lista `1, 2, 3`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-03/src/main.rs:here}}
```

</Listing>

Il primo valore `Cons` contiene `1` e un altro valore `List`. Questo valore `List` è
un altro valore `Cons` che contiene `2` e un altro valore `List`. Questo valore `List`
è un altro valore `Cons` che contiene `3` e un valore `List`, che è infine
`Nil`, la variante non ricorsiva che segnala la fine della lista.

Se proviamo a compilare il codice nel Listato 15-3, otteniamo l'errore mostrato nel
Listato 15-4.

<Listing number="15-4" caption="L'errore che otteniamo quando si tenta di definire un enum ricorsivo">

```console
{{#include ../listings/ch15-smart-pointers/listing-15-03/output.txt}}
```

</Listing>

L'errore indica che questo tipo "ha dimensione infinita". Il motivo è che abbiamo definito
`List` con una variante ricorsiva: contiene direttamente un altro valore di se stesso. Di conseguenza, Rust non riesce a calcolare quanto spazio è necessario per memorizzare un
valore `List`. Analizziamo il motivo per cui otteniamo questo errore. Innanzitutto, vedremo come
Rust determina quanto spazio è necessario per memorizzare un valore di un tipo non ricorsivo.

#### Calcolo della Dimensione di un Tipo Non Ricorsivo

Ricordate l'enum `Message` che abbiamo definito nel Listato 6-2 quando abbiamo discusso le definizioni
degli enum nel Capitolo 6:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

Per determinare quanto spazio allocare per un valore `Message`, Rust esamina
ciascuna delle varianti per vedere quale variante necessita di più spazio. Rust
vede che `Message::Quit` non necessita di spazio, `Message::Move` necessita di spazio sufficiente
per memorizzare due valori `i32` e così via. Poiché verrà
utilizzata una sola variante, lo spazio massimo di cui un valore `Message` avrà bisogno è lo spazio che richiederebbe
per memorizzare la più grande delle sue varianti.

Confrontiamo questo con ciò che accade quando Rust cerca di determinare quanto spazio necessita un
tipo ricorsivo come l'enum `List` nel Listato 15-2. Il compilatore inizia
esaminando la variante `Cons`, che contiene un valore di tipo `i32` e un valore
di tipo `List`. Pertanto, `Cons` necessita di una quantità di spazio pari alla dimensione di
un `i32` più la dimensione di un `List`. Per calcolare la quantità di memoria necessaria per il tipo `List`, il compilatore esamina le varianti, a partire dalla variante `Cons`. La variante `Cons` contiene un valore di tipo `i32` e un valore di tipo
`List`, e questo processo continua all'infinito, come mostrato nella Figura 15-1.

<img alt="Una lista Cons infinita: un rettangolo etichettato 'Cons' diviso in due rettangoli più piccoli. Il primo rettangolo più piccolo contiene l'etichetta 'i32', e il secondo rettangolo più piccolo contiene l'etichetta 'Cons' e una versione più piccola del rettangolo 'Cons' esterno. I rettangoli 'Cons' continuano a contenere versioni sempre più piccole di se stessi finché il rettangolo più piccolo, di dimensioni adeguate, contiene un simbolo di infinito, a indicare che questa ripetizione continua all'infinito" src="img/trpl15-01.svg" class="center" style="width: 50%;" />

<span class="caption">Figura 15-1: Una `List` infinita composta da infinite
varianti `Cons`</span>

#### Utilizzo di `Box<T>` per Ottenere un Yipo Ricorsivo con una Dimensione Nota

Poiché Rust non riesce a calcolare quanto spazio allocare per i tipi definiti
ricorsivamente, il compilatore genera un errore con questo utile suggerimento:

<!-- manual-regeneration
after doing automatic regeneration, look at listings/ch15-smart-pointers/listing-15-03/output.txt and copy the relevant line
-->

```text
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +
```

In questo suggerimento, _indirection_ significa che invece di memorizzare un valore
direttamente, dovremmo modificare la struttura dati per memorizzarlo indirettamente,
memorizzando invece un puntatore al valore.

Poiché `Box<T>` è un puntatore, Rust sa sempre quanto spazio un `Box<T>`
necessita: la dimensione di un puntatore non cambia in base alla quantità di dati a cui
punta. Questo significa che possiamo inserire `Box<T>` all'interno della variante `Cons` invece
di un altro valore `List` direttamente. `Box<T>` punterà al successivo valore `List`
che si troverà nell'heap anziché all'interno della variante `Cons`.
Concettualmente, abbiamo ancora una lista, creata con liste che contengono altre liste, ma
questa implementazione ora è più simile al posizionamento degli elementi uno accanto all'altro
piuttosto che uno dentro l'altro.

Possiamo modificare la definizione dell'enum `List` nel Listato 15-2 e l'utilizzo
di `List` nel Listato 15-3 con il codice nel Listato 15-5, che verrà compilato.

<Listing number="15-5" file-name="src/main.rs" caption="Definizione di `List` che utilizza `Box<T>` per avere una dimensione nota">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-05/src/main.rs}}
```

</Listing>

La variante `Cons` richiede la dimensione di un `i32` più lo spazio per memorizzare
i dati del puntatore del box. La variante `Nil` non memorizza alcun valore, quindi necessita di meno spazio
nello stack rispetto alla variante `Cons`. Ora sappiamo che qualsiasi valore `List` occuperà
le dimensioni di un `i32` più le dimensioni dei dati del puntatore di un box. Utilizzando un
box, abbiamo interrotto la catena infinita e ricorsiva, in modo che il compilatore possa calcolare
la dimensione necessaria per memorizzare un valore `List`. La Figura 15-2 mostra l'aspetto attuale della variante `Cons`.

<img alt="Un rettangolo etichettato 'Cons' diviso in due rettangoli più piccoli. Il primo rettangolo più piccolo contiene l'etichetta 'i32', e il secondo rettangolo più piccolo contiene l'etichetta 'Box' con un rettangolo interno che contiene l'etichetta 'usize', che rappresenta la dimensione finita del puntatore del box" src="img/trpl15-02.svg" class="center" />

<span class="caption">Figura 15-2: Una `List` che non ha dimensioni infinite
perché `Cons` contiene un `Box`</span>

I box forniscono solo l'indirezione e l'allocazione dell'heap; non hanno
altre funzionalità speciali, come quelle che vedremo con gli altri tipi di puntatori intelligenti. Inoltre, non hanno il sovraccarico di prestazioni che queste funzionalità
speciali comportano, quindi possono essere utili in casi come la cons list, in cui l'indirezione è l'unica funzionalità di cui abbiamo bisogno. Esamineremo altri casi d'uso per i box nel
Capitolo 18.

Il tipo `Box<T>` è un puntatore intelligente perché implementa il tratto `Deref`,
che consente di trattare i valori `Box<T>` come riferimenti. Quando un valore `Box<T>`
esce dall'ambito, anche i dati dell'heap a cui punta il box vengono ripuliti
grazie all'implementazione del tratto `Drop`. Questi due tratti saranno
ancora più importanti per le funzionalità fornite dagli altri tipi di puntatore intelligente che discuteremo nel resto di questo capitolo. Esploriamo questi due tratti
più in dettaglio.

[trait-objects]: ch18-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
