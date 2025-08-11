## Flusso di controllo

La possibilità di eseguire del codice a seconda che una condizione sia `vera` e
di eseguire ripetutamente del codice finché una data condizione è `vera` sono
elementi fondamentali della maggior parte dei linguaggi di programmazione. I
costrutti più comuni che ti permettono di controllare il flusso di esecuzione
del codice di Rust sono le espressioni `if` e i loop.

### L'espressione `if`

Un'espressione `if` (`se` in italiano) ti permette di ramificare il tuo codice a
seconda delle condizioni. Fornisci una condizione e poi dici: "Se questa
condizione è soddisfatta, esegui questo blocco di codice. Se la condizione non è
soddisfatta, non eseguire questo blocco di codice"

Crea un nuovo progetto chiamato _ramificazioni_ nella tua directory _progetti_
per sperimentare con l'espressione `if`. Nel file _src/main.rs_, inserisci
quanto segue:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

Tutte le espressioni `if` iniziano con la parola chiave `if`, seguita da una
condizione. In questo caso, la condizione verifica se la variabile `numero` ha o
meno un valore inferiore a 5. Il blocco di codice da eseguire se la condizione è
`true` viene posizionato subito dopo la condizione, all'interno di parentesi
graffe. I blocchi di codice associati alle condizioni nelle espressioni `if`
possono esser viste come dei _rami_, proprio come i _rami_ nelle espressioni
`match` di cui abbiamo parlato nella sezione ["Confrontare l'ipotesi con il
numero segreto"][confrontare-lipotesi-con-il-numero-segreto]<!-- ignore --> del
Capitolo 2.

Opzionalmente, possiamo anche includere un'espressione `else` (_altrimenti_ in
italiano), come abbiamo scelto di fare in questo caso, per dare al programma un
blocco di codice alternativo da eseguire nel caso in cui la condizione sia
valutata `false`. Se non fornisci un'espressione `else` e la condizione è
`false`, il programma salterà il blocco `if` e passerà alla parte di codice
successiva.

Prova a eseguire questo codice; dovresti vedere il seguente risultato:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

Proviamo a cambiare il valore di `numero` con un valore che renda la condizione
`false` per vedere cosa succede:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

Esegui nuovamente il programma e guarda l'output:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

Vale anche la pena di notare che la condizione in questo codice _deve_ essere un
`bool`. Se la condizione non è un `bool`, otterremo un errore. Ad esempio, prova
a eseguire il seguente codice:

<span class="filename">File: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

Questa volta la condizione `if` valuta un valore di `3` e Rust lancia un errore:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

L'errore indica che Rust si aspettava un `bool` ma ha ottenuto un numero intero.
A differenza di linguaggi come Ruby e JavaScript, Rust non cercherà
automaticamente di convertire i _type_ non booleani in booleani. Devi essere
esplicito e fornire sempre un `if` con un booleano come condizione. Se vogliamo
che il blocco di codice `if` venga eseguito solo quando un numero non è uguale a
`0`, ad esempio, possiamo modificare l'espressione `if` nel seguente modo:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

L'esecuzione di questo codice stamperà `numero era qualcosa di diverso da zero`.

#### Gestione di condizioni multiple con `else if`

Puoi utilizzare condizioni multiple combinando `if` e `else` in un'espressione `else if`. Ad esempio:

<span class="filename">File: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

Questo programma ha quattro possibili _rami_. Dopo averlo eseguito, dovresti vedere il seguente output:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```
Quando questo programma viene eseguito, controlla ogni espressione `if` a turno
ed esegue il primo corpo per il quale la condizione è valutata `true`. Nota che
anche se 6 è divisibile per 2, non vediamo l'output `numero è divisibile per 2`,
né vediamo il testo `numero non è divisibile per 4, 3 o 2` del blocco `else`.
Questo perché Rust esegue il blocco solo per la prima condizione `true` e una
volta che ne trova una, le restanti non vengono controllate.

L'uso di troppe espressioni `else if` può rendere il codice un po' confusionario
e difficile da leggere, quindi se ne hai più di una, potresti valutare di
riscrivere il codice. Il Capitolo 6 descrive un potente costrutto di
ramificazione di Rust chiamato `match` per gestire casi del genere.

#### Utilizzo di `if` in una dichiarazione `let`

Dato che `if` è un'espressione, possiamo usarla a destra di una dichiarazione
`let` per assegnare il risultato a una variabile, come nel Listato 3-2.

<Listing number="3-2" file-name="src/main.rs" caption="Assegnazione del risultato di un'espressione `if` as una variabile">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

</Listing>

La variabile `numero` sarà legata a un valore basato sul risultato
dell'espressione `if`. Esegui questo codice per vedere cosa succede:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

Ricorda che i blocchi di codice valutano l'ultima espressione in essi contenuta
e i numeri da soli sono anch'essi espressioni. In questo caso, il valore
dell'intera espressione `if` dipende da quale blocco di codice viene eseguito.
Ciò significa che i valori che possono essere i risultati di ogni _ramo_ di `if`
devono essere dello stesso tipo; nel Listato 3-2, i risultati sia del _ramo_
`if` che del _ramo_ `else` erano numeri interi `i32`. Se i _type_ non sono
corrispondenti, come nell'esempio seguente, otterremo un errore:

<span class="filename">File: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

Quando proviamo a compilare questo codice, otterremo un errore: i _rami_ `if` e
`else` hanno _type_ incompatibili e Rust indica esattamente dove trovare il
problema nel programma:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

L'espressione nel blocco `if` ritorna un _integer_ e l'espressione nel blocco
`else` ritorna una stringa. Questo non funziona perché le variabili devono avere
un _type_ univoco e Rust ha bisogno di sapere in fase di compilazione che di che
_type_ è la variabile `numero`, in modo definitivo. Conoscere il _type_ di
`numero` permette al compilatore di verificare che il _type_ sia valido ovunque
si utilizzi `numero`. Rust non sarebbe in grado di farlo se il _type_ di
`numero` fosse determinato solo in fase di esecuzione; il compilatore sarebbe
più complesso e darebbe meno garanzie sul codice se dovesse tenere traccia dei
più disparati _type_ possibili per ogni variabile.

### Repetition with Loops

It’s often useful to execute a block of code more than once. For this task,
Rust provides several _loops_, which will run through the code inside the loop
body to the end and then start immediately back at the beginning. To experiment
with loops, let’s make a new project called _loops_.

Rust has three kinds of loops: `loop`, `while`, and `for`. Let’s try each one.

#### Repeating Code with `loop`

The `loop` keyword tells Rust to execute a block of code over and over again
forever or until you explicitly tell it to stop.

As an example, change the _src/main.rs_ file in your _loops_ directory to look
like this:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

When we run this program, we’ll see `again!` printed over and over continuously
until we stop the program manually. Most terminals support the keyboard shortcut
<kbd>ctrl</kbd>-<kbd>c</kbd> to interrupt a program that is stuck in a continual
loop. Give it a try:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

The symbol `^C` represents where you pressed <kbd>ctrl</kbd>-<kbd>c</kbd>.

You may or may not see the word `again!` printed after the `^C`, depending on
where the code was in the loop when it received the interrupt signal.

Fortunately, Rust also provides a way to break out of a loop using code. You
can place the `break` keyword within the loop to tell the program when to stop
executing the loop. Recall that we did this in the guessing game in the
[“Uscita dopo un'ipotesi corretta”][uscita-dopo-unipotesi-corretta]<!-- ignore
--> section of Chapter 2 to exit the program when the user won the game by
guessing the correct number.

We also used `continue` in the guessing game, which in a loop tells the program
to skip over any remaining code in this iteration of the loop and go to the
next iteration.

#### Returning Values from Loops

One of the uses of a `loop` is to retry an operation you know might fail, such
as checking whether a thread has completed its job. You might also need to pass
the result of that operation out of the loop to the rest of your code. To do
this, you can add the value you want returned after the `break` expression you
use to stop the loop; that value will be returned out of the loop so you can
use it, as shown here:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

Before the loop, we declare a variable named `counter` and initialize it to
`0`. Then we declare a variable named `result` to hold the value returned from
the loop. On every iteration of the loop, we add `1` to the `counter` variable,
and then check whether the `counter` is equal to `10`. When it is, we use the
`break` keyword with the value `counter * 2`. After the loop, we use a
semicolon to end the statement that assigns the value to `result`. Finally, we
print the value in `result`, which in this case is `20`.

You can also `return` from inside a loop. While `break` only exits the current
loop, `return` always exits the current function.

#### Loop Labels to Disambiguate Between Multiple Loops

If you have loops within loops, `break` and `continue` apply to the innermost
loop at that point. You can optionally specify a _loop label_ on a loop that
you can then use with `break` or `continue` to specify that those keywords
apply to the labeled loop instead of the innermost loop. Loop labels must begin
with a single quote. Here’s an example with two nested loops:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

The outer loop has the label `'counting_up`, and it will count up from 0 to 2.
The inner loop without a label counts down from 10 to 9. The first `break` that
doesn’t specify a label will exit the inner loop only. The `break
'counting_up;` statement will exit the outer loop. This code prints:

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

#### Conditional Loops with `while`

A program will often need to evaluate a condition within a loop. While the
condition is `true`, the loop runs. When the condition ceases to be `true`, the
program calls `break`, stopping the loop. It’s possible to implement behavior
like this using a combination of `loop`, `if`, `else`, and `break`; you could
try that now in a program, if you’d like. However, this pattern is so common
that Rust has a built-in language construct for it, called a `while` loop. In
Listing 3-3, we use `while` to loop the program three times, counting down each
time, and then, after the loop, print a message and exit.

<Listing number="3-3" file-name="src/main.rs" caption="Using a `while` loop to run code while a condition evaluates to `true`">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

</Listing>

This construct eliminates a lot of nesting that would be necessary if you used
`loop`, `if`, `else`, and `break`, and it’s clearer. While a condition
evaluates to `true`, the code runs; otherwise, it exits the loop.

#### Looping Through a Collection with `for`

You can choose to use the `while` construct to loop over the elements of a
collection, such as an array. For example, the loop in Listing 3-4 prints each
element in the array `a`.

<Listing number="3-4" file-name="src/main.rs" caption="Looping through each element of a collection using a `while` loop">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

</Listing>

Here, the code counts up through the elements in the array. It starts at index
`0`, and then loops until it reaches the final index in the array (that is,
when `index < 5` is no longer `true`). Running this code will print every
element in the array:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

All five array values appear in the terminal, as expected. Even though `index`
will reach a value of `5` at some point, the loop stops executing before trying
to fetch a sixth value from the array.

However, this approach is error prone; we could cause the program to panic if
the index value or test condition is incorrect. For example, if you changed the
definition of the `a` array to have four elements but forgot to update the
condition to `while index < 4`, the code would panic. It’s also slow, because
the compiler adds runtime code to perform the conditional check of whether the
index is within the bounds of the array on every iteration through the loop.

As a more concise alternative, you can use a `for` loop and execute some code
for each item in a collection. A `for` loop looks like the code in Listing 3-5.

<Listing number="3-5" file-name="src/main.rs" caption="Looping through each element of a collection using a `for` loop">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

</Listing>

When we run this code, we’ll see the same output as in Listing 3-4. More
importantly, we’ve now increased the safety of the code and eliminated the
chance of bugs that might result from going beyond the end of the array or not
going far enough and missing some items. Machine code generated from `for`
loops can be more efficient as well, because the index doesn’t need to be
compared to the length of the array at every iteration.

Using the `for` loop, you wouldn’t need to remember to change any other code if
you changed the number of values in the array, as you would with the method
used in Listing 3-4.

The safety and conciseness of `for` loops make them the most commonly used loop
construct in Rust. Even in situations in which you want to run some code a
certain number of times, as in the countdown example that used a `while` loop
in Listing 3-3, most Rustaceans would use a `for` loop. The way to do that
would be to use a `Range`, provided by the standard library, which generates
all numbers in sequence starting from one number and ending before another
number.

Here’s what the countdown would look like using a `for` loop and another method
we’ve not yet talked about, `rev`, to reverse the range:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

This code is a bit nicer, isn’t it?

## Summary

You made it! This was a sizable chapter: you learned about variables, scalar
and compound data types, functions, comments, `if` expressions, and loops! To
practice with the concepts discussed in this chapter, try building programs to
do the following:

- Convert temperatures between Fahrenheit and Celsius.
- Generate the *n*th Fibonacci number.
- Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
  taking advantage of the repetition in the song.

When you’re ready to move on, we’ll talk about a concept in Rust that _doesn’t_
commonly exist in other programming languages: ownership.

[confrontare-lipotesi-con-il-numero-segreto]: ch02-00-guessing-game-tutorial.html#cconfrontare-lipotesi-con-il-numero-segreto
[uscita-dopo-unipotesi-corretta]: ch02-00-guessing-game-tutorial.html#uscita-dopo-unipotesi-corretta
