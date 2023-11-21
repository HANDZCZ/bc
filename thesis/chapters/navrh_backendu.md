
# Návrh backendu

## Architektura backendu

Následující text se bude zabývat jednotlivými architekturami, které se dají použít při vytváření backendu.
Hlavně se zaměří na dvě architektury monolit a mikroslužby,
[@fig:monoliths_and_microservices] znázorňuje jejich škálování a strukturu.

![Monolity a Mikroslužby [@microservices_fowler]](../pictures/Monoliths and Microservices.png){#fig:monoliths_and_microservices}

### Mikroslužby

Softwarová architektura založená na mikroslužbách (MSSA),
je preferovaným návrhovým modelem pro rostoucí počet společností v softwarovém odvětví [@microservice_based_projects_in_agile_world].
Tento návrhový model byl poprvé představen v roce 2011 jako výsledek neustálých změn,
které mají vyhovět současným požadavkům na vývoj softwaru [@microservice_based_projects_in_agile_world].
Popularita MSSA rychle roste [@microservice_based_projects_in_agile_world].
Dnešní poskytovatelé služeb, jako jsou Amazon, LinkedIn, Netflix, SoundCloud,
Uber a Verizon, si již osvojili tuto architekturu [@microservice_based_projects_in_agile_world].

Mikroslužby jsou distribuované aplikace s volnými vazbami, které pracují v jednotě [@microservice_based_projects_in_agile_world].
Mohou být vyvíjeny, zaváděny, testovány a škálovány nezávisle,
komunikují pomocí zpráv přes nenáročné komunikační mechanismy [@microservice_based_projects_in_agile_world].

Mikroslužby využívají distribuovaný systém ke zlepšení modularity [@microservices_fowler].
Distribuovaný software má však jednu zásadní nevýhodu, a to, že je distribuovaný [@microservices_fowler].
Jakmile začnete používat distribuci, vzniká celá řada problémů [@microservices_fowler].

Jedním z nich je i výkonnost [@microservices_trade_offs_fowler].
Pokud služba volá několik dalších vzdálených služeb,
z nichž každá volá dalších vzdálené služby,
tak délky těchto volání se sčítají a narostou do děsivých latenčních charakteristik  [@microservices_trade_offs_fowler].
Tento problém lze ale jednoduše vyřešit použitím asynchronního režimu  [@microservices_trade_offs_fowler].
Pokud služba provede několik asynchronních volání paralelně,
místo součtu jejich latencí bude nyní pomalá jen tak,
jak je pomalé její nejpomalejší volání  [@microservices_trade_offs_fowler].
Toto řešení může značně zvýšit výkon, ale přináší sebou další problémy  [@microservices_trade_offs_fowler].
Jeden z těchto problémů je, že asynchronní programování je obtížné,
je těžké ho správně naprogramovat a ještě těžší ho ladit  [@microservices_trade_offs_fowler].
Avšak většina společností, které používají mikroslužby,
používá asynchronní programování, aby dosáhly přijatelného výkonu  [@microservices_trade_offs_fowler].

Dalším z problémů je spolehlivost  [@microservices_trade_offs_fowler].
Očekáváte, že volání služeb bude fungovat, ale vzdálené volání může kdykoli selhat  [@microservices_trade_offs_fowler].
Při velkém množství mikroslužeb existuje čím dál více potenciálních míst,
kde služba může selhat  [@microservices_trade_offs_fowler].

Mikroslužby jsou samostatně nasaditelnou částí,
takže pro každou z nich je velká volnost při výběru technologie pro její vývoj  [@microservices_trade_offs_fowler].
Mikroslužby mohou být napsány v různých programovacích jazycích,
používat různé knihovny a různá datová úložiště  [@microservices_trade_offs_fowler].
Díky tomuto principu si mohou týmy vybrat vhodný nástroj pro danou činnost,
některé jazyky a knihovny se lépe hodí pro určité typy problémů  [@microservices_trade_offs_fowler].
Často se diskutuje o nejlepším nástroji pro danou činnost,
ale mnohdy největší přínos mikroslužeb spočívá v jednodušší správě verzí při vývoji  [@microservices_trade_offs_fowler].

### Monolit

Monolitická architektura je aplikace s jedinou kódovou základnou, která zahrnuje více služeb [@review_of_microservices_and_monolithic_architecture].
Tyto služby komunikují s externími systémy nebo uživateli prostřednictvím různých rozhraní,
jako jsou stránky HTML nebo rozhraní REST API [@review_of_microservices_and_monolithic_architecture].
Monolitická aplikace se obvykle skládá z databáze, klientského uživatelského rozhraní a serverové aplikace [@migration_of_monolithic_systems_to_microservice_architecture].

Největší výhodou monolitické architektury je její jednoduchost oproti distribuovaným aplikacím [@monolithic_vs_microservice_architecture_ieee].
Monolitické aplikace mnohem snadněji testují, nasazují, ladí a monitorují [@monolithic_vs_microservice_architecture_ieee; @migration_of_monolithic_systems_to_microservice_architecture].
Všechna data jsou uchovávána v jedné databázi bez nutnosti jejich synchronizace
a veškerá interní komunikace probíhá prostřednictvím vnitroprocesových mechanismů [@monolithic_vs_microservice_architecture_ieee].
Díky tomu je tento proces rychlý a nedochází u něj k problémům typickým pro komunikaci mezi procesy [@monolithic_vs_microservice_architecture_ieee].
Monolitický přístup pro vytváření aplikací je přirozeným a primárním přístupem k vytváření aplikací [@monolithic_vs_microservice_architecture_ieee].
Díky tomu, že veškerá logika pro zpracování požadavků je obsažena a běží v jediném procesu,
je práce s touto architekturou jednodušší.

Monolitická architektura je vhodná pro malé týmy, protože usnadňuje vývoj [@migration_of_monolithic_systems_to_microservice_architecture].
Každý vývojář bude moci provést změny v aplikaci nebo vytvořit něco nového,
protože všechna potřebná data a prvky jsou koncentrovány v jednom pracovním prostoru [@migration_of_monolithic_systems_to_microservice_architecture].
Komponenty monolitického softwaru jsou vzájemně propojené a závislé,
což umožňuje, aby byl software samostatný [@migration_of_monolithic_systems_to_microservice_architecture].

Většina aplikací se spoléhá na mnoho vzájemně provázaných funkcí,
jako jsou kontrolní záznamy, protokolování, omezování rychlosti atd.
U monolitických aplikací je řešení těchto problémů mnohem snazší,
protože mají společnou kódovou základnu [@migration_of_monolithic_systems_to_microservice_architecture].
Je snazší propojit komponenty s těmito úlohami, když je vše funkční v jedné aplikaci [@migration_of_monolithic_systems_to_microservice_architecture].
Propojení komponent s těmito úlohami je snazší,
protože vše je obsaženo v rámci jedné aplikace [@migration_of_monolithic_systems_to_microservice_architecture].

Malé monolitické aplikace mají tendenci dosahovat lepších výsledků než aplikace založené na mikroslužbách [@migration_of_monolithic_systems_to_microservice_architecture].
To lze vysvětlit tím, že monolitické aplikace mají společný kód a využívanou společnou paměť [@migration_of_monolithic_systems_to_microservice_architecture].

### Monolit vs Mikroslužby

Použití mikroslužeb vyžaduje automatické nasazení, monitorování, vypořádání se s konzistencí a komplexnosti systému [@microservice_premium_fowler].
Existují známá řešení jak se s těmito problémy vypořádat, ale tato řešení zabírají velké množství drahocenného času [@microservice_premium_fowler].

Monolit je jednodušší programovat a nezabírá tolik času na správu.
Toto však přestává platit, když se komplexita systémů zvýší na úroveň,
kde správa a přidávání nových funkcí se stává obtížné.
Nevýhodou však je, že orientace v něm bývá těžší,
protože jsou všechny služby obsaženy v jednom celku.

> *"O mikroslužbách vůbec neuvažujte, pokud nemáte systém, který je příliš složitý na to, abyste jej spravovali jako monolit.
> Většina softwarových systémů by měla být vytvořena jako jediná monolitická aplikace. V rámci tohoto monolitu dbejte na vhodnou modularitu, ale nesnažte se jej rozdělit na samostatné služby."*
\- Martin Fowler [@microservice_premium_fowler]

Tento úryvek je nádherně znázorněn v obrázku [-@fig:monoliths_vs_microservices] níže.

![Komplexnost vs Produktivita u monolitu a mikroslužeb [@microservice_premium_fowler]](../pictures/monolith vs microservices.png){#fig:monoliths_vs_microservices}

## Výběr technologií

## Výběr programovacích jazyků

## Návrh API pro komunikaci s frontendem

Úspěšný vývoj webových aplikací vyžaduje poskytnutí kvalitní komunikace
mezi backendovými servery a frontendovými aplikacemi [@designing_apis_that_enable_scalable_frontends].
Koncový uživatel rozhraní API musí snadno pochopit,
jak použít toto API k vývoji funkcí a vylepšení aplikace [@designing_apis_that_enable_scalable_frontends].

### REST

Nejvíce používané komunikační schéma,
které slouží k popisu komunikace mezi frontendovým a backendovým serverem [@designing_apis_that_enable_scalable_frontends].
Poskytuje backendu jednoduché rozhraní pro přístup k datům a provádění operací na straně serveru [@designing_apis_that_enable_scalable_frontends].
Toto rozhraní poskytuje společně dohodnutou skupinu metod, které jsou použity při vytváření požadavků HTTP,
jež poté může použít libovolná webová stránka [@designing_apis_that_enable_scalable_frontends].
Jedná se o existující standard, který se běžně používá napříč aplikacemi vyvinutými v posledních letech [@designing_apis_that_enable_scalable_frontends].

Při použití schématu REST lze implementovat klienta a server nezávisle [@what_is_rest_codecademy].
Což znamená, že kód na straně klienta může být kdykoli změněn,
aniž by byl ovlivněn provoz serveru, a kód na straně serveru může být změněn,
aniž by byl ovlivněn provoz klienta [@what_is_rest_codecademy].

Pokud každá strana zná formát zpráv, které má posílat té druhé,
mohou být tyto strany modulární a oddělené [@what_is_rest_codecademy].
Když se oddělí problematika uživatelského rozhraní od problematiky ukládání dat,
zlepší se flexibilita rozhraní napříč platformami
a zjednoduší se škálovatelnost serverových komponent [@what_is_rest_codecademy].
Díky tomuto oddělení je navíc možné, aby se každá komponenta vyvíjela nezávisle [@what_is_rest_codecademy].

Použití tohoto typu schématu také zajistí,
že různí klienti navštěvující stejné koncové body dostávají zcela stejné odpovědi.

Systémy, založené na schématu REST, jsou bezstavové, což znamená,
že server nepotřebuje vědět nic o tom, v jakém stavu se nachází klient, a naopak [@what_is_rest_codecademy].
Tímto způsobem může server i klient snadno porozumět jakékoli přijaté zprávě,
bez toho aniž by viděl předchozí zprávy [@what_is_rest_codecademy].
Díky tomuto přístupu lze výrazně zjednodušit vývoj jak backendu, tak frontendu.

Aplikace založené na schématu REST dosahují vysoké spolehlivosti,
rychlého výkonu a škálovatelnosti, protože komponenty, které lze spravovat,
aktualizovat a znovu používat, neovlivňují systém jako celek [@what_is_rest_codecademy].

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

## Zabezpečení a autentizace

\newpage

