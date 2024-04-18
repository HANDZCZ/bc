
### Tabulka managers_to_teams {#sec:table_managers_to_teams}

Tabulka managers_to_teams slouží ke spojení uživatelů a týmů.
Jejím hlavním úkolem je umožnit backendu zjistit, jací uživatelé jsou manažeři daného týmu.
Tato tabulka obsahuje jen manažery, kteří již potvrdily pozvánku do týmu.

![Tabulka managers_to_teams](../../../../pictures/databaze/tables/managers_to_teams.png){ height=8.5% }

Team_id vyjadřuje id navázaného týmu ([@sec:table_teams]).

Manager_id vyjadřuje id navázaného uživatele ([@sec:table_users]).

Primární klíč je složen z team_id a manager_id.

