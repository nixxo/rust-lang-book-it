## Lavorare con le Variabili d'Ambiente

Miglioreremo il programma `minigrep` implementando una funzionalità
aggiuntiva: un'opzione per la ricerca senza distinzione tra maiuscole e
minuscole, che l'utente può attivare tramite una variabile d'ambiente.
Potremmo rendere questa funzionalità un'opzione della riga di comando e
richiedere che gli utenti la inseriscano ogni volta che desiderano applicarla,
ma rendendola invece una variabile d'ambiente, consentiamo ai nostri utenti di
impostare la variabile d'ambiente una sola volta e di fare in modo che tutte
le loro ricerche in quella sessione di terminale siano senza distinzione
(_case-insensitive_).

### Scrivere un Test che Fallisce per la Funzione `cerca` _Case-Insensitive_

Aggiungiamo innanzitutto una nuova funzione `cerca_case_insensitive` alla
libreria `minigrep` che verrà chiamata quando la variabile d'ambiente ha un
valore. Continueremo a seguire il processo TDD, quindi il primo passo sarà di
scrivere un nuovo test che fallisce. Aggiungeremo un nuovo test per la nuova
funzione `cerca_case_insensitive` e rinomineremo il nostro vecchio test da
`un_risultato` a `case_sensitive` per chiarire le differenze tra i due test,
come mostrato nel Listato 12-20.

<Listing number="12-20" file-name="src/lib.rs" caption="Aggiunta di un nuovo test che fallisce per la funzione _case-insensitive_ che stiamo per aggiungere">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-20/src/lib.rs:here}}
```

</Listing>

Nota che abbiamo modificato anche il `contenuto` del vecchio test. Abbiamo
aggiunto una nuova riga con il testo `"Duttilità."` usando una _D_ maiuscola
che non dovrebbe corrispondere alla query `"dut"` quando effettuiamo una
ricerca con distinzione tra maiuscole e minuscole. Modificare il vecchio test
in questo modo ci aiuta a garantire di non interrompere accidentalmente la
funzionalità di ricerca con distinzione tra maiuscole e minuscole che abbiamo
già implementato. Questo test dovrebbe ora essere superato e dovrebbe
continuare a essere superato mentre lavoriamo sulla ricerca senza distinzione
tra maiuscole e minuscole.

Il nuovo test per la ricerca _case-insensitive_ utilizza `"rUsT"` come query.
Nella funzione `cerca_case_insensitive` che stiamo per aggiungere, la query
`"rUsT"` dovrebbe corrispondere alla riga contenente `"Rust:"` con una _R_
maiuscola e corrispondere alla riga `"Una frusta."` anche se entrambe
differiscono dalla query. Questo è il nostro test che fallisce e non verrà
compilato perché non abbiamo ancora definito la funzione
`cerca_case_insensitive`. Sentiti libero di aggiungere un'implementazione
scheletro che restituisca sempre un vettore vuoto, simile a quella che abbiamo
fatto per la funzione `cerca` nel Listato 12-16 per verificare che il test si
compili correttamente e fallisca.

### Implementazione della Funzione `cerca_case_insensitive`

La funzione `cerca_case_insensitive`, mostrata nel Listato 12-21, sarà quasi
la stessa della funzione `cerca`. L'unica differenza è che metteremo in
minuscolo la `query` e ogni `line` in modo che, qualunque sia il carattere
maiuscolo/minuscolo degli argomenti di input, saranno gli stessi quando
controlleremo se la riga contiene la query.

<Listing number="12-21" file-name="src/lib.rs" caption="Definizione della funzione `cerca_case_insensitive` per rendere minuscole la query e la riga prima di confrontarle">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-21/src/lib.rs:here}}
```

</Listing>

Per prima cosa, rendiamo minuscola la stringa `query` e la memorizziamo in una
nuova variabile con lo stesso nome, adombrando la `query` originale. La
chiamata a `to_lowercase` sulla query è necessaria affinché, indipendentemente
dal fatto che la query dell'utente sia `"rust"`, `"RUST"`, `"Rust"` o
`"rUsT"`, la query verrà trattata come se fosse `"rust"` e non sarà
_case-sensitive_. Sebbene `to_lowercase` gestisca Unicode di base, non sarà
accurato al 100%. Se stessimo scrivendo un'applicazione reale, dovremmo
lavorare un po' di più qui, ma questa sezione riguarda le variabili
d'ambiente, non Unicode, quindi ci fermeremo qui.

Nota che `query` ora è una `String` anziché una _slice_ di stringa, perché la
chiamata a `to_lowercase` crea nuovi dati anziché fare _reference_ a dati
esistenti. Supponiamo che la query sia `"rUsT"`, ad esempio: quella _slice_
non contiene una `u` o una `t` minuscola da utilizzare, quindi dobbiamo
allocare una nuova `String` contenente `"rust"`. Quando passiamo `query` come
argomento al metodo `contains` ora, dobbiamo aggiungere una & (e commerciale)
perché la firma di `contains` è definita per accettare una _slice_ di stringa.

Successivamente, aggiungiamo una chiamata a `to_lowercase` su ogni `line` per
convertire in minuscolo tutti i caratteri della riga su cui stiamo facendo la
ricerca. Ora che abbiamo convertito `line` e `query` in minuscolo, troveremo
corrispondenze indipendentemente dalle maiuscole e dalle minuscole della
query.

Vediamo se questa implementazione supera i test:

```console
{{#include ../listings/ch12-an-io-project/listing-12-21/output.txt}}
```

Ottimo! Hanno superato i test. Ora, chiamiamo la nuova funzione
`cerca_case_insensitive` dalla funzione `run`. Per prima cosa, aggiungeremo
un'opzione di configurazione alla struttura `Config` per passare dalla ricerca
_case-sensitive_ a quella _case-insensitive_. L'aggiunta di questo campo
causerà errori di compilazione perché non lo stiamo ancora inizializzando da
nessuna parte:

<span class="filename">File: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/main.rs:here}}
```

Abbiamo aggiunto il campo `ignora_maiuscole` che contiene un valore booleano.
Successivamente, abbiamo bisogno della funzione `run` per controllare il
valore del campo `ignora_maiuscole` e utilizzarlo per decidere se chiamare la
funzione `cerca` o la funzione `cerca_case_insensitive` come mostrato nel
Listato 12-22. Questo non verrà ancora compilato.

<Listing number="12-22" file-name="src/main.rs" caption="Chiamata `cerca` o `cerca_case_insensitive` in base al valore in `config.ignora_maiuscole`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/main.rs:there}}
```

</Listing>

Infine, dobbiamo verificare la variabile d'ambiente. Le funzioni per lavorare
con le variabili d'ambiente si trovano nel modulo `env` della libreria
standard, che è già nello _scope_ all'inizio di _src/main.rs_. Useremo la
funzione `var` del modulo `env` per verificare se è stato impostato un valore
per una variabile d'ambiente denominata `IGNORA_MAIUSCOLE`, come mostrato nel
Listato 12-23.

<Listing number="12-23" file-name="src/main.rs" caption="Controllo di qualsiasi valore in una variabile d'ambiente denominata `IGNORA_MAIUSCOLE`">

```rust,ignore,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-23/src/main.rs:here}}
```

</Listing>

Qui creiamo una nuova variabile, `ignora_maiuscole`. Per impostarne il valore,
chiamiamo la funzione `env::var` e le passiamo il nome della variabile
d'ambiente `IGNORA_MAIUSCOLE`. La funzione `env::var` restituisce un `Result`
che sarà la variante `Ok` corretta che contiene il valore della variabile
d'ambiente se la variabile d'ambiente è impostata su un valore qualsiasi.
Restituirà la variante `Err` se la variabile d'ambiente non è impostata.

Stiamo utilizzando il metodo `is_ok` su `Result` per verificare se la
variabile d'ambiente è impostata, il che significa che il programma dovrebbe
eseguire una ricerca senza distinzione tra maiuscole e minuscole. Se la
variabile d'ambiente `IGNORA_MAIUSCOLE` non è impostata, `is_ok` restituirà
`false` e il programma eseguirà una ricerca facendo distinzione tra maiuscole
e minuscole. Non ci interessa il _valore_ della variabile d'ambiente, ma solo
se è impostata o meno, quindi controlliamo `is_ok` anziché utilizzare
`unwrap`, `expect` o uno qualsiasi degli altri metodi che abbiamo visto su
`Result`.

Passiamo il valore nella variabile `ignora_maiuscole` all'istanza `Config` in
modo che la funzione `run` possa leggere quel valore e decidere se chiamare
`cerca_case_insensitive` o `cerca`, come abbiamo implementato nel Listato
12-22.

Proviamo! Per prima cosa eseguiamo il nostro programma senza la variabile
d'ambiente impostata e con la query `che`, che dovrebbe corrispondere a
qualsiasi riga che contenga la parola _che_ in minuscolo:

```console
{{#include ../listings/ch12-an-io-project/listing-12-23/output.txt}}
```

Sembra che funzioni ancora! Ora eseguiamo il programma con `IGNORA_MAIUSCOLE`
impostato a `1` ma con la stessa query _che_:

```console
$ IGNORA_MAIUSCOLE=1 cargo run -- che poesia.txt
```

Se si utilizza PowerShell, sarà necessario impostare la variabile d'ambiente
ed eseguire il programma con comandi separati:

```console
PS> $Env:IGNORA_MAIUSCOLE=1; cargo run -- che poesia.txt
```

Questo farà sì che `IGNORA_MAIUSCOLE` persista per il resto della sessione
shell. Può essere annullato con il cmdlet `Remove-Item`:

```console
PS> Remove-Item Env:IGNORA_MAIUSCOLE
```

Dovremmo ottenere righe che contengono _che_ e che potrebbero contenere
lettere maiuscole:

<!-- manual-regeneration
cd listings/ch12-an-io-project/listing-12-23
IGNORA_MAIUSCOLE=1 cargo run -- che poesia.txt
can't extract because of the environment variable
-->

```console
Sei Nessuno anche tu?
Che grande peso essere Qualcuno!
che gracida il tuo nome — tutto giugno —
```

Eccellente, abbiamo trovato anche le righe contenenti _C_ maiuscolo! Il nostro
programma `minigrep` ora può effettuare ricerche _case-insensitive_,
controllate da una variabile d'ambiente. Ora sai come gestire le opzioni
impostate utilizzando argomenti della riga di comando o variabili d'ambiente.

Alcuni programmi consentono argomenti _e_ variabili d'ambiente per la stessa
configurazione. In questi casi, i programmi decidono che l'uno o l'altro abbia
la precedenza. COme esercizio e per sperimentare un po', prova a controllare
la distinzione tra maiuscole e minuscole tramite un argomento della riga di
comando o una variabile d'ambiente. Decidi se l'argomento della riga di
comando o la variabile d'ambiente debbano avere la precedenza se il programma
viene eseguito con uno impostato _case-sensitive_ e l'altro impostato come
_case-insensitive_.

Il modulo `std::env` contiene molte altre utili funzionalità per gestire le
variabili d'ambiente: consulta la sua [documentazione][std] per scoprire quali sono
disponibili.

[std]: https://doc.rust-lang.org/std/env/index.html