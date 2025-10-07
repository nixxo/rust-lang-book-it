## _Unsafe_ Rust

Tutto il codice di cui abbiamo parlato finora ha avuto le garanzie di sicurezza
della memoria di Rust applicate durante la compilazione. Però, Rust ha un
secondo linguaggio nascosto al suo interno che non applica queste garanzie: si
chiama _unsafe Rust_ e funziona come il Rust regolare, ma ci dà dei superpoteri
extra.

_Unsafe Rust_ esiste perché, per natura, l’analisi statica è conservativa.
Quando il compilatore cerca di capire se il codice rispetta le garanzie, è
meglio per lui rifiutare qualche programma valido piuttosto che accettarne uno
non valido. Anche se il codice _potrebbe_ andare bene, se il compilatore Rust
non ha abbastanza informazioni per esserne sicuro, rifiuterà di compilare il
codice. In questi casi, puoi usare codice _unsafe_ per dire al compilatore:
"Fidati, so cosa sto facendo." Attenzione però, usare _unsafe Rust_ è un rischio
tuo: se usi codice _unsafe_ in modo sbagliato, possono succedere problemi legati
alla sicurezza della memoria, tipo il de-referenziamento di puntatori nulli.

Un altro motivo per cui Rust ha un alter ego _unsafe_ è che l'hardware del
computer è intrinsecamente _unsafe_. Se Rust non ti permettesse di fare
operazioni _unsafe_, non potresti fare certi compiti. Rust deve permetterti di
fare programmazione di sistema a basso livello, tipo interagire direttamente con
il sistema operativo o persino scrivere il tuo sistema operativo. La
programmazione di sistema a basso livello è uno degli obiettivi del linguaggio.
Vediamo cosa si può fare con _unsafe Rust_ e come farlo.

### Usare i Superpoteri _Unsafe_

Per passare a _unsafe Rust_, usa la parola chiave `unsafe` e poi inizia un nuovo
blocco dove metti il codice _unsafe_. Ci sono cinque azioni che puoi fare in
_unsafe Rust_ che non puoi fare in _safe Rust_, che chiamiamo _superpoteri
unsafe_. Questi superpoteri includono la possibilità di:

1. De-referenziare un puntatore grezzo
1. Chiamare una funzione o un metodo _unsafe_
1. Accedere o modificare una variabile statica mutabile
1. Implementare un _trait_ _unsafe_
1. Accedere ai campi di `union`

È importante capire che `unsafe` non disabilita il _borrow checker_ né gli altri
controlli di sicurezza di Rust: se usi un _reference_ in codice _unsafe_, esso
verrà comunque controllato. La parola chiave `unsafe` ti dà solo accesso a
queste cinque caratteristiche che non sono controllate dal compilatore
nell'aspetto della sicurezza della memoria. Avrai comunque un certo grado di
sicurezza dentro un blocco _unsafe_.

Inoltre, `unsafe` non significa che il codice dentro il blocco sia
necessariamente pericoloso o che sicuramente avrà problemi di sicurezza della
memoria: l’intento è che tu, programmatore, garantisca che il codice dentro un
blocco `unsafe` accederà alla memoria in modo valido.

Gli esseri umani sbagliano, possono fare errori, ma richiedendo che queste
cinque operazioni _unsafe_ siano usate dentro blocchi annotati con `unsafe`,
saprai che ogni errore legato alla sicurezza della memoria deve per forza essere
dentro un blocco `unsafe`. Mantieni i blocchi `unsafe` piccoli; ne sarai
contento quando dovrai cercare bug di memoria.

Per isolare il codice _unsafe_ il più possibile, è meglio racchiuderlo in
un'astrazione _safe_ e offrire un’API _safe_, di cui parleremo più avanti nel
capitolo quando esamineremo funzioni e metodi _unsafe_. Parti della libreria
standard sono implementate come astrazioni _safe_ su codice _unsafe_ che è stato
controllato. Racchiudere il codice _unsafe_ in un’astrazione _safe_ evita che
gli usi di `unsafe` vadano a infiltrarsi in tutte le parti del codice dove tu o
i tuoi utenti potreste voler usare la funzionalità scritta con codice _unsafe_,
perché usare un’astrazione _safe_ è sicuro.

Ora vediamo uno per uno i cinque superpoteri _unsafe_. Daremo anche un’occhiata
ad alcune astrazioni che forniscono una interfaccia _safe_ a codice _unsafe_.

### De-referenziare un Puntatore Grezzo

Nel Capitolo 4, nella sezione [“_Reference_ Pendenti”][dangling-references]<!--
ignore -->, abbiamo detto che il compilatore si assicura che i _reference_ siano
sempre validi. _Unsafe Rust_ ha due nuovi _type_ chiamati _puntatori grezzi_
(_raw pointer_) simili ai _reference_. Come con i _reference_, i puntatori
grezzi possono essere immutabili o mutabili e si scrivono `*const T` e `*mut T`
rispettivamente. L’asterisco non è l’operatore di de-referenziazione; fa parte
del nome del _type_. Nel contesto dei puntatori grezzi, _immutabile_ significa
che il puntatore non può essere assegnato direttamente dopo essere stato
de-referenziato.

Diversamente da _reference_ e puntatori intelligenti, i puntatori grezzi:

- Possono ignorare le regole di _borrowing_ avendo sia puntatori immutabili che
  mutabili o molteplici puntatori mutabili allo stesso dato
- Non è garantito che puntino a memoria valida
- Possono essere nulli
- Non fanno nessuna pulizia automatica

Rinunciando a far rispettare queste garanzie da parte di Rust, puoi rinunciare
alla sicurezza garantita in cambio di maggiori prestazioni o della possibilità
di interfacciarti con altri linguaggi o hardware dove le garanzie di Rust non
valgono.

Il Listato 20-1 mostra come creare un puntatore grezzo immutabile e uno
mutabile.

<Listing number="20-1" caption="Creazione di puntatori grezzi con gli operatori di prestito grezzi">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-01/src/main.rs:here}}
```

</Listing>

Nota che in questo codice non usiamo la parola chiave `unsafe`. Possiamo creare
puntatori grezzi in codice _safe_; non possiamo però de-referenziarli fuori da
un blocco _unsafe_, come vedremo tra poco.

Abbiamo creato puntatori grezzi usando gli operatori di prestito grezzi (_raw
borrow_): `&raw const num` crea un puntatore grezzo immutabile `*const i32`,
mentre `&raw mut num` crea un puntatore grezzo mutabile `*mut i32`. Siccome li
abbiamo creati direttamente da una variabile locale, sappiamo che questi
puntatori grezzi sono validi, ma non possiamo fare questa assunzione per
qualsiasi puntatore grezzo.

Per dimostrarlo, creiamo un puntatore grezzo di cui non possiamo essere così
certi che sia valido, usando la parola chiave `as` per fare un _cast_ invece di
usare l’operatore di prestito grezzo. Il Listato 20-2 mostra come creare un
puntatore grezzo verso una posizione arbitraria in memoria. Usare un indirizzo
di memoria arbitrario è un comportamento indefinito: potrebbe esserci qualche
dato a quell’indirizzo o magari no, il compilatore potrebbe ottimizzare il
codice e evitare l’accesso alla memoria, oppure il programma potrebbe terminare
con un errore accesso non valido alla memoria. Di solito non c’è una buona
ragione per scrivere codice così, specialmente quando si può usare un operatore
di prestito grezzo, ma è possibile farlo.

<Listing number="20-2" caption="Creazione di un puntatore grezzo a un indirizzo di memoria arbitrario">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-02/src/main.rs:here}}
```

</Listing>

Ricorda che possiamo creare puntatori grezzi in codice _safe_, però non possiamo
de-referenziarli per leggere i dati a cui puntano. Nel Listato 20-3 usiamo
l’operatore di de-referenziazione `*` su un puntatore grezzo che richiede un
blocco _unsafe_.

<Listing number="20-3" caption="De-referenziazione di puntatore grezzo dentro un blocco `unsafe`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-03/src/main.rs:here}}
```

Creare un puntatore non fa danno; è solo quando proviamo a leggere il valore a
cui punta che potremmo avere a che fare con un valore invalido.

Nota anche che nei Listati 20-1 e 20-3 abbiamo creato puntatori grezzi `*const
i32` e `*mut i32` dove entrambi puntavano alla stessa locazione di memoria, dove
si trova `num`. Se invece avessimo provato a creare un _reference_ immutabile e
uno mutabile a `num`, il codice non sarebbe stato compilato perché le regole di
_ownership_ di Rust non permettono un _reference_ mutabile contemporaneamente a
_reference_ immutabili. Con i puntatori grezzi possiamo creare un puntatore
mutabile e uno immutabile sugli stessi dati in memoria e modificarli tramite il
puntatore mutabile, creando potenzialmente una _data race_. Fai attenzione!

Con tutti questi pericoli, perché mai usare i puntatori grezzi? Un uso molto
comune è quando ci si interfaccia con codice in C, come vedremo nella prossima
sezione. Un altro caso è quando si costruiscono astrazioni _safe_ che il _borrow
checker_ non capisce. Introdurremo le funzioni _unsafe_ e poi vedremo un esempio
di astrazione _safe_ che usa codice _unsafe_.

### Chiamare una Funzione o Metodo _Unsafe_

Il secondo tipo di operazione che puoi fare in un blocco _unsafe_ è chiamare
funzioni _unsafe_. Le funzioni e i metodi _unsafe_ sembrano esattamente normali
funzioni e metodi, ma hanno `unsafe` prima della definizione. La parola chiave
`unsafe` qui indica che la funzione ha dei requisiti che dobbiamo rispettare
quando la chiamiamo, perché Rust non può garantire che li rispettiamo. Chiamando
una funzione _unsafe_ dentro un blocco _unsafe_ stiamo dicendo che abbiamo letto
la documentazione di quella funzione e ci assumiamo la responsabilità di
rispettarne i contratti.

Ecco una funzione _unsafe_ chiamata `pericolosa` che non fa nulla nel corpo:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-01-unsafe-fn/src/main.rs:here}}
```

Dobbiamo chiamare la funzione `pericolosa` dentro un blocco `unsafe` separato.
Se proviamo a chiamarla senza il blocco _unsafe_, avremo un errore:

```console
{{#include ../listings/ch20-advanced-features/output-only-01-missing-unsafe/output.txt}}
```

Con il blocco _unsafe_, stiamo dicendo a Rust che abbiamo letto la
documentazione della funzione, sappiamo come usarla correttamente e abbiamo
verificato di rispettare il contratto.

Per fare operazioni _unsafe_ dentro una funzione _unsafe_, serve comunque un
blocco _unsafe_ anche dentro il corpo, e il compilatore ti avvertirà se lo
dimentichi. Questo ci aiuta a tenere i blocchi _unsafe_ più piccoli possibile,
perché spesso non servono in tutto il corpo della funzione.

#### Creare un’Astrazione _Safe_ su Codice _Unsafe_

Solo perché una funzione contiene codice _unsafe_ non significa che debba essere
tutta marcata come _unsafe_. Infatti, incapsulare codice _unsafe_ in una
funzione _safe_ è una pratica comune. Come esempio, studiamo la funzione
`split_at_mut` della libreria standard, che richiede un po’ di codice _unsafe_.
Vedremo come potremmo implementarla. Questo metodo _safe_ è definito per _slice_
mutabili: prende una _slice_ e la divide in due a partire da un indice passato
come argomento. Il Listato 20-4 mostra come usare `split_at_mut`.

<Listing number="20-4" caption="Uso della funzione _safe_ `split_at_mut`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-04/src/main.rs:here}}
```

</Listing>

Non possiamo implementare questa funzione usando solo _safe Rust_. Una prova
potrebbe essere il Listato 20-5, che non si compila. Per semplicità,
implementeremo `split_at_mut` come funzione invece che come metodo, e solo per
_slice_ di `i32`, non per un _type_ generico `T`.

<Listing number="20-5" caption="Tentativo di implementazione di `split_at_mut` usando solo _safe_ Rust">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-05/src/main.rs:here}}
```

</Listing>

Questa funzione prima prende la lunghezza totale della _slice_. Poi assicura che
l’indice passato sia entro la _slice_, verificando che sia minore o uguale alla
lunghezza. Questa asserzione significa che se passiamo un indice maggiore della
lunghezza, la funzione farà _panic_ prima di usare quell’indice.

Poi ritorna due _slice_ mutabili in una tupla: una dalla parte iniziale fino a
`mid` e l’altra da `mid` fino alla fine della _slice_.

Quando proviamo a compilare il codice in Listato 20-5, otteniamo un errore:

```console
{{#include ../listings/ch20-advanced-features/listing-20-05/output.txt}}
```

Il _borrow checker_ di Rust non capisce che stiamo prendendo due parti diverse
della stessa _slice_; sa solo che stiamo prendendo la stessa _slice_ due volte.
Prendere in prestito parti diverse di una stessa _slice_ è fondamentalmente non
problematico perché le due _slice_ non si sovrappongono, ma Rust non è
abbastanza intelligente da capire questo. Quando sappiamo che il codice va bene,
ma Rust no, è ora di usare codice _unsafe_.

Il Listato 20-6 mostra come usare un blocco _unsafe_, un puntatore grezzo e
alcune chiamate a funzioni _unsafe_ per far funzionare `split_at_mut`.

<Listing number="20-6" caption="Uso di codice _unsafe_ nell’implementazione della funzione `split_at_mut`">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-06/src/main.rs:here}}
```

</Listing>

Ricordiamo da ["Il _Type_ _Slice_"][the-slice-type]<!-- ignore --> nel capitolo
4 che una _slice_ è un puntatore a un dato e la sua lunghezza. Usiamo il metodo
`len` per avere la lunghezza e il metodo `as_mut_ptr` per accedere al puntatore
grezzo della _slice_. In questo caso, poiché abbiamo una _slice_ mutabile di
`i32`, `as_mut_ptr` ritorna un puntatore grezzo di _type_ `*mut i32` che
conserviamo nella variabile `ptr`.

Manteniamo l’asserzione che `mid` stia dentro la _slice_. Poi arriviamo al
codice _unsafe_: la funzione `slice::from_raw_parts_mut` prende un puntatore
grezzo e una lunghezza e crea una _slice_. La usiamo per creare una _slice_ che
parte da `ptr` ed è lunga `mid` elementi. Poi chiamiamo il metodo `add` su `ptr`
con `mid` come argomento per ottenere un puntatore grezzo che punta a `mid`, e
creiamo una _slice_ usando quel puntatore e la lunghezza rimanente dopo `mid`.

La funzione `slice::from_raw_parts_mut` è _unsafe_ perché prende un puntatore
grezzo e deve fidarsi che quel puntatore sia valido. Anche il metodo `add` su
puntatore grezzo è _unsafe_ perché deve fidarsi che la locazione di memoria
puntata sia valida. Per questo abbiamo messo un blocco _unsafe_ attorno alle
nostre chiamate a `slice::from_raw_parts_mut` e `add` per poterle chiamare.
Guardando il codice e aggiungendo l’asserzione che `mid` deve essere minore o
uguale a `len`, possiamo dire che tutti i puntatori grezzi usati nel blocco
_unsafe_ saranno validi e punteranno a dati nella _slice_. Questo è un uso
accettabile e appropriato di `unsafe`.

Nota che non dobbiamo marcare la funzione `split_at_mut` risultante come
`unsafe` e possiamo chiamarla da _safe Rust_. Abbiamo creato un’astrazione
_safe_ su codice _unsafe_ con un’implementazione che usa codice _unsafe_ in modo
_safe_, perché crea solo puntatori validi dai dati a cui quella funzione ha
accesso.

Al contrario, usare `slice::from_raw_parts_mut` come nel Listato 20-7
probabilmente terminerebbe con un errore quando si usa la _slice_. Quel codice
prende una locazione arbitraria di memoria e crea una _slice_ lunga 10.000
elementi.

<Listing number="20-7" caption="Creazione di una _slice_ da una locazione arbitraria di memoria">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-07/src/main.rs:here}}
```

</Listing>

Non abbiamo _ownership_ della memoria in quella posizione arbitraria, e non ci
sono garanzie che la _slice_ così creata contenga valori `i32` validi. Usare
quei valori come se fosse una _slice_ valida è comportamento indefinito.

#### Usare Funzioni _extern_ per Chiamare Codice Esterno

A volte il tuo codice Rust potrebbe aver bisogno di interagire con codice
scritto in un altro linguaggio. Per questo, Rust ha la parola chiave `extern`
che facilita la creazione e l’uso di una _interfaccia per funzioni esterne_,
abbreviato in _FFI_ (_Foreign Function Interface_), cioè un modo per un
linguaggio di definire funzioni e consentire a un diverso linguaggio (esterno)
di chiamarle.

Il Listato 20-8 mostra come impostare l’integrazione con la funzione `abs` dalla
libreria standard di C. Le funzioni dichiarate dentro blocchi `extern` sono
generalmente _unsafe_ da chiamare da codice Rust, quindi anche i blocchi
`extern` devono essere marcati come _unsafe_. Il motivo è che altri linguaggi
non impongono le regole e garanzie di Rust, e Rust non può controllarle, quindi
la responsabilità è del programmatore.

<Listing number="20-8" file-name="src/main.rs" caption="Dichiarazione e chiamata di una funzione `extern` definita in un altro linguaggio">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-08/src/main.rs}}
```

</Listing>

Dentro il blocco `unsafe extern "C"`, elenchiamo nomi e firme delle funzioni
esterne di un altro linguaggio che vogliamo chiamare. La parte `"C"` definisce
quale _interfaccia binaria dell'applicazione_, abbreviato in _ABI_ (_application
binary interface_), quella funzione usa: l’_ABI_ definisce come chiamare la
funzione a livello assembly. L’_ABI_ `"C"` è il più comune ed è l’_ABI_ del
linguaggio C. Informazioni su tutte le _ABI_ supportate da Rust sono disponibili
nella [Rust Reference][ABI].

Ogni elemento dichiarato dentro un blocco `unsafe extern` è implicitamente
_unsafe_. Però, alcune funzioni _FFI_ *sono* sicure da chiamare. Per esempio, la
funzione `abs` della libreria standard C non ha considerazioni di sicurezza
della memoria di cui preoccuparsi e sappiamo che può essere chiamata con
qualunque `i32`. In questi casi possiamo usare la parola chiave `safe` per dire
che quella funzione specifica è sicura da chiamare anche se si trova dentro un
blocco `unsafe extern`. Dopo questa modifica, chiamarla non richiede più un
blocco _unsafe_, come mostra il Listato 20-9.

<Listing number="20-9" file-name="src/main.rs" caption="Marcatura esplicita di una funzione come _safe_ dentro un blocco `unsafe extern` e chiamata in maniera sicura">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-09/src/main.rs}}
```

</Listing>

Marcare una funzione come `safe` non la rende automaticamente sicura! È come una
promessa a Rust che è sicura. Sta comunque a te fare in modo che la promessa sia
mantenuta!

#### Chiamare Funzioni Rust da Altri Linguaggi

Possiamo anche usare `extern` per creare un’interfaccia che permetta ad altri
linguaggi di chiamare funzioni Rust. Invece di creare un blocco `extern`
completo, mettiamo la parola chiave `extern` e specifichiamo l’_ABI_ da usare
subito prima della parola `fn` per la funzione interessata. Dobbiamo anche
aggiungere l’annotazione `#[unsafe(no_mangle)]` per disabilitare il _mangling_
da parte del compilatore per quella funzione. Il _mangling_ è quando un
compilatore cambia il nome di una funzione in un nome diverso che contiene più
info per altre parti della compilazione, ma è meno leggibile dall’uomo. Ogni
linguaggio compila in modo diverso, quindi per permettere a una funzione Rust di
essere chiamata da altri linguaggi dobbiamo disabilitare il _name mangling_, ma
questo è _unsafe_ perché potrebbero esserci collisioni di nomi tra varie
librerie, quindi sta a noi scegliere un nome sicuro da esportare senza
_mangling_.

Nell’esempio seguente rendiamo la funzione `call_from_c` accessibile da codice
C, dopo essere stata compilata in una libreria condivisa e collegata dal C:

```rust
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Chiamata una funzione Rust da C!");
}
```

Questo uso di `extern` richiede `unsafe` solo nell’attributo, non nel blocco
`extern`.

#### Accedere o Modificare una Variabile Statica Mutabile

Nel libro finora non abbiamo parlato di variabili globali, che Rust supporta ma
che possono dare problemi con le regole di _ownership_. Se due _thread_ accedono
contemporaneamente alla stessa variabile globale mutabile, può succedere una
_data race_.

In Rust, le variabili globali si chiamano variabili _static_. Il Listato 20-10
mostra un esempio di dichiarazione e uso di una variabile statica con una
_slice_ di stringa come valore.

<Listing number="20-10" file-name="src/main.rs" caption="Definizione e uso di una variabile statica immutabile">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-10/src/main.rs}}
```

</Listing>

Le variabili statiche sono simili alle costanti, di cui abbiamo parlato in
[“Dichiarare le Costanti”][constants]<!-- ignore --> nel Capitolo 3. I nomi
delle variabili statiche sono, per convenzione, in `SNAKE_CASE_MAIUSCOLO`. Le
variabili statiche possono contenere solo _reference_ con longevità `'static`,
quindi il compilatore Rust può ricavarne la _lifetime_ e non serve annotarla
esplicitamente. Accedere a una variabile statica immutabile è sicuro.

Una sottile differenza tra costanti e variabili statiche immutabili è che i
valori in una variabile statica hanno un indirizzo fisso in memoria. Usare quel
valore significa sempre accedere agli stessi dati. Le costanti invece possono
duplicare i dati ogni volta che sono usate. Un’altra differenza è che le
variabili statiche possono essere mutabili. Accedere e modificare variabili
statiche mutabili è _unsafe_. Il Listato 20-11 mostra come dichiarare, accedere
e modificare una variabile statica mutabile chiamata `CONTATORE`.

<Listing number="20-11" file-name="src/main.rs" caption="Lettura o scrittura su una variabile statica mutabile è _unsafe_">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-11/src/main.rs}}
```

</Listing>

Come per le variabili normali, specifichiamo la mutabilità con la parola chiave
`mut`. Qualsiasi codice che legge o scrive da `CONTATORE` deve stare dentro un
blocco _unsafe_. Il codice nel Listato 20-11 viene compilato e stampa
`CONTATORE: 3`, come ci aspettiamo perché è a singolo _thread_. Se più _thread_
accedessero a `CONTATORE` probabilmente si creerebbero _data races_, quindi è un
comportamento indefinito. Per questo dobbiamo marcare tutta la funzione come
_unsafe_ e documentarne i limiti di sicurezza, così chi la chiama sa cosa può e
non può fare in sicurezza.

Quando scriviamo una funzione _unsafe_, è pratica comune scrivere un commento
che inizi con `SAFETY` per spiegare cosa deve fare chi chiama la funzione per
farla funzionare in sicurezza. Allo stesso modo, quando facciamo un’operazione
_unsafe_, scriviamo un commento con `SAFETY` per spiegare come vengono
rispettate le regole di sicurezza.

Il compilatore blocca di default ogni tentativo di creare _reference_ a
variabili statiche mutabili tramite i controlli del _linter_. Devi quindi o
disabilitare esplicitamente il _lint_ con `#[allow(static_mut_refs)]` o accedere
alla variabile statica mutabile tramite un puntatore grezzo creato con uno degli
operatori di prestito grezzi. Questo include i casi in cui il _reference_ è
creato in modo invisibile, come quando è usato in `println!` in quel codice.
Richiedere che i _reference_ alle variabili statiche mutabili siano creati
tramite puntatore grezzo rende più evidente quali sono i requisiti di sicurezza
per usarle.

Con dati mutabili che sono accessibili globalmente, è difficile assicurarsi che
non ci siano _data race_, motivo per cui Rust considera le variabili statiche
mutabili _unsafe_. Quando possibile, è preferibile usare tecniche di concorrenza
e puntatori intelligenti _thread-safe_ di cui abbiamo parlato nel Capitolo 16,
così il compilatore verifica che l’accesso da _thread_ diversi sia sicuro.

### Implementare un _Trait_ _Unsafe_

Possiamo usare `unsafe` per implementare un _trait_ _unsafe_. Un _trait_ è
_unsafe_ quando almeno uno dei suoi metodi ha una proprietà che il compilatore
non può verificare. Dichiariamo un _trait_ _unsafe_ mettendo la parola chiave
`unsafe` prima di `trait` e marcando anche l’implementazione del _trait_ come
_unsafe_, come mostra il Listato 20-12.

<Listing number="20-12" caption="Definizione e implementazione di un _trait_ _unsafe_">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-12/src/main.rs:here}}
```

</Listing>

Usando `unsafe impl` promettiamo che rispetteremo le proprietà che il
compilatore non può verificare.

Per esempio, ricordiamo i _trait_ marcatori `Send` e `Sync` di cui abbiamo
parlato in [“Concorrenza Estensibile con `Send` e
`Sync`”][extensible-concurrency]<!-- ignore --> nel Capitolo 16: il compilatore
implementa automaticamente questi _trait_ se i nostri _type_ sono composti solo
da _type_ che implementano `Send` e `Sync`. Se implementiamo un _type_ che
contiene un _type_ che non implementa `Send` o `Sync`, come i puntatori grezzi
ad esempio, e vogliamo marcare quel _type_ come `Send` o `Sync`, dobbiamo usare
`unsafe`. Rust non può verificare che il nostro _type_ rispetti le garanzie per
poterlo spostare in sicurezza tra _thread_ o essere usato da più _thread_,
quindi dobbiamo fare quei controlli manualmente e indicarlo con `unsafe`.

### Accedere ai Campi di una _Union_

L’ultima cosa che si può fare solo con _unsafe_ è accedere ai campi di una
_union_. Una _union_ è simile a una `struct`, ma solo uno dei campi dichiarati è
usato in una certa istanza in un dato momento. Le _union_ sono usate soprattutto
per interfacciarsi con le _union_ di codice C. Accedere ai campi di una _union_
è _unsafe_ perché Rust non può garantire il tipo dei dati conservati in quel
momento nell’istanza di _union_. Puoi imparare di più sulle _union_ nella [Rust
Reference][unions].

### Usare Miri per Controllare il Codice _Unsafe_

Quando scrivi codice _unsafe_, potresti voler controllare che quello che hai
scritto sia davvero sicuro e corretto. Uno dei modi migliori per farlo è usare
_Miri_, uno strumento ufficiale Rust per rilevare comportamenti indefiniti.
Mentre il _borrow checker_ è uno strumento _statico_ che lavora durante la
compilazione, _Miri_ è uno strumento _dinamico_ che lavora durante l'esecuzione.
Controlla il tuo codice eseguendo il programma o i vari test e rilevando quando
violi le regole che conosce su come dovrebbe funzionare Rust.

Usare _Miri_ richiede una build _nightly_ di Rust (di cui parliamo di più
nell'[Appendice G: Come è Fatto Rust e “Nightly Rust”][nightly]<!-- ignore -->).
Puoi installare sia la versione _nightly_ di Rust che lo strumento _Miri_
digitando `rustup +nightly component add miri`. Questo non cambia la versione di
Rust che usa il tuo progetto; aggiunge solo lo strumento al tuo sistema per
poterlo usare quando vuoi. Puoi far girare _Miri_ su un progetto digitando
`cargo +nightly miri run` o `cargo +nightly miri test`.

Per esempio, guarda cosa succede se lo usiamo con il codice nel Listato 20-7.

```console
{{#include ../listings/ch20-advanced-features/listing-20-07/output.txt}}
```

Miri ci avverte correttamente che stiamo facendo un _cast_ da intero a
puntatore, che potrebbe essere un problema, ma _Miri_ non può sapere se lo è
dato che non conosce l’origine del puntatore. Poi _Miri_ segnala un errore
perché il Listato 20-7 ha un comportamento indefinito dovuto a un puntatore
pendente. Grazie a _Miri_, sappiamo che c’è un rischio di comportamento
indefinito e possiamo pensare a come mettere in sicurezza il codice. In certi
casi _Miri_ può perfino suggerire come correggere gli errori.

_Miri_ non cattura tutto quello che potresti sbagliare scrivendo codice
_unsafe_. È uno strumento di analisi dinamica, quindi cattura solo i problemi
nel codice che viene realmente eseguito. Questo significa che devi usarlo
insieme a buone tecniche di testing per aumentare la fiducia nel codice _unsafe_
che hai scritto. _Miri_ non copre tutti i possibili modi in cui il tuo codice
può essere insicuro.

In altre parole: se _Miri_ rileva un problema, sai che c’è un bug, ma non è
detto che se _Miri_ non trova bug, il codice sia sicuro. Però in molti casi
aiuta davvero tanto. Prova a farlo girare sugli altri esempi di codice _unsafe_
in questo capitolo e vedi cosa dice!

Puoi saperne di più su _Miri_ nel [suo repository GitHub][miri].

### Quando Usare il Codice _Unsafe_

Usare `unsafe` per sfruttare uno dei cinque superpoteri appena visti non è
sbagliato o malvisto, ma è più difficile scrivere codice _unsafe_ corretto
perché il compilatore non può garantire la sicurezza della memoria. Quando hai
una buona ragione per usare codice _unsafe_, puoi farlo, e avere la marcatura
esplicita `unsafe` ti aiuta a rintracciare più facilmente la fonte di problemi
quando capitano. Ogni volta che scrivi codice _unsafe_, puoi usare _Miri_ per
essere più sicuro che il codice scritto rispetti le regole di Rust.

Per una trattazione molto più approfondita su come lavorare efficacemente con
_unsafe Rust_, leggi la guida ufficiale di Rust sull’argomento, il
[Rustonomicon][nomicon].

[dangling-references]: ch04-02-references-and-borrowing.html#reference-pendenti
[ABI]: https://doc.rust-lang.org/stable/reference/items/external-blocks.html#abi
[constants]: ch03-01-variables-and-mutability.html#dichiarare-le-costanti
[extensible-concurrency]: ch16-04-extensible-concurrency-sync-and-send.html#concorrenza-estensibile-con-send-e-sync
[the-slice-type]: ch04-03-slices.html#il-type-slice
[unions]: https://doc.rust-lang.org/stable/reference/items/unions.html
[miri]: https://github.com/rust-lang/miri
[nightly]: appendix-07-nightly-rust.html
[nomicon]: https://doc.rust-lang.org/nomicon/
