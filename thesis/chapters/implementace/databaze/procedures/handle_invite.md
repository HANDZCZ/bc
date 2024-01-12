
### Procedury handle_player_invite a handle_manager_invite

Tyto procedury obstarávají přijetí nebo odmítnutí pozvánky do týmu.
Nejdříve zjistí, jestli uživatel dostal pozvánku do týmu, pokud ne tak vrátí chybu.
Poté zjistí, jestli uživatel pozvánku přijímá nebo odmítá.
Podle toho jestli pozvánku přijal nebo odmítl,
tak procedura handle_player_invite upravuje tabulky players_to_teams ([@sec:table_players_to_teams]) a players_to_teams_invites ([@sec:table_players_to_teams_invites]).
V případě procedury handle_manager_invite jsou upravovány tabulky managers_to_teams ([@sec:table_managers_to_teams]) a managers_to_teams_invites ([@sec:table_managers_to_teams_invites]).
Konkrétněji, když uživatel pozvánku přijal, tak je přidán do týmu a pozvánka je smazána,
a když pozvánku odmítl tak je pozvánka smazána a do týmu přidán není.

