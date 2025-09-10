## Memorizzare Testo Codificato UTF-8 con Stringhe

Abbiamo parlato delle stringhe nel Capitolo 4, ma ora le analizzeremo più
approfonditamente. I nuovi Rustaceans spesso si bloccano sulle stringhe per una
combinazione di tre ragioni: la propensione di Rust a esporre possibili errori,
il fatto che le stringhe siano una struttura dati più complicata di quanto molti
programmatori credano e la codifica UTF-8. Questi fattori si combinano in un
modo che può sembrare difficile per chi proviene da altri linguaggi di
programmazione.

Parleremo delle stringhe nel contesto delle collezioni perché le stringhe sono
implementate come una collezione di byte, oltre ad alcuni metodi per fornire
funzionalità utili quando tali byte vengono interpretati come testo. In questa
sezione, parleremo delle operazioni su `String` che ogni tipo di collezione
prevede, come creazione, aggiornamento e lettura. Discuteremo anche le
differenze tra `String` e le altre collezioni, in particolare come
l'indicizzazione in una `String` sia complicata dalle differenze tra il modo in
cui le persone e i computer interpretano i dati `String`.

### Cos'è una Stringa?

Definiremo innanzitutto cosa intendiamo con il termine _stringa_. Rust ha un
solo _type_ di stringa nel linguaggio principale, ovvero la _slice_ di stringa
`str` che viene principalmente utilizzata come _reference_ `&str`. Nel Capitolo
4 abbiamo parlato delle _string slice_, che sono _reference_ ad alcuni dati
stringa codificati in UTF-8 memorizzati altrove. I letterali stringa, ad
esempio, sono memorizzati nel binario del programma e sono quindi _slice_ di
stringa.

Il _type_ `String`, fornito dalla libreria standard di Rust anziché esser
definito nel linguaggio principale, è un _type_ di stringa codificato in UTF-8,
con _ownership_, espandibile e modificabile. Quando i Rustaceans fanno
riferimento alle "stringhe" in Rust, potrebbero riferirsi sia al _type_ `String`
che alla _slice_ di stringa `&str`, e non solo a uno di questi _type_. Sebbene
questa sezione tratterà principalmente `String`, entrambe le tipologie sono
ampiamente utilizzate nella libreria standard di Rust, e sia `String` che le
_slice_ sono codificate in UTF-8.

### Creare una Nuova `String`

Molte delle operazioni disponibili con `Vec<T>` sono disponibili anche con
`String` perché `String` è in realtà implementata come _wrapper_ (_involucro_)
attorno a un vettore di byte con alcune garanzie, restrizioni e funzionalità
aggiuntive. Ad esempio, la funzione `new` per creare un'istanza, funziona allo
stesso modo sia con `Vec<T>` che con `String`. Eccola mostrata nel Listato 8-11.

<Listing number="8-11" caption="Creazione di una nuova `String` vuota">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-11/src/main.rs:here}}
```

</Listing>

Questa riga crea una nuova stringa vuota chiamata `s`, in cui possiamo quindi
caricare i dati. Spesso, avremo dei dati iniziali con cui vogliamo inizializzare
la stringa. Per questo, utilizziamo il metodo `to_string`, disponibile su
qualsiasi _type_ che implementi il _trait_ `Display`, come fanno i letterali
stringa. Il Listato 8-12 mostra due esempi.

<Listing number="8-12" caption="Utilizzo del metodo `to_string` per creare una `String` da un letterale stringa">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-12/src/main.rs:here}}
```

</Listing>

Questo codice crea una stringa contenente `contenuto iniziale`.

Possiamo anche utilizzare la funzione `String::from` per creare una `String` da
un letterale stringa. Il codice nel Listato 8-13 è equivalente al codice nel
Listato 8-12 che utilizza `to_string`.

<Listing number="8-13" caption="Utilizzo della funzione `String::from` per creare una `String` da un letterale stringa">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-13/src/main.rs:here}}
```

</Listing>

Poiché le stringhe vengono utilizzate per così tante cose, possiamo utilizzare
diverse API generiche per le stringhe, offrendoci numerose opzioni. Alcune
possono sembrare ridondanti, ma hanno tutte la loro importanza! In questo caso,
`String::from` e `to_string` svolgono la stessa funzione, quindi la scelta è una
questione di stile e leggibilità.

Ricorda che le stringhe sono codificate in UTF-8, quindi possiamo includere
qualsiasi dato codificato correttamente, come mostrato nel Listato 8-14.

<Listing number="8-14" caption="Memorizzazione di saluti in diverse lingue nelle stringhe">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:here}}
```

</Listing>

Tutti questi sono valori `String` validi.

### Aggiornare una `String`

Una `String` può crescere di dimensione e il suo contenuto può cambiare, proprio
come il contenuto di un `Vec<T>`, se vi si inseriscono più dati. Inoltre, è
possibile utilizzare comodamente l'operatore `+` o la macro `format!` per
concatenare valori `String`.

#### Aggiungere a una `String` con `push_str` e `push`

Possiamo far crescere una `String` utilizzando il metodo `push_str` per
aggiungere una _slice_ di stringa, come mostrato nel Listato 8-15.

<Listing number="8-15" caption="Aggiungere una _slice_ a una `String` utilizzando il metodo `push_str`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-15/src/main.rs:here}}
```

</Listing>

Dopo queste due righe, `s` conterrà `foobar`. Il metodo `push_str` accetta una
_slice_ di stringa perché non vogliamo necessariamente prendere _ownership_ del
parametro. Ad esempio, nel codice del Listato 8-16, vogliamo poter utilizzare
`s2` dopo averne aggiunto il contenuto a `s1`.

<Listing number="8-16" caption="Utilizzo di una _slice_ dopo averne aggiunto il contenuto a una `String`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-16/src/main.rs:here}}
```

</Listing>

Se il metodo `push_str` prendesse _ownership_ di `s2`, non saremmo in grado di
stamparne il valore sull'ultima riga. Tuttavia, questo codice funziona come
previsto!

Il metodo `push` prende un singolo carattere come parametro e lo aggiunge alla
`String`. Il Listato 8-17 aggiunge la lettera _l_ a una `String` utilizzando il
metodo `push`.

<Listing number="8-17" caption="Aggiunta di un carattere a un valore `String` utilizzando `push`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```

</Listing>

Di conseguenza, `s` conterrà `lol`.

#### Concatenare con l'Operatore `+` o la Macro `format!`

Spesso, si desidera combinare due stringhe esistenti. Un modo per farlo è
utilizzare l'operatore `+`, come mostrato nel Listato 8-18.

<Listing number="8-18" caption="Utilizzo dell'operatore `+` per combinare due valori `String` in un nuovo valore `String`">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```

</Listing>

La stringa `s3` conterrà `Hello, world!`. Il motivo per cui `s1` non è più
valido dopo l'aggiunta, e il motivo per cui abbiamo utilizzato un _reference_ a
`s2`, ha a che fare con la firma del metodo chiamato quando utilizziamo
l'operatore `+`. L'operatore `+` utilizza il metodo `add`, la cui firma è simile
a questa:

```rust,ignore
fn add(self, s: &str) -> String {
```

Nella libreria standard, `add` è definito utilizzando _type_ generici e _type_
associati. Qui, abbiamo sostituito _type_ concreti, che è ciò che accade quando
chiamiamo questo metodo con valori `String`. Parleremo dei generici nel Capitolo
10. Questa firma ci fornisce gli indizi necessari per comprendere le parti più
complesse dell'operatore `+`.

Innanzitutto, `s2` ha un `&`, il che significa che stiamo aggiungendo un
_reference_ della seconda stringa alla prima stringa. Questo è dovuto al
parametro `s` nella funzione `add`: possiamo solo aggiungere un `&str` a una
`String`; non possiamo aggiungere due valori `String` insieme. Ma aspetta: il
_type_ di `&s2` è `&String`, non `&str`, come specificato nel secondo parametro
di `add`. Quindi perché il Listato 8-18 si compila?

Il motivo per cui possiamo usare `&s2` nella chiamata a `add` è che il
compilatore può _costringere_ l'argomento `&String` in un `&str`. Quando
chiamiamo il metodo `add` , Rust usa una _deref coercion_ (_de-referenziazione
forzata_), che qui trasforma `&s2` in `&s2[..]`. Discuteremo la _deref coercion_
più approfonditamente nel Capitolo 15. Poiché `add` non prende _ownership_ del
parametro `s`, `s2` sarà comunque una `String` valida dopo questa operazione.

In secondo luogo, possiamo vedere nella firma che `add` prende _ownership_ di
`self` perché `self` _non_ ha un `&`. Ciò significa che `s1` nel Listato 8-18
verrà spostato nella chiamata `add` e non sarà più valido da quel momento in
poi. Quindi, sebbene `let s3 = s1 + &s2;` sembri copiare entrambe le stringhe e
crearne una nuova, questa istruzione in realtà prende _ownership_ di `s1`,
aggiunge una copia del contenuto di `s2` per poi restituire la _ownership_ del
risultato. In altre parole, sembra che stia facendo molte copie, ma non è così;
l'implementazione è più efficiente della semplice copia.

Se dobbiamo concatenare più stringhe, il comportamento dell'operatore `+`
diventa poco pratico:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```

A questo punto, `s` diventerà `uno-due-tre`. Con tutti i caratteri `+` e `"`, è
difficile capire cosa sta succedendo. Per combinare stringhe in modi più
complessi, possiamo invece usare la macro `format!`:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

Anche questo codice risulterà in `s` che contiene `uno-due-tre`. La macro
`format!` funziona come `println!`, ma invece di visualizzare l'output sullo
schermo, restituisce una `String` con il contenuto. La versione del codice che
utilizza `format!` è molto più facile da leggere e il codice generato dalla
macro `format!` utilizza _reference_ in modo che questa chiamata non assuma
_ownership_ di nessuno dei suoi parametri.

### Indicizzazione in `String`

In molti altri linguaggi di programmazione, l'accesso a singoli caratteri in una
stringa facendovi riferimento tramite indice è un'operazione valida e comune.
Tuttavia, se si tenta di accedere a parti di una `String` utilizzando la
sintassi di indicizzazione in Rust, si otterrà un errore. Considera il codice
non valido nel Listato 8-19.

<Listing number="8-19" caption="Tentativo di utilizzare la sintassi di indicizzazione con una `String`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```

</Listing>

Questo codice genererà il seguente errore:

```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```

L'errore e la nota spiegano la situazione: le stringhe Rust non supportano
l'indicizzazione. Ma perché no? Per rispondere a questa domanda, dobbiamo
discutere come Rust memorizza le stringhe in memoria.

#### Rappresentazione Interna

Una `String` è un _wrapper_ di un `Vec<u8>`. Diamo un'occhiata ad alcune delle
nostre stringhe di esempio correttamente codificate UTF-8 dal Listato 8-14.
Innanzitutto, questo:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:spanish}}
```

In questo caso, `len` sarà `4`, il che significa che il vettore che memorizza la
stringa `"Hola"` è lungo 4 byte. Ognuna di queste lettere occupa un byte se
codificata in UTF-8. La riga seguente, tuttavia, potrebbe sorprendervi (nota che
questa stringa inizia con la lettera maiuscola cirillica _Ze_, non con il numero
3):

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:russian}}
```

Se vi chiedessero quanto è lunga la stringa, potreste dire 12. In realtà, la
risposta di Rust è 24: questo è il numero di byte necessari per codificare
"Здравствуйте" in UTF-8, perché ogni valore scalare Unicode in quella stringa
occupa 2 byte di spazio. Pertanto, un indice nei byte della stringa non sarà
sempre correlato a un valore scalare Unicode valido. Per dimostrarlo,
consideriamo questo codice Rust non valido:

```rust,ignore,does_not_compile
let saluto = "Здравствуйте";
let risposta = &hello[0];
```

Sapete già che `risposta` non sarà `З`, la prima lettera. Quando codificato in
UTF-8, il primo byte di `З` è `208` e il secondo è `151`, quindi sembrerebbe che
`risposta` dovrebbe in effetti essere `208`, ma `208` non è un carattere valido
da solo. Restituire `208` probabilmente non è ciò che un utente vorrebbe se
chiedesse la prima lettera di questa stringa; Tuttavia, questo è l'unico dato
che Rust ha all'indice di byte 0. Gli utenti generalmente non vogliono che venga
restituito il valore in byte, anche se la stringa contiene solo lettere latine:
se `&"hi"[0]` fosse codice valido che restituisce il valore in byte,
restituirebbe `104`, non `h`.

La risposta, quindi, è che per evitare di restituire un valore inaspettato e
causare bug che potrebbero non essere scoperti immediatamente, Rust non compila
affatto questo codice e previene malintesi fin dalle prime fasi del processo di
sviluppo.

#### Byte, Valori Scalari e Cluster di Grafemi! Oddio!

Un altro punto su UTF-8 è che in realtà ci sono tre modi rilevanti per vedere le
stringhe dalla prospettiva di Rust: come byte, valori scalari e cluster di
grafemi (la cosa più vicina a ciò che chiameremmo _lettere_).

Se consideriamo la parola hindi "नमस्ते" scritta in alfabeto Devanagari, essa è
memorizzata come un vettore di valori `u8` che appare così:

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

Sono 18 byte ed è così che i computer memorizzano questi dati. Se li
consideriamo come valori scalari Unicode, che corrispondono al _type_ `char` di
Rust, quei byte appaiono così:

```text
['न', 'म', 'स', '्', 'त', 'े']
```

Ci sono sei valori `char` qui, ma il quarto e il sesto non sono lettere: sono
segni diacritici che da soli non hanno senso. Infine, se li consideriamo come
cluster di grafemi, otterremmo ciò che una persona chiamerebbe le quattro
lettere che compongono la parola hindi:

```text
["न", "म", "स्", "ते"]
```

Rust fornisce diversi modi di interpretare i dati stringa grezzi che i computer
memorizzano, in modo che ogni programma possa scegliere l'interpretazione di cui
ha bisogno, indipendentemente dal linguaggio umano in cui sono espressi i dati.

Un ultimo motivo per cui Rust non ci consente di indicizzare una `String` per
ottenere un carattere è che le operazioni di indicizzazione dovrebbero sempre
richiedere un tempo costante (O(1)). Ma non è possibile garantire tali
prestazioni con una `String`, perché Rust dovrebbe esaminare il contenuto
dall'inizio fino all'indice per determinare quanti caratteri validi ci sono.

### Slicing delle Stringhe

Indicizzare una stringa è spesso una cattiva idea perché non è chiaro quale
debba essere il _type_ di ritorno dell'operazione di indicizzazione della
stringa: un valore byte, un carattere, un cluster di grafemi o una _slice_. Se
proprio si ha bisogno di usare gli indici per creare _slice_ di stringa, Rust
chiede di essere più specifici.

Invece di indicizzare usando `[]` con un singolo numero, si può usare `[]` con
un intervallo per creare una _slice_ di stringa contenente byte specifici:

```rust
let saluto = "Здравствуйте";

let s = &saluto[0..4];
```

Qui, `s` sarà un `&str` che contiene i primi quattro byte della stringa. In
precedenza, abbiamo accennato al fatto che ognuno di questi caratteri era
composto da due byte, il che significa che `s` sarà `Зд`.

Se provassimo a suddividere solo una parte dei byte di un carattere con qualcosa
come `&saluto[0..1]`, Rust andrebbe in _panic_ in fase di esecuzione, proprio
come se si accedesse a un indice non valido in un vettore:

```console
{{#include ../listings/ch08-common-collections/output-only-01-not-char-boundary/output.txt}}
```

È necessario prestare attenzione quando si creano _slice_ di stringhe con
intervalli, perché ciò potrebbe causare l'arresto anomalo del programma.

### Metodi per Iterare sulle Stringhe

Il modo migliore per operare su stringhe è specificare esplicitamente se si
desidera caratteri o byte. Per singoli valori scalari Unicode, utilizzare il
metodo `chars`. Chiamando `chars` su "Зд" si separano e si restituiscono due
valori di _type_ `char`, ed è possibile iterare sul risultato per accedere a
ciascun elemento:

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

Questo codice stamperà quanto segue:

```text
З
д
```

In alternativa, il metodo `bytes` restituisce ogni byte grezzo, che potrebbe
essere appropriato per quello che ti serve:

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

Questo codice stamperà i quattro byte che compongono questa stringa:

```text
208
151
208
180
```

Ma ricorda che i valori scalari Unicode validi possono essere composti da più di
un byte.

Ottenere _cluster_ di grafemi dalle stringhe, come con l'alfabeto Devanagari, è
complesso, quindi questa funzionalità non è fornita dalla libreria standard. I
_crate_ sono disponibili su [crates.io](https://crates.io/search?q=grapheme)<!--
ignore --> se questa è la funzionalità di cui avete bisogno.

### Le Stringhe Non Sono Così Semplici

In sintesi, le stringhe sono complicate. Diversi linguaggi di programmazione
fanno scelte diverse su come presentare questa complessità al programmatore.
Rust ha scelto di rendere la corretta gestione dei dati `String` il
comportamento predefinito per tutti i programmi Rust, il che significa che i
programmatori devono dedicare da subito maggiore attenzione alla gestione dei
dati UTF-8. Questo compromesso rende più evidente la complessità delle stringhe
rispetto ad altri linguaggi di programmazione, ma evita di dover gestire errori
che coinvolgono caratteri non ASCII in una fase successiva del ciclo di
sviluppo.

La buona notizia è che la libreria standard offre numerose funzionalità basate
sui _type_ `String` e `&str` per aiutare a gestire correttamente queste
situazioni complesse. Assicuratevi di consultare la documentazione per metodi
utili come `contains` per la ricerca in una stringa e `replace` per sostituire
parti di una stringa con un'altra stringa.

Passiamo a qualcosa di un po' meno complesso: le _mappe hash_!
