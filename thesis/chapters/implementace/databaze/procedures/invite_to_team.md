
### Procedury invite_players_to_team<br>a invite_managers_to_team

Tyto procedury obstarávají pozvání uživatelů do týmu.
Nejdříve zjistí, jestli tým existuje, pokud neexistuje, tak vrátí chybu.
Dále zjistí, jestli uživatel, který chce pozvat jakéhokoli jiného uživatele do týmu, je manažerem týmu,
pokud není manažerem, tak vrátí chybu.
Poté, co proběhnou tyto kontroly, je vytvořena pozvánka v tabulce players_to_teams_invites ([@sec:table_players_to_teams_invites]) pro pozvání hráče
a pro pozvání manažera je pozvánka vytvořena v tabulce managers_to_teams_invites ([@sec:table_managers_to_teams_invites]).

