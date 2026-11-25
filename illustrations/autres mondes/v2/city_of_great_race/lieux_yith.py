#!/usr/bin/env python3
"""
Génère N lieux (8 par défaut) de la cité de la Grand-Race de Yith pour Horreur à Arkham.
Chaque lieu mène à 2 lieux tirés au hasard (les deux destinations peuvent être
identiques). Le graphe est retiré jusqu'à ce que tous les lieux soient
atteignables depuis le lieu de départ, pour garantir une progression.

Usage :
    python3 generer_lieux_yith.py                 # tirage aléatoire
    python3 generer_lieux_yith.py --seed 42       # tirage reproductible
    python3 generer_lieux_yith.py -o cite.md      # fichier de sortie
"""
import argparse
import random

CONTEXTE = (
    "Ruines millénaires abandonnées, à demi ensevelies dans le désert australien "
    "aride, de nos jours. Aucun personnage, aucune créature, aucune trace d'eau ; "
    "seulement de la pierre érodée, du sable et des objets brisés."
)

STYLE = (
    "Peinture numérique réaliste à la texture de gouache, ocre doré, ombres olive "
    "et bronze oxydé. Paysage 16:9, 1536 × 864 px, scène unique plein cadre, "
    "sans inscriptions, bordure, signature ni filigrane."
)

LIEUX = [
    ("Les Sables de l'Entrée",
     "Sous un ciel étoilé immense, des blocs cyclopéens sculptés et fendus "
     "émergent à peine des dunes de sable rouge. Une ouverture sombre s'enfonce "
     "sous la terre entre deux dalles basculées."),
    ("La Grande Avenue des Rampes",
     "Avenue colossale comblée de sable, bordée de tours de basalte écroulées aux "
     "toits convexes éventrés. De larges rampes inclinées, brisées, mènent vers "
     "des niveaux disparus, sous un soleil écrasant."),
    ("Les Jardins Pétrifiés",
     "Terrasses en surplomb couvertes de sable, où ne restent que des troncs de "
     "fougères géantes fossilisés et des empreintes de plantes dans la pierre. "
     "Des balustrades massives effondrées dominent un désert sans fin."),
    ("Les Archives Ensevelies",
     "Bibliothèque souterraine immense et silencieuse. Des étagères rectilignes "
     "renversées, des étuis de métal grisâtre cabossés et ouverts jonchent le "
     "sol, la poussière flotte dans un rai de lumière tombé d'une fissure."),
    ("La Salle des Machines Mortes",
     "Vaste salle voûtée à moitié remplie de sable, des machines étranges de "
     "métal corrodé et de cristal fêlé, démantelées et figées dans des alcôves "
     "vides, éclairées par une lumière pâle venue d'en haut."),
    ("Le Port Asséché",
     "Quais de pierre cyclopéens dominant une ancienne mer devenue plaine de sel "
     "craquelé et de sable, des carcasses d'embarcations aux formes non humaines "
     "réduites à des armatures de métal rongé."),
    ("Les Ruines de Basalte Interdites",
     "Édifice de basalte noir sans fenêtres, plus ancien que la cité, une trappe "
     "massive scellée au sol, fissurée, du sable qui s'écoule dans ses fentes, "
     "une impression de menace venue d'en dessous."),
    ("Le Puits des Polypes",
     "Gouffre vertical aux parois de pierre lisse, des débris et des blocs "
     "effondrés suspendus au bord, des tourbillons de poussière, une lumière qui "
     "faiblit vers des profondeurs insondables."),
    ("La Tour Tronquée",
     "Sommet décapité d'une tour titanesque aux fenêtres voûtées béantes, des "
     "instruments astronomiques inconnus tordus et brisés, une vue sur des ruines "
     "noyées de dunes au crépuscule."),
    ("Le Hall des Bas-Reliefs",
     "Galerie monumentale aux colonnes renversées, des murs ornés de larges "
     "bandes de motifs curvilignes abstraits, érodés et écaillés, des rayons de "
     "lumière poussiéreux à travers la voûte crevée."),
    ("Les Forges Éteintes",
     "Atelier souterrain où d'énormes mécanismes de métal oxydé et de cristal "
     "terne sont éventrés, des carcasses de véhicules aux formes étranges "
     "démontées, du sable accumulé contre les parois."),
    ("Le Couloir Effondré",
     "Galerie souterraine écroulée, des blocs cyclopéens fendus, du sable qui "
     "s'infiltre en cascades sèches par les fissures, une étroite clarté au "
     "fond du passage."),
]


def tirer_liens(n, rng):
    """Chaque lieu mène à 2 lieux (autres que lui-même, doublon possible)."""
    return {i: [rng.choice([j for j in range(n) if j != i]) for _ in range(2)]
            for i in range(n)}


def tous_atteignables(liens, depart=0):
    vus, pile = {depart}, [depart]
    while pile:
        for j in liens[pile.pop()]:
            if j not in vus:
                vus.add(j)
                pile.append(j)
    return len(vus) == len(liens)


def generer(seed=None, nb=5):
    rng = random.Random(seed)
    lieux = rng.sample(LIEUX, nb)
    while True:
        liens = tirer_liens(nb, rng)
        if tous_atteignables(liens):
            return lieux, liens


def markdown(lieux, liens, seed):
    noms = [nom for nom, _ in lieux]
    out = ["# La Cité de Yith : lieux et progression", ""]
    if seed is not None:
        out += [f"_Tirage : seed {seed}_", ""]
    out += ["## Carte des liens", "", "```mermaid", "flowchart LR"]
    for i, nom in enumerate(noms):
        out.append(f'    L{i + 1}["{i + 1}. {nom}"]')
    for i, dest in liens.items():
        for j in dict.fromkeys(dest):  # un trait par destination distincte
            out.append(f"    L{i + 1} --> L{j + 1}")
    out += ["```", "", "## Tableau des liens", "",
            "| # | Lieu | Mène à | Mène à |", "|---|------|--------|--------|"]
    for i, nom in enumerate(noms):
        a, b = liens[i]
        out.append(f"| {i + 1} | {nom} | {a + 1}. {noms[a]} | {b + 1}. {noms[b]} |")
    out += ["", "## Lieux et prompts", ""]
    for i, (nom, desc) in enumerate(lieux):
        a, b = liens[i]
        dests = f"**{a + 1}. {noms[a]}**" if a == b else \
            f"**{a + 1}. {noms[a]}** et **{b + 1}. {noms[b]}**"
        role = " (départ)" if i == 0 else ""
        out += [f"### {i + 1}. {nom}{role}", "", f"Mène à : {dests}", "",
                "```", f"{CONTEXTE} {desc} {STYLE}", "```", ""]
    return "\n".join(out)


def main():
    p = argparse.ArgumentParser(description=__doc__,
                                formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument("--seed", type=int, default=None, help="graine du tirage")
    p.add_argument("-o", "--output", default="cite_de_yith.md", help="fichier Markdown")
    p.add_argument("-n", "--nb", type=int, default=8,
                   help=f"nombre de lieux (3 à {len(LIEUX)}, défaut 8)")
    args = p.parse_args()
    if not 3 <= args.nb <= len(LIEUX):
        p.error(f"--nb doit être entre 3 et {len(LIEUX)}")
    lieux, liens = generer(args.seed, args.nb)
    with open(args.output, "w", encoding="utf-8") as f:
        f.write(markdown(lieux, liens, args.seed))
    print(f"Écrit : {args.output}")


if __name__ == "__main__":
    main()

