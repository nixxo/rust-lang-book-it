## Sintassi dei Pattern

In questa sezione, raccogliamo tutta la sintassi valida nei pattern e discutiamo
perché e quando potresti voler utilizzare ciascuna di esse.

### Corrispondenza di Letterali

Come hai visto nel Capitolo 6, puoi confrontare i pattern direttamente con i letterali. Il
codice seguente fornisce alcuni esempi:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-01-literals/src/main.rs:here}}
```

Questo codice stampa `uno` perché il valore in `x` è `1`. Questa sintassi è utile
quando vuoi che il codice esegua un'azione se ottiene un particolare valore
concreto.

### Corrispondenza di Variabili Denominate

Le variabili denominate sono pattern inconfutabili che corrispondono a qualsiasi valore e le abbiamo utilizzate
molte volte in questo libro. Tuttavia, si verifica una complicazione quando si utilizzano
variabili con nome nelle espressioni `match`, `if let` o `while let`. Poiché ciascuna
di queste espressioni inizia un nuovo scope, le variabili dichiarate come parte di
un pattern all'interno di queste espressioni copriranno quelle con lo stesso nome all'esterno
dei costrutti, come nel caso di tutte le variabili. Nel Listato 19-11, dichiariamo
una variabile denominata `x` con il valore `Some(5)` e una variabile `y` con il valore
`10`. Creiamo quindi un'espressione `match` sul valore `x`. Osserviamo i
pattern nel campo `match` e `println!` alla fine, e cerchiamo di capire
cosa stamperà il codice prima di eseguirlo o di proseguire con la lettura.

<Listing number="19-11" file-name="src/main.rs" caption="Un'espressione `match` con un braccio che introduce una nuova variabile che oscura una variabile esistente `y`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-11/src/main.rs:here}}
```

</Listing>

Esaminiamo cosa succede quando viene eseguita l'espressione `match`. Il pattern
nel primo braccio di corrispondenza non corrisponde al valore definito di `x`, quindi il codice
continua.

Il pattern nel secondo ramo di corrispondenza introduce una nuova variabile denominata `y` che
corrisponderà a qualsiasi valore all'interno di un valore `Some`. Poiché ci troviamo in un nuovo ambito all'interno
dell'espressione `match`, questa è una nuova variabile `y`, non la `y` che abbiamo dichiarato
all'inizio con il valore `10`. Questo nuovo `y` corrisponderà a qualsiasi valore
all'interno di `Some`, che è ciò che abbiamo in `x`. Pertanto, questo nuovo `y` si lega al
valore interno di `Some` in `x`. Quel valore è `5`, quindi l'espressione per
quel braccio viene eseguita e stampa `Matched, y = 5`.

Se `x` fosse stato un valore `None` invece di `Some(5)`, i pattern nei primi
due bracci non avrebbero trovato corrispondenza, quindi il valore avrebbe trovato corrispondenza con il
trattino basso. Non abbiamo introdotto la variabile `x` nel pattern del
braccio con trattino basso, quindi la `x` nell'espressione è ancora la `x` esterna che non è stata
ombreggiata. In questo caso ipotetico, `match` stamperebbe `Default case,
x = None`.

Quando l'espressione `match` è terminata, il suo ambito termina, così come quello della `y` interna. L'ultimo `println!` produce `alla fine: x = Some(5), y = 10`.

Per creare un'espressione `match` che confronti i valori delle variabili esterne `x` e
`y`, anziché introdurre una nuova variabile che oscura la variabile `y`
esistente, dovremmo usare una condizione di controllo della corrispondenza. Parleremo
delle condizioni di controllo della corrispondenza più avanti in [“Aggiungere espressioni condizionali con le condizioni di controllo della corrispondenza”](#aggiungere-istruzioni-condizionali-con-le-match-guard)<!-- ignore -->.

### Corrispondenza di più Pattern

Nelle espressioni `match`, è possibile confrontare più pattern utilizzando la sintassi `|`,
che è l'operatore di controllo della corrispondenza _or_. Ad esempio, nel codice seguente confrontiamo
il valore di `x` con i valori corrispondenti, il primo dei quali ha un'opzione _or_,
il che significa che se il valore di `x` corrisponde a uno dei valori in quel valore,
verrà eseguito il codice di quel valore:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-02-multiple-patterns/src/main.rs:here}}
```

Questo codice stampa `uno o due`.

### Corrispondenza di Intervalli di Valori con `..=`

La sintassi `..=` ci consente di confrontare un intervallo di valori inclusivo. Nel
codice seguente, quando un pattern corrisponde a uno qualsiasi dei valori all'interno dell'intervallo
indicato, quel braccio verrà eseguito:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-03-ranges/src/main.rs:here}}
```

Se `x` è `1`, `2`, `3`, `4` o `5`, il primo braccio corrisponderà. Questa sintassi è
più comoda per più valori di corrispondenza rispetto all'utilizzo dell'operatore `|` per
esprimere la stessa idea; se dovessimo usare `|`, dovremmo specificare `1 | 2 |
3 | 4 | 5`. Specificare un intervallo è molto più breve, soprattutto se vogliamo trovare una corrispondenza,
ad esempio, con un numero qualsiasi compreso tra 1 e 1.000!

Il compilatore verifica che l'intervallo non sia vuoto in fase di compilazione e, poiché
gli unici tipi per cui Rust può stabilire se un intervallo è vuoto o meno sono `char` e
i valori numerici, gli intervalli sono consentiti solo con valori numerici o `char`.

Ecco un esempio che utilizza intervalli di valori `char`:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-04-ranges-of-char/src/main.rs:here}}
```

Rust can tell that `'c'` is within the first pattern’s range and prints `early
ASCII letter`.

### Destructuring to Break Apart Values

We can also use patterns to destructure structs, enums, and tuples to use
different parts of these values. Let’s walk through each value.

#### Structs

Listing 19-12 shows a `Point` struct with two fields, `x` and `y`, that we can
break apart using a pattern with a `let` statement.

<Listing number="19-12" file-name="src/main.rs" caption="Destructuring a struct’s fields into separate variables">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-12/src/main.rs}}
```

</Listing>

Questo codice crea le variabili `a` e `b` che corrispondono ai valori dei campi `x`
e `y` della struttura `p`. Questo esempio mostra che i nomi delle
variabili nel pattern non devono necessariamente corrispondere ai nomi dei campi della struttura.
Tuttavia, è comune far corrispondere i nomi delle variabili ai nomi dei campi per
rendere più facile ricordare quali variabili provengono da quali campi. A causa di questo
uso comune, e poiché scrivere `let Point { x: x, y: y } = p;` contiene
molte duplicazioni, Rust ha una scorciatoia per i pattern che corrispondono ai campi della struttura:
è sufficiente elencare il nome del campo della struttura e le variabili create
dal pattern avranno gli stessi nomi. Il Listato 19-13 si comporta allo stesso
modo del codice del Listato 19-12, ma le variabili create nel pattern `let`
sono `x` e `y` invece di `a` e `b`.

<Listing number="19-13" file-name="src/main.rs" caption="Destrutturazione dei campi struct tramite abbreviazione dei campi struct">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-13/src/main.rs}}
```

</Listing>

Questo codice crea le variabili `x` e `y` che corrispondono ai campi `x` e `y`
della variabile `p`. Il risultato è che le variabili `x` e `y` contengono i
valori della struct `p`.

Possiamo anche destrutturare con valori letterali come parte del pattern struct
piuttosto che creare variabili per tutti i campi. In questo modo possiamo testare
alcuni campi per valori specifici mentre creiamo variabili per
destrutturare gli altri campi.

Nel Listato 19-14, abbiamo un'espressione `match` che separa i valori `Point`
in tre casi: punti che giacciono direttamente sull'asse `x` (che è vero quando
`y = 0`), sull'asse `y` (`x = 0`) o su nessuno dei due assi.

<Listing number="19-14" file-name="src/main.rs" caption="Destrutturazione e corrispondenza di valori letterali in un unico pattern">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-14/src/main.rs:here}}
```

</Listing>

Il primo caso corrisponderà a qualsiasi punto che si trovi sull'asse `x` specificando che
il campo `y` corrisponde se il suo valore corrisponde al letterale `0`. Il pattern crea comunque
una variabile `x` che possiamo utilizzare nel codice per questo ramo.

Analogamente, il secondo caso corrisponde a qualsiasi punto sull'asse `y` specificando che
il campo `x` corrisponde se il suo valore è `0` e crea una variabile `y` per il
valore del campo `y`. Il terzo ramo non specifica alcun letterale, quindi
corrisponde a qualsiasi altro `Point` e crea variabili per entrambi i campi `x` e `y`.

In questo esempio, il valore `p` corrisponde al secondo ramo in virtù del fatto che `x`
contiene uno `0`, quindi questo codice stamperà `Sull'asse y a 7`.

Ricorda che un'espressione `match` interrompe il controllo dei rami una volta trovato il
primo pattern corrispondente, quindi anche se `Point { x: 0, y: 0}` si trova sull'asse `x`
e sull'asse `y`, questo codice stamperà solo `Sull'asse x a 0`.

#### Enum

Abbiamo destrutturato gli enum in questo libro (ad esempio, Listato 6-5 nel Capitolo 6),
ma non abbiamo ancora spiegato esplicitamente che il pattern per destrutturare un enum
corrisponde al modo in cui sono definiti i dati memorizzati all'interno dell'enum. Ad esempio,
nel Listato 19-15 utilizziamo l'enum `Messaggio` del Listato 6-2 e scriviamo
un `match` con pattern che destruttureranno ogni valore interno.

<Listing number="19-15" file-name="src/main.rs" caption="Destrutturazione delle varianti di enum che contengono diversi tipi di valori">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-15/src/main.rs}}
```

</Listing>

Questo codice stamperà `Cambia colore in rosso 0, verde 160 e blu 255`. Prova
a modificare il valore di `msg` per vedere l'esecuzione del codice degli altri bracci.

Per le varianti di enum senza dati, come `Messaggio::Esci`, non possiamo destrutturare
ulteriormente il valore. Possiamo trovare corrispondenze solo sul valore letterale `Messaggio::Esci`,
e non ci sono variabili in quel pattern.

Per varianti di enum di tipo struct, come `Messaggio::Muovi`, possiamo usare un pattern
simile a quello che specifichiamo per la corrispondenza con le struct. Dopo il nome della variante,
inseriamo parentesi graffe e poi elenchiamo i campi con le variabili, in modo da separare
i pezzi da usare nel codice per questo braccio. Qui usiamo la forma abbreviata come
abbiamo fatto nel Listato 19-13.

Per varianti di enum di tipo tuple, come `Messaggio::Scrivi` che contiene una tupla con un
elemento e `Messaggio::CambiaColore` che contiene una tupla con tre elementi, il
pattern è simile a quello che specifichiamo per la corrispondenza con le tuple. Il numero di
variabili nel pattern deve corrispondere al numero di elementi nella variante che stiamo
correlando.

#### Strutture ed Enumerazioni Annidate

Finora, i nostri esempi hanno tutti confrontato strutture o enumerazioni con un solo livello di profondità,
ma la corrispondenza può funzionare anche su elementi annidati! Ad esempio, possiamo rifattorizzare il
codice nel Listato 19-15 per supportare i colori RGB e HSV nel messaggio `CambiaColore`
come mostrato nel Listato 19-16.

<Listing number="19-16" caption="Corrispondenza su enumerazioni annidate">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-16/src/main.rs}}
```

</Listing>

Il pattern del primo braccio nell'espressione `match` corrisponde a una
variante dell'enumerazione `Messaggio::CambiaColore` che contiene una variante `Colore::Rgb`; quindi
il pattern si lega ai tre valori `i32` interni. Il pattern del secondo
braccio corrisponde anche a una variante dell'enum `Messaggio::CambiaColore`, ma l'enum interno
corrisponde invece a `Colore::Hsv`. Possiamo specificare queste condizioni complesse in un'unica
espressione `match`, anche se sono coinvolte due enum.

#### Strutture e Tuple

Possiamo combinare, abbinare e annidare pattern di destrutturazione in modi ancora più complessi. L'esempio seguente mostra una destrutturazione complessa in cui annidiamo strutture e
tuple all'interno di una tupla e destrutturamo tutti i valori primitivi:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-05-destructuring-structs-and-tuples/src/main.rs:here}}
```

Questo codice ci permette di scomporre i tipi complessi nelle loro parti componenti in modo da poter utilizzare
i valori che ci interessano separatamente.

La destrutturazione con i pattern è un modo comodo per utilizzare parti di valori, come
il valore di ciascun campo in una struttura, separatamente l'uno dall'altro.

### Ignorare Valori In un _Pattern_

Avete visto che a volte è utile ignorare i valori in un pattern, come
nell'ultimo ramo di un `match`, per ottenere un catch-all che in realtà non fa
nulla ma tiene conto di tutti i valori possibili rimanenti. Esistono diversi
modi per ignorare interi valori o parti di valori in un pattern: utilizzare il pattern `_`
(che avete visto), utilizzare il pattern `_` all'interno di un altro pattern,
utilizzare un nome che inizia con un trattino basso o utilizzare `..` per ignorare le parti rimanenti
di un valore. Esploriamo come e perché utilizzare ciascuno di questi pattern.

#### Un Valore Intero con `_`

Abbiamo utilizzato il trattino basso come pattern jolly che corrisponde a qualsiasi valore ma
non si lega al valore. Questo è particolarmente utile come ultimo ramo in un'espressione `match`
, ma possiamo utilizzarlo anche in qualsiasi pattern, inclusi i parametri di funzione,
come mostrato nel Listato 19-17.

<Listing number="19-17" file-name="src/main.rs" caption="Usare `_` in una firma di funzione">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-17/src/main.rs}}
```

</Listing>

Questo codice ignorerà completamente il valore `3` passato come primo argomento
e stamperà `Questa funzione usa solo il parametro y: 4`.

Nella maggior parte dei casi, quando non è più necessario un particolare parametro di funzione,
si modifica la firma in modo che non includa il parametro non utilizzato. Ignorare
un parametro di funzione può essere particolarmente utile nei casi in cui, ad esempio,
si sta implementando un tratto per il quale è necessaria una certa firma di tipo, ma il
corpo della funzione nell'implementazione non richiede uno dei parametri. Si
evita così di ricevere un avviso del compilatore sui parametri di funzione non utilizzati, come si farebbe
utilizzando un nome.

<a id="ignoring-parts-of-a-value-with-a-nested-_"></a>

#### Parti di un Valore con un `_` Annidato

Possiamo anche usare `_` all'interno di un altro pattern per ignorare solo una parte di un valore, ad
esempio, quando vogliamo testare solo una parte di un valore ma non abbiamo bisogno delle
altre parti nel codice corrispondente che vogliamo eseguire. Il Listato 19-18 mostra il codice
responsabile della gestione del valore di un'impostazione. I requisiti aziendali prevedono che
l'utente non possa sovrascrivere una personalizzazione esistente di un'impostazione, ma possa annullarla e assegnarle un valore se è attualmente annullata.

<Listing number="19-18" caption="Utilizzo di un trattino basso all'interno di pattern che corrispondono alle varianti di `Some` quando non è necessario utilizzare il valore all'interno di `Some`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-18/src/main.rs:here}}
```

</Listing>

Questo codice stamperà `Non è possibile sovrascrivere un valore personalizzato esistente` e poi
`Valore setting è Some(5)`. Nel primo ramo di corrispondenza, non è necessario abbinare o utilizzare
i valori all'interno di una delle varianti `Some`, ma è necessario testare il caso
in cui `valore_setting` e `nuovo_valore_setting` siano la variante `Some`. In tal
caso, stampiamo il motivo per cui `valore_setting` non viene modificato, e non viene
modificato.

In tutti gli altri casi (se `valore_setting` o `nuovo_valore_setting` è `None`)
espressi dal pattern `_` nel secondo braccio, vogliamo consentire a `nuovo_valore_setting` di diventare `valore_setting`.

Possiamo anche utilizzare caratteri di sottolineatura in più punti all'interno di un pattern per ignorare
valori specifici. Il Listato 19-19 mostra un esempio di come ignorare il secondo e il
quarto valore in una tupla di cinque elementi.

<Listing number="19-19" caption="Ignorare più parti di una tupla">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-19/src/main.rs:here}}
```

</Listing>

Questo codice stamperà `Alcuni numeri: 2, 8, 32`, e i valori `4` e `16` saranno
ignorati.

#### Una Variabile Inutilizzata Iniziando il suo Nome con `_`

Se si crea una variabile ma non la si utilizza da nessuna parte, Rust di solito genera un
avviso perché una variabile inutilizzata potrebbe essere un bug. Tuttavia, a volte è
utile poter creare una variabile che non si utilizzerà ancora, ad esempio quando si
sta realizzando un prototipo o si sta appena iniziando un progetto. In questa situazione, puoi dire a Rust
di non avvisarti della variabile inutilizzata iniziando il nome della variabile
con un trattino basso. Nel Listato 19-20, creiamo due variabili inutilizzate, ma quando
compiliamo questo codice, dovremmo ricevere un avviso solo per una di esse.

<Listing number="19-20" file-name="src/main.rs" caption="Iniziare il nome di una variabile con un trattino basso per evitare di ricevere avvisi di variabili inutilizzate">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-20/src/main.rs}}
```

</Listing>

Qui, riceviamo un avviso sul mancato utilizzo della variabile `y`, ma non riceviamo un avviso sul mancato utilizzo di `_x`.

Si noti che c'è una sottile differenza tra l'utilizzo di solo `_` e l'utilizzo di un nome
che inizia con un trattino basso. La sintassi `_x` vincola comunque il valore alla
variabile, mentre `_` non lo vincola affatto. Per mostrare un caso in cui questa
distinzione è importante, il Listato 19-21 ci fornirà un errore.

<Listing number="19-21" caption="Una variabile inutilizzata che inizia con un trattino basso vincola comunque il valore, che potrebbe assumerne la proprietà">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-21/src/main.rs:here}}
```

</Listing>

Riceveremo un errore perché il valore `s` verrà comunque spostato in `_s`,
il che ci impedisce di utilizzare nuovamente `s`. Tuttavia, l'utilizzo del trattino basso da solo
non vincola mai il valore. Il Listato 19-22 verrà compilato senza errori
perché `s` non viene spostato in `_`.

<Listing number="19-22" caption="L'utilizzo di un trattino basso non vincola il valore">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-22/src/main.rs:here}}
```

</Listing>

Questo codice funziona perfettamente perché non vincola mai `s` a nulla; non viene spostato.

<a id="ignoring-remaining-parts-of-a-value-with-"></a>

#### Parti Rimanenti di un Valore con `..`

Con valori composti da molte parti, possiamo usare la sintassi `..` per usare parti specifiche
e ignorare il resto, evitando la necessità di elencare caratteri di sottolineatura per ogni
valore ignorato. Il pattern `..` ignora qualsiasi parte di un valore che non abbiamo
corrisposto esplicitamente nel resto del pattern. Nel Listato 19-23, abbiamo una
struttura `Punto` che contiene una coordinata nello spazio tridimensionale. Nell'espressione `match`, vogliamo operare solo sulla coordinata `x` e ignorare
i valori nei campi `y` e `z`.

<Listing number="19-23" caption="Ignorare tutti i campi di un `Point` tranne `x` usando `..`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-23/src/main.rs:here}}
```

</Listing>

Elenchiamo il valore `x` e poi includiamo semplicemente il pattern `..`. Questo è più veloce
che dover elencare `y: _` e `z: _`, soprattutto quando lavoriamo con
strutture che hanno molti campi in situazioni in cui solo uno o due campi sono
rilevanti.

La sintassi `..` si espanderà a tutti i valori necessari. Il Listato 19-24
mostra come usare `..` con una tupla.

<Listing number="19-24" file-name="src/main.rs" caption="Corrispondenza solo del primo e dell'ultimo valore in una tupla e ignorando tutti gli altri valori">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-24/src/main.rs}}
```

</Listing>

In questo codice, il primo e l'ultimo valore vengono confrontati con `primo` e `ultimo`.
`..` corrisponderà e ignorerà tutto ciò che si trova nel mezzo.

Tuttavia, l'utilizzo di `..` deve essere univoco. Se non è chiaro quali valori siano
destinati alla corrispondenza e quali debbano essere ignorati, Rust restituirà un errore.
Il Listato 19-25 mostra un esempio di utilizzo ambiguo di `..`, quindi non verrà
compilato.

<Listing number="19-25" file-name="src/main.rs" caption="Un tentativo di utilizzare `..` in modo ambiguo">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-25/src/main.rs}}
```

</Listing>

Quando compiliamo questo esempio, otteniamo questo errore:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-25/output.txt}}
```

È impossibile per Rust determinare quanti valori nella tupla ignorare
prima di abbinare un valore con `secondo` e poi quanti altri valori
ignorare successivamente. Questo codice potrebbe significare che vogliamo ignorare `2`, associare
`secondo` a `4` e quindi ignorare `8`, `16` e `32`; oppure che vogliamo ignorare
`2` e `4`, associare `secondo` a `8` e quindi ignorare `16` e `32`; e così via.
Il nome della variabile `secondo` non ha alcun significato particolare in Rust, quindi otteniamo un
errore del compilatore perché usare `..` in due punti come questo è ambiguo.

### Aggiungere Istruzioni Condizionali con le Match Guard

Una _match guard_ è una condizione `if` aggiuntiva, specificata dopo il pattern in
un ramo `match`, che deve corrispondere affinché quel ramo venga scelto. Le Match Guard sono
utili per esprimere idee più complesse di quelle consentite da un solo pattern. Si noti,
tuttavia, che sono disponibili solo nelle espressioni `match`, non nelle espressioni `if let` o
`while let`.

La condizione può utilizzare variabili create nel pattern. Il Listato 19-26 mostra un `match` in cui il primo ramo ha il pattern `Some(x)` e ha anche una match guard `if x % 2 == 0` (che sarà `true` se il numero è pari).

<Listing number="19-26" caption="Aggiungere una match guard a un pattern">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-26/src/main.rs:here}}
```

</Listing>

Questo esempio stamperà `Il numero 4 è pari`. Quando `num` viene confrontato con il
pattern nel primo ramo, corrisponde perché `Some(4)` corrisponde a `Some(x)`. Quindi
la match guard controlla se il resto della divisione di `x` per 2 è uguale a
0 e, poiché lo è, viene selezionato il primo ramo.

Se `num` fosse stato `Some(5)`, la match guard nel primo ramo sarebbe stata `false` perché il resto di 5 diviso 2 è 1, che è diverso da
0. Rust passerebbe quindi al secondo ramo, che corrisponderebbe perché il
secondo ramo non ha una match guard e quindi corrisponde a qualsiasi variante di `Some`.

Non c'è modo di esprimere la condizione `if x % 2 == 0` all'interno di un pattern, quindi
la match guard ci dà la possibilità di esprimere questa logica. Lo svantaggio di
questa espressività aggiuntiva è che il compilatore non cerca di verificare
l'esaustività quando sono coinvolte espressioni di match guard.

Nel Listato 19-11, abbiamo accennato alla possibilità di utilizzare le match guards per risolvere il nostro
problema di pattern-shadowing. Ricordiamo che abbiamo creato una nuova variabile all'interno del
pattern nell'espressione `match` invece di utilizzare la variabile esterna a
`match`. Questa nuova variabile ci impediva di testare il valore della
variabile esterna. Il Listato 19-27 mostra come possiamo usare una match guard per risolvere questo
problema.

<Listing number="19-27" file-name="src/main.rs" caption="Usare una match guard per verificare l'uguaglianza con una variabile esterna">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-27/src/main.rs}}
```

</Listing>

Questo codice ora stamperà `Caso predefinito, x = Some(5)`. Il pattern nel secondo
ramo di corrispondenza non introduce una nuova variabile `y` che oscurerebbe la `y` esterna,
il che significa che possiamo usare la `y` esterna nella match guard. Invece di specificare il
pattern come `Some(y)`, che avrebbe oscurato la `y` esterna, specifichiamo
`Some(n)`. Questo crea una nuova variabile `n` che non oscura nulla perché
non esiste alcuna variabile `n` al di fuori della `match`.

La clausola di controllo `if n == y` non è un pattern e quindi non introduce nuove
variabili. Questa `y` _è_ la `y` esterna anziché una nuova `y` che la oscura, e
possiamo cercare un valore che abbia lo stesso valore della `y` esterna confrontando
`n` con `y`.

È anche possibile utilizzare l'operatore _or_ `|` in una clausola di controllo per specificare più
pattern; la condizione di controllo si applicherà a tutti i pattern. Il Listato
19-28 mostra la precedenza quando si combina un pattern che utilizza `|` con una clausola di controllo. La parte importante di questo esempio è che la match guard `if y`
si applica a `4`, `5` e `6`, anche se potrebbe sembrare che `if y`
si applichi solo a `6`.

<Listing number="19-28" caption="Combinazione di più pattern con una match guard">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-28/src/main.rs:here}}
```

</Listing>

La condizione di corrispondenza stabilisce che il ramo corrisponde solo se il valore di `x` è
uguale a `4`, `5` o `6` e se `y` è `true`. Quando questo codice viene eseguito, il
pattern del primo ramo corrisponde perché `x` è `4`, ma la match guard `if y`
è `false`, quindi il primo ramo non viene scelto. Il codice passa al secondo
ramo, che corrisponde, e questo programma stampa `no`. Il motivo è che la
condizione `if` si applica all'intero pattern `4 | 5 | 6`, non solo all'ultimo
valore `6`. In altre parole, la precedenza di una match guard rispetto a un
pattern si comporta in questo modo:

```text
(4 | 5 | 6) if y => ...
```

piuttosto che in questo modo:

```text
4 | 5 | (6 if y) => ...
```

Dopo aver eseguito il codice, il comportamento della precedenza è evidente: se la match guard
fosse stata applicata solo al valore finale nell'elenco di valori specificato utilizzando l'operatore
`|`, armil ramo avrebbe trovato una corrispondenza e il programma avrebbe stampato
`yes`.

### Utilizzo dei Binding `@`

L'operatore _at_ `@` ci consente di creare una variabile che contiene un valore mentre
stiamo testando quel valore per una corrispondenza con il pattern. Nel Listato 19-29, vogliamo
verificare che un campo `Message::Hello` `id` sia compreso nell'intervallo `3..=7`. Vogliamo anche
associare il valore alla variabile `id` in modo da poterlo utilizzare nel codice
associato al ramo.

<Listing number="19-29" caption="Usare `@` per associare un valore in un pattern e testarlo">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-29/src/main.rs:here}}
```

</Listing>

Questo esempio stamperà `Trovato un id nell'intervallo: 5`. Specificando `id @` prima
dell'intervallo `3..=7`, catturiamo qualsiasi valore corrispondente all'intervallo in una
variabile denominata `id`, verificando anche che il valore corrisponda al pattern dell'intervallo.

Nel secondo ramo, dove abbiamo specificato solo un intervallo nel pattern, il codice
associato al ramo non ha una variabile che contenga il valore effettivo
del campo `id`. Il valore del campo `id` avrebbe potuto essere 10, 11 o 12, ma
il codice associato a quel pattern non sa quale sia. Il codice del pattern
non è in grado di utilizzare il valore del campo `id`, perché non abbiamo salvato il
valore `id` in una variabile.

Nell'ultimo ramo, dove abbiamo specificato una variabile senza intervallo, abbiamo
il valore disponibile da utilizzare nel codice del ramo in una variabile denominata `id`. Il
motivo è che abbiamo utilizzato la sintassi abbreviata del campo struct. Ma non abbiamo
applicato alcun test al valore del campo `id` in questo ramo, come abbiamo fatto con
i primi due rami: qualsiasi valore corrisponderebbe a questo pattern.

L'utilizzo di `@` ci consente di testare un valore e salvarlo in una variabile all'interno di un pattern.

## Riepilogo

I pattern di Rust sono molto utili per distinguere tra diversi tipi di
dati. Quando vengono utilizzati nelle espressioni `match`, Rust garantisce che i pattern coprano ogni
valore possibile, altrimenti il programma non verrà compilato. I pattern nelle istruzioni `let` e
nei parametri di funzione rendono questi costrutti più utili, consentendo la
destrutturazione dei valori in parti più piccole e l'assegnazione di tali parti a
variabili. Possiamo creare pattern semplici o complessi in base alle nostre esigenze.

Successivamente, nel penultimo capitolo del libro, esamineremo alcuni aspetti
avanzati di una varietà di funzionalità di Rust.
