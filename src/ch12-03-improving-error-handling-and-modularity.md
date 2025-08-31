## Refactoring per Migliorare la Modularità e la Gestione degli Errori

Per migliorare il nostro programma, risolveremo quattro problemi che riguardano
la struttura del programma e la gestione di potenziali errori. Innanzitutto, la nostra funzione `main`
ora esegue due attività: analizza gli argomenti e legge i file. Man mano che il nostro
programma cresce, il numero di attività separate gestite dalla funzione `main`
aumenterà. Man mano che una funzione acquisisce responsabilità, diventa più difficile
esaminare, testare e modificare senza danneggiare una delle sue
parti. È meglio separare le funzionalità in modo che ogni funzione sia responsabile di
un'attività.

Questo problema si collega anche al secondo problema: sebbene `query` e `file_path`
siano variabili di configurazione del nostro programma, variabili come `contents` vengono utilizzate
per eseguire la struttura logica del programma. Più `main` diventa lungo, più variabili
dovremo includere nello scope; più variabili abbiamo nello scope, più difficile
sarà tenere traccia dello scopo di ciascuna. È meglio raggruppare le
variabili di configurazione in un'unica struttura per chiarirne lo scopo.

Il terzo problema è che abbiamo usato `expect` per visualizzare un messaggio di errore quando
la lettura del file fallisce, ma il messaggio di errore visualizza solo `Avrei dovuto
essere in grado di leggere il file`. La lettura di un file può fallire in diversi modi: ad esempio, il file potrebbe essere mancante o potremmo non avere i permessi per aprirlo.
Al momento, indipendentemente dalla situazione, visualizzeremo lo stesso messaggio di errore per
tutto, il che non fornirebbe alcuna informazione all'utente!

In quarto luogo, usiamo `expect` per gestire un errore e, se l'utente esegue il nostro programma
senza specificare argomenti sufficienti, riceverà un errore `index out of bounds`
da Rust che non spiega chiaramente il problema. Sarebbe meglio se tutto il
codice di gestione degli errori fosse in un unico posto, in modo che i futuri manutentori abbiano un solo posto
in cui consultare il codice se la struttura di gestione degli errori dovesse cambiare. Avere tutto il codice di gestione degli errori in un unico posto garantirà anche la stampa di messaggi
comprensibili per i nostri utenti finali.

Affrontiamo questi quattro problemi riorganizzando il nostro progetto.

### Separazione delle Attività per i Progetti Binari

Il problema organizzativo di allocare la responsabilità di più attività alla
funzione `main` è comune a molti progetti binari. Di conseguenza, molti programmatori Rust
trovano utile suddividere le attività di un programma binario
quando la funzione `main` inizia a diventare più grande. Questo processo prevede i
seguenti passaggi:

- Suddividere il programma in un file _main.rs_ e un file _lib.rs_ e spostare
la logica del programma in _lib.rs_.
- Finché la logica di analisi della riga di comando è piccola, può rimanere nella
funzione `main`.
- Quando la logica di analisi della riga di comando inizia a complicarsi, estrarla
dalla funzione `main` in altre funzioni o tipi.

Le responsabilità che rimangono nella funzione `main` dopo questo processo
dovrebbero essere limitate a quanto segue:

- Chiamare la logica di analisi della riga di comando con i valori degli argomenti
- Impostare qualsiasi altra configurazione
- Chiamare una funzione `run` in _lib.rs_
- Gestire l'errore se `run` restituisce un errore

Questo schema riguarda la separazione delle attività: _main.rs_ gestisce l'esecuzione
del programma e _lib.rs_ gestisce tutta la logica dell'attività in corso. Poiché
non è possibile testare direttamente la funzione `main`, questa struttura consente di testare tutta
la logica del programma spostandola fuori dalla funzione `main`. Il codice che
rimane nella funzione `main` sarà sufficientemente piccolo da poterne verificare la correttezza
leggendolo. Rielaboriamo il nostro programma seguendo questo processo.

#### Estrazione del Parser degli Argomenti

Estrarremo la funzionalità per l'analisi degli argomenti in una funzione che
chiamerà `main`. Il Listato 12-5 mostra il nuovo avvio della funzione `main` che
chiama una nuova funzione `parse_config`, che definiremo in _src/main.rs_.

<Listing number="12-5" file-name="src/main.rs" caption="Estrazione di una funzione `parse_config` da `main`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

</Listing>

Stiamo ancora raccogliendo gli argomenti della riga di comando in un vettore, ma invece di
assegnare il valore dell'argomento all'indice 1 alla variabile `query` e il
valore dell'argomento all'indice 2 alla variabile `file_path` all'interno della funzione `main`,
passiamo l'intero vettore alla funzione `parse_config`. La funzione
`parse_config` contiene quindi la logica che determina quale argomento
andare in quale variabile e restituisce i valori a `main`. Creiamo ancora
le variabili `query` e `file_path` in `main`, ma `main` non ha più la
responsabilità di determinare come gli argomenti e le variabili della riga di comando
corrispondono.

Questa rielaborazione potrebbe sembrare eccessiva per il nostro piccolo programma, ma stiamo eseguendo il refactoring
in piccoli passaggi incrementali. Dopo aver apportato questa modifica, esegui nuovamente il programma per
verificare che l'analisi degli argomenti funzioni ancora. È consigliabile controllare spesso i progressi
per aiutare a identificare la causa dei problemi quando si verificano.

#### Raggruppamento dei Valori di Configurazione

Possiamo fare un altro piccolo passo per migliorare ulteriormente la funzione `parse_config`.
Al momento, restituiamo una tupla, ma poi la suddividiamo immediatamente
in singole parti. Questo è un segno che forse non abbiamo ancora
l'astrazione giusta.

Un altro indicatore che mostra che c'è margine di miglioramento è la parte `config`
di `parse_config`, che implica che i due valori restituiti sono correlati e
fanno entrambi parte di un unico valore di configurazione. Al momento non stiamo evidenziando questo
significato nella struttura dei dati se non raggruppando i due valori in
una tupla; inseriremo invece i due valori in un'unica struttura e daremo a ciascuno dei
campi della struttura un nome significativo. In questo modo, sarà più facile per i futuri manutentori di questo codice comprendere come i diversi valori si relazionano tra loro
e qual è il loro scopo.

Il Listato 12-6 mostra i miglioramenti alla funzione `parse_config`.

<Listing number="12-6" file-name="src/main.rs" caption="Refactoring di `parse_config` per restituire un'istanza di una struttura `Config`">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

</Listing>

Abbiamo aggiunto una struttura denominata `Config` definita per avere campi denominati `query` e
`file_path`. La firma di `parse_config` ora indica che restituisce un
valore `Config`. Nel corpo di `parse_config`, dove prima restituivamo
slice di stringa che fanno riferimento a valori `String` in `args`, ora definiamo `Config`
in modo che contenga valori `String` di proprietà. La variabile `args` in `main` è proprietaria dei
valori degli argomenti e consente solo alla funzione `parse_config` di prenderli in prestito,
il che significa che violeremmo le regole di Rust sui prestiti se `Config` tentasse di prendere
la proprietà dei valori in `args`.

Ci sono diversi modi per gestire i dati `String`; il modo più semplice,
anche se un po' inefficiente, è chiamare il metodo `clone` sui valori.
Questo creerà una copia completa dei dati per l'istanza di `Config`, che ne diverrà proprietaria,
il che richiede più tempo e memoria rispetto alla memorizzazione di un riferimento ai dati stringa.
Tuttavia, clonare i dati rende anche il nostro codice molto semplice perché
non dobbiamo gestire la lifetime dei riferimenti; in questo caso,
rinunciare a un po' di prestazioni per guadagnare semplicità è un compromesso che vale la pena accettare.

> ### I compromessi dell'Utilizzo di `Clone`
>
> Molti utenti di Rust tendono a evitare di usare `clone` per risolvere
> problemi di proprietà a causa del suo costo di esecuzione. Nel
> [Capitolo 13][ch13]<!-- ignore -->, imparerai come utilizzare metodi più efficienti
> in questo tipo di situazioni. Ma per ora, va bene copiare alcune
> stringhe per continuare, perché queste copie verranno eseguite solo
> una volta e il percorso del file e la stringa di query sono molto piccoli. È meglio avere
> un programma funzionante ma un po' inefficiente che cercare di iperottimizzare il codice
> al primo tentativo. Man mano che acquisirai esperienza con Rust, sarà
> più facile iniziare con la soluzione più efficiente, ma per ora è
> perfettamente accettabile chiamare `clone`.

Abbiamo aggiornato `main` in modo che inserisca l'istanza di `Config` restituita da
`parse_config` in una variabile denominata `config`, e abbiamo aggiornato il codice che
in precedenza utilizzava le variabili separate `query` e `file_path`, in modo che ora utilizzi
i campi della struttura `Config`.

Ora il nostro codice comunica più chiaramente da che `query` e `file_path` sono correlati e
che il loro scopo è configurare il funzionamento del programma. Qualsiasi codice che
utilizza questi valori sa come trovarli nell'istanza di `config` nei campi
denominati appositamente per il loro scopo.

#### Creazione di un Costruttore per `Config`

Finora, abbiamo estratto la logica responsabile dell'analisi degli argomenti della riga di comando
da `main` e l'abbiamo inserita nella funzione `parse_config`. In questo modo
abbiamo potuto verificare che i valori `query` e `file_path` erano correlati e che
questa relazione doveva essere comunicata nel nostro codice. Abbiamo quindi aggiunto una struttura `Config` per
denominare lo scopo correlato di `query` e `file_path` e per poter restituire i nomi dei
valori come nomi di campo della struttura dalla funzione `parse_config`.

Ora che lo scopo della funzione `parse_config` è creare un'istanza di `Config`,
possiamo modificare `parse_config` da una semplice funzione a una funzione
chiamata `new` associata alla struttura `Config`. Questa modifica
renderà il codice più idiomatico. Possiamo creare istanze di tipi nella
libreria standard, come `String`, chiamando `String::new`. Allo stesso modo,
modificando `parse_config` in una funzione `new` associata a `Config`, saremo
in grado di creare istanze di `Config` chiamando `Config::new`. Il Listato 12-7
mostra le modifiche che dobbiamo apportare.

<Numero di inserzione="12-7" nome-file="src/main.rs" didascalia="Modifica di `parse_config` in `Config::new`">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

</Listing>

We’ve updated `main` where we were calling `parse_config` to instead call
`Config::new`. We’ve changed the name of `parse_config` to `new` and moved it
within an `impl` block, which associates the `new` function with `Config`. Try
compiling this code again to make sure it works.

### Fixing the Error Handling

Now we’ll work on fixing our error handling. Recall that attempting to access
the values in the `args` vector at index 1 or index 2 will cause the program to
panic if the vector contains fewer than three items. Try running the program
without any arguments; it will look like this:

```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

The line `index out of bounds: the len is 1 but the index is 1` is an error
message intended for programmers. It won’t help our end users understand what
they should do instead. Let’s fix that now.

#### Improving the Error Message

In Listing 12-8, we add a check in the `new` function that will verify that the
slice is long enough before accessing index 1 and index 2. If the slice isn’t
long enough, the program panics and displays a better error message.

<Listing number="12-8" file-name="src/main.rs" caption="Adding a check for the number of arguments">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

</Listing>

This code is similar to [the `Guess::new` function we wrote in Listing
9-13][ch9-custom-types]<!-- ignore -->, where we called `panic!` when the
`value` argument was out of the range of valid values. Instead of checking for
a range of values here, we’re checking that the length of `args` is at least
`3` and the rest of the function can operate under the assumption that this
condition has been met. If `args` has fewer than three items, this condition
will be `true`, and we call the `panic!` macro to end the program immediately.

With these extra few lines of code in `new`, let’s run the program without any
arguments again to see what the error looks like now:

```console
{{#include ../listings/ch12-an-io-project/listing-12-08/output.txt}}
```

This output is better: we now have a reasonable error message. However, we also
have extraneous information we don’t want to give to our users. Perhaps the
technique we used in Listing 9-13 isn’t the best one to use here: a call to
`panic!` is more appropriate for a programming problem than a usage problem,
[as discussed in Chapter 9][ch9-error-guidelines]<!-- ignore -->. Instead,
we’ll use the other technique you learned about in Chapter 9—[returning a
`Result`][ch9-result]<!-- ignore --> that indicates either success or an error.

<!-- Old headings. Do not remove or links may break. -->

<a id="returning-a-result-from-new-instead-of-calling-panic"></a>

#### Returning a `Result` Instead of Calling `panic!`

We can instead return a `Result` value that will contain a `Config` instance in
the successful case and will describe the problem in the error case. We’re also
going to change the function name from `new` to `build` because many
programmers expect `new` functions to never fail. When `Config::build` is
communicating to `main`, we can use the `Result` type to signal there was a
problem. Then we can change `main` to convert an `Err` variant into a more
practical error for our users without the surrounding text about `thread
'main'` and `RUST_BACKTRACE` that a call to `panic!` causes.

Listing 12-9 shows the changes we need to make to the return value of the
function we’re now calling `Config::build` and the body of the function needed
to return a `Result`. Note that this won’t compile until we update `main` as
well, which we’ll do in the next listing.

<Listing number="12-9" file-name="src/main.rs" caption="Returning a `Result` from `Config::build`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

</Listing>

Our `build` function returns a `Result` with a `Config` instance in the success
case and a string literal in the error case. Our error values will always be
string literals that have the `'static` lifetime.

We’ve made two changes in the body of the function: instead of calling `panic!`
when the user doesn’t pass enough arguments, we now return an `Err` value, and
we’ve wrapped the `Config` return value in an `Ok`. These changes make the
function conform to its new type signature.

Returning an `Err` value from `Config::build` allows the `main` function to
handle the `Result` value returned from the `build` function and exit the
process more cleanly in the error case.

<!-- Old headings. Do not remove or links may break. -->

<a id="calling-confignew-and-handling-errors"></a>

#### Calling `Config::build` and Handling Errors

To handle the error case and print a user-friendly message, we need to update
`main` to handle the `Result` being returned by `Config::build`, as shown in
Listing 12-10. We’ll also take the responsibility of exiting the command line
tool with a nonzero error code away from `panic!` and instead implement it by
hand. A nonzero exit status is a convention to signal to the process that
called our program that the program exited with an error state.

<Listing number="12-10" file-name="src/main.rs" caption="Exiting with an error code if building a `Config` fails">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

</Listing>

In this listing, we’ve used a method we haven’t covered in detail yet:
`unwrap_or_else`, which is defined on `Result<T, E>` by the standard library.
Using `unwrap_or_else` allows us to define some custom, non-`panic!` error
handling. If the `Result` is an `Ok` value, this method’s behavior is similar
to `unwrap`: it returns the inner value that `Ok` is wrapping. However, if the
value is an `Err` value, this method calls the code in the _closure_, which is
an anonymous function we define and pass as an argument to `unwrap_or_else`.
We’ll cover closures in more detail in [Chapter 13][ch13]<!-- ignore -->. For
now, you just need to know that `unwrap_or_else` will pass the inner value of
the `Err`, which in this case is the static string `"not enough arguments"`
that we added in Listing 12-9, to our closure in the argument `err` that
appears between the vertical pipes. The code in the closure can then use the
`err` value when it runs.

We’ve added a new `use` line to bring `process` from the standard library into
scope. The code in the closure that will be run in the error case is only two
lines: we print the `err` value and then call `process::exit`. The
`process::exit` function will stop the program immediately and return the
number that was passed as the exit status code. This is similar to the
`panic!`-based handling we used in Listing 12-8, but we no longer get all the
extra output. Let’s try it:

```console
{{#include ../listings/ch12-an-io-project/listing-12-10/output.txt}}
```

Great! This output is much friendlier for our users.

<!-- Old headings. Do not remove or links may break. -->

<a id="extracting-logic-from-main"></a>

### Extracting Logic from the `main` Function

Now that we’ve finished refactoring the configuration parsing, let’s turn to
the program’s logic. As we stated in [“Separation of Concerns for Binary
Projects”](#separation-of-concerns-for-binary-projects)<!-- ignore -->, we’ll
extract a function named `run` that will hold all the logic currently in the
`main` function that isn’t involved with setting up configuration or handling
errors. When we’re done, the `main` function will be concise and easy to verify
by inspection, and we’ll be able to write tests for all the other logic.

Listing 12-11 shows the small, incremental improvement of extracting a `run`
function.

<Listing number="12-11" file-name="src/main.rs" caption="Extracting a `run` function containing the rest of the program logic">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

</Listing>

The `run` function now contains all the remaining logic from `main`, starting
from reading the file. The `run` function takes the `Config` instance as an
argument.

#### Returning Errors from the `run` Function

With the remaining program logic separated into the `run` function, we can
improve the error handling, as we did with `Config::build` in Listing 12-9.
Instead of allowing the program to panic by calling `expect`, the `run`
function will return a `Result<T, E>` when something goes wrong. This will let
us further consolidate the logic around handling errors into `main` in a
user-friendly way. Listing 12-12 shows the changes we need to make to the
signature and body of `run`.

<Listing number="12-12" file-name="src/main.rs" caption="Changing the `run` function to return `Result`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

</Listing>

We’ve made three significant changes here. First, we changed the return type of
the `run` function to `Result<(), Box<dyn Error>>`. This function previously
returned the unit type, `()`, and we keep that as the value returned in the
`Ok` case.

For the error type, we used the _trait object_ `Box<dyn Error>` (and we’ve
brought `std::error::Error` into scope with a `use` statement at the top).
We’ll cover trait objects in [Chapter 18][ch18]<!-- ignore -->. For now, just
know that `Box<dyn Error>` means the function will return a type that
implements the `Error` trait, but we don’t have to specify what particular type
the return value will be. This gives us flexibility to return error values that
may be of different types in different error cases. The `dyn` keyword is short
for _dynamic_.

Second, we’ve removed the call to `expect` in favor of the `?` operator, as we
talked about in [Chapter 9][ch9-question-mark]<!-- ignore -->. Rather than
`panic!` on an error, `?` will return the error value from the current function
for the caller to handle.

Third, the `run` function now returns an `Ok` value in the success case.
We’ve declared the `run` function’s success type as `()` in the signature,
which means we need to wrap the unit type value in the `Ok` value. This
`Ok(())` syntax might look a bit strange at first, but using `()` like this is
the idiomatic way to indicate that we’re calling `run` for its side effects
only; it doesn’t return a value we need.

When you run this code, it will compile but will display a warning:

```console
{{#include ../listings/ch12-an-io-project/listing-12-12/output.txt}}
```

Rust tells us that our code ignored the `Result` value and the `Result` value
might indicate that an error occurred. But we’re not checking to see whether or
not there was an error, and the compiler reminds us that we probably meant to
have some error-handling code here! Let’s rectify that problem now.

#### Handling Errors Returned from `run` in `main`

We’ll check for errors and handle them using a technique similar to one we used
with `Config::build` in Listing 12-10, but with a slight difference:

<span class="filename">File: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

We use `if let` rather than `unwrap_or_else` to check whether `run` returns an
`Err` value and to call `process::exit(1)` if it does. The `run` function
doesn’t return a value that we want to `unwrap` in the same way that
`Config::build` returns the `Config` instance. Because `run` returns `()` in
the success case, we only care about detecting an error, so we don’t need
`unwrap_or_else` to return the unwrapped value, which would only be `()`.

The bodies of the `if let` and the `unwrap_or_else` functions are the same in
both cases: we print the error and exit.

### Splitting Code into a Library Crate

Our `minigrep` project is looking good so far! Now we’ll split the
_src/main.rs_ file and put some code into the _src/lib.rs_ file. That way, we
can test the code and have a _src/main.rs_ file with fewer responsibilities.

Let’s define the code responsible for searching text in _src/lib.rs_ rather
than in _src/main.rs_, which will let us (or anyone else using our
`minigrep` library) call the searching function from more contexts than our
`minigrep` binary.

First, let’s define the `search` function signature in _src/lib.rs_ as shown in
Listing 12-13, with a body that calls the `unimplemented!` macro. We’ll explain
the signature in more detail when we fill in the implementation.

<Listing number="12-13" file-name="src/lib.rs" caption="Defining the `search` function in  *src/lib.rs*">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs}}
```

</Listing>

We’ve used the `pub` keyword on the function definition to designate `search`
as part of our library crate’s public API. We now have a library crate that we
can use from our binary crate and that we can test!

Now we need to bring the code defined in _src/lib.rs_ into the scope of the
binary crate in _src/main.rs_ and call it, as shown in Listing 12-14.

<Listing number="12-14" file-name="src/main.rs" caption="Using the `minigrep` library crate’s `search` function in *src/main.rs*">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

</Listing>

We add a `use minigrep::search` line to bring the `search` function from
the library crate into the binary crate’s scope. Then, in the `run` function,
rather than printing out the contents of the file, we call the `search`
function and pass the `config.query` value and `contents` as arguments. Then
`run` will use a `for` loop to print each line returned from `search` that
matched the query. This is also a good time to remove the `println!` calls in
the `main` function that displayed the query and the file path so that our
program only prints the search results (if no errors occur).

Note that the search function will be collecting all the results into a vector
it returns before any printing happens. This implementation could be slow to
display results when searching large files because results aren’t printed as
they’re found; we’ll discuss a possible way to fix this using iterators in
Chapter 13.

Whew! That was a lot of work, but we’ve set ourselves up for success in the
future. Now it’s much easier to handle errors, and we’ve made the code more
modular. Almost all of our work will be done in _src/lib.rs_ from here on out.

Let’s take advantage of this newfound modularity by doing something that would
have been difficult with the old code but is easy with the new code: we’ll
write some tests!

[ch13]: ch13-00-functional-features.html
[ch9-custom-types]: ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation
[ch9-error-guidelines]: ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling
[ch9-result]: ch09-02-recoverable-errors-with-result.html
[ch18]: ch18-00-oop.html
[ch9-question-mark]: ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
