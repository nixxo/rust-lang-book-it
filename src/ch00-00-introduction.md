# Introduzione

> Nota: questa edizione del libro in inglese è identica a [The Rust Programming
> Language][nsprust], disponibile in formato cartaceo ed ebook presso [No Starch
> Press][nsp].

[nsprust]: https://nostarch.com/rust-programming-language-2nd-edition
[nsp]: https://nostarch.com/

Benvenuti a _Il Linguaggio di Programmazione Rust_, un libro introduttivo su
Rust. Il linguaggio di programmazione Rust vi aiuta a scrivere software più
veloce e affidabile. L'ergonomia di alto livello e il controllo di basso livello
sono spesso in contrasto nella progettazione dei linguaggi di programmazione;
Rust sfida questo conflitto. Grazie al bilanciamento tra potenti capacità
tecniche e un'ottima esperienza di sviluppo, Rust vi offre la possibilità di
controllare i dettagli di basso livello (come l'utilizzo della memoria) senza
tutte le difficoltà tradizionalmente associate a tale controllo.

## A chi è rivolto Rust

Rust è ideale per molte persone per una serie di motivi. Diamo un'occhiata ad
alcuni dei gruppi più importanti.

### Team di sviluppatori

Rust si sta dimostrando uno strumento produttivo per la collaborazione tra
grandi team di sviluppatori con diversi livelli di conoscenza della
programmazione di sistemi. Il codice di basso livello è soggetto a vari bug
subdoli, che nella maggior parte degli altri linguaggi possono essere
individuati solo attraverso test approfonditi e un'attenta revisione del codice
da parte di sviluppatori esperti. In Rust, il compilatore svolge un ruolo di
gatekeeper rifiutando di compilare il codice con questi bug subdoli, compresi i
bug di concorrenza. Lavorando a fianco del compilatore, il team può dedicare il
proprio tempo a concentrarsi sulla logica del programma piuttosto che a cercare
i bug.

Rust porta anche strumenti di sviluppo contemporanei nel mondo della
programmazione di sistemi:

- Cargo, il gestore di dipendenze e strumento di compilazione integrato, rende
  l'aggiunta, la compilazione e la gestione delle dipendenze semplice e coerente
  in tutto l'ecosistema Rust.
- Lo strumento di formattazione Rustfmt garantisce uno stile di programmazione
  coerente tra gli sviluppatori.
- Rust-analyzer potenzia l'integrazione dell'ambiente di sviluppo integrato
  (IDE) per il completamento del codice e i messaggi di errore in linea.

Utilizzando questi e altri strumenti dell'ecosistema Rust, gli sviluppatori
possono essere produttivi mentre scrivono codice a livello di sistema.

### Studenti

Rust è rivolto agli studenti e a coloro che sono interessati a conoscere i
concetti di sistema. Utilizzando Rust, molte persone hanno imparato a conoscere
argomenti come lo sviluppo di sistemi operativi. La comunità è molto accogliente
e felice di rispondere alle domande degli studenti. Attraverso iniziative come
questo libro, i team di Rust vogliono rendere i concetti di sistema più
accessibili a un maggior numero di persone, soprattutto a chi è alle prime armi
con la programmazione.

### Aziende

Centinaia di aziende, grandi e piccole, utilizzano Rust in produzione per una
serie di compiti, come strumenti a riga di comando, servizi web, strumenti
DevOps, dispositivi embedded, analisi e transcodifica di audio e video,
criptovalute, bioinformatica, motori di ricerca, applicazioni per l'Internet
delle cose, machine learning e persino parti importanti del browser web Firefox.

### Sviluppatori Open Source

Rust è per le persone che vogliono costruire il linguaggio di programmazione
Rust, la comunità, gli strumenti per gli sviluppatori e le librerie. Ci
piacerebbe che tu contribuissi al linguaggio Rust.

### Persone che apprezzano la velocità e la stabilità

Rust è per le persone che desiderano velocità e stabilità in un linguaggio. Per
velocità intendiamo sia la velocità con cui il codice Rust può essere eseguito,
sia la velocità con cui Rust ti permette di scrivere programmi. I controlli del
compilatore di Rust assicurano la stabilità quando si aggiungono funzionalità e
riscrive il codice, in contrasto con la fragilità del codice legacy dei
linguaggi senza questi controlli, che spesso gli sviluppatori hanno paura di
modificare. Puntando ad astrazioni a costo zero, ovvero funzionalità di livello
superiore che vengono compilate in codice di livello inferiore con la stessa
velocità del codice scritto manualmente, Rust cerca di rendere il codice sia
sicuro che veloce.

Il linguaggio Rust spera di supportare anche molti altri utenti; quelli citati
qui sono solo alcuni dei maggiori interessati. Nel complesso, la più grande
ambizione di Rust è quella di eliminare i compromessi che i programmatori hanno
accettato per decenni, offrendo sicurezza _e_ produttività, velocità _e_
ergonomia. Prova Rust e vedi se le sue scelte funzionano per te.

## Per chi è questo libro

Questo libro presuppone che tu abbia scritto codice in un altro linguaggio di
programmazione, ma non fa alcuna ipotesi su quale sia. Abbiamo cercato di
rendere il materiale ampiamente accessibile a coloro che provengono da un'ampia
varietà di background di programmazione. Non dedichiamo molto tempo a parlare di
cosa _sia_ la programmazione o di come pensarla. Se sei completamente nuovo alla
programmazione, è meglio che tu legga un libro che fornisca un'introduzione
specifica alla programmazione.

## Come utilizzare questo libro

In generale, questo libro presuppone che tu lo legga in sequenza, dall'inizio
alla fine. I capitoli più avanzati si basano sui concetti dei capitoli
precedenti, e i capitoli iniziali potrebbero non approfondire un determinato
argomento, che invece viene sviluppato più approfonditamente in un capitolo
successivo.

In questo libro troverai due tipi di capitoli: i capitoli concettuali e i
capitoli di progetto. Nei capitoli concettuali, imparerai a conoscere un aspetto
di Rust. Nei capitoli di progetto, costruiremo insieme dei piccoli programmi,
applicando ciò che hai imparato finora. I capitoli 2, 12 e 21 sono capitoli di
progetto; il resto sono capitoli concettuali.

Il Capitolo 1 spiega come installare Rust, come scrivere un programma "Hello,
world!" e come utilizzare Cargo, il gestore di pacchetti e lo strumento di
compilazione di Rust. Il Capitolo 2 è un'introduzione pratica alla scrittura di
un programma in Rust, con la creazione di un gioco di indovinelli con i numeri.
Qui trattiamo i concetti ad un livello più superficiale e i capitoli successivi
forniranno ulteriori dettagli. Se vuoi sporcarti subito le mani, il Capitolo 2 è
dove puoi iniziare. Il Capitolo 3 tratta le caratteristiche di Rust simili a
quelle di altri linguaggi di programmazione e nel Capitolo 4 imparerai a
conoscere il sistema di controllo esclusivo (_ownership_ d'ora in poi) di Rust.
Se sei uno studente particolarmente meticoloso, che preferisce imparare ogni
dettaglio prima di passare al successivo, potresti voler saltare il Capitolo 2 e
passare direttamente al Capitolo 3, tornando al Capitolo 2 quando vorrai
lavorare su un progetto applicando i dettagli che hai imparato.

Il Capitolo 5 parla di strutture e metodi, mentre il Capitolo 6 tratta le
enumerazioni, le espressioni `match` e il costrutto della struttura di controllo
`if let`. Utilizzerai le strutture e le enumerazioni per creare tipologie
personalizzate in Rust.

Nel Capitolo 7 imparerai a conoscere il sistema di moduli di Rust, le regole di
visibità per l'organizzazione del codice e la sua Application Programming
Interface (API) pubblica. Il Capitolo 8 tratta alcune strutture di dati comuni
che la libreria standard mette a disposizione, come vettori, stringhe e mappe
hash. Il Capitolo 9 esplora la filosofia e le tecniche di gestione degli errori
di Rust.

Il Capitolo 10 approfondisce i generici, i tratti e la longevità (_generics_,
_traits_ e _lifetimes_  d'ora in poi), che ti danno la possibilità di definire
codice applicabile a più tipologie di dato. Il Capitolo 11 è dedicato ai test,
che anche con le garanzie di sicurezza di Rust sono necessari per garantire la
correttezza della logica del tuo programma. Nel Capitolo 12, costruiremo la
nostra implementazione di un sottoinsieme di funzionalità dello strumento da
riga di comando `grep`, che cerca il testo all'interno dei file. Per questo,
utilizzeremo molti dei concetti discussi nei capitoli precedenti.

Il Capitolo 13 esplora le chiusure e gli iteratori: caratteristiche di Rust che
derivano dai linguaggi di programmazione funzionale. Nel Capitolo 14 esamineremo
Cargo in modo più approfondito e parleremo delle migliori pratiche per
condividere le tue librerie con altri. Il Capitolo 15 parla dei puntatori
intelligenti che la libreria standard mette a disposizione e dei _traits_ che ne
consentono la funzionalità.

Nel Capitolo 16, esamineremo diversi modelli di programmazione concorrente e
parleremo di come Rust ti aiuti a programmare con più sotto-processi (_thread_
d'ora in poi) senza paura. Nel Capitolo 17, esploreremo la sintassi _async_ e
_await_ di Rust, insieme a task, futures e stream, e il modello di concomitanza
leggera che consentono.

Il Capitolo 18 analizza come gli idiomi di Rust si confrontano con i principi
della programmazione orientata agli oggetti che potresti conoscere. Il Capitolo
19 è un riferimento ai _pattern_ e al loro riconoscimento (_pattern matching_),
che sono modi potenti di esprimere idee nei programmi Rust. Il Capitolo 20
contiene una varietà di argomenti di interesse, tra cui Rust non sicuro (_unsafe
Rust_ d'ora in poi), macro e altre informazioni su longevità, _traits_,
tipologie (_types_), funzioni e chiusure.

Nel Capitolo 21, completeremo un progetto in cui implementeremo un server web
multi-thread di basso livello!

Infine, alcune appendici contengono informazioni utili sul linguaggio in un
formato meno strutturato. L'**Appendice A** tratta delle parole chiave di Rust,
l'**Appendice B** degli operatori e dei simboli di Rust, l'**Appendice C** dei
_traits_ derivabili forniti dalla libreria standard, l'**Appendice D** di alcuni
utili strumenti di sviluppo e l'**Appendice E** delle varie edizioni di Rust.
Nell'**Appendice F** puoi trovare un'elenco delle traduzioni del libro, mentre
nell'**Appendice G** si parlerà di come viene realizzato Rust e di cosa sia la
nightly Rust.

Non c'è un modo sbagliato di leggere questo libro: se vuoi andare avanti, fallo
pure! Potresti dover tornare indietro ai capitoli precedenti se ti senti
confuso, ma fai quello che va bene per te.


Una parte importante del processo di apprendimento di Rust consiste
nell'imparare a leggere i messaggi di errore visualizzati dal compilatore:
questi ti guideranno verso un codice funzionante. Per questo motivo, ti
forniremo molti esempi che non si compilano insieme al messaggio di errore che
il compilatore ti mostrerà in ogni situazione. Sappi che se inserisci ed esegui
un esempio a caso, potrebbe non compilarsi! Assicurati di leggere il testo
circostante per capire se l'esempio che stai cercando di eseguire è destinato a
dare un errore. Ferris ti aiuterà anche a distinguere il codice che non è
destinato a funzionare:

| Ferris | Significato |
| --- | --- |
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Ferris with a question mark"/> | Questo codice non si compila! |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris throwing up their hands"/> | Questo codice genera panico! |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Ferris with one claw up, shrugging"/> | Questo codice non funziona come dovrebbe. |

Nella maggior parte dei casi, ti guideremo alla versione corretta di qualsiasi
codice che non si compila.

## Codice sorgente

I file sorgente da cui è stato generato questo libro si trovano su
[GitHub][book-it] per la versione italiana.

La versione originale in inglese si trova anch'essa su [GitHub][book].

[book]: https://github.com/rust-lang/book/tree/main/src
[book-it]: https://github.com/nixxo/rust-lang-book-it/tree/it-translation/src
