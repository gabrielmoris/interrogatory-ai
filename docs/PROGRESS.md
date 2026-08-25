# PROGRESS — where we are

> Resume order for a fresh session: `CLAUDE.md` → this file → the current stage brief in `docs/stages/`.
> Last updated: 2026-08-25.

## Status

|            |                                                                   |
| ---------- | ----------------------------------------------------------------- |
| Phase      | **1 — Rust core & Tauri foundations**                             |
| Stage      | **4 complete** (borrowing / `Option<&T>` / lifetimes) — reviewed, 12/12 passing |
| Next       | Stage 5 — `AppError` with `thiserror`, `Result` across the IPC boundary. Not yet written. |
| Blocked on | nothing                                                           |

---

## Decisions log

Newest first. Record the _why_, not just the what.

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
4. **Next:** `AppError` with `thiserror`, and `Result` across the IPC boundary.
5. Parse-don't-validate: `RawCase` → `TryFrom` → `Case`, loading a real TOML case file.
6. `tauri::State` and managed app state — the first stage that touches the Tauri shell.
