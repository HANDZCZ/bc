
### Trigger pro tabulku brackets

Tento trigger při editaci zajistí,
že se týmy nepřepíšou na null, když se na backend pošle žádost o zapsání vítěze bez hodnot team1 a team2.
Dále také zajišťuje, že týmy, které se nastavují do bracketu jsou přihlášeny na turnaj, ke kterému bracket patří.
Pokud nějaký z týmů není přihlášeny na turnaj, ke kterému bracket patří, tak se editace nepovede a vrátí se chyba.

