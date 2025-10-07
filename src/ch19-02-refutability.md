## Confutabilità: Quando un Pattern Potrebbe non Corrispondere

I pattern si presentano in due forme: confutabili e inconfutabili. I pattern che corrispondono
per qualsiasi possibile valore passato sono _inconfutabili_. Un esempio sarebbe `x` nell'
istruzione `let x = 5;` perché `x` corrisponde a qualsiasi cosa e quindi non può
non corrispondere. I pattern che possono non corrispondere per un possibile valore sono
_confutabili_. Un esempio sarebbe `Some(x)` nell'espressione `if let Some(x) =
a_value` perché se il valore nella variabile `a_value` è `None` anziché
`Some`, il pattern `Some(x)` non corrisponderà.

I parametri di funzione, le istruzioni `let` e i cicli `for` possono accettare solo pattern
inconfutabili perché il programma non può fare nulla di significativo quando i valori
non corrispondono. Le espressioni `if let` e `while let` e l'istruzione
`let...else` accettano pattern confutabili e inconfutabili, ma il
compilatore mette in guardia contro i pattern inconfutabili perché, per definizione, sono
pensati per gestire possibili fallimenti: la funzionalità di una condizione risiede
nella sua capacità di comportarsi in modo diverso a seconda del successo o del fallimento.

In generale, non ci si dovrebbe preoccupare della distinzione tra pattern confutabili
e inconfutabili; tuttavia, è necessario avere familiarità con il concetto
di confutabilità in modo da poter rispondere quando lo si vede in un messaggio di errore. In
questi casi, sarà necessario modificare il pattern o il costrutto con cui si sta
utilizzando il pattern, a seconda del comportamento previsto per il codice.

Esaminiamo un esempio di cosa succede quando proviamo a utilizzare un pattern confutabile
dove Rust richiede un pattern inconfutabile e viceversa. Il Listato 19-8 mostra un'istruzione
`let`, ma per il pattern abbiamo specificato `Some(x)`, un pattern
confutabile. Come ci si potrebbe aspettare, questo codice non verrà compilato.

<Listing number="19-8" caption="Tentativo di utilizzare un pattern confutabile con `let`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-08/src/main.rs:here}}
```

</Listing>

Se `some_option_value` fosse un valore `None`, non corrisponderebbe al pattern
`Some(x)`, il che significa che il pattern è confutabile. Tuttavia, l'istruzione `let` può
accettare solo un pattern inconfutabile perché non c'è nulla di valido che il codice possa
fare con un valore `None`. In fase di compilazione, Rust si lamenterà del fatto che abbiamo provato a
utilizzare un pattern confutabile laddove è richiesto un pattern inconfutabile:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-08/output.txt}}
```

Poiché non abbiamo coperto (e non potevamo coprire!) ogni valore valido con il
pattern `Some(x)`, Rust genera giustamente un errore di compilazione.

Se abbiamo un pattern confutabile laddove è necessario un pattern inconfutabile, possiamo
correggerlo modificando il codice che utilizza il pattern: invece di usare `let`, possiamo
usare `let else`. Quindi, se il pattern non corrisponde, il codice salterà semplicemente
il codice tra parentesi graffe, consentendogli di continuare validamente. Il Listato
19-9 mostra come correggere il codice nel Listato 19-8.

<Listing number="19-9" caption="Usare `let...else` e ​​un blocco con pattern confutabili invece di `let`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-09/src/main.rs:here}}
```

</Listing>

Abbiamo dato al codice una via d'uscita! Questo codice è perfettamente valido, anche se significa che
non possiamo usare un pattern inconfutabile senza ricevere un avviso. Se diamo a `let...else` un pattern che corrisponderà sempre, come `x`, come mostrato nel Listato
19-10, il compilatore genererà un avviso.

<Listing number="19-10" caption="Tentativo di utilizzare un pattern inconfutabile con `let...else`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-10/src/main.rs:here}}
```

</Listing>

Rust lamenta che non ha senso utilizzare `let...else` con un
pattern inconfutabile:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-10/output.txt}}
```

Per questo motivo, i rami di corrispondenza devono utilizzare pattern inconfutabili, ad eccezione dell'ultimo
ramo, che dovrebbe corrispondere a tutti i valori rimanenti con un pattern inconfutabile. Rust
ci consente di utilizzare un pattern inconfutabile in un `match` con un solo ramo, ma
questa sintassi non è particolarmente utile e potrebbe essere sostituita con una più semplice
istruzione `let`.

Ora che sappiamo dove usare i pattern e la differenza tra pattern confutabili
e irrefutabili, esaminiamo tutta la sintassi che possiamo usare per creare
pattern.