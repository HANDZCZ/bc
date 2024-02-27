
### Trigger pro tabulku managers_to_teams

Tento trigger zajišťuje, že když se z tabulky managers_to_teams vymaže záznam,
tak proběhne kontrola, jestli tým má alespoň jednoho manažera.
Pokud by manažera neměl, tak se tým smaže.
Pokud tým má alespoň jednoho manažera, tak se tým nesmaže.
Jinak řečeno tento trigger zajišťuje,
že když se smaže uživatel, tak po něm nezůstane tým bez manažerů,
který by poté musel být odstraněn manuálně.

