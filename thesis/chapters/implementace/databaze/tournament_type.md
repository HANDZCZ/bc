
### Typ tournament_type {#sec:db_type_tournament_type}

Typ tournament_type slouží k určení typu turnaje.
Jeho hlavním úkolem je signalizovat backendu jak má probíhat vytváření bracketů.

```{.postgresql .linenos}
create type tournament_type as enum (
    'FFA',
    'OneBracketTwoFinalPositions',
    'OneBracketOneFinalPositions'
    );
```

: Kód na vytvoření tournament_type typu {#lst:db_type_tournament_type}

Tento typ může nabýt 3 hodnot.
V turnaji typu FFA (Free For All) bojuje každý tým s každým.
Obrázek níže znázorňuje FFA turnaj pro čtyři týmy, každé spojení znázorňuje souboj.

```{.d2 #fig:db_type_tournament_type_ffa caption="Znázornění typu turnaje FFA"}
direction: right

classes: {
  container: {
    style.border-radius: 8
  }
  conn: {
    style: {
      font-size: 28
    }
  }
}

team1: {
  class: container
}
team2: {
  class: container
}
team3: {
  class: container
}
team4: {
  class: container
}

team1 -- team2
team1 -- team3
team1 -- team4

team2 -- team3
team2 -- team4

team3 -- team4
```

V turnaji typu OneBracketTwoFinalPositions a OneBracketOneFinalPositions jsou týmy náhodně rozmístěny do poslední vrstvy stromu.
Podle tohoto rozmístění poté spolu bojují a postupují do vyšších vrstev.
Obrázek níže znázorňuje jednoduchou stromovou strukturu pro oba typy turnaje (OneBracketTwoFinalPositions a OneBracketOneFinalPositions) se čtyřmi týmy.

```{.d2 #fig:some_tree_diagram caption="Znázornění stromové struktury"}
direction: right

classes: {
  container: {
    style.border-radius: 8
  }
  conn: {
    style: {
      font-size: 28
    }
  }
}

team1: {
  class: container
}
team2: {
  class: container
}
team3: {
  class: container
}
team4: {
  class: container
}

l1p0: "Bracket" {
  class: container
}

l1p1: "Bracket" {
  class: container
}

l0p0: "Bracket" {
  class: container
}

team1 -> l1p0
team2 -> l1p0

team3 -> l1p1
team4 -> l1p1

l1p0 -> l0p0: teamX wins
l1p1 -> l0p0: teamX wins
```

V turnaji typu OneBracketTwoFinalPositions se na konci stromu vždy rozhodují dvě finální pozice.
Obrázek níže znázorňuje tento typ turnaje pro 4 týmy.

```{.d2 #fig:db_type_tournament_type_OneBracketTwoFinalPositions caption="Znázornění typu turnaje OneBracketTwoFinalPositions"}
direction: right

classes: {
  container: {
    style.border-radius: 8
  }
  conn: {
    style: {
      font-size: 28
    }
  }
}

team1: {
  class: container
}
team2: {
  class: container
}
team3: {
  class: container
}
team4: {
  class: container
}

l1p0: "Bracket" {
  class: container
}

l1p1: "Bracket" {
  class: container
}

l0p0: "Bracket" {
  class: container
}

team1 -> l1p0
team2 -> l1p0

team3 -> l1p1
team4 -> l1p1

l1p0 -> l0p0: team1 wins
l1p1 -> l0p0: team3 wins

winner: 1st place\n\n team1 {
  class: container
}

2nd_place: 2nd place\n\n team3 {
  class: container
}

l0p0 -> winner: team1 wins
l0p0 -> 2nd_place: team3 loses
```

V turnaji typu OneBracketOneFinalPositions se na konci stromu vždy rozhoduje pouze jedna finální pozice.
Obrázek níže znázorňuje tento typ turnaje pro 4 týmy.

```{.d2 #fig:db_type_tournament_type_OneBracketOneFinalPositions caption="Znázornění typu turnaje OneBracketOneFinalPositions"}
direction: up

classes: {
  container: {
    style.border-radius: 8
  }
  conn: {
    style: {
      font-size: 28
    }
  }
}

team1: {
  class: container
}
team2: {
  class: container
}
team3: {
  class: container
}
team4: {
  class: container
}

l1p0: Winners bracket\nsemi-finals {
  class: container
}

l1p1: Winners bracket\nsemi-finals {
  class: container
}

l0p0: Winners bracket\nfinals {
  class: container
}

team1 -> l1p0
team2 -> l1p0

team3 -> l1p1
team4 -> l1p1

l1p0 -> l0p0: team1 wins
l1p1 -> l0p0: team3 wins

winner: 1st place\n\n team1 {
  class: container
}

l0p0 -> winner: team1 wins

ll0p0: Losers bracket\nfinals {
  class: container
}

ll1p0: Losers Bracket\nsemi-finals {
  class: container
}

l1p0 -> ll1p0: team2 loses
l1p1 -> ll1p0: team4 loses

ll1p0 -> ll0p0: team2 wins
l0p0 -> ll0p0: team3 loses

2nd_place: 2nd place\n\n team3 {
  class: container
}

ll0p0 -> 2nd_place: team2 wins
```

