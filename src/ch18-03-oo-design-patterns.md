## Implementare un Modello di Design Orientato agli Oggetti

Lo _state pattern_ è un modello di design orientato agli oggetti. Il punto
centrale di questo modello è definire un insieme di stati che un valore può
avere internamente. Gli stati sono rappresentati da un insieme di _oggetti
stato_, e il comportamento del valore cambia in base allo stato in cui si trova.
Vedremo un esempio con una _struct_ di un _post_ del blog che ha un campo per
mantenere il suo stato, che può essere uno stato “bozza”, “in revisione” o
“pubblicato”.

Gli oggetti stato condividono la funzionalità: in Rust, ovviamente, usiamo
_struct_ e _trait_ invece di oggetti e ereditarietà. Ogni oggetto stato è
responsabile del proprio comportamento e di decidere quando deve cambiare stato.
Il valore che contiene l’oggetto stato non sa nulla del comportamento specifico
degli stati o di quando avvengono le transizioni tra stati.

Il vantaggio dello _state pattern_ è che, quando cambiano i requisiti del
programma, non serve modificare il codice del valore che contiene lo stato né
quello che usa il valore. Basterà aggiornare il codice dentro uno degli oggetti
stato per cambiare le regole o aggiungere nuovi stati.

Inizieremo implementando lo _state pattern_ in un modo più tradizionalmente
orientato agli oggetti, poi vedremo un approccio più naturale in Rust.
Cominciamo implementando passo passo un flusso di lavoro per un _post_ del blog
usando lo _state pattern_.

La funzionalità finale sarà questa:

1. Un _post_ inizia come bozza vuota.
1. Quando la bozza è pronta, si richiede la revisione del _post_.
1. Quando il _post_ è approvato, viene pubblicato.
1. Solo i _post_ pubblicati restituiscono contenuto da stampare, in modo che i
   _post_ non approvati non possano essere pubblicati accidentalmente.

Qualsiasi altra modifica tentata su un _post_ non avrà effetto. Per esempio, se
proviamo ad approvare una bozza prima di richiedere la revisione, il _post_
resterà una bozza non pubblicata.

### Tentativo in Tradizionale Stile Orientato agli Oggetti

Ci sono infiniti modi per strutturare il codice per risolvere lo stesso
problema, ciascuno con compromessi diversi. Questa implementazione è più in uno
stile orientato agli oggetti tradizionale, possibile in Rust, ma che non sfrutta
appieno i punti di forza di Rust. Più avanti mostreremo una soluzione diversa
che usa comunque lo _state pattern_ ma in modo meno familiare a chi ha
esperienza solo con OOP. Confronteremo le due soluzioni per capire i compromessi
di progettare in Rust in modo diverso da altri linguaggi.

Il Listato 18-11 mostra questo flusso di lavoro in forma di codice: un esempio
dell’uso dell’API che implementeremo nel _crate_ `blog`. Ancora non si compila
perché il _crate_ `blog` non è implementato.

<Listing number="18-11" file-name="src/main.rs" caption="Codice che dimostra il comportamento che vogliamo per il _crate_ `blog`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-11/src/main.rs:all}}
```

</Listing>

Vogliamo permettere all’utente di creare una nuova bozza con `Post::new`.
Vogliamo permettere di aggiungere testo al _post_. Se proviamo a richiedere
subito il contenuto del _post_, prima dell’approvazione, non dobbiamo ricevere
alcun testo perché il _post_ è ancora una bozza. Abbiamo aggiunto `assert_eq!`
come dimostrazione, che in un test potrebbe verificare che il metodo `contenuto`
di una bozza restituisca una stringa vuota, ma non scriveremo test in questo
esempio.

Vogliamo poi abilitare la richiesta di revisione, e `contenuto` deve restituire
una stringa vuota durante l’attesa di revisione. Quando il _post_ riceve
l’approvazione, viene pubblicato e il testo sarà disponibile quando chiamiamo
`contenuto`.

Nota che l’unico _type_ con cui interagiamo dal _crate_ è `Post`. Questo _type_
userà lo _state pattern_ e conterrà un valore che sarà uno tra tre oggetti
stato: bozza, revisione o pubblicato. Il passaggio da uno stato all’altro sarà
gestito internamente da `Post`. Gli stati cambiano rispondendo ai metodi
chiamati dall’utente sull’istanza di `Post`, ma l’utente non gestisce
direttamente le transizioni. Inoltre, l’utente non può sbagliare gli stati, per
esempio pubblicando senza revisione.

#### Definire `Post` e Creare una Nuova Istanza

Cominciamo l’implementazione della libreria! Sappiamo di aver bisogno di una
_struct_ pubblica `Post` che tenga un contenuto, quindi partiamo dalla
definizione della _struct_ e da una funzione associata pubblica `new` per creare
istanze di `Post`, come mostrato nel Listato 18-12. Creeremo anche un _trait_
privato `Stato` che definisce il comportamento che tutti gli oggetti stato per
`Post` devono avere.

`Post` terrà un oggetto _trait_ `Box<dyn Stato>` dentro un `Option<T>` in un
campo privato chiamato `stato` per memorizzare l’oggetto stato. Vediamo dopo
perché serve `Option<T>`.

<Listing number="18-12" file-name="src/lib.rs" caption="Definizione di una _struct_ `Post`, con funzione `new` che crea una nuova istanza di `Post`, un _trait_ `Stato`, e una _struct_ `Bozza`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-12/src/lib.rs}}
```

</Listing>

Il _trait_ `Stato` definisce il comportamento condiviso tra gli stati del
_post_. Gli oggetti stato sono `Bozza`, `AttesaRevisione` e `Pubblicato`, e
implementeranno tutti il _trait_ `Stato`. Per ora il _trait_ non ha alcun
metodo; inizieremo definendo `Bozza` perché è lo stato iniziale del _post_.

Quando creiamo un nuovo `Post`, impostiamo il campo `stato` con `Some` che
contiene una `Box` che punta a un’istanza della _struct_ `Bozza`. Questo
assicura che quando creiamo una nuova istanza di `Post`, inizia sempre come
bozza. Poiché il campo `stato` è privato, non si può creare un `Post` in altri
stati! La funzione `Post::new` imposta il campo `contenuto` come una `String`
vuota.

#### Memorizzare il Testo del Post

Abbiamo visto nel Listato 18-11 che vogliamo la possibilità di chiamare un
metodo `aggiungi_testo` che prende un `&str` e lo aggiunge al contenuto del
_post_. Lo facciamo come metodo, non esponendo il campo `contenuto` come `pub`,
così poi possiamo in seguito implementare un metodo per controllare come leggere
`contenuto`. Il metodo `aggiungi_testo` è semplice; lo aggiungiamo nel blocco
`impl Post` come nel Listato 18-13.

<Listing number="18-13" file-name="src/lib.rs" caption="Implementazione del metodo `aggiungi_testo` per aggiungere testo al `contenuto` del _post_">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-13/src/lib.rs:here}}
```

</Listing>

Il metodo `aggiungi_testo` prende un _reference_ mutabile a `self` perché stiamo
modificando l’istanza `Post` su cui chiamiamo il metodo `aggiungi_testo`.
Chiamiamo poi `push_str` sulla stringa `contenuto` aggiungendovi `testo`. Questo
comportamento non dipende dallo stato in cui si trova il _post_, quindi non fa
parte dello _state pattern_. Il metodo `aggiungi_testo` non interagisce affatto
con il campo `stato`, ma fa parte del comportamento che vogliamo supportare.

#### Garantire che il Contenuto di una Bozza sia Vuoto

Anche dopo aver chiamato `aggiungi_testo` aggiungendo del contenuto al nostro
_post_, vogliamo che il metodo `contenuto` ritorni una _slice_ vuota perché il
_post_ è ancora una bozza, come mostrato dal primo `assert_eq!` nel Listato
18-11. Per ora implementiamo il metodo `contenuto` con la cosa più semplice che
possa soddisfare questo requisito: restituire semplicemente una _slice_ vuota.
Lo cambieremo in seguito quando aggiungeremo la possibilità di cambiare stato
per la pubblicazione. Per ora, i _post_ possono essere solo bozze, e quindi il
contenuto del _post_ è sempre vuoto. Il Listato 18-14 mostra questa
implementazione temporanea.

<Listing number="18-14" file-name="src/lib.rs" caption="Implementazione temporanea del metodo `contenuto` di `Post` che restituisce sempre una _slice_ vuota">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-14/src/lib.rs:here}}
```

</Listing>

Con l’aggiunta del metodo `contenuto`, tutto nel Listato 18-11 fino al primo
`assert_eq!` funziona come previsto.

#### Richiedere una Revisione, Che Cambia lo Stato del Post

Ora dobbiamo aggiungere la funzionalità per richiedere una revisione di un
_post_, che dovrebbe cambiare il suo stato da `Bozza` a `AttesaRevisione`. Il
Listato 18-15 mostra questo codice.

<Listing number="18-15" file-name="src/lib.rs" caption="Implementazione dei metodi `richiedi_revisione` su `Post` e il _trait_ `Stato`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-15/src/lib.rs:here}}
```

</Listing>

Diamo a `Post` un metodo pubblico chiamato `richiedi_revisione` che prende un
_reference_ mutabile a `self`. Poi chiamiamo un metodo interno
`richiedi_revisione` sullo stato corrente di `Post`, e questo secondo metodo
consuma lo stato corrente e restituisce un nuovo stato.

Aggiungiamo il metodo `richiedi_revisione` al _trait_ `Stato`; tutti i _type_
che implementano il _trait_ dovranno implementare questo metodo. Nota che invece
di avere `self`, `&self` o `&mut self` come primo parametro del metodo, abbiamo
`self: Box<Self>`. Questa sintassi significa che il metodo è valido solo
chiamandolo su una `Box` che contiene il _type_. Questa sintassi prende
_ownership_ di `Box<Self>`, invalidando il vecchio stato in modo che il valore
di stato del `Post` possa trasformarsi in un nuovo stato.

Per consumare il vecchio stato, il metodo `richiedi_revisione` prende
_ownership_ del valore di stato. Qui entra in gioco l’`Option` nel campo `stato`
di `Post`: chiamiamo il metodo `take` per estrarre il valore `Some` dal campo
`stato` e sostituirlo con un `None` al suo posto, perché Rust non permette campi
non popolati nelle _struct_. Questo ci permette di spostare il valore `stato`
fuori da `Post` invece di prenderlo in prestito. Poi assegniamo al campo `stato`
del _post_ il risultato di questa operazione.

Dobbiamo impostare temporaneamente `stato` a `None` invece di assegnarlo
direttamente con codice come `self.stato = self.stato.richiedi_revisione();` per
ottenere la _ownership_ del valore `stato`. Questo evita che `Post` usi il
vecchio stato dopo averlo trasformato.

Il metodo `richiedi_revisione` su `Bozza` restituisce una nuova istanza
incapsulata in `Box` di una nuova _struct_ `AttesaRevisione`, che rappresenta lo
stato di un _post_ in attesa di revisione. Anche la _struct_ `AttesaRevisione`
implementa il metodo `richiedi_revisione` ma senza trasformazioni, semplicemente
restituisce sé stessa perché richiedere una revisione su un _post_ già in stato
`AttesaRevisione` lo mantiene nello stesso stato.

Qui si cominciano a comprendere i vantaggi dello _state pattern_: il metodo
`richiedi_revisione` su `Post` è lo stesso qualunque sia il valore di `stato`.
Ogni stato gestisce le sue regole.

Lasciamo il metodo `contenuto` su `Post` così com’è, che restituisce una _slice_
di stringa vuota. Ora possiamo avere un `Post` sia nello stato `AttesaRevisione`
sia nello stato `Bozza`, ma vogliamo lo stesso comportamento in entrambi gli
stati. Il Listato 18-11 funziona ora fino al secondo `assert_eq!`!

#### Aggiungere `approva` per Cambiare il Comportamento di `contenuto`

Il metodo `approva` sarà simile a `richiedi_revisione`: imposterà `stato` al
valore che lo stato corrente dice debba avere quando è stato approvato, come
mostrato nel Listato 18-16.

<Listing number="18-16" file-name="src/lib.rs" caption="Implementazione del metodo `approva` su `Post` e il _trait_ `Stato`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-16/src/lib.rs:here}}
```

</Listing>

Aggiungiamo il metodo `approva` al _trait_ `Stato` e una nuova _struct_ che
implementa `Stato`, lo stato `Pubblicato`.

Simile a come funziona `richiedi_revisione` su `AttesaRevisione`, se chiamiamo
il metodo `approva` su una `Bozza`, non avrà effetto perché `approva` restituirà
`self`. Quando chiamiamo `approva` su `AttesaRevisione`, restituisce una nuova
istanza incapsulata in `Box` di `Pubblicato`. La _struct_ `Pubblicato`
implementa il _trait_ `Stato`, e sia per `richiedi_revisione` che per `approva`
restituisce se stessa perché il _post_ dovrebbe rimanere nello stato
`Pubblicato` in quei casi.

Ora dobbiamo aggiornare il metodo `contenuto` su `Post`. Vogliamo che il valore
restituito da `contenuto` dipenda dallo stato corrente di `Post`, quindi faremo
in modo che `Post` deleghi a un metodo `contenuto` definito sul suo `stato`,
come mostrato nel Listato 18-17.

<Listing number="18-17" file-name="src/lib.rs" caption="Aggiornamento del metodo `contenuto` su `Post` per delegare a un metodo `contenuto` su `Stato`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-17/src/lib.rs:here}}
```

</Listing>

Poiché l’obiettivo è tenere tutte queste regole dentro le _struct_ che
implementano `Stato`, chiamiamo un metodo `contenuto` sul valore in `stato`
passando l’istanza del _post_ (`self`) come argomento. Quindi restituiamo il
valore restituito dall’uso del metodo `contenuto` sul valore `stato`.

Chiamiamo il metodo `as_ref` sull’`Option` perché vogliamo un _reference_ al
valore dentro l’`Option` piuttosto che la _ownership_ del valore. Poiché `stato`
è un `Option<Box<dyn Stato>>`, chiamando `as_ref` otteniamo un `Option<&Box<dyn
Stato>>`. Se non chiamassimo `as_ref`, otterremmo un errore perché non possiamo
spostare `stato` fuori dal parametro in prestito `&self` della funzione.

Chiamiamo poi il metodo `unwrap`, che sappiamo non andrà mai in _panic_ perché i
metodi su `Post` assicurano che `stato` conterrà sempre un valore `Some` quando
quei metodi sono terminati. Questo è uno di quei casi discussi in [“Quando Hai
Più Informazioni Del Compilatore”][more-info-than-rustc]<!-- ignore --> nel
Capitolo 9 quando sappiamo che un valore `None` è impossibile, anche se il
compilatore non è in grado di inferirlo.

A questo punto, quando chiamiamo `contenuto` su `&Box<dyn Stato>`, la
de-referenziazione forzata agirà su `&` e `Box` così che il metodo `contenuto`
sarà chiamato sul _type_ che implementa il _trait_ `Stato`. Ciò significa che
dobbiamo aggiungere `contenuto` alla definizione del _trait_ `Stato` e lì
metteremo la logica per cosa restituire in base allo stato, come mostrato nel
Listato 18-18.

<Listing number="18-18" file-name="src/lib.rs" caption="Aggiunta del metodo `contenuto` al _trait_ `Stato`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-18/src/lib.rs:here}}
```

</Listing>

Aggiungiamo un’implementazione di default per il metodo `contenuto` che
restituisce una _slice_ di stringa vuota. Ciò significa che non dobbiamo
implementare `contenuto` nelle _struct_ `Bozza` e `AttesaRevisione`. La _struct_
`Pubblicato` sovrascriverà il metodo `contenuto` e restituirà il valore in
`post.contenuto`. Anche se comodo, avere il metodo `contenuto` su `Stato` che
determina il contenuto di `Post` sfuma i confini tra la responsabilità di
`Stato` e quella di `Post`.

Nota che abbiamo bisogno di annotazioni di _lifetime_ su questo metodo, come
discusso nel Capitolo 10. Stiamo prendendo un _reference_ a un `post` come
argomento e restituiamo un _reference_ ad una parte di quel `post`, quindi la
longevità del _reference_ restituito è legata alla longevità dell’argomento
`post`.

E abbiamo finito: tutto il Listato 18-11 ora funziona! Abbiamo implementato lo
_state pattern_ con le regole del flusso di lavoro del blog. La logica relativa
alle regole vive negli oggetti di stato invece di essere sparsa in `Post`.

> #### Perché Non un’_enum_?
>
> Potresti chiederti perché non abbiamo usato un’_enum_ con i diversi possibili
> stati del _post_ come varianti. È certamente una soluzione possibile; provalo
> e confronta i risultati finali per vedere cosa preferisci! Uno svantaggio
> dell’uso di un’_enum_ è che ogni posto che verifica il valore dell’_enum_ avrà
> bisogno di un’espressione `match` o simile per gestire ogni variante
> possibile. Questo potrebbe diventare più ripetitivo rispetto a questa
> soluzione con un oggetto _trait_.

#### Valutazione dello _State Pattern_

Abbiamo mostrato che Rust è capace di implementare lo _state pattern_ orientato
agli oggetti per incapsulare i diversi tipi di comportamento che un _post_
dovrebbe avere in ogni stato. I metodi su `Post` non sanno nulla dei vari
comportamenti. Siccome abbiamo organizzando il codice in questo modo, dobbiamo
guardare in un solo posto per conoscere i diversi modi in cui un _post_
pubblicato può comportarsi: l’implementazione del _trait_ `Stato` sulla _struct_
`Pubblicato`.

Se creassimo un’implementazione alternativa che non usasse lo _state pattern_,
potremmo invece usare espressioni `match` nei metodi su `Post` o anche nel
codice `main` che verifica lo stato del _post_ e cambia comportamento in quei
posti. Ciò significherebbe dover guardare in più posti per capire tutte le
implicazioni di un _post_ che è nello stato “pubblicato”.

Con lo _state pattern_, i metodi `Post` e i posti dove usiamo `Post` non hanno
bisogno di espressioni `match`, e per aggiungere un nuovo stato dovremmo solo
aggiungere una nuova _struct_ e implementare i metodi del _trait_ su quella
_struct_ in un solo punto.

L’implementazione usando lo _state pattern_ è facile da estendere per aggiungere
più funzionalità. Per vedere la semplicità di mantenere codice che usa lo _state
pattern_, prova ad implementare qualcuna di queste proposte:

- Aggiungi un metodo `respingi` che cambia lo stato del _post_ da
  `AttesaRevisione` a `Bozza`.
- Richiedi due chiamate al metodo `approva` prima che lo stato possa essere
  cambiato in `Pubblicato`.
- Permetti agli utenti di aggiungere testo al contenuto solo quando il _post_ è
  nello stato `Bozza`. Suggerimento: fai in modo che l’oggetto stato sia
  responsabile di cosa può cambiare del contenuto ma non responsabile di
  modificare `Post`.

Un lato negativo dello _state pattern_ è che, siccome gli stati implementano le
transizioni tra stati, alcuni stati sono accoppiati tra loro. Se aggiungessimo
un altro stato tra `AttesaRevisione` e `Pubblicato`, come `Programmato`,
dovremmo modificare il codice in `AttesaRevisione` per passare a `Programmato`.
Sarebbe meno lavoro se `AttesaRevisione` non dovesse cambiare con l’aggiunta di
un nuovo stato, ma ciò significherebbe passare a un altro modello di design.

Un altro lato negativo è che abbiamo duplicato un po' di logica. Per eliminare
la duplicazione, potremmo provare a fare implementazioni predefinite per i
metodi `richiedi_revisione` e `approva` sul _trait_ `Stato` che restituiscono
`self`. Tuttavia, questo non funzionerebbe: quando usiamo `Stato` come oggetto
_trait_, il _trait_ non conosce esattamente il _type_ concreto di `self`, quindi
il _type_ di ritorno non è noto durante la compilazione. (Questa è una delle
regole di compatibilità `dyn` menzionate prima.)

Altra duplicazione è nelle implementazioni simili dei metodi
`richiedi_revisione` e `approva` su `Post`. Entrambi i metodi usano
`Option::take` con il campo `stato` di `Post`, e se `stato` è `Some`, delegano
all’implementazione del metodo con lo stesso nome del valore incapsulato e
impostano il nuovo valore del campo `stato` al risultato. Se avessimo molti
metodi su `Post` che seguono questo schema, potremmo considerare di definire una
macro per eliminare la ripetizione (vedi la sezione [“Macro”][macros]<!-- ignore
--> nel Capitolo 20).

Implementando lo _state pattern_ esattamente come definito per i linguaggi
orientati agli oggetti, non sfruttiamo appieno i punti di forza di Rust come
potremmo. Vediamo qualche cambiamento da fare al _crate_ `blog` che può
trasformare stati e transizioni invalide in errori durante la compilazione.

### Codifica di Stati e Comportamenti Come _Type_

Ti mostreremo come ripensare lo _state pattern_ per ottenere un diverso set di
compromessi. Invece di incapsulare completamente gli stati e le transizioni così
che il codice esterno non ne sappia nulla, codificheremo gli stati in _type_
differenti. Di conseguenza, il sistema di controllo dei _type_ di Rust impedirà
tentativi di usare _post_ in bozze dove sono permessi solo _post_ pubblicati,
generando un errore di compilazione.

Consideriamo la prima parte di `main` nel Listato 18-11:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-11/src/main.rs:here}}
```

</Listing>

Continuiamo a permettere la creazione di nuovi _post_ nello stato bozza usando
`Post::new` e la possibilità di aggiungere testo al contenuto del _post_. Ma
invece di avere un metodo `contenuto` su un _post_ bozza che restituisce una
stringa vuota, facciamo in modo che i _post_ bozza non abbiano affatto il metodo
`contenuto`. In questo modo, se proviamo a ottenere il contenuto di un _post_
bozza, otterremo un errore di compilazione che ci dice che il metodo non esiste.
Di conseguenza, sarà impossibile mostrare accidentalmente il contenuto di un
_post_ bozza in produzione perché quel codice nemmeno si compila. Il Listato
18-19 mostra la definizione di una _struct_ `Post` e una `PostBozza`, oltre ai
metodi su ciascuna.

<Listing number="18-19" file-name="src/lib.rs" caption="Un `Post` con un metodo `contenuto` e un `PostBozza` senza metodo `contenuto`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-19/src/lib.rs}}
```

</Listing>

Sia le _struct_ `Post` che `PostBozza` hanno un campo privato `contenuto` che
memorizza il testo del _post_. Le _struct_ non hanno più il campo `stato` perché
stiamo spostando la codifica dello stato nei _type_ delle _struct_. La _struct_
`Post` rappresenta un _post_ pubblicato e ha un metodo `contenuto` che
restituisce il contenuto.

Abbiamo ancora una funzione `Post::new`, ma invece di restituire un’istanza di
`Post`, restituisce un’istanza di `PostBozza`. Poiché `contenuto` è privato e
non ci sono funzioni che restituiscono `Post`, al momento non è possibile creare
un’istanza di `Post`.

La _struct_ `PostBozza` ha un metodo `aggiungi_testo`, quindi possiamo
aggiungere testo a `contenuto` come prima, ma nota che `PostBozza` non ha un
metodo `contenuto` definito! Quindi ora il programma assicura che tutti i _post_
inizino come bozze, e i _post_ bozza non hanno il loro contenuto disponibile per
la visualizzazione. Qualsiasi tentativo di aggirare questi vincoli causerà un
errore di compilazione.

Come facciamo allora a ottenere un _post_ pubblicato? Vogliamo imporre la regola
che un _post_ bozza deve essere revisionato e approvato prima di poter essere
pubblicato. Un _post_ nello stato di attesa di revisione non dovrebbe comunque
mostrare contenuti. Implementiamo questi vincoli aggiungendo un’altra _struct_,
`PostAttesaRevisione`, definendo il metodo `richiedi_revisione` su `PostBozza`
che restituisce un `PostAttesaRevisione` e un metodo `approva` su
`PostAttesaRevisione` che restituisce un `Post`, come mostrato nel Listato
18-20.

<Listing number="18-20" file-name="src/lib.rs" caption="Un `PostAttesaRevisione` creato chiamando `richiedi_revisione` su `PostBozza` e un metodo `approva` che trasforma un `PostAttesaRevisione` in un `Post` pubblicato">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-20/src/lib.rs:here}}
```

</Listing>

I metodi `richiedi_revisione` e `approva` prendono _ownership_ di `self`,
consumando così le istanze `PostBozza` e `PostAttesaRevisione` trasformandole
rispettivamente in un `PostAttesaRevisione` e un `Post` pubblicato. In questo
modo non avremo istanze residue di `PostBozza` dopo aver chiamato
`richiedi_revisione` su di loro, e così via. La _struct_ `PostAttesaRevisione`
non ha un metodo `contenuto` definito, quindi tentare di leggere il suo
contenuto causa un errore di compilazione, come per `PostBozza`. Poiché l’unico
modo per ottenere un’istanza di `Post` pubblicato che ha un metodo `contenuto`
definito è chiamare `approva` su un `PostAttesaRevisione`, e l’unico modo per
ottenere un `PostAttesaRevisione` è chiamare `richiedi_revisione` su un
`PostBozza`, abbiamo ora codificato il flusso di lavoro del blog col sistema dei
_type_.

Dobbiamo anche fare qualche piccolo cambiamento in `main`. I metodi
`richiedi_revisione` e `approva` restituiscono nuove istanze invece di
modificare la _struct_ su cui sono chiamati, quindi dobbiamo aggiungere più
assegnazioni di _shadowing_ `let post =` per salvare le istanze restituite. Non
possiamo nemmeno avere le asserzioni sui contenuti vuoti dei _post_ bozza e
revisione pendente, né ne abbiamo bisogno: non possiamo più compilare codice che
tenta di usare il contenuto dei _post_ in quegli stati. Il codice aggiornato in
`main` è mostrato nel Listato 18-21.

<Listing number="18-21" file-name="src/main.rs" caption="Modifiche a `main` per usare la nuova implementazione del flusso di lavoro del blog">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-21/src/main.rs}}
```

</Listing>

I cambiamenti necessari per riassegnare `post` significano che questa
implementazione non segue più esattamente lo _state pattern_ orientato agli
oggetti: le trasformazioni tra gli stati non sono più completamente incapsulate
nella implementazione di `Post`. Tuttavia, abbiamo guadagnato che stati invalidi
ora sono impossibili grazie al sistema dei _type_ e al controllo durante la
compilazione! Questo assicura che alcuni bug, come la visualizzazione del
contenuto di un _post_ non pubblicato, vengano scoperti prima che arrivino in
produzione.

Prova a realizzare i compiti suggeriti all’inizio di questa sezione sul _crate_
`blog` così com’è dopo il Listato 18-21 per vedere cosa pensi del design di
questa versione del codice. Nota che alcune attività potrebbero essere già
implementate con questo design.

Abbiamo visto che anche se Rust è capace di implementare modelli di design
orientati agli oggetti, altri modelli, come la codifica dello stato nel sistema
dei _type_, sono disponibili in Rust. Questi modelli hanno diversi compromessi.
Anche se potresti essere molto familiare con i modelli orientati agli oggetti,
ripensare il problema per sfruttare le caratteristiche di Rust può offrire
benefici, come prevenire alcuni bug già in fase di compilazione. I modelli
orientati agli oggetti non saranno sempre la miglior soluzione in Rust a causa
di alcune caratteristiche come la _ownership_, che i linguaggi orientati agli
oggetti non hanno.

## Riepilogo

Indipendentemente dal fatto che tu pensi che Rust sia un linguaggio orientato
agli oggetti dopo aver letto questo capitolo, ora sai che puoi usare oggetti
_trait_ per ottenere alcune funzionalità orientate agli oggetti in Rust. Il
_dynamic dispatch_ può dare al tuo codice un po' di flessibilità in cambio di
una piccola perdita in prestazioni durante l’esecuzione. Puoi usare questa
flessibilità per implementare modelli orientati agli oggetti che possono aiutare
nella manutenibilità del tuo codice. Rust ha anche altre caratteristiche, come
la _ownership_, che i linguaggi orientati agli oggetti non hanno. Un modello
orientato agli oggetti non sarà sempre il modo migliore per sfruttare i punti di
forza di Rust, ma è un’opzione disponibile.

In seguito parleremo di _pattern_, un’altra delle caratteristiche di Rust che
consente molta flessibilità. Li abbiamo visti brevemente in precedenza nel
libro, ma non ne abbiamo ancora visto tutto il potenziale. Cominciamo!

[more-info-than-rustc]: ch09-03-to-panic-or-not-to-panic.html#quando-hai-più-informazioni-del-compilatore
[macros]: ch20-05-macros.html#macro
