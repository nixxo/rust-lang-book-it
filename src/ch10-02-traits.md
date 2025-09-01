## Tratti (Traits): Definizione del Comportamento Condiviso

Un _tratto_ definisce la funzionalità di un particolare tipo e la sua condivisione con
altri tipi. Possiamo usare i tratti per definire il comportamento condiviso in modo astratto. Possiamo
usare i _limiti del tratto_ per specificare che un tipo generico può essere qualsiasi tipo che abbia
un determinato comportamento.

> Nota: i tratti sono simili a una funzionalità spesso chiamata _interfacce_ in altri
> linguaggi, sebbene con alcune differenze.

### Definizione di un Tratto

Il comportamento di un tipo consiste nei metodi che possiamo chiamare su quel tipo. Tipi diversi
condividono lo stesso comportamento se possiamo chiamare gli stessi metodi su tutti quei
tipi. Le definizioni dei tratti sono un modo per raggruppare le firme dei metodi per
definire un insieme di comportamenti necessari per raggiungere un determinato scopo.

Ad esempio, supponiamo di avere più strutture che contengono vari tipi e
quantità di testo: una struttura `ArticoloNews` che contiene una notizia archiviata in una
posizione specifica e una `SocialPost` che può contenere, al massimo, 280 caratteri
insieme a metadati che indicano se si tratta di un nuovo post, una ripubblicazione o una
risposta a un altro post.

Vogliamo creare una libreria di aggregazione multimediale denominata `aggregatore` in grado di
visualizzare riepiloghi dei dati che potrebbero essere memorizzati in un'istanza di `ArticoloNews` o
`SocialPost`. Per fare ciò, abbiamo bisogno di un riepilogo per ciascun tipo e
richiederemo tale riepilogo chiamando un metodo `riassunto` su un'istanza. Il Listato
10-12 mostra la definizione di un tratto pubblico `Sommario` che esprime questo
comportamento.

<Listing number="10-12" file-name="src/lib.rs" caption="Un tratto `Sommario` che consiste nel comportamento fornito da un metodo `riassunto`">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

</Listing>

Qui, dichiariamo un tratto usando la parola chiave `trait` e poi il nome del tratto,
che in questo caso è `Sommario`. Dichiariamo anche il tratto come `pub` in modo che
anche i crates che dipendono da questo crate possano utilizzare questo tratto, come vedremo
in alcuni esempi. All'interno delle parentesi graffe, dichiariamo le firme dei metodi
che descrivono i comportamenti dei tipi che implementano questa caratteristica, che in
questo caso è `fn sommario(&self) -> String`.

Dopo la firma del metodo, invece di fornire un'implementazione tra parentesi graffe, utilizziamo un punto e virgola. Ogni tipo che implementa questa caratteristica deve fornire
il proprio comportamento personalizzato per il corpo del metodo. Il compilatore imporrà
che qualsiasi tipo che abbia la caratteristica `Sommario` abbia il metodo `riassunto` definito esattamente con questa firma.

Una caratteristica può avere più metodi nel suo corpo: le firme dei metodi sono elencate
una per riga e ogni riga termina con un punto e virgola.

### Implementazione di un Tratto su un Tipo

Ora che abbiamo definito le firme desiderate dei metodi della caratteristica `Sommario`,
possiamo implementarla sui tipi nel nostro aggregatoree multimediale. Il Listato 10-13 mostra
un'implementazione del tratto `Sommario` sulla struttura `ArticoloNews` che utilizza
il titolo, l'autore e la posizione per creare il valore di ritorno di
`riassunto`. Per la struttura `SocialPost`, definiamo `riassunto` come il nome utente
seguito dall'intero testo del post, supponendo che il contenuto del post sia
già limitato a 280 caratteri.

<Listing number="10-13" file-name="src/lib.rs" caption="Implementazione del tratto `Sommario` sui tipi `ArticoloNews` e `SocialPost`">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

</Listing>

Implementare un tratto su un tipo è simile all'implementazione di metodi normali.
La differenza è che dopo `impl`, inseriamo il nome del tratto che vogliamo implementare,
quindi utilizziamo la parola chiave `for` e infine specifichiamo il nome del tipo per cui vogliamo
implementare il tratto. All'interno del blocco `impl`, inseriamo le firme dei metodi
definite dalla definizione del tratto. Invece di aggiungere un punto e virgola dopo ogni
firma, utilizziamo le parentesi graffe e riempiamo il corpo del metodo con il comportamento
specifico che vogliamo che i metodi del tratto abbiano per quel particolare tipo.

Ora che la libreria ha implementato il tratto `Sommario` su `ArticoloNews` e
`SocialPost`, gli utenti del crate possono chiamare i metodi del tratto sulle istanze di
`ArticoloNews` e `SocialPost` nello stesso modo in cui chiamiamo i metodi normali. L'unica
differenza è che l'utente deve includere il tratto nell'ambito oltre ai
tipi. Ecco un esempio di come un crate binario potrebbe utilizzare il nostro crate di libreria `aggregatore`:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs}}
```

Questo codice stampa `1 nuovo post: horse_ebooks: ovviamente, come probabilmente già
sapete, gente`.

Anche altri crate che dipendono dal crate `aggregatore` possono includere il tratto `Sommario`
nell'ambito per implementare `Sommario` sui propri tipi. Una restrizione da
notare è che possiamo implementare un tratto su un tipo solo se il tratto o il
tipo, o entrambi, sono locali al nostro crate. Ad esempio, possiamo implementare tratti di libreria
standard come `Display` su un tipo personalizzato come `SocialPost` come parte della funzionalità del nostro
crate `aggregatore`, perché il tipo `SocialPost` è locale al nostro
crate `aggregatore`. Possiamo anche implementare `Sommario` su `Vec<T>` nel nostro
crate `aggregatore`, perché il tratto `Sommario` è locale al nostro
crate `aggregatore`.

Ma non possiamo implementare tratti esterni su tipi esterni. Ad esempio, non possiamo
implementare il tratto `Display` su `Vec<T>` all'interno del nostro crate `aggregatore` perché
`Display` e `Vec<T>` sono entrambi definiti nella libreria standard e non sono
locali al nostro crate `aggregatore`. Questa restrizione fa parte di una proprietà chiamata
_coherence_, e più specificamente della _orphan rule_, così chiamata perché il
tipo padre non è presente. Questa regola garantisce che il codice di altri non possa
interrompere il nostro codice e viceversa. Senza questa regola, due crate potrebbero implementare
lo stesso tratto per lo stesso tipo e Rust non saprebbe quale implementazione
utilizzare.

### Implementazioni Predefinite

A volte è utile avere un comportamento predefinito per alcuni o tutti i metodi
in un tratto invece di richiedere implementazioni per tutti i metodi su ogni tipo.
Quindi, quando implementiamo il tratto su un tipo particolare, possiamo mantenere o sovrascrivere
il comportamento predefinito di ciascun metodo.

Nel Listato 10-14, specifichiamo una stringa predefinita per il metodo `riassunto` del
tratto `Sommario` invece di definire solo la firma del metodo, come abbiamo fatto nel
Listato 10-12.

<Listing number="10-14" file-name="src/lib.rs" caption="Definizione di un tratto `Sommario` con un'implementazione predefinita del metodo `riassunto`  ">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

</Listing>

Per utilizzare un'implementazione predefinita per `riassunto`re le istanze di `ArticoloNews`,
specifichiamo un blocco `impl` vuoto con `impl Sommario for ArticoloNews {}`.

Anche se non definiamo più il metodo `riassunto` su `ArticoloNews`
direttamente, abbiamo fornito un'implementazione predefinita e specificato che
`ArticoloNews` implementa il tratto `Sommario`. Di conseguenza, possiamo comunque chiamare
il metodo `riassunto` su un'istanza di `ArticoloNews`, in questo modo:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```

Questo codice stampa `Nuovo articolo disponibile! (Leggi di più...)`.

La creazione di un'implementazione predefinita non richiede alcuna modifica
all'implementazione di `Sommario` su `SocialPost` nel Listato 10-13. Il motivo è che
la sintassi per sovrascrivere un'implementazione predefinita è la stessa
della sintassi per implementare un metodo di un tratto che non ha un'implementazione predefinita.

Le implementazioni predefinite possono chiamare altri metodi nello stesso tratto, anche se questi
non hanno un'implementazione predefinita. In questo modo, un tratto può
fornire molte funzionalità utili e richiedere agli implementatori di specificarne solo
una piccola parte. Ad esempio, potremmo definire il tratto `Sommario` in modo che abbia un
metodo ``riassunto_autore` la cui implementazione è richiesta, e quindi definire un
metodo `riassunto` con un'implementazione predefinita che chiama il
metodo `riassunto_autore`:

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

Per utilizzare questa versione di `Sommario`, dobbiamo definire `riassunto_autore` solo
quando implementiamo il tratto su un tipo:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

Dopo aver definito `riassunto_autore`, possiamo chiamare `riassunto` sulle istanze della struttura
`SocialPost` e l'implementazione predefinita di `riassunto` chiamerà la
definizione di `riassunto_autore` che abbiamo fornito. Poiché abbiamo implementato
`riassunto_autore`, il tratto `Sommario` ci ha fornito il comportamento del
metodo `riassunto` senza richiedere ulteriore codice. Ecco come
appare:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```

Questo codice stampa `1 nuovo post: (Leggi di più su @horse_ebooks...)`.

Si noti che non è possibile chiamare l'implementazione predefinita da un'
implementazione sovrascritta dello stesso metodo.

### Tratti come Parametri

Ora che sai come definire e implementare i tratti, possiamo esplorare come usarli
per definire funzioni che accettano molti tipi diversi. Useremo il
tratto `Sommario` che abbiamo implementato sui tipi `ArticoloNews` e `SocialPost` nel
Listato 10-13 per definire una funzione `notifica` che chiama il metodo `riassunto` sul suo parametro `elemento`, che è di un tipo che implementa il
tratto `Sommario`. Per fare ciò, utilizziamo la sintassi `impl Trait`, in questo modo:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

Invece di un tipo concreto per il parametro `elemento`, specifichiamo la parola chiave `impl`
e il nome del tratto. Questo parametro accetta qualsiasi tipo che implementi il
tratto specificato. Nel corpo di `notifica`, possiamo chiamare qualsiasi metodo su `elemento`
che provenga dal tratto `Sommario`, come `riassunto`. Possiamo chiamare `notifica`
e passare qualsiasi istanza di `ArticoloNews` o `SocialPost`. Il codice che chiama la
funzione con qualsiasi altro tipo, come `String` o `i32`, non verrà compilato
perché questi tipi non implementano `Sommario`.

<!-- Vecchie intestazioni. Non rimuovere o i link potrebbero non funzionare. -->

<a id="fixing-the-largest-function-with-trait-bounds"></a>

#### Sintassi dei limiti di tratto

La sintassi `impl Trait` funziona per i casi più semplici, ma in realtà è solo una sintassi semplificata per una forma più lunga nota come _limite di tratto_; si presenta così:

```rust,ignore
pub fn notifica<T: Sommario>(elemento: &T) {
println!("Ultime notizie! {}", elemento.riassunto());
}
```

Questa forma più lunga è equivalente all'esempio della sezione precedente, ma è
più dettagliata. Posizioniamo i limiti dei tratti con la dichiarazione del parametro di tipo generico
dopo i due punti e tra parentesi angolari.

La sintassi `impl Trait` è comoda e consente di scrivere codice più conciso nei casi
semplici, mentre la sintassi più completa dei limiti dei tratti può esprimere una maggiore complessità in altri
casi. Ad esempio, possiamo avere due parametri che implementano `Sommario`. Con la sintassi `impl Trait`, ciò si ottiene in questo modo:

```rust,ignore
pub fn notifica(elemento1: &impl Sommario, elemento2: &impl Sommario) {
```

L'utilizzo di `impl Trait` è appropriato se vogliamo che questa funzione consenta a `elemento1` e
`elemento2` di avere tipi diversi (purché entrambi i tipi implementino `Sommario`). Tuttavia, se
vogliamo forzare entrambi i parametri ad avere lo stesso tipo, dobbiamo usare un
trait bound, come questo:

```rust,ignore
pub fn notifica<T: Sommario>(elemento1: &T, elemento2: &T) {
```

Il tipo generico `T` specificato come tipo dei parametri `elemento1` e `elemento2`
vincola la funzione in modo che il tipo concreto del valore
passato come argomento per `elemento1` e `elemento2` debba essere lo stesso.

#### Specificare più Limiti del Tratto (Traits Bound) con la sintassi `+`

Possiamo anche specificare più di un trait bound. Supponiamo di voler usare `notifica` sia la formattazione di visualizzazione sia `riassunto` su `elemento`: specifichiamo nella definizione di `notifica`
che `elemento` deve implementare sia `Display` che `Sommario`. Possiamo farlo
utilizzando la sintassi `+`:

```rust,ignore
pub fn notifica(elemento: &(impl Sommario + Display)) {
```

La sintassi `+` è valida anche con i limiti di tratto sui tipi generici:

```rust,ignore
pub fn notifica<T: Sommario + Display>(elemento: &T) {
```

Con i due limiti di tratto specificati, il corpo di `notifica` può chiamare `riassunto`  
e utilizzare `{}` per formattare `elemento`.

#### Limiti di Tratto più Chiari con le Clausole `where`

L'utilizzo di troppi limiti di tratto ha i suoi svantaggi. Ogni generico ha i suoi limiti di tratto, quindi le funzioni con più parametri di tipo generico possono contenere molte
informazioni sui limiti di tratto tra il nome della funzione e il suo elenco di parametri,
rendendo la firma della funzione difficile da leggere. Per questo motivo, Rust ha una sintassi alternativa
per specificare i limiti dei tratti all'interno di una clausola `where` dopo la firma della funzione. Quindi, invece di scrivere:

```rust,ignore
fn una_funzione<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

possiamo usare una clausola `where`, in questo modo:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-where-clause/src/lib.rs:here}}
```

La firma di questa funzione è meno confusa: il nome della funzione, l'elenco dei parametri
e il tipo di ritorno sono vicini, come in una funzione senza molti limiti dei tratti.

### Restituzione di tipi che implementano tratti

Possiamo anche usare la sintassi `impl Trait` nella posizione di ritorno per restituire un
valore di un tipo che implementa un tratto, come mostrato qui:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

Utilizzando `impl Sommario` come tipo di ritorno, specifichiamo che la
funzione `riassumibile` restituisce un tipo che implementa il tratto `Sommario`
senza nominare il tipo concreto. In questo caso, `riassumibile`
restituisce un `SocialPost`, ma il codice che chiama questa funzione non ha bisogno di saperlo.

La possibilità di specificare un tipo di ritorno solo tramite il tratto che implementa è
particolarmente utile nel contesto di chiusure e iteratori, che tratteremo nel
Capitolo 13. Chiusure e iteratori creano tipi che solo il compilatore conosce o
tipi che sono molto lunghi da specificare. La sintassi `impl Trait` consente di specificare in modo conciso
che una funzione restituisca un tipo che implementa il tratto `Iterator`
senza dover scrivere un tipo molto lungo.

Tuttavia, è possibile utilizzare `impl Trait` solo se si restituisce un singolo tipo. Ad
esempio, questo codice che restituisce un `ArticoloNews` o un `SocialPost` con
il tipo di ritorno specificato come `impl Sommario` non funzionerebbe:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```

Restituire un `ArticoloNews` o un `SocialPost` non è consentito a causa
di restrizioni relative all'implementazione della sintassi `impl Trait` nel compilatore. Spiegheremo come scrivere una funzione con questo comportamento nella sezione [“Utilizzo di oggetti tratto che consentono valori di tipi
diversi”][using-trait-objects]<!-- ignore
--> del Capitolo 18.

### Utilizzo di Vincoli di Tratto per Implementare Metodi in Modo Condizionale

Utilizzando un vincolo di tratto con un blocco `impl` che utilizza parametri di tipo generico,
possiamo implementare metodi in modo condizionale per i tipi che implementano i tratti specificati. Ad esempio, il tipo `Coppia<T>` nel Listato 10-15 implementa sempre la
funzione `new` per restituire una nuova istanza di `Coppia<T>` (ricordiamo dalla sezione
[“Definizione di metodi”][methods]<!-- ignore --> del Capitolo 5 che `Self`
è un alias di tipo per il tipo del blocco `impl`, che in questo caso è
`Coppia<T>`). Ma nel blocco `impl` successivo, `Coppia<T>` implementa il metodo
`cmp_display` solo se il suo tipo interno `T` implementa il tratto `PartialOrd`
che abilita il confronto _e_ il tratto `Display` che abilita la stampa.

<Listing number="10-15" file-name="src/lib.rs" caption="Implementazione condizionale di metodi su un tipo generico in base ai limiti del tratto">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/lib.rs}}
```

</Listing>

Possiamo anche implementare in modo condizionale un tratto per qualsiasi tipo che implementa
un altro tratto. Le implementazioni di un tratto su qualsiasi tipo che soddisfi i limiti del tratto sono chiamate _implementazioni generali_ e sono ampiamente utilizzate nella
libreria standard Rust. Ad esempio, la libreria standard implementa il tratto
`ToString` su qualsiasi tipo che implementi il ​​tratto `Display`. Il blocco `impl`
nella libreria standard è simile a questo codice:

```rust,ignore
impl<T: Display> ToString for T {
// --snip--
}
```

Poiché la libreria standard ha questa implementazione generale, possiamo chiamare il metodo
`to_string` definito dal tratto `ToString` su qualsiasi tipo che implementi
il tratto `Display`. Ad esempio, possiamo trasformare gli interi nei loro corrispondenti valori
`String` in questo modo, perché gli interi implementano `Display`:

```rust
let s = 3.to_string();
```

Le implementazioni generali compaiono nella documentazione per il tratto nella
sezione "Implementatori".

I tratti e i limiti dei tratti ci consentono di scrivere codice che utilizza parametri di tipo generico per
ridurre le duplicazioni, ma anche di specificare al compilatore che desideriamo che il tipo generico
abbia un comportamento particolare. Il compilatore può quindi utilizzare le informazioni sui limiti dei tratti
per verificare che tutti i tipi concreti utilizzati nel nostro codice forniscano il
comportamento corretto. Nei linguaggi a tipizzazione dinamica, otterremmo un errore a
runtime se chiamassimo un metodo su un tipo che non lo definisce. Ma
Rust sposta questi errori in fase di compilazione, quindi siamo costretti a correggere i problemi
prima ancora che il nostro codice possa essere eseguito. Inoltre, non dobbiamo scrivere codice
che verifichi il comportamento a runtime, perché abbiamo già verificato in fase di compilazione. Ciò migliora le prestazioni senza dover rinunciare alla flessibilità
dei tipi generici.

[using-trait-objects]: ch18-02-trait-objects.html#utilizzo-di-oggetti-caratteristica-che-consentono-valori-di-tipi-diversi
[methods]: ch05-03-method-syntax.html#definire-i-metodi