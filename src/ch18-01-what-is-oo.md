## Caratteristiche dei Linguaggi Orientati agli Oggetti

Non c’è consenso nella comunità di programmatori su quali caratteristiche un
linguaggio deve avere per essere considerato orientato agli oggetti. Rust è
influenzato da molti paradigmi di programmazione, inclusa la _OOP_; per esempio,
abbiamo esplorato le caratteristiche provenienti dalla programmazione funzionale
nel Capitolo 13. Si può dire che i linguaggi _OOP_ condividano certe
caratteristiche comuni, come oggetti, incapsulamento ed ereditarietà. Vediamo
cosa significa ognuna di queste caratteristiche e se Rust le supporta.

### Gli Oggetti Contengono Dati e Comportamenti

Il libro _Design Patterns: Elements of Reusable Object-Oriented Software_ di
Erich Gamma, Richard Helm, Ralph Johnson e John Vlissides (Addison-Wesley, 1994)
(traduzione in italiano pubblicata da Pearson, 2002), noto come il libro della
_Gang of Four_ (_Banda dei quattro_), è un catalogo di modelli di progettazione
orientati agli oggetti. Definisce la _OOP_ in questo modo:

> I programmi orientati agli oggetti sono composti da oggetti. Un **oggetto**
> raggruppa sia i dati sia le procedure che operano su quei dati. Le procedure
> sono tipicamente chiamate **metodi** o **operazioni**.

Secondo questa definizione, Rust è orientato agli oggetti: `struct` ed `enum`
contengono dati, e i blocchi `impl` forniscono metodi su `struct` ed `enum`.
Anche se `struct` ed `enum` con metodi non sono _chiamati_ oggetti, forniscono
la stessa funzionalità, secondo la definizione della _Gang of Four_.

### Incapsulamento che Nasconde i Dettagli di Implementazione

Un altro aspetto comunemente associato alla _OOP_ è l’idea di _incapsulamento_,
che significa che i dettagli di implementazione di un oggetto non sono
accessibili al codice che usa quell’oggetto. Quindi, l’unico modo per interagire
con un oggetto è attraverso la sua API pubblica; il codice che usa l’oggetto non
dovrebbe poter modificare direttamente i dati o il comportamento interni. Questo
permette al programmatore di cambiare e ristrutturare l’implementazione interna
di un oggetto senza dover modificare il codice che usa l’oggetto.

Abbiamo visto come controllare l’incapsulamento nel Capitolo 7: possiamo usare
la parola chiave `pub` per decidere quali moduli, _type_, funzioni e metodi del
nostro codice devono essere pubblici e, di default, tutto il resto è privato.
Per esempio, possiamo definire una _struct_ `CollezioneConMedia` che ha un campo
contenente un vettore di valori `i32`. La _struct_ può avere anche un campo che
contiene la media dei valori nel vettore, quindi la media non deve essere
calcolata ogni volta che serve. In altre parole, `CollezioneConMedia` tiene
memorizzata la media calcolata di volta in volta al posto nostro. Il Listato
18-1 mostra la definizione della _struct_ `CollezioneConMedia`.

<Listing number="18-1" file-name="src/lib.rs" caption="Una _struct_ `CollezioneConMedia` che mantiene una lista di interi e la media degli elementi nella collezione">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-01/src/lib.rs}}
```

</Listing>

La _struct_ è marcata `pub` così il resto del codice può usarla, ma i campi
interni restano privati. Questo è importante perché vogliamo assicurarci che
ogni volta che un valore viene aggiunto o rimosso dalla lista, anche la media
venga aggiornata. Lo facciamo implementando i metodi `aggiungi`, `rimuovi` e
`media` sulla _struct_, come nel Listato 18-2.

<Listing number="18-2" file-name="src/lib.rs" caption="Implementazioni dei metodi pubblici `aggiungi`, `rimuovi` e `media` su `CollezioneConMedia`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-02/src/lib.rs:here}}
```

</Listing>

I metodi pubblici `aggiungi`, `rimuovi` e `media` sono gli unici modi con cui
accedere o modificare i dati in un’istanza di `CollezioneConMedia`. Quando si
aggiunge un elemento alla `lista` usando `aggiungi` o si rimuove con `rimuovi`,
questi metodi chiamano il metodo privato `aggiorna_media` che si occupa di
aggiornare il campo `media`.

Lasciamo i campi `lista` e `media` privati, in questo modo il codice esterno non
può aggiungere o rimuovere elementi direttamente dalla lista; altrimenti, il
campo `media` potrebbe diventare non più corrispondente a quello che rappresenta
rispetto alla lista. Il metodo `media` restituisce il valore nel campo `media`,
permettendo al codice esterno di leggere la media ma non di modificarla.

Poiché abbiamo incapsulato i dettagli dell’implementazione della _struct_
`CollezioneConMedia`, possiamo cambiare facilmente aspetti come la struttura
dati in futuro. Per esempio, potremmo usare un `HashSet<i32>` al posto di un
`Vec<i32>` per il campo `lista`. Finché le firme dei metodi pubblici `aggiungi`,
`rimuovi` e `media` rimangono uguali, il codice esterno che usa
`CollezioneConMedia` non deve cambiare. Se invece avessimo reso `lista`
pubblico, non sarebbe così: `HashSet<i32>` e `Vec<i32>` hanno metodi diversi per
aggiungere e rimuovere elementi, quindi il codice esterno dovrebbe cambiare se
modificasse la lista direttamente.

Se l’incapsulamento è un requisito necessario per considerare un linguaggio
orientato agli oggetti, allora Rust lo soddisfa. La possibilità di usare o meno
`pub` per parti diverse del codice abilita l’incapsulamento dei dettagli di
implementazione.

### Ereditarietà come Sistema dei _Type_ e come Condivisione di Codice

_L’ereditarietà_ è un meccanismo per cui un oggetto può ereditare elementi dalla
definizione di un altro oggetto, ottenendo i dati e i comportamenti dell’oggetto
genitore senza doverli ridefinire.

Se un linguaggio deve avere l’ereditarietà per essere orientato agli oggetti,
allora Rust non lo è. Non c’è modo di definire una _struct_ che erediti i campi
e i metodi del genitore senza usare una macro.

Tuttavia, se sei abituato a usare l’ereditarietà, puoi usare in Rust altre
soluzioni a seconda del motivo per cui la vorresti usare.

Le due ragioni principali per scegliere l’ereditarietà sono: il riuso del codice
e il sistema dei _type_.

Per il riuso del codice, puoi implementare un comportamento particolare per un
_type_ e l’ereditarietà ti consente di riusarlo per un altro _type_. In Rust
puoi farlo in modo limitato con le implementazioni dei metodi default dei
_trait_, come nel Listato 10-14 quando abbiamo dato una implementazione di
default al metodo `riassunto` sul _trait_ `Sommario`. Qualsiasi _type_ che
implementa `Sommario` avrà il metodo `riassunto` senza dover scrivere ulteriore
codice. Questo è simile a una classe genitore che ha un’implementazione di un
metodo e una classe figlia che eredita quella implementazione. Possiamo anche
sovrascrivere l’implementazione di default di `riassunto` quando implementiamo
il _trait_ `Sommario`, simile a una classe figlia che modifica un metodo
ereditato.

L’altra ragione per usare l’ereditarietà riguarda il sistema dei _type_:
permettere a un _type_ figlio di essere usato nei posti in cui si usa il _type_
genitore. Questo si chiama anche _polimorfismo_, che significa poter sostituire
oggetti diversi durante l’esecuzione se hanno certe caratteristiche in comune.

> ### Polimorfismo
>
> Per molti, polimorfismo è sinonimo di ereditarietà. Ma in realtà è un concetto
> più generale che si riferisce a codice in grado di lavorare con dati di tipi
> diversi. Per l’ereditarietà, questi tipi sono solitamente sottoclassi.
>
> Rust invece usa i generici per astrarre su _type_ diversi e i vincoli di
> _trait_ per imporre cosa questi _type_ devono fornire. Questo viene
> solitamente chiamato _polimorfismo parametrico vincolato_.

Rust ha scelto un set di compromessi diverso non offrendo ereditarietà.
L’ereditarietà spesso condivide più codice del necessario. Le sottoclassi non
dovrebbero sempre condividere tutte le caratteristiche della classe genitore, ma
con l’ereditarietà lo fanno, il che può rendere il design del programma meno
flessibile. Può anche introdurre la possibilità di chiamare metodi su
sottoclassi che non hanno senso o causano errori perché quei metodi non si
applicano. Inoltre, alcuni linguaggi permettono solo _l’ereditarietà singola_
(cioè una sottoclasse può ereditare da una sola classe genitore), limitando
ulteriormente la flessibilità nel design.

Per questi motivi, Rust usa un approccio diverso basato sugli oggetti _trait_
invece dell’ereditarietà per ottenere il polimorfismo durante l’esecuzione.
Vediamo come funzionano gli oggetti _trait_.
