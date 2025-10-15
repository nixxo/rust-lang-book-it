## Appendice A: Parole chiave

I seguenti elenchi contengono parole chiave che sono riservate per l’uso attuale
o futuro del linguaggio Rust. In quanto tali, non possono essere utilizzate come
identificatori (tranne che come _identificatori grezzi_, come discuteremo nella
sezione [dedicata](#identificatori-grezzi)<!-- ignore -->). Gli *identificatori*
sono nomi di funzioni, variabili, parametri, elementi di struct, moduli,
_crate_, costanti, macro, valori statici, attributi, _type_, _trait_ o
_lifetime_.

### Parole Chiave Attualmente in Uso

Di seguito è riportato un elenco di parole chiave attualmente in uso, con la
loro funzionalità descritta.

- **`as`** - Eseguire un casting primitivo, disambiguare il _trait_ specifico.
  che contiene un elemento o rinominare elementi nelle dichiarazioni `use`.
- **`async`** - Restituire un `Future` invece di bloccare il _thread_ corrente.
- **`await`** - Sospendere l’esecuzione fino a quando il risultato di un
  `Future` è pronto.
- **`break`** - Uscire immediatamente da un ciclo.
- **`const`** - Definire elementi costanti o puntatori raw costanti.
- **`continue`** - Continuare all’iterazione successiva del ciclo.
- **`crate`** - In un percorso di modulo, si riferisce alla radice del _crate_.
- **`dyn`** - _Dispatch_ dinamico a un oggetto _trait_.
- **`else`** - Alternativa per i costrutti di controllo di flusso `if` e `if
  let`.
- **`enum`** - Definire un’enumerazione.
- **`extern`** - Collegare una funzione o una variabile esterna.
- **`false`** - Letterale booleano falso.
- **`fn`** - Definire una funzione o il tipo di puntatore a funzione.
- **`for`** - Iterare su elementi da un iteratore, implementare un _trait_ o
  specificare una _lifetime_ di rango superiore.
- **`if`** - Ramificazione in base al risultato di un’espressione condizionale.
- **`impl`** - Implementare funzionalità innate o di _trait_.
- **`in`** - Parte della sintassi del ciclo `for`.
- **`let`** - Inizializzare una variabile.
- **`loop`** - Ciclo senza condizioni.
- **`match`** - Abbinare un valore a _pattern_.
- **`mod`** - Definire un modulo.
- **`move`** - Fare in modo che una _closure_ prenda possesso di tutte le sue
  catture.
- **`mut`** - Denotare mutabilità in _reference_, puntatori raw o binding di
  _pattern_.
- **`pub`** - Denotare visibilità pubblica nei campi delle strutture, nei
  blocchi `impl` o nei moduli.
- **`ref`** - Inizializzare per _reference_.
- **`return`** - Ritorno dalla funzione.
- **`Self`** - Un alias di _type_ per il _type_ che stiamo definendo o
  implementando.
- **`self`** - Soggetto del metodo o modulo corrente.
- **`static`** - Variabile globale o _lifetime_ che dura per l’intera esecuzione
  del programma.
- **`struct`** - Definire una struttura.
- **`super`** - Modulo genitore del modulo corrente.
- **`trait`** - Definire un _trait_.
- **`true`** - Letterale booleano vero.
- **`type`** - Definire un alias di _type_ o un _type_ associato.
- **`union`** - Definire un’unione; è una parola chiave solo quando usata in una
  dichiarazione di unione.
- **`unsafe`** - Annotare codice, funzioni, _trait_ o implementazioni come non
  sicure.
- **`use`** - Portare simboli in _scope_; specificare catture precise per
  vincoli generici e di _lifetime_.
- **`where`** - Denotare clausole che vincolano un _type_.
- **`while`** - Ciclo condizionato al risultato di un’espressione.

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

L’errore indica che non è possibile utilizzare la parola chiave `match` come
identificatore di funzione. Per utilizzare `match` come nome di funzione, devi
utilizzare la sintassi dell’_identificatore grezzo_, in questo modo:

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
permettono di utilizzare librerie scritte in un’edizione di Rust diversa da
quella utilizzata dal tuo crate. Per esempio, `try` non è una parola chiave
nell’edizione 2015, ma lo è nelle edizioni 2018, 2021 e 2024. Se dipendi da una
libreria scritta con l’edizione 2015 e che ha una funzione `try`, dovrai
utilizzare la sintassi dell’identificatore grezzo, `r#try` in questo caso, per
richiamare quella funzione dal tuo codice nelle edizioni successive. Per
ulteriori informazioni sulle edizioni, consulta [Appendice E][appendix-e]<!--
ignore -->.

[appendix-e]: appendix-05-editions.html
