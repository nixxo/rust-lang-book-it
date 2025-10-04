## Personalizzare le Build con i Profili di Rilascio

In Rust, i _profili di rilascio_ (_release profiles_) sono profili predefiniti e
personalizzabili con diverse configurazioni che permettono al programmatore di
avere un maggiore controllo sulle varie opzioni di compilazione del codice. Ogni
profilo è configurato in modo indipendente dagli altri.

Cargo ha due profili principali: il profilo `dev` che Cargo utilizza quando
esegui `cargo build` e il profilo `release` che Cargo utilizza quando esegui
`cargo build --release`. Il profilo `dev` è definito con buoni valori
predefiniti per lo sviluppo, mentre il profilo `release` ha buoni valori
predefiniti per le build di rilascio.

I nomi di questi profili potrebbero esserti familiari dall’output che hai visto
finora quando compilavi il tuo codice:

<!-- manual-regeneration
anywhere, run:
cargo build
cargo build --release
and ensure output below is accurate
-->

```console
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.32s
```

I profili `dev` e `release` sono i diversi profili utilizzati dal compilatore.

Cargo ha delle impostazioni predefinite per ogni profilo che si applicano quando
non hai aggiunto esplicitamente alcuna sezione `[profile.*]` nel file
_Cargo.toml_ del progetto. Aggiungendo le sezioni `[profile.*]` per ogni profilo
che vuoi personalizzare, puoi sovrascrivere qualsiasi sottoinsieme delle
impostazioni predefinite. Ad esempio, ecco i valori predefiniti per
l’impostazione `opt-level` per i profili `dev` e `release`:

<span class="filename">File: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

L’impostazione `opt-level` controlla il numero di ottimizzazioni che Rust
applicherà al tuo codice, con un range che va da 0 a 3. L’applicazione di più
ottimizzazioni allunga i tempi di compilazione, quindi se sei in fase di
sviluppo e compili spesso il tuo codice, vorrai meno ottimizzazioni per
compilare più velocemente anche se il codice risultante gira più lentamente. Il
livello `opt-level` predefinito per `dev` è quindi `0`. Quando sei pronto a
rilasciare il tuo codice, è meglio dedicare più tempo alla compilazione.
Compilerai in modalità _release_ una sola volta, ma eseguirai il programma
compilato molte volte, quindi la modalità _release_ scambia un tempo di
compilazione più lungo con un codice che gira più velocemente. Ecco perché
`opt-level` predefinito per il profilo `release` è `3`.

Puoi sovrascrivere un’impostazione predefinita aggiungendo un valore diverso nel
file _Cargo.toml_. Ad esempio, se vogliamo utilizzare il livello di
ottimizzazione 1 nel profilo di sviluppo, possiamo aggiungere queste due righe
al file _Cargo.toml_ del nostro progetto:

<span class="filename">File: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 1
```

Questo codice sovrascrive l’impostazione predefinita di `0`. Ora, quando
lanciamo `cargo build`, Cargo utilizzerà le impostazioni predefinite per il
profilo `dev` più la nostra personalizzazione di `opt-level`. Poiché abbiamo
impostato `opt-level` a `1`, Cargo applicherà un maggior numero di
ottimizzazioni rispetto a quelle predefinite, ma non così tante come in una
build di rilascio.

Per l’elenco completo delle opzioni di configurazione e dei valori predefiniti
per ogni profilo, consulta [la documentazione di
Cargo](https://doc.rust-lang.org/cargo/reference/profiles.html).
