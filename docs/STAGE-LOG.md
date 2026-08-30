# STAGE-LOG — one entry per finished stage

Five lines each: what he built, how it went, where he got stuck, what to do differently. Teaching
lessons that generalise go to `MENTOR-NOTES.md` and become rules in `CLAUDE.md`; they are not
repeated here.

Concepts land in `CONCEPTS.md` at review time. That is part of the review.

---

### Stage 1 — `Difficulty` and `Tuning` ✅ 2026-08-21

**Built.** `src/difficulty.rs`: four-variant enum, `Tuning` payload struct, `Difficulty::ALL`
associated constant, `tuning()`. Spec `tests/difficulty.rs`, 7 tests.
**Headline concept.** `E0507 cannot move out of *d which is behind a shared reference` — move
semantics, and why `Difficulty` should be `Copy` while `Tuning` should not.
**Hints reached.** Got to the `impl` skeleton with guidance; needed the array-literal correction
(wrote TS object-literal syntax `Easy: "easy"` inside `[...]`).
**Outcome.** Pass, with a polish pass requested: derives on `Tuning`, `Self` inside the `impl`, doc
comments on public items.

### Stage 2 — `SuspectId` and `FactId` ✅ 2026-08-22

**Built.** `src/ids.rs`: two tuple structs wrapping a private `u32`, `new` / `get`, eight derives,
hand-written `Display` and `From<u32>`. Spec `tests/ids.rs`, 10 tests.
**Headline concept.** The newtype pattern, and derive vs hand-written `impl` — why std ships no
`derive(Display)`.
**Hints reached.** Needed the concepts unpacked in chat rather than through the `<details>` hints.
Wrote all four impl blocks himself.
**Outcome.** Pass, 10/10, clippy and fmt clean. Notable: his first `Display` compiled and printed
`"3 suspect #"` — the compiler had nothing to say, the test caught it. Good landing for
shapes-vs-behaviour.

### Stage 3 — `Fact`, `Suspect` and `Case` ✅ 2026-08-23

**Built.** `src/case.rs`: `Suspect`, `Fact` with a `HashSet<SuspectId>` `known_by` and
`reveal_to` / `is_known_by`, `Case` with private collections and `facts_known_by`. Spec
`tests/case.rs`, 9 tests.
**Headline concept.** `String` vs `&str` — structs own, parameters borrow, the constructor converts.
**Hints reached.** Needed the full signature skeleton handed over after failing to derive signatures
from the test's call sites — the correction that made skeletons standard from Stage 4 on. Wrote every
body himself and reached for the iterator chain in `facts_known_by` unprompted.
**Outcome.** Pass, 9/9, clippy and fmt clean. Doc comments now say what the type *means*; the polish
note open since Stage 1 closed here. Notable: with `!` missing from the ground-truth condition **both**
visibility tests failed in opposite directions — every case inverted rather than some cases wrong is
the tell for a missing negation.

### Stage 4 — borrowing, `Option<&T>` and lifetimes ✅ 2026-08-24

**Built.** In `src/case.rs`: `suspect`, `fact_mut`, `suspect_facts -> impl Iterator<Item = &Fact>`,
and free `longer_statement<'a>`. Spec `tests/borrowing.rs`, 12 tests.
**Headline concept.** Lifetimes as a named region relating inputs to outputs, not a duration.
**Rules that worked.** No `clone()` — cloning out of a borrow error is the habit the stage exists to
prevent. Three errors reproduced before issuing: `E0502`, `E0106`, `E0373`.
**Hints reached.** None. Hit `E0373` and read it rather than asking.
**Outcome.** Pass on the second round. First submission 7/12 with two bugs, and he had not run the
suite before saying he was done — told directly to run it before declaring. Second submission 12/12,
clean.

### Stage 5 — `AppError`, `thiserror` and `Result` ✅ 2026-08-25

**Built.** New `src/error.rs`: `AppError` with seven named-field variants, `pub type AppResult<T>`,
`Serialize` on both id newtypes; plus `require_suspect`, `require_fact_mut`, `reveal` on `Case`.
Spec `tests/errors.rs`, 14 tests.
**Headline concept.** `Result` as `Option` with a reason attached, and `?` as early return.
**Where he got stuck.** Two places, both mentor defects — `self.require_suspect(to)?;` as a line that
stores nothing, and where `#[serde(tag = ...)]` physically goes. Full account in `MENTOR-NOTES.md`.
**Outcome.** Pass, first submission. 14/14 and all 52 tests across five files still green. Wrote every
body himself; solved the quoted-slug message with a raw string rather than the `{slug:?}` the hint
suggested.
**Open polish.** The three new `Case` methods and both items in `error.rs` still have no doc comments,
while everything else public in `case.rs` does. Raise once, as tidying, not as a rule.

### Stage 6 — case files — **withdrawn and re-cut 2026-08-29**

Issued 2026-08-27 as a single 657-line brief teaching ten new concepts. Withdrawn 2026-08-29 and
re-cut into four stages, each inside the concept budget and each ending green:

| | Stage | Spec | New concepts |
|---|---|---|---|
| **6a** ✅ | the file's own vocabulary | `tests/case_raw.rs`, 6 tests | `Deserialize`, `#[serde(default)]`, raw vocabulary |
| **6b** ✅ | the one road | `tests/case_convert.rs`, 6 tests | `TryFrom`, associated types, `*` deref |
| **6c** | the four checks | `tests/case_checks.rs`, 8 tests | validation at the boundary, order as behaviour |
| **6d** | the front door | `tests/case_parse.rs`, 4 tests | `.map_err`, why `?` cannot convert |

The old 16-test `tests/case_file.rs` is in `archive/`. **He had already written 6a's implementation**
and stubbed 6b and 6d before the re-cut, so 6a needs only a test run and a review. Reasoning in
`DECISIONS.md`, 2026-08-29.

### Stage 6b — `TryFrom` and the one road ✅ 2026-08-29

**Built.** The `try_from` body: two loops, an inner loop revealing each `known_by` id, the
ground-truth flag carried across. 6/6.
**Headline concept.** `TryFrom` and its associated type. He wrote `for x in &raw.suspects` with the
`&` unprompted, so **`E0382` never fired** — the brief's rule 4 assumed it would. Move that error to
a stage where it is unavoidable, or drop it.
**Where he got stuck.** Two places, both one step each. (1) The empty loop body — he had the `for`
line and no idea what went in it. (2) `SuspectId::new(raw_known_by)` on a `&u32`: *"this is bringing
me some headache."* `*` had never been taught — `&` was introduced in Stage 4 and its other half was
not. **Add the dereference to the Stage 4 material, or teach it the first time a borrowed number is
passed by value.**
**Mentor defect.** The checkpoint table said 5/6 after the `known_by` step; the real count is 3/6,
because the three remaining tests each check `known_by` *and* the flag and flip together. Table
corrected. This is exactly the failure the "measure, do not guess" rule exists to prevent — it was
guessed.
**One note given.** `&raw_suspect.name.to_string()` in his first draft — a needless second `String`.
Fixed after one line of explanation.

### Stage 6a — `RawCase` and `Deserialize` ✅ 2026-08-29

**Built.** `src/case_file.rs`: `RawCase`, `RawSuspect`, `RawFact`, all fields `pub`,
`#[derive(Debug, Deserialize)]`, `#[serde(default)]` on `known_by` and `is_ground_truth_only`.
**Outcome.** Pass, first submission, 6/6. Written ahead of the brief, unprompted and correct.
**One note given.** `#[serde(default)]` also sits on `RawCase::facts` while `suspects` is required.
Both are the same kind of thing and a case with neither is a bug, so both should be required. Cosmetic
— it changes no test, because 6c's "every suspect has something to say" catches an empty file anyway.
**Warnings are expected until 6d.** `unused_mut` from his in-progress `try_from`, and two
`unused_variables` from `parse_case`'s `todo!()`. Told him not to run `clippy -D warnings` until 6d
closes the stage — it cannot pass mid-flight, and chasing it would mean deleting the stubs.
