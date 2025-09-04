# _Type_ Generici, _Trait_ e _Lifetime_

Ogni linguaggio di programmazione dispone di strumenti per gestire efficacemente
la duplicazione di concetti. In Rust, uno di questi strumenti sono i _type_
generici (_generics_): sostituti astratti per _type_ concreti o altre proprietà.
Possiamo esprimere il comportamento dei _type_ generici o come si relazionano ad
altri _type_ generici senza sapere cosa ci sarà al loro posto durante la
compilazione e l'esecuzione del codice.

Le funzioni possono accettare parametri di un _type_ generico, invece di un
_type_ concreto come `i32` o `String`, allo stesso modo in cui accettano
parametri con valori sconosciuti per eseguire lo stesso codice su più valori
concreti. Infatti, abbiamo già utilizzato i generici nel Capitolo 6 con
`Option<T>`, nel Capitolo 8 con `Vec<T>` e `HashMap<K, V>` e nel Capitolo 9 con
`Result<T, E>`. In questo capitolo, esplorerai come definire i tuoi _type_,
funzioni e metodi personalizzati con i generici!

Per prima cosa, esamineremo come estrarre una funzione per ridurre la
duplicazione del codice. Utilizzeremo quindi la stessa tecnica per creare una
funzione generica da due funzioni che differiscono solo per il _type_ dei loro
parametri. Spiegheremo anche come utilizzare i _type_ generici nelle definizioni
di _struct_ ed _enum_.

Poi imparerai come utilizzare i _trait_ per definire il comportamento in modo
generico. Puoi combinare i _trait_ con i _type_ generici per vincolare un _type_
generico ad accettare solo i _type_ che hanno un comportamento particolare,
anziché qualsiasi _type_.

Infine, parleremo della _longevità_ (_lifetime_ d'ora in poi): una varietà di
generici che forniscono al compilatore informazioni su come i _reference_ si
relazionano tra loro. I _lifetime_ ci permettono di fornire al compilatore
informazioni sufficienti sui valori presi in prestito in modo che possa
garantire che i _reference_ siano validi in più situazioni di quante ne potrebbe
avere senza il nostro aiuto.

## Rimozione della Duplicazione Mediante l'Estrazione di una Funzione

I _type_ generici ci permettono di sostituire _type_ specifici con un segnaposto
che rappresenta più _type_ per evitare la duplicazione del codice. Prima di
addentrarci nella sintassi dei _type_ generici, vediamo come rimuovere le
ripetizioni in un modo che non coinvolga _type_ generici, estraendo una funzione
che sostituisce valori specifici con un segnaposto che rappresenta più valori.
Poi applicheremo la stessa tecnica per estrarre una funzione generica!
Analizzando come riconoscere il codice duplicato che è possibile estrarre in una
funzione, inizieremo a riconoscere il codice duplicato che può utilizzare i
generici.

Inizieremo con il breve programma nel Listato 10-1 che trova il numero più
grande in un elenco.

<Listing number="10-1" file-name="src/main.rs" caption="Trovare il numero più grande in un elenco di numeri">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-01/src/main.rs:here}}
```

</Listing>

Memorizziamo un elenco di numeri interi nella variabile `lista_numeri` e
inseriamo un _reference_ al primo numero dell'elenco in una variabile denominata
`maggiore`. Quindi eseguiamo un'iterazione su tutti i numeri dell'elenco e, se
il numero corrente è più grende del numero memorizzato in `maggiore`,
sostituiamo il _reference_ in quella variabile. Tuttavia, se il numero corrente
è minore o uguale al numero più grande visto finora, la variabile non cambia e
il codice passa al numero successivo nell'elenco. Dopo aver considerato tutti i
numeri nell'elenco, `maggiore` dovrebbe riferirsi al numero più grande, che in
questo caso è 100.

Ora ci è stato chiesto di trovare il numero più grande in due diversi elenchi di
numeri. Per farlo, possiamo scegliere di duplicare il codice nel Listato 10-1 e
utilizzare la stessa logica in due punti diversi del programma, come mostrato
nel Listato 10-2.

<Listing number="10-2" file-name="src/main.rs" caption="Codice per trovare il numero più grande in *due* elenchi di numeri">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-02/src/main.rs}}
```

</Listing>

Sebbene questo codice funzioni, duplicarlo è noioso e soggetto a errori.
Dobbiamo anche ricordarci di aggiornare il codice in più punti quando vogliamo
modificarlo.

Per eliminare questa duplicazione, creeremo un'astrazione definendo una funzione
che opera su qualsiasi elenco di interi passati come parametro. Questa soluzione
rende il nostro codice più chiaro e ci permette di esprimere il concetto di
ricerca del numero più grande in un elenco in modo astratto.

Nel Listato 10-3, estraiamo il codice che trova il numero più grande in una
funzione denominata `maggiore`. Quindi chiamiamo la funzione per trovare il
numero più grande nelle due liste del Listato 10-2. Potremmo anche utilizzare la
funzione su qualsiasi altro elenco di valori `i32` che potremmo avere in futuro.

<Listing number="10-3" file-name="src/main.rs" caption="Codice astratto per trovare il numero più grande in due liste">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-03/src/main.rs:here}}
```

</Listing>

La funzione `maggiore` ha un parametro chiamato `lista`, che rappresenta
qualsiasi _slice_ concreta di valori `i32` che potremmo passare alla funzione.
Di conseguenza, quando chiamiamo la funzione, il codice viene eseguito sui
valori specifici che passiamo.

In sintesi, ecco i passaggi che abbiamo seguito per modificare il codice dal
Listato 10-2 al Listato 10-3

1. Identificare il codice duplicato.
1. Estrarre il codice duplicato nel corpo della funzione e specificare gli input
e i valori restituiti da tale codice nella firma della funzione.
1. Aggiornare le due istanze di codice duplicato per chiamare la funzione.

Successivamente, utilizzeremo gli stessi passaggi con i _type_ generici per
ridurre la duplicazione del codice. Allo stesso modo in cui il corpo della
funzione può operare su una `lista` astratta anziché su valori specifici, i
_type_ generici consentono al codice di operare su _type_ astratti.

Ad esempio, supponiamo di avere due funzioni: una che trova l'elemento più
grande in un insieme di valori `i32` e una che trova l'elemento più grande in un
insieme di valori `char`. Come elimineremmo questa duplicazione? Scopriamolo!