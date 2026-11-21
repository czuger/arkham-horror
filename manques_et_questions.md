# Horreur à Arkham — Manques et questions pour la création du jeu

Rédigé le 16/11/2026, en complément de `plan_creation_jeu.md`. Les numéros M1 à M20 renvoient à `plan_mise_en_ligne.md` Partie 2 ; cette page dit **où on en est** et ce qui reste à obtenir. Les questions sont numérotées Q1, Q2… : réponds-y directement ici, en dessous de chaque question, ou dans la conversation.

Légende : 🔴 bloquant · 🟠 important · 🟢 confort

---

## 1. Données du jeu

Depuis le 16/11/2026, les **images** de toutes les cartes et fiches du jeu de base sont dans `assets/`. Le texte y est lisible, mais pas encore transcrit. Le travail qui reste n'est donc plus de « trouver » les données, mais de les **lire et saisir**.

| # | Élément | Ce qu'on a | Ce qui manque | Priorité |
|---|---|---|---|---|
| M1 | ~~Graphe du plateau~~ | **Fait le 16/11/2026** : `data/board.ron` (lieux, rues, liaisons, flèches, Autres Mondes, cases spéciales, positions approximatives). Relevé sur l'image : à vérifier sur ton plateau. Icônes de rencontre saisies le 21/11/2026 (`icons`, et la zone blanche `panel` de chaque lieu) pour les bulles de survol ; à vérifier sur ton plateau. | — | 🟢 |
| M2 | Cartes Lieu (63) | images dans `assets/cartes/lieu/` | texte des rencontres | 🔴 |
| M3 | Cartes Portail (49) | images dans `assets/cartes/portail/` | texte des rencontres | 🔴 |
| M4 | Cartes Mythe (67) | images dans `assets/cartes/mythe/` | type, lieu du portail, lieu de l'Indice, flèches de mouvement, effet | 🔴 |
| M5 | ~~Grands Anciens (8)~~ | **Fait le 16/11/2026** : `data/ancient_ones.ron`, transcrit des fiches. | à relire | 🟢 |
| M6 | Objets uniques | images (27 cartes distinctes) | effet exact, et **nombre d'exemplaires** de chacune (39 cartes dans la boîte) | 🔴 |
| M7 | Sorts | images (11 cartes distinctes) + tableau du wiki (coût, modificateur, mains, phase) | effet exact, nombre d'exemplaires (40 cartes) | 🔴 |
| M8 | Compétences | images (10 cartes distinctes) | effet exact, nombre d'exemplaires (20 cartes) | 🔴 |
| M9 | Objets communs | images (22 cartes distinctes) + pages wiki partielles | liste complète, nombre d'exemplaires (44 cartes) | 🔴 |
| M10 | ~~Investigateurs (16)~~ | **Fait le 16/11/2026** : `data/investigators.ron`, transcrit des fiches (pouvoirs, curseurs, possessions). | à relire | 🟢 |
| M11 | Alliés (11) | images + tableau du wiki | texte exact | 🟠 |
| M12 | Cartes spéciales | images dans `assets/cartes/speciales/` | règles exactes, nombres d'exemplaires | 🟠 |
| M13 | ~~Monstres~~ | **Fait le 16/11/2026** : `data/monsters.ron`, relu sur les 29 jetons. Le tableau du wiki avait des erreurs (Chien de Tindalos, Sombre Rejeton…), corrigées d'après les images. | à relire | 🟢 |
| M14 | Errata | images dans `assets/errata/` | repérer quelles cartes de `assets/cartes/` doivent être remplacées par leur version corrigée | 🟠 |
| M15 | ~~Marqueurs Portail (16)~~ | **Fait le 16/11/2026** : `data/gate_markers.ron`, 2 par monde, modificateurs et symboles vérifiés sur les images. | — | 🟢 |

Manques nouveaux, constatés en préparant le plan :

| # | Élément | Détail | Priorité |
|---|---|---|---|
| M21 | ~~**Nombres d'exemplaires**~~ | **Comblé le 16/11/2026** : contenu de la boîte et répartition par carte des sorts, objets uniques et monstres fournis par l'utilisateur (voir ci-dessous). Seuls les objets communs restent supposés à 2 exemplaires chacun. | 🟢 |
| M22 | **Symboles** | Icônes de rencontre, symboles de dimension, lieu stable/instable, récompenses et pénalités : absents de l'aspiration (voir `assets/inventaire_assets.md` « Manquants »). Le moteur peut s'en passer (ce sont des données, pas des images), mais l'interface devra les redessiner ou s'en passer. Icônes de rencontre : découpées dans une légende fournie par l'utilisateur, `assets/marqueurs/icones_rencontre/` (21/11/2026). | 🟢 |
| M23 | **Dos des cartes Mythe** et pions des investigateurs | absents. L'interface utilisera un rectangle de couleur à la place. | 🟢 |
| M24 | ~~Couleurs de rencontre des Autres Mondes~~ | **Vérifié le 16/11/2026** : les pages du wiki et le plateau concordent pour les 8 mondes. | — | 🟢 |

### Contenu de la boîte (fourni par l'utilisateur le 16/11/2026)

Totaux officiels, utiles au moteur (taille des paquets, réserves de pions) :

| Élément | Quantité | Ce qu'on en déduit |
|---|---|---|
| Objets communs | 44 cartes, 22 distinctes dans `assets/` | **2 exemplaires de chacune** (à confirmer : 22 × 2 = 44) |
| Objets uniques | 39 cartes, 27 distinctes | répartition inégale, **encore inconnue** |
| Sorts | 40 cartes, 11 distinctes | répartition inégale, **encore inconnue** |
| Compétences | 20 cartes, 10 distinctes | **2 exemplaires de chacune** |
| Alliés | 11 cartes, 11 distinctes | 1 exemplaire de chacun |
| Cartes spéciales | 35 : 8 Acompte, 8 Membre de la Loge, 8 Prêt Bancaire, 8 Bénédiction/Malédiction (recto-verso), 3 Adjoint au Shérif | complet |
| Cartes Lieu / Mythe / Portail | 63 / 67 / 49 | une de chaque, complet |
| Monstres | 60 marqueurs dont 5 Masques, 29 jetons distincts dans `assets/` | répartition par monstre **encore inconnue** |
| Marqueurs Portail | 16 | répartition par Autre Monde à vérifier (M15) |
| Pions | 56 Argent, 34 Santé Mentale (10 × « 3 », 24 × « 1 »), 34 Résistance (idem), 48 Indice, 24 coulissants, 20 Destin, 3 Activité, 3 Exploré, 6 Lieu Fermé, 1 Terreur, 1 Premier Joueur, 5 dés | réserves ; le livret ne les traite pas comme limitées (sauf Destin), à trancher en R8 |
| Investigateurs / Grands Anciens | 16 feuilles / 8 feuilles | complet |

### Répartition par carte (fournie par l'utilisateur le 16/11/2026)

Les sommes sont vérifiées : 40 sorts, 39 objets uniques, 60 monstres. L'astérisque vient de la source (sans doute une carte corrigée par errata, à rapprocher de `assets/errata/`).

**Sorts (11 noms, 40 cartes)** : Atrophie ×6 · Brumes de Releh ×4 · Enchanter une Arme ×3 · Flétrissement ×5 · Lier un Monstre ×2 · Rempart de Chair\* ×4 · Signe Rouge de Shudde M'ell ×2 · Soin ×3 · Terrible Malédiction d'Azathoth ×4 · Trouver le Portail ×4 · Voix de Ra ×3.

**Objets uniques (26 noms, 39 cartes)** : Boîte à Portail · La Cabale de Saboth ×2 · Clé d'Argent · Couteau Enchanté ×2 · Culte des Goules ×2 · Cultes Innommables ×2 · Eau Bénite ×4 · Épée de Gloire · Flûte des Dieux Extérieurs · Gardien Bleu de la Pyramide · Joyaux Enchantés · Lame Enchantée ×2 · Lampe d'Alhazred · Livre de Dzyan · Masque Blême · Necronomicon · Œil du Dragon · Pierre de Soins\* · Poudre d'Ibn-Ghazi ×2 · Le Roi en Jaune ×2 · Rubis de R'lyeh · Signe des Anciens ×4 · Statue de Garde · Statue d'Obsidienne · Statue Extraterrestre · Tablette Antique. (Sans multiplicateur : 1 exemplaire.)

**Monstres (29 noms, 60 marqueurs)** : Byakhee ×3 · Chien de Tindalos ×2 · Chose Très Ancienne ×2 · Chthonien ×2 · Cultiste ×6 · Dhole · Fantôme ×3 · Goule ×3 · Grand Prêtre · Gug ×2 · Larve Amorphe ×2 · Larve Stellaire ×2 · Maigre Bête de la Nuit ×2 · Maniaque ×3 · Mi-Go ×3 · Polype Volant · Shoggoth ×2 · Sombre Rejeton ×3 · Sorcier ×2 · Sorcière ×2 · Vagabond Dimensionnel ×2 · Vampire · Vampire de Feu ×2 · Zombie ×3 (24 noms, 55 marqueurs) · plus les **5 Masques**, 1 chacun : Dieu de la Langue Sanglante, La Femme Boursouflée, L'Homme en Noir, Le Pharaon Noir, Tourmenteur des Ténèbres.

**M21 est donc comblé.** Les objets communs restent supposés à 2 exemplaires de chacun (22 × 2 = 44), à confirmer.

### Comment combler M2 à M12 : proposition

Je peux **lire les images moi-même** et transcrire le texte des cartes, paquet par paquet, directement dans le format de données choisi. Tu relis ensuite contre ta boîte. Cela évite l'OCR (et l'installation d'outils). Ordre proposé : investigateurs et Grands Anciens, puis Mythe, puis objets et sorts, puis Lieu et Portail (les plus longs).

Pour M1, je peux proposer le graphe à partir de l'image du plateau et de ma connaissance du jeu, et le mettre dans un fichier que tu valides contre ton plateau.

---

## 2. Points techniques à vérifier au jalon J0

| # | Point | Pourquoi |
|---|---|---|
| T1 | La base embarquée fonctionne bien sous tokio **current_thread** | La base peut lancer des tâches internes ; il faut confirmer qu'aucune n'exige un runtime multi-thread. |
| T2 | Temps de compilation et taille de l'exécutable avec la base embarquée | À mesurer pour savoir si c'est supportable au quotidien. |
| T3 | Maturité du stockage | Vérifier que le format est stable, et prévoir des sauvegardes régulières. |
| T4 | Format de sérialisation de l'État dans la base | Stocker l'État comme un bloc JSON est le plus simple ; à vérifier que la taille (quelques dizaines de ko par partie) et les écritures à chaque coup ne posent pas de problème. |
| T5 | Choix du moteur de gabarits | askama (gabarits dans des fichiers, vérifiés à la compilation) ou maud (HTML écrit en Rust). Les deux conviennent ; voir Q3. |
| T6 | Plateau cliquable sans JS | Des liens positionnés en absolu sur l'image, avec les coordonnées de M1. À vérifier sur petit écran (le plateau est haut : 1064×1576). |

Toutes ces vérifications se font **sans rien télécharger d'autre que les crates Rust** via cargo. Si même cela doit être évité, dis-le (Q2).

### Résultats du jalon J0 (16/11/2026)

Première version faite avec SurrealDB 4.0.1, puis **remplacée par PoloDB 5.3.0** à ta demande le même jour (SurrealDB : 23 threads, licence BSL 1.1, compilation à froid de 6 à 7 min, exécutable de 117 Mo, deux alertes de sécurité sans correctif). Mesures ci-dessous avec PoloDB, sur ton Mac (10 cœurs), axum 0.8.9, askama 0.16.1, tokio 1.54, Rust 1.100.0.

- **T1 — fait.** PoloDB tourne sans problème sous tokio current_thread : écriture, lecture, arrêt, redémarrage avec les données conservées. Le processus compte **4 threads** : 1 pour le jeu et le serveur web, 3 threads d'appoint du stockage (RocksDB). Les appels à PoloDB sont bloquants mais très courts ; la base se ferme et s'écrit sur disque toute seule à l'arrêt du serveur.
- **T2 — fait.** Compilation à froid : **2 min 25** (debug), dont l'essentiel pour RocksDB, écrit en C++. Exécutable release : **12 Mo**.
- **T3 — fait.** PoloDB 5 range ses données avec RocksDB, un moteur de stockage très éprouvé. Licence Apache-2.0. Sauvegarde : copier le dossier de base, serveur arrêté. Point d'attention : PoloDB est maintenu par une petite équipe ; comme la base ne contient que les parties sauvegardées, en changer plus tard resterait facile.
- **T4 — fait.** État stocké comme un texte JSON : 200 écritures de 50 ko prennent **6 ms** au total. Aucun problème.
- **T5 — fait : askama** (ta réponse du 16/11/2026 à Q3).
- **21/11/2026 : PoloDB remplacée par SQLite** (sqlx 0.9, SQLite compilé avec le programme). Un seul fichier `arkham.db` au lieu d'un dossier ; plus de RocksDB à compiler. T4 refait : 200 écritures de 50 ko en **26 ms** (debug). Les anciennes parties PoloDB ne sont pas reprises.
- **T6 — reporté au jalon J5** : il faut d'abord les coordonnées des lieux sur l'image du plateau (M1).

---

## 3. Règles à trancher (tranchées)

Reprises de `regles/base.md` §22 et de `plan_mise_en_ligne.md` §3.2.

**Toutes tranchées le 16/11/2026** : tu as accepté chaque recommandation. Le détail (décision et raison) est dans `regles/decisions.md`, qui fait foi.

- **R1** Monstres à l'ouverture d'un portail : 2 monstres à partir de **5 joueurs ou plus** (règle VO) ? *(Recommandé : oui.)*
- **R2** Le « nombre de joueurs » (limite de monstres, Périphérie, réveil, trophées, Bataille finale) compte **tous les investigateurs**, humain et IA ? *(Recommandé : oui.)*
- **R3** Monstre volant sans investigateur à portée : il reste sur place ? *(Recommandé : oui.)* → **Corrigée par la FAQ officielle** : il part dans le Ciel (voir `regles/decisions.md`).
- **R4** Le marqueur Premier Joueur tourne normalement entre l'humain et les IA ? *(Recommandé : oui. Quand une IA est premier joueur, c'est elle qui tranche les arbitrages, avec une règle fixe : le choix le pire pour les investigateurs, conformément à la règle d'or.)*
- **R5** Échanges d'équipement : les Alliés et Compétences ne sont pas échangeables (le livret ne les cite pas) ? *(Recommandé : oui, non échangeables.)*
- **R6** Points de victoire (règle optionnelle §10.3) : calculés et affichés en fin de partie ? *(Recommandé : oui, à titre indicatif.)*
- **R7** FAQ officielle (`FAQ_Officiel_032017_V2.pdf`) : je l'intègre dans `regles/base.md` avant de coder le moteur ? *(Recommandé : oui, au jalon J1.)* → **Fait le 16/11/2026** : `regles/base.md` §23.
- **R8** Réserves de pions (Argent, Santé Mentale, Résistance, Indices, Signes des Anciens…) : limitées aux quantités de la boîte, ou illimitées ? *(Recommandé : illimitées, sauf les 20 pions Destin et les 6 Signes des Anciens nécessaires à la victoire ; c'est l'usage courant et cela évite des cas limites.)*

### Nouveaux points apparus en écrivant le moteur (J2, 16/11/2026)

**Tous tranchés le 16/11/2026** : tu as accepté chaque recommandation. Le détail est dans `regles/decisions.md`, qui fait foi.

- **R9** Vague de monstres quand la limite est atteinte : le livret laisse les joueurs choisir quels portails reçoivent les monstres avant d'aller en Périphérie. *(Recommandé : ordre fixe, le portail de la vague d'abord puis les autres dans l'ordre d'ouverture ; l'effet est minime et cela évite une décision de plus.)*
- **R10** Armes et sorts en combat : le livret permet de changer d'armes d'un round à l'autre. Aujourd'hui, une carte prise en main le reste jusqu'à la fin du combat (sauf usage unique). *(Recommandé : ajouter la possibilité de lâcher une carte entre deux rounds, règle complète.)*
- **R11** Indices : en fin de mouvement, l'investigateur prend tous les Indices du lieu ; seul sur le lieu où le Mythe pose un Indice, il le prend. Le livret dit « peut ». *(Recommandé : automatique, ce n'est jamais un désavantage.)*
- **R12** Effets « qui finit son mouvement » (Couvre-feu) : un investigateur retardé n'a pas de mouvement, il n'est donc pas concerné. *(Recommandé : oui.)*
- **R13** La Femme Boursouflée : rater le test de Volonté fait rater tous les tests de Combat de ce combat, pas seulement le premier. *(Recommandé : oui, tout le combat.)*
- **R14** Investigateur dévoré : la nouvelle fiche est tirée parmi les fiches inutilisées, l'ancienne comprise (lecture littérale de « remélange sa fiche »). *(Recommandé : oui, lecture littérale.)*
- **R15** Ithaqua, début de la Bataille : « un échec » au dé lancé pour chaque objet = un dé qui n'est pas un succès (1 à 4 ; 1 à 3 si béni ; 1 à 5 si maudit). *(Recommandé : oui.)*
- **R16** Un bonus « aux tests de Discrétion » (Il Pleut des Cordes) compte aussi pour les tests d'Évasion, qui sont des tests de Discrétion (§11.6). Un bonus aux tests d'Évasion ne compte pas pour la Discrétion. *(Recommandé : oui.)*
- **R17** Chien de Tindalos : s'il n'y a aucun investigateur dans un lieu où il peut aller, il ne bouge pas. *(Recommandé : oui.)*

### Points apparus en transcrivant toutes les cartes (J3, 20/11/2026)

**Tous tranchés le 20/11/2026** (détail dans `regles/decisions.md`) :

- **R18** cp036 « … et votre repas » : texte d'ambiance.
- **R19** cp116 « perdez connaissance et restez ici » : simple retard.
- **R20** clacv14 : un portail s'ouvre et aspire l'investigateur, sans monstre.
- **R21** Restaurant de Velma : exactement 6 $ pour 6 points.
- **R22** « Allez à l'endroit inscrit sur cette carte » : lieu du portail de la carte Mythe piochée.
- **R23** Boule de cristal : la carte réservée sert au prochain investigateur qui fait une rencontre dans le quartier.
- **R24** R'lyeh Émerge : fermeture vers R'lyeh à −1 de plus ; scellement inchangé.
- **R25** Une Rumeur terminée retourne dans la boîte (texte de la carte).
- **R26** Lier un Monstre raté = test de Combat raté.
- **R27** « Combat sans issue » (FAQ) étendu au cas où la fuite est permise mais impossible.

---

## 4. Questions

### Technique

- ~~**Q1** Version de SurrealDB visée ?~~
  → **Réponse (16/11/2026) : SurrealDB abandonnée, remplacée par PoloDB** (dernière version, 5.3.0).
- **Q2** Les dépendances Rust (crates) peuvent être téléchargées par cargo ? *(Je pars du principe que oui : l'interdiction de téléchargement concerne les données du jeu.)*
  → **Réponse (16/11/2026) : oui.**
- **Q3** Moteur de gabarits : askama (fichiers HTML séparés) ou maud (HTML dans le code Rust) ? *(Recommandé : askama, plus facile à relire pour quelqu'un qui ne code pas.)*
  → **Réponse (16/11/2026) : askama.**
- **Q4** « Mono thread » : tokio en current_thread suffit ? Ou veux-tu aussi interdire les threads d'appoint de la base pour les écritures disque (ce qui n'est pas toujours possible) ?
  → **Constat (16/11/2026)** : avec PoloDB, 3 threads d'appoint seulement (voir T1). Le jeu tourne en current_thread.
- **Q5** Le JS « quand c'est nécessaire » : uniquement des scripts maison de quelques lignes, ou une petite bibliothèque comme htmx est acceptable ? *(Recommandé : scripts maison, aucune bibliothèque.)*
  → **Réponse (16/11/2026) : une bibliothèque est acceptée, et même préférable ; Vue est possible si c'est mieux.** Choix retenu : **htmx** (voir `plan_creation_jeu.md` §6.4). Vue reste en réserve si le plateau demande une interaction trop riche pour htmx.
  → **Précision (21/11/2026) : JavaScript est permis ; ce qui est refusé, c'est un client lourd** (pas d'application dans le navigateur, pas de règles ni d'état de partie côté navigateur). Jouer sans JavaScript n'est plus exigé.
- **Q6** Édition Rust 2024 et version stable la plus récente ?
  → **Réponse (16/11/2026) : oui, la dernière version.** Édition 2024, Rust 1.100.0, épinglé dans `rust-toolchain.toml`.

### Données

- **Q7** Es-tu d'accord pour que je transcrive les cartes à partir des images (voir §1), que tu relis ensuite ? Sinon, préfères-tu saisir toi-même, ou passer par un OCR ?
  → **Réponse (16/11/2026) : oui, transcription par moi.**
- **Q8** As-tu la boîte sous la main pour compter les exemplaires de chaque carte et de chaque monstre (M21) ? Sinon je proposerai les nombres de mémoire, marqués « à vérifier ».
  → **En suspens (16/11/2026) : tu fais les recherches.** En attendant, les nombres d'exemplaires seront saisis de mémoire et marqués « à vérifier ».
  → **Réponse complète (16/11/2026)** : contenu de la boîte et répartition par carte des sorts, objets uniques et monstres fournis (voir §1). Question close. Seul point à confirmer : 2 exemplaires de chaque objet commun.
- **Q9** Les cartes corrigées par errata remplacent-elles systématiquement l'original (M14) ? *(Recommandé : oui.)*

### Jeu et IA

- **Q10** Nombre d'IA : fixé à la création de la partie, entre 1 et 7 ? *(Recommandé : oui, choisi à chaque partie.)*
  → **Pris par défaut (21/11/2026) : oui, choisi à chaque partie, de 1 à 7.**
- **Q11** Investigateurs des IA : tirés au hasard ou choisis par toi ? *(Recommandé : au choix, avec « hasard » par défaut.)*
  → **Réponse (21/11/2026) : au choix, « hasard » par défaut.**
- **Q12** Les IA sont-elles totalement autonomes, ou veux-tu pouvoir leur donner un ordre (« va à tel lieu », « donne-moi tel objet ») ? *(Recommandé pour la première version : autonomes, échanges proposés par l'IA et acceptés par toi.)*
  → **Réponse (21/11/2026) : IA autonomes ; les échanges viendront plus tard, avec le réglage de l'IA (pas dans J5).**
- **Q13** Le Grand Ancien : tiré au hasard ou choisi ?
  → **Réponse (21/11/2026) : au choix, « hasard » par défaut.**
- **Q14** Veux-tu voir les décisions des IA pas à pas (une page par action d'IA, avec un bouton « suivant »), ou d'un bloc (le serveur joue tous les tours d'IA et tu ne vois que le journal) ? *(Recommandé : d'un bloc, avec un journal détaillé, et une option « pas à pas » plus tard.)*
  → **Réponse (16/11/2026) : d'un bloc.**
- **Q15** Les dés : montrés dans le journal (« 3 dés : 2, 5, 6 ») ? *(Recommandé : oui.)*
  → **Pris par défaut (21/11/2026) : oui.**
- **Q16** Annulation d'un coup : autorisée sans limite (confort, usage privé), ou interdite pour garder l'esprit du jeu ?
  → **Réponse (21/11/2026) : sans limite.**

### Interface

- **Q17** Navigateur de bureau seulement, ou aussi téléphone et tablette ? Cela change la façon d'afficher le plateau.
  → **Réponse (21/11/2026) : ordinateur seulement.**
- **Q18** Français uniquement ? *(Recommandé : oui.)*
  → **Pris par défaut (21/11/2026) : oui.**
- **Q19** Affichage des cartes : image seule, texte seul, ou les deux ? *(Recommandé : les deux.)*
  → **Pris par défaut (21/11/2026) : les deux.**

### Hébergement

- **Q20** Où tourne le serveur : un petit serveur loué (VPS), une machine chez toi, ou uniquement en local sur ton Mac au début ?
  → **Réponse (16/11/2026) : les deux, mais l'hébergement est hors périmètre : tu as un environnement dédié. Le projet doit seulement fonctionner en local.** Les questions Q21 à Q23 tombent donc.
- ~~**Q21** Qui a accès : toi seul, ou quelques proches ? Un seul compte ou plusieurs ?~~ *(hors périmètre)*
- ~~**Q22** Protection : mot de passe dans l'application + HTTPS par Caddy, ou Cloudflare Access via un tunnel ?~~ *(hors périmètre)*
- ~~**Q23** Nom de domaine disponible ?~~ *(hors périmètre)*
