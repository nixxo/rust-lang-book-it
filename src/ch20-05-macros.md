## Macro

Abbiamo usato macro come `println!` in tutto il libro, ma non abbiamo ancora
esplorato appieno cosa sia una macro e come funzioni. Il termine _macro_ si
riferisce a una famiglia di funzionalità delle macro _dichiarative_ in Rust con
`macro_rules!`, e tre tipi di macro _procedurali_:

- Macro `#[derive]` personalizzate che specificano codice aggiunto con
  l’attributo `derive` usato su _struct_ ed _enum_.
- Macro simil-attributo che definiscono attributi personalizzati usabili su
  qualsiasi elemento.
- Macro simil-funzione che sembrano chiamate di funzione ma operano sui _token_
  specificati come argomento.

Parleremo di ciascuna di queste a turno, ma prima vediamo perché abbiamo bisogno
delle macro se abbiamo già le funzioni.

### Differenza Tra Macro e Funzioni

Fondamentalmente, le macro sono un modo di scrivere codice che scrive altro
codice, noto come _meta-programmazione_. Nell’Appendice C parliamo
dell’attributo `derive`, che genera per te l’implementazione di vari _trait_.
Abbiamo anche usato le macro `println!` e `vec!` in tutto il libro. Tutte queste
macro _espandono_ il codice, producendo più codice di quello scritto
manualmente.

La meta-programmazione è utile per ridurre la quantità di codice da scrivere e
mantenere, che è uno degli scopi delle funzioni. Tuttavia, le macro hanno poteri
aggiuntivi che le funzioni non hanno.

La firma di una funzione deve dichiarare il numero ed il _type_ dei parametri.
Le macro, invece, possono accettare un numero variabile di parametri: possiamo
chiamare `println!("ciao")` con un argomento o `println!("ciao {}", nome)` con
due. Inoltre, le macro sono espanse prima che il compilatore interpreti il
codice, quindi una macro può, ad esempio, implementare un _trait_ su un _type_.
Una funzione non può farlo, perché viene chiamata durante l’esecuzione e i
_trait_ devono essere implementati durante la compilazione.

Lo svantaggio delle macro rispetto alle funzioni è che definire macro è più
complesso, perché stai scrivendo codice Rust che scrive codice Rust. Per questo,
definire macro è generalmente più difficile da leggere, capire e mantenere
rispetto alle funzioni.

Un’altra differenza importante è che devi definire o importare le macro _prima_
di usarle in un file, mentre le funzioni possono essere definite e chiamate
ovunque.

### Macro Dichiarative Per la Meta-Programmazione Generale

La forma di macro più diffusa in Rust è la _macro dichiarativa_. A volte queste
sono anche chiamate "macro per esempio", "macro `macro_rules!`" o semplicemente
"macro". Nel loro nucleo, le macro dichiarative permettono di scrivere qualcosa
di simile a un’espressione `match` di Rust. Come discusso nel Capitolo 6, le
espressioni `match` sono strutture di controllo che prendono un’espressione,
confrontano il valore risultante con dei _pattern_, e quindi eseguono il codice
associato al _pattern_ corrispondente. Le macro confrontano anch’esse un valore
con dei _pattern_ associati a codice particolare: in questa situazione, il
valore è il codice sorgente letterale di Rust passato alla macro; i _pattern_
sono confrontati con la struttura di quel codice sorgente; e il codice associato
a ciascun _pattern_, quando corrisponde, sostituisce il codice passato alla
macro. Tutto ciò accade durante la compilazione.

Per definire una macro, si usa il costrutto `macro_rules!`. Esploriamo come
utilizzare `macro_rules!` osservando come viene definita la macro `vec!`. Il
Capitolo 8 ha trattato come possiamo usare la macro `vec!` per creare un nuovo
vettore con valori particolari. Per esempio, la seguente macro crea un nuovo
vettore contenente tre interi:

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

Potremmo anche usare la macro `vec!` per creare un vettore di due interi o un
vettore di cinque stringhe. Non potremmo usare una funzione per fare lo stesso
perché non sapremmo a priori il numero o il tipo di valori.

Il Listato 20-35 mostra una definizione leggermente semplificata della macro
`vec!`.

<Listing number="20-35" file-name="src/lib.rs" caption="Una versione semplificata della definizione della macro `vec!`">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-35/src/lib.rs}}
```

</Listing>

> Nota: La definizione reale della macro `vec!` nella libreria standard include
> codice per pre-allocare la quantità corretta di memoria anticipatamente. Quel
> codice è un’ottimizzazione che non includiamo qui, per rendere l’esempio più
> semplice.

L’annotazione `#[macro_export]` indica che questa macro deve essere resa
disponibile ogni volta che il _crate_ in cui è definita la macro viene incluso
nello _scope_. Senza questa annotazione, la macro non può essere portata nello
_scope_.

Iniziamo quindi la definizione della macro con `macro_rules!` e il nome della
macro che stiamo definendo _senza_ il punto esclamativo. Il nome, in questo caso
`vec`, è seguito da parentesi graffe che indicano il corpo della definizione
della macro.

La struttura nel corpo di `vec!` è simile alla struttura di un’espressione
`match`. Qui abbiamo un ramo con il _pattern_ `( $( $x:expr ),* )`, seguito da
`=>` e dal blocco di codice associato a questo _pattern_. Se il _pattern_
corrisponde, il blocco di codice associato verrà espanso. Poiché questo è
l’unico _pattern_ in questa macro, c’è solo un modo valido per fare match;
qualsiasi altro _pattern_ porterà a un errore. Macro più complesse avranno più
rami.

La sintassi valida dei _pattern_ nelle definizioni di macro è diversa dalla
sintassi dei _pattern_ trattata nel Capitolo 19 perché i _pattern_ delle macro
vengono confrontati con la struttura del codice Rust piuttosto che con valori.
Vediamo cosa significano le parti del _pattern_ nel Listato 20-35; per la
sintassi completa dei _pattern_ nelle macro, consultare il [Rust
Reference][ref].

Prima usiamo una coppia di parentesi per racchiudere tutto il _pattern_. Usiamo
un segno del dollaro (`$`) per dichiarare una variabile nel sistema macro che
conterrà il codice Rust che corrisponde al _pattern_. Il segno del dollaro rende
chiaro che questa è una variabile macro e non una variabile Rust normale. Poi
viene una coppia di parentesi che cattura i valori che corrispondono al
_pattern_ dentro le parentesi per l’uso nel codice di sostituzione. Dentro `$()`
c’è `$x:expr`, che corrisponde a qualsiasi espressione Rust e assegna il nome
`$x` a quell’espressione.

La virgola che segue `$()` indica che deve apparire un carattere letterale di
separazione virgola tra ogni istanza del codice che corrisponde al codice in
`$()`. L’asterisco `*` specifica che il _pattern_ corrisponde a zero o più
occorrenze di qualunque cosa preceda l’asterisco.

Quando chiamiamo questa macro con `vec![1, 2, 3];`, il _pattern_ `$x` fa match
tre volte con le tre espressioni `1`, `2` e `3`.

Ora vediamo il _pattern_ nel corpo del codice associato a questo ramo:
`temp_vec.push()` dentro `$()*` viene generato per ciascuna parte che
corrisponde a `$()` nel _pattern_ da zero a più volte a seconda di quante volte
il _pattern_ fa match. Il `$x` viene sostituito con ogni espressione trovata.
Quando chiamiamo questa macro con `vec![1, 2, 3];`, il codice generato che
sostituisce questa chiamata di macro sarà il seguente:

```rust,ignore
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

Abbiamo definito una macro che può prendere qualsiasi numero di argomenti di
qualsiasi _type_ e può generare codice per creare un vettore contenente gli
elementi specificati.

Per imparare di più su come scrivere macro, consultare la documentazione online
o altre risorse, come [“The Little Book of Rust Macros”][tlborm] iniziato da
Daniel Keep e continuato da Lukas Wirth.

### Macro Procedurali Per Generare Codice da Attributi

La seconda forma di macro è la macro procedurale, che si comporta più come una
funzione (e è un tipo di procedura). Le _macro procedurali_ accettano del codice
come input, operano su quel codice, e producono del codice come output, invece
di fare _match_ su dei _pattern_ e sostituire il codice con altro codice come
fanno le macro dichiarative. Le tre tipologie di macro procedurali sono `derive`
personalizzate, macro simil-attributi, e macro simil-funzioni, e tutte
funzionano in modo simile.

Quando si creano macro procedurali, le definizioni devono risiedere in un
proprio _crate_ con un tipo speciale di _crate_. Questo per ragioni tecniche
complesse che si spera di eliminare in futuro. Nel Listato 20-36 mostriamo come
definire una macro procedurale, dove `qualche_attributo` è un segnaposto per
l’uso di una specifica varietà di macro.

<Listing number="20-36" file-name="src/lib.rs" caption="Un esempio di definizione di una macro procedurale">

```rust,ignore
use proc_macro::TokenStream;

#[qualche_attributo]
pub fn qualche_attributo(input: TokenStream) -> TokenStream {
}
```

</Listing>

La funzione che definisce una macro procedurale prende un `TokenStream` come
input e produce un `TokenStream` come output. Il _type_ `TokenStream` è definito
dal _crate_ `proc_macro` incluso in Rust e rappresenta una sequenza di token.
Questo è il nucleo della macro: il codice sorgente su cui la macro opera
costituisce il `TokenStream` di input, e il codice che la macro produce è il
`TokenStream` di output. La funzione ha anche un attributo che specifica quale
tipo di macro procedurale stiamo creando. Possiamo avere più tipi di macro
procedurali nello stesso _crate_.

Vediamo le diverse tipologie di macro procedurali. Inizieremo con una macro
`derive` personalizzata per poi spiegare le piccole differenze che
caratterizzano le altre forme.

### Macro `derive` Personalizzate

Creiamo un _crate_ chiamato `ciao_macro` che definisce un _trait_ chiamato
`CiaoMacro` con una funzione associata chiamata `ciao_macro`. Invece di far
implementare agli utenti il _trait_ `CiaoMacro` per ogni loro _type_, forniremo
una macro procedurale che consente agli utenti di annotare il loro _type_ con
`#[derive(CiaoMacro)]` per ottenere un’implementazione di default della funzione
`ciao_macro`. L’implementazione di default stamperà `Ciao, Macro! Il mio nome è
NomeType!` dove `NomeType` è il nome del _type_ su cui il _trait_ è stato
definito. In altre parole, scriveremo un _crate_ che permette a un altro
programmatore di scrivere codice come nel Listato 20-37 usando il nostro
_crate_.

<Listing number="20-37" file-name="src/main.rs" caption="Il codice che un utente del nostro crate potrà scrivere usando la nostra macro procedurale">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-37/src/main.rs}}
```

</Listing>

Questo codice stamperà `Ciao, Macro! Il mio nome è Pancake!` quando sarà
eseguito. Il primo passo è creare un nuovo _crate_ libreria come segue:

```console
$ cargo new ciao_macro --lib
```

Successivamente, nel Listato 20-38, definiremo il _trait_ `CiaoMacro` e la sua
funzione associata.

<Listing number="20-38" file-name="src/lib.rs" caption="Un semplice _trait_ che useremo con la macro `derive`">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-38/ciao_macro/src/lib.rs}}
```

</Listing>

Abbiamo un _trait_ e la sua funzione. A questo punto, l’utente del nostro
_crate_ potrebbe implementare il _trait_ per ottenere la funzionalità
desiderata, come mostrato nel Listato 20-39.

<Listing number="20-39" file-name="src/main.rs" caption="Come apparirebbe se gli utenti scrivessero manualmente l’implementazione del _trait_ `CiaoMacro`">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-39/pancake/src/main.rs}}
```

</Listing>

Tuttavia, gli utenti dovrebbero scrivere il blocco di implementazione per ogni
_type_ su cui vogliono usare `ciao_macro`; vogliamo risparmiarli da questo
lavoro.

Inoltre, non possiamo ancora fornire alla funzione `ciao_macro`
un’implementazione di default che stampi il nome del _type_ su cui il _trait_ è
implementato: Rust non possiede capacità riflessive (_reflection_), cioè  non
può ricavare il nome del _type_ durante l’esecuzione. Abbiamo bisogno di una
macro per generare il codice a durante la compilazione.

Il passo successivo è definire la macro procedurale. Al momento della scrittura,
le macro procedurali devono risiedere in un _crate_ a parte. Questa restrizione
potrebbe essere rimossa in futuro. La convenzione per strutturare _crate_ e
_crate_ macro è la seguente: per un _crate_ chiamato `foo`, un _crate_ di macro
procedurali `derive` personalizzate si chiama `foo_derive`. Creiamo quindi un
nuovo _crate_ chiamato `ciao_macro_derive` all’interno del progetto
`ciao_macro`:

```console
$ cargo new ciao_macro_derive --lib
```

I due _crate_ sono strettamente correlati, quindi creiamo il _crate_ di macro
procedurali nella cartella del _crate_ `ciao_macro`. Se cambiamo la definizione
del _trait_ in `ciao_macro`, dovremo cambiare anche l’implementazione della
macro procedurale in `ciao_macro_derive`. I due _crate_ dovranno essere
pubblicati separatamente, e i programmatori che usano questi _crate_ dovranno
aggiungerli entrambi come dipendenze e importarli entrambi nello _scope_.
Potremmo invece far sì che il _crate_ `ciao_macro` utilizzi `ciao_macro_derive`
come dipendenza e rimandi il codice della macro procedurale. Tuttavia, la
struttura scelta consente ai programmatori di usare `ciao_macro` anche se non
vogliono la funzionalità `derive`.

Dobbiamo dichiarare il _crate_ `ciao_macro_derive` come _crate_ di macro
procedurali. Avremo anche bisogno della funzionalità dai _crate_ `syn` e
`quote`, come vedremo a breve, quindi dobbiamo aggiungerli come dipendenze.
Aggiungi quanto segue al file _Cargo.toml_ di `ciao_macro_derive`:

<Listing file-name="ciao_macro_derive/Cargo.toml">

```toml
{{#include ../listings/ch20-advanced-features/listing-20-40/ciao_macro/ciao_macro_derive/Cargo.toml:6:12}}
```

</Listing>

Per iniziare a definire la macro procedurale, inserisci il codice del Listato
20-40 nel file _src/lib.rs_ del crate `ciao_macro_derive`. Nota che questo
codice non si compila fino a quando non aggiungiamo una definizione per la
funzione `impl_ciao_macro`.

<Listing number="20-40" file-name="ciao_macro_derive/src/lib.rs" caption="Codice che la maggior parte dei _crate_ di macro procedurali richiederà per processare codice Rust">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-40/ciao_macro/ciao_macro_derive/src/lib.rs}}
```

</Listing>

Nota che abbiamo diviso il codice in una funzione `ciao_macro_derive`,
responsabile del parsing del `TokenStream`, e una funzione `impl_ciao_macro`,
responsabile della trasformazione dell’albero sintattico: questo rende la
scrittura di una macro procedurale più comoda. Il codice nella funzione esterna
(`ciao_macro_derive` in questo caso) sarà simile in quasi tutti i _crate_ di
macro procedurali che vedrai o creerai. Il codice che specifichiamo nel corpo
della funzione interna (`impl_ciao_macro` in questo caso) sarà diverso a seconda
dello scopo della macro procedurale.

Abbiamo introdotto tre nuovi _crate_: `proc_macro`, [`syn`][syn]<!-- ignore --> e
[`quote`][quote]<!-- ignore -->. Il _crate_ `proc_macro` fa parte di Rust, quindi
non abbiamo dovuto aggiungerlo alle dipendenze in _Cargo.toml_. Il _crate_
`proc_macro` è l’API del compilatore che consente di leggere e manipolare codice
Rust dal nostro codice.

Il _crate_ `syn` analizza il codice Rust da una stringa in una struttura dati su
cui possiamo eseguire operazioni. Il _crate_ `quote` trasforma le strutture dati
di `syn` nuovamente in codice Rust. Questi _crate_ rendono molto più semplice
analizzare qualsiasi tipo di codice Rust che vogliamo gestire: scrivere un
parser completo per Rust non è un compito semplice.

La funzione `ciao_macro_derive` verrà chiamata quando un utente della nostra
libreria specifica `#[derive(CiaoMacro)]` su un _type_. Questo è possibile
perché abbiamo annotato la funzione `ciao_macro_derive` con `proc_macro_derive`
e specificato il nome `CiaoMacro`, che corrisponde al nostro nome di _trait_;
questa è la convenzione che la maggior parte delle macro procedurali segue.

La funzione `ciao_macro_derive` prima converte l’input da un `TokenStream` a una
struttura dati che possiamo interpretare ed elaborare. Qui entra in gioco `syn`.
La funzione `parse` di `syn` prende un `TokenStream` e restituisce una struttura
`DeriveInput` che rappresenta il codice Rust analizzato. Il Listato 20-41 mostra
le parti rilevanti della struttura `DeriveInput` ottenuta analizzando la stringa
`struct Pancake;`.

<Listing number="20-41" caption="L’istanza `DeriveInput` che otteniamo analizzando il codice con l’attributo macro del Listato 20-37">

```rust,ignore
DeriveInput {
    // --taglio--

    ident: Ident {
        ident: "Pancake",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

</Listing>

I campi di questa struttura mostrano che il codice Rust che abbiamo analizzato è
una _struct_ _unit_ con `ident` (identificatore, cioè il nome) `Pancake`. Ci
sono altri campi in questa struttura per descrivere ogni tipo di codice Rust;
consultare la documentazione [`syn` per `DeriveInput`][syn-docs] per maggiori
dettagli.

Presto definiremo la funzione `impl_ciao_macro`, dove costruiremo il nuovo
codice Rust da includere. Ma prima nota che l’output della nostra macro `derive`
è anch’esso un `TokenStream`. Il `TokenStream` ritornato viene aggiunto al
codice scritto dai nostri utenti, così che quando compilano il loro _crate_
ottengano la funzionalità aggiuntiva che forniamo nel `TokenStream` modificato.

Potresti aver notato che chiamiamo `unwrap` per far generare un _panic_ alla
funzione `ciao_macro_derive` se la chiamata a `syn::parse` fallisce. È
necessario che la macro procedurale vada in _panic_ su errori perché le funzioni
`proc_macro_derive` devono restituire un `TokenStream` e non un `Result` per
conformarsi all’API delle macro procedurali. Abbiamo semplificato questo esempio
usando `unwrap`; nei codici di produzione, si dovrebbero fornire messaggi di
errore più specifici usando `panic!` o `expect`.

Ora che abbiamo il codice per trasformare il codice Rust annotato da un
`TokenStream` a un’istanza `DeriveInput`, generiamo il codice che implementa il
_trait_ `CiaoMacro` sul _type_ annotato, come mostrato nel Listato 20-42.

<Listing number="20-42" file-name="hello_macro_derive/src/lib.rs" caption="Implementazione del _trait_ `CiaoMacro` usando il codice Rust analizzato">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-42/ciao_macro/ciao_macro_derive/src/lib.rs:here}}
```

</Listing>

Otteniamo un’istanza `Ident` contenente il nome (identificatore) del _type_
annotato usando `ast.ident`. La _struct_ nel Listato 20-41 mostra che quando
eseguiamo la funzione `impl_ciao_macro` sul codice nel Listato 20-37, il campo
`ident` sarà `Pancake`. Quindi la variabile `nome` nel Listato 20-42 sarà
un’istanza di `Ident` che quando stampata sarà la stringa `"Pancake"`, il nome
della _struct_ del Listato 20-37.

La macro `quote!` ci permette di definire il codice Rust che vogliamo
restituire. Il compilatore si aspetta qualcosa di diverso dal risultato diretto
dell’esecuzione della macro `quote!`, quindi dobbiamo convertirlo in un
`TokenStream`. Lo facciamo chiamando il metodo `into`, che consuma questa
rappresentazione intermedia e ritorna un valore richiesto di _type_
`TokenStream`.

La macro `quote!` fornisce anche un meccanismo di modellazione molto
interessante: possiamo inserire `#nome` e `quote!` lo sostituirà con il valore
contenuto nella variabile `nome`. Si possono anche fare ripetizioni simili alle
macro normali. Consulta la documentazione del _crate_ [`quote`][quote-docs] per
un’introduzione completa.

Vogliamo che la nostra macro procedurale generi un’implementazione del _trait_
`CiaoMacro` per il _type_ annotato dall’utente, che otteniamo usando `#nome`.
L’implementazione del _trait_ ha una funzione, `ciao_macro`, il cui corpo
contiene la funzionalità che vogliamo fornire: stampare `Ciao, Macro! Il mio
nome è` e poi il nome del _type_ annotato.

La macro `stringify!` utilizzata qui è incorporata in Rust. Prende
un’espressione Rust, come `1 + 2`, e durante la compilazione la trasforma in una
stringa letterale, ad esempio `"1 + 2"`. Questo è diverso da `format!` o
`println!`, che sono macro che valutano l’espressione e poi trasformano il
risultato in una `String`. C’è la possibilità che l’input `#nome` possa essere
un’espressione da stampare letteralmente, quindi usiamo `stringify!`. Usare
`stringify!` evita anche un’allocazione convertendo `#nome` in una stringa
letterale durante la compilazione.

A questo punto, il comando `cargo build` dovrebbe completarsi con successo sia
in `ciao_macro` che in `ciao_macro_derive`. Colleghiamo questi _crate_ al codice
nel Listato 20-37 per vedere la macro procedurale in azione! Creiamo un nuovo
progetto binario nella directory _progetti_ con `cargo new pancake`. Dobbiamo
aggiungere `ciao_macro` e `ciao_macro_derive` come dipendenze nel _Cargo.toml_
del _crate_ `pancake`. Se pubblichi le tue versioni di `ciao_macro` e
`ciao_macro_derive` su [crates.io](https://crates.io/), saranno dipendenze
normali; altrimenti puoi specificarle come dipendenze di tipo `path` come segue:

```toml
{{#include ../listings/ch20-advanced-features/no-listing-21-pancakes/pancake/Cargo.toml:6:8}}
```

Inserisci il codice del Listato 20-37 in _src/main.rs_, ed esegui `cargo run`:
dovrebbe stampare `Ciao, Macro! Il mio nome è Pancake!`. L’implementazione del
_trait_ `CiaoMacro` dalla macro procedurale è stata inclusa senza che il _crate_
`pancakes` dovesse implementarla; il `#[derive(CiaoMacro)]` ha aggiunto
l’implementazione del _trait_.

Successivamente, esploreremo come le altre tipologie di macro procedurali
differiscono dalle macro `derive` personalizzate.

### Macro Simil-Attributo

Le macro simil-attributo sono simili alle macro `derive` personalizzate, ma
invece di generare codice per l’attributo `derive`, permettono di creare nuovi
attributi. Sono anche più flessibili: `derive` funziona solo per _struct_ ed
_enum_; gli attributi possono essere applicati anche ad altri elementi, come
funzioni. Ecco un esempio di utilizzo di una macro simil-attributo. Supponiamo
di avere un attributo chiamato `route` che annota funzioni in un framework per
applicazioni web:

```rust,ignore
#[route(GET, "/")]
fn index() {
```

Questo attributo `#[route]` sarebbe definito dal framework come una macro
procedurale. La firma della funzione che definisce la macro sarebbe simile a
questa:

```rust,ignore
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

Qui abbiamo due parametri di _type_ `TokenStream`. Il primo è per il contenuto
dell’attributo: la parte `GET, "/"`. Il secondo è il corpo dell’elemento a cui
l’attributo è associato: in questo caso, `fn index() {}` e il resto del corpo
della funzione.

A parte questo, le macro simil-attributo funzionano come le macro `derive`
personalizzate: si crea un _crate_ con tipo _crate_ `proc-macro` e si implementa
una funzione che genera il codice desiderato.

### Macro Simil-Funzioni

Le macro simil-funzioni definiscono macro che sembrano chiamate di funzione.
Come le macro `macro_rules!`, sono più flessibili delle funzioni; per esempio,
possono prendere un numero variabile di argomenti. Tuttavia, le macro
`macro_rules!` possono essere definite solo usando la sintassi simile a `match`
vista nella sezione [“Macro Dichiarative Per la Meta-Programmazione
Generale”][decl] vista in precedenza. Le macro simil-funzioni prendono un
parametro `TokenStream` e la loro definizione manipola quel `TokenStream` usando
codice Rust, come fanno le altre due tipologie di macro procedurali.

Un esempio di macro simil-funzione è una macro `sql!` che potrebbe essere chiamata in questo modo:

```rust,ignore
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

Questa macro potrebbe analizzare la dichiarazione SQL al suo interno e
verificare che sia sintatticamente corretta, un’elaborazione molto più complessa
di quella che una macro `macro_rules!` può fare. La macro `sql!` sarebbe
definita così:

```rust,ignore
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

Questa definizione è simile alla firma di una macro `derive` personalizzata:
riceviamo i token che stanno dentro le parentesi e restituiamo il codice che
vogliamo generare.

## Riepilogo

Wow! Ora hai appreso alcune funzionalità di Rust che probabilmente non userai
troppo spesso, ma sarà utile sapere che sono disponibili in circostanze
particolari. Abbiamo introdotto diversi argomenti complessi in modo che, quando
li incontrerai nei suggerimenti dei messaggi di errore o nel codice scritto da
altri, tu possa riconoscere questi concetti e sintassi. Usa questo capitolo come
riferimento per guidarti nelle soluzioni.

Adesso metteremo in pratica tutto ciò di cui abbiamo discusso durante il libro e
realizzeremo un altro progetto!

[ref]: https://doc.rust-lang.org/stable/reference/macros-by-example.html
[tlborm]: https://veykril.github.io/tlborm/
[syn]: https://crates.io/crates/syn
[quote]: https://crates.io/crates/quote
[syn-docs]: https://docs.rs/syn/2.0/syn/struct.DeriveInput.html
[quote-docs]: https://docs.rs/quote
[decl]: #macro-dichiarative-per-la-meta-programmazione-generale
