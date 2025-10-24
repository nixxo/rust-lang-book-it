# Funzionalità Avanzate

A questo punto, hai imparato le parti più comunemente usate del linguaggio di
programmazione Rust. Prima di fare un altro progetto, nel Capitolo 21,
esamineremo alcuni aspetti del linguaggio che potresti incontrare di tanto in
tanto, ma che potresti non usare tutti i giorni. Puoi usare questo capitolo come
riferimento quando incontri qualcosa di sconosciuto. Le caratteristiche trattate
qui sono utili in situazioni molto specifiche. Anche se potresti non usarle
spesso, vogliamo assicurarci che tu abbia una comprensione di tutte le
funzionalità che Rust ha da offrire.

In questo capitolo, tratteremo:

- Unsafe Rust: come rinunciare ad alcune delle garanzie di Rust e assumersi la
  responsabilità di mantenere manualmente tali garanzie
- _Trait_ avanzati: _type_ associati, _type_ default dei parametri, sintassi
  completamente qualificata, _supertrait_ e il modello _newtype_ in relazione ai
  _trait_
- _Type_ avanzati: approfondimento sul modello _newtype_, _type_ alias, il
  _type_ _never_ e _type_ a dimensione dinamica
- Funzioni avanzate e chiusure: puntatori a funzione e restituzione di chiusure
- Macro: modi per definire codice che definisce altro codice durante la
  compilazione

È un insieme variegato di funzionalità di Rust con qualcosa per tutti! Iniziamo!
