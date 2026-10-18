# PROGRESS — where we are

Last updated: 2026-10-14 (Stage 16c cut).

|  |  |
|---|---|
| Phase | **2 — Async Rust & local LLM** (Phase 1 exit met at 10h) |
| Last reviewed | **Stage 16b**, 2026-10-14. 4/4 `pieces_travel`; 176 across thirty files, `fmt` and `clippy -D warnings` clean. Identical to the reference except blank lines. |
| Uncommitted | 16b's review notes in `STAGE-LOG.md`. All of Stage 16c so far: `docs/stages/stage-16c-react-shows-the-pieces.md`, `tests/pieces_to_react.rs` (new), `tests/ask_command.rs` and six test files that call `ask` (`app_engine`, `ask_async`, `ask_engine_thread`, `pieces`, `pieces_travel`, `room_exchange`); `CLAUDE.md` (30: Fill it, Order it); `DECISIONS.md`, `ROADMAP.md`, this file. |
| Next action | **His:** Stage 16c — `docs/stages/stage-16c-react-shows-the-pieces.md`. §2 pastes (four places), then steps A–D: Build it, Fill it, Order it, Type it (React). Then say ready. |
| Blocked on | nothing |

## Stage queue

Stages 1–16b ✅ — `STAGE-LOG.md`.

| # | Stage | New thing | Est |
|---|---|---|---|
| 16c | React shows the pieces | Tauri's `ipc::Channel`, Rust and TypeScript sides | cut, ~40 min. Spec `pieces_to_react.rs` (4); 180 measured |
| 17 | the player stops the suspect mid-sentence | to be settled when cut (`ROADMAP.md`) | not cut |

## Open, not blocking

- [ ] `llm.rs :: InferenceEngine::reply` — the doc comment lost its second line, the one saying what
      `Ok(())` and `Err` mean now that the text goes out through `say`. Tidying.

- [ ] Doc comments: `ipc.rs :: case_intro_from` and `case_intro` have none. `parse_case`'s says
      "but it it fails"; `RawCase`'s says only "Raw case". Tidying.
- [ ] `#[serde(default)]` on `RawCase::facts` should come off — `suspects` is required and both are
      the same kind of thing. Changes no test.
- [ ] Template leftovers: the logos `public/vite.svg`, `public/tauri.svg`, `src/assets/react.svg`
      (nothing uses them once 16c replaces `App.tsx`), the logo styles in `App.css`, and the `<title>`
      and icon in `index.html`.
- [ ] Add `rust-toolchain.toml` pinning a stable version.
- [ ] `withGlobalTauri` was turned on in 9b for the console demo. Turn it off before any release
      build — it exposes the whole API to any script in the page.
