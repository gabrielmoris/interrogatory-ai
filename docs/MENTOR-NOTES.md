# MENTOR-NOTES — the corrections, verbatim

Not mandatory reading. Open it before changing a rule in `CLAUDE.md`, or when one looks arbitrary.
Every entry is one real failure: what he said, what caused it, the rule it produced. The reasoning
has been cut — the rules carry it now.

---

### 2026-09-14 — He set the budget himself, and it is one *(correction 11)*

> "how can I know which shape ahs `CaseIntro` if I dont know what is it for???"

> "I want you to split the tasks in small chunks that I can do EASY introducing MAXIUMUM 1 new topic
> and reviewing something, but not more than 2 topics. Split the tasks and explain then much more
> basically so I, a rust NEWBIE can I uncerstand."

Both in the same sitting as correction 10, on the already re-cut 9a — the one that was down to two
concepts and 73 prose lines. So cutting the stage in half was not enough; **three was never the right
number and two is not either.** → Rule 1 is now **one new thing, plus at most one recalled thing.**

The first quote is the more useful one. `CaseIntro` was a name I invented, and the brief explained
what it *was* — a second, smaller type — without ever showing what it was *for*. He could not derive
the fields because he had never seen the screen. What unblocked it in one message: a sketch of the
briefing screen, and a pointer to the test that already held the finished value as JSON. → Rule 4
gains **show the artifact before naming the type**, and the standing habit worth teaching him with
it: *when you do not know the shape, read the test.* That is the premise of this whole format and it
had never been said out loud.

Note what did not work, twice running: cutting scope while leaving the explanation abstract. 9a's
§1 was shorter than Stage 8's and still opened on "a hatch between two worlds" and "being sendable is
a capability a type is given". Nothing concrete until §1d, and by then he had stopped.

→ Stage 9 re-cut a second time into 9a (done) + three stages of one topic each; `Mutex` moved out of
Phase 1 §1.5 to Stage 10, where there is finally something that changes — `DECISIONS.md`, 2026-09-14.

### 2026-09-14 — Two concepts on paper, ten new things in the document *(correction 10)*

> "I am a bit lsot. Can you please ecxplain less rust advanced and more LERNING RUST??????"

Stage 9a, sent the day before. He had already done steps 1 and 2 correctly and stopped at step 3.

The brief passed every check. Two concepts against the ledger, 89 prose lines, an `Assumes:` line
that held, checkpoints measured against a reference build. It was still too much, because the stage
was three files and five steps: an accessor, a new module, two new types, a conversion, a command,
and two lines of handler wiring. **The ledger counts concepts; it does not count how many unfamiliar
things are in the document.** Ten, here.

Worse, the rule added the day before made it heavier. The printed-line test says every call he has
never typed appears in §1 printed whole — so a stage that pulls in `Path::new`, `#[tauri::command]`,
`generate_handler!` and a `From` that walks a collection forces §1 to print, between them, the
finished answer. → **The printed-line test is also a sizing test.** If §1 has to print that much to
satisfy it, the stage is too big; split it, do not print less.

Second failure, and the one that stopped him exactly where it stopped him. Step 3 read: *"fill in the
fields of the two screen types. For each candidate field ask: does the briefing screen draw it? If
not, it does not belong."* That is a thing to weigh up, not a thing to do. What unblocked him was
five seconds of plain English: the small box holds one person, their number and their name; the big
box holds the title, the opening paragraph, and a list of small boxes. → **Rule 3 slot 1 governs
§3.1 too.** A task step says what to do. Criteria, if they are worth stating, go in §4.

→ 9a re-cut mid-stage to the four steps he was already on, Tauri moved whole to 9b. One concept,
73 prose lines. Steps 1–4 kept their numbers so nothing he had written was invalidated.

### 2026-09-13 — He set the explanation ladder himself *(correction 9)*

> "1. New concept. you explain with typescript comparations if there are some and clearly explains
> how it works
> 2. Concept that we already touched: you recall it and explains where we saw it before
> 3. Concept that we saw before many times, just mention it and tell that we saw it in 2 or 3
> examples before
> 4. Concept that I keep failing after recalling it? give the examples back and explain it again"

Three of the four were already the rule. The fourth was **forbidden** by it: `CLAUDE.md` Rule 1 said
a concept in the ledger gets one refresher line "never a second explanation", and `CONCEPTS.md` said
"never re-explain it from scratch". So a concept he had met and not absorbed was guaranteed a
refresher line — the treatment that had already failed on it. `.map_err` in Stage 8 is the case:
`defined` since 6d, and he wrote `Err(..)` inside the closure.

The ledger already had three statuses and no behaviour keyed to them; the statuses were decoration.

→ `CONCEPTS.md`'s status table now carries the behaviour for all four tiers and gains **`shaky`**,
set at review time from what actually went wrong. Rule 1 defers to it instead of stating a flat
never-re-explain rule.

### 2026-09-13 — Prose is not teaching *(correction 8)*

> "I have never used rust to read a file, your explanation assumes i know because you never explain
> me anything, just give me a boilerplate and expects me to find out from infuse science how to do
> it."

> "no, the two more lines are not mine because I suspect they have steps I have never touched."

> "ok, I am L O S T"

Stage 8, step 4, six exchanges. He was right all three times. The brief's §1b introduced the read as
a shape with its middle commented out — `ErrorKind::NotFound => /* one error */` — so the only thing
it taught was that a shape exists. `.display()` appeared in §1a as a sentence of prose and never once
as a line of code. `fs::read_to_string` was named and never shown. Then §3.1 step 4 asked for all
three in one statement, plus a closure that must *hand back* a value rather than return one.

The `Assumes:` line did not catch it, because it lists concepts from the ledger and these were not
concepts — they were calls. **A call he has never typed costs as much as a concept, and was being
counted as free.**

Made worse mid-stage twice. The unblocking reply told him to *paste a line I had given him*, which is
copying, and left him unable to place the two lines that followed. And hint 4 — the rung that exists
to end exactly this — contained step 5's `match`, so the ladder's last rung answered a question he
had not reached.

→ Rule 1 gains the **printed-line test**; Rule 2 gains the rung-4 constraint; Rule 3 gains the
no-paste bullet.

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
- **Teaching by shape with the body left out.** Stage 8's `/* one error */` taught nothing but the
  existence of a shape. Whatever the stage teaches must appear complete, in code, somewhere in §1.
- **Prerequisites outside the ledger.** 6b's real blocker was `*`, needed because Stage 4 taught `&`
  without its other half. → the `Assumes:` line, Rule 1.
- **A task step that names a criterion instead of a job.** "Ask whether the screen draws it" stopped
  him for a day in 9a. Say what to write.
