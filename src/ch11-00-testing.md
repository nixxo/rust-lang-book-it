# Scrivere Test Automatizzati

Nel suo saggio del 1972 "The Humble Programmer", Edsger W. Dijkstra ha affermato
che "il testing dei programmi può essere un modo molto efficace per mostrare la
presenza di bug, ma è irrimediabilmente inadeguato per mostrarne l'assenza."
Questo non significa che non dovremmo cercare di testare il più possibile!

La correttezza dei nostri programmi è la misura in cui il nostro codice fa ciò
che intendiamo fare. Rust è stato progettato con un alto grado di preoccupazione
per la correttezza dei programmi, ma la correttezza è complessa e non facile da
dimostrare. Il sistema dei _type_ di Rust si fa carico di gran parte di questo
onere, ma il sistema dei _type_ non può catturare tutto. Per questo motivo, Rust
include un supporto per la scrittura di test software automatizzati.

Immaginiamo di scrivere una funzione `aggiungi_due` che aggiunge 2 a qualsiasi
numero le venga passato. La firma di questa funzione accetta un intero come
parametro e restituisce un intero come risultato. Quando implementiamo e
compiliamo questa funzione, Rust esegue tutti i controlli di _type_ e di
prestito (_borrow_) che hai imparato finora per assicurarsi che, ad esempio, non
stiamo passando un valore `String` o un _reference_ non valido a questa
funzione. Ma Rust _non_ può controllare che questa funzione faccia esattamente
ciò che intendiamo, cioè restituire il parametro più 2 piuttosto che, ad
esempio, il parametro più 10 o il parametro meno 50! È qui che entrano in gioco
i test.

Possiamo scrivere dei test che verificano, ad esempio, che quando passiamo `3`
alla funzione `aggiungi_due`, il valore restituito è `5`. Possiamo eseguire
questi test ogni volta che apportiamo delle modifiche al nostro codice per
assicurarci che il comportamento corretto esistente non sia cambiato.

Il testing è un'abilità complessa: anche se non possiamo trattare in un solo
capitolo tutti i dettagli su come scrivere dei buoni test, in questo capitolo
parleremo dei meccanismi delle strutture di test di Rust. Parleremo delle
annotazioni e delle macro a tua disposizione quando scrivi i tuoi test, del
comportamento predefinito e delle opzioni fornite per l'esecuzione dei tuoi test
e di come organizzare i test in unità di test e test di integrazione.
