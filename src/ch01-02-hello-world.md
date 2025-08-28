## Hello, World!

Ora che hai installato Rust, è il momento di scrivere il tuo primo programma
Rust. Quando si impara un nuovo linguaggio, è consuetudine scrivere un piccolo
programma che stampi sullo schermo il testo `Hello, world!`

> Nota: questo libro presuppone una certa familiarità di base con la riga di
> comando. Rust non ha particolari esigenze per quanto riguarda l'editing o gli
> strumenti o dove risiede il tuo codice, quindi se preferisci usare _IDE_
> invece della riga di comando, sentiti libero di usare il tuo _IDE_ preferito.
> Molti _IDE_ ora hanno un certo grado di supporto per Rust; controlla la
> documentazione dell'_IDE_ per maggiori dettagli. Il team di Rust si è
> concentrato sull'integrazione ottima con gli _IDE_ tramite `rust-analyzer`.
> Vedi > [Appendice D][devtools] per maggiori dettagli.

### Creare una Directory di Progetto

Inizierai creando una directory per memorizzare il tuo codice Rust. Per Rust non
è importante dove si trovi il tuo codice, ma per gli esercizi e i progetti di
questo libro ti consigliamo di creare una cartella _progetti_ nella tua home
directory e di tenere tutti i tuoi progetti lì.

Apri un terminale e inserisci i seguenti comandi per creare una directory
_progetti_ e una directory per il progetto "Hello, world!" all'interno della
directory _progetti_.

Per Linux, macOS, e PowerShell su Windows, digita questo:

```console
$ mkdir ~/progetti
$ cd ~/progetti
$ mkdir hello_world
$ cd hello_world
```

Per Windows con CMD, digita questo:

```cmd
> mkdir "%USERPROFILE%\progetti"
> cd /d "%USERPROFILE%\progetti"
> mkdir hello_world
> cd hello_world
```

### Scrivere ed Eseguire un Programma Rust

Adesso crea un nuovo file sorgente e chiamalo _main.rs_. I file di Rust
terminano sempre con l'estensione _.rs_. Se usi più di una parola nel nome del
file, la convenzione è di usare un trattino basso per separarle. Ad esempio, usa
_hello_world.rs_ piuttosto che _helloworld.rs_.

Ora apri il file _main.rs_ che hai appena creato e inserisci il codice del
Listato 1-1.

<Listing number="1-1" file-name="main.rs" caption="Un programma che stampa `Hello, world!`">

```rust
fn main() {
    println!("Hello, world!");
}
```

</Listing>

Salva il file e torna alla finestra del terminale alla cartella
_~/progetti/hello_world_ . Su Linux o macOS, inserisci i seguenti comandi per
compilare ed eseguire il file:

```console
$ rustc main.rs
$ ./main
Hello, world!
```

Su Windows, inserisci il comando `.\main` invece di `./main`:

```powershell
> rustc main.rs
> .\main
Hello, world!
```

Indipendentemente dal sistema operativo, la stringa `Hello, world!` dovrebbe
essere stampata sul terminale. Se non vedi questo output, consulta la sezione
["Risoluzione dei problemi"][risoluzione-dei-problemi]<!-- ignore --> nel
capitolo Installazione per trovare aiuto.

Se `Hello, world!` è stato stampato, congratulazioni! Hai ufficialmente scritto
un programma Rust. Questo ti rende un programmatore Rust, benvenuto!

### Anatomia di un Programma Rust

Esaminiamo in dettaglio questo programma "Hello, world!". Ecco il primo pezzo
del puzzle:

```rust
fn main() {

}
```

Queste righe definiscono una funzione chiamata `main`. La funzione `main` è
speciale: è sempre il primo codice che viene eseguito in ogni eseguibile Rust.
Qui, la prima riga dichiara una funzione chiamata `main` che non ha parametri e
non restituisce nulla. Se ci fossero dei parametri, andrebbero dentro le
parentesi `()`.

Il corpo della funzione è racchiuso da `{}`. Rust richiede parentesi graffe
intorno a tutti i corpi delle funzioni. È buona norma posizionare la parentesi
graffa di apertura sulla stessa riga della dichiarazione della funzione,
aggiungendo uno spazio in mezzo.

> Nota: se vuoi attenerti a uno stile standard in tutti i progetti Rust, puoi
> usare uno strumento di formattazione automatica chiamato `rustfmt` per
> formattare il tuo codice in un particolare stile (maggiori informazioni su
> `rustfmt` in [Appendice D][devtools]<!-- ignore -->). Il team di Rust ha
> incluso questo strumento nella distribuzione standard di Rust, come `rustc`,
> quindi dovrebbe essere già installato sul tuo computer!

Il corpo della funzione `main` contiene il seguente codice:

```rust
println!("Hello, world!");
```

Questa riga fa tutto il lavoro di questo piccolo programma: stampa il testo
sullo schermo. Ci sono tre dettagli importanti da notare.

Innanzitutto, `println!` chiama una _macro_ di Rust. Se invece avesse chiamato
una funzione, sarebbe stata inserita come `println` (senza il `!`). Le macro di
Rust sono un modo per scrivere codice che genera codice per estendere la
sintassi di Rust e ne parleremo in modo più dettagliato nel [Capitolo
20][ch20-macros]<!-- ignore -->. Per ora, ti basterà sapere che usare una `!`
significa che stai chiamando una macro invece di una normale funzione e che le
macro non seguono sempre le stesse regole delle funzioni.

In secondo luogo, vedi la stringa `"Hello, world!"` che viene passata come
argomento a `println!` e la stringa viene stampata sullo schermo.

In terzo luogo, terminiamo la riga con un punto e virgola (`;`), che indica che
questa espressione è terminata e che la prossima è pronta per iniziare. La
maggior parte delle righe di codice Rust terminano con un punto e virgola

### La Compilazione e l'Esecuzione Sono Fasi Separate

Hai appena eseguito un programma appena creato, quindi esaminiamo ogni fase del
processo.

Prima di eseguire un programma Rust, devi compilarlo con il compilatore Rust
inserendo il comando `rustc` e passandogli il nome del tuo file sorgente, in
questo modo:

```console
$ rustc main.rs
```

Se hai un background in C o C++, noterai che è simile a `gcc` o `clang`. Dopo
aver compilato con successo, Rust produce un eseguibile binario.

Su Linux, macOS e PowerShell su Windows, puoi vedere l'eseguibile usando il
comando `ls` nella tua shell:

```console
$ ls
main  main.rs
```

Su Linux e macOS, vedrai due file. Con PowerShell su Windows, vedrai gli stessi
tre file che vedresti usando CMD. Con CMD su Windows, inserisci il seguente
comando:

```cmd
> dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```

Questo mostra il file del codice sorgente con estensione _.rs_, il file
eseguibile (_main.exe_ su Windows, solo _main_ su tutte le altre piattaforme) e,
se usi Windows, un file contenente informazioni di debug con estensione _.pdb_.
Ora puoi eseguire il file _main_ o _main.exe_, in questo modo:

```console
$ ./main # or .\main su Windows
```

Se il tuo _main.rs_ è il tuo programma "Hello, world!", questa riga stampa
`Hello, world!` sul tuo terminale.

Se hai più familiarità con un linguaggio dinamico, come Ruby, Python o
JavaScript, potresti non essere abituato alla compilazione e all'esecuzione di
un programma come fasi separate. Rust è un linguaggio _compilato in anticipo_,
il che significa che puoi compilare un programma e dare l'eseguibile a qualcun
altro, che potrà eseguirlo anche senza avere Rust installato. Se dai a qualcuno
un file _.rb_, _.py_ o _.js_, deve avere un'implementazione di Ruby, Python o
JavaScript installata (rispettivamente). Ma in questi linguaggi, hai bisogno di
un solo comando per compilare ed eseguire il tuo programma. Sono compromessi
diversi per ogni linguaggio di programmazione.

La semplice compilazione con `rustc` va bene per i programmi semplici, ma quando
il tuo progetto cresce, vorrai gestire tutte le opzioni e rendere facile la
condivisione del tuo codice. A seguire, ti presenteremo lo strumento Cargo, che
ti aiuterà a scrivere programmi Rust veri e propri.

[risoluzione-dei-problemi]: ch01-01-installation.html#risoluzione-dei-problemi
[devtools]: appendix-04-useful-development-tools.html
[ch20-macros]: ch20-05-macros.html
