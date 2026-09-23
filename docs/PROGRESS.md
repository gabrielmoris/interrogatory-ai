# PROGRESS — where we are

Last updated: 2026-09-23.

|  |  |
|---|---|
| Phase | **1 — Rust core & Tauri foundations** |
| Last reviewed | **Stage 10f**, 2026-09-23, in chat. 4/4 `app_room`, 4/4 `app_phase`; `fmt` and `clippy -D warnings` clean in the reference crate against his `state.rs`. Committed `591b991`. |
| Uncommitted | This review: `CONCEPTS.md`, `STAGE-LOG.md`, `PROGRESS.md`. |
| Next action | **His:** 10f's four questions (10e's went unasked — 10f's cover the same lock). **Mine:** 10g — first decide where an id from React is checked against the case (see below), then re-count it against Rule 1. |
| Blocked on | nothing |

**The teaching rules were rewritten 2026-09-20 (correction 18)** after *"I finish the stage without
understanding why Rust needed it. I feel I am copying."* `CLAUDE.md` gained Rule 0 (when a thing is
explained decides whether it teaches) and Rule 3 (the concept must live in his head — predict before
running, one version that passes and is wrong, four closed-book questions once green). Rules were
renumbered: old 3→4, 4+4a→5, 5→7; 6 is unchanged. `TEACHING-EVIDENCE.md` is the research base and is
what to read before relaxing any of it. **Stages 1–10c were written to the old rules — do not copy
their shape.**

## Stage queue

Stages 1–10f ✅ — `STAGE-LOG.md`. Phases 2–3: `ROADMAP.md`. Stage 10's shape: `DECISIONS.md`, 2026-09-17.

| # | Stage | New thing | Est |
|---|---|---|---|
| ~~10d~~ ✅ | what the room will show | **consolidation — zero new elements.** `&[Turn]` as the borrowed form of `Vec<Turn>`, which is Stage 3's `String`/`&str` on a second type. Two whole bodies, signatures given. | 25 |
| ~~10e~~ ✅ | the app holds the phase | `Mutex`, interior mutability, `AppError::Poisoned`. Printed for him after five exchanges — nothing from it counts as `used`. | 30 |
| ~~10f~~ ✅ | two more methods through the lock | **consolidation — zero new elements**, and **in chat, not a brief** (`DECISIONS.md`, 2026-09-22). `AppState::suspect()` and `AppState::record()`, one line at a time, typed by him. | 30 |
| 10g | React starts an interrogation | `Deserialize` on `SuspectId` for a command argument · recall `generate_handler!` | 25 |

10g: re-count against Rule 1 before issuing. Before writing it, decide where an id arriving from
React is checked against the case — `AppState` holds no `Case` yet, and `SuspectId` means "an id that
exists in this case" (`DECISIONS.md`, 2026-08-27).

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
