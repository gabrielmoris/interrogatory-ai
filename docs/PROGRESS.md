# PROGRESS — where we are

Last updated: 2026-10-04 (Stage 14b issued).

|  |  |
|---|---|
| Phase | **2 — Async Rust & local LLM** (Phase 1 exit met at 10h) |
| Last reviewed | **Stage 14a**, 2026-10-03. 4/4 `engine_thread`; 167 across twenty-seven files, `fmt` and `clippy -D warnings` clean. Committed `8e12a19`. |
| Uncommitted | Stage 14b's doc and spec: `docs/stages/stage-14b-ask-goes-through-the-engine-thread.md`, `src-tauri/tests/ask_engine_thread.rs`. The docs audit is in `542616a`. |
| Next action | **His:** Stage 14b — `docs/stages/stage-14b-ask-goes-through-the-engine-thread.md`. **Mine:** review when he says ready. |
| Blocked on | nothing |

## Stage queue

Stages 1–14a ✅ — `STAGE-LOG.md`. After 14b: Stage 15, sharing across threads (`ROADMAP.md`, Phase 2).

| # | Stage | New thing | Est |
|---|---|---|---|
| 14b | `ask` goes through the engine thread | **zero new Rust** — he types `channel()`, `send` and `recv()` (14a) and `spawn_blocking` (13). One new shape: three nested `Result`s, one `?` per line. Spec `tests/ask_engine_thread.rs`, 3 tests (1/3 before); reference measured 170 across twenty-eight files, `fmt` and `clippy -D warnings` clean. Doc issued 2026-10-04. | 25 |

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
