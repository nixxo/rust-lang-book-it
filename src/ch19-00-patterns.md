# _Pattern_ e Corrispondenza

I _Pattern_ sono una sintassi speciale in Rust per la corrispondenza
(_matching_) con la struttura dei _type_, sia complessi che semplici. L’utilizzo
di _pattern_ insieme a espressioni `match` e altri costrutti offre un maggiore
controllo sul flusso di controllo di un programma. Un _pattern_ consiste in una
combinazione dei seguenti elementi:

- Letterali
- Array destrutturati, _enum_, _struct_ o tuple
- Variabili
- Caratteri jolly
- Segnaposto

Alcuni esempi di _pattern_ includono `x`, `(a, 3)` e `Some(Colore::Rosso)`. Nei
contesti in cui i _pattern_ sono validi, questi componenti descrivono la forma
dei dati. Il nostro programma confronta quindi i valori con i _pattern_ per
determinare se ha la forma corretta dei dati per continuare a eseguire una
particolare porzione di codice.

Per utilizzare un _pattern_, lo confrontiamo con un valore. Se il _pattern_
corrisponde al valore, utilizziamo le parti del valore nel nostro codice.
Ricorda le espressioni `match` nel Capitolo 6 che utilizzavano _pattern_, come
l’esempio della macchina smista monete. Se il valore corrisponde alla forma del
_pattern_, possiamo usare i pezzi indicati. In caso contrario, il codice
associato al _pattern_ non verrà eseguito.

Questo capitolo è un riferimento su tutto ciò che riguarda i _pattern_.
Tratteremo le situazioni valide in cui utilizzare i _pattern_, la differenza tra
_pattern_ confutabili e inconfutabili e i diversi tipi di sintassi dei _pattern_
che potresti incontrare. Alla fine del capitolo, saprai come usare i _pattern_
per esprimere molti concetti in modo chiaro.
