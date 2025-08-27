## Percorsi per fare riferimento a un elemento nell'albero dei moduli

Per mostrare a Rust dove trovare un elemento in un albero dei moduli,
utilizziamo un _path_ (_percorso_) nello stesso modo in cui usiamo un _path_
quando navighiamo in un filesystem. Per chiamare una funzione, dobbiamo
conoscere il suo _path_.

Un _path_ può assumere due forme:

- Un _path assoluto_ è il percorso completo che inizia dalla radice del _crate_;
  per il codice di un _crate_ esterno, il percorso assoluto inizia con il nome
  del _crate_, e per il codice del _crate_ corrente, inizia con il letterale
  `crate`.
- Un _path relativo_ inizia dal modulo corrente e utilizza `self`, `super` o un
  identificatore nel modulo corrente.

Sia i _path_ assoluti che relativi sono seguiti da uno o più identificatori
separati da doppi due punti (`::`).

Tornando al Listato 7-1, supponiamo di voler chiamare la funzione
`aggiungi_in_lista`. Questo è lo stesso che chiedere: qual è il _path_ della
funzione `aggiungi_in_lista`? Il Listato 7-3 contiene il Listato 7-1 con alcuni
dei moduli e delle funzioni rimossi.

Mostreremo due modi per chiamare la funzione `aggiungi_in_lista` da una nuova
funzione, `mangiare_al_ristorante`, definita nella radice del _crate_. Questi
_path_ sono corretti, ma c'è un altro problema che impedirà a questo esempio di
compilare così com'è. Spiegheremo perché tra poco.

La funzione `mangiare_al_ristorante` fa parte dell'API pubblica del nostro
_crate libreria_, quindi la contrassegniamo con la parola chiave `pub`. Nella
sezione [“Esporre _Path_ con la Parola Chiave `pub`”][pub]<!-- ignore -->,
entreremo nei dettagli di `pub`.

<Listing number="7-3" file-name="src/lib.rs" caption="Chiamare la funzione `aggiungi_in_lista` utilizzando path assoluti e relativi">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

</Listing>

La prima volta che chiamiamo la funzione `aggiungi_in_lista` in
`mangiare_al_ristorante`, utilizziamo un percorso assoluto. La funzione
`aggiungi_in_lista` è definita nello stesso _crate_ di `mangiare_al_ristorante`,
il che significa che possiamo usare la parola chiave `crate` per iniziare un
percorso assoluto. Includiamo quindi ciascuno dei moduli successivi fino a
raggiungere `aggiungi_in_lista`. Puoi immaginare un filesystem con la stessa
struttura: specificheremmo il percorso `/sala/accoglienza/aggiungi_in_lista` per
eseguire il programma `aggiungi_in_lista`; utilizzare il nome del _crate_ per
partire dalla radice del _crate_ è come usare `/` per partire dalla radice del
filesystem nel tuo shell.

La seconda volta che chiamiamo `aggiungi_in_lista` in `mangiare_al_ristorante`,
utilizziamo un percorso relativo. Il percorso inizia con `sala`, il nome del
modulo definito allo stesso livello dell'albero dei moduli di
`mangiare_al_ristorante`. Qui l'equivalente nel filesystem sarebbe utilizzare il
percorso `sala/accoglienza/aggiungi_in_lista`. Iniziare con un nome di modulo
significa che il percorso è relativo.

Scegliere se utilizzare un percorso relativo o assoluto è una decisione che
prenderai in base al tuo progetto, e dipende da se è più probabile che tu sposti
il codice di definizione dell'elemento separatamente o insieme al codice che
utilizza l'elemento. Ad esempio, se spostassimo il modulo `sala` e la funzione
`mangiare_al_ristorante` in un modulo chiamato `gestione_cliente`, dovremmo
aggiornare il percorso assoluto per `aggiungi_in_lista`, ma il percorso relativo
rimarrebbe valido. Tuttavia, se spostassimo la funzione `mangiare_al_ristorante`
separatamente in un modulo chiamato `cena`, il percorso assoluto per la chiamata
a `aggiungi_in_lista` rimarrebbe lo stesso, ma il percorso relativo dovrebbe
essere aggiornato. La nostra preferenza in generale è specificare percorsi
assoluti perché è più probabile che vogliamo spostare le definizioni di codice e
le chiamate agli elementi in modo indipendente l'una dall'altra.

Proviamo a compilare il Listato 7-3 e scopriamo perché non compila ancora! Gli
errori che otteniamo sono mostrati nel Listato 7-4.

<Listing number="7-4" caption="Errori del compilatore durante la costruzione del codice nel Listato 7-3">

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

</Listing>

I messaggi di errore dicono che il modulo `accoglienza` è privato. In altre
parole, abbiamo i percorsi corretti per il modulo `accoglienza` e la funzione
`aggiungi_in_lista`, ma Rust non ci permette di usarli perché non ha accesso
alle sezioni private. In Rust, tutti gli elementi (funzioni, metodi, _struct_,
_enum_, moduli e costanti) sono privati rispetto ai moduli genitore come
impostazione predefinita. Se desideri rendere un elemento come una funzione o
una _struct_ privata, lo metti in un modulo.

Gli elementi in un modulo genitore non possono utilizzare gli elementi privati
all'interno dei moduli figli, ma gli elementi nei moduli figli possono
utilizzare gli elementi nei loro moduli genitore. Questo perché i moduli figli
avvolgono e nascondono i loro dettagli di implementazione, ma i moduli figli
possono vedere il contesto in cui sono definiti. Per continuare con la nostra
metafora, pensa alle regole di privacy come se fossero l'ufficio posteriore di
un ristorante: ciò che accade lì è privato e nascosto per i clienti del
ristorante, ma i manager dell'ufficio possono vedere e fare tutto nel ristorante
che gestiscono.

Rust ha scelto che far funzionare il sistema dei moduli in questo modo,
nascondendo i dettagli di implementazione interni come impostazione predefinita.
In questo modo, sai quali parti del codice interno puoi modificare senza
compromettere il codice esterno. Tuttavia, Rust ti offre la possibilità di
esporre le parti interne del codice dei moduli figli ai moduli genitore esterni
utilizzando la parola chiave `pub` per rendere pubblico un elemento.

### Esporre _Path_ con la Parola Chiave `pub`

Torniamo all'errore nel Listato 7-4 che ci diceva che il modulo `accoglienza` è
privato. Vogliamo che la funzione `mangiare_al_ristorante` nel modulo genitore
abbia accesso alla funzione `aggiungi_in_lista` nel modulo figlio, quindi
contrassegniamo il modulo `accoglienza` con la parola chiave `pub`, come
mostrato nel Listato 7-5.

<Listing number="7-5" file-name="src/lib.rs" caption="Dichiarare il modulo `accoglienza` come `pub` per usarlo da `mangiare_al_ristorante`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs:here}}
```

</Listing>

Sfortunatamente, il codice nel Listato 7-5 genera ancora errori del compilatore,
come mostrato nel Listato 7-6.

<Listing number="7-6" caption="Errori del compilatore durante la costruzione del codice nel Listato 7-5">

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

</Listing>

Cosa è successo? Aggiungere la parola chiave `pub` davanti a `mod accoglienza`
rende il modulo pubblico. Con questa modifica, se possiamo accedere a `sala`,
possiamo accedere a `accoglienza`. Ma i _contenuti_ di `accoglienza` sono ancora
privati; rendere pubblico il modulo non rende pubblici i suoi contenuti. La
parola chiave `pub` su un modulo consente solo al codice nei suoi moduli
genitore di fare riferimento ad esso, non di accedere al suo codice interno.
Poiché i moduli sono contenitori, non possiamo fare molto semplicemente rendendo
pubblico il modulo; dobbiamo andare oltre e scegliere di rendere pubblici uno o
più degli elementi all'interno del modulo.

Gli errori nel Listato 7-6 dicono che la funzione `aggiungi_in_lista` è privata.
Le regole di _privacy_ si applicano a _struct_, _enum_, funzioni e metodi, così
come ai moduli.

Facciamo in modo che anche la funzione `aggiungi_in_lista` sia pubblica
aggiungendo la parola chiave `pub` prima della sua definizione, come nel Listato
7-7.

<Listing number="7-7" file-name="src/lib.rs" caption="Aggiungere la parola chiave `pub` a `mod accoglienza` e `fn aggiungi_in_lista` ci consente di chiamare la funzione da `mangiare_al_ristorante`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs:here}}
```

</Listing>

Ora il codice compilerà! Per capire perché aggiungere la parola chiave `pub` ci
consente di utilizzare questi percorsi in `mangiare_al_ristorante` rispetto alle
regole di _privacy_, diamo un'occhiata ai percorsi assoluti e relativi.

Nel percorso assoluto, iniziamo con `crate`, la radice dell'albero dei moduli
del nostro _crate_. Il modulo `sala` è definito nella radice del _crate_. Anche
se `sala` non è pubblico, poiché la funzione `mangiare_al_ristorante` è definita
nello stesso modulo di `sala` (cioè, `mangiare_al_ristorante` e `sala` sono
_fratelli_), possiamo fare riferimento a `sala` da `mangiare_al_ristorante`.
Successivamente, c'è il modulo `accoglienza` contrassegnato con `pub`. Possiamo
accedere al modulo genitore di `accoglienza`, quindi possiamo accedere a
`accoglienza`. Infine, la funzione `aggiungi_in_lista` è contrassegnata con
`pub` e possiamo accedere al suo modulo genitore, quindi questa chiamata di
funzione funziona!

Nel percorso relativo, la logica è la stessa del percorso assoluto, tranne per
il primo passaggio: invece di partire dalla radice del crate, il percorso inizia
da `sala`. Il modulo `sala` è definito all'interno dello stesso modulo di
`mangiare_al_ristorante`, quindi il percorso relativo che inizia dal modulo in
cui è definita `mangiare_al_ristorante` funziona. Poi, poiché `accoglienza` e
`aggiungi_in_lista` sono contrassegnati con `pub`, il resto del percorso
funziona, e questa chiamata di funzione è valida!

Se prevedi di condividere il tuo _crate libreria_ affinché altri progetti
possano utilizzare il tuo codice, la tua API pubblica è il tuo contratto con gli
utenti del tuo crate che determina come possono interagire con il tuo codice. Ci
sono molte considerazioni relative alla gestione delle modifiche alla tua API
pubblica per facilitare la dipendenza delle persone dal tuo _crate_. Queste
considerazioni vanno oltre l'ambito di questo libro; se sei interessato a questo
argomento, consulta [Le Linee Guida per l'API di Rust][api-guidelines].

> #### Migliori Pratiche per Pacchetti con un Binario e una Libreria
>
> Abbiamo menzionato che un pacchetto può contenere sia una radice di _crate
> binario_ _src/main.rs_ che una radice di _crate libreria_ _src/lib.rs_, e
> entrambi i _crate_ avranno il nome del pacchetto come impostazione
> predefinita. Tipicamente, i pacchetti che contengono sia una libreria che un
> _crate binario_ avranno nel _crate binario_ il codice strattamente necessario
> ad avviare un eseguibile che chiama il codice definito nel _crate libreria_.
> Questo consente ad altri progetti di beneficiare della maggior parte delle
> funzionalità che il pacchetto fornisce, poiché il codice del _crate libreria_
> può essere condiviso.
>
> L'albero dei moduli dovrebbe essere definito in _src/lib.rs_. Quindi,
> qualsiasi elemento pubblico può essere utilizzato nel _crate binario_ facendo
> iniziare i percorsi con il nome del pacchetto. Il _crate binario_ diventa un
> utilizzatore del _crate libreria_ proprio come un crate completamente esterno
> utilizzerebbe il _crate libreria_: può utilizzare solo l'API pubblica. Questo
> ti aiuta a progettare una buona API; non solo sei l'autore, ma sei anche un
> cliente!
>
> Nel [Capitolo 12][ch12]<!-- ignore -->, dimostreremo questa pratica
> organizzativa con un programma da riga di comando che conterrà sia un _crate
> binario_ che un _crate libreria_.

### Iniziare _Path_ Relative con `super`

Possiamo costruire percorsi relativi che iniziano nel modulo genitore, piuttosto
che nel modulo corrente o nella radice del crate, utilizzando `super` all'inizio
del percorso. Questo è simile a iniziare un percorso del filesystem con la
sintassi `..` che significa andare nella directory genitore. Utilizzare `super`
ci consente di fare riferimento a un elemento che sappiamo essere nel modulo
genitore, il che può rendere più facile riorganizzare l'albero dei moduli quando
il modulo è strettamente correlato al genitore, ma il genitore potrebbe essere
spostato altrove nell'albero dei moduli in futuro.

Considera il codice nel Listato 7-8 che modella la situazione in cui un cuoco
corregge un ordine errato e lo porta personalmente al cliente. La funzione
`correzione_ordine` definita nel modulo `cucine` chiama la funzione
`servi_ordine` definita nel modulo genitore specificando il percorso per
`servi_ordine`, iniziando con `super`.

<Listing number="7-8" file-name="src/lib.rs" caption="Chiamare una funzione utilizzando un percorso relativo che inizia con `super`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

</Listing>

La funzione `correzione_ordine` si trova nel modulo `cucine`, quindi possiamo
usare `super` per andare al modulo genitore di `cucine`, che in questo caso è
`crate`, la radice. Da lì, cerchiamo `servi_ordine` e lo troviamo. Successo!
Pensiamo che il modulo `cucine` e la funzione `servi_ordine` siano probabilmente
destinati a rimanere nella stessa relazione l'uno con l'altro e verranno
spostati insieme se decidiamo di riorganizzare l'albero dei moduli del crate.
Pertanto,abbiamo usato `super` in modo da avere meno posti da aggiornare nel
codice in futuro se questo codice viene spostato in un modulo diverso.

### Rendere Pubbliche _Struct_ e _Enum_

Possiamo anche utilizzare `pub` per designare _struct_ ed _enum_ come pubblici,
ma ci sono alcuni dettagli aggiuntivi nell'uso di `pub` con _struct_ ed _enum_.
Se utilizziamo `pub` prima di una definizione di _struct_, rendiamo la _struct_
pubblica, ma i campi della _struct_ rimarranno privati (come per i moduli).
Possiamo rendere pubblici o meno ciascun campo caso per caso. Nel Listato 7-9,
abbiamo definito una _struct_ pubblica `cucine::Colazione` con un campo pubblico
`toast` ma un campo privato `frutta_di_stagione`. Questo modella il caso in un
ristorante in cui il cliente può scegliere il tipo di pane che accompagna un
pasto, ma il cuoco decide quale frutta accompagna il pasto in base a ciò che è
di stagione e disponibile. La frutta disponibile cambia rapidamente, quindi i
clienti non possono scegliere la frutta o vedere quale frutta riceveranno.

<Listing number="7-9" file-name="src/lib.rs" caption="Una _struct_ con alcuni campi pubblici e alcuni campi privati">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

</Listing>

Poiché il campo `toast` nella _struct_ `cucine::Colazione` è pubblico, in
`mangiare_al_ristorante` possiamo scrivere e leggere il campo `toast`
utilizzando la notazione a punto. Nota che non possiamo utilizzare il campo
`frutta_di_stagione` in `mangiare_al_ristorante`, perché `frutta_di_stagione` è
privato. Prova a decommentare la riga che modifica il valore del campo
`frutta_di_stagione` per vedere quale errore ottieni!

Inoltre, nota che poiché `cucine::Colazione` ha un campo privato, la _struct_
deve fornire una funzione associata pubblica che costruisce un'istanza di
`Colazione` (qui l'abbiamo chiamata `estate`). Se `Colazione` non avesse una
funzione del genere, non potremmo creare un'istanza di `Colazione` in
`mangiare_al_ristorante` perché non potremmo impostare il valore del campo
privato `frutta_di_stagione` in `mangiare_al_ristorante`.

Al contrario, se rendiamo un _enum_ pubblico, tutte le sue varianti diventano
pubbliche. Abbiamo bisogno solo di `pub` prima della parola chiave `enum`, come
mostrato nel Listato 7-10.

<Listing number="7-10" file-name="src/lib.rs" caption="Designare un _enum_ come pubblico rende pubbliche tutte le sue varianti.">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

</Listing>

Poiché abbiamo reso pubblico l'_enum_ `Antipasti`, possiamo utilizzare le
varianti `Zuppa` e `Insalata` in `mangiare_al_ristorante`.

Gli _enum_ non sono molto utili a meno che le loro varianti non siano pubbliche;
sarebbe fastidioso dover annotare tutte le varianti degli _enum_ con `pub` una
ad una, quindi la norma per le varianti degli _enum_ è essere pubbliche. Le
_struct_ sono spesso utili senza che i loro campi siano pubblici, quindi i campi
delle _struct_ seguono la regola generale che tutto è privato come impostazione
predefinita, a meno che non sia annotato con `pub`.

C'è un'ultima situazione che coinvolge `pub` che non abbiamo trattato, ed è la
nostra ultima caratteristica del sistema dei moduli: la parola chiave `use`.
Tratteremo `use` da solo prima, e poi mostreremo come combinare `pub` e `use`.

[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#esporre-path-con-la-parola-chiave-pub
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-an-io-project.html
