# DECISIONS — architecture, newest first

A lookup, not a read-through. Architecture only — teaching-process changes live in `MENTOR-NOTES.md`.
Per entry: decided / why / rejected / costs. If an entry needs more, it was two decisions.

---

### 2026-09-17 — Stage 10 is five stages, and `Phase` keeps its data inside one variant

**Decided.** `transcript.rs` holds `Speaker { Detective, Suspect }`, `Turn { speaker, text }` and
`Phase { Briefing, Interrogating { suspect: SuspectId, turns: Vec<Turn> }, Reporting }`. A wrong
transition is `AppError::InvalidState` (already in `error.rs`) at run time. Split: 10a read the data
out, 10b move between phases, 10c record a line, 10d the lock, 10e React starts an interrogation.
**Why.** "A suspect during the briefing" cannot be built, so illegal *states* do not compile. Illegal
*transitions* cannot be compile errors while the phase is one value behind one lock across IPC calls.
The suspect sits in the phase because Phase 1 plays one suspect per case; per-suspect sessions are §3.3.
**Rejected.** Typestate structs with consuming transitions — they cannot live in one `Mutex` field
without an enum around them anyway, and moving out of a lock guard needs `mem::replace`. A
`Transcript(Vec<Turn>)` newtype and a `Scored` variant — no behaviour or data for either until §2.7
and Stage 21. `Speaker::Suspect(SuspectId)` — the phase already knows who is in the room.
**Costs.** No way back from the room to the briefing until §3.3. `Reporting` holds nothing until Stage 21.
Phase 1's exit criterion reworded. 10d and 10e are counted against Rule 1 again before issuing.

### 2026-09-14 — `AppState` starts immutable, and the lock waits for Stage 10

**Decided.** Stage 9's `AppState` holds one field, `cases_dir: PathBuf`, and no lock. `Mutex` and
interior mutability move out of Phase 1 §1.5 and into **Stage 10d**, arriving with `Phase` — the
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
Stage 10e, since 9's command takes a slug.

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
