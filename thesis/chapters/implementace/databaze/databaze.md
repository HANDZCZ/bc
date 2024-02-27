
## Databázová část

Tato část se bude zabývat tím jak byla implementována databázová část.
Celý diagram databázových tabulek je uveden v obrázku [-@fig:database_table_diagram].

``` {.include}
tables/users.md
tables/teams.md
tables/games.md
tables/tournaments.md
tables/roles.md
tables/roles_to_users.md
tables/teams_to_tournaments.md
tables/teams_to_tournaments_applications.md
tables/managers_to_teams.md
tables/managers_to_teams_invites.md
tables/players_to_teams.md
tables/players_to_teams_invites.md
tables/players_to_tournaments_playing.md
tournament_type.md
tables/bracket_trees.md
tables/brackets.md
views/teams_tournaments_playing_players.md
views/teams_with_players_and_managers.md
views/tournaments_and_game_info.md
views/tournaments_signed_up_teams.md
views/tournaments_team_applications.md
views/users_and_roles.md
views/tournaments_with_bracket_trees_and_game_info.md
views/user_invites.md
triggers/players_to_teams_insert_trigger.md
triggers/managers_to_teams_insert_trigger.md
triggers/tournaments_update_trigger.md
triggers/teams_to_tournaments_insert_trigger.md
triggers/brackets_update_trigger.md
triggers/managers_to_teams_delete_trigger.md
procedures/handle_invite.md
procedures/invite_to_team.md
procedures/remove_from_team.md
procedures/team_ops.md
procedures/new_team.md
procedures/apply_for_tournament.md
procedures/set_playing.md
procedures/handle_application.md
procedures/handle_leave_tournament.md
```
