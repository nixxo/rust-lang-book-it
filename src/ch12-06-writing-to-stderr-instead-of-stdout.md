## Scrivere i Messaggi di Errore su Standard Error invece che su Standard Output

Al momento, stiamo scrivendo tutto il nostro output sul terminale usando la macro
`println!`. Nella maggior parte dei terminali, esistono due tipi di output: _standard output_ (`stdout`) per informazioni generali e _standard error_ (`stderr`) per
i messaggi di errore. Questa distinzione consente agli utenti di scegliere di indirizzare
l'output corretto di un programma a un file, ma di visualizzare comunque i messaggi di errore
sullo schermo.

La macro `println!` è in grado di visualizzare solo sullo standard output, quindi dobbiamo
usare qualcos'altro per visualizzare sullo standard error.

### Controllare dove Vengono Scritti gli Errori

Per prima cosa osserviamo come il contenuto stampato da `minigrep` viene attualmente
scritto sullo standard output, inclusi eventuali messaggi di errore che vorremmo scrivere sullo standard error. Lo faremo reindirizzando il flusso di standard output
a un file, causando intenzionalmente un errore. Non reindirizzeremo il flusso di standard error, quindi qualsiasi contenuto inviato allo standard error continuerà a essere visualizzato sullo
schermo.

Ci si aspetta che i programmi a riga di comando inviino messaggi di errore al flusso di errore standard, in modo da poterli comunque visualizzare sullo schermo anche se reindirizziamo il flusso di output standard a un file. Il nostro programma al momento non si comporta bene:
stiamo per vedere come salvare l'output del messaggio di errore in un file!

Per dimostrare questo comportamento, eseguiremo il programma con `>` e il percorso del file,
_output.txt_, a cui vogliamo reindirizzare il flusso di output standard. Non
passeremo alcun argomento, il che dovrebbe causare un errore:

```console
$ cargo run > output.txt
```

La sintassi `>` indica alla shell di scrivere il contenuto dell'output standard su
_output.txt_ invece che sullo schermo. Non abbiamo visto il messaggio di errore che ci aspettavamo di visualizzare sullo schermo, quindi significa che deve essere finito nel
file. Ecco cosa contiene _output.txt_:

```text
Problema nella parsing degli argomenti: non ci sono abbastanza argomenti
```

Sì, il nostro messaggio di errore viene visualizzato sullo standard output. È molto più
utile che messaggi di errore come questo vengano visualizzati sullo standard error, in modo che solo
i dati di un'esecuzione senza errori finiscano nel file. Cambieremo questa impostazione.

### Visualizzazione degli Errori sullo Standard Error

Useremo il codice del Listato 12-24 per modificare la modalità di visualizzazione dei messaggi di errore.
A causa del refactoring effettuato in precedenza in questo capitolo, tutto il codice che
visualizza i messaggi di errore si trova in un'unica funzione, `main`. La libreria standard fornisce
la macro `eprintln!` che visualizza sullo standard error, quindi modifichiamo
i due punti in cui chiamavamo `println!` per visualizzare gli errori in modo che utilizzino `eprintln!`
al loro posto.

<Listing number="12-24" file-name="src/main.rs" caption="Scrittura di messaggi di errore sullo standard error anziché sullo standard output usando `eprintln!`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-24/src/main.rs:here}}
```

</Listing>

Ora eseguiamo di nuovo il programma nello stesso modo, senza argomenti e
reindirizzando lo standard output con `>`:

```console
$ cargo run > output.txt
Problema nella parsing degli argomenti: non ci sono abbastanza argomenti
```

Ora vediamo l'errore sullo schermo e _output.txt_ non contiene nulla, che è il
comportamento che ci aspettiamo dai programmi a riga di comando.

Eseguiamo di nuovo il programma con argomenti che non causano errori ma che comunque
reindirizziamo l'output standard a un file, in questo modo:

```console
$ cargo run -- to poem.txt > output.txt
```

Non vedremo alcun output sul terminale e _output.txt_ conterrà i nostri
risultati:

<span class="filename">File: output.txt</span>

```text
Are you nobody, too?
How dreary to be somebody!
```

Questo dimostra che ora stiamo utilizzando l'output standard per l'output corretto
e l'errore standard per l'output di errore, a seconda dei casi.

## Riepilogo

Questo capitolo ha riepilogato alcuni dei concetti principali appresi finora e
ha spiegato come eseguire operazioni di I/O comuni in Rust. Utilizzando argomenti della riga di comando, file, variabili d'ambiente e la macro `eprintln!` per la stampa
degli errori, ora sei pronto a scrivere applicazioni da riga di comando. In combinazione con
i concetti dei capitoli precedenti, il codice sarà ben organizzato, memorizzerà i dati
in modo efficace nelle strutture dati appropriate, gestirà gli errori in modo efficiente e sarà
ben testato.

Successivamente, esploreremo alcune funzionalità di Rust influenzate dai linguaggi funzionali: closures (chiusure) e iteratori.
