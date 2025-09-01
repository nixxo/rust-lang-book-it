## Definire e Istanziare le _Struct_

Le _struct_ sono simili ai _tuple_, discussi nella sezione ["Il _Type_
Tupla"][tuples]<!-- ignore -->, in quanto entrambi possono contenere più valori
correlati. Come i _tuple_, i componenti di una _struct_ possono essere di _type_
diversi. A differenza dei _tuple_, in una _struct_ puoi denominare ogni pezzo di
dati in modo che sia chiaro il significato dei valori. L'aggiunta di questi nomi
significa che le _struct_ sono più flessibili dei _tuple_: non devi fare
affidamento sull'ordine dei dati per specificare o accedere ai valori di
un'istanza.

Per definire una _struct_, inseriamo la parola chiave `struct` e diamo un nome
all'intera _struct_. Il nome di una _struct_ dovrebbe descrivere il significato
dei dati raggruppati insieme. Poi, all'interno di parentesi graffe, definiamo i
nomi e i _type_ dei pezzi di dati, che chiamiamo _campi_ (_field_ in inglese).
Ad esempio, il Listato 5-1 mostra una _struct_ che memorizza informazioni su un
account utente.


<Listing number="5-1" file-name="src/main.rs" caption="Una definizione della struct `Utente`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

</Listing>

Per utilizzare una _struct_ dopo averla definita, creiamo un'_istanza_ di quella
_struct_ specificando valori concreti per ciascuno dei campi. Creiamo un'istanza
indicando il nome della _struct_ e poi aggiungendo parentesi graffe contenenti
coppie _`chiave: valore`_, dove le chiavi sono i nomi dei campi e i valori sono
i dati che vogliamo memorizzare in quei campi. Non dobbiamo specificare i campi
nello stesso ordine in cui li abbiamo dichiarati nella _struct_. In altre
parole, la definizione della _struct_ è come un modello generale per il _type_,
e le istanze riempiono quel modello con dati particolari per creare valori del
_type_. Ad esempio, possiamo dichiarare un utente particolare come mostrato nel
listato 5-2.

<Listing number="5-2" file-name="src/main.rs" caption="Creazione di un'istanza della struct `Utente`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

</Listing>

Per ottenere un valore specifico da una _struct_, usiamo la notazione col punto.
Ad esempio, per accedere all'indirizzo email di questo utente, usiamo
`utente1.email`. Se l'istanza è mutabile, possiamo cambiare un valore usando la
notazione col punto assegnando un valore a un campo in particolare. Il Listato
5-3 mostra come modificare il valore del campo `email` di un'istanza `Utente`
mutabile.

<Listing number="5-3" file-name="src/main.rs" caption="Cambiare valore del campo `email` di un'istanza `Utente`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

</Listing>

Nota che l'intera istanza deve essere mutabile; Rust non ci permette di
contrassegnare solo alcuni campi come mutabili. Come per qualsiasi espressione,
possiamo costruire una nuova istanza della _struct_ come ultima espressione nel
corpo di una funzione per restituire implicitamente quella nuova istanza.

Il Listato 5-4 mostra la funzione `nuovo_utente` che restituisce un'istanza
`Utente` con l'email e il nome utente indicati. Il campo `attivo` assume il
valore di `true` e `numero_accessi` prende il valore di `1`.

<Listing number="5-4" file-name="src/main.rs" caption="Una funzione `nuovo_utente` che prende una email e un nome utente per ritornare un'istanza `Utente`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

</Listing>

Ha senso chiamare i parametri della funzione con lo stesso nome dei campi della
_struct_, ma dover ripetere i nomi dei campi `email` e `nome_utente` e delle
variabili è un po' noioso. Se la _struct_ avesse più campi, la ripetizione di
ogni nome diventerebbe ancora più fastidiosa. Per fortuna esiste una comoda
scorciatoia!

<!-- Old heading. Do not remove or links may break. -->
<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>

### Utilizzo della Sintassi Abbreviata di Inizializzazione

Poiché i nomi dei parametri e i nomi dei campi della _struct_ sono esattamente
gli stessi, possiamo usare la sintassi di inizializzazione abbreviata dei campi
(_field init shorthand_) per riscrivere la funzione in modo che si comporti
esattamente allo stesso modo ma senza la ripetizione di `nome_utente` e `email`,
come mostrato nel Listato 5-5.

<Listing number="5-5" file-name="src/main.rs" caption="Una funzione `nuovo_utente` che usa la sintassi abbreviata perché i campi e i parametri `nome_utente` e `email` hanno lo stesso nome">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

</Listing>

Qui stiamo creando una nuova istanza della _struct_ `Utente`, che ha un campo
chiamato `email`. Vogliamo impostare il valore del campo `email` sul valore del
parametro `email` della funzione `nuovo_utente`. Dato che il campo `email` e il
parametro `email` hanno lo stesso nome, dobbiamo solo scrivere `email` invece di
`email: email`.

### Creare Istanze da Altre Istanze con la Sintassi di Aggiornamento delle _Struct_

Spesso è utile creare una nuova istanza di una _struct_ che include la maggior
parte dei valori da un'altra istanza dello stesso _type_, ma con alcune
modifiche. Puoi farlo usando la _sintassi di aggiornamento delle struct_
(_struct update_ d'ora in poi).

Per prima cosa, nel Listato 5-6 mostriamo come creare regolarmente una nuova
istanza `Utente` in `utente2`, senza la sintassi di aggiornamento. Impostiamo un
nuovo valore per `email` ma per il resto utilizziamo gli stessi valori di
`utente1` creati nel Listato 5-2.

<Listing number="5-6" file-name="src/main.rs" caption="Creazione di una nuova istanza `Utente` con gli stessi valori tranne uno di `utente1`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

</Listing>

Utilizzando la sintassi _struct update_, possiamo ottenere lo stesso effetto con
meno codice, come mostrato nel Listato 5-7. La sintassi `..` specifica che i
restanti campi non impostati esplicitamente dovrebbero avere lo stesso valore
dei campi nell’istanza data.

<Listing number="5-7" file-name="src/main.rs" caption="Utilizzo della sintassi _struct update_ per impostare un nuovo valore di `email` per un'istanza di `Utente`, ma utilizzando o restanti valori da `utente1`">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

</Listing>

Anche il codice nel Listato 5-7 crea un’istanza in `utente2` che ha un valore
diverso per `email` ma ha gli stessi valori per i campi `nome_utente`, `attivo`
e `numero_accessi` di `utente1`. La parola chiave `..utente1` deve venire per
ultima per specificare che tutti i campi rimanenti dovrebbero ottenere i propri
valori dai campi corrispondenti in `utente1`, ma possiamo scegliere di
specificare i valori per tutti i campi che vogliamo in qualsiasi ordine,
indipendentemente dall’ordine dei campi nella definizione della _struct_.

Si noti che la sintassi di _struct update_ utilizza `=` come un assegnazione;
questo perché sposta i dati, proprio come abbiamo visto nella sezione
[”Interazione tra Variabili e Dati con _Move_”][move]<!-- ignore -->. In questo
esempio, non possiamo più utilizzare `utente1` dopo aver creato `utente2` perché
la `String` nel campo `nome_utente` di `utente1` è stata spostata in `utente2`.
Se avessimo fornito a `utente2` nuovi valori `String` sia per l’`email` che per
`nome_utente` e quindi avessimo utilizzato solo i valori `attivo` e
`numero_accessi` di `utente1`, `utente1` sarebbe ancora valido dopo aver creato
`utente2`. Sia `attivo` che `numero_accessi` sono _type_ che implementano il
_trait_ `Copy`, quindi si applicherebbe il comportamento discusso nella sezione
[”Duplicazione di dati sullo _Stack_”][stack]<!-- ignore -->. In questo esempio
possiamo ancora utilizzare `utente1.email`, perché il suo valore non è stato
spostato da `utente1`.

### _Struct_ di _Tuple_ Senza Campi Denominati per Creare _Type_ Diversi

Rust supporta anche _struct_ che assomigliano alle _tuple_, chiamate _tuple
struct_. Le _tuple struct_ hanno il significato aggiuntivo che il nome della
_struct_ fornisce, ma non hanno nomi associati ai loro campi; piuttosto, hanno
solo i _type_ dei campi. Le _tuple struct_ sono utili quando si vuole dare un
nome all'intera _tuple_ e renderla un _type_ diverso da altre _tuple_, e quando
denominare ogni campo come in una _struct_ regolare sarebbe poco utile o
ridondante. Per definire una _tuple struct_, inizia con la parola chiave
`struct` e il nome della _struct_ seguito dai _type_ della _tuple_. Ad esempio,
qui definiamo e utilizziamo due _tuple struct_ chiamate `Colore` e `Punto`:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```

</Listing>

Tieni presente che i valori `nero` e `origine` sono di _type_ diverso perché
sono istanze di _tuple struct_ diverse. Ogni _struct_ che definisci diventa un
nuovo _type_ a sé stante, anche se i campi all’interno della _struct_ potrebbero
avere gli stessi _type_. Ad esempio, una funzione che accetta un parametro di
_type_ `Colore` non può accettare un `Punto` come argomento, anche se entrambi i
_type_ sono costituiti da tre valori `i32`. Oltretutto, le istanze di _tuple
struct_ sono simili alle _tuple_ in quanto puoi destrutturarle nelle loro
singole parti e puoi utilizzare un `.` seguito dall’indice per accedere a un
singolo valore. A differenza delle _tuple_ però, le _tuple struct_ richiedono di
nominare il _type_ di _struct_ quando le destrutturi. Ad esempio, scriveremo
`let Punto(x, y, z) = origine;` per destrutturare i valori del `Punto` `origine`
in variabili chiamate `x`, `y` e `z`.

### _Struct_ di _Unit_ Senza Campi

Puoi anche definire _struct_ che non hanno campi! Queste sono chiamate
_unit-like struct_ perché si comportano in modo simile a `()`, il _type_ _unit_
menzionato nella sezione ["Il _Type_ Tupla"][tuples]<!-- ignore -->. Le
_unit-like struct_ possono essere utili quando è necessario implementare un
_trait_ su un _type_ ma non si hanno dati che si vogliono memorizzare nel _type_
stesso.

Parleremo dei _trait_ nel [Capitolo 10](ch10-02-traits.html)<!-- ignore -->.
Ecco un esempio di dichiarazione e istanziazione di una _unit struct_ chiamata
`SempreUguale`:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-04-unit-like-structs/src/main.rs}}
```

</Listing>

Per definire `SempreUguale`, utilizziamo la parola chiave `struct`, il nome che
vogliamo e quindi un punto e virgola. Non c’è bisogno di parentesi graffe o
tonde! Quindi possiamo ottenere un’istanza di `SempreUguale` nella variabile
`soggetto` in un modo simile: utilizzando il nome che abbiamo definito, senza
parentesi graffe o tonde. Immagina che in seguito implementeremo il
comportamento per questo _type_ in modo tale che ogni istanza di `SempreUguale`
sia sempre uguale a ogni istanza di qualsiasi altro _type_, magari per avere un
risultato noto a scopo di test. Non avremmo bisogno di dati per implementare
quel comportamento! Vedremo nel Capitolo 10 come definire i _trait_ e
implementarli su qualsiasi _type_, comprese le _unit struct_.

> ### _Ownership_ dei Dati di _Struct_
>
> Nella definizione della _struct_ `Utente`, abbiamo utilizzato il _type_
> `String` invece del _type_ _slice_ di stringa `&str`. Questa è una scelta
> deliberata perché vogliamo che ogni istanza di questa _struct_ possieda tutti
> i suoi dati e che tali dati siano validi per tutto il tempo in cui la _struct_
> è valida.
>
> È anche possibile che le _struct_ memorizzino _reference_ a dati posseduti da
> qualcos’altro, ma per farlo è necessario l’uso di _lifetime_, una funzionalità
> di Rust di cui parleremo nel Capitolo 10. _Lifetime_ garantisce che i dati a
> cui fa _reference_ una _struct_ siano validi finché lo è la _struct_.
> Supponiamo che provi a memorizzare un _reference_ in una _struct_ senza
> specificare la _lifetime_, come nel seguente esempio; questo non funzionerà:
>
> <Listing file-name="src/main.rs">
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
> ```rust,ignore,does_not_compile
> struct Utente {
>     attivo: bool,
>     nome_utente: &str,
>     email: &str,
>     numero_accessi: u64,
> }
>
> fn main() {
>     let user1 = Utente {
>         attivo: true,
>         nome_utente: "qualcuno123",
>         email: "qualcuno@mia_mail.com",
>         numero_accessi: 1,
>     };
> }
> ```
>
> </Listing>
>
> Il compilatore si lamenterà richiedendo degli identificatori di _lifetime_:
>
> ```console
> $ cargo run
>    Compiling struct v0.1.0 (file:///progetti/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:18
>   |
> 3 |     nome_utente: &str,
>   |                  ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct Utente<'a> {
> 2 |     attivo: bool,
> 3 ~     nome_utente: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct Utente<'a> {
> 2 |     attivo: bool,
> 3 |     nome_utente: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `struct` (bin "struct") due to 2 previous errors
> ```
>
> Nel Capitolo 10, discuteremo come risolvere questi errori in modo da poter
> memorizzare _reference_ nelle _struct_, ma per ora risolveremo errori come
> questi usando _type_ con _ownership_ come `String` invece di _reference_ come
> `&str`.


<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add `> ` before every line -->

[tuples]: ch03-02-data-types.html#il-type-tupla
[move]: ch04-01-what-is-ownership.html#interazione-tra-variabili-e-dati-con-move
[stack]: ch04-01-what-is-ownership.md#duplicazione-di-dati-sullo-stack
[copy]: ch04-01-what-is-ownership.html#duplicazione-di-dati-sullo-stack
