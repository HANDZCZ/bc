
## Zabezpečení

Následující texty probírají nejčastějšími bezpečnostními problémy a jaký mohou mít vliv na zabezpečení.

### SQL injection

Velkým bezpečnostním problémem webových infrastruktur jsou útoky typu injection,
zejména SQL injection.
Útok SQL injection umožňuje útočníkovi zasahovat do databáze webové aplikace
a krást informace nebo dokonce měnit či mazat legitimní data uložená v aplikaci.
Open Web Application Security Project (OWASP) je celosvětový neziskový projekt usilující
o zlepšení bezpečnosti softwaru.
Tato komunita vydává "OWASP Top 10", dokument pro zvýšení povědomí vývojářů a zabezpečení webových aplikací.
Tento dokument obsahuje žebříček nejkritičtějších bezpečnostních rizik webových aplikací.
OWASP Top 10 řadí injekce jako třetí nejzávažnější bezpečnostní riziko webových aplikací v roce 2021.
Podobně i společnost MITRE zveřejňuje žebříček CWE Top 25 Most Dangerous Software Weaknesses.
V tomto žebříčku zaujímají SQL injekce rovněž třetí místo. [@sql_injection_attack_detection]

SQL injection umožňuje útočníkovy manipulovat s dotazy aplikace na databázi.
Obvykle umožňuje útočníkovi zobrazit data, ke kterým by neměl mít přístup.
Může jít například o data patřící jiným uživatelům, přihlašovací údaje,
struktury tabulek nebo jakákoli jiná data, ke kterým má aplikace přístup.
V mnoha případech může útočník tato data také upravovat nebo mazat,
čímž způsobí trvalé změny obsahu nebo i změny v chování aplikace.
V některých situacích je také možné, že útočník může eskalovat svá práva pomocí SQL injection
a napadnout server, na kterém tento útok byl proveden,
nebo jinou back-endovou infrastrukturu či provést útoky typu "denial of service". [@sql_injection_attack_detection; @web_backend_security_risks]

### Nezabezpečená deserializace

Nezabezpečená deserializace je bezpečnostní riziko,
při kterém jsou nedůvěryhodná nebo neznámá data použita ke spuštění útoku typu "denial of service,"
spuštění libovolného kódu, obejití autentizace nebo jinému zneužití logiky aplikace. [@what_is_insecure_deserialization; @web_backend_security_risks]

Serializace je proces převodu složitých datových struktur, jako jsou objekty a jejich pole, do formátu,
který lze odeslat a přijmout jako sekvenční proud bajtů.
Deserializace je proces obnovení tohoto proudu bajtů do plně funkční repliky původního objektu,
přesně ve stavu, v jakém byl při serializaci.
Logika webové stránky pak může s tímto deserializovaným objektem pracovat stejně jako s jakýmkoli jiným objektem. [@what_is_insecure_deserialization; @insecure_deserialization_portswigger; @web_backend_security_risks]

V některých případě je možné nahradit serializovaný objekt objektem zcela jiné třídy.
Alarmující je, že objekty libovolné třídy, které jsou na webové stránce k dispozici,
budou deserializovány a instancovány bez ohledu na to, která třída byla očekávána.
Z tohoto důvodu se nezabezpečená deserializace někdy označuje jako "object injection." [@insecure_deserialization_portswigger]

Při přijetí objektu neočekávané třídy může dojít k výjimce,
ale v této době však již mohlo dojít ke škodám.
Mnoho útoků založených na deserializaci je dokonáno ještě před dokončením deserializace.
To znamená, že samotný proces deserializace může zahájit útok,
i když funkce webové stránky přímo nepracují se škodlivým objektem.
Z tohoto důvodu mohou být vůči těmto technikám zranitelné i webové stránky,
jejichž logika je založena na silně typovaných jazycích. [@insecure_deserialization_portswigger]

Nezabezpečená deserializace obvykle vzniká z důvodu obecného nepochopení toho,
jak nebezpečná může být deserializace dat ovládaných uživatelem.
V ideálním případě by uživatelský vstup neměl být deserializován vůbec. [@insecure_deserialization_portswigger]

Dokonce ani v případě, že je na deserializovaná data implementována nějaká forma dodatečné kontroly.
Tento přístup je často neúčinný, protože je prakticky nemožné implementovat validaci nebo sanitizaci,
která by zohlednila všechny eventuality.
Uvedené kontroly jsou také zásadně chybné, protože se spoléhají na kontrolu dat po jejich deserializaci,
což v mnoha případech bude příliš pozdě na to, aby se zabránilo útoku. [@insecure_deserialization_portswigger]

Další z důvodů proč tento útok může být úspěšný je,
že deserializované objekty jsou často považovány za důvěryhodné.
Zejména při použití jazyků s binárním serializačním formátem se vývojáři mohou domnívat,
že uživatelé nemohou data efektivně číst nebo s nimi manipulovat.
Nicméně, i přestože tento serializační formát může vyžadovat více úsilí,
útočník je schopen zneužít binární serializované objekty stejně tak jako řetězcové formáty (JSON, XML, atd.). [@insecure_deserialization_portswigger]
