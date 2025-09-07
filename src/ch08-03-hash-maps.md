## Memorizzazione di Chiavi con Valori Associati in Mappe Hash

L'ultima delle nostre collezioni comuni è la _mappa hash_. Il _type_ `HashMap<K,
V>` memorizza una mappatura di chiavi di _type_ `K` a valori di _type_ `V`
utilizzando una _funzione di hashing_, che determina come queste chiavi e valori
vengono inseriti in memoria. Molti linguaggi di programmazione supportano questo
tipo di struttura dati, ma spesso usano un nome diverso, come _hash_, _map_,
_object_, _hash table_, _dictionary_ o _associative array_, solo per citarne
alcuni.

Le _mappe hash_ (_hash map_ d'ora in poi) sono utili quando si desidera
ricercare dati non utilizzando un indice, come è possibile con i vettori, ma
utilizzando una chiave che può essere di qualsiasi _type_. Ad esempio, in una
partita, è possibile tenere traccia del punteggio di ogni squadra in una _hash
map_ in cui ogni chiave è il nome di una squadra e i valori sono il punteggio di
ogni squadra. Dato il nome di una squadra, è possibile recuperarne il punteggio.

In questa sezione esamineremo l'API di base delle _hash map_, ma molte altre
funzionalità si nascondono nelle funzioni definite su `HashMap<K, V>` dalla
libreria standard. Come sempre, consultate la documentazione della libreria
standard per ulteriori informazioni.

### Creazione di una Nuova _Hash Map_

Un modo per creare una _hash map_ vuota è usare `new` e aggiungere elementi con
`insert`. Nel Listato 8-20, stiamo tenendo traccia del punteggio di due squadre
i cui nomi sono _Blu_ e _Gialla_. La squadra Blu inizia con 10 punti, mentre la
squadra Gialla inizia con 50.

<Listing number="8-20" caption="Creazione di una nuova _hash map_ e inserimento di chiavi e valori">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

</Listing>

Si noti che dobbiamo prima partare in _scope_ `HashMap` con `use` dalla sezione
dedicata alle collezioni della libreria standard. Delle nostre tre collezioni
comuni, questa è la meno utilizzata, quindi non è inclusa nelle funzionalità
aggiunte allo _scope_ dal _preludio_. Le _hash map_ hanno anche un supporto
minore dalla libreria standard; ad esempio, non esiste una macro integrata per
costruirle.

Proprio come i _vector_, le _hash map_ memorizzano i loro dati nell'_heap_.
Questa `HashMap` ha chiavi di _type_ `String` e valori di _type_ `i32`. Come i
_vector_, le _hash map_ sono omogenee: tutte le chiavi devono avere lo stesso
_type_ e tutti i valori devono avere lo stesso _type_.

### Accesso ai Valori in una _Hash Map_

Possiamo ottenere un valore dalla _hash map_ fornendo la sua chiave al metodo
`get`, come mostrato nel Listato 8-21.

<Listing number="8-21" caption="Accesso al punteggio per la squadra Blu memorizzato nella _hash map_">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

</Listing>

Qui, `punteggio` avrà il valore associato alla squadra Blu e il risultato sarà
`10`. Il metodo `get` restituisce `Option<&V>`; se non c'è alcun valore per
quella chiave nella _hash map_, `get` restituirà `None`. Questo programma
gestisce `Option` chiamando `copied` per ottenere `Option<i32>` anziché
`Option<&i32>`, quindi `unwrap_or` per impostare `punteggio` a zero se
`punteggio` non ha una voce per la chiave.

Possiamo iterare su ogni coppia chiave-valore in una _hash map_ in modo simile a come facciamo con i vettori, utilizzando un ciclo `for`:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

Questo codice stamperà ogni coppia in un ordine arbitrario:

```text
Gialla: 50
Blu: 10
```

### _Hash Map_ e _Ownership_

Per i _type_ che implementano il _trait_ `Copy`, come `i32`, i valori vengono
copiati nella _hash map_. Per i valori con _ownership_ come `String`, i valori
verranno spostati e la _hash map_ assumerà la _ownership_ di tali valori, come
dimostrato nel Listato 8-22.

<Listing number="8-22" caption="Mostra che chiavi e valori sono di proprietà della _hash map_ una volta inseriti">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

</Listing>

Non possiamo utilizzare le variabili `nome_campo` e `valore_campo` dopo che sono
state spostate nella _hash map_ con la chiamata a `insert`.

Se inseriamo _reference_ a valori nella _hash map_, i valori non verranno
spostati nella _hash map_. I valori a cui puntano i _reference_ devono essere
validi almeno per quanto tempo è valida la _hash map_. Approfondiremo questi
argomenti in [“Validazione dei _Reference_ con le _Lifetime_
”][validating-references-with-lifetimes]<!-- ignore --> nel Capitolo 10.

### Aggiornamento di una _Hash Map_

Sebbene il numero di coppie chiave-valore sia espandibile, a ogni chiave univoca
può essere associato un solo valore alla volta (ma non viceversa: ad esempio,
sia la squadra Blu che quella Gialla potrebbero avere il valore `10` memorizzato
nella _hash map_ `punteggi`).

Quando si desidera modificare i dati in una _hash map_, è necessario decidere
come gestire il caso in cui a una chiave sia già assegnato un valore. È
possibile sostituire il vecchio valore con il nuovo valore, ignorando
completamente il vecchio valore. È possibile mantenere il vecchio valore e
ignorare il nuovo valore, aggiungendo il nuovo valore solo se la chiave _non_ ha
già un valore. Oppure è possibile combinare il vecchio valore e il nuovo valore.
Vediamo come fare ciascuna di queste cose!

#### Sovrascrittura di un Valore

Se inseriamo una chiave e un valore in una _hash map_ e poi inseriamo la stessa
chiave con un valore diverso, il valore associato a quella chiave verrà
sostituito. Anche se il codice nel Listato 8-23 chiama `insert` due volte, la
_hash map_ conterrà solo una coppia chiave-valore perché stiamo inserendo il
valore per la chiave della squadra Blu entrambe le volte.

<Listing number="8-23" caption="Sostituzione di un valore memorizzato con una chiave specifica">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

</Listing>

Questo codice stamperà `{"Blu": 25}`. Il valore originale di `10` è stato
sovrascritto.

<!-- Old headings. Do not remove or links may break. -->
<a id="only-inserting-a-value-if-the-key-has-no-value"></a>

#### Aggiungere una Chiave e un Valore Solo Se una Chiave Non è Presente

È comune verificare se una particolare chiave esiste già nella _hash map_ con un
valore e quindi eseguire le seguenti azioni: se la chiave esiste nella _hash
map_, il valore esistente deve rimanere invariato; se la chiave non esiste,
inserirla e assegnarle un valore.

Le _hash map_ dispongono di un'API speciale per questo scopo, chiamata `entry`,
che accetta la chiave che si desidera controllare come parametro. Il valore
restituito dal metodo `entry` è un _enum_ chiamato `Entry` che rappresenta un
valore che potrebbe esistere o meno. Supponiamo di voler verificare se la chiave
per la squadra Gialla ha un valore associato. In caso contrario, vogliamo
inserire il valore `50`, e lo stesso vale per la squadra Blu. Utilizzando l'API
`entry`, il codice appare come nel Listato 8-24.

<Listing number="8-24" caption="Utilizzo del metodo `entry` per inserire solo se la chiave non ha già un valore">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

</Listing>

Il metodo `or_insert` su `Entry` è definito per restituire un _reference_
mutabile al valore della chiave `Entry` corrispondente se tale chiave esiste, e
in caso contrario, inserisce il parametro come nuovo valore per questa chiave e
restituisce un _reference_ mutabile al nuovo valore. Questa tecnica è molto più
pulita rispetto alla scrittura manuale della logica e, inoltre, si integra
meglio con il _borrow checker_.

L'esecuzione del codice nel Listato 8-24 stamperà `{"Gialla": 50, "Blu": 10}`.
La prima chiamata a `entry` inserirà la chiave per la squadra Gialla con il
valore `50` perché la squadra Gialla non ha già un valore. La seconda chiamata a
`entry` non modificherà la _hash map_ perché la squadra Blu ha già il valore
`10`.

#### Aggiornamento di un Valore in Base al Valore Precedente

Un altro caso d'uso comune per le _hash map_ è cercare il valore di una chiave e
quindi aggiornarlo in base al valore precedente. Ad esempio, il Listato 8-25
mostra un codice che conta quante volte ogni parola appare in un testo.
Utilizziamo una _hash map_ con le parole come chiavi e incrementiamo il valore
per tenere traccia di quante volte abbiamo visto quella parola. Se è la prima
volta che vediamo una parola, inseriremo il valore `0`.

<Listing number="8-25" caption="Conteggio delle occorrenze di parole utilizzando una _hash map_ che memorizza parole e conteggi">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

</Listing>

Questo codice stamperà `{"world": 2, "hello": 1, "wonderful": 1}`. Potresti
vedere le stesse coppie chiave-valore stampate in un ordine diverso: ricorda che
in uno dei paragrafi precedenti, ["Accesso ai Valori in una _Hash
Map_”][access]<!-- ignore -->, l'iterazione su una _hash map_ avviene in un
ordine arbitrario.

Il metodo `split_whitespace` restituisce un iteratore su _slice_, separate da
spazi, del valore in `testo`. Il metodo `or_insert` restituisce un _reference_
mutabile (`&mut V`) al valore della chiave specificata. Qui, memorizziamo quel
_reference_ mutabile nella variabile `conteggio`, quindi per assegnare quel
valore, dobbiamo prima dereferenziare `conteggio` usando l'asterisco (`*`). Il
_reference_ mutabile esce dallo _scope_ alla fine del ciclo `for`, quindi tutte
queste modifiche sono sicure e consentite dalle regole di prestito.

### Funzioni di _Hashing_

Come impostazione predefinita, `HashMap` utilizza una funzione di _hashing_
chiamata _SipHash_ che può fornire resistenza agli attacchi denial-of-service
(DoS) che coinvolgono tabelle di hash[^siphash]<!-- ignore -->. Questo non è
l'algoritmo di _hashing_ più veloce disponibile, ma è un buon compromesso in
termini di maggiore sicurezza che ne deriva, nonostante il costo prestazionale
derivato. Se si profila il codice e si scopre che la funzione di _hashing_
predefinita è troppo lenta per i propri scopi, è possibile passare a un'altra
funzione specificando un _hasher_ diverso. Un _hasher_ è un _type_ che
implementa il _trait_ `BuildHasher`. Parleremo dei _trait_ e di come
implementarli nel [Capitolo 10][traits]<!-- ignore -->. Non è necessario
implementare il proprio _hasher_ da zero; [crates.io](https://crates.io/)<!--
ignore --> offre librerie condivise da altri utenti Rust che forniscono _hasher_
che implementano molti algoritmi di _hashing_ comuni.

[^siphash]: [https://en.wikipedia.org/wiki/SipHash](https://en.wikipedia.org/wiki/SipHash)

## Riepilogo

Vettori, stringhe e _hash map_ forniranno una grande quantità di funzionalità
necessarie nei programmi quando si desidera memorizzare, accedere e modificare
dati. Ecco alcuni esercizi che ora dovresti essere in grado di risolvere:

1. Dato un elenco di interi, usa un _vector_ e restituisci la mediana (quando
ordinati, il valore in posizione centrale) e la moda (il valore che ricorre più
spesso; una _hash map_ sarà utile in questo caso) dell'elenco.
1. Converti delle stringhe in [pig latin][pig-latin]. La prima consonante di
ogni parola viene spostata alla fine della parola e viene aggiunto _ay_, quindi
_*p*rimo_ diventa _rimo-*p*ay_. Le parole che iniziano con una vocale hanno
invece _hay_ aggiunto alla fine (_ananas_ diventa _ananas-hay_). Tieni a mente i
dettagli sulla codifica UTF-8!
1. Utilizzando _hash map_ e _vector_, crea un'interfaccia testuale che consenta
a un utente di aggiungere i nomi dei dipendenti a un reparto di un'azienda; ad
esempio, "Aggiungi Sally a Ingegneria" o "Aggiungi Amir a Vendite". Quindi,
consenti all'utente di recuperare un elenco di tutte le persone in un reparto o
di tutte le persone in azienda per reparto, ordinate alfabeticamente.

La documentazione API della libreria standard descrive i metodi di _vector_, stringhe
e _hash map_ che saranno utili per questi esercizi!

Stiamo entrando in programmi più complessi in cui le operazioni possono fallire,
quindi è il momento perfetto per discutere della gestione degli errori. Lo
faremo in seguito!

[validating-references-with-lifetimes]: ch10-03-lifetime-syntax.html#validazione-dei-reference-con-le-lifetime
[access]: #accesso-ai-valori-in-una-hash-map
[traits]: ch10-02-traits.html
[pig-latin]: https://it.wikipedia.org/wiki/Pig_latin