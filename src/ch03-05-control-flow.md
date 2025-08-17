## Controllo del flusso

La possibilità di eseguire del codice a seconda che una condizione sia `vera` e
di eseguire ripetutamente del codice finché una data condizione è `vera` sono
elementi fondamentali della maggior parte dei linguaggi di programmazione. I
costrutti più comuni che ti permettono di controllare il flusso di esecuzione
del codice in Rust sono le espressioni `if` e i cicli.

### L'espressione `if`

Un'espressione `if` (`se` in italiano) ti permette di ramificare il tuo codice a
seconda delle condizioni. Fornisci una condizione e poi dici: "Se questa
condizione è soddisfatta, esegui questo blocco di codice. Se la condizione non è
soddisfatta, non eseguire questo blocco di codice"

Crea un nuovo progetto chiamato _ramificazioni_ nella tua directory _progetti_
per sperimentare con l'espressione `if`. Nel file _src/main.rs_, inserisci
quanto segue:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

Tutte le espressioni `if` iniziano con la parola chiave `if`, seguita da una
condizione. In questo caso, la condizione verifica se la variabile `numero` ha o
meno un valore inferiore a 5. Il blocco di codice da eseguire se la condizione è
`true` viene posizionato subito dopo la condizione, all'interno di parentesi
graffe. I blocchi di codice associati alle condizioni nelle espressioni `if`
possono esser viste come dei _rami_, proprio come i _rami_ nelle espressioni
`match` di cui abbiamo parlato nella sezione ["Confrontare l'ipotesi con il
numero segreto"][confrontare-lipotesi-con-il-numero-segreto]<!-- ignore --> del
Capitolo 2.

Opzionalmente, possiamo anche includere un'espressione `else` (`altrimenti` in
italiano), come abbiamo scelto di fare in questo caso, per dare al programma un
blocco di codice alternativo da eseguire nel caso in cui la condizione sia
valutata `false`. Se non fornisci un'espressione `else` e la condizione è
`false`, il programma salterà il blocco `if` e passerà alla parte di codice
successiva.

Prova a eseguire questo codice; dovresti vedere il seguente risultato:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

Proviamo a cambiare il valore di `numero` con un valore che renda la condizione
`false` per vedere cosa succede:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

Esegui nuovamente il programma e guarda l'output:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

Vale anche la pena di notare che la condizione in questo codice _deve_ essere un
`bool`. Se la condizione non è un `bool`, otterremo un errore. Ad esempio, prova
a eseguire il seguente codice:

<span class="filename">File: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

Questa volta la condizione `if` valuta un valore di `3` e Rust lancia un errore:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

L'errore indica che Rust si aspettava un `bool` ma ha ottenuto un numero intero.
A differenza di linguaggi come Ruby e JavaScript, Rust non cercherà
automaticamente di convertire i _type_ non booleani in booleani. Devi essere
esplicito e fornire sempre ad `if` un'espressione booleana come condizione. Se
vogliamo che il blocco di codice `if` venga eseguito solo quando un numero non è
uguale a `0`, ad esempio, possiamo modificare l'espressione `if` nel seguente
modo:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

L'esecuzione di questo codice stamperà `numero era qualcosa di diverso da zero`.

#### Gestione di condizioni multiple con `else if`

Puoi utilizzare condizioni multiple combinando `if` e `else` in un'espressione
`else if`. Ad esempio:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

Questo programma ha quattro possibili _rami_. Dopo averlo eseguito, dovresti
vedere il seguente output:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```
Quando questo programma viene eseguito, controlla ogni espressione `if` a turno
ed esegue il primo corpo per il quale la condizione è valutata `true`. Nota che
anche se 6 è divisibile per 2, non vediamo l'output `numero è divisibile per 2`,
né vediamo il testo `numero non è divisibile per 4, 3 o 2` del blocco `else`.
Questo perché Rust esegue il blocco solo per la prima condizione `true` e una
volta che ne trova una, le restanti non vengono controllate.

L'uso di troppe espressioni `else if` può rendere il codice un po' confusionario
e difficile da leggere, quindi se ne hai più di una, potresti valutare di
riscrivere il codice. Il Capitolo 6 descrive un potente costrutto di
ramificazione di Rust chiamato `match` per gestire casi del genere.

#### Utilizzo di `if` in una dichiarazione `let`

Dato che `if` è un'espressione, possiamo usarla a destra di una dichiarazione
`let` per assegnare il risultato a una variabile, come nel Listato 3-2.

<Listing number="3-2" file-name="src/main.rs" caption="Assegnazione del risultato di un'espressione `if` as una variabile">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

</Listing>

La variabile `numero` sarà legata a un valore basato sul risultato
dell'espressione `if`. Esegui questo codice per vedere cosa succede:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

Ricorda che i blocchi di codice valutano l'ultima espressione in essi contenuta
e i numeri da soli sono anch'essi espressioni. In questo caso, il valore
dell'intera espressione `if` dipende da quale blocco di codice viene eseguito.
Ciò significa che i valori che possono essere i risultati di ogni _ramo_ di `if`
devono essere dello stesso tipo; nel Listato 3-2, i risultati sia del _ramo_
`if` che del _ramo_ `else` erano numeri interi `i32`. Se i _type_ non sono
corrispondenti, come nell'esempio seguente, otterremo un errore:

<span class="filename">File: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

Quando proviamo a compilare questo codice, otterremo un errore: i _rami_ `if` e
`else` hanno _type_ incompatibili e Rust indica esattamente dove trovare il
problema nel programma:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

L'espressione nel blocco `if` ritorna un _integer_ e l'espressione nel blocco
`else` ritorna una stringa. Questo non funziona perché le variabili devono avere
un _type_ univoco e Rust ha bisogno di sapere in fase di compilazione di che
_type_ è la variabile `numero`, in modo definitivo. Conoscere il _type_ di
`numero` permette al compilatore di verificare che il _type_ sia valido ovunque
si utilizzi `numero`. Rust non sarebbe in grado di farlo se il _type_ di
`numero` fosse determinato solo in fase di esecuzione; il compilatore sarebbe
più complesso e darebbe meno garanzie sul codice se dovesse tenere traccia dei
più disparati _type_ possibili per ogni variabile.

### Ripetizione con i cicli

Spesso è utile eseguire un blocco di codice più di una volta. Per questo
compito, Rust mette a disposizione diversi _cicli_ (_loop_ in inglese), che
eseguono il codice all'interno del corpo del ciclo fino alla fine e poi
ripartono immediatamente dall'inizio. Per sperimentare con i cicli, creiamo un
nuovo progetto chiamato _cicli_.

Rust mette a disposizione tre tipologie di ciclo: `loop`, `while` e `for`.
Proviamo ciascuno di essi.

#### Ripetizione del codice con `loop`

La parola chiave `loop` dice a Rust di eseguire un blocco di codice più e più
volte per sempre o finché non gli dici esplicitamente di fermarsi.

A titolo di esempio, modifica il file _src/main.rs_ nella tua cartella _cicli_
in questo modo:

<span class="filename">File: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

Quando eseguiamo questo programma, vedremo `ancora!` stampato in continuazione
fino a quando non interromperemo il programma manualmente. La maggior parte dei
terminali supporta la scorciatoia da tastiera <kbd>ctrl</kbd>-<kbd>c</kbd> per
interrompere un programma che è bloccato in un ciclo continuo. Provaci:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling cicli v0.1.0 (file:///progetti/cicli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/cicli`
ancora!
ancora!
ancora!
ancora!
^Ccicli!
```

Il simbolo `^C` rappresenta quando hai premuto <kbd>ctrl</kbd>-<kbd>c</kbd>.

Potresti vedere o meno la parola `ancora!` stampata dopo la `^C`, a seconda di
dove si trovava il codice nel ciclo quando ha ricevuto il segnale di
interruzione.

Fortunatamente, Rust offre anche un modo per uscire da un ciclo utilizzando del
codice. Puoi inserire la parola chiave `break` all'interno del ciclo per
indicare al programma quando interrompere l'esecuzione del ciclo. Ricorda che
abbiamo fatto questo nel gioco di indovinelli nella sezione ["Uscita dopo
un'ipotesi corretta"][uscita-dopo-unipotesi-corretta]<!-- ignore --> del
Capitolo 2 per uscire dal programma quando l'utente indovinava il numero
segreto.

Nel gioco di indovinelli abbiamo usato anche `continue`, che in un ciclo indica
al programma di saltare tutto il codice rimanente in questa iterazione del ciclo
e di passare all'iterazione successiva.

#### Restituzione di valori dai cicli

Uno degli utilizzi di un `loop` è quello di riprovare un'operazione che sai che
potrebbe fallire, come ad esempio controllare se un _thread_ ha completato il
suo lavoro. Potresti anche aver bisogno di passare il risultato di questa
operazione al di fuori del ciclo al resto del tuo codice. Per farlo, puoi
aggiungere il valore che vuoi che venga restituito dopo l'espressione `break`
che utilizzi per interrompere il ciclo; quel valore verrà restituito al di fuori
del ciclo in modo da poterlo utilizzare, come mostrato qui:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```
Prima del ciclo, dichiariamo una variabile chiamata `contatore` e la
inizializziamo a `0`. Poi dichiariamo una variabile chiamata `risultato` per
contenere il valore restituito dal ciclo. A ogni iterazione del ciclo,
aggiungiamo `1` alla variabile `contatore` e poi controlliamo se `contatore` è
uguale a `10`. Quando lo è, usiamo la parola chiave `break` con il valore
`contatore * 2`. Dopo il ciclo, usiamo un punto e virgola per terminare
l'istruzione che assegna il valore a `risultato`. Infine, stampiamo il valore in
`risultato`, che in questo caso è `20`.

Puoi anche usare `return` all'interno di un ciclo. Mentre `break` esce solo dal
ciclo corrente, `return` esce sempre dalla funzione corrente.

#### Etichette di loop per distinguere tra cicli multipli

Se hai un ciclo annidati all'interno di un altro ciclo, `break` e `continue` si
applicano al loop più interno in quel momento. Puoi specificare facoltativamente
un'_etichetta_ (_loop label_) su uno specifico ciclo per poi usare con `break` o
`continue` quell'etichetta per specificare a quale ciclo applicare l'istruzione.
Le _loop label_ devono iniziare con una virgoletta singola. Ecco un esempio con
due cicli annidati:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

Il ciclo esterno ha la _label_ `'aumenta_conteggio` e conta da 0 a 2. Il ciclo
interno senza _label_ conta da 10 a 9. Il primo `break` che non specifica una
_label_ esce solo dal ciclo interno. L'istruzione `break 'aumenta_conteggio;`
esce dal ciclo esterno. Questo codice stamperà:

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

#### I cicli condizionali con `while`

Spesso un programma ha bisogno di valutare una condizione all'interno di un
ciclo. Quando la condizione è `true`, il ciclo viene eseguito. Quando la
condizione cessa di essere `true`, il programma chiama `break`, interrompendo il
ciclo. È possibile implementare un comportamento del genere utilizzando una
combinazione di `loop`, `if`, `else` e `break`; se vuoi, puoi provare a farlo in
un programma. Tuttavia, questo schema è così comune che Rust ha un costrutto di
linguaggio incorporato per questi casi, chiamato ciclo `while` (_finché_ in
italiano). Nel Listato 3-3, usiamo `while` per eseguire il ciclo del programma
tre volte, contando alla rovescia ogni volta, e poi, dopo il ciclo, stampiamo un
messaggio e usciamo.

<Listing number="3-3" file-name="src/main.rs" caption="Utilizzo di un ciclo `while` per eseguire codice finché la condizione è `true`">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

</Listing>

Questo costrutto elimina un sacco di annidamenti che sarebbero necessari se
usassi `loop`, `if`, `else` e `break`, ed è di più semplice lettura. Finchè una
condizione risulta `true`, il codice viene eseguito; altrimenti, esce dal ciclo.

#### Eseguire un ciclo su una collezione con `for`

Puoi scegliere di utilizzare il costrutto `while` per eseguire un ciclo sugli
elementi di una collezione, come un array. Ad esempio, il ciclo nel Listato 3-4
stampa ogni elemento dell'array `a`.

<Listing number="3-4" file-name="src/main.rs" caption="Passare in rassegna gli elementi di una collezione con un ciclo `while`">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

</Listing>

In questo caso, il codice conteggia tutti gli elementi dell'_array_: inizia
dall'indice `0` e poi esegue un ciclo fino a raggiungere l'ultimo indice
dell'_array_ (cioè quando `indice < 5` non è più `true`). L'esecuzione di questo
codice stamperà ogni elemento dell'array:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

Tutti e cinque i valori dell'_array_ appaiono nel terminale, come previsto.
Anche se `indice` raggiungerà un valore di `5` a un certo punto, il ciclo viene
bloccato prima che si tenti di leggere un sesto elemento dell'_array_.

Tuttavia, questo approccio è incline all'errore; potremmo causare il _panic_ del
programma se il valore dell'indice o la condizione di test non sono corretti.
Per esempio, se cambiassi la definizione dell'array `a` per avere quattro
elementi, ma dimenticassi di aggiornare la condizione a `while indice < 4`, il
codice andrebbe in _panic_. È anche lento, perché il compilatore aggiunge codice
di runtime per eseguire il controllo condizionale per verificare se l'indice è
entro i limiti dell'array a ogni iterazione del ciclo.

Come alternativa più concisa, puoi usare un ciclo `for` ed eseguire del codice
per ogni elemento di una collezione. Un ciclo `for` assomiglia al codice del
Listato 3-5.

<Listing number="3-5" file-name="src/main.rs" caption="Passare in rassegna gli elementi di una collezione con un ciclo `for`">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

</Listing>

Quando eseguiamo questo codice, vedremo lo stesso risultato del Listato 3-4. Ma,
cosa più importante, abbiamo aumentato la sicurezza del codice ed eliminato la
possibilità di bug che potrebbero derivare dall'andare oltre la fine dell'array
o dal non accedere ad ogni elemento dell'array. Il codice macchina generato dai
cicli `for` può essere anche più efficiente, perché l'indice non deve essere
confrontato con la lunghezza dell'array a ogni iterazione.

Utilizzando il ciclo `for`, non dovrai ricordarti di modificare altro codice se
cambierai il numero di valori nell'array, come invece faresti con il metodo
`while` usato nel Listato 3-4.

La sicurezza e la concisione dei cicli `for` li rendono il costrutto di ciclo
più usato in Rust. Anche nelle situazioni in cui vuoi eseguire un certo numero
di volte il codice, come nell'esempio del conto alla rovescia che utilizzava un
ciclo `while` nel Listato 3-3, la maggior parte dei Rustaceani userebbe un ciclo
`for`. Il modo per farlo sarebbe quello di usare un `Range`, fornito dalla
libreria standard, che genera tutti i numeri in sequenza partendo da un numero e
finendo prima di un altro numero.

Ecco come apparirebbe il conto alla rovescia utilizzando un ciclo `for` e un
altro metodo di cui non abbiamo ancora parlato, `rev`, per invertire
l'intervallo.

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

Questo codice è un po' più carino, vero?

## Riepilogo

Ce l'hai fatta! Questo capitolo è stato molto impegnativo: hai imparato a
conoscere le variabili, i _type_ di dati scalari e composti, le funzioni, i
commenti, le espressioni `if` e i cicli! Per esercitarti con i concetti discussi
in questo capitolo, prova a costruire dei programmi per eseguire le seguenti
operazioni:

- Convertire le temperature tra Fahrenheit e Celsius.
- Generare l'*n*esimo numero di Fibonacci.
- Stampare il testo del canto natalizio "The Twelve Days of Christmas",
  sfruttando la ripetizione della canzone.

Quando sarai pronto per andare avanti, parleremo di un concetto di Rust che non
esiste in altri linguaggi di programmazione: la _ownership_.

[confrontare-lipotesi-con-il-numero-segreto]:
    ch02-00-guessing-game-tutorial.html#confrontare-lipotesi-con-il-numero-segreto
[uscita-dopo-unipotesi-corretta]:
    ch02-00-guessing-game-tutorial.html#uscita-dopo-unipotesi-corretta
