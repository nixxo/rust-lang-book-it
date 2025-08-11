## Funzioni

Le funzioni sono molto diffuse nel codice di Rust. Hai già visto una delle
funzioni più importanti del linguaggio: la funzione `main`, che è il punto di
ingresso di molti programmi. Hai anche visto la parola chiave `fn`, che ti
permette di dichiarare nuove funzioni.

Il codice Rust utilizza lo _snake case_ come stile convenzionale per i nomi di
funzioni e variabili, in cui tutte le lettere sono minuscole e i trattini bassi
separano le parole. Ecco un programma che contiene un esempio di definizione di
funzione:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

In Rust definiamo una funzione inserendo `fn` seguito dal nome della funzione e
da una serie di parentesi. Le parentesi graffe indicano al compilatore dove
inizia e finisce il corpo della funzione.

Possiamo chiamare qualsiasi funzione che abbiamo definito inserendo il suo nome
seguito da una serie di parentesi. Poiché `altra_funzione` è definita nel
programma, può essere chiamata dall'interno della funzione `main`. Nota che
abbiamo definito `altra_funzione` _dopo_ la funzione `main` nel codice sorgente;
avremmo potuto definirla anche prima. A Rust non interessa dove definisci le tue
funzioni, ma solo che siano definite in una parte del codice che sia "visibile",
in _scope_, al chiamante.

Cominciamo un nuovo progetto binario chiamato _functioni_ per esplorare
ulteriormente le funzioni. Inserisci l'esempio `altra_funzione` in _src/main.rs_
ed eseguilo. Dovresti vedere il seguente output:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

Le righe vengono eseguite nell'ordine in cui appaiono nella funzione `main`.
Prima viene stampato il messaggio "Hello, world!", poi viene chiamata
`altra_funzione` e viene stampato il suo messaggio.

### Parametri

Possiamo definire le funzioni in modo che abbiano dei _parametri_, ovvero delle
variabili speciali che fanno parte della firma di una funzione. Quando una
funzione ha dei parametri, puoi fornirle dei valori concreti per questi
parametri. Tecnicamente, i valori concreti sono chiamati _argomenti_, ma in una
conversazione informale si tende a usare le parole _parametro_ e _argomento_ in
modo intercambiabile, sia per le variabili nella definizione di una funzione che
per i valori concreti passati quando si chiama una funzione.

In questa versione di `altra_funzione` aggiungiamo un parametro:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

Prova a eseguire questo programma; dovresti ottenere il seguente risultato:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```
La dichiarazione di `altra_funzione` ha un parametro chiamato `x`. Il _type_ di
`x` è specificato come `i32`. Quando passiamo `5` ad `altra_funzione`, la macro
`println!` mette `5` nel punto in cui si trovava la coppia di parentesi graffe
contenente `x` nella stringa di formato.

Nelle firme delle funzioni è _obbligatorio_ dichiarare il _type_ di ogni
parametro. Si tratta di una decisione deliberata nel design di Rust: richiedere
le annotazioni sul _type_ nelle definizioni delle funzioni significa che il
compilatore non ha quasi mai bisogno di usarle in altre parti del codice per
capire a quale _type_ ti riferisci. In questo modo il compilatore potrà anche
dare messaggi di errore più utili se sa quali _type_ si aspetta la funzione.

Quando definisci più parametri, separa le dichiarazioni dei parametri con delle
virgole, in questo modo:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

Questo esempio crea una funzione chiamata `stampa_unita_misura` con due
parametri. Il primo parametro si chiama `valore` ed è un `i32`. Il secondo si
chiama `unita_misura` ed è di _type_ `char`. La funzione stampa quindi un testo
contenente sia il `valore` che `unita_misura`.

Eseguiamo il codice. Sostituisci il codice attualmente presente nel file
src/main.rs_ del tuo progetto _funzioni_ con l'esempio precedente ed eseguilo
con `cargo run`:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

Poiché abbiamo chiamato la funzione con `5` come valore per `valore` e `'h'`
come valore per `unita_misura`, l'output del programma contiene questi valori.

### Dichiarazioni ed espressioni

I corpi delle funzioni sono costituiti da una serie di dichiarazioni che possono
eventualmete terminare con un'espressione. Finora le funzioni che abbiamo
trattato non hanno incluso un'espressione finale, ma hai visto un'espressione
come parte di una dichiarazione. Poiché Rust è un linguaggio basato sulle
espressioni, questa è una distinzione importante da capire. Altri linguaggi non
hanno le stesse distinzioni, quindi vediamo cosa sono le dichiarazioni e le
espressioni e come le loro differenze influenzano il corpo delle funzioni.

- Le dichiarazioni sono istruzioni che eseguono un'azione e non restituiscono un
  valore.
- Le espressioni vengono valutate e restituiscono un valore risultante.

Vediamo alcuni esempi.

In realtà abbiamo già usato le dichiarazioni e le espressioni. Creare una
variabile e assegnarle un valore con la parola chiave `let` è una dichiarazione.
Nel Listato 3-1, `let y = 6;` è una dichiarazione.

<Listing number="3-1" file-name="src/main.rs" caption="La funzione `main` contenente una dichiarazione">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

</Listing>

Anche la definizione di una funzione è una dichiarazione; l'intero esempio
precedente è, di per sé, una dichiarazione (Come vedremo più avanti, però,
chiamare una funzione non è una dichiarazione)

Le dichiarazioni non restituiscono valori. Pertanto, non puoi assegnare una
dichiarazione `let` a un'altra variabile, come cerca di fare il codice seguente;
otterrai un errore:

<span class="filename">File: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

Quando esegui questo programma, l'errore che otterrai è simile a questo:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

La dichiarazione `let y = 6` non restituisce un valore, quindi non c'è nulla a
cui `x` possa legarsi. Questo è diverso da ciò che accade in altri linguaggi,
come C e Ruby, dove l'assegnazione restituisce il valore dell'assegnazione. In
questi linguaggi, puoi scrivere `x = y = 6` e far sì che sia `x` che `y` abbiano
il valore `6`; questo non è il caso di Rust.

Le espressioni che valutate restituiscono un valore costituiscono la maggior
parte del resto del codice che scriverai in Rust. Considera un'operazione
matematica, come `5 + 6`, che è un'espressione che restituisce il valore `11`.
Le espressioni possono far parte di dichiarazioni: nel Listato 3-1, il `6` nella
dichiarazione `let y = 6;` è un'espressione che valuta il valore `6`. Chiamare
una funzione è un'espressione. Chiamare una macro è un'espressione. Pure
definire tramite parentesi graffe un nuovo _scope_ ad esempio:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

This expression:

```rust,ignore
{
    let x = 3;
    x + 1
}
```
è un blocco che, in questo caso, valuta `4`. Questo valore viene legato a `y`
come parte dell'istruzione `let`. Nota che la riga `x + 1` non ha un punto e
virgola alla fine, il che è diverso dalla maggior parte delle righe che hai
visto finora. Le espressioni non includono il punto e virgola finale. Se
aggiungi un punto e virgola alla fine di un'espressione, la trasformi in una
dichiarazione e quindi non restituirà un valore. Tienilo a mente mentre leggi il
prossimo paragrafo sui volori di ritorno delle funzioni e le espressioni.

### Funzioni con valori di ritorno

Le funzioni possono restituire dei valori al codice che le chiama. Non assegnamo
un nome ai valori di ritorno, ma dobbiamo esplicitarne il _type_ dopo una
freccia (`->`). In Rust, il valore di ritorno della funzione è sinonimo del
valore dell'espressione finale nel blocco del corpo di una funzione. Puoi far
ritornare un valore anche in anticipo alla funzione usando la parola chiave
`return` e specificando un valore, ma la maggior parte delle funzioni
restituisce l'ultima espressione in modo implicito. Ecco un esempio di funzione
che restituisce un valore:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

Non ci sono chiamate di funzione, macro o dichiarazioni `let` nella funzione
`cinque`, ma solo il numero `5` da solo. Si tratta di una funzione perfettamente
valida in Rust. Nota che anche il _type_ di ritorno della funzione è specificato
come `-> i32`. Prova a eseguire questo codice; l'output dovrebbe essere simile a
questo:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

Il `5` in `cinque` è il valore di ritorno della funzione, motivo per cui il tipo
di ritorno è `i32`. Esaminiamo il tutto più in dettaglio. Ci sono due elementi
importanti: innanzitutto, la riga `let x = cinque();` mostra che stiamo
utilizzando il valore di ritorno di una funzione per inizializzare una
variabile. Poiché la funzione `cinque` restituisce un `5`, questa riga è uguale
alla seguente:

```rust
let x = 5;
```

In secondo luogo, la funzione `cinque` non ha parametri e definisce il _type_
del valore di ritorno, ma il corpo della funzione è un solitario `5` senza punto
e virgola perché è un'espressione il cui valore vogliamo restituire.

Vediamo un altro esempio:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

Eseguendo questo codice verrà stampato `Il valore di x è: 6`. Ma se inseriamo un
punto e virgola alla fine della riga contenente `x + 1`, trasformandola da
espressione a dichiarazione, otterremo un errore:

<span class="filename">File: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

La compilazione di questo codice produce un errore, come segue:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

Il messaggio di errore principale, `mismatched types` (_type incompatibili_),
rivela il problema principale di questo codice. La definizione della funzione
`piu_uno` dice che restituirà un `i32`, ma le dichiarazioni non risultano in un
valore, restituendo un `()`, il _type_ _unit_. Pertanto, non viene restituito
nulla, il che contraddice la definizione della funzione e provoca un errore. In
questo output, Rust fornisce un messaggio che può aiutare a correggere questo
problema: suggerisce di rimuovere il punto e virgola, che risolverebbe l'errore.