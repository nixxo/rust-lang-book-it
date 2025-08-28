## Separare i Moduli in File Diversi

Finora tutti gli esempi di questo capitolo definivano più moduli in un unico
file. Quando i moduli diventano grandi, potresti voler spostare le loro
definizioni in file separati per rendere il codice più facile da navigare.

Per esempio, partiamo dal codice nel Listato 7-17 che aveva più moduli del
ristorante. Metteremo ogni modulo in un file invece di avere tutte le
definizioni nella radice del _crate_. In questo caso, il file radice del _crate_
è _src/lib.rs_, ma questa procedura funziona anche con _crate_ binari il cui
file radice è _src/main.rs_.

Per prima cosa spostiamo il modulo `sala` in un proprio file. Rimuovi il codice
dentro le parentesi graffe del modulo `sala`, lasciando solo la dichiarazione
`mod sala;`, così che _src/lib.rs_ contenga il codice mostrato nel Listato 7-21.
Nota che questo non compilerà finché non creiamo il file _src/sala.rs_ mostrato
nel Listato 7-22.

<Listing number="7-21" file-name="src/lib.rs" caption="Dichiarare il modulo `sala` il cui corpo sarà in *src/sala.rs*">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

</Listing>

Ora, metti il codice che era dentro le parentesi graffe in un nuovo file
chiamato _src/sala.rs_, come mostrato nel Listato 7-22. Il compilatore sa di
dover cercare in questo file perché ha incontrato la dichiarazione del modulo
nella radice del _crate_ con il nome `sala`.

<Listing number="7-22" file-name="src/sala.rs" caption="Definizioni all'interno del modulo `sala` in *src/sala.rs*">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/sala.rs}}
```

</Listing>

Nota che è necessario caricare un file usando una dichiarazione `mod` _una sola
volta_ nell'albero dei moduli. Una volta che il compilatore sa che il file fa
parte del progetto (e sa dove si trova nell'albero dei moduli grazie a dove hai
messo la dichiarazione `mod`), gli altri file del progetto dovrebbero riferirsi
al codice del file caricato usando un percorso verso il punto in cui è stato
dichiarato, come trattato nella sezione [“Percorsi per fare riferimento a un
elemento nell'albero dei moduli”][paths]<!-- ignore -->. In altre parole, `mod`
_non_ è un'operazione di “include” come potresti aver visto in altri linguaggi.

Successivamente, sposteremo il modulo `accoglienza` in un suo file. Il processo
è un po' diverso perché `accoglienza` è un modulo figlio di `sala`, non della
radice. Metteremo il file per `accoglienza` in una nuova directory che sarà
chiamata come i suoi antenati nell'albero dei moduli, in questo caso _src/sala_.

Per iniziare a spostare `accoglienza`, cambiamo _src/sala.rs_ in modo che
contenga solo la dichiarazione del modulo `accoglienza`:

<Listing file-name="src/sala.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/sala.rs}}
```

</Listing>

Poi creiamo una directory _src/sala_ e un file _accoglienza.rs_ per contenere le
definizioni del modulo `accoglienza`:

<Listing file-name="src/sala/accoglienza.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/sala/accoglienza.rs}}
```

</Listing>

Se invece mettessimo _accoglienza.rs_ nella directory _src_, il compilatore si
aspetterebbe che il codice di _accoglienza.rs_ sia in un modulo `accoglienza`
dichiarato nella radice del _crate_, e non come figlio del modulo `sala`. Le
regole del compilatore su quali file cercare per il codice di quali moduli fanno
sì che directory e file rispecchino più da vicino l'albero dei moduli.

> ### Percorsi di File Alternativi
>
> Finora abbiamo coperto i percorsi di file più idiomatici che il compilatore
> Rust usa, ma Rust supporta anche uno stile più vecchio. Per un modulo chiamato
> `sala` dichiarato nella radice del _crate_, il compilatore cercherà il codice
> del modulo in:
> - _src/sala.rs_ (quello che abbiamo visto)
> - _src/sala/mod.rs_ (stile più vecchio, ancora supportato)
>
> Per un modulo chiamato `accoglienza` che è un sotto-modulo di `sala`, il
> compilatore cercherà il codice del modulo in:
>
> - _src/sala/accoglienza.rs_ (quello che abbiamo visto)
> - _src/sala/accoglienza/mod.rs_ (stile più vecchio, ancora supportato)
>
> Se usi entrambi gli stili per lo stesso modulo, otterrai un errore del
> compilatore. Usare una combinazione di stili diversi per moduli differenti
> nello stesso progetto è permesso, ma potrebbe confondere chi oltre a te prende
> in mano il progetto.
>
> Lo svantaggio principale dello stile con file chiamati _mod.rs_ è che il
> progetto può finire con molti file chiamati _mod.rs_, il che può diventare
> confusionario quando li hai aperti contemporaneamente nell'editor.

Abbiamo spostato il codice di ogni modulo in file separati e l'albero dei moduli
rimane lo stesso. Le chiamate di funzione in `mangiare_al_ristorante`
funzioneranno senza alcuna modifica, anche se le definizioni vivono in file
diversi. Questa tecnica ti permette di muovere i moduli in nuovi file man mano
che crescono di dimensione.

Nota che la dichiarazione `pub use crate::sala::accoglienza` in _src/lib.rs_ non
è cambiata, né `use` ha alcun impatto su quali file vengono compilati come parte
del _crate_. La parola chiave `mod` dichiara moduli, e Rust cerca in un file con
lo stesso nome del modulo il codice che va in quel modulo.

## Riepilogo

Rust ti permette di dividere un pacchetto in più _crate_ e un _crate_ in moduli
così da poter riferire elementi definiti in un modulo da un altro modulo. Puoi
farlo specificando percorsi assoluti o relativi. Questi percorsi possono essere
portati nello _scope_ con una dichiarazione `use` così da poter usare un
percorso più corto per usi ripetuti dell'elemento in quello _scope_. Il codice
dei moduli è privato per default, ma puoi rendere le definizioni pubbliche
aggiungendo la parola chiave `pub`.

Nel prossimo capitolo vedremo alcune strutture dati di collezione nella standard
library che puoi usare per rendere ancor più ordinatop il tuo codice.

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
