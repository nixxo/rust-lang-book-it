## Appendice C: _Trait_ Derivabili

In vari punti del libro, abbiamo discusso dell’attributo `derive`, che si può
applicare a una definizione di _struct_ o _enum_. L’attributo `derive` genera
codice che implementerà un _trait_ con la sua implementazione predefinita sul
_type_ annotato con la sintassi `derive`.

In questa appendice, forniamo un riferimento di tutti i _trait_ nella libreria
standard che si possono usare con `derive`. Ogni sezione copre:

- Quali operatori e metodi l’implementazione derivata di questo _trait_ abilita
- Cosa fa l’implementazione del _trait_ fornita da `derive`
- Cosa significa per il _type_ implementare questo _trait_
- Le condizioni in cui è consentito o non consentito implementare il _trait_
- Esempi di operazioni che richiedono il _trait_

Se si desidera un comportamento diverso da quello fornito dall’attributo
`derive`, consultare la [documentazione della libreria standard][std] per ogni
_trait_ per dettagli su come implementarli manualmente.

I _trait_ elencati qui sono gli unici definiti dalla libreria standard che
possono essere implementati sui vostri _type_ usando `derive`. Altri _trait_
definiti nella libreria standard non hanno un comportamento predefinito sensato,
quindi sta a voi implementarli in modo coerente con gli obiettivi del vostro
codice.

Un esempio di _trait_ che non può essere derivato è `Display`, che gestisce la
formattazione per gli utenti finali. Bisogna sempre considerare il modo
appropriato per mostrare un _type_ all’utente finale. Quali parti del _type_
dovrebbero essere visibili? Quali parti sarebbero rilevanti? Quale formato dati
sarebbe più utile? Il compilatore Rust non ha questa conoscenza, quindi non può
fornire un comportamento predefinito adeguato per voi.

Questa lista di _trait_ derivabili non è esaustiva: le librerie possono
implementare `derive` per i propri _trait_, rendendo la lista di _trait_ che si
possono derivare praticamente aperta. L’implementazione di `derive` coinvolge
l’uso di macro procedurali, trattate nella sezione [“Macro”][macro] del Capitolo
20.

### `Debug` per Output da Programmatore

Il _trait_ `Debug` abilita la formattazione per il _debug_ nelle stringhe di
formato, indicata aggiungendo `:?` all’interno dei segnaposto `{}`.

Il _trait_ `Debug` permette di stampare istanze di un _type_ a scopo di _debug_,
così tu e altri programmatori che usano il tuo _type_ potete ispezionare
un’istanza in un punto particolare dell’esecuzione di un programma.

Il _trait_ `Debug` è richiesto, ad esempio, nell’uso della macro `assert_eq!`.
Questa macro stampa i valori delle istanze passate come argomenti se
l’asserzione di uguaglianza fallisce, così i programmatori possono vedere perché
le due istanze non erano uguali.

### `PartialEq` ed `Eq` per Confronti di Uguaglianza

Il _trait_ `PartialEq` permette di confrontare istanze di un _type_ per
verificarne l’uguaglianza e abilita l’uso degli operatori `==` e `!=`.

Derivare `PartialEq` implementa il metodo `eq`. Quando `PartialEq` è derivato su
_struct_, due istanze sono uguali solo se _tutti_ i campi sono uguali, e sono
diverse se anche solo un campo è diverso. Quando è derivato su _enum_, ogni
variante è uguale a se stessa e diversa dalle altre varianti.

Il _trait_ `PartialEq` è richiesto, ad esempio, dall’uso della macro
`assert_eq!`, che necessita di poter confrontare due istanze per l’uguaglianza.

Il _trait_ `Eq` non ha metodi. Il suo scopo è segnalare che per ogni valore del
_type_ annotato, il valore è uguale a se stesso. Il _trait_ `Eq` può essere
applicato solo a _type_ che implementano anche `PartialEq`, sebbene non tutti i
_type_ che implementano `PartialEq` possano implementare `Eq`. Un esempio sono i
_type_ numerici _float_: l’implementazione del _float_ stabilisce che due
istanze di valore “non-numerico” (“_not-a-number_”, `NaN`) non sono uguali.

Un esempio di quando `Eq` è richiesto è per le chiavi in una `HashMap<K, V>`,
così che la `HashMap<K, V>` possa dire se due chiavi sono uguali.

### `PartialOrd` ed `Ord` per Confronti di Ordinamento

Il _trait_ `PartialOrd` permette di confrontare istanze di un _type_ a scopo di
ordinamento. Un _type_ che implementa `PartialOrd` può essere usato con gli
operatori `<`, `>`, `<=` e `>=`. Il _trait_ `PartialOrd` può essere applicato
solo a _type_ che implementano anche `PartialEq`.

Derivare `PartialOrd` implementa il metodo `partial_cmp`, che restituisce un
`Option<Ordering>` e restituisce `None` quando i valori dati non producono un
ordinamento valido. Un esempio di valore che non produce ordinamento, sebbene la
maggior parte dei valori di quel _type_ possano essere confrontati, è il valore
_float_ `NaN`. Chiamare `partial_cmp` con qualunque numero _float_ e il valore
`NaN` restituirà `None`.

Quando derivato su _struct_, `PartialOrd` confronta due istanze confrontando i
campi in ordine di apparizione nella definizione della _struct_. Quando derivato
su _enum_, le varianti dichiarate prima sono considerate minori rispetto a
quelle definite dopo.

Il _trait_ `PartialOrd` è richiesto, ad esempio, dalla funzione `gen_range` del
_crate_ `rand` che genera un valore casuale nell’intervallo specificato da
un’espressione _range_.

Il _trait_ `Ord` permette di sapere che per qualunque coppia di valori del
_type_ annotato, esiste un ordinamento valido. Il _trait_ `Ord` implementa il
metodo `cmp`, che restituisce un `Ordering` anziché un `Option<Ordering>`,
poiché un ordinamento valido sarà sempre possibile. Il _trait_ `Ord` può essere
applicato solo a _type_ che implementano anche `PartialOrd` ed `Eq` (e `Eq`
richiede `PartialEq`). Quando derivato su _struct_ ed _enum_, `cmp` si comporta
allo stesso modo di `partial_cmp` derivato per `PartialOrd`.

Un esempio di quando `Ord` è richiesto è per la memorizzazione di valori in un
`BTreeSet<T>`, una struttura dati che memorizza dati i dati in base
all’ordinamento dei valori.

### `Clone` e `Copy` per Duplicare Valori

Il _trait_ `Clone` permette di creare esplicitamente una copia profonda di un
valore, e il processo di duplicazione può coinvolgere l’esecuzione di codice
arbitrario e la copia di dati nell’_heap_. Consultare [“Interazione tra
Variabili e Dati con _Clone_”][clone] per ulteriori informazioni.

Derivare `Clone` implementa il metodo `clone`, che per il _type_ chiama `clone`
su ciascuna parte del _type_. Questo significa che tutti i campi o valori del
_type_ devono implementare anch’essi `Clone` per poter derivare `Clone`.

Un esempio di quando `Clone` è richiesto è quando si chiama il metodo `to_vec`
su una _slice_. La _slice_ non possiede le istanze di _type_ che contiene, ma il
vettore restituito da `to_vec` deve possedere le sue istanze, quindi `to_vec`
chiama `clone` su ogni elemento. Di conseguenza, il _type_ contenuto nella
_slice_ deve implementare `Clone`.

Il _trait_ `Copy` permette di duplicare un valore copiando solo i dati
memorizzati sullo _stack_; non è necessario eseguire codice arbitrario.
Consultare [“Duplicare Dati Sullo _Stack_”][copy-stack] per ulteriori
informazioni.

Il _trait_ `Copy` non definisce alcun metodo, per prevenire che i programmatori
ne sovrascrivano i metodi e violino l’assunto che nessun codice arbitrario venga
eseguito. In questo modo, tutti possono assumere che copiare un valore sia molto
veloce.

Si può derivare `Copy` su qualunque _type_ in cui tutte le parti implementano
`Copy`. Un _type_ che implementa `Copy` deve anche implementare `Clone`, perché
un _type_ che implementa `Copy` avrà già una semplice implementazione di `Clone`
che replicherà quanto fa `Copy`.

Il _trait_ `Copy` è raramente richiesto; i _type_ che implementano `Copy` hanno
ottimizzazioni disponibili, il che significa che non bisogna chiamare `clone`,
rendendo il codice più conciso.

Tutto ciò che si può fare con `Copy` si può anche fare con `Clone`, ma il codice
potrebbe essere più lento o dover usare `clone` in più punti.

### `Hash` per Mappare un Valore a un Valore di Dimensione Fissa

Il _trait_ `Hash` permette di prendere un’istanza di un _type_ di dimensione
arbitraria e mappare quell’istanza a un valore di dimensione fissa usando una
funzione di _hash_. Derivare `Hash` implementa il metodo `hash`.
L’implementazione derivata del metodo `hash` combina il risultato di chiamare
`hash` su ciascuna parte del _type_, quindi tutti i campi o valori devono
anch’essi implementare `Hash` per poter derivare `Hash`.

Un esempio di quando `Hash` è richiesto è per memorizzare chiavi in una
`HashMap<K, V>` per archiviare dati in modo efficiente.

### `Default` per Valori Predefiniti

Il _trait_ `Default` permette di creare un valore predefinito per un _type_.
Derivare `Default` implementa la funzione `default`. L’implementazione derivata
della funzione `default` chiama la funzione `default` su ogni parte del _type_,
quindi tutti i campi o valori devono anch’essi implementare `Default` per
derivare `Default`.

La funzione `Default::default` è comunemente usata in combinazione con la
sintassi di aggiornamento delle _struct_ discussa in [“Creare Istanze con la
Sintassi di Aggiornamento delle _Struct_”][agg-struct] nel Capitolo 5. Si
possono personalizzare alcuni campi di una _struct_ e poi impostare un valore
predefinito per gli altri campi usando `..Default::default()`.

Il _trait_ `Default` è richiesto per esempio quando si usa il metodo
`unwrap_or_default` su istanze `Option<T>`. Se l’`Option<T>` è `None`, il metodo
`unwrap_or_default` restituirà il risultato di `Default::default` per il _type_
`T` contenuto nell’`Option<T>`.

[std]: https://doc.rust-lang.org/stable/std/index.html
[macro]: ch20-05-macros.html
[clone]: ch04-01-what-is-ownership.html#interazione-tra-variabili-e-dati-con-clone
[copy-stack]: ch04-01-what-is-ownership.html#duplicare-dati-sullo-stack
[agg-struct]: ch05-01-defining-structs.html#creare-istanze-con-la-sintassi-di-aggiornamento-delle-struct
