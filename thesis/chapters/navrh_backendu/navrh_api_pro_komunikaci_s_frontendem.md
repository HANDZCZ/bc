
## Návrh API pro komunikaci s frontendem

Úspěšný vývoj webových aplikací vyžaduje poskytnutí kvalitní komunikace
mezi backendovými servery a frontendovými aplikacemi.
Koncový uživatel rozhraní API musí snadno pochopit,
jak použít toto API k vývoji funkcí a vylepšení aplikace. [@designing_apis_that_enable_scalable_frontends]

### REST

Nejvíce používané komunikační schéma,
které slouží k popisu komunikace mezi frontendovým a backendovým serverem.
Poskytuje backendu jednoduché rozhraní pro přístup k datům a provádění operací na straně serveru.
Toto rozhraní poskytuje společně dohodnutou skupinu metod, které jsou použity při vytváření požadavků HTTP,
jež poté může použít libovolná webová stránka.
Jedná se o existující standard, který se běžně používá napříč aplikacemi vyvinutými v posledních letech. [@designing_apis_that_enable_scalable_frontends]

Při použití schématu REST lze implementovat klienta a server nezávisle.
Což znamená, že kód na straně klienta může být kdykoli změněn,
aniž by byl ovlivněn provoz serveru, a kód na straně serveru může být změněn,
aniž by byl ovlivněn provoz klienta. [@what_is_rest_codecademy]

Pokud každá strana zná formát zpráv, které má posílat té druhé,
mohou být tyto strany modulární a oddělené.
Když se oddělí problematika uživatelského rozhraní od problematiky ukládání dat,
zlepší se flexibilita rozhraní napříč platformami
a zjednoduší se škálovatelnost serverových komponent.
Díky tomuto oddělení je navíc možné, aby se každá komponenta vyvíjela nezávisle. [@what_is_rest_codecademy]

Použití tohoto typu schématu také zajistí,
že různí klienti navštěvující stejné koncové body dostávají zcela stejné odpovědi.

Systémy, založené na schématu REST, jsou bezstavové, což znamená,
že server nepotřebuje vědět nic o tom, v jakém stavu se nachází klient, a naopak. [@what_is_rest_codecademy]
Tímto způsobem může server i klient snadno porozumět jakékoli přijaté zprávě,
bez toho aniž by viděl předchozí zprávy. [@what_is_rest_codecademy]
Díky tomuto přístupu lze výrazně zjednodušit vývoj jak backendu, tak frontendu.

Aplikace založené na schématu REST dosahují vysoké spolehlivosti,
rychlého výkonu a škálovatelnosti, protože komponenty, které lze spravovat,
aktualizovat a znovu používat, neovlivňují systém jako celek. [@what_is_rest_codecademy]

```{.d2 #fig:rest_comunication_diagram caption="REST komunikační diagram"}
direction: down

classes: {
  imgonly: {
    label: ""
    shape: image
  }
  container: {
    style.border-radius: 8
  }
  conn: {
    style: {
      font-size: 28
    }
  }
}

client: REST clients {
  class: container

  pc: {
    class: imgonly
    icon: https://simpleicon.com/wp-content/uploads/pc.png
  }
  phone: {
    class: imgonly
    icon: https://cdn-icons-png.flaticon.com/512/0/191.png
  }
}

server: REST server {
  class: container

  img: {
    class: imgonly
    icon: https://static.thenounproject.com/png/17840-200.png
  }
  json: {
    class: imgonly
    icon: https://cdn-icons-png.flaticon.com/512/136/136443.png
  }
}

database: Database {
  class: container

  postgres: {
    class: imgonly
    icon: https://cdn-icons-png.flaticon.com/512/5968/5968342.png
  }
}

database -> server: response {
  source-arrowhead: table data
  class: conn
}
server -> database: sql request {
  source-arrowhead: SQL statement
  class: conn
}

client -> server: rest request {
  source-arrowhead: "GET, POST,\nPUT, DELETE\n method"
  class: conn
}
server -> client: rest response {
  source-arrowhead: JSON format
  class: conn
}
```

### GraphQL

GraphQL je open-source dotazovací a manipulační jazyk pro API
a runtime pro realizaci dotazů s existujícími daty [@what_is_graphql_hygraph].
Jazyk GraphQL byl vyvinut interně společností Facebook v roce 2012
a v roce 2015 byl zveřejněn [@what_is_graphql_hygraph].

Původ jazyka GraphQL pramení ze snahy společnosti Facebook škálovat svou mobilní aplikaci [@what_is_graphql_hygraph].
V té době byla jejich aplikace adaptací jejich webových stránek,
kdy jejich strategie pro mobilní zařízení spočívala v jednoduché "adopci" HTML5 na mobilní zařízení [@what_is_graphql_hygraph].
Kvůli problémům spojeným s velkým vytížením sítě
a neideálním UX se však tým rozhodl vytvořit aplikaci pro iOS od základu pomocí nativních technologií [@what_is_graphql_hygraph].

> *"Hlavní problém implementace kanálu novinek v mobilních zařízeních byl to,
že nebylo jednoduché získat zprávu, kdo ji napsal, co v ní stojí, seznam komentářů a kdo příspěvku dal lajk.
Stávající rozhraní API nebyla navržena tak, aby umožnila vývojářům poskytnout přístup k informacím potřebných
k vývoji kanálu novinek na mobilních zařízeních.
Neměly hierarchickou strukturu, neumožňovaly vývojářům vybrat si jaká data potřebují,
ani možnost zobrazit seznam různorodých příběhů v kanálu."*
\- Brenda Clark [@what_is_graphql_history_components_ecosystem]

GraphQL je dotazovací jazyk pro rozhraní API
a runtime pro realizaci dotazů pomocí existujících dat [@what_is_graphql_hygraph; @what_is_graphql_history_components_ecosystem].
Největší výhoda jazyka GraphQL spočívá především v tom,
že GraphQL poskytuje kompletní a srozumitelný popis dat v rozhraní API,
klientům dává možnost žádat přesně to, co potřebují, a nic navíc [@what_is_graphql_hygraph; @what_is_graphql_history_components_ecosystem].
Při zasílání dotazů na rozhraní API vrací jazyk GraphQL zcela předvídatelné výsledky,
aniž by docházelo k získávání více dat, nebo méně dat než je potřeba,
což zajišťuje, že aplikace využívající jazyk GraphQL jsou rychlé,
stabilní a škálovatelné [@what_is_graphql_hygraph].

```{.d2 #fig:graphql_comunication_diagram caption="GraphQL komunikační diagram"}
direction: up

classes: {
  imgonly: {
    label: ""
    shape: image
  }
  container: {
    style: {
      border-radius: 8
    }
  }
  conn: {
    style: {
      font-size: 28
    }
  }
}

client: Clients {
  class: container

  pc: {
    class: imgonly
    icon: https://simpleicon.com/wp-content/uploads/pc.png
  }
  phone: {
    class: imgonly
    icon: https://cdn-icons-png.flaticon.com/512/0/191.png
  }
}

graphql: GraphQL API {
  class: container

  graphql: {
    class: imgonly
    icon: https://upload.wikimedia.org/wikipedia/commons/thumb/1/17/GraphQL_Logo.svg/800px-GraphQL_Logo.svg.png
  }
}

client -> graphql: querry / mutation {
  class: conn
}
graphql -> client: response {
  class: conn
}
client <-> graphql: subscrition / events {
  class: conn
}

database: Database {
  class: container

  postgres: {
    class: imgonly
    icon: https://cdn-icons-png.flaticon.com/512/5968/5968342.png
  }
}

database -> graphql: response {
  class: conn
}
graphql -> database: request {
  class: conn
}

server: Legacy REST server {
  class: container

  img: {
    class: imgonly
    icon: https://static.thenounproject.com/png/17840-200.png
  }
  json: {
    class: imgonly
    icon: https://cdn-icons-png.flaticon.com/512/136/136443.png
  }
}

graphql -> server: request {
  class: conn
}
server -> graphql: response {
  class: conn
}

database -> server: response {
  source-arrowhead: table data
  class: conn
}
server -> database: sql request {
  source-arrowhead: SQL statement
  class: conn
}
```

