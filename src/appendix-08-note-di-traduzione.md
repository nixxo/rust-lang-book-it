## Appendice H - Note di Traduzione ( in corso )


In questa appendice verranno raccolte note e indicazioni sulle scelte di
traduzione usate nel corso di questo lavoro.

Come regola generale è stato scelto di tradurre in italiano termini tecnici che
sono di uso comune nella programmazione o in altri linguaggi di programmazione
(funzioni, moduli, ecc...) e mantenere in inglese termini che sono specifici del
linguaggio Rust.

Segue un elenco, si spera esaustivo, della terminologia usata in questo libro e
della traduzione/non-traduzione con una spiegazione della scelta se necessario.


### Tipi di dato / Strutture Dati

| Terminologia | Termini usati nel libro | Spiegazione |
| --------- | -------- | -------- |
| Type      | Type  | Tipo di dato.  |
| Integer   | Intero |          |
| Float     | Float    | |
| Boolean   | Boolean | |
| Struct    | _Struct_   | |
| Enum      | _Enum_     | Enumerazione |
| Tuple     | Tupla / Tuple | |
| Collection | Collezione | |
| Array     | _Array_    | specificatamente riferito al _type_ **Array** |
| Vector    | Vettore | specificatamente riferito al _type_ **Vec** |
| Hash Map  | Hash Map  | Mappa hash |
| Slice | _Slice_ | Riferimento ad una porzione di dati |
| String Slice | _Slice_ di stringa | Riferimento ad una porzione di stringa |
| --- | --- | --- |
| Reference | _Reference_ / Riferimento | Riferimento ad una variabile |
| Trait | _Trait_ | Tratto, Caratteristica |
| Trait Bound | Vincolo di _Trait_ | |
| Handle | Handle | Puntatore ad un thread/processo |
| String literal | Letterale stringa | |
| Numeric literal | Letterale numerico | |



### Ownership e varie

| Terminologia | Termini usati nel libro | Italiano |
| --------- | -------- | -------- |
| Ownership | _Ownership_ | Possesso / Proprietà / Controllo di una variabile sui dati che contiene |
| Borrow Checker | _Borrow Checker_ / Controllo dei prestiti | Funzionalità del compilatore Rust per verificare la consistenza dei riferimenti |
| Borrowed Type | _Type_ Preso in prestito | Tipo di dato di cui si è ricevuta la _ownership_ |
| Owned Type| _Type_ posseduto / _Type_ con _ownership_ | |
| Borrowing Rules | Regole di prestito | |
| Lifetime | _Lifetime_ / Longevità | Usato sia termine originale che tradotto per facilità di lettura |



### Concetti del linguaggio

| Terminologia | Termini usati nel libro | Italiano |
| --------- | -------- | -------- |
| Crate | _Crate_ | Contenitore. Mantenuto termine originale per semplicità |
| Package | Pacchetto |  |
| Path | _Path_ / Percorso | Percorso file o moduli |
| Root | _Root_ / Radice / Cartella principale | |
| Workspace | _Workspace_ / Spazio di lavoro | Spazio di lavoro gestito da Cargo |
| Namespace | ??? |  |
| Runtime | Esecuzione | Usato quando si intende l’esecuzione di un programma ecc. |
| Runtime | _Runtime_ | Usato quando si intende il gestore dei blocchi asincroni (Capitolo 16-17 ecc.) |
| Closure | Chiusura | Termine che si trova anche in altri linguaggi |
| Environment | Ambiente | Riferito alle chiusure |
| Refactoring | _Refactoring_ / Riscrittura | Riscrivere, spostare parte del codice |
| Panic | _Panic_ / Panico | |
| Return | Restituire / Ritornare | |
| Return Value | Valore di ritorno / Valore restituito | |
| Iterator | Iteratore |
| Iterator Adapter | Adattatore | Sarebbe “Adattatore all’iteratore” |
| Consuming Adapter | Consumatore | Sarebbe “Adattatore all’iteratore che consuma l’adattatore” |
| Lazy | _Lazy_ | Pigro / Pigrizia |

### Rust Asincrono

| Terminologia | Termini usati nel libro | Italiano |
| --------- | -------- | -------- |
| Concurrency | Concorrenza | |
| Async | _Async_ / Asincrono | Usato il termine originale quando specificamente richiesto, tradotto quando usato nella descrizione meno approfondita. |
| Thread | _Thread_ | Mantenuto termine originale per semplicità |
| Task | _Task_ | Mantenuto termine originale per semplicità |
| Spawned Thread/Task | _Thread_/_Task_ Generato | |
| Future | _Future_ | Mantenuto termine originale per semplicità |
| Stream | _Stream_ | Mantenuto termine originale per semplicità |

### Gerarchia Moduli

Per la gerarchia tra moduli sono utilizzati termini che si rifanno alla vita reale:

| Originale | Tradotto |
| --------- | -------- |
| Parent    | Genitore |
| Child     | Figlio   |
| Ancestor  | Antenato |
