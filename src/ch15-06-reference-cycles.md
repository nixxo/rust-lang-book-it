## I Cicli di Riferimento Possono Causare Perdite di Memoria

Le garanzie di sicurezza della memoria di Rust rendono difficile, ma non impossibile,
creare accidentalmente memoria che non viene mai ripulita (nota come _perdita di memoria_).
Prevenire completamente le perdite di memoria non è una delle garanzie di Rust, il che significa
che le perdite di memoria sono sicure in Rust. Possiamo vedere che Rust consente perdite di memoria
utilizzando `Rc<T>` e `RefCell<T>`: è possibile creare riferimenti in cui
gli elementi si riferiscono l'uno all'altro in un ciclo. Questo crea perdite di memoria perché il
conteggio dei riferimenti di ciascun elemento nel ciclo non raggiungerà mai 0 e i valori
non verranno mai eliminati.

### Creazione di un Ciclo di Riferimento

Esaminiamo come potrebbe verificarsi un ciclo di riferimento e come prevenirlo,
iniziando con la definizione dell'enum `List` e di un metodo `tail` nel Listato
15-25.

<Listing number="15-25" file-name="src/main.rs" caption="Una definizione di elenco Cons che contiene un `RefCell<T>` in modo da poter modificare a cosa fa riferimento una variante `Cons`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-25/src/main.rs}}
```

</Listing>

Stiamo utilizzando un'altra variante della definizione `List` del Listato 15-5. Il
secondo elemento nella variante `Cons` è ora `RefCell<Rc<List>>`, il che significa che
invece di poter modificare il valore `i32` come abbiamo fatto nel Listato
15-24, vogliamo modificare il valore `List` a cui punta una variante `Cons`. Stiamo anche aggiungendo un metodo `tail` per facilitare l'accesso al
secondo elemento se abbiamo una variante `Cons`.

Nel Listato 15-26, stiamo aggiungendo una funzione `main` che utilizza le definizioni nel
Listato 15-25. Questo codice crea una lista in `a` e una lista in `b` che punta
alla lista in `a`. Quindi modifica la lista in `a` per puntare a `b`, creando un
ciclo di riferimenti. Ci sono istruzioni `println!` lungo il percorso per mostrare quali sono i
conteggi dei riferimenti in vari punti di questo processo.

<Listing number="15-26" file-name="src/main.rs" caption="Creazione di un ciclo di riferimento di due valori `List` che puntano l'uno all'altro">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-26/src/main.rs:here}}
```

</Listing>

Creiamo un'istanza `Rc<List>` che contiene un valore `List` nella variabile `a`
con una lista iniziale di `5, Nil`. Creiamo quindi un'istanza `Rc<List>` che contiene
un altro valore `List` nella variabile `b` che contiene il valore `10` e punta
alla lista in `a`.

Modifichiamo `a` in modo che punti a `b` invece che a `Nil`, creando un ciclo. Lo facciamo
utilizzando il metodo `tail` per ottenere un riferimento a `RefCell<Rc<List>>`
in `a`, che inseriamo nella variabile `link`. Quindi utilizziamo il metodo `borrow_mut`
su `RefCell<Rc<List>>` per modificare il valore al suo interno da `Rc<List>`
che contiene un valore `Nil` a `Rc<List>` in `b`.

Quando eseguiamo questo codice, lasciando l'ultimo `println!` commentato per il momento,
otterremo questo output:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-26/output.txt}}
```

Il conteggio dei riferimenti delle istanze di `Rc<List>` sia in `a` che in `b` è 2 dopo
aver modificato la lista in `a` in modo che punti a `b`. Alla fine di `main`, Rust elimina la
variabile `b`, che riduce il conteggio dei riferimenti dell'istanza `b` `Rc<List>`
da 2 a 1. La memoria che `Rc<List>` ha sull'heap non verrà eliminata in
questo punto perché il suo conteggio dei riferimenti è 1, non 0. Quindi Rust elimina `a`, che
riduce anche il conteggio dei riferimenti dell'istanza `a` `Rc<List>` da 2 a 1. Anche la memoria di questa istanza non può essere eliminata, perché l'altra
istanza `Rc<List>` fa ancora riferimento ad essa. La memoria allocata alla lista
rimarrà non utilizzata per sempre. Per visualizzare questo ciclo di riferimenti, abbiamo creato il
diagramma in Figura 15-4.

<img alt="Un rettangolo etichettato 'a' che punta a un rettangolo contenente l'intero 5. Un rettangolo etichettato 'b' che punta a un rettangolo contenente l'intero 10. Il rettangolo contenente 5 punta al rettangolo contenente 10, e il rettangolo contenente 10 punta a sua volta al rettangolo contenente 5, creando un ciclo" src="img/trpl15-04.svg" class="center" />

<span class="caption">Figura 15-4: Un ciclo di riferimento delle liste `a` e `b`
che puntano l'una all'altra</span>

Se si rimuove il commento dall'ultimo `println!` ed si esegue il programma, Rust proverà a stampare
questo ciclo con `a` che punta a `b` che punta a `a` e così via fino a quando
non va in overflow.

Rispetto a un programma reale, le conseguenze della creazione di un ciclo di riferimento
in questo esempio non sono poi così gravi: subito dopo aver creato il ciclo di riferimento,
il programma termina. Tuttavia, se un programma più complesso allocasse molta memoria
in un ciclo e la mantenesse per un lungo periodo, utilizzerebbe più memoria
del necessario e potrebbe sovraccaricare il sistema, causando l'esaurimento
della memoria disponibile.

Creare cicli di riferimento non è facile, ma non è nemmeno impossibile.
Se si hanno valori `RefCell<T>` che contengono valori `Rc<T>` o simili combinazioni annidate
di tipi con mutabilità interna e conteggio dei riferimenti, è necessario
assicurarsi di non creare cicli; non ci si può affidare a Rust per individuarli.
Creare un ciclo di riferimento rappresenterebbe un bug logico nel programma che si dovrebbe
utilizzare per minimizzare test automatizzati, revisioni del codice e altre pratiche di sviluppo software.

Un'altra soluzione per evitare i cicli di riferimento è riorganizzare le strutture dati
in modo che alcuni riferimenti esprimano la proprietà e altri no.
Di conseguenza, si possono avere cicli composti da alcune relazioni di proprietà e
alcune relazioni di non proprietà, e solo le relazioni di proprietà influiscono
sulla possibilità o meno di eliminare un valore. Nel Listato 15-25, vogliamo sempre che le varianti `Cons`
posseggano la propria lista, quindi riorganizzare la struttura dati non è possibile. Diamo un'occhiata a un esempio che utilizza grafici composti da nodi padre e nodi figlio
per vedere quando le relazioni di non proprietà sono un modo appropriato per prevenire
i cicli di riferimento.

<!-- Old link, do not remove -->

<a id="preventing-reference-cycles-turning-an-rct-into-a-weakt"></a>

### Prevenzione dei Cicli di Riferimenti: Trasforma un `Rc<T>` in un `Weak<T>`

Finora, abbiamo dimostrato che la chiamata a `Rc::clone` aumenta lo `strong_count`
di un'istanza di `Rc<T>` e che un'istanza di `Rc<T>` viene pulita solo se il suo
`strong_count` è 0. È anche possibile creare un _weak reference (riferimento debole)_ al valore all'interno
di un'istanza di `Rc<T>` chiamando `Rc::downgrade` e passando un riferimento a
`Rc<T>`. Gli _Strong references_ rappresentano il modo in cui è possibile condividere la proprietà di un'istanza di `Rc<T>`. I _weak references_ non esprimono una relazione di ownership e il loro
conteggio non influisce sulla pulizia di un'istanza di `Rc<T>`. Non causeranno un
ciclo di riferimento perché qualsiasi ciclo che coinvolga riferimenti deboli verrà interrotto
quando il conteggio dei valori coinvolti nei riferimenti forti sarà pari a 0.

Quando si chiama `Rc::downgrade`, si ottiene un puntatore intelligente di tipo `Weak<T>`.
Invece di aumentare di 1 il valore `strong_count` nell'istanza di `Rc<T>`, la chiamata
`Rc::downgrade` aumenta di 1 il valore `weak_count`. Il tipo `Rc<T>` utilizza
`weak_count` per tenere traccia del numero di riferimenti `Weak<T>` esistenti, in modo simile a
`strong_count`. La differenza è che `weak_count` non deve essere 0 affinché l'istanza
`Rc<T>` venga pulita.

Poiché il valore a cui fa riferimento `Weak<T>` potrebbe essere stato eliminato, per
fare qualsiasi cosa con il valore a cui `Weak<T>` punta, è necessario assicurarsi che
il valore esista ancora. Per farlo, dovete chiamare il metodo `upgrade` su un'istanza `Weak<T>`
che restituirà `Option<Rc<T>>`. Otterrete il risultato `Some`
se il valore `Rc<T>` non è stato ancora eliminato e il risultato `None` se il valore
`Rc<T>` è stato eliminato. Poiché `upgrade` restituisce `Option<Rc<T>>`,
Rust garantirà che i casi `Some` e `None` vengano gestiti e
che non ci saranno puntatori non validi.

Ad esempio, invece di utilizzare un elenco i cui elementi conoscono solo l'elemento successivo, creeremo un albero i cui elementi conoscono i loro elementi figlio e
i loro elementi padre.

#### Creazione di una Struttura Dati ad Albero: un `nodo` con Nodi Figlio

Per iniziare, creeremo un albero con nodi che conoscono i loro nodi figlio. Creeremo una struttura denominata `Node` che contiene il proprio valore `i32` e
i riferimenti ai valori dei suoi `Node` figli:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:here}}
```

Vogliamo che un `Node` possieda i suoi figli e vogliamo condividere tale proprietà con
le variabili in modo da poter accedere direttamente a ciascun `Node` nell'albero. Per fare ciò,
definiamo gli elementi `Vec<T>` come valori di tipo `Rc<Node>`. Vogliamo anche
modificare quali nodi sono figli di un altro nodo, quindi abbiamo una `RefCell<T>` in
`figlio` attorno a `Vec<Rc<Node>>`.

Successivamente, utilizzeremo la nostra definizione di struttura e creeremo un'istanza `Node` denominata
`foglia` con valore `3` e nessun elemento figlio, e un'altra istanza denominata `ramo`
con valore `5` e `foglia` come elemento figlio, come mostrato nel Listato 15-27.

<Listing number="15-27" file-name="src/main.rs" caption="Creazione di un nodo `foglia` senza figli e di un nodo `ramo` con `foglia` come figlio">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-27/src/main.rs:there}}
```

</Listing>

Cloniamo `Rc<Node>` in `foglia` e lo memorizziamo in `ramo`, il che significa che il
`Node` in `foglia` ora ha due proprietari: `foglia` e `ramo`. Possiamo passare da
`ramo` a `foglia` tramite `ramo.figlioren`, ma non c'è modo di passare da
`foglia` a `ramo`. Il motivo è che `foglia` non ha alcun riferimento a `ramo` e
non sa che sono correlati. Vogliamo che `foglia` sappia che `ramo` è il suo
genitore. Lo faremo ora.

#### Aggiungere un Riferimento da un Nodo Figlio al Padre

Per far sì che il nodo figlio riconosca il suo padre, dobbiamo aggiungere un campo `padre` alla
definizione della nostra struttura `Node`. Il problema sta nel decidere quale tipo di
`padre` debba essere. Sappiamo che non può contenere un `Rc<T>`, perché ciò creerebbe
un ciclo di riferimenti con `foglia.padre` che punta a `ramo` e
`ramo.figlio` che punta a `foglia`, il che farebbe sì che i loro valori `strong_count`
non siano mai pari a 0.

Pensando alle relazioni in un altro modo, un nodo padre dovrebbe possedere i suoi
figli: se un nodo padre viene eliminato, anche i suoi nodi figli dovrebbero essere eliminati. Tuttavia, un figlio non dovrebbe possedere il suo padre: se eliminiamo un nodo figlio, il
genitore dovrebbe comunque esistere. Questo è un caso di weak references_!

Quindi, invece di `Rc<T>`, faremo in modo che il tipo di `padre` utilizzi `Weak<T>`,
in particolare `RefCell<Weak<Node>>`. Ora la definizione della struttura `Node` appare
così:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:here}}
```

Un nodo potrà fare riferimento al suo nodo padre, ma non ne sarà il proprietario.
Nel Listato 15-28, aggiorniamo `main` per utilizzare questa nuova definizione, in modo che il nodo `foglia`
abbia un modo per fare riferimento al suo padre, `ramo`.

<Listing number="15-28" file-name="src/main.rs" caption="Un nodo `foglia` con un weak reference al suo nodo padre, `ramo`">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-28/src/main.rs:there}}
```

</Listing>

La creazione del nodo `foglia` è simile a quella del Listato 15-27, ad eccezione
del campo `padre`: `foglia` inizia senza un padre, quindi creiamo una nuova
istanza vuota di riferimento `Weak<Node>`.

A questo punto, quando proviamo a ottenere un riferimento al padre di `foglia` utilizzando
il metodo `upgrade`, otteniamo il valore `None`. Lo vediamo nell'output della
prima istruzione `println!`:

```text
foglia padre = None
```

Quando creiamo il nodo `ramo`, avrà anche un nuovo riferimento `Weak<Node>`
nel campo `padre` perché `ramo` non ha un nodo padre.
Abbiamo ancora `foglia` come uno dei figli di `ramo`. Una volta che abbiamo l'istanza
`Node` in `ramo`, possiamo modificare `foglia` per assegnargli un riferimento `Weak<Node>`
al suo padre. Utilizziamo il metodo `borrow_mut` su
`RefCell<Weak<Node>>` nel campo `padre` di `foglia`, quindi utilizziamo la funzione
`Rc::downgrade` per creare un riferimento `Weak<Node>` a `ramo` da
`Rc<Node>` in `ramo`.

Quando stampiamo di nuovo il genitore di `foglia`, questa volta otterremo una variante `Some`
che contiene `ramo`: ora `foglia` può accedere al suo genitore! Quando stampiamo `foglia`,
evitiamo anche il ciclo che alla fine si è concluso con uno stack overflow come nel
Listato 15-26; i riferimenti `Weak<Node>` vengono stampati come `(Weak)`:

```testo
foglia padre = Some(Node { valore: 5, padre: RefCell { valore: (Weak) },
figlio: RefCell { valore: [Node { valore: 3, padre: RefCell { valore: (Weak) },
figlio: RefCell { valore: [] } }] } })
```

L'assenza di output infinito indica che questo codice non ha creato un ciclo di riferimento. Possiamo anche dedurne questo osservando i valori ottenuti chiamando
`Rc::strong_count` e `Rc::weak_count`.

#### Visualizzazione delle Modifiche a `strong_count` e `weak_count`

Osserviamo come i valori di `strong_count` e `weak_count` delle istanze di `Rc<Node>`
cambiano creando un nuovo ambito interno e spostando la creazione di
`ramo` in tale ambito. In questo modo, possiamo vedere cosa succede quando `ramo` viene
creato e poi eliminato quando esce dall'ambito. Le modifiche sono mostrate
nel Listato 15-29.

<Numero di inserzione="15-29" nome-file="src/main.rs" didascalia="Creazione di `ramo` in un ambito interno ed esame dei conteggi dei riferimenti forti e deboli">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-29/src/main.rs:here}}
```

</Listing>

Dopo la creazione di `foglia`, il suo `Rc<Node>` ha un conteggio forte di 1 e un conteggio debole
di 0. Nell'ambito interno, creiamo `ramo` e lo associamo a
`foglia`; a quel punto, quando stampiamo i conteggi, `Rc<Node>` in `ramo`
avrà un conteggio forte di 1 e un conteggio debole di 1 (per `foglia.padre` che punta
a `ramo` con un `Weak<Node>`). Quando stampiamo i conteggi in `foglia`, vedremo
che avrà un conteggio forte di 2 perché `ramo` ora ha un clone di
`Rc<Node>` di `foglia` memorizzato in `ramo.figlio`, ma avrà ancora un conteggio debole
di 0.

Quando l'ambito interno termina, `ramo` esce dall'ambito e il conteggio forte di
`Rc<Node>` scende a 0, quindi il suo `Node` viene eliminato. Il conteggio debole di 1
da `foglia.padre` non ha alcuna influenza sull'eliminazione o meno di `Node`, quindi
non si verificano perdite di memoria!

Se proviamo ad accedere al genitore di `foglia` dopo la fine dello scope, otterremo di nuovo
`None`. Alla fine del programma, `Rc<Node>` in `foglia` ha un conteggio forte
di 1 e un conteggio debole di 0 perché la variabile `foglia` è ora di nuovo l'unico
riferimento a `Rc<Node>`.

Tutta la logica che gestisce i conteggi e l'eliminazione dei valori è integrata in
`Rc<T>` e `Weak<T>` e nelle loro implementazioni del tratto `Drop`. Specificando
che la relazione tra un figlio e il suo padre debba essere un riferimento
`Weak<T>` nella definizione di `Node`, è possibile fare in modo che i nodi padre
puntino ai nodi figlio e viceversa senza creare un ciclo di riferimento
e perdite di memoria.

## Riepilogo

Questo capitolo ha spiegato come utilizzare i puntatori intelligenti per ottenere garanzie e
compromessi diversi da quelli che Rust applica di default con i riferimenti regolari. Il
tipo `Box<T>` ha una dimensione nota e punta ai dati allocati sull'heap. Il
tipo `Rc<T>` tiene traccia del numero di riferimenti ai dati sull'heap, in modo
che i dati possano avere più proprietari. Il tipo `RefCell<T>` con la sua
mutabilità interna ci fornisce un tipo che possiamo usare quando abbiamo bisogno di un tipo immutabile ma
devono modificare un valore interno di quel tipo; inoltre, applica le regole di prestito
in fase di esecuzione anziché in fase di compilazione.

Sono stati inoltre discussi i tratti `Deref` e `Drop`, che abilitano molte delle
funzionalità dei puntatori intelligenti. Abbiamo esplorato i cicli di riferimento che possono causare
perdite di memoria e come prevenirle utilizzando `Weak<T>`.

Se questo capitolo ha suscitato il vostro interesse e desiderate implementare i vostri
puntatori intelligenti, consultate [“The Rustonomicon”][nomicon] per ulteriori
informazioni utili.

In seguito, parleremo della concorrenza in Rust. Imparerete anche a conoscere alcuni nuovi
puntatori intelligenti.

[nomicon]: https://doc.rust-lang.org/stable/nomicon/index.html
