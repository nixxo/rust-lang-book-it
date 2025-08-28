## Sintassi dei Metodi

I metodi (_method_) sono simili alle funzioni: le dichiariamo con la parola
chiave `fn` e un nome, possono avere parametri e un valore di ritorno, e
contengono del codice che viene eseguito quando il metodo viene chiamato da
un’altra parte. Diversamente dalle funzioni, i metodi sono definiti nel contesto
di una _struct_ (o di un _enum_ o di un _trait object_, che tratteremo nel
[Capitolo 6][enums]<!-- ignore --> e [Capitolo 18][trait-objects]<!-- ignore
-->, rispettivamente), e il loro primo parametro è sempre `self`, che
rappresenta l’istanza della _struct_ su cui il metodo viene chiamato.

### Definire i Metodi

Trasformiamo la funzione `area` che prende un’istanza di `Rettangolo` come
parametro rendendola invece un metodo definito sulla _struct_ `Rettangolo`, come
mostrato nel Listato 5-13.

<Listing number="5-13" file-name="src/main.rs" caption="Definizione di un metodo `area` nella _struct_ `Rettangolo`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

</Listing>

Per definire la funzione nel contesto di `Rettangolo`, iniziamo un blocco `impl`
(_implementazione_) per `Rettangolo`. Tutto ciò che sta dentro questo blocco
`impl` sarà associato al _type_ `Rettangolo`. Poi spostiamo la funzione `area`
all’interno delle parentesi graffe dell’`impl` e cambiamo il primo (e in questo
caso, unico) parametro in `self` nella firma e ovunque nel corpo. In `main`,
dove chiamavamo la funzione `area` passando `rettangolo1` come argomento,
possiamo invece usare la _sintassi dei metodi_ per chiamare il metodo `area`
sull’istanza di `Rettangolo`. La sintassi del metodo va dopo un’istanza:
aggiungiamo un punto seguito dal nome del metodo, parentesi tonde ed eventuali
argomenti.

Nella firma di `area` usiamo `&self` invece di `rettangolo: &Rettangolo`. Il
`&self` è in realtà l’abbreviazione di `self: &Self`. All’interno di un blocco
`impl`, il _type_ `Self` è un alias del _type_ per cui il blocco `impl` è stato
scritto. I metodi devono avere un parametro chiamato `self` di _type_ `Self`
come primo parametro, quindi Rust permette di abbreviare questo con soltanto il
nome `self` nella prima posizione dei parametri. Nota che dobbiamo comunque
usare `&` davanti alla forma abbreviata `self` per indicare che questo metodo
prende in prestito l’istanza `Self`, esattamente come facevamo con `rettangolo:
&Rettangolo`. I metodi possono prendere la _ownership_ di `self`, prendere un
_reference_ immutabile a `self`, come abbiamo fatto qui, oppure prendere un
_reference_ mutabile a `self`, proprio come possono fare con qualsiasi altro
parametro.

Qui abbiamo scelto `&self` per lo stesso motivo per cui abbiamo usato
`&Rettangolo` nella versione precedente: non serve che prendiamo la _ownership_,
vogliamo solo leggere i dati nella _struct_, non modificarli. Se volessimo
modificare l’istanza su cui chiamiamo il metodo come parte di ciò che il metodo
fa, useremmo `&mut self` come primo parametro. Avere un metodo che prende la
_ownership_ dell’istanza usando semplicemente `self` come primo parametro è
raro; questa tecnica è solitamente usata quando il metodo trasforma `self` in
qualcos’altro e si vuole impedire al chiamante di usare l’istanza originale dopo
la trasformazione.

La ragione principale per usare i metodi invece delle funzioni, oltre a fornire
la sintassi dei metodi e a non dover ripetere il _type_ di `self` in ogni firma
dei metodi, è per organizzazione. Abbiamo messo tutte le cose che possiamo fare
con un’istanza di un _type_ in un unico blocco `impl` invece di costringere chi
dovrà in futuro leggere o manutenere il nostro codice a cercare le funzionalità
di `Rettangolo` in vari posti nella libreria che forniamo.

Nota che possiamo scegliere di dare a un metodo lo stesso nome di uno dei campi
della _struct_. Per esempio, possiamo definire un metodo su `Rettangolo` che si
chiama anch’esso `larghezza`:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

</Listing>

Qui scegliamo di fare in modo che il metodo `larghezza` ritorni `true` se il
valore nel campo `larghezza` dell’istanza è maggiore di `0` e `false` se il
valore è `0`: possiamo usare un campo all’interno di un metodo con lo stesso
nome per qualunque scopo. In `main`, quando seguiamo `rettangolo1.larghezza` con
le parentesi tonde, Rust sa che si intende il metodo `larghezza`. Quando non
usiamo le parentesi tonde, Rust sa che intendiamo il campo `larghezza`.

Spesso, ma non sempre, quando diamo a un metodo lo stesso nome di un campo
vogliamo che esso ritorni soltanto il valore del campo e non faccia altro. I
metodi di questo tipo sono chiamati _getter_ (_metodi di incapsulamento_) e Rust
non li implementa automaticamente per i campi della _struct_ come fanno alcuni
altri linguaggi di programmazione. I _getter_ sono utili perché puoi rendere il
campo privato ma il metodo pubblico, abilitando così accesso in sola lettura a
quel campo come parte dell’API pubblica del _type_. Discuteremo cosa sono
pubblico e privato e come designare un campo o un metodo come pubblico o privato
nel [Capitolo 7][public]<!-- ignore -->.

> ### Dov’è l’Operatore `->`?
> 
> In C e C++ si usano due operatori diversi per accedere ai membri: si usa `.`
> quando si lavora direttamente con un oggetto, e `->` quando si lavora con un
> puntatore all’oggetto e prima bisogna dereferenziarlo. In C++, questi
> operatori possono essere usati per chiamare i metodi; in C, sono usati solo
> per accedere ai campi delle _struct_. In altre parole, se `oggetto` è un
> puntatore, `oggetto->qualcosa()` è simile a `(*oggetto).qualcosa()`.
>
> Rust non ha un equivalente dell’operatore `->`; invece, Rust ha una
> funzionalità chiamata _referenziamento e de-referenziamento automatico_
> (_automatic referencing and dereferencing_). Chiamare i metodi è uno dei pochi
> posti in Rust che implementa questa funzionalità.
>
> Ecco come funziona: quando chiami un metodo con `oggetto.qualcosa()`, Rust
> aggiunge automaticamente `&`, `&mut`, o `*` affinché `oggetto` corrisponda
> alla firma del metodo. In altre parole, i seguenti sono equivalenti:
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Punto {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Punto {
> #    fn distanza(&self, altro: &Punto) -> f64 {
> #        let x_quad = f64::powi(altro.x - self.x, 2);
> #        let y_quad = f64::powi(altro.y - self.y, 2);
> #
> #        f64::sqrt(x_quad + y_quad)
> #    }
> # }
> # let p1 = Punto { x: 0.0, y: 0.0 };
> # let p2 = Punto { x: 5.0, y: 6.5 };
> p1.distanza(&p2);
> (&p1).distanza(&p2);
> ```
>
> Il primo sembra molto più pulito. Questo comportamento di _referencing
> automatico_ funziona perché i metodi hanno un _receiver_ (_recettore_) chiaro,
> il _type_ di `self`. Dato il _receiver_ e il nome di un metodo, Rust può
> determinare in modo definitivo se il metodo sta leggendo (`&self`), mutando
> (`&mut self`), o consumando (`self`). Il fatto che Rust renda implicito il
> _borrowing_ per i _receiver_ dei metodi è una parte importante per rendere
> l’_ownership_ ergonomica nella pratica.

### Metodi con Più Parametri

Esercitiamoci ad usare i metodi implementando un secondo metodo sulla _struct_
`Rettangolo`. Questa volta vogliamo che un’istanza di `Rettangolo` prenda
un’altra istanza di `Rettangolo` e ritorni `true` se la seconda `Rettangolo` può
entrare completamente dentro `self` (la prima `Rettangolo`); altrimenti dovrebbe
ritornare `false`. Cioè, una volta definito il metodo `puo_contenere`, dovremmo
poter scrivere il programma mostrato nel Listato 5-14.

<Listing number="5-14" file-name="src/main.rs" caption="Uso del metodo `puo_contenere` ancora da scrivere">

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

</Listing>

L’output atteso sarà il seguente perché entrambe le dimensioni di `rettangolo2`
sono più piccole delle dimensioni di `rettangolo1`, ma `rettangolo3` è più larga
di `rettangolo1`:

```text
Può rettangolo1 contenere rettangolo2? true
Può rettangolo1 contenere rettangolo3? false
```

Sappiamo che vogliamo definire un metodo, quindi sarà all’interno del blocco
`impl Rettangolo`. Il nome del metodo sarà `puo_contenere`, e prenderà un
_reference_ immutabile di un’altra `Rettangolo` come parametro. Possiamo dedurre
il _type_ del parametro osservando il codice che chiama il metodo:
`rettangolo1.puo_contenere(&rettangolo2)` passa `&rettangolo2`, che è un
_reference_ immutabile di `rettangolo2`, un’istanza di `Rettangolo`. Questo ha
senso perché abbiamo solo bisogno di leggere `rettangolo2` (invece di
modificarlo, il che richiederebbe un _reference_ mutabile), e vogliamo che
`main` mantenga l’_ownership_ di `rettangolo2` così da poterlo usare di nuovo
dopo la chiamata a `puo_contenere`. Il valore di ritorno di `puo_contenere` sarà
un Booleano, e l’implementazione verificherà se la larghezza e l’altezza di
`self` sono maggiori rispetto alla larghezza e all’altezza dell’altra
`Rettangolo`, rispettivamente. Aggiungiamo il nuovo metodo `puo_contenere` al
blocco `impl` del Listato 5-13, come mostrato nel Listato 5-15.

<Listing number="5-15" file-name="src/main.rs" caption="Implementazione del metodo `puo_contenere` in `Rettangolo` che riceve un'altra istanza di `Rettangolo` come parametro">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

</Listing>

Quando eseguiamo questo codice con la funzione `main` del Listato 5-14,
otterremo l’output desiderato. I metodi possono prendere parametri multipli che
aggiungiamo alla firma dopo il parametro `self`, e quei parametri funzionano
proprio come i parametri nelle funzioni.

### Funzioni Associate

Tutte le funzioni definite all’interno di un blocco `impl` sono chiamate
_funzioni associate_ (_associated functions_) perché sono associate al _type_
nominato dopo la parola `impl`. Possiamo definire funzioni associate che non
hanno `self` come primo parametro (e quindi non sono metodi) perché non hanno
bisogno di un’istanza del _type_ per svolgere il loro compito. Ne abbiamo già
usata una: la funzione `String::from` implementata sul _type_ `String`.

Le funzioni associate che non sono metodi sono spesso usate come _costruttori_
che ritornano una nuova istanza della _struct_. Spesso si chiamano `new` perchè
`new` non è una parola chiave e non è incorporata nel linguaggio. Per esempio,
potremmo decidere di fornire una funzione associata chiamata `quadrato` che
prende un parametro di dimensione e lo usa sia come larghezza sia come altezza,
rendendo più semplice creare un `Rettangolo` _quadrato_ invece di dover
specificare lo stesso valore due volte:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

La parola chiave `Self` nel _type_ di ritorno e nel corpo della funzione è un
alias per il _type_ che appare dopo la parola chiave `impl`, che in questo caso
è `Rettangolo`.

Per chiamare questa funzione associata, usiamo la sintassi `::` con il nome
della _struct_; `let quad = Rettangolo::quadrato(3);` è un esempio. Questa
funzione è organizzata nel _namespace_ della _struct_: la sintassi `::` è usata
sia per le funzioni associate sia per i _namespace_ creati dai moduli. Parleremo
più approfonditamente dei moduli nel [Capitolo 7][modules]<!-- ignore -->.

### Blocchi `impl` Multipli

A ogni _struct_ è permesso avere più blocchi `impl`. Per esempio, il Listato
5-15 è equivalente al codice mostrato nel Listato 5-16, che ha ognuno dei metodi
nel proprio blocco `impl`.

<Listing number="5-16" caption="Riscrittura del Listato 5-15 usando più blocchi `impl`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

</Listing>

Non c’è motivo di separare questi metodi in più blocchi `impl` in questo caso,
ma questa è una sintassi valida. Vedremo un caso in cui più blocchi `impl` sono
utili nel Capitolo 10, dove discuteremo i _type_ generici e i _trait_.

## Riepilogo

Le _struct_ ti permettono di creare _type_ personalizzati significativi per il
tuo dominio. Usando le _struct_, puoi mantenere pezzi di dati correlati tra loro
e dare un nome a ciascun pezzo per rendere il codice chiaro. Nei blocchi `impl`
puoi definire funzioni associate al tuo _type_, e i metodi sono un tipo di
funzione associata che ti permette di specificare il comportamento che le
istanze delle tue _struct_ hanno.

Ma le _struct_ non sono l’unico modo per creare _type_ personalizzati: passiamo
ad un altro _type_ di Rust, le _enumerazioni_, per aggiungere un altro strumento
alla tua cassetta degli attrezzi.

[enums]: ch06-00-enums.html
[trait-objects]: ch18-02-trait-objects.md
[public]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#esporre-path-con-la-parola-chiave-pub
[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
