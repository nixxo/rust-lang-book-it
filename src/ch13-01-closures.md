<!-- Old heading. Do not remove or links may break. -->

<a id="closures-anonymous-functions-that-can-capture-their-environment"></a>

## Closures: Funzioni Anonime che Catturano il loro Ambiente

Le closures di Rust sono funzioni anonime che è possibile salvare in una variabile o passare come
argomenti ad altre funzioni. È possibile creare la closure in un punto e poi
chiamarla altrove per valutarla in un contesto diverso. A differenza delle
funzioni, le closures possono catturare valori dall'ambito in cui sono definite.
Dimostreremo come queste funzionalità di closure consentano il riutilizzo del codice e la
personalizzazione del comportamento.

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-an-abstraction-of-behavior-with-closures"></a>
<a id="refactoring-using-functions"></a>
<a id="refactoring-with-closures-to-store-code"></a>

### Catturare l'Ambiente con le Closures

Esamineremo innanzitutto come possiamo utilizzare le closures per catturare valori dall'ambito in cui sono definite per un uso successivo. Ecco lo scenario: ogni tanto, la nostra azienda di magliette regala una maglietta esclusiva in edizione limitata a
qualcuno nella nostra mailing list come promozione. Gli utenti della mailing list possono
facoltativamente aggiungere il loro colore preferito al proprio profilo. Se la persona a cui viene assegnata
una maglietta gratuita ha impostato il suo colore preferito, riceverà la maglietta di quel colore. Se la persona
non ha specificato un colore preferito, riceverà il colore di cui l'azienda
ha attualmente la maggiore disponibilità.

Ci sono molti modi per implementarlo. Per questo esempio, useremo un'
enum chiamata `ShirtColor` che ha le varianti `Red` e `Blue` (limitando il
numero di colori disponibili per semplicità). Rappresentiamo l'inventario dell'azienda
con una struttura `Inventory` che ha un campo denominato `shirts` che
contiene un `Vec<ShirtColor>` che rappresenta i colori delle magliette attualmente disponibili in magazzino.
Il metodo `giveaway` definito su `Inventory` ottiene la preferenza opzionale per il colore della maglietta
del vincitore della maglietta gratuita e restituisce il colore della maglietta che la persona
riceverà. Questa configurazione è mostrata nel Listato 13-1.

<Listing number="13-1" file-name="src/main.rs" caption="Shirt company giveaway situation">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

</Listing>

Lo `store` definito in `main` ha due magliette blu e una rossa rimanenti
da distribuire per questa promozione in edizione limitata. Chiamiamo il metodo `giveaway`
per un utente con preferenza per una maglietta rossa e un utente senza alcuna preferenza.

Anche in questo caso, questo codice potrebbe essere implementato in molti modi e, per concentrarci sulle
closures, ci siamo attenuti ai concetti che avete già imparato, ad eccezione del corpo del
metodo `giveaway` che utilizza una closure. Nel metodo `giveaway`, otteniamo la
preferenza dell'utente come parametro di tipo `Option<ShirtColor>` e chiamiamo il
metodo `unwrap_or_else` su `user_preference`. Il metodo [`unwrap_or_else` su
`Option<T>`][unwrap-or-else]<!-- ignore --> è definito dalla libreria standard. Accetta un argomento: una closure senza argomenti che restituisce un valore `T`
(lo stesso tipo memorizzato nella variante `Some` di `Option<T>`, in questo caso
`ShirtColor`). Se `Option<T>` è la variante `Some`, `unwrap_or_else`
restituisce il valore presente all'interno di `Some`. Se `Option<T>` è la variante `None`
, `unwrap_or_else` chiama la closure e restituisce il valore restituito
dalla closure.

Specifichiamo l'espressione di closure `|| self.most_stocked()` come argomento di
`unwrap_or_else`. Questa è una closure che non accetta parametri (se la
closure avesse parametri, questi apparirebbero tra le due pipe verticali). Il
corpo della closure chiama `self.most_stocked()`. Stiamo definendo la closure
qui, e l'implementazione di `unwrap_or_else` valuterà la closure
in seguito, se il risultato è necessario.

L'esecuzione di questo codice stampa quanto segue:

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

Un aspetto interessante è che abbiamo passato una closure che chiama
`self.most_stocked()` sull'istanza corrente di `Inventory`. La libreria standard
non aveva bisogno di sapere nulla sui tipi `Inventory` o `ShirtColor` che abbiamo
definito, né sulla logica che vogliamo utilizzare in questo scenario. La closure cattura un
riferimento immutabile all'istanza `self` di `Inventory` e lo passa con il
codice che specifichiamo al metodo `unwrap_or_else`. Le funzioni, d'altra parte,
non sono in grado di catturare il loro ambito in questo modo.

### Inferenza e Annotazione del tipo di closure

Esistono ulteriori differenze tra funzioni e closures. Le closures 
di solito non richiedono di annotare i tipi dei parametri o il valore di ritorno,
come fanno le funzioni `fn`. Le annotazioni di tipo sono necessarie sulle funzioni perché
i tipi fanno parte di un'interfaccia esplicita esposta agli utenti. Definire rigidamente questa
interfaccia è importante per garantire che tutti concordino sui tipi
di valori che una funzione utilizza e restituisce. Le closures, d'altra parte, non vengono utilizzate
in un'interfaccia esposta come questa: vengono memorizzate in variabili e utilizzate senza
denominarle ed esporle agli utenti della nostra libreria.

Le closures sono in genere brevi e rilevanti solo in un contesto ristretto, piuttosto che in uno scenario arbitrario. In questi contesti limitati, il compilatore può
dedurre i tipi dei parametri e il tipo di ritorno, in modo simile a come è in grado
di dedurre i tipi della maggior parte delle variabili (ci sono rari casi in cui il compilatore
necessita anche di annotazioni del tipo di closure).

Come per le variabili, possiamo aggiungere annotazioni di tipo se vogliamo aumentare
l'esplicitezza e la chiarezza, a costo di essere più prolissi del necessario. L'annotazione dei tipi per una closure sarebbe simile alla definizione
mostrata nel Listato 13-2. In questo esempio, definiamo una closure e la memorizziamo
in una variabile, anziché definirla nel punto in cui la passiamo come
argomento, come abbiamo fatto nel Listato 13-1.

<Listing number="13-2" file-name="src/main.rs" caption="Aggiunta di annotazioni di tipo facoltative dei tipi di parametro e valore di ritorno nella closure">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

</Listing>

Con l'aggiunta delle annotazioni di tipo, la sintassi delle closures appare più simile alla
sintassi delle funzioni. Qui, per confronto, definiamo una funzione che aggiunge 1 al suo parametro e
una closure che ha lo stesso comportamento. Abbiamo aggiunto alcuni spazi
per allineare le parti rilevanti. Questo illustra come la sintassi delle closures sia simile
a quella delle funzioni, fatta eccezione per l'uso delle pipe e per la quantità di sintassi che è
facoltativa:

```rust,ignore
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

La prima riga mostra una definizione di funzione e la seconda una definizione di closure completamente annotata. Nella terza riga, rimuoviamo le annotazioni di tipo
dalla definizione di closure. Nella quarta riga, rimuoviamo le parentesi, che
sono facoltative perché il corpo della closure ha una sola espressione. Queste sono tutte
definizioni valide che produrranno lo stesso comportamento quando vengono chiamate. Le
righe `add_one_v3` e `add_one_v4` richiedono che le closures vengano valutate per
essere compilabili, poiché i tipi verranno dedotti dal loro utilizzo. Questo è
simile a `let v = Vec::new();` che richiede annotazioni di tipo o valori di
qualche tipo da inserire in `Vec` affinché Rust possa dedurre il tipo.

Per le definizioni di closure, il compilatore dedurrà un tipo concreto per ciascuno dei
loro parametri e per il loro valore di ritorno. Ad esempio, il Listato 13-3 mostra
la definizione di una closure breve che restituisce semplicemente il valore ricevuto come
parametro. Questa closure non è molto utile, se non per gli scopi di questo
esempio. Si noti che non abbiamo aggiunto alcuna annotazione di tipo alla definizione.
Poiché non ci sono annotazioni di tipo, possiamo chiamare la closure con qualsiasi tipo,
come abbiamo fatto qui con `String` la prima volta. Se poi proviamo a chiamare
`example_closure` con un intero, otterremo un errore.

<Listing number="13-3" file-name="src/main.rs" caption="Tentativo di chiamare una closure i cui tipi sono inferiti con due tipi diversi">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

</Listing>

Il compilatore ci dà questo errore:

```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

La prima volta che chiamiamo `example_closure` con il valore `String`, il compilatore
deduce che il tipo di `x` e il tipo di ritorno della closure siano `String`. Questi
tipi vengono quindi bloccati nella closure in `example_closure` e si verifica un errore di tipo
quando si tenta nuovamente di utilizzare un tipo diverso con la stessa closure.

### Cattura di Riferimenti o Trasferimento di Proprietà

Le closures possono catturare valori dal loro ambiente in tre modi, che
corrispondono direttamente ai tre modi in cui una funzione può accettare un parametro: prendendo in prestito
immutabilmente, prendendo in prestito mutabilmente e prendendo in possesso. La closure deciderà
quale di questi utilizzare in base a ciò che il corpo della funzione fa con i
valori catturati.

Nel Listato 13-4, definiamo una closure che cattura un riferimento immutabile al
vettore denominato `list` perché necessita solo di un riferimento immutabile per stampare
il valore.

<Listing number="13-4" file-name="src/main.rs" caption="Definizione e chiamata di una closure che cattura un riferimento immutabile">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

</Listing>

Questo esempio illustra anche che una variabile può essere associata a una definizione di closure,
e che possiamo successivamente chiamare la closure utilizzando il nome della variabile e le parentesi come
se il nome della variabile fosse il nome di una funzione.

Poiché possiamo avere più riferimenti immutabili a `list` contemporaneamente,
`list` è comunque accessibile dal codice prima della definizione della closure, dopo
la definizione della closure ma prima che la closure venga chiamata, e dopo che la closure
viene chiamata. Questo codice compila, esegue e stampa:

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

Successivamente, nel Listato 13-5, modifichiamo il corpo della chiusura in modo che aggiunga un elemento al
vettore `list`. La closure ora cattura un riferimento mutabile.

<Listing number="13-5" file-name="src/main.rs" caption="Defining and calling a closure that captures a mutable reference">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

</Listing>

This code compiles, runs, and prints:

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

Note that there’s no longer a `println!` between the definition and the call of
the `borrows_mutably` closure: when `borrows_mutably` is defined, it captures a
mutable reference to `list`. We don’t use the closure again after the closure
is called, so the mutable borrow ends. Between the closure definition and the
closure call, an immutable borrow to print isn’t allowed because no other
borrows are allowed when there’s a mutable borrow. Try adding a `println!`
there to see what error message you get!

If you want to force the closure to take ownership of the values it uses in the
environment even though the body of the closure doesn’t strictly need
ownership, you can use the `move` keyword before the parameter list.

This technique is mostly useful when passing a closure to a new thread to move
the data so that it’s owned by the new thread. We’ll discuss threads and why
you would want to use them in detail in Chapter 16 when we talk about
concurrency, but for now, let’s briefly explore spawning a new thread using a
closure that needs the `move` keyword. Listing 13-6 shows Listing 13-4 modified
to print the vector in a new thread rather than in the main thread.

<Listing number="13-6" file-name="src/main.rs" caption="Using `move` to force the closure for the thread to take ownership of `list`">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

</Listing>

We spawn a new thread, giving the thread a closure to run as an argument. The
closure body prints out the list. In Listing 13-4, the closure only captured
`list` using an immutable reference because that's the least amount of access
to `list` needed to print it. In this example, even though the closure body
still only needs an immutable reference, we need to specify that `list` should
be moved into the closure by putting the `move` keyword at the beginning of the
closure definition. If the main thread performed more operations before calling
`join` on the new thread, the new thread might finish before the rest of the
main thread finishes, or the main thread might finish first. If the main thread
maintained ownership of `list` but ended before the new thread and drops
`list`, the immutable reference in the thread would be invalid. Therefore, the
compiler requires that `list` be moved into the closure given to the new thread
so the reference will be valid. Try removing the `move` keyword or using `list`
in the main thread after the closure is defined to see what compiler errors you
get!

<!-- Old headings. Do not remove or links may break. -->

<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>
<a id="moving-captured-values-out-of-the-closure-and-the-fn-traits"></a>

### Moving Captured Values Out of Closures and the `Fn` Traits

Once a closure has captured a reference or captured ownership of a value from
the environment where the closure is defined (thus affecting what, if anything,
is moved _into_ the closure), the code in the body of the closure defines what
happens to the references or values when the closure is evaluated later (thus
affecting what, if anything, is moved _out of_ the closure).

A closure body can do any of the following: move a captured value out of the
closure, mutate the captured value, neither move nor mutate the value, or
capture nothing from the environment to begin with.

The way a closure captures and handles values from the environment affects
which traits the closure implements, and traits are how functions and structs
can specify what kinds of closures they can use. Closures will automatically
implement one, two, or all three of these `Fn` traits, in an additive fashion,
depending on how the closure’s body handles the values:

* `FnOnce` applies to closures that can be called once. All closures implement
  at least this trait because all closures can be called. A closure that moves
  captured values out of its body will only implement `FnOnce` and none of the
  other `Fn` traits because it can only be called once.
* `FnMut` applies to closures that don’t move captured values out of their
  body, but that might mutate the captured values. These closures can be
  called more than once.
* `Fn` applies to closures that don’t move captured values out of their body
  and that don’t mutate captured values, as well as closures that capture
  nothing from their environment. These closures can be called more than once
  without mutating their environment, which is important in cases such as
  calling a closure multiple times concurrently.

Let’s look at the definition of the `unwrap_or_else` method on `Option<T>` that
we used in Listing 13-1:

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

Recall that `T` is the generic type representing the type of the value in the
`Some` variant of an `Option`. That type `T` is also the return type of the
`unwrap_or_else` function: code that calls `unwrap_or_else` on an
`Option<String>`, for example, will get a `String`.

Next, notice that the `unwrap_or_else` function has the additional generic type
parameter `F`. The `F` type is the type of the parameter named `f`, which is
the closure we provide when calling `unwrap_or_else`.

The trait bound specified on the generic type `F` is `FnOnce() -> T`, which
means `F` must be able to be called once, take no arguments, and return a `T`.
Using `FnOnce` in the trait bound expresses the constraint that
`unwrap_or_else` is only going to call `f` at most one time. In the body of
`unwrap_or_else`, we can see that if the `Option` is `Some`, `f` won’t be
called. If the `Option` is `None`, `f` will be called once. Because all
closures implement `FnOnce`, `unwrap_or_else` accepts all three kinds of
closures and is as flexible as it can be.

> Note: If what we want to do doesn’t require capturing a value from the
> environment, we can use the name of a function rather than a closure where we
> need something that implements one of the `Fn` traits. For example, on an
> `Option<Vec<T>>` value, we could call `unwrap_or_else(Vec::new)` to get a
> new, empty vector if the value is `None`. The compiler automatically
> implements whichever of the `Fn` traits is applicable for a function
> definition.

Now let’s look at the standard library method `sort_by_key`, defined on slices,
to see how that differs from `unwrap_or_else` and why `sort_by_key` uses
`FnMut` instead of `FnOnce` for the trait bound. The closure gets one argument
in the form of a reference to the current item in the slice being considered,
and returns a value of type `K` that can be ordered. This function is useful
when you want to sort a slice by a particular attribute of each item. In
Listing 13-7, we have a list of `Rectangle` instances and we use `sort_by_key`
to order them by their `width` attribute from low to high.

<Listing number="13-7" file-name="src/main.rs" caption="Using `sort_by_key` to order rectangles by width">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

</Listing>

This code prints:

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

The reason `sort_by_key` is defined to take an `FnMut` closure is that it calls
the closure multiple times: once for each item in the slice. The closure `|r|
r.width` doesn’t capture, mutate, or move anything out from its environment, so
it meets the trait bound requirements.

In contrast, Listing 13-8 shows an example of a closure that implements just
the `FnOnce` trait, because it moves a value out of the environment. The
compiler won’t let us use this closure with `sort_by_key`.

<Listing number="13-8" file-name="src/main.rs" caption="Attempting to use an `FnOnce` closure with `sort_by_key`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

</Listing>

This is a contrived, convoluted way (that doesn’t work) to try and count the
number of times `sort_by_key` calls the closure when sorting `list`. This code
attempts to do this counting by pushing `value`—a `String` from the closure’s
environment—into the `sort_operations` vector. The closure captures `value` and
then moves `value` out of the closure by transferring ownership of `value` to
the `sort_operations` vector. This closure can be called once; trying to call
it a second time wouldn’t work because `value` would no longer be in the
environment to be pushed into `sort_operations` again! Therefore, this closure
only implements `FnOnce`. When we try to compile this code, we get this error
that `value` can’t be moved out of the closure because the closure must
implement `FnMut`:

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

The error points to the line in the closure body that moves `value` out of the
environment. To fix this, we need to change the closure body so that it doesn’t
move values out of the environment. Keeping a counter in the environment and
incrementing its value in the closure body is a more straightforward way to
count the number of times the closure is called. The closure in Listing 13-9
works with `sort_by_key` because it is only capturing a mutable reference to the
`num_sort_operations` counter and can therefore be called more than once:

<Listing number="13-9" file-name="src/main.rs" caption="Using an `FnMut` closure with `sort_by_key` is allowed">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

</Listing>

The `Fn` traits are important when defining or using functions or types that
make use of closures. In the next section, we’ll discuss iterators. Many
iterator methods take closure arguments, so keep these closure details in mind
as we continue!

[unwrap-or-else]: https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.unwrap_or_else
