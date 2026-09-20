# TEACHING-EVIDENCE — the research behind `CLAUDE.md` Rule 2

A lookup. Open it before relaxing a rule. Gathered 2026-09-16 at his instruction: *"search on the
internet the best way to learn by neuroscientists and the best way to explain new topics and apply it."*
All of it is about **novices** — what he is in Rust and nothing else. Several findings reverse for
experts, which is why `CONCEPTS.md` tracks status per concept.

**Amended 2026-09-20 (correction 17).** Every finding below survived; the sections they pointed at
did not. A worked example still comes before the problem — it is now written from his own code
instead of a second domain, because the translation step, not the example, is where he stopped.

| # | Finding | What it means here |
|---|---|---|
| 1 | **Worked example effect.** Novices learn more from a fully worked solution than from attempting the problem. Example→problem pairs beat problem solving; problem→example pairs do not — a novice cannot diagnose their own failed attempt. | The gather test. §1 is the fully worked example — and it is his own code, so nothing has to be translated (2026-09-20). |
| 2 | **Completion problems.** A partially worked solution the learner finishes is the bridge from example to independent work. | §3 prints the shape whole and leaves one statement for him to write. |
| 3 | **Guidance fading / expertise reversal.** Support that helps a novice becomes extra load once the pattern is known. | Ledger status decides what §1 quotes and what earns §2. |
| 4 | **Split attention / spatial contiguity.** Holding one source in mind while reading another costs capacity; integrating them helps. Robust in meta-analysis. | Every piece the task needs is printed in the brief. Nothing sends him to another file mid-task. |
| 5 | **Retrieval beats re-reading.** | Recall is one question in chat once the stage is green, not a section before the work. |
| 6 | **Desirable difficulties — only when survivable.** A difficulty helps only if the learner has the pieces to overcome it. | Printed-line test; one blank, one value, described in plain words beside it. |
| 7 | **Spacing.** Distributed beats massed practice (≈ *d* 0.85); longer retention wants longer gaps. | His 2–4 h/week gaps are free spacing: the 1–2–4 schedule. |
| 8 | **Interleaving.** Mixing problem kinds feels worse and retains better than blocks. | The stage queue interleaves: a stage reuses a pattern from a non-adjacent earlier one. |
| 9 | **Generation.** Guessing before being told helps even when wrong, if correction follows. | Rule 4a: before opening a hint, he says what he thinks the answer is. |
| 10 | **Elaboration.** Restating in your own words strengthens memory. | Rule 4a's one-sentence commit message. |
| 11 | **Fluency illusion.** Re-reading feels like mastery and is not; objective feedback corrects it. | The test suite is his calibration. When stuck: attempt, run, read the error. |

**What this does not license:** making everything easy (the test is whether he has the pieces, not
comfort); keeping examples once a pattern is known (fading); longer briefs — every finding removes
load rather than adding slots; a flashcard app — two questions per brief is the whole system.

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
