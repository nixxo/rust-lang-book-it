## _Trait_ Avanzati

Abbiamo già visto i _trait_ nella sezione [“Definire il Comportamento Condiviso
con i _Trait_”][traits] del Capitolo 10, ma non abbiamo trattato i dettagli più
avanzati. Ora che sai di più su Rust, possiamo mettere le mani in pasta in certi
dettagli più complessi.

### Definire _Trait_ con _Type_ Associati

I _type_ _associati_ collegano un _type_ segnaposto con un _trait_ in modo che
le definizioni dei metodi del _trait_ possano usare questi segnaposto nelle loro
firme. Chi implementa il _trait_ specificherà il _type_ concreto da usare per
quella particolare implementazione. In questo modo possiamo definire un _trait_
che usa qualche _type_ senza dover sapere esattamente quali siano fino a quando
il _trait_ non verrà implementato.

Abbiamo detto che molte delle funzionalità avanzate di questo capitolo sono
usate raramente. I _type_ associati stanno a metà: si usano meno rispetto ad
altre funzionalità spiegate nel resto del libro, ma più frequentemente di altre
funzionalità in questo capitolo.

Un esempio di _trait_ con un _type_ associato è il _trait_ `Iterator` della
libreria standard. Il _type_ associato si chiama `Item` e rappresenta il _type_
dei valori su cui il _type_ che implementa `Iterator` itera. La definizione del
_trait_ `Iterator` è mostrata nel Listato 20-13.

<Listing number="20-13" caption="La definizione del _trait_ `Iterator` con _type_ associato `Item`">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-13/src/lib.rs}}
```

</Listing>

Il _type_ `Item` è un segnaposto, e la definizione del metodo `next` mostra che
restituirà valori di _type_ `Option<Self::Item>`. Chi implementa il _trait_
`Iterator` specifica il _type_ concreto per `Item`, e il metodo `next` ritorna
un `Option` che contiene un valore di quel _type_.

I _type_ associati potrebbero sembrare simili ai generici, visto che anche
questi ultimi permettono di definire una funzione senza specificare i _type_.
Per capirne la differenza, vediamo un’implementazione di `Iterator` su un _type_
chiamato `Contatore` che specifica `Item` come `u32`:

<Listing file-name="src/lib.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-22-iterator-on-counter/src/lib.rs:ch19}}
```

</Listing>

Questa sintassi ricorda i _type_ generici. Allora perché non definire `Iterator`
usando solo generici, come mostra il Listato 20-14?

<Listing number="20-14" caption="Definizione ipotetica di `Iterator` usando i generici">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-14/src/lib.rs}}
```

</Listing>

La differenza è che usando i generici, come nel Listato 20-14, dobbiamo annotare
i _type_ in ogni implementazione; siccome potremmo anche implementare
`Iterator<String> for Contatore` o qualsiasi altro _type_, potremmo avere più
implementazioni di `Iterator` per `Contatore`. In altre parole, quando un
_trait_ ha un parametro generico, può essere implementato per un _type_ più
volte, cambiando i _type_ concreti dei parametri generici ogni volta. Quando
usiamo il metodo `next` su `Contatore`, dovremmo fornire annotazioni di _type_
per indicare quale implementazione di `Iterator` vogliamo usare.

Con i _type_ associati non serve annotare i _type_ perché non possiamo
implementare un _trait_ più volte su uno stesso _type_. Nel Listato 20-13, con i
_type_ associati, scegliamo il _type_ di `Item` una sola volta perché c’è una
sola `impl Iterator for Contatore`. Non serve indicare che vogliamo un iteratore
di `u32` ogni volta che chiamiamo `next` su `Contatore`.

I _type_ associati diventano parte del contratto del _trait_: chi implementa il
_trait_ deve fornire un _type_ per sostituire il segnaposto. Spesso i _type_
associati hanno nomi che descrivono come saranno usati, ed è buona prassi
documentarli nelle API.

### Usare Parametri Generici di Default e Sovrascrivere gli Operatori

Quando usiamo _type_ generici, possiamo specificare un _type_ concreto di
default per il _type_ generico. Questo elimina la necessità per chi implementa
il _trait_ di specificare un _type_ concreto se il _type_ di default va bene. Si
specifica un _type_ di default dichiarando il generico con la sintassi
`<TypeSegnaposto=TypeConcreto>`.

Un ottimo esempio di questa tecnica è la sovrascrittura degli operatori, dove
personalizzi il comportamento di un operatore (come `+`) in situazioni
particolari.

Rust non permette di creare operatori propri o sovrascrivere operatori
arbitrari. Ma puoi sovrascrivere le operazioni e i _trait_ corrispondenti
elencati in `std::ops` implementando i _trait_ associati all’operatore. Per
esempio, nel Listato 20-15 sovrascriviamo l’operatore `+` per sommare due
istanze di `Punto`. Lo facciamo implementando il _trait_ `Add` per la _struct_
`Punto`.

<Listing number="20-15" file-name="src/main.rs" caption="Implementazione del _trait_ `Add` per sovrascrivere l’operatore `+` per le istanze di `Punto`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-15/src/main.rs}}
```

</Listing>

Il metodo `add` somma i valori `x` di due istanze `Punto` e i valori `y` di due
istanze `Punto` per creare un nuovo `Punto`. Il _trait_ `Add` ha un _type_
associato chiamato `Output` che determina il _type_ restituito dal metodo `add`.

Il _type_ generico di default in questo codice si trova all’interno del _trait_
`Add`. Ecco la sua definizione:

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

Questo codice dovrebbe sembrarti familiare: un _trait_ con un metodo e un _type_
associato. La novità è `Rhs=Self`: questa sintassi si chiama _default type
parameters_ ( _type_ _di default dei parametri_). Il parametro generico `Rhs`
(abbreviazione di _right-hand side_, lato destro) definisce il _type_ del
parametro `rhs` nel metodo `add`. Se non specifichiamo un _type_ concreto per
`Rhs` nell’implementazione di `Add`, il _type_ di `Rhs` di default sarà `Self`,
il _type_ su cui stiamo implementando `Add`.

Quando abbiamo implementato `Add` per `Punto`, abbiamo usato il default per
`Rhs` perché volevamo sommare due `Punto`. Passiamo ora a un esempio di
implementazione del _trait_ `Add` in cui vogliamo personalizzare il _type_ `Rhs`
invece di usare il default.

Abbiamo due _struct_, `Millimetri` e `Metri`, che contengono valori in unità
diverse. Questo tipo di “incapsulamento sottile” attorno a un _type_ esistente è
chiamato _newtype pattern_, di cui parleremo più avanti nella sezione
[“Implementare _Trait_ Esterni con il Modello _Newtype_”][newtype]<!-- ignore
-->. Vogliamo sommare valori in millimetri a valori in metri e far sì che l’implementazione di `Add` si occupi di fare la conversione corretta. Possiamo implementare `Add` per `Millimetri` con `Metri` come `Rhs`, come mostrato nel Listato 20-16.

<Listing number="20-16" file-name="src/lib.rs" caption="Implementazione del _trait_ `Add` su `Millimetri` per sommare `Millimetri` con `Metri`">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-16/src/lib.rs}}
```

</Listing>

Per sommare `Millimetri` e `Metri`, specifichiamo `impl Add<Metri>` per
impostare il valore del parametro di _type_ `Rhs` invece di usare il default
`Self`.

Userai i _type_ di default per i parametri in due modi principali:

1. Per estendere un _type_ senza rompere il codice esistente
1. Per permettere personalizzazioni in casi specifici che la maggior parte degli
   utenti non userà

Il _trait_ `Add` della libreria standard è un esempio del secondo punto: di
solito si sommano due _type_ uguali, ma il _trait_ `Add` permette di
personalizzare questo comportamento. L’uso di un _type_ di default come
parametro nella definizione di `Add` significa che non devi specificare quel
parametro extra la maggior parte delle volte, riducendo il codice ripetitivo e
facilitandone l’uso.

Il primo punto è simile ma all’opposto: se vuoi aggiungere un _type_ come
parametro a un _trait_ esistente, puoi dargli un default per permettere
l’estensione della funzionalità del _trait_ senza rompere il codice esistente.

### Disambiguare Tra Metodi Con lo Stesso Nome

Nulla in Rust vieta ad un _trait_ di avere metodi che hanno lo stesso nome di
metodi in un altro _trait_. E Rust non ti impedisce di implementare entrambi i
_trait_ su di un _type_. È possibile anche definire un metodo sul _type_ con lo
stesso nome di un metodo del _trait_.

Quando chiami metodi con lo stesso nome devi indicare a Rust quale vuoi usare.
Considera il codice nel Listato 20-17, dove sono definiti due _trait_, `Pilota`
e `Mago`, entrambi con un metodo chiamato `vola`. Entrambi i _trait_ sono
implementati su un _type_ `Umano`, che ha anche un metodo `vola` definito
direttamente.

<Listing number="20-17" file-name="src/main.rs" caption="Due _trait_ con un metodo `vola` implementati sul _type_ `Umano`, e un metodo `vola` definito direttamente su `Umano`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-17/src/main.rs:here}}
```

</Listing>

Quando chiamiamo `vola` su un’istanza `Umano`, come impostazione predefinita il
compilatore chiama il metodo definito direttamente sul _type_, come mostrato nel
Listato 20-18.

<Listing number="20-18" file-name="src/main.rs" caption="Chiamare `vola` su un’istanza di `Umano`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-18/src/main.rs:here}}
```

</Listing>

Questo codice stampa `*sbatte furiosamente le braccia*`, mostrando che Rust
chiama il metodo `vola` definito direttamente su `Umano`.

Per chiamare i metodi `vola` dal _trait_ `Pilota` o `Mago` serve una sintassi
più esplicita per indicare quale metodo si intende. Il Listato 20-19 mostra
questa sintassi.

<Listing number="20-19" file-name="src/main.rs" caption="Specificare quale metodo `vola` di quale _trait_ si vuole chiamare">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-19/src/main.rs:here}}
```

</Listing>

Specificare il nome del _trait_ prima del metodo chiarisce a Rust quale
implementazione di `vola` vogliamo chiamare. Possiamo anche scrivere
`Umano::vola(&persona)`, che è equivalente a `person.vola()`, ma è più verboso
se non serve disambiguare.

Questo codice stampa:

```console
{{#include ../listings/ch20-advanced-features/listing-20-19/output.txt}}
```

Poiché il metodo `vola` prende `self` come parametro, se avessimo due _type_ che
implementano entrambi un _trait_, Rust potrebbe inferire quale implementazione
del _trait_ usare in base al _type_ di `self`.

Tuttavia, le funzioni associate che non sono metodi non hanno `self`. Quando ci
sono più _type_ o _trait_ che definiscono funzioni con lo stesso nome, Rust non
sa sempre quale _type_ intendi, a meno che non si usi la sintassi completamente
qualificata. Per esempio, nel Listato 20-20 creiamo un _trait_ per un rifugio
per animali che vuole chiamare tutti i cuccioli di cane Rex. Realizziamo un
_trait_ `Animale` con una funzione associata chiamata `nomignolo`. Il _trait_
`Animale` è implementato per la _struct_ `Cane`, sulla quale definiamo una
funzione associata `nomignolo`.

<Listing number="20-20" file-name="src/main.rs" caption="_Trait_ con funzione associata e _type_ con funzione associata dello stesso nome che implementa il _trait_">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-20/src/main.rs}}
```

</Listing>

Implementiamo il codice che chiama tutti i cuccioli Rex nella funzione associata
`nomignolo` definita direttamente in `Cane`. Il _type_ `Cane` implementa anche
il _trait_ `Animale`, che descrive caratteristiche comuni a tutti gli animali. I
piccoli di cane sono chiamati cuccioli, e questo è espresso nell’implementazione
del _trait_ `Animale` per `Cane` nella funzione `nomignolo` associata al _trait_
`Animale`.

In `main`, chiamando `Cane::nomignolo` chiamiamo la funzione associata definita
direttamente su `Cane`. Questo codice stampa:

```console
{{#include ../listings/ch20-advanced-features/listing-20-20/output.txt}}
```

Questo output non è quello voluto: vogliamo chiamare la funzione `nomignolo` del
_trait_ `Animale` implementato su `Cane` così che il codice stampi `Un piccolo
di cane è detto cucciolo`. La tecnica di specificare il nome del _trait_ come
nel Listato 20-19 non aiuta; se cambiamo `main` con il codice del Listato 20-21,
otterremo un errore di compilazione.

<Listing number="20-21" file-name="src/main.rs" caption="Tentativo di chiamare la funzione `nomignolo` del _trait_ `Animale`, ma Rust non sa quale implementazione usare">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-21/src/main.rs:here}}
```

</Listing>

Poiché `Animale::nomignolo` non ha il parametro `self`, e potrebbero esserci
altri _type_ che implementano il _trait_ `Animale`, Rust non riesce a inferire
quale implementazione di `Animale::nomignolo` usare. Otterremo questo errore del
compilatore:

```console
{{#include ../listings/ch20-advanced-features/listing-20-21/output.txt}}
```

Per disambiguare e indicare a Rust che vogliamo usare l’implementazione del
_trait_ `Animale` per `Cane` anziché quella per un altro _type_, usiamo la
sintassi completamente qualificata. Il Listato 20-22 mostra come.

<Listing number="20-22" file-name="src/main.rs" caption="Uso della sintassi completamente qualificata per chiamare la funzione `nomignolo` del _trait_ `Animale` implementato su `Cane`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-22/src/main.rs:here}}
```

</Listing>

Forniamo un’annotazione di _type_ tra parentesi angolari `< >` per dire a Rust
che vogliamo chiamare il metodo `nomignolo` del _trait_ `Animale` implementato
su `Cane`, trattando il _type_ `Cane` come un _type_ `Animale` per questa
chiamata. Ora il codice stampa quello che vogliamo:

```console
{{#include ../listings/ch20-advanced-features/listing-20-22/output.txt}}
```

In generale la sintassi completamente qualificata è:

```rust,ignore
<Tipo as Trait>::funzione(ricevente_se_metodo, prossimo_argomento,...);
```

Per funzioni associate che non sono metodi, non c’è un parametro `self`, solo la
lista degli altri argomenti. Puoi usare la sintassi completamente qualificata
ovunque chiami funzioni o metodi. Tuttavia, puoi omettere parti che Rust può
dedurre dal contesto. Devi usarla solo quando ci sono più implementazioni con lo
stesso nome e Rust ha bisogno di aiuto per distinguere quale usare.

### Usare _Supertrait_

A volte puoi scrivere un _trait_ che dipende da un altro _trait_: per un _type_
che implementa il primo _trait_, richiedi che implementi anche il secondo
_trait_. Lo fai perché la definizione del _trait_ può usare gli elementi
associati del secondo. Il _trait_ da cui il _trait_ che implementi dipende viene
chiamato _supertrait_ del tuo _trait_.

Per esempio, supponiamo di voler creare un _trait_ `StampaContorno` con un
metodo `stampa_contorno` che stampa un valore delimitato da un contorno di
asterischi. Data una _struct_ `Punto` che implementa il _trait_ `Display` della
libreria standard per mostrare `(x, y)`, quando chiami `stampa_contorno` su
un’istanza di `Punto` con `x=1` e `y=3`, dovrebbe stampare:

```text
**********
*        *
* (1, 3) *
*        *
**********
```

Nell’implementazione di `stampa_contorno` vogliamo usare la funzionalità del
_trait_ `Display`. Quindi il _trait_ `StampaContorno` dovrebbe funzionare solo
per _type_ che implementano anche `Display`. Lo specifichiamo nella definizione
con `StampaContorno: Display`. Questo è simile ad aggiungere un vincolo di
_trait_ al _trait_ in questione. Il Listato 20-23 mostra un implementazione del
_trait_ `StampaContorno`.

<Listing number="20-23" file-name="src/main.rs" caption="Implementazione del _trait_ `StampaContorno` che richiede la funzionalità di `Display`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-23/src/main.rs:here}}
```

</Listing>

Dichiarando che `StampaContorno` richiede il _trait_ `Display`, possiamo usare
la funzione `to_string` che è implementata automaticamente su tutti i _type_ che
implementano `Display`. Se provassimo a usare `to_string` senza specificare
`Display`, otterremmo un errore perché il metodo `to_string` non sarebbe trovato
per il _type_ `&Self` nello _scope_ corrente.

Vediamo cosa succede se provassimo a implementare `StampaContorno` su un _type_
che non implementa `Display` come la _struct_ `Punto`:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-02-impl-outlineprint-for-point/src/main.rs:here}}
```

</Listing>

Riceveremmo un errore che dice che `Display` è richiesto ma non implementato:

```console
{{#include ../listings/ch20-advanced-features/no-listing-02-impl-outlineprint-for-point/output.txt}}
```

Lo risolviamo implementando `Display` su `Punto` e soddisfiamo le necessità di
`StampaContorno`:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-03-impl-display-for-point/src/main.rs:here}}
```

</Listing>

A questo punto l’implementazione di `StampaContorno` per `Punto` si compila
correttamente e possiamo chiamare `stampa_contorno` su un’istanza di `Punto` per
stamparlo con un contorno di asterischi.

### Implementare _Trait_ Esterni con il Modello _Newtype_

Nella sezione [“Implementare un _Trait_ su un
_Type_”][implementing-a-trait-on-a-type]<!-- ignore --> del Capitolo 10 abbiamo
parlato della _orphan rule_ che dice che possiamo implementare un _trait_ su un
_type_ solo se il _trait_ o il _type_ (o entrambi) sono locali al _crate_. Si
può aggirare questa restrizione usando il modello _newtype_, che consiste nel
creare un nuovo _type_ come _struct_ tupla. (Ne abbiamo già parlato in [“Creare
_Type_ Diversi con _Struct_ Tupla”][tuple-structs]<!-- ignore --> del Capitolo
5.) La _struct_ tupla avrà un solo campo e sarà un “incapsulamento sottile”
attorno al _type_ su cui vuoi implementare un _trait_. L’incapsulamento è locale
al _crate_ e puoi implementare il _trait_ sull’incapsulatore. La parola
_newtype_ deriva dal linguaggio Haskell. Non c’è alcuna penalità in prestazioni
nell’uso di questo modello, e il _type_ dell’involucro viene eliso in fase di
compilazione.

Per esempio, supponiamo di voler implementare `Display` su `Vec<T>`, cosa che la
_orphan rule_ ci impedisce perché sia il _trait_ `Display` che il _type_
`Vec<T>` sono definiti fuori dal nostro _crate_. Possiamo invece creare una
_struct_ `Capsula` che contiene un’istanza di `Vec<T>`, poi implementare
`Display` su `Capsula` e usare il valore `Vec<T>` all’interno, come mostrato nel
Listato 20-24.

<Listing number="20-24" file-name="src/main.rs" caption="Creazione di un _type_ `Capsula` attorno a `Vec<String>` per implementare `Display`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-24/src/main.rs}}
```

</Listing>

L’implementazione di `Display` usa `self.0` per accedere a `Vec<T>` perché
`Capsula` è una _struct_ tupla e `Vec<T>` è il campo all’indice 0 della tupla.
Così possiamo usare la funzionalità del _trait_ `Display` su `Capsula`.

Lo svantaggio di questa tecnica è che `Capsula` è un nuovo _type_ e non ha i
metodi del valore che incapsula. Dovresti implementare manualmente tutti i
metodi di `Vec<T>` in `Capsula`, in modo che i metodi deleghino a `self.0`, per
poter usare `Capsula` come fosse effettivamente un `Vec<T>`. Se volessimo che il
nuovo _type_ abbia tutti i metodi del _type_ interno, implementare il _trait_
`Deref` su `Capsula` per restituire il _type_ interno potrebbe essere una
soluzione (abbiamo parlato dell’implementazione di `Deref` in [“Trattare i
Puntatori Intelligenti Come Normali _Reference_”][smart-pointer-deref]<!--
ignore --> in Chapter 15). Se invece non vogliamo che `Capsula` abbia tutti i
metodi del _type_ interno, ad esempio per restringerne le funzionalità, dovremmo
implementare i metodi che vogliamo manualmente.

Il modello _newtype_ è utile anche quando non si tratta di _trait_. Passiamo ora
a delle tecniche avanzate per interagire col sistema dei _type_ di Rust.

[newtype]: ch20-02-advanced-traits.html#implementare-trait-esterni-con-il-modello-newtype
[implementing-a-trait-on-a-type]: ch10-02-traits.html#implementare-un-trait-su-un-type
[traits]: ch10-02-traits.html#definire-il-comportamento-condiviso-con-i-trait
[smart-pointer-deref]: ch15-02-deref.html#trattare-i-puntatori-intelligenti-come-normali-reference
[tuple-structs]: ch05-01-defining-structs.html#creare-type-diversi-con-struct-tupla
