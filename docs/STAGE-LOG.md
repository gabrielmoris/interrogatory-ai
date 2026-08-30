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

Issued 2026-08-27 as a single 657-line brief teaching ten new concepts. Withdrawn and re-cut into
four stages, each inside the concept budget and each ending green. The old 16-test
`tests/case_file.rs` is in `archive/`. Reasoning in `DECISIONS.md`, 2026-08-29.

### Stage 6a — `RawCase` and `Deserialize` ✅ 2026-08-29

**Built.** `src/case_file.rs`: `RawCase`, `RawSuspect`, `RawFact`, `#[derive(Debug, Deserialize)]`,
`#[serde(default)]` on `known_by` and `is_ground_truth_only`. Spec `tests/case_raw.rs`, 6 tests.
**Headline concept.** `Deserialize` — text in, Rust value out; raw types speak the file's vocabulary.
**Outcome.** Pass, first submission, 6/6. Written ahead of the brief, unprompted and correct.
**One note given.** `#[serde(default)]` also sits on `RawCase::facts` while `suspects` is required;
both are the same kind of thing and both should be required. Cosmetic — no test changes, because
6c's "every suspect has something to say" catches an empty file anyway. **Still open.**

### Stage 6b — `TryFrom` and the one road ✅ 2026-08-29

**Built.** The `try_from` body: two loops, an inner loop revealing each `known_by` id, the
ground-truth flag carried across. Spec `tests/case_convert.rs`, 6 tests. 6/6.
**Headline concept.** `TryFrom` and its associated type.
**Where he got stuck.** Two places, one step each: the empty loop body, and
`SuspectId::new(raw_known_by)` on a `&u32` — *"this is bringing me some headache."* `*` had never
been taught; `&` was introduced in Stage 4 and its other half was not. **Produced the `Assumes:`
line in Rule 1.**
**Do differently.** He wrote `for x in &raw.suspects` unprompted, so the brief's planned `E0382`
never fired. Move that error to a stage where it is unavoidable, or drop it.

### Stage 6c — the four checks ✅ 2026-08-29

**Built.** All four checks inside `try_from`: duplicate suspect, duplicate fact, `require_suspect?`
on every `known_by` entry, and a third loop asking `suspect_facts(id).next().is_none()`. Spec
`tests/case_checks.rs`, 8 tests. 8/8.
**Headline concept.** Validation at the boundary — one place, and after it a `Case` is proof.
**One note given, taken.** His duplicate-fact check first used `require_fact_mut(..).is_ok()`,
building and discarding an `AppError` per non-duplicate fact. Swapped to `fact_mut(..).is_some()`
after one explanation: **`Option` when absence is a normal answer, `Result` when it is a failure.**
Check 3 is the same rule pointing the other way, which made a clean pair.
**Do differently.** He asked twice *which function* the work went in. Name `file.rs :: function()`,
never "inside your suspects loop". Two mentor defects (guessed checkpoints, correction seven) are in
`MENTOR-NOTES.md`.

### Stage 6d — the front door ✅ 2026-08-30

**Built.** `parse_case`: `toml::from_str` with `.map_err` into `AppError::Parse { path, message }`,
then `raw.try_into()` as the last line. Spec `tests/case_parse.rs`, 4 tests. 4/4.
**Headline concept.** `?` calls `From::from` on the error on its way out — invisible in Stage 5
because both sides were `AppError`, and `E0277` the moment they differ.
**Outcome.** Pass, first submission. **76 tests across eight files green, `fmt` and
`clippy -D warnings` clean. Stage 6 and Phase 1 §1.3 closed.** He wrote `.map_err` in the right
place — wrapping only the parser's failure, not the whole function, which is the mistake hint 4
existed to catch.
**Do differently.** Nothing on the teaching side. One cosmetic note carried forward: the `parse_case`
doc comment reads "but it it fails", and `RawCase`'s is "Raw case" where the other public items say
what the type *means*.
