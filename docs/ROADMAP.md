# Interrogator — Engineering & Learning Roadmap

**Owner:** Gabriel Chamorro Moris
**Budget:** 2–4 h/week. At that rate this is a **9–12 month build**. Plan accordingly; do not compress it by skipping Phase 1.
**Target hardware:** Windows + NVIDIA GPU, 8 GB+ VRAM.
**Inference backend:** `llama-cpp-rs` (llama.cpp bindings), in-process, CUDA offload.

---

## 0. Current state of the repo (as audited)

`interrogatory-ai` is an unmodified `create-tauri-app` scaffold: Tauri v2, React 19, Vite 7, TypeScript 5.8. One commit (`0d27598 init commit`). `src-tauri/src/lib.rs` contains only the `greet` demo command. There is no module structure, no error type, no managed state, no domain model.

Three defects to clear before any feature work:

1. **CRLF churn.** `git status` shows all 20 files modified; `git diff --stat` is `2479 insertions(+), 2479 deletions(-)`. Every file was committed with LF and checked out as CRLF. There is no `.gitattributes` and `core.autocrlf` is unset. Every future diff will be unreadable until this is fixed.
2. **Package-manager mismatch.** `tauri.conf.json` runs `bun run dev` / `bun run build`, but the repo has `package-lock.json` and no bun lockfile. Pick one, delete the other's lockfile.
3. **Demo code still present.** `greet`, the Vite/React/Tauri logo page, `src/assets/react.svg`. Delete it in Phase 0 — leaving it invites cargo-culting its shape.

---

## 1. The architectural decision that matters most

> **Ground truth lives in Rust. The LLM is a constrained actor, never the source of truth.**

The case's real facts, who knows what, and the scoring rubric are Rust data. The model is handed a *filtered view* of that data and a persona, and its output is treated as untrusted text. Everything else in this design follows from that one rule:

- Scoring is reproducible, because the deterministic tier runs on `FactId`s, not on model opinion.
- The suspect cannot leak what it was never given, because hidden facts never enter the context window — enforced at the type level, not by prompt wording.
- Phases 1, 3 and 4 can be built and tested with **zero inference**, because a mock engine satisfies the same trait.

The second decision, which enforces the first physically:

> **Split into a Cargo workspace.** A pure `interrogator-core` crate that knows nothing about Tauri, async, or IO; and `src-tauri` as a thin shell around it.

```
interrogatory-ai/
├─ Cargo.toml                  # [workspace] members = ["crates/core", "src-tauri"]
├─ crates/
│  └─ core/                    # interrogator-core — pure, sync, no tauri, no tokio
│     └─ src/
│        ├─ lib.rs
│        ├─ case.rs            # Case, Suspect, Fact, FactId, Difficulty
│        ├─ transcript.rs      # Turn, Speaker, Transcript
│        ├─ prompt.rs          # deterministic String assembly (snapshot-testable)
│        └─ scoring.rs         # Report, Verdict, ScoreBreakdown
└─ src-tauri/
   └─ src/
      ├─ main.rs               # entry only
      ├─ lib.rs                # tauri::Builder wiring only
      ├─ error.rs              # AppError (thiserror) + Serialize for the IPC boundary
      ├─ state.rs              # AppState, managed via tauri::State
      ├─ session/              # stateful interrogation orchestration
      ├─ llm/
      │  ├─ mod.rs             # trait InferenceEngine
      │  ├─ llama.rs           # llama-cpp-rs implementation
      │  └─ mock.rs            # deterministic engine — build this FIRST
      └─ ipc/                  # #[tauri::command] wrappers, nothing else
```

**Invariants to hold for the whole project:**

- If `interrogator-core` ever needs `tauri`, `tokio`, or `std::fs`, the boundary has leaked. Fix the boundary, not the import.
- `ipc/` functions deserialize, delegate, and map errors. Nothing else. Commands are the least interesting code in the app.
- No `unwrap()` or `expect()` outside `main.rs` and `#[cfg(test)]`. Enforce with `#![deny(clippy::unwrap_used, clippy::expect_used)]`.
- Every phase ends with `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, and a commit.

---

## Phase 0 — Repo hygiene (1 session, ~2 h)

No Rust learning here. This is removing friction that would otherwise tax every future session.

- [ ] Add `.gitattributes`: `* text=auto eol=lf`, plus `*.png binary`, `*.ico binary`, `*.icns binary`. Then `git add --renormalize . && git commit`.
- [ ] Choose npm **or** bun. Align `tauri.conf.json`'s `beforeDevCommand`/`beforeBuildCommand` with the lockfile you keep.
- [ ] Delete `greet`, the demo `App.tsx`, `src/assets/react.svg`, `public/vite.svg`.
- [ ] Convert to a Cargo workspace; create `crates/core` with an empty `lib.rs`.
- [ ] Add deps: `thiserror`, `tracing`, `tracing-subscriber`, `serde`, `toml`.
- [ ] Add `rust-toolchain.toml` pinning a stable version, and `rustfmt.toml`.
- [ ] `.gitignore`: `*.gguf`, `models/`. Model weights never enter git.
- [ ] Rewrite `README.md` to describe Interrogator, not the Tauri template.

**Exit criterion:** `git status` is clean, `cargo clippy -D warnings` passes, `bun/npm run tauri dev` opens a blank window with your own title.

---

## Phase 1 — Rust core & Tauri foundations (6–8 sessions)

**Rust concepts:** ownership vs. borrowing, move semantics, structs, enums with data, pattern matching, `Option`/`Result`, `?`, custom errors, traits, `TryFrom`, interior mutability.

### 1.1 — Domain model (`crates/core`)
Model `Case`, `Suspect`, `Fact`, `FactId`, `Difficulty`. Design decisions to make deliberately, not by accident:
- `FactId` as a newtype (`struct FactId(u32)` or a `&'static str` wrapper) — not a bare `String`. This is your first taste of making illegal states unrepresentable.
- `Fact { id, statement, known_by: Vec<SuspectId>, is_ground_truth_only: bool }` — visibility is *data*, so it can be filtered mechanically. *(Amended: `known_by` is a `HashSet<SuspectId>`, and the accessor is `Case::suspect_facts` — see the decisions log in `PROGRESS.md`, 2026-08-21 and 2026-08-25.)*
- `Difficulty` as an enum with associated data or an `impl` returning a tuning struct (`temperature`, `evasiveness`, `facts_volunteered`).

**Drill:** write `fn suspect_facts<'a>(case: &'a Case, s: SuspectId) -> impl Iterator<Item = &'a Fact>`. Explain to yourself why the lifetime is needed and what happens if you return `Vec<Fact>` instead.

### 1.2 — Error handling (`src-tauri/src/error.rs`)
`AppError` via `thiserror`, with `impl Serialize` so it can cross the IPC boundary as a structured object rather than a string. Variants for `CaseNotFound`, `Io`, `Parse`, `Inference`, `InvalidState`. Every command returns `Result<T, AppError>`.

**Rule:** `anyhow` is for binaries and prototypes; `thiserror` is for library boundaries. Your core crate gets `thiserror`. Do not reach for `anyhow` in `crates/core`.

### 1.3 — Case files (parse, don't validate)
Case format in TOML. Two types: `RawCase` (what serde deserializes — permissive) and `Case` (validated — every `FactId` referenced actually exists, every suspect has ≥1 known fact). Bridge them with `impl TryFrom<RawCase> for Case`. After that conversion, the rest of the codebase can never see an invalid case.

Write **two** real case files. One is not enough to find the abstraction.

### 1.4 — Managed state
`AppState { session: Mutex<Option<Session>> }`, registered with `.manage()`, read in commands via `tauri::State<'_, AppState>`.

**Concept to internalize now, because Phase 2 will punish you for it:** `std::sync::Mutex` guards are not `Send` across `.await`. Understand why before you write your first async command.

### 1.5 — Mock inference engine
```rust
#[async_trait]
pub trait InferenceEngine: Send + Sync {
    async fn stream(&self, prompt: Prompt, tx: Sender<Token>, cancel: CancellationToken)
        -> Result<Completion, AppError>;
}
```
Implement `MockEngine` returning canned, deterministic lines with an artificial delay. This unblocks Phases 3 and 4 entirely and keeps your test suite fast forever.

**Exit criterion:** app loads a case from disk, renders the intro screen, and you can "interrogate" the mock suspect through a real chat UI. No LLM involved. All domain logic unit-tested with `cargo test -p interrogator-core`.

---

## Phase 2 — Async Rust & local LLM (10–14 sessions — the hard phase)

**Rust concepts:** `async`/`await`, tokio runtime, `Send`/`Sync`, `Arc`, channels (`mpsc`, `broadcast`), `spawn_blocking`, dedicated threads, cancellation, FFI lifetimes.

### 2.1 — Async inside Tauri
Tauri v2 already runs a tokio runtime. Learn what `async fn` commands actually do, and when to use `tauri::async_runtime::spawn` vs. `spawn_blocking`.

### 2.2 — Get `llama-cpp-rs` to build on Windows *(budget a full session for this alone)*
MSVC build tools, CMake, CUDA toolkit, the crate's `cuda` feature flag. This will fight you. It is a separate task from writing any inference code — do not schedule feature work in the same session. Record the exact working toolchain versions in `docs/BUILD.md` the moment it compiles.

Model to start with: an 8B instruct model at Q4_K_M (~4.7 GB) fits 8 GB VRAM with room for context. Keep weights in `models/`, gitignored, with a documented download step.

### 2.3 — The threading model *(the most important lesson in the project)*
llama.cpp decoding is CPU/GPU-bound blocking FFI. It must **not** run on a tokio worker.

Correct shape: one dedicated OS thread owns the `LlamaContext`; it receives work over a channel and emits tokens over another channel; the async side only ever talks to channels. `LlamaModel` behind an `Arc`, loaded once at startup; contexts created per session.

Do this wrong and you get a UI that freezes and an async runtime that starves — which is exactly the lesson. Consider deliberately doing it wrong once, observing the freeze, then fixing it.

### 2.4 — Streaming to React
`mpsc::channel` → `app_handle.emit("interrogation://token", payload)` → `listen()` on the React side. Include a session/turn ID in every event so a late token from a cancelled generation can be discarded by the frontend.

### 2.5 — Cancellation
`tokio_util::sync::CancellationToken`, checked inside the decode loop. Required behaviour: user sends a new question mid-generation, or hits stop. Teaches `select!`, drop semantics, and cleanup ordering.

### 2.6 — Context & KV cache
Reuse the KV cache across turns rather than re-prompting the whole transcript. Decide and document the truncation strategy when the transcript exceeds the context window (recommendation: keep system prompt + first N turns + last M turns, summarize the middle).

**Exit criterion:** real local model, streaming token-by-token into the chat UI, cancellable mid-sentence, UI never blocks, `MockEngine` still passes the same tests.

---

## Phase 3 — Case engine, knowledge gating & scoring (8–10 sessions)

**Rust concepts:** trait objects vs. generics, iterators and closures, snapshot testing, state machines via enums.

### 3.1 — Prompt assembly in Rust
`fn build_prompt(case: &Case, suspect: &Suspect, difficulty: Difficulty, transcript: &Transcript) -> Prompt`, living in `crates/core`, pure and synchronous. Snapshot-test the rendered string (`insta`). Prompts are code; regressions in them are bugs, and they must show up in a diff.

### 3.2 — Knowledge gating, enforced by types
*(Confirmed 2026-08-25 over the split-storage alternative. Scheduled as Stage 6; `Case::suspect_facts` is its Phase-1 ancestor. Reasoning in the decisions log in `PROGRESS.md`.)*

`build_prompt` accepts only `&[VisibleFact]`, produced solely by `case.visible_to(suspect_id)`. There is no path by which a hidden fact reaches the context window, because there is no function that accepts one. Do not rely on "the system prompt tells it not to reveal X" — that is not enforcement.

### 3.3 — Interrogation state machine
`enum Phase { Intro, Interrogating { turns: u32 }, Reporting, Scored }`. Illegal transitions are unrepresentable. Track per-suspect pressure/consistency here — the game logic that makes it a game rather than a chat window.

### 3.4 — Two-tier scoring
- **Tier 1, deterministic:** extract claimed `FactId`s from the player's report (structured output from a constrained LLM call, or keyword/embedding match) and compare against ground truth. Precision/recall over facts → the numeric score. Reproducible and unit-testable.
- **Tier 2, narrative:** a separate LLM call generates the written critique, given the Tier-1 result.

**Never let the model own the number.** A score you cannot reproduce is a score you cannot test or balance.

### 3.5 — Difficulty tuning
Map `Difficulty` → temperature, evasiveness instructions, facts volunteered per turn, whether the suspect lies outright and how consistently, number of red herrings in the case.

**Exit criterion:** end-to-end playthrough — intro → interrogation → report → score + critique — on at least two cases at two difficulties.

---

## Phase 4 — React UI & typed boundary (6–8 sessions)

This is where your existing seniority pays off; keep it tight and do not let it expand to fill the space.

- One typed IPC module (`src/lib/ipc.ts`). Every `invoke` and `listen` in the app goes through it. Nothing calls `invoke` directly.
- Generate TS types from the Rust structs with `ts-rs` or `specta` so the boundary cannot silently drift. Do this rather than hand-writing interfaces.
- Screens: Case Intro → Interrogation (streaming chat, cancel button, turn counter) → Report submission → Scoring/critique.
- Handle the states that only exist because inference is real: model loading, first-token latency, cancelled generation, inference error.

---

## Phase 5 — 3D suspect faces (deferred)

React Three Fiber, blend shapes, expressions driven by sentiment/pressure from the Rust side.

**Do not start this before Phase 3 is stable.** It is the most visible and least load-bearing part of the app, which makes it the single largest scope-creep risk in the project.

---

## Cross-cutting practices

| Practice | Why |
|---|---|
| `cargo clippy --all-targets -- -D warnings` every session | Clippy is the cheapest Rust tutor you have. Read the lint names, not just the fixes. |
| Domain tests run without launching Tauri | If scoring can't be tested headlessly, the layering is wrong. |
| `MockEngine` stays working forever | It is your fast test path and your offline dev path. |
| `tracing` from day one, not `println!` | Async token streams are unreadable in `println!`. |
| One-paragraph log per session in `docs/LEARNING-LOG.md` | Name the Rust concept the session actually taught. Ownership lessons are learned by being annoyed by them and then articulating why. |

---

## Risk register

| Risk | Mitigation |
|---|---|
| `llama-cpp-rs` won't build on Windows/CUDA | Isolated session, no feature work alongside it. Document exact versions. Fallback: `llama-server` sidecar over HTTP for one phase — same `InferenceEngine` trait, so the swap is contained. |
| Blocking FFI on the async runtime | Dedicated thread + channels (§2.3). Non-negotiable. |
| Everything ends up in `lib.rs` | Workspace split in Phase 0 makes this structurally hard. |
| Prompt-engineering rabbit hole | Snapshot-tested prompts; time-box tuning sessions. |
| 3D scope creep | Phase 5 is gated on Phase 3 being finished. |
| Model weights in git | `.gitignore` entry in Phase 0 + documented download script. |
| 2–4 h/week fragmentation | Every session ends at a commit, with the next action written down. Never stop mid-refactor. |

---

## Next three actions

1. Phase 0 in one sitting: `.gitattributes` + renormalize, pick the package manager, delete the demo code.
2. Convert to a Cargo workspace with an empty `crates/core`.
3. Write `Case`, `Suspect`, `Fact`, `Difficulty` and one real case file in TOML — before any inference code exists.

---

## Phase 2.5 — Android bring-up (added 2026-08-21)

Android is **not** a Phase 2 concern; it slots in after Phase 3, once the game works. Same engine (`llama-cpp-2`, built for `aarch64-linux-android` via the NDK), different model and memory budget. Full reasoning, constraints and the zero-cost design rules to follow now: **[docs/adr/ADR-0001-cross-platform-inference.md](adr/ADR-0001-cross-platform-inference.md)**.
