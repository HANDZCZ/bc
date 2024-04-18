
### Procedura set_playing

Tato procedura přidává nebo odebírá hráče z turnaje.
Jinak řečeno nastavuje, jestli hráč v tomto turnaji bude hrát hrát či ne.
Nejprve zjistí, jestli tým existuje, pokud neexistuje, tak vrátí chybu.
Dále zjistí, jestli uživatel, který zavolal tuto proceduru, je manažerem týmu,
pokud není manažerem, tak vrátí chybu.
Další kontrola zjistí, jestli je tým přijat na tento turnaj, pokud ne, tak vrátí chybu.
Poslední kontrola je, zda jsou přihlášky uzavřeny,
pokud jsou uzavřeny, tak se také vrátí chyba, že již nelze měnit kdo hraje v turnaji.
Poté co proběhnou všechny tyto kontroly a ani jedna nevrátí chybu,
tak jsou do tabulky players_to_tournaments_playing ([@sec:table_players_to_tournaments_playing]) hráči přidáni,
nebo jsou z ní hráči odebráni.

