# TEACHING-EVIDENCE — the research every rule in `CLAUDE.md` is built on

A lookup. Open it before relaxing a rule, not for context. **Rewritten 2026-09-20** at his
instruction — *"Research science of learning and teaching deeply to adapt all this process"* — after
he reported the failure the old evidence base could not see: the briefs worked and taught nothing.

All of it is about **novices**, which is what he is in Rust and nothing else. Several findings
reverse for experts; that is why `CONCEPTS.md` tracks status per concept rather than per person.

---

## The correction of 2026-09-20 — why the previous format failed

The old §3 printed the whole shape and left one value blank. That is a named format with a measured
result, and the result is exactly what he reported.

| # | Finding | Consequence here |
|---|---|---|
| 1 | **Completion beats generation on construction and not on comprehension.** van Merriënboer (1990) taught programming to 57 high-schoolers: the completion group beat the generation group on writing programs and on dropout, but *"no differences occurred in the ability to interpret programs."* | The format bought exactly the thing he stopped valuing, and could not buy the thing he asked for. Rule 3. |
| 2 | **Incomplete worked examples specifically inflate confidence.** Leppink et al. (2015), N=67, three conditions: incomplete worked examples produced *"an overestimation of performance, that is, an illusion of understanding."* Completion problems and conventional problems were both accurately calibrated. | One blank in a printed shape is the bad condition. A whole body he writes from a given signature is the good one. Rule 2's "signatures given, bodies his". |
| 3 | **Copying is the measurable signature of the weak learner, not a style.** Chi et al. (1989), think-aloud on worked examples: good learners noticed 9.3 comprehension failures per example, poor learners 1.1; poor learners re-read the example 4.1 times per problem and treated examples as *solution templates*. | *"I feel I am copying"* is a literally accurate self-report of a known profile — and detecting it is what the poor-learner profile normally cannot do. Take the report as data. |
| 4 | **Performance during learning and actual learning dissociate, and fluency fools people.** Soderstrom & Bjork (2015). Learners' judgments track how smoothly the session went, not what was retained. | A green suite is not evidence. Rule 3b prints a version that passes and is wrong. |
| 5 | **More printed explanation does not fix it.** Wittwer & Renkl (2010), 21 experiments: the benefit of instructional explanations in example-based learning is *"minimal"*, and they are *not* more effective than having the learner self-explain. | The fix is a swap, never an addition. Hence the 60-line ceiling stays. |

**What the same literature says the fix is** — and it is additive to the scaffolding, not a
replacement for it:

| # | Finding | Consequence here |
|---|---|---|
| 6 | **Self-explanation prompts.** Bisra et al. (2018) meta-analysis: g = .55 over 69 effects, N = 5,917. **Conceptual** prompts ("explain what this means", "why does Rust need this") g = .87; justification prompts .42; metacognitive prompts .19, not significant. Self-explaining **beats** being handed the explanation (.67 vs .35). Chi et al. (1994): prompted learners gained 32% vs 22%, and **100% of high self-explainers reached the correct mental model vs 22% of controls.** | Rule 3c, the four questions. Conceptual wording only — never "are you sure?" |
| 7 | **Prompts and fading are independent and free.** Atkinson, Renkl & Merrill (2003): fading and self-explanation prompts both had main effects, **no interaction**, and **no additional time on task**. | Keep the scaffolding. Add the questions. Rule 3's opening line. |
| 8 | **Predicting before running beats being told, in programming specifically.** Randomised, N=121, R programming: predict-first beat tell-and-practice on learning (d = 0.36), produced more distinct solutions (d = 0.47), **lowered perceived extraneous load** (p = .005), improved response to errors (p = .002), at no time cost. | Rule 3a. Note it lowers his felt load — it is not an extra burden. |
| 9 | **Surprise is the mechanism, and it scales with confidence.** Brod; Butterfield & Metcalfe: errors made *confidently* are corrected best; low-confidence guesses produce no surprise and little learning. Feedback must be immediate. High-confidence errors return after about a week. | Why the prediction is marked `sure` or `guessing`, and why a confident miss is re-asked later. |
| 10 | **Subgoal labels, in programming.** Margulieux, Catrambone & Guzdial: plain-English purpose labels over chunks of a worked example, three experiments, f = .53 / .59 / 1.01, and **46% better delayed retention**. Their stated mechanism is his complaint: learners are made to acquire the procedure *and* the syntax at once. | Rule 2, "subgoals before syntax". |
| 11 | **Isolated elements.** Pollock, Chandler & Sweller: when interacting elements exceed working memory, teach them in isolation first, then integrated. Intrinsic load is a function of complexity **and the learner's existing knowledge** — so it is domain-local, and his TypeScript seniority does not lower it. | Rule 1's three element kinds, and the header naming which is new. |
| 12 | **Guidance fading / expertise reversal.** Kalyuga; Renkl & Atkinson. Support that helps a novice becomes load once the pattern is known — withdraw one step at a time, and say which. | Ledger status decides what §2 quotes and what earns §3. |
| 13 | **Split attention.** Schroeder & Cenkci meta-analysis: holding one source in mind while reading another costs capacity. | Every piece the task needs is printed in the brief. Nothing sends him to another file mid-task. |

## Learning a second language is not a discount — it is a hazard

| # | Finding | Consequence here |
|---|---|---|
| 14 | **61% of cross-language questions contain an incorrect assumption imported from the first language.** Shrestha, Botta, Barik & Parnin (ICSE 2020), 450 Stack Overflow posts across 18 language pairs plus 16 professionals averaging 12.8 years' experience. Rust's borrow checker is classified under *"little-to-no mapping"*; one participant: *"a very alien concept in Rust."* | Rule 5. A TypeScript analogy for ownership does not help him; it plants something to unlearn. |
| 15 | **Similar syntax + different semantics = negative transfer.** Tshukudu & Cutts. | `let`, `&`, `mut`, `match`, `impl` all look familiar and are not. State what each does *not* mean. |
| 16 | **Analogies need an explicit breakage clause.** Richland et al.: on problems where surface and structure conflict, accuracy fell to .51 from .80. Marking where the analogy breaks — plus keeping the source visible — recovered much of it: .41 → .62, d = .55, holding at one week. | Rule 5's "*like X in that ___; differs in ___; stops at ___*". |

## Rust is measurably hard, and the hard part is not the syntax

| # | Finding | Consequence here |
|---|---|---|
| 17 | **Learners who have finished the book still cannot say what the rule prevents.** Crichton, Gray & Krishnamurthi (OOPSLA 2023), N=36 who had read The Rust Programming Language: **64%** could predict *that* the compiler rejects a program, **46%** could fix it, and only **31%** could say what would actually go wrong at runtime if the rule were lifted. Only 3 of 15 recognised a safe program the checker rejects anyway. | The four questions are theirs. Question 2 is the 31% one — the copying detector. Questions 3 and 4 are the 46% and the 3-of-15. |
| 18 | **Teaching an explicit notional machine measurably fixes it.** Same paper: rebuilding the ownership chapter around a permissions model plus concrete counterexamples of what breaks moved accuracy 48% → 57%, p < 0.001, d = 0.56, with the largest gains on the undefined-behaviour and fixing items. Caveat: temporal A/B, not randomised. | Rule 5's "two machines, named", and §1 of the template. |
| 19 | **Difficulty is construct-specific, not rule-specific.** Zhu et al. (ICSE 2022), 15,509 Stack Overflow questions plus 101 practitioners: move violations are 1.96× more likely inside loops, borrow violations 1.57× with hash maps. Lifetimes beat ownership as the hardest feature — only **10%** always understand a lifetime error message. | "He knows the borrow rule" does not mean he knows it inside a `for` loop. Re-check per construct; that is what a consolidation stage is for. |
| 20 | **Learners cannot tell a soundness rejection from an incompleteness one**, and the two need different fixes. Crichton (2020). The observed failure is a gratuitous `.clone()`. | Question 4. And why Rule 3b prints the cloning version. |

## Durability on 2–4 h/week

| # | Finding | Consequence here |
|---|---|---|
| 21 | **Retrieval practice.** Adesope et al. (2017), 272 effects: testing beats restudy by 0.51 and beats no activity by 0.93. **Multiple choice (0.70) outperformed short answer (0.48)** — a cheap format is not a weak one. | Rule 3's session-opening recall question. MC is allowed. |
| 22 | **Spacing, and the non-necessity of clever schedules.** Spaced beats massed at g ≈ 1.01. **Expanding intervals are not superior to uniform ones** (g = 0.03, n.s.). | His 2–4 h/week gaps are free spacing. The plain 1–2–4 table is enough; do not build anything more elaborate. |
| 23 | **Interleaving works only for confusable things.** Brunmair & Richter (2019), 59 studies: g = 0.42 overall, but **null for expository text and −0.39 for vocabulary** — there, blocking wins. It pays when between-category similarity is high and within-category similarity is low. | Interleave `T` / `&T` / `&mut T`, `String` / `&str` / `&[T]`, `?` / `unwrap` / `match`. Do **not** interleave unrelated topics or reading. |
| 24 | **Generation.** Guessing before being told helps even when the guess is wrong, provided correction follows. | Rule 5's third habit, and Rule 3a. |

## Contested, and handled deliberately

- **Self-explanation × worked examples.** Barbieri et al. (2023), 55 studies on the worked-example
  effect in mathematics, found self-explanation prompts *negatively* moderated it, against Atkinson
  (row 7) and Bisra (row 6). Rittle-Johnson et al. (2017) give the likely mechanism: explanation is
  **time-competitive**, and why-prompts can raise conceptual knowledge while *lowering* transfer by
  eating practice; explaining one's *own wrong idea* can make things worse. **What we do about it:**
  conceptual prompts only (the strongest moderator), explain the *contrast* between the version that
  compiled and the version that did not rather than his own wrong idea, cap it at four questions and
  two minutes, and take the time out of printed explanation rather than out of practice.
- **Germane load as a third load type** is disputed (Kalyuga 2011) and was redefined by Sweller et al.
  (2019) as redistribution, not a separate budget. Nothing here depends on the distinction.
- **Not a learning-styles claim.** Pashler et al. (2008): the meshing hypothesis is unsupported. His
  *"show me in a way I can relate with"* is a request to activate prior knowledge, which is well
  evidenced. Do not build a learner profile out of it.

## What this does not license

Making everything easy — the test is whether he has the pieces, not whether he is comfortable.
Keeping examples once a pattern is known (row 12). **Longer briefs** — every finding above removes
load or swaps one thing for another; none adds a slot. A flashcard app: one recall question and four
exit questions is the entire system.

## Sources

Primary, in the order the rows use them.

- van Merriënboer (1990), *Strategies for Programming Instruction in High School: Program Completion
  vs. Program Generation*, J. Educational Computing Research —
  <https://journals.sagepub.com/doi/10.2190/4NK5-17L7-TWQV-1EHL>
- Leppink et al. (2015), *Completion problems can reduce the illusions of understanding*,
  Contemporary Educational Psychology —
  <https://www.sciencedirect.com/science/article/abs/pii/S0361476X15000028>
- Chi, Bassok, Lewis, Reimann & Glaser (1989), *Self-Explanations*, Cognitive Science 13(2) —
  <https://files.eric.ed.gov/fulltext/ED296291.pdf>
- Chi, de Leeuw, Chiu & LaVancher (1994), *Eliciting Self-Explanations Improves Understanding* —
  <https://andymatuschak.org/files/papers/Chi%20et%20al%20-%201994%20-%20Eliciting%20self-explanations%20improves%20understanding.pdf>
- Soderstrom & Bjork (2015), *Learning Versus Performance: An Integrative Review* —
  <https://bjorklab.psych.ucla.edu/wp-content/uploads/sites/13/2016/11/soderstorm_ra_learningvsperformance.pdf>
- Wittwer & Renkl (2010), *How Effective are Instructional Explanations in Example-Based Learning?* —
  <https://link.springer.com/article/10.1007/s10648-010-9136-5>
- Bisra, Liu, Nesbit, Salimi & Winne (2018), *Inducing Self-Explanation: a Meta-Analysis* —
  <https://link.springer.com/article/10.1007/s10648-018-9434-x>
- Atkinson, Renkl & Merrill (2003), *Transitioning From Studying Examples to Solving Problems*, JEP
  95(4) —
  <https://mrbartonmaths.com/resourcesnew/8.%20Research/Making%20the%20most%20of%20examples/Fading%20out%20and%20Prompts.pdf>
- *Prediction versus production for teaching computer programming* (2023), Learning and Instruction —
  <https://uclatall.com/pdfs/predictionversusproduction.pdf>
- Brod (2021), *Predicting as a learning strategy*, Psychonomic Bulletin & Review —
  <https://d-nb.info/1355035333/34>
- Margulieux, Catrambone & Guzdial (2016), *Employing subgoals in computer programming education* —
  <https://bpb-us-e1.wpmucdn.com/sites.gatech.edu/dist/b/1555/files/2020/09/MargulieuxCatramboneGuzdial2016.pdf>
- Sweller, van Merriënboer & Paas (2019), *Cognitive Architecture and Instructional Design: 20 Years
  Later* — <https://link.springer.com/article/10.1007/s10648-019-09465-5>
- Kalyuga (2011), *How Many Types of Load Does It Really Need?* —
  <https://link.springer.com/article/10.1007/s10648-010-9150-7>
- Renkl & Atkinson (2003), *How Fading Worked Solution Steps Works* —
  <https://link.springer.com/article/10.1023/B:TRUC.0000021815.74806.f6>
- Schroeder & Cenkci (2018), *Spatial Contiguity and Spatial Split-Attention Effects: a
  Meta-Analysis* — <https://link.springer.com/article/10.1007/s10648-018-9435-9>
- Shrestha, Botta, Barik & Parnin (2020), *Here We Go Again: Why Is It Difficult for Developers to
  Learn Another Programming Language?*, ICSE —
  <https://www.chrisparnin.me/pdf/Cross-lang-interference.pdf>
- Tshukudu & Cutts (2020), *Understanding conceptual transfer for students learning new programming
  languages*, ICER — <https://dl.acm.org/doi/abs/10.1145/3372782.3406270>
- Richland et al. (2010), *Learning by analogy: Discriminating between potential analogs* —
  <https://www.uciscienceoflearning.org/uploads/1/1/7/8/117864006/16_richland__2010__learning_by_analogy.pdf>
- Crichton, Gray & Krishnamurthi (2023), *A Grounded Conceptual Model for Ownership Types in Rust*,
  OOPSLA — <https://arxiv.org/abs/2309.04134> · tooling <https://cel.cs.brown.edu/aquascope/> ·
  book fork with the quizzes <https://rust-book.cs.brown.edu/>
- Crichton (2020), *The Usability of Ownership* — <https://arxiv.org/pdf/2011.06171>
- Zhu, Zhang, Qin, Xiong & Song (2022), *Learning and Programming Challenges of Rust*, ICSE —
  <https://songlh.github.io/paper/survey.pdf>
- Adesope, Trevisan & Sundararajan (2017), *Rethinking the Use of Tests*, Review of Educational
  Research — <https://journals.sagepub.com/doi/abs/10.3102/0034654316689306>
- Brunmair & Richter (2019), *Similarity matters: A meta-analysis of interleaved learning*,
  Psychological Bulletin —
  <https://www.psychologie.uni-wuerzburg.de/fileadmin/06020400/2019/Brunmair_Richter_in_press__2019_META-ANALYSIS_OF_INTERLEAVED_LEARNING.pdf>
- Barbieri, Miller-Cotto et al. (2023), *A Meta-analysis of the Worked Examples Effect on Mathematics
  Performance* — <https://link.springer.com/article/10.1007/s10648-023-09745-1>
- Rittle-Johnson, Loehr & Durkin (2017), *Eliciting explanations: Constraints on when
  self-explanation aids learning* — <https://link.springer.com/article/10.3758/s13423-016-1079-5>
- Pashler, McDaniel, Rohrer & Bjork (2008), *Learning Styles: Concepts and Evidence* —
  <https://journals.sagepub.com/doi/full/10.1111/j.1539-6053.2009.01038.x>
