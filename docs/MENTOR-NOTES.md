# MENTOR-NOTES — the corrections, verbatim

Not mandatory reading. Open it before changing a rule in `CLAUDE.md`, or when one looks arbitrary.
Every entry is one real failure: what he said, what caused it, the rule it produced. The reasoning
has been cut — the rules carry it now.

---

### 2026-08-30 — The line ceiling was measuring the wrong thing

Prose lines per brief, code blocks and `<details>` tags excluded:

| Brief | 1 | 2 | 3 | 4 | 5 | 6a | 6b | 6c | 6d |
|---|---|---|---|---|---|---|---|---|---|
| Prose | 57 | 137 | 195 | 205 | 235 | 75 | 85 | 87 | 75 |
| Code | 30 | 71 | 131 | 140 | 202 | 59 | 48 | 31 | 34 |
| Landed | ✅ | ✗ | ✗ | ✗ | ✗ | ✅ | ✅ | ✅ | ✅ |

Everything at or under ~90 prose lines landed; everything over 130 produced a correction. **Code
volume does not separate them** — 6a carries more code than 6c and both were fine — and he has twice
asked for *more* code in place ("show me the surrounding lines", the placement failure of
2026-08-25). A total-line ceiling therefore penalises exactly the thing that helps.

→ Rule 2 now caps **prose at 90 lines** and does not count code. Caught immediately: the Stage 7
brief came out at 221 total lines and 89 prose. Under the old rule it would have been cut by
removing scaffolding and hints.

### 2026-08-29 — Reasons are not instructions *(correction 7)*

> "You never explain in plain language WHAT I have to do? The logic? You mix rust terms with machine
> logic and expect me to understand everything. I am a person!"

Mid-6c, on the fourth check. The reply had, in order: why the check matters, the trap that makes a
naive version wrong, the method name, that iterators have no `.len()`, a line of `.next().is_none()`,
a note on laziness, where the loop goes, a code block. Eight items. **The instruction — "go through
the suspects again and ask each one whether they can say anything" — was never stated.**

The reply that worked was six lines: what to do in ordinary words, the function name, a handback. He
wrote it correctly first try. → **Rule 3, slot 1.** Note this is not the volume failure of
corrections 4–5; that reply was not long. It was ordered for a reference document, not for a person
who is stuck.

### 2026-08-29 — Briefs grew after every request to slow down *(the volume failure)*

| Stage | 1 | 2 | 3 | 4 | 5 | 6 |
|---|---|---|---|---|---|---|
| Lines | 148 | 295 | 442 | 472 | 610 | 657 |
| New concepts | 10 | 10 | 9 | 9 | 13 | 10 |

He asked for less on 2026-08-22 and again on 2026-08-25. Stage 6 was written two days after the
second correction and came out at 657 lines. **Every rule then in force governed wording — short
paragraphs, concrete example first, no jargon. None governed volume.** So the response to "too
technical" was to rewrite the prose more gently and add more of it.

→ **Rule 1** (hard budget of three, split rather than compress), **Rule 2** (fixed template and a
line ceiling), and `CONCEPTS.md`, which makes refresh-vs-explain mechanical instead of a judgement.

Same day: `ROADMAP.md` still specified the rejected `crates/core` split as the plan of record —
11 references, eight days after the rejection — and `task.md` told him to run
`cargo test -p interrogator-core`. → **A decision is not recorded until every document stating the
opposite has been amended. Grep before closing.**

### 2026-08-27 — Now it is the chat replies *(corrections 4 and 5)*

> "Your explanations are too detailed and saying too much information at once, making the learning
> process too difficult. I asked you to be pedagogic already the last 3 stages."

> "I am blindly trying to follow you up but I am stopping having interest because it is getting too
> deep and introducing many things at the same time some of them even without explanation. You wrote
> it in CLAUDE.md but seems that you forget all time."

His count was right: 2026-08-22, twice on 2026-08-25, twice here. The Stage 6 brief *had* been
written to the rules — the old habit came straight back in the mid-stage help replies.

Correction 4: he asked "what have I done wrong?" about one compile error and got four headed
sections. Correction 5 came from a *shorter* reply that quietly contained three untaught things — an
explicit `let` annotation, `?` on a foreign error, and `try_into()`. **Short is not the same as
small.**

→ **Rule 3.** The second quote is the one that matters most in this file: he named losing interest,
not confusion.

### 2026-08-25 — Architecture questions are not homework

> "you made a mistake, it is not my responsibility to solve your problem... we are not doing a
> product, you are teaching and I am learning."

Stage 4's brief was written without re-reading the roadmap section that had already assigned the
fact-visibility rule an owner, so a second implementation was issued alongside the first. They
drifted inside one stage. That bug was then handed back to him as evidence for a design question
*he* was asked to resolve, twice. → **Rule 5**, and the standing instruction to check the roadmap
section a stage touches before writing the brief.

Footnote: do not keep citing the dropped filter as evidence for anything. From his side it was a
one-line bug in a function he wrote correctly the second time.

### 2026-08-25 — "Be more pedagogical" meant the briefs, not just the chat

> "You keep using language too technical and too into rust... this is the FIRST time I code in rust.
> Be more pedagogical."

The first Stage 5 draft was an *argued* document: it defended its own architecture decisions at him,
ran essay-length paragraphs, and used words he had never met — extern prelude, supertrait,
orthogonal, desugars, internally tagged representation. → the "no mentor-facing reasoning in the
brief" half of Rule 2, and the banned-vocabulary list in `CONCEPTS.md`.

### 2026-08-25 — The two places he got properly stuck in Stage 5

Both mentor defects. **(1)** `self.require_suspect(to)?;` — a line with `?` that stores nothing. Four
exchanges, ending in *"No idea dude"* and *"either I am dumb or you are not being clear"*. What
worked: `?` does **two** jobs, and here only job one matters. Interim damage worth reusing: he
deleted the call to make `E0502` go away, and it compiled with the validation silently gone.
**Making the compiler happy by deleting the code it complained about is the most common way a borrow
error becomes a behaviour bug.** **(2)** Where `#[serde(tag = ...)]` physically goes — "add one line
above the enum" was not placeable. → the "show the surrounding lines of the real file" rule.

### 2026-08-23 — Reading signatures off the test was a step too far

Stage 3 asked him to derive signatures from the test's call sites; he guessed wrong three times.
→ **issue the signature skeleton, bodies elided.** Also: **teach `todo!()` before it is needed** (he
stubbed with `-> Self {}` and got a wall of `E0308`; the exception is `impl Iterator`, which needs
`std::iter::empty()`), and **name Rust vocabulary against its TypeScript equivalent on first use** —
"one boolean in your filter closure" did not parse, though he had written the closure.

### 2026-08-22 — "Like if I were writing Rust for years"

> "the way you explain is like if I were writing Rust for years... go slowly."

Dumping all remaining compiler errors at once made it worse. What worked: one concept per reply,
built from zero — what a trait is → what `derive` generates → moves → `impl Trait for Type` → `From`
— each ending with one command to run. → **Rule 4**, first two items.

---

## Defects to stop repeating

- **Guessed checkpoint counts.** 6b said 5/6, real 3/6. 6c said 2/8, real 3/8. Two stages running,
  with the "measure, do not guess" rule already in force. Now mechanical — Rule 2.
- **Not naming the function.** In 6c he asked twice whether the work went in `try_from` or
  `parse_case`. "Inside your suspects loop" is not an address.
- **Prerequisites outside the ledger.** 6b's real blocker was `*`, needed because Stage 4 taught `&`
  without its other half. → the `Assumes:` line, Rule 1.
