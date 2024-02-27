/*
players
*/

create or replace function players_to_teams_insert_trigger_procedure()
    returns trigger
    language plpgsql as
$$
declare
    v_exists bool;
begin
    select exists(select *
                  from players_to_teams
                  where players_to_teams.team_id = new.team_id
                    and players_to_teams.player_id = new.player_id)
    into v_exists;

    if v_exists then
        return null;
    end if;
    return new;
end
$$;

create trigger players_to_teams_insert_trigger
    before insert
    on players_to_teams_invites
    for each row
execute procedure players_to_teams_insert_trigger_procedure();


/*
managers
*/

create or replace function managers_to_teams_insert_trigger_procedure()
    returns trigger
    language plpgsql as
$$
declare
    v_exists bool;
begin
    select exists(select *
                  from managers_to_teams
                  where managers_to_teams.team_id = new.team_id
                    and managers_to_teams.manager_id = new.manager_id)
    into v_exists;

    if v_exists then
        return null;
    end if;
    return new;
end
$$;

create trigger managers_to_teams_insert_trigger
    before insert
    on managers_to_teams_invites
    for each row
execute procedure managers_to_teams_insert_trigger_procedure();

/* delete */
create or replace function managers_to_teams_delete_trigger_procedure()
    returns trigger
    language plpgsql as
$$
declare
    v_exists bool;
begin
    select exists(select *
                  from managers_to_teams
                  where managers_to_teams.team_id = old.team_id
                    and managers_to_teams.manager_id != old.manager_id)
    into v_exists;

    if not v_exists then
        delete from teams where id = old.team_id;
    end if;
    return old;
end
$$;

create trigger managers_to_teams_delete_trigger
    after delete
    on managers_to_teams
    for each row
execute procedure managers_to_teams_delete_trigger_procedure();

/*
tournaments
*/

create or replace function tournaments_update_trigger_procedure()
    returns trigger
    language plpgsql as
$$
begin
    /* don't change tournament type if applications are closed */
    if old.applications_closed and new.applications_closed and old.tournament_type != new.tournament_type then
        raise exception 'tournament type cannot be changed if tournament applications are closed' using errcode = 44444;
    end if;

    /* delete all bracket trees when applications are reopened */
    if old.applications_closed and not new.applications_closed then
        delete from bracket_trees where tournament_id = new.id;
    end if;

    /* delete not approved applications */
    if new.applications_closed then
        delete from teams_to_tournaments_applications where tournament_id = new.id;
    end if;

    /* it the tournament changes to not requiring applications auto approve all */
    if not new.requires_application then
        insert into teams_to_tournaments (team_id, tournament_id)
        select team_id, tournament_id
        from teams_to_tournaments_applications
        where tournament_id = new.id
        on conflict do nothing;

        delete from teams_to_tournaments_applications where tournament_id = new.id;
    end if;

    return new;
end
$$;

create trigger tournaments_update_trigger
    before update
    on tournaments
    for each row
execute procedure tournaments_update_trigger_procedure();


create or replace function teams_to_tournaments_insert_trigger_procedure()
    returns trigger
    language plpgsql as
$$
declare
    v_exists bool;
begin
    select exists(select *
                  from teams_to_tournaments
                  where teams_to_tournaments.team_id = new.team_id
                    and teams_to_tournaments.tournament_id = new.tournament_id)
    into v_exists;

    if v_exists then
        return null;
    end if;
    return new;
end
$$;

create trigger teams_to_tournaments_insert_trigger
    before insert
    on teams_to_tournaments_applications
    for each row
execute procedure teams_to_tournaments_insert_trigger_procedure();

/*
brackets
*/

create or replace function brackets_update_trigger_procedure()
    returns trigger
    language plpgsql as
$$
declare
    v_exists bool;
begin
    if new.team1 is null and new.team2 is null and new.winner is not null then
        new.team1 := old.team1;
        new.team2 := old.team2;
    end if;

    if new.team1 != old.team1 then
        select exists(select *
                      from teams_to_tournaments
                      join bracket_trees bt on teams_to_tournaments.tournament_id = bt.tournament_id
                      where teams_to_tournaments.team_id = new.team1
                        and bt.id = new.bracket_tree_id)
        into v_exists;

        if not v_exists then
            raise exception 'team1 is not signed up for this tournament' using errcode = 44444;
        end if;
    end if;

    if new.team2 != old.team2 then
        select exists(select *
                      from teams_to_tournaments
                      join bracket_trees bt on teams_to_tournaments.tournament_id = bt.tournament_id
                      where teams_to_tournaments.team_id = new.team2
                        and bt.id = new.bracket_tree_id)
        into v_exists;

        if not v_exists then
            raise exception 'team2 is not signed up for this tournament' using errcode = 44444;
        end if;
    end if;

    return new;
end
$$;

create trigger brackets_update_trigger
    before update
    on brackets
    for each row
execute procedure brackets_update_trigger_procedure();