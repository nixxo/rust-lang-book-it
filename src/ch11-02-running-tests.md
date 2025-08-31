## Controllare Come Vengono Eseguiti i Test

Così come `cargo run` compila il tuo codice e poi esegue il binario risultante,
`cargo test` compila il tuo codice in modalità test ed esegue il binario
risultante. Il comportamento predefinito del binario prodotto da `cargo test` è
quello di eseguire tutti i test in parallelo e di catturare l'output generato
durante l'esecuzione dei test, impedendo la visualizzazione dell'output e
rendendo più facile la lettura dell'output relativo ai risultati dei test. Puoi,
tuttavia, specificare alcune opzioni della riga di comando per modificare questo
comportamento predefinito.

Alcune opzioni della riga di sono per `cargo test`, mentre altre vengono
passater al binario di test risultante. 

Per separare questi due tipi di argomenti, devi elencare gli argomenti che vanno
a `cargo test` seguiti dal separatore `--` e poi quelli che vanno al binario di
test. Eseguendo `cargo test --help` vengono visualizzate le opzioni che puoi
usare con `cargo test`, mentre eseguendo `cargo test -- --help` vengono
visualizzate le opzioni che puoi usare dopo il separatore. Queste opzioni sono
documentate anche nella [sezione "Tests"][tests] del [libro di rustc][rustc].

[tests]: https://doc.rust-lang.org/rustc/tests/index.html
[rustc]: https://doc.rust-lang.org/rustc/index.html

### Eseguire i Test in Parallelo o Sequenzialmente

Quando esegui più test, come impostazione predefinita questi vengono eseguiti in
parallelo utilizzando i _thread_, il che significa che finiscono di essere
eseguiti più velocemente e che ricevi più rapidamente un feedback. Poiché i test
vengono eseguiti contemporaneamente, devi assicurarti che i tuoi test non
dipendano l'uno dall'altro o da un qualsivoglia stato condiviso, incluso un
ambiente condiviso, come la directory di lavoro corrente o le variabili
d'ambiente.

Ad esempio, supponiamo che ogni test esegua del codice che crea un file su disco
chiamato _test-output.txt_ e scrive alcuni dati in quel file. Poi ogni test
legge i dati in quel file e verifica che il file contiene un particolare valore,
che è diverso in ogni test. Poiché i test vengono eseguiti contemporaneamente,
un test potrebbe sovrascrivere il file nel tempo che intercorre tra la scrittura
e la lettura del file da parte di un altro test. Il secondo test fallirà, non
perché il codice non è corretto, ma perché i test hanno interferito l'uno con
l'altro durante l'esecuzione in parallelo. Una soluzione può essere
nell'assicurarsi che ogni test scriva in un file diverso; un'altra soluzione
consiste nell'eseguire i test uno alla volta.

Se non vuoi eseguire i test in parallelo o se vuoi un controllo più preciso sul
numero di _thread_ utilizzati, puoi inviare il flag `--test-threads` e il numero
di thread che vuoi utilizzare al binario di test. Guarda il seguente esempio:

```console
$ cargo test -- --test-threads=1
```

Impostiamo il numero di _thread_ di test a `1`, indicando al programma di non
utilizzare alcun parallelismo. L'esecuzione dei test con un solo _thread_
richiederà più tempo rispetto all'esecuzione in parallelo, ma i test non
interferiranno l'uno con l'altro se condividono lo stato.

### Mostrare l'Output Della Funzione

Per impostazione predefinita, se un test viene superato, la libreria di test di
Rust cattura tutto ciò che viene stampato sullo standard output. Ad esempio, se
chiamiamo `println!` in un test e il test viene superato, non vedremo l'output
`println!` nel terminale; vedremo solo la riga che indica che il test è stato
superato. Se un test fallisce, vedremo tutto ciò che è stato stampato sullo
standard output con il resto del messaggio di fallimento.

Come esempio, il listato 11-10 contiene una funzione stupida che stampa il
valore del suo parametro e restituisce 10, oltre a un test che passa e uno che
fallisce.

<Listing number="11-10" file-name="src/lib.rs" caption="Test per una funzione che chiama `println!`">

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-10/src/lib.rs}}
```

</Listing>

When we run these tests with `cargo test`, we’ll see the following output:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-10/output.txt}}
```

Nota che in nessun punto di questo output vediamo `Ho ricevuto il valore 4`, che
viene stampato quando viene eseguito il test che passa. Quell'output è stato
catturato. L'output del test che è fallito, `Ho ricevuto il valore 8`, appare
nella sezione dell'output di riepilogo del test, che mostra anche la causa del
fallimento del test.

Se vogliamo vedere anche i valori stampati per i test superati, possiamo dire a
Rust di mostrare anche l'output dei test riusciti con `--show-output`:

```console
$ cargo test -- --show-output
```

Quando eseguiamo nuovamente i test del Listato 11-10 con il flag
`--show-output`, vediamo il seguente output:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-01-show-output/output.txt}}
```

### Eseguire un Sottoinsieme di Test in Base al Nome

A volte, l'esecuzione di un'intera suite di test può richiedere molto tempo. Se
stai lavorando sul codice di una particolare area, potresti voler eseguire solo
i test relativi a quel codice. Puoi scegliere quali test eseguire passando a
`cargo test` il nome o i nomi dei test che vuoi eseguire come argomento.

Per dimostrare come eseguire un sottoinsieme di test, creeremo prima tre test
per la nostra funzione `aggiungi_due`, come mostrato nel Listato 11-11, e
sceglieremo quali eseguire.

<Listing number="11-11" file-name="src/lib.rs" caption="Tre test con tre nomi diversi">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-11/src/lib.rs}}
```

</Listing>

Se eseguiamo i test senza passare alcun argomento, come abbiamo visto in
precedenza, tutti i test verranno eseguiti in parallelo:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-11/output.txt}}
```

#### Eseguire un Singolo Test

Possiamo passare il nome di qualsiasi funzione di test a `cargo test` per
eseguire solo quel test:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-02-single-test/output.txt}}
```

Solo il test con il nome `cento` è stato eseguito; gli altri due test non
corrispondevano a quel nome. L'output del test ci fa sapere che ci sono altri
test che non sono stati eseguiti mostrando `2 filtered out` alla fine.

Non possiamo specifare più di un nome in questo modo; verrà utilizzato solo il
primo valore dato a `cargo test`. Esiste però un modo per eseguire più test.

#### Filtrare Per Eseguire Più Test

Possiamo specificare una parte del nome di un test e ogni test il cui nome
corrisponde a quel valore verrà eseguito. Ad esempio, poiché due dei nomi dei
nostri test contengono `somma`, possiamo eseguirli eseguendo `cargo test somma`:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-03-multiple-tests/output.txt}}
```

Questo comando ha eseguito tutti i test con `somma` nel nome e ha filtrato il
test chiamato `cento`. Nota anche che il modulo in cui appare un test diventa
parte del nome del test, quindi possiamo eseguire tutti i test di un modulo
filtrando sul nome del modulo.

### Ignorare Alcuni Test Se Non Specificamente Richiesti

A volte alcuni test specifici possono richiedere molto tempo per essere
eseguiti, quindi potresti volerli escludere durante la maggior parte delle
esecuzioni di `cargo test`. Invece di elencare come argomenti tutti i test che
vuoi eseguire, puoi annotare i test che richiedono molto tempo utilizzando
l'attributo `ignore` per escluderli, come mostrato qui:

<span class="filename">File: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/src/lib.rs:here}}
```

Dopo `#[test]`, aggiungiamo la riga `#[ignore]` al test che vogliamo escludere. Ora quando eseguiamo i nostri test, `it_works` viene eseguito, ma `test_impegnativo` no:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/output.txt}}
```

La funzione `test_impegnativo` è elencata come `ignored`. Se vogliamo eseguire solo i test ignorati, possiamo usare `cargo test -- --ignored`:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-04-running-ignored/output.txt}}
```

Controllando quali test vengono eseguiti, puoi assicurarti che i risultati del
tuo `cargo test` vengano restituiti rapidamente. Quando ha senso controllare i
risultati dei test `ignorati` e hai il tempo di aspettare i risultati, puoi
invece eseguire `cargo test -- --ignored`. Se vuoi eseguire tutti i test,
indipendentemente dal fatto che siano ignorati o meno, puoi eseguire `cargo test
-- --include-ignored`
