#!/bin/bash

# Definizione della variabile array contenente coppie di testo
declare -A text_pairs
text_pairs=(
    ["\"Show hidden lines\""]="\"Mostra linee nascoste\""
    ["'Show hidden lines'"]="'Mostra linee nascoste'"
    ["'Hide lines'"]="'Nascondi linee'"
    ["'Copy to clipboard'"]="'Copia negli appunti'"
    ["'Run this code'"]="'Esegui questo codice'"
)

# Definizione della variabile che rappresenta il path al file di testo
file_path="book/book.js"

# Controllo se il file esiste
if [[ ! -f "$file_path" ]]; then
    echo "Il file $file_path non esiste."
    exit 1
fi

# Sostituzione delle stringhe di testo nel file
for key in "${!text_pairs[@]}"; do
    value="${text_pairs[$key]}"
    sed -i "s/$key/$value/g" "$file_path"
done

echo "Sostituzioni completate con successo."
