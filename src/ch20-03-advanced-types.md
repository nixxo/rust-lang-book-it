## _Type_ Avanzati

Il sistema dei _type_ di Rust ha alcune caratteristiche che abbiamo già
menzionato ma non ancora discusso. Inizieremo parlando dei _newtype_ in
generale, esaminando perché i _newtype_ sono utili come _type_. Poi passeremo
agli _alias_ di _type_, una caratteristica simile ai _newtype_ ma con una
semantica leggermente diversa. Discuteremo anche del _type_ `!` e dei _type_ a
dimensione dinamica.

### Sicurezza dei _Type_ e Astrazione Con il Modello _Newtype_

Questa sezione presuppone che tu abbia letto la sezione precedente
[“Implementare _Trait_ Esterni con il Modello
_Newtype_”][using-the-newtype-pattern]<!-- ignore -->. Il modello _newtype_ è
utile anche per compiti oltre quelli che abbiamo già discusso, tra cui far
rispettare staticamente che i valori non vengano confusi e indicare le unità di
misura di un valore. Hai visto un esempio dell’uso dei _newtype_ per indicare
unità di misura nel Listato 20-16: ricorda che le _struct_ `Millimetri` e
`Metri` incapsulavano valori `u32` come _newtype_. Se scrivessimo una funzione
con un parametro di _type_ `Millimetri`, non potremmo compilare un programma che
accidentalmente provasse a chiamare quella funzione con un valore di _type_
`Metri` o con un semplice `u32`.

Possiamo anche usare il modello _newtype_ per astrarre alcuni dettagli di
implementazione di un _type_: il nuovo _type_ può esporre una API pubblica
diversa dall’API del _type_ interno privato.

I _newtype_ possono anche nascondere l’implementazione interna. Ad esempio,
potremmo fornire un _type_ `Persone` per incapsulare un `HashMap<i32, String>`
che associa l’ID di una persona al nome. Il codice che usa `Persone`
interagirebbe solo con l’API pubblica che definiamo, ad esempio un metodo per
aggiungere un nome alla collezione `Persone`; quel codice non avrebbe bisogno di
sapere che internamente associamo un ID `i32` ai nomi. Il modello _newtype_ è un
modo leggero per ottenere l’incapsulamento per nasconde dettagli di
implementazione, come abbiamo discusso in [“Incapsulamento che Nasconde i
Dettagli di Implementazione”][encapsulation-that-hides]<!-- ignore --> nel
Capitolo 18.

### Sinonimi e _Alias_ di _Type_

Rust permette di dichiarare un _alias di _type__ per dare a un _type_ esistente
un altro nome. Per fare questa cosa usiamo la parola chiave `type`. Ad esempio,
possiamo creare l’_alias_ `Chilometri` per `i32` così:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:here}}
```

Ora `Chilometri` è un _sinonimo_ di `i32`; a differenza dei _type_ `Millimetri`
e `Metri` che abbiamo creato nel Listato 20-16, `Chilometri` non è un _type_
distinto e separato. I valori di _type_ `Chilometri` saranno trattati allo
stesso modo di quelli di _type_ `i32`:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:there}}
```

Poiché `Chilometri` e `i32` sono lo stesso _type_, possiamo sommare valori di
entrambi i _type_ e possiamo passare valori di _type_ `Chilometri` a funzioni
che accettano parametri `i32`. Tuttavia, usando questo metodo non otteniamo i
benefici di controllo dei _type_ che otteniamo con il modello _newtype_ discusso
prima. In altre parole, se confondiamo valori di _type_ `Chilometri` e `i32` da
qualche parte, il compilatore non ci darà errore.

Il caso d’uso principale per i sinonimi di _type_ è ridurre la ripetizione. Ad
esempio, potremmo avere una definizione di _type_ un po’ lunga:

```rust,ignore
Box<dyn Fn() + Send + 'static>
```

Scrivere questo _type_ lungo nelle firme delle funzioni e come annotazioni di
_type_ in tutto il codice può essere verboso e soggetto a errori. Immagina un
progetto pieno di codice come quello del Listato 20-25.

<Listing number="20-25" caption="Uso di un _type_ lungo in molti posti">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-25/src/main.rs:here}}
```

</Listing>

Un _alias_ di _type_ rende questo codice più gestibile riducendo la ripetizione.
Nel Listato 20-26, abbiamo introdotto un _alias_ chiamato `Thunk` per il _type_
verboso e possiamo sostituire tutti gli usi di quel _type_ con l’_alias_ più
corto `Thunk`.

<Listing number="20-26" caption="Introduzione di un _alias_ di _type_, `Thunk`, per ridurre la ripetizione">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-26/src/main.rs:here}}
```

</Listing>

Questo codice è molto più facile da leggere e scrivere! Scegliere un nome
significativo per un _alias_ di _type_ può anche aiutare a comunicare
chiaramente la nostra intenzione (_thunk_ è una parola tecnica usata per
indicare codice che verrà eseguito e valutato in un secondo momento, quindi è un
nome appropriato per una closure che viene memorizzata).

Gli _alias_ di _type_ sono anche comunemente usati con il _type_ `Result<T, E>`
per ridurre la ripetizione. Considera il modulo `std::io` nella libreria
standard. Le operazioni I/O spesso restituiscono un `Result<T, E>` per gestire
le situazioni in cui le operazioni possono fallire. Questa libreria ha una
_struct_ `std::io::Error` che rappresenta tutti i possibili errori di I/O. Molte
funzioni in `std::io` restituiscono un `Result<T, E>` dove `E` è
`std::io::Error`, come nelle funzioni del _trait_ `Write`:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-05-write-trait/src/lib.rs}}
```

Il `Result<..., Error>` si ripete molto. Per questo motivo, `std::io` ha questa
dichiarazione di _alias_ di _type_:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:here}}
```

Poiché questa dichiarazione è nel modulo `std::io`, possiamo usare l’_alias_
completamente qualificato `std::io::Result<T>`; cioè, un `Result<T, E>` con `E`
riempito come `std::io::Error`. Le firme delle funzioni del _trait_ `Write`
diventano così:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:there}}
```

L’alias di _type_ aiuta in due modi: rende il codice più facile da scrivere e
leggere _e_ ci fornisce un’interfaccia coerente in tutto `std::io`. Poiché è un
_alias_, è solo un altro `Result<T, E>`, il che significa che possiamo usare
tutti i metodi che funzionano su `Result<T, E>`, oltre alla sintassi speciale
come l’operatore `?`.

### Il _Type_ _Never_ Che Non Ritorna Mai

Rust ha un _type_ speciale chiamato `!` che in gergo di teoria dei _type_ è
chiamato _type_ _vuoto_ perché non ha valori. Preferiamo chiamarlo _type_
_never_ (_type_ _mai_) perché rappresenta il _type_ di ritorno di una funzione
che non restituirà mai nulla. Ecco un esempio:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-07-never-type/src/lib.rs:here}}
```

Questo codice significa “la funzione `bar` non restituirà mai”. Le funzioni che
non restituiscono mai sono chiamate _funzioni divergenti_. Non possiamo creare
valori di _type_ `!`, per cui `bar` non potrà mai restituirli.

Ma a cosa serve un _type_ per cui non si possono creare valori? Ricorda il
codice del Listato 2-5, parte del gioco degli indovinelli; ne riproduciamo un
pezzo qui nel Listato 20-27.

<Listing number="20-27" caption="Un `match` con un ramo che finisce con `continue`">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:ch20}}
```

</Listing>

All’epoca abbiamo trascurato alcuni dettagli di questo codice. In [“Controllare
il Flusso con il costrutto `match`”][the-match-control-flow-construct] nel
Capitolo 6 abbiamo spiegato che tutti i rami di un `match` devono ritornare lo
stesso _type_. Quindi, per esempio, il seguente codice non funziona:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-08-match-arms-different-types/src/main.rs:here}}
```

Il _type_ di `ipotesi` in questo codice dovrebbe essere un intero _e_ una
stringa, e Rust richiede che `ipotesi` sia di un solo _type_. Allora cosa
ritorna `continue`? Come facciamo a ritornare un `u32` da un ramo e avere un
altro ramo che termina con `continue` nel Listato 20-27?

Come avrai intuito, `continue` ha un valore di _type_ `!`. Questo significa che
quando Rust calcola il _type_ di `ipotesi`, guarda entrambi i rami: il primo con
valore `u32` e il secondo con valore `!`. Poiché `!` non può avere un valore,
Rust decide che il _type_ di `ipotesi` è `u32`.

Il modo formale di descrivere questo comportamento è che le espressioni di
_type_ `!` possono essere forzate in qualsiasi altro _type_. È permesso
terminare questo ramo di `match` con `continue` perché `continue` non
restituisce un valore; invece, sposta il controllo in cima al ciclo, quindi nel
caso di `Err` non assegniamo mai un valore a `ipotesi`.

Il _type_ _never_ è utile anche con la macro `panic!`. Ricorda la funzione
`unwrap` che chiamiamo sui valori `Option<T>` per ottenere un valore o fare
_panic_, con questa definizione:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-09-unwrap-definition/src/lib.rs:here}}
```

In questo codice, succede la stessa cosa vista nel `match` del Listato 20-27:
Rust vede che `val` ha _type_ `T` e `panic!` ha _type_ `!`, quindi il risultato
dell’espressione complessiva `match` è `T`. Questo codice funziona perché
`panic!` non produce un valore; termina il programma. Nel caso `None` non
ritorneremo un valore da `unwrap`, quindi questo codice è valido.

Un’ultima espressione che ha _type_ `!` è un `loop`:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-10-loop-returns-never/src/main.rs:here}}
```

Qui il _loop_ non termina mai, per cui il valore dell’espressione è `!`. Questo
non varrebbe se usassimo un `break` perché il ciclo terminerebbe quando
incontrasse `break`.

### _Type_ a Dimensione Dinamica e il _Trait_ `Sized`

Rust ha bisogno di conoscere alcune informazioni sui _type_, tipo quanto spazio
allocare per un valore di quel _type_. Questo rende un po’ complicato il
concetto di _type_ a dimensione dinamica_. Detti anche _DST_ o _type_ _non
dimensionati_, consentono di scrivere codice che usa valori la cui dimensione è
nota solo in fase di esecuzione.

Parliamo nel dettaglio di un _type_ a dimensione dinamica chiamato `str`, che
abbiamo usato spesso nel libro. Proprio così, non `&str` ma `str` da solo è un
_DST_. In molti casi, come nel caso di stringhe inserite da un utente, non
possiamo sapere la lunghezza della stringa a priori se non durante l’esecuzione.
Questo significa che non possiamo creare una variabile di _type_ `str`, né
ricevere un argomento di _type_ `str`. Considera il seguente codice, che non
funziona:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-11-cant-create-str/src/main.rs:here}}
```

Rust deve sapere quanto spazio allocare per un valore di un qualsiasi _type_ e
tutti i valori di quel _type_ devono occupare la stessa quantità di memoria. Se
Rust ci permettesse di scrivere questo codice, quei due valori `str` dovrebbero
occupare la stessa quantità di spazio, ma hanno lunghezze diverse: `s1`
necessita di 12 byte, `s2` di 15. Ecco perché non è possibile creare una
variabile di _type_ `str`.

Quindi cosa facciamo? La risposta la conosci già: cambiamo i _type_ di `s1` e
`s2` da `str` a `&str`. Ricorda da [“_Slice_ di Stringa”][string-slices] nel
Capitolo 4 che la struttura dati _slice_ memorizza solo l’indirizzo di partenza
e la lunghezza della _slice_. Perciò, anche se un `&T` è un singolo valore che
memorizza l’indirizzo di memoria di `T`, un `&str` è _due_ valori: l’indirizzo
di `str` e la sua lunghezza. Perciò sappiamo sempre la dimensione statica di un
valore `&str`: è doppia rispetto alla lunghezza di un `usize`. E quindi,
conosciamo sempre la dimensione di una `&str` indipendentemente dalla lunghezza
della stringa. In generale, questo è il modo in cui si usano i _type_
dimensionati dinamicamente in Rust: hanno un pezzettino di metadati in più per
memorizzare la dimensione dell’informazione dinamica. La regola d’oro dei _DST_
è che dobbiamo sempre mettere valori di _type_ dimensionato dinamicamente dietro
a qualche tipo di puntatore.

Possiamo combinare `str` con tanti _type_ di puntatori: ad esempio, `Box<str>` o
`Rc<str>`. Hai già visto questo ma con un altro _type_ a dimensione dinamica: i
_trait_. Ogni _trait_ è un _type_ dimensionato dinamicamente che può essere
indicato usando il nome del _trait_. In [“Usare Oggetti _Trait_ per Astrarre
Comportamenti Condivisi”][using-trait-objects]<!-- ignore --> nel Capitolo 18
abbiamo menzionato che per usare _trait_ come oggetti _trait_ dobbiamo metterli
dietro a un puntatore, come `&dyn Trait` o `Box<dyn Trait>` (anche `Rc<dyn
Trait>` andrebbe bene).

Per lavorare con i _DST_, Rust fornisce il _trait_ `Sized`, che determina se la
dimensione di un _type_ è nota a tempo di compilazione. Questo _trait_ è
implementato automaticamente per tutto ciò che ha dimensione nota a
compilazione. Inoltre, Rust aggiunge implicitamente un vincolo su `Sized` per
ogni funzione generica: questo vuol dire che la definizione di una funzione
generica come questa:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-12-generic-fn-definition/src/lib.rs}}
```

viene trattata come se fosse scritta così:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-13-generic-implicit-sized-bound/src/lib.rs}}
```

Per default, le funzioni generiche funzionano solo su _type_ con dimensione nota
a tempo di compilazione. Però puoi usare questa sintassi speciale per allentare
questa restrizione:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-14-generic-maybe-sized/src/lib.rs}}
```

Un vincolo `?Sized` significa “`T` può essere o no di dimensione fissa” e questa
notazione sovrascrive il comportamento predefinito che i generici debbano avere
dimensione nota a tempo di compilazione. La sintassi `?Trait` con questo
significato è disponibile solo per `Sized` e non per altri _trait_.

Nota anche che abbiamo cambiato il _type_ del parametro `t` da `T` a `&T`.
Poiché il _type_ potrebbe non essere `Sized`, dobbiamo usarlo dietro a un
qualche tipo di puntatore. In questo caso usiamo un _reference_.

Ed ora, continuiamo parlando di funzioni e chiusure!

[encapsulation-that-hides]: ch18-01-what-is-oo.html#incapsulamento-che-nasconde-i-dettagli-di-implementazione
[string-slices]: ch04-03-slices.html#slice-di-stringa
[the-match-control-flow-construct]: ch06-02-match.html#controllare-il-flusso-col-costrutto-match
[using-trait-objects]: ch18-02-trait-objects.html#usare-gli-oggetti-trait-per-astrarre-comportamenti-condivisi
[using-the-newtype-pattern]: ch20-02-advanced-traits.html#implementare-trait-esterni-con-il-modello-newtype