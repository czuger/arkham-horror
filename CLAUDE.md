# CLAUDE.md

Guidance for Claude Code in this repository.

## Project

Browser adaptation of **Arkham Horror 2nd edition** (French edition, *Horreur à Arkham*), private use, **solo mode** (one human, the other investigators played by AIs), **base game only**. Project documents (`.md`) are in French, plain style; keep new ones so. Base-game images are in `assets/` (`assets/inventaire_assets.md` lists them, with a « Manquants » section).

Milestones (plan in `plan_creation_jeu.md`, J0–J6):

- **J0 foundation** (2026-11-16): Cargo workspace under `crates/`: `data`, `engine`, `ai` (libs), `simulator` (CLI), `server` (binary `arkham`).
- **J1 data** (2026-11-16): `data/*.ron` (board, gate markers, monsters, investigators, Ancient Ones, cards) loaded and cross-checked by `GameData::load`, `check`, `check_assets`. Effect vocabulary: types in `crates/data/src/cards/`, described in `vocabulaire_cartes.md` (update it when the vocabulary changes).
- **J2 engine** (2026-11-16): plays whole games.
- **J3 all cards** (2026-11-20): all 267 cards in `data/*_cards.ron` (raw text in `J3_releve_cartes.md`, notes in `J3.md`); location abilities in `board.ron`; 16 investigator abilities in `rules/investigator_powers.rs`; 16 cards with a coded special effect.
- **J4 AI, first version** (2026-11-20, `J4.md`): `ai::Policy`, priority-based. AI strength tuning and trades between investigators (Q12) postponed.
- **J5 server, first version** (2026-11-21, `J5.md`): home, new game, game page, journal, unlimited undo, resume. Still to do: the user plays a whole game in the browser. `static/htmx.min.js` may be absent: **never download it**, the user drops it in.

## Commands

- Build: `cargo build` (cold ≈ 2.5 min, RocksDB). Toolchain pinned to Rust 1.100.0.
- Text play (human = seat 0): `cargo run -p simulator -- jouer --graine 5 --joueurs 3`; watch an AI game: `… regarder --graine 5 --joueurs 3`; bulk: `cargo run --release -p simulator -- masse --parties 1000 --joueurs 3` (`--hasard` random answers, `--graine N` seed range, `--couverture FILE` writes what was never played). A bulk run that never ends = a hanging game: bisect with `--parties 1 --graine N`.
- Server: `cargo run -p server` from the repo root → http://127.0.0.1:3000. Env: `ARKHAM_ADDR`, `ARKHAM_DB` (default `arkham.db`), `ARKHAM_ASSETS`, `ARKHAM_STATIC`, `ARKHAM_DATA` (default `data`), `RUST_LOG`.
- Tests: `cargo test`; one test: `cargo test -p server a_game_resumes_after_a_restart`.
- Local CI: `scripts/ci.sh` (rustfmt, clippy `-D warnings`, tests, cargo-audit, cargo-deny); same in `.github/workflows/ci.yml`. No git remote.

## Architecture and constraints

- One executable: engine, AIs, axum server, embedded DB. Tokio **`current_thread`**. Local only (`127.0.0.1`, no login; hosting out of scope).
- **UI**: server-rendered HTML with **askama**, forms POST → redirect → GET, **htmx** and small JS served locally (never a CDN). No heavy client: no SPA, no rules or game state in the browser.
- AI turns and the Mythos phase play **in one block** after each human answer.
- Game data stays in versioned RON files, never in the DB or code. Card texts are transcribed by Claude from `assets/` images, proofread by the user; unknown copy counts are entered from memory and marked « à vérifier ».

Technical facts (details in `manques_et_questions.md` §2):

- **DB: PoloDB** (`polodb_core` 5.3, RocksDB under). SurrealDB was dropped (2026-11-16): never bring it back. Calls are synchronous, run on the runtime thread. The DB unlocks when the last `Database` clone is dropped: drop it before reopening the same folder. Store game state as a JSON **string** (BSON has no u64).
- One RustSec advisory (askama's build-time `basic-toml`) is ignored in `deny.toml` and `.cargo/audit.toml`; re-check on each askama upgrade.
- Cargo warns on unused deps: add inter-crate deps only when used.
- **RON files**: ids English snake_case, player-facing values French; start with `#![enable(implicit_some)]` and a `version` checked against the type's `VERSION`; `deny_unknown_fields`. Locations have no arrow: a monster goes to its own street. Board values need user proofreading; monster token images beat the wiki table.
- **Engine**: `engine::play(data, setup, responder)` replays from `Setup` (seed) and asks the `Responder` for each `Decision` (labelled options, answer = index); `Replay` feeds recorded answers, `Waiting` stops at the first open decision. Rules are `impl Game` blocks in `crates/engine/src/rules/`, one topic per file, unwinding with `Halt` via `?`. Ancient One and monster texts are coded by id in `rules/ancient_one_*.rs`, `rules/monster_*.rs`. All randomness via `Game::die` / in-repo `Rng` (xoshiro256**, output pinned by a test: never change it). `BTreeMap`, never `HashMap`, in the state. Unit tests in `crates/engine/src/tests/` force dice with `run(state, answers, dice, |game| …)`.
- **AI**: `Responder::answer(&mut self, view: &View, decision: &Decision) -> Option<usize>`. `View` = read-only game window. Each `Decision` carries a `Context` and each option its card `effects`: keep them filled when adding decisions. `Outcome.tally` counts game events; `ai::intention` is the AI's turn plan. `crates/ai/tests/ai_games.rs` plays 160 games (≈35 s debug).
- **Server**: no game state in memory. Collection `partie`, one `GameRow` per game, `record` = JSON of `Progress` (setup, all answers, human answer positions for undo, journal lines seen). Each request replays with `Replay` + `ai::Seated`; `engine::inspect` gives a `View` for page models in `crates/server/src/views/`. Forms carry the move number (`coup`) to ignore stale posts; `same_origin.rs` refuses cross-site posts. HTTP tests in `crates/server/src/tests/`.

## Reference files

| File | Role |
|---|---|
| `plan_creation_jeu.md` | **Current plan. Read first.** |
| `manques_et_questions.md` | Missing data (M), technical points (T), rules (R), questions for the user (Q). |
| `plan_mise_en_ligne.md` | Old plan (WASM), superseded. |
| `J3.md`, `J4.md`, `J5.md` | Milestone notes. |
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
