
### Funkce handle_leave_tournament

Tato funkce obstarává odstranění týmu z turnaje a vrací typ boolean.
Když tato funkce vrátí hodnotu true, tak vše proběhlo v pořádku a není potřeba, žádné manuální úpravy.
Pokud ale tato funkce vrátí hodnotu false, tak to znamená, že turnaj již začal a byly vygenerovány brackety, které je potřeba upravit manuálně.

Nejdříve je zjištěno, jestli tým existuje, pokud neexistuje tak se vrátí chyba.
Dále se zjistí, jestli uživatel, který zavolal tuto funkci, je manažerem týmu nebo manažerem turnaje,
pokud není, tak vrátí chybu.
Poslední kontrola je, zda jsou přihlášky uzavřeny,
pokud jsou uzavřeny, tak tato funkce vrátí hodnotu false.
Poté co proběhnou všechny kontroly a ani jedna nevrátí chybu,
tak je vrácena hodnota true a je tým vymazán z tabulek teams_to_tournaments ([@sec:table_teams_to_tournaments]),
teams_to_tournaments_applications ([@sec:table_teams_to_tournaments_applications])
a players_to_tournaments_playing ([@sec:table_players_to_tournaments_playing]).

