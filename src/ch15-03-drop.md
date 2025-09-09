# Esecuzione del Codice Durante la Pulizia con il Trait `Drop`

Il secondo tratto importante per il pattern dei puntatori intelligenti è `Drop`, che consente
di personalizzare ciò che accade quando un valore sta per uscire dall'ambito. È possibile
fornire un'implementazione per il tratto `Drop` su qualsiasi tipo e tale codice può
essere utilizzato per rilasciare risorse come file o connessioni di rete.

Stiamo introducendo `Drop` nel contesto dei puntatori intelligenti perché la
funzionalità del tratto `Drop` viene quasi sempre utilizzata quando si implementa un
puntatore intelligente. Ad esempio, quando un `Box<T>` viene eliminato, dealloca
lo spazio sull'heap a cui punta il box.

In alcuni linguaggi, per alcuni tipi, il programmatore deve richiamare il codice per liberare memoria
o risorse ogni volta che termina di utilizzare un'istanza di quei tipi. Esempi
includono handle di file, socket e lock. Se il programmatore dimentica di farlo, il sistema potrebbe
sovraccaricarsi e bloccarsi. In Rust, è possibile specificare che un particolare pezzo di
codice venga eseguito ogni volta che un valore esce dallo scope, e il compilatore inserirà
questo codice automaticamente. Di conseguenza, non è necessario prestare attenzione a
inserire codice di pulizia ovunque in un programma in cui un'istanza di un particolare
tipo è terminata: non si perderanno comunque risorse!

Si specifica il codice da eseguire quando un valore esce dallo scope implementando il
tratto `Drop`. Il tratto `Drop` richiede l'implementazione di un metodo chiamato
`drop` che accetta un riferimento mutabile a `self`. Per vedere quando Rust chiama `drop`,
implementiamo per ora `drop` con istruzioni `println!`.

Il Listato 15-14 mostra una struttura `MioSmartPointer` la cui unica funzionalità
personalizzata è quella di stampare `Pulizia MioSmartPointer!` quando l'istanza
esce dallo scope, per mostrare quando Rust esegue il metodo `drop`.

<Listing number="15-14" file-name="src/main.rs" caption="Una struttura `MioSmartPointer` che implementa il tratto `Drop` dove inseriremmo il nostro codice di pulizia">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-14/src/main.rs}}
```

</Listing>

Il tratto `Drop` è incluso nel preludio, quindi non è necessario inserirlo
nell'ambito. Implementiamo il tratto `Drop` su `MioSmartPointer` e forniamo un'
implementazione per il metodo `drop` che chiama `println!`. Il corpo del
metodo `drop` è dove inseriremmo qualsiasi logica che si desidera eseguire quando un'istanza
del tipo esce dall'ambito. Stiamo stampando del testo qui per
mostrare visivamente quando Rust chiamerà `drop`.

In `main`, creiamo due istanze di `MioSmartPointer` e poi stampiamo
`MioSmartPointers creati`. Alla fine di `main`, le nostre istanze di
`MioSmartPointer` usciranno dallo scope e Rust chiamerà il codice che abbiamo inserito
nel metodo `drop`, stampando il nostro messaggio finale. Si noti che non è stato necessario
chiamare esplicitamente il metodo `drop`.

Quando eseguiamo questo programma, vedremo il seguente output:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-14/output.txt}}
```

Rust ha chiamato automaticamente `drop` per noi quando le nostre istanze sono uscite dallo scope,
chiamando il codice che abbiamo specificato. Le variabili vengono eliminate nell'ordine inverso
della loro creazione, quindi `d` è stata eliminata prima di `c`. Lo scopo di questo esempio è
fornire una guida visiva al funzionamento del metodo `drop`; Di solito, si specifica il codice di pulizia che il tipo deve eseguire anziché un messaggio di stampa.

<!-- Old link, do not remove -->

<a id="dropping-a-value-early-with-std-mem-drop"></a>

Purtroppo, non è semplice disabilitare la funzionalità automatica `drop`.
Disabilitare `drop` di solito non è necessario; lo scopo del tratto `Drop` è che venga gestito automaticamente. Occasionalmente, tuttavia,
potrebbe essere necessario pulire un valore in anticipo. Un esempio è quando si utilizzano puntatori intelligenti che gestiscono i blocchi: si potrebbe voler forzare il metodo `drop` che
rilascia il blocco in modo che altro codice nello stesso ambito possa acquisirlo.
Rust non consente di chiamare manualmente il metodo `drop` del tratto `Drop`; invece,
è necessario chiamare la funzione `std::mem::drop` fornita dalla libreria standard
se si desidera forzare l'eliminazione di un valore prima della fine del suo ambito.

Se proviamo a chiamare manualmente il metodo `drop` del tratto `Drop` modificando la
funzione `main` del Listato 15-14, come mostrato nel Listato 15-15, otterremo un
errore del compilatore.

<Listing number="15-15" file-name="src/main.rs" caption="Tentativo di chiamare manualmente il metodo `drop` dal tratto `Drop` per una pulizia anticipata">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-15/src/main.rs:here}}
```

</Listing>

Quando proviamo a compilare questo codice, otterremo questo errore:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-15/output.txt}}
```

Questo messaggio di errore indica che non siamo autorizzati a chiamare esplicitamente `drop`. Il
messaggio di errore utilizza il termine _destructor_, che è il termine di programmazione generale
per una funzione che pulisce un'istanza. Un _destructor_ è analogo a un
_costructor_, che crea un'istanza. La funzione `drop` in Rust è un
distruttore particolare.

Rust non ci permette di chiamare `drop` esplicitamente perché Rust chiamerebbe comunque
automaticamente `drop` sul valore alla fine di `main`. Questo causerebbe un errore di tipo `double free` perché Rust cercherebbe di ripulire lo stesso valore
due volte.

Non possiamo disabilitare l'inserimento automatico di `drop` quando un valore esce
dall'ambito, e non possiamo chiamare esplicitamente il metodo `drop`. Quindi, se dobbiamo forzare
la pulizia anticipata di un valore, usiamo la funzione `std::mem::drop`.

La funzione `std::mem::drop` è diversa dal metodo `drop` nel tratto `Drop`. La chiamiamo passando come argomento il valore che vogliamo forzare.
La funzione si trova nel preludio, quindi possiamo modificare `main` nel Listato 15-15 per
chiamare la funzione `drop`, come mostrato nel Listato 15-16.

<Listing number="15-16" file-name="src/main.rs" caption="Chiamata a `std::mem::drop` per eliminare esplicitamente un valore prima che esca dall'ambito">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-16/src/main.rs:here}}
```

</Listing>

L'esecuzione di questo codice stamperà quanto segue:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-16/output.txt}}
```

Il testo ``Eliminazione di MioSmartPointer con dati `alcuni dati`!`` viene stampato
tra `MioSmartPointer creato.` e `MioSmartPointer dropped
prima della fine del testo main.`, a dimostrazione del fatto che il codice del metodo `drop` è chiamato per
eliminare `c` in quel punto.

È possibile utilizzare il codice specificato in un'implementazione del tratto `Drop` in molti modi per
rendere la pulizia comoda e sicura: ad esempio, è possibile utilizzarlo per creare il proprio
allocatore di memoria! Con il tratto `Drop` e il sistema di proprietà di Rust,
non è necessario ricordarsi di pulire perché Rust lo fa automaticamente.

Inoltre, non è necessario preoccuparsi dei problemi derivanti dalla pulizia accidentale
di valori ancora in uso: il sistema di proprietà che garantisce
che i riferimenti siano sempre validi garantisce anche che `drop` venga chiamato solo una volta quando
il valore non è più in uso.

Ora che abbiamo esaminato `Box<T>` e alcune delle caratteristiche dei puntatori intelligenti,
diamo un'occhiata ad alcuni altri puntatori intelligenti definiti nella libreria
standard.