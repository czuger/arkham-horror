# Horreur à Arkham

*Horreur à Arkham* (2e édition, jeu de base) jouable dans le navigateur, en solo : vous jouez un investigateur, l'ordinateur joue les autres. Usage privé.

## Ce qu'il faut

- **Rust** installé avec `rustup` (https://rustup.rs). La bonne version (1.100.0) s'installe toute seule au premier lancement, grâce au fichier `rust-toolchain.toml`.
- Un compilateur C++ (sur Mac : `xcode-select --install`), pour la base de données.

## Lancer le serveur

Depuis le dossier du projet (c'est important : le serveur y cherche les images et les données) :

```
cargo run --release -p server
```

Puis ouvrir http://127.0.0.1:3000 dans le navigateur.

- La **première fois**, la compilation prend quelques minutes. Les fois suivantes, quelques secondes.
- Pour **arrêter** le serveur : `Ctrl+C` dans le terminal.
- Les parties sont enregistrées dans le dossier `arkham.db`, créé au premier lancement. Elles reprennent là où vous les avez laissées après un redémarrage.

## Réglages (facultatifs)

Par variables d'environnement, à mettre devant la commande, par exemple `ARKHAM_ADDR=127.0.0.1:8080 cargo run --release -p server` :

| Variable | Rôle | Par défaut |
|---|---|---|
| `ARKHAM_ADDR` | adresse et port d'écoute | `127.0.0.1:3000` |
| `ARKHAM_DB` | dossier de la base des parties | `arkham.db` |
| `ARKHAM_DATA` | dossier des données du jeu (cartes, plateau…) | `data` |
| `ARKHAM_ASSETS` | dossier des images | `assets` |
| `ARKHAM_STATIC` | dossier de la feuille de style (et de `htmx.min.js` s'il y est) | `static` |
| `RUST_LOG` | niveau de détail des messages (`info`, `debug`…) | — |

## Pour aller plus loin

- Jouer en mode texte dans le terminal : `cargo run -p simulator -- jouer --joueurs 3`.
- Regarder une partie jouée entièrement par l'ordinateur : `cargo run -p simulator -- regarder --joueurs 3`.
- Lancer les tests : `cargo test` ; tous les contrôles : `scripts/ci.sh`.
- Le plan du projet est dans `plan_creation_jeu.md`, l'état du serveur dans `J5.md`.
