
## Zabezpečení

### SQL injection

Velkým bezpečnostním problémem webových infrastruktur jsou útoky typu injection,
zejména SQL injection [@sql_injection_attack_detection].
Útok SQL injection umožňuje útočníkovi zasahovat do databáze webové aplikace
a krást informace nebo dokonce měnit či mazat legitimní data uložená v aplikaci [@sql_injection_attack_detection].
Open Web Application Security Project (OWASP) je celosvětový neziskový projekt usilující
o zlepšení bezpečnosti softwaru [@sql_injection_attack_detection].
Tato komunita vydává "OWASP Top 10", dokument pro zvýšení povědomí vývojářů a zabezpečení webových aplikací [@sql_injection_attack_detection].
Tento dokument vyjadřuje širokou shodu ohledně nejkritičtějších bezpečnostních rizik webových aplikací [@sql_injection_attack_detection].
OWASP Top 10 řadí injekce jako třetí nejzávažnější bezpečnostní riziko webových aplikací v roce 2021 [@sql_injection_attack_detection].
Podobně i společnost MITRE zveřejňuje žebříček CWE Top 25 Most Dangerous Software Weaknesses [@sql_injection_attack_detection].
V tomto žebříčku zaujímají SQL injekce rovněž třetí místo [@sql_injection_attack_detection].

SQL injection umožňuje útočníkovy manipulovat s dotazy aplikace na databázi [@sql_injection_attack_detection; @web_backend_security_risks].
Obvykle umožňuje útočníkovi zobrazit data, ke kterým by neměl mít přístup [@sql_injection_attack_detection; @web_backend_security_risks].
Může jít například o data patřící jiným uživatelům, přihlašovací údaje,
struktury tabulek nebo jakákoli jiná data, ke kterým má aplikace přístup [@sql_injection_attack_detection].
V mnoha případech může útočník tato data také upravovat nebo mazat,
čímž způsobí trvalé změny obsahu nebo i změny v chování aplikace [@sql_injection_attack_detection].
V některých situacích je také možné, že útočník může eskalovat svá práva pomocí SQL injection
a napadnout server, na kterém tento útok byl proveden,
nebo jinou back-endovou infrastrukturu či provést útoky typu "denial of service." [@sql_injection_attack_detection]

