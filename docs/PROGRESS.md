# PROGRESS — where we are

> Resume order for a fresh session: `CLAUDE.md` → this file → the current stage brief in `docs/stages/`.
> Last updated: 2026-08-25.

## Status

|            |                                                                   |
| ---------- | ----------------------------------------------------------------- |
| Phase      | **1 — Rust core & Tauri foundations**                             |
| Stage      | **5 complete** (`AppError` / `thiserror` / `Result`) — reviewed, 14/14 passing |
| Last done  | Stage 5, 2026-08-25. All 52 tests across five files green, clippy and fmt clean |
| Next       | Stage 6 — parse-don't-validate: `RawCase` → `TryFrom` → `Case`, loading a real TOML case file. Not yet written. |
| Blocked on | nothing                                                           |

---

## Decisions log

Newest first. Record the _why_, not just the what.

### 2026-08-25 — `AppError` is plain serializable data; `std::io::Error` never goes inside it.

Three decisions taken while writing Stage 5, none of them left open for him.

**1. Variants hold owned, serializable fields only.** `Io { path: String, message: String }`, not
`Io(#[from] std::io::Error)`.

`AppError` has to be `Serialize` (it crosses the IPC boundary — `ROADMAP.md` §1.2), `PartialEq` (so
tests can `assert_eq!` on a failure rather than pattern-matching it), and `Clone` (a `Session` will
hold the last failure). `std::io::Error` is none of the three, and putting it in the enum breaks all
three derives at once — reproduced, four errors: `E0277 Clone`, `E0369 ==`, `E0277 Eq`,
`E0277 serde::Serialize`.

The rejected alternative — `#[from]` plus a hand-written `Serialize` — buys automatic `?` conversion
and a `source()` chain. It loses on its own merits before the derives are even considered: a bare
`std::io::Error` does not know *which file* failed, because the path only exists at the call site.
So the call site has to add context regardless, which is one `.map_err` in the one function that
touches the filesystem (Stage 6). Paying for that with a hand-written serializer and no `assert_eq!`
is a bad trade.

Standing rule this establishes: **our failures are structured, foreign diagnostics are text.**
`SuspectNotFound` carries a `SuspectId`. `Io` / `Parse` / `Inference` carry a `String`, because those
sentences come from the OS, the TOML parser and llama.cpp respectively and we cannot decompose what
we did not write. Inventing structure we do not have is worse than admitting we have none.

**2. The wire format is `#[serde(tag = "kind", rename_all = "camelCase")]`, and the message is not
on it.** `{ "kind": "suspectNotFound", "id": 99 }`. The English sentence stays in `Display` (which
`thiserror` generates) for logs and `tracing`; React branches on `kind` and writes its own copy,
because a UI that may need to restyle or translate should not be handed a fixed English string from
the backend. Consistent with the project's core principle — Rust owns the truth, the presentation
layer owns the presentation.

Consequence, recorded because it is a runtime failure rather than a compile error: internal tagging
**cannot serialize a newtype variant holding an integer**. `SuspectNotFound(SuspectId)` compiles
fine and then fails at runtime with `cannot serialize tagged newtype variant … containing an
integer`. Hence: every variant of `AppError` uses named fields. The brief teaches this as a
types-check-shapes / tests-check-behaviour moment, matching his Stage 2 `Display` bug.

**3. Stage 5 does not touch the Tauri shell.** `ROADMAP.md` §1.2 says "every command returns
`Result<T, AppError>`", but a real `#[tauri::command]` needs managed state, which is Stage 7 and was
already sequenced as "the first stage that touches the Tauri shell". So Stage 5 makes the error
*serializable* and pins the JSON shape with `serde_json` in the integration test; the command
wrapper comes later. `error.rs` therefore stays a **domain module** under the `CLAUDE.md` purity
rule — no `tauri::`, `tokio::` or `std::fs` — and moves to `crates/core` intact if the split is ever
revisited. `serde` is a pure data-shape crate and does not violate the rule.

### 2026-08-25 — One owner for the visibility rule; enforcement moves to a newtype, not to split storage.

Two questions decided by the mentor. **Both existed only because of a mentor defect**, recorded
here rather than quietly fixed — see the process correction at the end of this entry.

**1. `facts_known_by` is deleted. `suspect_facts` is the single owner of the rule.**

The owned-`Vec<FactId>` version was a Stage 3 teaching scaffold and was documented as such when it
was issued. It is strictly weaker: any caller that wants ids writes `.map(|f| f.id).collect()`, and
callers that want the statements can no longer get them from the other one. Two implementations of
one rule drifted within a single stage — Stage 4's first submission dropped `!is_ground_truth_only`
from `suspect_facts` and would have fed the case solution into the LLM's context window. That is the
evidence; one function, one `!`, one place to audit.

Anticipated objection, rejected: "the IPC layer needs owned, serializable data, so keep the owned
version." It does, but that is `ipc/`'s job — it collects. `Case` does not grow a method because a
serializer downstream prefers a `Vec`.

`src-tauri/tests/case.rs` (Stage 3's spec) was amended by the mentor to reach the same assertions
through `suspect_facts` via a local `known_ids` helper. Coverage unchanged, 9/9 still green.
Specs migrate when the API they pinned is superseded; that is normal and worth him seeing once.

**2. Type-level enforcement will be a `VisibleFact` newtype, not two collections.**

`ROADMAP.md` §3.2 already specified this and it is upheld: `build_prompt` accepts only
`&[VisibleFact]`, produced solely by `Case::visible_to(..)`, so no hidden fact can reach the context
window because no function on that path accepts one.

The rejected alternative was splitting storage — `facts: Vec<Fact>` plus `solution: Vec<Fact>`.
Three reasons it loses:

- **It guards the wrong end.** Hiding the solution by location does nothing to `build_prompt(&Fact)`,
  which still accepts any fact from anywhere. The guarantee has to live in the *consumer's parameter
  type*, and only the newtype puts it there.
- **It encodes exactly one axis of visibility.** Phase 3.5 adds difficulty-gated facts and
  pressure-released facts; §3.3 adds per-suspect state. A filtered constructor extends to any number
  of predicates; a second `Vec` extends to none.
- **It costs a data migration for less safety.** The TOML format (§1.3) would need two tables and a
  split id space, versus a zero-copy `VisibleFact<'a>(&'a Fact)` that drops straight onto Stage 4's
  iterator.

The `is_ground_truth_only` flag itself **stays on `Fact`** — visibility is data (§1.1), so it can be
filtered mechanically. What changes is that exactly one function in the codebase is allowed to read
it. Likely refined to a two-variant enum at the same time; a `bool` with a name that long is an enum
in disguise.

**Sequencing:** `facts_known_by` gets deleted now — one line, no ceremony, tests already migrated.
The newtype is **Stage 6**, after Stage 5's `thiserror` work — it needs a struct holding a reference,
which means lifetimes in type definitions, which he has not met yet. Do not pull it forward.

**Process correction (the actual lesson of this entry).** Stage 4's brief was written without
re-reading `ROADMAP.md`. §3.2 had already assigned the visibility rule a single owner
(`case.visible_to` feeding `build_prompt(&[VisibleFact])`), so issuing `suspect_facts` *alongside*
`facts_known_by` created a duplicate implementation of a rule that was never supposed to have one.
When his first submission dropped `!is_ground_truth_only` from the new copy, that was presented to
him as evidence for a design question **he** should resolve — and he was asked for a position twice.

Both moves were wrong. The duplication was the mentor's, and the decision was the mentor's to make.
He pushed back: *"you made a mistake, it is not my responsibility to solve your problem... we are not
doing a product, you are teaching and I am learning."* Correct on both counts. `CLAUDE.md` now
carries two rules as a result: read `ROADMAP.md` before writing any brief, and never leave an
architectural question open as homework. See [[mentoring-loop]].

### 2026-08-21 — No `crates/core` workspace split. Everything in `src-tauri`.

Proposed and **rejected by Gabriel**: "I prefer to do everything on src-tauri and don't optimize
prematurely. This project is just to learn, won't be a production project."

He is right that it is reversible: if the domain modules stay pure, extraction later is `git mv` +
a `Cargo.toml` + fixing `use` paths — roughly an hour, not a weekend.

The mentor's original argument was oversold and was corrected: with incremental compilation the
loop is ~1–2s vs ~10–20s, not 2s vs 2min. The remaining real argument was _enforcement_ (a crate
boundary makes purity a compile error rather than a promise) — noted, not decisive.

**Tripwire for revisiting:** if `cargo test` inside the Tauri crate turns flaky on Windows for
Tauri-specific reasons (`generate_context!` config validation, `staticlib`/`cdylib` crate-type
linking, `tauri::test` mock issues), split immediately without further debate. The test loop is
the product in this format.

**Mitigation in force:** domain modules carry no `tauri::` / `tokio::` / `std::fs` imports.
If a scoring or domain function ever wants an `AppHandle`, that is the signal — discuss, do not
quietly reach for it.

### 2026-08-21 — Windows + Android, one engine.

`llama-cpp-2` compiled twice with different feature flags. Windows CUDA; Android via NDK with
Vulkan/OpenCL or CPU floor. Full reasoning: `docs/adr/ADR-0001-cross-platform-inference.md`.
Android is **Phase 2.5**, sequenced _after_ Phase 3.

### 2026-08-21 — Package manager: bun.

`package-lock.json` dropped, `bun.lock` in place, matches `tauri.conf.json`'s `bun run` commands.

---

## Stage log

### Stage 1 — `Difficulty` and `Tuning` ✅

`src-tauri/src/difficulty.rs`, spec at `src-tauri/tests/difficulty.rs`, brief at
`docs/stages/stage-01-difficulty.md`.

Built: four-variant `Difficulty` enum, `Tuning` payload struct, `Difficulty::ALL` associated
constant, `tuning()` returning per-difficulty settings.

Concepts landed: Rust's module system (`pub mod`, files are invisible until declared); `struct`
vs `enum` as "payload" vs "which one"; field-level `pub`; narrow integer types (`u8`) as a way to
make nonsense unrepresentable; `impl` blocks; associated constants; fixed-size arrays vs `Vec`;
`match` as an expression; **traits and `derive`**; and the headline lesson — `E0507 cannot move
out of *d which is behind a shared reference`, i.e. **move semantics and `Copy`**, plus why
`Difficulty` should be `Copy` and `Tuning` should not.

Hints reached: got to the `impl` skeleton with guidance; needed the array-literal correction
(wrote TS object-literal syntax `Easy: "easy"` inside `[...]`).

Review outcome: **pass**, with a polish pass requested — derives on `Tuning`, `Self` inside the
`impl` block, and doc comments on public items.

### Stage 2 — `SuspectId` and `FactId` ✅

`src-tauri/src/ids.rs`, spec at `src-tauri/tests/ids.rs`, brief at
`docs/stages/stage-02-ids.md`.

Built: two tuple structs wrapping a **private** `u32`, each with `new` / `get`, eight derives,
and hand-written `Display` and `From<u32>`.

Concepts targeted: the newtype pattern (vs bare `u32`, vs `String`, vs TS branded types); tuple
structs and field-level privacy as a real escape-hatch-free boundary; **derive vs hand-written
`impl`** and why std ships no `derive(Display)`; the `Debug`/`Display` split; the trait hierarchy
(`Copy: Clone`, `Eq: PartialEq`, `Ord: PartialOrd + Eq`); the `Hash`/`Eq` consistency contract and
why `f32` can never be a map key; **blanket impls** as the source of `to_string()` and `.into()`;
the orphan rule. Optional extra: `compile_fail` doctests as type-level tests.

Instruction given: add derives **one at a time**, driven by the top compiler error. `E0507` returns
in a new costume via `.iter().map(|f| f.get())`.

Hints reached: needed the concepts unpacked in-chat rather than the `<details>` hints — see the
teaching-format note below. Wrote all four impl blocks himself.

Review outcome: **pass** — 10/10, `clippy --all-targets -D warnings` clean, `cargo fmt` clean.
Polish requested: `Self(id)` instead of `SuspectId(id)` inside the impl blocks (third time this
note has come up), and doc comments moved off the `impl` blocks onto the two public types, saying
what an id _means_ rather than what a trait _is_.

**Teaching-format correction, 2026-08-22.** Mid-stage he said: _"the way you explain is like if I
were writing Rust for years... go slowly."_ The written brief and its progressive hints were
pitched too high, and dumping all remaining errors at once made it worse. What worked instead:
one concept per reply, built from zero (what a trait is → what `derive` generates → ownership and
moves → `impl Trait for Type` → `From`), each ending with a single command to run and a checkpoint
to come back to. `CLAUDE.md` and [[explanation-depth]] updated. Write Stage 3's brief at that
level from the start.

Notable moment: his first `Display` impl compiled but printed `"3 suspect #"`. The compiler had
nothing to say about it; the test caught it. Good place to have landed the shapes-vs-behaviour
distinction.

Spec was verified against a reference implementation in a throwaway crate before issuing.

### Stage 3 — `Fact`, `Suspect` and `Case` ✅

`src-tauri/src/case.rs`, spec at `src-tauri/tests/case.rs`, brief at
`docs/stages/stage-03-case.md`.

Built: `Suspect { id, name: String }`; `Fact { id, statement: String, known_by: HashSet<SuspectId>,
is_ground_truth_only }` with `reveal_to(&mut self)` / `is_known_by(&self)`; `Case` with `title` /
`briefing` public and `suspects` / `facts` **private**, plus `add_*` / `*_count` and
`facts_known_by(&self, SuspectId) -> Vec<FactId>`.

Concepts landed: `String` vs `&str` — structs own, parameters borrow, the constructor converts, and
why a struct cannot store the `&str` it was handed; `Vec<T>` vs Stage 1's fixed array; `HashSet` as
the right shape for a membership question, cashing in Stage 2's `Hash`/`Eq` derives; **`self` is an
explicit parameter** — `&self` reads, `&mut self` writes, and omitting it silently gives you an
associated function rather than a method; no uninitialized fields in Rust, so "I don't have that
data yet" always resolves to "what is the empty value"; `E0204` — why `Copy` is impossible once a
`String` is in the struct, and why `Clone` is not free; `E0382` on moving a `Fact` into the case;
closures and `.filter().map().collect()`.

Design decisions recorded in the brief: `Fact`/`Suspect` fields public (no invariant of their own),
`Case`'s collections private (they carry the "every referenced id exists" invariant from Stage 5).
`known_by` is a `HashSet`, overriding the `Vec` sketched in `ROADMAP.md` §1.1. `facts_known_by`
returns owned `Vec<FactId>` on purpose, so Stage 4 can upgrade it to the iterator-of-references
version and make the lifetime the whole subject.

Hints reached: needed the full signature skeleton handed over (Hint 5's shape) after trying and
failing to derive signatures from the test's call sites — see the teaching note below. Wrote every
body himself, and reached for the iterator chain in `facts_known_by` unprompted rather than the
loop.

Review outcome: **pass** — 9/9, `clippy --all-targets -D warnings` clean, `cargo fmt` clean. Doc
comments present on all three types and saying what the type *means*; that polish note, open since
Stage 1, is now closed. `Self` used throughout without prompting.

Notable moment: with `!` missing from the ground-truth condition, **both** visibility tests failed
in opposite directions — one returned everything, the other returned nothing. Used to teach the
debugging tell: every case inverted rather than some cases wrong means a missing negation.

**Teaching notes, 2026-08-23.**
1. Asking him to read signatures off the test's call sites was a step too far and cost several
   rounds — he guessed instead (`Fact::new` taking every field, `is_known_by` with no `self`,
   `statement` missing entirely). From Stage 4 on, **issue the signature skeleton with the brief**,
   bodies elided. Shapes are scaffolding; bodies are the exercise.
2. Teach `todo!()` before Stage 4. He stubbed with `-> Self {}` and got a wall of `E0308`;
   `todo!()` type-checks as anything, so a stubbed file links and he gets a red suite to drive
   instead of a build error.
3. Jargon, not just depth: "one boolean in your filter closure" did not parse, though he had
   written the closure himself. Name Rust vocabulary against its TS equivalent on first use.

Spec was verified against a reference implementation in a throwaway crate before issuing, and his
implementation re-verified in the same crate at review.

### Stage 4 — borrowing, `Option<&T>` and lifetimes ✅

Spec at `src-tauri/tests/borrowing.rs` (12 tests), brief at `docs/stages/stage-04-borrowing.md`.
Implementation goes in the existing `src-tauri/src/case.rs`.

Asked for: `Case::suspect(&self, SuspectId) -> Option<&Suspect>`,
`Case::fact_mut(&mut self, FactId) -> Option<&mut Fact>`,
`Case::suspect_facts(&self, SuspectId) -> impl Iterator<Item = &Fact>`, and a free
`longer_statement<'a>(&'a Fact, &'a Fact) -> &'a Fact`.

Concepts targeted: `&T` / `&mut T` / `T` as three distinct things TypeScript collapses into one;
the borrow rule (many readers XOR one writer) and non-lexical lifetimes; `Option<&T>` as a return
type with no `null` and no `?.` escape hatch; `iter()` vs `iter_mut()` and the `_mut` naming
convention; **lifetimes as a named region relating inputs to outputs, not a duration**, plus
elision and why `&self` methods never needed one; `impl Trait` in return position and iterator
laziness; closure capture modes and why `move` is required when the iterator outlives the frame.

Three compiler errors are load-bearing and each was reproduced before issuing:
`E0502` (commented-out line in `one_writer_excludes_every_reader`), `E0106` (written deliberately
without the annotation first, per rule 4), `E0373` (the missing `move`).

Teaching notes applied from Stage 3: signature skeleton issued **with the brief**, bodies elided;
`todo!()` taught in section 0 *before* the task, including the case where it does not work — a
function returning `impl Iterator` stubbed with `todo!()` infers `()` and fails `E0277`, so that one
stubs with `std::iter::empty()`; Rust vocabulary (borrow, closure, capture, elision, NLL) named
against its TS equivalent on first use.

Rule added for this stage: **no `clone()`**. Cloning out of a borrow error is precisely the habit
this stage exists to prevent.

Open design question deliberately left for review: `facts_known_by` (owned ids) and `suspect_facts`
(borrowed facts) now answer the same question in two shapes. He was told to arrive with an opinion.

Spec verified against a reference implementation in a throwaway crate before issuing — 12/12
passing, `clippy --all-targets -D warnings` clean.

Hints reached: none reported. Wrote all four bodies himself, `move` included — he hit `E0373` and
read it rather than asking.

Review outcome: **pass on the second round.** First submission was 7/12 with two bugs, and he had
not run the suite before saying he was done — told directly to run it before declaring. Both bugs
were his own: `suspect_facts` filtered on `is_known_by` alone and **dropped the
`!is_ground_truth_only` half**, and `longer_statement` returned `b` on a tie. Second submission
12/12, `clippy --all-targets -D warnings` clean, `cargo fmt` clean, doc comments retained.

On the dropped ground-truth filter: it was a real bug and he fixed it, but do **not** keep citing it
as evidence for anything. The rule only had two implementations because the mentor issued a second
one without reading `ROADMAP.md` §3.2 — see the decisions log entry for 2026-08-25. Treat it as a
one-line filter bug, which is all it was from his side.

### Stage 5 — `AppError`, `thiserror` and `Result` ✅

Spec at `src-tauri/tests/errors.rs` (14 tests), brief at `docs/stages/stage-05-errors.md`.
New file `src-tauri/src/error.rs`; edits to `case.rs`, `ids.rs`, `lib.rs`, `Cargo.toml`.

Asked for: `AppError` (seven variants, `thiserror::Error` + `Serialize`, named fields throughout),
`pub type AppResult<T>`, `Serialize` on both id newtypes, and three methods on `Case` —
`require_suspect(&self, SuspectId) -> AppResult<&Suspect>`,
`require_fact_mut(&mut self, FactId) -> AppResult<&mut Fact>`,
`reveal(&mut self, FactId, SuspectId) -> AppResult<()>`.

Concepts targeted: `Result<T, E>` as `Option` with a reason attached, and why Rust has no exceptions
for recoverable failure (the failure is in the signature or it does not exist); `thiserror` as a
derive macro that writes `Display` + `impl std::error::Error`, and `#[error("...")]` as the *body*
of that `Display` — which cashes in the `Display` he hand-wrote in Stage 2, since `{id}` renders as
`suspect #99`; `thiserror` vs `anyhow` and why this crate is a library boundary; `Option::ok_or` as
the bridge, and `ok_or` vs `ok_or_else` (eager vs lazy construction); `?` — what it desugars to,
that it is control flow returning from the *enclosing function*, and that it calls `From::from`
(identity today, the hook Stage 6 turns on); `#[must_use]` on `Result`; the unit type `()`;
`cargo add` and feature flags; type aliases; serde's internally-tagged enum representation and
newtype transparency on the wire.

Four compiler/clippy errors are load-bearing and each was reproduced before issuing:
`E0502` in `reveal` (holding the shared borrow from `require_suspect` across the `&mut` call — the
fix is to drop the borrow with a bare `?;`, not to `clone()`); `clippy::unnecessary_lazy_evaluations`
if he reaches for `ok_or_else`; `unused_must_use` if he drops the `?`; `E0277` if `SuspectId` is
missing its own `Serialize`. Plus the runtime serde failure from a newtype variant, which the type
system cannot catch — the test does.

Teaching notes applied: signature skeleton issued with the brief, bodies elided; `todo!()` noted as
working for all three this time (no `impl Trait` returns); every Rust term named against its TS
equivalent on first use; the three architecture questions decided by the mentor and stated as
decided, with the reasoning in the decisions log rather than posed to him.

Rules for this stage: derives on `AppError` added **one at a time** (six derives, six failure modes);
no `clone()` in the `require_*` methods; write `reveal` the broken way first and read `E0502`.

**Brief rewritten the same day after he pushed back a second time on explanation depth** — *"You keep
using language too technical and too into rust... this is the FIRST time I code in rust. Be more
pedagogical"* — and clarified he meant the `docs/stages/*.md` files, not just the chat replies. The
first draft was an argued document: it defended its own architecture decisions at him (four compiler
errors as evidence against `#[from] std::io::Error`), ran long essay-style paragraphs, and used
words he has never been taught (extern prelude, supertrait, orthogonal, desugars, internally tagged
representation). The rewrite leads every idea with a concrete example, keeps paragraphs to two or
three sentences, moves the architecture reasoning to a three-line pointer at the decisions log, and
cuts the concept count from eleven to six. Test file unchanged. See [[explanation-depth]].

Checkpoint counts in the brief were measured, not guessed: enum written with all three `Case`
methods stubbed → 2/14; plus `require_suspect` and `require_fact_mut` → 7/14; plus `reveal` → 12/14;
plus the `#[serde(tag = "kind", rename_all = "camelCase")]` line → 14/14.

**Review outcome: pass, first submission.** 14/14, and all 52 tests across the five test files still
green — no regressions in Stages 1–4. `clippy --all-targets -D warnings` clean, `cargo fmt` clean.
Re-verified by dropping his `src/` into the throwaway crate.

He wrote every body himself. Two things he did unprompted and well: `require_suspect` /
`require_fact_mut` reuse the Stage 4 lookups rather than repeating the `find` (he had duplicated it
at first and fixed it when told once), and he solved `CaseNotFound`'s quoted-slug message with a raw
string, `#[error(r#"no case file named "{slug}" was found"#)]`, rather than the `{slug:?}` the hint
suggested — a different and defensible answer.

Polish note, open: **the three new `Case` methods and both items in `error.rs` have no doc
comments**, while every other public item in `case.rs` does. This note was raised in Stages 1 and 2
and closed in Stage 3; it has reopened on new code. Mention once at the start of Stage 6 rather than
as a rule.

Also noted for him, not a defect: in `reveal` he shadows the `fact: FactId` parameter with
`let fact = ...` holding a `&mut Fact`. Legal and idiomatic Rust, and TypeScript does not allow it in
the same scope, so it is worth him knowing he did it deliberately rather than by accident.

**Where he got stuck, and what it cost** — both are mentor defects, not his:

1. **`self.require_suspect(to)?;` — a line with `?` that stores nothing.** This took four exchanges
   and ended with him saying *"No idea dude"* and *"either I am dumb or you are not being clear"*.
   The wording that failed was "a line that keeps nothing", which reads as "a line that does
   nothing". What finally worked: state that `?` does **two** jobs — (1) stop the function and
   return the error, (2) hand back the value — and that job 1 is the point, job 2 is leftover. Then
   trace `reveal(FactId(1), SuspectId(99))` line by line with and without the check. Then show his
   own method with a commented gap where the line goes.
   Interim damage: he deleted the `require_suspect` call entirely to make `E0502` go away, which
   compiled and silently dropped the validation. Worth reusing as a lesson — *making the compiler
   happy by deleting the code it complained about* is the most common way a borrow error turns into
   a behaviour bug.
2. **Where `#[serde(tag = ...)]` physically goes.** The brief said "add one line above the enum" and
   never showed it in place. He could not place it. Fix that generally: **when a brief introduces an
   attribute, show the two or three surrounding lines of the real file**, not a description. Attribute
   stacking (`#[derive(..)]` and `#[serde(..)]` both above the same item, order irrelevant) had never
   been taught — he had only ever seen one attribute at a time.

Spec verified against a reference implementation in a throwaway crate — 14/14 passing,
`clippy --all-targets -D warnings` clean, and the test file as written to his disk is `cargo fmt`
clean.

---

## Phase 0 leftovers (deferred, not forgotten)

- [x] `.gitattributes` + line-ending renormalization (`940912b`)
- [x] `/target`, `models/`, `*.gguf` ignored
- [x] Package manager settled on bun
- [x] **`docs` is still listed in `.gitignore`** — every planning document, ADR and stage brief is
      therefore untracked and will not survive a fresh clone. Remove that line.
- [ ] Delete the `greet` demo command, the template `App.tsx`, `public/vite.svg`,
      `src/assets/react.svg`
- [ ] Rewrite `README.md` — it still describes the Tauri template
- [ ] Add `rust-toolchain.toml` pinning a stable version
- [ ] Delete the orphaned root `target/` directory (left over from the reverted workspace)

---

## What comes next

Phase 1 continues, roughly in this order — each becomes one stage with its own failing test:

1. ~~`SuspectId` / `FactId` newtypes~~ — done, Stage 2.
2. ~~`Fact` with `known_by`, and `Case` holding suspects and facts~~ — issued, Stage 3.
3. ~~Borrowing and lifetimes: `suspect_facts` returning `impl Iterator<Item = &Fact>`~~ — issued, Stage 4.
4. ~~`AppError` with `thiserror`, and `Result` across the IPC boundary~~ — done, Stage 5.
5. **Next:** parse-don't-validate: `RawCase` → `TryFrom` → `Case`, loading a real TOML case file.
   This is where `?`'s `From::from` conversion and the `Io` / `Parse` / `CaseNotFound` variants get
   their first real use, and where the `.map_err` promised in Stage 5's brief actually gets written.
6. The `VisibleFact<'a>` newtype (`ROADMAP.md` §3.2) — needs lifetimes in type definitions.
7. `tauri::State` and managed app state — the first stage that touches the Tauri shell, and where
   `AppError` finally crosses a real `#[tauri::command]`.
