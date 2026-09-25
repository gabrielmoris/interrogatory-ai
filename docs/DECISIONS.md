# DECISIONS — architecture, newest first

A lookup, not a read-through. Architecture only — teaching-process changes live in `MENTOR-NOTES.md`.
Per entry: decided / why / rejected / costs. If an entry needs more, it was two decisions.

---

### 2026-09-25 — 10h's spec only checks the command's shape; no `tauri::test`

**Decided.** `tests/start_interrogation.rs` assigns `begin_interrogation` to a
`fn(State<'_, AppState>, String, SuspectId) -> AppResult<()>`. It compiles or it does not. What the
command does is `begin_interrogation_from`, already tested in `pick_suspect.rs`. No `[dev-dependencies]`.
**Why.** The first version (2026-09-24) used `tauri::test::mock_app`. On Windows its test binary dies
with `STATUS_ENTRYPOINT_NOT_FOUND` (0xc0000139): test binaries lack the Common Controls v6 manifest
that `tauri-build` embeds only in the app (tauri-apps/tauri#13419). Measured on his machine 2026-09-25.
**Rejected.** Embedding the manifest for every target from `build.rs` — cannot be verified from the
Linux container, and it is linker plumbing with nothing to teach. A full IPC round trip —
`generate_handler!` cannot name a command from another crate (measured).
**Costs.** The handler list and the JSON argument names are checked only by the console step.
Revisit if a later stage needs a running Tauri in tests; fix the manifest then, on his machine.

### 2026-09-23 — An id from React is checked against the case file inside `begin_interrogation_from`

**Decided.** `ipc.rs :: begin_interrogation_from(state: &AppState, slug, suspect: SuspectId)` loads
the case by slug, calls `Case::require_suspect`, and only then `AppState::begin`. `SuspectId` derives
`Deserialize`; `FactId` does not. Stage 10g is this function; 10h is the `#[tauri::command]` wrapper,
`generate_handler!` and the React call — split so each carries one new thing.
**Why.** Deserializing proves the shape, not that the suspect exists. The check sits right after it,
using `require_suspect`, the rule's one owner. `AppState` holding no `Case` is fine while the case
file is small; `case_intro` already reads it per call.
**Rejected.** Storing the loaded `Case` in `AppState` now — needs a "choose case" command and a second
lock with no stage asking for it. Taking `u32` and building the id in the command — the same check,
with a second conversion to keep in step.
**Costs.** The case file is read on every begin. Revisit when `AppState` needs the case for the
report (Phase 3), and then load once.

### 2026-09-22 — 10f is a consolidation stage, and it runs in chat instead of a brief

**Decided.** 10f carries zero new elements: `AppState::suspect()` and `AppState::record()`, the same
lock lines on two more methods, typed by him one line at a time in chat, one run per method. No
document. `Deserialize` on `SuspectId`, the command and the handler list move to 10g (then split:
10g the checked function, 10h the command — 2026-09-23).
**Why.** Rule 1 schedules a consolidation stage when two ledger entries go `shaky`; four did in 10e,
and the lock itself was printed rather than typed, so nothing in 10e is `used`. Rule 4's last rung is
*change the medium*, and 10e was the second stage running where the document was what failed.
**Rejected.** 10f as planned — stacking `Deserialize`, a command and the handler list on syntax he
cannot yet type unaided. A shorter brief: the medium is the thing being changed, not the length.
**Costs.** One extra sitting before React can start an interrogation. Nothing lands in
`docs/stages/`, so the `STAGE-LOG.md` entry has to carry what was taught.

### 2026-09-21 — The phase lives in `AppState` behind a `std::sync::Mutex`; poison is an `AppError`

**Decided.** `AppState { cases_dir, phase: Mutex<Phase> }`, `phase` private, starting at `Briefing`.
Commands change it through `&self` methods on `AppState` named for the game — `begin(&self, suspect)`
first (10e). Each takes the lock, asks `Phase`, and drops the guard before it returns. A poisoned
lock becomes `AppError::Poisoned { message }` through `.map_err` — never a panic.
**Why.** `State<'_, AppState>` only ever gives `&AppState`: every command shares it and async ones run
on other threads. So a change needs a lock, and the check and the change must sit under one guard —
the double-click test. `std`'s `Mutex`, not tokio's: nothing `.await`s while holding it (Stage 15).
`ipc.rs` stays "deserialize, delegate, map errors".
**Rejected.** `.lock().unwrap()` — one panic while holding the guard makes every later command panic,
with no `kind` for React to branch on. Recovering with `PoisonError::into_inner` — right only while
every holder is panic-free, which nothing checks, and it hides the crash. A `pub` field locked from
`ipc.rs` — the lock and its error mapping repeated in every command. A private `lock_phase()` helper
returning the guard — one caller today; it comes with the second.
**Costs.** No test reaches `Poisoned`: it needs a panic while holding a private lock. Once poisoned,
every later command fails until restart (`Mutex::clear_poison` when a "new case" command exists).
Reading through the lock (`suspect()`), `record` and `finish` arrive with the commands that need them.

### 2026-09-19 — `record` takes a speaker and a `&str`; one `refusal` helper builds the error

**Decided.** `Phase::record(&mut self, speaker: Speaker, text: &str) -> AppResult<()>`, refused
outside `Interrogating` with action `"record a line"`. The `InvalidState` literal moves into a private
`Phase::refusal(&self, action: &str) -> AppError`, used by `begin`, `finish` and `record`.
**Why.** Callers hold a speaker and some text, never a ready-made `Turn`; `&str` in, `String` stored
is the Stage 3 rule he already follows. Three copies of a four-line error is where a typo in one goes
unseen — `finish`'s exact wording is pinned by no test.
**Rejected.** `record(turn: Turn)` — every call site builds the struct. `text: String` — saves one copy
nobody will measure. No helper — a four-line literal inside `record`'s `_` arm.
**Costs.** Phase 2 copies each streamed reply once into the transcript. Revisit only if a profile says so.

### 2026-09-17 — Stage 10 is six stages, and `Phase` keeps its data inside one variant

**Decided.** `transcript.rs` holds `Speaker { Detective, Suspect }`, `Turn { speaker, text }` and
`Phase { Briefing, Interrogating { suspect: SuspectId, turns: Vec<Turn> }, Reporting }`. A wrong
transition is `AppError::InvalidState` (already in `error.rs`) at run time. Split: 10a read the data
out, 10b move between phases, 10c record a line, 10d what the room shows (a consolidation stage,
inserted 2026-09-20), 10e the lock, 10f React starts an interrogation.
**Why.** "A suspect during the briefing" cannot be built, so illegal *states* do not compile. Illegal
*transitions* cannot be compile errors while the phase is one value behind one lock across IPC calls.
The suspect sits in the phase because Phase 1 plays one suspect per case; per-suspect sessions are §3.3.
**Rejected.** Typestate structs with consuming transitions — they cannot live in one `Mutex` field
without an enum around them anyway, and moving out of a lock guard needs `mem::replace`. A
`Transcript(Vec<Turn>)` newtype and a `Scored` variant — no behaviour or data for either until §2.7
and Stage 21. `Speaker::Suspect(SuspectId)` — the phase already knows who is in the room.
**Costs.** No way back from the room to the briefing until §3.3. `Reporting` holds nothing until Stage 21.
Phase 1's exit criterion reworded. 10e and 10f are counted against Rule 1 again before issuing.

### 2026-09-14 — `AppState` starts immutable, and the lock waits for Stage 10

**Decided.** Stage 9's `AppState` holds one field, `cases_dir: PathBuf`, and no lock. `Mutex` and
interior mutability move out of Phase 1 §1.5 and into **Stage 10e**, arriving with `Phase` — the
first thing in this app that changes while it runs. Stage 9 is 9a (the wire types) + 9b (the
command) + 9c (`.manage()` and `State<'_, T>`).
**Why.** A lock around a value that is only ever read teaches the syntax and none of the reason, and
the reason is the whole lesson. Sequenced this way `Mutex` shows up the first time two things want to
change one value, which is also where `MutexGuard` not crossing `.await` starts to matter (Stage 15).
**Rejected.** `AppState { cases_dir: Mutex<PathBuf> }` now, to "get the habit in early". That is the
pattern he has objected to five times: machinery first, motivation later.
**Costs.** Stage 10 grows by one topic and will itself need splitting. `ROADMAP.md` §1.5 and §1.6
amended; the "9c is where the habit forms" line now points at Stage 10.

### 2026-09-13 — What crosses to React is a screen-shaped type, built in `ipc.rs`

**Decided.** `src/ipc.rs` holds the wire types and the `#[tauri::command]` wrappers, and nothing
else: `CaseIntro { title, briefing, suspects }`, `SuspectSummary { id, name }`, and
`impl From<&Case> for CaseIntro`. `Case`, `Fact` and `Suspect` never derive `Serialize`. One file,
not the `ipc/` directory the roadmap drew — it becomes one when it holds a second command group.
**Why.** A hidden fact must have no route to the front end, and the only enforcement that survives
a tired afternoon is the absence of the trait. It is also 6a's rule pointed outward: raw types speak
the file's vocabulary, wire types speak the screen's, and `Case` in the middle speaks neither.
**Rejected.** `Serialize` on `Case` with `#[serde(skip)]` on the facts — one forgotten attribute
ships the solution, and the skip list grows a branch per screen. Also rejected: borrowing in the
wire type, since what crosses must outlive the call.
**Costs.** One small type per screen, each with a `From` and a `clone` per field. The folder was a
placeholder `const CASES_DIR` until 9c moved it into `AppState`.

### 2026-09-12 — `storage.rs` is two free functions over a `&Path`, and the slug is checked there

**Decided.** `case_path(dir: &Path, slug: &str) -> PathBuf` and `load_case(dir: &Path, slug: &str)
-> AppResult<Case>`, plus a private `is_slug`. A slug is `[a-z0-9-]+` and nothing else; anything
else is `CaseNotFound` before a path is built. A missing file is `CaseNotFound`, every other read
failure is `Io`.
**Why.** The slug reaches Rust from React in Stage 9a, so the shell boundary is also the trust
boundary — `dir.join(slug)` with an unchecked slug reads any file the process can reach. And the
front end has to tell "no such case" apart from "the disk failed": they are different screens.
**Rejected.** A `CaseStore { dir: PathBuf }` struct — the case directory belongs to `AppState` in
Stage 9c, and putting it in two places now means moving it in three weeks. Also rejected:
canonicalising the path and comparing prefixes, which is a filesystem round trip and symlink-shaped
surprises in exchange for a check a character class already makes.
**Costs.** Slugs are ASCII-only, so a case file can never be named in another script. Acceptable:
slugs are identifiers, and titles are already a separate field inside the file.

### 2026-08-29 — Cases are generated structure-first, and solvability is not a parse rule

**Decided.** Rust generates the case skeleton from a `Difficulty` and a seed — culprit, fact roles,
`known_by` distribution — and the model only writes the prose. `is_solvable` lives in `generator.rs`
and nowhere else; generated cases enter through the ordinary `parse_case` path. Scheduled as §3.6.
**Why.** A model asked for "a mystery" writes atmosphere, not a soluble one: two viable culprits, or
a culprit no visible fact points at. Structure is a constraint problem; prose is what models are for.
**Rejected.** Prompting the model for a whole TOML case and validating after — the failure is
semantic, so validation can only reject, never repair. Also rejected: ten hand-authored cases per
difficulty; two or three, as fixtures and quality bar.
**Costs.** `parse_case` keeps its four structural checks and gains nothing — solvability is the
generator's obligation about its own output, deliberately not a fifth rule. Phase 3 grows to 10–13
sessions. Nothing in Stages 6–9 changes.

### 2026-08-27 — Stage 6 is case files, and it does not touch the filesystem

**Decided.** TOML with `[[suspects]]` / `[[facts]]`, ids as plain integers, `known_by` a list of
integers, `is_ground_truth_only` optional. Raw types hold `u32` and `String`, never the id newtypes.
Three new structured `AppError` variants: `DuplicateSuspect`, `DuplicateFact`, `SuspectKnowsNothing`.
**Why.** `SuspectId` means "an id that exists in this case" — precisely the claim the file has not
earned. Ids appear on the far side of the conversion and nowhere else.
**Rejected.** One `InvalidCase { message: String }` covering all three failures — throws away the id
and hands React a sentence to regex. Also rejected: reading the file here.
**Costs.** The filesystem read moves to Stage 8 with `Io` / `CaseNotFound` and a shell-side
`storage.rs`; tests reach the two real files with `include_str!`, so `case_file.rs` stays pure.
`SuspectId` / `FactId` get `Deserialize` at the first command that takes an id as an argument —
Stage 10g, since 9's command takes a slug (renumbered 2026-09-22/23).

### 2026-08-25 — `AppError` holds owned, serializable data; `std::io::Error` never goes inside it

**Decided.** `Io { path: String, message: String }`, not `Io(#[from] std::io::Error)`. Standing rule:
**our failures are structured, foreign diagnostics are text.**
**Why.** `AppError` must be `Serialize` (it crosses IPC), `PartialEq` (tests `assert_eq!` on it) and
`Clone` (a `Session` will hold the last failure). `std::io::Error` is none of the three.
**Rejected.** `#[from]` plus a hand-written `Serialize`. It also loses on its own merits: a bare
`std::io::Error` does not know *which file* failed, so the call site must add context regardless.
**Costs.** One `.map_err` at each foreign boundary. `SuspectNotFound` carries a `SuspectId`;
`Io` / `Parse` / `Inference` carry a `String`, because we did not write those sentences.

### 2026-08-25 — The IPC wire format is `#[serde(tag = "kind", rename_all = "camelCase")]`

**Decided.** `{ "kind": "suspectNotFound", "id": 99 }`. The English sentence stays in `Display` for
logs; React branches on `kind` and writes its own copy.
**Why.** Rust owns the truth, the presentation layer owns the presentation. A UI that may restyle or
translate should not be handed a fixed English string from the backend.
**Rejected.** Putting the `Display` message on the wire.
**Costs.** Every variant must use **named fields** — internally-tagged serde cannot serialize a
newtype variant holding an integer. `SuspectNotFound(SuspectId)` compiles and fails at run time.

### 2026-08-25 — One owner for the visibility rule, enforced by a newtype

**Decided.** `facts_known_by` deleted; `Case::suspect_facts` is the single owner until the
`VisibleFact<'a>` newtype lands in **Stage 7**, after which `build_prompt` accepts only
`&[VisibleFact]`, produced solely by `Case::visible_to(..)`.
**Why.** Two implementations of one rule drifted inside a single stage. The guarantee has to live in
the *consumer's parameter type*; nothing else stops `build_prompt(&Fact)` accepting any fact.
**Rejected.** Splitting storage into `facts` + `solution`. It guards the wrong end, encodes exactly
one axis of visibility (Phase 3.5 adds difficulty-gated and pressure-released facts), and costs a
data migration for less safety.
**Costs.** `is_ground_truth_only` stays on `Fact` — visibility is data — but exactly one function is
allowed to read it. Likely becomes a two-variant enum at the same time.

### 2026-08-21 — No `crates/core` workspace split. Everything in `src-tauri`.

**Decided.** One crate. Rejected by Gabriel: *"I prefer to do everything on src-tauri and don't
optimize prematurely. This project is just to learn, won't be a production project."*
**Why it holds.** Extraction later is `git mv` + a `Cargo.toml` + fixing `use` paths — about an hour,
if the domain modules stay pure.
**Rejected argument, corrected.** The mentor's compile-time case was oversold: with incremental
compilation the loop is ~1–2 s vs ~10–20 s, not 2 s vs 2 min. The real argument was *enforcement* —
a crate boundary makes purity a compile error rather than a promise. Noted, not decisive.
**Tripwire.** If `cargo test` inside the Tauri crate turns flaky on Windows for Tauri-specific
reasons (`generate_context!` validation, `staticlib`/`cdylib` linking, `tauri::test` mocks), split
immediately without further debate. The test loop is the product in this format.
**Mitigation in force.** Domain modules carry no `tauri::` / `tokio::` / `std::fs` imports. If a
domain function ever wants an `AppHandle`, that is the signal — discuss, do not quietly reach for it.

### 2026-08-21 — Windows and Android on one engine

**Decided.** `llama-cpp-2`, compiled twice with different feature flags. Windows CUDA; Android via
the NDK with Vulkan/OpenCL or a CPU floor. Android is **Phase 2.5**, sequenced after Phase 3.
**Why / rejected.** Full reasoning in `adr/ADR-0001-cross-platform-inference.md`.

### 2026-08-21 — Package manager is bun

**Decided.** `package-lock.json` dropped, `bun.lock` in place, matching `tauri.conf.json`'s
`bun run` commands.
