# PROGRESS — where we are

Last updated: 2026-09-25 (11a issued).

|  |  |
|---|---|
| Phase | **2 — Async Rust & local LLM** (Phase 1 exit met at 10h) |
| Last reviewed | **Stage 10h**, 2026-09-25. 1/1 `start_interrogation`; 145 across twenty files, `fmt` and `clippy -D warnings` clean. Committed `9de6939`. |
| Uncommitted | `PROGRESS.md`, `STAGE-LOG.md` (10h review); `DECISIONS.md`, `ROADMAP.md`, `stages/stage-11a-the-suspect-gets-a-voice.md`, `tests/engine.rs` (11a). |
| Next action | **His:** Stage 11a — `docs/stages/stage-11a-the-suspect-gets-a-voice.md`. Verified: 3 tests, 148 across twenty-one files, `fmt` and `clippy -D warnings` clean. |
| Blocked on | nothing |

**The teaching method was rewritten from the base on 2026-09-24** — see `CLAUDE.md`. Stages up to
10g used the old rules; do not copy their shape.

## Stage queue

Stages 1–10h ✅ — `STAGE-LOG.md`. Phases 2–3: `ROADMAP.md`. Stage 10's shape: `DECISIONS.md`, 2026-09-17.

| # | Stage | New thing | Est |
|---|---|---|---|
| ~~10d~~ ✅ | what the room will show | **consolidation — zero new elements.** `&[Turn]` as the borrowed form of `Vec<Turn>`, which is Stage 3's `String`/`&str` on a second type. Two whole bodies, signatures given. | 25 |
| ~~10e~~ ✅ | the app holds the phase | `Mutex`, interior mutability, `AppError::Poisoned`. Printed for him after five exchanges — nothing from it counts as `used`. | 30 |
| ~~10f~~ ✅ | two more methods through the lock | **consolidation — zero new elements**, and **in chat, not a brief** (`DECISIONS.md`, 2026-09-22). `AppState::suspect()` and `AppState::record()`, one line at a time, typed by him. | 30 |
| ~~10g~~ ✅ | the player picks a suspect | `Deserialize` on `SuspectId`: shape checked, meaning not — `begin_interrogation_from` checks it with `require_suspect`. Brief and spec written and verified (6 tests; 144 across nineteen files). | 25 |
| ~~10h~~ ✅ | React starts an interrogation | **zero new elements** — `#[tauri::command]` wrapper, `generate_handler!`, a console `invoke`. Recall of 9b/9c. Doc and spec written and verified (1 test; `tauri::test` dropped — Windows, `DECISIONS.md` 2026-09-25). | 20 |
| **11a** | the suspect gets a voice | **your own trait** — `trait InferenceEngine { fn reply }`, `impl … for MockEngine`. Implementing a trait is known (`From`, 9a); declaring one is new. | 20 |
| 11b | the game holds an engine | trait objects — `Box<dyn InferenceEngine>`. Count against Rule 2 before issuing. | — |

Where an id from React is checked: `DECISIONS.md`, 2026-09-23.

## Open, not blocking

- [ ] Rule 5's three habits were asked once, in 10a. Do not ask again.
- [ ] Doc comments: `ipc.rs :: case_intro_from` / `case_intro` (9c), the three Stage 5 `Case` methods
      and both items in `error.rs` have none.
      `parse_case`'s says "but it it fails"; `RawCase`'s says only "Raw case". Tidying.
- [ ] `#[serde(default)]` on `RawCase::facts` should come off — `suspects` is required and both are
      the same kind of thing. Changes no test.
- [ ] Template leftovers: `greetMsg` naming in `App.tsx`, `public/vite.svg`, `src/assets/react.svg`.
- [ ] Rewrite `README.md` — it still describes the Tauri template.
- [ ] Add `rust-toolchain.toml` pinning a stable version.
- [ ] `withGlobalTauri` was turned on in 9b for the console demo. Turn it off before any release
      build — it exposes the whole API to any script in the page.
