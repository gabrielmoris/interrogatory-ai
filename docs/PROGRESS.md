# PROGRESS — where we are

Last updated: 2026-10-07 (Stage 14c reviewed).

|  |  |
|---|---|
| Phase | **2 — Async Rust & local LLM** (Phase 1 exit met at 10h) |
| Last reviewed | **Stage 14c**, 2026-10-07. Consolidation; `ask` input renamed `text`. 170 across twenty-eight files, `fmt` and `clippy -D warnings` clean. Three `dbg!` lines still to delete. |
| Uncommitted | His `state.rs` (14c: rename + the `dbg!` lines), this file, `STAGE-LOG.md`. |
| Next action | **His:** delete the three `dbg!` lines in `ask`, then commit. **Mine:** cut Stage 15 once he says the picture is clear. |
| Blocked on | nothing |

## Stage queue

Stages 1–14c ✅ — `STAGE-LOG.md`.

| # | Stage | New thing | Est |
|---|---|---|---|
| 15 | sharing across threads | not cut yet — `ROADMAP.md` Phase 2 lists `Arc<Mutex<T>>` and why a lock must be let go before `.await`. Must pass Rule 2; likely splits. | — |

## Open, not blocking

- [ ] Doc comments: `ipc.rs :: case_intro_from` and `case_intro` have none. `parse_case`'s says
      "but it it fails"; `RawCase`'s says only "Raw case". Tidying.
- [ ] `#[serde(default)]` on `RawCase::facts` should come off — `suspects` is required and both are
      the same kind of thing. Changes no test.
- [ ] Template leftovers: `greetMsg` in `App.tsx`; the logos `public/vite.svg`, `public/tauri.svg`,
      `src/assets/react.svg`; the `<title>` and icon in `index.html`.
- [ ] Add `rust-toolchain.toml` pinning a stable version.
- [ ] `withGlobalTauri` was turned on in 9b for the console demo. Turn it off before any release
      build — it exposes the whole API to any script in the page.
