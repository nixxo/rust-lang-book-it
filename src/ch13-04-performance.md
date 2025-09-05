## Confronto delle Prestazioni: Cicli vs. Iteratori

Per determinare se utilizzare cicli o iteratori, è necessario sapere quale
implementazione è più veloce: la versione della funzione `cerca` con un ciclo `for` esplicito o la versione con iteratori.

Abbiamo eseguito un benchmark caricando l'intero contenuto di _Le avventure di
Sherlock Holmes_ di Sir Arthur Conan Doyle in una `String` e cercando la
parola _the_ nel contenuto. Ecco i risultati del benchmark sulla
versione di `cerca` che utilizza il ciclo `for` e sulla versione che utilizza gli iteratori:

```text
test bench_search_for ... bench: 19.620.300 ns/iter (+/- 915.700)
test bench_search_iter ... bench: 19.234.900 ns/iter (+/- 657.200)
```

Le due implementazioni hanno prestazioni simili! Non spiegheremo il
codice del benchmark qui perché lo scopo non è dimostrare che le due versioni
siano equivalenti, ma avere un'idea generale di come queste due implementazioni
si confrontino in termini di prestazioni.

Per un benchmark più completo, si consiglia di utilizzare testi di
varie dimensioni come `contents`, parole diverse e parole di lunghezza diversa
come `query` e tutti i tipi di altre varianti. Il punto è questo:
gli iteratori, sebbene siano un'astrazione di alto livello, vengono compilati all'incirca nello
stesso codice che si otterrebbe scrivendo il codice di livello inferiore. Gli iteratori sono una
delle _astrazioni a costo zero_ di Rust, con cui intendiamo che l'utilizzo dell'astrazione
non impone alcun overhead aggiuntivo in fase di esecuzione. Questo è analogo a come Bjarne
Stroustrup, il progettista e implementatore originale del C++, definisce
_zero-overhead_ in "Foundations of C++" (2012):

> In generale, le implementazioni del C++ obbediscono al principio di zero overhead: ciò che
> non usi, non paghi. E inoltre: ciò che usi, non potresti scriverlo
> meglio a mano.

In molti casi, il codice Rust che utilizza gli iteratori viene compilato nello stesso assembly che
scriveresti a mano. Ottimizzazioni come lo srotolamento dei cicli e l'eliminazione dei limiti
di controllo sull'accesso agli array si applicano e rendono il codice risultante estremamente efficiente. Ora che lo sai, puoi usare iteratori e chiusure senza timore!
Fanno sembrare il codice di livello superiore, ma non impongono una penalizzazione alle prestazioni in fase di esecuzione.

## Riepilogo

Chiusure e iteratori sono funzionalità di Rust ispirate alle idee del linguaggio di programmazione funzionale.
Contribuiscono alla capacità di Rust di esprimere chiaramente
idee di alto livello con prestazioni di basso livello. Le implementazioni di chiusure e
iteratori sono tali da non compromettere le prestazioni in fase di esecuzione. Questo fa parte dell'obiettivo
di Rust di fornire astrazioni a costo zero.

Ora che abbiamo migliorato l'espressività del nostro progetto I/O, diamo un'occhiata
ad altre funzionalità di `cargo` che ci aiuteranno a condividere il progetto con
il mondo.