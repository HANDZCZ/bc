
### Procedury remove_players_from_team<br>a remove_managers_from_team

Tyto procedury obstarávají odebrání uživatelů z týmu.
Nejdříve zjistí, jestli tým existuje, pokud neexistuje, tak vrátí chybu.
Dále zjistí, jestli uživatel, který chce odebrat jiné uživatele z týmu, je manažerem týmu,
pokud není manažerem, tak vrátí chybu.
Poté co proběhnou tyto kontroly, jsou uživatelé odebráni z tabulky players_to_teams ([@sec:table_players_to_teams]) pro odebrání hráče.
Pro odebrání manažera je prováděna ještě jedna kontrola
a to kontrola, jestli odebíraný uživatel je stejný jako uživatel, který zavolal tuto proceduru.
Pokud je odebíraný uživatel stejný, tak se z tabulky managers_to_teams ([@sec:table_managers_to_teams]) neodebere,
ale pokud je odebíraný uživatel jiný než uživatel, co zavolal tuto proceduru, tak je odebrán.

