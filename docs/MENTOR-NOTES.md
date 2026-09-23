# MENTOR-NOTES — the corrections, verbatim

Not mandatory reading. Open it before changing or relaxing a rule in `CLAUDE.md`. Each entry: what he
said, what caused it, the rule it produced. The quotes are the point — they stop a rule being
softened by a session that never saw the failure.

---

### 2026-09-23 — The four questions asked about a refusal that never happened _(correction 22)_

> "1. It waws never refused... 2. I dont understadnd the question, allow what? 3. No idea
> 4. When was unsafe? I just did it stragith"

10f went green first time. Questions 1, 2 and 4 all presuppose a compiler refusal; asked verbatim,
three of four had no referent, and the one real gap (question 3, why no `mut` on the `suspect`
guard) was buried among them.
→ **Rule 3c amended**: the four questions only follow a refusal. A clean stage gets one question
about a choice he made.

### 2026-09-22 — The ledger said `solid` about code he had never typed unaided _(correction 21)_

> "You are not giving me enough pi3ces to build the puzzle. Youa re assuming I know rust... NO, I
> DONT DONT DO NOT know rust!"

`self.phase.begin(suspect)` — value, dot, method — stood at `solid` in the ledger, on the strength of
lines written inside heavily scaffolded briefs. With no scaffold, 10e got three wrong forms in a row:
`*self.phase::suspect()`, `self.phase(suspect)`, and `&mut self` accepted from the editor twice. A
green suite had been read as evidence, and so had code a brief had printed.
→ **`CONCEPTS.md`: code he did not type is not evidence — a printed line stays `defined`, never
`used`.** Method calls on a field and `&self` vs `&mut self` set to `shaky`, `0 of 2`. **And the
Rule 4 ladder in practice:** when the line is plumbing and the stage's lesson is elsewhere, print it
whole at the second exchange. Correction 20's rewrite was the right move two exchanges too late.

### 2026-09-22 — "Hand X to Y" is not an instruction _(correction 20)_

> "Hand the suspect to the phase's own `begin` (Stage 10b) and hand back its answer. This sentence is
> completelly nosense for me... I have NO CLUE of what I need to you. are you explaining for a machine
> or for a person who needs to learn and understand?"

10e step 1 asked for one line he had every piece of, in a sentence that named neither the value it is
written on nor what the line does. "Hand X to Y" and "hand back its answer" are my idiom; the shape
example (`case.rs :: suspect_count()`) was folded into the same sentence as a subordinate clause, so
the instruction and the example cancelled each other. His draft, `*self.phase::suspect()`, shows what
was missing: `::` where a `.` goes, the argument's name where the method's goes, and a `*` nothing
asked for.
→ **Rule 2 amended**: the *what to do* sentence names the value and the method, in words that work
without Rust; the shape example gets its own paragraph. Step 1 rewritten in place, not appended to.

### 2026-09-21 — The goal came last _(correction 19)_

> "I readed the stage 10d. I have no idea what I have to do. What method in which implementation am
> I doing? What is the objective? Why you give me examples of things before you explain me what am I
> building and where?"

The first 10d brief — written the same day as correction 18, to the new template — opened with the
gameplay reason, then *the picture before you start*, then a separate section gathering his code,
then the new thing, and only in §4 said what to write. The file and method were only in the header's
`Write:` line. Rule 0 had said "the model before the task" and was read as "the model before
saying what the task is". An explanation has nothing to attach to until the goal is known.

→ **Rule 0 re-cut into four moments, goal first. The template's §0 is now *what you are building, and
where*: file, `impl`, method names, what the game uses each for, the stubs to paste, the first run.
No separate "your code" section — each step quotes the piece it copies, beside it.**

### 2026-09-20 — The briefs worked and taught nothing _(correction 18)_

> "You assume I know too much... When you explain a topic, I need you to be less abstract and show me
> in a way I can relate with previous exercise, typescript or natural language."

> "don't make me learn the logic of the app + rust syntax + rust logic. You must tell me in natural
> language what has specifically to be done and give me examples on how rust does that."

And, asked where it broke, he named three: *I say why and not what* · *too many things at once* ·
**no mental model — "I finish the stage without understanding why Rust needed it. I feel I'm
copying."**

The third is the one the old rules could not see. Correction 17's fix — print the shape whole, leave
one value blank — worked on its own terms: 10c went green. That format is an *incomplete worked
example*, and its measured result is construction without comprehension plus inflated confidence
(`TEACHING-EVIDENCE.md` rows 1–4). The ledger said `defined`; the understanding was not there. He
caught it himself, which the research says learners usually cannot.

He also asked for the research: *"Research science of learning and teaching deeply to adapt all this
process and adapt all your .md files to do it. Since I don't want you to have too long context,
adapt, do not just add text."*

→ **Rule 0** — supportive explanation before the task and after it is green; mid-task he gets only
the step. This is what reconciles *"you say why, not what"* with *"I have no mental model"*: both
are true, at different moments. **Rule 3** — the locus test (blanks go where the concept is, not
where the typing is), predict-before-run marked `sure`/`guessing`, one printed version that passes
every test and is still wrong, and four closed-book questions once green. Question 2 — *what would
actually have gone wrong if the compiler had allowed it?* — is the copying detector, and a blank on
it now sets `shaky`. **Rule 5** — a TypeScript *bug he has shipped*, never a TypeScript feature, and
every analogy carries its breakage clause in the same breath. Old Rules 3→4, 4+4a→5, 5→7.

### 2026-09-20 — The example was never his code _(correction 17)_

> "you just give me a bunch of texrt explaining me about depot and you expect me to relate it and get
> WHAT I have to do and translate it then to the interrogatory."

> "the gap between the explanation and the task is too big, assuming that I know everything on the
> midele... It is so hard that I am loosing itnerest on this whole project."

> "When you change the .md files you forget everything because you add text and text instead of
> reading everything and adapting what has to be done."

Five exchanges on one line of Stage 10c. Each reply was shorter than the last and none of them
worked, because all five asked him to carry something across: from the depot to the game, from
another file to this one, from a description to a line of code. The worked example was correct and
it was never *his* code. Asked how to continue, he chose: rewrite the brief first, then he writes
the function.
→ **Rule 2 replaced.** The parcel-depot domain is retired; §1 of a brief is his own code, gathered
and quoted; recall moved out of the brief into one question in chat; the ceiling dropped 90 → 60
lines and the template from six sections to five. 10c was rewritten in place, 288 lines → 121, not
patched. Related: **when a correction lands, rewrite the document, do not append to it** — the first
10c patch left the old example sitting above the new comments and made the brief worse.

### 2026-09-20 — Same words, two jobs _(correction 16)_

> "I am blocked, I think yourt explanations are vague, too technical and unrelated with the tasks."

Mid-10c, step 2. Under the first hole he wrote `action: Phase::Interrogating { suspect: speaker,
turns: self.turn_count() };` — a *build*, where the hole wanted an *ask*. `Phase::Interrogating { … }`
builds after `=` (10b) and asks before `=>` (10a), and no brief had put the two side by side. The
hole's comment described the answer in abstract words ("the phase the player must be in NOW, naming
its list") instead of reprinting his own `turn_count` line, which the fading table already requires
for a `defined` concept. §2c spent its words on borrow theory, not on what he types.
→ **Same syntax doing two jobs is shown side by side before a task uses one** (Rule 2). The hole
comment was rewritten mid-stage; the unblocking reply showed his two lines, `begin` and `turn_count`.

### 2026-09-19 — One word for two things _(correction 15)_

> "I thought I have all right... I don't understannd then, I will need fuirther explanation, not riddles"

Mid-10b, `finish`'s guard. The unblocking reply said the check should "ask about the room, not the
briefing", and handed back his line with a hole. But a move has two phases in it — where you must be
now, and where you end up — and "the room" named one in the metaphor and the other in his head, so
the second attempt was `Reporting` in both places. **A hole plus a metaphor is a riddle when the
learner has not separated the two things the metaphor covers.** What worked was naming them as two
different slots, in a table: *check = where you are now* · *`*self =` = where you end up*.
→ **When one line takes two attempts, stop blanking and name the parts.** `CONCEPTS.md` carries it
as a `shaky` row.

### 2026-09-16 — A pointer is not an explanation _(correction 14)_

> "step 3 is so bad explained that I can't understand what I have to do there"

9c step 3 was a bare `____ // one line, same shape as the depot's`, with the depot line sixty lines
up and two imports never mentioned. Step 2 in the same brief printed its before/after in place and he
did it unaided. → **Every hole reprints its answer-shaped line beside it and names its imports.**

### 2026-09-16 — The explanation and the task were never the same problem _(correction 13)_

> "Right now I feel you explain something and the tasks are just a bit related. NO DUDE, be
> pedagogical! search on the internet the best way to learn by neuroscientists and the best way to
> explain new topics and apply it, change whatever it takes in the docs and in Claude.md"

9c's example showed a struct and never its `new`, then asked for `AppState::new`. 9a's example was a
flat `From`, then the task needed `.map().collect()`. Both times the gap was where he stopped.
→ **Rule 2 rewritten around the match test**, holes, fading and spaced recall. Research:
`TEACHING-EVIDENCE.md`.

### 2026-09-16 — Two kinds of blank _(correction 12)_

> "I thought that the case_intro was actually already done in step 9b!!!!!"

> "AppState struct has no hints how should it look like, you think I can learn something just going
> throught the test files and checking the shape or is just a waste of time because it adds
> cognitive load and actually doesn't bring me any new knowledge?"

9c re-issued his 9b body as `todo!()`, and hid a struct with exactly one possible field. In 9a, hiding
`CaseIntro`'s fields *was* the lesson. → **Show moves as before/after. Hide a shape only when choosing
it is the lesson.**

### 2026-09-14 — He set the budget himself: one _(correction 11)_

> "how can I know which shape ahs `CaseIntro` if I dont know what is it for???"

> "I want you to split the tasks in small chunks that I can do EASY introducing MAXIUMUM 1 new topic
> and reviewing something, but not more than 2 topics. Split the tasks and explain then much more
> basically so I, a rust NEWBIE can I uncerstand."

The already re-cut 9a had two concepts and 73 prose lines, and still explained what `CaseIntro` *was*
without showing the screen it is *for*. → **Rule 1: one new thing plus one recall. Rule 5: show the
artifact before naming the type.**

### 2026-09-14 — Two concepts on paper, ten new things in the document _(correction 10)_

> "I am a bit lsot. Can you please ecxplain less rust advanced and more LERNING RUST??????"

9a passed every check but was three files and five steps. And step 3 said *"for each candidate field
ask: does the briefing screen draw it?"* — a thing to weigh up, not a thing to do. → **Count the
document, not the ledger; the printed-line test is a sizing test; a step says what to do.**

### 2026-09-13 — He set the explanation ladder himself _(correction 9)_

> "1. New concept. you explain with typescript comparations if there are some and clearly explains
> how it works 2. Concept that we already touched: you recall it and explains where we saw it before 3. Concept that we saw before many times, just mention it and tell that we saw it in 2 or 3
> examples before 4. Concept that I keep failing after recalling it? give the examples back and explain it again"

The fourth tier was forbidden by the rules then in force ("never a second explanation"). `.map_err`,
`defined` since 6d, went wrong in Stage 8. → **the `shaky` status.**

### 2026-09-13 — Prose is not teaching _(correction 8)_

> "I have never used rust to read a file, your explanation assumes i know because you never explain
> me anything, just give me a boilerplate and expects me to find out from infuse science how to do
> it."

> "no, the two more lines are not mine because I suspect they have steps I have never touched."

> "ok, I am L O S T"

Stage 8 step 4 needed `fs::read_to_string`, `.display()` and a value-returning `map_err` closure, none
ever printed as code. The unblocking reply told him to paste a line; hint 4 answered step 5.
→ **Printed-line test; rung 4 answers the current step; never hand him a line to paste.**

### 2026-08-30 — The line ceiling measured the wrong thing

| Brief  | 1   | 2   | 3   | 4   | 5   | 6a  | 6b  | 6c  | 6d  |
| ------ | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Prose  | 57  | 137 | 195 | 205 | 235 | 75  | 85  | 87  | 75  |
| Code   | 30  | 71  | 131 | 140 | 202 | 59  | 48  | 31  | 34  |
| Landed | ✅  | ✗   | ✗   | ✗   | ✗   | ✅  | ✅  | ✅  | ✅  |

Everything ≤ ~90 prose lines landed; everything over 130 produced a correction. Code volume does not
separate them, and he asks for more code. → **90 lines of prose, code not counted.** Later: 8 (~90) ✗,
9a (73) ✗, 9b (69) ✅ — volume is necessary, not sufficient.

### 2026-08-29 — Reasons are not instructions _(correction 7)_

> "You never explain in plain language WHAT I have to do? The logic? You mix rust terms with machine
> logic and expect me to understand everything. I am a person!"

Mid-6c, an eight-item reply — why, the trap, the method, laziness, a code block — and the instruction
was never stated. The six-line reply that worked: what to do, the function name, a handback.
→ **Rule 4, slot 1.**

### 2026-08-29 — Briefs grew after every request to slow down

| Stage        | 1   | 2   | 3   | 4   | 5   | 6   |
| ------------ | --- | --- | --- | --- | --- | --- |
| Lines        | 148 | 295 | 442 | 472 | 610 | 657 |
| New concepts | 10  | 10  | 9   | 9   | 13  | 10  |

Every rule then governed wording, none governed volume, so each correction produced gentler prose and
more of it. → **a concept budget, a fixed template, `CONCEPTS.md`.** Same day: `ROADMAP.md` still
specified the rejected `crates/core` split in 11 places. → **Grep for the rejected thing before
closing a decision.**

### 2026-08-27 — Now it is the chat replies _(corrections 4 and 5)_

> "Your explanations are too detailed and saying too much information at once, making the learning
> process too difficult. I asked you to be pedagogic already the last 3 stages."

> "I am blindly trying to follow you up but I am stopping having interest because it is getting too
> deep and introducing many things at the same time some of them even without explanation. You wrote
> it in CLAUDE.md but seems that you forget all time."

Four headed sections in answer to one compile error; then a *shorter* reply carrying three untaught
things. → **Rule 4. Short is not small. Losing his interest is the failure state.**

### 2026-08-25 — Architecture questions are not homework

> "you made a mistake, it is not my responsibility to solve your problem... we are not doing a
> product, you are teaching and I am learning."

Stage 4 duplicated a rule the roadmap had already given an owner, and the resulting bug was handed
back to him as a design question. → **Rule 7.**

### 2026-08-25 — "Be more pedagogical" meant the briefs too

> "You keep using language too technical and too into rust... this is the FIRST time I code in rust.
> Be more pedagogical."

The Stage 5 draft argued its own architecture at him and used extern prelude, supertrait, orthogonal,
desugars. → **No mentor-facing reasoning in a brief; the banned-vocabulary list.**

Same stage: `self.require_suspect(to)?;` cost four exchanges (*"either I am dumb or you are not being
clear"*). What worked: "`?` does two jobs; here only the first matters." He had deleted the call to
silence `E0502` and it compiled with the check gone. "Add one line above the enum" was not placeable.
→ **print the surrounding lines of the real file.**

### 2026-08-23 — Reading signatures off the test was a step too far

Three wrong guesses. → **Issue the signature skeleton; teach `todo!()`** (not for `impl Iterator` —
use `std::iter::empty()`). "One boolean in your filter closure" did not parse.

### 2026-08-22 — "Like if I were writing Rust for years"

> "the way you explain is like if I were writing Rust for years... go slowly."

What worked: one concept per reply, from zero — trait → `derive` → moves → `From` — each ending with
one command. → **Rule 5.**
