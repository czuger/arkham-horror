# Vocabulaire des cartes

Comment l'effet d'une carte est écrit dans les fichiers `data/*_cards.ron`, pour que le moteur sache l'appliquer. Choisi le 16/11/2026 sur un échantillon de 39 cartes, étendu au J3 (20/11/2026) à toutes les cartes du jeu de base. Les noms des briques sont en anglais, comme le code ; le texte imprimé de la carte reste en français dans le champ `text`.

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
| `Gain(r, n)` / `Lose(r, n)` | gagnez / perdez n ; deux `Lose` qui se suivent sont une perte simultanée (les deux à 0 en même temps = dévoré) |
| `GainRolled(r)` / `LoseRolled(r)` | lancez un dé, gagnez / perdez autant |
| `GainAmount(r, quantité)` / `LoseAmount([r…], quantité)` | gain ou perte d'une quantité calculée (voir **Quantités**) ; la perte vaut pour chaque ressource listée, en même temps |
| `GainSplit(quantité)` | points à répartir entre Résistance et Santé Mentale |
| `LoseAll(r)` | perdez tout |
| `Pay(r, n)` | payez n : seulement si on a tout ; une option qui commence par un paiement n'est proposée que si on peut payer |
| `RestoreToMax(r)` | remonte au maximum |
| `LowerMax(r, n)` | baisse le maximum pour le reste de la partie |

**Quantités** (`Amount`) : `Fixed(n)` ; `X` (le résultat du dernier jet de dés ou le nombre choisi, « autant ») ; `XMinus(n)` (X − n, minimum 0) ; `Successes(n)` (n par succès du dernier test) ; `HalfUp(r)` (la moitié de ce qu'on a, arrondie au-dessus).

**Tests et dés**

| Brique | Sens |
|---|---|
| `Check { skill, modifier, successes, pass, fail }` | test de compétence ; `successes` vaut 1 si absent (le `[2]` imprimé) |
| `CheckTable { skill, modifier, rows }` | test puis table « nombre de succès » : la ligne au seuil le plus haut atteint |
| `RollDie { at_most, effects }` | lancez un dé ; effets si le résultat est au plus `at_most` (« sur un 1 », « de 1 à 3 ») |
| `RollDice(n)` | lancez n dés ; le total devient X |
| `RollEach { … success, failure }` | n dés ; chaque succès / échec applique ses effets |
| `RollPerPoint` | un dé par point d'une ressource ; un point perdu par échec |
| `ChooseUpTo(n)` | l'investigateur choisit un nombre de 0 à n, qui devient X |

**Cartes**

| Brique | Sens |
|---|---|
| `Draw { deck, count, keep }` | piochez `count` cartes (1 par défaut), gardez-en `keep` au choix ; les autres vont sous le paquet |
| `TakeCard { card, otherwise }` | prenez cette carte précise ; `otherwise` si elle n'est pas disponible |
| `TakeFirst { deck, kind }` | prenez la première carte de ce genre (Tome, Arme) ; les cartes retournées vont sous le paquet |
| `SearchDeck(deck)` | prenez la carte de votre choix dans le paquet, puis remélangez |
| `Shop([decks])` | regardez la première carte de chaque paquet, achetez-en au prix indiqué |
| `Buy { draw, price, everyone }` | une carte par paquet, achat au prix `Printed`, `Plus(n)`, `Minus(n)` ou `Half` ; le reste est défaussé |
| `BuyAny([decks])` | cherchez n'importe quelle carte de ces paquets et achetez-la |
| `Sell { deck, multiplier }` | vous pouvez vendre une de vos cartes à n fois son prix |
| `Discard { cards, count }` / `DiscardAll(cards)` / `DiscardHalf` | défaussez n cartes au choix / toutes / la moitié (filtres : `Items`, `Weapons`, `Tomes`, `Deck(paquet)`) ; les cartes « ne peut être perdu » sont épargnées |
| `DiscardCard(carte)` | défaussez un exemplaire de cette carte |
| `DiscardThis` / `ReturnThisToBox` | défaussez cette carte / remettez-la dans la boîte |
| `ReturnRandomCardToBox(deck)` | une carte au hasard du paquet retourne dans la boîte |

**Déplacements et états** : `MoveToStreet` (allez dans la rue), `MoveTo(lieu)`, `MoveAnywhereInArkham`, `MoveToMythosGate` (lieu du portail d'une carte Mythe piochée, R22), `EncounterHere { draw }` (nouvelle rencontre sur place), `RedrawEncounter` (Œil du Dragon), `VisitOtherWorld(monde)` (une rencontre là-bas puis retour), `ReturnToArkham`, `AdvanceInOtherWorld`, `Arrested`, `LostInTimeAndSpace`, `Devoured`, `Delayed` (« restez ici au prochain tour »), `LoseNextTurn`, `GainMovementPoints(n)`, `BonusUntilEndOfTurn(n)`.

**Monstres, portails, pistes** : `MonsterAppears`, `GateAndMonsterAppear`, `GateAppears` (sans monstre, R20), `MonstersInStreet { street, count }`, `ReturnMonsters { area, only, terror_if_any }` (zones : `Locations`, `Streets`, `District(rue)`, `SkyAndOutskirts`, `Arkham`), `TrophyFromCup`, `TrophyFromBoard`, `SpendMonsterTrophies(force)`, `SpendGateTrophies(n)`, `CloseEntryGate` / `CloseGateOfChoice` (sans trophée, le marqueur retourne sous la pile : FAQ), `AddDoom`, `RemoveDoom`, `RaiseTerror(n)`, `TerrorTo(n)`, `RemoveAllElderSigns`.

**Structure**

| Brique | Sens |
|---|---|
| `Choice([options])` | il faut choisir une option possible ; rien si aucune ne l'est (« si possible ») |
| `Optional(option)` | « vous pouvez » : on fait ou on passe |
| `If { condition, then, otherwise }` | selon une condition |
| `EachInvestigator { only, effects }` | pour chaque investigateur (filtré ou non) |
| `OneInvestigatorHere(effets)` | un investigateur au même endroit, au choix |
| `AnyInvestigator(effets)` | « l'investigateur de votre choix », n'importe où |

**Rumeurs et jetons** : `AddTokensToThis(n)`, `RemoveTokensFromThis(n)`, `AddMonstersToThis(n)` (monstres posés sur la Rumeur), `RumorPasses`.

**Réactions** (capacités utilisées pendant un test, une perte ou un combat) : `CombatBonus(n)` (jusqu'à la fin du combat), `ReduceLoss(n)`, `CancelLoss`, `PassCheck`, `NoCombatDamage`, `Reroll`, `RerollOneDie`.

**Effet spécial** : `Special(nom)`.

**Conditions** : `HasCard(carte)`, `HasAny(filtre)`, `CardCountAtMost { deck, count }`, `HasAtLeast(r, n)`, `InStreets`, `InStreet(rue)`, `AtLocation(lieu)`, `InArkham`, `InOtherWorld`, `LostInTimeAndSpace`, `InJail`, `TokensOnThisAtLeast(n)`, `ClosingGate` (le test qui suit sert à fermer un portail), `Not(condition)`.

## Effets durables (`Ongoing`)

Tant que la carte est en jeu (Environnement, Rumeur, Gros Titre qui dure) ou en main (objets, compétences, alliés…) :

- **Tests et dés** : `CheckBonus { check, amount, scope }` (scope `Arkham` ou partout), `SkillBonus(skill, n)`, `ExtraClueDie(skill)`, `SuccessOn(face)` (Béni 4, Maudit 6), `ClueSubstitute` (se défausse à la place d'un Indice).
- **Investigateur** : `MaxBonus(r, n)`, `MovementPoints(n)`, `SpellSanityCost(n)`, `NoSpells`, `NoGain(r)` (sauf les soins), `CannotBeLost`, `DiscardOnAwakening`, `AnyGateReturn`, `NoArrests`.
- **Monstres** : `MonsterToughness { monsters, amount }`, `UndeadToughness(n)`, `MonsterBanned(monstre)`, `OnDefeat { … }` (effets en plus pour qui bat ce monstre), `Ignore(trait)` (Cauchemar, Irrésistible, Résistance Physique / Magique, Inépuisable), `FlyingDontMove`, `FastMoveNormally`.
- **Plateau** : `TerrorCannotRise`, `SealCost(n)`, `NoSealing`, `GateCloseModifier { world, amount }`, `LocationClosed(lieu)`, `StreetClosed(rue)` (ni entrer ni sortir).
- **Déclencheur** : `Trigger { when, only, effects }`, aux moments `EndOfMovement`, `EndOfMythos`, `EndOfCombat`, `BackFromOtherWorld`.

## Capacités (`Ability`)

Une capacité = **moment** + **lancer un sort ?** + **condition** (`only`) + **coût** + **effets**.

- Moments : `Upkeep` (Entretien ; obligatoire si sans coût), `Movement`, `ArkhamEncounter`, `OtherWorldEncounter`, `AnyPhase`, `OnTake` (quand on reçoit la carte), `OnLoss(r)` (au moment d'une perte), `AfterCheck(test)` / `AfterAnyCheck` (juste après un test raté), `BeforeCheck(test)` (juste avant), `AfterEncounterDraw` (juste après la pioche d'une rencontre).
- Coûts : `Exhaust` (décharger), `DiscardThis`, `MovementPoints(n)`, `Pay(r, n)`, `WholeMovement` (remplace tout le mouvement : Véhicule de Patrouille).

**Armes** (`weapon`) : bonus, physique ou magique, défausse après usage, plus `undead_bonus` (Croix), `alone_bonus` (Hache, si l'autre main est libre), `sixes_double` (Fusil à Pompe), `sanity_loss` (Poudre d'Ibn-Ghazi). **Sorts** (`casting`) : modificateur, coût, `awareness_modifier` (Brumes de Releh : le modificateur est la Conscience du monstre).

## Capacités des lieux du plateau

Dans `board.ron`, champ `ability` d'un lieu : nom, texte, effets (avec les mêmes briques, coûts en tête). Utilisée **à la place** de la rencontre, une seule fois par tour, et proposée seulement si on peut la résoudre entièrement (FAQ) : le moteur vérifie que les coûts du début peuvent être payés et que la carte précise visée existe encore. Les soins (Asile, Hôpital) passent outre les cartes Mythe qui empêchent de gagner. « Cœur de la Loge » n'a pas d'effet : ce n'est pas une action.

## Capacités des investigateurs

Les 16 capacités des fiches sont codées par identifiant d'investigateur dans `crates/engine/src/rules/investigator_powers.rs` (pas dans les données). Elles s'appliquent dès la préparation (livret §3 étape 9) : Amanda, Bob, Dexter et Monterey piochent une carte de plus dans leur paquet et en défaussent une ; Pete choisit dessus ou dessous ; Darrell pioche une carte Lieu de plus et choisit, Gloria deux cartes Portail ; Harvey et Michael réduisent chaque perte de 1 ; Joe a un dé de plus par Indice ; Jenny gagne 1 $, Carolyn et Vincent soignent 1 point à l'Entretien ; Kate empêche portails et monstres d'apparaître chez elle (vague annulée sur son portail, répartie ailleurs sinon, pas de pion Destin) ; Mandy fait relancer les dés ratés une fois par tour ; Sœur Marie n'est jamais perdue dans le temps et l'espace.

## Effets spéciaux

| Nom | Carte | Pourquoi |
|---|---|---|
| `elder_sign` | Signe des Anciens | lié au déroulé de la fermeture d'un portail |
| `blessing_and_curse_cancel` | Bénédiction, Malédiction | l'une annule l'autre |
| `no_more_bank_loans` | Prêt Bancaire | interdit un nouveau prêt pour la partie |
| `flute_of_the_outer_gods` | Flûte des Dieux Extérieurs | bat tous les monstres du lieu pendant un combat |
| `bind_monster` | Lier un Monstre | succès nécessaires = force du monstre ; échec = test de Combat raté (R26) |
| `warding_statue` | Statue de Garde | annule une attaque du Grand Ancien |
| `enchant_weapon` | Enchanter une Arme | une arme physique devient magique pour le combat |
| `red_sign` | Signe Rouge de Shudde M'ell | affaiblit le monstre pour le combat (force −1, une capacité ignorée) |
| `crystal_ball` | claqre15 | carte Lieu réservée pour le prochain dans le quartier (R23) |
| `body_swap` | clauv10 | l'investigateur part avec cartes et trophées, une fiche inutilisée au choix le remplace |
| `dimensional_ray` | clauv15 | un dé par portail ouvert, un succès le ferme (sans trophée) |
| `great_ritual_spells_discarded`, `two_mythos_cards_each_turn` | Le Grand Rituel | condition de réussite ; conséquence de l'échec |
| `rumor_clue_removal` | Du Bon Travail Gâché | Indices dépensés un pour un pour retirer les pions de la Rumeur |
| `elder_things_return` | Retour de l'Expédition en Arctique | les trophées Choses Très Anciennes reviennent aux Docks |
| `experiment_fight`, `experiment_release` | La Terrible Expérience | combat contre les monstres de la Rumeur ; leur libération |

16 cartes sur 267 en ont besoin (6 %).

## Contrôles au chargement

Identifiants de cartes uniques entre tous les fichiers ; chaque carte Lieu couvre une fois chaque lieu de son quartier et aucun autre ; carte Portail : mondes existants, une seule rencontre « Autre », en dernier ; une carte sans couleur a un texte et aucune rencontre ; carte Mythe : indice et portail seulement sur des lieux instables, partie Rumeur présente si et seulement si c'est une Rumeur, seul un Gros Titre peut durer jusqu'au prochain tour ; seuls les Sorts ont un lancer, seuls les Objets un prix ; au moins un exemplaire par carte ; toute carte, rue, lieu, monde ou monstre cité existe, y compris dans les capacités des lieux ; les tests portent sur une compétence ; chaque image existe.

## Couverture

`simulator masse --couverture FICHIER` compte, sur les parties jouées, chaque rencontre de Lieu (par lieu), chaque rencontre de Portail (par monde), chaque carte Mythe, chaque carte Investigateur reçue et chaque capacité utilisée, et écrit dans FICHIER ce qui n'a jamais servi. Au 20/11/2026, 30 000 parties à 2 joueurs couvrent tout (Lieu 182/182, Portail 144/144, Mythe 67/67, cartes reçues 88/88, capacités 69/69).
