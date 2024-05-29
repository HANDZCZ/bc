---
marp: true
theme: uncover
class: invert
size: 16:9
paginate: true
transition: slide 0.15s
footer: Jan Najman
header: Informační systém pro správu e-sportových turnajů

title: Informační systém pro správu e-sportových turnajů - návrh a implementace backendu
description: Prezentace pro bakalářskou práci na téma Informační systém pro správu e-sportových turnajů - návrh a implementace backendu
author: Jan Najman
keywords: Rust,API,REST API,Actix,Actix Web,Backend,Server,framework,esport,e-sport,tournaments
url: https://github.com/HANDZCZ/bc
lang: cs
---

<!--
_class: lead invert
_footer: ""
_header: ""
_paginate: false
-->

# <!-- fit --> Informační systém pro správu<br>e-sportových turnajů
### Návrh a implementace backendu

<br>

Jan Najman

---

## Osnova

- Cíl práce
- Proč bylo zvoleno toto téma?
- Použité technologie
- Vývojové prostředí
- Implementace
    - Návh databáze
    - Zabezpečení
- Problémy

---

## Cíl práce

Cílem bakalářské práce je analýza vhodných technologií, návrh a implementace backendové části informačního systému pro správu e‑sportových turnajů.

---

## Proč bylo zvoleno toto téma?

- Nedostatek softwarových řešení
    - Jen některá jsou open-source
    - Málokterá obsahují chtěné funkce
- Technologická výzva
    - Nutnost použití různých technologií
- Pochopení rozsahu a obtížnosti realizace

---

## Použité technologie

![h:170px](images/rust_logo.png) ![h:170px](images/Postgresql_logo.svg) ![h:180px](images/docker_logo.png)
![h:150px](images/actix_logo.png) ![h:140px](images/jwt_logo.svg) ![h:150px](images/sqlx_logo.jpg)

---

## Vývojové prostředí

<br>

![h:150px](images/vscode_logo.png) ![h:150px](images/rust_analyzer_logo.svg) ![h:150px](images/datagrip_logo.png)

---

## Návh databáze

Konečný návrh má:
|||
|-:|:-|
15 | tabulek
8 | pohledů
6 | triggerů
11 | procedur
2 | funkce

![bg left:45% 140%](images/db_image.png)

---

## Stromové struktury v DB

Struktury jsou rozděleny na "označení" stromu a jeho uzly

---

### Označení stromů

Tabulka `bracket_trees`
- id
    - id stromu
- tournament_id
    - cizí klíč turnaje, ke kterému patří
- position
    - určuje pozici stromu v turnaji

---

### Uzly

Tabulka `brackets`
- bracket_tree_id
    - cizí klíč stromu, ve kterém se nachází
- layer
    - určuje, na které vrstvě se nachází
- position
    - určuje pozici ve vrstvě

---

### Výhody

- Není potřeba rekurzivní vyhledání

### Nevýhody

- Nutost ukládání přesné pozice uzlů
- Potřeba dvou tabulek
- Pouze předdefinovaný stom

![bg right:40% 140%](images/scale.png)

---

## Zabezpečení

- JWT
- Role
- SQL Query Binding
- Hashování hesla

![bg left](images/lock.png)

---

## JWT

- použit balík jsonwebtoken
- token je získán z headeru `AUTHORIZATION`
- implementovány metody
    - decode_jwt
        - extrakce dat a verifikace tokenu z headeru
    - encode_jwt
        - zakódování dat a přidání claims do tokenu

---

## SQL Query Binding

- použit balík sqlx
    - makra query_as a query

```rust
...
match query_as!(
    ReturningRow,
    "insert into games (name, description, version) values ($1, $2, $3) returning games.id",
    data.name,
    data.description,
    data.version
)
.fetch_one(pool.get_ref())
.await
{
...
```

---

## Hashování hesla

Implementovány metody:
- make_salt
    - vytváří 128 znaků dlouhoů sůl
    - každý znak může nabýt 71 hodnot
    - až $71^{128} \approx 9.1426\cdot10^{236}$ unikátních hodnot
- make_hash
    - použit balík argon2rs
- verify_password

---

## Problémy

- Reprezentace stromové struktury v databázy
- Algoritmus propagace výsledku turnaje
    - Nutnost propagace, jak ve stejném stromu, tak i propagace do dalsího stromu

---

<!--
_class: lead invert
_footer: ""
_header: ""
_paginate: false
-->

# Děkuji za pozornost
