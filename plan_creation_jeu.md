# Horreur à Arkham — Plan de création du jeu

Rédigé le 16/11/2026. Ce plan remplace la partie « architecture » de `plan_mise_en_ligne.md` (qui prévoyait un client WebAssembly sans serveur). Les manques de données (M1 à M15) et les questions ouvertes sont repris et complétés dans `manques_et_questions.md`.

Cadre retenu :

- **Serveur** : axum + tokio, **sur un seul thread** (runtime « current_thread »).
- **Base de données** : PoloDB (`polodb_core`) embarquée dans le serveur, stockage sur disque dans un dossier. Pas de serveur de base de données séparé. *(Décision du 16/11/2026 : SurrealDB, retenue d'abord, est abandonnée : trop de contraintes — 22 threads d'appoint, licence BSL, compilation longue, alertes de sécurité.)*
- **Interface** : pages HTML rendues par le serveur, formulaires classiques, avec **htmx** pour rafraîchir des morceaux de page sans tout recharger. **Pas de client JavaScript lourd** : pas d'application dans le navigateur, le serveur garde toute la logique.
- **Jeu** : un humain joue un investigateur, le système joue les autres investigateurs (IA) et le Mythe. Les tours des IA sont joués **d'un bloc** ; l'humain les suit dans le journal. **Jeu de base seul**, usage privé.
- **Hébergement** : hors périmètre (environnement dédié de l'utilisateur). Le projet doit **fonctionner en local**.

---

## 1. Vue d'ensemble

Le programme est **un seul exécutable** qui contient tout : le moteur de règles, les IA, le serveur web et la base de données. On le lance, il ouvre un port, et on joue dans le navigateur.

```
navigateur  ──HTTP──▶  serveur axum (1 thread)
                          │
                          ├── pages HTML (gabarits)
                          ├── moteur de règles  ◀── données du jeu (fichiers)
                          ├── IA des investigateurs
                          └── PoloDB (dossier sur disque) : parties, journal des actions
```

Principe central : **le moteur ne décide jamais seul.** Il fait avancer la partie jusqu'au prochain point où quelqu'un doit choisir, et il pose une « décision à prendre ». Si elle revient à l'humain, le serveur affiche un formulaire. Si elle revient à une IA, le serveur appelle l'IA, applique sa réponse, et continue. Le moteur traite exactement de la même façon une réponse humaine et une réponse d'IA.

---

## 2. Découpage du code (espace de travail Cargo)

| Module | Rôle | Dépend de |
|---|---|---|
| `data` | Types des données du jeu (lieux, cartes, monstres, investigateurs, Grands Anciens) et leur chargement depuis les fichiers, avec contrôles de cohérence. | — |
| `engine` | Les règles. État de partie, décisions, application des réponses, événements. Déterministe, sans entrée-sortie, sans base de données. | `data` |
| `ai` | Les politiques qui répondent aux décisions à la place des investigateurs IA. | `engine` |
| `server` | axum : pages, formulaires, sessions, PoloDB, fichiers statiques (images d'`assets/`). | tous |
| `simulator` | Petit exécutable en ligne de commande : joue des parties IA seules, en masse, pour tester et calibrer. **Hors du serveur**, parce qu'une longue simulation bloquerait le thread unique. | `engine`, `ai` |

Règle de dépendance : `engine` ne connaît ni le web ni la base. On doit pouvoir le tester et le rejouer sans rien d'autre.

Langue (décision du 16/11/2026, détail dans `CLAUDE.md`) : le code est en anglais (noms de crates, identifiants, commentaires, commits, logs). Tout ce que le joueur lit est en français. Dans les fichiers de données RON, les clés et les noms de briques d'effet sont en anglais, les valeurs lues par le joueur (noms et textes des cartes, lieux…) en français.

---

## 3. Données du jeu

### 3.1 Forme

Les données vivent dans un dossier `data/` du dépôt, en fichiers texte (format RON), **jamais dans le code**. Un fichier par famille : plateau, investigateurs, Grands Anciens, monstres, cartes Lieu, cartes Portail, cartes Mythe, objets communs, objets uniques, sorts, compétences, alliés, cartes spéciales, marqueurs Portail. Chaque fichier porte un numéro de version.

Chaque fiche renvoie à son image dans `assets/` (par exemple la carte Mythe « Alignement planétaire » pointe sur `assets/cartes/mythe/alignement_planetaire.jpg`), pour affichage dans le navigateur.

### 3.2 Le plateau comme graphe

Le plateau est décrit par :

- la liste des **lieux** (26) avec leur quartier, stable ou instable, icônes de rencontre, capacité spéciale ;
- les **zones de rue** (9) ;
- les **liaisons** lieu ↔ rue et rue ↔ rue (déplacement des investigateurs) ;
- les **flèches** blanches et noires (déplacement des monstres) ;
- les **Autres Mondes** (8) avec leurs deux zones et leurs couleurs de rencontre ;
- les cases spéciales : Périphérie, Ciel, Perdus dans le Temps et l'Espace, Cellule de Prison ;
- pour l'affichage : la **position en pixels** de chaque lieu, rue et Autre Monde sur l'image du plateau.

### 3.3 Le langage des cartes

C'est le point le plus délicat. Le texte des cartes (rencontres de Lieu et de Portail, effets Mythe, objets, sorts, compétences, alliés) doit devenir quelque chose que le moteur sait exécuter.

Approche retenue : un **petit vocabulaire d'effets** écrit dans les fichiers de données, avec un nombre limité de briques : gagner ou perdre (Santé Mentale, Résistance, argent, Indices), faire un test de compétence avec un résultat par issue, proposer un choix, piocher une carte d'un paquet, déplacer l'investigateur, être retardé, faire apparaître un monstre ou un portail, fermer un portail, obtenir un statut (Béni, Maudit, Adjoint…), etc. Une carte est une suite de ces briques.

On tolère que quelques cartes aient un effet introuvable avec ce vocabulaire : elles sont marquées « effet spécial » et codées une par une dans le moteur, sous un nom explicite. L'objectif est que ce reste rare (moins de 10 % des cartes).

Ordre de travail : d'abord choisir le vocabulaire sur un **échantillon** (une vingtaine de cartes variées), construire le moteur dessus, puis transcrire le reste.

### 3.4 Contrôles automatiques

Au chargement et dans les tests : chaque lieu cité existe, chaque liaison est symétrique, chaque carte référencée existe, les paquets ont le bon nombre d'exemplaires, chaque image citée existe dans `assets/`, aucun lieu stable ne reçoit de portail par une carte Mythe, etc.

---

## 4. Le moteur de règles

### 4.1 État de partie

Un seul objet « État » qui contient tout : les investigateurs (position, curseurs, possessions, statuts), les monstres sur le plateau, les portails ouverts, les sceaux, les pions Indice, la Terreur, la piste de Destin, les paquets et défausses, le premier joueur, la phase en cours, le générateur aléatoire (graine et position). Il est sérialisable tel quel, pour la sauvegarde et le rejeu.

### 4.2 Décisions et réponses

Le moteur expose trois opérations :

1. **Avancer** : à partir de l'État, joue tout ce qui ne demande aucun choix, et s'arrête soit sur une **Décision** (qui doit choisir, quoi, quelles options sont légales), soit sur la fin de partie.
2. **Répondre** : applique une réponse à la Décision en cours. Le moteur **refuse** une réponse illégale (hors options proposées), qu'elle vienne d'un humain ou d'une IA.
3. **Événements** : chaque opération produit une liste d'événements lisibles (« Amanda se déplace à la Gare », « test de Volonté : 2 dés, 1 succès »), qui alimente le journal de partie.

Décisions prévues : ajustement des compétences, actions d'Entretien, mouvement (destination), choix dans une rencontre, dépenser des Indices sur un test, combattre ou fuir, choix des armes et sorts en mains, lancer un sort, achats, échanges d'équipement, fermer ou sceller, choix du premier joueur pour les arbitrages (placement des monstres, Indice à prendre), Bataille finale. Les lancers de dés ne sont **pas** des décisions : ils sortent du générateur aléatoire du moteur.

### 4.3 Déterminisme et rejeu

La partie est entièrement définie par la graine aléatoire, les données du jeu (version) et la liste ordonnée des réponses. Rejouer cette liste reproduit la même partie. Cela donne gratuitement :

- la **reprise** d'une partie sauvegardée ;
- l'**annulation** d'un coup (rejouer jusqu'à la réponse précédente) ;
- des tests reproductibles.

### 4.4 Règles non tranchées

Avant de coder une règle, vérifier `regles/base.md` §22 et les questions de `manques_et_questions.md`. Chaque règle tranchée est consignée dans `regles/decisions.md`, avec la date et la raison. R1 à R8 sont tranchées depuis le 16/11/2026.

---

## 5. Les investigateurs IA

### 5.1 Principe

Une IA est une fonction : **Décision + État visible → Réponse**. Elle ne voit que ce qu'un joueur verrait (pas le dessus des paquets, pas la graine). Elle répond aux mêmes décisions que l'humain.

### 5.2 Première version : règles de priorité

Une IA à base de règles simples, lisibles, dans cet ordre :

1. survivre (se soigner si Santé Mentale ou Résistance ≤ 2, éviter les monstres trop forts) ;
2. sceller un portail si elle a 5 Indices ou un Signe des Anciens ;
3. fermer un portail exploré ;
4. aller vers le portail ouvert le plus proche, ou ramasser des Indices ;
5. combattre un monstre si ses chances sont bonnes, sinon fuir ;
6. acheter une arme ou se soigner en passant.

Les choix de rencontre suivent une évaluation simple : gain attendu moins risque. L'ajustement des curseurs dépend de l'objectif du tour (Vitesse pour se déplacer, Vigueur pour combattre, Connaissance pour lancer un sort…).

### 5.3 Calibrage

Le `simulator` joue des milliers de parties avec des IA seules et mesure : taux de victoire par Grand Ancien, durée moyenne, nombre de morts d'investigateurs. On ajuste les priorités pour que les IA soient **utiles sans gagner à la place du joueur**.

### 5.4 Ce qui est remis à plus tard

Plusieurs niveaux d'IA, une stratégie d'équipe coordonnée, et la possibilité de donner des ordres aux IA. Les questions correspondantes sont dans `manques_et_questions.md`.

---

## 6. Le serveur web

### 6.1 Technique

- tokio en mode **current_thread** : tout le code du jeu tourne sur le thread principal. Note : PoloDB (qui repose sur RocksDB) garde 3 threads d'appoint pour les écritures disque, c'est inévitable et sans effet sur la logique du jeu. Les appels à PoloDB sont bloquants mais très courts (moins d'une milliseconde) : ils tournent directement sur le thread unique.
- axum pour les routes, un moteur de gabarits HTML (askama ou maud, à choisir), tower-http pour servir `assets/` et le CSS.
- **Aucun état de partie en mémoire** : à chaque requête, le serveur charge la partie depuis PoloDB, applique la réponse, fait jouer les IA jusqu'à la prochaine décision humaine, puis enregistre. C'est simple, robuste, et le thread unique ne pose aucun problème de verrou. Si un jour c'est trop lent, on ajoutera un cache.
- Formulaire → POST → redirection → GET (le rafraîchissement de la page ne rejoue jamais un coup). Avec htmx, le serveur renvoie à la place le **fragment** de page à remplacer (le plateau, la décision en cours, le journal) ; sans htmx, la page entière. Les mêmes gabarits servent aux deux.
- Après une réponse de l'humain, le serveur joue **tous les tours d'IA et la phase Mythe d'un bloc**, jusqu'à la prochaine décision humaine, puis renvoie la page avec le journal de ce qui s'est passé entre-temps.

### 6.2 Base de données PoloDB

Collections prévues (PoloDB range des documents dans des « collections », l'équivalent des tables) :

| Collection | Contenu |
|---|---|
| `partie` | identifiant, date, graine, version des données, investigateur humain, statut (en cours, gagnée, perdue), **État complet** (sérialisé) |
| `action` | partie, numéro d'ordre, décision posée, réponse donnée, auteur (humain ou IA), horodatage |
| `evenement` | partie, numéro d'ordre, texte du journal (pour l'affichage sans recalcul) |

Les données du jeu (cartes, plateau…) restent dans les fichiers : la base ne contient que ce qui change. Un numéro de version de l'État permet de migrer les parties sauvegardées quand le format évolue.

### 6.3 Pages

| Page | Contenu |
|---|---|
| Accueil | parties en cours et terminées, bouton « Nouvelle partie » |
| Nouvelle partie | choix de l'investigateur humain, nombre d'IA, Grand Ancien (hasard ou choix), graine facultative |
| Partie | **le plateau** (image d'`assets/plateau/` avec les lieux cliquables positionnés dessus, les pions dessinés par-dessus en HTML/CSS), la fiche de l'investigateur humain, l'état de la partie (Terreur, Destin, portails), la **décision en cours** sous forme de formulaire, le journal des derniers événements |
| Fiche | une fiche d'investigateur ou de Grand Ancien, un monstre, une carte, avec son image |
| Journal | tous les événements de la partie, bouton « Annuler le dernier coup » |
| Aide | résumé des règles (tiré de `regles/base.md`) |

Les cartes et jetons sont montrés par leur image d'`assets/`, avec le texte en dessous (utile pour la lisibilité et pour l'accessibilité).

### 6.4 JavaScript : htmx, et quelques scripts maison

Choix : **htmx**, une petite bibliothèque (un seul fichier, servi en local depuis le dépôt, pas de CDN) qui permet à un lien ou un formulaire de ne remplacer qu'un morceau de la page. Elle convient mieux que Vue ici parce que toute la logique reste côté serveur : il n'y a aucun état à maintenir dans le navigateur, donc rien à synchroniser. Vue reste en réserve si le plateau finit par demander une interaction trop riche (glisser-déposer, animations), ce qui n'est pas prévu.

Ce que htmx prendra en charge :

- envoyer la réponse à une décision et ne remplacer que le plateau, la fiche et le journal ;
- ouvrir le détail d'une carte ou d'un monstre dans un panneau, sans quitter la partie ;
- faire défiler le journal des tours d'IA sans recharger la page.

Quelques scripts maison, facultatifs, de quelques lignes : zoom et déplacement du plateau, surlignage des lieux atteignables au survol. Tout doit rester jouable sans JS, avec rechargement complet des pages.

### 6.5 Accès et sécurité

L'hébergement et la protection de l'accès sont **hors périmètre** : l'utilisateur dispose d'un environnement dédié. Le serveur :

- écoute par défaut sur l'adresse locale uniquement (`127.0.0.1`), port configurable ;
- lit sa configuration (port, chemin de la base, chemin des données) dans un fichier ou des variables d'environnement, pour s'installer ailleurs sans changement de code ;
- pose un jeton anti-falsification sur chaque formulaire (peu coûteux, et utile le jour où il est exposé) ;
- envoie `robots.txt` et un en-tête `noindex`.

Pas de page de connexion ni de table `utilisateur` dans la première version.

---

## 7. Étapes

| Jalon | Contenu | Critère de fin |
|---|---|---|
| **J0 — Socle** | Dépôt git, espace de travail Cargo, CI, vérification technique de la pile (PoloDB + tokio current_thread + axum compilent et tournent ensemble ; temps de compilation acceptable). Choix du moteur de gabarits. | Une page « bonjour » servie par axum, qui lit et écrit une ligne dans la base. |
| **J1 — Données** | Format des fichiers, graphe du plateau, investigateurs, Grands Anciens, monstres, marqueurs Portail. Vocabulaire des cartes choisi sur un échantillon. Contrôles de cohérence. | Toutes les données hors texte de cartes chargées et validées. |
| **J2 — Moteur (un investigateur)** | Tour complet : Entretien, Mouvement, Rencontres, Mythe, combat, tests, statuts, portails, Terreur, réveil, Bataille finale. Cartes limitées à l'échantillon. Pilotable depuis le `simulator` en mode texte. | Une partie entière se joue en ligne de commande, avec rejeu exact depuis la graine. |
| **J3 — Cartes** | Transcription de toutes les cartes du jeu de base dans le vocabulaire (Lieu, Portail, Mythe, objets, sorts, compétences, alliés, spéciales). Effets spéciaux codés. | 100 % des cartes chargées ; chaque carte jouée au moins une fois en simulation. |
| **J4 — IA** | Politique de priorité, simulations massives, calibrage. | Des milliers de parties IA seules sans plantage ; statistiques publiées dans le dépôt. |
| **J5 — Serveur** | Pages, formulaires, htmx, plateau cliquable, sauvegarde, reprise, annulation, journal. | Une partie complète jouée dans le navigateur en local, humain + IA, avec reprise après redémarrage du serveur. |
| **J6 — Livraison locale** | Configuration par fichier, mode d'emploi d'installation (compiler, lancer, où est la base), sauvegarde du fichier de base. | Le projet se lance sur une machine neuve en suivant le mode d'emploi. |

**Avancement** : J0 terminé le 16/11/2026 (résultats des vérifications T1 à T6 dans `manques_et_questions.md` §2). J1 terminé le 16/11/2026 : FAQ intégrée aux règles (`regles/base.md` §23) ; plateau, marqueurs Portail, monstres, investigateurs et Grands Anciens saisis dans `data/`, chargés et contrôlés par le module `data` ; vocabulaire des cartes choisi sur un échantillon de 39 cartes (`vocabulaire_cartes.md`, fichiers `data/*_cards.ron`), avec 4 cartes en effet spécial.

Les jalons J1 et J3 dépendent de la transcription des cartes, que je fais à partir des images (`manques_et_questions.md` §1, Q7). C'est le vrai chemin critique. Les nombres d'exemplaires (Q8) sont saisis de mémoire et marqués « à vérifier » en attendant tes recherches.

---

## 8. Tests

1. **Moteur** : tests unitaires par phase et par règle (chaque ligne de `regles/base.md` §21.2 a son test), tests de propriétés (jamais de valeur négative, jamais deux investigateurs dans un état impossible, limite de monstres respectée), rejeu exact depuis graine + réponses.
2. **Données** : contrôles de cohérence lancés en test ; nombre d'exemplaires de chaque paquet ; chaque image existe.
3. **Cartes** : pour chaque carte, un test « la carte s'exécute sans erreur dans un état type », généré automatiquement depuis les données.
4. **IA** : simulations en masse (CI : quelques centaines de parties ; en local : des milliers).
5. **Serveur** : tests des routes avec une base PoloDB temporaire ; une réponse illégale envoyée par formulaire est refusée ; le rejeu d'une partie sauvegardée redonne le même État.
6. **Manuel** : quelques parties complètes dans le navigateur avant la mise en ligne.

---

## 9. Intégration continue et livraison

- À chaque modification : rustfmt, clippy (avertissements interdits), tests, cargo-audit, cargo-deny.
- Livrable : un exécutable + le dossier `data/` + le dossier `assets/` + le fichier htmx + le dossier de base PoloDB, créé au premier lancement. L'installation sur la machine de l'utilisateur est hors périmètre.
- Sauvegarde : copie du dossier de base, serveur arrêté (PoloDB embarquée : un seul processus écrit dedans).
- Mise à jour : arrêt, remplacement de l'exécutable, redémarrage ; les parties sauvegardées sont migrées au premier chargement si le format a changé.
- Retour en arrière : garder l'exécutable précédent et une copie de la base d'avant la mise à jour.

---

## 10. Liste de vérification avant la livraison

- [ ] Toutes les données du jeu de base chargées et validées, cartes comprises
- [ ] Règles tranchées consignées dans `regles/decisions.md`
- [ ] Simulations IA sans plantage
- [ ] Partie complète jouée dans le navigateur en local, avec reprise et annulation
- [ ] Jouable sans JavaScript (rechargement complet) et avec htmx
- [ ] Sauvegarde de la base testée (copie puis restauration)
- [ ] Mode d'emploi d'installation locale écrit et suivi sur une machine neuve
- [ ] Version étiquetée et journal des modifications à jour
