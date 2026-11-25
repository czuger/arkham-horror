# Correspondance des illustrations d’Arkham

Les originaux des lieux et des Autres Mondes se trouvent sur le [plateau d’Arkham](assets/plateau/arkham.jpg). Les coordonnées `(x, y)` sont les centres approximatifs relevés dans [board.ron](data/board.ron), en pixels depuis le coin supérieur gauche de l’image de 1064 × 1576 px.

Depuis le 22/11/2026, chaque chemin est écrit dans les données : `illustration` pour les lieux et les rues et `scenes` pour les Autres Mondes (depuis le 25/11/2026) dans [board.ron](data/board.ron), `portrait` pour les monstres dans [monsters.ron](data/monsters.ron). Le test `every_illustration_exists_and_none_is_orphan` vérifie que chaque fichier existe et qu'aucun PNG n'est inutilisé. Les textes de commande des images sont dans [illustrations/prompts](illustrations/prompts), un fichier par image, nommé comme elle. Le serveur en fait au build des copies JPEG réduites (`grand`, 1280 px, et `vignette`, 320 px) : 21,5 Mo au lieu de 186.

## Lieux

| Lieu | Quartier | Centre sur le plateau | Nouvelle illustration |
| --- | --- | --- | --- |
| Gare | Quartier Nord | `(165, 55)` | [train_station.png](illustrations/lieux/train_station.png) |
| Journal | Quartier Nord | `(75, 175)` | [newspaper.png](illustrations/lieux/newspaper.png) |
| Boutique de Souvenirs | Quartier Nord | `(75, 300)` | [curiositie_shoppe.png](illustrations/lieux/curiositie_shoppe.png) |
| Banque d’Arkham | Centre-Ville | `(315, 60)` | [bank_of_arkham.png](illustrations/lieux/bank_of_arkham.png) |
| Asile d’Arkham | Centre-Ville | `(460, 60)` | [arkham_asylum.png](illustrations/lieux/arkham_asylum.png) |
| Square de l’Indépendance | Centre-Ville | `(605, 55)` | [independence_square.png](illustrations/lieux/independence_square.png) |
| Relais Routier de Hibb | Quartier Est | `(605, 205)` | [hibbs_roadhouse.png](illustrations/lieux/hibbs_roadhouse.png) |
| Restaurant de Velma | Quartier Est | `(760, 205)` | [velmas_diner.png](illustrations/lieux/velmas_diner.png) |
| Poste de Police | Quartier Est | `(760, 350)` | [police_station.png](illustrations/lieux/police_station.png) |
| Île Inexplorée | Quartier Marchand | `(70, 485)` | [unvisited_isle.png](illustrations/lieux/unvisited_isle.png) |
| Les Quais | Quartier Marchand | `(70, 610)` | [river_docks.png](illustrations/lieux/river_docks.png) |
| L’Innommable | Quartier Marchand | `(215, 665)` | [the_unnamable.png](illustrations/lieux/the_unnamable.png) |
| Cimetière | Quartier de la Rivière | `(765, 545)` | [graveyard.png](illustrations/lieux/graveyard.png) |
| Caverne Noire | Quartier de la Rivière | `(765, 665)` | [black_cave.png](illustrations/lieux/black_cave.png) |
| Magasin | Quartier de la Rivière | `(590, 670)` | [general_store.png](illustrations/lieux/general_store.png) |
| Département Scientifique | Université Miskatonic | `(65, 770)` | [science_building.png](illustrations/lieux/science_building.png) |
| Administration | Université Miskatonic | `(145, 960)` | [administration.png](illustrations/lieux/administration.png) |
| Bibliothèque | Université Miskatonic | `(285, 915)` | [library.png](illustrations/lieux/library.png) |
| La Maison de la Sorcière | French Hill | `(765, 870)` | [witch_house.png](illustrations/lieux/witch_house.png) |
| Loge du Crépuscule d’Argent | French Hill | `(628, 935)` | [silver_twilight_lodge.png](illustrations/lieux/silver_twilight_lodge.png) |
| Hôpital Sainte-Marie | Quartier Résidentiel | `(70, 1130)` | [st_marys_hospital.png](illustrations/lieux/st_marys_hospital.png) |
| La Vieille Échoppe de Magie | Quartier Résidentiel | `(178, 1305)` | [ye_olde_magick_shoppe.png](illustrations/lieux/ye_olde_magick_shoppe.png) |
| Bois | Quartier Résidentiel | `(365, 1310)` | [woods.png](illustrations/lieux/woods.png) |
| Société des Historiens | Quartier Sud | `(535, 1310)` | [historical_society.png](illustrations/lieux/historical_society.png) |
| Pension de Ma | Quartier Sud | `(765, 1095)` | [mas_boarding_house.png](illustrations/lieux/mas_boarding_house.png) |
| Église Méridionale | Quartier Sud | `(705, 1275)` | [south_church.png](illustrations/lieux/south_church.png) |

## Quartiers

Les coordonnées indiquent le centre de la case de rue de chaque quartier sur le plateau de la boîte de base.

| Quartier | Centre sur le plateau | Nouvelle illustration |
| --- | --- | --- |
| Quartier Nord | `(245, 265)` | [northside.png](illustrations/quartiers/northside.png) |
| Centre-Ville | `(447, 262)` | [downtown.png](illustrations/quartiers/downtown.png) |
| Quartier Est | `(548, 410)` | [easttown.png](illustrations/quartiers/easttown.png) |
| Quartier Marchand | `(315, 542)` | [merchant_district.png](illustrations/quartiers/merchant_district.png) |
| Quartier de la Rivière | `(532, 538)` | [rivertown.png](illustrations/quartiers/rivertown.png) |
| Université Miskatonic | `(318, 800)` | [miskatonic_u.png](illustrations/quartiers/miskatonic_u.png) |
| French Hill | `(535, 828)` | [french_hill.png](illustrations/quartiers/french_hill.png) |
| Quartier Résidentiel | `(355, 1137)` | [uptown.png](illustrations/quartiers/uptown.png) |
| Quartier Sud | `(553, 1142)` | [southside.png](illustrations/quartiers/southside.png) |

## Autres Mondes

Sur l’image du plateau, la première zone de chaque monde se trouve en bas et la seconde en haut : zone 1 en `(955, y)` puis zone 2 85 px plus haut environ (voir `first_area` et `second_area` dans [board.ron](data/board.ron)).

Depuis le 25/11/2026 (`board.ron` version 7), chaque monde a un réseau de lieux illustrés (`scenes`), recopié du tableau des liens du `.md` du monde. Chaque lieu mène à deux autres lieux. Une visite commence à un lieu tiré au hasard ; la 2e zone montre un des deux lieux où il mène. Les lieux qui ont deux images (`_day`, `_night`) sont montrés de nuit à partir du niveau de Terreur `night_from_terror` (5).

| Autre Monde | Dossier et génération retenue | Plan des lieux | Lieux | Images |
| --- | --- | --- | --- | --- |
| R’lyeh | [rlyeh/generation2](illustrations/autres%20mondes/v2/rlyeh/generation2) | [rlyeh_12_lieux.md](illustrations/autres%20mondes/v2/rlyeh/rlyeh_12_lieux.md) | 12 | 24 |
| Plateau de Leng | [plateau_de_leng/generation1](illustrations/autres%20mondes/v2/plateau_de_leng/generation1) | [lieux_plateau_de_leng.md](illustrations/autres%20mondes/v2/plateau_de_leng/lieux_plateau_de_leng.md) | 18 | 31 |
| Les Contrées du Rêve | [contrees_du_reve/generation1](illustrations/autres%20mondes/v2/contrees_du_reve/generation1) | [lieux_contrees_du_reve.md](illustrations/autres%20mondes/v2/contrees_du_reve/lieux_contrees_du_reve.md) | 18 | 29 |
| Grand Hall de Celeano | [celeano/generation1](illustrations/autres%20mondes/v2/celeano/generation1) | [lieux_celaeno.md](illustrations/autres%20mondes/v2/celeano/lieux_celaeno.md) | 12 | 14 |
| Yuggoth | [yuggoth/generation1](illustrations/autres%20mondes/v2/yuggoth/generation1) | [lieux_yuggoth.md](illustrations/autres%20mondes/v2/yuggoth/lieux_yuggoth.md) | 12 | 12 |
| Cité de la Grande Race | [city_of_great_race/generation3](illustrations/autres%20mondes/v2/city_of_great_race/generation3) | [cite_de_yith.md](illustrations/autres%20mondes/v2/city_of_great_race/cite_de_yith.md) | 12 | 24 |
| Les Abysses | [abysses/generation2](illustrations/autres%20mondes/v2/abysses/generation2) | [abysses_12_lieux.md](illustrations/autres%20mondes/v2/abysses/abysses_12_lieux.md) | 12 | 24 |
| Une Autre Dimension | [autre_dimension/generation3](illustrations/autres%20mondes/v2/autre_dimension/generation3) | [autre_dimension_12_lieux.md](illustrations/autres%20mondes/v2/autre_dimension/autre_dimension_12_lieux.md) | 12 | 12 |

Les anciennes images (deux par monde, `v1`) et les générations non retenues sont rangées dans `illustrations_archives/autres mondes/`, hors du dossier `illustrations/` : le programme ne les intègre pas.

## Cases spéciales

Depuis le 22/11/2026 (`board.ron` version 6), une case spéciale peut avoir plusieurs visuels (`illustrations: [...]`). La scène en tire un au hasard, qui reste le même pendant tout le tour.

| Case | Centre sur le plateau | Illustrations |
| --- | --- | --- |
| Cellule de Prison | `(680, 425)` | [jail_cell.png](illustrations/lieux/jail_cell.png) |
| Perdu dans le Temps et l’Espace | `(760, 1515)` | [lost_in_time_and_space.png](illustrations/zones_speciales/lost_in_time_and_space.png), [lost_in_time_and_space_house.png](illustrations/zones_speciales/lost_in_time_and_space_house.png) |

## Monstres

Les jetons d’origine sont référencés dans [monsters.ron](data/monsters.ron). Chaque portrait PNG porte l’identifiant du monstre.

| Monstre | Jeton original | Portrait retenu |
| --- | --- | --- |
| Byakhee | [byakhee.jpg](assets/monstres/byakhee.jpg) | [byakhee.png](illustrations/monstres/byakhee.png) |
| Chien de Tindalos | [chien_de_tindalos.jpg](assets/monstres/chien_de_tindalos.jpg) | [hound_of_tindalos.png](illustrations/monstres/hound_of_tindalos.png) |
| Chose Très Ancienne | [chose_tres_ancienne.jpg](assets/monstres/chose_tres_ancienne.jpg) | [elder_thing.png](illustrations/monstres/elder_thing.png) |
| Chthonien | [chthonien.jpg](assets/monstres/chthonien.jpg) | [chthonian.png](illustrations/monstres/chthonian.png) |
| Cultiste | [cultiste.jpg](assets/monstres/cultiste.jpg) | [cultist.png](illustrations/monstres/cultist.png) |
| Dhole | [dhole.jpg](assets/monstres/dhole.jpg) | [dhole.png](illustrations/monstres/dhole.png) |
| Dieu de la Langue Sanglante | [dieu_de_la_langue_sanglante.jpg](assets/monstres/dieu_de_la_langue_sanglante.jpg) | [god_of_the_bloody_tongue.png](illustrations/monstres/god_of_the_bloody_tongue.png) |
| Fantôme | [fantome.jpg](assets/monstres/fantome.jpg) | [ghost.png](illustrations/monstres/ghost.png) |
| Goule | [goule.jpg](assets/monstres/goule.jpg) | [ghoul.png](illustrations/monstres/ghoul.png) |
| Grand Prêtre | [grand_pretre.jpg](assets/monstres/grand_pretre.jpg) | [high_priest.png](illustrations/monstres/high_priest.png) |
| Gug | [gug.jpg](assets/monstres/gug.jpg) | [gug.png](illustrations/monstres/gug.png) |
| La Femme Boursouflée | [la_femme_boursouflee.jpg](assets/monstres/la_femme_boursouflee.jpg) | [the_bloated_woman.png](illustrations/monstres/the_bloated_woman.png) |
| Larve Amorphe | [larve_amorphe.jpg](assets/monstres/larve_amorphe.jpg) | [formless_spawn.png](illustrations/monstres/formless_spawn.png) |
| Larve Stellaire | [larve_stellaire.jpg](assets/monstres/larve_stellaire.jpg) | [star_spawn.png](illustrations/monstres/star_spawn.png) |
| Le Pharaon Noir | [le_pharaon_noir.jpg](assets/monstres/le_pharaon_noir.jpg) | [the_black_pharaoh.png](illustrations/monstres/the_black_pharaoh.png) |
| L’Homme en Noir | [lhomme_en_noir.jpg](assets/monstres/lhomme_en_noir.jpg) | [the_black_man.png](illustrations/monstres/the_black_man.png) |
| Maigre Bête de la Nuit | [maigre_bete_de_la_nuit.jpg](assets/monstres/maigre_bete_de_la_nuit.jpg) | [nightgaunt.png](illustrations/monstres/nightgaunt.png) |
| Maniaque | [maniaque.jpg](assets/monstres/maniaque.jpg) | [maniac.png](illustrations/monstres/maniac.png) |
| Migo | [migo.jpg](assets/monstres/migo.jpg) | [mi_go.png](illustrations/monstres/mi_go.png) |
| Polype Volant | [polype_volant.jpg](assets/monstres/polype_volant.jpg) | [flying_polyp.png](illustrations/monstres/flying_polyp.png) |
| Shoggoth | [shoggoth.jpg](assets/monstres/shoggoth.jpg) | [shoggoth.png](illustrations/monstres/shoggoth.png) |
| Sombre Rejeton | [sombre_rejeton.jpg](assets/monstres/sombre_rejeton.jpg) | [dark_young.png](illustrations/monstres/dark_young.png) |
| Sorcier | [sorcier.jpg](assets/monstres/sorcier.jpg) | [warlock.png](illustrations/monstres/warlock.png) |
| Sorcière | [sorciere.jpg](assets/monstres/sorciere.jpg) | [witch.png](illustrations/monstres/witch.png) |
| Tourmenteur des Ténèbres | [tourmenteur_des_tenebres.jpg](assets/monstres/tourmenteur_des_tenebres.jpg) | [haunter_of_the_dark.png](illustrations/monstres/haunter_of_the_dark.png) |
| Vagabond Dimensionnel | [vagabond_dimensionnel.jpg](assets/monstres/vagabond_dimensionnel.jpg) | [dimensional_shambler.png](illustrations/monstres/dimensional_shambler.png) |
| Vampire | [vampire.jpg](assets/monstres/vampire.jpg) | [vampire.png](illustrations/monstres/vampire.png) |
| Vampire de Feu | [vampire_de_feu.jpg](assets/monstres/vampire_de_feu.jpg) | [fire_vampire.png](illustrations/monstres/fire_vampire.png) |
| Zombie | [zombie.jpg](assets/monstres/zombie.jpg) | [zombie.png](illustrations/monstres/zombie.png) |
