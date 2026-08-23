# PROGRESS — where we are

> Resume order for a fresh session: `CLAUDE.md` → this file → the current stage brief in `docs/stages/`.
> Last updated: 2026-08-23.

## Status

|            |                                                                   |
| ---------- | ----------------------------------------------------------------- |
| Phase      | **1 — Rust core & Tauri foundations**                             |
| Stage      | **3 complete** (`Fact` / `Suspect` / `Case`) — reviewed, 9/9 passing |
| Next       | Stage 4 — borrowing and lifetimes. Not yet written.                  |
| Blocked on | nothing                                                           |

---

## Decisions log

Newest first. Record the _why_, not just the what.

### 2026-08-21 — No `crates/core` workspace split. Everything in `src-tauri`.

Proposed and **rejected by Gabriel**: "I prefer to do everything on src-tauri and don't optimize
prematurely. This project is just to learn, won't be a production project."

He is right that it is reversible: if the domain modules stay pure, extraction later is `git mv` +
a `Cargo.toml` + fixing `use` paths — roughly an hour, not a weekend.

The mentor's original argument was oversold and was corrected: with incremental compilation the
loop is ~1–2s vs ~10–20s, not 2s vs 2min. The remaining real argument was _enforcement_ (a crate
boundary makes purity a compile error rather than a promise) — noted, not decisive.

**Tripwire for revisiting:** if `cargo test` inside the Tauri crate turns flaky on Windows for
Tauri-specific reasons (`generate_context!` config validation, `staticlib`/`cdylib` crate-type
linking, `tauri::test` mock issues), split immediately without further debate. The test loop is
the product in this format.

**Mitigation in force:** domain modules carry no `tauri::` / `tokio::` / `std::fs` imports.
If a scoring or domain function ever wants an `AppHandle`, that is the signal — discuss, do not
quietly reach for it.

### 2026-08-21 — Windows + Android, one engine.

`llama-cpp-2` compiled twice with different feature flags. Windows CUDA; Android via NDK with
Vulkan/OpenCL or CPU floor. Full reasoning: `docs/adr/ADR-0001-cross-platform-inference.md`.
Android is **Phase 2.5**, sequenced _after_ Phase 3.

### 2026-08-21 — Package manager: bun.

`package-lock.json` dropped, `bun.lock` in place, matches `tauri.conf.json`'s `bun run` commands.

---

## Stage log

### Stage 1 — `Difficulty` and `Tuning` ✅

`src-tauri/src/difficulty.rs`, spec at `src-tauri/tests/difficulty.rs`, brief at
`docs/stages/stage-01-difficulty.md`.

Built: four-variant `Difficulty` enum, `Tuning` payload struct, `Difficulty::ALL` associated
constant, `tuning()` returning per-difficulty settings.

Concepts landed: Rust's module system (`pub mod`, files are invisible until declared); `struct`
vs `enum` as "payload" vs "which one"; field-level `pub`; narrow integer types (`u8`) as a way to
make nonsense unrepresentable; `impl` blocks; associated constants; fixed-size arrays vs `Vec`;
`match` as an expression; **traits and `derive`**; and the headline lesson — `E0507 cannot move
out of *d which is behind a shared reference`, i.e. **move semantics and `Copy`**, plus why
`Difficulty` should be `Copy` and `Tuning` should not.

Hints reached: got to the `impl` skeleton with guidance; needed the array-literal correction
(wrote TS object-literal syntax `Easy: "easy"` inside `[...]`).

Review outcome: **pass**, with a polish pass requested — derives on `Tuning`, `Self` inside the
`impl` block, and doc comments on public items.

### Stage 2 — `SuspectId` and `FactId` ✅

`src-tauri/src/ids.rs`, spec at `src-tauri/tests/ids.rs`, brief at
`docs/stages/stage-02-ids.md`.

Built: two tuple structs wrapping a **private** `u32`, each with `new` / `get`, eight derives,
and hand-written `Display` and `From<u32>`.

Concepts targeted: the newtype pattern (vs bare `u32`, vs `String`, vs TS branded types); tuple
structs and field-level privacy as a real escape-hatch-free boundary; **derive vs hand-written
`impl`** and why std ships no `derive(Display)`; the `Debug`/`Display` split; the trait hierarchy
(`Copy: Clone`, `Eq: PartialEq`, `Ord: PartialOrd + Eq`); the `Hash`/`Eq` consistency contract and
why `f32` can never be a map key; **blanket impls** as the source of `to_string()` and `.into()`;
the orphan rule. Optional extra: `compile_fail` doctests as type-level tests.

Instruction given: add derives **one at a time**, driven by the top compiler error. `E0507` returns
in a new costume via `.iter().map(|f| f.get())`.

Hints reached: needed the concepts unpacked in-chat rather than the `<details>` hints — see the
teaching-format note below. Wrote all four impl blocks himself.

Review outcome: **pass** — 10/10, `clippy --all-targets -D warnings` clean, `cargo fmt` clean.
Polish requested: `Self(id)` instead of `SuspectId(id)` inside the impl blocks (third time this
note has come up), and doc comments moved off the `impl` blocks onto the two public types, saying
what an id _means_ rather than what a trait _is_.

**Teaching-format correction, 2026-08-22.** Mid-stage he said: _"the way you explain is like if I
were writing Rust for years... go slowly."_ The written brief and its progressive hints were
pitched too high, and dumping all remaining errors at once made it worse. What worked instead:
one concept per reply, built from zero (what a trait is → what `derive` generates → ownership and
moves → `impl Trait for Type` → `From`), each ending with a single command to run and a checkpoint
to come back to. `CLAUDE.md` and [[explanation-depth]] updated. Write Stage 3's brief at that
level from the start.

Notable moment: his first `Display` impl compiled but printed `"3 suspect #"`. The compiler had
nothing to say about it; the test caught it. Good place to have landed the shapes-vs-behaviour
distinction.

Spec was verified against a reference implementation in a throwaway crate before issuing.

### Stage 3 — `Fact`, `Suspect` and `Case` ✅

`src-tauri/src/case.rs`, spec at `src-tauri/tests/case.rs`, brief at
`docs/stages/stage-03-case.md`.

Built: `Suspect { id, name: String }`; `Fact { id, statement: String, known_by: HashSet<SuspectId>,
is_ground_truth_only }` with `reveal_to(&mut self)` / `is_known_by(&self)`; `Case` with `title` /
`briefing` public and `suspects` / `facts` **private**, plus `add_*` / `*_count` and
`facts_known_by(&self, SuspectId) -> Vec<FactId>`.

Concepts landed: `String` vs `&str` — structs own, parameters borrow, the constructor converts, and
why a struct cannot store the `&str` it was handed; `Vec<T>` vs Stage 1's fixed array; `HashSet` as
the right shape for a membership question, cashing in Stage 2's `Hash`/`Eq` derives; **`self` is an
explicit parameter** — `&self` reads, `&mut self` writes, and omitting it silently gives you an
associated function rather than a method; no uninitialized fields in Rust, so "I don't have that
data yet" always resolves to "what is the empty value"; `E0204` — why `Copy` is impossible once a
`String` is in the struct, and why `Clone` is not free; `E0382` on moving a `Fact` into the case;
closures and `.filter().map().collect()`.

Design decisions recorded in the brief: `Fact`/`Suspect` fields public (no invariant of their own),
`Case`'s collections private (they carry the "every referenced id exists" invariant from Stage 5).
`known_by` is a `HashSet`, overriding the `Vec` sketched in `ROADMAP.md` §1.1. `facts_known_by`
returns owned `Vec<FactId>` on purpose, so Stage 4 can upgrade it to the iterator-of-references
version and make the lifetime the whole subject.

Hints reached: needed the full signature skeleton handed over (Hint 5's shape) after trying and
failing to derive signatures from the test's call sites — see the teaching note below. Wrote every
body himself, and reached for the iterator chain in `facts_known_by` unprompted rather than the
loop.

Review outcome: **pass** — 9/9, `clippy --all-targets -D warnings` clean, `cargo fmt` clean. Doc
comments present on all three types and saying what the type *means*; that polish note, open since
Stage 1, is now closed. `Self` used throughout without prompting.

Notable moment: with `!` missing from the ground-truth condition, **both** visibility tests failed
in opposite directions — one returned everything, the other returned nothing. Used to teach the
debugging tell: every case inverted rather than some cases wrong means a missing negation.

**Teaching notes, 2026-08-23.**
1. Asking him to read signatures off the test's call sites was a step too far and cost several
   rounds — he guessed instead (`Fact::new` taking every field, `is_known_by` with no `self`,
   `statement` missing entirely). From Stage 4 on, **issue the signature skeleton with the brief**,
   bodies elided. Shapes are scaffolding; bodies are the exercise.
2. Teach `todo!()` before Stage 4. He stubbed with `-> Self {}` and got a wall of `E0308`;
   `todo!()` type-checks as anything, so a stubbed file links and he gets a red suite to drive
   instead of a build error.
3. Jargon, not just depth: "one boolean in your filter closure" did not parse, though he had
   written the closure himself. Name Rust vocabulary against its TS equivalent on first use.

Spec was verified against a reference implementation in a throwaway crate before issuing, and his
implementation re-verified in the same crate at review.

---

## Phase 0 leftovers (deferred, not forgotten)

- [x] `.gitattributes` + line-ending renormalization (`940912b`)
- [x] `/target`, `models/`, `*.gguf` ignored
- [x] Package manager settled on bun
- [x] **`docs` is still listed in `.gitignore`** — every planning document, ADR and stage brief is
      therefore untracked and will not survive a fresh clone. Remove that line.
- [ ] Delete the `greet` demo command, the template `App.tsx`, `public/vite.svg`,
      `src/assets/react.svg`
- [ ] Rewrite `README.md` — it still describes the Tauri template
- [ ] Add `rust-toolchain.toml` pinning a stable version
- [ ] Delete the orphaned root `target/` directory (left over from the reverted workspace)

---

## What comes next

Phase 1 continues, roughly in this order — each becomes one stage with its own failing test:

1. ~~`SuspectId` / `FactId` newtypes~~ — done, Stage 2.
2. ~~`Fact` with `known_by`, and `Case` holding suspects and facts~~ — issued, Stage 3.
3. **Next:** borrowing and lifetimes: `fn suspect_facts<'a>(&'a Case, SuspectId) -> impl Iterator<Item = &'a Fact>`.
4. `AppError` with `thiserror`, and `Result` across the IPC boundary.
5. Parse-don't-validate: `RawCase` → `TryFrom` → `Case`, loading a real TOML case file.
6. `tauri::State` and managed app state — the first stage that touches the Tauri shell.
