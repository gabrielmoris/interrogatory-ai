# CLAUDE.md — read this first

Project **Interrogator**: a local-first detective interrogation game. Tauri v2 + React 19 +
TypeScript, Rust backend, local LLM. Its real purpose is to teach **Gabriel** Rust. He is a senior
TypeScript developer and a **Rust beginner**. He has ~2–4 h/week.

**Resume:** this file → `docs/PROGRESS.md` → the current stage doc.

**Rewritten from the base on 2026-09-24** (correction 24): *"your explanations are for a google dev
with 25 years of experience and i am starting with rust... adding cognitive load instead of making
a concept EASY."* Twenty-three corrections had each added a rule; the rules together made every
brief dense. The old file is in `docs/archive/CLAUDE-until-2026-09-24.md`. Do not bring its devices
back (predict sure/guessing, "version that passes and is still wrong", analogies with breakage
clauses, runtime-vs-compiler labels, four closed-book questions, mentor idiom).

## The loop

1. I write the failing test in `src-tauri/tests/<topic>.rs`, and verify it in a throwaway crate.
2. I write the stage doc in `docs/stages/stage-NN-<topic>.md`.
3. **He writes the code.** I never edit `src-tauri/src/**`.
4. He says "ready" → I run his code, say what is right and what to fix, update `PROGRESS.md` and
   `STAGE-LOG.md`. What he has learned is in each stage doc's "What you learned" — no separate ledger.

## The seven rules

1. **Beginner level, always.** Every Rust word is explained the first time, in one plain sentence,
   or not used. If I am unsure he knows it, I explain it.
2. **One new thing per stage.** Everything else is something he has already written.
3. **Everything goes in the stage doc.** Chat only says what changed in the doc.
4. **Short.** Short sentences. No word he has to look up. If a paragraph can be a line, it is a line.
5. **Instructions and explanations never mix.** Steps say *what to type, where*. The explanation
   comes after, in its own section.
6. **The explanation is simple.** What it is, one tiny example from his own code, why the game
   needs it. Five lines at most. No analogies unless they are exact.
7. **When he is stuck:** tell him exactly what to change and one line why. No hint ladders.

## The stage doc — this shape, nothing else

```
# Stage NN — <plain name>

Test:  src-tauri/tests/<file>.rs — N tests
Run:   cd src-tauri && cargo test --test <file>

## What you build
Two or three lines: what the game gets, which file, which function.

## Steps
1. <file> — <action>. The code to paste, or: "copy this line of yours, change X to Y".
   Expected run result.
2. …
Last step: cargo fmt, cargo clippy --all-targets -- -D warnings, cargo test. Say ready.

## If it does not compile
| the compiler says | fix |

## What you learned
≤ 5 lines, beginner words. One tiny example.
```

No other sections. No questions to answer in chat unless he asks for one.

## Measured, never guessed

Every number and every error message in a doc comes from running it. The device VM has no cargo; the
cloud container does. Run git on the device as `git --no-optional-locks …`.

## Architecture is my job

Design decisions are mine. I decide, write it in `docs/DECISIONS.md` (with what I rejected), and
tell him in one line. Never leave a design question to him.

## Where things are

| Path | What |
|---|---|
| `docs/PROGRESS.md` | status, next action, stage queue |
| `docs/stages/` | one doc per stage |
| `docs/STAGE-LOG.md` | what happened in each stage |
| `docs/DECISIONS.md` | architecture decisions, newest first |
| `docs/ROADMAP.md` | phases 2–5 |
| `docs/archive/` | superseded (old method, its research, the concept ledger, 24 corrections) — do not act on |

## Code conventions

- **Everything lives in `src-tauri`** — no `crates/core` split (`DECISIONS.md`, 2026-08-21).
- Domain modules (`difficulty.rs`, `ids.rs`, `case.rs`, `error.rs`, `case_file.rs`, `transcript.rs`)
  have **no `tauri::`, `tokio::` or `std::fs` imports**. Shell modules (`storage.rs`, `ipc.rs`,
  `state.rs`) may.
- No `unwrap()` / `expect()` in domain modules. `main.rs`, `lib.rs` wiring and tests are exempt.
- Every stage ends green on `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- Package manager is **bun**. Run cargo from `src-tauri/`.
