## Funzioni e Chiusure Avanzate 

Questa sezione esplora alcune funzionalità avanzate legate a funzioni e
chiusure, inclusi i puntatori a funzione e il ritorno di chiusure.

### Puntatori a Funzione

Abbiamo parlato di come passare chiusure alle funzioni; puoi anche passare
funzioni normali a funzioni! Questa tecnica è utile quando vuoi passare una
funzione che hai già definito piuttosto che definire una nuova chiusura. Le
funzioni si convertono automaticamente al _type_ `fn` (con la _f_ minuscola), da
non confondere con il _trait_ `Fn` delle chiusure. Il _type_ `fn` è chiamato
_puntatore a funzione_. Passare funzioni con puntatori a funzione ti permette di
utilizzare funzioni come argomenti per altre funzioni.

La sintassi per specificare che un parametro è un puntatore a funzione è simile
a quella delle chiusure, come mostrato nel Listato 20-28, dove abbiamo definito
una funzione `più_uno` che aggiunge 1 al suo parametro. La funzione `due_volte`
prende due parametri: un puntatore a funzione verso qualsiasi funzione che
prende un parametro `i32` e ritorna un `i32`, e un valore `i32`. La funzione
`due_volte` chiama la funzione `f` due volte, passando il valore `arg`, poi
somma i risultati delle due chiamate. La funzione `main` chiama `due_volte` con
gli argomenti `più_uno` e `5`.

<Listing number="20-28" file-name="src/main.rs" caption="Uso del _type_ `fn` per accettare un puntatore a funzione come argomento">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-28/src/main.rs}}
```

</Listing>

Questo codice stampa `La risposta è: 12`. Specifichiamo che il parametro `f` in
`due_volte` è un `fn` che prende un parametro di _type_ `i32` e ritorna un
`i32`. Possiamo quindi chiamare `f` nel corpo di `due_volte`. In `main`,
possiamo passare il nome della funzione `più_uno` come primo argomento a
`due_volte`.

A differenza delle chiusure, `fn` è un _type_, non un _trait_, quindi
specifichiamo `fn` come _type_ del parametro direttamente piuttosto che
dichiarare un _type_ generico con uno dei _trait_ `Fn` come vincolo.

I puntatori a funzione implementano tutti e tre i _trait_ delle chiusure (`Fn`,
`FnMut` e `FnOnce`), il che significa che puoi sempre passare un puntatore a
funzione come argomento a una funzione che si aspetta una chiusura. È meglio
scrivere funzioni usando un _type_ generico e uno dei _trait_ delle chiusure
così che le tue funzioni possano accettare sia funzioni che chiusure.

Detto questo, un esempio in cui potresti voler accettare solo `fn` e non
chiusure è quando ti interfacci con codice esterno che non ha chiusure: le
funzioni in C possono accettare funzioni come argomenti, ma C non ha chiusure.

Come esempio di dove potresti usare sia una chiusura definita che una funzione
nominata, diamo un’occhiata all’uso del metodo `map` fornito dal _trait_
`Iterator` nella libreria standard. Per usare il metodo `map` per trasformare un
vettore di numeri in un vettore di stringhe, potremmo usare una chiusura, come
nel Listato 20-29.

<Listing number="20-29" caption="Uso di una chiusura con il metodo `map` per convertire numeri in stringhe">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-29/src/main.rs:here}}
```

</Listing>

Oppure potremmo nominare una funzione come argomento di `map` al posto della
chiusura. Il Listato 20-30 mostra come sarebbe.

<Listing number="20-30" caption="Uso della funzione `String::to_string` con il metodo `map` per convertire numeri in stringhe">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-30/src/main.rs:here}}
```

</Listing>

Nota che dobbiamo usare la sintassi completamente qualificata di cui abbiamo
parlato in [“_Trait_ Avanzati”][advanced-traits]<!-- ignore --> perché ci sono
più funzioni con nome `to_string`.

Qui usiamo la funzione `to_string` definita nel _trait_ `ToString`, che la
libreria standard ha implementato per ogni _type_ che implementa `Display`.

Ricorda dalla sezione [“Valori di _Enum_”][enum-values]<!-- ignore --> del
Capitolo 6 che il nome di ogni variante _enum_ diventa anche una funzione
inizializzatrice. Possiamo usare queste funzioni inizializzatrici come puntatori
a funzione che implementano i _trait_ delle chiusure, il che significa che
possiamo specificare le funzioni inizializzatrici come argomenti per metodi che
accettano chiusure, come nel Listato 20-31.

<Listing number="20-31" caption="Uso di una funzione inizializzatrice di un _enum_ con il metodo `map` per creare un’istanza `Stato` dai numeri">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-31/src/main.rs:here}}
```

</Listing>

Qui creiamo istanze di `Stato::Valore` usando ogni valore `u32` nell’intervallo
su cui è chiamato `map`, usando la funzione inizializzatrice di `Stato::Valore`.
Alcuni preferiscono questo stile, altri preferiscono usare chiusure. Entrambi i
metodi si compilano allo stesso modo, quindi usa quello che trovi più chiaro.

### Restituire Chiusure

Le chiusure sono rappresentate da _trait_, il che significa che non puoi
restituire direttamente una chiusura. Nella maggior parte dei casi in cui
potresti voler ritornare un _trait_, puoi invece usare il _type_ concreto che
implementa il _trait_ come _type_ di ritorno della funzione. Tuttavia, di solito
non puoi fare questo con le chiusure perché non hanno un _type_ concreto
restituibile; per esempio, non puoi usare il puntatore a funzione `fn` come
_type_ di ritorno se la chiusura cattura qualche valore dal suo _scope_.

Al contrario, normalmente userai la sintassi `impl Trait` che abbiamo imparato
nel Capitolo 10. Puoi restituire qualsiasi tipo di funzione usando `Fn`,
`FnOnce` e `FnMut`. Per esempio, il codice nel Listato 20-32 verrà compilato
senza problemi.

<Listing number="20-32" caption="Restituire una chiusura da una funzione usando la sintassi `impl Trait`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-32/src/lib.rs}}
```

</Listing>

Tuttavia, come abbiamo notato nella sezione [“Inferenza e Annotazione del _Type_
delle Chiusure”][closure-types]<!-- ignore --> del Capitolo 13, ogni chiusura è
anche il suo _type_ distinto. Se ti serve lavorare con più funzioni che hanno la
stessa firma ma implementazioni diverse, dovrai usare un oggetto _trait_ per
loro. Considera cosa succede se scrivi un codice come nel Listato 20-33.

<Listing number="20-33" file-name="src/main.rs" caption="Creazione di un `Vec<T>` di chiusure definite tramite funzioni che restituiscono _type_ `impl Fn`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-33/src/main.rs}}
```

</Listing>

Qui abbiamo due funzioni, `ritorna_chiusura` e `ritorna_chiusura_inizializzata`,
che entrambe ritornano `impl Fn(i32) -> i32`. Nota che le chiusure restituite
sono diverse anche se implementano lo stesso _type_. Se provi a compilare, Rust
ti dice che non funziona:

```text
{{#include ../listings/ch20-advanced-features/listing-20-33/output.txt}}
```

Il messaggio di errore dice che ogni volta che ritorni un `impl Trait`, Rust
crea un _type_ _opaco_ univoco, un _type_ di cui non possiamo conoscere i
dettagli di come Rust l’ha costruito né sapere il _type_ generato. Quindi anche
se queste funzioni ritornano chiusure che implementano lo stesso _trait_, i
_type_ opachi che Rust genera sono diversi. (Questo è simile a come Rust genera
_type_ concreti distinti per blocchi _async_ diversi anche se hanno lo stesso
_type_ di output, come abbiamo visto nella sezione [“Il _Type_ `Pin` e il
_Trait_ `Unpin`”][future-types]<!-- ignore --> del Capitolo 17.) Abbiamo già
visto una soluzione a questo problema: possiamo usare un oggetto _trait_, come
nel Listato 20-34.

<Listing number="20-34" caption="Creazione di un `Vec<T>` di chiusure definite tramite funzioni che ritornano `Box<dyn Fn>`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-34/src/main.rs:here}}
```

</Listing>

Questo codice si compila correttamente. Per più informazioni sugli oggetti
_trait_, vedi la sezione [“Usare gli Oggetti _Trait_ per Astrarre Comportamenti
Condivisi”][using-trait-objects]<!-- ignore --> del Capitolo 18.

Passiamo ora a vedere le macro!

[advanced-traits]: ch20-02-advanced-traits.html#trait-avanzati
[enum-values]: ch06-01-defining-an-enum.html#valori-di-enum
[closure-types]: ch13-01-closures.html#inferenza-e-annotazione-del-type-delle-chiusure
[future-types]: ch17-05-traits-for-async.html#il-type-pin-e-il-trait-unpin
[using-trait-objects]: ch18-02-trait-objects.html#usare-gli-oggetti-trait-per-astrarre-comportamenti-condivisi
