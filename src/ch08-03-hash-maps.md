## Memorizzazione di Chiavi con Valori Associati in Mappe Hash

L'ultima delle nostre raccolte comuni è la _mappa hash_. Il tipo `HashMap<K, V>`
memorizza una mappatura di chiavi di tipo `K` a valori di tipo `V` utilizzando una _funzione hashing_, che determina come queste chiavi e valori vengono inseriti in memoria.
Molti linguaggi di programmazione supportano questo tipo di struttura dati, ma spesso
usano un nome diverso, come _hash_, _map_, _object_, _hash table_,
_dictionary_ o _associative array_, solo per citarne alcuni.

Le mappe hash sono utili quando si desidera ricercare dati non utilizzando un indice, come
è possibile con i vettori, ma utilizzando una chiave che può essere di qualsiasi tipo. Ad esempio,
in una partita, è possibile tenere traccia del punteggio di ogni squadra in una mappa hash in cui
ogni chiave è il nome di una squadra e i valori sono il punteggio di ogni squadra. Dato il nome di una squadra, è possibile
recuperarne il punteggio.

In questa sezione esamineremo l'API di base delle mappe hash, ma molte altre funzionalità
si nascondono nelle funzioni definite su `HashMap<K, V>` dalla libreria standard.
Come sempre, consultate la documentazione della libreria standard per ulteriori informazioni.

### Creazione di una Nuova Mappa Hash

Un modo per creare una mappa hash vuota è usare `new` e aggiungere elementi con
`insert`. Nel Listato 8-20, stiamo tenendo traccia dei punteggio di due squadre i cui
nomi sono _Blu_ e _Gialla_. La squadra Blu inizia con 10 punti, mentre la squadra
Gialla inizia con 50.

<Listing number="8-20" caption="Creazione di una nuova mappa hash e inserimento di chiavi e valori">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

</Listing>

Si noti che dobbiamo prima `usare` la `HashMap` dalla sezione dedicata alle collezioni della
libreria standard. Delle nostre tre collezioni comuni, questa è la meno
utilizzata, quindi non è inclusa nelle funzionalità introdotte nell'ambito
automaticamente nella prefazione. Le mappe hash hanno anche un supporto minore dalla
libreria standard; ad esempio, non esiste una macro integrata per costruirle.

Proprio come i vettori, le mappe hash memorizzano i loro dati nell'heap. Questa `HashMap` ha
chiavi di tipo `String` e valori di tipo `i32`. Come i vettori, le mappe hash sono
omogenee: tutte le chiavi devono avere lo stesso tipo e tutti i valori
devono avere lo stesso tipo.

### Accesso ai Valori in una Mappa Hash

Possiamo ottenere un valore dalla mappa hash fornendo la sua chiave al metodo `get`,
come mostrato nel Listato 8-21.

<Listing number="8-21" caption="Accesso al punteggio per la squadra Blu memorizzato nella mappa hash">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

</Listing>

Qui, `punteggio` avrà il valore associato alla squadra Blu e il
risultato sarà `10`. Il metodo `get` restituisce `Option<&V>`; se non c'è alcun
valore per quella chiave nella mappa hash, `get` restituirà `None`. Questo programma
gestisce `Option` chiamando `copied` per ottenere `Option<i32>` anziché
`Option<&i32>`, quindi `unwrap_or` per impostare `punteggio` a zero se `punteggio` non
ha una voce per la chiave.

Possiamo iterare su ogni coppia chiave-valore in una mappa hash in modo simile a come facciamo con i vettori, utilizzando un ciclo `for`:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

Questo codice stamperà ogni coppia in un ordine arbitrario:

```testo
Gialla: 50
Blu: 10
```

### Mappe Hash e Proprietà

Per i tipi che implementano il tratto `Copy`, come `i32`, i valori vengono copiati
nella mappa hash. Per i valori di proprietà come `String`, i valori verranno spostati e
la mappa hash diventerà la proprietaria di tali valori, come dimostrato nel Listato 8-22.

<Listing number="8-22" caption="Mostra che chiavi e valori sono di proprietà della mappa hash una volta inseriti">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

</Listing>

Non possiamo utilizzare le variabili `nome_campo` e `valore_campo` dopo
che sono state spostate nella mappa hash con la chiamata a `insert`.

Se inseriamo riferimenti a valori nella mappa hash, i valori non verranno spostati
nella mappa hash. I valori a cui puntano i riferimenti devono essere validi almeno per
quanto tempo è valida la mappa hash. Approfondiremo questi argomenti in
[“Validazione dei riferimenti con
i tempi di vita”][validating-references-with-lifetimes]<!-- ignore --> nel Capitolo 10.

### Aggiornamento di una Mappa Hash

Sebbene il numero di coppie chiave-valore sia espandibile, a ogni chiave univoca può
essere associato un solo valore alla volta (ma non viceversa:
ad esempio, sia la squadra Blu che quella Gialla potrebbero avere il valore `10`
memorizzato nella mappa hash `punteggi`).

Quando si desidera modificare i dati in una mappa hash, è necessario decidere come
gestire il caso in cui a una chiave sia già assegnato un valore. È possibile sostituire il
vecchio valore con il nuovo valore, ignorando completamente il vecchio valore. È possibile
mantenere il vecchio valore e ignorare il nuovo valore, aggiungendo il nuovo valore solo se la
chiave _non_ ha già un valore. Oppure è possibile combinare il vecchio valore e il
nuovo valore. Vediamo come fare ciascuna di queste cose!

#### Sovrascrittura di un Valore

Se inseriamo una chiave e un valore in una mappa hash e poi inseriamo la stessa chiave
con un valore diverso, il valore associato a quella chiave verrà sostituito.
Anche se il codice nel Listato 8-23 chiama `insert` due volte, la mappa hash
conterrà solo una coppia chiave-valore perché stiamo inserendo il valore per la chiave del Blu
team entrambe le volte.

<Listing number="8-23" caption="Sostituzione di un valore memorizzato con una chiave specifica">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

</Listing>

Questo codice stamperà `{"Blu": 25}`. Il valore originale di `10` è stato
sovrascritto.

<!-- Titoli precedenti. Non rimuovere o i link potrebbero non funzionare. -->

<a id="only-inserting-a-value-if-the-key-has-no-value"></a>

#### Aggiungere una Chiave e un Valore Solo Se una Chiave Non è Presente

È comune verificare se una particolare chiave esiste già nella mappa hash
con un valore e quindi eseguire le seguenti azioni: se la chiave esiste nella
mappa hash, il valore esistente deve rimanere invariato; se la chiave
non esiste, inserirla e assegnarle un valore.

Le mappe hash dispongono di un'API speciale per questo scopo, chiamata `entry`, che accetta la chiave che si desidera
controllare come parametro. Il valore restituito dal metodo `entry` è un enum
chiamato `Entry` che rappresenta un valore che potrebbe esistere o meno. Supponiamo di voler verificare se la chiave per la squadra Gialla ha un valore associato. In caso contrario, vogliamo inserire il valore `50`, e lo stesso vale per la
squadra Blu. Utilizzando l'API `entry`, il codice appare come nel Listato 8-24.

<Listing number="8-24" caption="Utilizzo del metodo `entry` per inserire solo se la chiave non ha già un valore">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

</Listing>

Il metodo `or_insert` su `Entry` è definito per restituire un riferimento mutabile al
valore della chiave `Entry` corrispondente, se tale chiave esiste, e in caso contrario,
inserisce il parametro come nuovo valore per questa chiave e restituisce un riferimento mutabile al nuovo valore. Questa tecnica è molto più pulita rispetto alla scrittura manuale
della logica e, inoltre, si integra meglio con il controllo dei prestiti.

L'esecuzione del codice nel Listato 8-24 stamperà `{"Gialla": 50, "Blu": 10}`. La
prima chiamata a `entry` inserirà la chiave per la squadra Gialla con il valore
`50` perché la squadra Gialla non ha già un valore. La seconda chiamata a
`entry` non modificherà la mappa hash perché la squadra Blu ha già il
valore `10`.

#### Aggiornamento di un Valore in Base al Valore Precedente

Un altro caso d'uso comune per le mappe hash è cercare il valore di una chiave e quindi
aggiornarlo in base al valore precedente. Ad esempio, il Listato 8-25 mostra un codice che
conta quante volte ogni parola appare in un testo. Utilizziamo una mappa hash con
le parole come chiavi e incrementiamo il valore per tenere traccia di quante volte abbiamo
visto quella parola. Se è la prima volta che vediamo una parola, inseriremo prima
il valore `0`.

<Listing number="8-25" caption="Conteggio delle occorrenze di parole utilizzando una mappa hash che memorizza parole e conteggi">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

</Listing>

Questo codice stamperà `{"world": 2, "hello": 1, "wonderful": 1}`. Potresti vedere
le stesse coppie chiave-valore stampate in un ordine diverso: ricorda da ["Accesso
ai valori in una mappa hash”][access]<!-- ignore --> che l'iterazione su una mappa hash
avviene in un ordine arbitrario.

Il metodo `split_whitespace` restituisce un iteratore su sottosezioni, separate da
spazi, del valore in `text`. Il metodo `or_insert` restituisce un riferimento
mutabile (`&mut V`) al valore della chiave specificata. Qui, memorizziamo quel
riferimento mutabile nella variabile `count`, quindi per assegnare quel valore,
dobbiamo prima dereferenziare `count` usando l'asterisco (`*`). Il riferimento
mutabile esce dall'ambito alla fine del ciclo `for`, quindi tutte queste
modifiche sono sicure e consentite dalle regole di prestito.

### Funzioni di Hashing

Per impostazione predefinita, `HashMap` utilizza una funzione di hashing chiamata _SipHash_ che può fornire
resistenza agli attacchi denial-of-service (DoS) che coinvolgono hash
tables[^siphash]<!-- ignore -->. Questo non è l'algoritmo di hashing più veloce
disponibile, ma il compromesso in termini di maggiore sicurezza che ne deriva,
con un calo delle prestazioni, ne vale la pena. Se si profila il codice e si scopre che la funzione hash predefinita
è troppo lenta per i propri scopi, è possibile passare a un'altra funzione
specificando un hasher diverso. Un _hasher_ è un tipo che implementa il
tratto `BuildHasher`. Parleremo dei tratti e di come implementarli nel
[Capitolo 10][traits]<!-- ignore -->. Non è necessario implementare
il proprio hasher da zero; [crates.io](https://crates.io/)<!-- ignore -->
offre librerie condivise da altri utenti Rust che forniscono hasher che implementano molti
algoritmi di hashing comuni.

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

## Riepilogo

Vettori, stringhe e mappe hash forniranno una grande quantità di funzionalità
necessarie nei programmi quando si desidera memorizzare, accedere e modificare dati. Ecco
alcuni esercizi che ora dovresti essere in grado di risolvere:

1. Dato un elenco di interi, usa un vettore e restituisci la mediana (quando ordinati,
il valore in posizione centrale) e la moda (il valore che ricorre più
spesso; una mappa hash sarà utile in questo caso) dell'elenco.
1. Converti le stringhe in pig latin. La prima consonante di ogni parola viene spostata
alla fine della parola e viene aggiunto _ay_, quindi _first_ diventa _irst-fay_. Le parole
che iniziano con una vocale hanno invece _hay_ aggiunto alla fine (_apple_ diventa
_apple-hay_). Tieni a mente i dettagli sulla codifica UTF-8!
1. Utilizzando una mappa hash e vettori, crea un'interfaccia testuale che consenta a un utente di aggiungere
i nomi dei dipendenti a un reparto di un'azienda; ad esempio, "Aggiungi Sally a
Ingegneria" o "Aggiungi Amir a Vendite". Quindi, consenti all'utente di recuperare un elenco di tutte
le persone in un reparto o di tutte le persone in azienda per reparto, ordinate
alfabeticamente.

La documentazione API della libreria standard descrive i metodi di vettori, stringhe
e mappe hash che saranno utili per questi esercizi!

Stiamo entrando in programmi più complessi in cui le operazioni possono fallire, quindi è
il momento perfetto per discutere della gestione degli errori. Lo faremo in seguito!

[validating-references-with-lifetimes]: ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[access]: #accessing-values-in-a-hash-map
[traits]: ch10-02-traits.html
