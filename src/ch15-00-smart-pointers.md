# Puntatori Intelligenti

Un _puntatore_ è un concetto generale che rappresenta una variabile che contiene
un indirizzo in memoria. Questo indirizzo fa riferimento, o “punta a”, altri
dati. Il tipo più comune di puntatore in Rust è un _reference_, come hai
imparato nel Capitolo 4. I _reference_ sono indicati dal simbolo `&` e prendono
in prestito il valore a cui puntano. Non hanno capacità speciali oltre al
riferimento ai dati, e non hanno _costi prestazionali aggiuntivi_ (_overhead_).

I _puntatori intelligenti_ (_smart pointers_), d’altra parte, sono strutture
dati che si comportano come un puntatore ma hanno anche metadati e capacità
aggiuntive. Il concetto di puntatori intelligenti non è esclusivo di Rust: i
puntatori intelligenti hanno avuto origine in C++ ed esistono anche in altri
linguaggi. Rust ha una varietà di puntatori intelligenti definiti nella libreria
standard che forniscono funzionalità che vanno oltre quelle fornite dai
_reference_. Per esplorare il concetto generale, esamineremo un paio di esempi
diversi di puntatori intelligenti, incluso un tipo di puntatore intelligente con
_conteggio dei riferimenti_. Questo puntatore consente ai dati di avere più
proprietari tenendo traccia del loro numero e, quando non ne rimane nessuno,
de-allocare i dati.

Rust, con il suo concetto di _ownership_ e _borrowing_, presenta un’ulteriore
differenza tra _reference_ e i puntatori intelligenti: mentre i _reference_
prendono solo in prestito dati, in molti casi i puntatori intelligenti
_posseggono_ i dati a cui puntano.

I puntatori intelligenti sono solitamente implementati tramite _struct_. A
differenza di una normale _struct_, i puntatori intelligenti implementano i
_trait_ `Deref` e `Drop`. Il _trait_ `Deref` consente a un’istanza della
_struct_ del puntatore intelligente di comportarsi come un _reference_ in modo
da poter scrivere codice che funzioni sia con _reference_ che con puntatori
intelligenti. Il _trait_ `Drop` consente di personalizzare il codice che viene
eseguito quando un’istanza del puntatore intelligente esce dallo _scope_. In
questo capitolo, discuteremo entrambi questi _trait_ e dimostreremo perché sono
importanti per i puntatori intelligenti.

Dato che il puntatore intelligente è un design generale utilizzato
frequentemente in Rust, questo capitolo non tratterà tutti i puntatori
intelligenti esistenti. Molte librerie hanno i propri puntatori intelligenti, ed
è anche possibile scriverne di propri. Tratteremo i più comuni nella libreria
standard:

- `Box<T>`, per l’allocazione di valori nell’_heap_
- `Rc<T>`, un _type_ di conteggio dei _reference_ che consente la _ownership_
  multipla
- `Ref<T>` e `RefMut<T>`, accessibili tramite `RefCell<T>`, _type_ che applicano
  le regole di prestito durante l’esecuzione anziché in fase di compilazione

Inoltre, tratteremo il modello di _mutabilità interna_, in cui un _type_
immutabile espone un’API per la mutazione di un valore interno. Discuteremo
anche dei _cicli di riferimento_ e di come possono causare perdite di memoria e
come prevenirle.

Cominciamo!
