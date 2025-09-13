#!/bin/bash

mdbook build

# traduzione book.js
file_path="book/book.js"
declare -A text_pairs
text_pairs=(
    ["\"Show hidden lines\""]="\"Mostra linee nascoste\""
    ["'Show hidden lines'"]="'Mostra linee nascoste'"
    ["'Hide lines'"]="'Nascondi linee'"
    ["'Copy to clipboard'"]="'Copia negli appunti'"
    ["'Copied!'"]="'Copiato!'"
    ["'Clipboard error!'"]="'Errore nella copia!'"
    ["'Run this code'"]="'Esegui questo codice'"
    ["'Undo changes'"]="'Annulla modifiche'"    
)

if [[ ! -f "$file_path" ]]; then
    echo "Il file $file_path non esiste."
    exit 1
fi

for key in "${!text_pairs[@]}"; do
    value="${text_pairs[$key]}"
    sed -i "s/$key/$value/g" "$file_path"
done

echo "Traduzione book.js completata con successo."

# fix regex per supportare lettere accentate
sed -i 's/{className:\"title",begin:\"\[a-zA-Z_\]/{className:\"title\",begin:\"\[a-zA-Z_àèéìòù\]/g' book/highlight.js
