# Enumerazioni e Corrispondenza dei Pattern

In questo capitolo parleremo delle _enumerazioni_, abbreviato in _enum_, termine
che sarà usato d'ora in poi. Gli _enum_ permettono di definire un _type_
enumerando le sue possibili _varianti_. Per prima cosa definiremo e useremo un
_enum_ per mostrare come un _enum_ possa codificare un significato insieme ai
dati. Poi esploreremo un _enum_ particolarmente utile, chiamato `Option`, che
consente di esprimere se un valore può essere o qualcosa o niente. Poi vedremo
come la corrispondenza dei _pattern_ nell'espressione `match` rende facile
eseguire codice diverso per valori diversi di un _enum_. Infine, vedremo come il
costrutto `if let` sia un altro idioma comodo e conciso disponibile per gestire
gli _enum_ nel tuo codice.