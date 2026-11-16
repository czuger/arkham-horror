# Vocabulaire des cartes

Comment l'effet d'une carte est écrit dans les fichiers `data/*_cards.ron`, pour que le moteur sache l'appliquer. Choisi le 16/11/2026 sur un échantillon de 39 cartes (voir plus bas). Les noms des briques sont en anglais, comme le code ; le texte imprimé de la carte reste en français dans le champ `text`.

## Principe

- Chaque carte garde son **texte imprimé** (pour l'affichage) et, à côté, ses **effets** : une liste de briques appliquées dans l'ordre.
- Une brique s'applique à l'investigateur qui résout la carte, sauf si elle dit le contraire.
- Ce que le vocabulaire ne sait pas dire est marqué **effet spécial** (`Special("nom")`) et codé à la main dans le moteur sous ce nom.
- Errata de la FAQ appliqués dans le texte et les effets (ex. « test de Combat » hors combat → test de Vigueur).

## Les quatre fichiers

| Fichier | Contenu d'une carte |
|---|---|
| `location_cards.ron` | quartier, puis une rencontre par lieu (lieu, texte, effets). Une rencontre peut avoir une condition `requires` : elle remplace alors la rencontre normale (Cœur de la Loge pour les Membres de la Loge). |
| `gate_cards.ron` | couleur du bord, puis les rencontres par Autre Monde ; la dernière, sans monde, est « Autre ». |
| `mythos_cards.ron` | nom, genre (Gros Titre, Environnement Mystique/Temps/Urbain, Rumeur), lieu de l'indice, lieu du portail, symboles qui bougent par les flèches blanches et noires, effets immédiats (`effects`), effets durables (`ongoing`), partie Rumeur (`rumor`). |
| `investigator_cards.ron` | paquet, nom, exemplaires, prix, mains, sous-titre, arme (bonus, physique/magique, défausse après usage), lancer d'un sort (modificateur, coût), effets durables (`ongoing`), capacités (`abilities`), effet spécial éventuel. |

## Briques d'effet (`Effect`)

**Ressources** (`Sanity` Santé Mentale, `Stamina` Résistance, `Money` argent, `Clues` Indices)

| Brique | Sens |
|---|---|
| `Gain(r, n)` / `Lose(r, n)` | gagnez / perdez n |
| `GainRolled(r)` / `LoseRolled(r)` | lancez un dé, gagnez / perdez autant |
| `LoseAll(r)` | perdez tout |
| `Pay(r, n)` | payez n : seulement si on a tout ; une option qui commence par un paiement n'est proposée que si on peut payer |
| `RestoreToMax(r)` | remonte au maximum |

**Tests et dés**

| Brique | Sens |
|---|---|
| `Check(skill, modifier, successes, pass, fail)` | test de compétence ; `successes` vaut 1 si absent (le `[2]` imprimé) ; effets en cas de réussite / d'échec |
| `RollDie(at_most, effects)` | lancez un dé ; effets si le résultat est au plus `at_most` (« sur un 1 », « de 1 à 3 ») |

**Cartes**

| Brique | Sens |
|---|---|
| `Draw(deck, count, keep)` | piochez `count` cartes (1 par défaut), gardez-en `keep` au choix |
| `TakeCard(card, otherwise)` | prenez cette carte précise (Acompte, Thomas F. Malone…) ; `otherwise` si elle n'est pas disponible |
| `SearchDeck(deck)` | prenez la carte de votre choix dans le paquet |
| `Shop([decks])` | regardez la première carte de chaque paquet, achetez-en au prix indiqué |
| `Sell(deck, multiplier)` | vous pouvez vendre une de vos cartes à n fois son prix |
| `Discard(cards, count)` / `DiscardAll(cards)` | défaussez n cartes au choix / toutes (`Items` = objets, ou un paquet) |
| `DiscardThis` / `ReturnThisToBox` | défaussez cette carte / remettez-la dans la boîte |

**Déplacements et états** : `MoveToStreet` (allez dans la rue), `MoveAnywhereInArkham`, `ReturnToArkham` (retour d'un Autre Monde), `Arrested`, `LostInTimeAndSpace`, `Delayed` (« restez ici au prochain tour »), `LoseNextTurn` (« perdez votre prochain tour »).

**Monstres et portails** : `MonsterAppears`, `GateAndMonsterAppear`, `MonstersInStreet(street, count)`.

**Structure**

| Brique | Sens |
|---|---|
| `Choice([options])` | il faut choisir une option possible ; rien si aucune ne l'est (« si possible ») |
| `Optional(option)` | « vous pouvez » : on fait ou on passe |
| `If(condition, then, otherwise)` | selon une condition |
| `EachInvestigator(only, effects)` | pour chaque investigateur (filtré ou non) |

**Divers** : `AddTokensToThis(n)` (pions sur une Rumeur), `CombatBonus(n)` (jusqu'à la fin du combat), `ReduceLoss(n)`, `Reroll`, `Special(nom)`.

**Conditions** : `HasCard(carte)`, `CardCountAtMost(deck, count)`, `InStreets`, `InOtherWorld`, `LostInTimeAndSpace`.

## Effets durables (`Ongoing`)

Tant que la carte est en jeu (Environnement, Rumeur) ou en main (objets, compétences, alliés…) :

`CheckBonus(check, amount, scope)` (scope `Arkham` ou partout), `SkillBonus(skill, amount)`, `ExtraClueDie(skill)`, `MaxBonus(r, n)`, `SuccessOn(face)` (Béni 4, Maudit 6), `ClueSubstitute` (se défausse à la place d'un Indice), `MovementPoints(n)`, `SpellSanityCost(n)`, `MonsterToughness(monsters, amount)`, `MonsterBanned(monstre)`, `Trigger(when, only, effects)` (à un moment donné : fin du mouvement, fin du Mythe).

## Capacités (`Ability`)

Une capacité = **moment** + **lancer un sort ?** + **condition** + **coût** + **effets**.

- Moments : `Upkeep` (Entretien, obligatoire), `Movement`, `ArkhamEncounter`, `OtherWorldEncounter`, `AnyPhase`, `OnTake` (quand on reçoit la carte), `OnLoss(r)` (au moment d'une perte), `AfterCheck(check)` (juste après un test).
- Coûts : `Exhaust` (décharger), `DiscardThis`, `MovementPoints(n)`, `Pay(r, n)`.

## Effets spéciaux de l'échantillon

| Nom | Carte | Pourquoi |
|---|---|---|
| `elder_sign` | Signe des Anciens | lié au déroulé de la fermeture d'un portail |
| `blessing_and_curse_cancel` | Bénédiction, Malédiction | l'une annule l'autre |
| `no_more_bank_loans` | Prêt Bancaire | interdit un nouveau prêt pour la partie |
| `great_ritual_spells_discarded` | Le Grand Rituel | condition de réussite (défausser 3 ou 4 sorts dans les rues de French Hill) |
| `two_mythos_cards_each_turn` | Le Grand Rituel | conséquence de l'échec |

4 cartes sur 39 en ont besoin (10 %), à la limite de l'objectif. Deux d'entre elles partagent la même règle.

## Échantillon (16/11/2026)

- **Lieu** (9, une par quartier, 28 rencontres) : clacv02, claqri01, clafh10, claqe03, claqn02, clauv07, claqm01, claqre05, claqs02.
- **Portail** (4) : cp003, cp004, cp011, cp050.
- **Mythe** (6) : Alignement Planétaire, Un Étrangleur dans le Quartier Sud ?, Le Grand Rituel, Il Pleut des Cordes, Le Couvre-feu est Renforcé !, Des Disparus Reviennent !
- **Investigateur** (20) : Révolver .38, Dynamite, Tome Ancien, Whisky, Lanterne, Matériel de Recherche, Eau Bénite, Signe des Anciens, Flétrissement, Trouver le Portail, Bravoure, Chance, Anna Kaslow, Duke, Thomas F. Malone, Bénédiction, Malédiction, Prêt Bancaire, Acompte, Membre de la Loge.

Les nombres d'exemplaires des cartes Investigateur sont de mémoire, **à vérifier** sur la boîte (sauf Prêt Bancaire, Acompte, Membre de la Loge : 8, donnés par la règle).

## Contrôles au chargement

Identifiants de cartes uniques entre tous les fichiers ; chaque carte Lieu couvre une fois chaque lieu de son quartier et aucun autre ; carte Portail : mondes existants, une seule rencontre « Autre », en dernier ; carte Mythe : indice et portail seulement sur des lieux instables, partie Rumeur présente si et seulement si c'est une Rumeur ; seuls les Sorts ont un lancer, seuls les Objets un prix et une arme ; toute carte, rue ou monstre cité existe ; chaque image existe.

Le vocabulaire évoluera quand le moteur sera construit dessus (J2-J3) puis pendant la transcription du reste des cartes.
