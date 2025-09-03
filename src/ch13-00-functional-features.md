# Caratteristiche del Linguaggio Funzionale: Iteratori e Closures

Il design di Rust si è ispirato a molti linguaggi e tecniche esistenti,
e un'influenza significativa è la _programmazione funzionale_.
La programmazione in stile funzionale spesso include l'utilizzo di funzioni come valori,
passandole come argomenti, restituendole da altre funzioni, assegnandole
a variabili per l'esecuzione successiva e così via.

In questo capitolo, non discuteremo la questione di cosa sia o non sia la programmazione funzionale,
ma discuteremo invece alcune caratteristiche di Rust simili a
caratteristiche di molti linguaggi spesso definiti funzionali.

Più specificamente, tratteremo:

- _Closures (Chiusure), un costrutto simile a una funzione che puoi memorizzare in una variabile
- _Iteratori_, un modo per elaborare una serie di elementi
- Come usare closures e iteratori per migliorare il progetto I/O nel Capitolo 12
- Le prestazioni di closures e iteratori (attenzione, spoiler: sono più veloci di
quanto possiate pensare!)

Abbiamo già trattato altre funzionalità di Rust, come il pattern matching e
le enum, che sono anch'esse influenzate dallo stile funzionale. Poiché padroneggiare
closures e iteratori è una parte importante della scrittura di codice Rust idiomatico e veloce, gli dedicheremo l'intero capitolo.
