# Capire la _Ownership_

Il _controllo esclusivo_ (la _ownership_ d'ora in poi) è la caratteristica più
qualificante di Rust e ha profonde implicazioni per il resto del linguaggio.
Permette a Rust di garantire la sicurezza della memoria senza bisogno di un
[_garbage collector_][gc], quindi è importante capire come funziona la
_ownership_. In questo capitolo parleremo della _ownership_ e di diverse
caratteristiche correlate: il _prestito_ (_borrowing_ d'ora in poi), le _slice_
e il modo in cui Rust dispone i dati in memoria

[gc]: https://it.wikipedia.org/wiki/Garbage_collection