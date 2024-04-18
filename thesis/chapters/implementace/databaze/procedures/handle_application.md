
### Procedura handle_application

Tato procedura obstarává přijetí nebo odmítnutí přihlášky na turnaj.
Nejdříve zjistí, jestli přihláška existuje, pokud ne, tak vrátí chybu.
Podle toho, jestli je přihláška přijata nebo odmítnuta,
tak procedura handle_application upravuje tabulky teams_to_tournaments ([@sec:table_teams_to_tournaments]) a teams_to_tournaments_applications ([@sec:table_teams_to_tournaments_applications]).
Konkrétněji, když manažer turnaje přihlášku přijme, tak je přidán záznam do tabulky teams_to_tournaments ([@sec:table_teams_to_tournaments])
a přihláška je smazána z tabulky teams_to_tournaments_applications ([@sec:table_teams_to_tournaments_applications]),
a když přihlášku odmítne, tak je přihláška smazána z tabulky teams_to_tournaments_applications ([@sec:table_teams_to_tournaments_applications]) a do turnaje tým přidán není.

