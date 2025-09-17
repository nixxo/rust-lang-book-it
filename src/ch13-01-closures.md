<!-- Old heading. Do not remove or links may break. -->
<a id="closures-anonymous-functions-that-can-capture-their-environment"></a>

## Chiusure: Funzioni Anonime che Catturano il Loro Ambiente

Le chiusure di Rust sono funzioni anonime che è possibile salvare in una
variabile o passare come argomenti ad altre funzioni. È possibile creare la
chiusura in un punto e poi chiamarla altrove per valutarla in un contesto
diverso. A differenza delle funzioni, le chiusure possono catturare valori dallo
_scope_ in cui sono definite. Dimostreremo come queste funzionalità di chiusura
consentano il riutilizzo del codice e la personalizzazione del comportamento.

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-an-abstraction-of-behavior-with-closures"></a>
<a id="refactoring-using-functions"></a>
<a id="refactoring-with-closures-to-store-code"></a>

### Catturare l'Ambiente con le Chiusure

Esamineremo innanzitutto come possiamo utilizzare le chiusure per catturare
valori dall'ambiente in cui sono definite per un uso successivo. Ecco lo
scenario: ogni tanto, la nostra azienda di magliette regala una maglietta
esclusiva in edizione limitata a qualcuno nella nostra _mailing list_ come
promozione. Gli utenti della _mailing list_ possono facoltativamente aggiungere
il loro colore preferito al proprio profilo. Se la persona a cui viene assegnata
una maglietta gratuita ha impostato il suo colore preferito, riceverà la
maglietta di quel colore. Se la persona non ha specificato un colore preferito,
riceverà il colore di cui l'azienda ha attualmente la maggiore disponibilità.

Ci sono molti modi per implementarlo. Per questo esempio, useremo un'_enum_
chiamata `ColoreMaglietta` che ha le varianti `Rosso` e `Blu` (limitando il
numero di colori disponibili per semplicità). Rappresentiamo l'inventario
dell'azienda con una struttura `Inventario` che ha un campo denominato
`magliette` che contiene un `Vec<ColoreMaglietta>` che rappresenta i colori
delle magliette attualmente disponibili in magazzino. Il metodo `regalo`
definito su `Inventario` ottiene la preferenza opzionale per il colore della
maglietta del vincitore della maglietta gratuita e restituisce il colore della
maglietta che la persona riceverà. Questa configurazione è mostrata nel Listato
13-1.

<Listing number="13-1" file-name="src/main.rs" caption="Situazione di un'azienda di magliette che deve fare un regalo">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

</Listing>

Il `negozio` definito in `main` ha due magliette blu e una rossa rimanenti da
distribuire per questa promozione in edizione limitata. Chiamiamo il metodo
`regalo` per un utente con preferenza per una maglietta rossa e un utente senza
alcuna preferenza.

Anche in questo caso, questo codice potrebbe essere implementato in molti modi
e, per concentrarci sulle chiusure, ci siamo attenuti ai concetti che avete già
imparato, ad eccezione del corpo del metodo `regalo` che utilizza una chiusura.
Nel metodo `regalo`, otteniamo la preferenza dell'utente come parametro di
_type_ `Option<ColoreMaglietta>` e chiamiamo il metodo `unwrap_or_else` su
`preferenza_utente`. Il metodo [`unwrap_or_else` su
`Option<T>`][unwrap-or-else]<!-- ignore --> è definito dalla libreria standard.
Accetta un argomento: una chiusura senza argomenti che restituisce un valore `T`
(lo stesso _type_ memorizzato nella variante `Some` di `Option<T>`, in questo
caso `ColoreMaglietta`). Se `Option<T>` è la variante `Some`, `unwrap_or_else`
restituisce il valore presente all'interno di `Some`. Se `Option<T>` è la
variante `None` , `unwrap_or_else` chiama la chiusura e restituisce il valore
restituito dalla chiusura.

Specifichiamo l'espressione di chiusura `|| self.most_stocked()` come argomento
di `unwrap_or_else`. Questa è una chiusura che non accetta parametri (se la
chiusura avesse parametri, questi apparirebbero tra le due barre verticali). Il
corpo della chiusura chiama `self.most_stocked()`. Stiamo definendo la chiusura
qui, e l'implementazione di `unwrap_or_else` valuterà la chiusura in seguito, se
il risultato è necessario.

L'esecuzione di questo codice stampa quanto segue:

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

Un aspetto interessante è che abbiamo passato una chiusura che chiama
`self.maggior_stock()` sull'istanza corrente di `Inventario`. La libreria
standard non aveva bisogno di sapere nulla sui _type_ `Inventario` o
`ColoreMaglietta` che abbiamo definito, né sulla logica che vogliamo utilizzare
in questo scenario. La chiusura cattura un _reference_ immutabile all'istanza
`self` di `Inventario` e lo passa con il codice che specifichiamo al metodo
`unwrap_or_else`. Le funzioni, d'altra parte, non sono in grado di catturare il
loro ambiente in questo modo.

### Inferenza e Annotazione del _Type_ Delle Chiusure

Esistono ulteriori differenze tra funzioni e chiusure. Le chiusure di solito non
richiedono di annotare i _type_ dei parametri o dei valori di ritorno, come
fanno le funzioni `fn`. Le annotazioni del _type_ sono necessarie sulle funzioni
perché i _type_ fanno parte di un'interfaccia esplicita esposta agli utenti.
Definire rigidamente questa interfaccia è importante per garantire che tutti
concordino sui tipi di valori che una funzione utilizza e restituisce. Le
chiusure, d'altra parte, non vengono utilizzate in un'interfaccia esposta come
questa: vengono memorizzate in variabili e utilizzate senza denominarle ed
esporle agli utenti della nostra libreria.

Le chiusure sono in genere brevi e rilevanti solo in un contesto ristretto,
piuttosto che in uno scenario arbitrario. In questi contesti limitati, il
compilatore può dedurre i _type_ dei parametri e il _type_ restituito, in modo
simile a come è in grado di dedurre i _type_ della maggior parte delle variabili
(ci sono rari casi in cui il compilatore necessita di annotazioni del _type_
anche per le chiusure).

Come per le variabili, possiamo aggiungere annotazioni del _type_ se vogliamo
aumentare l'esplicitezza e la chiarezza, a costo di essere più prolissi del
necessario. L'annotazione dei _type_ per una chiusura sarebbe simile alla
definizione mostrata nel Listato 13-2. In questo esempio, definiamo una chiusura
e la memorizziamo in una variabile, anziché definirla nel punto in cui la
passiamo come argomento, come abbiamo fatto nel Listato 13-1.

<Listing number="13-2" file-name="src/main.rs" caption="Aggiunta di annotazioni facoltative dei _type_ di parametro e valore di ritorno nella chiusura">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-02/src/main.rs:here}}
```

</Listing>

Con l'aggiunta delle annotazioni del _type_, la sintassi delle chiusure appare
più simile alla sintassi delle funzioni. Qui, per confronto, definiamo una
funzione che aggiunge 1 al suo parametro e una chiusura che ha lo stesso
comportamento. Abbiamo aggiunto alcuni spazi per allineare le parti rilevanti.
Questo illustra come la sintassi delle chiusure sia simile a quella delle
funzioni, fatta eccezione per l'uso delle barre verticali e per la quantità di
sintassi che è facoltativa:

```rust,ignore
fn  agg_uno_v1   (x: u32) -> u32 { x + 1 }
let agg_uno_v2 = |x: u32| -> u32 { x + 1 };
let agg_uno_v3 = |x|             { x + 1 };
let agg_uno_v4 = |x|               x + 1  ;
```

La prima riga mostra una definizione di funzione e la seconda una definizione di
chiusura completamente annotata. Nella terza riga, rimuoviamo le annotazioni del
_type_ dalla definizione di chiusura. Nella quarta riga, rimuoviamo le
parentesi, che sono facoltative perché il corpo della chiusura ha una sola
espressione. Queste sono tutte definizioni valide che produrranno lo stesso
comportamento quando vengono chiamate. Le righe `agg_uno_v3` e `agg_uno_v4`
richiedono che le chiusure vengano valutate per essere compilabili, poiché i
_type_ verranno dedotti dal loro utilizzo. Questo è simile a `let v =
Vec::new();` che richiede annotazioni del _type_ o valori di qualche tipo da
inserire in `Vec` affinché Rust possa dedurne il _type_.

Per le definizioni delle chiusure, il compilatore dedurrà un _type_ concreto per ciascuno dei
loro parametri e per il loro valore di ritorno. Ad esempio, il Listato 13-3 mostra
la definizione di una chiusura breve che restituisce semplicemente il valore ricevuto come
parametro. Questa chiusura non è molto utile, se non per gli scopi di questo
esempio. Nota che non abbiamo aggiunto alcuna annotazione del _type_ alla definizione.
Poiché non ci sono annotazioni, possiamo chiamare la chiusura con qualsiasi _type_,
come abbiamo fatto qui con `String` la prima volta. Se poi proviamo a chiamare
`esempio_chiusura` con un intero, otterremo un errore.

<Listing number="13-3" file-name="src/main.rs" caption="Tentativo di chiamare una chiusura i cui _type_ sono inferiti con due _type_ diversi">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-03/src/main.rs:here}}
```

</Listing>

Il compilatore ci dà questo errore:

```console
{{#include ../listings/ch13-functional-features/listing-13-03/output.txt}}
```

La prima volta che chiamiamo `esempio_chiusura` con il valore `String`, il
compilatore deduce che il _type_ di `x` e il _type_ di ritorno della chiusura
siano `String`. Questi _type_ vengono quindi bloccati nella chiusura in
`esempio_chiusura` e si verifica un errore di _type_ quando si tenta nuovamente
di utilizzare un _type_ diverso con la stessa chiusura.

### Catturare i _Reference_ o Trasferire la _Ownership_

Le chiusure possono catturare valori dal loro ambiente in tre modi, che
corrispondono direttamente ai tre modi in cui una funzione può accettare un
parametro: un prestito immutabile, un prestito mutabile o prendendo la
_ownership_. La chiusura deciderà quale di questi utilizzare in base a ciò che
il corpo della funzione fa con i valori catturati.

Nel Listato 13-4, definiamo una chiusura che cattura un _reference_ immutabile al
vettore denominato `lista` perché necessita solo di un riferimento immutabile per stampare
il valore.

<Listing number="13-4" file-name="src/main.rs" caption="Definizione e chiamata di una chiusura che cattura un _reference_ immutabile">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

</Listing>

Questo esempio illustra anche che una variabile può essere associata a una
definizione di chiusura, e che possiamo successivamente chiamare la chiusura
utilizzando il nome della variabile e le parentesi come se il nome della
variabile fosse il nome di una funzione.

Poiché possiamo avere più _reference_ immutabili a `lista` contemporaneamente,
`lista` è comunque accessibile dal codice prima della definizione della
chiusura, dopo la definizione della chiusura ma prima che la chiusura venga
chiamata, e dopo che la chiusura viene chiamata. Questo codice si compila,
esegue e stampa:

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

Successivamente, nel Listato 13-5, modifichiamo il corpo della chiusura in modo
che aggiunga un elemento al vettore `list`. La chiusura ora cattura un
_reference_ mutabile.

<Listing number="13-5" file-name="src/main.rs" caption="Definizione e chiamata di una chiusura che cattura un _reference_ mutabile">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

</Listing>

Questo codice si compila, esegue e stampa:

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

Nota che non c'è più `println!` tra la definizione e la chiamata della chiusura
`prestito_mutabile`: quando `prestito_mutabile` è definita, cattura un
_reference_ mutabile a `lista`. Non usiamo più la chiusura dopo che è stata
chiamata, quindi il prestito mutabile termina. Tra la definizione della chiusura
e la chiamata alla chiusura, non è consentito un prestito immutabile per
stampare perché, quando c'è un prestito mutabile, non sono consentiti altri
prestiti. Prova ad aggiungere `println!` per vedere quale messaggio di errore
ottieni!

Se vuoi forzare la chiusura ad assumere la _ownership_ dei valori che usa
nell'ambiente, anche se il corpo della chiusura non ne ha strettamente bisogno,
puoi usare la parola chiave `move` prima dell'elenco dei parametri.

Questa tecnica è utile soprattutto quando si passa una chiusura a un nuovo
_thread_ per spostare i dati in modo che siano di proprietà del nuovo _thread_.
Discuteremo i _thread_ e perché dovreste utilizzarli in dettaglio nel Capitolo
16, quando parleremo di concorrenza, ma per ora, esploriamo brevemente la
creazione di un nuovo _thread_ utilizzando una chiusura che richiede la parola
chiave `move`. Il Listato 13-6 mostra il Listato 13-4 modificato per stampare il
vettore in un nuovo _thread_ anziché nel _thread_ principale.

<Listing number="13-6" file-name="src/main.rs" caption="Utilizzo di `move` per forzare la chiusura affinché il _thread_ prenda la _ownership_ di `lista`">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

</Listing>

Generiamo un nuovo _thread_, assegnandogli una chiusura da eseguire come
argomento. Il corpo della chiusura stampa la lista. Nel Listato 13-4, la
chiusura catturava solo `lista` utilizzando un _reference_ immutabile, perché
questo rappresenta il minimo accesso a `lista` necessario per stamparlo. In
questo esempio, anche se il corpo della chiusura richiede ancora solo un
_reference_ immutabile, dobbiamo specificare che `lista` debba essere spostato
nella chiusura inserendo la parola chiave `move` all'inizio della definizione
della chiusura. Se il _thread_ principale eseguisse più operazioni prima di
chiamare `join` sul nuovo _thread_, il nuovo _thread_ potrebbe terminare prima
del _thread_ principale, oppure il _thread_ principale potrebbe terminare per
primo. Se il _thread_ principale mantenesse la _ownership_ di `lista` ma
terminasse prima del nuovo _thread_ e liberasse la memoria di `lista`, il
_reference_ immutabile nel _thread_ non sarebbe valido. Pertanto, il compilatore
richiede che `lista` venga spostato nella chiusura assegnata al nuovo _thread_,
affinché il _reference_ sia valido. Prova a rimuovere la parola chiave `move` o
a utilizzare `lista` nel _thread_ principale dopo la definizione della chiusura
per vedere quali errori del compilatore ottieni!

<!-- Old headings. Do not remove or links may break. -->
<a id="storing-closures-using-generic-parameters-and-the-fn-traits"></a>
<a id="limitations-of-the-cacher-implementation"></a>
<a id="moving-captured-values-out-of-the-closure-and-the-fn-traits"></a>

### Restituire i Valori Catturati dalle Chiusure e i _Trait_ `Fn`

Una volta che una chiusura ha catturato un _reference_ o preso la _ownership_ di
un valore nell'ambiente in cui è definita (influenzando quindi cosa, se
presente, viene spostato _all'interno_ della chiusura), il codice nel corpo
della chiusura definisce cosa succede ai _reference_ o ai valori quando la
chiusura viene valutata in seguito (influenzando quindi cosa, se presente, viene
spostato _fuori_ dalla chiusura).

Il corpo di una chiusura può eseguire una delle seguenti operazioni: spostare un
valore catturato fuori dalla chiusura, mutare il valore catturato, non spostare
né mutare il valore, oppure non catturare nulla dall'ambiente fin dall'inizio.

Il modo in cui una chiusura cattura e gestisce i valori dell'ambiente influenza
quali _trait_ implementa la chiusura, e i _trait_ sono il modo in cui funzioni e
_struct_ possono specificare quali tipi di chiusure possono utilizzare. Le
chiusure implementeranno automaticamente uno, due o tutti e tre questi _trait_
`Fn`, in modo additivo, a seconda di come il corpo della chiusura gestisce i
valori:

* `FnOnce` si applica alle chiusure che possono essere chiamate una sola volta.
  Tutte le chiusure implementano almeno questo trait perché tutte le chiusure
  possono essere chiamate. Una chiusura che sposta i valori catturati fuori dal
  suo corpo implementerà solo `FnOnce` e nessuno degli altri tratti `Fn` perché
  può essere chiamata una sola volta.
* `FnMut` si applica alle chiusure che non spostano i valori catturati fuori dal
  loro corpo, ma che potrebbero mutarli. Queste chiusure possono essere chiamate
  più di una volta.
* `Fn` si applica alle chiusure che non spostano i valori catturati fuori dal
  loro corpo e che non mutano i valori catturati, così come alle chiusure che
  non catturano nulla dal loro ambiente. Queste chiusure possono essere chiamate
  più di una volta senza mutare il loro ambiente, il che è importante in casi
  come quando una chiusura viene chiamata più volte contemporaneamente.

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

Ricorda che `T` è il _type_ generico che rappresenta il _type_ del valore nella
variante `Some` di un'`Option`. Quel `T` è anche il _type_ restituito dalla
funzione `unwrap_or_else`: il codice che chiama `unwrap_or_else` su
un'`Option<String>`, ad esempio, otterrà una `String`.

Nota inoltre che la funzione `unwrap_or_else` ha il parametro di _type_ generico
aggiuntivo `F`. `F` è il _type_ del parametro denominato `f`, che è la chiusura
che forniamo quando chiamiamo `unwrap_or_else`.

Il vincolo di _trait_ specificato sul _type_ generico `F` è `FnOnce() -> T`, il
che significa che `F` deve poter essere chiamato una sola volta, non accettare
argomenti e restituire una `T`. L'utilizzo di `FnOnce` nel vincolo del _trait_
esprime il limite che `unwrap_or_else` chiamerà `f` al massimo una volta. Nel
corpo di `unwrap_or_else`, possiamo vedere che se `Option` è `Some`, `f` non
verrà chiamata. Se `Option` è `None`, `f` verrà chiamata una volta. Poiché tutte
le chiusure implementano `FnOnce`, `unwrap_or_else` accetta tutti e tre i tipi
di chiusure ed è il più flessibile possibile.

> Nota: se ciò che vogliamo fare non richiede l'acquisizione di un valore
> dall'ambiente, possiamo usare il nome di una funzione anziché una chiusura
> quando abbiamo bisogno di qualcosa che implementi uno dei _trait_ `Fn`. Ad
> esempio, su un valore `Option<Vec<T>>`, potremmo chiamare
> `unwrap_or_else(Vec::new)` per ottenere un nuovo vettore vuoto se il valore è
> `None`. Il compilatore implementa automaticamente qualsiasi dei _trait_ `Fn`
> applicabile per una definizione di funzione.

Ora diamo un'occhiata al metodo della libreria standard `sort_by_key`, definito
sulle _slice_, per vedere in che modo differisce da `unwrap_or_else` e perché
`sort_by_key` utilizza `FnMut` invece di `FnOnce` come vincolo del _trait_. La
chiusura riceve un argomento sotto forma di _reference_ all'elemento corrente
nella _slice_ in esame e restituisce un valore di _type_ `K` che può essere
ordinato. Questa funzione è utile quando si desidera ordinare una _slice_ in
base a un particolare attributo di ciascun elemento. Nel Listato 13-7, abbiamo
un elenco di istanze di `Rettangolo` e utilizziamo `sort_by_key` per ordinarle
in base al loro attributo `larghezza` dal più basso al più alto.

<Listing number="13-7" file-name="src/main.rs" caption="Utilizzo di `sort_by_key` per ordinare i rettangoli in base alla larghezza">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

</Listing>

Questo codice stampa:

```console
{{#include ../listings/ch13-functional-features/listing-13-07/output.txt}}
```

Il motivo per cui `sort_by_key` è definito per accettare una chiusura `FnMut` è
che chiama la chiusura più volte: una volta per ogni elemento nella _slice_. La
chiusura `|r| r.larghezza` non cattura, modifica o sposta nulla dal suo
ambiente, quindi soddisfa i requisiti del _vincolo_ di _trait_.

Al contrario, il Listato 13-8 mostra un esempio di una chiusura che implementa
solo il _trait_ `FnOnce`, perché sposta un valore fuori dall'ambiente. Il
compilatore non ci permette di usare questa chiusura con `sort_by_key`.

<Listing number="13-8" file-name="src/main.rs" caption="Tentativo di usare una chiusura `FnOnce` con `sort_by_key`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-08/src/main.rs}}
```

</Listing>

Questo è un modo artificioso e contorto (che non funziona) per provare a contare
il numero di volte in cui `sort_by_key` chiama la chiusura durante l'ordinamento
di `lista`. Questo codice tenta di effettuare questo conteggio inserendo
`valore`, una `String` dall'ambiente della chiusura, nel vettore
`azioni_ordinamento`. La chiusura cattura `valore` e quindi sposta `valore`
fuori dalla chiusura trasferendo la _ownership_ di `valore` al vettore
`azioni_ordinamento`. Questa chiusura può essere chiamata una sola volta; prova
a chiamarla una seconda volta non funzionerebbe perché `valore` non sarebbe più
nell'ambiente da inserire nuovamente in `azioni_ordinamento`! Pertanto, questa
chiusura implementa solo `FnOnce`. Quando proviamo a compilare questo codice,
otteniamo questo errore che indica che `valore` non può essere spostato fuori
dalla chiusura perché la chiusura deve implementare `FnMut`:

```console
{{#include ../listings/ch13-functional-features/listing-13-08/output.txt}}
```

L'errore punta alla riga nel corpo della chiusura che sposta `valore` fuori
dall'ambiente. Per risolvere questo problema, dobbiamo modificare il corpo della
chiusura in modo che non sposti valori fuori dall'ambiente. Mantenere un
contatore nell'ambiente e incrementarne il valore nel corpo della chiusura è un
modo più semplice per contare il numero di volte in cui la chiusura viene
chiamata. La chiusura nel Listato 13-9 funziona con `sort_by_key` perché cattura
solo un _reference_ mutabile al contatore `numero_azioni_ordinamento` e può
quindi essere chiamata più volte:

<Listing number="13-9" file-name="src/main.rs" caption="È consentito l'utilizzo di una chiusura `FnMut` con `sort_by_key`">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-09/src/main.rs}}
```

</Listing>

I traits `Fn` sono importanti quando si definiscono o si utilizzano funzioni o
_type_ che fanno uso di chiusure. Nella prossima sezione, parleremo degli
iteratori. Molti metodi iteratori accettano _argomenti chiusura_, quindi tieni a
mente questi dettagli sulle chiusure mentre proseguiamo!

[unwrap-or-else]: https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.unwrap_or_else
