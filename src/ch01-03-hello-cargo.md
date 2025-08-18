## Hello, Cargo!

Cargo è il sistema di compilazione e il gestore di pacchetti di Rust. La maggior
parte dei Rustaceans utilizza questo strumento per gestire i propri progetti
Rust perché Cargo gestisce molte attività al posto tuo, come la compilazione del
codice, il download delle librerie da cui dipende il tuo codice e la
compilazione di tali librerie (chiamiamo le librerie di cui il tuo codice ha
bisogno _dipendenze_)

I programmi Rust più semplici, come quello che abbiamo scritto finora, non hanno
dipendenze. Se avessimo costruito il progetto "Hello, world!" con Cargo, questo
avrebbe utilizzato solo la parte di Cargo che si occupa della costruzione del
codice. Man mano che scriverai programmi Rust più complessi, aggiungerai delle
dipendenze e se inizierai un progetto utilizzando Cargo, sarà molto più facile
aggiungere dipendenze.

Poiché la stragrande maggioranza dei progetti Rust utilizza Cargo, il resto di
questo libro presuppone che anche tu stia utilizzando Cargo. Cargo viene
installato insieme a Rust se hai utilizzato l'installazione di cui si parla
nella sezione ["Installazione"][installazione]<!-- ignore -->. Se hai installato
Rust in altro modo, controlla se Cargo è installato inserendo quanto segue nel
tuo terminale:

```console
$ cargo --version
```

Se viene visualizzato un numero di versione, allora è fatta! Se viene
visualizzato un errore, come ad esempio `comando non trovato`, consulta la
documentazione relativa al tuo metodo di installazione per determinare come
installare Cargo separatamente.

### Creare un progetto con Cargo

Creiamo un nuovo progetto utilizzando Cargo e vediamo come si differenzia dal
nostro progetto originale "Hello, world!". Torna alla tua cartella _progetti_ (o
dove hai deciso di memorizzare il tuo codice). Poi, su qualsiasi sistema
operativo, esegui il seguente comando:

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

Il primo comando crea una nuova directory e un nuovo progetto chiamato
_hello_cargo_. Abbiamo chiamato il nostro progetto _hello_cargo_ e Cargo crea i
suoi file in una directory con lo stesso nome.

Vai nella directory _hello_cargo_ ed elenca i file. Vedrai che Cargo ha generato
due file e una directory per noi: un file _Cargo.toml_ e una directory _src_ con
un file _main.rs_ al suo interno.

Ha anche inizializzato un nuovo repository Git insieme a un file _.gitignore_. I
file Git non verranno generati se esegui `cargo new` all'interno di un
repository Git esistente; puoi annullare questo comportamento utilizzando `cargo
new --vcs=git`.

> Nota: Git è un diffuso software di controllo di versione distribuito. Puoi
> modificare `cargo new` per utilizzare un altro sistema di controllo di
> versione o nessun sistema di controllo di versioni utilizzando il flag
> `--vcs`. Esegui `cargo new --help` per vedere le opzioni disponibili.

Apri _Cargo.toml_ nell'editor di testo che preferisci. Dovrebbe assomigliare al
codice del Listato 1-2.

<Listing number="1-2" file-name="Cargo.toml" caption="Contenuto di *Cargo.toml* generato da `cargo new`">

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

</Listing>

Questo file è nel formato [_TOML_][toml]<!-- ignore --> (_Tom's Obvious, Minimal
Language_), che è il formato di configurazione di Cargo.

La prima riga, `[package]`, è un'intestazione di sezione che indica che le
dichiarazioni seguenti stanno configurando un pacchetto. Man mano che
aggiungeremo altre informazioni a questo file, aggiungeremo altre sezioni.

Le tre righe successive definiscono le informazioni di configurazione di cui
Cargo ha bisogno per compilare il tuo programma: il nome, la versione e
l'edizione di Rust da utilizzare. Parleremo della chiave `edition` in [Appendice
E][appendice-e]<!-- ignore -->.

L'ultima riga, `[dependencies]`, è l'inizio di una sezione in cui puoi elencare
tutte le dipendenze del tuo progetto. In Rust, i pacchetti di codice sono
chiamati _crates_ (ndt: inteso come cassetta, cestino...). Non avremo bisogno di
altri _crates_ per questo progetto, ma lo faremo nel primo progetto del Capitolo
2, quindi useremo questa sezione di dipendenze.

Ora apri _src/main.rs_ e dai un'occhiata:

<span class="filename">File: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo ha generato per te un programma "Hello, world!", proprio come quello che
abbiamo scritto nel Listato 1-1! Finora, le differenze tra il nostro progetto e
quello generato da Cargo sono che Cargo ha inserito il codice nella directory
_src_ e che c'è un file di configurazione _Cargo.toml_ nella directory
principale.

Cargo si aspetta che i tuoi file sorgente si trovino all'interno della directory
_src_. La directory principale del progetto è solo per i file README, le
informazioni sulla licenza, i file di configurazione e tutto ciò che non
riguarda il tuo codice. L'utilizzo di Cargo ti aiuta a organizzare i tuoi
progetti: c'è un posto per ogni cosa e ogni cosa è al suo posto.

If you started a project that doesn’t use Cargo, as we did with the “Hello,
world!” project, you can convert it to a project that does use Cargo. Move the
project code into the _src_ directory and create an appropriate _Cargo.toml_
file. One easy way to get that _Cargo.toml_ file is to run `cargo init`, which
will create it for you automatically. Se hai iniziato un progetto che non
utilizza Cargo, come abbiamo fatto con il progetto "Hello, world!", puoi
convertirlo in un progetto che utilizza Cargo. Sposta il codice del progetto
nella directory _src_ e crea un file _Cargo.toml_ appropriato. Un modo semplice
per ottenere il file _Cargo.toml_ è eseguire `cargo init`, che lo creerà
automaticamente.

### Costruire e eseguire un progetto Cargo

Ora vediamo cosa cambia quando costruiamo ed eseguiamo il programma "Hello,
world!" con Cargo! Dalla cartella _hello_cargo_, costruisci il tuo progetto
inserendo il seguente comando:

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

Questo comando crea un file eseguibile in _target/debug/hello_cargo_ (o
_target\debug\hello_cargo_.exe_ su Windows) anziché nella tua directory
corrente. Poiché la compilazione predefinita è una compilazione di debug, Cargo
mette il binario in una directory chiamata _debug_. Puoi eseguire l'eseguibile
con questo comando:

```console
$ ./target/debug/hello_cargo # o .\target\debug\hello_cargo.exe su Windows
Hello, world!
```

Se tutto è andato bene, `Hello, world!` dovrebbe essere stampato sul terminale.
L'esecuzione di `cargo build` per la prima volta fa sì che Cargo crei anche un
nuovo file nella directory principale: _Cargo.lock_. Questo file tiene traccia
delle versioni esatte delle dipendenze nel tuo progetto. Questo progetto non ha
dipendenze, quindi il file è un po' scarno. Non dovrai mai modificare questo
file manualmente; Cargo gestisce il suo contenuto per te.

Abbiamo appena costruito un progetto con `cargo build` e lo abbiamo eseguito con
`./target/debug/hello_cargo`, ma possiamo anche usare `cargo run` per compilare
il codice e poi eseguire l'eseguibile risultante con un solo comando:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Utilizzare `cargo run` è più comodo che doversi ricordare di eseguire `cargo
build` e poi utilizzare l'intero percorso del binario, quindi la maggior parte
degli sviluppatori utilizza `cargo run`.

Nota che questa volta non abbiamo visto l'output che indicava che Cargo stava
compilando `hello_cargo`. Cargo ha capito che i file non erano cambiati, quindi
non ha ricostruito ma ha semplicemente eseguito il binario. Se avessi modificato
il codice sorgente, Cargo avrebbe ricostruito il progetto prima di eseguirlo e
avresti visto questo output:

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

cargo offre anche un comando chiamato `cargo check`, che controlla rapidamente
il tuo codice per assicurarsi che venga compilato ma che non produce un
eseguibile:

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

E perché non dovresti volere un eseguibile? Spesso, `cargo check` è molto più
veloce di `cargo build` perché salta il passaggio della produzione di un
eseguibile. Se controlli continuamente il tuo lavoro mentre scrivi il codice,
l'uso di `cargo check` accelererà il processo di sapere se il tuo progetto è
privo di errori e compilabile! Per questo motivo, molti Rustaceans eseguono
`cargo check` periodicamente mentre scrivono il loro programma per assicurarsi
che si compili. Poi eseguono `cargo build` quando sono pronti a creare
l'eseguibile.

Ricapitoliamo quello che abbiamo imparato finora su Cargo:

- Possiamo creare un progetto utilizzando `cargo new`.
- Possiamo costruire un progetto utilizzando `cargo build`.
- Possiamo costruire ed eseguire un progetto in un unico passaggio utilizzando
  `cargo run`.
- Possiamo costruire un progetto senza produrre un binario per controllare gli
  errori utilizzando `cargo check`.
- Invece di salvare il risultato della compilazione nella stessa directory del
  nostro codice, Cargo lo salva nella directory _target/debug_.

Un ulteriore vantaggio dell'utilizzo di Cargo è che i comandi sono gli stessi
indipendentemente dal sistema operativo su cui stai lavorando. Quindi, a questo
punto, non forniremo più istruzioni specifiche per Linux e macOS rispetto a
Windows.

### Costruire per il Rilascio

Quando il tuo progetto è finalmente pronto per essere rilasciato, puoi usare
`cargo build --release` per compilarlo con le ottimizzazioni. Questo comando
creerà un eseguibile in _target/release_ invece che in _target/debug_. Le
ottimizzazioni rendono il tuo codice Rust più veloce, ma attivarle allunga i
tempi di compilazione del tuo programma. Per questo motivo esistono due profili
diversi: uno per lo sviluppo, quando vuoi ricostruire rapidamente e spesso, e un
altro per la creazione del programma finale che darai a un utente e che non sarà
ricostruito più volte e che funzionerà il più velocemente possibile. Se vuoi
fare un benchmark del tempo di esecuzione del tuo codice, assicurati di eseguire
`cargo build --release` e di fare il benchmark con l'eseguibile in
_target/release_.

### Cargo come convenzione

Con i progetti semplici, Cargo non offre molti vantaggi rispetto all'uso di
`rustc`, ma si dimostrerà utile quando i tuoi programmi diventeranno più
complessi. Quando i programmi diventano più file o hanno bisogno di una
dipendenza, è molto più facile lasciare che Cargo coordini la compilazione.

Anche se il progetto `hello_cargo` è semplice, ora utilizza gran parte degli
strumenti che userai nel resto della tua carriera in Rust. Infatti, per lavorare
su qualsiasi progetto esistente, puoi usare i seguenti comandi per verificare il
codice usando Git, passare alla directory del progetto e compilare:

```console
$ git clone example.org/un_progetto_nuovo
$ cd un_progetto_nuovo
$ cargo build
```

Per maggiori informazioni su Cargo, consulta la [documentazione][cargo].

## Riepilogo

Sei già partito alla grande nel tuo viaggio assieme a Rust! In questo capitolo
hai imparato a..:

- Installare l'ultima versione stabile di Rust usando `rustup`
- Aggiornare a una versione più recente di Rust
- Aprire la documentazione installata localmente
- Scrivere ed eseguire un programma "Hello, world!" usando direttamente `rustc`
- Creare ed eseguire un nuovo progetto usando le convenzioni di Cargo

Questo è un ottimo momento per costruire un programma più sostanzioso per
abituarsi a leggere e scrivere codice in Rust. Quindi, nel Capitolo 2,
costruiremo un programma di gioco di indovinelli. Se invece preferisci iniziare
imparando come funzionano in Rust alcuni concetti base della programmazione,
consulta il Capitolo 3 e poi ritorna al Capitolo 2.

[installazione]: ch01-01-installation.html#installazione
[toml]: https://toml.io
[appendice-e]: appendix-05-editions.html
[cargo]: https://doc.rust-lang.org/cargo/
