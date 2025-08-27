## Tipi di Dati Generici

Utilizziamo i tipi generici per creare definizioni per elementi come firme di funzioni o
strutture, che possiamo poi utilizzare con molti tipi di dati concreti diversi. Vediamo
prima come definire funzioni, strutture, enum e metodi utilizzando
i tipi generici. Poi discuteremo di come i generici influiscono sulle prestazioni del codice.

### Nelle Definizioni di Funzione

Quando definiamo una funzione che utilizza i tipi generici, li inseriamo nella
firma della funzione, dove normalmente specificheremmo i tipi dei parametri e il tipo del valore restituito. In questo modo il nostro codice diventa più flessibile e fornisce
maggiori funzionalità ai chiamanti della nostra funzione, evitando al contempo la duplicazione del codice.

Continuando con la nostra funzione `maggiore`, il Listato 10-4 mostra due funzioni che
trovano entrambe il valore più grande in una slice. Le combineremo quindi in un'unica
funzione che utilizza i tipi generici.

<Listing number="10-4" file-name="src/main.rs" caption="Due funzioni che differiscono solo per i nomi e per i tipi nelle loro firme">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```

</Listing>

La funzione `maggior_i32` è quella che abbiamo estratto nel Listato 10-3 e che trova
l'`i32` più grande in una slice. La funzione `maggior_char` trova il `char` più grande in una slice. I corpi delle funzioni hanno lo stesso codice, quindi eliminiamo
la duplicazione introducendo un parametro di tipo generico in una singola funzione.

Per parametrizzare i tipi in una nuova singola funzione, dobbiamo assegnare un nome al parametro di tipo,
proprio come facciamo per i parametri di valore di una funzione. È possibile utilizzare
qualsiasi identificatore come nome di parametro di tipo. Ma useremo `T` perché, per
convenzione, i nomi dei parametri di tipo in Rust sono brevi, spesso di una sola lettera, e
la convenzione di denominazione dei tipi di Rust è CamelCase (le parole di un nome composto sono unite senza spazi, e la prima lettera di ogni parola ,eccetto la prima, è in maiuscolo). Abbreviazione di _type_, `T` è la scelta predefinita
della maggior parte dei programmatori Rust.

Quando utilizziamo un parametro nel corpo della funzione, dobbiamo dichiarare il
nome del parametro nella firma in modo che il compilatore ne conosca il significato.
Allo stesso modo, quando utilizziamo un nome di parametro di tipo nella firma di una funzione, dobbiamo
dichiarare il nome del parametro di tipo prima di utilizzarlo. Per definire la funzione generica
`maggiore`, inseriamo le dichiarazioni del nome di tipo tra parentesi angolari,
`<>`, tra il nome della funzione e l'elenco dei parametri, in questo modo:

```rust,ignore
fn maggiore<T>(lista: &[T]) -> &T {
```

Leggiamo questa definizione come: la funzione `maggiore` è generica su un certo tipo
`T`. Questa funzione ha un parametro denominato `lista`, che è una porzione di valori
di tipo `T`. La funzione `maggiore` restituirà un riferimento a un valore dello
stesso tipo `T`.

Il Listato 10-5 mostra la definizione combinata della funzione `maggiore` utilizzando il tipo di dati generico
nella sua firma. Il listato mostra anche come possiamo chiamare la funzione
con una porzione di valori `i32` o `char`. Si noti che questo codice non verrà
ancora compilato.

<Listing number="10-5" file-name="src/main.rs" caption="La funzione `più grande` che utilizza parametri di tipo generico; non è ancora compilabile">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```

</Listing>

Se compiliamo questo codice adesso, otterremo questo errore:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

Il testo di aiuto menziona `std::cmp::PartialOrd`, che è un _trait_, e  parleremo dei tratti nella prossima sezione. Per ora, sappiate che questo errore
indica che il corpo di `maggiore` non funzionerà per tutti i possibili tipi di `T`. Poiché vogliamo confrontare valori di tipo `T` nel corpo, possiamo
utilizzare solo tipi i cui valori possono essere ordinati. Per abilitare i confronti, la libreria
standard include il tratto `std::cmp::PartialOrd` che è possibile implementare sui tipi
(vedere l'Appendice C per maggiori informazioni su questo trait). Per correggere il Listato 10-5, possiamo seguire
il suggerimento del testo di aiuto e limitare i tipi validi per `T` solo a quelli che
implementano `PartialOrd`. Il listato verrà quindi compilato, poiché la libreria
standard implementa `PartialOrd` sia su `i32` che su `char`.

### Nelle Definizioni delle Strutture

Possiamo anche definire strutture per utilizzare un parametro di tipo generico in uno o più
campi utilizzando la sintassi `<>`. Il Listato 10-6 definisce una struttura `Punto<T>` per contenere
i valori delle coordinate `x` e `y` di qualsiasi tipo.

<Listing number="10-6" file-name="src/main.rs" caption="Una struttura `Punto<T>` che contiene i valori `x` e `y` di tipo `T`">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

</Listing>

La sintassi per l'utilizzo di tipi generici nelle definizioni di struct è simile a quella utilizzata nelle definizioni di funzione. Per prima cosa dichiariamo il nome del parametro di tipo tra parentesi angolari subito dopo il nome della struct. Quindi utilizziamo il tipo generico nella definizione della struct, dove altrimenti specificheremmo tipi di dati concreti.

Si noti che, poiché abbiamo utilizzato un solo tipo generico per definire `Punto<T>`, questa definizione afferma che la struct `Punto<T>` è generica su un tipo `T` e che i campi `x` e `y` sono entrambi dello stesso tipo, qualunque esso sia. Se
creiamo un'istanza di `Punto<T>` che ha valori di tipi diversi, come nel
Listato 10-7, il nostro codice non verrà compilato.

<Listing number="10-7" file-name="src/main.rs" caption="I campi `x` e `y` devono essere dello stesso tipo perché entrambi hanno lo stesso tipo di dati generico `T`.">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

</Listing>

In questo esempio, quando assegniamo il valore intero `5` a `x`, comunichiamo al
compilatore che il tipo generico `T` sarà un intero per questa istanza di
`Punto<T>`. Quindi, quando specifichiamo `4.0` per `y`, che abbiamo definito come dello
stesso tipo di `x`, otterremo un errore di mancata corrispondenza di tipo come questo:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```

Per definire una struttura `Punto` in cui `x` e `y` sono entrambi tipi generici ma potrebbero avere
tipi diversi, possiamo utilizzare più parametri di tipo generico. Ad esempio, nel
Listato 10-8, modifichiamo la definizione di `Punto` in modo che sia generico sui tipi `T`
e `U`, dove `x` è di tipo `T` e `y` è di tipo `U`.

<Listing number="10-8" file-name="src/main.rs" caption="Un `Punto<T, U>` generico su due tipi in modo che `x` e `y` possano essere valori di tipi diversi">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

</Listing>

Ora tutte le istanze di `Punto` mostrate sono consentite! Puoi usare tutti i parametri di tipo generico che vuoi in una definizione, ma usarne di più rende il codice difficile da leggere. Se ti accorgi di aver bisogno di molti tipi generici nel tuo codice, potrebbe essere necessario ristrutturarlo in parti più piccole.

### Nelle Definizioni di Enum

Come abbiamo fatto con le strutture, possiamo definire le enum per contenere tipi di dati generici nelle loro
varianti. Diamo un'altra occhiata all'enum `Option<T>` fornito dalla libreria standard, che abbiamo usato nel Capitolo 6:

```rust
enum Option<T> {
Some(T),
None,
}
```

Questa definizione dovrebbe ora esservi più chiara. Come potete vedere, l'enum
`Option<T>` è generico sul tipo `T` e ha due varianti: `Some`, che
contiene un valore di tipo `T`, e una variante `None` che non contiene alcun valore.
Utilizzando l'enum `Option<T>`, possiamo esprimere il concetto astratto di un
valore opzionale e, poiché `Option<T>` è generico, possiamo usare questa astrazione
indipendentemente dal tipo del valore opzionale.

Anche gli enum possono usare più tipi generici. La definizione dell'enum `Result`
che abbiamo usato nel Capitolo 9 è un esempio:

```rust
enum Result<T, E> {
Ok(T),
Err(E),
}
```

L'enum `Result` è generico su due tipi, `T` ed `E`, e ha due varianti:
`Ok`, che contiene un valore di tipo `T`, e `Err`, che contiene un valore di tipo
`E`. Questa definizione rende comodo usare l'enum `Result` ovunque
abbiamo un'operazione che potrebbe avere successo (restituire un valore di tipo `T`) o fallire
(restituire un errore di tipo `E`). In effetti, questo è ciò che abbiamo usato per aprire un
file nel Listato 9-3, dove `T` veniva compilato con il tipo `std::fs::File` quando
il file veniva aperto correttamente e `E` veniva compilato con il tipo
`std::io::Error` quando si verificavano problemi durante l'apertura del file.

Quando si riconoscono situazioni nel codice con più definizioni di struct o enum
che differiscono solo per il tipo dei valori che contengono, è possibile
evitare la duplicazione utilizzando invece tipi generici.

### Nelle Definizioni dei Metodi

Possiamo implementare metodi su struct ed enum (come abbiamo fatto nel Capitolo 5) e utilizzare
tipi generici anche nelle loro definizioni. Il Listato 10-9 mostra la struct `Punto<T>`
definita nel Listato 10-6 con un metodo denominato `x` implementato su di essa.

<Listing number="10-9" file-name="src/main.rs" caption="Implementazione di un metodo denominato `x` sulla struttura `Punto<T>` che restituirà un riferimento al campo `x` di tipo `T`">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```

</Listing>

Qui, abbiamo definito un metodo denominato `x` su `Punto<T>` che restituisce un riferimento
ai dati nel campo `x`.

Si noti che dobbiamo dichiarare `T` subito dopo `impl` in modo da poter usare `T` per specificare
che stiamo implementando metodi sul tipo `Punto<T>`. Dichiarando `T` come
tipo generico dopo `impl`, Rust può identificare che il tipo tra parentesi
angolari in `Punto` è un tipo generico piuttosto che un tipo concreto. Avremmo
potuto scegliere un nome diverso per questo parametro generico rispetto al parametro generico
dichiarato nella definizione della struttura, ma utilizzare lo stesso nome è
convenzionale. Se si scrive un metodo all'interno di un `impl` che dichiara un tipo
generico, tale metodo verrà definito su qualsiasi istanza del tipo, indipendentemente dal
tipo concreto che finisce per sostituire il tipo generico.

Possiamo anche specificare vincoli sui tipi generici quando si definiscono metodi sul
tipo. Ad esempio, potremmo implementare metodi solo su istanze di `Punto<f32>`
piuttosto che su istanze di `Punto<T>` con qualsiasi tipo generico. Nel Listato 10-10 utilizziamo
il tipo concreto `f32`, il che significa che non dichiariamo alcun tipo dopo `impl`.

<Listing number="10-10" file-name="src/main.rs" caption="Un blocco `impl` che si applica solo a una struttura con un particolare tipo concreto per il parametro di tipo generico `T`">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```

</Listing>

Questo codice indica che il tipo `Punto<f32>` avrà un metodo `distanza_da_origine`
; Altre istanze di `Punto<T>` in cui `T` non è di tipo `f32` non
avranno questo metodo definito. Il metodo misura la distanza del nostro punto dal
punto alle coordinate (0.0, 0.0) e utilizza operazioni matematiche
disponibili solo per i tipi a virgola mobile.

I parametri di tipo generico nella definizione di una struttura non sono sempre gli stessi
di quelli utilizzati nelle firme dei metodi della stessa struttura. Il Listato 10-11 utilizza i tipi generici
`X1` e `Y1` per la struttura `Punto` e `X2` e `Y2` per la firma del metodo `mixup`
per rendere l'esempio più chiaro. Il metodo crea una nuova istanza di `Punto`
con il valore `x` dal `self` `Punto` (di tipo `X1`) e il valore `y`
dal `Punto` passato (di tipo `Y2`).

<Listing number="10-11" file-name="src/main.rs" caption="Un metodo che utilizza tipi generici diversi dalla definizione della sua struttura">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```

</Listing>

In `main`, abbiamo definito un `Punto` che ha un `i32` per `x` (con valore `5`)
e un `f64` per `y` (con valore `10.4`). La variabile `p2` è una struttura `Punto`
che ha una slice di stringa per `x` (con valore `"Hello"`) e un `char` per `y`
(con valore `c`). Chiamando `mixup` su `p1` con l'argomento `p2` otteniamo `p3`,
che avrà un `i32` per `x` perché `x` proviene da `p1`. La variabile `p3`
avrà un `char` per `y` perché `y` proviene da `p2`. La chiamata alla macro `println!`
stamperà `p3.x = 5, p3.y = c`.

Lo scopo di questo esempio è dimostrare una situazione in cui alcuni parametri generici
sono dichiarati con `impl` e altri con la definizione del metodo. Qui, i parametri generici `X1` e `Y1` sono dichiarati dopo
`impl` perché vanno con la definizione della struttura. I parametri generici `X2`
e `Y2` sono dichiarati dopo `fn mixup` perché sono rilevanti solo per il
metodo.

### Prestazioni del Codice con Parametri Generici

Potresti chiederti se l'utilizzo di parametri di tipo generico
costi in termini di runtime. La buona notizia è che l'utilizzo di tipi generici non renderà il tuo programma
più lento di quanto lo sarebbe con tipi concreti.

Rust ottiene questo risultato eseguendo la monomorfizzazione del codice utilizzando
parametri generici in fase di compilazione. La _monomorfizzazione_ è il processo di trasformazione del codice generico
in codice specifico inserendo i tipi concreti utilizzati in fase di
compilazione. In questo processo, il compilatore esegue l'opposto dei passaggi che abbiamo utilizzato
per creare la funzione generica nel Listato 10-5: il compilatore esamina tutti
i punti in cui viene chiamato il codice generico e genera codice per i tipi concreti
con cui viene chiamato il codice generico.

Vediamo come funziona utilizzando l'enum generico
`Option<T>` della libreria standard:

```rust
let integer = Some(5);
let float = Some(5.0);
```

Quando Rust compila questo codice, esegue la monomorfizzazione. Durante questo
processo, il compilatore legge i valori utilizzati nelle istanze di `Option<T>`
e identifica due tipi di `Option<T>`: uno è `i32` e l'altro
è `f64`. Pertanto, espande la definizione generica di `Option<T>` in due
definizioni specializzate per `i32` e `f64`, sostituendo così la definizione generica
con quelle specifiche.

La versione monomorfizzata del codice è simile alla seguente (il
compilatore usa nomi diversi da quelli che stiamo usando qui a scopo illustrativo):

<Listing file-name="src/main.rs">

```rust
enum Option_i32 {
Some(i32),
None,
}

enum Option_f64 {
Some(f64),
None,
}

fn main() {
let integer = Option_i32::Some(5);
let float = Option_f64::Some(5.0);
}
```

</Listing>

Il generico `Option<T>` viene sostituito con le definizioni specifiche create dal
compilatore. Poiché Rust compila il codice generico in codice che specifica il
tipo in ogni istanza, non si paga alcun costo di runtime per l'utilizzo di tipi generici. Quando il codice
viene eseguito, si comporta esattamente come se avessimo duplicato ogni definizione manualmente. Il processo di monomorfizzazione rende i generici di Rust estremamente efficienti
in fase di esecuzione.