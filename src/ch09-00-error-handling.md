# Gestione degli Errori

Gli errori nel software sono una realtà, quindi Rust offre diverse funzionalità
per gestire le situazioni in cui qualcosa non va. In molti casi, Rust ti forza a
riconoscere la possibilità di un errore e ti chiede di intraprendere un'azione
prima che il codice venga compilato. Questo requisito rende il programma più
robusto, garantendo che gli errori vengano rilevati e gestiti in modo
appropriato prima di distribuire il tuo codice all'utente finale!

Rust raggruppa gli errori in due categorie principali: errori _reversibili_ e
_irreversibili_. Per un errore reversibile, come un errore _file non trovato_,
molto probabilmente vorremo semplicemente segnalare il problema all'utente e
riprovare l'operazione. Gli errori irreversibili sono sempre sintomi di bug,
come il tentativo di accedere a una posizione oltre la fine di un array, e
quindi vogliamo interrompere immediatamente il programma.

La maggior parte dei linguaggi non distingue tra questi due tipi di errori e li
gestisce entrambi allo stesso modo, utilizzando meccanismi come le eccezioni.
Rust non ha eccezioni. Al contrario, ha il _type_ `Result<T, E>` per gli errori
reversibile e la macro `panic!` che interrompe l'esecuzione quando il programma
incontra un errore irreversibile. Questo capitolo tratta prima la chiamata a
`panic!` e poi parla della restituzione dei valori `Result<T, E>`. Inoltre,
esploreremo le considerazioni da tenere presente quando si decide se tentare di
recuperare da un errore o interrompere l'esecuzione.