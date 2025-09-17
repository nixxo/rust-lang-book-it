## Appendice H - Note di Traduzione ( in corso )


In questa appendice verranno raccolte note e indicazioni sulle scelte di traduzione usate nel corso di questo lavoro.

Termini inglesi non tradotti ecc.

TODO: scrivere intro...

### Tipi di dato / Strutture Dati

| Terminologia | Termini usati nel libro | Spiegazione Italiano |
| --------- | -------- | -------- |
| Type      | Type / Tipo | Tipo di dato |
| --- | --- | --- |
| Integer   | Intero   |          |
| Float     | Float    | |
| Boolean | Boolean | |
| Struct    | _Struct_   | |
| Enum      | _Enum_     | Enumerazione |
| Tuple     | Tupla / Tuple | |
| Collection | Collezione | |
| Array     | _Array_    | specificatamente riferito al Type Array |
| Vector    | Vettore | specificatamente riferito al Type Vector |
| Hash Map  | Hash Map  | Mappa hash |
| String Slice / Slice | _Slice_ di stringa / _Slice_ | Riferimento ad una porzione di stringa |
| --- | --- | --- |
| Reference | _Reference_ / Riferimento | Riferimento ad una variabile |
| Trait | Trait | Tratto |
| Trait Bound | Vincolo di Trait | |
| Handle | Handle | Puntatore ad un thread/processo |
| String literal | letterale di stringa o stringa letterale ????
| Numeric literal | lettarale numerico o numero letterale ????



### Ownership e varie

| Terminologia | Termini usati nel libro | Italiano |
| --------- | -------- | -------- |
| Ownership | _Ownership_ | Possesso / Proprietà / Controllo di una variabile sui dati che contiene |
| Borrow Checker | _Borrow Checker_ / Controllo dei prestiti | Funzionalità del compilatore Rust per verificare la consistenza dei riferimenti |
| Borrowed Type | _Type_ Preso in prestito | Tipo di dato di cui si è ricevuta la _ownership_ |
| Owned Type| _Type_ posseduto / _Type_ con _ownership_ | |
| Borrowing Rules | Regole di prestito | |



### Concetti del linguaggio

| Terminologia | Termini usati nel libro | Italiano |
| --------- | -------- | -------- |
| Lifetime | _Lifetime_ / Longevità | |
| Crate | _Crate_ | Contenitore |
| Package | Pacchetto |  |
| Path | _Path_ / Percorso | Percorso file o moduli |
| Root | _Root_ / Radice | 
| Namespace | ??? |  |
| Runtime | Esecuzione | |
| Thread | _Thread_ | |
| Spawned Thread | _Thread_ Generato | |
| Closure | _Closure_ / Chiusura | |
| Environment | Ambiente |
| Refactoring | _Refactoring_ / Riscrittura / Ristrutturazione | |
| Panic | Panic / Panico | |
| Return | Restituire / Ritornare | |
| Return Value | Valore di ritorno / Valore restituito | |
| Iterator | Iteratore |
| Iterator Adapter | Adattatore | Saraebbe "Adattatore all'iteratore"
| Consuming Adapters | Consumatore | Sarebbe "Adattatore all'iteratore che consuma l'adattatore"
| Lazy | _Lazy_ | |


### Gerarchia Moduli

Per la gerarchia tra moduli sono utilizzati termini che si rifanno alla vita reale:

| Originale | Tradotto |
| --------- | -------- |
| Parent    | Genitore |
| Child     | Figlio   |
| Ancestor  | Antenato |