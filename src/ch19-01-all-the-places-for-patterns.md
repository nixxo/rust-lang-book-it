## Tutti i Posti Dove Possiamo Utilizzare i _Pattern_

I _pattern_ compaiono in diversi punti di Rust e li hai usati senza rendertene
conto! Questa sezione illustra tutti i posti in cui i _pattern_ sono validi.

### Rami `match`

Come discusso nel Capitolo 6, utilizziamo i _pattern_ nei rami delle espressioni
`match`. Formalmente, le espressioni `match` sono definite come la parola chiave
`match`, un valore su cui effettuare la corrispondenza e uno o più rami di
corrispondenza costituiti da un _pattern_ e un’espressione da eseguire se il
valore corrisponde al _pattern_ di quel ramo, in questo modo:

<!-- Formattato manualmente perchè Markdown non supporta il corsivo nel corpo di
un blocco come questo! -->

<pre><code>match <em>VALORE</em> {
    <em>PATTERN</em> => <em>ESPRESSIONE</em>,
    <em>PATTERN</em> => <em>ESPRESSIONE</em>,
    <em>PATTERN</em> => <em>ESPRESSIONE</em>,
}</code></pre>

Ad esempio, ecco l’espressione `match` del Listato 6-5 che corrisponde a un
valore `Option<i32>` nella variabile `x`:

```rust,ignore
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

I _pattern_ in questa espressione `match` sono `None` e `Some(i)` a sinistra di
ciascuna freccia.

Un requisito per le espressioni `match` è che siano _esaustive_, nel senso che
tutte le possibilità per il valore nell’espressione `match` devono essere
considerate. Un modo per assicurarsi di aver coperto ogni possibilità è avere un
_pattern_ pigliatutto per l’ultimo ramo: ad esempio, un nome di variabile che
corrisponde a qualsiasi valore non può mai fallire e quindi copre tutti i casi
rimanenti.

Il _pattern_ specifico `_` corrisponderà a qualsiasi cosa, ma non si vincola mai
a una variabile, quindi viene spesso utilizzato nell’ultimo ramo di
corrispondenza. Il _pattern_ `_` può essere utile quando si desidera ignorare
qualsiasi valore non specificato, ad esempio. Tratteremo il _pattern_ `_` più
dettagliatamente in [“Ignorare Valori In un
_Pattern_”][ignoring-values-in-a-pattern]<!-- ignore --> più avanti in questo
capitolo.

### Dichiarazioni `let`

Prima di questo capitolo, avevamo discusso esplicitamente dell’uso dei _pattern_
solo con `match` e `if let`, ma in realtà abbiamo utilizzato _pattern_ anche in
altri contesti, incluse le dichiarazioni `let`. Ad esempio, si consideri questa
semplice assegnazione di variabile con `let`:

```rust
let x = 5;
```

Ogni volta che hai utilizzato una dichiarazione `let` come questa, hai
utilizzato dei _pattern_, anche se potresti non essertene accorto! Più
formalmente, una dichiarazione `let` si presenta così:

<!-- Formattato manualmente perchè Markdown non supporta il corsivo nel corpo di
un blocco come questo! -->

<pre>
<code>let <em>PATTERN</em> = <em>ESPRESSIONE</em>;</code>
</pre>

In dichiarazioni come `let x = 5;` con un nome di variabile nel posto di
_PATTERN_, il nome della variabile è solo una forma particolarmente semplice di
_pattern_. Rust confronta l’espressione con il _pattern_ e assegna qualsiasi
nome trovi. Quindi, nell’esempio `let x = 5;`, `x` è un _pattern_ che significa
“associa ciò che corrisponde qui alla variabile `x`”. Poiché il nome `x` è
l’intero _pattern_, questo _pattern_ significa effettivamente “associa tutto
alla variabile `x`, qualunque sia il valore”.

Per comprendere più chiaramente l’aspetto di corrispondenza del _pattern_ di
`let`, considera il Listato 19-1, che utilizza un _pattern_ con `let` per
destrutturare una tupla.

<Listing number="19-1" caption="Utilizzo di un _pattern_ per destrutturare una tupla e creare tre variabili contemporaneamente">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-01/src/main.rs:here}}
```

</Listing>

Qui, confrontiamo una tupla con un _pattern_. Rust confronta il valore `(1, 2,
3)` con il _pattern_ `(x, y, z)` e verifica che il valore corrisponde al
_pattern_, in quanto vede che il numero di elementi è lo stesso in entrambi,
quindi Rust associa `1` a `x`, `2` a `y` e `3` a `z`. Si può pensare a questo
_pattern_ di tupla come all’annidamento di tre _pattern_ di variabili
individuali al suo interno.

Se il numero di elementi nel _pattern_ non corrisponde al numero di elementi
nella tupla, il _type_ complessivo non corrisponderà e si verificherà un errore
del compilatore. Ad esempio, il Listato 19-2 mostra un tentativo di
destrutturare una tupla con tre elementi in due variabili, che non funzionerà.

<Listing number="19-2" caption="Costruzione errata di un _pattern_ le cui variabili non corrispondono al numero di elementi nella tupla">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-02/src/main.rs:here}}
```

</Listing>

Il tentativo di compilare questo codice genera questo tipo di errore:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-02/output.txt}}
```

Per correggere l’errore, potremmo ignorare uno o più valori nella tupla usando
`_` o `..`, come vedrai nella sezione [“Ignorare Valori In un
_Pattern_”][ignoring-values-in-a-pattern]<!-- ignore -->. Se il problema è che
abbiamo troppe variabili nel _pattern_, la soluzione è far corrispondere i
_type_ rimuovendo le variabili in modo che il numero di variabili sia uguale al
numero di elementi nella tupla.

### Espressioni Condizionali `if let`

Nel Capitolo 6, abbiamo discusso come utilizzare le espressioni `if let`
principalmente come un modo più breve per scrivere l’equivalente di un `match`
che corrisponde a un solo caso. Facoltativamente, `if let` può avere un `else`
corrispondente contenente il codice da eseguire se il _pattern_ in `if let` non
corrisponde.

Il Listato 19-3 mostra che è anche possibile combinare e abbinare le espressioni
`if let`, `else if` e `else if let`. Ciò offre maggiore flessibilità rispetto a
un’espressione `match` in cui possiamo esprimere un solo valore da confrontare
con i _pattern_. Inoltre, Rust non richiede che le condizioni in una serie di
rami `if let`, `else if` e `else if let` siano correlate tra loro.

Il codice nel Listato 19-3 determina il colore da utilizzare per lo sfondo in
base a una serie di controlli per diverse condizioni. Per questo esempio,
abbiamo creato variabili con valori predefiniti che un programma reale potrebbe
ricevere dall’input dell’utente.

<Listing number="19-3" file-name="src/main.rs" caption="Combinazione di `if let`, `else if`, `else if let` e `else`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-03/src/main.rs}}
```

</Listing>

Se l’utente specifica un colore preferito, quel colore viene utilizzato come
sfondo. Se non viene specificato alcun colore preferito e oggi è martedì, il
colore di sfondo è verde. Altrimenti, se l’utente specifica la propria età come
stringa e possiamo analizzarla come un numero correttamente, il colore sarà
viola o arancione a seconda del valore del numero. Se nessuna di queste
condizioni si applica, il colore di sfondo è blu.

Questa struttura condizionale ci consente di supportare requisiti complessi. Con
i valori specificati che abbiamo qui, questo esempio stamperà `Usando il viola
come colore di sfondo`.

Si può notare che `if let` può anche introdurre nuove variabili che oscurano le
variabili esistenti allo stesso modo in cui `match` può farlo: la riga `if let
Ok(eta) = eta` introduce una nuova variabile `eta` che contiene il valore
all’interno della variante `Ok`, oscurando la variabile `eta` esistente. Ciò
significa che dobbiamo inserire la condizione `if eta > 30` all’interno di quel
blocco: non possiamo combinare queste due condizioni in `if let Ok(eta) = eta &&
eta > 30`. Il nuovo `eta` che vogliamo confrontare con 30 non è valido finché il
nuovo _scope_ non inizia con la parentesi graffa.

Lo svantaggio dell’utilizzo di espressioni `if let` è che il compilatore non
verifica l’esaustività, mentre con le espressioni `match` lo fa. Se omettessimo
l’ultimo blocco `else` e quindi non gestissimo alcuni casi, il compilatore non
ci avviserebbe del possibile bug logico.

### Cicli Condizionali `while let`

Simile nella costruzione a `if let`, il ciclo condizionale `while let` consente
a un ciclo `while` di essere eseguito finché un _pattern_ continua a
corrispondere. Nel Listato 19-4 mostriamo un ciclo `while let` che attende i
messaggi inviati tra _thread_, ma in questo caso controlla un `Result` invece di
un `Option`.

<Listing number="19-4" caption="Utilizzo di un ciclo `while let` per stampare valori finché `rx.recv()` restituisce `Ok`">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-04/src/main.rs:here}}
```

</Listing>

Questo esempio stampa `1`, `2` e poi `3`. Il metodo `recv` prende il primo
messaggio dall’estremità ricevente del canale e restituisce `Ok(value)`. Quando
abbiamo visto per la prima volta `recv` nel Capitolo 16, abbiamo analizzato
direttamente l’errore, o abbiamo interagito con esso come un iteratore usando un
ciclo `for`. Come mostra il Listato 19-4, tuttavia, possiamo anche usare `while
let`, perché il metodo `recv` restituisce `Ok` ogni volta che arriva un
messaggio, finché il mittente esiste, e poi produce un `Err` una volta che il
mittente si disconnette.

### Cicli `for`

In un ciclo `for`, il valore che segue direttamente la parola chiave `for` è un
_pattern_. Ad esempio, in `for x in y`, `x` è il _pattern_. Il Listato 19-5
mostra come utilizzare un _pattern_ in un ciclo `for` per *destrutturare*, o
scomporre una tupla come parte del ciclo `for`.

<Listing number="19-5" caption="Utilizzo di un _pattern_ in un ciclo `for` per destrutturare una tupla">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-05/src/main.rs:here}}
```

</Listing>

Il codice nel Listato 19-5 stamperà quanto segue:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-05/output.txt}}
```

Adattiamo un iteratore utilizzando il metodo `enumerate` in modo che produca un
valore e l’indice per quel valore, inserito in una tupla. Il primo valore
prodotto è la tupla `(0, 'a')`. Quando questo valore viene confrontato con il
_pattern_ `(indice, valore)`, `indice` sarà `0` e `valore` sarà ``a``, stampando
la prima riga dell’output.

### Parametri di Funzione

Anche i parametri di funzione possono essere _pattern_. Il codice nel Listato
19-6, che dichiara una funzione chiamata `foo` che accetta un parametro chiamato
`x` di _type_ `i32`, dovrebbe ormai risultare familiare.

<Listing number="19-6" caption="Una firma di funzione usa _pattern_ nei parametri">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-06/src/main.rs:here}}
```

</Listing>

La parte `x` è un _pattern_! Come abbiamo fatto con `let`, potremmo abbinare una
tupla negli argomenti di una funzione al _pattern_. Il Listato 19-7 suddivide i
valori in una tupla mentre la passiamo a una funzione.

<Listing number="19-7" file-name="src/main.rs" caption="Una funzione con parametri che destrutturano una tupla">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-07/src/main.rs}}
```

</Listing>

Questo codice stampa `Posizione corrente: (3, 5)`. I valori `&(3, 5)`
corrispondono al _pattern_ `&(x, y)`, quindi `x` è il valore `3` e `y` è il
valore `5`.

Possiamo anche usare i _pattern_ nelle liste di parametri delle chiusure allo
stesso modo delle liste di parametri di funzione, perché le chiusure sono simili
alle funzioni, come discusso nel Capitolo 13.

A questo punto, hai visto diversi modi per usare i _pattern_, ma i _pattern_ non
funzionano allo stesso modo in tutti i casi in cui possiamo usarli. In alcuni
casi, i _pattern_ devono essere inconfutabili; in altre circostanze, possono
essere confutabili. Discuteremo questi due concetti più avanti.

[ignoring-values-in-a-pattern]: ch19-03-pattern-syntax.html#ignorare-valori-in-un-pattern
