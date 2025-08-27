## Pacchetti e Crate

Le prime parti del sistema di moduli che tratteremo sono i pacchetti e i
_crate_.

Un _crate_ è la quantità minima di codice che il compilatore Rust considera in
un dato momento. Anche se esegui `rustc` invece di `cargo` e passi un singolo
file di codice sorgente (come abbiamo fatto all'inizio in “Scrivere ed eseguire
un programma Rust” nel Capitolo 1), il compilatore considera quel file come un
_crate_. I _crate_ possono contenere moduli, e i moduli possono essere definiti
in altri file che vengono compilati con il _crate_, come vedremo nelle sezioni
successive.

Un _crate_ può presentarsi in una delle due forme: un _crate_ binario o un
_crate libreria_. _I crate binari_ sono programmi che puoi compilare in un
eseguibile che puoi eseguire, come un programma da riga di comando o un server.
Ognuno deve avere una funzione chiamata `main` che definisce cosa succede quando
l'eseguibile viene eseguito. Tutti i _crate_ che abbiamo creato finora sono
stati _crate binari_.

I _crate libreria_ non hanno una funzione `main`, e non si compilano in un
eseguibile. Invece, definiscono funzionalità destinate a essere condivise con
progetti multipli. Ad esempio, il _crate_ `rand` che abbiamo usato nel [Capitolo
2][rand]<!-- ignore --> fornisce funzionalità che generano numeri casuali. La
maggior parte delle volte, quando i Rustaceani dicono “_crate_”, intendono
_crate libreria_, e usano “_crate_” in modo intercambiabile con il concetto
generale di programmazione di una “_libreria_”.

La _radice del crate_ (_crate root_) è un file sorgente da cui il compilatore
Rust inizia e costituisce il modulo radice del tuo _crate_ (spiegheremo i moduli
in dettaglio nel capitolo [“Definire Moduli per Controllare _Scope_ e
Privacy”][modules]<!-- ignore -->).

Un _pacchetto_ (_package_) è un insieme di uno o più _crate_ che fornisce un
insieme di funzionalità. Un pacchetto contiene un file _Cargo.toml_ che descrive
come costruire quei _crate_. Cargo è anch'esso in reltà un pacchetto che
contiene il _crate binario_ per lo strumento da riga di comando che hai usato
per costruire il tuo codice finora. Il pacchetto Cargo contiene anche un _crate
libreria_ di cui il _crate binario_ ha bisogno. Altri progetti possono dipendere
dal _crate libreria_ Cargo per utilizzare la stessa logica che utilizza lo
strumento da riga di comando Cargo.

Un pacchetto può contenere quanti più _crate binari_ desideri, ma al massimo
solo un _crate libreria_. Un pacchetto deve contenere almeno un _crate_, sia
esso un _crate libreria_ o binario.

Esploriamo cosa succede quando creiamo un pacchetto. Cominciamo con il comando
`cargo new mio-progetto`:

```console
$ cargo new mio-progetto
     Created binary (application) `mio-progetto` package
$ ls mio-progetto
Cargo.toml
src
$ ls mio-progetto/src
main.rs
```

Dopo aver eseguito `cargo new mio-progetto`, usiamo `ls` per vedere cosa crea
Cargo. Nella directory del progetto, c'è un file _Cargo.toml_, che definisce un
pacchetto. C'è anche una directory _src_ che contiene _main.rs_. Apri
_Cargo.toml_ nel tuo editor di testo e nota che non c'è menzione di
_src/main.rs_. Cargo segue una convenzione secondo cui _src/main.rs_ è la radice
del _crate_ di un _crate binario_ con lo stesso nome del pacchetto. Allo stesso
modo, Cargo sa che se la directory del pacchetto contiene _src/lib.rs_, il
pacchetto contiene un _crate libreria_ con lo stesso nome del pacchetto, e
_src/lib.rs_ è la sua radice del _crate_. Cargo passa i file di radice del
_crate_ a `rustc` per costruire la libreria o il binario.

Qui, abbiamo un pacchetto che contiene solo _src/main.rs_, il che significa che
contiene solo un _crate binario_ chiamato `mio-progetto`. Se un pacchetto
contiene _src/main.rs_ e _src/lib.rs_, avraà due _crate_: uno binario e uno
libreria, entrambi con lo stesso nome del pacchetto. Un pacchetto può avere più
_crate binari_ posizionando file nella directory _src/bin_: ogni file sarà un
_crate binario_ separato.

[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#generare-un-numero-casuale
