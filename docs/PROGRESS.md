# PROGRESS — where we are

Last updated: 2026-10-08 (Stage 15 issued).

|  |  |
|---|---|
| Phase | **2 — Async Rust & local LLM** (Phase 1 exit met at 10h) |
| Last reviewed | **Stage 14c**, 2026-10-07. Consolidation; `ask` input renamed `text`; `dbg!` lines removed. 170 across twenty-eight files, `fmt` and `clippy -D warnings` clean. Committed `6ccd236`. |
| Uncommitted | Stage 15: its doc, the two changed specs (`tests/engine_thread.rs`, `tests/ask_engine_thread.rs`), `DECISIONS.md`, `ROADMAP.md`, this file, the 14c doc's last line. |
| Next action | **His:** Stage 15 — `docs/stages/stage-15-the-answer-can-be-awaited.md`. **Mine:** review when he says ready. |
| Blocked on | nothing |

## Stage queue

Stages 1–14c ✅ — `STAGE-LOG.md`.

| # | Stage | New thing | Est |
|---|---|---|---|
| 15 | the answer can be awaited | **an async channel** (`tauri::async_runtime::channel(1)`, `blocking_send`, `recv().await` → `Option`). Pipe 2 only; `spawn_blocking` leaves `ask`. Specs: `tests/engine_thread.rs` (4) and `tests/ask_engine_thread.rs` (3), both changed; before: does not compile (5 × `E0308`). Reference measured 170, `fmt`/`clippy` clean. Re-cut 2026-10-08 (`DECISIONS.md`): `Arc` moves to 17. | 30 |
| 16 | the reply comes in pieces | not cut yet — streaming to React (`ROADMAP.md` §2.4) | — |

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
