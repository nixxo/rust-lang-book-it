## Usare gli Oggetti _Trait_ per Astrarre Comportamenti Condivisi

Nel Capitolo 8, abbiamo detto che un limite dei vettori è che possono contenere
elementi di un solo _type_. Abbiamo trovato una soluzione nel Listato 8-9
definendo una _enum_ `CellaFoglioDiCalcolo` che aveva varianti per interi, float
e testo. Questo ci permetteva di mettere _type_ diversi in ogni cella e comunque
avere un vettore che rappresentava una riga di celle. Questa è una soluzione
perfetta quando gli elementi intercambiabili sono un insieme fisso di _type_
noti a tempo di compilazione.

Però a volte vogliamo che chi usa la nostra libreria possa estendere l’insieme
di _type_ validi in una situazione. Per mostrare come farlo, creeremo un esempio
di interfaccia grafica (_GUI_) che scorre una lista di elementi, chiamando per
ognuno un metodo `disegna` per disegnarlo a schermo, una tecnica comune negli
strumenti _GUI_. Creeremo un _crate_ libreria chiamato `gui` che contiene la
struttura base di una libreria _GUI_. Questo _crate_ includerà _type_ da usare,
come `Bottone` o `CampoTesto`. Inoltre, gli utenti della libreria vorranno
creare i propri _type_ disegnabili: per esempio, uno aggiungerà un `Immagine` e
un altro una `BoxSelezione`.

Quando scriviamo la libreria, non possiamo sapere e definire tutti i _type_ che
altri programmatori potrebbero voler creare. Ma sappiamo che `gui` deve tenere
traccia di molti valori di _type_ diversi e deve chiamare un metodo `disegna` su
ognuno di questi valori. Non deve sapere esattamente cosa succede quando chiama
`disegna`, solo che quel metodo è disponibile.

In un linguaggio con ereditarietà, potremmo definire una classe `Componente` con
un metodo `disegna`. Le altre classi, come `Bottone`, `Immagine` e
`BoxSelezione`, erediterebbero da `Componente` e quindi avrebbero il metodo
`disegna`. Ognuno potrebbe sovrascriverlo per comportamenti personalizzati, ma
il framework tratterebbe tutti i _type_ come istanze di `Componente` e chiamare
`disegna`. Ma dato che Rust non ha ereditarietà, serve un altro modo per
strutturare `gui` permettendo agli utenti di creare nuovi _type_ compatibili con
la libreria.

### Definire un _Trait_ per un Comportamento Comune

Per implementare il comportamento che vogliamo in `gui`, definiamo un _trait_
chiamato `Disegna` con un metodo `disegna`. Poi definiamo un vettore che
contiene oggetti _trait_. Un oggetto _trait_ punta sia a un’istanza di un _type_
che implementa il _trait_, sia a una tabella usata per cercare durante
l’esecuzione i metodi _trait_ su quel _type_. Creiamo un oggetto _trait_
specificando un puntatore, come un _reference_ `&` o uno puntatore intelligente
`Box<T>`, poi la parola chiave `dyn` e infine specifichiamo il _trait_
rilevante. (Parleremo del motivo per cui gli oggetti _trait_ devono usare un
puntatore in [“_Type_ a Dimensione Dinamica e il _Trait_
`Sized`”][dynamically-sized]<!-- ignore --> nel Capitolo 20.) Possiamo usare
oggetti _trait_ al posto di _type_ generici o concreti. Ovunque usiamo un
oggetto _trait_, il sistema dei _type_ di Rust garantisce durante la
compilazione che ogni valore in quel contesto implementi il _trait_ dell’oggetto
_trait_, quindi non serve conoscere tutti i _type_ possibili al momento della
compilazione.

Abbiamo detto che in Rust evitiamo di chiamare “oggetti” _struct_ ed _enum_ per
distinguerli dagli oggetti di altri linguaggi. In una _struct_ o _enum_, dati e
comportamento in blocchi `impl` sono separati, mentre in altri linguaggi dati e
comportamento uniti formano un oggetto. E quindi gli oggetti _trait_ sono in
qualche modo simili agli oggetti in altri linguaggi nel senso che combinano dati
e comportamento. Ma gli oggetti _trait_ differiscono dalla tradizionale
definizione di oggetto in altri linguaggi perché non possono contenere dati. Gli
oggetti _trait_ non hanno la completezza che si trova in altri linguaggi:
servono specificamente solo per astrarre comportamenti comuni.

Il Listato 18-3 mostra come definire un _trait_ `Disegna` con un metodo
`disegna`.

<Listing number="18-3" file-name="src/lib.rs" caption="Definizione del _trait_ `Disegna`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-03/src/lib.rs}}
```

</Listing>

La sintassi dovrebbe essere familiare dalle discussioni sui _trait_ nel Capitolo
10. Poi, nel Listato 18-4, definiamo una _struct_ `Schermo` che contiene un
vettore `componenti`. Questo vettore è di _type_ `Box<dyn Disegna>`, che è un
oggetto _trait_; è un contenitore per qualunque _type_ in una `Box` che
implementi il _trait_ `Disegna`.

<Listing number="18-4" file-name="src/lib.rs" caption="Definizione della _struct_ `Schermo` con una campo `componenti` contenente un vettore di oggetti _trait_ che implementano `Disegna`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-04/src/lib.rs:here}}
```

</Listing>

Definiamo un metodo `esegui` su `Schermo` che chiama `disegna` su ogni elemento
di `componenti`, come nel Listato 18-5.

<Listing number="18-5" file-name="src/lib.rs" caption="Un metodo `esegui` in `Schermo` che chiama `disegna` per ogni componente">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-05/src/lib.rs:here}}
```

</Listing>

Questo funziona diversamente da una _struct_ con un parametro di _type_ generico
con vincoli di _trait_. Un _type_ generico può essere sostituito da un solo
_type_ concreto alla volta, mentre gli oggetti _trait_ permettono a più _type_
concreti di poter essere usati per quel ruolo durante l’esecuzione. Per esempio,
potremmo aver definito la _struct_ `Schermo` con un _type_ generico e un vincolo
di _trait_, come nel Listato 18-6.

<Listing number="18-6" file-name="src/lib.rs" caption="Implementazione alternativa della _struct_ `Schermo` e del metodo `esegui` usando _type_ generici e vincoli di _trait_">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-06/src/lib.rs:here}}
```

</Listing>

Questo limita a istanze di `Schermo` con una lista di componenti tutte dello
stesso _type_, per esempio tutti `Bottone` o tutti `CampoTesto`. Se si hanno
solo collezioni omogenee, usare generici è preferibile perché il codice sarà
monomorfizzato durante la compilazione usando i _type_ concreti.

Con gli oggetti _trait_, invece, una singola istanza di `Schermo` può contenere
un `Vec<T>` con una `Box<Bottone>` e una `Box<CampoTesto>` insieme. Vediamo come
funziona e poi parleremo delle implicazioni sulle prestazioni durante
l’esecuzione.

### Implementare il _Trait_

Ora aggiungiamo _type_ che implementano il _trait_ `Disegna`. Aggiungiamo un
_type_ `Bottone`. Scrivere una vera e propria libreria _GUI_ va oltre lo scopo
del libro, quindi il metodo `disegna` in non contiene nulla di utile nel corpo.
Per farsi un’idea di una possibile implementazione, un `Bottone` potrebbe avere
campi `larghezza`, `altezza` e `etichetta`, come nel Listato 18-7.

<Listing number="18-7" file-name="src/lib.rs" caption="La _struct_ `Bottone` che implementa il _trait_ `Disegna`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-07/src/lib.rs:here}}
```

</Listing>

I campi `larghezza`, `altezza` e `etichetta` su `Bottone` sono diversi dagli
altri componenti; per esempio, un _type_ `CampoTesto` potrebbe avere gli stessi
campi più un campo `temporaneo`. Ogni _type_ che vogliamo disegnare implementerà
il _trait_ `Disegna` usando codice diverso in `disegna` per definire come
disegnarsi, come fa `Bottone` (senza codice _GUI_ reale). `Bottone` potrebbe
anche avere altri metodi nel suo blocco `impl`, ad esempio per gestire cosa
succede al _click_, metodi che non si applicano a _type_ come `CampoTesto`.

Se qualcuno che usa la libreria definisce una `BoxSelezione` con campi
`larghezza`, `altezza` e `opzioni`, implementerà il _trait_ `Disegna` anche su
`BoxSelezione`, come nel Listato 18-8.

<Listing number="18-8" file-name="src/main.rs" caption="Un altro _crate_ che usa `gui` e implementa `Disegna` su `BoxSelezione`">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-08/src/main.rs:here}}
```

</Listing>

Chi userà la nostra libreria può quindi scrivere la funzione `main` creando
un’istanza di `Schermo`. All’istanza di `Schermo` aggiunge una `BoxSelezione` e
un `Bottone` mettendoli in `Box<T>`, facendoli diventare oggetti _trait_. Poi
chiama `esegui` sull’istanza di `Schermo`, che a sua volta chiama `disegna` su
ogni componente. Il Listato 18-9 mostra l’implementazione:

<Listing number="18-9" file-name="src/main.rs" caption="Uso di oggetti _trait_ per memorizzare valori di differente _type_ che implementano il medesimo _trait_">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-09/src/main.rs:here}}
```

</Listing>

Quando scriviamo la libreria, non sapevamo che qualcuno avrebbe aggiunto
`BoxSelezione`, ma l’implementazione di `Schermo` funziona comunque con quel
_type_ perché `BoxSelezione` implementa il _trait_ `Disegna` che quindi ha il
metodo `disegna`.

Questo concetto, preoccuparsi solo dei messaggi a cui un valore risponde invece
che del _type_ concreto, somiglia al _duck typing_ (_tipizzazione ad anatra_)
nei linguaggi a tipizzazione dinamica: _se cammina come un’anatra e fa “qua
qua”, allora è un’anatra_! Nel metodo `esegui` di `Schermo` nel Listato 18-5,
`esegui` non sa di che _type_ concreto è ogni componente, non controlla se è
un’istanza di `Bottone` o `BoxSelezione`, chiama semplicemente `disegna` sul
componente. Specificando `Box<dyn Disegna>` come _type_ dei valori nel vettore
`componenti`, abbiamo definito `Schermo` per accettare solo valori su cui si può
chiamare `disegna`.

Il vantaggio di usare oggetti _trait_ e il sistema dei _type_ di Rust per
scrivere codice simile a quello con _duck typing_ è che non dobbiamo mai
controllare durante l’esecuzione se un valore implementa un metodo o temere
errori se non l’implementa ma lo chiamiamo comunque. Rust non compila il codice
se i valori non implementano i _trait_ richiesti dagli oggetti _trait_.

Per esempio, il Listato 18-10 mostra cosa succede se proviamo a creare uno
`Schermo` con una `String` come componente.

<Listing number="18-10" file-name="src/main.rs" caption="Tentativo di usare un _type_ che non implementa il _trait_ dell’oggetto _trait_">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-10/src/main.rs}}
```

</Listing>

Avremo questo errore perché `String` non implementa il _trait_ `Disegna`:

```console
{{#include ../listings/ch18-oop/listing-18-10/output.txt}}
```

L’errore ci dice che o stiamo passando a `Schermo` qualcosa che non volevamo,
oppure dobbiamo implementare `Disegna` su `String` per permettere che `Schermo`
chiami `disegna` anche su quel _type_.

### Eseguire il Dynamic Dispatch

Come detto in [“Prestazioni del Codice utilizzando _Type_
Generici”][performance-of-code-using-generics]<!-- ignore --> nel Capitolo 10
sulla monomorfizzazione eseguita dal compilatore per i _type_ generici: il
compilatore genera implementazioni non generiche di funzioni e metodi per ogni
_type_ concreto usato al posto del _type_ generico. Il codice che risulta dalla
monomorfizzazione usa _static dispatch_, durante la compilazione il compilatore
conosce quale metodo stai chiamando. Questo è all’opposto del _dynamic
dispatch_, dove il compilatore non può sapere durante la compilazione quale
metodo stai chiamando. Nel caso di _dynamic dispatch_, il compilatore genera
codice che solo durante l’esecuzione saprà quale metodo chiamare.

Quando usiamo oggetti _trait_, Rust deve usare il _dynamic dispatch_. Il
compilatore non conosce tutti i _type_ che possono essere usati con il codice
che usa oggetti _trait_, quindi non sa quale metodo di quale _type_ chiamare.
Durante l’esecuzione, Rust usa i puntatori dentro l’oggetto _trait_ per decidere
il metodo da chiamare. Questa ricerca ha un costo prestazionale che non c’è con
lo _static dispatch_. Inoltre, il _dynamic dispatch_ impedisce che il
compilatore possa fare alcune ottimizzazioni, e Rust ha regole su dove si può
usare _dynamic dispatch_, chiamate _compatibilità dyn_. Queste regole esulano da
questa discussione, ma puoi leggere di più a riguardo nella
[documentazione][dyn-compatibility]<!-- ignore -->. Però abbiamo guadagnato più
flessibilità nel codice del Listato 18-5 e possiamo supportarla come nel Listato
18-9, quindi è un compromesso da considerare.

[performance-of-code-using-generics]: ch10-01-syntax.html#prestazioni-del-codice-utilizzando-type-generici
[dynamically-sized]: ch20-03-advanced-types.html#type-a-dimensione-dinamica-e-il-trait-sized
[dyn-compatibility]: https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility
