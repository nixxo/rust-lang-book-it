## Costruire un Server Web Single-_Thread_

Inizieremo facendo funzionare un server web con un singolo thread. Prima di
cominciare, vediamo una breve panoramica dei protocolli coinvolti nella
costruzione di un server web. I dettagli di questi protocolli sono oltre lo
scopo di questo libro, ma una breve panoramica ti fornirà le informazioni di cui
hai bisogno.

I due protocolli principali coinvolti nei server web sono _Hypertext Transfer
Protocol_ _(HTTP)_ e _Transmission Control Protocol_ _(TCP)_. Entrambi i
protocolli sono protocolli _request-response_, il che significa che un _client_
avvia le richieste e un _server_ ascolta le richieste e fornisce una risposta al
client. I contenuti di quelle richieste e risposte sono definiti dai protocolli.

TCP è il protocollo di livello inferiore che descrive i dettagli su come le
informazioni vengono trasferite da un server all’altro, ma non specifica cosa
siano quelle informazioni. HTTP si basa su TCP definendo i contenuti delle
richieste e delle risposte. Tecnicamente è possibile utilizzare HTTP con altri
protocolli, ma nella stragrande maggioranza dei casi, HTTP invia i suoi dati su
TCP. Lavoreremo con i byte grezzi delle richieste e risposte TCP e HTTP.

### Ascoltare la Connessione TCP

Il nostro server web deve ascoltare una connessione TCP, quindi questa è la
prima parte su cui lavoreremo. La libreria standard offre un modulo `std::net`
che ci permette di farlo. Creiamo un nuovo progetto nel modo abituale:

```console
$ cargo new ciao
     Created binary (application) `ciao` project
$ cd ciao
```

Ora inserisci il codice nel Listato 21-1 in _src/main.rs_ per iniziare. Questo
codice ascolterà all’indirizzo locale `127.0.0.1:7878` per flussi TCP in arrivo.
Quando riceve un flusso in arrivo, stamperà `Connessione stabilita!`.

<Listing number="21-1" file-name="src/main.rs" caption="Ascolto per flussi in arrivo e stampa di un messaggio quando si riceve un flusso">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-01/src/main.rs}}
```

</Listing>

Utilizzando `TcpListener`, possiamo ascoltare le connessioni TCP all’indirizzo
`127.0.0.1:7878`. Nell’indirizzo, la sezione prima dei due punti è un indirizzo
IP che rappresenta il tuo computer (è lo stesso su ogni computer e non
rappresenta specificamente il computer degli autori), e `7878` è la porta.
Abbiamo scelto questa porta per due motivi: HTTP non è normalmente accettato su
questa porta, quindi il nostro server è improbabile che entri in conflitto con
qualsiasi altro server web che potresti avere in esecuzione sulla tua macchina,
e 7878 è _rust_ digitato sul tastierino numerico di un telefono.

La funzione `bind` in questo scenario funziona come la funzione `new` nel senso
che restituirà una nuova istanza di `TcpListener`. La funzione si chiama `bind`
perché, in ambito di connessioni di rete, connettersi a una porta per ascoltare
è noto come _“binding_ a una porta”.

La funzione `bind` restituisce un `Result<T, E>`, il che indica che è possibile
che il _binding_ fallisca, ad esempio, se eseguissimo due istanze del nostro
programma e quindi avessimo due programmi che ascoltano sulla stessa porta.
Poiché stiamo scrivendo un server di base solo per scopi di apprendimento, non
ci preoccuperemo di gestire questo tipi di errori; invece, usiamo `unwrap` per
interrompere il programma se si verificano errori.

Il metodo `incoming` su `TcpListener` restituisce un iteratore che ci fornisce
una sequenza di _stream_ (più specificamente, _stream_ di _type_ `TcpStream`).
Un singolo _stream_ rappresenta una connessione aperta tra il _client_ e il
_server_. _Connessione_ è il nome per l’intero processo di richiesta e risposta
in cui un _client_ si connette al _server_, il _server_ genera una risposta e il
_server_ chiude la connessione. Di conseguenza, leggeremo dal `TcpStream` per
vedere cosa il _client_ ha inviato e poi scriveremo la nostra risposta sullo
_stream_ per inviare dati indietro al _client_. Nel complesso, questo ciclo
`for` elaborerà ogni connessione a turno e produrrà per noi una serie di
_stream_ da gestire.

Per ora, la nostra gestione dello _stream_ consiste nel chiamare `unwrap` per
terminare il nostro programma se lo _stream_ ha errori; se non ci sono errori,
il programma stampa un messaggio. Aggiungeremo più funzionalità per il caso di
successo nel prossimo Listato. Il motivo per cui potremmo ricevere errori dal
metodo `incoming` quando un _client_ si connette al _server_ è che non stiamo
effettivamente iterando sulle connessioni. Invece, stiamo iterando sui
_tentativi di connessione_. La connessione potrebbe non riuscire per un certo
numero di motivi, molti dei quali specifici del sistema operativo. Ad esempio,
molti sistemi operativi hanno un limite al numero di connessioni aperte
simultanee che possono supportare; i nuovi tentativi di connessione oltre quel
numero produrranno un errore finché alcune delle connessioni aperte non vengono
chiuse.

Proviamo a eseguire questo codice! Invoca `cargo run` nel terminale e poi apri
_127.0.0.1:7878_ in un browser web. Il browser dovrebbe mostrare un messaggio di
errore come “Connection reset” perché il server non sta attualmente inviando
indietro alcun dato. Ma quando guardi il tuo terminale, dovresti vedere diversi
messaggi che sono stati stampati quando il browser si è connesso al server!

```text
     Running `target/debug/ciao`
Connessione stabilita!
Connessione stabilita!
Connessione stabilita!
```

A volte vedrai più messaggi stampati per una singola richiesta del browser; il
motivo potrebbe essere che il browser sta facendo una richiesta per la pagina
nonché una richiesta per altre risorse, come l’icona _favicon.ico_ che appare
nella scheda del browser.

Potrebbe anche essere che il browser stia cercando di connettersi al server più
volte perché il server non sta rispondendo con alcun dato. Quando `stream` esce
dallo _scope_ e viene de-allocato alla fine del ciclo, la connessione viene
chiusa come parte dell’implementazione di `drop`. I browser a volte gestiscono
le connessioni chiuse tentando di ristabilirle, perché il problema potrebbe
essere temporaneo.

A volte i browser aprono più connessioni al server senza inviare alcuna
richiesta, in modo che se devono poi *fare* delle richieste, quelle richieste
possano avvenire più rapidamente. Quando ciò accade, il nostro server vedrà ogni
connessione, indipendentemente dal fatto che ci siano richieste su quella
connessione. Molte versioni di browser basati su Chrome fanno questo, ad
esempio; puoi disabilitare quell’ottimizzazione utilizzando la modalità di
navigazione privata o utilizzando un browser diverso.

Il fattore importante è che abbiamo ottenuto con successo un _handle_ a una
connessione TCP!

Ricorda di fermare il programma premendo <kbd>ctrl</kbd>-<kbd>C</kbd> quando hai
finito di eseguire una versione particolare del codice. Poi, riavvia il
programma invocando il comando `cargo run` dopo aver apportato ogni modifica al
codice per assicurarti di eseguire il codice più recente.

### Leggere la Richiesta

Implementiamo ora la funzionalità per leggere la richiesta dal browser! Per
separare le responsabilità di ottenere prima una connessione e poi intraprendere
qualche azione con la connessione, inizieremo una nuova funzione per elaborare
le connessioni. In questa nuova funzione `gestisci_connessione`, leggeremo dati
dal flusso TCP e li stamperemo in modo da poter vedere i dati inviati dal
browser. Modifica il codice per essere come nel Listato 21-2.

<Listing number="21-2" file-name="src/main.rs" caption="Lettura dal `TcpStream` e stampa dei dati">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-02/src/main.rs}}
```

</Listing>

Portiamo `std::io::BufReader` e `std::io::prelude` nello _scope_ per ottenere
accesso a _trait_ e _type_ che ci permettono di “leggere da” e “scrivere sullo”
_stream_. Nel ciclo `for` nella funzione `main`, invece di stampare un messaggio
che dice che abbiamo fatto una connessione, ora chiamiamo la nuova funzione
`gestisci_connessione` e le passiamo lo `stream`.

Nella funzione `gestisci_connessione`, creiamo una nuova istanza di `BufReader`
che incapsula un _reference_ allo `stream`. Il `BufReader` aggiunge _buffering_
gestendo le chiamate ai metodi del _trait_ `std::io::Read` per noi.

Creiamo una variabile chiamata `http_request` per raccogliere le righe della
richiesta che il browser invia al nostro server. Indichiamo che vogliamo
raccogliere queste righe in un vettore aggiungendo l’annotazione `Vec<_>`.

`BufReader` implementa il _trait_ `std::io::BufRead`, che fornisce il metodo
`lines`. Il metodo `lines` restituisce un iteratore di `Result<String,
std::io::Error>` dividendo lo _stream_ di dati ogni volta che vede un byte
_newline_ (_nuova riga_). Per ottenere ogni `String`, usiamo `map` e `unwrap` su
ogni `Result`. Il `Result` potrebbe essere un errore se i dati non sono UTF-8
validi o se c’è stato un problema durante la lettura dallo _stream_. Di nuovo,
un programma di produzione dovrebbe gestire questi errori in modo più elegante,
ma stiamo scegliendo di fermare il programma in caso di errore per semplicità.

Il browser segnala la fine di una richiesta HTTP inviando due caratteri
_newline_ di seguito, quindi per ottenere una richiesta dallo _stream_,
prendiamo righe fino a ottenere una riga che è una stringa vuota. Una volta che
abbiamo raccolto le righe nel vettore, le stampiamo usando una formattazione
_debug_ in modo da poter dare un’occhiata alle istruzioni che il browser web sta
inviando al nostro server.

Proviamo questo codice! Avvia il programma e fai una richiesta nel browser web
di nuovo. Nota che otterremo ancora una pagina di errore nel browser, ma
l’output del nostro programma nel terminale ora apparirà simile a questo:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-02
cargo run
make a request to 127.0.0.1:7878
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling ciao v0.1.0 (file:///progetti/ciao)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/ciao`
Request: [
    "GET / HTTP/1.1",
    "Host: 127.0.0.1:7878",
    "User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:144.0) Gecko/20100101 Firefox/144.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
    "Accept-Language: it-IT,it;q=0.8,en-US;q=0.5,en;q=0.3",
    "Accept-Encoding: gzip, deflate, br, zstd",
    "Sec-GPC: 1",
    "Connection: keep-alive",
    "Upgrade-Insecure-Requests: 1",
    "Sec-Fetch-Dest: document",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-Site: cross-site",
    "Priority: u=0, i",
]
```

A seconda del tuo browser, potresti ottenere un output leggermente diverso. Ora
che stiamo stampando i dati della richiesta, possiamo vedere perché otteniamo
più connessioni da una singola richiesta del browser guardando il percorso dopo
`GET` nella prima riga della richiesta. Se le connessioni ripetute stanno tutte
richiedendo `/`, sappiamo che il browser sta cercando di recuperare `/`
ripetutamente perché non sta ottenendo una risposta dal nostro programma.

Scomponiamo questi dati di richiesta per capire cosa il browser sta chiedendo al
nostro programma.

### Guardare Più da Vicino una Richiesta HTTP

HTTP è un protocollo basato su testo, e una richiesta assume questo formato:

```text
Metodo URI-Richiesto Versione-HTTP CRLF
headers CRLF
corpo-messaggio
```

La prima riga è la _request line_ che contiene informazioni su cosa il _client_
sta richiedendo. La prima parte della _request line_ indica il metodo usato,
come `GET` o `POST`, che descrive come il _client_ sta facendo questa richiesta.
Il nostro _client_ ha usato una richiesta `GET`, che significa che sta
richiedendo informazioni.

La parte successiva della _request line_ è `/`, che indica l’_uniform resource
identifier_ _(URI)_ che il _client_ sta richiedendo: un URI è quasi, ma non del
tutto, lo stesso di un _uniform resource locator_ _(URL)_. La differenza tra URI
e URL non è importante per i nostri scopi in questo capitolo, ma la specifica
HTTP usa il termine _URI_, quindi possiamo sostituire mentalmente _URL_ per
_URI_ qui.

L’ultima parte è la versione HTTP che il client usa, e poi la _request line_
termina con una sequenza CRLF. (_CRLF_ sta per _carriage return_ e _line feed_,
che sono termini dai giorni della macchina da scrivere!) La sequenza CRLF può
anche essere scritta come `\r\n`, dove `\r` è un “ritorno a capo” e `\n` è un
“nuova linea”. La _sequenza CRLF_ separa la _request line_ dal resto dei dati
della richiesta. Nota che quando la CRLF viene stampata, vediamo iniziare una
nuova riga piuttosto che `\r\n`.

Guardando i dati della _request line_ che abbiamo ricevuto eseguendo il nostro
programma finora, vediamo che `GET` è il metodo, `/` è l’URI della richiesta, e
`HTTP/1.1` è la versione.

Dopo la _request line_, le righe rimanenti a partire da `Host:` in poi sono
_header_ (_intestazioni_). Le richieste `GET` non hanno corpo.

Prova a fare una richiesta da un browser diverso o chiedendo un indirizzo
diverso, come _127.0.0.1:7878/test_, per vedere come cambiano i dati della
richiesta.

Ora che sappiamo cosa il browser sta chiedendo, rispondiamo con qualche dato!

### Scrivere una Risposta

Stiamo per implementare l’invio di dati in risposta a una richiesta del
_client_. Le risposte hanno il seguente formato:

```text
Versione-HTTP Codice-di-Stato Enunciazione CRLF
headers CRLF
corpo-messaggio
```

La prima riga è una _status line_ che contiene la versione HTTP usata nella
risposta, un _codice di stato_ numerico che riassume il risultato della
richiesta, e una _enunciazione_ che fornisce una descrizione testuale del codice
di stato. Dopo la sequenza CRLF ci sono eventuali _header_, un’altra sequenza
CRLF, e il corpo della risposta.

Ecco un esempio di risposta che usa la versione HTTP 1.1 e ha un codice di stato
di 200, una _enunciazione_ OK, nessun _header_, e nessun corpo:

```text
HTTP/1.1 200 OK\r\n\r\n
```

Il codice di stato 200 è la risposta standard di successo. Il testo è una
minuscola risposta HTTP di successo. Scriviamola sullo _stream_ come nostra
risposta a una richiesta di successo! Dalla funzione `gestisci_connessione`,
rimuovi il `println!` che stava stampando i dati della richiesta e sostituiscilo
con il codice nel Listato 21-3.

<Listing number="21-3" file-name="src/main.rs" caption="Scrivere una minuscola risposta HTTP di successo sullo _stream_">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-03/src/main.rs:here}}
```

</Listing>

La prima nuova riga definisce la variabile `risposta` che contiene i dati del
messaggio di successo. Poi, chiamiamo `as_bytes` sulla nostra `risposta` per
convertire i dati della stringa in byte. Il metodo `write_all` su `stream`
prende un `&[u8]` e invia quei byte direttamente sulla connessione. Poiché
l’operazione `write_all` potrebbe fallire, usiamo `unwrap` su qualsiasi
risultato di errore come prima. Di nuovo, in un’applicazione reale,
aggiungeresti la gestione degli errori qui.

Con questi cambiamenti, eseguiamo il nostro codice e facciamo una richiesta. Non
stiamo più stampando alcun dato sul terminale, quindi non vedremo alcun output
diverso dall’output di Cargo. Quando carichi _127.0.0.1:7878_ in un browser web,
dovresti ottenere una pagina vuota invece di un errore. Hai appena codificato
manualmente la ricezione di una richiesta HTTP e l’invio di una risposta!

### Restituire Vero HTML

Implementiamo la funzionalità per restituire qualcosa più di una pagina vuota.
Crea un nuovo file _ciao.html_ nella radice della directory del tuo progetto,
non nella directory _src_. Puoi inserire qualsiasi HTML tu voglia; il Listato
21-4 mostra una possibilità.

<Listing number="21-4" file-name="ciao.html" caption="Un file HTML di esempio da restituire in una risposta">

```html
{{#include ../listings/ch21-web-server/listing-21-05/ciao.html}}
```

</Listing>

Questo è un documento HTML5 minimale con un’intestazione e del testo. Per
restituire questo dal _server_ quando viene ricevuta una richiesta,
modificheremo `gestisci_connessione` come mostrato nel Listato 21-5 per leggere
il file HTML, aggiungerlo alla risposta come corpo, e inviarlo.

<Listing number="21-5" file-name="src/main.rs" caption="Invio del contenuto di *ciao.html* come corpo della risposta">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-05/src/main.rs:here}}
```

</Listing>

Abbiamo aggiunto `fs` all’istruzione `use` per portare il modulo _filesystem_
della libreria standard nello _scope_. Il codice per leggere i contenuti di un
file in una stringa dovrebbe apparire familiare; l’abbiamo usato quando abbiamo
letto i contenuti di un file per il nostro progetto I/O nel Listato 12-4.

Poi usiamo `format!` per aggiungere il contenuto del file come corpo della
risposta di successo. Per garantire una risposta HTTP valida, aggiungiamo
l’_header_ `Content-Length`, che è impostato alla dimensione del nostro corpo
della risposta, in questo caso la dimensione di `ciao.html`.

Esegui questo codice con `cargo run` e carica _127.0.0.1:7878_ nel tuo browser;
dovresti vedere il tuo HTML renderizzato!

Attualmente, stiamo ignorando i dati della richiesta in `http_request` e stiamo
solo inviando indietro i contenuti del file HTML incondizionatamente. Questo
significa che se provi a richiedere _127.0.0.1:7878/altra-pagina_ nel tuo
browser, otterrai ancora questa stessa risposta HTML. Al momento, il nostro
server è molto limitato e non fa quello che fanno la maggior parte dei server
web. Vogliamo personalizzare le nostre risposte a seconda della richiesta e
rispondere con il file HTML solo alle richieste corrette a `/`.

### Validare la Richiesta e Rispondere Selettivamente

Al momento, il nostro server web restituirà l’HTML nel file indipendentemente da
cosa il _client_ abbia richiesto. Aggiungiamo funzionalità al nostro codice per
controllare che il browser stia richiedendo `/` prima di restituire il file HTML
e per restituire un errore se il browser richiede qualcos’altro. Per questo
dobbiamo modificare `gestisci_connessione`, come mostrato nel Listato 21-6.
Questo nuovo codice controlla il contenuto della richiesta ricevuta rispetto a
quello che sappiamo essere una richiesta per _/_, e aggiunge blocchi `if` e
`else` per trattare le richieste in modo diverso.

<Listing number="21-6" file-name="src/main.rs" caption="Gestione delle richieste a */* in modo diverso dalle altre richieste">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-06/src/main.rs:here}}
```

</Listing>

Guarderemo solo alla prima riga della richiesta HTTP, quindi piuttosto che
leggere l’intera richiesta in un vettore, stiamo chiamando `next` per ottenere
il primo elemento dall’iteratore. Il primo `unwrap` si occupa dell’`Option` e
ferma il programma se l’iteratore non ha elementi. Il secondo `unwrap` gestisce
il `Result` e ha lo stesso effetto dell’`unwrap` che era nella `map` aggiunta
nel Listato 21-2.

Successivamente, controlliamo la `request_line` per vedere se è uguale alla
_request line_ di una richiesta GET al percorso _/_ . Se lo è, il blocco `if`
restituisce i contenuti del nostro file HTML.

Se la `request_line` _non_ è uguale alla richiesta GET al percorso _/_ ,
significa che abbiamo ricevuto qualche altra richiesta. Aggiungeremo codice al
blocco `else` tra un momento per rispondere a tutte le altre richieste.

Esegui questo codice ora e richiedi _127.0.0.1:7878_; dovresti ottenere l’HTML
in _ciao.html_. Se fai qualsiasi altra richiesta, come
_127.0.0.1:7878/altra-pagina_, otterrai un errore di connessione come quelli che
hai visto eseguendo il codice dei Listati 21-1 e 21-2.

Ora aggiungiamo il codice nel Listato 21-7 al blocco `else` per restituire una
risposta con il codice di stato 404, che segnala che il contenuto per la
richiesta non è stato trovato. Restituiremo anche un po’ di HTML per una pagina
da renderizzare nel browser indicando la risposta all’utente finale.

<Listing number="21-7" file-name="src/main.rs" caption="Risposta con codice di stato 404 e una pagina di errore se è stato richiesto qualcosa di diverso da */*">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-07/src/main.rs:here}}
```

</Listing>

Qui, la nostra risposta ha una _status line_ con codice di stato `404` ed
enunciazione `NOT FOUND`. Il corpo della risposta sarà l’HTML nel file
_404.html_. Dovrai creare un file _404.html_ accanto a _ciao.html_ per la pagina
di errore; di nuovo, sentiti libero di usare qualsiasi HTML tu voglia, o usa
l’esempio HTML nel Listato 21-8.

<Listing number="21-8" file-name="404.html" caption="Contenuto di esempio per la pagina da inviare come risposta con codice 404">

```html
{{#include ../listings/ch21-web-server/listing-21-07/404.html}}
```

</Listing>

Con questi cambiamenti, esegui di nuovo il tuo server. Richiedere
_127.0.0.1:7878_ dovrebbe restituire i contenuti di _ciao.html_, e qualsiasi
altra richiesta, come _127.0.0.1:7878/altra-pagina_, dovrebbe restituire l’HTML
di errore da _404.html_.

### Riscrittura

Al momento, i blocchi `if` e `else` hanno molta ripetizione: entrambi stanno
leggendo file e scrivendo i contenuti dei file sullo _stream_. Le uniche
differenze sono la _status line_ e il nome del file. Rendiamo il codice più
conciso estraendo quelle differenze in righe `if` e `else` separate che
assegneranno i valori della _status line_ e del nome del file a variabili;
possiamo poi usare quelle variabili incondizionatamente nel codice per leggere
il file e scrivere la risposta. Il Listato 21-9 mostra il codice risultante dopo
aver sostituito i blocchi `if` e `else`.

<Listing number="21-9" file-name="src/main.rs" caption="Riscrittura dei blocchi `if` e `else` per contenere solo il codice che differisce tra i due casi">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-09/src/main.rs:here}}
```

</Listing>

Ora i blocchi `if` e `else` restituiscono solo i valori appropriati per la
_status line_ e il nome del file in una tupla; poi usiamo la destrutturazione
per assegnare questi due valori a `status_line` e `filename` usando un _pattern_
nella dichiarazione `let`, come discusso nel Capitolo 19.

Il codice precedentemente duplicato è ora fuori dai blocchi `if` e `else` e usa
le variabili `status_line` e `filename`. Questo rende più facile vedere la
differenza tra i due casi, e significa che abbiamo solo un posto dove guardare
per aggiornare il codice se vogliamo cambiare come funziona la lettura del file
e la scrittura della risposta. Il comportamento del codice nel Listato 21-9 sarà
lo stesso di quello nel Listato 21-7.

Fantastico! Ora abbiamo un semplice server web in circa 40 righe di codice Rust
che risponde a una richiesta con una pagina di contenuto e risponde a tutte le
altre richieste con una risposta 404.

Attualmente, il nostro server gira in un singolo _thread_, il che significa che
può servire solo una richiesta alla volta. Esaminiamo come questo possa essere
un problema simulando alcune richieste lente. Poi, lo risolveremo in modo che il
nostro server possa gestire più richieste contemporaneamente.
