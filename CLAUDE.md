# CLAUDE.md

Guidance for Claude Code in this repository.

## Project

Browser adaptation of **Arkham Horror 2nd edition** (French edition, *Horreur à Arkham*), private use, **solo mode** (one human, the other investigators played by AIs), **base game only**. Project documents (`.md`) are in French, plain style; keep new ones so. Base-game images are in `assets/` (`assets/inventaire_assets.md` lists them, with a « Manquants » section).

Milestones (plan in `plan_creation_jeu.md`, J0–J7):

- **J0 foundation** (2026-11-16): Cargo workspace under `crates/`: `data`, `engine`, `ai` (libs), `simulator` (CLI), `server` (binary `arkham`).
- **J1 data** (2026-11-16): `data/*.ron` (board, gate markers, monsters, investigators, Ancient Ones, cards) loaded and cross-checked by `GameData::load`, `check`, `check_assets`. Effect vocabulary: types in `crates/data/src/cards/`, described in `vocabulaire_cartes.md` (update it when the vocabulary changes).
- **J2 engine** (2026-11-16): plays whole games.
- **J3 all cards** (2026-11-20): all 267 cards in `data/*_cards.ron` (raw text in `J3_releve_cartes.md`, notes in `J3.md`); location abilities in `board.ron`; 16 investigator abilities in `rules/investigator_powers.rs`; 16 cards with a coded special effect.
- **J4 AI, first version** (2026-11-20, `J4.md`): `ai::Policy`, priority-based. AI strength tuning and trades between investigators (Q12) postponed.
- **J5 server, first version** (2026-11-21, `J5.md`): home, new game, game page, journal, unlimited undo, resume, one page per investigator where the human sets sliders; a combat page (monster, every die, hands by drag and drop); cards drawn and dice shown in a journal column (`Outcome.shown`). Still to do: the user plays a whole game in the browser. `static/htmx.min.js` may be absent: **never download it**, the user drops it in.
- **J6 illustrated display** (2026-11-22, `J6.md`): the game page centres on a scene (illustration of where the human stands: location, district, Other World area 1/2, the monster faced, dark fallback otherwise) with a small server-drawn SVG map on the left whose offered places are real buttons and others preview (`?voir=<key>`); fits the window. Illustration fields in `board.ron` v5 / `monsters.ron` v2, checked by `every_illustration_exists_and_none_is_orphan`; `crates/server/build.rs` shrinks `illustrations/` to JPEG (grand 1280 px, vignette 320 px) embedded and served under `illustrations/` with a fingerprint. The landscape board page is gone. The old J6 « Livraison locale » is now J7.

## Commands

- Build: `cargo build`. Toolchain pinned to Rust 1.100.0.
- Text play (human = seat 0): `cargo run -p simulator -- jouer --graine 5 --joueurs 3`; watch an AI game: `… regarder --graine 5 --joueurs 3`; bulk: `cargo run --release -p simulator -- masse --parties 1000 --joueurs 3` (`--hasard` random answers, `--graine N` seed range, `--couverture FILE` writes what was never played). A bulk run that never ends = a hanging game: bisect with `--parties 1 --graine N`.
- Server: `cargo run -p server` → http://127.0.0.1:3000. Env: `ARKHAM_ADDR`, `ARKHAM_SOCKET` (Unix socket instead of TCP), `ARKHAM_DATA_DIR` (persistent folder), `ARKHAM_DB` (default `$ARKHAM_DATA_DIR/arkham.db`, else `arkham.db`), `RUST_LOG`. `data/`, `assets/`, `static/` are built into the binary with rust-embed (`embedded_files.rs`; read from disk in debug builds).
- Deploy: `cargo deploy` (settings in `[package.metadata.deploy]` of `crates/server/Cargo.toml`, served under `/arkham/`). The user runs it; Claude only runs `cargo deploy init`/`render` (local), never the commands that touch the server.
- Tests: `cargo test`; one test: `cargo test -p server a_game_resumes_after_a_restart`.
- **For now (user's request, 2026-11-21): do not run the AI tests.** No `simulator masse` bulk runs, no `crates/ai/tests/ai_games.rs`; test with `cargo test --workspace --exclude ai` until the user says otherwise.
- Local CI: `scripts/ci.sh` (rustfmt, clippy `-D warnings`, tests, cargo-audit, cargo-deny); same in `.github/workflows/ci.yml`. No git remote.

## Architecture and constraints

- One executable: engine, AIs, axum server, embedded DB. Tokio **`current_thread`**. No login. Locally on `127.0.0.1`; deployed on a Unix socket behind nginx under a sub-path, so **every link is relative** (no leading `/`): pages carry `<base href>` from `relative_path::root_of`, redirects use `relative_path::redirect_from`, `Session::page` paths have no leading `/`. SIGTERM stops cleanly.
- **UI**: server-rendered HTML with **askama**, forms POST → redirect → GET, **htmx** and small JS served locally (never a CDN). No heavy client: no SPA, no rules or game state in the browser.
- AI turns and the Mythos phase play **in one block** after each human answer.
- Game data stays in versioned RON files, never in the DB or code. Card texts are transcribed by Claude from `assets/` images, proofread by the user; unknown copy counts are entered from memory and marked « à vérifier ».

Technical facts (details in `manques_et_questions.md` §2):

- **DB: SQLite via sqlx** (0.9, `runtime-tokio`, `sqlite-bundled`), since 2026-11-21; PoloDB (2026-11-16) and SurrealDB were dropped: never bring them back. One file (`arkham.db`), code in `crates/server/src/database.rs`, schema steps in `crates/server/migrations/` applied by `schema.rs` (counted by `PRAGMA user_version`; never edit a released step, add one). Game state stored as JSON text. Rules:
  - Connection via `SqliteConnectOptions`, never a bare URL: journal_mode WAL, synchronous NORMAL, foreign_keys ON, busy_timeout 5 s, create_if_missing (writer).
  - SQLite has a single writer: every write goes through a write pool with `max_connections(1)`; reads go through a separate `read_only(true)` pool. Call `Database::close` before exiting.
  - Every table is `STRICT`. Types only INTEGER, REAL, TEXT, BLOB, ANY. Mapping: bool → `INTEGER CHECK (x IN (0,1))`; datetime → TEXT ISO 8601 UTC; money → INTEGER cents; UUID → TEXT; JSON → `TEXT CHECK (json_valid(x))`; max length → `TEXT CHECK (length(x) <= N)`.
  - NOT NULL by default; `id INTEGER PRIMARY KEY`; foreign keys with explicit `ON DELETE` and an index on each FK column.
  - `*.db`, `*.db-wal`, `*.db-shm` are gitignored.
- One RustSec advisory (askama's build-time `basic-toml`) is ignored in `deny.toml` and `.cargo/audit.toml`; re-check on each askama upgrade.
- Cargo warns on unused deps: add inter-crate deps only when used.
- **RON files**: ids English snake_case, player-facing values French; start with `#![enable(implicit_some)]` and a `version` checked against the type's `VERSION`; `deny_unknown_fields`. Locations have no arrow: a monster goes to its own street. Board values need user proofreading; monster token images beat the wiki table.
- **Engine**: `engine::play(data, setup, responder)` replays from `Setup` (seed) and asks the `Responder` for each `Decision` (labelled options, answer = index); `Replay` feeds recorded answers, `Waiting` stops at the first open decision. Rules are `impl Game` blocks in `crates/engine/src/rules/`, one topic per file, unwinding with `Halt` via `?`. Ancient One and monster texts are coded by id in `rules/ancient_one_*.rs`, `rules/monster_*.rs`. All randomness via `Game::die` / in-repo `Rng` (xoshiro256**, output pinned by a test: never change it). `BTreeMap`, never `HashMap`, in the state. Unit tests in `crates/engine/src/tests/` force dice with `run(state, answers, dice, |game| …)`.
- **Dice rolls (user's rule, 2026-11-21)**: every roll is shown in the journal the same way: one die glyph per die (⚀…⚅, successes in green), then « N succès » (or « résultat : N » / « total : N » for a roll counted by its value; a roll of no dice shows « aucun dé : 0 succès »). So every roll emits `Exhibit::Dice`: checks via `Game::roll_dice`, value rolls via `Game::die_shown` / `Game::dice_shown`; never a bare `Game::die` for a roll the player should see.
- **AI**: `Responder::answer(&mut self, view: &View, decision: &Decision) -> Option<usize>`. `View` = read-only game window. Each `Decision` carries a `Context` and each option its card `effects`: keep them filled when adding decisions. `Outcome.tally` counts game events; `ai::intention` is the AI's turn plan. `crates/ai/tests/ai_games.rs` plays 160 games (≈35 s debug).
- **Server**: no game state in memory. Table `game`, one `GameRow` per game, `record` = JSON of `Progress` (setup, all answers, human answer positions for undo, journal lines seen). Each request replays with `Replay` + `ai::Seated`; `engine::inspect` gives a `View` for page models in `crates/server/src/views/`. A saved game whose answers no longer fit the rules (illegal answer on replay, after a rules change) is **deleted**, at startup and when opened (user's rule, 2026-11-21); never try to repair it. Forms carry the move number (`coup`) to ignore stale posts; `same_origin.rs` refuses cross-site posts. HTTP tests in `crates/server/src/tests/`.

## Reference files

| File | Role |
|---|---|
| `plan_creation_jeu.md` | **Current plan. Read first.** |
| `manques_et_questions.md` | Missing data (M), technical points (T), rules (R), questions for the user (Q). |
| `plan_mise_en_ligne.md` | Old plan (WASM), superseded. |
| `J3.md`, `J4.md`, `J5.md`, `J6.md` | Milestone notes. |
| `vocabulaire_cartes.md` | Card effect vocabulary. |
| `regles/decisions.md` | Rules settled with the user (R1–R27). Authoritative. |
| `regles/base.md` | French rulebook; §22 inconsistencies; §23 official FAQ (overrides the rulebook). |
| `inventaire.md` | Inventory of the scraped sources and what is missing. |

**Scraped sources**: `/Users/ced/python/perso/scraper/pages/wiki.horreuraarkham.fr/` (flat folder, images named `images__x__xy__<name>`; ignore `_mom`/`_MoM` and `_es` pages).

- **Only allowed source. Never download anything**; if something is missing, tell the user (they re-run the scraper).
- **Read-only**: write outputs to the scratchpad or the project.
- Recount on disk before saying a file is missing.

## Domain rules

- Before coding a rule, check `regles/decisions.md`, then `regles/base.md` §23 and §22. A new rule gap: never decide silently, ask the user, then record it in `decisions.md` with date and reason.
- Golden rule: when in doubt, the outcome worst for the investigator.
- **Vigueur** = the skill once called « Combat » (*Fight*).
- Card texts are copyrighted: the repository stays **private**.

## Rust code guidelines

From the user's style guide (2026-11-16), enforced by rustfmt and clippy `-D warnings`.

- **Language**: code, identifiers, comments, docs, commits, logs in English. User-facing text (messages, CLI output, labels, templates) in French with accents; never translate it. RON keys and effect names English, player-read values French.
- **Imports**: `crate::` paths; `super::` only one level (e.g. tests); no globs except preludes or `use super::*;` in tests.
- **Types**: annotate all `pub` signatures; let inference handle locals; concrete types over generics unless genericity is really used; no `Box<dyn Any>` without a justifying comment; no `Box<dyn Error>` in libraries (fine in `main`).
- **Docs**: rustdoc on every `pub` item, with `# Arguments`/`# Returns`/`# Errors`/`# Panics` as needed, proportionate; private items only when non-obvious. `#![warn(missing_docs)]` everywhere, `#![deny(missing_docs)]` in `data`, `engine`, `ai`.
- **Comments**: only for non-obvious logic, short, about intent.
- **Errors**: `Result` / `Option`, propagate with `?`; real error enums (or `thiserror`), no string errors. No `unwrap`/`expect` outside tests unless provably safe with a `// safe:` comment. Never silently drop errors (`.ok()`, `let _ =`). `panic!`/`unreachable!` only for invariant violations.
- **Size**: soft limit 50 lines per function; split into small named functions; early returns over nesting.
- **Files**: one primary public type per file, file named after it (small tightly coupled types may share it; a family of tiny types may live together). Traits with several implementors get their own file. No monolithic files; `mod.rs`, `lib.rs`, `main.rs` thin (declarations, wiring, re-exports).
- Readability over cleverness, explicit over implicit, composition over hierarchies, idiomatic clippy/rustfmt style.
