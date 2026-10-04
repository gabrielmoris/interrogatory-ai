# Interrogator — Engineering & Learning Roadmap

**Budget:** 2–4 h/week — a **9–12 month build**, started 2026-08-21.
**Target:** his Mac mini, Apple Silicon (`DECISIONS.md`, 2026-10-04). Android is Phase 2.5.
**Inference:** `llama-cpp-2` in-process, Metal offload — `adr/ADR-0001-cross-platform-inference.md`.

The next one or two stages live in `PROGRESS.md`; the later queue is here. This file is the
**plan**; `DECISIONS.md` is the **why**. When they overlap, this file carries a pointer only.

---

## 1. The decision that matters most

> **Ground truth lives in Rust. The LLM is a constrained actor, never the source of truth.**

The case's real facts, who knows what, and the scoring rubric are Rust data. The model is handed a
*filtered view* and a persona, and its output is untrusted text. Everything follows from that:

- Scoring is reproducible — the deterministic tier runs on `FactId`s, not on model opinion.
- The suspect cannot leak what it was never given, because hidden facts never enter the context
  window. Enforced at the type level, not by prompt wording.
- Phases 1, 3 and 4 build and test with **zero inference**, because a mock engine satisfies the
  same trait.

> **Domain modules are pure. Shell modules may touch the outside world.** A convention enforced by
> review, not a crate boundary — the price of not splitting (`DECISIONS.md`, 2026-08-21).

```
src-tauri/src/
  main.rs           entry only
  lib.rs            tauri::Builder wiring + module declarations
  # ---- domain: no tauri::, no tokio::, no std::fs ----
  difficulty.rs     Difficulty, Tuning                       Stage 1
  ids.rs            SuspectId, FactId                        Stage 2
  case.rs           Case, Suspect, Fact, VisibleFact         Stages 3–5, 7
  error.rs          AppError (thiserror) + Serialize         Stage 5
  case_file.rs      RawCase -> TryFrom -> Case               Stage 6
  transcript.rs     Speaker, Turn, Phase                     Stages 10a–10d
  llm.rs            trait InferenceEngine + MockEngine       Stage 11a
  prompt.rs         deterministic String assembly            Stage 20
  scoring.rs        Report, Verdict, ScoreBreakdown          Stage 21
  generator.rs      seeded case skeletons + is_solvable      Stage 22
  # ---- shell: allowed Tauri, tokio, the filesystem ----
  storage.rs        reads case files from disk               Stage 8
  ipc.rs            wire types + #[tauri::command] wrappers  Stages 9a–13
  state.rs          AppState, managed via tauri::State       Stages 9c, 10e, 11b–12
  engine_thread.rs  the engine's own thread + channels       Stage 14
  llama.rs          the real model (FFI)                     Stage 19
  session/          stateful interrogation orchestration     Phase 3
```

**Invariants for the whole project:**

- A domain module that needs `tauri`, `tokio` or `std::fs` means the boundary has leaked. Fix the
  boundary, not the import.
- `ipc` functions deserialize, delegate, map errors. Nothing else.
- The rest of the code conventions: `CLAUDE.md`.

---

## Phase 1 — Rust core & Tauri foundations — Stages 1–10 ✅

Ownership, borrowing, lifetimes, structs, enums with data, `Option`/`Result`, `?`, custom errors,
traits, `TryFrom`, the IPC boundary, interior mutability. Domain model (1–4), `AppError` (5), case
files (6a–6d), `VisibleFact` (7), disk and IPC (8–9c), transcript, phase and lock (10a–10h).
What was built: `STAGE-LOG.md`. Why: `DECISIONS.md`.

**Exit (met at 10h):** a case loads from disk, one command returns it to React, a phase that holds
data it should not have does not compile, a wrong transition is an `InvalidState` error, all domain
logic tested without launching Tauri.

## Phase 2 — Async Rust & local LLM — Stages 11–19 *(the hard phase)*

`async`/`await`, the tokio runtime, `Send`/`Sync`, `Arc`, channels, `spawn_blocking`, cancellation,
FFI lifetimes.

| # | Stage | Headline concept | Est |
|---|---|---|---|
| 11a–11d ✅ | the engine trait, held by `AppState`, asked from React | declaring a trait; `Box<dyn …>` | — |
| 12 ✅ | a question waits until it is awaited | a future does nothing until awaited | 25 |
| 13 ✅ | slow work gets its own thread | `spawn_blocking` + `move` | 25 |
| 14a ✅ | the engine gets its own thread | a channel | 25 |
| 14b | `ask` goes through the engine thread | zero new — recall of 14a | 20 |
| 15 | sharing across threads | `Arc<Mutex<T>>`, and why a std guard cannot cross `.await` | 55 |
| 16 | streaming to React | Tauri events with a typed payload | 45 |
| 17 | cancellation | `select!` and cooperative cancellation | 55 |
| 18 | **toolchain session** — `llama-cpp-2` on macOS/Metal | **zero new concepts. Environment only.** | 2 h |
| 19 | owning the model | one thread owns the FFI handle | 60 |

Each of these still has to pass Rule 2 (one new thing) before it is issued; several will split.

- **2.1 The engine trait, mock first** — `trait InferenceEngine`, `MockEngine` with canned
  deterministic lines. Unblocks Phases 3–4 and keeps the suite fast forever. Stage 11 ✅.
- **2.2 Async inside Tauri** — Tauri v2 already runs tokio. `spawn` vs `spawn_blocking`.
  Stages 12–13 ✅.
- **2.3 The threading model** — *the most important lesson in the project.* llama.cpp decoding is
  blocking FFI and must not run on a tokio worker. One dedicated OS thread owns the `LlamaContext`;
  work in over a channel, tokens out over another; the async side only ever talks to channels.
  `LlamaModel` behind an `Arc`, loaded once. The thread and its channels are built with the mock
  first (14a–14b); the real model moves onto it in Stage 19.
- **2.4 Streaming** — `mpsc` → `app_handle.emit("interrogation://token", …)` → `listen()`. Every
  event carries a session/turn id so late tokens from a cancelled generation are discarded.
  Stage 16.
- **2.5 Cancellation** — `CancellationToken` checked inside the decode loop; `select!`, drop
  semantics, cleanup order. Stage 17.
- **2.6 The build** — Xcode command-line tools, CMake, Metal offload. **A session of its own,
  zero feature work alongside it.** Record the working toolchain versions in `docs/BUILD.md` the
  moment it compiles. Size the model to the Mac's unified memory: an 8B instruct at Q4_K_M is ~4.7 GB. Weights in `models/`,
  gitignored. Stage 18.
- **2.7 KV cache** — reuse across turns rather than re-prompting the transcript. Truncation:
  system prompt + first N + last M, summarize the middle.

**Exit:** real local model, token-by-token streaming, cancellable mid-sentence, UI never blocks,
`MockEngine` still passes the same tests.

## Phase 3 — Case engine, gating & scoring — Stages 20–23

| # | Stage | Headline concept | Est |
|---|---|---|---|
| 20 | `build_prompt` | prompts are code — snapshot tests | 55 |
| 21 | tier-1 scoring | pure functions over ids, reproducible | 55 |
| 22 | the generator skeleton | determinism from a seed | 60 |
| 23 | grammar-constrained output | constraining a model at decode time | 55 |

- **3.1 Prompt assembly** — `build_prompt(...) -> Prompt` in `prompt.rs`, pure and synchronous,
  snapshot-tested with `insta`. Prompts are code; regressions in them are bugs and must show in a
  diff. Stage 20.
- **3.2 Gating enforced by types** — `build_prompt` accepts only `&[VisibleFact]`. There is no path
  by which a hidden fact reaches the context window because there is no function that accepts one.
  "The system prompt tells it not to" is not enforcement. Type lands in Stage 7; the gate closes
  here.
- **3.3 Interrogation state machine** — per-suspect pressure and consistency. The game logic that
  makes it a game rather than a chat window.
- **3.4 Two-tier scoring** — tier 1 deterministic: extract claimed `FactId`s from the report,
  precision/recall against ground truth → the number. Tier 2: a separate call writes the critique
  given tier 1's result. **Never let the model own the number.** Stages 21, 23.
- **3.5 Difficulty tuning** — `Difficulty` → temperature, evasiveness, facts volunteered per turn,
  whether the suspect lies and how consistently, red herrings per case.
- **3.6 Case generation** — Rust builds the skeleton, the model writes the words. `generator.rs`
  elects a culprit, assigns fact roles, distributes `known_by` so the case is solvable by
  construction; seeded RNG. `is_solvable` lives there and **only** there — not a `parse_case` rule.
  Generated cases enter through the ordinary `parse_case` path; no privileged route in. Two or three
  hand-authored cases per difficulty stay as fixtures and quality bar. `DECISIONS.md`, 2026-08-29.
  Stage 22.

**Exit:** end-to-end playthrough — intro → interrogation → report → score + critique — on two cases
at two difficulties, plus one generated case played to a score.

## Phase 4 — React UI & typed boundary

His home turf. **No Rust teaching budget — keep it tight, it is the phase most likely to expand to
fill the space.** One typed IPC module (`src/lib/ipc.ts`); nothing calls `invoke` directly. Types
generated from the Rust structs with `ts-rs`/`specta`, never hand-written. Four screens. Handle the
states that exist only because inference is real: model loading, first-token latency, cancelled
generation, inference error.

## Phase 5 — 3D suspect faces *(deferred)*

React Three Fiber, blend shapes, expressions driven by pressure from the Rust side. **Gated on Phase
3 being stable.** Most visible, least load-bearing, largest scope-creep risk in the project.

---

## Cross-cutting practices

| Practice | Why |
|---|---|
| `cargo clippy --all-targets -- -D warnings` every stage | The cheapest Rust tutor you have. Read the lint names, not just the fixes. |
| Domain tests run without launching Tauri | If scoring can't be tested headlessly, the layering is wrong. |
| `MockEngine` stays working forever | The fast test path and the offline dev path. |
| `tracing`, not `println!`, from Stage 16 | Async token streams are unreadable in `println!`. |

## Risk register

| Risk | Mitigation |
|---|---|
| `llama-cpp-2` won't build on macOS/Metal | Isolated session, no feature work alongside. Document exact versions. Fallback: `llama-server` sidecar over HTTP for one phase — same trait, so the swap is contained. |
| Blocking FFI on the async runtime | Dedicated thread + channels (§2.3). Non-negotiable. |
| Everything ends up in `lib.rs` | One module per concept, domain/shell split, reviewed each stage. There is no crate boundary to catch it. |
| Prompt-engineering rabbit hole | Snapshot-tested prompts; time-box tuning. |
| 3D scope creep | Phase 5 gated on Phase 3. |
| Generated cases atmospheric but unsolvable | Structure first; `is_solvable` gates every skeleton before a word is generated (§3.6). |
| Model weights in git | `.gitignore` + documented download step. |
| 2–4 h/week fragmentation | Every session ends at a commit with the next action written down. Never stop mid-stage. |
