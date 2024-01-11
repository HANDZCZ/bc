
### Pohled teams_players_to_tournaments_playing {#sec:view_teams_players_to_tournaments_playing}

Pohled teams_players_to_tournaments_playing slouží ke přehlednějšímu zobrazení a jednoduššímu načtení dat.
Jeho hlavním úkolem je zobrazit jestli hráči v týmu hrají v daném turnaji nebo ne.
Tento pohled umožňuje jednoduché načtení a vyhledání dat backendem.

![Pohled teams_players_to_tournaments_playing](../../../../pictures/databaze/views/teams_players_to_tournaments_playing.png){ height=10% }

Id vyjadřuje id týmu ([@sec:table_teams]).

Tournament_id vyjadřuje id turnaje ([@sec:table_tournaments]).

Sloupec players obsahuje pole s hráči ve formátu json.
Hráči dále obsahují informace jak o sobě, tak i o tom jestli hrají v turnaji za daný tým.

```{.json .linenos}
[{"player_id": "d685f026-f505-4e59-a927-e91f11f92cf0", "nick": "TEST :)", "playing": true}, 
 {"player_id": "264fb521-ba85-4572-931b-d22157b69b2d", "nick": "TEST 2", "playing": false}]
```

: Pohled teams_players_to_tournaments_playing ([@sec:view_teams_players_to_tournaments_playing]) - příklad hodnoty sloupce players {#lst:view_teams_players_to_tournaments_playing_players_example}

