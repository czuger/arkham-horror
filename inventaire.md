# Inventaire des sources — Horreur à Arkham (2e édition)

Source analysée : `../../python/perso/scraper/pages/wiki.horreuraarkham.fr/`
(= `/Users/ced/python/perso/scraper/pages/wiki.horreuraarkham.fr/`, aspiration du wiki francophone *wiki.horreuraarkham.fr*, MediaWiki, figé en 2017).

Volume (mise à jour du 2026-11-13, après réaspiration) : **832 fichiers, 395 Mo** → **766 pages HTML** + **66 PDF**.
- 1re aspiration : 98 fichiers (67 HTML + 31 PDF) → décrits aux §2 à §4 (inchangés).
- Ajouts : **699 HTML + 35 PDF** → décrits aux §5 et §6.
- Mise à jour du 2026-11-16 : l'aspiration contient désormais **953 HTML + 66 PDF + 3 116 images** (3 055 JPEG, 61 PNG ; 801 Mo). Sur les 3 185 images affichées par les pages, 100 manquent encore. Les images du jeu de base sont rangées dans `assets/` (voir `assets/inventaire_assets.md`). Les nouvelles pages HTML ne sont pas encore décrites ici.

Rien n'a été développé.

---

## 1. Synthèse

### 1.1 Trois jeux mélangés
| Jeu | Repérage | Pertinent ? |
|---|---|---|
| **Horreur à Arkham 2e éd.** (Arkham Horror 2nd ed., FFG/Edge, base + 8 extensions) | la grande majorité des pages | **oui, cible** |
| *Les Demeures de l'Épouvante* (Mansions of Madness 1re éd.) | suffixe `_mom`/`_MoM`, + Demeures, Expériences Interdites, PoD, scénarios (§3.1, §6.19), guides PDF | non |
| *Le Signe des Anciens* (Elder Sign) | suffixe `_es`, + pages de règles « Aventure / Entrée / tâches » (§3.2, §6.20) | non |

### 1.2 Ce qu'on a maintenant
- **Règles complètes en texte, en français, pour le jeu de base ET les 8 extensions** (nouveaux PDF `Notice_HaA_*`, §5.1) : base, Dunwich (enfin en VF), Kingsport, Innsmouth, Miskatonic, Pharaon Noir (éd. révisée), Roi en Jaune, Chèvre Noire, Rôdeur. Les anciens scans deviennent superflus.
- **FAQ / erratas en texte** : `FAQ_Officiel_032017_V2.pdf` + 9 pages `Erratas_-_<extension>.html` + `Erratas.html` + `FAQ_VO.html` (entrées anglaises).
- **Règles détaillées découpées en pages wiki** (§6.1–6.4) : préparation (sommaire seulement), tour de jeu, chaque phase, tests (Horreur, Combat, Évasion, Sort), combat, mains, bonus d'armes, statuts (Retardé, Fou, Inconscient, Dévoré, Perdu dans le Temps et l'Espace…), mouvements des monstres par type, limite de monstres, Périphérie, Terreur, portails, sceller, réveil, Bataille finale, victoire et points.
- **Données tabulaires en texte** (§6.6) — c'est la grosse nouveauté pour le développement :
  - `Liste_détaillée_des_Investigateurs.html` : **48 investigateurs** (SAN, RES, $, indices, nombre de cartes de départ, compétences, concentration, maison, résumé du pouvoir) ;
  - `Liste_détaillée_des_Monstres.html` : **~93 monstres** (nombre d'exemplaires, symbole dimensionnel, mouvement, conscience, horreur, combat, force, capacités) ;
  - `Liste_Des_Sorts.html` : sorts (coût, modificateur, mains, phase) ;
  - `Liste_Des_Alliés_Par_Extension.html` : alliés (lieu, condition, bonus).
- **Une page par lieu d'Arkham** (stable/instable, capacité spéciale, PNJ) et par Autre Monde (couleurs de rencontre) ; **une page par objet commun** (type d'arme, mains, bonus, prix) — partiellement.
- **Fiches et cartes corrigées en PDF image** (`*-ToutesLesCartes*.pdf`, `*-ToutesLesFiches.pdf`, §5.2) : planches des cartes ayant reçu un errata, avec texte complet lisible sur l'image (pas extractible : OCR nécessaire).

### 1.3 Ce qu'on n'a toujours PAS
- **Le texte de la plupart des cartes** : les pages de cartes (quartiers, portails, mythe, objets uniques, sorts, alliés, compétences) restent des **galeries d'images** (`/images/...jpg`). *Depuis le 16/11/2026, les images sont aspirées : le texte est lisible sur l'image, mais pas extractible (OCR nécessaire).* Les 24 pages de quartier listent les codes des cartes Lieu (ex. `CLAFH01`…`CLAFH35`) mais **pas leur texte de rencontre**. Idem pour les ~284 cartes Portail et ~294 cartes Mythe.
- **Feuilles des Grands Anciens** en texte (seulement pouvoirs résumés/erratas sur les pages GA ; quelques feuilles corrigées en image dans `Base-ToutesLesFiches.pdf`).
- **Le plateau** (graphe lieux/rues/connexions) : les pages donnent la liste des lieux par quartier, pas les liaisons ni les flèches de mouvement des monstres.
- **167 pages internes encore non aspirées** (surtout une page par monstre : `Cultiste`, `Goule`, `Shoggoth`…, et quelques redirections : `Armes`, `Monstre`, `Tests_De_Combat`, `Séparation_Des_Paquets`…) + **1 362 liens rouges** (pages inexistantes sur le wiki, ex. les sous-étapes de `Préparation_Du_Jeu`).
- ~~Les images elles-mêmes~~ : aspirées le 16/11/2026, sauf 100 (dont 85 d'Arkham, surtout des icônes). Voir la section « Manquants » de `assets/inventaire_assets.md`.

### 1.4 Particularités des nouvelles pages
- **55 doublons** : même page sous plusieurs noms (redirections MediaWiki, variantes de casse/accents, suffixes `~xxxxxxxx`). Ils sont signalés à la fin de chaque groupe du §6.
- **166 « fiches vides »** : pages de carte qui ne contiennent que l'image (non téléchargée) et « Erratas & Clarifications : Aucun ».
- Quelques **erreurs du wiki** (contenu copié sur une autre page) : `Arme.html` (= `Conscience`), `Bombe_A_Retardement.html` (= `Athame`), `Brasier_Des_Ames.html` (= `Pistolet_De_Détresse`), `Insaisissable.html` (= `Irrésistible`).
- 9 nouvelles pages reprennent le contenu exact d'une page déjà inventoriée au §2 (`Portail` = `Portails`, `Investigateur` = `Investigateurs`, `Grand_Ancien` = `Grands_Anciens`, `Carte_Mythe` = `Cartes_Mythe`…).

Légende des colonnes : **mots** = mots de texte dans le contenu de la page ; **img** = images de contenu (cartes, icônes d'extension incluses).

---

> **§2 à §4 : fichiers de la 1re aspiration** (description inchangée). Les mentions « page non aspirée » y renvoient à l'état initial : la plupart de ces pages sont maintenant présentes, voir §6.

## 2. Pages HTML — Horreur à Arkham 2e édition (1re aspiration, 67 fichiers)

### 2.1 Accueil et présentation

| Fichier | mots / img | Contenu |
|---|---|---|
| `Accueil.html` | 554 / 15 | Page d'accueil (news 2017 : forum fermé). Portail de navigation à 3 colonnes : **Horreur à Arkham**, **Demeures de l'Épouvante**, **Signe des Anciens** (chacun « Jeu & Matériel » + « Créations de Fans »), + liens externes. Liens vers plusieurs PDF Demeures (portes, jetons, tuiles créatures, révision complète, errata Tablette d'Argent). |
| `Le_Jeu.html` | 647 / 21 | Historique (version 1987 de Chaosium → 2e éd. FFG 2005), principe et but du jeu, jeu de base, liste des **8 extensions** avec photos de boîtes : Malédiction du Pharaon Noir (+ éd. révisée), Horreur de Dunwich, Roi en Jaune, Horreur de Kingsport, Chèvre Noire des Bois, Horreur d'Innsmouth, Rôdeur Devant le Seuil, Horreur de Miskatonic. Section « extension obsolète » (Pharaon Noir 1re version). |
| `Critiques_&_Tests.html` | 988 / 0 | Citations de critiques francophones du jeu (avis, notes), liens vers les sites sources. Aucune valeur pour le développement. |
| `Conseils_Pour_Débuter.html` | 1 136 / 0 | Conseils de règles (généralités, phase de Mythe) et de stratégie (composer une équipe variée, surveiller le Grand Ancien, avis subjectifs). |

### 2.2 Règles, FAQ, variantes

| Fichier | mots / img | Contenu |
|---|---|---|
| `Règles.html` | 469 / 0 | Règle d'or (« dans le doute, la solution la plus pénalisante pour l'investigateur »). Renvoie vers Préparation / Tour / Fin du jeu (non aspirées). **Liste des livrets de règles PDF officiels** par extension, avec les options de jeu de chacune (héraut, modes Exposition Permanente / Visite de l'Exposition, Représentation Permanente / Itinérante, Gardiens, Bataille Épique, Complot, niveaux de difficulté, Histoires Personnelles, Relations). |
| `Glossaire.html` | 452 / 0 | Index alphabétique A→Z des termes de jeu (Abandon, Acte, Activité, Aquatique, Arrêté, Bataille Finale, Bicolore, Blessure…). **Liste de termes seulement**, les définitions sont sur des pages non aspirées. Utile comme vocabulaire de référence. |
| `Foire_Aux_Questions.html` | 6 196 / 2 | FAQ du wiki en **texte** : questions puis réponses, par thème — Divers, Grands Anciens, Lieux et Rencontres, Limite de Monstres et Périphérie, Monstres, Cartes Mythe, Portails et Autres Mondes, Sorts. Traduction partielle de la FAQ VO. Précieux pour les cas limites de règles. |
| `Erratas_+_Fichiers_Eons.html` | 288 / 0 | Note : clarifications intégrées aux pages de cartes. Point de traduction « Fight/Combat » (Force vs Combat). 1 petit tableau. Mention des fichiers Strange Eons (éditeur de cartes fan). |
| `Moduler_La_Difficulté.html` | 5 460 / 0 | **~25 variantes de règles** en texte, chacune marquée (-) plus facile, (+) plus dur, (*) neutre : Ouverture/Fermeture de faille, Violente irruption de portail, Terreur ! Malheur !, Gambler's Variant, StormKnight Variant, Failles à Kingsport, Indices sur le Grand Ancien, Arkham-les-Bains, Dunwich-les-Bains, Eaux sombres, Monstre tapi, Portail vers l'inconnu, À chacun sa tâche, Deux monstres sur le premier portail, Choisir les investigateurs, Perte de mémoire, Bloquer un investigateur dans un Autre Monde, Un round de combat/évasion… |
| `Aides_de_Jeu.html` | 100 / 0 | Liens vers la FAQ officielle v2 (PDF) et l'aide de jeu illustrée v3 de Damn3d (PDF), + une aide de phases (lien). |

### 2.3 Plateau : villes, lieux, autres mondes, portails

| Fichier | mots / img | Contenu |
|---|---|---|
| `Plateaux_de_Jeu.html` | 97 / 4 | Les 4 plateaux (Arkham, Dunwich, Kingsport, Innsmouth) en vignettes. Règle : tout effet « Arkham » s'applique aussi aux autres villes. |
| `Quartier.html` | 368 / 23 | Liste des quartiers par ville avec dos de cartes Lieu (mêmes images que `Cartes_Lieu`). Arkham : Nord, Centre-Ville, Est, Quartier Marchand, Rivière, Université Miskatonic, French Hill, Résidentiel, Sud. Dunwich : Bois Noirs, Terrains Communaux, Lande Dévastée. Kingsport : Central Hill, Côte Sud, Port, Pic de Kingsport. Innsmouth : Quartier de la Fabrique, Église Verte, Côte d'Innsmouth. Les lieux de chaque quartier sont des liens (pages non aspirées). |
| `Cartes_Lieu.html` | 399 / 39 | Descriptif des cartes Lieu (rencontres par quartier) et nombre de quartiers par ville (Arkham 9, Dunwich 3, Kingsport 4, Innsmouth 3). Seulement les **dos** de cartes : **aucun texte de rencontre**. |
| `Zones_Spéciales.html` | 73 / 0 | Liste des zones spéciales : Arkham (Cellule de Prison, Limites de la Ville, Périphérie, Ciel, Perdu dans le Temps et l'Espace), Dunwich (Vortex, Colline de la Sentinelle), Kingsport (aucune), Innsmouth (Vortex, Bus de Joe Sargent, Sawbone Alley). Liens seuls. |
| `Échelles.html` | 83 / 0 | Les pistes de score par plateau : Échelle de Terreur (Arkham), Horreur de Dunwich, Piste des Failles (Kingsport), Échelle de Soulèvement profond (Innsmouth). Liens seuls. |
| `Autres_Mondes.html` | 151 / 4 | Description générique des Autres Mondes (grands cercles en bord de plateau, 2 zones, accès par Portail). Pas de liste détaillée. |
| `Portails.html` | 1 469 / 63 | Citation d'ambiance, **statistiques des portails** (par Autre Monde, modificateurs de fermeture), galerie des marqueurs Portail (base, Dunwich, Kingsport ; images), **fréquence d'apparition** des portails, section Rôdeur devant le Seuil (12 portails spéciaux + 6 portails scénario). Partie la plus exploitable en texte pour cette thématique. |
| `Cartes_Portail.html` | 303 / 1 | Fonctionnement des cartes Portail (rencontres dans les Autres Mondes, paquet unique, symboles de couleur, cartes bicolores). Galerie = seulement le dos. |

### 2.4 Personnages, ennemis, entités

| Fichier | mots / img | Contenu |
|---|---|---|
| `Investigateurs.html` | 754 / 84 | Description annotée de la **fiche d'investigateur** (nom, SAN, RES, curseurs de compétences, focus, possessions fixes/aléatoires, capacité spéciale, lieu de départ). Galerie des **48 investigateurs officiels** : base (16) Amanda Sharpe, Ashcan Pete, Bob Jenkins, Carolyn Fern, Darrell Simmons, Dexter Drake, Gloria Goldberg, Harvey Walters, Jenny Barnes, Joe Diamond, Kate Winthrop, Mandy Thompson, Michael McGlen, Monterey Jack, Sœur Mary, Vincent Lee ; Dunwich (8) Diana Stanley, Jacqueline Fine, Jim Culver, Leo Anderson, Marie Lambeau, Mark Harrigan, Rita Young, Wilson Richards ; Kingsport (8) Charlie Kane, Daisy Walker, Lily Chen, Lola Hayes, Luke Robinson, Rex Murphy, Tony Morgan, Wendy Adams ; Innsmouth (16) Agnes Baker, Akachi Onyele, Finn Edwards, George Barnaby, Hank Samson, Minh Thi Phan, Norman Withers, Patrice Hathaway, Roland Banks, « Skids » O'Toole, Silas Marsh, Tommy Muldoon, Trish Scarborough, Ursula Downs, William Yorick, Zoey Samaras. + listes, errata. **Stats/capacités = dans les images uniquement.** |
| `Grands_Anciens.html` | 871 / 45 | Description de la **feuille de Grand Ancien** (piste de destin, pouvoir de vénération, défense, attaque). Galerie : base (8) Azathoth, Cthulhu, Hastur, Ithaqua, Nyarlathotep, Shub-Niggurath, Yig, Yog-Sothoth ; Dunwich (4) Abhoth, Glaaki, Shudde M'ell, Tsathoggua ; Kingsport (4) Atlach-Nacha, Eihort, Y'Golonac, Yibb-Tstll ; Innsmouth (8) Bokrug, Chaugnar Faugn, Cthugha, Ghatanothoa, Nyogtha, Quachil Uttaus, Rhan-Tegoth, Zhar ; promos Arkham Nights (Cthulhu 2010, Daoloth 2010, Hastur 2011). + errata, variante de Bataille Finale, conseils pour choisir un GA. |
| `Hérauts.html` | 116 / 19 | Principe des hérauts (entité qui prépare la venue du GA, ajoutée à la mise en place). 10 hérauts officiels : Horreur de Dunwich, Pharaon Noir, Roi en Jaune, Ghroth, Tulzscha, Chèvre Noire, Dagon, Hydra, Rôdeur devant le Seuil, + 1 planche. Images seules. |
| `Gardiens.html` | 48 / 6 | Gardiens (entités alliées, variante Kingsport) : Bastet, Hypnos, Nodens + leurs cartes associées (Bien-aimés de Bastet, Visions d'Hypnos, Bénédiction de Nodens). |
| `Monstres.html` | 790 / 2 | **Description des marqueurs Monstre** : face Mouvement (symbole dimensionnel, couleur de bordure = type de déplacement : normal, rapide, immobile, volant, traqueur, aquatique, spécial), face Combat (modificateur de discrétion, d'horreur, de combat, dégâts, endurance, capacités : physique/magique résistant ou immunisé, cauchemardesque, écrasant, embuscade, sans fin). Dangerosité, monstres particuliers, errata. **Pas de liste des monstres.** |

### 2.5 Cartes (galeries)

| Fichier | mots / img | Contenu |
|---|---|---|
| `Objets_Communs.html` | 453 / 100 | Descriptif (où les obtenir : Magasin, Gare, Boutique de souvenirs…). Galerie par extension : base 44, Dunwich 15, Pharaon Noir, Roi en Jaune 19, Kingsport 14, Chèvre Noire 11, Innsmouth, Rôdeur 4. ~53 noms (Automatique .45, Couteau, Croix, Derringer, Dynamite, Fusil, Hache, Lanterne…). |
| `Objets_Uniques.html` | 567 / 144 | Idem pour objets uniques : base 39, Dunwich 25, Roi en Jaune 22, Kingsport 12, Chèvre Noire 11, Rôdeur 4… ~78 noms (Boîte à portail, Clé d'argent, Couteau enchanté, Eau bénite, Signe des Anciens…). + errata. |
| `Sorts.html` | 571 / 96 | Sorts par extension : base 40, Pharaon Noir 21, Dunwich 21, Roi en Jaune 15, Kingsport 11, Chèvre Noire 4, Rôdeur 12, Miskatonic. ~42 noms (Atrophie, Brumes, Enchanter une arme, Flétrissement, Lier un monstre, Signe rouge, Soin, Trouver un portail, Voix de Ra…). |
| `Compétences.html` | 1 096 / 49 | Descriptif + cartes Compétence : base 20 (Vitesse, Discrétion, Combat, Volonté, Savoir, Chance, Bravoure, Tireur d'élite…), Dunwich 11, Kingsport 12, Miskatonic 7, etc. Errata. |
| `Alliés.html` | 510 / 73 | Alliés : base 11 (Anna Kaslow, Duke, Eric Colt, John Legrasse, Pr Armitage, Pickman, Ruby Standish, Ryan Dean, Sir William Brinton, Thomas Malone, Tom « Mountain » Murphy), Pharaon Noir 7, Dunwich 5, Kingsport 11. Errata. |
| `Cartes_Mythe.html` | 1 392 / 409 | Anatomie d'une carte Mythe (portail, indice, mouvement des monstres, type : gros titre / environnement / rumeur), errata. **Galerie de ~294 cartes** : base 67, Pharaon Noir 18, Dunwich 36, Roi en Jaune 27, Kingsport 22, Chèvre Noire 23, Innsmouth 36, Rôdeur 22, Miskatonic 43. ~224 noms uniques (Alignement Planétaire, Brouillard Maléfique…). Texte des effets = images. |
| `Spéciales.html` | 176 / 96 | Cartes spéciales (max 1 exemplaire par investigateur) : Abonnement Ferroviaire, Adjoint du Shérif, Bénédiction, Malédiction, Prêt Bancaire, Blessure, Folie, Corruption, Malheur, Membre du Culte, Membre de la Loge d'Argent, Bénéfices / Désavantages, Bateau Blanc, Effets Magiques… (~41). |
| `Objets_De_L'Exposition.html` | 339 / 48 | Objets de l'Exposition (extension Pharaon Noir) : 17 objets + 5 masques + 5 Miskatonic (Balance de Thoth, Cloche de Ra, Livre d'Anubis…). |
| `Cartes_Spéciales.html` | 22 / 9 | Simple menu vers d'autres types de cartes : Horreur de Dunwich, Acte, Bataille Épique, Complot, Difficulté, Engeance d'Innsmouth, Poussière, Dette (pages non aspirées). |
| `Marqueurs,_Jetons_&_Pions.html` | 71 / 33 | Liste illustrée des pions : Activité, Bastet, Coulissant, Couvée, Décombres, Destin, Dollars, Exploré, Faille, Horreur de Dunwich, Indice, Premier Joueur, Lieu Fermé, Signe des Anciens, Terreur, Progression de Faille, Résistance, Santé Mentale, Malheur, Zone Aquatique, Pouvoir. |

### 2.6 Créations de fans (non officielles, Horreur à Arkham)

| Fichier | mots / img | Contenu |
|---|---|---|
| `La_Page_des_Créations.html` | 590 / 27 | Hub des créations fan : personnages, objets/monstres, scénarios, extensions fan, variante « Arkham avec un Gardien » (PDF Gardien1/2), « Phase Finale », jeu dérivé **Enquêtes à Arkham** (règles + jetons + scénarios L'Appel du Vide, Cthulhu l'Effroyable Rêveur de R'lyeh). |
| `Investigateurs_Non-Officiels.html` | 360 / 41 | ~40 investigateurs fan (Andy Taylor, Ann Sanders, Eliot Ness, Herbert West…), liens vers fichiers .eon (Strange Eons). |
| `Grands_Anciens_Non-Officiels.html` | 223 / 48 | ~48 Grands Anciens fan (Angra Mainyu, Anubis, Aphoom-Zhah, Atlach-Nacha…), fichiers .eon. |
| `Hérauts_Non-Officiels.html` | 1 042 / 47 | ~39 hérauts fan classés par extension, avec description de leur mécanique en texte. |
| `Gardiens_Non-Officiels.html` | 282 / 6 | Propositions de modification des gardiens officiels (Bastet jugée sans effet…) + gardiens fan (Hermès, Serviteur de Nodens, Sherlock Holmes, Porte d'Argent). |
| `Monstres_Non-Officiels.html` | 183 / 22 | ~22 monstres fan (Aîné de Yith, Bête Lunaire…), fichiers .eon. |
| `Objets_Communs_Non-Officiels.html` | 23 / 10 | 10 objets communs fan (Tronçonneuse, Pied de biche, Morphine…). |
| `Objets_Uniques_Non-Officiels.html` | 37 / 6 | 6 objets uniques fan (Calice, Bio-armure Mi-Go…). |
| `Sorts_Non-Officiels.html` | 29 / 8 | 8 sorts fan (Contrôle Météorologique, Forme Animale…). |
| `Scenarios_Non_Officiels.html` | 605 / 20 | Scénarios fan et de ligues « officielles » (Ligues américaines I-III, ligue espagnole), séries (Série Noire, Dans l'Abîme du Temps, Casier #21). Liens de téléchargement externes. |

---

## 3. Pages HTML — hors sujet (1re aspiration)

### 3.1 Les Demeures de l'Épouvante (Mansions of Madness)
| Fichier | Contenu |
|---|---|
| `Demeures_de_l'Epouvante.html` | Les 5 scénarios de la boîte de base, liens vers guides PDF. |
| `Expériences_Interdites.html` | Extension : 3 scénarios (Retour du Réanimateur, Perdu dans l'Espace et le Temps, La Matière Jaune), guides PDF. |
| `PoD.html` | Scénarios imprimés à la demande (Saison de la Sorcière, Tablette d'Argent, Jusqu'à ce que la mort nous sépare). |
| `Le_Compagnon_Numérique.html` | Application fan (Windows/Flash) pour jouer sans Gardien. |
| `Regles_mom.html` | Conseils d'explication des règles, liens livrets, variantes. |
| `Investigateurs_mom.html` | Fiche + 12 investigateurs (8 base, 4 EI) avec cartes équipement/physique/mental ; 14 tableaux. |
| `Monstres_mom.html` | 24 images de monstres (Maniaque, Cultiste, Shoggoth…). |
| `Sorts_mom.html` | 6 sorts avec variantes A/B. |
| `Alchimie_mom.html` | 13 cartes Alchimie (traumas). |
| `Pions_mom.html` | Pions personnages, gardien, terrains. |
| `Plateau_mom.html` | 42 tuiles de plateau (1A…15B, EI-1A…EI-6B). |
| `Cartes_du_Gardien.html` | Types de cartes du Gardien (Action, Mythe, Trauma). |
| `Cartes_du_Plateau.html` | Exploration, Obstacle, Verrou (dos). |
| `Combats.html` | 63 cartes Combat (Humanoïde, Bête, Créature étrange). |
| `Enigmes.html` | Tuiles d'énigmes (Connexions, Verrous, Runes). |
| `Tuiles.html` | Tuiles fan supplémentaires (Ordre Ésotérique de Dagon, Mines de Yuggoth…). |
| `Scenarios_de_Fans.html` | Scénarios fan francophones pour les Demeures. |

### 3.2 Le Signe des Anciens (Elder Sign)
| Fichier | Contenu |
|---|---|
| `Le_Jeu_es.html` | Présentation du jeu et extensions. |
| `Règles_es.html` | Liens livrets de règles (externes, non récupérés). |
| `Glossaire_es.html` | **Copie identique** du glossaire d'Horreur à Arkham. |
| `Erratas_es.html` | FAQ Signe des Anciens (~1 900 mots : GA, dés, destin, investigateurs, musée…). |
| `Conseils_Pour_Débuter_es.html` | Conseils de règles et de stratégie. |
| `Critiques_&_Tests_es.html` | Critiques. |
| `Aides_de_Jeu_es.html` | Liens aides de jeu (externes). |

---

## 4. PDF (1re aspiration, 31 fichiers)

Nom réel = `images__X__XY__<nom>.pdf` (préfixe = chemin MediaWiki). « Texte » = texte extractible (sinon scan/image).

### 4.1 Horreur à Arkham — officiel
| Fichier | Pages | Texte | Contenu | Lié depuis |
|---|---|---|---|---|
| `images__b__b9__Arkham.pdf` | 24 | oui (107 k car.) | **Livret de règles du jeu de base**, FR, édition 2011. Source principale des règles. | Règles |
| `images__a__a4__FAQ_Officiel_032017_V2.pdf` | 45 | oui (206 k car.) | **FAQ officielle v2.0 (24/03/2017)** Edge : FAQ, errata, clarifications pour base (éd. révisée) et toutes les extensions. | Aides_de_Jeu |
| `images__c__c0__Roi_en_Jaune.pdf` | 2 | oui (14 k car.) | Règles de l'extension **Le Roi en Jaune** (FR). | Règles |
| `images__6__6b__Dunwinch_VO.pdf` | 12 | oui (55 k car.) | Règles de l'extension **Dunwich Horror**, **en anglais**. → VF en texte : `Notice_HaA_Dunwich.pdf` (§5.1). | Règles |
| `images__a__a5__Pharaon_Noir.pdf` | 2 | non (scan) | Règles de l'extension **La Malédiction du Pharaon Noir** (FR) : matériel, Objets de l'Exposition, Banni du Quartier, Bénéfices/Désavantages, Portails bicolores, modes de jeu. | Règles |
| `images__5__5a__Chèvre_Noire.pdf` | 2 | non (scan) | Règles de l'extension **La Chèvre Noire des Bois** (FR) : Membres du Culte, Corruption, Difficulté, préparation. → version texte : `Notice_HaA_LCNDB.pdf`. | Règles |
| `images__2__28__Rodeur_Seuil.pdf` | 2 | non (scan) | Règles de l'extension **Le Rôdeur devant le Seuil** (FR) : Relations, Pactes Maléfiques, Dette, nouveaux portails, pions Pouvoir. → version texte : `Notice_HaA_LRDLS.pdf`. | Règles |
| `images__c__c2__Kingsport.pdf` | 16 | non (scan) | Livret de règles de l'extension **L'Horreur de Kingsport** (FR, Edge). → version texte : `Notice_HaA_Kingsport.pdf`. | Règles |
| `images__d__dc__Innsmouth.pdf` | 16 | non (scan) | Livret de règles de l'extension **L'Horreur d'Innsmouth** (FR). → version texte : `Notice_HaA_Innsmouth.pdf`. | Règles |

### 4.2 Horreur à Arkham — aides et créations fan
| Fichier | Pages | Texte | Contenu | Lié depuis |
|---|---|---|---|---|
| `images__f__fa__Aide_jeu_v3_Damn3d.pdf` | 13 | oui (31 k car.) | **Aide de jeu illustrée v3** (Damn3d) : mise en place et déroulement des phases (Entretien, Mouvement, Rencontres, Mythe) intégrant toutes les extensions. Très utile comme résumé algorithmique du tour. | Aides_de_Jeu |
| `images__6__66__Enquete_a_Arkham.regles.pdf` | 9 | oui (20 k car.) | Règles de **Enquêtes à Arkham** (variante/jeu dérivé fan, 2011) : enquêtes scénarisées liées au Grand Ancien. | La_Page_des_Créations |
| `images__c__cc__Enquete_a_Arkham.jetons.pdf` | 8 | non | Planches de jetons pour Enquêtes à Arkham (pistes A–E numérotées 1–5). | La_Page_des_Créations |
| `images__3__30__AppelduVide.pdf` | 37 | oui (101 k car.) | Scénario Enquêtes à Arkham **« L'Appel du Vide »** (amikezor, v3 2011) : dossier d'enquête. | La_Page_des_Créations |
| `images__0__0c__Rlyeh.pdf` | 35 | oui (106 k car.) | Scénario Enquêtes à Arkham **« Cthulhu, l'effroyable rêveur de R'lyeh »** (Nephilim, VF 1.2). | La_Page_des_Créations |
| `images__9__9d__Objets.pdf` | 2 | non | Cartes objets de scénario (Nectar des Âmes, Élixir du Dr Müller, Clef de la Maison…) pour « L'Appel du Vide ». | La_Page_des_Créations |
| `images__6__6e__Gardien1.pdf` | 1 | oui | Variante fan **« Arkham avec un Gardien »** : cartes Action 1 (Portail courant, Invoquer monstre…). | La_Page_des_Créations |
| `images__a__a4__Gardien2.pdf` | 1 | oui | Même variante : cartes Action 2 (Sacrifice, Pickpocket…). | La_Page_des_Créations |

### 4.3 Les Demeures de l'Épouvante — hors sujet
| Fichier | Pages | Texte | Contenu |
|---|---|---|---|
| `images__8__87__MAD_Rulebook_FR_low.pdf` | 27 | oui | Livret de règles Demeures de l'Épouvante (FR). |
| `images__0__03__MAD01_Invest_Rulebook_FR_low.pdf` | 13 | oui | Guide des investigateurs Demeures (FR). |
| `images__e__ea__Guide_du_Gardien.pdf` | 16 | oui | Guide du Gardien Demeures (5 histoires). |
| `images__b__b5__EI_Regles.pdf` | 9 | oui | Règles Expériences Interdites. |
| `images__1__11__EI_Guide_du_Gardien.pdf` | 11 | oui | Guide du Gardien Expériences Interdites. |
| `images__c__c9__EI_Guide_des_Investigateurs.pdf` | 7 | partiel | Guide des investigateurs Expériences Interdites. |
| `images__9__91__DDE_-_Revision_Complete.1.2.pdf` | 16 | oui | Révision complète (bleached_lizard) des scénarios de base. |
| `images__d__d4__Errata_Tablette_Argent.pdf` | 3 | illisible (encodage) | Errata du scénario PoD « La Tablette d'Argent ». |
| `images__3__3e__Le_demon_se_reveille.pdf` | 4 | oui | Scénario fan « Le démon se réveille ». |
| `images__6__6b__Explorations.Alternatif.pdf` | 8 | oui | Cartes Exploration alternatives (fan). |
| `images__d__d6__Creatures.pdf` | 5 | illisible (encodage) | Tuiles de créatures additionnelles (fan). |
| `images__e__ee__Door_Tokens.pdf` | 1 | non | Planche de jetons Porte. |
| `images__4__49__Porte_mom_light.pdf` | 3 | non | Planches de portes (version légère). |
| `images__6__6e__Square_Exploration_Tokens_v3.pdf` | 2 | non | Jetons d'exploration numérotés 1–23. |


---

## 5. Nouveaux PDF (35 fichiers)

### 5.1 Livrets de règles officiels FR (« notices ») — texte extractible
Liés depuis les pages `Extension_*` / `Horreur_à_Arkham` (§6.16).

| Fichier | Pages | Texte | Contenu |
|---|---|---|---|
| `images__9__98__Notice_HaA_Base.pdf` | 24 | oui (111 k car.) | **Livret de règles du jeu de base**, FR, version © 2005 (1re impression VF). Même contenu que `Arkham.pdf` (version 2011) à la révision près. |
| `images__4__4b__Notice_HaA_Dunwich.pdf` | 12 | oui (61 k car.) | **L'Horreur de Dunwich**, livret de règles **en français** (remplace `Dunwinch_VO.pdf`). 56 Mo (images haute déf.). |
| `images__5__53__Notice_HaA_Kingsport.pdf` | 16 | oui (71 k car.) | **L'Horreur de Kingsport**, livret de règles FR (version texte du scan `Kingsport.pdf`). |
| `images__9__9b__Notice_HaA_Innsmouth.pdf` | 16 | oui (68 k car.) | **L'Horreur d'Innsmouth**, livret de règles FR (version texte du scan `Innsmouth.pdf`). |
| `images__7__76__Notice_HaA_Miskatonic.pdf` | 4 | oui (52 k car.) | **L'Horreur de Miskatonic**, règles FR (petite extension : nouvelles cartes, variante Bataille Épique, cartes Engeance d'Innsmouth, cartes de remplacement). |
| `images__0__05__Notice_HaA_LMDPN(ER).pdf` | 2 | oui (20 k car.) | **La Malédiction du Pharaon Noir (édition révisée)**, règles FR. |
| `images__9__9f__Notice_HaA_LREJ.pdf` | 2 | oui (14 k car.) | **Le Roi en Jaune**, règles FR (même extension que `Roi_en_Jaune.pdf`, texte extrait dans un ordre différent). |
| `images__b__bb__Notice_HaA_LCNDB.pdf` | 2 | oui (13 k car.) | **La Chèvre Noire des Bois**, règles FR (version texte du scan `Chèvre_Noire.pdf`). |
| `images__6__66__Notice_HaA_LRDLS.pdf` | 2 | oui (17 k car.) | **Le Rôdeur devant le Seuil**, règles FR (version texte du scan `Rodeur_Seuil.pdf`). |

### 5.2 Planches de cartes corrigées (erratas) — image seule, pas de texte extractible
Chacune est liée depuis la page `Erratas_-_<extension>.html` correspondante (§6.15). Format A4 (595×842), cartes au format réel, prêtes à imprimer, réalisées avec Strange Eons. **Seules les cartes ayant reçu un errata** y figurent (≈ 10 cartes ou 2 fiches par page), pas l'extension entière. Le texte est complet et lisible sur l'image → exploitable par OCR.

| Fichier | Pages | Contenu |
|---|---|---|
| `images__b__b5__Base-ToutesLesCartes-1.pdf` | 1 | Jeu de base : cartes corrigées (Lieu, Mythe… ex. Asile d'Arkham, Boutique de Souvenirs, Vague de Chaleur). |
| `images__d__db__Base-ToutesLesCartes-2.pdf` | 1 | Jeu de base : suite. |
| `images__9__95__Base-ToutesLesCartes-3.pdf` | 1 | Jeu de base : suite. |
| `images__4__49__Base-ToutesLesFiches.pdf` | 2 | Jeu de base : **feuilles de Grands Anciens corrigées** (ex. Ithaqua, Shub-Niggurath : adorateurs, pouvoir, attaque, piste de destin). |
| `images__2__27__Dunwich-ToutesLesCartes-1.pdf` | 1 | Dunwich : cartes corrigées. |
| `images__7__7c__Dunwich-ToutesLesCartes-2.pdf` | 2 | Dunwich : suite. |
| `images__2__24__Dunwich-ToutesLesFiches.pdf` | 2 | Dunwich : **fiches d'investigateurs corrigées** recto/verso (ex. Léo Anderson : SAN/RES, maison, possessions, pouvoir, 3 curseurs à 4 crans, histoire). |
| `images__7__7a__Kingsport-ToutesLesCartes.pdf` | 2 | Kingsport : cartes corrigées. |
| `images__e__e7__Innsmouth-ToutesLesCartes.pdf` | 2 | Innsmouth : cartes corrigées. |
| `images__6__62__Innsmouth-ToutesLesFiches.pdf` | 1 | Innsmouth : fiche d'investigateur corrigée (Trish Scarborough). |
| `images__2__2b__Miskatonic-ToutesLesCartes.pdf` | 1 | Miskatonic : cartes corrigées (Engeance…). |
| `images__6__6c__ChevreNoir-ToutesLesCartes.pdf` | 1 | Chèvre Noire : cartes corrigées. |
| `images__6__66__PharaonNoir-ToutesLesCartes-1.pdf` | 1 | Pharaon Noir (1re éd.) : cartes corrigées. |
| `images__9__91__PharaonNoir-ToutesLesCartes-2.pdf` | 1 | Pharaon Noir (1re éd.) : suite. |
| `images__b__bb__PharaonNoir-ToutesLesCartes-3.pdf` | 1 | Pharaon Noir (1re éd.) : suite. |
| `images__8__80__PharaonNoirNE-ToutesLesCartes-1.pdf` | 1 | Pharaon Noir (éd. révisée) : cartes corrigées. |
| `images__5__51__PharaonNoirNE-ToutesLesCartes-2.pdf` | 1 | Pharaon Noir (éd. révisée) : suite. |
| `images__f__f5__PharaonNoirNE-ToutesLesCartes-3.pdf` | 1 | Pharaon Noir (éd. révisée) : suite. |
| `images__a__a1__PharaonNoirNE-ToutesLesCartes-4.pdf` | 1 | Pharaon Noir (éd. révisée) : suite. |
| `images__e__eb__RoiEnJaune-ToutesLesCartes-1.pdf` | 1 | Roi en Jaune : cartes corrigées. |
| `images__2__20__RoiEnJaune-ToutesLesCartes-2.pdf` | 2 | Roi en Jaune : suite. |
| `images__7__7c__Rodeur-ToutesLesCartes-1.pdf` | 1 | Rôdeur : cartes corrigées. |
| `images__2__24__Rodeur-ToutesLesCartes-2.pdf` | 1 | Rôdeur : suite. |

### 5.3 Les Demeures de l'Épouvante — hors sujet
| Fichier | Pages | Texte | Contenu |
|---|---|---|---|
| `images__2__21__Jusqua-ce-que-la-mort-nous-separe-guides.pdf` | 6 | oui (8 k car.) | Guides (Gardien + investigateurs) du scénario PoD « Jusqu'à ce que la Mort nous sépare ». |
| `images__f__fc__La_Saison_de_la_Sorciere_guides.pdf` | 6 | oui (8 k car.) | Guides du scénario PoD « La Saison de la Sorcière ». |
| `images__f__fa__Tablette-Argent_guides.pdf` | 6 | non (scan) | Guides du scénario PoD « La Tablette d'Argent ». |

---

## 6. Nouvelles pages HTML (699 fichiers)

Structure commune : citation de Lovecraft (souvent), texte de règle ou description, section « Erratas & Clarifications », liens de navigation. Les pages de carte individuelles contiennent l'image de la carte (non téléchargée) et, pour les objets communs, un résumé en texte (extension, type d'arme, mains, bonus, prix). Dans chaque groupe : un tableau des pages qui ont du contenu, puis la liste des fiches vides et des doublons.

### 6.1 Règles et glossaire

| Fichier | mots / img | Contenu |
|---|---|---|
| `Abandon_d'un_investigateur.html` | 68 / 0 | Un joueur peut volontairement abandonner un Investigateur qui possède au moins deux Blessures ou Folies. Le joueur saute simplement son tour, annonce que son Investigateur prend sa retraite et pioche un … |
| `Activer_Les_Capacités_Du_Mythe.html` | 107 / 1 | Le Premier Joueur regarde le texte spécial de la Carte Mythe et son attribut (en haut de la carte (A)). Attribut Chaque Carte Mythe possède un attribut qui détermine si la carte … |
| `Ajustement_Des_Compétences.html` | 254 / 1 | Chaque joueur peut ajuster les compétences de son investigateur en utilisant les 3 Pions Coulissants de sa feuille d’investigateur. Tout est expliqué sur le schéma de cette page. Exception: Lors de … |
| `Arme_Sans_Mains.html` | 73 / 6 | Les armes sans main sont des armes qu'il est possible d'utiliser en combat sans leur attribuer une main. Ces armes apparaissent dans l'extension l'Horreur de Dunwich. Il est donc possible de … |
| `Banni_Du_Quartier.html` | 338 / 18 | Arkham est une ville insulaire, et il ne faut pas prendre à la légère ses gens et ses autorités. Lorsqu'on joue avec l' Extension La Malédiction Du Pharaon Noir, et qu'un … |
| `Bataille_Finale.html` | 700 / 2 | Avant que la Bataille Finale ne commence, on défausse les cartes Environnement ou Rumeur actives. De plus, une fois que la Bataille Finale commence, les Investigateurs ne peuvent plus collecter d’argent des … |
| `Bonus_De_Combat.html` | 638 / 0 | Sections : Armes standards ; Armes à bonus ; Armes à usage unique ; Armes à recharger ; Les Sorts ; Erratas & Clarifications. |
| `Combattre.html` | 223 / 1 | Il arrive que les investigateurs doivent surmonter leur peur pour combattre le mythe. Quand un investigateur engage un Combat contre un Monstre, il faut suivre les étapes suivantes dans l’ordre: … |
| `Combattre_un_Grand_Ancien.html` | 569 / 1 | Le Grand Ancien se réveille quand la dernière case de sa piste Destin reçoit un pion Destin. Si ce pion est placé suite à la récompense ou à la pénalité reçue par … |
| `Compétence_D'Investigateur.html` | 181 / 0 | Ce sont les attributs de base que les investigateurs utilisent pour accomplir des choses dans le jeu. Une valeur de compétence représente le nombre de dés que l’investigateur lance pour un Test … |
| `Concentration.html` | 20 / 0 | C’est la capacité de votre personnage à Ajuster Ses Compétences à chaque tour. |
| `Décharger.html` | 47 / 0 | Décharger & Epuiser sont deux termes synonymes. Décharger une carte signifie la retourner face cachée. Elle ne peut plus être utilisée tant qu'elle est face cachée. Une carte est Déchargée tant qu'elle … |
| `Défaite.html` | 73 / 0 | Si le Grand Ancien se Réveille et bat les Investigateurs lors de la Bataille Finale, il y a une rupture du continuum espace-temps et le Grand Ancien est relâché. L’humanité entière … |
| `Défausser.html` | 66 / 0 | Défausser une carte ou Pion, c'est le retirer de la partie active du jeu pour le remettre en disponibilité dans le paquet de cartes ou le tas de pions d'où il est … |
| `Dépenser_Des_Pions_Indice.html` | 157 / 0 | Un joueur peut dépenser des pions Indice, un à la fois, après un Test de compétence (réussi ou non). Chaque pion Indice dépensé permet au joueur de lancer un dé supplémentaire. Si … |
| `Dévoré.html` | 93 / 0 | Quand un Investigateur n'a plus de Santé Mentale NI de Résistance au même moment (les deux caractéristiques sont à zéro), il est Dévoré ! De même, quand le maximum de Santé Mentale … |
| `Echanger.html` | 75 / 0 | Pendant la Phase De Mouvement, un Investigateur dans le même Lieu, la même Zone De Rues ou la même zone de l' Autre Monde qu'un autre Investigateur peut échanger avec … |
| `Echapper.html` | 430 / 1 | Quand un Investigateur tente de sortir d’une zone contenant un Monstre, ou termine son mouvement dans une telle zone, il doit Combattre le monstre ou s’en Echapper. Un Investigateur peut tenter d’Echapper … |
| `Effectuer_Les_Actions_D’Entretien.html` | 145 / 0 | Après avoir Restauré les cartes Déchargées, chaque joueur doit vérifier sur ses cartes investigateur s’il a des actions d’Entretien. Chaque joueur doit faire toutes les actions d’Entretien listées sur ses cartes … |
| `Environnement.html` | 114 / 1 | Le texte spécial d’une Carte Mythe ' Environnement reste en jeu pour plusieurs tours. Le Premier Joueur place la carte face visible à côté du plateau, et défausse l’éventuelle carte mythe Environnement … |
| `Epuiser.html` | 12 / 0 | Décharger & Epuiser sont deux termes synonymes. |
| `Fermer.html` | 345 / 1 | Avant de pouvoir fermer un portail, l’Investigateur doit y entrer, Explorer l’ Autre Monde où il mène, puis revenir à Arkham. Quand un Investigateur revient d’un Autre Monde, le joueur doit … |
| `Fin_Du_Jeu.html` | 32 / 0 | Le jeu se termine de l’une de ces deux façons: les investigateurs ont surmonté la menace ou la menace les a vaincus. |
| `Fin_Du_Tour.html` | 101 / 0 | Quand le Premier Joueur a résolu toutes les étapes de la Phase De Mythe, le marqueur de Premier Joueur est donné au joueur à sa gauche. Le tour est maintenant fini … |
| `Fou.html` | 170 / 0 | Si la Santé Mentale d'un Investigateur est réduite à 0, il devient temporairement Fou. Règle du jeu de base: Il doit immédiatement choisir et défausser la moitié (arrondi à l'inférieur) de … |
| `Fuir.html` | 68 / 0 | Fuir est une option lorsqu'un Investigateur doit Combattre un Monstre après le Test D’Horreur (réussi ou non). L’investigateur tente d’ Echapper au monstre, en faisant un Test D’Evasion normal. S’il réussit le … |
| `Gros_Titre.html` | 38 / 1 | Le Premier Joueur résout immédiatement le texte spécial du gros titre. Puis, il défausse la carte mythe face cachée au fond du paquet mythe. Activer Les Capacités Du Mythe |
| `Inconscient.html` | 171 / 0 | Si la Résistance d'un Investigateur est réduite à 0, il devient temporairement Inconscient. Règle du jeu de base: Il doit immédiatement choisir et défausser la moitié (arrondi à l'inférieur) de ses … |
| `Investigateur.html` | 759 / 84 | Même contenu que `Investigateurs.html` (déjà inventorié, §2). |
| `Irruption_De_Portail.html` | 155 / 1 | Une Irruption De Portail est une Carte Mythe dont le portail ((1) d'une Carte Mythe) est coloré en rouge orangé. Une Irruption De Portail fonctionne comme une Carte Mythe normale sauf di … |
| `Mains.html` | 342 / 0 | Il y a une limite pour le nombre d’ Armes et de Sorts qu’un investigateur peut utiliser au même moment. Cette limite est représentée par les icônes Mains imprimées dans le coin … |
| `Ouvrir_Un_Portail_Et_Engendrer_Des_Monstres.html` | 663 / 0 | Le Premier Joueur regarde le Lieu (indiqué en bas à gauche de la Carte Mythe (1)) où les forces maléfiques attaquent à ce tour. Il peut se produire une de ces trois … |
| `Perdre_Son_Tour.html` | 119 / 0 | D'après les questions / réponses de Kevin Wilson, l'utilisation de perdre son tour est une erreur de vocabulaire de sa part. "Perdre son tour" devrait toujours être considéré comme être Retardé. … |
| `Phase_D'Entretien.html` | 44 / 0 | Durant cette phase, chaque joueur fait les actions suivantes, dans l'ordre. 1 Restaurer Les Cartes Déchargées 2 Effectuer Les Actions D’Entretien 3 Ajustement Des Compétences Puis on passe à la Phase De … |
| `Phase_De_Mouvement.html` | 596 / 2 | Lors de cette phase, chaque joueur fait l’une des deux actions de mouvement selon la position de son Investigateur à Arkham ou dans un Autre Monde: Mouvement à Arkham Si l’investigateur … |
| `Phase_De_Mythe.html` | 52 / 1 | Lors de cette phase, le Premier Joueur pioche une Carte Mythe et effectue les actions suivantes dans l'ordre: 1 Ouvrir Un Portail Et Engendrer Des Monstres 2 Placer Des Pions Indice … |
| `Phase_De_Rencontres_A_Arkham.html` | 380 / 0 | Lors de cette phase, chaque joueur dont l’Investigateur est dans un Lieu (pas une Zone De Rues ni un Autre Monde ) doit faire une des actions suivantes. L’action de l’Investigateur dépend … |
| `Phase_De_Rencontres_Dans_Un_Autre_Monde.html` | 271 / 1 | Durant cette phase, les investigateurs dans une case Autre Monde y font une rencontre. Les cases de l’Autre Monde sont marquées par des icônes colorées. Ces cercles colorés sont appelés symboles de … |
| `Placer_Des_Pions_Indice.html` | 78 / 0 | La plupart des Cartes Mythe indiquent un Lieu où un indice apparaît. On place un Pion Indice sur le Lieu indiqué, sauf s’il y a un Portail ouvert à cet endroit. Si … |
| `Points_De_Victoire.html` | 199 / 0 | D’abord, on soustrait le Niveau De Terreur à la fin du jeu au plus grand chiffre de l’ Echelle Du Destin du Grand Ancien. Puis on applique les modificateurs suivants: … |
| `Premier_Joueur.html` | 117 / 1 | Le marqueur Premier Joueur est possédé par le joueur qui joue le premier dans un tour. C'est lui qui est garant du bon déroulement du tour de jeu, jusqu'à passé la main. … |
| `Préparation_Du_Jeu.html` | 104 / 1 | 1 Préparation De La Surface De Jeu 2 Placement Des Pions Indice Initiaux 3 Choix Du Premier Joueur 4 Détermination Des Investigateurs 5 Révélation Du Grand Ancien 6 Séparation Des Paquets 7 … |
| `Restaurer_Les_Cartes_Déchargées.html` | 128 / 0 | Certaines cartes sont Déchargées quand on les utilise, ce qui veut dire qu’on les retourne face cachée pour le reste du tour. Au début de la Phase D’Entretien, chaque joueur restaure ces … |
| `Retardé.html` | 82 / 0 | Durant le jeu, certains effets peuvent Retarder un Investigateur. Quand cela arrive, on couche le marqueur de l’Investigateur, pour indiquer ce retard. Les Investigateurs Retardés ne reçoivent pas de points de mouvement … |
| `Rumeur.html` | 189 / 1 | Le texte spécial d’une Carte Mythe Rumeur fait effet jusqu’à ce que la condition Réussite ou Échec de la carte soit remplie. On résout alors la condition et on défausse la carte … |
| `Résistance.html` | 48 / 1 | Un Investigateur commence le jeu avec un nombre de pions Résistance égal aux valeurs indiquées sur sa feuille. Cette valeur est le maximum de Résistance de l'Investigateur. La Valeur de Résistance ne … |
| `Réveil.html` | 399 / 0 | En dépit de leurs efforts désespérés, il se peut que le Grand Ancien se Réveille et entre dans Arkham. Si cela se produit, les investigateurs devront le repousser en se servant des … |
| `Santé_Mentale.html` | 51 / 1 | Un Investigateur commence le jeu avec un nombre de pions Santé Mentale égal aux valeurs indiquées sur sa feuille. Cette valeur est le maximum de Sante Mentale de l'Investigateur. La Valeur de … |
| `Sceller.html` | 168 / 1 | Quand un Investigateur réussit à Fermer un Portail, il peut immédiatement dépenser 5 Pions Indice pour le Sceller définitivement. Le joueur prend un Pion Destin de la réserve, le pose … |
| `Sommeil.html` | 80 / 0 | Le Grand Ancien est plongé dans son Sommeil durant toute la partie, et en général il dispose d'un pouvoir qui s'applique tant qu'il est dans cet état. Lorsqu'il se Réveille, c'est … |
| `Statut_De_L'Investigateur.html` | 45 / 0 | Le Statut D'Un Investigateur peut être modifié par des cartes ( Spéciales ou non) ou des règles de jeu. Les différents statuts sont: Arrêté Béni Dévoré Fou Inconscient Maudit Perdu Dans … |
| `Test.html` | 593 / 0 | Les Investigateurs sont souvent amenés à faire des Tests de Compétence pour accomplir ou éviter quelque chose. Quand le jeu demande un test de Compétence, il est présenté sous cette forme: … |
| `Test_D'Evasion.html` | 205 / 1 | Un Test D’Evasion est un Test de Compétence basé sur la valeur de Discrétion de l’investigateur. Le test est modifié par la valeur de Conscience du Monstre (valeur imprimée dans le coin … |
| `Test_D'Horreur.html` | 198 / 0 | Un test d’Horreur est un Test de compétence basé sur la valeur de Volonté de l’investigateur qui se produit lorsqu'un combat s'engage avec un Monstre. IMPORTANT: Si l’investigateur a réussi … |
| `Test_De_Combat.html` | 284 / 0 | Un Test De Combat est un Test de compétence basé sur la valeur de Combat de l’investigateur. Cette valeur est modifiée par la Valeur De Combat du Monstre. IMPORTANT: Il est … |
| `Timing.html` | 41 / 0 | Quand plusieurs effets du jeu se produisent simultanément, c’est aux joueurs de choisir l’ordre dans lequel ils se produisent. Si les joueurs n’arrivent pas à se mettre d’accord, c’est le Premier Joueur … |
| `Tour_De_Jeu.html` | 122 / 0 | Un tour d’Horreur à Arkham est divisé en cinq phases. Lors de chaque phase, chaque joueur, en commençant par le Premier Joueur puis dans le sens horaire, effectue les actions qui ont … |
| `Trophée_De_Portail.html` | 83 / 0 | En général, un Portail devient un Trophée pour l'Investigateur qui vient de le Fermer ou de le Sceller. On garde le marqueur de Portail à côté de la fiche d'Investigateur. Dépenser … |
| `Utiliser_Les_Armes_Et_Sorts_En_Combat.html` | 187 / 0 | Les Objets transportés par les Investigateurs servent à prendre l'avantage dans le combat à mort qui les oppose contre un Monstre en donnant des Bonus De Combat. Le plus grand avantage … |
| `Victoire.html` | 328 / 0 | Les investigateurs gagnent la partie de l’une de ces trois manières: Portails 2 Sceller les Portails 3 Bannir le Grand Ancien 4 Points De Victoire Fermer les Portails Pour satisfaire cette … |

**Doublons** (texte identique) — 5 : `Compétences_D'Investigateur.html` = `Compétence_D'Investigateur.html`, `Irruption_de_Portail~5065c563.html` = `Irruption_De_Portail.html`, `Test_D’Evasion.html` = `Test_D'Evasion.html`, `Test_D’Horreur.html` = `Test_D'Horreur.html`, `Tour_de_jeu~bba56618.html` = `Tour_De_Jeu.html`.


### 6.2 Monstres (règles, types, capacités)

| Fichier | mots / img | Contenu |
|---|---|---|
| `Aquatique.html` | 195 / 14 | Un Monstre Aquatique possède un marqueur à bord orange. L' Extension L'Horreur De Kingsport introduit ce nouveau type de mouvement de monstre, qui est également utilisé par l' Extension l'Horreur d'Innsmouth. … |
| `Arme.html` | 249 / 134 | **Erreur du wiki** : la page « Armes » contient la liste des monstres classés par Conscience/Discrétion (+2 à −4), copie de `Conscience.html`. |
| `Capacité_De_Monstre.html` | 263 / 0 | En fonction de la nature d'un Monstre, outre ses caractéristiques de combat, il peut posséder une ou plusieurs des capacités ou attributs suivants, afin de se renforcer: Un Monstre de type … |
| `Capacité_de_mouvement.html` | 83 / 0 | Pour le déplacement des Monstres, on utilise le code couleur de leur marqueur tel que: Un Monstre à Bord orange est Aquatique Un Monstre à Bord mauve est Chasseur Un Monstre … |
| `Cauchemardesque.html` | 87 / 1 | Cette capacité est toujours accompagné d'une valeur numérique X à sa suite (en général 1 ou 2). Un Investigateur qui réussit un Test D’Horreur contre un Monstre avec cette capacité perd tout … |
| `Chasseur.html` | 127 / 10 | Un Monstre Chasseur possède un marqueur à bord mauve. L' Extension L'Horreur De Dunwich introduit ce nouveau type de mouvement de monstre. Les monstres chasseurs se déplacent normalement sauf si un investigateur … |
| `Conscience.html` | 249 / 134 | Monstres classés par valeur de Conscience (modificateur d'évasion, +2 à −4), noms + images. Texte identique à `Arme.html` (erreur du wiki). |
| `Dégâts_De_Combat.html` | 114 / 1 | Les Dégâts De Combat (6) indiquent combien de Résistance est perdue par un Investigateur qui échoue à son Test De Combat ou un Test D'Evasion contre un Monstre. Le joueur défausse alors … |
| `Dégâts_D’Horreur.html` | 64 / 1 | Les Dégâts D’Horreur (4) indiquent combien de Santé Mentale est perdue par un Investigateur qui échoue à son test d’Horreur contre ce Monstre. Le joueur défausse alors le nombre approprié de jetons … |
| `Déplacer_les_monstres.html` | 393 / 1 | Les monstres apparaissent dans différents Lieux mais les quittent rapidement pour errer dans les rues d’Arkham. Pour déterminer le mouvement des Monstres, le Premier Joueur regarde le coin inférieur droit de la … |
| `Embuscade.html` | 54 / 12 | Une fois que le combat a commencé avec un Monstre possédant l'Embuscade, un Investigateur n’a plus la possibilité de s’enfuir, il doit combattre jusqu’à la victoire de l’un des deux protagonistes. Goule … |
| `Force.html` | 54 / 1 | La Force est la difficulté d’un Test De Combat contre un Monstre qui est représentée par le nombre de « gouttes de sang » (5) (Ses points de vie en somme). Elle … |
| `Immobile.html` | 52 / 9 | Un Monstre Immobile possède un marqueur à bord jaune. Les monstres Immobiles ne bougent JAMAIS. Ils restent toujours à l'endroit où ils sont entrés en jeu. Fantome (x3) Sorcier (x2) Chasseur Spectral … |
| `Immunité_Physique.html` | 103 / 15 | Un Objet (en général une Arme ) qui donnerait des dés de Bonus De Combat de type physique à un Investigateur ne fournit aucun bonus. Chien de Tindalos Fantome Larve Amorphe Vampire … |
| `Insaisissable.html` | 155 / 6 | Règle « Insaisissable » ; texte identique à `Irrésistible.html` (copie côté wiki : les deux capacités y sont décrites). |
| `Inépuisable.html` | 55 / 5 | Un monstre avec cette capacité ne peut pas être récupéré comme Trophée de Monstre. À chaque fois qu’il est battu au Combat, il retourne dans la Tasse des Monstres. … |
| `Irrésistible.html` | 155 / 6 | Un Monstre avec cette Capacité essaie d'éviter le Combat avec les Investigateurs au lieu de le déclencher ! Sur la face mouvement d'un Monstre Insaisissable la couleur de sa valeur de Conscience … |
| `La_Bête.html` | 80 / 2 | C'est un Monstre Masque. C'est un Monstre Chasseur. La carte présentée ici est une version révisée que vous pouvez sauvegarder et imprimer pour remplacement ! Erratas & Clarifications La Bête, … |
| `Limite_De_Monstres.html` | 701 / 1 | Il y a une limite au nombre de Monstres qu'il peut y avoir à Arkham au même moment: Cette limite est égale au nombre de joueurs plus trois. Cette Limite … |
| `Masque.html` | 159 / 15 | Ils ne sont pas ajoutés à la Tasse Des Monstres lors de la mise en place du jeu, sauf si Nyarlathotep est le grand ancien de la partie. Le monstre " Pharaon … |
| `Monstres_Altérés.html` | 290 / 39 | Sections : Horreur à Arkham ; Horreur à Dunwich ; Horreur à Kingsport ; Horreur d'Innsmouth. |
| `Mort-Vivant.html` | 41 / 9 | Un Monstre Mort-Vivant n'a pas de pouvoir spécial mais certaines cartes peuvent avantager ceux qui reviennent d'entre les morts ou au contraire les pénaliser. Fantome Vampire Zombie Momie Spectre Squelette Capacité De … |
| `Normal.html` | 105 / 46 | Un Monstre Normal possède un marqueur à bord noir. Les monstres normaux se Déplacent normalement. Etonnant ! Chose Très Ancienne (x2) (x1) Cultiste (x6) (x1) (x1) (x1) Dhole Dieu De La Langue … |
| `Périphérie.html` | 617 / 1 | Lorsque la Limite De Monstres est dépassée, tous les Monstres en trop sont placés dans la Zone Spéciale du Plateau de Jeu d' Arkham de Périphérie. Les Monstres présents à Dunwich et … |
| `Rapide.html` | 65 / 13 | Un Monstre Rapide possède un marqueur à bord rouge. Les monstres rapides se déplacent DEUX fois en suivant les flèches appropriées aux deux étapes de déplacement. Ils arrêtent de se déplacer s'ils … |
| `Rejeton.html` | 41 / 22 | Voir le Monstre L' Horreur De Dunwich pour le détail de l'utilisation de ces cartes. L'Horreur de Miskatonic rajoute 6 nouvelles cartes Horreur de Dunwich. Manquante.jpg Manquante.jpg Manquante.jpg Manquante.jpg Manquante.jpg Manquante.jpg Cartes |
| `Résistance_Physique.html` | 127 / 29 | Un Objet (en général une Arme ) qui donnerait des dés de Bonus De Combat de type physique à un Investigateur ne fournit que la moitié (arrondi au supérieur) du bonus normal. … |
| `Serviteur_De_Glaaki.html` | 59 / 2 | Serviteur Glaaki2.jpg Serviteur Glaaki3.jpg Serviteur Glaaki4.jpg Serviteur Glaaki5.jpg C'est un Monstre Rejeton. Au nombre de 5, ils sont liés au Grand Ancien Glaaki. Erratas & Clarifications Aucun. Normal Liste des … |
| `Surprise.html` | 125 / 0 | Quand un Monstre vous Surprend, vous ne pouvez pas vous enfuir et devez le combattre. De plus il faut suivre les étapes suivantes pour le premier round de Combat: 1) … |
| `Symbole_Dimensionnel.html` | 108 / 9 | Des Portails Le Symbole Dimensionnel d’un Portail est imprimé sur le bas du recto du Marqueur, à l'opposé de la destination du Portail. Des Monstres Le Symbole Dimensionnel d’un Monstre est imprimé … |
| `Tasse_Des_Monstres.html` | 96 / 0 | Pendant la Préparation Du Jeu, placez les marqueurs Monstres dans un récipient opaque et mélangez-les. Une tasse de café, un sac en toile ou en plastique font très bien l’affaire. Ce … |
| `Trophée_De_Monstre.html` | 117 / 0 | En général, un Monstre devient un Trophée pour l'Investigateur qui vient de le battre au Combat sauf si ce Monstre est soumis à une règle spéciale comme être un Rejeton ou Inépuisable … |
| `Unique.html` | 68 / 16 | Un Monstre Unique possède un marqueur à bord vert. Les monstres uniques ont des capacités spéciales de mouvement indiquées sur la face combat de leur jeton et il convient de suivre les … |
| `Vague_de_monstres.html` | 264 / 0 | On n'ajoute pas de pions destin lors d'une vague de monstres ! Quand une Vague De Monstres se produit pendant la Phase De Mythe, le nombre de Monstres piochés et placés … |
| `Valeur_De_Combat.html` | 29 / 1 | La Valeur De Combat (7) est le modificateur utilisé pour un Test De Combat contre un Monstre. Test De Combat |
| `Valeur_D’Horreur.html` | 26 / 1 | La Valeur D’Horreur (3) est le modificateur utilisé pour un Test D’Horreur contre un Monstre. Test D’Horreur |
| `Volant.html` | 327 / 18 | Un Monstre Volant possède un marqueur à bord bleu clair. Ils ne se déplacent que lorsque leur Symbole Dimensionnel sort lors de la Phase De Mythe, comme les autres monstres et … |

**Doublons** (texte identique) — 4 : `Cartes_Horreur_De_Dunwich.html` = `Rejeton.html`, `Déplacer_Les_Monstres~9c895459.html` = `Déplacer_les_monstres.html`, `Symboles_Dimensionnels.html` = `Symbole_Dimensionnel.html`, `Vague_De_Monstres~a5cc0350.html` = `Vague_de_monstres.html`.


### 6.3 Pions et marqueurs

| Fichier | mots / img | Contenu |
|---|---|---|
| `Activité.html` | 40 / 1 | Ces marqueurs indiquent les zones du plateau où se produisent des événements inhabituels. En général ces sont les cartes mythe qui produisent des Activités à un endroit du plateau. |
| `Couvée.html` | 21 / 1 | Ces jetons sont utilisés avec le Grand Ancien Eihort. Eihort |
| `Dollars.html` | 98 / 1 | Le Dollar est la monnaie du Jeu;) Vous trouverez des jetons de valeur 1, 5 ou 10 Dollars. Souvent, les investigateurs se verront dire qu’ils peuvent acheter des objets “au prix … |
| `Décombres.html` | 74 / 1 | Ces pions sont utilisés avec le Grand Ancien Shudde M'ell qui détruit des Lieux Du Plateau de jeu au cours de la partie. Ces 7 jetons Décombres sont placées face cachée à … |
| `Exploré.html` | 93 / 1 | Lorsqu'un Investigateur revient d'un Autre Monde par un Portail, il place un marqueur Exploré sous son Investigateur pour indiquer qu’il a Exploré le Portail. Ce marqueur reste en jeu tant que … |
| `Lieu_Fermé.html` | 92 / 1 | Ces marqueurs sont placés sur des Lieux qui ont été fermés, soit à cause du Niveau De Terreur ou à cause d'événements précis. Aucun Monstre, ni Investigateur ne peut entrer dans … |
| `Marqueur_De_Terreur.html` | 62 / 1 | Le Marqueur De Terreur est placé sur l' Echelle De Terreur pour indiquer le moral et l'état mental des habitants d'Arkham. Avec l'augmentation du Niveau De Terreur, les boutiques ferment et … |
| `Marqueur_Horreur_De_Dunwich.html` | 35 / 1 | Ces Marqueurs servent à remplir l' Échelle de l'Horreur de Dunwich et ainsi marquer la progression de l'Horreur dans la ville de Dunwich. |
| `Marqueur_aquatique.html` | 43 / 1 | Avec l' Extension L'Horreur De Kingsport (ou l' Extension l'Horreur d'Innsmouth ), ces marqueurs servent à identifier deux originaux d'Arkham (les Quais et l' Ile Inexplorée en tant que Lieux Aquatiques. |
| `Pion_Bastet.html` | 21 / 1 | Ces jetons sont utilisés avec le Gardien Bastet. Marqueurs, Jetons & Pions Gardien Bastet |
| `Pion_Destin.html` | 61 / 1 | Ces pions sont placés sur l' Echelle Du Destin de la feuille du Grand Ancien quand de nouveaux Portails sont ouverts. IMPORTANT: On ajoute un Pion Destin à chaque fois qu'un … |
| `Pion_Signe_Des_Anciens.html` | 45 / 1 | Au dos de chaque pion destin se trouve le Pion Signe Des Anciens. Quand un investigateur peut Fermer et Sceller définitivement un Portail, on place un Pion Signe Des Anciens sur le … |
| `Pion_indice.html` | 44 / 1 | Les pions Indice représentent des informations sur la menace du mythe que les investigateurs peuvent collecter. On peut Dépenser Des Pions Indice pour améliorer le résultat d'un Test de Compétences Des Investigateurs. … |
| `Pions_Coulissants.html` | 37 / 1 | Les Pions Coulissants servent à l' Ajustement Des Compétences sur la feuille d'Investigateur. Chaque Pion Coulissant fait correspondre deux Compétences Des Investigateurs tel Vitesse et Discrétion. |
| `Pions_Failles_Supplémentaires.html` | 161 / 6 | Ces pions Progression De Faille permettent de remplacer ceux de l' Extension L'Horreur De Kingsport. Tous les lieux stables de tous les plateaux de jeu disponibles ont été représentés, ainsi les … |
| `Progression_De_Faille.html` | 62 / 18 | Ces jetons sont utilisés avec l' Extension L'Horreur De Kingsport en suivant les règles de Failles. 607 Water Street (x2) 7eme Maison Gauche (x2) Boutique De Curiosités La Corde Et L'Ancre … |
| `Signe_Jaune.html` | 28 / 1 | Ces pions spéciaux représentent le pouvoir de destruction de l'esprit du Héraut Le Roi En Jaune. |

**Doublons** (texte identique) — 2 : `Pion_Indice~13364f2f.html` = `Pion_indice.html`, `Pions_Indice.html` = `Pion_indice.html`.


### 6.4 Plateau : villes, lieux, zones, Autres Mondes, portails

| Fichier | mots / img | Contenu |
|---|---|---|
| `Administration.html` | 104 / 0 | Lieu stable ; capacité spéciale « les cours » expliquée ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Arkham.html` | 228 / 10 | IMPORTANT: Les cartes et règles du jeu qui se réfèrent à "Arkham" s'appliquent aussi aux lieux et zones de rues de Dunwich, de Kingsport et d' Innsmouth. Par exemple … |
| `Asile_d'Arkham.html` | 112 / 0 | Lieu stable ; capacité spéciale « les soins psychiatriques » expliquée ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Autre_Monde.html` | 156 / 4 | Même contenu que `Autres_Mondes.html` (déjà inventorié, §2). |
| `Banque_d'Arkham.html` | 118 / 0 | Lieu stable ; capacité spéciale « le prêt bancaire » expliquée ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Bibliothèque.html` | 163 / 0 | citation Lovecraft ; Lieu stable ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Bois.html` | 89 / 0 | Lieu instable ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Boutique_de_Souvenirs.html` | 139 / 0 | Lieu stable ; capacité spéciale « l'achat d'objets uniques » expliquée ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Capacité_Spéciale_De_Lieu.html` | 133 / 0 | Certains Lieux du plateau ont une Capacité Spéciale. Elle est inscrite sur le Plateau De Jeu sous le nom du Lieu où se trouve l'Investigateur. Quand un investigateur entre dans un tel … |
| `Carcosa.html` | 51 / 1 | Autre Monde : couleurs de rencontre bleu et jaune. Pas le texte des rencontres. |
| `Caverne_Noire.html` | 63 / 0 | Lieu instable ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Ciel.html` | 113 / 1 | Le Ciel est une zone de transit considérée comme une Zone De Rues connectée à toutes les Zones De Rues d' Arkham et de tous les Plateaux De Jeu supplémentaires. En général, … |
| `Cimetière.html` | 97 / 0 | Lieu instable ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Cité_De_La_Grande_Race.html` | 207 / 0 | Autre Monde : citation Lovecraft ; couleurs de rencontre vert et jaune. Pas le texte des rencontres. |
| `Coeur_de_la_Loge.html` | 92 / 1 | Lieu secret. Pas le texte des rencontres. |
| `Colline_de_la_Sentinelle.html` | 60 / 0 | La Colline de la Sentinelle est la zone de rue centrale de Dunwich. C'est sur ses hauteurs qu'apparaît L'Horreur de Dunwich lorsque l' Échelle de l'Horreur de Dunwich est remplie. Les … |
| `Dunwich.html` | 213 / 4 | IMPORTANT: Les cartes et règles du jeu qui se réfèrent à "Arkham" s'appliquent aussi aux lieux et zones de rues de Dunwich. Par exemple: Une carte environnement qui ajoute … |
| `Département_Scientifique.html` | 131 / 0 | Lieu instable ; capacité spéciale « la Dissection » expliquée ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Echelle_De_L'Horreur_A_Dunwich.html` | 151 / 1 | Au début du jeu, l'Échelle de l'Horreur de Dunwich est vide (Il n'y a aucun Marqueur Horreur de Dunwich ). Divers événements dans le jeu ajouteront des jetons sur cette Échelle dont … |
| `Echelle_Du_Destin.html` | 84 / 0 | Cette échelle montre la progression du Réveil du Grand Ancien. On la trouve sur le bas de la feuille du Grand Ancien. On y pose un Pion Destin à chaque fois … |
| `Failles.html` | 501 / 4 | Comme les barrières dimensionnelles proches de Kingsport sont très fines, la région est remplie de failles dimensionnelles. Ce sont des déchirures mobiles dans la structure de l'univers d'où émergent des monstres si … |
| `Gare.html` | 109 / 0 | Lieu stable ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Grand_Hall_De_Celeano.html` | 26 / 0 | Autre Monde : couleurs de rencontre vert et bleu. Pas le texte des rencontres. |
| `Hôpital_Sainte_Marie.html` | 127 / 0 | Lieu stable ; capacité spéciale « les soins médicaux » expliquée ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Ile_Inexplorée.html` | 93 / 0 | Lieu instable ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Innsmouth.html` | 201 / 4 | IMPORTANT: Les cartes et règles du jeu qui se réfèrent à "Arkham" s'appliquent aussi aux lieux et zones de rues de Innsmouth. Par exemple: Une carte environnement qui ajoute … |
| `Instable.html` | 55 / 0 | Les Monstres et les Portails peuvent apparaître en ces lieux, parfois comme résultat de rencontres à Arkham. Ces lieux doivent être approchés avec prudence ! Arkham a lieux 11 instables Dunwich … |
| `Journal.html` | 69 / 0 | Lieu stable ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Kadath_L'Inconnue.html` | 214 / 0 | Autre Monde : citation Lovecraft ; couleurs de rencontre rouge et jaune. Pas le texte des rencontres. |
| `Kingsport.html` | 209 / 5 | IMPORTANT: Les cartes et règles du jeu qui se réfèrent à "Arkham" s'appliquent aussi aux lieux et zones de rues de Kingsport. Par exemple: Une carte environnement qui ajoute … |
| `L'Innommable.html` | 166 / 0 | citation Lovecraft ; Lieu instable ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Le_Monde_Inférieur.html` | 51 / 0 | Autre Monde : couleurs de rencontre bleu et vert. Pas le texte des rencontres. |
| `Les_Abysses.html` | 24 / 0 | Autre Monde : couleurs de rencontre rouge et bleu. Pas le texte des rencontres. |
| `Les_Contrées_Du_Rêve.html` | 82 / 0 | Autre Monde : citation Lovecraft ; couleurs de rencontre rouge, jaune, vert et bleu. Pas le texte des rencontres. |
| `Lieu.html` | 291 / 2 | Les Lieux sont représentés par des illustrations circulaires sur les Plateaux De Jeu dans les villes où les Investigateurs peuvent faire des Rencontres ou activer une Capacité Spéciale De Lieu. Un … |
| `Lieu_aquatique.html` | 39 / 0 | Un Lieu Aquatique est marqué par un Symbole De Vague sur sa case Lieu ou par un Marqueur aquatique. Les Monstres Aquatiques peuvent se déplacer aisément entre ces différents Lieux. Aquatique |
| `Limites_de_la_Ville.html` | 67 / 0 | Les Limites de la Ville sont des Zones Spéciales du plateau de jeu d' Arkham. Elles sont considérées comme "à Arkham", mais pas "dans Arkham". Il s'agit des trois zones suivantes … |
| `Loge_du_Crépuscule_d'Argent.html` | 102 / 0 | Lieu instable ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Magasin.html` | 142 / 0 | Lieu stable ; capacité spéciale « l'achat d'objets communs » expliquée ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Maison_de_la_Sorcière.html` | 162 / 0 | citation Lovecraft ; Lieu instable ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Niveau_De_Terreur.html` | 321 / 1 | L'Échelle de Terreur indique le niveau actuel de terreur des habitants d'Arkham, reflétant leur état mental. Le Niveau De Terreur ne peut JAMAIS baisser ! Au début du jeu, l'Echelle De Terreur … |
| `Pension_de_Ma.html` | 189 / 0 | Lieu stable ; capacité spéciale « Recruter » expliquée ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Perdu_Dans_Le_Temps_Et_L'Espace.html` | 116 / 1 | C'est une zone spéciale du jeu. Un Investigateur Perdu Dans Le Temps Et L'Espace est immédiatement placé dans cette case. Il est alors Retardé et il redresse son marqueur et perd son … |
| `Piste_des_failles.html` | 177 / 1 | Chaque piste de faille est constituée de quatre cases divisées en deux groupes, chacun ayant un motif de déplacement de monstre différent (comme sur les cartes Mythe) qui lui est associé. Chaque … |
| `Plateau_De_Jeu.html` | 102 / 4 | Même contenu que `Plateaux_de_Jeu.html` (déjà inventorié, §2). |
| `Plateau_De_Leng.html` | 107 / 0 | Autre Monde : citation Lovecraft ; couleurs de rencontre rouge et vert. Pas le texte des rencontres. |
| `Portail.html` | 1474 / 63 | Même contenu que `Portails.html` (déjà inventorié, §2). |
| `Portail_Dévorant.html` | 23 / 0 | Quand un portail avec ce symbole s'ouvre sur un lieu occupé par un investigateur, ce dernier est dévoré. |
| `Portail_Errant.html` | 79 / 0 | Considérez ce portail comme un monstre pouvant se déplacer: Si un Portail Errant se déplace sur un investigateur, ce dernier est aspiré comme lorsque le portail s'ouvre au-dessous de ce dernier. … |
| `Portail_Monstrueux.html` | 34 / 0 | Quand un investigateur rate son test pour fermer ce type de portail, un monstre apparait (dans la périphérie si le nombre de monstre maximum a Arkham est déjà atteint). |
| `Portail_Sanglant.html` | 22 / 0 | Quand un investigateur rate son test pour fermer ce type de portail, ce dernier perd 1 Résistance. |
| `Portail_de_la_Folie.html` | 24 / 0 | Quand un investigateur avec rate son test pour fermer ce type de portail, ce dernier perd 1 Santé Mentale. |
| `Portail_du_Destin.html` | 27 / 0 | Quand un portail avec ce symbole s'ouvre sur un lieu occupé par un investigateur, on ajoute un pion sur l'échelle du destin. |
| `Portail_sans_fin.html` | 22 / 0 | Ce portail ne peux pas être réclamé comme trophée. Il est remis dans la pile des Portails. |
| `Poste_de_Police.html` | 259 / 3 | Lieu stable ; capacité spéciale « l'adjoint » expliquée ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Quais.html` | 136 / 0 | Lieu stable ; capacité spéciale « le personnage louche » expliquée ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `R'lyeh.html` | 141 / 0 | Autre Monde : citation Lovecraft ; couleurs de rencontre rouge et jaune. Pas le texte des rencontres. |
| `Relais_Routier_de_Hibb.html` | 109 / 0 | Lieu instable ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Rencontres.html` | 62 / 0 | Les Rencontres sont les événéments qui arriveront aux Investigateurs lorsqu'ils se trouvent dans les Lieux d'Arkham ou d'autre villes ou dans un Autre Monde. Elles se produisent via les Cartes Lieu … |
| `Restaurant_de_Velma.html` | 76 / 0 | Lieu stable ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Société_des_Historiens.html` | 81 / 0 | Lieu instable ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Square_de_l'Indépendance.html` | 88 / 0 | Lieu instable ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Stable.html` | 51 / 0 | Les Monstres et les Portails n’apparaissent jamais à ces endroits, bien qu’il soit possible pour les monstres de s’y déplacer dans certaines circonstances. Arkham a 15 Lieux stables Dunwich a 4 lieux … |
| `Symbole_De_Rencontre.html` | 238 / 11 | A Arkham Les Symboles De Rencontre se trouve sur les case de Lieu en bas à droite et à gauche du rond de la case. Ils préviennent les joueurs des événements qui … |
| `Symbole_De_Train.html` | 56 / 1 | Le Symbole De Train d'un Lieu représente la possibilité pour un Investigateur de Voyager Entre Les Villes à partir de ce Lieu ! Ce symbole est présent en bas au milieu du … |
| `Un_Autre_Temps.html` | 25 / 0 | Autre Monde : couleurs de rencontre rouge et vert. Pas le texte des rencontres. |
| `Une_Autre_Dimension.html` | 29 / 0 | Autre Monde : couleurs de rencontre rouge, jaune, vert et bleu. Pas le texte des rencontres. |
| `Vieille_Echoppe_De_Magie.html` | 122 / 0 | Lieu stable ; capacité spéciale « les cours de magie » expliquée ; PNJ rencontrés (noms). Pas le texte des rencontres. |
| `Vortex.html` | 173 / 1 | On trouve ces cases spéciales sur le plateau de Dunwich ou d' Innsmouth. Les investigateurs ne peuvent en aucun cas aller dans les Vortex (même à l'aide d'un objet ou en … |
| `Voyager_Entre_Les_Villes.html` | 122 / 0 | Pour se déplacer d' Arkham vers d'autres villes (Comme Dunwich ), l'Investigateur doit se trouver à la Gare d'Arkham ou dans un Lieu d'une autre ville possédant le Symbole De Train. … |
| `Yuggoth.html` | 23 / 0 | Autre Monde : couleurs de rencontre jaune et bleu. Pas le texte des rencontres. |
| `Zone_De_Rues.html` | 45 / 0 | Les Zones De Rue sont représentées sur les Plateaux De Jeu par des cases rectangulaires, une par Quartier de chaque ville (par exemple, Université Miskatonic à Arkham, Pays Des Bois Noirs … |
| `Église_Méridionale.html` | 114 / 0 | Lieu stable ; capacité spéciale « Bénédiction » expliquée ; PNJ rencontrés (noms). Pas le texte des rencontres. |

**Doublons** (texte identique) — 24 : `Asile_D'Arkham~b5fb1ab6.html` = `Asile_d'Arkham.html`, `Banque_D'Arkham~711fd33a.html` = `Banque_d'Arkham.html`, `Boutique_De_Souvenirs~d6c9984d.html` = `Boutique_de_Souvenirs.html`, `Boutique_de_souvenirs~2074e58d.html` = `Boutique_de_Souvenirs.html`, `Echelle_De_Terreur.html` = `Niveau_De_Terreur.html`, `La_Vieille_Échoppe_de_Magie.html` = `Vieille_Echoppe_De_Magie.html`, `Les_Quais.html` = `Quais.html`, `Lieu_Aquatique~c60e1fca.html` = `Lieu_aquatique.html`, `Lieux.html` = `Lieu.html`, `Loge_Du_Crépuscule_D'Argent~e5411efb.html` = `Loge_du_Crépuscule_d'Argent.html`, `Maison_De_La_Sorcière~751b426d.html` = `Maison_de_la_Sorcière.html`, `Pension_De_Ma~e667a7d0.html` = `Pension_de_Ma.html`, `Perdu_dans_le_Temps_et_l'Espace~ac5fa0af.html` = `Perdu_Dans_Le_Temps_Et_L'Espace.html`, `Plateau_de_Leng~0c26387f.html` = `Plateau_De_Leng.html`, `Plateaux_De_Jeu~02545f64.html` = `Plateau_De_Jeu.html`, `Poste_De_Police~7140cf89.html` = `Poste_de_Police.html`, `Relais_Routier_De_Hibb~4254b3e7.html` = `Relais_Routier_de_Hibb.html`, `Restaurant_De_Velma~684da248.html` = `Restaurant_de_Velma.html`, `Société_Des_Historiens~6005e601.html` = `Société_des_Historiens.html`, `Square_De_L'Indépendance~56015d17.html` = `Square_de_l'Indépendance.html`, `Symboles_De_Rencontre.html` = `Symbole_De_Rencontre.html`, `Échelle_de_Terreur.html` = `Niveau_De_Terreur.html`, `Échelle_de_l'Horreur_de_Dunwich.html` = `Echelle_De_L'Horreur_A_Dunwich.html`, `Île_Inexplorée.html` = `Ile_Inexplorée.html`.


### 6.5 Quartiers (galeries de cartes Lieu)

| Fichier | mots / img | Contenu |
|---|---|---|
| `Central_Hill.html` | 82 / 60 | Galerie des **cartes Lieu** du quartier : 34 cartes (codes `CLKCH01`…), 60 images non téléchargées ; **texte des rencontres absent**. |
| `Centre-Ville.html` | 67 / 76 | Galerie des **cartes Lieu** du quartier : 39 cartes (codes `CLACV02`…), 76 images non téléchargées ; **texte des rencontres absent**. |
| `Cote_d'Innsmouth.html` | 64 / 60 | Galerie des **cartes Lieu** du quartier : 34 cartes (codes `I_CI_1`…), 60 images non téléchargées ; **texte des rencontres absent**. |
| `Côte_Sud.html` | 82 / 60 | Galerie des **cartes Lieu** du quartier : 34 cartes (codes `CLKCS01`…), 60 images non téléchargées ; **texte des rencontres absent**. |
| `French_Hill.html` | 64 / 68 | Galerie des **cartes Lieu** du quartier : 35 cartes (codes `CLAFH10`…), 68 images non téléchargées ; **texte des rencontres absent**. |
| `Lande_Dévastée.html` | 85 / 60 | Galerie des **cartes Lieu** du quartier : 34 cartes (codes `CLDLD01`…), 60 images non téléchargées ; **texte des rencontres absent**. |
| `Pays_des_Bois_Noirs.html` | 83 / 60 | Galerie des **cartes Lieu** du quartier : 34 cartes (codes `CLDPBN01`…), 60 images non téléchargées ; **texte des rencontres absent**. |
| `Pic_de_Kingsport.html` | 171 / 60 | Intro sur le quartier (texte). Galerie des **cartes Lieu** du quartier : 34 cartes (codes `CLKPK01`…), 60 images non téléchargées ; **texte des rencontres absent**. |
| `Port.html` | 82 / 60 | Galerie des **cartes Lieu** du quartier : 34 cartes (codes `CLKPO01`…), 60 images non téléchargées ; **texte des rencontres absent**. |
| `Quartier_Est.html` | 68 / 76 | Galerie des **cartes Lieu** du quartier : 39 cartes (codes `CLAQE03`…), 76 images non téléchargées ; **texte des rencontres absent**. |
| `Quartier_Marchand.html` | 67 / 76 | Galerie des **cartes Lieu** du quartier : 39 cartes (codes `CLAQM01`…), 76 images non téléchargées ; **texte des rencontres absent**. |
| `Quartier_Nord.html` | 112 / 76 | Intro sur le quartier (texte). Galerie des **cartes Lieu** du quartier : 39 cartes (codes `CLAQN02`…), 76 images non téléchargées ; **texte des rencontres absent**. |
| `Quartier_Résidentiel.html` | 64 / 68 | Galerie des **cartes Lieu** du quartier : 35 cartes (codes `CLAQRE05`…), 68 images non téléchargées ; **texte des rencontres absent**. |
| `Quartier_Sud.html` | 64 / 68 | Galerie des **cartes Lieu** du quartier : 35 cartes (codes `CLAQS02`…), 68 images non téléchargées ; **texte des rencontres absent**. |
| `Quartier_de_la_Fabrique.html` | 64 / 60 | Galerie des **cartes Lieu** du quartier : 34 cartes (codes `I_QDF_1`…), 60 images non téléchargées ; **texte des rencontres absent**. |
| `Quartier_de_la_Rivière.html` | 63 / 68 | Galerie des **cartes Lieu** du quartier : 35 cartes (codes `CLAQRI01`…), 68 images non téléchargées ; **texte des rencontres absent**. |
| `Terrains_Communaux.html` | 82 / 60 | Galerie des **cartes Lieu** du quartier : 34 cartes (codes `CLDTC01`…), 60 images non téléchargées ; **texte des rencontres absent**. |
| `Université_Miskatonic.html` | 131 / 68 | Intro sur le quartier (texte). Galerie des **cartes Lieu** du quartier : 35 cartes (codes `CLAUV23`…), 68 images non téléchargées ; **texte des rencontres absent**. |
| `Verte_Eglise.html` | 64 / 60 | Galerie des **cartes Lieu** du quartier : 34 cartes (codes `I_VE_1`…), 60 images non téléchargées ; **texte des rencontres absent**. |

**Doublons** (texte identique) — 5 : `Le_Port.html` = `Port.html`, `Pays_Des_Bois_Noirs~d434c215.html` = `Pays_des_Bois_Noirs.html`, `Pic_De_Kingsport~71f9c928.html` = `Pic_de_Kingsport.html`, `Quartier_De_La_Rivière~5d321f1b.html` = `Quartier_de_la_Rivière.html`, `Verte_Église.html` = `Verte_Eglise.html`.


### 6.6 Listes de données (tableaux)

| Fichier | mots / img | Contenu |
|---|---|---|
| `Liste_Des_Alliés_Par_Extension.html` | 928 / 23 | **Tableau texte des alliés** : nom, extension, lieu de résidence, condition de recrutement (test), bonus et objets apportés, gain si l'allié n'est plus disponible. |
| `Liste_Des_Monstres_Par_Extension.html` | 364 / 143 | Liste des monstres par extension (base 60, Dunwich 28, Roi en Jaune 3, Kingsport 28, Chèvre Noire 9, Innsmouth 32…), noms + images. |
| `Liste_Des_Monstres_Par_Symbole.html` | 323 / 122 | Liste des monstres par symbole dimensionnel (carré, croix, cercle…), noms + images. |
| `Liste_Des_Sorts.html` | 545 / 37 | **Tableau texte des sorts** : nom, extension, coût en SAN, modificateur de lancement, nombre de mains, phase d'utilisation, type d'effet. Pas le texte complet des cartes. |
| `Liste_des_Monstres_ayant_cinq_points_point_de_Force.html` | 40 / 2 | Le seul monstre de Force 5. |
| `Liste_des_Monstres_ayant_deux_points_de_Force.html` | 217 / 52 | Monstres de Force 2, classés par dangerosité. Noms + images. |
| `Liste_des_Monstres_ayant_quatre_points_point_de_Force.html` | 54 / 3 | Monstres de Force 4. Noms + images. |
| `Liste_des_Monstres_ayant_trois_points_de_Force.html` | 222 / 35 | Monstres de Force 3, classés par dangerosité. Noms + images. |
| `Liste_des_Monstres_ayant_un_point_de_Force.html` | 194 / 21 | Monstres de Force 1, classés par dangerosité (somme des modificateurs). Noms + images. |
| `Liste_détaillée_des_Investigateurs.html` | 1387 / 32 | **Tableau texte des 48 investigateurs** : nom, profession, lieu de départ, SAN, RES, $, pions Indice, nombre d'objets communs / uniques / sorts / compétences tirés au départ, une valeur par compétence (Vitesse, Discrétion, Combat, Volonté, Savoir, Chance ; pas les 4 crans des curseurs), résumé de la capacité spéciale, extension. **Source de données exploitable.** |
| `Liste_détaillée_des_Monstres.html` | 1872 / 157 | **Tableau texte de ~93 monstres** : nom, extension, nombre d'exemplaires, symbole dimensionnel, type de mouvement, Conscience, test et dégâts d'Horreur, test et dégâts de Combat, Force (endurance), capacités (Inépuisable, Irrésistible, Cauchemardesque, résistance/immunité physique et magique, Insaisissable, Mort-vivant, Embuscade), autres effets. **Source de données exploitable.** |


### 6.7 Investigateurs (une page par investigateur)

| Fichier | mots / img | Contenu |
|---|---|---|
| `Amanda_Sharpe.html` | 168 / 1 | Possessions Fixes Aucune. Son Histoire Amanda est étudiante à l'Université Mikatonic depuis 2 ans. Le mois dernier, en allant discuter avec l'un de ses professeurs, elle aperçu une peinture dans le hall … |
| `Bob_Jenkins.html` | 181 / 1 | Possessions Fixes Aucune. Son Histoire En tant que représentant, Bob est toujours sur les routes. Mais hier, il a vu quelque chose qui' l'a poussé à rater son train pour rester à … |
| `Carolyn_Fern.html` | 124 / 1 | Possessions Fixes Aucune. Son Histoire Carolyn travaille au sanatorium de Providence depuis un an. Ces six derniers mois, elle a étudié les rêves de ses patients sous hypnose. Un patient, en particulier, … |
| `Charlie_Kane.html` | 134 / 1 | Possessions Fixes Aucune. Erratas & Clarifications Le pouvoir "Relations" ne permet pas de récupérer les alliés qui vont dans la boite durant la partie suite à l'élévation du niveau de terreur. Il … |
| `Daisy_Walker.html` | 16 / 2 | Possessions Fixes Livre D'Ivon Erratas & Clarifications Aucun. Investigateurs |
| `Diana_Stanley.html` | 21 / 2 | Possessions Fixes Membre De La Loge Du Crépuscule D'Argent Erratas & Clarifications Aucun. Investigateurs |
| `Finn_Edwards.html` | 18 / 2 | Possessions Fixes Prêt Bancaire Whisky Erratas & Clarifications Aucun. Investigateurs |
| `Harvey_Walters.html` | 179 / 1 | Possessions Fixes Aucune. Son Histoire Harvey Walters est un professeur invité à l'Université Miskatonic. Avec ses doctorats en Histoire et en Archéologie, il a découvert plusieurs artefacts intéressants et a un peu … |
| `Jacqueline_Fine.html` | 178 / 4 | Possessions Fixes Joyaux Enchantés $7 2 pions Indice Son Histoire Toutes les nuits, Jacqueline voit l'avenir. Ses rêves sont remplis de choses qui ne sont pas encore arrivées - généralement de terribles … |
| `Jenny_Barnes.html` | 162 / 1 | Possessions Fixes Aucune. Son Histoire Il y a plusieurs mois, Jenny visitait Paris quand elle reçut une lettre de sa sœur, Isabelle. La lettre d'Isabelle était décousue, elle parlait d'hommes en manteaux … |
| `Jim_Culver.html` | 16 / 2 | Possessions Fixes Trompette Dorée Erratas & Clarifications Aucun. Investigateurs |
| `Joe_Diamond.html` | 173 / 4 | Possessions Fixes Automatique 45 $8 3 pions Indice Son Histoire Le boulot semblait simple - récupérer une statue au Musée de Providence et la livrer à un gars de la Loge du … |
| `Leo_Anderson.html` | 231 / 3 | Possessions Fixes $5 2 pions Indice Son Histoire Anderson est parti pour le Yucatan avec sept hommes. Seuls deux en sont revenus - plus ou moins. Techniquement, trois hommes sont sortis en … |
| `Lily_Chen.html` | 16 / 2 | Possessions Fixes Arts Martiaux Erratas & Clarifications Aucun. Investigateurs |
| `Luke_Robinson.html` | 17 / 2 | Possessions Fixes Boîte A Portail Erratas & Clarifications Aucun. Investigateurs |
| `Marie_Lambeau.html` | 32 / 3 | Possessions Fixes Couteau Enchanté Voix De Ra Erratas & Clarifications Son pouvoir d'enlever un Pion Destin peut être utilisé pendant la Bataille Finale. Investigateurs |
| `Mark_Harrigan.html` | 18 / 3 | Possessions Fixes Lance Flammes Cocktail Molotov Erratas & Clarifications Aucun. Investigateurs |
| `Michael_McGlen.html` | 59 / 3 | Possessions Fixes Dynamite Mitraillette Erratas & Clarifications Q: Est-ce que les capacités qui permettent de réduire de 1 toutes les pertes de Santé Mentale ou de Résistance d'un Investigateur s'appliquent aux … |
| `Minh_Thi_Phan.html` | 18 / 2 | Possessions Fixes Le Roi en Jaune Erratas & Clarifications Aucun. Investigateurs |
| `Monterey_Jack.html` | 205 / 3 | Possessions Fixes Fouet Révolver.38 Son Histoire Monterey est depuis des années un globe-trotter et un aventurier à la recherche de trésors. Suivant les traces de son père, il a toujours essayé … |
| `Norman_Withers.html` | 18 / 1 | Possessions Fixes Trouver le Portail Erratas & Clarifications Aucun. Investigateurs |
| `Silas_Marsh.html` | 18 / 1 | Possessions Fixes Sabre de Cavalerie Erratas & Clarifications Aucun. Investigateurs |
| `Soeur_Marie.html` | 125 / 4 | Possessions Fixes Bénédiction Croix Eau Bénite Erratas & Clarifications D'après les règles officielles, elle doit tester et peut perdre sa bénédiction normalement. Mais plusieurs variantes sont jouées pour lui permettre de conserver … |
| `Tommy_Muldoon.html` | 18 / 2 | Possessions Fixes Fusil Plan d'Arkham Erratas & Clarifications Aucun. Investigateurs |
| `Trish_Scarborough.html` | 253 / 4 | Possessions Fixes Étui à Cigarette Porte-bonheur $4 2 pions Indice Son Histoire Des années auparavant, cela lui avait semblé si surréaliste d'avoir été approchée par un recruteur après une de ses rencontres … |
| `Wendy_Adams.html` | 17 / 2 | Possessions Fixes Signe Des Anciens Erratas & Clarifications Aucun. Investigateurs |
| `Wilson_Richards.html` | 47 / 4 | Possessions Fixes Fusil à Pompe Motocyclette Whisky Erratas & Clarifications Sa concentration est infinie pour déplacer les marqueurs mais on considère qu'elle est de 4 pour certaines cartes ou pouvoir (gains ou … |
| `Zoey_Samaras.html` | 17 / 2 | Possessions Fixes Croix Couteau Erratas & Clarifications Aucun. Investigateurs |

**Fiches sans contenu** (image de la carte non téléchargée + « Erratas : aucun ») — 20 : `"Skids"_O'Toole.html`, `Agnes_Baker.html`, `Akachi_Onyele.html`, `Ashcan_Pete.html`, `Darrell_Simmons.html`, `Dexter_Drake.html`, `George_Barnaby.html`, `Gloria_Goldberg.html`, `Hank_Samson.html`, `Kate_Winthrop.html`, `Lola_Hayes.html`, `Mandy_Thompson.html`, `Patrice_Hathaway.html`, `Rex_Murphy.html`, `Rita_Young.html`, `Roland_Banks.html`, `Tony_Morgan.html`, `Ursula_Downs.html`, `Vincent_Lee.html`, `William_Yorick.html`.


### 6.8 Grands Anciens, Hérauts, Gardiens

| Fichier | mots / img | Contenu |
|---|---|---|
| `Abhoth.html` | 194 / 2 | Adorateurs Aucun ! Il faut même retirer les Cultistes de la Tasse Des Monstres car Abhoth ne possède aucun culte humain. A la place ses abominations sèment la terreur ! Progéniture D’Abhoth … |
| `Atlach-Nacha.html` | 18 / 2 | Adorateurs Araignée De Leng D'Atlach-Nacha Erratas & Clarifications Aucun. Grands Anciens |
| `Azathoth.html` | 93 / 3 | "Dans cet ultime abîme du plus grand désordre où les chimères et les blasphèmes sont le centre de toute infinité, l'illimité Azathoth, ce sultan des démons dont aucune bouche n'ose dire le … |
| `Bastet.html` | 19 / 1 | Bastet - Bien-aimés De Bastet Grands Anciens Investigateurs Hérauts Gardiens |
| `Chaugnar_Faugn.html` | 27 / 1 | Adorateurs Image manquante Tcho-tchos de Chaugnar Faugn Image manquante Prêtres Tcho-tchos de Chaugnar Faugn Erratas & Clarifications Aucun. Grands Anciens |
| `Cthugha.html` | 21 / 1 | Adorateurs Image manquante Vampire De Feu de Cthugha Erratas & Clarifications Aucun. Grands Anciens |
| `Cthulhu.html` | 183 / 3 | "Un monstre à la silhouette vaguement anthropoïde, avec une tête de pieuvre dont la face n'aurait été qu'une masse de tentacules, un corps écailleux d'une grande élasticité, des griffes prodigieuses aux pattes … |
| `Gardien_Bastet.html` | 19 / 1 | Bastet - Bien-aimés De Bastet Grands Anciens Investigateurs Hérauts Gardiens |
| `Gardien_Nodens.html` | 19 / 1 | Nodens - Bénédiction de Nodens Grands Anciens Investigateurs Hérauts Gardiens |
| `Ghatanotoa.html` | 24 / 1 | Adorateurs Image manquante Migo de Ghatanothoa Image manquante Lloigors de Ghatanothoa Erratas & Clarifications Aucun. Grands Anciens |
| `Glaaki.html` | 49 / 3 | Adorateurs Cultiste De Glaaki Serviteur De Glaaki Erratas & Clarifications Lors de la bataille finale, un Serviteur De Glaaki pas encore en jeu peut arriver (et il faut activer sa capacité spéciale) … |
| `Grand_Ancien.html` | 873 / 45 | Même contenu que `Grands_Anciens.html` (déjà inventorié, §2). |
| `Hastur.html` | 127 / 3 | "Paysage tout à fait extraterrestre... A l'arrière plan, un lac profond. Hali ? Au bout de cinq minutes, l'eau commence à frisonner en cercles concentriques, comme si quelque chose montait à la … |
| `Heraut_Rodeur.html` | 43 / 1 | L'utilisation de ce Héraut entraine l'utilisation des cartes Pacte Maléfique ( Pacte de Sang, Pacte de l'Âme et Allié Forcé ), des Cartes Dette, des Pion Pouvoir et des Nouveaux … |
| `Hypnos.html` | 101 / 1 | [...] une tête de marbre magnifique, digne des plus grands sculpteurs hellènes, d'une jeunesse intemporelle. Le beau visage porte une barbe fournie et ondulée, des lèvres souriantes, des cheveux longs et bouclés, … |
| `Héraut.html` | 121 / 19 | Même contenu que `Hérauts.html` (déjà inventorié, §2). |
| `Héraut_Le_Pharaon_Noir.html` | 38 / 1 | Pour que le Héraut soit le plus dangereux, nous vous recommandons d’utiliser l'extension de la Malédiction du Pharaon Noir et Nyarlathotep en tant que Grand Ancien... Bonne chance:) ! Hérauts |
| `Héraut_Le_Roi_En_Jaune.html` | 66 / 1 | Le Roi En Jaune est un être puissant et diabolique, dont l'objectif est de préparer la venue du Grand Ancien. Cela augmente la difficulté du jeu et il faut utiliser le … |
| `Ithaqua.html` | 241 / 2 | "Les astres avaient disparu... le grand nuage qui avait obscurci le ciel rappelait étrangement la silhouette d'un homme immense. Et... là où aurait dû être le sommet du "nuage", là où aurait … |
| `Nodens.html` | 19 / 1 | Nodens - Bénédiction de Nodens Grands Anciens Investigateurs Hérauts Gardiens |
| `Nyarlathotep.html` | 90 / 2 | "Une grande silhouette élancée ayant ce jeune visage propre aux antiques pharaons et portant des robes prismatiques et un diadème d'or qui semblait luire comme d'une lumière intérieure... la fascination d'un dieu … |
| `Nyogtha.html` | 29 / 1 | Adorateurs Image manquante Sorcière de Nyogtha Image manquante Goule de Nyogtha Image manquante Sorcier de Nyogtha Erratas & Clarifications Aucun Grands Anciens |
| `Quatchil-Uttaus.html` | 102 / 1 | Adorateurs Image manquante Sorcier de Quachil Uttaus Règle Spéciale Lorsque que Quatchil-Uttaus est le Grand Ancien, le premier joueur ne change pas tant que le précédent premier joueur est encore en vie. … |
| `Rhan_Tegoth.html` | 19 / 1 | Adorateurs Image manquante Gnoph-Kehs de Rhan-Tegoth Erratas & Clarifications Aucun. Grands Anciens |
| `Shub-Niggurath.html` | 56 / 2 | "Iä ! Iä ! Shub-Niggurath ! Le Bouc Noir des Forêts aux Milles Chevreaux !" H.P.Lovecraft, "Celui Qui Chuchotait dans les Ténèbres". Adorateurs Sombre Rejeton De Shub-Niggurath Erratas & Clarifications Pour son … |
| `Shudde_M'ell.html` | 58 / 2 | Adorateurs Cthonien De Shudde M'ell Erratas & Clarifications Les 7 jetons décombres sont placées face cachée à côté du plateau. Quand il détruit un lieu avec un Pion Décombres, un investigateur … |
| `Tsathoggua.html` | 125 / 2 | Adorateurs Larve Amorphe De Tsathoggua Erratas & Clarifications Les capacités spéciales des Lieux indiqués par la capacité Malédiction de Tsathoggua sont celles imprimées sur le plateau. Lors de la première édition de … |
| `Yibb-Tstll.html` | 21 / 2 | Adorateurs Maigre Bête De La Nuit De Yibb-Tstll Erratas & Clarifications Aucun. Grands Anciens |
| `Yig.html` | 59 / 2 | "Le père à demi humain des reptiles... le dieu serpent des tribus des plaines centrales - qui se trouve probablement à l'origine de Quetzalcoatl oiu Kukulcan -, était un démon étrange à … |
| `Yog-Sothoth.html` | 141 / 2 | "De grands globes d'agglutinant vers l'ouverture... l'éclatement des globes les plus proches, et la chair protoplasmique qui refluait, noire, pour former cette horreur fantastique et hideuse de l'espace extérieur... dont le masque … |
| `Zhar.html` | 25 / 1 | Adorateurs Image manquante Tcho-tchos de Zhar Image manquante Prêtres Tcho-tchos de Zhar Erratas & Clarifications Aucun. Grands Anciens |

**Fiches sans contenu** (image de la carte non téléchargée + « Erratas : aucun ») — 4 : `Atlach-Nacha_1.html`, `Bokrug.html`, `Eihort.html`, `Y'Golonac.html`.

**Pages vides** (image seule, non téléchargée) — 6 : `Heraut_Dagon.html`, `Heraut_Hydra.html`, `Héraut_Chèvre_Noire_Des_Bois.html`, `Héraut_Ghroth.html`, `Héraut_L'Horreur_de_Dunwich.html`, `Héraut_Tulzscha.html`.

**Doublons** (texte identique) — 1 : `Gardien_Hypnos.html` = `Hypnos.html`.


### 6.9 Objets communs

| Fichier | mots / img | Contenu |
|---|---|---|
| `Athame.html` | 74 / 2 | Cet Objet apparait dans l' Extension Le Roi En Jaune. C'est une Arme Physique. C'est une Arme A Une Main. C'est une Arme A Bonus. Donne +1 aux … |
| `Automatique_45.html` | 61 / 1 | Cet Objet apparait dans l' Horreur à Arkham. C'est une Arme Physique. C'est une Arme A Une Main. Donne +4 aux Tests De Combat. Son prix au Magasin … |
| `Bombe_A_Retardement.html` | 74 / 2 | **Erreur du wiki** : contient le texte de `Athame.html`. |
| `Brasier_Des_Ames.html` | 49 / 2 | **Erreur du wiki** : contient le texte de `Pistolet_De_Détresse.html`. |
| `Carabine.html` | 59 / 2 | Cet Objet apparait dans l' Extension L'Horreur De Dunwich C'est une Arme Physique. C'est une Arme A Une Main. C'est une Arme à recharger (gratuitement) en Phase d'entretien Donne +5 … |
| `Carte_De_Chemins_Anciens.html` | 84 / 2 | Cet Objet apparait dans l' Extension Le Roi En Jaune. C'est un Objet commun à usage unique permettant sous certaines condition les effets néfastes une carte du Mythe de type Environnement … |
| `Carte_De_Presse.html` | 53 / 2 | Cet Objet apparait dans l' Extension Le Roi En Jaune. C'est un Objet commun permettant de collecter 1 Pion indice supplémentaire, à chaque fois qu'un Investigateur à la possibilité d'en acquérir. … |
| `Clé_Du_Coffre.html` | 54 / 2 | Cet Objet apparait dans l' Extension Le Roi En Jaune. C'est un Objet commun à usage unique qui permet sous certaines conditions de récupérer 1 Objet unique ou 2 Pions indices … |
| `Cocktail_Molotov.html` | 61 / 2 | Cet Objet apparait dans l' Extension L'Horreur De Dunwich C'est une Arme Physique. C'est une Arme A Une Main. Donne +6 aux Tests De Combat. Son prix au Magasin … |
| `Couteau.html` | 119 / 1 | "Elle était morte vers sept heures du soir, et son mari, perdant la tête, avait essayé de tuer West, à qui il reprochait de ne pas l'avoir sauvée. Des amis l'avaient retenu … |
| `Croix.html` | 146 / 1 | "Sentant la chaîne du modeste crucifix lui scier le cou, il se demanda dans le danger qui le menaçait quel effet la vue de l'objet lui-même produirait sur l'infernale créature. Elle était … |
| `Derringer.html` | 53 / 1 | Cet Objet apparait dans l' Horreur à Arkham. C'est une Arme Physique. C'est une Arme A Une Main. Donne +2 aux Tests De Combat. Cet objet fait partie … |
| `Dynamite.html` | 68 / 1 | Cet Objet apparait dans l' Horreur à Arkham. C'est une Arme Physique. C'est une Arme A Deux Mains. C'est une Arme A Usage Unique. Donne +8 aux Tests … |
| `Eau_Bénite.html` | 71 / 1 | Cet Objet apparait dans l' Horreur à Arkham. C'est une Arme Magique. C'est une Arme A Deux Mains. C'est une Arme A Usage Unique. Donne +6 aux Tests … |
| `Enquête_Journalistique.html` | 51 / 1 | Cet Objet apparait dans l' Horreur de Dunwich. C'est une Tâche permettant de regagner dollars et pions indices. Elle est relativement facile à réaliser car deux des trois Lieux à visiter … |
| `Fouet.html` | 84 / 1 | Cet Objet apparait dans le jeu de base. C'est une Arme Physique. C'est une Arme A Une Main. C'est une Arme A Bonus Donne +1 aux Tests De Combat … |
| `Fusil.html` | 58 / 1 | Cet Objet apparait dans l' Horreur à Arkham. C'est une Arme Physique C'est une Arme A Deux Mains Donne +5 aux Tests De Combat Son prix au Magasin est de 6 … |
| `Fusil_A_Eléphant.html` | 62 / 2 | Cet Objet apparait dans l' Extension L'Horreur De Kingsport C'est une Arme Physique. C'est une Arme A Deux Mains. C'est une Arme à recharger (pour 2 dollars) en Phase d'entretien … |
| `Fusil_à_Pompe.html` | 101 / 1 | "...Tandis que Morgan sortait de son étui le fusil de chasse de gros calibre auquel il faisait confiance malgré ce qu'affirmait son collègue sur l'inutilité des armes matérielles." H.P.Lovecraft, "L'Abomination de Dunwich". … |
| `Hache.html` | 81 / 1 | Cet Objet apparait dans l' Horreur à Arkham. C'est une Arme Physique C'est une Arme A Une Main Donne +3 aux Tests De Combat si la hache est le seul objet … |
| `Journal_Intime_Du_Directeur.html` | 48 / 2 | Cet Objet apparait dans l' Extension Le Roi En Jaune. Cet objet est un Tome à usage unique permettant d'empêcher l'augmentation du niveau de terreur. Son prix au Magasin est de … |
| `Kit_De_Maquillage.html` | 73 / 2 | Cet Objet apparait dans l' Extension L'Horreur De Kingsport C'est un Objet Commun fournissant un bonus de +1 pour tous les tests de volonté (dans les rencontres de lieux par exemple) qui … |
| `Kérosène.html` | 62 / 2 | Cet Objet apparait dans l' Extension L'Horreur De Kingsport C'est une Arme Physique. C'est une Arme A Une Main. C'est une Arme à recharger (pour 1 dollar) en Phase d'entretien … |
| `Lance_Flammes.html` | 70 / 2 | Cet Objet apparait dans l' Extension L'Horreur De Dunwich C'est une Arme Physique. C'est une Arme A Deux Mains. C'est une Arme à recharger (gratuitement) en Phase d'entretien Donne +7 … |
| `Lanterne.html` | 55 / 1 | "Prise de panique, la troupe entière s'enfuit. Seul un homme resta. Je reconnus en lui le barbu qui m'avait conduit en ce lieu. Debout, seul, une lanterne à la main, il attendait. … |
| `Livraison_De_Courrier.html` | 50 / 1 | Cet Objet apparait dans l' Horreur de Dunwich. C'est une Tâche permettant de regagner beaucoup de dollars. Elle est relativement facile à réaliser car deux des trois Lieux à visiter sont … |
| `Longue-vue.html` | 73 / 2 | Cet Objet apparait dans l' Extension L'Horreur De Kingsport C'est un Objet Commun fournissant un bonus de +1 pour tous les tests de discrétion (dans les rencontres de lieux par exemple) qui … |
| `Manuscrits_Poussiéreux.html` | 46 / 2 | Cet Objet apparait dans l' Extension L'Horreur De Dunwich. Cet objet est un Tome apportant un bonus permanent pour les Tests de Sort. Son prix au Magasin est de 4 dollars. … |
| `Menottes.html` | 64 / 2 | Cet Objet apparait dans l' Extension L'Horreur De Kingsport C'est un Objet Commun fournissant un bonus lors des combats contre les Cultistes, les Goules, les Maniaques et les Tcho-tchos. … |
| `Mitraillette.html` | 61 / 1 | Cet Objet apparait dans l' Horreur à Arkham. C'est une Arme Physique. C'est une Arme A Deux Mains. Donne +6 aux Tests De Combat. Son prix au Magasin … |
| `Motocyclette.html` | 50 / 1 | Cet Objet apparait dans l' Horreur à Arkham. C'est un Objet Commun fournissant un bonus de déplacement. Son prix au Magasin est de 4 dollars. Cet objet fait partie des possessions … |
| `Nourriture.html` | 101 / 1 | Cet Objet apparait dans l' Horreur à Arkham. C'est un Objet Commun fournissant un bonus unique face à une perte de résistance. Il est utile pour certaines rencontres d'Arkham, notamment dans … |
| `Patrouille_Dans_Les_Rues.html` | 101 / 1 | Cet Objet apparait dans l' Horreur de Dunwich. C'est une Tâche permettant de gagner dollars et 1 pion indice, et 1 objet commun. Elle est relativement facile à réaliser car deux … |
| `Pistolet_De_Détresse.html` | 49 / 2 | Objet commun (Kingsport) à usage unique, permet de provoquer un combat avec un monstre volant ; prix 2 $. (Son texte a été recopié par erreur dans `Brasier_Des_Ames.html`.) |
| `Plan_d'Arkham.html` | 50 / 1 | Cet Objet apparait dans l' Horreur à Arkham. C'est un Objet Commun fournissant un bonus de déplacement. Son prix au Magasin est de 2 dollars. Cet objet fait partie des possessions … |
| `Poing_Américain.html` | 49 / 2 | Cet Objet apparait dans l' Extension L'Horreur De Dunwich C'est une Arme Physique. C'est une Arme Sans Mains. Donne +1 aux Tests De Combat. Son prix au Magasin est … |
| `Poudre_d'Ibn-Ghazi.html` | 102 / 1 | "Il constata que Rice, du fait de la position avantageuse du trio en arrière et au dessus de l'entité, avait bien des chances de projeter la puissante poudre avec une merveilleuse efficacité." … |
| `Rapport_De_Minéralogie.html` | 58 / 1 | Cet Objet apparait dans l' Horreur de Dunwich. C'est une Tâche permettant de gagner un allié, des dollars et des pions indices. Elle est intéressante, mais par contre difficile à … |
| `Recherche_Généalogique.html` | 61 / 1 | Cet Objet apparait dans l' Horreur de Dunwich. C'est une Tâche permettant de gagner un objet unique, des dollars et un pion indice. Elle est donc très intéressante, mais relativement … |
| `Révolver_.38.html` | 120 / 1 | "J'avais naturellement mon revolver dans la poche de mon veston, mais le moindre mouvement que j'aurais fait pour le saisir et le tirer de ma poche aurait été immédiatement vu de lui. … |
| `Sabre_de_Cavalerie.html` | 58 / 1 | Cet Objet apparait dans l' Horreur à Arkham. C'est une Arme Physique C'est une Arme A Une Main Donne +2 aux Tests De Combat Son prix au Magasin est de 3 … |
| `Tome_Ancien.html` | 143 / 1 | "Ses angles étaient magnifique et fort étranges; j'étais enchanté et ensorcelé par leur hideuse beauté et je pensais à ces fous du monde extérieur qui ne voyait qu'une erreur dans l'agencement … |
| `Vieux_Journal.html` | 146 / 1 | "Bien des noms, bien des formes, pour la même chose et le même but... Besoin d'aide... Trop important, trop effroyable. Ces rêves... les mêmes que Carlyle ? Voir les dossiers de ce … |
| `Whisky.html` | 78 / 1 | Cet Objet apparait dans l' Horreur à Arkham. C'est un Objet Commun fournissant un bonus unique face à une perte de santé mentale. Il est utile pour certaines rencontres d'Arkham, notamment … |
| `Étui_à_Cigarette_Porte-bonheur.html` | 77 / 1 | Cet Objet apparait dans l' Horreur à Arkham. C'est un Objet Commun fournissant un bonus unique pour faire un nouveau lancé de dés. Il est utile pour certaines rencontres de Kingsport, … |

**Fiches sans contenu** (image de la carte non téléchargée + « Erratas : aucun ») — 12 : `Faire_Des_Sacrifices.html`, `Les_Lumières_De_La_Raison.html`, `Manteau_Sombre.html`, `Matériel_de_Recherche.html`, `Pour_Le_Plus_Grand_Bien.html`, `Purifier_La_Ville.html`, `Rejoindre_L'Equipe_Gagnante.html`, `Sceller_Le_Pouvoir_De_La_Bête.html`, `Script_De_La_Doublure.html`, `Sur_Les_Chemins_De_Ley.html`, `Vague_De_Destruction.html`, `Vieille_Voiture.html`.


### 6.10 Objets uniques

| Fichier | mots / img | Contenu |
|---|---|---|
| `Boîte_A_Portail.html` | 34 / 1 | Commentaires Les Objets Uniques Précisions Lorsqu'un investigateur revient d'un Autre monde avec cet objet, il suit la procédure normale, et place donc bien un jeton exploré sur le portail. |
| `Bâton_De_Quête.html` | 62 / 2 | Cet Objet apparait dans l' Extension L'Horreur De Dunwich C'est une Arme Magique. C'est une Arme A Deux Mains. C'est une Arme à recharger (gratuitement) en Phase d'entretien Donne +7 … |
| `Canne_Enchantée.html` | 75 / 1 | Cet Objet apparait dans l' Horreur de Dunwich. C'est une Arme Magique C'est une Arme A Une Main Donne +3 aux Tests De Combat Elle ne peut pas être perdue si … |
| `Clé_d'Argent.html` | 56 / 1 | "Un vieux serviteur ébranlant, comme il avait déjà fait lui-même, les hideux visages aux regards méchants taillés dans le bois noir, força le couvercle sculpté. A l'intérieur, enveloppée dans un parchemin décoloré, … |
| `Couteau_Enchanté.html` | 127 / 1 | "Bref, le sol était un véritable ossuaire fait de squelettes de petits enfants - certains assez récents, mais d'autres remontant par des degrés infinis jusqu'à une telle ancienneté qu'ils tombaient presque en … |
| `De_Vermiis_Mysteriis.html` | 25 / 1 | Cet objet unique est un Tome. errata Cette carte s'utilise pendant la phase de mouvement et non de rencontres. |
| `Epée_D'Or_De_Y'ha_Talla.html` | 62 / 2 | Cet Objet apparait dans l' Extension L'Horreur De Dunwich C'est une Arme Magique. C'est une Arme A Une Main. C'est une Arme à recharger (gratuitement) en Phase d'entretien Donne +5 … |
| `Fragments_D'Eltdown.html` | 12 / 1 | Cet objet unique est un Tome. |
| `Fusil_Yithien.html` | 66 / 2 | Cet Objet apparait dans l' Extension L'Horreur De Kingsport C'est une Arme Magique. C'est une Arme A Deux Mains. C'est une Arme à recharger en dépensant toute sa concentration en … |
| `Glaive_De_Carcosa.html` | 75 / 2 | Cet Objet apparait dans l' Extension Le Roi En Jaune. C'est une Arme Magique C'est une Arme A Une Main Donne +5 aux Tests De Combat si le glaive est le … |
| `Joyaux_Enchantés.html` | 103 / 1 | "[La tiare] était haute en avant, très large et d'un contour curieusement irrégulier, tel qu'on l'aurait conçu pour une tête monstrueusement elliptique. L'or semblait y dominer, mais un mystérieux éclat plus lumineux … |
| `Lame_Enchantée.html` | 50 / 1 | Cet Objet apparait dans l' Horreur à Arkham. C'est une Arme Magique C'est une Arme A Une Main Donne +4 aux Tests De Combat Son prix à la Boutique De Souvenirs … |
| `Lampe_d'Alhazred.html` | 121 / 1 | "La lampe d'Alhazred offrait un aspect singulier. Destinée à brûler de l'huile, on l'eût dite en or massif. Elle avait la forme d'un petit récipient oblong. Sur un des côtés se recourbait … |
| `Les_Chants_De_Dhole.html` | 57 / 2 | Cet Objet apparait dans l' Extension L'Horreur De Dunwich. Cet objet est un Tome à usage unique permettant d'acquérir 1 Allié pour une perte de 2 points de Santé Mentale. … |
| `Les_Sept_Livres_Cryptiques_De_Hsan.html` | 58 / 2 | Cet Objet apparait dans l' Extension L'Horreur De Dunwich. Cet objet est un Tome permettant -une fois par tour- de se substituer totallement ou partiellement au coût de lancement d'un Sort … |
| `Livre_D'Ivon.html` | 23 / 1 | Cet objet unique est un Tome. L'investigateur Daisy Walker possède cet objet au début de la partie. |
| `Livre_de_Dzyan.html` | 130 / 1 | "Quoique nous soyons la plus grande race conçue à ce jour, nous ne sommes pas les premiers et nous ne sommes pas destinés à être les derniers. Comme pour Mu, comme pour … |
| `Magie_Véritable.html` | 67 / 2 | Cet Objet apparait dans l' Extension L'Horreur De Dunwich. Cet objet est un Tome à usage permanent - un des seuls tomes avec cette capacité. Il permet d'acquérir 1 pion indice … |
| `Manuscrit_Illuminé.html` | 12 / 1 | Cet objet unique est un Tome. |
| `Messa_Di_Requiem_Per_Shuggay.html` | 12 / 1 | Cet objet unique est un Tome. |
| `Necronomicon.html` | 109 / 1 | "Ne pensez pas que l'homme soit le plus ancien ou le dernier des maîtres de la Terre, ou qu'il n'existe pas d'autre forme de vie et de matière. Les Anciens furent, les … |
| `Outil_Extraterrestre.html` | 49 / 2 | Cet Objet apparait dans l' Extension L'Horreur De Dunwich C'est une Arme Magique. C'est une Arme Sans Mains. Donne +2 aux Tests De Combat. Son prix au Magasin est … |
| `Statue_Extraterrestre.html` | 79 / 1 | "La tête céphalopode était inclinée en avant de sorte que les extrêmités des tentacules faciaux allaient effleurer par-derrière les énormes pattes antérieures qui étraignaient les genoux de la créature accroupie. L'ensemble était … |
| `Tablette_Antique.html` | 86 / 1 | "Par toutes les profondeurs de Y'ha-nthlei - et ses habitants, pour Celui Qui règne; Par le Signe de Kish - et tous ceux qui lui obéissent, pour son Auteur; Par … |
| `Épée_de_Gloire.html` | 50 / 1 | Cet Objet apparait dans l' Horreur à Arkham. C'est une Arme Magique C'est une Arme A deux Mains Donne +6 aux Tests De Combat Son prix à la Boutique De Souvenirs … |

**Pages vides** (image seule, non téléchargée) — 9 : `Arme_à_Foudre.html`, `Cthaat_Aquadingen.html`, `Flûte_des_Dieux_Extérieurs.html`, `Gardien_Bleu_de_la_Pyramide.html`, `Masque_Blême.html`, `Rubis_de_R'lyeh.html`, `Statue_d'Obsidienne.html`, `Statue_de_Garde.html`, `Œil_du_Dragon.html`.


### 6.11 Sorts

| Fichier | mots / img | Contenu |
|---|---|---|
| `Atrophie.html` | 23 / 1 | Commentaires Cet objet fait partie des possessions fixes de Agnes Baker. Retour vers la liste des Sorts |
| `Lancer_Un_Sort.html` | 160 / 0 | Un investigateur doit réussir à lancer un sort afin de bénéficier de ses effets. Chaque sort a un modificateur de lancer, et la plupart ont un coût en Santé Mentale. Pour lancer … |
| `Lier_un_Monstre.html` | 83 / 1 | Commentaires Aucun Erratas & Clarifications Q: Lorsque je lance le sort "Lier un Monstre", je fais autant de réussites que la force du monstre mais je perds 2 points de santé … |
| `Rempart_de_Chair.html` | 66 / 1 | Commentaires Aucun Erratas & Clarifications Q: Quand peut-on lancer Rempart de Chair ? R: Quand on vous dit de perdre au moins 1 point de Résistance, vous pouvez lancer ce … |
| `Signe_Rouge_de_Shudde_M'ell.html` | 41 / 1 | Commentaires Aucun Erratas & Clarifications Q: Combien de temps dure le sort Signe Rouge de Shudde M'ell ? R: Il dure tant que vous y consacrez une main. Retour vers … |
| `Test_De_Sort.html` | 157 / 1 | Un Test De Sort est un Test de compétence basé sur la valeur de Savoir de l’Investigateur, ajusté par le modificateur de lancer du sort. Si le test est réussi, le sort … |
| `Trouver_le_Portail.html` | 23 / 1 | Commentaires Ce sort fait partie des possessions fixes de Norman Withers. Retour vers la liste des Sorts |

**Fiches sans contenu** (image de la carte non téléchargée + « Erratas : aucun ») — 38 : `Appeler_La_Flamme_D'Azure.html`, `Appeler_le_Grand_Ancien.html`, `Appeler_un_ami.html`, `Apprentissage_Forcé.html`, `Bannissement_Suprême.html`, `Brumes_de_Releh.html`, `Bénédiction(Sort).html`, `Divination.html`, `Enchanter_une_Arme.html`, `Flétrissement.html`, `Fracassement.html`, `Implanter_une_Suggestion.html`, `Intervention.html`, `Invasion_de_Sauterelles.html`, `Invoquer_La_Bête_Intérieure.html`, `Invoquer_un_Monstre.html`, `Invoquer_un_Shantak.html`, `La_Brume_Jaune.html`, `Les_Marques_d'Isis.html`, `Leurre.html`, `Linceul_des_Ombres.html`, `Nourrir_l'Esprit.html`, `Ouvrir_Le_Troisième_Oeil.html`, `Poing_de_Yog-Sothoth.html`, `Prendre_la_Vie.html`, `Procédé_alchimique.html`, `Prémonition.html`, `Quête_pour_une_vision.html`, `Rasoir_Spectral.html`, `Renier_le_Grand_Ancien.html`, `Révélation_des_Ecritures.html`, `Soin.html`, `Souvenirs_brumeux.html`, `Tempête_D'Esprit.html`, `Terrible_Malédiction_d'Azathoth.html`, `Un_Aperçu_Des_Arcanes.html`, `Voix_de_Ra.html`, `Voyage_Astral.html`.


### 6.12 Alliés

| Fichier | mots / img | Contenu |
|---|---|---|
| `John_Legrasse.html` | 86 / 1 | "il ne faut pas s'imaginer que l'inspecteur Legrasse témoignait du moindre intérêt pour l'archéologie. Son désir d'être éclairé ne provenait en effet que de considérations purement professionnelles. La statuette... avait été saisie … |
| `Professeur_Armitage.html` | 48 / 1 | "Finalement les trois hommes d'Arkham - le vieux docteur Armitage à la barbe blanche, le professeur Rice qui grisonnait, et l'assez jeune et maigre docteur Morgan - firent seuls l'ascension du sommet." … |
| `Richard_Upton_Pickman.html` | 87 / 1 | "Il montrait là ce qui arrive à ces enfants volés - comment ils sont élevés - et c'est alors que je pris conscience peu à peu d'une hideuse ressemblance entre les personnages … |
| `Ryan_Dean.html` | 45 / 1 | Histoires et rencontres Ryan Dean est un vendeur itinérant que l'on peut rencontrer à la Pension de Ma. recrutement Pour recruter Ryan Dean, il faut aller au Relais Routier de Hibb … |
| `Thomas_F._Malone.html` | 65 / 1 | "Dans sa jeunesse, il avait senti la beauté cachée et l'extase des choses, et il avait été un poète. Mais la pauvreté, le chagrin et l'exil avaient tourné son regard vers des … |
| `Tom_"Mountain"_Murphy.html` | 49 / 1 | Erratas & Clarifications Dans le texte originale de cette carte, Il faut remplacer le terme Déferlement par Irrésistible. La capacité Déferlement n'existe pas. La carte présentée ici est une version révisée … |

**Fiches sans contenu** (image de la carte non téléchargée + « Erratas : aucun ») — 28 : `Ammi_Pierce.html`, `Anna_Kaslow.html`, `Asenath_Waite.html`, `Basil_Elton.html`, `Caton.html`, `Charles_Dexter_Ward.html`, `Corinna_Jones.html`, `David_Packard.html`, `Dr_Ali_Kafour.html`, `Dr_Herbert_West.html`, `Duke.html`, `Earl_Sawyer.html`, `Eric_Colt.html`, `Erica_Carlyle.html`, `Erich_Weiss.html`, `Grand-Mère_Orne.html`, `Jeune_Zoog.html`, `Le_Messager.html`, `Professeur_Morgan.html`, `Professeur_Rice.html`, `Père_Iwanicki.html`, `Ruby_Standish.html`, `Sarah_Danforth.html`, `Sir_William_Brinton.html`, `Terrible_Vieillard.html`, `Thomas_Olney.html`, `William_Bain.html`, `Zebulon_Whateley.html`.


### 6.13 Compétences

| Fichier | mots / img | Contenu |
|---|---|---|
| `Chance.html` | 104 / 1 | Erratas & Clarifications Q: La carte Compétence "Chance" indique “Chance +1” en caractères gras, puis en caractères normaux "Quand vous dépensez un jeton Indice pour ajouter un dé à un test … |
| `Combat.html` | 106 / 1 | Erratas & Clarifications Q: La carte Compétence "Combat" indique “Combat +1” en caractères gras, puis en caractères normaux "Quand vous dépensez un jeton Indice pour ajouter un dé à un test … |
| `Discrétion.html` | 104 / 1 | Erratas & Clarifications Q: La carte Compétence "Discrétion" indique “Discrétion +1” en caractères gras, puis en caractères normaux "Quand vous dépensez un jeton Indice pour ajouter un dé à un test … |
| `Savoir.html` | 103 / 1 | Erratas & Clarifications Q: La carte Compétence "Savoir" indique “Savoir +1” en caractères gras, puis en caractères normaux "Quand vous dépensez un jeton Indice pour ajouter un dé à un test … |
| `Vitesse.html` | 104 / 1 | Erratas & Clarifications Q: La carte Compétence "Vitesse" indique “Vitesse +1” en caractères gras, puis en caractères normaux "Quand vous dépensez un jeton Indice pour ajouter un dé à un test … |
| `Volonté.html` | 104 / 1 | Erratas & Clarifications Q: La carte Compétence "Volonté" indique “Volonté +1” en caractères gras, puis en caractères normaux "Quand vous dépensez un jeton Indice pour ajouter un dé à un test … |

**Fiches sans contenu** (image de la carte non téléchargée + « Erratas : aucun ») — 27 : `Agressif.html`, `Arts_Martiaux.html`, `Bravoure.html`, `Camouflage.html`, `Courir.html`, `Crédit.html`, `Dissimulation.html`, `Ecouter.html`, `Effacé.html`, `Endurance.html`, `Force_Mentale.html`, `Furtivité.html`, `Inébranlable.html`, `Langage_Ancien.html`, `Linguistique.html`, `Lutteur.html`, `Mystique.html`, `Occultiste_Chevronné.html`, `Oeil_D'Aigle.html`, `Perspicace.html`, `Persuader.html`, `Pratique_Du_Catch.html`, `Rat_De_Bibliothèque.html`, `Se_Déguiser.html`, `Spécialiste_Du_Mythe.html`, `Tireur_D'Elite.html`, `Vif.html`.


### 6.14 Cartes spéciales et paquets annexes

| Fichier | mots / img | Contenu |
|---|---|---|
| `Acompte.html` | 93 / 1 | Les acomptes représentent des sources de revenus (emploi, rente) qui permettent aux joueurs de gagner 2$ à chaque tour, durant la phase d'entretien. Pour savoir si les activités de l'investigateur ne lui … |
| `Arrêté.html` | 89 / 0 | Certaines rencontres peuvent se finir sur l'arrestation d'un Investigateur, qui est emmené au Poste De Police. Quand cela se produit, le joueur place son investigateur dans la Cellule De Prison. … |
| `Bien-aimés_De_Bastet.html` | 116 / 2 | Les cartes Bien-aimès De Bastet de l' Extension L'Horreur De Kingsport ne sont employées qu'à partir du moment ou les joueurs on choisit le Gardien Bastet pour leur partie. Les deux cartes … |
| `Blessure.html` | 372 / 37 | Cartes Blessure (Innsmouth) : règle de prise de blessure à 0 RES + galerie (images). |
| `Bénédiction.html` | 90 / 1 | Les bénédictions représentent des interventions de puissances supérieures. Les investigateurs peuvent gagner cette carte grâce aux rencontres ou aux rumeurs. Tant qu’un investigateur est Béni, chaque résultat de 4 ou plus compte … |
| `Bénédiction_de_Nodens.html` | 124 / 27 | Les cartes Bénédiction de Nodens de l' Extension L'Horreur De Kingsport ne sont employées qu'à partir du moment ou les joueurs on choisit le Gardien Nodens pour leur partie. Une carte est … |
| `Bénéfice.html` | 102 / 4 | Similaires à la Bénédiction, les Bénéfices rendent les choses plus faciles pour les Investigateurs. Chaque carte a un certain événement ou une situation décrit sous les capacités fournies. Si votre Investigateur … |
| `Capitaines_Du_Bateau_Blanc.html` | 14 / 2 | Capitaine Du Bateau Blanc Le Bateau Blanc |
| `Carte_Bataille_Epique.html` | 382 / 44 | Variante Bataille Épique (Miskatonic) : règles en texte + galerie des cartes Bataille Épique par GA (images). |
| `Carte_Complot.html` | 205 / 118 | Cartes Complot (variante Bataille Épique) : descriptif + galerie par extension (118 images, texte absent). |
| `Carte_Lieu.html` | 404 / 39 | Même contenu que `Cartes_Lieu.html` (déjà inventorié, §2). |
| `Carte_Mythe.html` | 1397 / 409 | Même contenu que `Cartes_Mythe.html` (déjà inventorié, §2). |
| `Carte_Portail.html` | 308 / 1 | Même contenu que `Cartes_Portail.html` (déjà inventorié, §2). |
| `Carte_Portail_Bicolores.html` | 100 / 6 | Cartes Portail bicolores (Pharaon Noir) : règle + 6 images. |
| `Cartes_Acte.html` | 54 / 11 | Les cartes Acte sont utilisées dans les variantes Représentation Itinérante ou Représentation Permanente de l' Extension Le Roi En Jaune. Acte 1 Acte 2 Acte 3 L'Horreur de Miskatonic propose 4 … |
| `Cartes_Dette.html` | 282 / 42 | Paquet Dette (Rôdeur, 28 cartes) : usage + liste/galerie (images). |
| `Cartes_Difficulté.html` | 75 / 10 | Avec cette "variante", les joueurs choisissent une des 5 cartes Difficulté au début de la partie. Il y a 2 cartes qui rendent le jeu plus facile et 2 cartes qui rendent … |
| `Cartes_Engeance_d'Innsmouth.html` | 61 / 11 | Certains évènements d'Innsmouth vous oblige a tirer une (ou plusieurs) carte Engeance d'Innsmouth. Neuf des dix cartes engeance d'origine sont inoffensives, l'une d'elle est mortelle... Inoffensive (x9) Aïe Dos L'Horreur de Miskatonic … |
| `Cartes_Poussière.html` | 55 / 30 | Les cartes Poussières ne sont en jeu que lorsque Quatchil-Uttaus est le Grand Ancien. Le premier joueur ne change pas jusqu'à ce qu'il est dévoré l'investigateur qu'il poursuit. Les cartes poussière décrivent … |
| `Condition.html` | 173 / 8 | On utilise ces cartes pour indiquer une situation ou un bénéfice à long terme. Les cartes Condition commencent la partie face cachée et on les retourne sous l'effet d'une rencontre au lieu … |
| `Corruption.html` | 509 / 32 | Cartes Corruption (Chèvre Noire) : règles + galerie vertes/rouges (images, texte absent). |
| `Culte_des_Goules.html` | 85 / 1 | "Même si d'aucuns doivent parler de blasphème, j'ai choisi d'expliquer certaines actions et croyances et laisserai Dieu être le juge suprême." François Honoré de Balfour, 1703. Cet objet apparait dans le jeu … |
| `Cultes_Innommables.html` | 76 / 1 | "J'ai voyagé dans le monde en quête de l'obscurité - un voyage qui s'est terminé en moi-même." - Friedrich Wilhelm von Juntz, 1839. Cet Objet apparait dans le jeu de base. … |
| `Désavantage.html` | 100 / 4 | Similaires à la Malédiction, les Désavantages rendent les choses plus dures pour les Investigateurs. Chaque carte a un certain événement ou une situation décrit sous les capacités fournies. Si votre Investigateur … |
| `Effet_Magique.html` | 130 / 6 | Ces cartes sont placées avec les autres cartes Investigateurs Spéciales. Elles représentent des Effets Magiques persistants créés par certains Sorts. Quand le Sort approprié est lancé, l'Investigateur l'ayant lancé prend … |
| `Folie.html` | 325 / 37 | Cartes Folie (Innsmouth) : règle à 0 SAN + galerie (images). |
| `Galerie_Des_Cartes_Portail.html` | 234 / 395 | Galerie des **cartes Portail** (rencontres d'Autres Mondes) par extension : base 49, Pharaon Noir 27, Dunwich 32, Roi en Jaune 24, Kingsport 32, Chèvre Noire 24, Innsmouth 16, Rôdeur 24, Miskatonic 56. 395 images, **texte des rencontres absent**. |
| `La_Cabale_de_Saboth.html` | 47 / 1 | Cet Objet apparait dans le jeu de base. Cet objet est un Tome à usage unique permettant d'acquérir 1 compétence. Son prix à la Boutique De Souvenirs est de 5 … |
| `Malheur.html` | 186 / 41 | Les cartes Malheur sont utilisées avec le Héraut Le Roi En Jaune. IMPORTANT: Comme les cartes Malheur représentent des gens importants d'Arkham, il est possible que les Investigateurs tombent sur … |
| `Malédiction.html` | 89 / 1 | Les Malédictions représentent des interventions de puissances supérieures. Les investigateurs peuvent gagner cette carte grâce aux rencontres ou aux rumeurs. Tant qu’un investigateur est Maudit, seul un résultat de 6 compte comme … |
| `Missions.html` | 188 / 17 | Les Missions sont un type d' Objet Unique. Ils consistent en une courte liste de Lieu et / ou Zone De Rues qui doivent être visités dans l'ordre indiqué. Un … |
| `Objet.html` | 97 / 6 | Règle générale sur les objets (perte de la moitié des objets, etc.) + catégories. |
| `Pacte_Maléfique.html` | 81 / 3 | Les cartes Pacte Maléfique indiquent un marché entre l'investigateur et Le Rôdeur sur le Seuil. Quand un joueur gagne une de ces cartes, elle lui accorde des avantages uniques mais également … |
| `Prêt_Bancaire.html` | 19 / 1 | Cet objet fait partie des possessions fixes de Finn Edwards. |
| `Relation.html` | 191 / 19 | Le paquet Relation est composé de cartes représentant les effets concernant deux investigateurs résultant de leur histoire commune et de leur attitude l'un envers l'autre. On utilise ces cartes uniquement pour les … |
| `Rencontre_du_Culte.html` | 205 / 42 | Liste des 24 cartes Rencontre du Culte (Chèvre Noire), noms seuls. |
| `Signe_des_anciens.html` | 288 / 1 | Règle complète du Signe des Anciens (sceller un portail : conditions, coût, effets), en texte. |
| `Tome.html` | 533 / 46 | Les Tomes (objets communs/uniques qui se lisent pendant le mouvement) : règle générale + galerie (images). |
| `Transformés.html` | 11 / 4 | Le Grand Sceau Transformé |
| `Tâche_Et_Mission.html` | 26 / 0 | Les Tâches sont un type d' Objet Commun. Les Missions sont un type d' Objet Unique. |
| `Tâches.html` | 152 / 5 | Les Tâches sont un type d' Objet Commun. Ils consistent en une courte liste de Lieu et / ou Zone De Rues qui doivent être visités dans l'ordre indiqué. Il … |
| `Tâches_Et_Missions.html` | 26 / 0 | Les Tâches sont un type d' Objet Commun. Les Missions sont un type d' Objet Unique. |
| `Visions_d'Hypnos.html` | 133 / 59 | Les cartes Visions d'Hypnos de l' Extension L'Horreur De Kingsport ne sont employées qu'à partir du moment ou les joueurs on choisit le Gardien Hypnos pour leur partie. Une carte est piochée … |

**Pages vides** (image seule, non téléchargée) — 2 : `Cultistes_à_Arkham.html`, `Membre_De_La_Loge_Du_Crépuscule_D'Argent.html`.

**Doublons** (texte identique) — 6 : `Cartes_Bataille_Epique.html` = `Carte_Bataille_Epique.html`, `Cartes_Complot.html` = `Carte_Complot.html`, `Cartes_Portail_Bicolores.html` = `Carte_Portail_Bicolores.html`, `Objets.html` = `Objet.html`, `Signe_Des_Anciens~3f012c6b.html` = `Signe_des_anciens.html`, `Tomes.html` = `Tome.html`.


### 6.15 Erratas

| Fichier | mots / img | Contenu |
|---|---|---|
| `Erratas.html` | 1086 / 4 | Page d'erratas (MAJ mars 2010) : règle de traduction Fight/Combat, puis erratas texte par élément (fiches GA, investigateurs, monstres, cartes). Doublonne en partie les pages par extension. |
| `Erratas_-_Horreur_à_Arkham.html` | 1000 / 34 | Sections : Horreur à Arkham ; Fiches Grand Ancien ; Fiches Investigateur ; Jetons Monstre ; Cartes Portail ; Cartes Mythe ; Cartes Lieu ; Cartes Investigateur ; PDFs. |
| `Erratas_-_L'Horreur_d'Innsmouth.html` | 382 / 17 | Sections : L'Horreur d'Innsmouth ; Fiches Investigateur ; Cartes Portail ; Cartes Mythe ; Cartes Lieu ; PDFs. |
| `Erratas_-_L'Horreur_de_Dunwich.html` | 727 / 26 | Sections : L'Horreur de Dunwich ; Fiches Investigateur ; Jetons Monstre ; Cartes Portail ; Cartes Mythe ; Cartes Lieu ; Cartes Investigateur ; PDFs. |
| `Erratas_-_L'Horreur_de_Kingsport.html` | 360 / 14 | Sections : L'Horreur de Kingsport ; Fiches Investigateur ; Cartes Portail ; Cartes Lieu ; Cartes Investigateur ; PDFs. |
| `Erratas_-_L'Horreur_de_Miskatonic.html` | 102 / 2 | Sections : L'Horreur de Miskatonic ; Cartes Engeance ; PDFs. |
| `Erratas_-_La_Chèvre_Noire_des_Bois.html` | 238 / 11 | Sections : La Chèvre Noire des Bois ; Cartes Portail ; Cartes Lieu ; Cartes Investigateur ; PDFs. |
| `Erratas_-_La_Malédiction_du_Pharaon_Noir_(1ère_Édition).html` | 691 / 27 | Sections : La Malédiction du Pharaon Noir (1ère Édition) ; Cartes Portail ; Cartes Mythe ; Cartes Lieu ; Cartes Investigateur ; PDFs. |
| `Erratas_-_La_Malédiction_du_Pharaon_Noir_(Édition_révisée).html` | 855 / 37 | Sections : La Malédiction du Pharaon Noir (Édition Révisée) ; Cartes Portail ; Cartes Mythe ; Cartes Lieu ; Cartes Investigateur ; PDFs. |
| `Erratas_-_Le_Roi_en_Jaune.html` | 405 / 22 | Sections : Le Roi en Jaune ; Cartes Portail ; Cartes Mythe ; Cartes Lieu ; Cartes Investigateur ; PDFs. |
| `Erratas_-_Le_Rôdeur_Devant_le_Seuil.html` | 373 / 19 | Sections : Le Rôdeur Devant le Seuil ; Cartes Portail ; Cartes Mythe ; Cartes Lieu ; Cartes Investigateur ; PDFs. |
| `FAQ_VO.html` | 1131 / 0 | Entrées de la FAQ officielle **en anglais** non encore traduites (mouvement, monstres, rencontres, combat…), datées 2007-2008. |


### 6.16 Extensions et modes de jeu

| Fichier | mots / img | Contenu |
|---|---|---|
| `Difficulté_Du_Jeu.html` | 226 / 0 | Attention, si vous jouez avec Plusieurs Extensions de ville, la difficulté est amoindrie ! Apparition des Monstres De 1 à 4 joueurs, 1 Monstre apparaît à l'ouverture d'un Portail. De 5 … |
| `Exposition_Permanente.html` | 207 / 0 | Dans ce mode de jeu, L'Exposition "L'Héritage Des Pharaons" fait partie de la vie normale d’Arkham. L'Exposition s'y est installée (et les gens continuent à la visiter) mais elle n’est pas au … |
| `Extension_L'Horreur_de_Miskatonic.html` | 172 / 1 | Parue en Janvier 2012, Elle est la dernière des grosses extensions du jeu Horreur à Arkham. Dans une boîte de "L'Horreur de Miskatonic", vous trouverez le matériel suivant: 1 Livret … |
| `Extension_La_Chèvre_Noire_des_Bois.html` | 391 / 1 | Parue en 2009 La police recherche une jeune fille, dont le nom est volontairement tu, après qu'une tragédie ait frappé sa famille cette semaine. L'adolescente de 14 ans a été vue pour … |
| `Extension_Le_Rôdeur_Devant_le_Seuil.html` | 208 / 0 | Fichier:Boite LRdlS.jpg Parue en 2011 Il promet le poison, puis un traitement Mes doigts se déforment pendant que j'ouvre sa porte Je ne peux fuir la puissance que je détiens Mais le … |
| `Extension_l'Horreur_d'Innsmouth.html` | 217 / 10 | Parue en 2010 De toutes les cités décrites dans les visions de Lovecraft sur le Massachusetts, aucune n’est peut-être aussi mortelle que le petit village de pêcheurs d’Innsmouth. Décatie et mourante, Innsmouth … |
| `Extension_l'Horreur_de_Dunwich.html` | 688 / 16 | Parue en 2007 Le Frère de Wilbur Whateley Appelle Le Nom de Son Père sur la Colline du Sorcier... Les évènements surnaturels qui tourmentent Arkham, ne sont pas limités à ce malheureux … |
| `Extension_l'Horreur_de_Kingsport.html` | 470 / 14 | Parue en 2009 "...Une brume monte de la mer près des falaises de Kingsport. Blanche et duveteuse, elle s'élève des profondeurs vers ses frères les nuages, toute pleine de rêves de pâturages … |
| `Extension_le_Roi_en_Jaune.html` | 239 / 1 | Une nouvelle pièce de théâtre va être jouée à l’université de Miskatonic et tous seront là pour la première. Mais il y a quelque chose de sinistre dans cette comédie et ceux … |
| `Horreur_A_Arkham.html` | 501 / 1 | Présentation du jeu de base (contexte, matériel, principe) avec lien vers la notice PDF `Notice_HaA_Base.pdf`. |
| `La_Malédiction_du_Pharaon_Noir.html` | 332 / 1 | Cette version de La Malédiction du Pharaon Noir est aujourd'hui obsolète et définitivement épuisé Elle est remplacée par La Malédiction du Pharaon Noir (Édition Révisée) Extrait de de l'Arkham Gazette Le professeur … |
| `La_Malédiction_du_Pharaon_Noir_(Édition_Révisée).html` | 113 / 1 | Dans une boîte de "La Malédiction du Pharaon Noir (Édition Révisée)", vous trouverez le matériel suivant: 1 Livret de règles 76 Cartes Investigateurs: 22 Objets De L'Exposition (ressemble aux Objets … |
| `Le_Pharaon_Noir.html` | 73 / 1 | C'est un Monstre Masque. La fiche présentée ici est une version révisée que vous pouvez sauvegarder et imprimer pour remplacement ! Pour n'avoir que la face combat: Le Pharaon Noir … |
| `Le_Roi_en_Jaune.html` | 166 / 1 | " Sur la rive les vagues nuageuses se brisent, Les soleils jumeaux s'enfoncent derrière le lac, Les ombres s'allongent A Carcosa. Etrange est la nuit des étoiles noires. D'étranges lunes tournent dans … |
| `Plusieurs_Extensions.html` | 184 / 0 | Lorsque vous jouez avec plus d'une extension de ville (par exemple Dunwich + Kingsport ), vous devez considérer le nombre de joueurs comme inférieur de 1 par rapport au nombre réel pour … |
| `Représentation_Itinérante.html` | 297 / 3 | Dans ce mode de jeu, "Le Roi En Jaune" est une pièce de théâtre en tournée qui vient d'arriver à Arkham. Mais lors de sa présence en ville, d'étranges événements entourant la … |
| `Représentation_Permanente.html` | 178 / 3 | Dans ce mode de jeu, La pièce «Le Roi En Jaune» fait partie de la vie d’Arkham. La pièce se joue toujours (et les gens continuent à la lire) mais elle n’est … |
| `Symbole_D'Extension.html` | 45 / 7 | Tableau des numéros d'extension (0 = base, 1 = Dunwich, 2 = Pharaon Noir… 8 = Miskatonic) utilisés dans les listes, + icônes. |
| `Visite_De_L'Expositon.html` | 406 / 0 | Dans ce mode de jeu, l'Expoisition "L'Héritage Des Pharaons" vient d'arriver à Arkham. Mais lors de sa présence en ville, d'étranges événements entourant l'exposition vont attirer l'attention des Investigateurs. Des reliques aniques … |

**Doublons** (texte identique) — 8 : `Extension_L'Horreur_De_Dunwich~7921a5f3.html` = `Extension_l'Horreur_de_Dunwich.html`, `Extension_L'Horreur_De_Kingsport~04a06972.html` = `Extension_l'Horreur_de_Kingsport.html`, `Extension_La_Chèvre_Noire_Des_Bois~12786380.html` = `Extension_La_Chèvre_Noire_des_Bois.html`, `Extension_La_Malédiction_Du_Pharaon_Noir~a80ec1d0.html` = `La_Malédiction_du_Pharaon_Noir.html`, `Extension_La_Malédiction_du_Pharaon_Noir_(Édition_Révisée).html` = `La_Malédiction_du_Pharaon_Noir_(Édition_Révisée).html`, `Extension_Le_Roi_En_Jaune~aa639f70.html` = `Extension_le_Roi_en_Jaune.html`, `Extension_la_Malédiction_du_Pharaon_Noir.html` = `La_Malédiction_du_Pharaon_Noir.html`, `Horreur_à_Arkham.html` = `Horreur_A_Arkham.html`.


### 6.17 Créations de fans

| Fichier | mots / img | Contenu |
|---|---|---|
| `Calice_de_Breuvage_de_l'Espace.html` | 13 / 1 | Calice de Breuvage de l'Espace Les Objets Uniques |
| `Choisir_Un_Grand_Ancien_Pour_Sa_Partie.html` | 1687 / 1 | Long guide de stratégie (avis des joueurs) : difficulté et particularités de chaque Grand Ancien, conseils pour débutants. |
| `La_Nuit_des_Morts_Vivants.html` | 88 / 0 | Traduction d'une mini-extension fan « La Nuit des Morts Vivants » (lien). |
| `Le_Culte_de_Kingsport.html` | 288 / 2 | Variante fan : un culte qui rend le plateau de Kingsport dangereux (règles en texte). |
| `Le_Retour_des_Serviteurs.html` | 64 / 5 | Mini-extension fan sur les Serviteurs des Grands Anciens (2 versions), images. |
| `Les_Montagnes_Hallucinées.html` | 253 / 3 | Mini-extensions fan « Expédition en Antarctique » / « Les Montagnes Hallucinées » / « L'Horreur Polaire » : descriptif et liens. |
| `Les_histoires_des_investigateurs.html` | 486 / 0 | Histoires et campagnes personnelles fan centrées sur des investigateurs officiels (liens). |
| `Maquillage_de_scène.html` | 30 / 1 | Cet objet vient des fichiers de démo de Strange Eons ! Il a été apparement inspiré par le penchant de Sherlock Holmes pour le déguisement. |
| `Métamorphoses.html` | 56 / 0 | Mini-extension fan « Métamorphose(s) », 2 versions (liens). |
| `Pied_de_biche.html` | 15 / 2 | Pied de biche ' Hommage au Dr Gordon Freeman ! |
| `Projecteur_à_ultraviolets_des_Mi-Go.html` | 13 / 2 | Projecteur à ultraviolets des Mi-Go Les Objets Uniques |
| `Strange_Eons.html` | 672 / 0 | Strange Eons (éditeur de cartes fan) : installation de la VF, éléments à traduire, quickscripts, fiches investigateur, FAQ. |
| `Tronçonneuse.html` | 52 / 2 | Tronçonneuse ' Commentaires L'idée de cet objet créé par Greg, est que c'est une arme puissante, mais "salissante" (pas la peine de détailler, vous me comprenez...) et du coup le choc provoqué … |

**Pages vides** (image seule, non téléchargée) — 4 : `Bio-armure_des_Mi-Go.html`, `Morphine.html`, `Stimulants.html`, `Tranquillisant.html`.


### 6.18 Pages vides diverses


**Pages vides** (image seule, non téléchargée) — 16 : `Abonnement_Ferroviaire.html`, `Bénédiction_De_Darke.html`, `Cache_Secrète.html`, `Clef_anglaise.html`, `Collection_De_Livres_Rares.html`, `Contrôle_Météorologique.html`, `Faux.html`, `Forme_Animale.html`, `Frayeur.html`, `Fusil_de_Sniper.html`, `Malédiction_de_Chaugnar_Faughn.html`, `Membre_Du_Gang_Sheldon.html`, `Messages_Codés.html`, `Oeil_de_Lumière_et_de_Ténèbres.html`, `Poigne_de_Nyogtha.html`, `Reconnaissance_De_Velma.html`.


### 6.19 Hors sujet — Les Demeures de l'Épouvante

| Fichier | mots / img | Contenu |
|---|---|---|
| `Cartes_Action_MoM.html` | 119 / 26 | Sections : Demeures de l'Epouvante ; Expériences Interdites ; Saison de la Sorcière ; Tablette d'Argent ; Jusqu'à ce que la Mort Nous Sépare. |
| `Cartes_Exploration.html` | 217 / 73 | Sections : Demeures de l'Epouvante ; Expériences Interdites ; Saison de la Sorcière ; Tablette d'Argent ; Jusqu'à ce que la Mort Nous Sépare. |
| `Cartes_Mythe_MoM.html` | 161 / 48 | Sections : Les Demeures de l'Epouvante ; Expériences Interdites. |
| `Cartes_Obstacles.html` | 93 / 17 | Sections : Demeures de l'Epouvante ; Expériences Interdites ; Saison de la Sorcière ; Tablette d'Argent ; Jusqu'à ce que la Mort Nous Sépare. |
| `Cartes_Trauma_MoM.html` | 92 / 32 | Sections : Les Demeures de l'Epouvante ; Expériences Interdites. |
| `Cartes_Verrou.html` | 107 / 25 | Sections : Demeures de l'Epouvante ; Expériences Interdites ; Saison de la Sorcière ; Tablette d'Argent ; Jusqu'à ce que la Mort Nous Sépare. |
| `Jusqu'à_ce_que_la_Mort_nous_Sépare.html` | 88 / 41 | Sections : Guides ; Evènements ; Indices ; Objectifs ; Actions Spécifiques ; Exploration, Obstacles, Verrous Spécifiques ; Cartes Spéciales. |
| `La_Matière_Jaune.html` | 20 / 22 | Sections : Evènements ; Indices ; Objectifs. |
| `La_Retour_du_Réanimateur.html` | 21 / 20 | Sections : Evènements ; Indices ; Objectifs. |
| `La_Saison_de_la_Sorcière.html` | 79 / 34 | Sections : Guides ; Evènements ; Indices ; Objectifs ; Actions Spécifiques ; Exploration, Obstacles, Verrous Spécifiques. |
| `La_Tablette_d'Argent.html` | 82 / 33 | Sections : Guides ; Evènements ; Indices ; Objectifs ; Actions Spécifiques ; Exploration, Obstacles, Verrous Spécifiques. |
| `La_chute_de_la_maison_Lynch.html` | 21 / 20 | Sections : Evènements ; Indices ; Objectifs. |
| `La_malédiction_de_la_salle_de_classe.html` | 21 / 20 | Sections : Evènements ; Indices ; Objectifs. |
| `Le_Garçon_aux_yeux_verts.html` | 21 / 26 | Sections : Evènements ; Indices ; Objectifs. |
| `Le_coeur_de_la_loge.html` | 21 / 22 | Sections : Evènements ; Indices ; Objectifs. |
| `Le_lien_du_sang.html` | 21 / 24 | Sections : Evènements ; Indices ; Objectifs. |
| `Perdu_dans_l'Esapce_et_le_Temps.html` | 21 / 22 | Sections : Evènements ; Indices ; Objectifs. |


### 6.20 Hors sujet — Le Signe des Anciens

| Fichier | mots / img | Contenu |
|---|---|---|
| `Accomplir_les_tâches.html` | 543 / 1 | Accomplir une Tâche Quand un investigateur accomplit une tâche, pour chacune de ses conditions correspondant à un résultat de dé, le joueur place le dé avec ce résultat sur la condition requise. … |
| `Concentration_et_Aide.html` | 255 / 0 | Après avoir fait un jet de dés ne permettant pas d’accomplir une tâche, le joueur peut décider de se concentrer ou d’aider un autre investigateur de la même carte Aventure. Dans les … |
| `Essayer_de_Résoudre_une_Aventure_ou_Attendre_à_l’Entrée.html` | 295 / 10 | Essayer de Résoudre une Aventure Si le marqueur Investigateur du joueur actif est sur une carte Aventure, il lance les 6 dés verts pour essayer d’accomplir une des tâches de cette carte … |
| `Fin_Du_Jeu_es.html` | 80 / 0 | Tous les investigateurs, y compris ceux ayant été dévorés durant la bataille finale, gagnent la partie s’ils ont collecté un nombre de Signes des Anciens supérieur ou égal à la Limite de … |
| `Gates_of_Arkham_es.html` | 195 / 0 | Gates of Arkham The struggle against the Ancient Ones spreads out of the museum and into the haunted city of Arkham in the Gates of Arkham expansion for Elder Sign. Adventures take … |
| `Matériel_es.html` | 417 / 15 | Horloge Aiguille et Connecteur Plastique L’horloge sert à mesurer le passage du temps dans le jeu et détermine quand on pioche des cartes Mythe. L’aiguille est attachée à l’horloge avec le connecteur … |
| `Préparation_Du_Jeu_es.html` | 424 / 0 | 1. Installer l’horloge et l’entrée: on place l’horloge au centre de la table avec l’aiguille sur XII (minuit). On place la feuille de référence Entrée ainsi que tous les dés à … |
| `Se_Déplacer.html` | 65 / 0 | Se Déplacer Le joueur actif prend son marqueur Investigateur et le place sur une des cartes Aventure visibles en jeu ou sur la feuille de référence Entrée. S’il place son marqueur sur … |
| `Tour_De_Jeu_es.html` | 179 / 0 | Le Signe des Anciens se joue en plusieurs tours. Ces tours commencent par le premier joueur puis passe au joueur suivant dans le sens des aiguilles d’une montre. Le joueur effectuant son … |

---

## 7. Encore manquant (constat)

- **Images** : aucune image n'est présente dans le dossier. Or c'est là que se trouve le texte des cartes : rencontres de lieu (24 galeries de quartier), rencontres d'Autres Mondes (~284 cartes Portail), cartes Mythe (~294), objets uniques, sorts, compétences, alliés, cartes spéciales, marqueurs Monstre, feuilles de Grands Anciens, fiches d'investigateurs.
- **167 pages internes liées mais non aspirées**, surtout une page par monstre (`Cultiste`, `Goule`, `Shoggoth`, `Byakhee`, `Profond`…), plus `Armes`, `Monstre`, `Tests_De_Combat`, `Arme_Physique`, `Arme_A_Une_Main`, `Arme_Magique`, `Les_Objets_Uniques`, `Les_Sorts`, `Allié`, `Séparation_Des_Paquets`, `Compétences_Des_Investigateurs`…
- **1 362 liens rouges** : pages qui n'existent pas sur le wiki (on ne pourra pas les récupérer). Exemple : les 11 sous-étapes de `Préparation_Du_Jeu`.
- **Le plateau** : pas de description des liaisons entre lieux et rues ni des flèches blanches et noires de mouvement des monstres (seulement sur l'image du plateau). Le PDF du plateau imprimable reste un lien externe.
- Liens PDF externes non récupérés : `ArkhamHorror_board.pdf`, `GMW Deck.pdf`, `elder_sign_faq_1.0_fr.pdf`, `Elder_Sign_Low.pdf`, `Ligue-Espagnole-1.pdf`.

---

## 8. Bilan pour le projet (constat, mis à jour)

| Besoin | Disponible ? | Où |
|---|---|---|
| Règles du tour de jeu (base) | **oui**, texte | `Notice_HaA_Base.pdf` / `Arkham.pdf`, `Aide_jeu_v3_Damn3d.pdf`, pages §6.1 |
| Règles des 8 extensions | **oui**, texte FR | `Notice_HaA_*.pdf` (§5.1), pages `Extension_*` et modes de jeu (§6.16) |
| Errata / cas limites | **oui**, texte | `FAQ_Officiel_032017_V2.pdf`, `Erratas_-_*.html` (§6.15), `Foire_Aux_Questions.html`, `FAQ_VO.html` (VO) |
| Variantes | oui, texte | `Moduler_La_Difficulté.html`, `Cartes_Difficulté`, Bataille Épique, Complot, `Plusieurs_Extensions` |
| Stats des 48 investigateurs | **oui**, tableau texte | `Liste_détaillée_des_Investigateurs.html` (une valeur par compétence ; les 4 crans des curseurs sont visibles seulement sur les fiches corrigées en image) |
| Pouvoirs des investigateurs | résumé texte | même tableau (colonne « Autres ») ; texte exact seulement en image |
| Possessions fixes des investigateurs | partiel, texte | pages individuelles §6.7 |
| Liste et stats des monstres | **oui**, tableau texte | `Liste_détaillée_des_Monstres.html`, listes par extension, symbole et force, règles par type de mouvement (§6.2) |
| Feuilles des Grands Anciens | **partiel** | adorateurs et erratas sur les pages GA (§6.8) ; feuilles complètes seulement en image (`Base-ToutesLesFiches.pdf` pour quelques-unes) |
| Sorts | partiel, tableau texte | `Liste_Des_Sorts.html` (coût, modificateur, mains, phase ; pas l'effet exact) |
| Alliés | partiel, tableau texte | `Liste_Des_Alliés_Par_Extension.html` (lieu, condition, bonus) |
| Objets communs | partiel, texte | pages individuelles §6.9 (bonus, mains, prix) ; pas de liste complète |
| Objets uniques, compétences | **non** (noms seulement) | galeries §2.5, fiches vides §6.10, §6.13 |
| Lieux (stable/instable, capacité spéciale) | **oui**, texte | pages Lieu §6.4 |
| Texte des rencontres de Lieu | **non** | galeries de quartier §6.5 : codes seulement |
| Texte des rencontres d'Autres Mondes | **non** | `Galerie_Des_Cartes_Portail.html` : images |
| Autres Mondes (couleurs de rencontre) | oui, texte | pages Autre Monde §6.4 |
| Portails (stats, fréquences, portails spéciaux) | oui, texte | `Portails.html`, pages `Portail_*` |
| Texte des cartes Mythe | **non** (noms seulement) | `Cartes_Mythe.html` ; mécanique générale en texte (§6.1) |
| Plateau (graphe des connexions) | **non** | lieux par quartier seulement (`Arkham.html`, `Dunwich.html`…) |
| Cartes corrigées par errata | oui, en image | §5.2 (OCR possible) |
