#!/bin/sh

echo 'Started conversion.'
echo ''

pandoc --standalone --listings --no-highlight --lua-filter ./filters/include-files.lua --lua-filter ./filters/include-code-files.lua --pdf-engine-opt=--shell-escape --pdf-engine-opt=--output-directory=temp --lua-filter ./filters/linebreaks.lua --filter pandoc-plot --filter pandoc-crossref --lua-filter ./filters/add-missing-source.lua --lua-filter ./filters/minted.lua --lua-filter ./filters/pandoc-plot-fix-figures.lua --template template.latex --pdf-engine lualatex --bibliography bib.yaml --citeproc --csl iso690-numeric-brackets-cs.csl --lua-filter ./filters/remove-citation-brackets.lua -o 'thesis.pdf' main.md

mv temp/input.pdf ./thesis.pdf
rm -r temp

echo ''
echo 'Finished.'
