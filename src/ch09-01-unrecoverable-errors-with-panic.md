## Errori Irreversibili con `panic!`

A volte si verificano problemi nel codice e non c'è nulla che si possa fare al
riguardo. In questi casi, Rust dispone della macro `panic!`. Esistono
praticamente due modi per causare un _panic_: eseguendo un'azione che causa il
_panic_ del codice (come tentare di accedere a un array oltre la fine) o
chiamando esplicitamente la macro `panic!`. In entrambi i casi, causiamo un
_panic_ nel nostro programma. Per impostazione predefinita, questi _panic_
stampano un messaggio di errore, eseguono un _unwind_, puliscono lo _stack_ e
terminano. Tramite una variabile d'ambiente, è anche possibile fare in modo che
Rust visualizzi lo _stack_ delle chiamate quando si verifica un _panic_, per
facilitare l'individuazione della causa del _panic_.

> ### _Unwinding_ dello _Stack_ o Interruzione in Risposta a un _Panic_
>
> Per impostazione predefinita, quando si verifica un _panic_ il programma avvia
> l'_unwinding_, il che significa che Rust risale lo _stack_ e pulisce i dati da
> ogni funzione che incontra. Tuttavia, risalire lo stack e pulire richiede
> molto lavoro. Rust, quindi, consente di scegliere l'alternativa di _abortire_
> immediatamente, che termina il programma senza pulizia. La memoria che il
> programma stava utilizzando dovrà quindi essere ripulita dal sistema
> operativo. Se nel progetto è necessario ridurre al minimo il binario
> risultante, è possibile passare dall'_unwinding_ all'interruzione in caso di
> _panic_ aggiungendo `panic = 'abort'` alle sezioni `[profile]` appropriate nel
> file _Cargo.toml_. Ad esempio, se si desidera interrompere l'esecuzione in
> caso di _panic_ nell'eseguibile finale (_release_) ma non in fase di sviluppo,
> aggiungere quanto segue:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

Proviamo a chiamare `panic!` in un semplice programma:

<Listing file-name="src/main.rs">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

</Listing>

Quando si esegue il programma, si vedrà qualcosa di simile a questo:

```console
{{#include ../listings/ch09-error-handling/no-listing-01-panic/output.txt}}
```

La chiamata a `panic!` causa il messaggio di errore contenuto nelle ultime due
righe. La prima riga mostra il nostro messaggio di errore e il punto del codice
sorgente in cui si è verificato l'errore: _src/main.rs:2:5_ indica che si tratta
della seconda riga, quinto carattere del nostro file _src/main.rs_.

In questo caso, la riga indicata fa parte del nostro codice e, se andiamo a
quella riga, vediamo la chiamata alla macro `panic!`. In altri casi, la chiamata
a `panic!` potrebbe essere nel codice richiamato dal nostro codice e il nome del
file e il numero di riga riportati dal messaggio di errore saranno il codice di
qualcun altro in cui viene richiamata la macro `panic!`, non la riga del nostro
codice che alla fine ha portato alla chiamata a `panic!`.

<!-- Old heading. Do not remove or links may break. -->
<a id="using-a-panic-backtrace"></a>

Possiamo usare il _backtrace_ (_andare a ritroso_) delle funzioni da cui
proviene la chiamata `panic!` per capire la parte del nostro codice che sta
causando il problema. Per capire come usare un _backtrace_ di `panic!`, diamo
un'occhiata a un altro esempio e vediamo cosa succede quando una chiamata
`panic!` proviene da una libreria a causa di un bug nel nostro codice invece che
dal nostro codice che chiama direttamente la macro. Il Listato 9-1 contiene del
codice che tenta di accedere a un indice in un _vector_ oltre l'intervallo di
indici validi.

<Listing number="9-1" file-name="src/main.rs" caption="Tentativo di accedere a un elemento oltre la fine di un _vector_, che causerà una chiamata a `panic!`">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

</Listing>

Qui stiamo tentando di accedere al centesimo elemento del nostro vettore (che si
trova all'indice 99 perché l'indicizzazione inizia da zero), ma il vettore ha
solo tre elementi. In questa situazione, Rust andrà in _panic_. L'utilizzo di
`[]` dovrebbe restituire un elemento, ma se si passa un indice non valido, non
c'è alcun elemento valido che Rust potrebbe restituire.

In C, tentare di leggere oltre la fine di una struttura dati è un comportamento
indefinito. Si potrebbe ottenere qualsiasi cosa si trovi nella posizione in
memoria che corrisponderebbe a quell'elemento nella struttura dati, anche se la
memoria non appartiene a quella struttura. Questo è chiamato _buffer overread_
(_lettura oltre il buffer_) e può portare a vulnerabilità di sicurezza se un
aggressore riesce a manipolare l'indice in modo tale da leggere dati che non
dovrebbe essere autorizzato a leggere e che sono memorizzati dopo la struttura
dati.

Per proteggere il programma da questo tipo di vulnerabilità, se si tenta di
leggere un elemento in un indice che non esiste, Rust interromperà l'esecuzione
e si rifiuterà di continuare. Proviamo e vediamo:

```console
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

Questo errore punta alla riga 4 del nostro _main.rs_, dove tentiamo di accedere
all'indice `99` del vettore in `v`.

La riga `note:` ci dice che possiamo impostare la variabile d'ambiente
`RUST_BACKTRACE` per ottenere un _backtrace_ di ciò che ha causato l'errore. Un
_backtrace_ è un elenco di tutte le funzioni che sono state chiamate per
arrivare a questo punto. I backtrace in Rust funzionano come in altri linguaggi:
la chiave per leggere il _backtrace_ è iniziare dall'inizio e leggere fino a
quando non si vedono i file che si sono creati. Quello è il punto in cui si è
originato il problema. Le righe sopra quel punto sono il codice che il tuo
codice ha chiamato; le righe sottostanti sono il codice che ha chiamato il tuo
codice. Queste righe prima e dopo potrebbero includere codice Rust core, codice
di libreria standard o pacchetti che stai utilizzando. Proviamo a ottenere un
_backtrace_ impostando la variabile d'ambiente `RUST_BACKTRACE` su un valore
qualsiasi tranne `0`. Il Listato 9-2 mostra un output simile a quello che
vedrai.

<!-- manual-regeneration
cd listings/ch09-error-handling/listing-09-01
RUST_BACKTRACE=1 cargo run > output_backtrace.txt 2>&1
-->

<Listing number="9-2" caption="Il _backtrace_ generato da una chiamata a `panic!` viene visualizzato quando la variabile d'ambiente `RUST_BACKTRACE` è impostata">

```console
{{#include ../listings/ch09-error-handling/listing-09-01/output_backtrace.txt}}
```

</Listing>

Un output enorme! L'output esatto che vedi potrebbe variare a seconda del
sistema operativo e della versione di Rust. Per ottenere _backtrace_ con queste
informazioni, i simboli di debug devono essere abilitati. I simboli di debug
sono abilitati per impostazione predefinita quando si utilizza `cargo build` o
`cargo run` senza il flag `--release`, come in questo caso.

Nell'output del Listato 9-2, la riga 6 del backtrace punta alla riga del nostro
progetto che causa il problema: la riga 4 di _src/main.rs_. Se non vogliamo che
il nostro programma vada in _panic_, dovremmo iniziare la nostra analisi dalla
posizione indicata dalla prima riga che menziona un file che abbiamo scritto.
Nel Listato 9-1, dove abbiamo volutamente scritto codice che andrebbe in
_panic_, il modo per risolvere il problema è non richiedere un elemento oltre
l'intervallo degli indici del vettore. Quando in futuro il codice andrà in
_panic_, dovrai capire quale azione sta eseguendo il codice con quali valori
tali da causare il _panic_ e cosa dovrebbe fare il codice al suo posto.

Torneremo su `panic!` e su quando dovremmo e non dovremmo usare `panic!` per
gestire le condizioni di errore nella sezione [“`panic!` o non
`panic!`”][to-panic-or-not-to-panic]<!-- ignore --> più avanti in questo
capitolo. Successivamente, vedremo come risolvere un errore utilizzando
`Result`.

[to-panic-or-not-to-panic]: ch09-03-to-panic-or-not-to-panic.html#panic-o-non-panic
