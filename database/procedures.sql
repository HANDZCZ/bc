create or replace procedure handle_player_invite(_player_id uuid, _team_id uuid, _accepted bool)
    language plpgsql
as
$$
declare
    v_exists bool;
begin
    select exists(select * from players_to_teams_invites where player_id = _player_id and team_id = _team_id)
    into v_exists;

    if not v_exists then
        raise exception 'no invite found' using errcode = 44444;
    end if;

    if not _accepted then
        delete from players_to_teams_invites where player_id = _player_id and team_id = _team_id;
        return;
    end if;

    insert into players_to_teams (player_id, team_id) values (_player_id, _team_id);
    delete from players_to_teams_invites where player_id = _player_id and team_id = _team_id;
end;
$$;


create or replace procedure handle_manager_invite(_manager_id uuid, _team_id uuid, _accepted bool)
    language plpgsql
as
$$
declare
    v_exists bool;
begin
    select exists(select * from managers_to_teams_invites where manager_id = _manager_id and team_id = _team_id)
    into v_exists;

    if not v_exists then
        raise exception 'no invite found' using errcode = 44444;
    end if;

    if not _accepted then
        delete from managers_to_teams_invites where manager_id = _manager_id and team_id = _team_id;
        return;
    end if;

    insert into managers_to_teams (manager_id, team_id) values (_manager_id, _team_id);
    delete from managers_to_teams_invites where manager_id = _manager_id and team_id = _team_id;
end;
$$;

create or replace procedure invite_players_to_team(_manager_id uuid, _team_id uuid, _player_ids uuid[])
    language plpgsql
as
$$
declare
    v_exists bool;
begin
    select exists(select * from teams where id = _team_id) into v_exists;
    if not v_exists then
        raise exception 'team does not exist' using errcode = 44444;
    end if;

    select exists(select * from managers_to_teams where manager_id = _manager_id and team_id = _team_id)
    into v_exists;

    if not v_exists then
        raise exception 'not a team manager' using errcode = 66666;
    end if;

    insert into players_to_teams_invites (player_id, team_id)
    select player_id__, _team_id
    from unnest(_player_ids) player_id__
    where player_id__ not in (select player_id from players_to_teams where team_id = _team_id)
    on conflict do nothing;
end;
$$;

create or replace procedure invite_managers_to_team(_manager_id uuid, _team_id uuid, _manager_ids uuid[])
    language plpgsql
as
$$
declare
    v_exists bool;
begin
    select exists(select * from teams where id = _team_id) into v_exists;
    if not v_exists then
        raise exception 'team does not exist' using errcode = 44444;
    end if;

    select exists(select * from managers_to_teams where manager_id = _manager_id and team_id = _team_id)
    into v_exists;

    if not v_exists then
        raise exception 'not a team manager' using errcode = 66666;
    end if;

    insert into managers_to_teams_invites (manager_id, team_id)
    select manager_id__, _team_id
    from unnest(_manager_ids) manager_id__
    where manager_id__ not in (select manager_id from managers_to_teams where team_id = _team_id)
    on conflict do nothing;
end;
$$;


create or replace procedure remove_managers_from_team(_manager_id uuid, _team_id uuid, _manager_ids uuid[])
    language plpgsql
as
$$
declare
    v_exists bool;
begin
    select exists(select * from teams where id = _team_id) into v_exists;
    if not v_exists then
        raise exception 'team does not exist' using errcode = 44444;
    end if;

    select exists(select * from managers_to_teams where manager_id = _manager_id and team_id = _team_id)
    into v_exists;

    if not v_exists then
        raise exception 'not a team manager' using errcode = 66666;
    end if;

    delete
    from managers_to_teams
    where manager_id != _manager_id
      and team_id = _team_id
      and manager_id = any (_manager_ids);
end;
$$;

create or replace procedure remove_players_from_team(_manager_id uuid, _team_id uuid, _player_ids uuid[])
    language plpgsql
as
$$
declare
    v_exists bool;
begin
    select exists(select * from teams where id = _team_id) into v_exists;
    if not v_exists then
        raise exception 'team does not exist' using errcode = 44444;
    end if;

    select exists(select * from managers_to_teams where manager_id = _manager_id and team_id = _team_id)
    into v_exists;

    if not v_exists then
        raise exception 'not a team manager' using errcode = 66666;
    end if;

    delete
    from players_to_teams
    where team_id = _team_id
      and player_id = any (_player_ids);
end;
$$;


create or replace procedure delete_team(_manager_id uuid, _team_id uuid)
    language plpgsql
as
$$
declare
    v_exists bool;
begin
    select exists(select * from teams where id = _team_id) into v_exists;
    if not v_exists then
        raise exception 'team does not exist' using errcode = 44444;
    end if;

    select exists(select * from managers_to_teams where manager_id = _manager_id and team_id = _team_id) or
           exists(select *
                  from roles_to_users
                           join roles r on r.id = roles_to_users.role_id
                  where user_id = _manager_id
                    and r.name = 'Tournament Manager')
    into v_exists;

    if not v_exists then
        raise exception 'not a team manager or tournament manager' using errcode = 66666;
    end if;

    delete from teams where id = _team_id;
end;
$$;

create or replace procedure edit_team(_manager_id uuid, _name text, _description text, _team_id uuid)
    language plpgsql
as
$$
declare
    v_exists bool;
begin
    select exists(select * from teams where id = _team_id) into v_exists;
    if not v_exists then
        raise exception 'team does not exist' using errcode = 44444;
    end if;

    select exists(select * from managers_to_teams where manager_id = _manager_id and team_id = _team_id)
    into v_exists;

    if not v_exists then
        raise exception 'not a team manager' using errcode = 66666;
    end if;

    update teams
    set name        = coalesce(_name, name),
        description = coalesce(_description, description)
    where id = _team_id;
end;
$$;

create or replace function new_team(_manager_id uuid, _name text, _description text)
    returns uuid
    language plpgsql
as
$$
declare
    v_team_id uuid;
begin
    insert into teams (name, description) values (_name, _description) returning teams.id into v_team_id;
    insert into managers_to_teams (manager_id, team_id) values (_manager_id, v_team_id);

    return v_team_id;
end;
$$;

create or replace procedure apply_for_tournament(_manager_id uuid, _team_id uuid, _tournament_id uuid)
    language plpgsql
as
$$
declare
    v_exists               bool;
    v_requires_application bool;
    v_applications_closed  bool;
begin
    select exists(select * from teams_to_tournaments where team_id = _team_id and tournament_id = _tournament_id) or
           exists(select *
                  from teams_to_tournaments_applications
                  where team_id = _team_id
                    and tournament_id = _tournament_id)
    into v_exists;
    if v_exists then
        raise exception 'team already applied for this tournament' using errcode = 44444;
    end if;

    select exists(select * from teams where id = _team_id) into v_exists;
    if not v_exists then
        raise exception 'team does not exist' using errcode = 44444;
    end if;

    select exists(select * from managers_to_teams where manager_id = _manager_id and team_id = _team_id)
    into v_exists;
    if not v_exists then
        raise exception 'not a team manager' using errcode = 66666;
    end if;

    select exists(select * from tournaments where id = _tournament_id) into v_exists;
    if not v_exists then
        raise exception 'tournament does not exist' using errcode = 44444;
    end if;

    select applications_closed, requires_application
    from tournaments
    where id = _tournament_id
    into v_applications_closed, v_requires_application;

    if v_applications_closed then
        raise exception 'tournament applications are closed' using errcode = 66666;
    end if;

    --select requires_application from tournaments where id = _tournament_id into v_requires_application;

    if v_requires_application then
        insert into teams_to_tournaments_applications (team_id, tournament_id) values (_team_id, _tournament_id);
    else
        insert into teams_to_tournaments (team_id, tournament_id) values (_team_id, _tournament_id);
    end if;
end;
$$;

create or replace procedure set_playing(_manager_id uuid, _team_id uuid, _tournament_id uuid, _playing uuid[],
                                        _not_playing uuid[])
    language plpgsql
as
$$
declare
    v_exists              bool;
    v_applications_closed bool;
begin
    select exists(select * from teams where id = _team_id) into v_exists;
    if not v_exists then
        raise exception 'team does not exist' using errcode = 44444;
    end if;

    select exists(select * from managers_to_teams where manager_id = _manager_id and team_id = _team_id)
    into v_exists;

    if not v_exists then
        raise exception 'not a team manager' using errcode = 66666;
    end if;

    select exists(select * from teams_to_tournaments where tournament_id = _tournament_id and team_id = _team_id)
    into v_exists;

    if not v_exists then
        raise exception 'team is not signed up for this tournament' using errcode = 66666;
    end if;

    select applications_closed
    from tournaments
    where id = _tournament_id
    into v_applications_closed;

    if v_applications_closed then
        raise exception 'tournament applications are closed - editing is forbidden' using errcode = 66666;
    end if;

    insert into players_to_tournaments_playing (player_id, tournament_id, team_id)
    select __player_id, _tournament_id, _team_id
    from unnest(_playing) __player_id;
    delete
    from players_to_tournaments_playing
    where player_id = any (_not_playing)
      and team_id = _team_id
      and tournament_id = _tournament_id;
end;
$$;

create or replace procedure handle_application(_tournament_id uuid, _team_id uuid, _accepted bool)
    language plpgsql
as
$$
declare
    v_exists bool;
begin
    select exists(select *
                  from teams_to_tournaments_applications
                  where tournament_id = _tournament_id
                    and team_id = _team_id)
    into v_exists;

    if not v_exists then
        raise exception 'no invite found' using errcode = 44444;
    end if;

    if not _accepted then
        delete from teams_to_tournaments_applications where tournament_id = _tournament_id and team_id = _team_id;
        return;
    end if;

    insert into teams_to_tournaments (tournament_id, team_id) values (_tournament_id, _team_id);
    delete from teams_to_tournaments_applications where tournament_id = _tournament_id and team_id = _team_id;
end;
$$;

create or replace function handle_leave_tournament(_tournament_id uuid, _team_id uuid, _manager_id uuid)
    returns bool
    language plpgsql
as
$$
declare
    v_exists              bool;
    v_applications_closed bool;
begin
    select exists(select * from teams where id = _team_id) into v_exists;
    if not v_exists then
        raise exception 'team does not exist' using errcode = 44444;
    end if;

    select exists(select * from managers_to_teams where manager_id = _manager_id and team_id = _team_id) or
           exists(select *
                  from roles_to_users
                           join roles r on r.id = roles_to_users.role_id
                  where user_id = _manager_id
                    and r.name = 'Tournament Manager')
    into v_exists;

    if not v_exists then
        raise exception 'not a team manager' using errcode = 66666;
    end if;

    select applications_closed
    from tournaments
    where id = _tournament_id
    into v_applications_closed;

    if not v_applications_closed then
        delete from teams_to_tournaments where tournament_id = _tournament_id and team_id = _team_id;
        delete from teams_to_tournaments_applications where tournament_id = _tournament_id and team_id = _team_id;
        delete from players_to_tournaments_playing where tournament_id = _tournament_id and team_id = _team_id;
        return true;
    end if;

    /* tournament already started -> manual edit required */
    return false;
end;
$$;