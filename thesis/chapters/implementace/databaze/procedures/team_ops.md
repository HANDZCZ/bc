
### Procedury delete_team a edit_team

Tyto procedury obstarávají správu týmu.
Nejdříve zjistí, jestli tým existuje, pokud neexistuje tak vrátí chybu.
Dále zjistí, jestli uživatel, který zavolal tyto procedury, je manažerem týmu,
pokud není manažerem, tak vrátí chybu.
Poté co proběhnou tyto kontroly je provedena editace v případě zavolání procedury edit_team
a v případě zavolání procedury delete_team je tým smazán.

