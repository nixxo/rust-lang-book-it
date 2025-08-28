## Installazione

Il primo passo è quello di installare Rust. Scaricheremo Rust attraverso
`rustup`, uno strumento a riga di comando per gestire le versioni di Rust e gli
strumenti associati. Per il download è necessaria una connessione a internet.

> Nota: Se per qualche motivo preferisci non utilizzare `rustup`, consulta la
> pagina [Altri metodi di installazione di Rust][otherinstall] per ulteriori
> opzioni.

I passaggi seguenti installano l'ultima versione stabile del compilatore Rust.
Le garanzie di stabilità di Rust assicurano che tutti gli esempi del libro che
vengono compilati continueranno a essere compilati anche con le versioni più
recenti di Rust. L'output potrebbe differire leggermente da una versione
all'altra perché Rust spesso migliora i messaggi di errore e gli avvertimenti.
In altre parole, qualsiasi versione più recente e stabile di Rust che
installerai utilizzando questi passaggi dovrebbe funzionare come previsto con il
contenuto di questo libro.

> ### Annotazioni per la Riga di Comando
>
> In questo capitolo e in tutto il libro, mostreremo alcuni comandi utilizzati
> nel terminale. Le linee che dovresti inserire in un terminale iniziano tutte
> con `$`. Non è necessario digitare il carattere `$`; è il prompt della riga di
> comando mostrato per indicare l'inizio di ogni comando. Le linee che non
> iniziano con `$` mostrano solitamente  l'output del comando precedente.
> Inoltre, gli esempi specifici per PowerShell useranno `>` anziché `$`.

### Installare `rustup` su Linux o macOS

Se stai usando Linux o macOS, apri un terminale e inserisci il seguente comando:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Il comando scarica uno script e avvia l'installazione dello strumento `rustup`,
che installa l'ultima versione stabile di Rust. Potrebbe esserti richiesta la
tua password. Se l'installazione ha successo, apparirà la seguente riga:

```text
Rust is installed now. Great!
```

Avrai anche bisogno di un _linker_, che è un programma che Rust utilizza per
unire i suoi output compilati in un unico file. È probabile che tu ne abbia già
uno. Se ottieni errori di linker, dovresti installare un compilatore C, che di
solito include un linker. Un compilatore C è utile anche perché alcuni pacchetti
comuni di Rust dipendono dal codice C e avranno bisogno di un compilatore C.

Su macOS, puoi ottenere un compilatore C eseguendo:

```console
$ xcode-select --install
```

Gli utenti Linux dovrebbero generalmente installare GCC o Clang, in base alla
documentazione della loro distribuzione. Ad esempio, se utilizzi Ubuntu, puoi
installare il pacchetto `build-essential`.

### Installare `rustup` su Windows

Su Windows, vai su [https://www.rust-lang.org/tools/install][install] e segui le
istruzioni per installare Rust. A un certo punto dell'installazione, ti verrà
richiesto di installare Visual Studio, che fornisce un linker e le librerie
native necessarie per compilare i programmi. Se hai bisogno di aiuto per questo
passaggio, consulta
[https://rust-lang.github.io/rustup/installation/windows-msvc.html][msvc]

Il resto di questo libro utilizza comandi che funzionano sia in _cmd.exe_ che in
PowerShell. Se ci sono differenze specifiche, ti spiegheremo quale utilizzare.

### Risoluzione dei Problemi

Per verificare se Rust è stato installato correttamente, apri il terminale e
inserisci questo comando:

```console
$ rustc --version
```

Dovresti vedere il numero di versione, l'hash del commit e la data del commit
dell'ultima versione stabile rilasciata, nel seguente formato:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Se vedi queste informazioni, hai installato Rust con successo! Se non vedi
queste informazioni, controlla che Rust sia nella tua variabile di sistema
`%PATH%` come segue.

Su Windows con CMD, usa:

```console
> echo %PATH%
```

In PowerShell, usa:

```powershell
> echo $env:Path
```

In Linux e macOS, usa:

```console
$ echo $PATH
```

Se sembra essere tutto in ordine ma Rust non funziona ancora, ci sono diversi
posti in cui puoi trovare aiuto. Scopri come metterti in contatto con altri
Rustaceani (_Rustaceans_ d'ora in poi) (uno stupido soprannome con cui ci
chiamiamo) sulla [pagina della comunità][community].

### Aggiornamento e Disinstallazione

Una volta che Rust è stato installato tramite `rustup`, l'aggiornamento a una
nuova versione è semplice. Dalla tua _shell_, esegui il seguente script di
aggiornamento:

```console
$ rustup update
```

Per disinstallare Rust e `rustup`, esegui il seguente script di disinstallazione
dalla tua shell:

```console
$ rustup self uninstall
```

### Documentazione in Locale

L'installazione di Rust include anche una copia locale della documentazione per
poterla leggere offline. Esegui `rustup doc` per aprire la documentazione locale
nel tuo browser.

Ogni qual volta hai un dubbio su un _type_ o una funzione fornita dalla libreria
standard e non sei sicuro di cosa faccia o di come usarla, usa la documentazione
delle _API_ per scoprirlo!

### Editor di Testo e Ambienti di Sviluppo Integrati (_IDE_)

Questo libro non fa alcuna ipotesi sugli strumenti che utilizzi per scrivere il
codice Rust. Qualsiasi editor di testo è in grado di fare il suo lavoro!
Tuttavia, molti editor di testo e ambienti di sviluppo integrati (_IDE_ d'ora in
poi) hanno un supporto integrato per Rust. Puoi sempre trovare un elenco
abbastanza aggiornato di molti editor e _IDE_ nella [pagina degli
strumenti][tools] sul sito web di Rust.

### Lavorare Offline con Questo Libro

In diversi esempi, utilizzeremo pacchetti Rust oltre alla libreria standard. Per
lavorare a questi esempi, dovrai disporre di una connessione a internet o aver
scaricato le dipendenze in anticipo. Per scaricare le dipendenze in anticipo,
puoi eseguire i seguenti comandi. (Spiegheremo cos'è `cargo` e cosa fa ciascuno
di questi comandi in dettaglio più avanti)

```console
$ cargo new get-dependencies
$ cd get-dependencies
$ cargo add rand@0.8.5 trpl@0.2.0
```

In questo modo i download di questi pacchetti verranno memorizzati nella cache e
non sarà necessario scaricarli in seguito. Una volta eseguito questo comando,
non dovrai conservare la cartella `get-dependencies`. Se hai eseguito questo
comando, puoi aggiungere il flag `--offline` quando userai il comando `cargo`
nel resto del libro per utilizzare queste versioni memorizzate nella cache
invece di scaricarle da internet in quel momento.

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html
[community]: https://www.rust-lang.org/community
[tools]: https://www.rust-lang.org/tools
