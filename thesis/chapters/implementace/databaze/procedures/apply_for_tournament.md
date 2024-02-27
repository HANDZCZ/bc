
### Procedura apply_for_tournament {#sec:procedure_apply_for_tournament}

Tato procedura obstarává podání přihlášky na turnaj.
Nejprve zjistí, jestli tým již podal přihlášku na tento turnaj, pokud ano tak vrátí chybu.
Dále zjistí, jestli turnaj a tým existují, pokud ne tak vrátí chybu.
Poté zjistí, jestli uživatel co volá tuto proceduru je manažerem týmu, pokud není, tak vrátí chybu.
Poslední kontrolou je kontrola, jestli jsou přihlášky uzavřeny, pokud jsou uzavřeny, tak se také vrátí chyba.
Když proběhnou všechny tyto kontroly a ani jedna nevrátí chybu,
tak je podle hodnoty sloupce requires_application uložené v tabulce tournaments ([@sec:table_tournaments])
přidán tým buď do tabulky teams_to_tournaments_applications ([@sec:table_teams_to_tournaments_applications]),
a nebo do tabulky teams_to_tournaments ([@sec:table_teams_to_tournaments]).

