## _Stream_: _Future_ in Sequenza

Ricorda come abbiamo utilizzato il ricevitore per il nostro canale _async_ in
precedenza nella sezione [“Inviare Dati Tra Due _Task_ Usando il Passaggio di
Messaggi”][17-02-messages]<!-- ignore --> di questo capitolo. Il metodo _async_
`recv` produce una sequenza di elementi nel tempo. Questo è un esempio di un
modello molto più generale noto come _stream_. Molti concetti sono naturalmente
rappresentati come _stream_: elementi che diventano disponibili in una coda,
blocchi di dati che vengono estratti in modo incrementale dal filesystem quando
l'intero set di dati è troppo grande per la memoria del computer, o dati che
arrivano attraverso la rete nel tempo. Poiché gli _stream_ sono _future_,
possiamo usarli con qualsiasi altro tipo di _future_ e combinarli in modi
interessanti. Ad esempio, possiamo raggruppare gli eventi per evitare di
innescare troppe chiamate di rete, impostare timeout su sequenze di operazioni
di lunga durata o limitare gli eventi dell'interfaccia utente per evitare di
svolgere lavori inutili.

Abbiamo visto una sequenza di elementi nel Capitolo 13, quando abbiamo esaminato
il _trait_ `Iterator` nella sezione [“Il _Trait_ `Iterator` e il Metodo
`next`”][iterator-trait]<!-- ignore -->, ma ci sono due differenze tra gli
iteratori e il ricevitore del canale _async_. La prima differenza sono le
tempistiche: gli iteratori sono sincroni, mentre il ricevitore del canale è
asincrono. La seconda è l’API. Quando lavoriamo direttamente con `Iterator`,
chiamiamo il suo metodo sincrono `next`. Con lo _stream_ `trpl::Receiver`, in
particolare, abbiamo invece chiamato un metodo asincrono `recv`. A parte questo,
le API si somigliano molto, e questa somiglianza non è una coincidenza. Uno
_stream_ è come una forma asincrona di iterazione. Mentre il `trpl::Receiver`
aspetta specificamente di ricevere messaggi, però, l’API dello _stream_ di uso
generale è molto più ampia: fornisce il prossimo elemento come fa `Iterator`, ma
in modo asincrono.

La somiglianza tra iteratori e _stream_ in Rust significa che possiamo
effettivamente creare uno _stream_ da qualsiasi iteratore. Come con un
iteratore, possiamo lavorare con uno _stream_ chiamando il suo metodo `next` e
poi aspettare l’output, come nel Listato 17-21, che per ora non si compilerà.

<Listing number="17-21" file-name="src/main.rs" caption="Creazione di uno _stream_ da un iteratore e stampa dei suoi valori">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-21/src/main.rs:stream}}
```

</Listing>

Iniziamo con un array di numeri, che convertiamo in un iteratore e poi chiamiamo
`map` su di esso per raddoppiare tutti i valori. Poi convertiamo l’iteratore in
uno _stream_ usando la funzione `trpl::stream_from_iter`. Successivamente,
iteriamo sugli elementi nello _stream_ man mano che arrivano con il ciclo `while
let`.

Sfortunatamente, quando proviamo a eseguire il codice, non si compila ma invece
riporta che non c’è alcun metodo `next` disponibile:

```text
{{#include ../listings/ch17-async-await/listing-17-21/output.txt:2:18}}
```

Come spiega questo output, la ragione dell’errore del compilatore è che abbiamo
bisogno del _trait_ giusto in _scope_ per poter utilizzare il metodo `next`.
Dato il nostro discorso finora, potresti ragionevolmente aspettarti che quel
_trait_ sia `Stream`, ma in realtà è `StreamExt`. Abbreviazione di _estensione_,
`Ext` è un modello comune nella comunità Rust per estendere un _trait_ con un
altro.

Il _trait_ `Stream` definisce un’interfaccia a basso livello che combina
efficacemente i _trait_ `Iterator` e `Future`. `StreamExt` fornisce un insieme
di API di livello superiore costruite sulla base di `Stream`, inclusi il metodo
`next` e altri metodi utili simili a quelli forniti dal _trait_ `Iterator`.
`Stream` e `StreamExt` non fanno ancora parte della libreria standard di Rust,
ma la maggior parte dei _crate_ dell’ecosistema utilizza definizioni simili.

La soluzione all’errore del compilatore è aggiungere una dichiarazione `use` per
`trpl::StreamExt`, come nel Listato 17-22.

<Listing number="17-22" file-name="src/main.rs" caption="Utilizzo con successo di un iteratore come base per uno _stream_">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-22/src/main.rs:all}}
```

</Listing>

Con tutti questi pezzi messi insieme, questo codice funziona come vogliamo!
Inoltre, ora che abbiamo `StreamExt` in _scope_, possiamo utilizzare tutti i
suoi metodi utili, proprio come con gli iteratori.

[17-02-messages]: ch17-02-concurrency-with-async.html#inviare-dati-tra-due-task-usando-il-passaggio-di-messaggi
[iterator-trait]: ch13-02-iterators.html#il-trait-iterator-e-il-metodo-next
