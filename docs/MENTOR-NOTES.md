# MENTOR-NOTES — why the rules in CLAUDE.md exist

Not mandatory reading. Open it when a rule in `CLAUDE.md` looks arbitrary, or before changing one.
Every entry is a real failure with the correction it produced.

---

## 2026-08-29 — The briefs grew after every request to slow down

**The evidence.** Brief lengths and the count of new concepts each one introduced, taken from the
stage log's own "concepts targeted" lists:

| Stage | Lines | New concepts |
|---|---|---|
| 1 | 148 | 10 |
| 2 | 295 | 10 |
| 3 | 442 | 9 |
| 4 | 472 | 9 |
| 5 | 610 | 13 |
| 6 | 657 | 10 |

He asked for less on 2026-08-22 and again on 2026-08-25. `CLAUDE.md` said "keep the whole thing near
300 lines." Stage 6 was written two days after the second correction and came out at 657 lines, and
the stage log recorded the correction as *applied*.

**The diagnosis.** Every rule in `CLAUDE.md` was about **wording** — short paragraphs, concrete
example first, no jargon, TypeScript first. None was about **volume**. So the response to "too
technical" was to rewrite the prose more gently and then add more of it. Stage 6 as issued taught
parse-don't-validate, `Deserialize`, `#[serde(default)]`, `TryFrom`, associated types, `try_into`,
`.map_err`, `?`-calls-`From`, and `E0382`-in-a-`for`-loop: three stages of material in one.

**The corrections.** Rule 1 (hard concept budget of three, split the stage rather than compress the
writing), Rule 2 (fixed template, 200-line ceiling), and `docs/CONCEPTS.md`, which makes the
refresher-vs-explain decision mechanical instead of a judgement call.

**Also found the same day**, and fixed: `docs/ROADMAP.md` still specified the `crates/core` workspace
split as the plan of record — 11 references, including the Phase 1 exit criterion
(`cargo test -p interrogator-core`) and "Next action #2" — eight days after the split was rejected.
`task.md` at the repo root was Stage 1's chat message, frozen, asserting the workspace existed and
telling him to run `npm run tauri dev` against a `package-lock.json` that had been deleted. A fresh
session reading either one would have acted on a rejected architecture.

Lesson generalised: **a decision is not recorded until every document that states the opposite has
been amended.** Grep for the rejected thing before closing a decision.

---

## 2026-08-29 — Correction seven: reasons are not instructions

Mid-6c, on the last of the four checks:

> "You never explain in plain language WHAT I have to do? The logic? You mix rust terms with machine
> logic and expect me to understand everything. I am a person!"

The reply that caused it had, in order: why the check matters for the game, the trap that makes a
naive version wrong, which method answers the question, that iterators have no `.len()`, a line of
`.next().is_none()`, a note on laziness, where the loop goes, and a parcel-depot code block. Eight
items. **The instruction — "go through the suspects again and ask each one whether they can say
anything" — was never stated.** Every sentence was about *why* or *with what*, none about *what*.

The reply that worked, immediately after, was six lines: what to do, in ordinary words, as two
bullets for the yes and no branches, then one sentence naming the function he already owned, then a
handback. He wrote it correctly on the first try.

Produced the first bullet of Rule 3b: **lead with the plain-language steps, always; reasons and Rust
names come after.** Note this is a *different* failure from corrections four and five, which were
about volume. This reply was not especially long. It was correctly ordered for a reference document
and backwards for a person who is stuck.

Also fixed the same day: the checkpoint table in the 6c brief labelled its rows "check 4 (nothing to
say)", and he read the table as if it were the spec. Rows now name the section that explains them,
with a line saying the label is a reminder and not the instruction.

---

## 2026-08-27 — Corrections four and five: now it is the chat replies

Mid-Stage 6, two in the same session:

> "Your explanations are too detailed and saying too much information at once, making the learning
> process too difficult. I asked you to be pedagogic already the last 3 stages."

> "I am blindly trying to follow you up but I am stopping having interest because it is getting too
> deep and introducing many things at the same time some of them even without explanation. You wrote
> it in CLAUDE.md but seems that you forget all time."

His count was right: 2026-08-22 (Stage 2), 2026-08-25 twice (Stage 5 chat, then the Stage 5 brief),
and twice here. The Stage 6 brief had been written to the rules, so this one was **not** about the
documents — the old habit came straight back in the mid-stage help replies.

The reply that triggered the fourth: he asked "what have I done wrong?" while stuck on one compile
error, and got four headed sections — the error, the mistake, "two more things the compiler cannot
tell you yet", and "ignore these warnings". Every part true; only the first was asked for.

The reply that triggered the fifth was *shorter* and worse: a "two-line step" that quietly contained
three things he had never been taught — an explicit type annotation on a `let`, `?` against a foreign
error type, and `try_into()`. **Short is not the same as small.**

Produced Rule 3b. Note the second quote is the one that matters most in this file: he named losing
interest, not confusion. A correct reply that costs him motivation is a bad reply.

---

## 2026-08-25 — Architecture questions are not homework

Stage 4's brief was written without re-reading `ROADMAP.md` §3.2, which had already assigned the
fact-visibility rule a single owner. So `suspect_facts` was issued *alongside* Stage 3's
`facts_known_by`, creating two implementations of one rule. They drifted inside a single stage: his
first submission dropped `!is_ground_truth_only` from the new copy, which would have fed the case
solution into the LLM's context window.

That bug was then presented to him as evidence for a design question **he** should resolve, and he
was asked for a position twice. His answer:

> "you made a mistake, it is not my responsibility to solve your problem... we are not doing a
> product, you are teaching and I am learning."

Correct on both counts. The duplication was the mentor's and the decision was the mentor's to make.
Produced Rule 4, and the standing instruction to check the roadmap section a stage touches before
writing the brief.

Footnote: do not keep citing the dropped filter as evidence for anything. From his side it was a
one-line bug in a function he wrote correctly the second time.

---

## 2026-08-25 — "Be more pedagogical" meant the briefs, not the chat

Mid-Stage 5:

> "You keep using language too technical and too into rust... this is the FIRST time I code in rust.
> Be more pedagogical."

He clarified he meant the `docs/stages/*.md` files, not only the chat replies. The first Stage 5
draft was an *argued* document: it defended its own architecture decisions at him (four compiler
errors as evidence against `#[from] std::io::Error`), ran essay-length paragraphs, and used words he
had never been taught — extern prelude, supertrait, orthogonal, desugars, internally tagged
representation.

The rewrite led every idea with a concrete example, cut paragraphs to two or three sentences, moved
the architecture reasoning to a three-line pointer, and cut the concept count from eleven to six.
Still 610 lines. See the 2026-08-29 entry for why that was not enough.

Produced: the "no mentor-facing reasoning in the brief" half of Rule 2.

---

## 2026-08-25 — Two places he got properly stuck in Stage 5

Both were mentor defects.

**1. `self.require_suspect(to)?;` — a line with `?` that stores nothing.** Four exchanges, ending in
*"No idea dude"* and *"either I am dumb or you are not being clear"*. The failed wording was "a line
that keeps nothing", which reads as "a line that does nothing".

What worked: state that `?` does **two** jobs — (1) stop the function and return the error, (2) hand
back the value — and that here job 1 is the point and job 2 is leftover. Then trace
`reveal(FactId(1), SuspectId(99))` line by line with and without the check. Then show his own method
with a commented gap where the line goes.

Interim damage worth reusing as a lesson: he deleted the `require_suspect` call entirely to make
`E0502` go away. It compiled and silently dropped the validation. **Making the compiler happy by
deleting the code it complained about** is the most common way a borrow error becomes a behaviour
bug.

**2. Where `#[serde(tag = ...)]` physically goes.** The brief said "add one line above the enum" and
never showed it in place, so he could not place it. Attribute stacking — two `#[…]` lines above one
item, order irrelevant — had never been taught; he had only ever seen one attribute at a time.
Produced the "show the surrounding lines of the real file" rule.

---

## 2026-08-23 — Reading signatures off the test was a step too far

Stage 3 asked him to derive the function signatures from the test's call sites. It cost several
rounds and he guessed wrong three times (`Fact::new` taking every field, `is_known_by` with no
`self`, `statement` missing entirely).

Correction, in force from Stage 4 on: **issue the signature skeleton with the brief, bodies elided.**
Shapes are scaffolding; bodies are the exercise.

Two smaller things from the same review:

- **Teach `todo!()` before it is needed.** He stubbed with `-> Self {}` and got a wall of `E0308`.
  `todo!()` type-checks as anything, so a stubbed file links and he gets a red suite to drive instead
  of a build error. Exception worth stating when it applies: a function returning `impl Iterator`
  stubbed with `todo!()` infers `()` and fails `E0277` — stub that one with `std::iter::empty()`.
- **Jargon, not just depth.** "One boolean in your filter closure" did not parse, though he had
  written the closure himself. Name Rust vocabulary against its TypeScript equivalent on first use.

---

## 2026-08-22 — "Like if I were writing Rust for years"

Mid-Stage 2:

> "the way you explain is like if I were writing Rust for years... go slowly."

The written brief and its progressive hints were pitched too high, and dumping all remaining compiler
errors at once made it worse. What worked instead: one concept per reply, built from zero — what a
trait is → what `derive` generates → ownership and moves → `impl Trait for Type` → `From` — each
ending with a single command to run and a checkpoint to come back to.

Produced the "one concept per reply, finished" and "calibrate to first month of Rust" items in
Rule 3.

---

## Things he does well, so they are not mistaken for gaps

- Pushes back when he disagrees, and is frequently right. Engage with the argument.
- Reached for the iterator chain in `facts_known_by` unprompted rather than a loop (Stage 3).
- Hit `E0373` in Stage 4 and read it rather than asking.
- Solved `CaseNotFound`'s quoted-slug message with a raw string,
  `#[error(r#"no case file named "{slug}" was found"#)]`, rather than the `{slug:?}` the hint
  suggested — a different and defensible answer (Stage 5).
- Reused his Stage 4 lookups inside the Stage 5 `require_*` methods rather than repeating the `find`,
  after being told once.
