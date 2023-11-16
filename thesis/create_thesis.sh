#!/bin/sh

echo 'Started conversion.'
echo ''

pandoc --standalone --listings --no-highlight --lua-filter ./filters/include-files.lua --lua-filter ./filters/include-code-files.lua --pdf-engine-opt=--shell-escape --pdf-engine-opt=--output-directory=temp --filter pandoc-crossref --lua-filter ./filters/minted.lua --template template.latex --pdf-engine lualatex --bibliography bib.yaml --citeproc --csl iso690-numeric-brackets-cs.csl -o 'thesis.pdf' main.md

mv temp/input.pdf ./thesis.pdf
rm -r temp

echo ''
echo 'Finished.'
