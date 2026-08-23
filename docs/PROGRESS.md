# PROGRESS — where we are

> Resume order for a fresh session: `CLAUDE.md` → this file → the current stage brief in `docs/stages/`.
> Last updated: 2026-08-23.

## Status

|            |                                                                   |
| ---------- | ----------------------------------------------------------------- |
| Phase      | **1 — Rust core & Tauri foundations**                             |
| Stage      | **3 issued** (`Fact` / `Suspect` / `Case`) — awaiting implementation |
| Next       | Stage 4 — borrowing and lifetimes (`suspect_facts` returning refs)   |
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

### Stage 3 — `Fact`, `Suspect` and `Case` — issued 2026-08-23, in progress

Spec at `src-tauri/tests/case.rs` (9 tests), brief at `docs/stages/stage-03-case.md`.
Implementation goes in `src-tauri/src/case.rs`.

To build: `Suspect { id, name: String }`; `Fact { id, statement: String,
known_by: HashSet<SuspectId>, is_ground_truth_only: bool }` with `reveal_to(&mut self)` /
`is_known_by(&self)`; `Case { title, briefing, suspects: Vec<Suspect>, facts: Vec<Fact> }` with the
two collections **private**, `add_suspect` / `add_fact` / `suspect_count` / `fact_count`, and
`facts_known_by(&self, SuspectId) -> Vec<FactId>` filtering out `is_ground_truth_only`.

Concepts targeted: `String` vs `&str` (structs own, parameters borrow) and why a struct cannot
store the `&str` it was handed; `Vec<T>` vs Stage 1's fixed array; `HashSet` as the right shape for
a membership question, cashing in Stage 2's `Hash`/`Eq` derives; the three receivers
(`&self` / `&mut self` / `self`) and `let mut` at the call site; `E0204` — why `Copy` is now
impossible and `Clone` is not free; `E0382` on moving a `Fact` into the case.

Design decisions recorded in the brief: `Fact`/`Suspect` fields public (no invariant of their own),
`Case`'s collections private (they will carry the "every referenced id exists" invariant from Stage
5). `known_by` is a `HashSet`, not the `Vec` sketched in `ROADMAP.md` §1.1 — dedupe and O(1)
membership, and the ROADMAP line predates the id newtypes.

Deliberately excluded, to keep the stage to one theme: `Option`, explicit lifetimes, and anything
returning a reference. `facts_known_by` returns owned `Vec<FactId>` precisely so that Stage 4 can
upgrade it to `suspect_facts<'a>(&'a self, ...) -> impl Iterator<Item = &'a Fact>` and make the
lifetime the whole subject.

Written at the depth agreed on 2026-08-22 — built from zero, one concept per section, with
checkpoints telling him to run the test and come back rather than read straight through.

Spec verified against a reference implementation in a throwaway crate before issuing: 9/9 passing,
`clippy --all-targets -D warnings` clean, `cargo fmt --check` clean. `E0382` confirmed to be the
error the commented-out line produces.

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
