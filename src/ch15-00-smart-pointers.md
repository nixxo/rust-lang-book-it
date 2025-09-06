# Puntatori Intelligenti

Un _puntatore_ è un concetto generale che rappresenta una variabile che contiene un indirizzo in
memoria. Questo indirizzo fa riferimento, o "punta a", altri dati. Il tipo più
comune di puntatore in Rust è un riferimento, che hai imparato nel
Capitolo 4. I riferimenti sono indicati dal simbolo `&` e prendono in prestito il valore a cui
puntano. Non hanno capacità speciali oltre al riferimento ai
dati, e non hanno overhead.

I _puntatori intelligenti_, d'altra parte, sono strutture dati che si comportano come un
puntatore ma hanno anche metadati e capacità aggiuntive. Il concetto di
puntatori intelligenti non è esclusivo di Rust: i puntatori intelligenti hanno avuto origine in C++ ed esistono
anche in altri linguaggi. Rust ha una varietà di puntatori intelligenti definiti nella
libreria standard che forniscono funzionalità che vanno oltre quelle fornite dai riferimenti.
Per esplorare il concetto generale, esamineremo un paio di esempi diversi di
puntatori intelligenti, incluso un tipo di puntatore intelligente con _conteggio dei riferimenti_. Questo
puntatore consente di consentire ai dati di avere più proprietari tenendo traccia del
numero di proprietari e, quando non ne rimane nessuno, ripulendo i dati.

Rust, con il suo concetto di proprietà e prestito, presenta un'ulteriore differenza
tra riferimenti e puntatori intelligenti: mentre i riferimenti prendono solo in prestito dati, in
molti casi i puntatori intelligenti _posseggono_ i dati a cui puntano.

I puntatori intelligenti sono solitamente implementati tramite struct. A differenza di una normale
struct, i puntatori intelligenti implementano i tratti `Deref` e `Drop`. Il tratto `Deref`
consente a un'istanza della struct del puntatore intelligente di comportarsi come un riferimento
in modo da poter scrivere codice che funzioni sia con riferimenti che con puntatori intelligenti.
Il tratto `Drop` consente di personalizzare il codice che viene eseguito quando un'istanza
del puntatore intelligente esce dall'ambito. In questo capitolo, discuteremo entrambi
questi tratti e dimostreremo perché sono importanti per i puntatori intelligenti.

Dato che lo smart pointer pattern è un design pattern generale utilizzato
frequentemente in Rust, questo capitolo non tratterà tutti gli smart pointer esistenti. Molte
librerie hanno i propri smart pointer, ed è anche possibile scriverne di propri. Tratteremo
i più comuni nella libreria standard:

- `Box<T>`, per l'allocazione di valori sull'heap
- `Rc<T>`, un tipo di conteggio dei riferimenti che consente la proprietà multipla
- `Ref<T>` e `RefMut<T>`, accessibili tramite `RefCell<T>`, un tipo che applica
le regole di prestito a runtime anziché in fase di compilazione

Inoltre, tratteremo il pattern di _mutabilità interna_, in cui un tipo immutabile
espone un'API per la mutazione di un valore interno. Discuteremo anche
i cicli di riferimento: come possono causare perdite di memoria e come prevenirle.

Approfondiamo!
