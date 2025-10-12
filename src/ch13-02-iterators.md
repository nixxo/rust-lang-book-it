## Elaborare una Serie di Elementi con Iteratori

Il modello dell’iteratore consente di eseguire un’attività su una sequenza di
elementi a turno. Un iteratore è responsabile della logica di iterazione su
ciascun elemento e di determinare quando la sequenza è terminata. Quando si
utilizzano gli iteratori, non è necessario re-implementare questa logica da
soli.

In Rust, gli iteratori sono _lazy_[^lazy]<!-- ignore --> (_pigri_), il che
significa che non hanno effetto finché non si chiamano metodi che consumano
l’iteratore per utilizzarlo. Ad esempio, il codice nel Listato 13-10 crea un
iteratore sugli elementi nel vettore `v1` chiamando il metodo `iter` definito su
`Vec<T>`. Questo codice di per sé non fa nulla di utile.

<Listing number="13-10" file-name="src/main.rs" caption="Creazione di un iteratore">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-10/src/main.rs:here}}
```

</Listing>

L’iteratore è memorizzato nella variabile `v1_iter`. Una volta creato un
iteratore, possiamo utilizzarlo in diversi modi. Nel Listato 3-5, abbiamo
iterato su un _array_ utilizzando un ciclo `for` per eseguire del codice su
ciascuno dei suoi elementi. In pratica, questo creava e poi consumava
implicitamente un iteratore, ma finora abbiamo omesso come funziona in pratica.

Nell’esempio del Listato 13-11, separiamo la creazione dell’iteratore dal suo
utilizzo nel ciclo `for`. Quando il ciclo `for` viene chiamato utilizzando
l’iteratore in `v1_iter`, ogni elemento dell’iteratore viene utilizzato in
un’iterazione del ciclo, che stampa ciascun valore.

<Listing number="13-11" file-name="src/main.rs" caption="Utilizzo di un iteratore in un ciclo `for`">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

</Listing>

Nei linguaggi che non hanno iteratori forniti dalle loro librerie standard,
probabilmente scriveresti questa stessa funzionalità inizializzando una
variabile all’indice 0, usando quella variabile per indicizzare il vettore per
ottenere un valore e incrementando il valore della variabile in un ciclo fino a
raggiungere il numero totale di elementi nel vettore.

Gli iteratori gestiscono tutta questa logica per te, riducendo il codice
ripetitivo che potrebbe potenzialmente creare errori. Gli iteratori offrono
maggiore flessibilità nell’utilizzare la stessa logica con molti tipi diversi di
sequenze, non solo con strutture dati in cui puoi indicizzare, come i vettori.
Esaminiamo come riescono a farlo.

### Il _Trait_ `Iterator` e il Metodo `next`

Tutti gli iteratori implementano un _trait_ chiamato `Iterator` definito nella
libreria standard. La definizione del _trait_ è la seguente:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // metodi con implementazioni predefinite tralasciati
}
```

Nota che questa definizione utilizza una sintassi che non abbiamo mai visto:
`type Item` e `Self::Item`, che definiscono un _type_ _associato_ (_associated
type_) a questo _trait_. Parleremo approfonditamente dei _type_ associati nel
Capitolo 20. Per ora, tutto ciò che devi sapere è che questo codice afferma che
l’implementazione del _trait_ `Iterator` richiede anche la definizione di un
_type_ `Item`, e questo _type_ `Item` viene utilizzato nel _type_ di ritorno del
metodo `next`. In altre parole, il _type_ `Item` sarà il _type_ restituito
dall’iteratore.

Il _trait_ `Iterator` richiede agli implementatori di definire un solo metodo:
il metodo `next`, che restituisce un elemento dell’iteratore alla volta,
racchiuso in `Some`, e, al termine dell’iterazione, restituisce `None`.

Possiamo chiamare direttamente il metodo `next` sugli iteratori; il Listato
13-12 mostra quali valori vengono restituiti da chiamate ripetute a `next`
sull’iteratore creato dal vettore.

<Listing number="13-12" file-name="src/lib.rs" caption="Chiamata del metodo `next` su un iteratore">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/lib.rs:here}}
```

</Listing>

Nota che è stato necessario rendere `v1_iter` mutabile: chiamare il metodo
`next` su un iteratore modifica lo stato interno che l’iteratore utilizza per
tenere traccia della propria posizione nella sequenza. In altre parole, questo
codice _consuma_, o esaurisce, l’iteratore. Ogni chiamata a `next` consuma un
elemento dall’iteratore. Non era necessario rendere `v1_iter` mutabile quando
abbiamo usato un ciclo `for`, perché il ciclo prendeva _ownership_ di `v1_iter`
e lo rendeva mutabile in background.

Nota inoltre che i valori ottenuti dalle chiamate a `next` sono _reference_
immutabili ai valori nel vettore. Il metodo `iter` produce un iteratore su
_reference_ immutabili. Se vogliamo creare un iteratore che prende _ownership_
di `v1` e restituisce i valori posseduti, possiamo chiamare `into_iter` invece
di `iter`. Allo stesso modo, se vogliamo iterare su _reference_ mutabili,
possiamo chiamare `iter_mut` invece di `iter`.

### Metodi che Consumano l’Iteratore

Il _trait_ `Iterator` ha diversi metodi con implementazioni predefinite fornite
dalla libreria standard; è possibile scoprire di più su questi metodi
consultando la documentazione API della libreria standard per il _trait_
`Iterator`. Alcuni di questi metodi chiamano il metodo `next` nella loro
definizione, motivo per cui è necessario implementare il metodo `next` quando si
implementa il _trait_ `Iterator` su un proprio _type_.

I metodi che chiamano `next` sono chiamati _consumatori_ (_consuming adapters_),
perché chiamandoli si consuma l’iteratore. Un esempio è il metodo `sum`, che
prende _ownership_ dell’iteratore e itera attraverso gli elementi chiamando
ripetutamente `next`, consumandolo. Durante l’iterazione, aggiunge ogni elemento
a un totale parziale e restituisce il totale al termine dell’iterazione. Il
Listato 13-13 contiene un test che illustra l’uso del metodo `sum`.

<Listing number="13-13" file-name="src/lib.rs" caption="Chiamata del metodo `sum` per ottenere il totale di tutti gli elementi nell’iteratore">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/lib.rs:here}}
```

</Listing>

Non è consentito utilizzare `v1_iter` dopo la chiamata a `sum` perché `sum`
prende _ownership_ dell’iteratore su cui viene chiamato.

### Metodi che Producono Altri Iteratori

Gli _adattatori di iteratore_ (_iterator adapter_) sono metodi definiti sul
_trait_ `Iterator` che non consumano l’iteratore. Invece, producono iteratori
diversi modificando qualche aspetto dell’iteratore originale.

Il Listato 13-14 mostra un esempio di chiamata del metodo dell’adattatore `map`,
che accetta una chiusura per chiamare ogni elemento durante l’iterazione. Il
metodo `map` restituisce un nuovo iteratore che produce gli elementi modificati.
La chiusura qui crea un nuovo iteratore in cui ogni elemento del vettore verrà
incrementato di 1.

<Listing number="13-14" file-name="src/main.rs" caption="Chiamata dell’adattatore `map` per creare un nuovo iteratore">

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

</Listing>

Tuttavia, questo codice genera un avviso:

```console
{{#include ../listings/ch13-functional-features/listing-13-14/output.txt}}
```

Il codice nel Listato 13-14 non fa nulla; la chiusura che abbiamo specificato
non viene mai chiamata. L’avviso ci ricorda il motivo: gli adattatori sono
_lazy_ e qui dobbiamo consumare l’iteratore.

Per correggere questo avviso e consumare l’iteratore, useremo il metodo
`collect`, che abbiamo usato con `env::args` nel Listato 12-1. Questo metodo
consuma l’iteratore e raccoglie i valori risultanti in un collezione di _type_
appropriato.

Nel Listato 13-15, raccogliamo i risultati dell’iterazione sull’iteratore
restituito dalla chiamata a `map` in un vettore. Questo vettore finirà per
contenere ogni elemento del vettore originale, incrementato di 1.

<Listing number="13-15" file-name="src/main.rs" caption="Chiamata del metodo `map` per creare un nuovo iteratore, quindi chiamata del metodo `collect` per consumare il nuovo iteratore e creare un vettore">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/main.rs:here}}
```

</Listing>

Poiché `map` accetta una chiusura, possiamo specificare qualsiasi operazione
desideriamo eseguire su ciascun elemento. Questo è un ottimo esempio di come le
chiusure consentano di personalizzare alcuni comportamenti, riutilizzando al
contempo il comportamento di iterazione fornito dal _trait_ `Iterator`.

È possibile concatenare più chiamate agli adattatori per eseguire azioni
complesse in modo leggibile. Tuttavia, poiché tutti gli iteratori sono _lazy_, è
necessario chiamare uno dei metodi consumatori per ottenere risultati dalle
chiamate agli adattatori.

### Chiusure che Catturano il Loro Ambiente

Molti adattatori accettano le chiusure come argomenti e, di solito, le chiusure
che specificheremo come argomenti degli adattatori saranno chiusure che
catturano il loro ambiente.

Per questo esempio, useremo il metodo `filter` che accetta una chiusura. La
chiusura riceve un elemento dall’iteratore e restituisce un valore `bool`. Se la
chiusura restituisce `true`, il valore verrà incluso nell’iterazione prodotta da
`filter`. Se la chiusura restituisce `false`, il valore non verrà incluso.

Nel Listato 13-16, utilizziamo `filter` con una chiusura che cattura la
variabile `misura_scarpe` dal suo ambiente per iterare su una collezione di
istanze della _struct_ `Scarpa` . Restituirà solo le scarpe della taglia
specificata.

<Listing number="13-16" file-name="src/lib.rs" caption="Utilizzo del metodo `filter` con una chiusura che cattura `misura_scarpa`">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs}}
```

</Listing>

La funzione `misura_scarpe` prende _ownership_ di un vettore di scarpe e una
taglia di scarpa come parametri. Restituisce un vettore contenente solo scarpe
della taglia specificata.

Nel corpo di `misura_scarpe`, chiamiamo `into_iter` per creare un iteratore che
prende _ownership_ del vettore. Quindi chiamiamo `filter` per adattare
quell’iteratore in un nuovo iteratore che contiene solo elementi per i quali la
chiusura restituisce `true`.

La chiusura cattura il parametro `misura_scarpa` dall’ambiente e confronta il
valore con la taglia di ogni scarpa, mantenendo solo le scarpe della taglia
specificata. Infine, la chiamata a `collect` raccoglie i valori restituiti
dall’iteratore adattato in un vettore restituito dalla funzione.

Il test mostra che quando chiamiamo `misura_scarpe`, otteniamo solo le scarpe
che hanno la stessa taglia del valore specificato.

[^lazy]: [Lazy su wikipedia (ita)](https://it.wikipedia.org/wiki/Lazy_initialization)
