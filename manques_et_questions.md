# Horreur à Arkham — Manques et questions pour la création du jeu

Rédigé le 16/11/2026, en complément de `plan_creation_jeu.md`. Les numéros M1 à M20 renvoient à `plan_mise_en_ligne.md` Partie 2 ; cette page dit **où on en est** et ce qui reste à obtenir. Les questions sont numérotées Q1, Q2… : réponds-y directement ici, en dessous de chaque question, ou dans la conversation.

Légende : 🔴 bloquant · 🟠 important · 🟢 confort

---

## 1. Données du jeu

Depuis le 16/11/2026, les **images** de toutes les cartes et fiches du jeu de base sont dans `assets/`. Le texte y est lisible, mais pas encore transcrit. Le travail qui reste n'est donc plus de « trouver » les données, mais de les **lire et saisir**.

| # | Élément | Ce qu'on a | Ce qui manque | Priorité |
|---|---|---|---|---|
| M1 | Graphe du plateau | l'image `assets/plateau/arkham.jpg` (et l'agrandissement ×1,5), la liste des lieux par quartier, stable/instable | liaisons lieu↔rue et rue↔rue, flèches blanches et noires, icônes de rencontre, zones des Autres Mondes, **position en pixels** de chaque case pour l'affichage | 🔴 |
| M2 | Cartes Lieu (63) | images dans `assets/cartes/lieu/` | texte des rencontres | 🔴 |
| M3 | Cartes Portail (49) | images dans `assets/cartes/portail/` | texte des rencontres | 🔴 |
| M4 | Cartes Mythe (67) | images dans `assets/cartes/mythe/` | type, lieu du portail, lieu de l'Indice, flèches de mouvement, effet | 🔴 |
| M5 | Grands Anciens (8) | images dans `assets/fiches/grands_anciens/` + pages wiki partielles | fiches complètes en texte | 🔴 |
| M6 | Objets uniques | images (27 cartes distinctes) | effet exact, et **nombre d'exemplaires** de chacune (39 cartes dans la boîte) | 🔴 |
| M7 | Sorts | images (11 cartes distinctes) + tableau du wiki (coût, modificateur, mains, phase) | effet exact, nombre d'exemplaires (40 cartes) | 🔴 |
| M8 | Compétences | images (10 cartes distinctes) | effet exact, nombre d'exemplaires (20 cartes) | 🔴 |
| M9 | Objets communs | images (22 cartes distinctes) + pages wiki partielles | liste complète, nombre d'exemplaires (44 cartes) | 🔴 |
| M10 | Investigateurs (16) | images des fiches + tableau du wiki | texte exact du pouvoir, les 4 crans de chaque curseur, possessions fixes et aléatoires | 🔴 |
| M11 | Alliés (11) | images + tableau du wiki | texte exact | 🟠 |
| M12 | Cartes spéciales | images dans `assets/cartes/speciales/` | règles exactes, nombres d'exemplaires | 🟠 |
| M13 | Monstres | images (29 jetons distincts) + tableau du wiki | nombre d'exemplaires de chaque monstre dans la boîte, capacités hors mots-clés, les 5 Masques | 🟠 |
| M14 | Errata | images dans `assets/errata/` | repérer quelles cartes de `assets/cartes/` doivent être remplacées par leur version corrigée | 🟠 |
| M15 | Marqueurs Portail (16) | 8 images + page wiki | vérifier monde et modificateur de chacun, et combien de marqueurs par monde | 🟢 |

Manques nouveaux, constatés en préparant le plan :

| # | Élément | Détail | Priorité |
|---|---|---|---|
| M21 | ~~**Nombres d'exemplaires**~~ | **Comblé le 16/11/2026** : contenu de la boîte et répartition par carte des sorts, objets uniques et monstres fournis par l'utilisateur (voir ci-dessous). Seuls les objets communs restent supposés à 2 exemplaires chacun. | 🟢 |
| M22 | **Symboles** | Icônes de rencontre, symboles de dimension, lieu stable/instable, récompenses et pénalités : absents de l'aspiration (voir `assets/inventaire_assets.md` « Manquants »). Le moteur peut s'en passer (ce sont des données, pas des images), mais l'interface devra les redessiner ou s'en passer. | 🟢 |
| M23 | **Dos des cartes Mythe** et pions des investigateurs | absents. L'interface utilisera un rectangle de couleur à la place. | 🟢 |
| M24 | **Texte des rencontres des Autres Mondes par couleur** | Les pages Autre Monde donnent les couleurs de rencontre de chaque monde ; à vérifier qu'elles sont complètes pour les 8 mondes du jeu de base. | 🟠 |

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
| T1 | SurrealDB embarquée avec `kv-surrealkv` fonctionne bien sous tokio **current_thread** | SurrealDB lance des tâches internes ; il faut confirmer qu'aucune n'exige un runtime multi-thread. |
| T2 | Temps de compilation et taille de l'exécutable avec SurrealDB embarquée | SurrealDB est une grosse dépendance (plusieurs minutes de compilation à froid). À mesurer pour savoir si c'est supportable au quotidien. |
| T3 | Maturité de surrealkv | C'est le moteur de stockage le plus récent de SurrealDB. Vérifier dans la version choisie qu'il est déclaré stable, et prévoir des sauvegardes régulières du fichier. |
| T4 | Format de sérialisation de l'État dans SurrealDB | Stocker l'État comme un bloc JSON est le plus simple ; à vérifier que la taille (quelques dizaines de ko par partie) et les écritures à chaque coup ne posent pas de problème. |
| T5 | Choix du moteur de gabarits | askama (gabarits dans des fichiers, vérifiés à la compilation) ou maud (HTML écrit en Rust). Les deux conviennent ; voir Q3. |
| T6 | Plateau cliquable sans JS | Des liens positionnés en absolu sur l'image, avec les coordonnées de M1. À vérifier sur petit écran (le plateau est haut : 1064×1576). |

Toutes ces vérifications se font **sans rien télécharger d'autre que les crates Rust** via cargo. Si même cela doit être évité, dis-le (Q2).

---

## 3. Règles non tranchées

Reprises de `regles/base.md` §22 et de `plan_mise_en_ligne.md` §3.2. Chaque réponse sera consignée dans `regles/decisions.md`.

- **R1** Monstres à l'ouverture d'un portail : 2 monstres à partir de **5 joueurs ou plus** (règle VO) ? *(Recommandé : oui.)*
- **R2** Le « nombre de joueurs » (limite de monstres, Périphérie, réveil, trophées, Bataille finale) compte **tous les investigateurs**, humain et IA ? *(Recommandé : oui.)*
- **R3** Monstre volant sans investigateur à portée : il reste sur place ? *(Recommandé : oui.)*
- **R4** Le marqueur Premier Joueur tourne normalement entre l'humain et les IA ? *(Recommandé : oui. Quand une IA est premier joueur, c'est elle qui tranche les arbitrages, avec une règle fixe : le choix le pire pour les investigateurs, conformément à la règle d'or.)*
- **R5** Échanges d'équipement : les Alliés et Compétences ne sont pas échangeables (le livret ne les cite pas) ? *(Recommandé : oui, non échangeables.)*
- **R6** Points de victoire (règle optionnelle §10.3) : calculés et affichés en fin de partie ? *(Recommandé : oui, à titre indicatif.)*
- **R7** FAQ officielle (`FAQ_Officiel_032017_V2.pdf`) : je l'intègre dans `regles/base.md` avant de coder le moteur ? *(Recommandé : oui, au jalon J1.)*
- **R8** Réserves de pions (Argent, Santé Mentale, Résistance, Indices, Signes des Anciens…) : limitées aux quantités de la boîte, ou illimitées ? *(Recommandé : illimitées, sauf les 20 pions Destin et les 6 Signes des Anciens nécessaires à la victoire ; c'est l'usage courant et cela évite des cas limites.)*

---

## 4. Questions

### Technique

- **Q1** Version de SurrealDB visée : la dernière version stable au moment de J0, ou une version précise que tu as déjà en tête ?
- **Q2** Les dépendances Rust (crates) peuvent être téléchargées par cargo ? *(Je pars du principe que oui : l'interdiction de téléchargement concerne les données du jeu.)*
  → **Réponse (16/11/2026) : oui.**
- **Q3** Moteur de gabarits : askama (fichiers HTML séparés) ou maud (HTML dans le code Rust) ? *(Recommandé : askama, plus facile à relire pour quelqu'un qui ne code pas.)*
- **Q4** « Mono thread » : tokio en current_thread suffit ? Ou veux-tu aussi interdire les threads d'appoint de tokio et SurrealDB pour les écritures disque (ce qui n'est pas toujours possible) ?
- **Q5** Le JS « quand c'est nécessaire » : uniquement des scripts maison de quelques lignes, ou une petite bibliothèque comme htmx est acceptable ? *(Recommandé : scripts maison, aucune bibliothèque.)*
  → **Réponse (16/11/2026) : une bibliothèque est acceptée, et même préférable ; Vue est possible si c'est mieux.** Choix retenu : **htmx** (voir `plan_creation_jeu.md` §6.4). Vue reste en réserve si le plateau demande une interaction trop riche pour htmx.
- **Q6** Édition Rust 2024 et version stable la plus récente ?

### Données

- **Q7** Es-tu d'accord pour que je transcrive les cartes à partir des images (voir §1), que tu relis ensuite ? Sinon, préfères-tu saisir toi-même, ou passer par un OCR ?
  → **Réponse (16/11/2026) : oui, transcription par moi.**
- **Q8** As-tu la boîte sous la main pour compter les exemplaires de chaque carte et de chaque monstre (M21) ? Sinon je proposerai les nombres de mémoire, marqués « à vérifier ».
  → **En suspens (16/11/2026) : tu fais les recherches.** En attendant, les nombres d'exemplaires seront saisis de mémoire et marqués « à vérifier ».
  → **Réponse complète (16/11/2026)** : contenu de la boîte et répartition par carte des sorts, objets uniques et monstres fournis (voir §1). Question close. Seul point à confirmer : 2 exemplaires de chaque objet commun.
- **Q9** Les cartes corrigées par errata remplacent-elles systématiquement l'original (M14) ? *(Recommandé : oui.)*

### Jeu et IA

- **Q10** Nombre d'IA : fixé à la création de la partie, entre 1 et 7 ? *(Recommandé : oui, choisi à chaque partie.)*
- **Q11** Investigateurs des IA : tirés au hasard ou choisis par toi ? *(Recommandé : au choix, avec « hasard » par défaut.)*
- **Q12** Les IA sont-elles totalement autonomes, ou veux-tu pouvoir leur donner un ordre (« va à tel lieu », « donne-moi tel objet ») ? *(Recommandé pour la première version : autonomes, échanges proposés par l'IA et acceptés par toi.)*
- **Q13** Le Grand Ancien : tiré au hasard ou choisi ?
- **Q14** Veux-tu voir les décisions des IA pas à pas (une page par action d'IA, avec un bouton « suivant »), ou d'un bloc (le serveur joue tous les tours d'IA et tu ne vois que le journal) ? *(Recommandé : d'un bloc, avec un journal détaillé, et une option « pas à pas » plus tard.)*
  → **Réponse (16/11/2026) : d'un bloc.**
- **Q15** Les dés : montrés dans le journal (« 3 dés : 2, 5, 6 ») ? *(Recommandé : oui.)*
- **Q16** Annulation d'un coup : autorisée sans limite (confort, usage privé), ou interdite pour garder l'esprit du jeu ?

### Interface

- **Q17** Navigateur de bureau seulement, ou aussi téléphone et tablette ? Cela change la façon d'afficher le plateau.
- **Q18** Français uniquement ? *(Recommandé : oui.)*
- **Q19** Affichage des cartes : image seule, texte seul, ou les deux ? *(Recommandé : les deux.)*

### Hébergement

- **Q20** Où tourne le serveur : un petit serveur loué (VPS), une machine chez toi, ou uniquement en local sur ton Mac au début ?
  → **Réponse (16/11/2026) : les deux, mais l'hébergement est hors périmètre : tu as un environnement dédié. Le projet doit seulement fonctionner en local.** Les questions Q21 à Q23 tombent donc.
- ~~**Q21** Qui a accès : toi seul, ou quelques proches ? Un seul compte ou plusieurs ?~~ *(hors périmètre)*
- ~~**Q22** Protection : mot de passe dans l'application + HTTPS par Caddy, ou Cloudflare Access via un tunnel ?~~ *(hors périmètre)*
- ~~**Q23** Nom de domaine disponible ?~~ *(hors périmètre)*
