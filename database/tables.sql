create table games
(
    id          uuid default gen_random_uuid() not null
        constraint games_pk
            primary key,
    name        text                           not null,
    description text                           not null,
    version     text default 'latest'::text    not null,
    constraint games_unique_name_and_version
        unique (name, version)
);

create table tournaments
(
    id                   uuid    default gen_random_uuid() not null
        constraint tournaments_pk
            primary key,
    name                 text                              not null
        constraint tournaments_unique_name
            unique,
    description          text                              not null,
    game_id              uuid                              not null
        constraint tournaments_games_id_fk
            references games,
    max_team_size        integer                           not null,
    requires_application boolean                           not null,
    applications_closed  boolean default false             not null,
    tournament_type      tournament_type                   not null,
    min_team_size        integer                           not null
);

create table bracket_trees
(
    tournament_id uuid                           not null
        constraint bracket_trees_tournaments_id_fk
            references tournaments
            on delete cascade,
    id            uuid default gen_random_uuid() not null
        constraint bracket_trees_pk
            primary key,
    position      integer                        not null
);

comment on column bracket_trees.position is 'signifies order of subtrees
-1 - ffa tree
0 - winner tree
1 - 1st losers tree
2 - 2nd losers tree
...';

create table teams
(
    id          uuid default gen_random_uuid() not null
        constraint teams_pk
            primary key,
    name        text                           not null
        constraint teams_unique_name
            unique,
    description text                           not null
);

create table brackets
(
    bracket_tree_id uuid             not null
        constraint brackets_bracket_trees_id_fk
            references bracket_trees
            on delete cascade,
    team1           uuid
        constraint brackets_teams_id_fk_team1
            references teams,
    team2           uuid
        constraint brackets_teams_id_fk_team2
            references teams,
    layer           smallint         not null,
    winner          boolean,
    position        integer          not null,
    team1_score     bigint default 0 not null,
    team2_score     bigint default 0 not null,
    constraint brackets_pk
        primary key (bracket_tree_id, layer, position)
);

comment on column brackets.layer is 'signifies at which tree layer is bracket located';

comment on column brackets.winner is 'true - first team wins
false - second team wins
null - no winner yet';

comment on column brackets.position is 'signifies order of the brackets';

create table users
(
    id         uuid      default gen_random_uuid() not null
        constraint users_pk
            primary key,
    nick       text                                not null,
    salt       text                                not null,
    hash       bytea                               not null,
    created_at timestamp default CURRENT_TIMESTAMP not null,
    email      text                                not null
        constraint users_unique_email
            unique
);

create table players_to_teams
(
    player_id uuid not null
        constraint players_to_teams_users_id_fk
            references users,
    team_id   uuid not null
        constraint players_to_teams_teams_id_fk
            references teams
            on delete cascade,
    constraint players_to_teams_pk
        primary key (player_id, team_id)
);

create table roles
(
    id   uuid default gen_random_uuid() not null
        constraint roles_pk
            primary key,
    name text                           not null
        constraint roles_unique_name
            unique
);

create table roles_to_users
(
    role_id uuid not null
        constraint roles_to_users_roles_id_fk
            references roles,
    user_id uuid not null
        constraint roles_to_users_users_id_fk
            references users
            on delete cascade,
    constraint roles_to_users_pk
        primary key (role_id, user_id)
);

create table managers_to_teams
(
    manager_id uuid not null
        constraint managers_to_teams_users_id_fk
            references users
            on delete cascade,
    team_id    uuid not null
        constraint managers_to_teams_teams_id_fk
            references teams
            on delete cascade,
    constraint managers_to_teams_pk
        primary key (manager_id, team_id)
);

create table players_to_teams_invites
(
    player_id uuid not null
        constraint players_to_teams_invites_users_id_fk
            references users
            on delete cascade,
    team_id   uuid not null
        constraint players_to_teams_invites_teams_id_fk
            references teams
            on delete cascade,
    constraint players_to_teams_invites_pk
        primary key (player_id, team_id)
);

create table managers_to_teams_invites
(
    manager_id uuid not null
        constraint managers_to_teams_invites_users_id_fk
            references users
            on delete cascade,
    team_id    uuid not null
        constraint managers_to_teams_invites_teams_id_fk
            references teams
            on delete cascade,
    constraint managers_to_teams_invites_pk
        primary key (manager_id, team_id)
);

create table teams_to_tournaments
(
    team_id       uuid not null
        constraint teams_to_tournaments_teams_id_fk
            references teams,
    tournament_id uuid not null
        constraint teams_to_tournaments_tournaments_id_fk
            references tournaments
            on delete cascade,
    constraint teams_to_tournaments_pk
        primary key (team_id, tournament_id)
);

create table teams_to_tournaments_applications
(
    team_id       uuid not null
        constraint teams_to_tournaments_applications_teams_id_fk
            references teams
            on delete cascade,
    tournament_id uuid not null
        constraint teams_to_tournaments_applications_tournaments_id_fk
            references tournaments
            on delete cascade,
    constraint teams_to_tournaments_applications_pk
        primary key (team_id, tournament_id)
);

create table players_to_tournaments_playing
(
    player_id     uuid not null
        constraint players_to_tournaments_playing_users_id_fk
            references users,
    tournament_id uuid not null
        constraint players_to_tournaments_playing_tournaments_id_fk
            references tournaments
            on delete cascade,
    team_id       uuid not null
        constraint players_to_tournaments_playing_teams_id_fk
            references teams,
    constraint players_to_tournaments_playing_pk
        primary key (player_id, tournament_id)
);

