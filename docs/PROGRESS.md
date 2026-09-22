# PROGRESS — where we are

Last updated: 2026-09-21.

|  |  |
|---|---|
| Phase | **1 — Rust core & Tauri foundations** |
| Last reviewed | **Stage 10d**, 2026-09-21. 6/6, `fmt` and `clippy -D warnings` clean; 130 tests across sixteen files, measured in the reference crate against his files. Committed `1fb653a`. |
| Uncommitted | Stage 10e's brief and spec; `DECISIONS.md` 2026-09-21 and the 10d→10e renumbering it forced in two older entries; correction 20 (`MENTOR-NOTES.md`, `CLAUDE.md` Rule 2, 10e step 1 rewritten). |
| Next action | **His.** The two recall questions in chat (10c, 10d — `CONCEPTS.md`), then Stage 10e: brief `docs/stages/stage-10e-the-app-holds-the-phase.md`, spec `src-tauri/tests/app_phase.rs`. 10d's four questions went unanswered; its recall question stands in for question 2 — do not re-ask them. |
| Blocked on | nothing |

**The teaching rules were rewritten 2026-09-20 (correction 18)** after *"I finish the stage without
understanding why Rust needed it. I feel I am copying."* `CLAUDE.md` gained Rule 0 (when a thing is
explained decides whether it teaches) and Rule 3 (the concept must live in his head — predict before
running, one version that passes and is wrong, four closed-book questions once green). Rules were
renumbered: old 3→4, 4+4a→5, 5→7; 6 is unchanged. `TEACHING-EVIDENCE.md` is the research base and is
what to read before relaxing any of it. **Stages 1–10c were written to the old rules — do not copy
their shape.**

## Stage queue

Stages 1–10d ✅ — `STAGE-LOG.md`. Phases 2–3: `ROADMAP.md`. Stage 10's shape: `DECISIONS.md`, 2026-09-17.

| # | Stage | New thing | Est |
|---|---|---|---|
| ~~10d~~ ✅ | what the room will show | **consolidation — zero new elements.** `&[Turn]` as the borrowed form of `Vec<Turn>`, which is Stage 3's `String`/`&str` on a second type. Two whole bodies, signatures given. | 25 |
| 10e | the app holds the phase | `Mutex` — changing a value every command shares · recall `.map_err` *(shaky)*. **Issued 2026-09-21**: one method, `AppState::begin`, first refused (`E0596`, then `E0524` on `&mut self`), then through the lock. Poison → `AppError::Poisoned` (`DECISIONS.md`). 4 tests, 60 prose lines. | 30 |
| 10f | React starts an interrogation | `Deserialize` on `SuspectId` for a command argument · recall `generate_handler!` | 25 |

10f: re-count against Rule 1 before issuing, and write it to the rewritten rules. Before writing it,
decide where an id arriving from React is checked against the case — `AppState` holds no `Case` yet,
and `SuspectId` means "an id that exists in this case" (`DECISIONS.md`, 2026-08-27). Reading the
phase through the lock (`AppState::suspect()`) was cut from 10e to fit the ceiling; it goes with the
first command that reads the phase, or into a consolidation stage.

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
