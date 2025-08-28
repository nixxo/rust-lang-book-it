# Common Collections

# Collezioni Comuni

La libreria standard di Rust include diverse strutture dati molto utili chiamate
_collections_. La maggior parte degli altri tipi di dati rappresenta un valore specifico, ma
le collezioni possono contenere più valori. A differenza dei tipi array e dei tipi tuple integrati,
i dati a cui puntano queste collezioni vengono memorizzati nell'heap, il che
significa che la quantità di dati non deve essere nota in fase di compilazione e può aumentare
o diminuire durante l'esecuzione del programma. Ogni tipo di collezione ha
funzionalità e costi diversi, e sceglierne una appropriata per la propria
situazione attuale è un'abilità che si svilupperà nel tempo. In questo capitolo, parleremo
di tre collezioni utilizzate molto spesso nei programmi Rust:

- Un _vector_ che consente di memorizzare un numero variabile di valori uno accanto all'altro.
- Una _string_ è una raccolta di caratteri. Abbiamo menzionato il tipo `String`
in precedenza, ma in questo capitolo ne parleremo in modo approfondito.
- Una _hash map_ che consente di associare un valore a una chiave specifica. Si tratta di una
particolare implementazione della struttura dati più generale chiamata _map_.

Per saperne di più sugli altri tipi di raccolte fornite dalla libreria standard,
vedere [la documentazione][collections].

Discuteremo come creare e aggiornare vettori, stringhe e mappe hash, nonché
cosa rende ciascuna di esse speciale.

[collections]: https://doc.rust-lang.org/stable/std/collections/index.html