## Elaborazione di una Serie di Elementi con Iteratori

Il pattern iteratore consente di eseguire un'attività su una sequenza di elementi a turno. Un iteratore è responsabile della logica di iterazione su ciascun elemento e
di determinare quando la sequenza è terminata. Quando si utilizzano gli iteratori, non è necessario
reimplementare questa logica da soli.

In Rust, gli iteratori sono _lazy_, il che significa che non hanno effetto finché non si chiamano
metodi che consumano l'iteratore per utilizzarlo. Ad esempio, il codice nel
Listato 13-10 crea un iteratore sugli elementi nel vettore `v1` chiamando
il metodo `iter` definito su `Vec<T>`. Questo codice di per sé non fa nulla
di utile.

<Listing number="13-10" file-name="src/main.rs" caption="Creazione di un iteratore">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-10/src/main.rs:here}}
```

</Listing>

L'iteratore è memorizzato nella variabile `v1_iter`. Una volta creato un
iteratore, possiamo utilizzarlo in diversi modi. Nel Listato 3-5, abbiamo iterato su
un array utilizzando un ciclo `for` per eseguire del codice su ciascuno dei suoi elementi.
In pratica, questo creava e poi consumava implicitamente un iteratore, ma finora abbiamo omesso
l'utilizzo esatto.

Nell'esempio del Listato 13-11, separiamo la creazione dell'iteratore dal
suo utilizzo nel ciclo `for`. Quando il ciclo `for` viene chiamato utilizzando
l'iteratore in `v1_iter`, ogni elemento dell'iteratore viene utilizzato in un'iterazione del ciclo, che stampa ciascun valore.

<Listing number="13-11" file-name="src/main.rs" caption="Utilizzo di un iteratore in un ciclo `for`">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

</Listing>

Nei linguaggi che non hanno iteratori forniti dalle loro librerie standard,
probabilmente scriveresti questa stessa funzionalità iniziando una variabile all'indice
0, usando quella variabile per indicizzare il vettore per ottenere un valore e
incrementando il valore della variabile in un ciclo fino a raggiungere il numero totale di elementi nel vettore.

Gli iteratori gestiscono tutta questa logica per te, riducendo il codice ripetitivo che
potrebbe potenzialmente creare errori. Gli iteratori offrono maggiore flessibilità nell'utilizzare la stessa
logica con molti tipi diversi di sequenze, non solo con strutture dati in cui puoi
indicizzare, come i vettori. Esaminiamo come lo fanno gli iteratori.

### Il Trait `Iterator` e il Metodo `next`

Tutti gli iteratori implementano un tratto chiamato `Iterator` definito nella
libreria standard. La definizione del tratto è la seguente:

```rust
pub trait Iterator {
type Item;

fn next(&mut self) -> Option<Self::Item>;

// metodi con implementazioni predefinite
}
```

Si noti che questa definizione utilizza una nuova sintassi: `type Item` e `Self::Item`,
che definiscono un _tipo associato (associated type)_ a questo tratto. Parleremo approfonditamente dei
tipi associati nel Capitolo 20. Per ora, tutto ciò che dovete sapere è che
questo codice afferma che l'implementazione del tratto `Iterator` richiede anche la definizione di
un tipo `Item`, e questo tipo `Item` viene utilizzato nel tipo di ritorno del metodo `next`.
In altre parole, il tipo `Item` sarà il tipo restituito dall'
iteratore.

Il trait `Iterator` richiede agli implementatori di definire un solo metodo: il metodo `next`, che restituisce un elemento dell'iteratore alla volta, racchiuso in
`Some`, e, al termine dell'iterazione, restituisce `None`.

Possiamo chiamare direttamente il metodo `next` sugli iteratori; il Listato 13-12 mostra
quali valori vengono restituiti da chiamate ripetute a `next` sull'iteratore creato
dal vettore.

<Listing number="13-12" file-name="src/lib.rs" caption="Chiamata del metodo `next` su un iteratore">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/lib.rs:here}}
```

</Listing>

Si noti che era necessario rendere `v1_iter` mutabile: chiamare il metodo `next` su un
iteratore modifica lo stato interno che l'iteratore utilizza per tenere traccia della propria
posizione nella sequenza. In altre parole, questo codice _consuma_, o esaurisce, l'
iteratore. Ogni chiamata a `next` consuma un elemento dall'iteratore. Non era necessario
rendere `v1_iter` mutabile quando abbiamo usato un ciclo `for`, perché il ciclo prendeva
la proprietà di `v1_iter` e la rendeva mutabile in background.

Si noti inoltre che i valori ottenuti dalle chiamate a `next` sono riferimenti immutabili
ai valori nel vettore. Il metodo `iter` produce un iteratore
su riferimenti immutabili. Se vogliamo creare un iteratore che prende
la proprietà di `v1` e restituisce i valori posseduti, possiamo chiamare `into_iter` invece di
`iter`. Allo stesso modo, se vogliamo iterare su riferimenti mutabili, possiamo chiamare
`iter_mut` invece di `iter`.

### Metodi che Consumano l'Iteratore

Il tratto `Iterator` ha diversi metodi con implementazioni predefinite
fornite dalla libreria standard; è possibile scoprire di più su questi
metodi consultando la documentazione API della libreria standard per il tratto `Iterator`. Alcuni di questi metodi chiamano il metodo `next` nella loro definizione, motivo per cui
è necessario implementare il metodo `next` quando si implementa il tratto
`Iterator`.

I metodi che chiamano `next` sono chiamati _adattatori di consumo_, perché chiamandoli
si consuma l'iteratore. Un esempio è il metodo `sum`, che prende possesso
dell'iteratore e itera attraverso gli elementi chiamando ripetutamente `next`,
consumandolo. Durante l'iterazione, aggiunge ogni elemento a un totale parziale
e restituisce il totale al termine dell'iterazione. Il Listato 13-13 contiene un
test che illustra l'uso del metodo `sum`.

<Listing number="13-13" file-name="src/lib.rs" caption="Chiamata del metodo `sum` per ottenere il totale di tutti gli elementi nell'iteratore">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/lib.rs:here}}
```

</Listing>

Non è consentito utilizzare `v1_iter` dopo la chiamata a `sum` perché `sum` assume
la proprietà dell'iteratore su cui viene chiamato.

### Metodi che Producono Altri Iteratori

Gli _Iterator adapter (adattatori di iteratore)_ sono metodi definiti sul tratto `Iterator` che non
consumano l'iteratore. Invece, producono iteratori diversi modificando
qualche aspetto dell'iteratore originale.

Il Listato 13-14 mostra un esempio di chiamata del metodo dell'iteratore adapter `map`,
che accetta una closure per chiamare ogni elemento durante l'iterazione.
Il metodo `map` restituisce un nuovo iteratore che produce gli elementi modificati. La
closure qui crea un nuovo iteratore in cui ogni elemento del vettore verrà
incrementato di 1.

<Listing number="13-14" file-name="src/main.rs" caption="Chiamata dell'adattatore iteratore `map` per creare un nuovo iteratore">

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

</Listing>

Tuttavia, questo codice genera un avviso:

```console
{{#include ../listings/ch13-functional-features/listing-13-14/output.txt}}
```

Il codice nel Listato 13-14 non fa nulla; la closure che abbiamo specificato
non viene mai chiamata. L'avviso ci ricorda il motivo: gli adattatori di iteratori sono lazy e
dobbiamo consumare qui l'iteratore.

Per correggere questo avviso e consumare l'iteratore, useremo il metodo `collect`,
che abbiamo usato con `env::args` nel Listato 12-1. Questo metodo consuma l'iteratore
e raccoglie i valori risultanti in un tipo di dati di tipo collection.

Nel Listato 13-15, raccogliamo i risultati dell'iterazione sull'iteratore
restituito dalla chiamata a `map` in un vettore. Questo vettore finirà per contenere
ogni elemento del vettore originale, incrementato di 1.

<Listing number="13-15" file-name="src/main.rs" caption="Chiamata del metodo `map` per creare un nuovo iteratore, quindi chiamata del metodo `collect` per utilizzare il nuovo iteratore e creare un vettore">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/main.rs:here}}
```

</Listing>

Poiché `map` accetta una chiusura, possiamo specificare qualsiasi operazione desideriamo eseguire
su ciascun elemento. Questo è un ottimo esempio di come le chiusure consentano di personalizzare alcuni
comportamenti, riutilizzando al contempo il comportamento di iterazione fornito dal trait `Iterator`.

È possibile concatenare più chiamate agli adattatori di iteratori per eseguire azioni complesse in
modo leggibile. Tuttavia, poiché tutti gli iteratori sono lazy, è necessario chiamare uno dei
metodi dell'adattatore che li consuma per ottenere risultati dalle chiamate agli adattatori di iteratori.

### Utilizzo di Closure che Catturano il loro Ambiente

Molti adattatori di iteratori accettano le chiusure come argomenti e, di solito, le chiusure
che specificheremo come argomenti degli adattatori di iteratori saranno chiusure che catturano
il loro ambiente.

Per questo esempio, useremo il metodo `filter` che accetta una chiusura. La
chiusura riceve un elemento dall'iteratore e restituisce un valore `bool`. Se la chiusura
restituisce `true`, il valore verrà incluso nell'iterazione prodotta da
`filter`. Se la chiusura restituisce `false`, il valore non verrà incluso.

Nel Listato 13-16, utilizziamo `filter` con una chiusura che cattura la variabile `misura_scarpa`
dal suo ambiente per iterare su una collection di istanze della struttura `Scarpa`
. Restituirà solo le scarpe della taglia specificata.

<Listing number="13-16" file-name="src/lib.rs" caption="Utilizzo del metodo `filter` con una chiusura che cattura `misura_scarpa`">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs}}
```

</Listing>

La funzione `misura_scarpe` prende la proprietà di un vettore di scarpe e una taglia di scarpa
come parametri. Restituisce un vettore contenente solo scarpe della taglia
specificata.

Nel corpo di `misura_scarpe`, chiamiamo `into_iter` per creare un iteratore
che prende la proprietà del vettore. Quindi chiamiamo `filter` per adattare quell'iteratore
in un nuovo iteratore che contiene solo elementi per i quali la chiusura
restituisce `true`.

La chiusura cattura il parametro `misura_scarpa` dall'ambito e
confronta il valore con la taglia di ogni scarpa, mantenendo solo le scarpe della taglia
specificata. Infine, la chiamata a `collect` raccoglie i valori restituiti dall'iteratore
adattato in un vettore restituito dalla funzione.

Il test mostra che quando chiamiamo `misura_scarpe`, otteniamo solo le scarpe
che hanno la stessa taglia del valore specificato.
