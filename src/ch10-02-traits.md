## _Trait_: Definizione del Comportamento Condiviso

Un _tratto_ (_trait_) definisce la funzionalità che un particolare _type_ ha e
può condividere con altri _type_. Possiamo usare i _trait_ per definire il
comportamento condiviso in modo astratto. Possiamo usare i _vincoli del tratto_
(_trait bound_) per specificare che un _type_ generico può essere qualsiasi
_type_ che abbia un determinato comportamento.

> Nota: i _trait_ sono simili a una funzionalità spesso chiamata _interfacce_
> (_interfaces_) in altri linguaggi, sebbene con alcune differenze.

### Definizione di un _Trait_

Il comportamento di un _type_ consiste nei metodi che possiamo chiamare su quel
_type_. _Type_ diversi condividono lo stesso comportamento se possiamo chiamare
gli stessi metodi su tutti quei _type_. Le definizioni dei _trait_ sono un modo
per raggruppare le firme dei metodi per definire un insieme di comportamenti
necessari per raggiungere un determinato scopo.

Ad esempio, supponiamo di avere più _struct_ che contengono vari tipi e quantità
di testo: una struttura `ArticoloNews` che contiene una notizia archiviata in
una posizione specifica e una `SocialPost` che può contenere, al massimo, 280
caratteri insieme a metadati che indicano se si tratta di un nuovo post, una
ripubblicazione o una risposta a un altro post.

Vogliamo creare una libreria di aggregazione multimediale denominata
`aggregatore` in grado di visualizzare riepiloghi dei dati che potrebbero essere
memorizzati in un'istanza di `ArticoloNews` o `SocialPost`. Per fare ciò,
abbiamo bisogno di un riepilogo per ciascun _type_ e richiederemo tale riepilogo
chiamando un metodo `riassunto` su un'istanza. Il Listato 10-12 mostra la
definizione di un _trait_ pubblico `Sommario` che esprime questo comportamento.

<Listing number="10-12" file-name="src/lib.rs" caption="Un _trait_ `Sommario` che consiste nel comportamento fornito da un metodo `riassunto`">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

</Listing>

Qui, dichiariamo un _trait_ usando la parola chiave `trait` e poi il nome del
_trait_, che in questo caso è `Sommario`. Dichiariamo anche il _trait_ come
`pub` in modo che anche i _crate_ che dipendono da questo _crate_ possano
utilizzare questo _trait_, come vedremo in alcuni esempi. All'interno delle
parentesi graffe, dichiariamo le firme dei metodi che descrivono i comportamenti
dei _type_ che implementano questo _trait_, che in questo caso è `fn
riassunto(&self) -> String`.

Dopo la firma del metodo, invece di fornire un'implementazione tra parentesi
graffe, utilizziamo un punto e virgola. Ogni _type_ che implementa questo
_trait_ deve fornire il proprio comportamento personalizzato per il corpo del
metodo. Il compilatore imporrà che qualsiasi _type_ che abbia il _trait_
`Sommario` abbia il metodo `riassunto` definito esattamente con questa firma.

Una _trait_ può avere più metodi nel suo corpo: le firme dei metodi sono
elencate una per riga e ogni riga termina con un punto e virgola.

### Implementazione di un _Trait_ su un _Type_

Ora che abbiamo definito le firme desiderate dei metodi del _trait_ `Sommario`,
possiamo implementarlo sui _type_ nel nostro aggregatore multimediale. Il
Listato 10-13 mostra un'implementazione del _trait_ `Sommario` sulla _struct_
`ArticoloNews` che utilizza il titolo, l'autore e la posizione per creare il
valore di ritorno di `riassunto`. Per la _struct_ `SocialPost`, definiamo
`riassunto` come il nome utente seguito dall'intero testo del post, supponendo
che il contenuto del post sia già limitato a 280 caratteri.

<Listing number="10-13" file-name="src/lib.rs" caption="Implementazione del _trait_ `Sommario` sui _type_ `ArticoloNews` e `SocialPost`">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

</Listing>

Implementare un _trait_ su un _type_ è simile a come normalmente sono
implementati i metodi. La differenza è che dopo `impl`, inseriamo il nome del
_trait_ che vogliamo implementare, poi utilizziamo la parola chiave `for` e
infine specifichiamo il nome del _type_ per cui vogliamo implementare il
_trait_. All'interno del blocco `impl`, inseriamo le firme dei metodi definite
dalla definizione del _trait_. Invece di aggiungere un punto e virgola dopo ogni
firma, utilizziamo le parentesi graffe e riempiamo il corpo del metodo con il
comportamento specifico che vogliamo che i metodi del _trait_ abbiano per quel
particolare _type_.

Ora che la libreria ha implementato il _trait_ `Sommario` su `ArticoloNews` e
`SocialPost`, gli utenti del _crate_ possono chiamare i metodi del _trait_ sulle
istanze di `ArticoloNews` e `SocialPost` nello stesso modo in cui chiamiamo i
metodi normali. L'unica differenza è che l'utente deve includere il _trait_
nello _scope_ oltre ai _type_. Ecco un esempio di come un _crate_ binario
potrebbe utilizzare il nostro _crate_ libreria `aggregatore`:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs}}
```

Questo codice stampa `1 nuovo post: horse_ebooks: ovviamente, come probabilmente
già sapete, gente`.

Anche altri _crate_ che dipendono dal _crate_ `aggregatore` possono includere il
_trait_ `Sommario` nello _scope_ per implementare `Sommario` sui propri _type_.
Una restrizione da notare è che possiamo implementare un _trait_ su un _type_
solo se il _trait_ o il _type_, o entrambi, sono locali al nostro _crate_. Ad
esempio, possiamo implementare _trait_ della libreria standard come `Display` su
un _type_ personalizzato come `SocialPost` come parte della funzionalità del
nostro _crate_ `aggregatore`, perché il _type_ `SocialPost` è locale al nostro
_crate_ `aggregatore`. Possiamo anche implementare `Sommario` su `Vec<T>` nel
nostro _crate_ `aggregatore`, perché il _trait_ `Sommario` è locale al nostro
_crate_ `aggregatore`.

Ma non possiamo implementare _trait_ esterni su _type_ esterni. Ad esempio, non
possiamo implementare il _trait_ `Display` su `Vec<T>` all'interno del nostro
_crate_ `aggregatore` perché `Display` e `Vec<T>` sono entrambi definiti nella
libreria standard e non sono locali al nostro _crate_ `aggregatore`. Questa
restrizione fa parte di una proprietà chiamata _coerenza_ (_coherence_), e più
specificamente della _regola dell'orfano_ (_orphan rule _), così chiamata perché
il _type_ genitore non è presente. Questa regola garantisce che il codice di
altri non possa _rompere_ il tuo codice e viceversa. Senza questa regola, due
_crate_ potrebbero implementare lo stesso _trait_ per lo stesso _type_ e Rust
non saprebbe quale implementazione utilizzare.

### Implementazioni Predefinite

A volte è utile avere un comportamento predefinito per alcuni o tutti i metodi
in un _trait_ invece di richiedere implementazioni per tutti i metodi su ogni
_type_. Quindi, quando implementiamo il _trait_ su un _type_ particolare,
possiamo mantenere o sovrascrivere il comportamento predefinito di ciascun
metodo.

Nel Listato 10-14, specifichiamo una stringa predefinita per il metodo
`riassunto` del _trait_ `Sommario` invece di definire solo la firma del metodo,
come abbiamo fatto nel Listato 10-12.

<Listing number="10-14" file-name="src/lib.rs" caption="Definizione di un _trait_ `Sommario` con un'implementazione predefinita del metodo `riassunto`  ">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

</Listing>

Per utilizzare un'implementazione predefinita per riassumere le istanze di
`ArticoloNews`, specifichiamo un blocco `impl` vuoto con `impl Sommario for
ArticoloNews {}`.

Anche se non definiamo più il metodo `riassunto` su `ArticoloNews` direttamente,
abbiamo fornito un'implementazione predefinita e specificato che `ArticoloNews`
implementa il _trait_ `Sommario`. Di conseguenza, possiamo comunque chiamare il
metodo `riassunto` su un'istanza di `ArticoloNews`, in questo modo:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```

Questo codice stampa `Nuovo articolo disponibile! (Leggi di più...)`.

La creazione di un'implementazione predefinita non richiede alcuna modifica
all'implementazione di `Sommario` su `SocialPost` nel Listato 10-13. Il motivo è
che la sintassi per sovrascrivere un'implementazione predefinita è la stessa
della sintassi per implementare un metodo di un _trait_ che non ha
un'implementazione predefinita.

Le implementazioni predefinite possono chiamare altri metodi nello stesso
_trait_, anche se questi non hanno un'implementazione predefinita. In questo
modo, un _trait_ può fornire molte funzionalità utili e richiedere agli
implementatori di specificarne solo una piccola parte. Ad esempio, potremmo
definire il _trait_ `Sommario` in modo che abbia un metodo `riassunto_autore` la
cui implementazione è richiesta, e quindi definire un metodo `riassunto` con
un'implementazione predefinita che chiama il metodo `riassunto_autore`:

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

Per utilizzare questa versione di `Sommario`, dobbiamo definire
`riassunto_autore` solo quando implementiamo il _trait_ su un _type_:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

Dopo aver definito `riassunto_autore`, possiamo chiamare `riassunto` sulle
istanze della _struct_ `SocialPost` e l'implementazione predefinita di
`riassunto` chiamerà la definizione di `riassunto_autore` che abbiamo fornito.
Poiché abbiamo implementato `riassunto_autore`, il _trait_ `Sommario` ci ha
fornito il comportamento del metodo `riassunto` senza richiedere ulteriore
codice. Ecco come appare:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```

Questo codice stampa `1 nuovo post: (Leggi di più su @horse_ebooks...)`.

Nota che non è possibile chiamare l'implementazione predefinita da una
implementazione sovrascritta dello stesso metodo.

### _Trait_ come Parametri

Ora che sai come definire e implementare i _trait_, possiamo esplorare come
usarli per definire funzioni che accettano molti _type_ diversi. Useremo il
_trait_ `Sommario` che abbiamo implementato sui _type_ `ArticoloNews` e
`SocialPost` nel Listato 10-13 per definire una funzione `notifica` che chiama
il metodo `riassunto` sul suo parametro `elemento`, che è di un _type_ che
implementa il _trait_ `Sommario`. Per fare ciò, utilizziamo la sintassi `impl
Trait`, in questo modo:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

Invece di un _type_ concreto per il parametro `elemento`, specifichiamo la
parola chiave `impl` e il nome del _trait_. Questo parametro accetta qualsiasi
_type_ che implementi il _trait_ specificato. Nel corpo di `notifica`, possiamo
chiamare qualsiasi metodo su `elemento` che provenga dal _trait_ `Sommario`,
come `riassunto`. Possiamo chiamare `notifica` e passare qualsiasi istanza di
`ArticoloNews` o `SocialPost`. Il codice che chiama la funzione con qualsiasi
altro _type_, come `String` o `i32`, non verrà compilato perché questi _type_
non implementano `Sommario`.

<!-- Old headings. Do not remove or links may break. -->
<a id="fixing-the-largest-function-with-trait-bounds"></a>

#### Sintassi dei Vincoli di _trait_

La sintassi `impl Trait` funziona per i casi più semplici, ma in realtà è solo
una sintassi semplificata per una forma più lunga nota come _vincoli del tratto_
(_trait bound_); si presenta così:

```rust,ignore
pub fn notifica<T: Sommario>(elemento: &T) {
    println!("Ultime notizie! {}", elemento.riassunto());
}
```

Questa forma più lunga è equivalente all'esempio della sezione precedente, ma è
più dettagliata. Posizioniamo i _trait bound_ con la dichiarazione del parametro
di _type_ generico dopo i due punti e tra parentesi angolari.

La sintassi `impl Trait` è comoda e consente di scrivere codice più conciso nei casi
semplici, mentre la sintassi più completa dei _trait bound_ può esprimere una maggiore complessità in altri
casi. Ad esempio, possiamo avere due parametri che implementano `Sommario`. Con la sintassi `impl Trait`, ciò si ottiene in questo modo:

```rust,ignore
pub fn notifica(elemento1: &impl Sommario, elemento2: &impl Sommario) {
```

L'utilizzo di `impl Trait` è appropriato se vogliamo che questa funzione
consenta a `elemento1` e `elemento2` di avere _type_ diversi (purché entrambi i
_type_ implementino `Sommario`). Tuttavia, se vogliamo forzare entrambi i
parametri ad avere lo stesso _type_, dobbiamo usare un _trait bound_, come
questo:

```rust,ignore
pub fn notifica<T: Sommario>(elemento1: &T, elemento2: &T) {
```

Il _type_ generico `T` specificato come _type_ dei parametri `elemento1` e
`elemento2` vincola la funzione in modo che il _type_ concreto del valore
passato come argomento per `elemento1` e `elemento2` debba essere lo stesso.

#### Specificare più _Trait Bound_ con la sintassi `+`

Possiamo anche specificare più di un _trait bound_. Supponiamo di voler usare
`notifica` sia la formattazione di visualizzazione sia per `riassunto` su
`elemento`: specifichiamo nella definizione di `notifica` che `elemento` deve
implementare sia `Display` che `Sommario`. Possiamo farlo utilizzando la
sintassi `+`:

```rust,ignore
pub fn notifica(elemento: &(impl Sommario + Display)) {
```

La sintassi `+` è valida anche con i _trait bound_ sui _type_ generici:

```rust,ignore
pub fn notifica<T: Sommario + Display>(elemento: &T) {
```

Con i due _trait bound_ specificati, il corpo di `notifica` può chiamare
`riassunto`  
e utilizzare `{}` per formattare `elemento`.

#### Specificare i _Trait Bound_ con le Clausole `where`

L'utilizzo di troppi _trait bound_ ha i suoi svantaggi. Ogni generico ha i suoi
_trait bound_, quindi le funzioni con più parametri di _type_ generico possono
contenere molte informazioni sui _trait bound_ tra il nome della funzione e il
suo elenco di parametri, rendendo la firma della funzione difficile da leggere.
Per questo motivo, Rust ha una sintassi alternativa per specificare i _trait
bound_ all'interno di una clausola `where` dopo la firma della funzione. Quindi,
invece di scrivere:

```rust,ignore
fn una_funzione<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

possiamo usare una clausola `where`, in questo modo:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-where-clause/src/lib.rs:here}}
```

La firma di questa funzione è meno confusionaria: il nome della funzione,
l'elenco dei parametri e il _type_ di ritorno sono vicini, come in una funzione
senza molti _trait bound_.

### Restituzione di _Type_ che Implementano _Trait_

Possiamo anche usare la sintassi `impl Trait` nella posizione di ritorno per
restituire un valore di un _type_ che implementa un _trait_, come mostrato qui:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

Utilizzando `impl Sommario` come _type_ di ritorno, specifichiamo che la
funzione `riassumibile` restituisce un _type_ che implementa il _trait_
`Sommario` senza nominare il _type_ concreto. In questo caso, `riassumibile`
restituisce un `SocialPost`, ma il codice che chiama questa funzione non ha
bisogno di saperlo.

La possibilità di specificare un _type_ di ritorno solo tramite il _trait_ che
implementa è particolarmente utile nel contesto di _chiusure_ (_closure_) e
_iteratori_, che tratteremo nel Capitolo 13. Chiusure e iteratori creano _type_
che solo il compilatore conosce o _type_ che sono molto lunghi da specificare.
La sintassi `impl Trait` consente di specificare in modo conciso che una
funzione restituisca un _type_ che implementa il _trait_ `Iterator` senza dover
scrivere un _type_ molto lungo.

Tuttavia, è possibile utilizzare `impl Trait` solo se si restituisce un singolo
_type_. Ad esempio, questo codice che restituisce un `ArticoloNews` o un
`SocialPost` con il _type_ di ritorno specificato come `impl Sommario` non
funzionerebbe:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```

Restituire un `ArticoloNews` o un `SocialPost` non è consentito a causa di
restrizioni relative all'implementazione della sintassi `impl Trait` nel
compilatore. Spiegheremo come scrivere una funzione con questo comportamento
nella sezione sugli [“Oggetti _Trait_”][using-trait-objects]<!-- ignore --> del
Capitolo 18.

### Utilizzo di Vincoli di _trait_ per Implementare Metodi in Modo Condizionale

Utilizzando un _trait bound_ con un blocco `impl` che utilizza parametri di
_type_ generico, possiamo implementare metodi in modo condizionale per i _type_
che implementano i _trait_ specificati. Ad esempio, il _type_ `Coppia<T>` nel
Listato 10-15 implementa sempre la funzione `new` per restituire una nuova
istanza di `Coppia<T>` (ricorda dalla sezione [“Definire i Metodi”][methods]<!--
ignore --> del Capitolo 5 che `Self` è un alias di _type_ per il _type_ del
blocco `impl`, che in questo caso è `Coppia<T>`). Ma nel blocco `impl`
successivo, `Coppia<T>` implementa il metodo `mostra_comparazione` solo se il
suo _type_ interno `T` implementa il _trait_ `PartialOrd` che abilita il
confronto _e_ il _trait_ `Display` che abilita la stampa.

<Listing number="10-15" file-name="src/lib.rs" caption="Implementazione condizionale di metodi su un _type_ generico in base ai vincoli del _trait_">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/lib.rs}}
```

</Listing>

Possiamo anche implementare in modo condizionale un _trait_ per qualsiasi _type_
che implementa un altro _trait_. Le implementazioni di un _trait_ su qualsiasi
_type_ che soddisfi i _trait bound_ sono chiamate _implementazioni generali_
(_blanket implementations_) e sono ampiamente utilizzate nella libreria standard
di Rust. Ad esempio, la libreria standard implementa il _trait_ `ToString` su
qualsiasi _type_ che implementi il _trait_ `Display`. Il blocco `impl` nella
libreria standard è simile a questo codice:

```rust,ignore
impl<T: Display> ToString for T {
    // --taglio--
}
```

Poiché la libreria standard ha questa implementazione generale, possiamo
chiamare il metodo `to_string` definito dal _trait_ `ToString` su qualsiasi
_type_ che implementi il _trait_ `Display`. Ad esempio, possiamo trasformare gli
_integer_ nei loro corrispondenti valori `String` in questo modo, perché gli
_integer_ implementano `Display`:

```rust
let s = 3.to_string();
```

Le implementazioni generali compaiono nella documentazione per il _trait_ in
questione nella sezione "Implementatori" (_Implementors_).

I _trait_ e i vincoli dei _trait_ ci consentono di scrivere codice che utilizza
parametri di _type_ generico per ridurre le duplicazioni, ma anche di
specificare al compilatore che desideriamo che il _type_ generico abbia un
comportamento particolare. Il compilatore può quindi utilizzare le informazioni
sui vincoli dei _trait_ per verificare che tutti i _type_ concreti utilizzati
nel nostro codice forniscano il comportamento corretto. Nei linguaggi a
tipizzazione dinamica, otterremmo un errore durante l'esecuzione se chiamassimo
un metodo su un _type_ che non lo definisce. Ma Rust sposta questi errori in
fase di compilazione, quindi siamo costretti a correggere i problemi prima
ancora che il nostro codice possa essere eseguito. Inoltre, non dobbiamo
scrivere codice che verifichi il comportamento a runtime, perché lo abbiamo già
verificato in fase di compilazione. Ciò migliora le prestazioni senza dover
rinunciare alla flessibilità dei _type_ generici.

[using-trait-objects]: ch18-02-trait-objects.html
[methods]: ch05-03-method-syntax.html#definire-i-metodi