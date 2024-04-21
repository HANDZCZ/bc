
### Procedury delete_team a edit_team

Tyto procedury obstarávají správu týmu.
Nejdříve zjistí, jestli tým existuje, pokud neexistuje, tak vrátí chybu.
Dále zjistí, jestli uživatel, který zavolal tyto procedury, je manažerem týmu,
pokud není manažerem, tak vrátí chybu.
V případě zavolání procedury delete_team je ještě kontrolováno, jestli uživatel,
který zavolal tuto proceduru, je manažer turnaje,
pokud ano, tak se nevrací chyba, že není manažer týmu.
Poté, co proběhnou tyto kontroly, je v případě zavolání procedury edit_team provedena editace
a v případě zavolání procedury delete_team je tým smazán.

