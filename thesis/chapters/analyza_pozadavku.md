
# Analýza požadavků

## Identifikace potřeb uživatelů

Současný výzkum e-sportu je velice rozmanitý a týká se celého ekosystému e-sportu, [@fig:overview_of_the_esport_ecosystem_and_stakeholders] shrnuje ekosystém [@esport_business_and_information_systems_engineering].
Následující text se bude zabývat potřeb jednotlivých aktéru v tomto ekosystému,
bude zaměřen na tři hlavní subjekty, které provozují a využívají platformu e-sportových turnajů: organizátory turnajů, diváky a profesionální týmy a hráče [@esport_business_and_information_systems_engineering].

![Přehled ekosystému esportu a zúčastněných stran [@esport_business_and_information_systems_engineering]](../pictures/Overview of the esport ecosystem and stakeholders.png){#fig:overview_of_the_esport_ecosystem_and_stakeholders}

### Hráči

Potřeby hráčů spočívají v přístupu k přehledné a intuitivní platformě pro snadnou registraci na turnaje.
Dále vyžadují snadný přístup k informacím o pravidlech turnajů, herních formátech
a možnost sledování svého postupu v turnaji spolu s přístupem k vlastním statistikám.

### Organizátoři turnajů

Pro organizátory turnajů je klíčové poskytnout efektivní systém pro správu a registraci týmů a hráčů.
Důležitá je také možnost vytvářet a publikovat pravidla turnajů
a samozřejmě sledovat průběh turnaje v reálném čase s možností okamžitého řešení případných problémů.

### Diváci

Diváci vyžadují přístup k živým vysíláním a komentářům během turnajů.
Dále chtějí možnost interakce s ostatními diváky a vyjádření svých názorů na průběh turnaje.
Rovněž je důležitý snadný přístup k statistikám a informacím o hráčích a týmech.

### Ostatní aktéři

Pro ostatní aktéry, jako jsou techničtí operátoři, administrátoři
a jiní členi administrátorského týmu, je nezbytné zajistit bezpečné
a spolehlivé prostředí pro všechny účastníky.
Implementace systému zabezpečení a autentizace je nezbytná pro ochranu citlivých informací.

## Specifikace funkčních požadavků

### Registrace turnaje

Uživatelé by měli mít snadný přístup k registraci do turnajů [@how_to_make_an_esports_tournament_website_mobilecoderz; @how_to_design_an_esports_tournament_website_perfectionheeks; @how_to_make_an_esports_tournament_website_devteam].
Proces registrace by měl obsahovat relevantní informace,
jako jsou jména týmů, údaje o hráčích a kontaktní informace [@how_to_make_an_esports_tournament_website_mobilecoderz; @how_to_design_an_esports_tournament_website_perfectionheeks].
Zároveň by měla být k dispozici intuitivní navigace a uživatelsky přívětivé rozhraní,
aby se minimalizovala obtížnost při přihlašování.

### Správa týmů

Pro kapitány týmů je klíčové poskytnout nástroje,
které usnadní správu jejich týmu [@how_to_make_an_esports_tournament_website_mobilecoderz; @how_to_design_an_esports_tournament_website_perfectionheeks].
To zahrnuje přidávání a odebírání hráčů,
aktualizaci profilů hráčů a sledování statistik týmu [@how_to_make_an_esports_tournament_website_mobilecoderz; @how_to_design_an_esports_tournament_website_perfectionheeks].
Rozhraní pro správu týmů by mělo být intuitivní,
aby vyhovovalo potřebám každého týmu.

### Rozpis zápasů

Uživatelé by měli mít k dispozici přehledný rozpis zápasů,
který zahrnuje data, časy a informace o soupeřích [@how_to_make_an_esports_tournament_website_mobilecoderz; @how_to_design_an_esports_tournament_website_perfectionheeks; @how_to_make_an_esports_tournament_website_devteam].
Strukturované a snadno čitelné informace o plánovaných zápasech jsou klíčové pro to,
aby si uživatelé mohli jednoduše naplánovat sledování turnajů a aktivně se zapojit.

### Přímé přenosy a VOD

Mělo by být zajištěny možnosti živých přenosů,
umožňující divákům sledovat dění v reálném čase [@how_to_make_an_esports_tournament_website_mobilecoderz; @how_to_design_an_esports_tournament_website_perfectionheeks].
Pro ty, kteří nestihli živé přenosy,
by jistě ocenili funkci pro přístup k nahraným videím/turnajům (Video on Demand) [@how_to_make_an_esports_tournament_website_mobilecoderz; @how_to_design_an_esports_tournament_website_perfectionheeks].
Tímto se zajistí maximální flexibilita pro fanoušky a účastníky turnaje.

### Hráčské profily

Vytvoření individuálních hráčských profilů, prezentující herní historii, úspěchy a statistiky,
je klíčové pro poskytnutí informací o hráčích [@how_to_make_an_esports_tournament_website_mobilecoderz; @how_to_design_an_esports_tournament_website_perfectionheeks; @how_to_make_an_esports_tournament_website_devteam].
Hráčské profily umožní lepší sledování oblíbených hráčů.

### Závěrečné tabulky a pořadí

Pro uživatele je důležité mít přístup k aktuálním turnajovým žebříčkům,
které ukazují postup týmů skrze různé fáze soutěže [@how_to_make_an_esports_tournament_website_mobilecoderz; @how_to_design_an_esports_tournament_website_perfectionheeks; @how_to_make_an_esports_tournament_website_devteam].
Pravidelně aktualizované tabulky a pořadí poskytují jasný přehled o výkonech týmů
a vytvářejí napětí a rivalitu v průběhu turnaje.

## Specifikace nefunkčních požadavků

### Dostupnost a spolehlivost

Aplikace by měla být dostupná, když ji uživatelé potřebují [@nonfunctional_requirements_medium].
Udává očekávanou doba provozuschopnosti v procentech
(např. "99,9% dostupnost" znamená, že systém může mít až 8,76 hodin výpadku za rok) [@nonfunctional_requirements_medium].

Systém by měl fungovat za stanovených podmínek po určitou dobu [@nonfunctional_requirements_medium].
Za těchto podmínek by systém měl běžet bez jakéhokoli problému teoreticky do nekonečna.

### Výkon a škálovatelnost

Škálovatelnost je schopnost systému zvládnout zvýšenou zátěž, aniž by to mělo vliv na uživatelský komfort [@nonfunctional_requirements_medium].
Systém by tedy měl efektivně reagovat na náhlou zátěž a nezkolabovat během průběhu turnajů.
Zajištěna by také měla být stabilita a rychlost komunikace mezi frontendem a backendem pro plynulý průběh turnajů.

Backendová architektura musí být navržena s ohledem na škálovatelnost,
aby bylo možné bezproblémově přizpůsobit systém růstu uživatelské základny.

### Zabezpečení a ochrana osobních údajů

Bezpečné přihlašovací systémy, šifrování dat a opatření na ochranu soukromí,
je nezbytné, aby byla zajištěna bezpečnost osobních údajů uživatelů [@how_to_make_an_esports_tournament_website_mobilecoderz; @how_to_design_an_esports_tournament_website_perfectionheeks].
Důkladné zabezpečení informací včetně osobních údajů hráčů
a týmů je prioritou pro vytvoření důvěryhodné
a bezpečné platformy pro správu e-sportových turnajů.

\newpage

