# Un progetto I/O: Creare un Programma da Riga di Comando

Questo capitolo è un riepilogo delle numerose competenze acquisite finora e un'
esplorazione di alcune funzionalità aggiuntive della libreria standard. Creeremo uno strumento da riga di comando
che interagisce con l'input/output di file e da riga di comando per mettere in pratica alcuni dei
concetti di Rust che ora avete acquisito.

La velocità, la sicurezza, l'output binario singolo e il supporto multipiattaforma di Rust lo rendono
un linguaggio ideale per la creazione di strumenti da riga di comando, quindi per il nostro progetto creeremo
la nostra versione del classico strumento di ricerca da riga di comando `grep`
(cerca **g**lobalmente un'**espressione regolare** e  **stampala**). Nel
caso d'uso più semplice, `grep` cerca una stringa come parmetro in un file specificato. Per
farlo, `grep` accetta come argomenti un percorso di file e una stringa. Quindi legge
il file, trova le righe in quel file che contengono l'argomento stringa e visualizza
quelle righe.

Lungo il percorso, mostreremo come far sì che il nostro strumento da riga di comando utilizzi le funzionalità del terminale
che molti altri strumenti da riga di comando utilizzano. Leggeremo il valore di una
variabile d'ambiente per consentire all'utente di configurare il comportamento del nostro programma.
Mostreremo anche i messaggi di errore nel flusso di errore standard della console (`stderr`)
invece che nell'output standard (`stdout`), in modo che, ad esempio, l'utente possa
reindirizzare l'output corretto a un file continuando a visualizzare i messaggi di errore sullo schermo.

Un membro della community Rust, Andrew Gallant, ha già creato una versione completa
e molto veloce di `grep`, chiamata `ripgrep`. In confronto, la nostra
versione sarà piuttosto semplice, ma questo capitolo vi fornirà alcune
conoscenze di base necessarie per comprendere un progetto reale come
`ripgrep`.

Il nostro progetto `grep` combinerà diversi concetti che hai imparato finora:

- Organizzazione del codice ([Capitolo 7][ch7]<!-- ignore -->)
- Utilizzo di vettori e stringhe ([Capitolo 8][ch8]<!-- ignore -->)
- Gestione degli errori ([Capitolo 9][ch9]<!-- ignore -->)
- Utilizzo di tratti e lifetimes quando opportuno ([Capitolo 10][ch10]<!-- ignore -->)
- Scrittura di test ([Capitolo 11][ch11]<!-- ignore -->)

Introdurremo anche brevemente chiusure, iteratori e oggetti traits, che
[Capitolo 13][ch13]<!-- ignore --> e [Capitolo 18][ch18]<!-- ignore -->
affronteremo in dettaglio.

[ch7]: ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
[ch8]: ch08-00-common-collections.html
[ch9]: ch09-00-error-handling.html
[ch10]: ch10-00-generics.html
[ch11]: ch11-00-testing.html
[ch13]: ch13-00-functional-features.html
[ch18]: ch18-00-oop.html
