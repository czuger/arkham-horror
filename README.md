# Horreur à Arkham

*Horreur à Arkham* (2e édition, jeu de base) jouable dans le navigateur, en solo : vous jouez un investigateur, l'ordinateur joue les autres. Usage privé.

## Ce qu'il faut

- **Rust** installé avec `rustup` (https://rustup.rs). La bonne version (1.100.0) s'installe toute seule au premier lancement, grâce au fichier `rust-toolchain.toml`.
- Un compilateur C (sur Mac : `xcode-select --install`), pour la base de données SQLite.

## Lancer le serveur

Depuis le dossier du projet :

```
cargo run --release -p server
```

Puis ouvrir http://127.0.0.1:3000 dans le navigateur.

- La **première fois**, la compilation prend quelques minutes. Les fois suivantes, quelques secondes.
- Pour **arrêter** le serveur : `Ctrl+C` dans le terminal.
- Les parties sont enregistrées dans le fichier `arkham.db` (base SQLite), créé au premier lancement. Elles reprennent là où vous les avez laissées après un redémarrage. Sauvegarde : copier ce fichier, serveur arrêté.

## Réglages (facultatifs)

Par variables d'environnement, à mettre devant la commande, par exemple `ARKHAM_ADDR=127.0.0.1:8080 cargo run --release -p server` :

| Variable | Rôle | Par défaut |
|---|---|---|
| `ARKHAM_ADDR` | adresse et port d'écoute | `127.0.0.1:3000` |
| `ARKHAM_SOCKET` | si elle est définie, le serveur écoute sur ce socket Unix au lieu de l'adresse ci-dessus | — |
| `ARKHAM_DATA_DIR` | dossier des données qui doivent durer (la base des parties) | dossier courant |
| `ARKHAM_DB` | fichier de la base des parties | `$ARKHAM_DATA_DIR/arkham.db`, sinon `arkham.db` |
| `RUST_LOG` | niveau de détail des messages (`info`, `debug`…) | — |

Les données du jeu (`data/`), les images (`assets/`) et `static/` sont intégrées au programme : il fonctionne seul, sans ces dossiers à côté. En développement (`cargo run` sans `--release`), il les relit sur le disque à chaque page : une modification se voit sans recompiler.

## Déploiement

Le serveur se déploie avec `cargo deploy` (outil personnel) : il compile le programme, l'envoie sur le serveur et le lance comme service, derrière un nginx qui le sert sous `/arkham/`. Les réglages propres à l'application sont dans la section `[package.metadata.deploy]` de `crates/server/Cargo.toml` ; ceux du serveur viennent des variables d'environnement `DEPLOY_*` de la machine, jamais d'un fichier du dépôt.

- `cargo deploy setup` : prépare le serveur (une seule fois).
- `cargo deploy` : déploie la version actuelle.
- `cargo deploy rollback` : revient à la version précédente.
- `cargo deploy status`, `cargo deploy logs` : état du service, journaux.

## Pour aller plus loin

- Jouer en mode texte dans le terminal : `cargo run -p simulator -- jouer --joueurs 3`.
- Regarder une partie jouée entièrement par l'ordinateur : `cargo run -p simulator -- regarder --joueurs 3`.
- Lancer les tests : `cargo test` ; tous les contrôles : `scripts/ci.sh`.
- Le plan du projet est dans `plan_creation_jeu.md`, l'état du serveur dans `J5.md`.
