
## Autentizace

### Základní HTTP autentizace

Jedná se o nejjednodušší metodu při,
které umístí odesílatel do hlavičky požadavku uživatelské jméno a heslo [@authentication_ultimate_guide; @authentication_methods_hubspot; @authentication_methods_restcase; @authentication_methods_techtarget].
Uživatelské jméno a heslo jsou zakódovány pomocí Base64,
toto kódování převádí uživatelské jméno a heslo na sadu 64 znaků,
aby byl zajištěn bezpečný přenos [@authentication_ultimate_guide; @authentication_methods_hubspot; @authentication_methods_restcase; @authentication_methods_techtarget].
Tato metoda nevyžaduje soubory cookie, identifikaci relace,
přihlašovací stránky a další podobné možnosti identifikace uživatele,
protože využívá samotnou hlavičku HTTP [@authentication_ultimate_guide; @authentication_methods_hubspot; @authentication_methods_restcase; @authentication_methods_techtarget].

Tato metoda autentizace se doporučuje jen zřídka kvůli tomu, že se dá snadno napadnout [@authentication_ultimate_guide; @authentication_methods_hubspot; @authentication_methods_restcase; @authentication_methods_techtarget].
I přesto, že se používá kódování Base64, lze toto kódování snadno dekódovat [@authentication_ultimate_guide; @authentication_methods_restcase; @authentication_methods_techtarget].
Dokonce i když obsah autentizace nelze dekódovat do původního uživatelského jména a hesla,
je tento typ autentizace stále nedostatečně zabezpečený [@authentication_ultimate_guide].
Útočníci mohou získat autentizační obsah a opakovaně odesílat požadavky na server.
Tento typ útoku se označuje jako replay útok [@authentication_ultimate_guide].
