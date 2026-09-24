# PROGRESS — where we are

Last updated: 2026-09-24.

|  |  |
|---|---|
| Phase | **1 — Rust core & Tauri foundations** |
| Last reviewed | **Stage 10g**, 2026-09-24. 6/6 `pick_suspect`; 144 across nineteen files, `fmt` and `clippy -D warnings` clean. Committed `a7c66a2`. |
| Uncommitted | `CLAUDE.md` rewritten from the base; `MENTOR-NOTES.md`, `CONCEPTS.md`, `TEACHING-EVIDENCE.md` moved to `docs/archive/`; 10g doc, `STAGE-LOG.md`, `PROGRESS.md`. |
| Next action | **Mine:** write 10h in the new `CLAUDE.md` shape. |
| Blocked on | nothing |

**The teaching method was rewritten from the base on 2026-09-24** — see `CLAUDE.md`. Stages up to
10g used the old rules; do not copy their shape.

## Stage queue

Stages 1–10g ✅ — `STAGE-LOG.md`. Phases 2–3: `ROADMAP.md`. Stage 10's shape: `DECISIONS.md`, 2026-09-17.

| # | Stage | New thing | Est |
|---|---|---|---|
| ~~10d~~ ✅ | what the room will show | **consolidation — zero new elements.** `&[Turn]` as the borrowed form of `Vec<Turn>`, which is Stage 3's `String`/`&str` on a second type. Two whole bodies, signatures given. | 25 |
| ~~10e~~ ✅ | the app holds the phase | `Mutex`, interior mutability, `AppError::Poisoned`. Printed for him after five exchanges — nothing from it counts as `used`. | 30 |
| ~~10f~~ ✅ | two more methods through the lock | **consolidation — zero new elements**, and **in chat, not a brief** (`DECISIONS.md`, 2026-09-22). `AppState::suspect()` and `AppState::record()`, one line at a time, typed by him. | 30 |
| ~~10g~~ ✅ | the player picks a suspect | `Deserialize` on `SuspectId`: shape checked, meaning not — `begin_interrogation_from` checks it with `require_suspect`. Brief and spec written and verified (6 tests; 144 across nineteen files). | 25 |
| 10h | React starts an interrogation | **zero new elements** — `#[tauri::command]` wrapper, `generate_handler!`, a console `invoke`. Recall of 9b/9c. | 20 |

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
