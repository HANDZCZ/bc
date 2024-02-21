#!/bin/bash

# Function to display script usage
usage() {
    echo "Usage: $0 [--codeblocks]"
    exit 1
}

# Check if the script was called with the --help flag
if [ "$1" == "--help" ] || [ "$1" == "-h" ]; then
    usage
fi

# Check if the script was called with the --codeblocks flag
if [ "$1" == "--codeblocks" ]; then
    codeblocks_flag=true
    shift   # Remove the flag from the arguments
else
    codeblocks_flag=false
fi

pandoc --standalone --listings --no-highlight --lua-filter ./filters/include-files.lua --lua-filter ./filters/include-code-files.lua --pdf-engine-opt=--shell-escape --pdf-engine-opt=--output-directory=temp --filter pandoc-plot --filter pandoc-crossref --lua-filter ./filters/add-missing-source.lua -M wordcount-codeblocks=$codeblocks_flag --lua-filter ./filters/wordcount.lua --lua-filter ./filters/minted.lua --lua-filter ./filters/pandoc-plot-fix-figures.lua --template template.latex --pdf-engine lualatex --bibliography bib.yaml --citeproc --csl iso690-numeric-brackets-cs.csl --lua-filter ./filters/remove-citation-brackets.lua -o 'thesis.tex' main.md
