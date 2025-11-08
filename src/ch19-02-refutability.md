## Confutabilità: Quando un _Pattern_ Potrebbe non Corrispondere

I _pattern_ si presentano in due forme: confutabili e inconfutabili
(_refutable/irrefutable_). I _pattern_ che corrispondono per qualsiasi possibile
valore passato sono _inconfutabili_. Un esempio sarebbe `x` nella dichiarazione
`let x = 5;` perché `x` corrisponde a qualsiasi cosa e quindi non può non
corrispondere. I _pattern_ che possono non corrispondere per un possibile valore
sono _confutabili_. Un esempio sarebbe `Some(x)` nell’espressione `if let
Some(x) = un_valore` perché se il valore nella variabile `un_valore` è `None`
anziché `Some`, il _pattern_ `Some(x)` non corrisponderà.

I parametri di funzione, le dichiarazioni `let` e i cicli `for` possono
accettare solo _pattern_ inconfutabili perché il programma non può fare nulla di
significativo quando i valori non corrispondono. Le espressioni `if let` e
`while let` e la dichiarazione `let...else` accettano sia _pattern_ confutabili
che inconfutabili, ma il compilatore mette in guardia contro i _pattern_
inconfutabili perché, per definizione, sono pensati per gestire possibili
fallimenti: la funzionalità di una condizione risiede nella sua capacità di
comportarsi in modo diverso a seconda del successo o del fallimento.

In generale, non ci si dovrebbe preoccupare della distinzione tra _pattern_
confutabili e inconfutabili; tuttavia, è necessario avere familiarità con il
concetto di confutabilità in modo da poterlo comprendere quando lo si vede in un
messaggio di errore. In questi casi, sarà necessario modificare il _pattern_ o
il costrutto con cui si sta utilizzando il _pattern_, a seconda del
comportamento previsto per il codice.

Esaminiamo un esempio di cosa succede quando proviamo a utilizzare un _pattern_
confutabile dove Rust richiede un _pattern_ inconfutabile e viceversa. Il
Listato 19-8 mostra una dichiarazione `let`, ma per il _pattern_ abbiamo
specificato `Some(x)`, un _pattern_ confutabile. Come ci si potrebbe aspettare,
questo codice non verrà compilato.

<Listing number="19-8" caption="Tentativo di utilizzare un _pattern_ confutabile con `let`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-08/src/main.rs:here}}
```

</Listing>

Se `un_valore_option` fosse un valore `None`, non corrisponderebbe al _pattern_
`Some(x)`, il che significa che il _pattern_ è confutabile. Tuttavia, la
dichiarazione `let` può accettare solo un _pattern_ inconfutabile perché non c’è
nulla di valido che il codice possa fare con un valore `None`. In fase di
compilazione, Rust ci segnalerà il fatto che abbiamo provato a utilizzare un
_pattern_ confutabile laddove è richiesto un _pattern_ inconfutabile:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-08/output.txt}}
```

Poiché non abbiamo coperto (e non potevamo coprire!) ogni valore valido con il
_pattern_ `Some(x)`, Rust genera giustamente un errore di compilazione.

Se abbiamo un _pattern_ confutabile laddove è necessario un _pattern_
inconfutabile, possiamo correggerlo modificando il codice che utilizza il
_pattern_: invece di usare `let`, possiamo usare `let...else`. Quindi, se il
_pattern_ non corrisponde, il codice tra parentesi graffe gestirà il valore. Il
Listato 19-9 mostra come correggere il codice nel Listato 19-8.

<Listing number="19-9" caption="Usare `let...else` e un blocco con _pattern_ confutabili invece di `let`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-09/src/main.rs:here}}
```

</Listing>

Abbiamo dato al codice una via d’uscita! Questo codice è perfettamente valido,
anche se significa che non potremmo usare un _pattern_ inconfutabile senza
ricevere un avviso. Se diamo a `let...else` un _pattern_ che corrisponderà
sempre, ed esempio `x`, come mostrato nel Listato 19-10, il compilatore genererà
un avviso.

<Listing number="19-10" caption="Tentativo di utilizzare un _pattern_ inconfutabile con `let...else`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-10/src/main.rs:here}}
```

</Listing>

Rust segnala che non ha senso utilizzare `let...else` con un _pattern_
inconfutabile:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-10/output.txt}}
```

Per questo motivo, i rami di `match` devono utilizzare _pattern_ inconfutabili,
ad eccezione dell’ultimo ramo, che dovrebbe corrispondere a tutti i valori
rimanenti con un _pattern_ inconfutabile. Rust ci consente di utilizzare un
_pattern_ inconfutabile in un `match` con singolo ramo, ma questa sintassi non è
particolarmente utile e potrebbe essere sostituita con una più semplice
dichiarazione `let`.

Ora che sappiamo dove usare i _pattern_ e la differenza tra _pattern_
confutabili e inconfutabili, esaminiamo tutta la sintassi che possiamo usare per
creare _pattern_.
