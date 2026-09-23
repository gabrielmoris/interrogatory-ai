# STAGE-LOG — one entry per finished stage

A lookup. What he built, where he got stuck, what to do differently. Lessons that became rules are in
`MENTOR-NOTES.md`; open tidy-ups are in `PROGRESS.md`. Neither is repeated here.

---

### Stage 1 — `Difficulty` and `Tuning` ✅ 2026-08-21

**Built.** `src/difficulty.rs`: four-variant enum, `Tuning`, `Difficulty::ALL`, `tuning()`. Spec
`tests/difficulty.rs`, 7 tests. **Headline:** moves and `Copy` (`E0507`).
**Stuck.** Wrote TS object-literal syntax `Easy: "easy"` inside an array. Pass after a polish pass
(derives on `Tuning`, `Self` in the `impl`, doc comments).

### Stage 2 — `SuspectId` and `FactId` ✅ 2026-08-22

**Built.** `src/ids.rs`: two tuple structs over a private `u32`, `new` / `get`, eight derives,
hand-written `Display` and `From<u32>`. Spec `tests/ids.rs`, 10/10. **Headline:** the newtype pattern.
**Stuck.** Needed the concepts unpacked in chat rather than through hints. His first `Display`
compiled and printed `"3 suspect #"` — the test caught what the compiler could not.

### Stage 3 — `Fact`, `Suspect` and `Case` ✅ 2026-08-23

**Built.** `src/case.rs`: `Suspect`, `Fact` with `HashSet<SuspectId>` `known_by`, `Case` with private
collections. Spec `tests/case.rs`, 9/9. **Headline:** `String` vs `&str`.
**Stuck.** Could not derive signatures from the test's call sites → skeletons are standard from Stage 4.
Reached for the iterator chain unprompted. A missing `!` inverted both visibility tests at once — the
tell for a missing negation.

### Stage 4 — borrowing, `Option<&T>` and lifetimes ✅ 2026-08-24

**Built.** `suspect`, `fact_mut`, `suspect_facts -> impl Iterator<Item = &Fact>`, free
`longer_statement<'a>`. Spec `tests/borrowing.rs`, 12/12. **Headline:** lifetimes as regions.
**Stuck.** No hints needed; read `E0373` himself. First submission 7/12 without having run the suite —
told to run it before declaring done.

### Stage 5 — `AppError`, `thiserror` and `Result` ✅ 2026-08-25

**Built.** `src/error.rs`: `AppError` (named-field variants), `AppResult<T>`, `Serialize` on the ids;
`require_suspect`, `require_fact_mut`, `reveal` on `Case`. Spec `tests/errors.rs`, 14/14; 52 total.
**Stuck.** Two mentor defects: `require_suspect(to)?;` as a line that stores nothing, and where
`#[serde(tag = ...)]` goes. Pass on first submission.

### Stage 6 — withdrawn 2026-08-29, re-cut into 6a–6d (657 lines, ten concepts in one brief).

### Stage 6a — `RawCase` and `Deserialize` ✅ 2026-08-29

**Built.** `RawCase`, `RawSuspect`, `RawFact` with `#[serde(default)]`. Spec `tests/case_raw.rs`, 6/6.
Written ahead of the brief, unprompted and correct.

### Stage 6b — `TryFrom` ✅ 2026-08-29

**Built.** The `try_from` body: two loops, `known_by` revealed, ground-truth flag carried. Spec
`tests/case_convert.rs`, 6/6.
**Stuck.** The empty loop body, and `SuspectId::new` on a `&u32` — `*` had never been taught. He wrote
`for x in &raw.suspects` unprompted, so the planned `E0382` never fired.

### Stage 6c — the four checks ✅ 2026-08-29

**Built.** All four checks in `try_from`. Spec `tests/case_checks.rs`, 8/8.
**Stuck.** Asked twice *which function* the work went in. Swapped `require_fact_mut(..).is_ok()` for
`fact_mut(..).is_some()` after one note: `Option` when absence is normal, `Result` when it is a failure.

### Stage 6d — the front door ✅ 2026-08-30

**Built.** `parse_case`: `toml::from_str` + `.map_err` into `AppError::Parse`, then `raw.try_into()`.
Spec `tests/case_parse.rs`, 4/4; 76 total. Pass on first submission; `.map_err` wrapped only the
parser's failure, which is the mistake hint 4 existed to catch.

### Stage 7 — `VisibleFact<'a>` ✅ 2026-09-01

**Built.** `VisibleFact<'a>` over `&'a Fact`, `id()`, `statement() -> &'a str`, `Case::visible_to`,
`visible_statements`. Spec `tests/visible_fact.rs`, 9/9. No questions mid-stage.
**Do differently.** He wrote `.map(VisibleFact)` point-free, so the planned clippy lint never fired.
He committed without saying "ready" — check `git log` rather than waiting for the word.

### Stage 8 — `storage.rs`, the first shell module ✅ 2026-09-13

**Built.** `case_path`, private `is_slug`, `load_case` with the `ErrorKind::NotFound` split. Spec
`tests/storage.rs`, 9/9; 94 total.
**Stuck.** Step 4, six exchanges, ending in *"ok, I am L O S T"* — three calls named in prose, never
printed, asked for in one step. Once reset to four straight-line steps he wrote the rest himself.

### Stage 9a — the box the screen gets ✅ 2026-09-14

**Built.** `src/ipc.rs`: `SuspectSummary`, `CaseIntro`, both `Serialize`, `impl From<&Case> for
CaseIntro`; `Case::suspects()`. Spec `tests/commands.rs`, 6/6; 100 total.
**Stuck.** All of it — the brief was three files wide and was cut twice mid-flight. Two real slips:
a struct literal with no struct name, and re-wrapping an id already converted at the door.
**Do differently.** When one line takes four exchanges, stop correcting it token by token.

### Stage 9b — a function React can call ✅ 2026-09-15

**Built.** `case_intro`'s body and its place on the handler list. Spec `tests/commands.rs`, 11/11;
105 total. First stage since 7 to land in one sitting. His verdict: *"This way works for me, I learned."*
**Stuck.** Once: `generate_handler![case_intro]` with no path; took rust-analyzer's `use` suggestion.
**Do differently.** He deleted `greet` while `App.tsx` still called it — say what else touches a
command before inviting him to remove one.

### Stage 9c — the app holds the case folder ✅ 2026-09-17

**Built.** `src/state.rs`: `AppState { cases_dir }` and `new`; the 9b body moved into
`case_intro_from(&Path, &str)`; the command down to one line; `.manage(AppState::new(PathBuf::from("cases")))`.
Spec `tests/commands.rs`, 12/12; 106 total across twelve files. `fmt` and `clippy -D warnings` clean.
**Stuck.** On the brief, not the Rust — reissued mid-stage after corrections 12–14 (a `todo!()` over
code he had already written, a hidden one-field struct, a hole whose answer was sixty lines away).
**Do differently.** Neither `case_intro_from` nor `case_intro` has a doc comment; the depot's did. Tidying.

### Stage 10a — where the player is ✅ 2026-09-17

**Built.** `src/transcript.rs`: `Speaker`, `Turn`, `Phase { Briefing, Interrogating { suspect, turns },
Reporting }`, with `suspect()` and `turn_count()`. Spec `tests/phase.rs`, 6/6; 112 total across
thirteen files. `fmt` and `clippy -D warnings` clean. Committed `780d337`.
**Stuck.** Nothing. Pass on first submission, both arms right including `Some(*suspect)` — the `*`
recalled from 6b without help. Written in the same sitting the brief was issued.
**Do differently.** Rule 5's commit-sentence habit was asked once, in §4; the message came out
generated as usual. Asked once — do not ask again.

### Stage 10b — moving between phases ✅ 2026-09-19

**Built.** `Phase::name`, `Phase::begin`, `Phase::finish` in `transcript.rs`. Spec
`tests/phase_moves.rs`, 7/7; 119 total across fourteen files. `fmt` and `clippy -D warnings` clean.
**Stuck.** `finish`'s guard, three attempts: `Briefing`, then `Reporting`, then `Interrogating { .. }`.
He had the replacement line right every time — the confusion was *which* of a move's two phases the
check names. Also copied two of the depot's words into `name()`.
**Do differently.** The unblocking reply said "leaving the room is only allowed from the room", and
"the room" read as the destination — a metaphor doing two jobs. What worked was a two-column table:
*check = where you are now* · *`*self =` = where you end up*. Mentor defect, logged as correction 15.

### Stage 10c — keeping what was said ✅ 2026-09-20

**Built.** `Phase::refusal` (one error for all three moves) and `Phase::record` in `transcript.rs`.
Spec `tests/phase_record.rs`, 5/5; 124 total across fifteen files. `clippy -D warnings` clean;
`cargo fmt` still had a diff at review time — committed as `02a7c33` before it was run.
**Stuck.** Eight exchanges on one line, the worst of the project. The brief's example was in the
parcel-depot domain, and each unblocking reply asked him to carry something across: depot → game,
another file → this one, description → code. *"the gap between the explanation and the task is too
big... I am loosing itnerest on this whole project."* He wrote the arm once the brief was rewritten
from his own code; the last blocker was a struct value with no name, which is 9a's lesson a second
time.
**Do differently.** The rewrite should have been the first reply, not the fifth. Rule 2 is replaced
(correction 17). Recall was not asked at all this stage — due at the start of 10d.

### Stage 10d — what the room will show ✅ 2026-09-21

**Built.** `Phase::transcript` and `Phase::last_line` in `transcript.rs`. Spec
`tests/phase_transcript.rs`, 6/6; `fmt` and `clippy -D warnings` clean against his file in the
reference crate. `last_line` is `self.transcript().last()` — of the two routes the brief offered he
took the one that leaves *where the lines live* decided in one place.
**Stuck.** The first brief, written to the new template, explained before it said what to build:
*"I have no idea what I have to do"* (correction 19, brief and template rewritten). Then one draft,
`&[turns.last()]` — step 2's tool in step 1, wrapped in a new list (`E0308`, expected `Turn`, found
`Option<&Turn>`). One exchange, fixed.
**Do differently.** Goal first, always. Predictions were asked for at both runs and not reported.
Four questions asked at review — outcome recorded in `CONCEPTS.md`.

### Stage 10e — the app holds the phase ✅ 2026-09-22

**Built.** `AppState { cases_dir, phase: Mutex<Phase> }`, `AppState::begin`, and `AppError::Poisoned`
in `error.rs`. Spec `tests/app_phase.rs`, 4/4; 134 across seventeen files, `fmt` and
`clippy -D warnings` clean in the reference crate. Committed `7b5ceba`.
**Stuck.** The whole stage, and the body was printed for him in the end. Step 1 was one line he had
every piece of, and its instruction was mentor idiom — *"This sentence is completelly nosense for
me... I have NO CLUE of what I need to [do]"* (correction 20). Then `self.phase(suspect)`; then the
editor's "make it `&mut self`" quick fix, accepted twice, so the measured `E0596` / `E0524`
checkpoints arrived as failure rather than as the evidence they were written to be.
**Do differently.** The unblocking ladder was too slow — when the line is plumbing and the lesson is
elsewhere, print it whole at the second exchange, not the fifth. Method calls on a field and
`&self` vs `&mut self` are `shaky`, `0 of 2`; 10f is now a consolidation stage run in chat
(`DECISIONS.md`, 2026-09-22), and the lock is asked about there, since he did not type it here.

### Stage 10f — two more methods through the lock ✅ 2026-09-23

**Built.** `AppState::record` and `AppState::suspect` in `state.rs`, the 10e lock on two more
methods. Spec `tests/app_room.rs`, 4/4; `app_phase` 4/4 still, `fmt` and `clippy -D warnings` clean
in the reference crate against his `state.rs`. Committed `591b991`. Run in chat, no brief
(`DECISIONS.md`, 2026-09-22): §0 and one step per message, measured counts `0/4` → `2/4` → `4/4`.
**Stuck.** Nowhere reported. Both method calls on the guard typed unaided; `suspect` came in with
`let phase` (no `mut`) and `Ok(…)` around the answer. He did not report predictions or run output,
so whether he met `E0308` / `unused_mut` on the way is unknown. Recall: both multiple-choice right.
**Do differently.** Asked him for "sure / guessing" twice; he skipped it both times. Don't ask a
third time this stage — ask once at the top of 10g, then drop it if skipped again.
**After green.** The four questions misfired (correction 22). One question on `mut` instead: model
said back correctly in his words.
