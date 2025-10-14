## Appendice B: Operatori e Simboli

Questa appendice contiene un glossario della sintassi di Rust, compresi gli
operatori e altri simboli che appaiono da soli o nel contesto di percorsi,
_generics_, _traits_, _macro_, attributi, commenti, tuple e parentesi.

### Operatori

La Tabella B-1 contiene gli operatori in Rust, un esempio di come l’operatore
appare nel contesto, una breve spiegazione e se l’operatore in questione è
sovrascrivibile. Se un operatore è sovrascrivibile, viene elencato il relativo
_trait_ da utilizzare per sovrascriverlo.

<span class="caption">Tabella B-1: Operatori</span>

| Operatore | Esempio | Spiegazione | Sovrascrivibile |
| ----- | --- | --- | --- |
| `!`   | `ident!(...)`, `ident!{...}`, `ident![...]` | Espansione delle macro | |
| `!`   | `!expr` | Complemento logico o bit per bit | `Not` |
| `!=`  | `expr != expr` | Differente | `PartialEq` |
| `%`   | `expr % expr` | Resto aritmetico | `Rem` |
| `%=`  | `var %= expr` | Resto aritmetico con assegnazione | `RemAssign` |
| `&`   | `&expr`, `&mut expr` | Prestito (_Borrow_) | |
| `&`   | `&type`, `&mut type`, `&'a type`, `&'a mut type` | _Type_ Puntatore a Prestito | |
| `&`   | `expr & expr` | AND Bit per Bit | `BitAnd` |
| `&=`  | `var &= expr` | AND Bit per Bit con assegnazione | `BitAndAssign` |
| `&&`  | `expr && expr` | AND logico | |
| `*`   | `expr * expr` | Moltiplicazione aritmetica | `Mul` |
| `*=`  | `var *= expr` | Moltiplicazione aritmetica con assegnazione | `MulAssign` |
| `*`   | `*expr` | De-referenziazione | `Deref` |
| `*`   | `*const type`, `*mut type` | Puntatore grezzo (_Raw pointer_) | |
| `+`   | `trait + trait`, `'a + trait` | Vincolo per _type_ composto | |
| `+`   | `expr + expr` | Addizione aritmetica | `Add` |
| `+=`  | `var += expr` | Addizione aritmetica con assegnazione | `AddAssign` |
| `,`   | `expr, expr` | Separatore di argomenti ed elementi | |
| `-`   | `- expr` | Negazione aritmetica | `Neg` |
| `-`   | `expr - expr` | Sottrazione aritmetica | `Sub` |
| `-=`  | `var -= expr` | Sottrazione aritmetica con assegnazione | `SubAssign` |
| `->`  | `fn(...) -> type`, <code>&vert;...&vert; -> type</code> | _Type_ di ritorno per funzioni e chiusure | |
| `.`   | `expr.ident` | Accesso a campo | |
| `.`   | `expr.ident(expr, ...)` | Chiamata a metodo | |
| `.`   | `expr.0`, `expr.1`, etc. | Indicizzazione tupla | |
| `..`  | `..`, `expr..`, `..expr`, `expr..expr` | Range esclusivo | `PartialOrd` |
| `..=` | `..=expr`, `expr..=expr` | Range inclusivo | `PartialOrd` |
| `..`  | `..expr` | Aggiornamento _struct_ | |
| `..`  | `variant(x, ..)`, `struct_type { x, .. }` | “E il resto” con assegnazione tramite _pattern_ | |
| `...` | `expr...expr` | (Non più utilizzabile, usa `..=`) In un _pattern_: range inclusivo | |
| `/`   | `expr / expr` | Divisione aritmetica | `Div` |
| `/=`  | `var /= expr` | Divisione aritmetica con assegnazione | `DivAssign` |
| `:`   | `pat: type`, `ident: type` | Vincoli | |
| `:`   | `ident: expr` | Inizializzazione campo di _struct_ | |
| `:`   | `'a: loop {...}` | Etichetta _loop_ | |
| `;`   | `expr;` | Terminatore di dichiarazioni ed elementi | |
| `;`   | `[...; len]` | Parte della sintassi per vettori a grandezza fissa | |
| `<<`  | `expr << expr` | _Shift_ a sinistra | `Shl` |
| `<<=` | `var <<= expr` | _Shift_ a sinistra con assegnazione | `ShlAssign` |
| `<`   | `expr < expr` | Minore | `PartialOrd` |
| `<=`  | `expr <= expr` | Minore o uguale | `PartialOrd` |
| `=`   | `var = expr`, `ident = type` | Assegnazione/equivalenza | |
| `==`  | `expr == expr` | Comparazione di uguaglianza | `PartialEq` |
| `=>`  | `pat => expr` | Parte della sintassi del ramo di _match_ | |
| `>`   | `expr > expr` | Maggiore | `PartialOrd` |
| `>=`  | `expr >= expr` | Maggiore o uguale | `PartialOrd` |
| `>>`  | `expr >> expr` | _Shift_ a destra | `Shr` |
| `>>=` | `var >>= expr` | _Shift_ a destra con assegnazione | `ShrAssign` |
| `@`   | `ident @ pat` | Vincolo di _Pattern_ | |
| `^`   | `expr ^ expr` | OR esclusivo Bit per Bit | `BitXor` |
| `^=`  | `var ^= expr` | OR esclusivo Bit per Bit con assegnazione | `BitXorAssign` |
| <code>&vert;</code> | <code>pat &vert; pat</code> | _Pattern_ alternativi | |
| <code>&vert;</code> | <code>expr &vert; expr</code> | OR Bit per Bit | `BitOr` |
| <code>&vert;=</code> | <code>var &vert;= expr</code> | OR Bit per Bit con assegnazione | `BitOrAssign` |
| <code>&vert;&vert;</code> | <code>expr &vert;&vert; expr</code> | OR logico | |
| `?`   | `expr?` | Propagazione errore | |

### Simboli

L’elenco seguente contiene tutti i simboli che non funzionano come operatori,
cioè non si comportano come una funzione o una chiamata di metodo.

La Tabella B-2 mostra i simboli che appaiono da soli e sono validi in diverse
posizioni.

<span class="caption">Tabella B-2: Sintassi stand alone</span>

| Simbolo  | Spiegazione |
| -------- | --- |
| `'ident` | _Lifetime_ nominale o _etichetta loop_ |
| Numeri immediatamente seguiti da `u8`, `i32`, `f64`, `usize`, ecc. | Letterale numerico di un _type_ specifico |
| `"..."`  | Letterale stringa |
| `r"..."`, `r#"..."#`, `r##"..."##`, etc. | Letterale stringa grezzo, senza elaborazione dei caratteri di _escape_ |
| `b"..."` | Letterale byte di stringa; costituisce un vettore di byte anziché una stringa |
| `br"..."`, `br#"..."#`, `br##"..."##`, etc. | Letterale byte di stringa grezzo |
| `'...'`  | Letterale carattere |
| `b'...'` | Letterale byte ASCII |
| <code>&vert;...&vert; expr</code> | Chiusure |
| `!`      | _Type_ vuoto per funzioni divergenti |
| `_`      | Pattern “Ignorato” nel ramo di `match`; usato anche per rendere i letterali più leggibili |

----

La Tabella B-3 mostra i simboli che vengono usati nel contesto dei percorsi di
un elemento nella gerarchia dei moduli.

<span class="caption">Tabella B-3: Sintassi relativa ai Percorsi</span>

| Simbolo              | Spiegazione |
| -------------------- | --- |
| `ident::ident`       | Nomenclatura percorso |
| `::path`             | Percorso relativo al preludio esterno, dove sono tutti gli altri _crate_ (es., un percorso assoluto esplicito che include il nome del _crate_) |
| `self::path`         | Percorso relativo al modulo corrente (es., un percorso relativo esplicito). |
| `super::path`        | Percorso relativo al genitore del modulo corrente |
| `type::ident`, `<type as trait>::ident` | Costanti, funzioni o _type_ associati |
| `<type>::...`        | Elemento associato per un _type_ generico (es, `<&T>::...`, `<[T]>::...`, etc.) |
| `trait::method(...)` | Disambiguare una chiamata a un metodi specificando il _trait_ che lo definisce |
| `type::method(...)`  | Disambiguare una chiamata a un metodo specificando il _type_ in cui per cui è definito |
| `<type as trait>::method(...)` | Disambiguare una chiamata a un metodo specificando il _trait_ e il _type_ |

----

La Tabella B-4 mostra i simboli che appaiono quando si usano _type_ generici
come parametri.

<span class="caption">Table B-4: Generici</span>

| Simbolo                 | Spiegazione |
| ----------------------- | --- |
| `percorso<...>`         | Specifica parametri a _type_ generici in u n _type_ (es., `Vec<u8>`) |
| `percorso::<...>`, `metodo::<...>` | Specifica parametri a _type_, funzioni, metodi generici in un’espressione; spesso chiamato operatore _turbofish_ (e.g., `"42".parse::<i32>()`) |
| `fn ident<...> ...`     | Definizione di funzione generica |
| `struct ident<...> ...` | Definizione di _struct_ generica |
| `enum ident<...> ...`   | Definizione di _enum_ generica |
| `impl<...> ...`         | Definizione di implementazione generica |
| `for<...> type`         | Vincolo di _lifetime_ prioritario |
| `type<ident=type>`      | Un _type_ generico dove uno o più _type_ associati hanno assegnazioni specifiche (es., `Iteratore<Elemento=T>`) |

----

La Tabella B-5 mostra i simboli che appaiono nel contesto della dichiarazioni di
_type_ generici come parametri e dei corrispettivi vincoli di _trait_.

<span class="caption">Table B-5: Vincoli di _Trait_</span>

| Simbolo      | Spiegazione |
| ------------ | --- |
| `T: U`       | Parametro generico `T` vincolato a _type_ che implementano `U` |
| `T: 'a`      | _Type_ generico type `T` con longevità `'a` (implica che non possa contenere _reference_ con _lifetime_ inferiore ad `'a`) |
| `T: 'static` | _Type_ generico `T` contenente solo _reference_ con longevità infinita |
| `'b: 'a`     | _Lifetime_ generica `'b` deve essere maggiore di _lifetime_ `'a` |
| `T: ?Sized`  | Consente a parametri con _type_ generico di essere _type_ a dimensione dinamica |
| `'a + trait`, `trait + trait` | Definizione di vincolo multiplo |

----

La Tabella B-6 mostra i simboli utilizzati nell’ambito della invocazione o
definizione di macro e degli attributi di un dato elemento.

<span class="caption">Table B-6: Macro e Attributi</span>

| Simbolo       | Spiegazione         |
| ------------- | ------------------- |
| `#[meta]`     | Attributo esterno   |
| `#![meta]`    | Attributo interno   |
| `$ident`      | Sostituzione macro  |
| `$ident:kind` | Metavariabile macro |
| `$(...)...`   | Ripetizione macro   |
| `ident!(...)`, `ident!{...}`, `ident![...]` | Invocazione macro |

----

La Tabella B-7 mostra i simboli che creano commenti.

<span class="caption">Tabella B-7: Commenti</span>

| Simbolo    | Spiegazione                              |
| ---------- | ---------------------------------------- |
| `//`       | Commento in linea                        |
| `//!`      | Linea interna commento documentazione    |
| `///`      | Linea esterna commento documentazione    |
| `/*...*/`  | Blocco di commento, commento multi-linea |
| `/*!...*/` | Blocco interno commento documentazione   |
| `/**...*/` | Blocco esterno commento documentazione   |

----

La Tabella B-8 mostra i contesti in cui sono usate le  parentesi tonde.

<span class="caption">Table B-8: Parentesi Tonde</span>

| Simbolo           | Spiegazione |
| ----------------- | --- |
| `()`              | Tupla vuota (_unit_), sia letterale che _type_ |
| `(expr)`          | Espressione tra parentesi |
| `(expr,)`         | Espressione di tupla con singolo elemento |
| `(type,)`         | Tupla con singolo _type_ |
| `(expr, ...)`     | Espressione tupla |
| `(type, ...)`     | _Type_ tupla |
| `expr(expr, ...)` | Espressione di chiamata di funzione; usato anche per inizializzare le varianti _struct_ tupla e _enum_ tupla |

----

La Tabella B-9 mostra i contesti di utilizzo delle parentesi graffe.

<span class="caption">Tabella B-9: Parentesi Graffe</span>

| Contesto     | Spiegazione        |
| ------------ | ------------------ |
| `{...}`      | Blocco di codice   |
| `Type {...}` | _Struct_ letterali |

----

La Tabella B-10 mostra i contesti in cui vengono utilizzate le parentesi quadre.

<span class="caption">Tabella B-10: Parentesi Quadre</span>

| Contesto        | Spiegazione |
| --------------- | --- |
| `[...]`         | Vettore letterale |
| `[x; n]`        | Vettore letterale contenente `n` copie di `x` |
| `[type; n]`     | Vettore tipizzato contenente `n` istanze di  `type` |
| `collezione[i]` | Indicizzazione di una collezione. Sovrascrivibile (`Index`, `IndexMut`) |
| `expr[..]`, `expr[a..]`, `expr[..b]`, `expr[a..b]` | Indicizzazione in collezioni per estrazione _slice_, usando `Range`, `RangeFrom`, `RangeTo`, o `RangeFull` come “indici” |
