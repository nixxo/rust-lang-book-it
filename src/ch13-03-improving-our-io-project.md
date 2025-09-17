## Migliorare il Nostro Progetto I/O

Con queste nuove conoscenze sugli iteratori, possiamo migliorare il progetto I/O
del Capitolo 12 utilizzando gli iteratori per rendere alcuni punti del codice
più chiari e concisi. Vediamo come gli iteratori possono migliorare
l'implementazione della funzione `Config::build` e della funzione `cerca`.

### Rimuovere `clone` Utilizzando un Iteratore

Nel Listato 12-6, abbiamo aggiunto del codice che prendeva una _slice_ di valori
`String` e creava un'istanza della _struct_ `Config` indicizzando nella _slice_
e clonando i valori, consentendo alla _struct_ `Config` di avere _ownership_ di
tali valori. Nel Listato 13-17, abbiamo riprodotto l'implementazione della
funzione `Config::build` così com'era nel Listato 12-23.

<Listing number="13-17" file-name="src/main.rs" caption="Riproduzione della funzione `Config::build` dal Listing 12-23">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/main.rs:ch13}}
```

</Listing>

Allora, avevamo detto di non preoccuparci delle chiamate inefficienti a `clone`
perché le avremmo rimosse in futuro. Bene, quel momento è arrivato!

Qui ci serviva `clone` perché abbiamo una _slice_ con elementi `String` nel
parametro `args`, ma la funzione `build` non ha _ownership_ su `args`. Per
restituire la _ownership_ di un'istanza di `Config`, abbiamo dovuto clonare i
valori dai campi `query` e `percorso_file` di `Config` in modo che l'istanza di
`Config` possa possederne i valori.

Grazie alle nostre nuove conoscenze sugli iteratori, possiamo modificare la
funzione `build` per prendere la _ownership_ di un iteratore come argomento
invece di prendere in prestito una _slice_. Utilizzeremo la funzionalità
dell'iteratore invece del codice che controlla la lunghezza della _slice_ e la
indicizza in posizioni specifiche. Questo chiarirà cosa fa la funzione
`Config::build`, perché l'iteratore accederà ai valori.

Una volta che `Config::build` assume la _ownership_ dell'iteratore e smette di
utilizzare le operazioni di indicizzazione che prendono in prestito, possiamo
spostare i valori `String` dall'iteratore a `Config` anziché chiamare `clone` ed
effettuare una nuova allocazione.

#### Utilizzare Direttamente l'Iteratore Restituito

Apri il file _src/main.rs_ del tuo progetto I/O, che dovrebbe apparire così:

<span class="filename">File: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```

Per prima cosa modifichiamo l'inizio della funzione `main` che avevamo nel
Listato 12-24 con il codice nel Listato 13-18, che questa volta utilizza un
iteratore. Questo non verrà compilato finché non aggiorneremo anche
`Config::build`.

<Listing number="13-18" file-name="src/main.rs" caption="Passaggio del valore restituito da `env::args` a `Config::build`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

</Listing>

La funzione `env::args` restituisce un iteratore! Invece di raccogliere i valori
dell'iteratore in un vettore e poi passare una _slice_ a `Config::build`, ora
passiamo la _ownership_ dell'iteratore restituito da `env::args` direttamente a
`Config::build`.

Dobbiamo quindi aggiornare la definizione di `Config::build`. Modifichiamo la
firma di `Config::build` in modo che assomigli al Listato 13-19. Questo non
verrà comunque ancora compilato, perché dobbiamo aggiornare il corpo della
funzione.

<Listing number="13-19" file-name="src/main.rs" caption="Aggiornamento della firma di `Config::build` per aspettarsi un iteratore">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/main.rs:here}}
```

</Listing>

La documentazione della libreria standard per la funzione `env::args` mostra che
il tipo di iteratore restituito è `std::env::Args`, e che tale _type_ implementa
il _trait_ `Iterator` e restituisce valori `String`.

Abbiamo aggiornato la firma della funzione `Config::build` in modo che il
parametro `args` abbia un _type_ generico con i vincoli del _trait_ `impl
Iterator<Item = String>` invece di `&[String]`. Questo utilizzo della sintassi
`impl Trait`, discusso nella sezione [“_Trait_ come Parametri”][impl-trait]<!--
ignore --> del Capitolo 10, significa che `args` può essere qualsiasi _type_ che
implementi il _trait_ `Iterator` e che restituisca elementi `String`.

Poiché stiamo prendendo la _ownership_ di `args` e lo muteremo iterandolo,
possiamo aggiungere la parola chiave `mut` nella specifica del parametro `args`
per renderlo mutabile.

#### Utilizzare i Metodi del _Trait_ `Iterator` invece dell'Indicizzazione

Successivamente, correggeremo il corpo di `Config::build`. Poiché `args`
implementa il _trait_ `Iterator`, sappiamo di poter chiamare il metodo `next` su
di esso! Il Listato 13-20 aggiorna il codice del Listato 12-23 per utilizzare il
metodo `next`.

<Listing number="13-20" file-name="src/main.rs" caption="Modifica del corpo di `Config::build` per utilizzare i metodi iteratori">

```rust,ignore,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/main.rs:here}}
```
</Listing>

Ricorda che il primo valore restituito da `env::args` è il nome del programma.
Vogliamo ignorarlo e passare al valore successivo, quindi prima chiamiamo `next`
e non facciamo nulla con il valore restituito. Poi chiamiamo `next` per ottenere
il valore che vogliamo inserire nel campo `query` di `Config`. Se `next`
restituisce `Some`, usiamo `match` per estrarre il valore. Se restituisce
`None`, significa che non sono stati forniti abbastanza argomenti e restituiamo
subito un valore `Err`. Facciamo la stessa cosa per il valore `percorso_file`.

### Rendere il Codice più Chiaro con gli Adattatori

Possiamo anche sfruttare gli iteratori nella funzione `cerca` nel nostro progetto di I/O,
che è riprodotta qui nel Listato 13-21 come nel Listato 12-19.

<Listing number="13-21" file-name="src/lib.rs" caption="L'implementazione della funzione `cerca` del Listato 12-19">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```

</Listing>

Possiamo scrivere questo codice in modo più conciso utilizzando gli adattatori.
In questo modo evitiamo anche di avere un vettore mutabile `risultato`. Lo stile
di programmazione funzionale preferisce ridurre al minimo la quantità di stato
mutabile per rendere il codice più chiaro. La rimozione dello stato mutabile
potrebbe consentire un miglioramento futuro per far sì che la ricerca avvenga in
parallelo, poiché non dovremmo gestire l'accesso simultaneo al vettore
`risultati`. Il Listato 13-22 mostra questa modifica.

<Listing number="13-22" file-name="src/lib.rs" caption="Utilizzo degli adattatori nell'implementazione della funzione `cerca`">

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

</Listing>

Ricorda che lo scopo della funzione `cerca` è restituire tutte le righe in
`contenuto` che contengono la `query`. Analogamente all'esempio `filter` nel
Listato 13-16, questo codice utilizza l'adattatore `filter` per conservare solo
le righe per le quali `line.contains(query)` restituisce `true`. Quindi
raccogliamo le righe corrispondenti in un altro vettore con `collect`. Molto più
semplice! Sentitevi liberi di apportare la stessa modifica per utilizzare i
metodi di adattamento iterativo anche nella funzione `cerca_case_insensitive`.

Per un ulteriore miglioramento, restituisci un iteratore dalla funzione `cerca`
rimuovendo la chiamata a `collect` e modificando il _type_ di ritorno in `impl
Iterator<Item = &'a str>` in modo che la funzione diventi essa stessa un
adattatore. Nota che dovrai anche aggiornare i test! Prova ad utilizzare lo
strumento `minigrep` per cercare in un file di grandi dimensioni prima e dopo
aver apportato questa modifica ed osserva la differenza di comportamento. Prima
di questa modifica, il programma non visualizzava alcun risultato finché non
aveva raccolto tutti i risultati, ma dopo la modifica, i risultati verranno
visualizzati man mano che viene trovata ogni riga corrispondente, perché il
ciclo `for` nella funzione `esegui` è in grado di sfruttare "la pigrizia"
(_laziness_) dell'iteratore.

<!-- Old heading. Do not remove or links may break. -->
<a id="choosing-between-loops-or-iterators"></a>

### Scegliere tra Cicli e Iteratori

La domanda logica successiva è quale stile scegliere nel proprio codice e
perché: l'implementazione originale nel Listato 13-21 o la versione che utilizza
gli iteratori nel Listato 13-22 (supponendo che stiamo raccogliendo tutti i
risultati prima di restituirli piuttosto che restituire l'iteratore). La maggior
parte dei programmatori Rust preferisce usare lo stile iteratore. È un po' più
difficile da capire all'inizio, ma una volta che si è presa familiarità con i
vari adattatori e con il loro funzionamento, gli iteratori possono essere più
facili da capire. Invece di armeggiare con i vari pezzi del ciclo e creare nuovi
vettori, il codice si concentra sull'obiettivo di alto livello del ciclo. Questo
astrae parte del codice più comune, rendendo più facile comprendere i concetti
specifici di questo codice, come la condizione di filtro che ogni elemento
dell'iteratore deve soddisfare.

Ma le due implementazioni sono davvero equivalenti? L'ipotesi intuitiva potrebbe
essere che il ciclo di livello inferiore sia più veloce. Parliamo di
prestazioni.

[impl-trait]: ch10-02-traits.html#trait-come-parametri
