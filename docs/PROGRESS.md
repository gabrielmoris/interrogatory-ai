# PROGRESS — where we are

Last updated: 2026-10-12 (Stage 16a issued).

|  |  |
|---|---|
| Phase | **2 — Async Rust & local LLM** (Phase 1 exit met at 10h) |
| Last reviewed | **Stage 15**, 2026-10-12. 4/4 `engine_thread`, 3/3 `ask_engine_thread`; 170 across twenty-eight files, `fmt` and `clippy -D warnings` clean. Passed first submission. Committed `c189e3c`. |
| Uncommitted | Stage 16a: `tests/pieces.rs` (new), the test engines in four test files, the stage doc, `DECISIONS.md`, `ROADMAP.md`, this file. |
| Next action | **His:** Stage 16a — `docs/stages/stage-16a-the-suspect-speaks-in-pieces.md`. Then say ready. |
| Blocked on | nothing |

## Stage queue

Stages 1–15 ✅ — `STAGE-LOG.md`.

| # | Stage | New thing | Est |
|---|---|---|---|
| 16a | the suspect speaks in pieces | a function as an input: `say: &mut dyn FnMut(&str)` | 30 — spec `tests/pieces.rs` (5), measured 175 |
| 16b | each piece travels to `ask` | `loop` until the last message | not cut |
| 16c | React shows the pieces | Tauri's `ipc::Channel` | not cut |

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
