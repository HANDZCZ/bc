
### Tabulka managers_to_teams_invites {#sec:table_managers_to_teams_invites}

Tabulka managers_to_teams_invites slouží ke spojení uživatelů a týmů pro pozvánky.
Jejím hlavním úkolem je umožnit backendu zjistit jací uživatelé jsou pozvání do týmu jako manažeři.
Tato tabulka obsahuje jen pozvánky, které ještě nebyly přijaty.

![Tabulka managers_to_teams_invites](../../../pictures/databaze/managers_to_teams_invites.png){ height=8.5% }

Team_id vyjadřuje id navázaného týmu ([@sec:table_teams]).

Manager_id vyjadřuje id navázaného uživatele ([@sec:table_users]).

Primární klíč je složen z team_id a manager_id.

