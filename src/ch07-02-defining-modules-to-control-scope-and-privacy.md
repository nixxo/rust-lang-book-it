## Definire Moduli per Controllare _Scope_ e Privacy

In questa sezione, parleremo dei moduli e di altre parti del sistema dei moduli,
in particolare dei _path_ (_percorsi_), che ti permettono di nominare gli
elementi; la parola chiave `use` che porta un _path_ in scope; e la parola
chiave `pub` per rendere pubblici gli elementi. Discuteremo anche della parola
chiave `as`, dei pacchetti esterni e dell'operatore _glob_.

### Scheda Informativa sui Moduli

Prima di entrare nei dettagli dei moduli e dei _path_, qui forniamo un rapido
riferimento su come funzionano i moduli, i _path_, la parola chiave `use` e la
parola chiave `pub` nel compilatore, e come la maggior parte degli sviluppatori
organizza il proprio codice. Esamineremo esempi di ciascuna di queste regole nel
corso di questo capitolo, ma questo è un ottimo posto da consultare come
promemoria su come funzionano i moduli.

- **Inizia dalla radice del _crate_**: Quando compili un _crate_, il compilatore
  prima cerca nel file di radice del _crate_ (di solito _src/lib.rs_ per un
  _crate libreria_ o _src/main.rs_ per un _crate binario_) il codice da
  compilare.
- **Dichiarare moduli**: Nel file di radice del _crate_, puoi dichiarare nuovi
  moduli; ad esempio, dichiari un modulo “giardino” con `mod giardino;`. Il
  compilatore cercherà il codice del modulo in questi luoghi:
  - Sulla linea, all'interno delle parentesi graffe che sostituiscono il punto e
    virgola dopo `mod giardino`
  - Nel file _src/giardino.rs_
  - Nel file _src/giardino/mod.rs_
- **Dichiarare sottomoduli**: In qualsiasi file diverso dalla radice del
  _crate_, puoi dichiarare sottomoduli. Ad esempio, potresti dichiarare `mod
  verdure;` in _src/giardino.rs_. Il compilatore cercherà il codice del
  sottomodulo all'interno della directory nominata per il modulo genitore
  (_parent_) in questi luoghi:
  - Sulla linea, direttamente dopo `mod verdure`, all'interno delle parentesi
    graffe invece del punto e virgola
  - Nel file _src/giardino/verdure.rs_
  - Nel file _src/giardino/verdure/mod.rs_
- **_Path_ per il codice nei moduli**: Una volta che un modulo è parte del tuo
  _crate_, puoi fare riferimento al codice in quel modulo da qualsiasi altro
  punto dello stesso _crate_, purché le regole di _privacy_ lo consentano,
  utilizzando il _path_ per il codice. Ad esempio, un _type_ `Asparagi` nel
  modulo delle verdure del giardino si troverebbe a
  `crate::giardino::verdure::Asparagi`.
- **Privato vs. pubblico**: Il codice all'interno di un modulo è non
  utilizzabile, _privato_, dai suoi moduli genitore come impostazione
  predefinita. Per rendere un modilo utilizzabile, _pubblico_, è necessario
  dichiaralo con `pub mod` invece di `mod`. Per rendere _pubblici_ anche gli
  elementi all'interno di un modulo pubblico, usa `pub` prima delle loro
  dichiarazioni.
- **La parola chiave `use`**: All'interno di uno _scope_, la parola chiave `use`
  crea scorciatoie per gli elementi per ridurre la ripetizione di lunghi _path_.
  In qualsiasi _scope_ che può fare riferimento a
  `crate::giardino::verdure::Asparagi`, puoi creare una scorciatoia con `use
  crate::giardino::verdure::Asparagi;` e da quel momento in poi devi solo
  scrivere `Asparagi` per utilizzare quel _type_ nello _scope_.

Era creiamo un _crate binario_ chiamato `cortile` che illustra queste regole. La
directory del crate, anch'essa chiamata `cortile`, contiene questi file e
directory:

```text
cortile
├── Cargo.lock
├── Cargo.toml
└── src
    ├── giardino
    │   └── verdure.rs
    ├── giardino.rs
    └── main.rs
```

La radice del crate in questo caso è _src/main.rs_, e contiene:

<Listing file-name="src/main.rs">

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/main.rs}}
```

</Listing>

La riga `pub mod giardino;` dice al compilatore di includere il codice che trova
in _src/giardino.rs_, che è:

<Listing file-name="src/giardino.rs">

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/giardino.rs}}
```

</Listing>

Qui, `pub mod verdure;` significa che il codice in _src/giardino/verdure.rs_ è
incluso anch'esso. Quel codice è:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/giardino/verdure.rs}}
```

Ora entriamo nei dettagli di queste regole e dimostriamo come funzionano!

### Raggruppare Codice Correlato in Moduli

I _moduli_ ci permettono di organizzare il codice all'interno di un _crate_ per
migliore leggibilità e facilità di riutilizzo. I moduli ci consentono anche di
controllare la _privacy_ degli elementi, poiché il codice all'interno di un
modulo è privato come impostazione predefinita. Gli elementi privati sono
dettagli di implementazione interni non disponibili per l'uso esterno. Possiamo
scegliere di rendere pubblici i moduli e gli elementi al loro interno, il che li
espone per consentire al codice esterno di utilizzarli e dipendere da essi.

Come esempio, scriviamo un _crate libreria_ che fornisce la funzionalità di un
ristorante. Definiremo le firme delle funzioni ma lasceremo i loro corpi vuoti
per concentrarci sull'organizzazione del codice piuttosto che
sull'implementazione vera e propria.

Nel settore della ristorazione, alcune "funzioni" di un ristorante sono chiamate
_sala_ e altre _cucina_. La "sala" è dove si trovano i clienti; questo comprende
dove l'oste riceve i clienti, i camerieri prendono ordini e pagamenti, e i
baristi preparano drink. La "cucina" è dove gli chef e i cuochi lavorano in
cucina, i lavapiatti puliscono e i manager svolgono lavori amministrativi.

Per strutturare il nostro _crate_ in questo modo, possiamo organizzare le sue
funzioni in moduli annidati. Crea una nuova libreria chiamata `ristorante`
eseguendo `cargo new ristorante --lib`. Poi inserisci il codice nel Listato 7-1
in _src/lib.rs_ per definire alcuni moduli e firme di funzione; questo codice è
la sezione _sala_.

<Listing number="7-1" file-name="src/lib.rs" caption="Un modulo `sala` contenente altri moduli che poi contengono funzioni">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

</Listing>

Definiamo un modulo con la parola chiave `mod` seguita dal nome del modulo (in
questo caso, `sala`). Il corpo del modulo va quindi all'interno delle parentesi
graffe. All'interno dei moduli, possiamo inserire altri moduli, come in questo
caso con i moduli `accoglienza` e `servizio`. I moduli possono anche contenere
definizioni per altri elementi, come _struct_, _enum_, costanti, _trait_ e, come
nel Listato 7-1, funzioni.

Utilizzando i moduli, possiamo raggruppare definizioni correlate insieme e
nominare il motivo per cui sono correlate. I programmatori che utilizzano questo
codice possono navigare nel codice in base ai gruppi piuttosto che dover leggere
tutte le definizioni, rendendo più facile trovare le definizioni rilevanti per
loro. I programmatori che aggiungono nuove funzionalità a questo codice
saprebbero dove posizionare il codice per mantenere organizzato il programma.

In precedenza, abbiamo menzionato che _src/main.rs_ e _src/lib.rs_ sono chiamati
radici del _crate_. Il motivo del loro nome è che i contenuti di uno di questi
due file formano un modulo chiamato `crate` alla radice della struttura del
modulo del _crate_, nota come _albero dei moduli_ (_module tree_).

Il Listato 7-2 mostra l'albero dei moduli per la struttura nel Listato 7-1.

<Listing number="7-2" caption="L'albero dei moduli per il codice nel Listato 7-1">

```text
crate
 └── sala
     ├── accoglienza
     │   ├── aggiungi_in_lista
     │   └── metti_al_tavolo
     └── servizio
         ├── prendi_ordine
         ├── servi_ordine
         └── prendi_pagamento
```

</Listing>

Questo albero mostra come alcuni dei moduli si annidano all'interno di altri
moduli; ad esempio, `accoglienza` si annida all'interno di `sala`. L'albero
mostra anche che alcuni moduli sono _fratelli_, il che significa che sono
definiti nello stesso modulo; `accoglienza` e `servizio` sono fratelli definiti
all'interno di `sala`. Se il modulo A è contenuto all'interno del modulo B,
diciamo che il modulo A è il _figlio_ del modulo B e che il modulo B è il
_genitore_ del modulo A. Nota che l'intero albero dei moduli è radicato sotto il
modulo implicito chiamato `crate`.

L'albero dei moduli potrebbe ricordarti l'albero delle directory del filesystem
sul tuo computer; questo è un confronto molto appropriato! Proprio come le
directory in un filesystem, usi i moduli per organizzare il tuo codice. E
proprio come i file in una directory, abbiamo bisogno di un modo per trovare i
nostri moduli.
