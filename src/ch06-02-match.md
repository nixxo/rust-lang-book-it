<!-- Old heading. Do not remove or links may break. -->
<a id="the-match-control-flow-operator"></a>

## Controllo del Flusso col Costrutto `Match`

Rust offre un costrutto di controllo del flusso estremamente potente chiamato
`match` (_corrisponde_, _combacia_) che permette di confrontare un valore con
una serie di _pattern_ ed eseguire codice in base al _pattern_ che corrisponde.
I _pattern_ possono essere composti da valori letterali, nomi di variabili,
caratteri jolly e molte altre cose; il [Capitolo 19](ch19-00-patterns.html)
copre tutte le diverse tipologie di _pattern_ e cosa fanno. La potenza di
`match` deriva dall’espressività dei _pattern_ e dal fatto che il compilatore
conferma che tutti i casi possibili sono gestiti.

Pensa a un’espressione `match` come a una macchina che smista monete: le monete
scivolano lungo una guida con fori di varie dimensioni e ciascuna moneta cade
nel primo foro in cui entra. Allo stesso modo, i valori passano attraverso ogni
_pattern_ in un `match`, e al primo _pattern_ in cui il valore «entra», il
valore viene fatto ricadere nel blocco di codice associato per essere usato
durante l’esecuzione.

Parlando di monete, usiamole come esempio con `match`! Possiamo scrivere una
funzione che prende una moneta USA sconosciuta e, in modo simile alla macchina
contamonete, determina quale moneta sia e restituisce il suo valore in
centesimi, come mostrato nel Listato 6-3.

<Listing number="6-3" caption="Un _enum_ e un'espressione `match` che ha come _pattern_ le varianti dell'_enum_">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

</Listing>

Analizziamo il `match` nella funzione `valore_in_cent`. Prima troviamo la parola
chiave `match` seguita da un’espressione, che in questo caso è il valore
`moneta`. Questo sembra molto simile a un’espressione condizionale usata con
`if`, ma c’è una grande differenza: con `if` la condizione deve valutarsi a un
valore Booleano, mentre qui può essere di qualsiasi _type_. Il _type_ di
`moneta` in questo esempio è l’_enum_ `Moneta` che abbiamo definito nella prima
riga.

Seguono i _rami_ di `match`. Un ramo è composto da due parti: un _pattern_ e del
codice. Il primo ramo qui ha come _pattern_ il valore `Moneta::Penny` e poi
l’operatore `=>` che separa il _pattern_ dal codice da eseguire. Il codice in
questo caso è semplicemente il valore `1`. Ogni ramo è separato dal successivo
da una virgola.

Quando l’espressione `match` viene eseguita, confronta il valore risultante con
il _pattern_ di ogni ramo, in ordine. Se un _pattern_ corrisponde al valore,
viene eseguito il codice associato a quel _pattern_. Se quel _pattern_ non
corrisponde, l’esecuzione continua con il ramo successivo, proprio come nella
macchina che smista monete. Possiamo avere tanti rami quanti ce ne servono: nel
Listato 6-3, il nostro `match` ha quattro rami.

Il codice associato a ciascun ramo è un’espressione, e il valore risultante
dell’espressione nel ramo che corrisponde è il valore restituito per l’intera
espressione `match`.

Di solito non usiamo le parentesi graffe se il codice del ramo è breve, come nel
Listato 6-3 dove ogni ramo restituisce solo un valore. Se vuoi eseguire più
righe di codice in un ramo del `match`, devi usare le parentesi graffe, e la
virgola che segue il ramo diventa opzionale. Per esempio, il codice seguente
stampa “Penny fortunato!” ogni volta che il metodo viene chiamato con una
`Coin::Penny`, ma restituisce comunque l’ultimo valore del blocco, `1`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

### _Pattern_ che Si Legano ai Valori

Un’altra caratteristica utile dei rami del `match` è che possono legarsi alle
parti dei valori che corrispondono al _pattern_. È così che possiamo estrarre
valori dalle varianti degli _enum_.

Per esempio, modifichiamo una delle nostre varianti dell’_enum_ per contenerci
dei dati. Dal 1999 al 2008, gli Stati Uniti coniarono _quarter_ con design
diversi per ciascuno dei 50 stati su un lato. Nessun’altra moneta aveva design
statali, quindi solo i _quarter_ hanno questa caratteristica peculiare. Possiamo
aggiungere questa informazione al nostro _enum_ cambiando la variante `Quarter`
per includere un valore `StatoUSA` all’interno, come fatto nel Listato 6-4.

<Listing number="6-4" caption="Un _enum_ `Moneta` in cui la variante `Quarter` contiene anche un valore `StatoUSA`">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

</Listing>

Immaginiamo che un amico stia cercando di collezionare tutti e 50 i _quarter_
statali. Mentre separiamo il nostro resto per tipo di moneta, guarderemo anche
il nome dello stato associato a ciascun _quarter_ così, se è uno che al nostro
amico manca, può aggiungerlo alla collezione.

Nell’espressione `match` per questo codice, aggiungiamo una variabile chiamata
`stato` al _pattern_ che corrisponde ai valori della variante `Coin::Quarter`.
Quando un `Coin::Quarter` corrisponde, la variabile `stato` si legherà al valore
dello stato di quel _quarter_. Possiamo poi usare `stato` nel codice di quel
ramo, così:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

Se chiamassimo `valore_in_cent(Moneta::Quarter(StatoUSA::Alaska))`, `moneta`
sarebbe `Moneta::Quarter(StatoUSA::Alaska)`. Quando confrontiamo quel valore con
ciascuno dei rami del `match`, nessuna corrisponde fino a che non raggiungiamo
`Moneta::Quarter(stato)`. A quel punto `stato` sarà vincolato al valore
`StatoUSA::Alaska`. Possiamo quindi usare quel vincolo nell’espressione
`println!`, ottenendo così il valore interno dello stato dalla variante
`Moneta::Quarter`.

### Corrispondenza con `Option<T>`

Nella sezione precedente volevamo ottenere il valore interno `T` di `Some`
quando si usa `Option<T>`; possiamo anche gestire `Option<T>` usando `match`,
proprio come abbiamo fatto con l’_enum_ `Moneta`! Invece di confrontare monete,
confronteremo le varianti di `Option<T>`, ma il funzionamento dell’espressione
`match` rimane lo stesso.

Supponiamo di voler scrivere una funzione che prende un `Option<i32>` e, se c’è
un valore dentro, aggiunge 1 a quel valore. Se non c’è un valore dentro, la
funzione dovrebbe restituire il valore `None` e non tentare di eseguire alcuna
operazione.

Questa funzione è molto semplice da scrivere, grazie a `match`, e apparirà come
nel Listato 6-5.

<Listing number="6-5" caption="Una funzione che utilizza un'espressione `match` su una `Option<i32>`">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

</Listing>

Esaminiamo la prima esecuzione di `piu_uno` in maggiore dettaglio. Quando
chiamiamo `piu_uno(cinque)`, la variabile `x` nel corpo di `piu_uno` avrà il
valore `Some(5)`. Quindi confrontiamo quello rispetto a ciascun ramo del
`match`:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

Il valore `Some(5)` non corrisponde al _pattern_ `None`, quindi si continua con
il ramo successivo:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

`Some(5)` corrisponde a `Some(i)`? Sì! Abbiamo la stessa variante. `i` si lega
al valore contenuto in `Some`, quindi `i` assume il valore `5`. Il codice nel
ramo del `match` viene quindi eseguito: aggiungiamo 1 al valore di `i` e creiamo
un nuovo valore `Some` con il totale `6` all’interno.

Consideriamo ora la seconda chiamata di `piu_uno` nel Listato 6-5, dove `x` è
`None`. Entriamo nel `match` e confrontiamolo con il primo ramo:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

Corrisponde! Non c’è alcun valore a cui aggiungere, quindi il programma si ferma
e restituisce il valore `None` sul lato destro di `=>`. Poiché il primo ramo ha
corrisposto, nessun altro ramo viene confrontato.

Combinare `match` ed _enum_ è utile in molte situazioni. Vedrai questo schema
spesso nel codice Rust: fai `match` su un _enum_, leghi una variabile ai dati
interni e poi esegui codice basato su di essi. All’inizio è un po’ ostico, ma
una volta che ci prendi la mano vorrai averlo in tutti i linguaggi. È un
costrutto tra i preferiti dagli utenti.

### Le Corrispondenze sono Esaustive

C’è un altro aspetto di `match` da discutere: i _pattern_ dei rami devono
coprire tutte le possibilità. Considera questa versione della nostra funzione
`piu_uno`, che contiene un bug e non si compilerà:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

Non abbiamo gestito il caso `None`, quindi questo codice provocherà un errore.
Per fortuna, è un errore che Rust sa come intercettare. Se proviamo a compilare
questo codice, otterremo questo errore:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

Rust sa che non abbiamo coperto ogni possibile caso, e sa persino quale
_pattern_ abbiamo dimenticato! I `match` in Rust sono esaustivi (_exhaustive_):
dobbiamo coprire ogni possibilità affinché il codice sia valido. Soprattutto nel
caso di `Option<T>`, quando Rust ci impedisce di dimenticare di gestire
esplicitamente il caso `None`, ci protegge dall’assumere che abbiamo un valore
quando potremmo avere _null_, rendendo impossibile "l'errore da un miliardo di
dollari” accennato nel capitolo precedente.

### _Pattern_ Pigliatutto e Segnaposto `_`

Usando gli _enum_, possiamo anche eseguire determinate azioni per alcuni valori
particolari, ma per tutti gli altri valori adottare un’azione predefinita.
Immagina di implementare un gioco dove, se tiri un 3 in un lancio di dadi, il
tuo giocatore non si muove ma riceve un nuovo cappello buffo. Se tiri un 7, il
giocatore perde il cappello buffo. Per tutti gli altri valori, il giocatore si
muove di quel numero di spazi sulla tavola di gioco. Ecco un `match` che
implementa quella logica, con il risultato del lancio specificato anziché
casuale, e tutta l’altra logica rappresentata da funzioni senza corpo perché
implementarli esula da questo esempio:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

Per i primi due rami, i _pattern_ sono i valori letterali `3` e `7`. Per
l’ultimo ramo che copre tutti gli altri valori possibili, il _pattern_ è la
variabile che abbiamo scelto di chiamare `altro`. Il codice che viene eseguito
per il ramo `altro` usa la variabile passando il suo valore alla funzione
`muovi_giocatore`.

Questo codice compila, anche se non abbiamo elencato tutti i possibili valori
che un `u8` può avere, perché l’ultimo _pattern_ corrisponderà a tutti i valori
non specificamente elencati. Questo _pattern_ pigliatutto (_catch-all_) soddisfa
il requisito che `match` deve essere esaustivo. Nota che dobbiamo mettere il
ramo pigliatutto per ultimo perché i _pattern_ sono valutati in ordine. Se
mettessimo il ramo pigliatutto prima, gli altri rami non verrebbero mai
eseguiti, quindi Rust ci avvertirebbe se aggiungessimo rami dopo un pigliatutto!

Rust ha anche un _pattern_ che possiamo usare quando vogliamo un pigliatutto ma
non vogliamo _usare_ il valore corrispondente: `_` è un _pattern_ speciale che
corrisponde a qualsiasi valore e non si lega a quel valore. Questo dice a Rust
che non useremo il valore, quindi Rust non ci segnalerà una variabile
inutilizzata.

Cambiamo le regole del gioco: ora, se tiri qualsiasi cosa diversa da 3 o 7, devi
rilanciare. Non abbiamo più bisogno di usare il valore pigliatutto, quindi
possiamo cambiare il codice per usare `_` al posto della variabile chiamata
`altro`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

Anche questo esempio soddisfa il requisito di esaustività perché stiamo
esplicitamente ignorando tutti gli altri valori nell’ultimo ramo; non abbiamo
dimenticato nulla.

Infine, cambiamo ancora una volta le regole del gioco in modo che non succeda
nient’altro nel tuo turno se tiri qualcosa di diverso da 3 o 7. Possiamo
esprimerlo usando il valore _unit_ ([la _tuple_ vuota][tuples]<!-- ignore -->)
come codice associato al ramo `_`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

Qui stiamo dicendo esplicitamente a Rust che non useremo alcun altro valore che
non corrisponda a un _pattern_ in un ramo precedente, e non vogliamo eseguire
alcun codice in questo caso.

C’è molto altro sui _pattern_ e sul _matching_ che tratteremo nel [Capitolo
19](ch19-00-patterns.html)<!-- ignore -->. Per ora, passiamo alla sintassi `if let`,
che può essere utile nelle situazioni più semplici in cui l’espressione `match`
risulta un po’ verbosa.

[tuples]: ch03-02-data-types.html#il-type-tupla
