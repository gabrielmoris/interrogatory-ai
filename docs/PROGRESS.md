# PROGRESS — where we are

> Resume order: `CLAUDE.md` → this file → `CONCEPTS.md` → the current stage brief.
> Last updated: 2026-08-29.

## Status

|            |                                                                  |
| ---------- | ---------------------------------------------------------------- |
| Phase      | **1 — Rust core & Tauri foundations**                            |
| Stage      | **6b, not started** — `TryFrom`, the one road. Spec `src-tauri/tests/case_convert.rs`, 6 tests |
| Last done  | Stage 6a, 2026-08-29. 6/6 first submission |
| Next       | He implements 6b and says "ready". Then 6c, then 6d closes Stage 6. |
| Blocked on | nothing                                                          |

**Standing note:** the three `Case` methods from Stage 5, both items in `error.rs` and the three raw
types have no doc comments, while everything else public in `case.rs` does. Tidying, not a rule.

**Clippy is expected to fail until 6d.** `unused_mut` and two `unused_variables` come from the 6b/6d
stubs. Run `clippy -D warnings` when 6d closes the stage, not before.

---

## The stage queue

Each becomes one stage with its own failing test, inside the three-concept budget from `CLAUDE.md`.

| # | Stage | Headline concept |
|---|---|---|
| ~~1~~ | ~~`Difficulty` / `Tuning`~~ | ~~moves and `Copy`~~ |
| ~~2~~ | ~~`SuspectId` / `FactId`~~ | ~~the newtype pattern~~ |
| ~~3~~ | ~~`Fact`, `Suspect`, `Case`~~ | ~~`String` vs `&str`~~ |
| ~~4~~ | ~~borrowing and lifetimes~~ | ~~lifetimes as regions~~ |
| ~~5~~ | ~~`AppError` and `Result`~~ | ~~`Result` and `?`~~ |
| ~~6a~~ | ~~the file's own vocabulary~~ | ~~`Deserialize` — text in, Rust value out~~ |
| **6b** | the one road | `TryFrom`, a conversion that can fail |
| 6c | the four checks | validation at the boundary |
| 6d | the front door | why `?` cannot convert every error |
| 7 | `VisibleFact<'a>` newtype | a struct that holds a reference |
| 8 | Loading a case from disk | `std::fs`, and the first shell module |
| 9 | `tauri::State` and managed state | the first stage that touches Tauri |

## Phase 0 leftovers

- [x] `.gitattributes` + line-ending renormalization (`940912b`)
- [x] `/target`, `models/`, `*.gguf` ignored
- [x] Package manager settled on bun
- [x] `docs` removed from `.gitignore` — briefs and ADRs survive a fresh clone
- [x] `task.md` archived; `ROADMAP.md` amended for the rejected workspace split (2026-08-29)
- [ ] Delete the `greet` demo command, the template `App.tsx`, `public/vite.svg`,
      `src/assets/react.svg`
- [ ] Rewrite `README.md` — it still describes the Tauri template
- [ ] Add `rust-toolchain.toml` pinning a stable version

## Where the rest of it lives

`CONCEPTS.md` — what he has been taught, and the vocabulary rules.
`DECISIONS.md` — architecture decisions, newest first.
`STAGE-LOG.md` — one entry per finished stage.
`MENTOR-NOTES.md` — why the teaching rules exist.
`ROADMAP.md` — Phases 0–5 and the risk register.
