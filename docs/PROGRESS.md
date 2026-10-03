# PROGRESS — where we are

Last updated: 2026-10-03 (Stage 14a reviewed).

|  |  |
|---|---|
| Phase | **2 — Async Rust & local LLM** (Phase 1 exit met at 10h) |
| Last reviewed | **Stage 14a**, 2026-10-03. 4/4 `engine_thread` (20 repeat runs, no flake); 167 across twenty-seven files, `fmt` and `clippy -D warnings` clean against his exact files. |
| Uncommitted | His Stage 14a: `src/engine_thread.rs` (new), `src/lib.rs` (`pub mod engine_thread;`), and this review's `PROGRESS.md` / `STAGE-LOG.md`. The spec and doc are `63b937c`. |
| Next action | **His:** commit 14a. **Mine:** write 14b (`DECISIONS.md`, 2026-09-30). |
| Blocked on | nothing |

**The teaching method was rewritten from the base on 2026-09-24** — see `CLAUDE.md`. Stages up to
10g used the old rules; do not copy their shape.

## Stage queue

Stages 1–14a ✅ — `STAGE-LOG.md`. Phases 2–3: `ROADMAP.md`. Stage 10's shape: `DECISIONS.md`, 2026-09-17.

| # | Stage | New thing | Est |
|---|---|---|---|
| ~~10d~~ ✅ | what the room will show | **consolidation — zero new elements.** `&[Turn]` as the borrowed form of `Vec<Turn>`, which is Stage 3's `String`/`&str` on a second type. Two whole bodies, signatures given. | 25 |
| ~~10e~~ ✅ | the app holds the phase | `Mutex`, interior mutability, `AppError::Poisoned`. Printed for him after five exchanges — nothing from it counts as `used`. | 30 |
| ~~10f~~ ✅ | two more methods through the lock | **consolidation — zero new elements**, and **in chat, not a brief** (`DECISIONS.md`, 2026-09-22). `AppState::suspect()` and `AppState::record()`, one line at a time, typed by him. | 30 |
| ~~10g~~ ✅ | the player picks a suspect | `Deserialize` on `SuspectId`: shape checked, meaning not — `begin_interrogation_from` checks it with `require_suspect`. Brief and spec written and verified (6 tests; 144 across nineteen files). | 25 |
| ~~10h~~ ✅ | React starts an interrogation | **zero new elements** — `#[tauri::command]` wrapper, `generate_handler!`, a console `invoke`. Recall of 9b/9c. Doc and spec written and verified (1 test; `tauri::test` dropped — Windows, `DECISIONS.md` 2026-09-25). | 20 |
| ~~11a~~ ✅ | the suspect gets a voice | **your own trait** — `trait InferenceEngine { fn reply }`, `impl … for MockEngine`. Implementing a trait is known (`From`, 9a); declaring one is new. | 20 |
| ~~11b~~ ✅ | the game holds an engine | **`Box<dyn InferenceEngine>`** as a field of `AppState`; `AppState::ask` passes the question on. `: Send + Sync` on the trait is printed, not taught (Stage 15). Doc and spec written and verified (3 tests; 151 across twenty-two files). | 25 |
| ~~11c~~ ✅ | the room keeps the exchange | **zero new elements** — `ask` = `record` + `reply` + `record`; `AppState::turn_count` is a copy of `suspect`. Doc and spec written and verified (4 tests; 155 across twenty-three files). | 20 |
| ~~11d~~ ✅ | React asks a question | **zero new elements** — `#[tauri::command] ask_suspect`, handler list, console `invoke` (10h pattern). Doc and spec written and verified (1 test; 156 across twenty-four files). | 20 |
| ~~12~~ ✅ | a question waits until it is awaited | **`async fn` + `.await`** — a future does nothing until awaited. `AppState::ask` and `ask_suspect` become async; tests use `block_on`. Doc and spec written and verified (3 tests; 159 across twenty-five files). | 25 |
| ~~13~~ ✅ | slow work gets its own thread | **`spawn_blocking` + `move`** — work on another thread must own what it uses. `case_intro_in_background` (disk read), `case_intro` async, `AppError::BackgroundFailed`. Doc and spec written and verified (4 tests; 163 across twenty-six files). | 25 |
| ~~14a~~ ✅ | the engine gets its own thread | **a channel** — `Sender`/`Receiver`, `send` moves the value, `for … in inbox` ends when every `Sender` is gone. `engine_thread.rs :: start`, `Question`. Doc and spec written and verified (4 tests; 167 across twenty-seven files). | 25 |
| 14b | `ask` goes through the engine thread | **zero new elements** — `AppState` holds the `Sender`; `ask` sends a `Question` and waits with `spawn_blocking` (13). Shape measured, not written. | 20 |

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
