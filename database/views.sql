create or replace view users_and_roles as
select users.*,
       case
           when (array_agg(r.name))[1] is null then json_build_array()
           else json_agg(r.name)
           end as roles
from users
         left join roles_to_users rtu on users.id = rtu.user_id
         left join roles r on r.id = rtu.role_id
group by users.id;

create or replace view tournaments_and_game_info as
select tournaments.id,
       tournaments.name,
       tournaments.description,
       tournaments.min_team_size,
       tournaments.max_team_size,
       tournaments.requires_application,
       tournaments.applications_closed,
       tournaments.tournament_type,
       g.id          as game_id,
       g.name        as game_name,
       g.description as game_description,
       g.version     as game_version
from tournaments
         join games g on g.id = tournaments.game_id;

create or replace view tournaments_with_bracket_trees_and_game_info as
select tournaments.id,
       tournaments.name,
       tournaments.description,
       tournaments.min_team_size,
       tournaments.max_team_size,
       tournaments.requires_application,
       tournaments.applications_closed,
       tournaments.tournament_type,
       g.id                         as game_id,
       g.name                       as game_name,
       g.description                as game_description,
       g.version                    as game_version,
       (select coalesce(json_agg(d), json_build_array())
        from (select id,
                     position,
                     (select coalesce(json_agg(b), json_build_array())
                      from (select case
                                       when t1 is null then null
                                       else json_build_object('id', t1.id, 'name', t1.name)
                                       end as team1,
                                   case
                                       when t2 is null then null
                                       else json_build_object('id', t2.id, 'name', t2.name)
                                       end as team2,
                                   winner,
                                   layer,
                                   team1_score,
                                   team2_score,
                                   brackets.position
                            from brackets
                                     left join teams t1 on t1.id = brackets.team1
                                     left join teams t2 on t2.id = brackets.team2
                            where bracket_tree_id = bracket_trees.id
                            order by layer, position) as b) as brackets
              from bracket_trees
              where tournament_id = tournaments.id
              order by position) d) as bracket_trees
from tournaments
         join games g on g.id = tournaments.game_id;


create or replace view teams_with_players_and_managers as
select teams.id,
       teams.name,
       teams.description,
       (select coalesce(json_agg(a), json_build_array())
        from (select u.*
              from players_to_teams
                       join users u on players_to_teams.player_id = u.id
              where team_id = teams.id) as a) as players,
       (select coalesce(json_agg(a), json_build_array())
        from (select u.id, u.nick
              from managers_to_teams
                       join users u on managers_to_teams.manager_id = u.id
              where team_id = teams.id) as a) as managers
from teams;

create or replace view tournaments_signed_up_teams as
select tournaments.id,
       (select coalesce(json_agg(d), json_build_array())
        from (select *,
                     num_playing <= tournaments.max_team_size and
                     num_playing >= tournaments.min_team_size as valid_team_size
              from (select t.*,
                           (select count(*)
                            from players_to_tournaments_playing
                            where players_to_tournaments_playing.tournament_id = tournaments.id
                              and players_to_tournaments_playing.team_id = teams_to_tournaments.team_id) as num_playing
                    from teams_to_tournaments
                             join teams t on t.id = teams_to_tournaments.team_id
                    where tournament_id = tournaments.id) b) d) as teams
from tournaments;

create or replace view tournaments_team_applications as
select tournaments.id,
       (select coalesce(json_agg(b), json_build_array())
        from (select t.*
              from teams_to_tournaments_applications
                       join teams t on t.id = teams_to_tournaments_applications.team_id
              where tournament_id = tournaments.id) b) as teams
from tournaments;

create or replace view teams_tournaments_playing_players as
select teams.id                            as team_id,
       teams.name                          as team_name,
       t.id as tournament_id,
       t.name as tournament_name,
       (select coalesce(json_agg(a), json_build_array())
        from (select player_id,
                     u.nick,
                     exists(select *
                            from players_to_tournaments_playing
                            where players_to_tournaments_playing.team_id = players_to_teams.team_id
                              and players_to_tournaments_playing.player_id = players_to_teams.player_id
                              and players_to_tournaments_playing.tournament_id = ttt.tournament_id) as playing
              from players_to_teams
                       join users u on u.id = players_to_teams.player_id
              where team_id = teams.id) a) as players
from teams
         join teams_to_tournaments ttt on teams.id = ttt.team_id
         join tournaments t on ttt.tournament_id = t.id;


create or replace view user_invites as
select users.id,
       (select coalesce(json_agg(a), json_build_array())
        from (select t.id, t.name
              from players_to_teams_invites
                       join teams t on t.id = players_to_teams_invites.team_id
              where player_id = users.id) a)  as player_invites,
       (select coalesce(json_agg(a), json_build_array())
        from (select t.id, t.name
              from managers_to_teams_invites
                       join teams t on t.id = managers_to_teams_invites.team_id
              where manager_id = users.id) a) as manager_invites
from users;