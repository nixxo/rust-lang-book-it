## Lettura di un File

Ora aggiungeremo la funzionalità per leggere il file specificato nell'argomento `file_path`
. Per prima cosa abbiamo bisogno di un file di esempio con cui testarlo: useremo un file con una
piccola quantità di testo su più righe con alcune parole ripetute. Il Listato 12-3
contiene una poesia di Emily Dickinson che funzionerà bene! Crea un file chiamato
_poem.txt_ alla radice del tuo progetto e inserisci la poesia "Non sono nessuno!
Chi sei tu?"

<Listing number="12-3" file-name="poem.txt" caption="Una poesia di Emily Dickinson è un buon caso di test.">

```text
{{#include ../listings/ch12-an-io-project/listing-12-03/poem.txt}}
```

</Listing>

Con il testo inserito, modifica _src/main.rs_ e aggiungi il codice per leggere il file, come
mostrato nel Listato 12-4.

<Listing number="12-4" file-name="src/main.rs" caption="Lettura del contenuto del file specificato dal secondo argomento">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/src/main.rs:here}}
```

</Listing>

Per prima cosa introduciamo una parte rilevante della libreria standard con un'istruzione `use`
: abbiamo bisogno di `std::fs` per gestire i file.

In `main`, la nuova istruzione `fs::read_to_string` accetta `file_path`, apre
quel file e restituisce un valore di tipo `std::io::Result<String>` che contiene
il contenuto del file.

Dopodiché, aggiungiamo di nuovo un'istruzione temporanea `println!` che stampa il valore
di `contents` dopo la lettura del file, in modo da poter verificare che il programma
funzioni finora.

Eseguiamo questo codice con una stringa qualsiasi come primo argomento della riga di comando (perché
non abbiamo ancora implementato la parte di ricerca) e il file _poem.txt_ come
secondo argomento:

```console
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/output.txt}}
```

Ottimo! Il codice ha letto e poi stampato il contenuto del file. Ma il codice
presenta alcuni difetti. Al momento, la funzione `main` ha più
responsabilità: in genere, le funzioni sono più chiare e facili da gestire se
ogni funzione è responsabile di una sola idea. L'altro problema è che
non gestiamo gli errori al meglio delle nostre possibilità. Il programma è ancora piccolo, quindi questi
difetti non rappresentano un grosso problema, ma man mano che il programma cresce, sarà più difficile correggerli
in modo pulito. È buona norma iniziare il refactoring fin dall'inizio
quando si sviluppa un programma, perché è molto più facile rifare piccole quantità di
codice. Lo faremo in seguito.
