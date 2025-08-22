## Definire un _Enum_

Laddove le _struct_ ti danno un modo per raggruppare campi e dati correlati, per
esempio un `Rettangolo` con i propri `larghezza` e `altezza`, gli _enum_ ti
danno un modo per indicare che un valore è uno di un insieme possibile di
valori. Per esempio, potremmo voler dire che `Rettangolo` è una delle possibili
forme che include anche `Cerchio` e `Triangolo`. Per farlo, Rust ci permette di
codificare queste possibilità come un _enum_.

Esaminiamo una situazione che potremmo voler esprimere nel codice e vediamo
perché gli _enum_ sono utili e più appropriati delle _struct_ in questo caso.
Supponiamo di dover lavorare con gli indirizzi IP. Attualmente, per gli
indirizzi IP si usano due standard principali: versione quattro e versione sei.
Poiché queste sono le uniche possibilità di indirizzo IP che il nostro programma
incontrerà, possiamo _enumerare_ tutte le varianti possibili, da cui il nome
_enum_.

Qualsiasi indirizzo IP può essere o versione quattro o versione sei, ma non
entrambi contemporaneamente. Questa proprietà degli indirizii IP rende la
struttura dati _enum_ appropriata perché un valore di _enum_ può essere solo una
delle sue varianti. Sia i versione quattro sia i versione sei sono comunque
fondamentalmente indirizzi IP, quindi dovrebbero essere trattati come dati dello
stesso _type_ quando il codice andrà a gestire situazioni che si applicano agli
indirizzi IP d'ogni genere.

Possiamo esprimere questo concetto nel codice definendo una _enum_
`VersioneIndirizzoIp` e elencando le possibili tipologie che un indirizzo IP può
essere: `V4` e `V6`. Queste sono le varianti dell’_enum_:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:def}}
```

`VersioneIndirizzoIp` è ora un _type_ di dato personalizzato che possiamo usare
altrove nel nostro codice.

### Valori di _Enum_

Possiamo creare istanze di ciascuna delle due varianti di `VersioneIndirizzoIp`
in questo modo:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:instance}}
```

Nota che le varianti dell’_enum_ sono nel _namespace_ del suo identificatore, e
usiamo il doppio-due punti `::` per separarle. Questo è utile perché ora
entrambi i valori `VersioneIndirizzoIp::V4` e `VersioneIndirizzoIp::V6` sono
dello stesso _type_: `VersioneIndirizzoIp`. Possiamo quindi, per esempio,
definire una funzione che accetta qualsiasi `VersioneIndirizzoIp`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn}}
```

E possiamo chiamare questa funzione con entrambe le varianti:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn_call}}
```

L’uso degli _enum_ ha ulteriori vantaggi. Pensando meglio al nostro _type_ per
gli indirizzi IP, al momento non abbiamo un modo per memorizzare il vero e
proprio indirizzo IP; sappiamo solo di che tipo si tratta. Dato che hai appena
imparato le _struct_ nel Capitolo 5, potresti essere tentato di risolvere questo
problema con le _struct_ come mostrato nel Listato 6-1.

<Listing number="6-1" caption="Memorizzare indirizzo e la variante `VersioneIndirizzoIp` di un indirizzo IP usando una `struct`">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-01/src/main.rs:here}}
```

</Listing>

Qui abbiamo definito una _struct_ `IpAddr` che ha due campi: un campo `tipo` di
_type_ `VersioneIndirizzoIp` (l’_enum_ definito prima) e un campo `indirizzo` di
_type_ `String`. Abbiamo due istanze di questa _struct_. La prima è `home`, e ha
il valore `VersioneIndirizzoIp::V4` come suo `tipo` con l’indirizzo associato
`127.0.0.1`. La seconda istanza è `loopback`. Essa ha l’altra variante di
`VersioneIndirizzoIp` come valore del suo `tipo`, `V6`, e ha associato
l’indirizzo `::1`. Abbiamo usato una _struct_ per raggruppare i valori `tipo` e
`indirizzo`, così la variante è ora associata al valore.

Tuttavia, rappresentare lo stesso concetto usando solo un _enum_ è più conciso:
invece di un _enum_ dentro una _struct_, possiamo mettere i dati direttamente in
ogni variante dell’_enum_. Questa nuova definizione dell’_enum_ `IpAddr` indica
che entrambe le varianti `V4` e `V6` avranno valori `String` associati:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-02-enum-with-data/src/main.rs:here}}
```

Alleghiamo i dati direttamente a ciascuna variante dell’_enum_, quindi non c’è
bisogno di una _struct_ aggiuntiva. Qui è anche più facile vedere un altro
dettaglio di come funzionano gli _enum_: il nome di ciascuna variante che
definiamo diventa anche una funzione che costruisce un’istanza dell’_enum_.
Cioè, `IpAddr::V4()` è una chiamata di funzione che prende un argomento `String`
e ritorna un’istanza del _type_ `IpAddr`. Otteniamo automaticamente questa
funzione costruttrice come risultato della definizione dell’_enum_.

C’è un altro vantaggio nell’usare un _enum_ invece di una _struct_: ogni
variante può avere _type_ e quantità diverse di dati associati. Gli indirizzi
versione quattro, ad esempio, avranno sempre quattro componenti numeriche con
valori tra 0 e 255. Se volessimo memorizzare gli indirizzi `V4` come quattro
valori `u8` ma rappresentare gli indirizzi `V6` come una singola `String`, non
potremmo farlo con un _struct_. Gli _enum_ gestiscono questo caso con facilità:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-03-variants-with-different-data/src/main.rs:here}}
```

Abbiamo mostrato diversi modi per definire strutture dati per memorizzare
indirizzi IP versione quattro e versione sei. Tuttavia, risulta che voler
memorizzare indirizzi IP e codificare di quale tipo siano è così comune che la
[libreria standard fornisce una definizione che possiamo usare!][IpAddr]<!--
ignore --> Diamo un’occhiata a come la libreria standard definisce `IpAddr`: ha
l’esatto _enum_ e le varianti che abbiamo definito e usato, ma incapsula i dati
dell’indirizzo dentro le varianti sotto forma di due diverse _struct_, definite
in modo differente per ciascuna variante:

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Questo codice illustra che puoi mettere qualsiasi tipo di dato dentro una
variante di _enum_: stringhe, _type_ numerici o _struct_, per esempio. Puoi
persino includere un altro _enum_! Inoltre, i _type_ della libreria standard
spesso non sono molto più complicati di quello che potresti creare tu.

Nota che anche se la libreria standard contiene una definizione per `IpAddr`,
possiamo comunque creare e usare la nostra definizione senza conflitti perché
non abbiamo importato la definizione della libreria standard nel nostro _scope_.
Parleremo più avanti dell’importazione dei _type_ nello _scope_ nel Capitolo 7.

Diamo un’occhiata a un altro esempio di _enum_ nel Listato 6-2: questo ha una
grande varietà di _type_ incorporati nelle sue varianti.

<Listing number="6-2" caption="Un _enum_ `Messagio` le cui varianti memorizzano ciascuna quantità e _type_ diversi di valori">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

</Listing>

Questo _enum_ ha quattro varianti con _type_ diversi:

- `Esci`: non ha dati associati
- `Muovi`: ha campi nominati, come fa un _struct_
- `Scrivi`: include una singola `String`
- `CambiaColore`: include tre valori `i32`

Definire un _enum_ con varianti come quelle nel Listato 6-2 è simile a definire
diversi _type_ di _struct_, eccetto che l’_enum_ non usa la parola chiave
`struct` e tutte le varianti sono raggruppate sotto il _type_ `Messaggio`. Le
seguenti _struct_ potrebbero contenere gli stessi dati che le varianti
dell’_enum_ precedente contengono:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-04-structs-similar-to-message-enum/src/main.rs:here}}
```

Ma se usassimo le diverse _struct_, ognuna con il proprio _type_, non potremmo
definire altrettanto facilmente una funzione che accetti uno qualsiasi di questi
_type_ di messaggi come potremmo fare con l’_enum_ `Messaggio` definito nel
Listato 6-2, che è un singolo _type_.

C’è un’ulteriore somiglianza tra _enum_ e _struct_: proprio come possiamo
definire metodi sulle _struct_ usando `impl`, possiamo anche definire metodi
sugli _enum_. Ecco un metodo chiamato `chiama` che potremmo definire sul nostro
_enum_ `Messaggio`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-05-methods-on-enums/src/main.rs:here}}
```

Il corpo del metodo userebbe `self` per ottenere il valore su cui abbiamo
chiamato il metodo. In questo esempio, abbiamo creato una variabile `m` che ha
il valore `Messaggio::Scrivi(String::from("ciao"))`, e quello sarà `self` nel
corpo del metodo `chiama` quando `m.chiama()` viene eseguito.

Diamo un’occhiata a un altro _enum_ nella libreria standard che è molto comune e
utile: `Option`.

### L'_Enum_ `Option` e i Suoi Vantaggi Rispetto ai Valori Nulli

Questa sezione esplora un caso di studio su `Option`, che è un altro _enum_
definito dalla libreria standard. Il _type_ `Option` codifica lo scenario molto
comune in cui un valore può essere qualcosa oppure niente.

Per esempio, se richiedi il primo elemento di una lista non vuota, otterrai un
valore. Se richiedi il primo elemento di una lista vuota, non otterrai niente.
Esprimere questo concetto in termini del sistema dei _type_ permette al
compilatore di verificare se hai gestito tutti i casi che dovresti gestire;
questa funzionalità può prevenire bug estremamente comuni in altri linguaggi di
programmazione.

La progettazione dei linguaggi di programmazione è spesso pensata in termini
delle funzionalità che includi, ma anche le funzionalità che escludi sono
importanti. Rust non prevede l'uso di _null_ che molti altri linguaggi
possiedono. _Null_ è un valore che significa che non c’è alcun valore. Nei
linguaggi con _null_, le variabili possono essere sempre in uno dei due stati:
_null_ o non-_null_.

Nella sua presentazione del 2009 “Null References: The Billion Dollar Mistake”, Tony Hoare, l’inventore del _null_, disse:

> Lo chiamo il mio errore da un miliardo di dollari. All’epoca stavo progettando
> il primo sistema di _type_ completo per i _reference_ in un linguaggio
> orientato agli oggetti. Il mio obiettivo era garantire che ogni uso dei
> _reference_ fosse assolutamente sicuro, con i controlli effettuati
> automaticamente dal compilatore. Ma non ho resistito alla tentazione di
> inserire un _reference_ nullo, semplicemente perché era così facile da
> implementare. Questo ha portato a innumerevoli errori, vulnerabilità e crash
> di sistema, che probabilmente hanno causato un miliardo di dollari di dolore e
> danni negli ultimi quarant'anni.

Il problema con i valori _null_ è che se provi a usare un valore _null_ come se
fosse un valore non-_null_, otterrai un errore di qualche tipo. Poiché questa
proprietà _null_ o no-_null_ è pervasiva, è estremamente facile commettere
questo tipo di errore.

Tuttavia, il concetto che il _null_ cerca di esprimere è ancora utile: _null_ è
un valore che è attualmente invalido o assente per qualche motivo.

Il problema non è veramente il concetto ma l’implementazione. Di conseguenza,
Rust non ha i _null_, ma ha un _enum_ che può codificare il concetto di un
valore presente o assente. Questo _enum_ è `Option<T>`, ed è [definito dalla
libreria standard][option]<!-- ignore --> come segue:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

L’_enum_ `Option<T>` è così utile che è incluso nel _prelude_ (_preludio_ è
l'insieme di funzionalità che Rust include di default in ogni programma fornite
dalla libreria standard); non è necessario portarlo nello _scope_
esplicitamente. Le sue varianti sono anch’esse incluse nel _prelude_: puoi usare
`Some` e `None` direttamente senza il prefisso `Option::`. L’_enum_ `Option<T>`
è comunque un normale _enum_, e `Some(T)` e `None` sono ancora varianti del
_type_ `Option<T>`.

La sintassi `<T>` è una caratteriastica di Rust di cui non abbiamo ancora
parlato. È un parametro di _type_ generico, e tratteremo i _type generici_ più
in dettaglio nel Capitolo 10. Per ora, tutto ciò che devi sapere è che `<T>`
significa che la variante `Some` dell’_enum_ `Option` può contenere un pezzo di
dato di qualsiasi _type_, e che ogni _type_ concreto che viene usato al posto di
`T` rende il _type_ complessivo `Option<T>` un _type_ diverso. Ecco alcuni
esempi di utilizzo di valori `Option` per contenere _type_ numerici e _type_
carattere:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-06-option-examples/src/main.rs:here}}
```

Il _type_ di `un_numero` è `Option<i32>`. Il _type_ di `un_carattere` è
`Option<char>`, che è un _type_ diverso. Rust può inferire questi _type_ perché
abbiamo specificato un valore dentro la variante `Some`. Per `nessun_numero`,
Rust ci richiede di annotare il _type_ di `Option`: il compilatore non può
inferire il _type_ che la variante `Some` corrispondente conterrà guardando solo
al valore `None`. Qui diciamo a Rust che intendiamo che `nessun_numero` sia di
_type_ `Option<i32>`.

Quando abbiamo un valore `Some`, sappiamo che un valore è presente e il valore è
contenuto all’interno di `Some`. Quando abbiamo un valore `None`, in un certo
senso significa la stessa cosa di _null_: non abbiamo un valore valido. Quindi
perché avere `Option<T>` è meglio che avere _null_?

In breve, perché `Option<T>` e `T` (dove `T` può essere qualsiasi _type_) sono
_type_ diversi, il compilatore non ci permetterà di usare un valore `Option<T>`
come se fosse sicuramente un valore valido. Per esempio, questo codice non
compilerà, perché sta cercando di sommare un `Option<i8>` a un `i8`:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/src/main.rs:here}}
```

Se eseguiamo questo codice, otteniamo un messaggio di errore come questo:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/output.txt}}
```

Intenso! In effetti, questo messaggio di errore significa che Rust non capisce
come sommare un `i8` e un `Option<i8>`, perché sono _type_ diversi. Quando
abbiamo un valore di un _type_ come `i8` in Rust, il compilatore deve
assicurarsi che abbiamo sempre un valore valido. Possiamo procedere con fiducia
senza dover controllare il _null_ prima di usare quel valore. Solo quando
abbiamo un `Option<i8>` (o qualunque _type_ di valore con cui stiamo lavorando)
dobbiamo preoccuparci della possibilità di non avere un valore, e il compilatore
ci assicurerà di gestire quel caso prima di usare il valore.

In altre parole, devi convertire un `Option<T>` in un `T` prima di poter
eseguire operazioni su `T`. Generalmente, questo aiuta a catturare uno dei
problemi più comuni del _null_: presumere che qualcosa non sia _null_ quando in
realtà lo è.

Eliminare il rischio di presumere erroneamente un valore non-_null_ ti aiuta a
essere più sicuro nel tuo codice. Per avere un valore che può essere
eventualmente _null_, devi esplicitamente optare per questo facendo sì che il
_type_ di quel valore sia `Option<T>`. Poi, quando usi quel valore, sei
obbligato a gestire esplicitamente il caso in cui il valore sia _null_. Ovunque
un valore abbia un _type_ che non è `Option<T>`, _puoi_ assumere in sicurezza
che il valore non sia _null_. Questa è stata una decisione di design voluta in
Rust per limitare la pervasività del _null_ e aumentare la sicurezza del codice
Rust.

Quindi come si estrae il valore `T` da una variante `Some` quando si ha un
valore di _type_ `Option<T>` in modo da poter usare quel valore? L’_enum_
`Option<T>` ha un gran numero di metodi utili in varie situazioni; puoi
consultarli nella [sua documentazione][docs]<!-- ignore -->. Familiarizzare con
i metodi su `Option<T>` sarà estremamente utile nel tuo percorso con Rust.

In generale, per usare un valore `Option<T>` devi avere codice che gestisca ogni
variante. Vuoi del codice che venga eseguito solo quando hai un valore
`Some(T)`, e quel codice può usare il `T` interno. Vuoi altro codice che venga
eseguito solo se hai un valore `None`, e quel codice non ha un valore `T`
disponibile. L’espressione `match` è una costruzione di controllo del flusso che
fa esattamente questo quando viene usata con gli _enum_: eseguirà codice diverso
a seconda di quale variante dell’_enum_ ha, e quel codice può usare i dati
all’interno del valore che corrisponde.

[IpAddr]: https://doc.rust-lang.org/stable/std/net/enum.IpAddr.html
[option]: https://doc.rust-lang.org/stable/std/option/enum.Option.html
[docs]: https://doc.rust-lang.org/stable/std/option/enum.Option.html
