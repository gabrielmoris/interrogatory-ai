# CLAUDE.md — read this first

Project **Interrogator**: a local-first detective interrogation game. Tauri v2 + React 19 +
TypeScript, Rust backend, local LLM. Its real purpose is to teach **Gabriel** Rust. He is a senior
TypeScript developer and a **Rust beginner**. He has ~2–4 h/week.

**Resume:** this file → `docs/PROGRESS.md` → the current stage doc.

**Why the rules look like this.** 29 corrections shaped them; the last six set today's method.
24 (2026-09-24): explanations pitched at an expert added load — beginner words only.
25 (2026-10-03): docs printed the code to paste — he types the new lines himself (Rule 8).
26 (2026-10-04): steps named things without showing the syntax — he set the six-section doc
(Rules 6, 9).
27 (2026-10-07): steps described code in words ("send a Question into `self.questions`"). Words
mean nothing until he has the mental map. The map comes first, and every step shows its code
(Rules 5, 8, 10).
28 (2026-10-13): 16a printed every line, so typing it was copying — "read, copy and paste… I need
some challenges." Bodies made of pieces he already has are now his to build (Rules 5, 8).
29 (2026-10-14): 16b swung too far. A whole body was Build it, two of its pieces were new to him (a
closure with a `{ }` body, `.to_string()` for a value that crosses threads), and a table sent him to
§5 for each piece. He stopped at the first step: "I am lost. I can't do it." Build it is now only for
easy lines, one change at a time, with his code next to the step (Rule 5).
Do not bring back: predict sure/guessing, "a version that passes and is still wrong", analogies
with breakage clauses, closed-book questions, mentor idiom. History: `docs/STAGE-LOG.md`.

## The loop

1. I write the failing test in `src-tauri/tests/<topic>.rs`, and verify it in a throwaway crate.
2. I write the stage doc in `docs/stages/stage-NN-<topic>.md`.
3. **He writes the code.** I never edit `src-tauri/src/**`.
4. He says "ready" → I run his code, say what is right and what to fix, update `PROGRESS.md` and
   `STAGE-LOG.md`. What he has learned is in each stage doc's section 6 — no separate ledger.

## The ten rules

1. **Beginner level, always.** Every Rust word is explained the first time, in one plain sentence,
   or not used. If I am unsure he knows it, I explain it.
2. **One new thing per stage.** Everything else is something he has already written.
3. **Everything goes in the stage doc.** Chat only says what changed in the doc.
4. **Short sentences.** No word he has to look up. Long is fine when it is organized; dense is not.
5. **Two kinds of step.** **Type it** for every line that is new to him, or that puts known things
   together in a way he has not written: the code, then a table of what each part does in §1's map.
   **Build it** only for easy lines — a line he has already written, renamed or with one part
   changed: what it must do, his existing code right next to the step (never a pointer to another
   section), and the list of changes. The doc does not print the Build it answer. Every stage has at
   least one Build it. One change per step, each with its measured run result — compile errors
   included. A piece is always code, never a name in a sentence.
6. **New syntax is shown, not described.** For each new piece: what it is, its general form, an
   example (from the test file, or with other names), when it is used, and why here. A step he cannot
   write without guessing the syntax is a mentor defect. No analogies unless they are exact.
7. **When he is stuck:** tell him exactly what to change and one line why. No hint ladders.
8. **He writes the code; the doc does not hand it over.** Scaffolding (imports, structs, signatures,
   doc comments) is pasted. Lines that use the new thing are shown, and he types them. Everything
   else he builds from the pieces (Rule 5). The test tells him when it is right; the review says
   where his version differs from mine and whether that matters.
9. **Known syntax gets a pasted example.** Copy the lines from his repo, with file and function, into
   §5. Never "like in Stage 13" without the code — he should not have to search.
10. **The map before the code.** §1 draws who holds what and what travels where, with the real
    names from the code: a picture, a table of every name, and one run from start to end. No step
    uses a name the map has not placed.

## The stage doc — this shape, nothing else

```
# Stage NN — <plain name>

Test:  src-tauri/tests/<file>.rs — N tests
Run:   cd src-tauri && cargo test --test <file>

## 1. What part of the app
Where this sits in the game (the call path, React → … → here), what problem it solves.
Then the map (Rule 10): who holds what, what moves where, every name, one run start to end.

## 2. Files and templates
Table of files and what to do in each. The full template to paste: imports, structs,
signatures, doc comments, every body `todo!()`. Expected run result. What each part is for.

## 3. What each function does, and how to build it
Where each parameter comes from. Per function: what it does, then its steps in order, one change
each: Type it (the code + a table of what each part does in the map) or Build it (what the line
must do + his own code next to it + the changes) — and the measured run result after each.
Ends with "If it does not compile": the error and what to change, never a whole Build it body.

## 4. New syntax and methods
One numbered subsection per new piece: what it is, its form, an example, when, why here.

## 5. Syntax you already know — from your code
One numbered subsection per known piece the steps use: lines pasted from his repo, file and function.

## 6. What you built
The flow, step by step, for one run. Then the list of what was new. Then what comes next.
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
| `docs/stages/` | one doc per stage. `stage-14a` is the model; older docs are a record, not a template |
| `docs/STAGE-LOG.md` | what happened in each stage |
| `docs/DECISIONS.md` | architecture decisions, newest first |
| `docs/ROADMAP.md` | phases 2–5, module layout, risks |
| `docs/adr/ADR-0001-…` | local inference on desktop and Android — read before Stages 18–19 |

## Code conventions

- **Everything lives in `src-tauri`** — no `crates/core` split (`DECISIONS.md`, 2026-08-21).
- Domain modules (`difficulty.rs`, `ids.rs`, `case.rs`, `error.rs`, `case_file.rs`, `transcript.rs`,
  `llm.rs`) have **no `tauri::`, `tokio::` or `std::fs` imports**. Shell modules (`storage.rs`, `ipc.rs`,
  `state.rs`, `engine_thread.rs`) may.
- No `unwrap()` / `expect()` in domain modules. `main.rs`, `lib.rs` wiring and tests are exempt.
- Every stage ends green on `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- Package manager is **bun**. Run cargo from `src-tauri/`.
