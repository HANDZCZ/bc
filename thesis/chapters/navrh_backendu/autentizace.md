
## Autentizace

### Základní HTTP autentizace

Jedná se o nejjednodušší metodu při,
které umístí odesílatel do hlavičky požadavku uživatelské jméno a heslo.
Uživatelské jméno a heslo jsou zakódovány pomocí Base64,
toto kódování převádí uživatelské jméno a heslo na sadu 64 znaků,
aby byl zajištěn bezpečný přenos.
Tato metoda nevyžaduje soubory cookie, identifikaci relace,
přihlašovací stránky a další podobné možnosti identifikace uživatele,
protože využívá samotnou hlavičku HTTP. [@authentication_ultimate_guide; @authentication_methods_hubspot; @authentication_methods_restcase; @authentication_methods_techtarget]

![HTTP autentizace [@authentication_ultimate_guide]](../../pictures/http_auth.png){#fig:basic_http_auth}

Tato metoda autentizace se doporučuje jen zřídka kvůli tomu, že se dá snadno napadnout.
I přesto, že se používá kódování Base64, lze toto kódování snadno dekódovat.
Dokonce i když obsah autentizace nelze dekódovat do původního uživatelského jména a hesla,
je tento typ autentizace stále nedostatečně zabezpečený.
Útočníci mohou získat autentizační obsah a opakovaně odesílat požadavky na server.
Tento typ útoku se označuje jako replay útok. [@authentication_ultimate_guide; @authentication_methods_hubspot; @authentication_methods_restcase; @authentication_methods_techtarget]



### Autentizace pomocí souborů cookies

Autentizace pomocí souborů cookies je model ověřování komunikace mezi relací na straně serveru a souborem cookie prohlížeče (na straně klienta). [@authentication_ultimate_guide]

Webové stránky a webové aplikace používající soubory cookies k ověřování uživatelů,
nejprve požádají uživatele, aby se přihlásil na webové stránky.
Po přihlášení je vytvořen unikátní malý textový soubor.
Tento soubor je zvláštním identifikátorem spojeným s účtem uživatele.
Zařízení uživatele pak tento soubor cookie obdrží a uloží do svého prohlížeče. [@cookie_based_auth_dev]

Webové stránky mohou uživatele ověřit, aniž by se musel znovu přihlašovat,
díky tomu, že při dalších návštěvách odešle tento soubor cookie. [@cookie_based_auth_dev]

![Autentizace pomocí souborů cookies [@authentication_ultimate_guide]](../../pictures/cookie_based_auth.png){#fig:cookie_based_auth}

Hlavní výhoda této metody je, že se uživatelé nemusí opakovaně přihlašovat,
aby získali přístup ke svým účtům.
U této metody je, ale nezbytné zajistit,
že soubory cookies používané k ověřování identity uživatele byly zabezpečené a těžko manipulovatelné,
aby nedošlo k ohrožení bezpečnosti uživatelského účtu. [@cookie_based_auth_dev]



### Ověření pomocí tokenu

Ověření pomocí souborů cookie má několik nevýhod,
včetně obtížné údržby na straně serveru,
zejména v distribuovaných systémech.
To vedlo k hledání efektivnější alternativy,
která by tyto problémy vyřešila. [@authentication_ultimate_guide; @authentication_methods_techtarget; @authentication_methods_restcase; @authentication_methods_hubspot]

Jako odpověď na tento problém se našla autentizace pomocí tokenu. [@authentication_ultimate_guide; @authentication_methods_techtarget; @authentication_methods_restcase; @authentication_methods_hubspot]

Ověření pomocí tokenu je typ ověřování,
který k ověření identity uživatele používá tokeny.
Tokeny jsou malé části dat, které generuje server a posílá je klientovi.
Klient pak token uloží a použije jej k ověření na serveru při vytváření dotazů. [@authentication_ultimate_guide; @authentication_methods_techtarget; @authentication_methods_restcase; @authentication_methods_hubspot]

![Ověření pomocí tokenu [@authentication_ultimate_guide]](../../pictures/token_auth.png){#fig:token_auth}



### OAuth 2.0

OAuth je otevřený standard,
který umožňuje uživatelům udělovat ostatním stránkám oprávnění pro přístup k jejich uloženým informacím,
uložených u jiných poskytovatelů různých služeb.
Uživatelé tak nemusí poskytovat své uživatelské jméno a heslo přímo těmto stránkám. [@authentication_ultimate_guide; @authentication_methods_techtarget; @authentication_methods_restcase; @authentication_methods_hubspot]

OAuth slouží jako autorizační mechanismus,
který umožňuje vlastníkovi dat udělovat aplikacím třetích stran oprávnění k přístupu do systému a získávání dat.
Systém pro tento účel vygeneruje dočasný přístupový token, který při použití v aplikacích třetích stran nahradí nutnost zadávání hesla. [@authentication_ultimate_guide; @authentication_methods_techtarget; @authentication_methods_restcase; @authentication_methods_hubspot]

```{.d2 #fig:oauth_diagram caption="Ověření pomocí OAuth 2.0" height=50%}
shape: sequence_diagram

# direction: down

classes: {
  container: {
    style.border-radius: 8
  }
  conn: {
    style: {
      # font-size: 28
    }
  }
}

client: Client {
  class: container
}
oauth: OAuth server {
  class: container
}
res-server: Resource server {
  class: container
}

Aquire access token: {
  client.t1 -> oauth.t1: Request access token {
    class: conn
  }

  oauth.t1 -> oauth.t1: Verify client {
    class: conn
  }
  oauth.t1 -> oauth.t1: User consent {
    class: conn
  }

  oauth.t1 -> client.t1: Return access token {
    class: conn
  }
}

Fetch resource: {
  client.t1 -> res-server.t1: Request resource {
    class: conn
  }

  Validate access token: {
    res-server.t1 -> oauth.t2: Validate token {
      class: conn
    }
    oauth.t2 -> res-server.t1: Valid token {
      class: conn
    }
  }

  res-server.t1 -> client.t1: Return resource {
    class: conn
  }
}
```
