# TEACHING-EVIDENCE — why the rules in `CLAUDE.md` are the rules

A lookup, not a read-through. Open it when a rule in `CLAUDE.md` looks arbitrary or when you are
tempted to relax one. Added 2026-09-16, at his instruction: *"search on the internet the best way to
learn by neuroscientists and the best way to explain new topics and apply it."*

Everything here is about **novices**, which is what he is in Rust and nothing else. Several of these
findings reverse for experts — that reversal is itself one of the findings, and it is why
`CONCEPTS.md` tracks status per concept rather than treating him as a beginner forever.

---

## 1. Worked examples, then a problem of the same type

**Finding.** Novices learn more, in less time, from studying a fully worked solution than from
attempting the problem — the *worked example effect*. Comparing orderings directly: worked examples
alone and **example→problem pairs** both beat problem-solving alone on learning outcomes and on
measured cognitive load. **Problem→example pairs did not** — no better than plain problem solving,
because a novice cannot reliably diagnose what went wrong in their own attempt.

**Why.** Facing an unfamiliar problem, a novice falls back on means–ends analysis — thrashing
toward the goal. That consumes working memory without leaving a schema behind. A worked example
lets the schema form first.

**→ Rule 2.** The worked example comes first, in full, and the task that follows is *the same
problem*. Hence the match test. This is the rule he asked for in his own words — "the task MUST
match the explanation" — and it is the single best-supported claim in this file.

## 2. Completion problems are the step in between

**Finding.** The *completion problem effect*: a problem statement plus a partially worked solution,
with the learner finishing it. It combines studying and solving in one task, and is the documented
bridge from worked example to independent work.

**→ Rule 2, "holes, not blanks".** §3.2 gives the shape with one or two pieces missing. A blank body
is a cliff; a hole is a step.

## 3. Fade the guidance as the pattern becomes known

**Finding.** The *guidance fading effect*: gradually removing support as expertise grows beats both
constant support and an abrupt switch to unaided work. The *expertise reversal effect* is the reason
— instruction that helps a novice becomes redundant for someone who has the schema, and processing
redundant material itself loads working memory. Support that helped in week one actively hurts in
week six.

**→ Rule 2's fading table, keyed to `CONCEPTS.md`.** Absent → full example plus a completion
problem. `used` → the bare task. `shaky` → back to the full example. The ledger now governs
scaffolding, not only word count.

## 4. Keep the explanation next to the thing it explains

**Finding.** The *split-attention effect* and its remedy, the *spatial contiguity principle*: when
learners must hold one source in mind while reading another to make either make sense, the mental
integration itself consumes capacity. Physically integrating them improves learning; the effect is
robust across meta-analysis.

**→ Rule 2, "adjacency".** An explanation in §2 and its task forty lines later in §3 is a
split-attention violation with extra steps. Each hole sits beside the step that fills it.

## 5. Retrieval beats re-reading

**Finding.** Retrieval practice — being asked to produce something from memory — strengthens both
immediate access and long-term retention more than restudying the same material. Spacing and
interleaving work for the same reason: they force retrieval when the material has partly faded.

**→ Rule 2, §1.** Two questions he answers from memory, answers folded away. Not a refresher
paragraph he skims. The old §2 "Refresher" was re-reading, which is the weak version.

## 6. Difficulty has to be survivable to be useful

**Finding.** *Desirable difficulties* — spacing, interleaving, varied conditions, generating rather
than being shown — improve long-term retention even though they make practice feel worse and look
slower. But a difficulty is only desirable if the learner **can** overcome it: "many difficulties are
undesirable during instruction and forever after," and a difficulty becomes undesirable when the
learner lacks the background knowledge to respond successfully.

**→ Rule 1's printed-line test and Rule 2's hole rule.** Making him produce a call he has never
typed is not a desirable difficulty; it is a wall, and it produced *"ok, I am L O S T"* in Stage 8
and *"no idea, dude"* in 9a. The difficulty belongs where he has the pieces — which is what
a completion problem guarantees.

## 7. Spacing — his schedule already provides it; the system has to use it

**Finding.** Distributed practice beats massed practice, reliably and by a lot (around *d* = 0.85 for
verbal material). The meta-analytic result that matters for planning: **the longer you want to
retain something, the longer the gap between practices should be.** Expanding gaps did somewhat
better than equal ones, though that particular comparison is noisy.

**Why it matters here.** He works 2–4 hours a week, so weeks pass between touching the same idea.
That is not a problem to work around — it is the optimal condition, free. What was missing is
*retrieval at those gaps*: every brief handed him a refresher paragraph, which is re-reading.

**→ Rule 2, the 1–2–4 schedule.** A concept taught in stage N gets a recall question at N+1, N+3 and
N+7 — gaps of one, two and four stages, which at his pace is roughly one week, two weeks, a month.
`CONCEPTS.md` carries the due list.

## 8. Interleaving — mix the kinds of problem, even though it feels worse

**Finding.** Practising one kind of problem in a block feels more fluent and produces worse
retention and transfer than mixing kinds. Learners consistently rate blocked practice as more
effective while performing worse on it. It is the clearest case of a difficulty that is desirable
and feels undesirable.

**→ Rule 2's interleaved step.** Every stage's task carries **one step that uses a pattern from a
non-adjacent earlier stage** — not the one just taught. This was in the template as the "cold call"
and it earns its place on the evidence, not on instinct.

## 9. Generation — guess before being told

**Finding.** Attempting to produce an answer before being shown it improves learning, **even when
the attempt is wrong**, provided corrective feedback follows. Generating beats recognising;
producing an example or analogy beats reading one.

**→ Rule 2, §1's prediction line.** One question he commits to an answer for before reading §2, with
the answer in §2 itself. Costs one line.

## 10. Elaboration and reflection — say it in your own words

**Finding.** Restating an idea in your own words and connecting it to what you already know
strengthens it more than re-reading the original phrasing. Reflection — asking what worked, what
didn't, what it resembles — is retrieval and elaboration combined.

**→ His habit, not a brief section.** The commit message at the end of a stage carries one sentence,
in his own words, saying what the new thing does. Thirty seconds, and it is elaboration plus
retrieval at exactly the moment the material is fresh.

## 11. Calibration, and the illusions that make good study feel bad

**Finding.** Re-reading produces a **fluency illusion**: familiarity that feels like mastery. It is
the most popular study strategy and among the least effective. Massed practice produces fast
apparent gains that fade. Learners are poor judges of their own learning, and the fix is objective
feedback — low-stakes testing that exposes the gap between believing and knowing.

**→ Two things.** The test suite and the checkpoint table are his calibration instrument: the number
is the truth and his sense of "I've got this" is not. And when he is stuck, **re-reading the brief is
the illusion, not the remedy** — attempt, run it, read the error. Tell him that rather than pointing
him back up the page.

---

## What this does not license

- It does not license making everything easy. Generation and retrieval are effortful on purpose; the
  test is whether he has the pieces, not whether it feels comfortable.
- It does not license skipping the examples once a pattern is known — §3 says the opposite, and the
  fading table is the mechanism.
- It does not license longer briefs. Every finding here cuts material rather than adding it: the
  extra load being removed is *extraneous* load. Sections 7–11 change **what goes in the slots the
  template already has** — which two questions §1 asks, which step §3 interleaves — not how many
  slots there are.
- It does not license a flashcard app. The 1–2–4 schedule runs on `CONCEPTS.md` and two questions per
  brief. Anything heavier will be skipped, and a schedule that is skipped is worse than none.

## Sources

- Sweller, *The Guidance Fading Effect* — worked example, completion problem, guidance fading,
  expertise reversal, and the recommended novice → intermediate → advanced sequence:
  <https://cogscisci.wordpress.com/wp-content/uploads/2019/08/sweller-guidance-fading.pdf>
- Van Gog, Kester & Paas, *Effects of worked examples, example–problem, and problem–example pairs on
  novices' learning*: <https://www.sciencedirect.com/science/article/abs/pii/S0361476X1000055X>
- Renkl & Atkinson, *How Fading Worked Solution Steps Works — A Cognitive Load Perspective*:
  <https://link.springer.com/article/10.1023/B:TRUC.0000021815.74806.f6>
- Mayer, *Applying the Science of Learning: Evidence-Based Principles for the Design of Multimedia
  Instruction*:
  <https://pressbooks.pub/learningenvironmentsdesign/chapter/mayer-applying-the-science-of-learning-evidence-based-principles-for-the-design-of-multimedia-instruction/>
- Schroeder & Cenkci, *Spatial Contiguity and Spatial Split-Attention Effects in Multimedia Learning
  Environments: a Meta-Analysis*: <https://link.springer.com/article/10.1007/s10648-018-9435-9>
- Bjork & Bjork, *Introducing Desirable Difficulties Into Practice and Instruction*:
  <https://www.unh.edu/teaching-learning-resource-hub/sites/default/files/media/2023-06/itow-introducing-desirable-difficulties-into-practice-and-instruction-bjork-and-bjork.pdf>
- Brown, Roediger & McDaniel, *Make It Stick: The Science of Successful Learning* — retrieval,
  spacing, interleaving, elaboration, generation, reflection, calibration, and the illusions of
  knowing. Publisher sample: <https://www.hup.harvard.edu/file/feeds/PDF/9780674729018_sample.pdf> ·
  principles summarised:
  <https://blog.apaonline.org/2020/02/19/takeaways-from-make-it-stick-the-science-of-successful-learning/>
- Cepeda, Pashler, Vul, Wixted & Rohrer, *Distributed practice in verbal recall tasks: A review and
  quantitative synthesis* — the spacing-interval / retention-interval relationship:
  <https://augmentingcognition.com/assets/Cepeda2006.pdf>
