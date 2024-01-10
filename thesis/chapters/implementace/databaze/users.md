
### Tabulka users {#sec:table_users}

Tabulka users reprezentuje uživatele v systém.
Termín uživatel v tomto kontextu může označovat organizátora turnaje, manažera týmu či hráče v týmu.

Jejím hlavním úkolem je umožnit backendu identifikovat uživatele a uživateli umožnit přihlášení.

![Tabulka users](../../../pictures/databaze/users.png){ height=17% }

Nick slouží jako přezdívka pro uživatele, tento termín je hlavně používán v herní komunitě.

Salt a hash jsou použity při ověřování uživatele při přihlášení (TODO).

Email je nejen použit při přihlašování (TODO), ale také při registraci (TODO).
Slouží jako unikátní identifikátor uživatele.

Primární klíč id je primárně použit k identifikaci a vázání uživatele na ostatních tabulky,
protože email je možné kdykoli změnit a také hlavně proto,
že email je soukromá informace a neměla by být sdílena s ostatními tabulkami.

