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

Ci sono molti modi per implementarlo. Per questo esempio, useremo un'enum chiamata `ShirtColor` che ha le varianti `Red` e `Blue` (limitando il
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

### Cattura di _Reference_ o Trasferimento di _Ownership_

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

Si noti che non c'è più `println!` tra la definizione e la chiamata della
chiusura `prestito_mutabile`: quando `prestito_mutabile` è definita, cattura un
riferimento mutabile a `list`. Non usiamo più la chiusura dopo che
è stata chiamata, quindi il prestito mutabile termina. Tra la definizione della chiusura e la
chiamata alla chiusura, non è consentito un prestito immutabile per stampare perché, quando c'è un prestito mutabile, non sono consentiti altri
prestiti. Prova ad aggiungere `println!`
per vedere quale messaggio di errore ottieni!

Se vuoi forzare la chiusura ad assumere la proprietà dei valori che usa nell'ambito, anche se il corpo della chiusura non ne ha strettamente bisogno, puoi usare la parola chiave `move` prima dell'elenco dei parametri.

Questa tecnica è utile soprattutto quando si passa una chiusura a un nuovo thread per spostare
i dati in modo che siano di proprietà del nuovo thread. Discuteremo i thread e perché
dovreste utilizzarli in dettaglio nel Capitolo 16, quando parleremo di
concorrenza, ma per ora, esploriamo brevemente la creazione di un nuovo thread utilizzando una
chiusura che richiede la parola chiave `move`. Il Listato 13-6 mostra il Listato 13-4 modificato
per stampare il vettore in un nuovo thread anziché nel thread principale.

<Listing number="13-6" file-name="src/main.rs" caption="Utilizzo di `move` per forzare la chiusura affinché il thread assuma la proprietà di `list`">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

</Listing>

Generiamo un nuovo thread, assegnandogli una chiusura da eseguire come argomento.
Il corpo della chiusura stampa la lista. Nel Listato 13-4, la chiusura catturava solo `list` utilizzando un riferimento immutabile, perché questo rappresenta il minimo accesso
a `list` necessario per stamparlo. In questo esempio, anche se il corpo della chiusura
richiede ancora solo un riferimento immutabile, dobbiamo specificare che `list` debba
essere spostato nella chiusura inserendo la parola chiave `move` all'inizio della
definizione della chiusura. Se il thread principale eseguisse più operazioni prima di chiamare
`join` sul nuovo thread, il nuovo thread potrebbe terminare prima del
thread principale, oppure il thread principale potrebbe terminare per primo. Se il thread principale
mantenesse la proprietà di `list` ma terminasse prima del nuovo thread e eliminasse
`list`, il riferimento immutabile nel thread non sarebbe valido. Pertanto, il
compilatore richiede che `list` venga spostato nella chiusura assegnata al nuovo thread,
affinché il riferimento sia valido. Provate a rimuovere la parola chiave `move` o a utilizzare `list`
nel thread principale dopo la definizione della chiusura per vedere quali errori del compilatore
ottenete!

<!-- Old headings. Do not remove or links may break. -->

<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>
<a id="moving-captured-values-out-of-the-closure-and-the-fn-traits"></a>

### Spostamento dei Valori Catturati dalle Chiusure e dai Traits `Fn`

Una volta che una chiusura ha catturato un riferimento o acquisito la proprietà di un valore nell'ambito in cui è definita (influenzando quindi cosa, se presente,
viene spostato _all'interno_ della chiusura), il codice nel corpo della chiusura definisce cosa
succede ai riferimenti o ai valori quando la chiusura viene valutata in seguito (influenzando quindi cosa, se presente, viene spostato _fuori_ dalla chiusura).

Il corpo di una chiusura può eseguire una delle seguenti operazioni: spostare un valore catturato fuori dalla
chiusura, mutare il valore catturato, non spostare né mutare il valore, oppure
non catturare nulla dall'ambiente fin dall'inizio.

Il modo in cui una chiusura cattura e gestisce i valori dell'ambito influenza
quali tratti implementa la chiusura, e i tratti sono il modo in cui funzioni e struct
possono specificare quali tipi di chiusure possono utilizzare. Le chiusure implementeranno automaticamente
uno, due o tutti e tre questi tratti `Fn`, in modo additivo,
a seconda di come il corpo della chiusura gestisce i valori:

* `FnOnce` si applica alle chiusure che possono essere chiamate una sola volta. Tutte le chiusure implementano
almeno questo trait perché tutte le chiusure possono essere chiamate. Una chiusura che sposta
i valori catturati fuori dal suo corpo implementerà solo `FnOnce` e nessuno
degli altri tratti `Fn` perché può essere chiamata una sola volta. * `FnMut` si applica alle chiusure che non spostano i valori catturati fuori dal loro
corpo, ma che potrebbero mutarli. Queste chiusure possono essere
chiamate più di una volta.
* `Fn` si applica alle chiusure che non spostano i valori catturati fuori dal loro corpo
e che non mutano i valori catturati, così come alle chiusure che non catturano
nulla dal loro ambiente. Queste chiusure possono essere chiamate più di una volta
senza mutare il loro ambiente, il che è importante in casi come
quando una chiusura viene chiamata più volte contemporaneamente.

Diamo un'occhiata alla definizione del metodo `unwrap_or_else` su `Option<T>` che
abbiamo usato nel Listato 13-1:

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

Ricordiamo che `T` è il tipo generico che rappresenta il tipo del valore nella variante
`Some` di un'`Option`. Quel tipo `T` è anche il tipo restituito dalla
funzione `unwrap_or_else`: il codice che chiama `unwrap_or_else` su un'
`Option<String>`, ad esempio, otterrà una `String`.

Si noti inoltre che la funzione `unwrap_or_else` ha il parametro di tipo generico aggiuntivo `F`. Il tipo `F` è il tipo del parametro denominato `f`, che è
la chiusura che forniamo quando chiamiamo `unwrap_or_else`.

Il vincolo di trait specificato sul tipo generico `F` è `FnOnce() -> T`, il che
significa che `F` deve poter essere chiamato una sola volta, non accettare argomenti e restituire una `T`. L'utilizzo di `FnOnce` nel vincolo del trait esprime il limite che
`unwrap_or_else` chiamerà `f` al massimo una volta. Nel corpo di
`unwrap_or_else`, possiamo vedere che se `Option` è `Some`, `f` non verrà
chiamata. Se `Option` è `None`, `f` verrà chiamata una volta. Poiché tutte
le chiusure implementano `FnOnce`, `unwrap_or_else` accetta tutti e tre i tipi di
chiusure ed è il più flessibile possibile.

> Nota: se ciò che vogliamo fare non richiede l'acquisizione di un valore dall'
> ambito, possiamo usare il nome di una funzione anziché una chiusura quando
> abbiamo bisogno di qualcosa che implementi uno dei tratti `Fn`. Ad esempio, su un valore
> `Option<Vec<T>>`, potremmo chiamare `unwrap_or_else(Vec::new)` per ottenere un
> nuovo vettore vuoto se il valore è `None`. Il compilatore implementa automaticamente
> qualsiasi dei tratti `Fn` applicabile per una definizione di funzione.

Ora diamo un'occhiata al metodo della libreria standard `sort_by_key`, definito sulle slice,
per vedere in che modo differisce da `unwrap_or_else` e perché `sort_by_key` utilizza
`FnMut` invece di `FnOnce` per il vincolo del trait. La chiusura riceve un argomento
sotto forma di riferimento all'elemento corrente nella slice in esame
e restituisce un valore di tipo `K` che può essere ordinato. Questa funzione è utile
quando si desidera ordinare una slice in base a un particolare attributo di ciascun elemento. Nel
Listato 13-7, abbiamo un elenco di istanze di `Rettangolo` e utilizziamo `sort_by_key`
per ordinarle in base al loro attributo `larghezza` dal più basso al più alto.

<Listing number="13-7" file-name="src/main.rs" caption="Utilizzo di `sort_by_key` per ordinare i rettangoli in base alla larghezza">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

</Listing>

Questo codice stampa:

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

Il motivo per cui `sort_by_key` è definito per accettare una chiusura `FnMut` è che chiama
la chiusura più volte: una volta per ogni elemento nella slice. La chiusura `|r|
r.larghezza` non cattura, modifica o sposta nulla dal suo ambiente, quindi
soddisfa i requisiti di vincolo dei tratti.

Al contrario, il Listato 13-8 mostra un esempio di una chiusura che implementa solo
il trait `FnOnce`, perché sposta un valore fuori dall'ambiente. Il
compilatore non ci permette di usare questa chiusura con `sort_by_key`.

<Listing number="13-8" file-name="src/main.rs" caption="Tentativo di usare una chiusura `FnOnce` con `sort_by_key`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

</Listing>

Questo è un modo artificioso e contorto (che non funziona) per provare a contare
il numero di volte in cui `sort_by_key` chiama la chiusura durante l'ordinamento di `list`. Questo codice
tenta di effettuare questo conteggio inserendo `valore`, una `String` dall'ambiente della chiusura,
nel vettore `operazioni_sort`. La chiusura cattura `valore` e
quindi sposta `valore` fuori dalla chiusura trasferendo la proprietà di `valore` al
vettore `operazioni_sort`. Questa chiusura può essere chiamata una sola volta; provare a chiamarla
una seconda volta non funzionerebbe perché `valore` non sarebbe più nell'ambiente
da inserire nuovamente in `operazioni_sort`! Pertanto, questa chiusura
implementa solo `FnOnce`. Quando proviamo a compilare questo codice, otteniamo questo errore
che indica che `valore` non può essere spostato fuori dalla chiusura perché la chiusura deve
implementare `FnMut`:

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

L'errore punta alla riga nel corpo della chiusura che sposta `valore` fuori dall'
ambiente. Per risolvere questo problema, dobbiamo modificare il corpo della chiusura in modo che non
sposti values fuori dall'ambiente. Mantenere un contatore nell'ambiente e
incrementarne il valore nel corpo della chiusura è un modo più semplice per
contare il numero di volte in cui la chiusura viene chiamata. La chiusura nel Listato 13-9
funziona con `sort_by_key` perché cattura solo un riferimento mutabile al contatore
`numero_operazioni_sort` e può quindi essere chiamata più volte:

<Listing number="13-9" file-name="src/main.rs" caption="È consentito l'utilizzo di una chiusura `FnMut` con `sort_by_key`">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

</Listing>

I traits `Fn` sono importanti quando si definiscono o si utilizzano funzioni o tipi che
fanno uso di chiusure. Nella prossima sezione, parleremo degli iteratori. Molti
metodi iteratori accettano argomenti di chiusura, quindi tenete a mente questi dettagli sulle chiusure
mentre proseguiamo!

[unwrap-or-else]: https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.unwrap_or_else
