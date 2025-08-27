# Gestione di Progetti in Crescita con Pacchetti, _Crates_, e Moduli

Quando scriverai programmi sempre più complessi, organizzare il tuo codice
diventerà sempre più importante. Raccogliendo funzionalità correlate e separando
il codice con caratteristiche distinte, chiarirai dove trovare il codice che
implementa una particolare funzionalità e quindi dove guardare per modificare il
codice di quella funzionalità se necessario.

I programmi che abbiamo scritto finora sono stati tutti implementati con un
singolo _modulo_ in un unico file. Man mano che un progetto cresce, dovresti
organizzare il codice suddividendolo in più _moduli_ e poi in più file. Un
_pacchetto_ (_package_) può contenere più _crate binari_ (_binary crate_) e
opzionalmente un _crate libreria_ (_library crate_). Man mano che un _pachetto_
cresce, puoi estrarne parti in _crate_ separate che diventeranno dipendenze
esterne. Questo capitolo copre tutte queste tecniche. Per progetti molto grandi
che comprendono un insieme di _pacchetti_ interconnessi che evolvono insieme,
Cargo fornisce degli _spazi di lavoro_ (_workspace_), che tratteremo in [“Cargo
Workspace”](ch14-03-cargo-workspaces.html)<!-- ignore --> nel Capitolo 14.

Discuteremo anche l'incapsulamento dei dettagli di implementazione, che ti
consente di riutilizzare il codice a un livello più alto: una volta implementata
un'operazione, l'altro codice può chiamare il tuo codice tramite la sua
interfaccia pubblica senza dover sapere come funziona l'implementazione. Il modo
in cui scrivi il codice definisce quali parti sono pubbliche per l'uso da parte
di altri codici e quali parti sono dettagli di implementazione privati che ti
riservi il diritto di modificare. Questo è un altro modo per limitare la
quantità di dettagli che devi tenere a mente.

Un concetto correlato è lo _scope_: il contesto annidato in cui è scritto il
codice ha un insieme di nomi definiti come "in _scope_". Quando si legge, si
scrive e si compila il codice, i programmatori e i compilatori devono sapere se
un particolare nome in un particolare punto si riferisce a una variabile,
funzione, _struct_, _enum_, modulo, costante o altro elemento e cosa significa
quell'elemento. Puoi creare _scope_ e modificare quali nomi sono in o fuori
_scope_. Non puoi avere due elementi con lo stesso nome nello stesso _scope_;
sono disponibili strumenti per risolvere i conflitti di nomenclatura.

Rust ha una serie di funzionalità che ti consentono di gestire l'organizzazione
del tuo codice, inclusi i dettagli esposti, i dettagli privati e quali nomi sono
in ogni _scope_ nei tuoi programmi. Queste funzionalità, a volte definite
collettivamente _sistema dei moduli_ (_module system_), includono:

* **Pacchetti**: Una funzionalità di Cargo che ti consente di costruire, testare
  e condividere _crate_
* **Crate**: Un albero di _moduli_ che produce una libreria o un eseguibile
* **Moduli** e **use**: Ti consentono di controllare l'organizzazione, lo
  _scope_ e la privacy dei percorsi (_paths_)
* **Path**: Un modo per nominare un elemento, come una _struct_, una funzione o
  un modulo

In questo capitolo, tratteremo tutte queste funzionalità, discuteremo come
interagiscono e spiegheremo come usarle per gestire lo _scope_. Alla fine,
dovresti avere una solida comprensione del _module system_ e essere in grado di
lavorare con gli _scope_ come un professionista!

[workspaces]: ch14-03-cargo-workspaces.html
