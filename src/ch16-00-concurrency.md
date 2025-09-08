# Concorrenza Senza Paura

Gestire la programmazione concorrente in modo sicuro ed efficiente è un altro
degli obiettivi principali di Rust. La _programmazione concorrente_, in cui le
diverse parti di un programma vengono eseguite in modo indipendente, e la
_programmazione parallela_, in cui le diverse parti di un programma vengono
eseguite contemporaneamente, stanno diventando sempre più importanti per
sfruttare i processori multi-core dei moderni computer. Storicamente, la
programmazione in questi contesti è stata difficile e soggetta a errori. Rust
spera di cambiare questa situazione.

Inizialmente, il team di Rust pensava che garantire la sicurezza della memoria e
prevenire i problemi di concorrenza fossero due sfide distinte da risolvere con
metodi diversi. Con il tempo, il team ha scoperto che i sistemi di _ownership_ e
dei _type_ sono un potente insieme di strumenti che aiutano a gestire la
sicurezza della memoria _e_ i problemi di concorrenza! Sfruttando la _ownership_
e il controllo dei _type_, molti errori di concorrenza sono rilevati in fase di
compilazione in Rust piuttosto che presentarsi durante l'esecuzione. Pertanto,
invece di farti perdere molto tempo a cercare di riprodurre le circostanze
esatte in cui si verifica un bug di concorrenza in fase di esecuzione, il codice
errato non verrà compilato e presenterà un errore che spiega il problema. Di
conseguenza, puoi correggere il tuo codice mentre ci stai lavorando piuttosto
che potenzialmente dopo che è stato compilato e distrubuito a chi lo utilizza.
Abbiamo soprannominato questo aspetto di Rust _fearless concurrency_
(_concorrenza senza paura_). La concorrenza senza paura ti permette di scrivere
codice privo di bug subdoli e facile da rifattorizzare senza introdurre nuovi
bug.

> Nota: per semplicità, ci riferiremo a molti problemi come _concorrenti_
> piuttosto che essere più precisi dicendo _concorrenti e/o paralleli_. Per
> questo capitolo, sostituisci mentalmente _concorrenti e/o paralleli_ ogni
> volta che usiamo _concorrenti_. Nel prossimo capitolo, dove la distinzione è
> più importante, saremo più specifici.

Molti linguaggi sono dogmatici sulle soluzioni che offrono per gestire i
problemi di concorrenza. Ad esempio, Erlang dispone di eleganti funzionalità per
la concomitanza con il passaggio di messaggi, ma mette a disposizione solo
metodi astrusi per condividere lo stato tra i _thread_. Supportare solo un
sottoinsieme di soluzioni possibili è una strategia ragionevole per i linguaggi
di livello superiore, perché un linguaggio di livello superiore promette di
trarre vantaggio dalla rinuncia a un po' di controllo per ottenere astrazioni.
Tuttavia, i linguaggi di livello inferiore devono fornire la soluzione con le
migliori prestazioni in ogni situazione e hanno meno astrazioni sull'hardware.
Per questo motivo, Rust offre una varietà di strumenti per modellare i problemi
in qualsiasi modo sia appropriato per la tua situazione e i tuoi requisiti.

Ecco gli argomenti che tratteremo in questo capitolo:

- Come creare _thread_ per eseguire più parti di codice contemporaneamente
- Concorrenza con _passaggio di messaggi_ (_Message-passing_ concurrency), in
  cui i canali inviano messaggi tra i _thread_
- Concorrenza con _stato condiviso_ (_Shared-state_ concurrency), in cui più
  _thread_ hanno accesso ad alcuni dati
- I _trait_ `Sync` e `Send`, che estendono le garanzie di concorrenza di Rust ai
  _type_ definiti dall'utente oltre che ai _type_ forniti dalla libreria
  standard
