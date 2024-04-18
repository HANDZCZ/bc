
### Trigger pro tabulku tournaments

Tento trigger se spustí při editaci turnaje a má za úkol udržet turnaj v logickém stavu.
Jedna z funkcí je, že vymaže všechny přihlášky na turnaj, pokud se při editaci nastaví,
že turnaj nepřijímá přihlášky.
Další funkcí je automatické přijetí všech přihlášek, když se turnaj změní z turnaje,
který vyžaduje přihlášku, na turnaj, který přihlášku nevyžaduje.
Dále také kontroluje, zda se změnil typ turnaje.
Pokud by se změnil a přihlášky jsou stále uzavřeny, tak vrátí chybu.
Další kontrolou je kontrola, zda byly přihlášky na turnaj otevřeny.
Pokud by byly otevřeny, tak se smažou veškeré stromové struktury patřící k turnaji.

