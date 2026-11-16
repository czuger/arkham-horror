# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project status

A browser adaptation of **Arkham Horror 2nd edition** (French edition, *Horreur à Arkham*), for private use: **solo mode** (one human player, the other investigators played by AIs), **base game only**.

**No code exists yet**, and the folder is not a git repository. There is no build, lint or test command to run. The repository currently holds planning and reference documents, all written in French, plus the base-game images in `assets/`. Keep new project documents in French, matching their plain style.

| File | Role |
|---|---|
| `plan_creation_jeu.md` | **The current plan (2026-11-16). Read this first.** Architecture (axum + tokio single-thread server, embedded SurrealDB, server-rendered HTML with minimal JS), crate layout, data format, engine design (decision/response model), AI, milestones J0–J6, tests, hosting. |
| `manques_et_questions.md` | Up-to-date list of missing data (M1–M24), technical points to verify (T1–T6), unsettled rules (R1–R7) and questions for the user (Q1–Q23). |
| `plan_mise_en_ligne.md` | The earlier plan (WASM client, no server). Its architecture is superseded; its Parts 2–3 (missing data M1–M20, open questions) are kept for reference. |
| `regles/base.md` | Base-game rules, transcribed from the official French rulebook (Edge 2016 revised edition). Transcription notes are marked ⚠ in italics; §22 lists rulebook inconsistencies that are still unresolved. |
| `inventaire.md` | Inventory of the source material (a 2017 scrape of the French wiki `wiki.horreuraarkham.fr`): what each HTML/PDF file contains and what data is still missing (§7–§8). |

The scraped sources live **outside** this repo, at `~/python/perso/scraper/pages/wiki.horreuraarkham.fr/` (= `/Users/ced/python/perso/scraper/pages/wiki.horreuraarkham.fr/`). As of 2026-11-16 the folder holds 953 HTML, 66 PDF and about 3,100 wiki images, all in one flat folder (images are named `images__x__xy__<name>`). They mix three games; only Arkham Horror 2nd ed. is relevant — ignore *Mansions of Madness* (`_mom`/`_MoM` suffix) and *Elder Sign* (`_es` suffix) pages.

- **This local scrape is the only allowed source. Never download anything** (wiki, archive.org or elsewhere). If something is missing, tell the user; they re-run the scraper themselves.
- Treat the scrape folder as **read-only**. Write outputs to the scratchpad or the project, never next to the sources.
- Counts change when the user re-runs the scraper, so recount on disk before saying a file is missing.

Base-game images are sorted by theme in `assets/` (plateau, cartes, fiches, monstres, marqueurs, errata, regles, illustrations). `assets/inventaire_assets.md` lists every file with its source and has a « Manquants » section.

## Planned architecture (from `plan_creation_jeu.md`)

One executable holds everything: rules engine, AIs, web server and embedded database.

- **Server**: axum + tokio on a **single thread** (`current_thread` runtime). No game state in memory: each request loads the game from the DB, applies the answer, lets the AIs play up to the next human decision, then saves.
- **Database**: SurrealDB embedded with `kv-surrealkv` (tables `partie`, `action`, `evenement`). Game data (cards, board…) stays in versioned RON files under `data/`, never in the DB or the code.
- **UI**: server-rendered HTML (askama or maud, to decide), classic forms with POST → redirect → GET, plus **htmx** (served locally, no CDN) to swap page fragments. **No heavy JavaScript** and no client-side state; a few small hand-written optional scripts at most. Must stay playable without JS. Card and token images come from `assets/`.
- AI turns and the Mythos phase are played **in one block** after each human answer; the human follows them in the journal.
- **Hosting and access control are out of scope** (the user has a dedicated environment). The project only has to run locally: listen on `127.0.0.1`, no login page.
- Cargo workspace: `data` (types + loading + consistency checks), `engine` (rules: deterministic, serializable, no I/O), `ai` (policies answering decisions), `server` (axum), `simulator` (CLI for mass AI-only games; kept out of the server because it would block the single thread).
- **Engine model**: the engine advances until someone must choose, then exposes a *Decision* (who, what, legal options). Humans answer through forms, AIs through a policy function; the engine validates both the same way and refuses illegal answers. Dice come from the engine's seeded RNG. Seed + ordered answers replay a game exactly, which gives save/resume, undo and reproducible tests.
- Card effects are written in the data files with a small vocabulary of effect primitives; the rare cards that do not fit are flagged « effet spécial » and coded by name.
- Deliverable: single binary + `data/` + `assets/` + htmx file + DB file created on first run, configured by file or environment variables.
- Card texts are transcribed by Claude from the images in `assets/` (user's decision, 2026-11-16), then proofread by the user against the physical box. Copy counts per card are pending the user's own research: enter them from memory and mark them « à vérifier ».

Planned tests: unit tests per phase and rule, property tests (no impossible states, no negative resources), data consistency checks, one auto-generated test per card, thousands of AI-only games that must end without crashing, seed + answer-list replay, HTTP route tests with a temporary DB. CI: rustfmt, clippy, tests, cargo-audit, cargo-deny.

Settled rules go in `regles/decisions.md` (to create) with date and reason.

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
- Before writing engine code for a rule, check `regles/base.md` §22 and the unsettled rules R1–R7 in `manques_et_questions.md`; several rules are not settled yet. Do not decide them silently — ask the user.
