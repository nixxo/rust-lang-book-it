## Appendice A: Parole chiave

Il seguente elenco contiene parole chiave che sono riservate per l'uso attuale o
futuro del linguaggio Rust. In quanto tali, non possono essere utilizzate come
identificatori (tranne che come _identificatori grezzi_, come discuteremo nella
sezione [dedicata](#identificatori-grezzi)<!-- ignore -->). Gli identificatori
sono nomi di funzioni, variabili, parametri, elementi di struct, moduli,
_crate_, costanti, macro, valori statici, attributi, _type_, _trait_ o
_lifetime_.

### Parole Chiave Attualmente in Uso

Di seguito è riportato un elenco di parole chiave attualmente in uso, con la
loro funzionalità descritta.

- `as` - eseguire un casting primitivo, disambiguare il _trait_ specifico che
  contiene un elemento o rinominare elementi nelle dichiarazioni `use`
- `async` - restituire un `Future` invece di bloccare il _thread_ corrente
- `await` - sospendere l'esecuzione fino a quando il risultato di un `Future` è
  pronto
- `break` - uscire immediatamente da un ciclo
- `const` - definire elementi costanti o puntatori raw costanti
- `continue` - continuare all'iterazione successiva del ciclo
- `crate` - in un percorso di modulo, si riferisce alla radice del crate
- `dyn` - dispatch dinamico a un oggetto _trait_
- `else` - alternativa per i costrutti di controllo di flusso `if` e `if let`
- `enum` - definire un'enumerazione
- `extern` - collegare una funzione o una variabile esterna
- `false` - letterale booleano falso
- `fn` - definire una funzione o il tipo di puntatore a funzione
- `for` - iterare su elementi da un iteratore, implementare un _trait_ o
  specificare una _lifetime_ di rango superiore
- `if` - ramificazione in base al risultato di un'espressione condizionale
- `impl` - implementare funzionalità innate o di _trait_
- `in` - parte della sintassi del ciclo `for`
- `let` - inizializzare una variabile
- `loop` - ciclo senza condizioni
- `match` - abbinare un valore a _pattern_
- `mod` - definire un modulo
- `move` - fare in modo che una _closure_ prenda possesso di tutte le sue
  catture
- `mut` - denotare mutabilità in _reference_, puntatori raw o binding di
  _pattern_
- `pub` - denotare visibilità pubblica nei campi delle strutture, nei blocchi
  `impl` o nei moduli
- `ref` - inizializzare per _reference_
- `return` - ritorno dalla funzione
- `Self` - un alias di _type_ per il _type_ che stiamo definendo o implementando
- `self` - soggetto del metodo o modulo corrente
- `static` - variabile globale o _lifetime_ che dura per l'intera esecuzione del
  programma
- `struct` - definire una struttura
- `super` - modulo genitore del modulo corrente
- `trait` - definire un _trait_
- `true` - letterale booleano vero
- `type` - definire un alias di _type_ o un _type_ associato
- `union` - definire un'[unione][union]<!-- ignore -->; è solo una parola chiave
  quando utilizzata in una dichiarazione di unione
- `unsafe` - denotare codice, funzioni, _trait_ o implementazioni non sicure
- `use` - portare simboli in _scope_; specificare catture precise per vincoli
  generici e di _lifetime_
- `where` - denotare clausole che vincolano un _type_
- `while` - ciclo condizionato al risultato di un'espressione

[union]: https://doc.rust-lang.org/stable/reference/items/unions.html

### Parole Chiave Riservate per Usi Futuri

Le seguenti parole chiave non hanno ancora alcuna funzionalità ma sono riservate
da Rust per un potenziale uso futuro.

- `abstract`
- `become`
- `box`
- `do`
- `final`
- `gen`
- `macro`
- `override`
- `priv`
- `try`
- `typeof`
- `unsized`
- `virtual`
- `yield`

### Identificatori Grezzi

Gli _Identificatori grezzi_ (_raw identifiers_) sono la sintassi che ti permette
di utilizzare parole chiave dove normalmente non sarebbero consentite. Utilizzi
un identificatore grezzo anteponendo a una parola chiave il prefisso `r#`. Ad
esempio, `match` è una parola chiave. Se provi a compilare la seguente funzione
che utilizza `match` come nome:

<span class="filename">File: src/main.rs</span>

```rust,ignore,does_not_compile
fn match(ago: &str, pagliaio: &str) -> bool {
    pagliaio.contains(ago)
}
```

otterrai questo errore:

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(ago: &str, pagliaio: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

L'errore indica che non è possibile utilizzare la parola chiave `match` come
identificatore di funzione. Per utilizzare `match` come nome di funzione, devi
utilizzare la sintassi dell'_identificatore grezzo_, in questo modo:

<span class="filename">File: src/main.rs</span>

```rust
fn r#match(ago: &str, pagliaio: &str) -> bool {
    pagliaio.contains(ago)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

Questo codice verrà compilato senza errori. Nota il prefisso `r#` sul nome della
funzione nella sua definizione e il punto in cui la funzione viene chiamata in
`main`.

Gli _identificatori grezzi_ ti permettono di utilizzare qualsiasi parola che
scegli come identificatore, anche se si tratta di una parola chiave riservata.
Questo ci dà maggiore libertà nella scelta dei nomi degli identificatori e ci
permette di integrarci con programmi scritti in un linguaggio in cui queste
parole non sono parole chiave. Inoltre, gli _identificatori grezzi_ ti
permettono di utilizzare librerie scritte in un'edizione di Rust diversa da
quella utilizzata dal tuo crate. Per esempio, `try` non è una parola chiave
nell'edizione 2015, ma lo è nelle edizioni 2018, 2021 e 2024. Se dipendi da una
libreria scritta con l'edizione 2015 e che ha una funzione `try`, dovrai
utilizzare la sintassi dell'identificatore grezzo, `r#try` in questo caso, per
richiamare quella funzione dal tuo codice nelle edizioni successive. Per
ulteriori informazioni sulle edizioni, consulta [Appendice E][appendix-e]<!--
ignore -->.

[appendix-e]: appendix-05-editions.html
