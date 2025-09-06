## Migliorare il nostro progetto I/O

Con queste nuove conoscenze sugli iteratori, possiamo migliorare il progetto I/O del
Capitolo 12 utilizzando gli iteratori per rendere i punti del codice più chiari e
concisi. Vediamo come gli iteratori possono migliorare l'implementazione della
funzione `Config::build` e della funzione `cerca`.

### Rimuovere un `clone` utilizzando un iteratore

Nel Listato 12-6, abbiamo aggiunto del codice che prendeva una porzione di valori `String` e creava
un'istanza della struttura `Config` indicizzando nella porzione e clonando i
valori, consentendo alla struttura `Config` di possedere tali valori. Nel Listato 13-17,
abbiamo riprodotto l'implementazione della funzione `Config::build` così com'era
nel Listato 12-23.

<Listing number="13-17" file-name="src/main.rs" caption="Riproduzione della funzione `Config::build` dal Listing 12-23">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/main.rs:ch13}}
```

</Listing>

Allora, avevamo detto di non preoccuparci delle chiamate inefficienti `clone` perché
le avremmo rimosse in futuro. Bene, quel momento è arrivato!

Qui ci serviva `clone` perché abbiamo una slice con elementi `String` nel
parametro `args`, ma la funzione `build` non possiede `args`. Per restituire
la proprietà di un'istanza di `Config`, abbiamo dovuto clonare i valori dai campi `query`
e `file_path` di `Config` in modo che l'istanza di `Config` possa possederne i valori.

Grazie alle nostre nuove conoscenze sugli iteratori, possiamo modificare la funzione `build` per
assumere la proprietà di un iteratore come argomento invece di prendere in prestito una slice.
Utilizzeremo la funzionalità dell'iteratore invece del codice che controlla la lunghezza
della slice e la indicizza in posizioni specifiche. Questo chiarirà cosa fa la
funzione `Config::build`, perché l'iteratore accederà ai valori.

Una volta che `Config::build` assume la proprietà dell'iteratore e smette di utilizzare le operazioni di indicizzazione
che prendono il borrowing, possiamo spostare i valori `String` dall'iteratore a
`Config` anziché chiamare `clone` ed effettuare una nuova allocazione.

#### Utilizzo Diretto dell'Iteratore restituito

Apri il file _src/main.rs_ del tuo progetto I/O, che dovrebbe apparire così:

<span class="filename">File: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```

Per prima cosa modifichiamo l'inizio della funzione `main` che avevamo nel Listato
12-24 con il codice nel Listato 13-18, che questa volta utilizza un iteratore. Questo
non verrà compilato finché non aggiorneremo anche `Config::build`.

<Listing number="13-18" file-name="src/main.rs" caption="Passaggio del valore restituito da `env::args` a `Config::build`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

</Listing>

La funzione `env::args` restituisce un iteratore! Invece di raccogliere i
valori dell'iteratore in un vettore e poi passare una slice a `Config::build`, ora
passiamo la proprietà dell'iteratore restituito da `env::args` direttamente a
`Config::build`.

Dobbiamo quindi aggiornare la definizione di `Config::build`. Modifichiamo la
firma di `Config::build` in modo che assomigli al Listato 13-19. Questo non verrà comunque
compilato, perché dobbiamo aggiornare il corpo della funzione.

<Listing number="13-19" file-name="src/main.rs" caption="Aggiornamento della firma di `Config::build` per aspettarsi un iteratore">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/main.rs:here}}
```

</Listing>

La documentazione della libreria standard per la funzione `env::args` mostra che il
tipo di iteratore restituito è `std::env::Args`, e che tale tipo implementa
il tratto `Iterator` e restituisce valori `String`.

Abbiamo aggiornato la firma della funzione `Config::build` in modo che il parametro
`args` abbia un tipo generico con i limiti del tratto `impl Iterator<Item = String>`
invece di `&[String]`. Questo utilizzo della sintassi `impl Trait`, discusso nella
sezione [“_Trait_ come Parametri”][impl-trait]<!-- ignore --> del Capitolo 10,
significa che `args` può essere qualsiasi tipo che implementi il ​​tratto `Iterator` e
restituisca elementi `String`.

Poiché stiamo prendendo possesso di `args` e lo muteremo
iterandolo, possiamo aggiungere la parola chiave `mut` nella specifica del
parametro `args` per renderlo mutabile.

#### Utilizzo dei Metodi del Triat `Iterator` invece dell'Indicizzazione

Successivamente, correggeremo il corpo di `Config::build`. Poiché `args` implementa il
carattere `Iterator`, sappiamo di poter chiamare il metodo `next` su di esso! Il Listato 13-20
aggiorna il codice del Listato 12-23 per utilizzare il metodo `next`.

<Listing number="13-20" file-name="src/main.rs" caption="Modifica del corpo di `Config::build` per utilizzare i metodi iteratori">

```rust,ignore,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/main.rs:here}}
```
</Listing>

Ricordate che il primo valore restituito da `env::args` è il nome del
programma. Vogliamo ignorarlo e passare al valore successivo, quindi prima chiamiamo
`next` e non facciamo nulla con il valore restituito. Poi chiamiamo `next` per ottenere il
valore che vogliamo inserire nel campo `query` di `Config`. Se `next` restituisce `Some`,
usiamo `match` per estrarre il valore. Se restituisce `None`, significa che non sono stati forniti abbastanza
argomenti e restituiamo subito un valore `Err`. Facciamo la stessa cosa
per il valore `file_path`.

### Rendere il Codice più Chiaro con gli Iterator Adapters

Possiamo anche sfruttare gli iteratori nella funzione `search` nel nostro progetto di I/O,
che è riprodotta qui nel Listato 13-21 come nel Listato 12-19.

<Listing number="13-21" file-name="src/lib.rs" caption="L'implementazione della funzione `cerca` del Listato 12-19">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```

</Listing>

Possiamo scrivere questo codice in modo più conciso utilizzando metodi di adattamento iterativo.
In questo modo evitiamo anche di avere un vettore `results` intermedio mutabile. Lo
stile di programmazione funzionale preferisce ridurre al minimo la quantità di stato mutabile per
rendere il codice più chiaro. La rimozione dello stato mutabile potrebbe consentire un miglioramento futuro
per far sì che la ricerca avvenga in parallelo, poiché non dovremmo gestire
l'accesso simultaneo al vettore `results`. Il Listato 13-22 mostra questa modifica.

<Listing number="13-22" file-name="src/lib.rs" caption="Utilizzo dei metodi di adattamento iterativo nell'implementazione della funzione `cerca`">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

</Listing>

Ricordiamo che lo scopo della funzione `cerca` è restituire tutte le righe in
`contents` che contengono la `query`. Analogamente all'esempio `filter` nel Listato
13-16, questo codice utilizza l'adattatore `filter` per conservare solo le righe per le quali
`line.contains(query)` restituisce `true`. Quindi raccogliamo le righe corrispondenti in
un altro vettore con `collect`. Molto più semplice! Sentitevi liberi di apportare la stessa modifica
per utilizzare i metodi di adattamento iterativo anche nella funzione `search_case_insensitive`.

Per un ulteriore miglioramento, restituisci un iteratore dalla funzione `cerca` rimuovendo
la chiamata a `collect` e modificando il tipo di ritorno in `impl
Iterator<Item = &'a str>` in modo che la funzione diventi un adattatore per iteratori.
Nota che dovrai anche aggiornare i test! Cerca in un file di grandi dimensioni
utilizzando lo strumento `minigrep` prima e dopo aver apportato questa modifica per osservare
la differenza di comportamento. Prima di questa modifica, il programma non visualizzava alcun risultato
finché non aveva raccolto tutti i risultati, ma dopo la modifica, i risultati
verranno visualizzati man mano che viene trovata ogni riga corrispondente, perché il ciclo `for` nella
funzione `run` è in grado di sfruttare "la pigrizia" dell'iteratore.

<!-- Old heading. Do not remove or links may break. -->

<a id="choosing-between-loops-or-iterators"></a>

### Scegliere tra Cicli e Iteratori

La domanda logica successiva è quale stile scegliere nel proprio codice e
perché: l'implementazione originale nel Listato 13-21 o la versione che utilizza
gli iteratori nel Listato 13-22 (supponendo che stiamo raccogliendo tutti i risultati prima
di restituirli piuttosto che restituire l'iteratore). La maggior parte dei programmatori Rust
preferisce usare lo stile iteratore. È un po' più difficile da capire all'inizio,
ma una volta che si è presa familiarità con i vari adattatori di iteratori e con il loro
funzionamento, gli iteratori possono essere più facili da capire. Invece di armeggiare con i vari
pezzi del ciclo e creare nuovi vettori, il codice si concentra sull'obiettivo di alto livello
del ciclo. Questo astrae parte del codice più comune, rendendo
più facile comprendere i concetti specifici di questo codice, come la condizione di filtro
che ogni elemento dell'iteratore deve soddisfare.

Ma le due implementazioni sono davvero equivalenti? L'ipotesi intuitiva
potrebbe essere che il ciclo di livello inferiore sia più veloce. Parliamo di prestazioni.

[impl-trait]: ch10-02-traits.html#trait-come-parametri
