# PROGRESS — where we are

Last updated: 2026-10-13 (Stage 16a reviewed; correction 28).

|  |  |
|---|---|
| Phase | **2 — Async Rust & local LLM** (Phase 1 exit met at 10h) |
| Last reviewed | **Stage 16a**, 2026-10-13. 5/5 `pieces`; 175 across twenty-nine files, `fmt` and `clippy -D warnings` clean. Passed first submission. Committed `3d8cb75`. |
| Uncommitted | `CLAUDE.md` (correction 28: Rules 5 and 8, Build it / Type it), `STAGE-LOG.md`, this file. |
| Next action | **Mine:** cut Stage 16b with the new steps — the bodies made of known pieces are his to build (Rule 5). |
| Blocked on | nothing |

## Stage queue

Stages 1–16a ✅ — `STAGE-LOG.md`.

| # | Stage | New thing | Est |
|---|---|---|---|
| 16b | each piece travels to `ask` | `loop` until the last message | not cut |
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
