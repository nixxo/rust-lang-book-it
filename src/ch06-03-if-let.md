## Controllo di flusso conciso con `if let` e `let else`

La sintassi `if let` consente di combinare `if` e `let` in un modo meno verboso
per gestire i valori che corrispondono a un singolo _pattern_, ignorando gli
altri. Considera il programma nel Listato 6-6 che fa _matching_ su un
`Option<u8>` nella variabile `config_max` ma vuole eseguire codice solo se il
valore è la variante `Some`.

<Listing number="6-6" caption="Un `match` che si interessa solo di eseguire codice quando il valore è `Some`">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

</Listing>

Se il valore è `Some`, stampiamo il valore contenuto nella variante `Some`
legandolo alla variabile `max` nel _pattern_. Non vogliamo fare nulla per il
valore `None`. Per soddisfare l’espressione `match` dobbiamo aggiungere `_ =>
()` dopo aver processato una sola variante, il che, a ben vedere, sembra codice
un po' inutile.

Invece, possiamo scrivere questo in modo più breve usando `if let`. Il codice
seguente si comporta allo stesso modo del `match` nel Listato 6-6:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```

La sintassi `if let` prende un _pattern_ e un’espressione separati da un segno
di uguale. Funziona come un `match`, dove l’espressione è data al `match` e il
_pattern_ è il suo primo ramo. In questo caso il _pattern_ è `Some(max)`, e
`max` si lega al valore dentro il `Some`. Possiamo quindi usare `max` nel corpo
del blocco `if let` nello stesso modo in cui lo usavamo nel corrispondente ramo
del `match`. Il codice nel blocco `if let` viene eseguito solo se il valore
corrisponde al _pattern_.

Usare `if let` significa meno digitazione, meno indentazione e meno codice poco
utile. Tuttavia si perde il controllo di esaustività che il `match` impone e che
garantisce di non dimenticare di gestire dei casi. La scelta tra `match` e `if
let` dipende da cosa stai facendo in quella situazione particolare e se un
codice più conciso valga la perdita del controllo esaustivo.

In altre parole, puoi pensare a `if let` come ad una espressione `match` ridotta
all'osso che esegue codice quando il valore corrisponde a un _pattern_ e poi
ignora tutti gli altri valori.

Possiamo includere un `else` con un `if let`. Il blocco di codice che accompagna
l’`else` è lo stesso blocco che andrebbe con il caso `_` nell’espressione
`match` equivalente all’`if let` con `else`. Ricorda la definizione dell’_enum_
`Moneta` nel Listato 6-4, dove la variante `Quarter` conteneva anche un valore
`StatoUSA`. Se volessimo contare tutte le monete non-quarter che vediamo
annunciando anche lo stato dei quarter, potremmo farlo con un’espressione
`match`, così:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

Oppure potremmo usare un `if let` e un `else`, così:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

## Restare sul “percorso felice” con `let...else`

Una buona pratica è eseguire una computazione quando un valore è presente e
restituire un valore di default altrimenti. Continuando con il nostro esempio
delle monete con un valore `StatoUSA`, se volessimo dire qualcosa di divertente
a seconda di quanto fosse vecchio lo stato sul quarter, potremmo introdurre un
metodo su `StatoUSA` per verificare l’età dello stato, così:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:state}}
```

Poi potremmo usare `if let` per fare _matching_ sul tipo di moneta, introducendo
una variabile `stato` all’interno del corpo della condizione, come nel Listato
6-7.

<Listing number="6-7" caption="Verificare se uno stato esisteva nel 1900 usando condizionali annidati dentro un `if let`.">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:describe}}
```

</Listing>

Questo risolve il problema, ma ha spostato il lavoro nel corpo dell’`if let`, e
se il lavoro da fare è più complicato potrebbe risultare difficile seguire come
i rami di alto livello si relazionano. Potremmo anche sfruttare il fatto che le
espressioni producono un valore, o per produrre `stato` dall’`if let` o per
ritornare anticipatamente, come in Listato 6-8. (Si potrebbe fare qualcosa di
simile anche con un `match`.)

<Listing number="6-8" caption="Usare `if let` per produrre un valore o ritornare anticipatamente.">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-08/src/main.rs:describe}}
```

</Listing>

Questo però è un po' scomodo da seguire: un ramo dell’`if let` produce un valore
e l’altro ritorna dalla funzione completamente.

Per rendere più esprimibile questo _pattern_ comune, Rust ha `let...else`. La
sintassi `let...else` prende un _pattern_ a sinistra e un’espressione a destra,
molto simile a `if let`, ma non ha un ramo `if`, soltanto un ramo `else`. Se il
_pattern_ corrisponde, legherà il valore estratto dal _pattern_ nello _scope_
esterno. Se il _pattern_ non corrisponde, il flusso va nel ramo `else`, che deve
restituire dalla funzione.

Nel Listato 6-9 puoi vedere come Listato 6-8 appare usando `let...else` al posto
di `if let`.

<Listing number="6-9" caption="Usare `let...else` per semplificare il flusso della funzione.">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-09/src/main.rs:describe}}
```

</Listing>

Nota che in questo modo si resta sul “percorso felice” nel corpo principale
della funzione, senza avere un controllo di flusso significativamente diverso
per i due rami come invece succedeva con `if let`.

Se hai una situazione in cui la logica è troppo verbosa per essere espressa con
un `match`, ricorda che anche `if let` e `let...else` sono nella tua cassetta
degli attrezzi di Rust.

## Riepilogo

Abbiamo ora coperto come usare gli _enum_ per creare _type_ personalizzati che
possono essere uno tra un insieme di valori enumerati. Abbiamo mostrato come il
_type_ `Option<T>` della libreria standard ti aiuta a usare il sistema dei
_type_ per prevenire errori. Quando i valori degli _enum_ contengono dati, puoi
usare `match` o `if let` per estrarre e usare quei valori, a seconda di quanti
casi devi gestire.

I tuoi programmi Rust possono ora esprimere concetti nel tuo dominio usando
`struct` ed `enum`. Creare _type_ personalizzati da usare nella tua API assicura
maggiore sicurezza dei dati (_type safety_): il compilatore garantirà che le tue
funzioni ricevano solo i valori del _type_ che ciascuna funzione si aspetta.

Per fornire un’API ben organizzata ai tuoi utenti, chiara da usare ed esponendo
solo ciò che serve, passiamo ora ai moduli di Rust.
