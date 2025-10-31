## Appendice D: Utili Strumenti di Sviluppo

In questa appendice parliamo di alcuni utili strumenti di sviluppo che il
progetto Rust mette a disposizione: la formattazione automatica, i modi rapidi
per applicare le correzioni degli avvisi, un _linter_ e l’integrazione con gli
_IDE_.

### Formattazione Automatica con `rustfmt`

Lo strumento `rustfmt` riformatta il tuo codice secondo lo stile di codice della
comunità. Molti progetti collaborativi utilizzano `rustfmt` per evitare
discussioni su quale stile utilizzare quando si scrive Rust: tutti formattano il
loro codice utilizzando lo strumento.

Le installazioni di Rust includono `rustfmt` come impostazione predefinita,
quindi dovresti già avere i programmi `rustfmt` e `cargo-fmt` sul tuo sistema.
Questi due comandi hanno lo stesso rapporto che esiste tre `rustc` e `cargo`,
nel senso che `rustfmt` è il _formattatore_ vero e proprio mentre `cargo-fmt`
usa e comprende le convenzioni di un progetto che utilizza Cargo per quanto
riguarda la formattazione di quel progetto. Per formattare un qualsiasi
progetto, inserisci quanto segue:

```console
$ cargo fmt
```

L’esecuzione di questo comando riformatta tutto il codice Rust nel _crate_
corrente. Questo dovrebbe cambiare solo lo stile del codice, non la sua
semantica. Per maggiori informazioni su `rustfmt`, consulta [la sua
documentazione][rustfmt].

### Correggere il Tuo Codice con `rustfix`

Lo strumento `rustfix` è incluso nelle installazioni di Rust ed è in grado di
correggere automaticamente gli avvertimenti del compilatore in cui è specificato
in modo preciso come quell’errore vada risolto. Probabilmente hai già visto
degli avvertimenti del compilatore. Per esempio, considera questo codice:

<span class="filename">File: src/main.rs</span>

```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```

In questo caso, stiamo definendo la variabile `x` come mutabile, ma in realtà
non la mutiamo mai. Rust ci avverte di questo:

```console
$ cargo build
   Compiling mioprogramma v0.1.0 (file:///progetti/mioprogramma)
warning: variable does not need to be mutable
 --> src/main.rs:2:9
  |
2 |     let mut x = 0;
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default
```

L’avviso suggerisce di rimuovere la parola chiave `mut`. Possiamo applicare
automaticamente questo suggerimento utilizzando lo strumento `rustfix` eseguendo
il comando `cargo fix`:

```console
$ cargo fix
    Checking mioprogramma v0.1.0 (file:///progetti/mioprogramma)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

Se guardiamo di nuovo _src/main.rs_, vedremo che `cargo fix` ha modificato il
codice:

<span class="filename">File: src/main.rs</span>

```rust
fn main() {
    let x = 42;
    println!("{x}");
}
```
La variabile `x` è ora immutabile e l’avviso non appare più.

Puoi anche usare il comando `cargo fix` per far passare il tuo codice tra
diverse edizioni di Rust. Le edizioni sono trattate nell’[Appendice
E][editions]<!-- ignore -->.

### Altri Strumenti di Analisi del Codice con Clippy

Lo strumento _Clippy_ è una raccolta di strumenti di analisi, _lint_ in inglese,
per analizzare il tuo codice in modo da individuare gli errori più comuni e
migliorare il tuo codice Rust. Clippy è incluso nelle installazioni standard di
Rust.

Per eseguire i _lint_ di Clippy su qualsiasi progetto Cargo, inserisci quanto
segue:

```console
$ cargo clippy
```

Ad esempio, supponiamo di scrivere un programma che utilizza un’approssimazione
di una costante matematica, come il pi greco, come fa questo programma:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("l'area del cerchio è {}", x * r * r);
}
```

</Listing>

Eseguendo `cargo clippy` su questo progetto si ottiene questo errore:

```text
error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: `#[deny(clippy::approx_constant)]` on by default
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
```

Questo errore ti informa che in Rust c’è già definita una costante `PI` più
precisa e che il tuo programma sarebbe più corretto se usassi questa costante.
Dovresti quindi modificare il tuo codice per usare la costante `PI`.

Il codice seguente non produce alcun errore o avviso da parte di Clippy:

<Listing file-name="src/main.rs">

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("l'area del cerchio è {}", x * r * r);
}
```

</Listing>

Per maggiori informazioni su Clippy, consulta [la sua documentazione][clippy].

### Integrazione nell’_IDE_ con `rust-analyzer`

Per aiutare l’integrazione con l’_IDE_, la comunità di Rust raccomanda l’uso di
[`rust-analyzer`][rust-analyzer]<!-- ignore -->. Questo strumento è un insieme
di utility incentrate sul compilatore che _parlano_ in [Language Server
Protocol][lsp]<!-- ignore -->, che è una specifica per gli _IDE_ e i linguaggi
di programmazione per comunicare tra loro. Diversi _client_ possono usare
`rust-analyzer`, come ad esempio [il plug-in Rust Analyzer per Visual Studio
Code][vscode].

Visita la [home page del progetto `rust-analyzer`][rust-analyzer]<!-- ignore -->
per le istruzioni di installazione, quindi installa il supporto per il server
linguistico nel tuo _IDE_ specifico. Il tuo _IDE_ otterrà funzionalità come
l’auto-completamento, il salto alla definizione e gli errori in linea.

[rustfmt]: https://rust-lang.github.io/rustfmt/
[editions]: appendix-05-editions.md
[clippy]: https://doc.rust-lang.org/clippy/
[rust-analyzer]: https://rust-analyzer.github.io
[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
