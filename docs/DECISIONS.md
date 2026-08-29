# DECISIONS — architecture, newest first

A lookup, not a read-through. Five lines per entry: what was decided, why, what was rejected, what
it costs. Longer reasoning belongs nowhere — if an entry needs more than five lines, the decision
was two decisions.

**Closing a decision means amending every document that states the opposite.** Grep for the rejected
thing before you consider it recorded.

---

### 2026-08-29 — Cases are generated structure-first, and solvability is not a parse rule

**Decided.** Rust generates the case skeleton from a `Difficulty` and a seed — culprit, fact roles,
`known_by` distribution — and the model only writes the prose. `is_solvable` lives in `generator.rs`
and nowhere else; generated cases enter through the ordinary `parse_case` path. Scheduled as §3.6.
**Why.** A model asked for "a mystery" writes atmosphere, not a soluble one: two viable culprits, or
a culprit no visible fact points at. Structure is a constraint problem; prose is what models are for.
**Rejected.** Prompting the model for a whole TOML case and validating after — the failure is
semantic, so validation can only reject, never repair. Also rejected: ten hand-authored cases per
difficulty; two or three, as fixtures and quality bar.
**Costs.** `parse_case` keeps its four structural checks and gains nothing — solvability is the
generator's obligation about its own output, deliberately not a fifth rule. Phase 3 grows to 10–13
sessions. Nothing in Stages 6–9 changes.

### 2026-08-29 — Hard concept budget, and the docs split into four files

**Decided.** Three new concepts per stage maximum, one of them the headline; a 200-line ceiling and
fixed template for briefs; `PROGRESS.md` split into `PROGRESS` / `DECISIONS` / `STAGE-LOG`; a concept
ledger at `docs/CONCEPTS.md`.
**Why.** Briefs grew 148 → 657 lines across six stages, teaching 9–13 concepts each, *after* two
requests to slow down. Every existing rule governed wording, none governed volume.
**Rejected.** Keeping the "near 300 lines" guidance and trying harder — it had already failed twice.
**Costs.** More stages, each smaller. Stage 6 becomes 6a–6d. Evidence in `MENTOR-NOTES.md`.

### 2026-08-27 — Stage 6 is case files, and it does not touch the filesystem

**Decided.** TOML with `[[suspects]]` / `[[facts]]`, ids as plain integers, `known_by` a list of
integers, `is_ground_truth_only` optional. Raw types hold `u32` and `String`, never the id newtypes.
Three new structured `AppError` variants: `DuplicateSuspect`, `DuplicateFact`, `SuspectKnowsNothing`.
**Why.** `SuspectId` means "an id that exists in this case" — precisely the claim the file has not
earned. Ids appear on the far side of the conversion and nowhere else.
**Rejected.** One `InvalidCase { message: String }` covering all three failures — throws away the id
and hands React a sentence to regex. Also rejected: reading the file here.
**Costs.** The filesystem read moves to Stage 8 with `Io` / `CaseNotFound` and a shell-side
`storage.rs`; tests reach the two real files with `include_str!`, so `case_file.rs` stays pure.
`SuspectId` / `FactId` get `Deserialize` at Stage 9, when IPC commands take ids as arguments.

### 2026-08-25 — `AppError` holds owned, serializable data; `std::io::Error` never goes inside it

**Decided.** `Io { path: String, message: String }`, not `Io(#[from] std::io::Error)`. Standing rule:
**our failures are structured, foreign diagnostics are text.**
**Why.** `AppError` must be `Serialize` (it crosses IPC), `PartialEq` (tests `assert_eq!` on it) and
`Clone` (a `Session` will hold the last failure). `std::io::Error` is none of the three.
**Rejected.** `#[from]` plus a hand-written `Serialize`. It also loses on its own merits: a bare
`std::io::Error` does not know *which file* failed, so the call site must add context regardless.
**Costs.** One `.map_err` at each foreign boundary. `SuspectNotFound` carries a `SuspectId`;
`Io` / `Parse` / `Inference` carry a `String`, because we did not write those sentences.

### 2026-08-25 — The IPC wire format is `#[serde(tag = "kind", rename_all = "camelCase")]`

**Decided.** `{ "kind": "suspectNotFound", "id": 99 }`. The English sentence stays in `Display` for
logs; React branches on `kind` and writes its own copy.
**Why.** Rust owns the truth, the presentation layer owns the presentation. A UI that may restyle or
translate should not be handed a fixed English string from the backend.
**Rejected.** Putting the `Display` message on the wire.
**Costs.** Every variant must use **named fields** — internally-tagged serde cannot serialize a
newtype variant holding an integer. `SuspectNotFound(SuspectId)` compiles and fails at run time.

### 2026-08-25 — One owner for the visibility rule, enforced by a newtype

**Decided.** `facts_known_by` deleted; `Case::suspect_facts` is the single owner until the
`VisibleFact<'a>` newtype lands in **Stage 7**, after which `build_prompt` accepts only
`&[VisibleFact]`, produced solely by `Case::visible_to(..)`.
**Why.** Two implementations of one rule drifted inside a single stage. The guarantee has to live in
the *consumer's parameter type*; nothing else stops `build_prompt(&Fact)` accepting any fact.
**Rejected.** Splitting storage into `facts` + `solution`. It guards the wrong end, encodes exactly
one axis of visibility (Phase 3.5 adds difficulty-gated and pressure-released facts), and costs a
data migration for less safety.
**Costs.** `is_ground_truth_only` stays on `Fact` — visibility is data — but exactly one function is
allowed to read it. Likely becomes a two-variant enum at the same time.

### 2026-08-21 — No `crates/core` workspace split. Everything in `src-tauri`.

**Decided.** One crate. Rejected by Gabriel: *"I prefer to do everything on src-tauri and don't
optimize prematurely. This project is just to learn, won't be a production project."*
**Why it holds.** Extraction later is `git mv` + a `Cargo.toml` + fixing `use` paths — about an hour,
if the domain modules stay pure.
**Rejected argument, corrected.** The mentor's compile-time case was oversold: with incremental
compilation the loop is ~1–2 s vs ~10–20 s, not 2 s vs 2 min. The real argument was *enforcement* —
a crate boundary makes purity a compile error rather than a promise. Noted, not decisive.
**Tripwire.** If `cargo test` inside the Tauri crate turns flaky on Windows for Tauri-specific
reasons (`generate_context!` validation, `staticlib`/`cdylib` linking, `tauri::test` mocks), split
immediately without further debate. The test loop is the product in this format.
**Mitigation in force.** Domain modules carry no `tauri::` / `tokio::` / `std::fs` imports. If a
domain function ever wants an `AppHandle`, that is the signal — discuss, do not quietly reach for it.

### 2026-08-21 — Windows and Android on one engine

**Decided.** `llama-cpp-2`, compiled twice with different feature flags. Windows CUDA; Android via
the NDK with Vulkan/OpenCL or a CPU floor. Android is **Phase 2.5**, sequenced after Phase 3.
**Why / rejected.** Full reasoning in `adr/ADR-0001-cross-platform-inference.md`.

### 2026-08-21 — Package manager is bun

**Decided.** `package-lock.json` dropped, `bun.lock` in place, matching `tauri.conf.json`'s
`bun run` commands.
