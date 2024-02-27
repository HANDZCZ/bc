insert into roles (name)
values ('Tournament Manager');

insert into public.users (nick, salt, hash, email)
values ('root',
        '5le^amm47Tzp1@KAQlhL7I~ujn~K!tnWLD51UZCmhk!Me&eGfJ$NMvYou6j)Z(Q!u)@KN#)vwj0jZTO6!GDd&6zmQ~Ile8U(dme(qJfDafQCO!i5bmyk4iy!6*uKKwkO',
        E'\\xE368F796A801CB70A1F8E406330B874EB68121C681CDE742FEB3E556B2823F2C',
        'root@ui.ui');
insert into roles_to_users (role_id, user_id)
values ((select id from roles where name = 'Tournament Manager'), (select id from users where nick = 'root'));