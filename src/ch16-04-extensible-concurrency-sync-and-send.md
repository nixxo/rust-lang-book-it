## Concorrenza Estensibile con i _Trait_ `Send` e `Sync`

È interessante notare che quasi tutte le funzioni di concorrenza di cui abbiamo
parlato finora in questo capitolo fanno parte della libreria standard, non del
linguaggio stesso. Le opzioni per gestire la concorrenza non sono limitate al
linguaggio o alla libreria standard; puoi scrivere le tue funzioni di
concorrenza o usare quelle scritte da altri.

Tuttavia, tra i concetti chiave della concorrenza che sono incorporati nel
linguaggio piuttosto che nella libreria standard ci sono i _tratti_
`std::marker`, `Send` e `Sync`.

### Permettere il Trasferimento di _Ownership_ tra _Thread_ con `Send`

Il _trait_ marcatore `Send` indica che la _ownership_ dei valori del _type_
che implementa `Send` può essere trasferita tra i _thread_. Quasi tutti i _type_
di Rust implementano `Send`, ma ci sono alcune eccezioni, tra cui `Rc<T>`:
questo non può implementare `Send` perché se si clona un valore `Rc<T>` e si
cerca di trasferire la _ownership_ del clone a un altro _thread_, entrambi i
_thread_ potrebbero aggiornare il conteggio dei _reference_ allo stesso tempo.
Per questo motivo, `Rc<T>` è implementato per essere utilizzato in situazioni a
_thread_ singolo in cui non si vuole pagare una penalizzazione in prestazioni
rispetto ad una maggiore sicurezza.

Pertanto, il sistema dei _type_ di Rust e i vincoli di _trait_ assicurano che
non si possa mai inviare accidentalmente un valore `Rc<T>` tra i _thread_ in
modo non sicuro. Quando abbiamo provato a farlo nel listato 16-14, abbiamo
ottenuto l'errore `` the trait `Send` is not implemented for `Rc<Mutex<i32>>`.
Quando siamo passati ad `Arc<T>`, che implementa `Send`, il codice è stato
compilato.

Qualsiasi _type_ composto interamente da _type_ che implementano `Send`,
anch'esso implementerà automaticamente il _trait_ `Send`. Quasi tutti i _type_
primitivi implementano `Send`, a parte i puntatori grezzi, di cui parleremo nel
Capitolo 20.

### Permettere l'Accesso da Più _Thread_ con `Sync`

Il _trait_ marcatore `Sync` indica che il _type_ che implementa `Sync` può
essere referenziato da più _thread_. In altre parole, qualsiasi _type_ `T`
implementa `Sync` se `&T` (un _reference_ immutabile a `T`) implementa `Send`,
il che significa che il _reference_ può essere inviato in modo sicuro a un altro
_thread_. Analogamente a `Send`, i _type_ primitivi implementano tutti `Sync` e
i _type_ composti interamente da _type_ che implementano `Sync`, anch'essi
implementano `Sync`.

Il puntatore intelligente `Rc<T>` non implementa neanche `Sync` per le stesse
ragioni per cui non implementa `Send`. Il _type_ `RefCell<T>` (di cui abbiamo
parlato nel Capitolo 15) e la famiglia correlata di _type_ `Cell<T>` non
implementano `Sync`. L'implementazione del controllo dei prestiti che
`RefCell<T>` fa durante l'esecuzione non è sicura per l'uso coi _thread_. Invece
il puntatore intelligente `Mutex<T>` implementa `Sync` e può essere utilizzato
per condividere l'accesso con più _thread_, come hai visto in ["Condividere
`Mutex<T>` tra più _Thread_"][sharing-a-mutext]<!-- ignore -->.

### Implementare Manualmente `Send` e `Sync` È Insicuro

Poiché i _type_ composti interamente da altri _type_ che implementano i _trait_
`Send` e `Sync` implementano automaticamente anche `Send` e `Sync`, non dobbiamo
implementare questi _trait_ manualmente. Come _trait_ marcatori, non hanno
nemmeno metodi da implementare. Sono solo utili per far rispettare gli
invarianti relativi alla concorrenza.

L'implementazione manuale di questi _trait_ comporta l'implementazione di codice
Rust insicuro. Parleremo dell'utilizzo di codice Rust insicuro (_Unsafe Rust_)
nel Capitolo 20; per ora, l'informazione importante è che la creazione di nuovi
_type_ concorrenti non costituiti da parti `Send` e `Sync` richiede un'attenta
riflessione per mantenere le garanzie di sicurezza. [“The
Rustonomicon”][nomicon] contiene maggiori informazioni su queste garanzie e su
come rispettarle.

## Summary
## Riepilogo

Non è l'ultima volta che vedrai la concorrenza in questo libro: il prossimo
capitolo si concentra sulla programmazione asincrona e il progetto del Capitolo
21 utilizzerà i concetti di questo capitolo in una situazione più realistica
rispetto agli esempi minori discussi qui.

Come accennato in precedenza, dato che la gestione della concorrenza in Rust fa
parte del linguaggio solo in minima parte, molte soluzioni per la concorrenza
sono implementate sotto forma di _crate_. Questi si evolvono più rapidamente
rispetto alla libreria standard, quindi assicurati di cercare online i _crate_
più aggiornati e all'avanguardia da utilizzare in situazioni in cui necessiti di
elaborazioni _multi-thread_.

La libreria standard di Rust fornisce canali per il passaggio di messaggi e
_type_ di puntatori intelligenti, come `Mutex<T>` e `Arc<T>`, che sono sicuri da
usare in contesti concorrenti. Il sistema dei _type_ e il controllo di prestiti
assicurano che il codice che utilizza queste soluzioni non finisca con accessi
ai dati conflittuali o riferimenti non validi. Una volta che avrai compilato il
tuo codice, potrai essere certo che verrà eseguito felicemente su più _thread_
senza i tipi di bug difficili da rintracciare comuni in altri linguaggi. La
programmazione concorrente non è più un concetto di cui aver paura: vai avanti e
rendi i tuoi programmi concorrenti, senza paura!

[sharing-a-mutext]: ch16-03-shared-state.html#condividere-mutext-tra-più-thread
[nomicon]: https://doc.rust-lang.org/stable/nomicon/index.html
