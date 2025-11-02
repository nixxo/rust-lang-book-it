## Sintassi dei _Pattern_

In questa sezione, raccogliamo tutta la sintassi valida nei _pattern_ e
discutiamo perché e quando potresti voler utilizzare ciascuna di esse.

### Corrispondenza di Letterali

Come hai visto nel Capitolo 6, puoi confrontare i _pattern_ direttamente con i
letterali. Il codice seguente fornisce alcuni esempi:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-01-literals/src/main.rs:here}}
```

Questo codice stampa `uno` perché il valore in `x` è `1`. Questa sintassi è
utile quando vuoi che il codice esegua un’azione se ottiene un particolare
valore concreto.

### Corrispondenza di Variabili Denominate

Le variabili denominate sono _pattern_ inconfutabili che corrispondono a
qualsiasi valore e le abbiamo utilizzate molte volte in questo libro. Tuttavia,
si verifica una complicazione quando si utilizzano variabili denominate nelle
espressioni `match`, `if let` o `while let`. Poiché ciascuna di queste
espressioni inizia un nuovo _scope_, le variabili dichiarate come parte di un
_pattern_ all’interno di queste espressioni oscureranno quelle con lo stesso
nome all’esterno dei costrutti, come è il caso di tutte le variabili. Nel
Listato 19-11, dichiariamo una variabile denominata `x` con il valore `Some(5)`
e una variabile `y` con il valore `10`. Creiamo quindi un’espressione `match`
sul valore `x`. Osserva i _pattern_ nel campo `match` e `println!` alla fine, e
cerca di capire cosa stamperà il codice prima di eseguirlo o di proseguire con
la lettura.

<Listing number="19-11" file-name="src/main.rs" caption="Un’espressione `match` con un ramo che introduce una nuova variabile che oscura una variabile esistente `y`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-11/src/main.rs:here}}
```

</Listing>

Esaminiamo cosa succede quando viene eseguita l’espressione `match`. Il
_pattern_ nel primo ramo di corrispondenza non corrisponde al valore definito di
`x`, quindi il codice continua.

Il _pattern_ nel secondo ramo di corrispondenza introduce una nuova variabile
denominata `y` che corrisponderà a qualsiasi valore all’interno di un valore
`Some`. Poiché ci troviamo in un nuovo _scope_ all’interno dell’espressione
`match`, questa è una nuova variabile `y`, non la `y` che abbiamo dichiarato
all’inizio con il valore `10`. Questa nuova `y` corrisponderà a qualsiasi valore
all’interno di `Some`, che è ciò che abbiamo in `x`. Pertanto, questa nuova `y`
si lega al valore interno di `Some` in `x`. Quel valore è `5`, quindi
l’espressione per quel ramo viene eseguita e stampa `Corrisponde, y = 5`.

Se `x` fosse stato un valore `None` invece di `Some(5)`, i _pattern_ nei primi
due rami non avrebbero trovato corrispondenza, quindi il valore avrebbe trovato
corrispondenza con il trattino basso. Non abbiamo introdotto la variabile `x`
nel _pattern_ del ramo con trattino basso, quindi la `x` nell’espressione è
ancora la `x` esterna che non è stata oscurata. In questo caso ipotetico,
`match` stamperebbe `Caso predefinito, x = None`.

Quando l’espressione `match` termina, il suo _scope_ termina, così come quello
della `y` interna. L’ultimo `println!` produce `alla fine: x = Some(5), y = 10`.

Per creare un’espressione `match` che confronti i valori delle variabili esterne
`x` e `y`, anziché introdurre una nuova variabile che oscura la variabile `y`
esistente, dovremmo usare una condizione di controllo della corrispondenza. Ne
parleremo più avanti nella sezione [“Aggiungere Istruzioni Condizionali con le
Match Guard”](#aggiungere-istruzioni-condizionali-con-le-match-guard)<!-- ignore
-->.

### Corrispondenza di Più _Pattern_

Nelle espressioni `match`, è possibile confrontare più _pattern_ utilizzando la
sintassi `|`, che è l’operatore _OR_ di controllo della corrispondenza. Ad
esempio, nel codice seguente confrontiamo il valore di `x` con i valori
corrispondenti, il primo dei quali ha un’opzione _or_, il che significa che se
il valore di `x` corrisponde a l’uno o l’altro dei valori del ramo, verrà
eseguito il codice di quel ramo:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-02-multiple-patterns/src/main.rs:here}}
```

Questo codice stampa `uno o due`.

### Corrispondenza di Intervalli di Valori con `..=`

La sintassi `..=` ci consente di confrontare con un intervallo di valori
inclusivo. Nel codice seguente, quando un _pattern_ corrisponde a uno qualsiasi
dei valori all’interno dell’intervallo indicato, quel ramo verrà eseguito:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-03-ranges/src/main.rs:here}}
```

Se `x` è `1`, `2`, `3`, `4` o `5`, il primo ramo corrisponderà. Questa sintassi
è più comoda per più valori di corrispondenza rispetto all’utilizzo
dell’operatore `|` per esprimere la stessa idea; se dovessimo usare `|`,
dovremmo specificare `1 | 2 | 3 | 4 | 5`. Specificare un intervallo è molto più
conciso, soprattutto se vogliamo trovare una corrispondenza, ad esempio, con un
numero qualsiasi compreso tra 1 e 1.000!

Il compilatore verifica che l’intervallo non sia vuoto in fase di compilazione
e, poiché gli unici _type_ per cui Rust può stabilire se un intervallo è vuoto o
meno sono `char` e i valori numerici, gli intervalli sono consentiti solo con
valori numerici o `char`.

Ecco un esempio che utilizza intervalli di valori `char`:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-04-ranges-of-char/src/main.rs:here}}
```

Rust può capire che `'c'` si trova all‘interno del primo intervallo e stamperà
`lettere ASCII iniziali`.

### Destrutturare per Separare Valori

Possiamo inoltre usare i _pattern_ per destrutturare _struct_, _enum_ e _tuple_
per estrarre parti diverse di questi valori. Passiamo in rassegna ognuna di
queste strutture dati.

#### _Struct_

Il Listato 19-12 mostra una _struct_ `Punto` con due campi, `x` e `y`, che
possiamo separare usando un _pattern_ con una dichiarazione `let`.

<Listing number="19-12" file-name="src/main.rs" caption="Destrutturazione dei campi di una _struct_ in variabili separate">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-12/src/main.rs}}
```

</Listing>

Questo codice crea le variabili `a` e `b` che corrispondono ai valori dei campi
`x` e `y` della _struct_ `p`. Questo esempio mostra che i nomi delle variabili
nel _pattern_ non devono necessariamente corrispondere ai nomi dei campi della
_struct_. Tuttavia, è comune far corrispondere i nomi delle variabili ai nomi
dei campi per rendere più facile ricordare quali variabili provengono da quali
campi. A causa di questo uso comune, e poiché scrivere `let Punto { x: x, y: y }
= p;` contiene molte duplicazioni, Rust ha una scorciatoia per i _pattern_ che
corrispondono ai campi della _struct_: è sufficiente elencare il nome del campo
della _struct_ e le variabili create dal _pattern_ avranno gli stessi nomi. Il
Listato 19-13 si comporta allo stesso modo del codice del Listato 19-12, ma le
variabili create nel _pattern_ `let` sono `x` e `y` invece di `a` e `b`.

<Listing number="19-13" file-name="src/main.rs" caption="Destrutturazione dei campi _struct_ tramite dichiarazione abbreviata">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-13/src/main.rs}}
```

</Listing>

Questo codice crea le variabili `x` e `y` che corrispondono ai campi `x` e `y`
della variabile `p`. Il risultato è che le variabili `x` e `y` contengono i
valori della _struct_ `p`.

Possiamo anche destrutturare con valori letterali come parte del _pattern_
_struct_ piuttosto che creare variabili per tutti i campi. In questo modo
possiamo testare alcuni campi per valori specifici mentre creiamo variabili per
destrutturare gli altri campi.

Nel Listato 19-14, abbiamo un’espressione `match` che separa i valori `Punto` in
tre casi: punti che giacciono direttamente sull’asse `x` (che è vero quando `y =
0`), sull’asse `y` (`x = 0`) o su nessuno dei due assi.

<Listing number="19-14" file-name="src/main.rs" caption="Destrutturazione e corrispondenza di valori letterali in un unico _pattern_">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-14/src/main.rs:here}}
```

</Listing>

Il primo caso corrisponderà a qualsiasi punto che si trovi sull’asse `x`
specificando che il campo `y` corrisponde se il suo valore corrisponde al
letterale `0`. Il _pattern_ crea comunque una variabile `x` che possiamo
utilizzare nel codice per questo ramo.

Analogamente, il secondo caso corrisponde a qualsiasi punto sull’asse `y`
specificando che il campo `x` corrisponde se il suo valore è `0` e crea una
variabile `y` per il valore del campo `y`. Il terzo ramo non specifica alcun
letterale, quindi corrisponde a qualsiasi altro `Punto` e crea variabili per
entrambi i campi `x` e `y`.

In questo esempio, il valore `p` corrisponde al secondo ramo in virtù del fatto
che `x` contiene uno `0`, quindi questo codice stamperà `Sull’asse y a 7`.

Ricorda che un’espressione `match` interrompe il controllo dei rami una volta
trovato il primo _pattern_ corrispondente, quindi anche se `Punto { x: 0, y: 0
}` si trova sull’asse `x` e sull’asse `y`, questo codice stamperà solo
`Sull'asse x a 0`.

#### _Enum_

Abbiamo destrutturato gli _enum_ in questo libro (ad esempio, Listato 6-5 nel
Capitolo 6), ma non abbiamo ancora spiegato esplicitamente che il _pattern_ per
destrutturare un’_enum_ corrisponde al modo in cui sono definiti i dati
memorizzati all’interno dell’_enum_ stesso. Ad esempio, nel Listato 19-15
utilizziamo l’_enum_ `Messaggio` del Listato 6-2 e scriviamo un `match` con
_pattern_ che destruttureranno ogni valore interno.

<Listing number="19-15" file-name="src/main.rs" caption="Destrutturazione delle varianti di _enum_ che contengono diversi tipi di valori">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-15/src/main.rs}}
```

</Listing>

Questo codice stamperà `Cambia colore in rosso 0, verde 160 e blu 255`. Prova a
modificare il valore di `msg` per vedere l’esecuzione del codice degli altri
rami.

Per le varianti di _enum_ senza dati, come `Messaggio::Esci`, non possiamo
destrutturare ulteriormente il valore. Possiamo trovare corrispondenze solo sul
valore letterale `Messaggio::Esci`, e non ci sono variabili in quel _pattern_.

Per varianti di _enum_ di tipo _struct_, come `Messaggio::Muovi`, possiamo usare
un _pattern_ simile a quello che specifichiamo per la corrispondenza con le
_struct_. Dopo il nome della variante, inseriamo parentesi graffe e poi
elenchiamo i campi con le variabili, in modo da separare i pezzi da usare nel
codice per questo ramo. Qui usiamo la forma abbreviata come abbiamo fatto nel
Listato 19-13.

Per varianti di _enum_ di _type_ tupla, come `Messaggio::Scrivi` che contiene
una tupla con un elemento e `Messaggio::CambiaColore` che contiene una tupla con
tre elementi, il _pattern_ è simile a quello che specifichiamo per la
corrispondenza con le tuple. Il numero di variabili nel _pattern_ deve
corrispondere al numero di elementi nella variante che stiamo correlando.

#### _Struct_ ed _Enum_ Annidate

Finora, i nostri esempi hanno tutti confrontato _struct_ o _enum_ con un solo
livello di profondità, ma la corrispondenza può funzionare anche su elementi
annidati! Ad esempio, possiamo riscrivere il codice nel Listato 19-15 per
supportare i colori RGB e HSV nel messaggio `CambiaColore` come mostrato nel
Listato 19-16.

<Listing number="19-16" caption="Corrispondenza su _enum_ annidate">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-16/src/main.rs}}
```

</Listing>

Il _pattern_ del primo ramo nell’espressione `match` corrisponde a una variante
dell’_enum_ `Messaggio::CambiaColore` che contiene una variante `Colore::Rgb`;
quindi il _pattern_ si lega ai tre valori `i32` interni. Anche il _pattern_ del
secondo ramo corrisponde a una variante dell’_enum_ `Messaggio::CambiaColore`,
ma l’_enum_ interno corrisponde invece a `Colore::Hsv`. Possiamo specificare
queste condizioni complesse in un’unica espressione `match`, anche se sono
coinvolte due _enum_.

#### _Struct_ e Tuple

Possiamo combinare, abbinare e annidare _pattern_ di destrutturazione in modi
ancora più complessi. L’esempio seguente mostra una destrutturazione complessa
in cui annidiamo _struct_ e tuple all’interno di una tupla e destrutturiamo
tutti i valori primitivi:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-05-destructuring-structs-and-tuples/src/main.rs:here}}
```

Questo codice ci permette di scomporre i _type_ complessi nelle loro componenti
in modo da poter utilizzare i valori che ci interessano separatamente.

La destrutturazione con i _pattern_ è un modo comodo per utilizzare parti di
valori, come il valore di ciascun campo in una _struct_, separatamente l’uno
dall’altro.

### Ignorare Valori In un _Pattern_

Hai visto che a volte è utile ignorare i valori in un _pattern_, come
nell’ultimo ramo di un `match`, per ottenere un pigliatutto che in realtà non fa
nulla ma tiene conto di tutti i valori possibili rimanenti. Esistono diversi
modi per ignorare interi valori o parti di valori in un _pattern_: utilizzare il
_pattern_ `_` (che hai visto), utilizzare il _pattern_ `_` all’interno di un
altro _pattern_, utilizzare un nome che inizia con un trattino basso o
utilizzare `..` per ignorare le parti rimanenti di un valore. Esploriamo come e
perché utilizzare ciascuno di questi _pattern_.

#### Un Intero Valore con `_`

Abbiamo utilizzato il trattino basso come _pattern_ pigliatutto che corrisponde
a qualsiasi valore ma non si lega al valore. Questo è particolarmente utile come
ultimo ramo in un’espressione `match` , ma possiamo utilizzarlo anche in
qualsiasi _pattern_, inclusi i parametri di funzione, come mostrato nel Listato
19-17.

<Listing number="19-17" file-name="src/main.rs" caption="Usare `_` in una firma di funzione">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-17/src/main.rs}}
```

</Listing>

Questo codice ignorerà completamente il valore `3` passato come primo argomento
e stamperà `Questa funzione usa solo il parametro y: 4`.

Nella maggior parte dei casi, quando non è più necessario un particolare
parametro di funzione, si modifica la firma in modo che non includa il parametro
non utilizzato. Ignorare un parametro di funzione può essere particolarmente
utile nei casi in cui, ad esempio, si sta implementando un _trait_ per il quale
è necessaria una certa firma di _type_, ma il corpo della funzione
nell’implementazione non richiede uno dei parametri. Si evita così di ricevere
un avviso del compilatore sui parametri di funzione non utilizzati, come si
farebbe utilizzando un nome.

#### Parti di un Valore Con un `_` Annidato

Possiamo anche usare `_` all’interno di un altro _pattern_ per ignorare solo una
parte di un valore, ad esempio, quando vogliamo testare solo una parte di un
valore ma non abbiamo bisogno delle altre parti nel codice corrispondente che
vogliamo eseguire. Il Listato 19-18 mostra il codice responsabile della gestione
del valore di un’impostazione. I requisiti aziendali prevedono che l’utente non
possa sovrascrivere una personalizzazione esistente di un’impostazione, ma possa
annullarla e assegnarle un valore se è attualmente non impostata.

<Listing number="19-18" caption="Utilizzo di un trattino basso all’interno di _pattern_ che corrispondono alle varianti `Some` quando non è necessario utilizzare il valore all’interno di `Some`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-18/src/main.rs:here}}
```

</Listing>

Questo codice stamperà `Non è possibile sovrascrivere un valore personalizzato
esistente` e poi `Valore impostazioni è Some(5)`. Nel primo ramo di
corrispondenza, non è necessario abbinare o utilizzare i valori all’interno di
una delle varianti `Some`, ma è necessario testare il caso in cui
`val_impostazioni` e `nuovo_val_impostazioni` siano la variante `Some`. In tal
caso, stampiamo il motivo della mancata modifica di `val_impostazioni`, e quindi
non viene modificato.

In tutti gli altri casi (se `val_impostazioni` o `nuovo_val_impostazioni` è
`None`) espressi dal _pattern_ `_` nel secondo ramo, vogliamo consentire a
`nuovo_val_impostazioni` di diventare `val_impostazioni`.

Possiamo anche utilizzare il trattino basso in più punti all’interno di un
_pattern_ per ignorare valori specifici. Il Listato 19-19 mostra un esempio di
come ignorare il secondo e il quarto valore in una tupla di cinque elementi.

<Listing number="19-19" caption="Ignorare più parti di una tupla">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-19/src/main.rs:here}}
```

</Listing>

Questo codice stamperà `Alcuni numeri: 2, 8, 32`, e i valori `4` e `16` saranno
ignorati.

#### Una Variabile Inutilizzata Iniziando il Suo Nome con `_`

Se si crea una variabile ma non la si utilizza da nessuna parte, Rust di solito
genera un avviso perché una variabile inutilizzata potrebbe essere un bug.
Tuttavia, a volte è utile poter creare una variabile che non si utilizzerà
ancora, ad esempio quando si sta realizzando un prototipo o si è nelle prime
fasi di un progetto. In questa situazione, puoi dire a Rust di non avvisarti
della variabile inutilizzata facendo iniziare il nome della variabile con un
trattino basso. Nel Listato 19-20, creiamo due variabili inutilizzate, ma quando
compiliamo questo codice, dovremmo ricevere un avviso solo per una di esse.

<Listing number="19-20" file-name="src/main.rs" caption="Far iniziare il nome di una variabile con un trattino basso per evitare di ricevere avvisi di variabili inutilizzate">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-20/src/main.rs}}
```

</Listing>

Qui, riceviamo un avviso sul mancato utilizzo della variabile `y`, ma non
riceviamo un avviso sul mancato utilizzo di `_x`.

Nota che c’è una sottile differenza tra l’utilizzo di solo `_` e l’utilizzo di
un nome che inizia con un trattino basso. La sintassi `_x` vincola comunque il
valore alla variabile, mentre `_` non lo vincola affatto. Per mostrare un caso
in cui questa distinzione è importante, il Listato 19-21 ci fornirà un errore.

<Listing number="19-21" caption="Una variabile inutilizzata che inizia con un trattino basso vincola comunque il valore, che potrebbe assumerne la proprietà">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-21/src/main.rs:here}}
```

</Listing>

Riceveremo un errore perché il valore `s` verrà comunque spostato in `_s`, il
che ci impedisce di utilizzare nuovamente `s`. Tuttavia, l’utilizzo del trattino
basso da solo non vincola mai il valore. Il Listato 19-22 verrà compilato senza
errori perché `s` non viene spostato in `_`.

<Listing number="19-22" caption="L’utilizzo di un trattino basso non vincola il valore">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-22/src/main.rs:here}}
```

</Listing>

Questo codice funziona perfettamente perché non vincola mai `s` a nulla; non
viene spostato.

#### Parti Rimanenti di un Valore con `..`

Con valori composti da molte parti, possiamo usare la sintassi `..` per usare
parti specifiche e ignorare il resto, evitando la necessità di elencare trattini
bassi per ogni valore ignorato. Il _pattern_ `..` ignora qualsiasi parte di un
valore che non abbiamo corrisposto esplicitamente nel resto del _pattern_. Nel
Listato 19-23, abbiamo una _struct_ `Punto` che contiene coordinate nello spazio
tridimensionale. Nell’espressione `match`, vogliamo operare solo sulla
coordinata `x` e ignorare i valori nei campi `y` e `z`.

<Listing number="19-23" caption="Ignorare tutti i campi di un `Punto` tranne `x` usando `..`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-23/src/main.rs:here}}
```

</Listing>

Elenchiamo il valore `x` e poi includiamo semplicemente il _pattern_ `..`.
Questo è più veloce che dover elencare `y: _` e `z: _`, soprattutto quando
lavoriamo con _struct_ che hanno molti campi in situazioni in cui solo uno o due
campi sono rilevanti.

La sintassi `..` si espanderà a tutti i valori necessari. Il Listato 19-24
mostra come usare `..` con una tupla.

<Listing number="19-24" file-name="src/main.rs" caption="Corrispondenza solo del primo e dell’ultimo valore in una tupla e ignorando tutti gli altri valori">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-24/src/main.rs}}
```

</Listing>

In questo codice, il primo e l’ultimo valore vengono confrontati con `primo` e
`ultimo`. `..` corrisponderà e ignorerà tutto ciò che si trova nel mezzo.

Tuttavia, l’utilizzo di `..` deve essere univoco. Se non è chiaro quali valori
siano destinati alla corrispondenza e quali debbano essere ignorati, Rust
restituirà un errore. Il Listato 19-25 mostra un esempio di utilizzo ambiguo di
`..`, che quindi non verrà compilato.

<Listing number="19-25" file-name="src/main.rs" caption="Un tentativo di utilizzare `..` in modo ambiguo">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-25/src/main.rs}}
```

</Listing>

Quando compiliamo questo esempio, otteniamo questo errore:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-25/output.txt}}
```

È impossibile per Rust determinare quanti valori nella tupla ignorare prima di
abbinare un valore con `secondo` e poi quanti altri valori ignorare
successivamente. Questo codice potrebbe significare che vogliamo ignorare `2`,
associare `secondo` a `4` e quindi ignorare `8`, `16` e `32`; oppure che
vogliamo ignorare `2` e `4`, associare `secondo` a `8` e quindi ignorare `16` e
`32`; e così via. Il nome della variabile `secondo` non ha alcun significato
particolare in Rust, quindi otteniamo un errore del compilatore perché usare
`..` in due punti come questo è ambiguo.

### Aggiungere Istruzioni Condizionali con le _Match Guard_

Una _match guard_ è una condizione `if` aggiuntiva, specificata dopo il
_pattern_ in un ramo `match`, che deve corrispondere affinché quel ramo venga
scelto. Le _match guard_ sono utili per esprimere idee più complesse di quelle
consentite dal solo _pattern_. Nota, tuttavia, che sono disponibili solo nelle
espressioni `match`, non nelle espressioni `if let` o `while let`.

La condizione può utilizzare variabili create nel _pattern_. Il Listato 19-26
mostra un `match` in cui il primo ramo ha il _pattern_ `Some(x)` e ha anche una
_match guard_ `if x % 2 == 0` (che sarà `true` se il numero è pari).

<Listing number="19-26" caption="Aggiungere una _match guard_ a un _pattern_">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-26/src/main.rs:here}}
```

</Listing>

Questo esempio stamperà `Il numero 4 è pari`. Quando `num` viene confrontato con
il _pattern_ nel primo ramo, corrisponde perché `Some(4)` corrisponde a
`Some(x)`. Quindi la _match guard_ controlla se il resto della divisione di `x`
per 2 è uguale a 0 e, poiché lo è, viene selezionato il primo ramo.

Se `num` fosse stato `Some(5)`, la _match guard_ nel primo ramo sarebbe stata
`false` perché il resto di 5 diviso 2 è 1, che è diverso da 0. Rust passerebbe
quindi al secondo ramo, che corrisponderebbe perché il secondo ramo non ha una
_match guard_ e quindi corrisponde a qualsiasi variante di `Some`.

Non c’è modo di esprimere la condizione `if x % 2 == 0` all’interno di un
_pattern_, quindi la _match guard_ ci dà la possibilità di esprimere questa
logica. Lo svantaggio di questa espressività aggiuntiva è che il compilatore non
cerca di verificare l’esaustività quando sono coinvolte espressioni con _match
guard_.

Nel Listato 19-11, avevamo accennato alla possibilità di utilizzare le _match
guard_ per risolvere il nostro problema di oscuramento della variabili. Ricorda
che avevamo creato una nuova variabile all’interno del _pattern_
nell’espressione `match` invece di utilizzare la variabile esterna a `match`.
Questa nuova variabile ci impediva di testare il valore della variabile esterna.
Il Listato 19-27 mostra come possiamo usare una _match guard_ per risolvere
questo problema.

<Listing number="19-27" file-name="src/main.rs" caption="Usare una _match guard_ per verificare l’uguaglianza con una variabile esterna">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-27/src/main.rs}}
```

</Listing>

Questo codice ora stamperà `Caso predefinito, x = Some(5)`. Il _pattern_ nel
secondo ramo di corrispondenza non introduce una nuova variabile `y` che
oscurerebbe la `y` esterna, il che significa che possiamo usare la `y` esterna
nella _match guard_. Invece di specificare il _pattern_ come `Some(y)`, che
avrebbe oscurato la `y` esterna, specifichiamo `Some(n)`. Questo crea una nuova
variabile `n` che non oscura nulla perché non esiste alcuna variabile `n` al di
fuori del `match`.

La _match guard_ `if n == y` non è un _pattern_ e quindi non introduce nuove
variabili. Questa `y` _è_ la `y` esterna anziché una nuova `y` che la oscura, e
possiamo cercare un valore che abbia lo stesso valore della `y` esterna
confrontando `n` con `y`.

È anche possibile utilizzare l’operatore _or_ `|` in una _match guard_ per
specificare più _pattern_; la _match guard_ si applicherà a tutti i _pattern_.
Il Listato 19-28 mostra la precedenza quando si combina un _pattern_ che
utilizza `|` con una _match guard_. La parte importante di questo esempio è che
la _match guard_ `if y` si applica a `4`, `5` e `6`, anche se potrebbe sembrare
che `if y` si applichi solo a `6`.

<Listing number="19-28" caption="Combinazione di più _pattern_ con una _match guard_">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-28/src/main.rs:here}}
```

</Listing>

La condizione di corrispondenza stabilisce che il ramo corrisponde solo se il
valore di `x` è uguale a `4`, `5` o `6` e se `y` è `true`. Quando questo codice
viene eseguito, il _pattern_ del primo ramo corrisponde perché `x` è `4`, ma la
_match guard_ `if y` è `false`, quindi il primo ramo non viene scelto. Il codice
passa al secondo ramo, che corrisponde, e questo programma stampa `no`. Il
motivo è che la condizione `if` si applica all’intero _pattern_ `4 | 5 | 6`, non
solo all’ultimo valore `6`. In altre parole, la precedenza di una _match guard_
rispetto a un _pattern_ si comporta in questo modo:

```text
(4 | 5 | 6) if y => ...
```

piuttosto che in questo modo:

```text
4 | 5 | (6 if y) => ...
```

Dopo aver eseguito il codice, il comportamento della precedenza è evidente: se
la _match guard_ fosse stata applicata solo al valore finale nell’elenco di
valori specificato utilizzando l’operatore `|`, il ramo avrebbe trovato una
corrispondenza e il programma avrebbe stampato `sì`.

### Utilizzare il _Binding_ `@`

L’operatore _at_ `@` ci consente di creare una variabile che contiene un valore
mentre stiamo testando quel valore per una corrispondenza con il _pattern_. Nel
Listato 19-29, vogliamo verificare che il campo `id` di `Messaggio::Ciao` sia
compreso nell’intervallo `3..=7`. Vogliamo anche associare il valore alla
variabile `id` in modo da poterlo utilizzare nel codice associato al ramo.

<Listing number="19-29" caption="Usare `@` per associare un valore in un _pattern_ e testarlo">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-29/src/main.rs:here}}
```

</Listing>

Questo esempio stamperà `Trovato un id nell'intervallo: 5`. Specificando `id @`
prima dell’intervallo `3..=7`, catturiamo qualsiasi valore corrispondente
all’intervallo in una variabile denominata `id`, verificando anche che il valore
corrisponda al _pattern_ dell’intervallo.

Nel secondo ramo, dove abbiamo specificato solo un intervallo nel _pattern_, il
codice associato al ramo non ha una variabile che contenga il valore effettivo
del campo `id`. Il valore del campo `id` avrebbe potuto essere 10, 11 o 12, ma
il codice associato a quel _pattern_ non sa quale sia. Il codice del _pattern_
non è in grado di utilizzare il valore del campo `id`, perché non abbiamo
salvato il valore `id` in una variabile.

Nell’ultimo ramo, dove abbiamo specificato una variabile senza intervallo,
abbiamo il valore disponibile da utilizzare nel codice del ramo in una variabile
denominata `id`. Il motivo è che abbiamo utilizzato la sintassi abbreviata del
campo _struct_. Ma non abbiamo applicato alcun test al valore del campo `id` in
questo ramo, come abbiamo fatto con i primi due rami: qualsiasi valore
corrisponderebbe a questo _pattern_.

L’utilizzo di `@` ci consente di testare un valore e salvarlo in una variabile
all’interno di un _pattern_.

## Riepilogo

I _pattern_ di Rust sono molto utili per distinguere tra diversi tipi di dati.
Quando vengono utilizzati nelle espressioni `match`, Rust garantisce che i
_pattern_ coprano ogni valore possibile, altrimenti il programma non verrà
compilato. I _pattern_ nelle dichiarazioni `let` e nei parametri di funzione
rendono questi costrutti più utili, consentendo la destrutturazione dei valori
in parti più piccole e l’assegnazione di tali parti a variabili. Possiamo creare
_pattern_ semplici o complessi in base alle nostre esigenze.

Successivamente, nel penultimo capitolo del libro, esamineremo alcuni aspetti
avanzati di una varietà di funzionalità di Rust.
