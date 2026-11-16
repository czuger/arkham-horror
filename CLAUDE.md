# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project status

A browser adaptation of **Arkham Horror 2nd edition** (French edition, *Horreur à Arkham*), for private use: **solo mode** (one human player, the other investigators played by AIs), **base game only**.

Milestone **J0 (foundation) is done** (2026-11-16): a Cargo workspace under `crates/` (`data`, `engine`, `ai` are empty library stubs; `simulator` is a stub binary; `server` builds the `arkham` binary, which serves a « bonjour » page that counts visits in PoloDB). Milestone **J1 (data) is done** (2026-11-16): `data/` holds `board.ron`, `gate_markers.ron`, `monsters.ron`, `investigators.ron`, `ancient_ones.ron`, and a 39-card sample in `location_cards.ron`, `gate_cards.ron`, `mythos_cards.ron`, `investigator_cards.ron`, all loaded and cross-checked by the `data` crate (`GameData::load`, `check`, `check_assets`; tests in `crates/data/tests/` load the real files). The card-effect vocabulary (types in `crates/data/src/cards/`) is described in French in `vocabulaire_cartes.md`. Next: J2 (engine). The rest of the cards is transcribed later, on the same vocabulary. The repository also holds planning and reference documents, all written in French, plus the base-game images in `assets/`. Keep new project documents in French, matching their plain style.

## Commands

- Build: `cargo build` (cold build ≈ 2.5 min, mostly RocksDB's C++ under PoloDB; incremental: seconds). Toolchain pinned to Rust 1.100.0 in `rust-toolchain.toml`.
- Run the server: `cargo run -p server` → http://127.0.0.1:3000. Settings via env vars `ARKHAM_ADDR`, `ARKHAM_DB` (database folder, default `arkham.db`), `ARKHAM_ASSETS`, `ARKHAM_STATIC`; logs via `RUST_LOG`. Run it from the repo root so `assets/` and `static/` resolve.
- Tests: `cargo test`; a single test: `cargo test -p server visits_are_counted`.
- Full local CI: `scripts/ci.sh` (rustfmt, clippy `-D warnings`, tests, cargo-audit, cargo-deny). The same steps are in `.github/workflows/ci.yml`; there is no git remote yet.

Technical facts found at J0 (details in `manques_et_questions.md` §2):

- Database: **PoloDB** (`polodb_core` 5.3, MongoDB-like, serde documents, RocksDB underneath). SurrealDB was tried first and dropped by the user on 2026-11-16 (too many constraints); do not bring it back.
- PoloDB calls are synchronous and short; they run directly on the single tokio `current_thread` runtime. RocksDB adds 3 helper threads.
- The database is flushed and unlocked when the last `Database` clone is dropped; drop it before reopening the same folder (tests do).
- Game state is meant to be stored as a JSON **string** field (BSON has no unsigned 64-bit integers).
- One RustSec advisory with no upstream fix is ignored, with its reason, in both `deny.toml` and `.cargo/audit.toml` (askama's build-time `basic-toml`). Re-check it on every askama upgrade.
- Data files (`data/*.ron`): ids are English snake_case (`train_station`), player-facing values French; each file starts with `#![enable(implicit_some)]` and a `version` checked against the type's `VERSION`; structs use `deny_unknown_fields`. Locations have no arrow field: a monster in a location always goes to its own street. Board values were read from the board image and must be proofread by the user; the wiki monster table had errors, token images are authoritative.
- Rust 1.100 cargo warns on unused crate dependencies: add inter-crate deps (`engine` → `data`, etc.) only when the code uses them.

| File | Role |
|---|---|
| `plan_creation_jeu.md` | **The current plan (2026-11-16). Read this first.** Architecture (axum + tokio single-thread server, embedded database — now PoloDB, server-rendered HTML with minimal JS), crate layout, data format, engine design (decision/response model), AI, milestones J0–J6, tests, hosting. |
| `manques_et_questions.md` | Up-to-date list of missing data (M1–M24), technical points to verify (T1–T6), rules R1–R8 (settled, see `regles/decisions.md`) and questions for the user (Q1–Q23); §2 also records the J0 measurements. |
| `plan_mise_en_ligne.md` | The earlier plan (WASM client, no server). Its architecture is superseded; its Parts 2–3 (missing data M1–M20, open questions) are kept for reference. |
| `vocabulaire_cartes.md` | How card effects are written in the data files: effect primitives, lasting effects, abilities, the « effet spécial » list, the sample and the load-time checks. Update it whenever the vocabulary changes. |
| `regles/decisions.md` | Rules the rulebook leaves open, settled with the user (date and reason each). Authoritative for the engine. |
| `regles/base.md` | Base-game rules, transcribed from the official French rulebook (Edge 2016 revised edition). Transcription notes are marked ⚠ in italics; §22 lists rulebook inconsistencies; §23 holds the base-game part of the official FAQ v2.0 (2017: card errata + rulings), which overrides the rulebook. |
| `inventaire.md` | Inventory of the source material (a 2017 scrape of the French wiki `wiki.horreuraarkham.fr`): what each HTML/PDF file contains and what data is still missing (§7–§8). |

The scraped sources live **outside** this repo, at `~/python/perso/scraper/pages/wiki.horreuraarkham.fr/` (= `/Users/ced/python/perso/scraper/pages/wiki.horreuraarkham.fr/`). As of 2026-11-16 the folder holds 953 HTML, 66 PDF and about 3,100 wiki images, all in one flat folder (images are named `images__x__xy__<name>`). They mix three games; only Arkham Horror 2nd ed. is relevant — ignore *Mansions of Madness* (`_mom`/`_MoM` suffix) and *Elder Sign* (`_es` suffix) pages.

- **This local scrape is the only allowed source. Never download anything** (wiki, archive.org or elsewhere). If something is missing, tell the user; they re-run the scraper themselves.
- Treat the scrape folder as **read-only**. Write outputs to the scratchpad or the project, never next to the sources.
- Counts change when the user re-runs the scraper, so recount on disk before saying a file is missing.

Base-game images are sorted by theme in `assets/` (plateau, cartes, fiches, monstres, marqueurs, errata, regles, illustrations). `assets/inventaire_assets.md` lists every file with its source and has a « Manquants » section.

## Planned architecture (from `plan_creation_jeu.md`)

One executable holds everything: rules engine, AIs, web server and embedded database.

- **Server**: axum + tokio on a **single thread** (`current_thread` runtime). No game state in memory: each request loads the game from the DB, applies the answer, lets the AIs play up to the next human decision, then saves.
- **Database**: PoloDB embedded (collections `partie`, `action`, `evenement`). Game data (cards, board…) stays in versioned RON files under `data/`, never in the DB or the code.
- **UI**: server-rendered HTML with **askama** (user's choice, 2026-11-16), classic forms with POST → redirect → GET, plus **htmx** (served locally, no CDN) to swap page fragments. **No heavy JavaScript** and no client-side state; a few small hand-written optional scripts at most. Must stay playable without JS. Card and token images come from `assets/`.
- AI turns and the Mythos phase are played **in one block** after each human answer; the human follows them in the journal.
- **Hosting and access control are out of scope** (the user has a dedicated environment). The project only has to run locally: listen on `127.0.0.1`, no login page.
- Cargo workspace: `data` (types + loading + consistency checks), `engine` (rules: deterministic, serializable, no I/O), `ai` (policies answering decisions), `server` (axum), `simulator` (CLI for mass AI-only games; kept out of the server because it would block the single thread).
- **Engine model**: the engine advances until someone must choose, then exposes a *Decision* (who, what, legal options). Humans answer through forms, AIs through a policy function; the engine validates both the same way and refuses illegal answers. Dice come from the engine's seeded RNG. Seed + ordered answers replay a game exactly, which gives save/resume, undo and reproducible tests.
- Card effects are written in the data files with a small vocabulary of effect primitives; the rare cards that do not fit are flagged « effet spécial » and coded by name.
- Deliverable: single binary + `data/` + `assets/` + htmx file + DB file created on first run, configured by file or environment variables.
- Card texts are transcribed by Claude from the images in `assets/` (user's decision, 2026-11-16), then proofread by the user against the physical box. Copy counts per card are pending the user's own research: enter them from memory and mark them « à vérifier ».

Planned tests: unit tests per phase and rule, property tests (no impossible states, no negative resources), data consistency checks, one auto-generated test per card, thousands of AI-only games that must end without crashing, seed + answer-list replay, HTTP route tests with a temporary DB. CI: rustfmt, clippy, tests, cargo-audit, cargo-deny.

Settled rules go in `regles/decisions.md` with date and reason. R1–R8 were all settled on 2026-11-16 (the user accepted every recommendation).

## Rust code guidelines

Adopted on 2026-11-16 from the user's style guide (gist `czuger/e216e802e61c9c6d156c1a0f03b31003`). They apply to all future code. The planned CI (rustfmt, clippy with warnings forbidden) is how they are enforced.

### Language

- **Code is English, whatever language the prompt is in.** Identifiers, function names, crate and module names, comments, docstrings, commit messages and log lines are in English even when the request or the discussion is in French.
- **User-facing text is French, and stays French.** Error messages shown to users, CLI output, labels, templates, anything a reader sees: French, UTF-8, accents kept. Do not translate it; write new user-facing text in French.
- Project-specific precisions (user's decisions, 2026-11-16):
  - Crate names are English (`data`, `engine`, `ai`, `server`, `simulator`).
  - In the RON data files, keys, field names and effect-primitive names are English like the code; values read by the player (card names, card texts, location names…) are French.
  - Project documents (`.md`) stay in French, as stated in « Project status ».

### Imports

- Prefer `crate::`-rooted paths across modules (the Rust equivalent of absolute imports).
- `super::` is acceptable within tightly nested hierarchies (a `tests` submodule referencing its parent, a submodule referencing its direct parent's types).
- No deep `super::super::` chains: restructure the module tree instead.
- No glob imports (`use foo::*;`) except prelude modules designed for it, or `use super::*;` in test modules.

### Type annotations

- Annotate all `pub` function signatures explicitly (parameters and return type), even where inference would work.
- Let inference handle local variable bindings inside function bodies.
- Prefer concrete types over generics unless genericity provides real, used value (called with more than one concrete type, or part of a public API meant to be generic).
- Avoid `Box<dyn Any>` and other type-erasure patterns unless strictly necessary; justify with a short comment.
- Avoid `Box<dyn Error>` in library code; prefer a concrete, well-typed error enum. `Box<dyn Error>` is acceptable at binary/application boundaries (e.g. `main`).

### Docstrings

- All `pub` items (functions, methods, structs, enums, traits) require a rustdoc comment (`///`), with the standard sections (`# Arguments`, `# Returns`, `# Errors`, `# Panics`) as applicable.
- Private items get a docstring only when the logic isn't obvious from the signature and name.
- Keep docstrings proportionate: short for simple functions, detailed only when it adds real clarity. Do not restate what the signature already says.
- Enforce at crate level: `#![warn(missing_docs)]` everywhere, `#![deny(missing_docs)]` for the library crates (`data`, `engine`, `ai`).

### Comments

- No comments for straightforward or self-evident code; never caption the obvious.
- Comment only when the logic is genuinely complex, non-obvious, or could be misread.
- Keep comments short: a brief note on intent or reasoning, not a narration of what the code does.

### Error handling

- `Result<T, E>` for recoverable errors, `Option<T>` for absence. No panics as control flow.
- Propagate with `?` rather than manual `match`/`if let` chains, unless the error must be transformed or contextualized at that point.
- No `unwrap()` or `expect()` in library or production code paths; reserve them for tests, examples and prototypes. If used elsewhere because failure is provably impossible, justify with a short comment (e.g. `// safe: regex is a compile-time constant`).
- Define proper error types (plain enums, or via `thiserror`), not stringly-typed errors (`Err("something failed".to_string())`) nor blanket `Box<dyn Error>` in library code.
- Never silently discard errors with `.ok()`, `let _ =` or similar: handle, log or propagate explicitly.
- `panic!` (and `unreachable!`, `todo!`, `unimplemented!` left in shipped code) is reserved for genuine invariant violations (bugs), not expected failure modes.

### Function size and decomposition

- Soft limit of 50 lines per function.
- Beyond that, split into smaller, well-named sub-functions, each with a single clear responsibility and its own concise docstring.
- Prefer composition of small functions over deeply nested logic or long procedural blocks.
- Prefer early returns and `?`-based flattening over deeply nested `match`/`if let` pyramids.

### Types and modules

- One primary public type (struct, enum or trait) per file, with its associated `impl` blocks. The file name (snake_case) matches the type name (`struct UserAccount` → `user_account.rs`).
- Exceptions: small, tightly-coupled types may share the file with the primary type (an error enum returned by its methods, a config/builder struct feeding it, a result type it produces).
- A closely-related family of small types with no independent identity (AST node variants, a state machine's states) may live together in one file organized around the concept they express. Split it out once it grows into its own topic.
- Traits implemented by multiple unrelated types get their own file; a trait implemented by only one type may be co-located with that type.

### File and module organization

- No monolithic files. Split code by topic/responsibility into modules (`mod foo;`).
- Group related modules into a directory-based module tree when a topic grows large enough.
- Keep `mod.rs` / inline module-declaration files thin: `pub use` re-exports only, no implementation logic.
- Keep `lib.rs` / `main.rs` as thin entry points: module declarations, top-level wiring and re-exports, no business logic.

### General principles

- Favor readability and small, testable units over cleverness.
- Prefer explicit over implicit in naming, typing and structure.
- Split files as soon as they start doing « too much » rather than waiting for a refactor pass.
- Prefer composition over inheritance-like patterns; use traits for shared behavior, not deep hierarchies.
- Align with idiomatic Rust and clippy/rustfmt defaults rather than fighting them; deviate only where this guide explicitly says so.

## Domain notes

- Terminology follows the 2016 French edition: **Vigueur** is the skill the first French edition called « Combat » (*Fight* in English).
- Golden rule of the game: when in doubt, choose the outcome that is worst for the investigator.
- Card texts are copyrighted. The planned repository must stay **private**.
- Before writing engine code for a rule, check `regles/decisions.md` (settled rules, authoritative), `regles/base.md` §23 (official FAQ) and §22. Any new gap in the rules: do not decide it silently — ask the user, then record it in `decisions.md`.
