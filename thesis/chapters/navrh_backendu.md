
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

## Zabezpečení a autentizace

\newpage

