## Appendice G: Come è Fatto Rust e “Nightly Rust”

Questa appendice parla di come viene sviluppato Rust e di come ciò influisca su
di te come sviluppatore Rust.

### Stabilità Senza Stagnazione

Come linguaggio, Rust si preoccupa molto della stabilità del tuo codice.
Vogliamo che Rust sia una base solida su cui costruire, e se le cose cambiassero
costantemente, questo sarebbe impossibile. Allo stesso tempo, se non possiamo
sperimentare nuove funzionalità, potremmo non scoprire difetti importanti fino a
dopo il rilascio, quando non possiamo più apportare cambiamenti.

La nostra soluzione a questo problema si chiama “stabilità senza stagnazione”, e
il nostro principio guida è questo: non dovrai mai temere di aggiornarti a una
nuova versione di Rust stabile. Ogni aggiornamento dovrebbe essere indolore, ma
dovrebbe anche portarti nuove funzionalità, meno bug e tempi di compilazione più
veloci.

### Choo, Choo! I Canali di Rilascio e Viaggiare sui Treni

Lo sviluppo di Rust segue un _orario ferroviario_. Cioè, tutto lo sviluppo
avviene sul ramo `master` del repository di Rust. I rilasci seguono un “modello
ferroviario” del rilascio software, usato da Cisco IOS e altri progetti
software. Ci sono tre _canali di rilascio_ per Rust:

- Nightly
- Beta
- Stable

La maggior parte degli sviluppatori Rust usa principalmente il canale _stable_
(_stabile_), ma chi vuole provare funzionalità sperimentali può usare _nightly_
o _beta_.

Ecco un esempio di come funziona il processo di sviluppo e rilascio: supponiamo
che il team di Rust stia lavorando al rilascio di Rust 1.5. Quel rilascio è
avvenuto nel dicembre 2015, ma ci fornisce numeri di versione realistici. Viene
aggiunta una nuova funzionalità a Rust: un nuovo _commit_ arriva sul _branch_
`master`. Ogni notte, viene prodotto un nuovo rilascio _nightly_ di Rust. Ogni
giorno è un giorno di rilascio, e questi rilasci sono creati automaticamente
dalla nostra infrastruttura. Quindi, col passare del tempo, i nostri rilasci
appaiono così, una volta per notte:

```text
nightly: * - - * - - *
```

Ogni sei settimane, è il momento di preparare un nuovo rilascio! Il _branch_
`beta` del repository Rust si dirama dal _branch_ `master` usato da _nightly_.
Ora, ci sono due rilasci:

```text
nightly: * - - * - - *
                     |
beta:                *
```

La maggior parte degli utenti Rust non usa attivamente i rilasci _beta_, ma li
testa nel loro sistema _CI_ per aiutare Rust a scoprire possibili regressioni.
Nel frattempo, c’è ancora un rilascio nightly ogni notte:

```text
nightly: * - - * - - * - - * - - *
                     |
beta:                *
```

Supponiamo che venga trovata una regressione. È un bene aver avuto tempo per
testare la _beta_ prima che la regressione entrasse in un rilascio stabile! La
correzione viene applicata a `master`, così _nightly_ viene sistemato, poi la
correzione viene portata anche sul _branch_ `beta`, e viene prodotto un nuovo
rilascio _beta_:

```text
nightly: * - - * - - * - - * - - * - - *
                     |
beta:                * - - - - - - - - *
```

Sei settimane dopo la creazione della prima _beta_, è il momento per un rilascio
stabile! Il _branch_ `stable` viene prodotto dal _branch_ `beta`:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *
```

Evviva! Rust 1.5 è pronto! Però abbiamo dimenticato una cosa: dato che sono
passate sei settimane, abbiamo bisogno anche di una nuova _beta_ della
_prossima_ versione di Rust, la 1.6. Quindi, dopo che `stable` si dirama da
`beta`, la prossima versione di `beta` si dirama di nuovo da `nightly`:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```

Questo si chiama “modello ferroviario” perché ogni sei settimane un rilascio
“parte dalla stazione”, ma deve ancora viaggiare attraverso il canale _beta_
prima di arrivare come rilascio stabile.

Rust rilascia ogni sei settimane, come un orologio. Se conosci la data di un
rilascio Rust, puoi conoscere la data del prossimo: è sei settimane dopo. Un
aspetto piacevole di avere rilasci programmati ogni sei settimane è che il
prossimo treno arriverà a breve. Se una funzionalità salta un rilascio, non c’è
bisogno di preoccuparsi: ce n’è un altro in arrivo a breve! Questo aiuta a
ridurre la pressione di inserire funzionalità possibilmente non finite vicino
alla scadenza del rilascio.

Grazie a questo processo, puoi sempre scaricare la prossima build di Rust e
verificare di persona che è facile da aggiornare: se un rilascio _beta_ non
funziona come previsto, puoi segnalarlo al team e farlo correggere prima che
avvenga il prossimo rilascio stabile! I problemi in un rilascio _beta_ sono
relativamente rari, ma `rustc` è comunque un software, e i bug esistono.

### Tempo di Manutenzione

Il progetto Rust supporta la versione stabile più recente. Quando una nuova
versione stabile viene rilasciata, la vecchia versione raggiunge la fine della
sua vita (_EOL_). Questo significa che ogni versione è supportata per sei
settimane.

### Funzionalità Instabili

C’è un altro aspetto in questo modello di rilascio: le funzionalità instabili.
Rust utilizza una tecnica chiamata “feature flags” per determinare quali
funzionalità sono abilitate in un certo rilascio. Se una nuova funzionalità è in
sviluppo attivo, arriva su `master` e quindi in _nightly_, ma dietro a una
_feature flag_. Se tu, come utente, vuoi provare questa funzionalità ancora in
sviluppo, puoi farlo, ma devi usare un rilascio _nightly_ di Rust e annotare il
codice sorgente con il flag appropriato per abilitare la funzionalità.

Se state usando una versione _beta_ o _stable_ di Rust, non puoi usare alcuna
_feature flag_. Questa è la chiave che ci permette di usare praticamente le
nuove funzionalità prima di dichiararle stabili per sempre. Chi vuole provare le
funzionalità più avanzate avanzate può farlo, e chi vuole un’esperienza solida
può restare sul canale stabile sapendo che il proprio codice non si romperà.
Stabilità senza stagnazione.

Questo libro contiene solo informazioni sulle funzionalità stabili, poiché
quelle in sviluppo sono ancora in cambiamento e sicuramente saranno diverse tra
il momento in cui questo libro è stato scritto e quando saranno abilitate nei
build stabili. Puoi trovare la documentazione per funzionalità presenti solo in
rilasci _nightly_ online.

### Rustup e il Ruolo di Rust Nightly

Rustup rende facile passare tra i diversi canali di rilascio di Rust, a livello
globale o per progetto. Di default verrà installato Rust stabile. Per installare
_nightly_, ad esempio:

```console
$ rustup toolchain install nightly
```

Puoi vedere tutte le _toolchain_ (versioni di Rust e componenti associati) che
hai installato con `rustup`. Ecco un esempio sul computer Windows di uno degli
autori:

```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

Come vedi, la toolchain _stable_ è quella predefinita. La maggior parte degli
utenti Rust usa _stable_ la maggior parte del tempo. Potresti voler usare
_stable_ la maggior parte del tempo, ma usare _nightly_ su un progetto
specifico, perché ti interessa una funzionalità all’avanguardia. Per farlo, puoi
usare `rustup override` nella directory di quel progetto per impostare la
_toolchain_ _nightly_ come quella che `rustup` deve usare quando sei in quella
directory:

```console
$ cd ~/progetti/usare-nightly
$ rustup override set nightly
```

Ora, ogni volta che chiami `rustc` o `cargo` dentro _~/progetti/usare-nightly_,
`rustup` si assicurerà che stai usando Rust _nightly_ invece del default
_stable_. Questo è molto comodo quando hai molti progetti Rust!

### Il Processo RFC e i Team

Come si viene a sapere di queste nuove funzionalità? Il modello di sviluppo di
Rust segue il processo _Request For Comments (RFC)_. Se vuoi un miglioramento in
Rust, puoi scrivere una proposta chiamata _RFC_.

Chiunque può scrivere _RFC_ per migliorare Rust, e le proposte vengono
revisionate e discusse dal team Rust, che è composto da molti sotto-team
tematici. C’è una lista completa dei team [sul sito di
Rust](https://www.rust-lang.org/governance), che include team per ogni area del
progetto: design del linguaggio, implementazione del compilatore,
infrastruttura, documentazione e altro ancora. Il team appropriato legge la
proposta e i commenti, scrive qualche commento a sua volta, e infine si
raggiunge un consenso per accettare o rifiutare la funzionalità.

Se la funzionalità è accettata, viene aperta una _issue_ sul _repository_ Rust,
e qualcuno la implementa. Chi implementa potrebbe non essere la stessa persona
che ha proposto la funzionalità! Quando l’implementazione è pronta, viene
inserita nel _branch_ `master` dietro a un _feature flag_, come discusso nella
sezione [“Funzionalità Instabili”](#funzionalità-instabili).

Dopo un po’ di tempo, quando gli sviluppatori Rust che usano i rilasci _nightly_
hanno potuto provare la nuova funzionalità, i membri del team discutono la
funzionalità, come è andata su _nightly_, e decidono se inserirla in Rust
stabile o no. Se la decisione è di andare avanti, il _feature flag_ viene
rimosso, e la funzionalità è ora considerata stabile! Viaggia sul treno verso un
nuovo rilascio _stable_ di Rust.
