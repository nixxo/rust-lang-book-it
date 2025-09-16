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

echo "> $file_path DONE"


# traduzione pagine html

book_path=(book/*.html)
declare -A text_pairs
text_pairs=(
    ["\"Table of contents\""]="\"Tabella dei contenuti\""
    ["\"Toggle Table of Contents\""]="\"Mostra\/Nascondi tabella contenuti\""
    ["\"Change theme\""]="\"Cambia tema\""
    ["\"Search (\`\/\`)\""]="\"Cerca (\`\/\`)\""
    ["\"Toggle Searchbar\""]="\"Mostra\/Nascondi barra ricerca\""
    ["\"Search this book ...\""]="\"Cerca nel libro ...\""
    ["\"Print this book\""]="\"Stampa questo libro\""
    ["\"Previous chapter\""]="\"Capitolo precedente\""
    ["\"Next chapter\""]="\"Prossimo capitolo\""
)

for fname in "${book_path[@]}"; do
    for key in "${!text_pairs[@]}"; do
        value="${text_pairs[$key]}"
        sed -i "s/$key/$value/g" "$fname"
    done
    echo "> $fname DONE"
done


# fix regex per supportare lettere accentate nei nomi funzione
fname="book/highlight.js"
sed -i 's/{className:\"title",begin:\"\[a-zA-Z_\]/{className:\"title\",begin:\"\[a-zA-Z_àèéìòù\]/g' $fname

echo "> $fname DONE"