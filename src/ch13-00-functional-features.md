# Funzionalità dei Linguaggi Funzionali: Iteratori e Chiusure

Il design di Rust si è ispirato a molti linguaggi e tecniche esistenti, e
un’influenza significativa è la _programmazione funzionale_. La programmazione
in stile funzionale spesso include l’utilizzo di funzioni come valori,
passandole come argomenti, restituendole da altre funzioni, assegnandole a
variabili per l’esecuzione successiva e così via.

In questo capitolo, non discuteremo la questione di cosa sia o non sia la
programmazione funzionale, ma discuteremo invece alcune caratteristiche di Rust
simili a caratteristiche di molti linguaggi spesso definiti funzionali.

Più specificamente, tratteremo:

- _Chiusure_ (_Closures_), un costrutto simile a una funzione che puoi
  memorizzare in una variabile
- _Iteratori_, un modo per elaborare una serie di elementi
- Come usare chiusure e iteratori per migliorare il progetto I/O del Capitolo 12
- Le prestazioni di chiusure e iteratori (spoiler: sono più veloci di quanto
  possiate pensare!)

Abbiamo già trattato altre funzionalità di Rust, come il _pattern_ _matching_ e
le _enum_, che sono anch’esse influenzate dallo stile funzionale. Poiché
padroneggiare chiusure e iteratori è una parte importante della scrittura di
codice Rust veloce e idiomatico, gli dedicheremo l’intero capitolo.
