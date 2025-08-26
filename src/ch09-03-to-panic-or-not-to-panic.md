## `panic!` o non `panic!`

Quindi, come si decide quando chiamare `panic!` e quando restituire
`Result`? Quando il codice va in panico, non c'è modo di recuperare tornare indietro. Si potrebbe chiamare `panic!`
per qualsiasi situazione di errore, indipendentemente dal fatto che ci sia o meno una possibile soluzione, ma
in tal caso si sta prendendo la decisione che una situazione non è recuperabile per conto
del codice chiamante. Quando si sceglie di restituire un valore `Result`, si forniscono al codice chiamante delle opzioni. Il codice chiamante potrebbe scegliere di tentare di rispondere in un modo
appropriato alla situazione, oppure potrebbe decidere che un valore `Err`
in questo caso è irrecuperabile, quindi può chiamare `panic!` e trasformare il tuo
errore recuperabile in un errore irrecuperabile. Pertanto, restituire `Result` è una
buona scelta predefinita quando si definisce una funzione che potrebbe fallire.

In situazioni come esempi, codice prototipo e test, è più
appropriato scrivere codice che vada in panico invece di restituire un `Result`. Esploriamo
il motivo, poi analizziamo le situazioni in cui il compilatore non può dire che
il fallimento è impossibile, ma tu, in quanto essere umano, sì. Il capitolo si concluderà con
alcune linee guida generali su come decidere se andare in panico nel codice di libreria.

### Esempi, Codice Prototipale e Test

Quando si scrive un esempio per illustrare un concetto, includere anche
un codice robusto per la gestione degli errori può rendere l'esempio meno chiaro. Negli esempi,
si comprende che una chiamata a un metodo come `unwrap` che potrebbe andare in panico è intesa come
segnaposto per il modo in cui si desidera che l'applicazione gestisca gli errori, che può
differire in base al comportamento del resto del codice.

Allo stesso modo, i metodi `unwrap` ed `expect` sono molto utili durante la prototipazione,
prima di decidere come gestire gli errori. Lasciano chiari punti nel
codice per quando si è pronti a rendere il programma più robusto.

Se una chiamata a un metodo fallisce in un test, si desidera che l'intero test fallisca, anche se
quel metodo non è la funzionalità in fase di test. Poiché `panic!` è il modo in cui un test
viene contrassegnato come fallito, chiamare `unwrap` o `expect` è esattamente ciò che dovrebbe
accadere.

### Casi In Cui Si Hanno Più Informazioni Del Compilatore

Sarebbe anche appropriato chiamare `expect` quando si dispone di un'altra logica
che garantisce che `Result` abbia un valore `Ok`, ma la logica non è
qualcosa che il compilatore capisce. Si avrà comunque un valore `Result` che
bisogna gestire: qualsiasi operazione si stia chiamando ha comunque la possibilità di
fallire in generale, anche se è logicamente impossibile nella propria
situazione specifica. Se puoi assicurarti, ispezionando manualmente il codice, che non avrai mai
una variante `Err`, è perfettamente accettabile chiamare `expect` e documentare
il motivo per cui pensi di non avere mai una variante `Err` nel testo dell'argomento.
Ecco un esempio:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-08-unwrap-that-cant-fail/src/main.rs:here}}
```

Stiamo creando un'istanza di `IpAddr` analizzando una stringa hardcoded. Possiamo vedere
che `127.0.0.1` è un indirizzo IP valido, quindi è accettabile usare `expect`
qui. Tuttavia, avere una stringa valida hardcoded non cambia il tipo di ritorno
del metodo `parse`: otteniamo comunque un valore `Result` e il compilatore
ci farà comunque gestire `Result` come se la variante `Err` fosse una possibilità
perché il compilatore non è abbastanza intelligente da vedere che questa stringa è sempre un
indirizzo IP valido. Se la stringa dell'indirizzo IP provenisse da un utente anziché essere
hardcoded nel programma e quindi _avesse_ una possibilità di errore,
vorremmo sicuramente gestire `Result` in un modo più robusto. Menzionare il presupposto che questo indirizzo IP sia hardcoded ci indurrà a
modificare `expect` con un codice di gestione degli errori migliore se, in futuro, dovessimo ottenere
l'indirizzo IP da un'altra fonte.

### Linee Guida Per la Gestione degli Errori

È consigliabile mandare in panico il codice quando è possibile che il codice possa
finire in uno stato non valido. In questo contesto, uno _stato non valido_ si verifica quando un presupposto o
un contratto non è stato rispettato, ad esempio quando vengono passati al codice valori non validi,
valori contraddittori o valori mancanti, più uno o
più dei seguenti:

- Lo stato non valido è qualcosa di inaspettato, al contrario di qualcosa che
probabilmente accadrà occasionalmente, come un utente che inserisce dati nel formato
sbagliato.
- Il codice da questo punto in poi deve fare affidamento sul fatto di non trovarsi in questo stato non valido,
piuttosto che verificare la presenza del problema a ogni passaggio.
- Non esiste un buon modo per codificare queste informazioni nei tipi utilizzati. Faremo un esempio di ciò che intendiamo in ["Codifica di stati e comportamento come
tipi”][encoding]<!-- ignore --> nel Capitolo 18.

Se qualcuno chiama il tuo codice e passa valori che non hanno senso, è
meglio restituire un errore, se possibile, in modo che l'utente della libreria possa decidere cosa
fare in quel caso. Tuttavia, nei casi in cui continuare potrebbe essere
insicuro o dannoso, la scelta migliore potrebbe essere quella di chiamare `panic!` e avvisare
la persona che utilizza la tua libreria del bug nel suo codice in modo che possa correggerlo durante
lo sviluppo. Allo stesso modo, `panic!` è spesso appropriato se stai chiamando
codice esterno fuori dal tuo controllo e restituisce uno stato non valido che
non hai modo di correggere.

Tuttavia, quando è previsto un errore, è più appropriato restituire un `Result`
piuttosto che effettuare una chiamata `panic!`. Esempi includono un parser che riceve dati non validi
o una richiesta HTTP che restituisce uno stato che indica il raggiungimento di un limite di velocità. In questi casi, restituire un `Result` indica che un errore è una
possibilità prevista che il codice chiamante deve decidere come gestire.

Quando il codice esegue un'operazione che potrebbe mettere a rischio un utente se viene
chiamata utilizzando valori non validi, il codice dovrebbe prima verificare che i valori siano validi
e generare un errore di panico se i valori non sono validi. Questo avviene principalmente per motivi di sicurezza:
tentare di operare su dati non validi può esporre il codice a vulnerabilità.
Questo è il motivo principale per cui la libreria standard chiamerà `panic!` se si tenta
un accesso alla memoria fuori dai limiti: tentare di accedere a memoria che non appartiene
alla struttura dati corrente è un problema di sicurezza comune. Le funzioni spesso hanno dei
_contracts (contratti)_: il loro comportamento è garantito solo se gli input soddisfano determinati
requisiti. Andare nel panico quando il contratto viene violato ha senso perché una
violazione del contratto indica sempre un bug lato chiamante, e non è un tipo di
errore che si desidera che il codice chiamante debba gestire esplicitamente. In effetti, non esiste
un modo ragionevole per il codice chiamante di recuperare; i _programmatori_ che chiamano il codice devono
correggerlo. I contratti per una funzione, soprattutto quando una violazione
causerà un panico, dovrebbero essere spiegati nella documentazione API della funzione.

Tuttavia, avere molti controlli di errore in tutte le funzioni sarebbe prolisso
e fastidioso. Fortunatamente, è possibile utilizzare il sistema di tipi di Rust (e quindi il controllo di tipo
effettuato dal compilatore) per eseguire molti dei controlli al posto vostro. Se la vostra
funzione ha un tipo particolare come parametro, potete procedere con la logica del codice
sapendo che il compilatore ha già verificato la presenza di un valore valido. Ad
esempio, se avete un tipo anziché un'`Option`, il vostro programma si aspetta di
avere _qualcosa_ anziché _niente_. Il codice non dovrà quindi gestire
due casi per le varianti `Some` e `None`: avrà un solo caso per
avere sicuramente un valore. Il codice che tenta di non passare nulla alla funzione non verrà
nemmeno compilato, quindi la funzione non dovrà verificare quel caso in fase di esecuzione.
Un altro esempio è l'utilizzo di un tipo intero senza segno come `u32`, che garantisce
che il parametro non sia mai negativo.

### Creazione di Tipi Personalizzati per la Convalida

Sviluppiamo ulteriormente l'idea di utilizzare il sistema di tipi di Rust per garantire un valore valido e proviamo a creare un tipo personalizzato per la convalida. Ricordiamo il
gioco di indovinelli del Capitolo 2 in cui il nostro codice chiedeva all'utente di indovinare un numero
compreso tra 1 e 100. Non abbiamo mai verificato che la risposta dell'utente fosse compresa tra
quei numeri prima di confrontarla con il nostro numero segreto; abbiamo solo verificato che
la risposta fosse positiva. In questo caso, le conseguenze non sono state poi così gravi: il nostro
output "Troppo alto" o "Troppo basso" sarebbe stato comunque corretto. Ma sarebbe stato un
utile miglioramento guidare l'utente verso risposte valide e avere un comportamento
diverso quando l'utente indovina un numero fuori dall'intervallo rispetto a quando
l'utente digita, ad esempio, delle lettere.

Un modo per farlo sarebbe analizzare l'ipotesi come `i32` invece che solo come
`u32` per consentire numeri potenzialmente negativi, e quindi aggiungere un controllo per verificare che il
numero sia compreso nell'intervallo, in questo modo:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-09-guess-out-of-range/src/main.rs:here}}
```

</Listing>

L'espressione `if` verifica se il nostro valore è fuori dall'intervallo, informa l'utente
del problema e chiama `continue` per avviare l'iterazione successiva del ciclo
e richiedere un'altra ipotesi. Dopo l'espressione `if`, possiamo procedere con i
confronti tra `ipotesi` e il numero segreto, sapendo che `ipotesi` è
compreso tra 1 e 100.

Tuttavia, questa non è una soluzione ideale: se fosse assolutamente fondamentale che il
programma operasse solo su valori compresi tra 1 e 100, e avesse molte funzioni
con questo requisito, avere un controllo di questo tipo in ogni funzione sarebbe
noioso (e potrebbe influire sulle prestazioni).

Invece, possiamo creare un nuovo tipo in un modulo dedicato e inserire le validazioni in
una funzione per creare un'istanza del tipo, anziché ripetere le validazioni
ovunque. In questo modo, le funzioni possono utilizzare il nuovo tipo nelle
loro firme in tutta sicurezza e utilizzare con sicurezza i valori che ricevono. Il Listato 9-13 mostra
un modo per definire un tipo `Ipotesi` che creerà un'istanza di `Ipotesi` solo se
la funzione `new` riceve un valore compreso tra 1 e 100.

<Listing number="9-13" caption="Un tipo `Ipotesi` che continuerà solo con valori compresi tra 1 e 100" file-name="src/guessing_game.rs">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-13/src/guessing_game.rs}}
```

</Listing>

Si noti che questo codice in *src/guessing_game.rs* dipende dall'aggiunta di una dichiarazione di modulo
`mod guessing_game;` in *src/lib.rs* che non abbiamo mostrato qui. All'interno del file di questo nuovo modulo, definiamo una struttura in quel modulo denominata `Ipotesi`
che ha un campo denominato `valore` che contiene un `i32`. È qui che verrà memorizzato il numero.

Quindi implementiamo una funzione associata denominata `new` su `Ipotesi` che crea
istanze di valori `Ipotesi`. La funzione `new` è definita per avere un
parametro denominato `valore` di tipo `i32` e restituire un `Ipotesi`. Il codice nel
corpo della funzione `new` verifica `valore` per assicurarsi che sia compreso tra 1 e 100.
Se `valore` non supera questo test, effettuiamo una chiamata `panic!`, che avviserà
il programmatore che sta scrivendo il codice chiamante che ha un bug da
correggere, perché creare un `Ipotesi` con un `valore` al di fuori di questo intervallo violerebbe
il contratto su cui si basa `Ipotesi::new`. Le condizioni in cui
`Ipotesi::new` potrebbe generare un errore di panic dovrebbero essere discusse nella documentazione dell'API pubblica; tratteremo le convenzioni di documentazione che indicano la possibilità
di un errore di panic!` nella documentazione dell'API creata nel Capitolo 14. Se
`valore` supera il test, creiamo un nuovo `Ipotesi` con il suo campo `valore` impostato
sul parametro `valore` e restituiamo `Ipotesi`.

Successivamente, implementiamo un metodo chiamato `valore` che prende in prestito (borrows) `self`, non ha
altri parametri e restituisce un `i32`. Questo tipo di metodo è talvolta chiamato
_getter_ perché il suo scopo è ottenere alcuni dati dai suoi campi e restituirli. È necessario dichiarare questo metodo come public perché il campo `valore` della struttura `Ipotesi`
è privato. È importante che il campo `valore` sia privato, in modo che il codice
che utilizza la struttura `Ipotesi` non possa impostare `valore` direttamente: il codice esterno
al modulo `guessing_game` _deve_ utilizzare la funzione `Ipotesi::new` per creare
un'istanza di `Ipotesi`, garantendo così che `Ipotesi` non possa avere un
`valore` che non sia stato verificato dalle condizioni della funzione `Ipotesi::new`.

Una funzione che ha un parametro o restituisce solo numeri compresi tra 1 e 100 potrebbe
quindi dichiarare nella sua definizione che accetta o restituisce un `Ipotesi` anziché un
`i32` e non avrebbe bisogno di effettuare ulteriori controlli nel suo corpo.

## Riepilogo

Le funzionalità di gestione degli errori di Rust sono progettate per aiutarti a scrivere codice più robusto.
La macro `panic!` segnala che il programma si trova in uno stato che non può gestire e
ti consente di dire al processo di interrompersi invece di provare a procedere con valori non validi o
errati. L'enum `Result` utilizza il sistema di tipi di Rust per indicare che
le operazioni potrebbero fallire in un modo da cui il codice potrebbe ritornare. Puoi usare
`Result` anche per indicare al codice chiamante che deve gestire potenziali
successi o fallimenti. L'utilizzo di `panic!` e `Result` nelle situazioni appropriate
renderà il tuo codice più affidabile di fronte a inevitabili problemi.

Ora che hai visto i modi utili in cui la libreria standard utilizza i generici con
gli enum `Option` e `Result`, parleremo di come funzionano i generics e di come
puoi usarli nel tuo codice.

[encoding]: ch18-03-oo-design-patterns.html#encoding-states-and-behavior-as-types
