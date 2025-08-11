## Variabili e mutabilità

Come accennato nella sezione ["Memorizzare i valori con le
Variabili"][memorizzare-i-valori-con-le-variabili]<!-- ignore -->, come
impostazione predefinita, le variabili sono immutabili. Questo è uno dei tanti
stimoli che Rust ti dà per scrivere il tuo codice in modo da sfruttare la
sicurezza e la facilità di concorrenza che Rust offre. Tuttavia, hai ancora la
possibilità di rendere le tue variabili mutabili. Esploriamo come e perché Rust
ti incoraggia a favorire l'immutabilità e perché a volte potresti voler
rinunciare.

Quando una variabile è immutabile, una volta che un valore è legato a un nome,
non è più possibile cambiarlo. Per vederlo con mano, genera un nuovo progetto
chiamato _variabili_ nella tua cartella _progetti_ utilizzando `cargo new
variabili`.

Ed ora, nella nuova cartella _variabili_, apri _src/main.rs_ e sostituisci il
suo codice con il seguente, che per il momento risulterà non compilabile:

<span class="filename">File: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

Salva ed esegui il programma utilizzando `cargo run`. Dovresti ricevere un
messaggio di errore relativo a un errore di immutabilità, come mostrato in
questo output:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```
Questo esempio mostra come il compilatore ti aiuta a trovare gli errori nei tuoi
programmi. Gli errori del compilatore possono essere frustranti, ma in realtà
significano solo che il tuo programma non sta ancora facendo in modo sicuro ciò
che vuoi; non significano che non sei un buon programmatore! Anche ai Rustaceani
più esperti escon fuori errori del compilatore.

Hai ricevuto il messaggio di errore `` cannot assign twice to immutable variable
`` perché hai cercato di assegnare un secondo valore alla variabile immutabile
`x`.

È importante che ci vengano segnalati errori in tempo di compilazione quando si
tenta di modificare un valore che è stato definito immutabile, perché proprio
questa situazione può portare a dei bug. Se una parte del nostro codice opera
sulla base del presupposto che un valore non cambierà mai e un'altra parte del
codice modifica quel valore, è possibile che la prima parte del codice non
faccia ciò per cui è stata progettata. La causa di questo tipo di bug può essere
difficile da rintracciare a posteriori, soprattutto quando la seconda parte del
codice modifica il valore solo _qualche volta_. Il compilatore di Rust
garantisce che quando si afferma che un valore non cambierà, non cambierà
davvero, quindi non dovrai tenerne traccia tu stesso. Il tuo codice sarà quindi
più facile da analizzare.

Ma la mutabilità può essere molto utile e può rendere il codice più comodo da
scrivere. Sebbene le variabili siano immutabili come impostazione predefinita,
puoi renderle mutabili aggiungendo `mut` davanti al nome della variabile, come
hai fatto nel [Capitolo 2][memorizzare-i-valori-con-le-variabili]<!-- ignore
-->. L'aggiunta di `mut` rende anche palese quando si andrà a rileggere il
codice in futuro che altre parti del codice cambieranno il valore di questa
variabile.

Ad esempio, cambiamo _src/main.rs_ con il seguente:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

Quando eseguiamo il programma ora, otteniamo questo:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

Siamo autorizzati a cambiare il valore legato a `x` da `5` a `6` quando si usa
`mut`. In definitiva, decidere se usare la mutabilità o meno dipende da te e da
ciò che ritieni più chiaro in quella particolare situazione.

### Costanti

Come le variabili immutabili, le _costanti_ sono valori legati a un nome che non
possono essere modificati, ma ci sono alcune differenze tra le costanti e le
variabili.

Innanzitutto, non puoi usare `mut` con le costanti. Le costanti non solo sono
immutabili come impostazione predefinita, sono sempre immutabili. Dichiari le
costanti usando la parola chiave `const` invece della parola chiave `let` e il
_type_ del valore _deve_ essere annotato. Tratteremo i _type_ e le annotazioni
dei _type_ nella prossima sezione, ["Datatype - Tipi di dato"][data-types]<!--
ignore -->, quindi non preoccuparti dei dettagli in questo momento. Sappi solo
che devi sempre annotare il _type_.

Le costanti possono essere dichiarate in qualsiasi _scope_, compreso quello
globale, il che le rende utili per i valori che molte parti del codice devono
conoscere.

L'ultima differenza è che le costanti possono essere impostate solo su
un'espressione costante, non sul risultato di un valore che può essere calcolato
solo in fase di esecuzione.

Ecco un esempio di dichiarazione di una costante:

```rust
const TRE_ORE_IN_SECONDI: u32 = 60 * 60 * 3;
```

Il nome della costante è `TRE_ORE_IN_SECONDI` e il suo valore è impostato come
il risultato della moltiplicazione di 60 (il numero di secondi in un minuto) per
60 (il numero di minuti in un'ora) per 3 (il numero di ore che vogliamo contare
in questo programma). La convenzione di Rust per la denominazione delle costanti
prevede l'uso di maiuscole con trattini bassi tra le parole. Il compilatore è in
grado di valutare il risultato di un'operazione in fase di compilazione, il che
ci permette di scegliere di scrivere questo valore in un modo più facile da
capire e da verificare, piuttosto che impostare questa costante al valore
10.800. Consulta la sezione [Valutazione delle costanti (in
inglese)][const-eval] per maggiori informazioni sulle operazioni che possono
essere utilizzate quando si dichiarano le costanti.

Le costanti sono valide per tutto il tempo di esecuzione di un programma,
all'interno dello _scope_ in cui sono state dichiarate. Questa proprietà rende
le costanti utili per dei valori nella tua applicazione che più parti del
programma potrebbero avere bisogno di conoscere, come ad esempio il numero
massimo di punti che un giocatore di un gioco può guadagnare o la velocità della
luce.

Dichiarare come costanti i valori codificati usati nel tuo programma è utile per
trasmettere il significato di quel valore a chi leggerà il codice in futuro.
Inoltre, è utile per avere un solo punto del codice da modificare se il valore
codificato deve essere aggiornato in futuro.

### Shadowing

Come hai visto nel tutorial sul gioco dell'indovinello nel [Capitolo
2][confrontare-lipotesi-con-il-numero-segreto]<!-- ignore -->, puoi dichiarare una
nuova variabile con lo stesso nome di una variabile precedente. I Rustaceani
dicono che la prima variabile è _messa in ombra_, _shadowing_, dalla seconda, il
che significa che la seconda variabile è quella che il compilatore vedrà quando
userai il nome della variabile. In effetti, la seconda variabile mette in ombra
la prima, portando a sé qualsiasi uso del nome della variabile fino a quando non
sarà essa stessa messa in ombra o lo _scope_ terminerà. Possiamo fare
_shadowing_ di una variabile usando lo stesso nome della variabile e ripetendo
l'uso della parola chiave `let` come segue:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

Questo programma vincola innanzitutto `x` a un valore di `5`. Poi crea una nuova
variabile `x` ripetendo `let x =`, prendendo il valore originale e aggiungendo
`1` in modo che il valore di `x` sia `6`. Quindi, all'interno di uno _scope_
interno creato con le parentesi graffe, la terza istruzione `let` _mette in
ombra_ `x` e crea una nuova variabile, moltiplicando il valore precedente per
`2` per dare a `x` un valore di `12`. Quando lo _scope_ termina, finisce pure lo
_shadowing_ e `x` torna a essere `6`. Quando si esegue questo programma, si
ottiene il seguente risultato:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

Lo _shadowing_ è diverso dall'indicare una variabile come `mut` perché otterremo
un errore in fase di compilazione se cerchassimo accidentalmente di riassegnare
questa variabile senza usare la parola chiave `let`. Usando `let`, possiamo
eseguire alcune trasformazioni su un valore ma far sì che la variabile sia
immutabile dopo che le trasformazioni sono state completate.

L'altra differenza tra `mut` e lo _shadowing_ è che, poiché stiamo
effettivamente creando una nuova variabile quando usiamo di nuovo la parola
chiave `let`, possiamo cambiare il _type_ del valore ma riutilizzare lo stesso
nome. Ad esempio, supponiamo che il nostro programma chieda a un utente di
mostrare quanti spazi vuole tra un testo e l'altro inserendo dei caratteri di
spazio, e poi vogliamo memorizzare questo input come un numero:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

La prima variabile `spazi` è di _type_ stringa e la seconda variabile `spazi` è
di _type_ numerico. Lo _shadowing_ ci evita quindi di dover inventare nomi
diversi, come `spazi_str` e `spazi_num`; possiamo invece riutilizzare il nome
più semplice `spazi`. Tuttavia, se proviamo a usare `mut` per fare questa cosa,
come mostrato qui, otterremo un errore di compilazione:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

L'errore dice che non è consentito mutare il _type_ di una variabile:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

Ora che abbiamo visto il funzionamento delle variabili, passiamo in rassegna le
varie tipologie di dato, _type_, che possono essere.

[confrontare-lipotesi-con-il-numero-segreto]: ch02-00-guessing-game-tutorial.html#confrontare-lipotesi-con-il-numero-segreto
[data-types]: ch03-02-data-types.html#datatype---tipi-di-dato
[memorizzare-i-valori-con-le-variabili]: ch02-00-guessing-game-tutorial.html#memorizzare-i-valori-con-le-variabili
[const-eval]: https://doc.rust-lang.org/stable/reference/const_eval.html
