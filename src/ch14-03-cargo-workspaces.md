## Spazi di Lavoro Cargo

Nel Capitolo 12 abbiamo costruito un pacchetto che includeva un _crate_ binario
e un _crate_ libreria. Man mano che il tuo progetto si sviluppa, potresti
scoprire che il _crate_ libreria continua a diventare più grande e vorresti
dividere ulteriormente il tuo pacchetto in più _crate_ libreria. Cargo offre una
funzione chiamata _spazi di lavoro_ (_workspace_) che può aiutarti a gestire più
pacchetti correlati che vengono sviluppati in tandem.

### Creare un _Workspace_

Uno spazio di lavoro è un insieme di pacchetti che condividono lo stesso
_Cargo.lock_ e la stessa directory di output. Creiamo un progetto utilizzando un
_workspace_: useremo del codice banale in modo da poterci concentrare sulla
struttura del _workspace_. Esistono diversi modi per strutturare uno spazio di
lavoro, quindi ci limiteremo a mostrarne uno comune. Avremo un _workspace_
contenente un binario e due librerie. Il binario, che fornirà la funzionalità
principale, dipenderà dalle due librerie. Una libreria fornirà una funzione
`più_uno` e l'altra libreria una funzione `più_due`. Questi tre _crate_ faranno
parte dello stesso _workspace_. Inizia creando una nuova cartella per lo spazio
di lavoro:

```console
$ mkdir somma
$ cd somma
```

Successivamente, nella cartella _somma_, creiamo il file _Cargo.toml_ che
configurerà l'intero _workspace_. Questo file non avrà una sezione `[package]`,
ma inizierà con una sezione `[workspace]` che ci permetterà di aggiungere membri
al _workspace_. Inoltre, esplicitiamo di voler usare l'ultima versione
dell'algoritmo di risoluzione delle dipendenze di Cargo nel nostro spazio di
lavoro, impostando il valore `resolver` a `"3"`.

<span class="filename">File: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-01-workspace/somma/Cargo.toml}}
```

Successivamente, creeremo il _crate_ binario `sommatore` eseguendo `cargo new`
nella directory _somma_:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-01-adder-crate/somma
remove `members = ["sommatore"]` from Cargo.toml
rm -rf sommatore
cargo new sommatore
copy output below
-->

```console
$ cargo new sommatore
     Created binary (application) `sommatore` package
      Adding `sommatore` as member of workspace at `file:///progetti/somma`
```

L'esecuzione di `cargo new` all'interno di uno spazio di lavoro aggiunge
automaticamente il pacchetto appena creato alla chiave `members` nella
definizione `[workspace]` del file _Cargo.toml_, in questo modo:

```toml
{{#include ../listings/ch14-more-about-cargo/output-only-01-adder-crate/somma/Cargo.toml}}
```

A questo punto, possiamo costruire l'intero _workspace_ con `cargo build`. I
file nella tua cartella _somma_ dovrebbero avere questo aspetto:

```text
├── Cargo.lock
├── Cargo.toml
├── sommatore
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

Lo spazio di lavoro ha una cartella _target_ al livello superiore in cui
verranno inseriti gli artefatti compilati; il pacchetto `sommatore` non ha una
propria directory _target_. Anche se dovessimo eseguire `cargo build`
dall'interno della cartella _sommatore_, gli artefatti compilati finirebbero
comunque in _somma/target_ piuttosto che in _somma/sommatore/target_. Cargo
struttura la cartella _target_ in uno spazio di lavoro in questo modo perché i
_crate_ in un _workspace_ sono destinati a dipendere l'uno dall'altro. Se ogni
_crate_ avesse la propria cartella _target_, ogni _crate_ dovrebbe ricompilare
ogni altro _crate_ nello spazio di lavoro per posizionare gli artefatti nella
propria cartella _target_. Condividendo una cartella _target_, i _crate_ possono
evitare inutili ricostruzioni.

### Creare un Secondo Pacchetto nel _Workspace_

Ora creiamo un altro pacchetto membro dell'area di lavoro e chiamiamolo
`più_uno`. Generiamo un nuovo _crate_ libreria chiamato `più_uno`:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-02-add-one/somma
remove `"più_uno"` from `members` list in Cargo.toml
rm -rf più_uno
cargo new più_uno --lib
copy output below
-->

```console
$ cargo new più_uno --lib
     Created library `più_uno` package
      Adding `più_uno` as member of workspace at `file:///progetti/somma`
```

Il file _Cargo.toml_ nella cartella _somma_ ora includerà il percorso _più_uno_
nell'elenco dei membri `members`:

<span class="filename">File: Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/somma/Cargo.toml}}
```

La tua cartella _somma_ dovrebbe ora contenere queste cartelle e questi file:

```text
├── Cargo.lock
├── Cargo.toml
├── più_uno
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── sommatore
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

Nel file _più_uno/src/lib.rs_, aggiungiamo una funzione `più_uno`:

<span class="filename">File: più_uno/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/somma/più_uno/src/lib.rs}}
```

Ora possiamo fare in modo che il pacchetto `sommatore` con il nostro binario
dipenda dal pacchetto `più_uno` che contiene la nostra libreria. Per prima cosa,
dovremo aggiungere un percorso di dipendenza a `più_uno` nel file
_sommatore/Cargo.toml_.

<span class="filename">File: sommatore/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-02-workspace-with-two-crates/somma/sommatore/Cargo.toml:6:7}}
```

Cargo non presuppone che i _crate_ dello stesso _workspace_ dipendano l'uno
dall'altro, quindi dobbiamo essere espliciti sulle relazioni di dipendenza.

Quindi, utilizziamo la funzione `più_uno` (dal crate `più_uno`) nel crate
`sommatore`. Apri il file _sommatore/src/main.rs_ e modifica la funzione `main`
per richiamare la funzione `più_uno`, come nel Listato 14-7

<Listing number="14-7" file-name="sommatore/src/main.rs" caption="Utilizzo del _crate_ libreria `più_uno` dal _crate_ `sommatore`">

```rust,ignore
{{#rustdoc_include ../listings/ch14-more-about-cargo/listing-14-07/somma/sommatore/src/main.rs}}
```

</Listing>

Compiliamo lo spazio di lavoro eseguendo `cargo build` nella directory di primo
livello _somma_!

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/somma
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
   Compiling più_uno v0.1.0 (file:///progetti/somma/più_uno)
   Compiling sommatore v0.1.0 (file:///progetti/somma/sommatore)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
```

Per eseguire il _crate_ binario dalla directory _somma_, possiamo specificare
quale pacchetto del _workspace_ vogliamo eseguire utilizzando l'argomento `-p` e
il nome del pacchetto con `cargo run`:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/listing-14-07/somma
cargo run -p sommatore
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo run -p sommatore
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/sommatore`
Hello, world! 10 plus one is 11!
```

Questo esegue il codice in _sommatore/src/main.rs_, che dipende dal _crate_
`più_uno`.

### Dipendere da un Pacchetto Esterno

Avrai notato che lo spazio di lavoro ha un solo file _Cargo.lock_ al livello
superiore, invece di avere un _Cargo.lock_ nella cartella di ogni _crate_.
Questo assicura che tutti i _crate_ utilizzino la stessa versione di tutte le
dipendenze. Se aggiungiamo il pacchetto `rand` ai file _sommatore/Cargo.toml_ e
_più_uno/Cargo.toml_, Cargo li risolverà entrambi in un'unica versione di `rand`
e la registrerà nell'unico _Cargo.lock_. Fare in modo che tutti i _crate_ nel
_workspace_ utilizzino le stesse dipendenze significa che i _crate_ saranno
sempre compatibili tra loro. Aggiungiamo il _crate_ `rand` alla sezione
`[dependencies]` nel file _più_uno/Cargo.toml_ in modo da poter utilizzare il
_crate_ `rand` nel _crate_ `più_uno`:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
-->

<span class="filename">File: più_uno/Cargo.toml</span>

```toml
{{#include ../listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/somma/più_uno/Cargo.toml:6:7}}
```

Ora possiamo aggiungere `use rand;` al file _più_uno/src/lib.rs_ e la creazione
dell'intero _workspace_ eseguendo `cargo build` nella cartella _somma_
scaricherà e compilerà il _crate_ `rand`. Riceveremo un avviso perché non stiamo
effettivamente usando `rand` che abbiamo portato nello _scope_:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-03-workspace-with-external-dependency/somma
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
   --taglio--
   Compiling rand v0.8.5
   Compiling più_uno v0.1.0 (file:///progetti/somma/più_uno)
warning: unused import: `rand`
 --> più_uno/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `più_uno` (lib) generated 1 warning (run `cargo fix --lib -p più_uno` to apply 1 suggestion)
   Compiling sommatore v0.1.0 (file:///progetti/somma/sommatore)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.95s
```

Il file _Cargo.lock_ al livello più alto ora contiene informazioni sulla
dipendenza di `più_uno` da `rand`. Tuttavia, anche se `rand` è utilizzato da
qualche parte nello spazio di lavoro, non possiamo utilizzarlo in altri _crate_
del _workspace_ a meno che non aggiungiamo `rand` anche ai loro file
_Cargo.toml_. Ad esempio, se aggiungiamo `use rand;` al file
_sommatore/src/main.rs_ per il pacchetto `sommatore`, otterremo un errore:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/output-only-03-use-rand/somma
cargo build
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo build
  --taglio--
   Compiling sommatore v0.1.0 (file:///progetti/somma/sommatore)
error[E0432]: unresolved import `rand`
 --> sommatore/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

Per risolvere questo problema, modifica il file _Cargo.toml_ per il pacchetto
`sommatore` e indica che `rand` è una dipendenza anche per esso. Costruendo il
pacchetto `sommatore` aggiungerà `rand` all'elenco delle dipendenze di
`sommatore` in _Cargo.lock_, ma non verranno scaricate copie aggiuntive di
`rand`. Cargo farà in modo che ogni _crate_ in ogni pacchetto dell'area di
lavoro che utilizza il pacchetto `rand` utilizzi la stessa versione, a patto che
specifichi versioni compatibili di `rand`, risparmiando spazio e assicurando che
i _crate_ nel _workspace_ siano compatibili tra loro.

Se i _crate_ nel _workspace_ specificano versioni incompatibili della stessa
dipendenza, Cargo risolverà ciascuna di esse, ma cercherà comunque di risolvere
il minor numero possibile di versioni.

### Aggiungere un Test a un _Workspace_

Per un altro miglioramento, aggiungiamo un test della funzione
`più_uno::più_uno` all'interno del _crate_ `più_uno`:

<span class="filename">File: più_uno/src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/somma/più_uno/src/lib.rs}}
```

Ora esegui `cargo test` nella cartella di primo livello _somma_. Eseguendo
`cargo test` in un _workspace_ strutturato come questo, verranno eseguiti i test
per tutti i _crate_ presenti nello spazio di lavoro:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/somma
cargo test
copy output below; the output updating script doesn't handle subdirectories in
paths properly
-->

```console
$ cargo test
   Compiling più_uno v0.1.0 (file:///progetti/somma/più_uno)
   Compiling sommatore v0.1.0 (file:///progetti/somma/sommatore)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (target/debug/deps/più_uno-93c49ee75dc46543)

running 1 test
test tests::funziona ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/sommatore-3a47283c568d2b6a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests più_uno

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

La prima sezione dell'output mostra che il test `funziona` nel _crate_ `più_uno`
è passato. La sezione successiva mostra che sono stati trovati zero test nel
_crate_ `sommatore` e l'ultima sezione mostra che sono stati trovati zero test
di documentazione nel _crate_ `più_uno`.

Possiamo anche eseguire i test per un particolare _crate_ in un _workspace_
dalla directory di primo livello utilizzando il flag `-p` e specificando il nome
del _crate_ che vogliamo testare:

<!-- manual-regeneration
cd listings/ch14-more-about-cargo/no-listing-04-workspace-with-tests/somma
cargo test -p più_uno
copy output below; the output updating script doesn't handle subdirectories in paths properly
-->

```console
$ cargo test -p più_uno
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/più_uno-93c49ee75dc46543)

running 1 test
test tests::funziona ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests più_uno

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Questo output mostra che `cargo test` ha eseguito solo i test del _crate_
`più_uno` e non ha eseguito i test del _crate_ `sommatore`.

Se pubblichi i _crate_ nello _workspace_ su [crates.io](https://crates.io/)<!--
ignore -->, ogni _crate_ nello spazio di lavoro dovrà essere pubblicato
separatamente. Come nel caso di `cargo test`, possiamo pubblicare un particolare
_crate_ nel nostro spazio di lavoro utilizzando il flag `-p` e specificando il
nome del _crate_ che vogliamo pubblicare.

Per fare ulteriore pratica, aggiungi un _crate_ `più_due` a questo spazio di
lavoro in modo simile al crate `più_uno`!

Quando il tuo progetto cresce, prendi in considerazione l'utilizzo di uno spazio
di lavoro: ti permette di lavorare con componenti più piccoli e più facili da
capire rispetto a un unico grande blocco di codice. Inoltre, mantenere i _crate_
in uno spazio di lavoro può rendere più facile il coordinamento tra i _crate_ se
questi vengono spesso modificati nello stesso momento.
