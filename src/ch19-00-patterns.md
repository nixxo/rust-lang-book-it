# Pattern e Matching

I _Pattern_ sono una sintassi speciale in Rust per il matching con la struttura dei
tipi, sia complessi che semplici. L'utilizzo di pattern insieme a espressioni `match`
e altri costrutti offre un maggiore controllo sul flusso di controllo di un programma.
Un pattern consiste in una combinazione dei seguenti elementi:

- Letterali
- Array destrutturati, enum, struct o tuple
- Variabili
- Caratteri jolly
- Segnaposto

Alcuni esempi di pattern includono `x`, `(a, 3)` e `Some(Color::Red)`. Nei
contesti in cui i pattern sono validi, questi componenti descrivono la forma dei
dati. Il nostro programma confronta quindi i valori con i pattern per determinare se
ha la forma corretta dei dati per continuare a eseguire una particolare porzione di codice.

Per utilizzare un pattern, lo confrontiamo con un valore. Se il pattern corrisponde al
valore, utilizziamo le parti del valore nel nostro codice. Ricordate le espressioni `match` nel
Capitolo 6 che utilizzavano pattern, come l'esempio della macchina selezionatrice di monete. Se il
valore corrisponde alla forma del pattern, possiamo usare i pezzi indicati. In caso contrario, il codice associato al pattern non verrà eseguito.

Questo capitolo è un riferimento su tutto ciò che riguarda i pattern. Tratteremo le situazioni valide in cui utilizzare i pattern, la differenza tra pattern confutabili e inconfutabili
e i diversi tipi di sintassi dei pattern che potreste incontrare. Entro
la fine del capitolo, saprete come usare i pattern per esprimere molti concetti in modo
chiaro.