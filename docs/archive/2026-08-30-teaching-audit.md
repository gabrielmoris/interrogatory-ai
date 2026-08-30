# AUDIT 2026-08-30 — doc set, teaching process, curriculum

Working document. Execute it, then archive it. It is not part of the resume path.

Measured before: `CLAUDE.md` 172 · `CONCEPTS` 178 · `MENTOR-NOTES` 218 · `ROADMAP` 289 ·
`STAGE-LOG` 135 · `DECISIONS` 107 · `PROGRESS` 61 = **1160 lines**, of which the stated resume
order (`CLAUDE` → `PROGRESS` → `CONCEPTS` → brief) reads **~600 before a word of teaching**.
Target: **~520 total, ~230 on the resume path.**

---

## 1. File audit

### Verdicts

| File | Verdict | Target |
|---|---|---|
| `CLAUDE.md` | **Keep, cut ~35%.** Rules 3 and 3b overlap; the project-instructions block already injects the profile and phase list every session — stop repeating it. | 110 |
| `docs/CONCEPTS.md` | **Keep. Highest-value file in the repo.** Split out one section (below). | 150 |
| `docs/PROGRESS.md` | **Keep.** Delete one section. | 45 |
| `docs/STAGE-LOG.md` | **Keep, enforce its own 5-line rule.** Entries have drifted to 12 and are out of order (6c, 6b, 6a). | 90 |
| `docs/DECISIONS.md` | **Keep as is.** Only file with no redundancy. | 107 |
| `docs/MENTOR-NOTES.md` | **Cut to the quotes.** Every rule it justifies is already in `CLAUDE.md`; the prose reconstructions are read once and never again. | 80 |
| `docs/ROADMAP.md` | **Cut ~60%.** Largest single source of stale and duplicated content. | 110 |
| `docs/adr/ADR-0001` | **Keep, untouched.** Correctly gated behind "inference work only". | 166 |
| `docs/archive/**` | **Keep.** Working as intended. | — |

### Delete outright

| What | Why |
|---|---|
| `ROADMAP` §0 "Current state of the repo (as audited)" (25 lines) | Describes an unmodified scaffold at 1 commit with CRLF churn. Every line stale. → `archive/` |
| `ROADMAP` Phase 0 checklist (12 lines) | Duplicate of `PROGRESS` "Phase 0 leftovers", and the two copies **disagree** — `.gitattributes` is unchecked here, done there. One owner: `PROGRESS`. |
| `ROADMAP` "Next three actions" (8 lines) | All three struck through and self-declared superseded. |
| `ROADMAP` cross-cutting row `docs/LEARNING-LOG.md` | The file does not exist. `STAGE-LOG` does the job. |
| `PROGRESS` "Where the rest of it lives" (6 lines) | Third copy of the file map. `CLAUDE.md`'s table is the owner. |
| `PROGRESS` "Standing note" (doc comments) | A TODO in a status file. Fold into a `TODO` list or drop — it has ridden along for five stages. |
| `MENTOR-NOTES` "Things he does well" | Learner profile, not a rule justification. Move to `CONCEPTS.md` header or the project instructions. |

### Merge / relocate

| From → To | Why |
|---|---|
| `ROADMAP`'s six inline `(Amended …)` footnotes → one-line pointers to `DECISIONS.md` | Two full statements of one decision is exactly the failure `MENTOR-NOTES` 2026-08-29 diagnoses. The roadmap states the *plan*; `DECISIONS` states the *why*. |
| `ROADMAP` Phase 2–5 detail (§2.2 toolchain, §2.3 threading, §3.4 scoring, §3.6 generator) → one paragraph + exit criterion each; detail moves into the stage brief when reached | It is 120 lines of material he will not touch for months, loaded on every read. |
| `CONCEPTS.md` "Phrasings that have failed" → `CLAUDE.md` Rule 3 | It is a teaching rule, not a concept ledger. It also duplicates `MENTOR-NOTES` entries. |
| `CLAUDE.md` Rule 3 + Rule 3b → one ordered checklist | ~40% overlap: both state "one concept per reply", "short question, short answer", "plain language first". |
| `STAGE-LOG` "mentor defect" notes → `MENTOR-NOTES` only | The guessed-checkpoint defect currently exists in **three** places (`STAGE-LOG` 6b, `STAGE-LOG` 6c, `CLAUDE.md` Rule 2 item 6). |

### Do not touch
`DECISIONS.md`, `ADR-0001`, `archive/`, the 6a–6d briefs. The 6a–6d briefs are the first four that
obey the template; they are the reference shape.

---

## 2. Pedagogical audit

Eight failure modes, each with the evidence and a structural fix. "Structural" means the fix is a
slot in a template or a mechanical check, not an intention.

**a. The rules govern briefs; the failures happen in chat.**
Corrections 4, 5 and 7 were all mid-stage replies. The brief has a fixed template and a hard line
ceiling. The stuck-reply has prose guidance only, so it regresses.
→ **Fix: the stuck-reply gets a template as rigid as the brief's.** §4 below.

**b. "Plain language first" is stated but not enforced by the template.**
Rule 3b says lead with plain language. The template's §4 "The new idea" is a *Rust* section and it
comes before "Your task". In 6c the plain algorithm — "walk the suspects again, ask each one whether
they can say anything" — appears nowhere as its own block; it is inside a Rust section, after a
`.next().is_none()` line. He asked for exactly this and the template does not give it to him.
→ **Fix: a mandatory "The logic, in ordinary words" block, numbered, before any Rust token appears.**

**c. Checkpoint counts are guessed. Caught twice, in consecutive stages.**
6b said 5/6, real 3/6. 6c said 2/8, real 3/8. The rule "measured, not guessed" already existed.
→ **Fix: mechanical. The reference implementation is built in a throwaway crate stub-by-stub and the
brief pastes the literal `cargo test` summary line. A row that was not run is written `unmeasured`,
never a number.**

**d. "Which function does this go in?" — asked twice in one stage.**
"Inside your suspects loop" was not enough; he could not tell `try_from` from `parse_case`.
→ **Fix: every task step is written `file.rs :: function()` — no bare loop references.**

**e. The concept budget counts what is taught, not what is needed.**
6b's real blocker was `*`. It was in the 6b budget, but it was needed because `&` had been taught in
Stage 4 without its other half. The ledger has no notion of *prerequisite*.
→ **Fix: a `Assumes:` line in the brief header listing every concept the task requires with its stage
number. Anything on it that is not in `CONCEPTS.md` becomes a supporting concept, or the stage
splits. Run before writing the brief, not after.**

**f. The hint ladder is two rungs, not four.**
6d hint 3 of 4 is the complete answer with names changed. 6c hint 2 likewise. The gap between "where
it goes" and "here is the line" is the whole ladder.
→ **Fix: four fixed rungs — (1) where it goes, (2) the question the code must ask, (3) the shape with
names blanked, (4) the parcel-depot line in full.**

**g. Nothing is ever recalled cold.**
At 2–4 h/week there are ~6 days between sessions. Concepts move to `used` only if the next stage
happens to need them. Stage 2's `From` has not been touched since Stage 2.
→ **Fix: one "cold call" per stage — a task step that needs a concept from ≥2 stages back, with a
pointer and no refresher line. Costs zero concept budget.**

**h. Stage size and session size are not the same unit and neither is stated.**
Stages are estimated 20–55 min; sessions are 2–4 h. On 2026-08-29 he did three stages in one sitting.
→ **Fix: `PROGRESS` tracks a **session** as 2–3 stages ending on a green suite and a commit. Stage
estimates stay; the session boundary is the commit.**

---

## 3. The brief template (replaces `CLAUDE.md` Rule 2)

Hard ceiling **180 lines**. Fixed order. Nothing else.

```
# Stage NN — <plain-English name, no Rust words>

Test:    src-tauri/tests/<file>.rs — N tests
Run:     cd src-tauri && cargo test --test <file>
Writes:  src-tauri/src/<file>.rs :: <function>()
Assumes: <concept (Stage n)>, <concept (Stage n)>, …      <- checked against CONCEPTS.md
Est.     N min

## 0. What this has to do, in ordinary words              [MAX 6 LINES. NO RUST TOKENS.]
    The gameplay reason, then the algorithm as instructions to a person.

## 1. CONCEPT INTRODUCTION                                 [1 headline + max 2 supporting]
    Per concept, in this order:
      a. Plain English — what it is, no Rust names.
      b. The TypeScript you would write.
      c. What Rust does differently, and the sentence where the analogy breaks.
      d. The shape, in the parcel-depot / weather domain. Never the detective game.

## 2. REFRESHER                                            [one line each, pointer only]
    `?` returns early on an error — Stage 5 §4.
    Never a second explanation. If it needs one, it belongs in §1 and the budget is blown.

## 3. TASKS
    3.1 Step-by-step plain logic — numbered, ordinary words, each step naming
        file.rs :: function(). This is the section he reads when stuck.
    3.2 Scaffolding — full signatures, bodies `todo!()`. Shapes are given; bodies are the exercise.
    3.3 Checkpoints — measured, one row per step, pasted from a real run.
    3.4 Cold call — the one step reaching back ≥2 stages.

## 4. Rules — numbered, ≤6, one line each.

## 5. Hints — exactly 4 <details>, the fixed ladder, parcel-depot domain.
```

Removed from the old template: "architecture arguments", "rejected alternatives", and the
`What you're building, and why` prose block — §0 absorbs it at a sixth of the length.

---

## 4. The stuck-reply template (new — this is where it actually breaks)

Fits on a phone screen. Five slots, in order, nothing else. No headers, no bullet inventory, no
"also worth knowing".

```
1. What to do.        Ordinary words. Two sentences. No Rust name, no method name.
2. Where.             file.rs :: function(), and the line it goes near.
3. The one new thing. Only if the step needs machinery he has not met — named and explained
                      before it is used. If there are two, the step is too big: split it.
4. Run this.          One command.
5. Tell me what it says.
```

Rules attached to it:
- **Reasons are not instructions.** Never open with why.
- **One step, not a diagnosis.** "What did I do wrong?" is answered about the error he asked about.
- **Blind copying ⇒ stop advancing** and re-teach the last thing he copied.
- When the idiomatic form is dense, build it once with tools he already has, then replace it.

---

## 5. Curriculum — 0 to hero

Every stage: 1 headline + ≤2 supporting. Done stages listed for coverage, not for reading.

### Phase 1 — Rust core (done: 1–6c)

| # | Stage | Headline | Supporting | Est |
|---|---|---|---|---|
| ~~1~~ | `Difficulty` / `Tuning` | moves and `Copy` | `match` as expression, `derive` | ✅ |
| ~~2~~ | `SuspectId` / `FactId` | the newtype | `From`, `Display` vs `Debug` | ✅ |
| ~~3~~ | `Fact` / `Suspect` / `Case` | `String` vs `&str` | `Vec`/`HashSet`, `&self` | ✅ |
| ~~4~~ | borrowing | lifetimes as regions | `Option<&T>`, `iter_mut` | ✅ |
| ~~5~~ | `AppError` | `Result` and `?` | `thiserror`, `ok_or` | ✅ |
| ~~6a~~ | `RawCase` | `Deserialize` | `#[serde(default)]` | ✅ |
| ~~6b~~ | the one road | `TryFrom` | associated types, `*` | ✅ |
| ~~6c~~ | the four checks | validation at the boundary | `Option` vs `Result` | ✅ |
| **6d** | the front door | `?` calls `From` on the error | `.map_err` | 30 |
| 7 | `VisibleFact<'a>` | a struct that holds a borrow | `impl<'a>` blocks | 45 |
| 8 | reading a case from disk | the domain/shell boundary | `std::path`, `Io` at the call site | 50 |
| 9a | the first command | the IPC boundary — what crosses it | `#[tauri::command]` | 40 |
| 9b | managed state | interior mutability (`Mutex`) | `.manage()` + `State<'_, T>` | 55 |
| 10 | `Transcript` and `Phase` | a state machine as an enum with data | exhaustive `match`, `Default` | 50 |

**Phase 1 exit:** a case loads from disk, one command returns it to React, illegal phase transitions
do not compile.

### Phase 2 — async and inference (the hard phase — thinnest slices in the project)

| # | Stage | Headline | Supporting | Est |
|---|---|---|---|---|
| 11 | `trait InferenceEngine` + `MockEngine` | trait objects vs generics (`Box<dyn T>` vs `impl T`) | default methods; `Send`/`Sync` **named only** | 55 |
| 12 | the first `async fn` | a future does nothing until polled — **the TS analogy breaks here**, promises are eager | `.await`, async Tauri commands | 45 |
| 13 | where blocking work goes | blocking work must leave the runtime | `spawn` vs `spawn_blocking` | 50 |
| 14 | channels | an `mpsc` pipe moves ownership | dropping the sender closes it | 50 |
| 15 | sharing across threads | `Arc<Mutex<T>>`, and why a std guard cannot cross `.await` | `tokio::sync::Mutex` | 55 |
| 16 | streaming to React | Tauri events with a typed payload | turn ids, discarding late tokens | 45 |
| 17 | cancellation | `select!` and cooperative cancellation | `Drop` and cleanup order | 55 |
| 18 | **toolchain session** — `llama-cpp-2` on Windows/CUDA | **zero new concepts. Environment only.** | — | 2 h |
| 19 | owning the model | one thread owns the FFI handle | `unsafe` boundaries, `Drop` on a handle | 60 |

**Deliberate failure, scheduled:** do 13 wrong once, watch the UI freeze, then fix it. It is the
cheapest lesson in the project and it only works if it is felt.

**Phase 2 exit:** real model, token stream, cancellable, UI never blocks, `MockEngine` still green.

### Phase 3 — game engine

| # | Stage | Headline | Supporting | Est |
|---|---|---|---|---|
| 20 | `build_prompt` | prompts are code — snapshot tests (`insta`) | `fmt::Write`, `&[VisibleFact]` as the gate | 55 |
| 21 | tier-1 scoring | pure functions over ids, reproducible | `HashMap`, precision/recall | 55 |
| 22 | the generator skeleton | determinism from a seed (`StdRng`) | `is_solvable`, one owner | 60 |
| 23 | grammar-constrained output | constraining a model at the decode level | GBNF, bounded retries | 55 |

### Phases 4–5 — his home turf
Typed IPC via `ts-rs`/`specta`, four screens, then 3D. **No Rust teaching budget.** Keep it tight;
this is the phase most likely to expand to fill the space.

---

## 6. Execution order

1. Archive `ROADMAP` §0 + Phase 0 checklist; delete the superseded sections. *(15 min)*
2. Strip `ROADMAP`'s six amendment footnotes to pointers. *(15 min)*
3. Cut `MENTOR-NOTES` to quote + correction, one entry each. *(20 min)*
4. Merge `CLAUDE.md` Rules 3/3b; paste the two templates from §3 and §4 over Rule 2. *(20 min)*
5. Move the failed-phrasings table out of `CONCEPTS.md`. *(5 min)*
6. Delete the two duplicated `PROGRESS` sections. *(5 min)*
7. Re-cut the stage queue in `PROGRESS` to §5's table. *(10 min)*

Do **not** do this in a stage session. It is doc work, it is the mentor's, and it should cost him
nothing.
