## Sviluppo delle Funzionalità della Libreria con il Test-Driven Development


Ora che abbiamo la logica di ricerca in _src/lib.rs_ separata dalla funzione
`main`, è molto più facile scrivere test per le funzionalità principali del
nostro codice. Possiamo chiamare le funzioni direttamente con vari argomenti e
controllare i valori di ritorno senza dover chiamare il nostro binario dalla
riga di comando.

In questa sezione, aggiungeremo la logica di ricerca al programma `minigrep`
utilizzando il processo di _sviluppo guidato dai test_ (_test-driven
development_, abbreviato _TDD_) con i seguenti passaggi:

1. Scrivere un test che fallisce ed eseguirlo per assicurarsi che fallisca per
   il motivo previsto.
2. Scrivere o modificare solo il codice necessario per far passare il nuovo
   test.
3. Ristrutturare il codice appena aggiunto o modificato e assicurarsi che i
   test continuino a passare.
4. Ripetere dal passaggio 1!

Sebbene sia solo uno dei tanti modi per scrivere software, il TDD può aiutare
a guidare la progettazione del codice. Scrivere il test prima di scrivere il
codice che lo supera aiuta a mantenere un'elevata copertura dei test durante
l'intero processo.

Testeremo l'implementazione della funzionalità che effettivamente eseguirà la
ricerca della stringa di query nel contenuto del file e produrrà un elenco di
righe che corrispondono alla query. Aggiungeremo questa funzionalità in una
funzione chiamata `cerca`.

### Scrivere un Test che Fallisce

In _src/lib.rs_, aggiungeremo un modulo `tests` con una funzione di test, come
abbiamo fatto nel [Capitolo 11][ch11-anatomy]<!-- ignore -->. La funzione di
test specifica il comportamento che vogliamo che abbia la funzione `cerca`:
accetterà una query e il testo in cui cercare e ritornerà solo le righe del
testo che contengono la query. Il Listato 12-15 mostra questo test.

<Listing number="12-15" file-name="src/lib.rs" caption="Creazione di un test che fallisce per la funzione `cerca` per la funzionalità che vorremmo implementare">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```

</Listing>

Questo test cerca la stringa `"dut"`. Il testo che stiamo cercando è composto
da tre righe, solo una delle quali contiene `"dut"` (nota che la barra
rovesciata dopo le virgolette doppie di apertura indica a Rust di non inserire
un carattere di nuova riga all'inizio del contenuto di questa stringa
letterale). Affermiamo che il valore restituito dalla funzione `cerca`
contiene solo la riga che ci aspettiamo.

Se eseguiamo questo test, al momento fallirà perché la macro `unimplemented!`
si blocca con il messaggio "not implemented" (_non implementato_). In
conformità con i principi TDD, aggiungeremo solo il codice necessario per
evitare che il test vada in panico quando si chiama la funzione, definendo la
funzione `cerca` in modo che ritorni sempre un vettore vuoto, come mostrato
nel Listato 12-16. Quindi il test dovrebbe compilare e fallire perché un
vettore vuoto non corrisponde a un vettore contenente la riga `"sicuro,
veloce, produttivo."`

<Listing number="12-16" file-name="src/lib.rs" caption="Definire solo una parte sufficiente della funzione `cerca` in modo che chiamarla non provochi _panic_">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```

</Listing>

Ora discutiamo perché è necessario esplicitare la longevità `'a` nella firma
di `cerca` e utilizzare tale longevità con l'argomento `contenuto` e con il
valore di ritorno. Ricorda che nel [Capitolo 10][ch10-lifetimes]<!-- ignore
--> i parametri di _lifetime_ specificano quale _lifetime_ dell'argomento è
collegata a quella del valore di ritorno. In questo caso, indichiamo che il
vettore restituito deve contenere _slice_ di stringa che fanno riferimento
alla _slice_ dell'argomento `contenuto` (piuttosto che all'argomento `query`).

In altre parole, diciamo a Rust che i dati restituiti dalla funzione `cerca`
rimarranno validi finché saranno validi i dati passati alla funzione `cerca`
nell'argomento `contenuto`. Questo è importante! I dati referenziati da una
_slice_ devono essere validi affinché il _reference_ sia valido; se il
compilatore presume che stiamo creando _slice_ di `query` anziché di
`contenuto`, eseguirà i suoi controlli di sicurezza in modo errato.

Se dimentichiamo le annotazioni di longevità e proviamo a compilare questa
funzione, otterremo questo errore:

```console
{{#include ../listings/ch12-an-io-project/output-only-02-missing-lifetimes/output.txt}}
```

Rust non può sapere quale dei due parametri ci serve per l'output, quindi
dobbiamo indicarlo esplicitamente. Nota che il testo di aiuto suggerisce di
specificare lo stesso parametro di longevità per tutti i parametri e il _type_
di output, il che è sbagliato! Poiché `contenuto` è il parametro che contiene
tutto il nostro testo e vogliamo restituire le parti di quel testo che
corrispondono, sappiamo che `contenuto` è l'unico parametro che dovrebbe
essere collegato al valore di ritorno utilizzando la sintassi di longevità.

Altri linguaggi di programmazione non richiedono di collegare gli argomenti ai
valori di ritorno nella firma, ma questa pratica diventerà più semplice col
tempo. Potresti confrontare questo esempio con gli esempi nella sezione
["Validazione dei _Reference_ con le
_Lifetime_"][validating-references-with-lifetimes]<!-- ignore --> nel Capitolo
10.

### Scrivere Codice per Superare il Test

Attualmente, il nostro test fallisce perché restituisce sempre un vettore
vuoto. Per risolvere il problema e implementare `cerca`, il nostro programma
deve seguire questi passaggi:

1. Iterare ogni riga del contenuto.
2. Verificare che la riga contenga la nostra stringa di query.
3. In caso affermativo, aggiungerla all'elenco dei valori ritornati.
4. In caso contrario, non fare nulla.
5. Ritornare l'elenco dei risultati corrispondenti.

Esaminiamo ogni passaggio, iniziando con l'iterazione delle righe.

#### Iterazione delle Righe con il Metodo `lines`

Rust dispone di un metodo utile per gestire l'iterazione riga per riga delle
stringhe, opportunamente chiamato `lines`, che funziona come mostrato nel
Listato 12-17. Nota che questo non verrà ancora compilato.

<Listing number="12-17" file-name="src/lib.rs" caption="Iterazione di ogni riga in `contenuto`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```

</Listing>

Il metodo `lines` restituisce un iteratore. Parleremo degli iteratori in modo
approfondito nel [Capitolo 13][ch13-iterators]<!-- ignore -->, ma ricorda che
hai visto questo modo di usare un iteratore nel [Listato 3-5][ch3-iter]<!--
ignore -->, dove abbiamo usato un ciclo `for` con un iteratore per eseguire
del codice su ogni elemento di una collezione.

#### Ricerca della Query in Ogni Riga

Successivamente, controlleremo se la riga corrente contiene la nostra stringa
di query. Fortunatamente, le stringhe hanno un metodo utile chiamato
`contains` che fa proprio questo per noi! Aggiungiamo una chiamata al metodo
`contains` nella funzione `cerca`, come mostrato nel Listato 12-18. Nota che
questo non verrà ancora compilato.

<Listing number="12-18" file-name="src/lib.rs" caption="Aggiunta di funzionalità per verificare se la riga contiene la stringa in `query`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```

</Listing>

Al momento, stiamo sviluppando la funzionalità. Per compilare il codice,
dobbiamo restituire un valore dal corpo, come indicato nella firma della
funzione.

#### Memorizzazione delle Righe Corrispondenti

Per completare questa funzione, abbiamo bisogno di un modo per memorizzare le
righe corrispondenti che vogliamo restituire. Per farlo, possiamo creare un
vettore mutabile prima del ciclo `for` e chiamare il metodo `push` per
memorizzare una `line` nel vettore. Dopo il ciclo `for`, ritorniamo il
vettore, come mostrato nel Listato 12-19.

<Listing number="12-19" file-name="src/lib.rs" caption="Memorizzazione delle righe corrispondenti in modo da poterle ritornare">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```

</Listing>

Ora la funzione `cerca` dovrebbe restituire solo le righe che contengono
`query`, e il nostro test dovrebbe essere superato. Eseguiamo il test:

```console
{{#include ../listings/ch12-an-io-project/listing-12-19/output.txt}}
```

Il nostro test è stato superato, quindi sappiamo che funziona!

A questo punto, potremmo valutare l'opportunità di ristrutturare e migliorare
l'implementazione della funzione di ricerca, controllando che i test
continuino a passare per mantenere la stessa funzionalità. Il codice nella
funzione di ricerca non è male, ma non sfrutta alcune utili funzionalità degli
iteratori. Torneremo su questo esempio nel [Capitolo 13][ch13-iterators]<!--
ignore -->, dove esploreremo gli iteratori in dettaglio e vedremo come
migliorarli.

Ora l'intero programma dovrebbe funzionare! Proviamolo, prima con una parola
che dovrebbe restituire esattamente una riga della poesia di Emily Dickinson:
_rana_.

```console
{{#include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```

Fantastico! Ora proviamo una parola che corrisponda a più righe, come _uno_:

```console
{{#include ../listings/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```

E infine, assicuriamoci di non ottenere alcuna riga quando cerchiamo una
parola che non è presente da nessuna parte nella poesia, come
_monomorfizzazione_:

```console
{{#include ../listings/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```

Ottimo! Abbiamo creato la nostra versione in miniatura di uno strumento
classico e abbiamo imparato molto su come strutturare le applicazioni. Abbiamo
anche imparato qualcosa sull'input e l'output dei file, sui _lifetime_, sui
test e sull'analisi della riga di comando.

Per completare questo progetto, mostreremo brevemente come lavorare con le
variabili d'ambiente e come stampare su standard error, entrambi utili quando
si scrivono programmi da riga di comando.

[validating-references-with-lifetimes]: ch10-03-lifetime-syntax.html#validazione-dei-reference-con-le-lifetime
[ch11-anatomy]: ch11-01-writing-tests.html#anatomia-di-una-funzione-di-test
[ch10-lifetimes]: ch10-03-lifetime-syntax.html
[ch13-iterators]: ch13-02-iterators.html
[ch3-iter]: ch03-05-control-flow.html#eseguire-un-ciclo-su-una-collezione-con-for
