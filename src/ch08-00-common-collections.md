# Collezioni Comuni

La libreria standard di Rust include diverse strutture dati molto utili
chiamate _collezioni_ (_collections_). La maggior parte degli altri _type_
rappresenta un valore specifico, ma le collezioni possono contenere più valori.
A differenza di array e tuple, i dati a cui puntano queste collezioni vengono
memorizzati nell'_heap_, il che significa che la quantità di dati non deve
essere nota in fase di compilazione e può aumentare o diminuire durante
l'esecuzione del programma. Ogni tipo di collezione ha funzionalità e costi
diversi, e sceglierne una appropriata per le necessità del momento è un'abilità
che si svilupperà nel tempo. In questo capitolo, parleremo di tre collezioni
utilizzate molto spesso nei programmi Rust:

- Un _vector_ che consente di memorizzare un numero variabile di valori uno
  accanto all'altro.
- Una _string_ è una raccolta di caratteri. Abbiamo menzionato il _type_
  `String` in precedenza, ma in questo capitolo ne parleremo in modo
  approfondito.
- Una _hash map_ che consente di associare un valore a una chiave specifica. Si
  tratta di una particolare implementazione della struttura dati più generale
  chiamata _map_.

Per saperne di più sugli altri tipi di collezioni fornite dalla libreria
standard, vedere [la documentazione][collections].

Parleremo di come creare e aggiornare _vector_, _string_ e _hash map_, nonché
cosa rende ciascuna di esse speciale.

[collections]: https://doc.rust-lang.org/stable/std/collections/index.html