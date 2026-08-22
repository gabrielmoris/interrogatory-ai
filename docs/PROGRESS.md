# PROGRESS — where we are

> Resume order for a fresh session: `CLAUDE.md` → this file → the current stage brief in `docs/stages/`.
> Last updated: 2026-08-22.

## Status

| | |
|---|---|
| Phase | **1 — Rust core & Tauri foundations** |
| Stage | **2 complete** (`SuspectId` / `FactId`) — reviewed, 10/10 passing |
| Next | Stage 3 — `Fact` (with `known_by`) and `Case`. Not yet written. |
| Blocked on | nothing |

---

## Decisions log

Newest first. Record the *why*, not just the what.

### 2026-08-22 — rustfmt owns `.rs`; Prettier is fenced out.
Rust has one formatter and its default output is the community style — `rustfmt.toml` exists but
most of its options are nightly-only, so we keep defaults and configure nothing.

Added `.vscode/settings.json` (format-on-save via `rust-analyzer`, `linkedProjects` pointed at
`src-tauri/Cargo.toml` since the crate is not at the repo root, and `check.command = clippy` with
`--all-targets -- -D warnings` so the stage-exit bar shows up while typing) and `.prettierignore`
excluding `src-tauri/` and `*.rs` ahead of Prettier arriving for the React side in Phase 4.

**Cause confirmed:** the `jinxdash.prettier-rust` VS Code extension (Prettier + `prettier-plugin-rust`)
was formatting `.rs` on save. Proven by running that plugin over the repo's files — `difficulty.rs`
and `lib.rs` are byte-exact fixed points of it. It disagrees with rustfmt in at least two places:
it breaks after `=>` before a struct literal, and it treats `::` as a member chain
(`tauri::Builder\n    ::default()`).

Note the plugin is effectively abandoned: v0.1.9 crashes under Prettier 3 (`Unexpected doc.type
'concat'`) and only runs on Prettier 2.

Gabriel to uninstall/disable the extension; the workspace `[rust]` formatter override covers this
repo regardless, since workspace settings beat user settings. `cargo fmt` still needs running over
`difficulty.rs` and `lib.rs` *after* the extension is off.

Diagnostic misstep worth remembering: the mentor first concluded "nothing is formatting" because
`src/ids.rs` was rustfmt-clean. Invalid — `ids.rs` is simple enough that rustfmt and prettier-rust
produce identical output, so it was a fixed point of both and proved nothing. Test the suspected
tool directly instead of reasoning from a file that both candidates agree on.

### 2026-08-21 — No `crates/core` workspace split. Everything in `src-tauri`.
Proposed and **rejected by Gabriel**: "I prefer to do everything on src-tauri and don't optimize
prematurely. This project is just to learn, won't be a production project."

He is right that it is reversible: if the domain modules stay pure, extraction later is `git mv` +
a `Cargo.toml` + fixing `use` paths — roughly an hour, not a weekend.

The mentor's original argument was oversold and was corrected: with incremental compilation the
loop is ~1–2s vs ~10–20s, not 2s vs 2min. The remaining real argument was *enforcement* (a crate
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
Android is **Phase 2.5**, sequenced *after* Phase 3.

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
what an id *means* rather than what a trait *is*.

**Teaching-format correction, 2026-08-22.** Mid-stage he said: *"the way you explain is like if I
were writing Rust for years... go slowly."* The written brief and its progressive hints were
pitched too high, and dumping all remaining errors at once made it worse. What worked instead:
one concept per reply, built from zero (what a trait is → what `derive` generates → ownership and
moves → `impl Trait for Type` → `From`), each ending with a single command to run and a checkpoint
to come back to. `CLAUDE.md` and [[explanation-depth]] updated. Write Stage 3's brief at that
level from the start.

Notable moment: his first `Display` impl compiled but printed `"3 suspect #"`. The compiler had
nothing to say about it; the test caught it. Good place to have landed the shapes-vs-behaviour
distinction.

Spec was verified against a reference implementation in a throwaway crate before issuing.

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
2. **Next:** `Fact` with visibility data (`known_by`), and `Case` holding suspects and facts.
   First encounter with `String` vs `&str`, and with `Vec` / `HashSet` as struct fields.
3. Borrowing and lifetimes: `fn suspect_facts<'a>(&'a Case, SuspectId) -> impl Iterator<Item = &'a Fact>`.
4. `AppError` with `thiserror`, and `Result` across the IPC boundary.
5. Parse-don't-validate: `RawCase` → `TryFrom` → `Case`, loading a real TOML case file.
6. `tauri::State` and managed app state — the first stage that touches the Tauri shell.
