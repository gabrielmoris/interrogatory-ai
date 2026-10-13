# PROGRESS — where we are

Last updated: 2026-10-14 (Stage 16b §3 rewritten — correction 29).

|  |  |
|---|---|
| Phase | **2 — Async Rust & local LLM** (Phase 1 exit met at 10h) |
| Last reviewed | **Stage 16a**, 2026-10-13. 5/5 `pieces`; 175 across twenty-nine files, `fmt` and `clippy -D warnings` clean. Passed first submission. Committed `3d8cb75`. |
| Uncommitted | Stage 16b: `docs/stages/stage-16b-each-piece-travels-to-ask.md`, `tests/pieces_travel.rs` (new), `tests/engine_thread.rs` and `tests/pieces.rs` (changed to `Said`), his §2 paste in `engine_thread.rs` / `state.rs`, `CLAUDE.md` (correction 29, Rule 5), `DECISIONS.md`, this file. |
| Next action | **His:** Stage 16b §3 — `docs/stages/stage-16b-each-piece-travels-to-ask.md`. §2 is done. Steps A–E: two Type it (the closure, the loop), four small Build it. Then say ready. |
| Blocked on | nothing |

## Stage queue

Stages 1–16a ✅ — `STAGE-LOG.md`.

| # | Stage | New thing | Est |
|---|---|---|---|
| 16b | each piece travels to `ask` | `loop` / `break` | cut, ~45 min. Spec `pieces_travel.rs` (4); 176 measured |
| 16c | React shows the pieces | Tauri's `ipc::Channel` | not cut |

## Open, not blocking

- [ ] `llm.rs :: InferenceEngine::reply` — the doc comment lost its second line, the one saying what
      `Ok(())` and `Err` mean now that the text goes out through `say`. Tidying.

- [ ] Doc comments: `ipc.rs :: case_intro_from` and `case_intro` have none. `parse_case`'s says
      "but it it fails"; `RawCase`'s says only "Raw case". Tidying.
- [ ] `#[serde(default)]` on `RawCase::facts` should come off — `suspects` is required and both are
      the same kind of thing. Changes no test.
- [ ] Template leftovers: `greetMsg` in `App.tsx`; the logos `public/vite.svg`, `public/tauri.svg`,
      `src/assets/react.svg`; the `<title>` and icon in `index.html`.
- [ ] Add `rust-toolchain.toml` pinning a stable version.
- [ ] `withGlobalTauri` was turned on in 9b for the console demo. Turn it off before any release
      build — it exposes the whole API to any script in the page.
