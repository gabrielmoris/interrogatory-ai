# PROGRESS — where we are

Last updated: 2026-10-04 (docs audit).

|  |  |
|---|---|
| Phase | **2 — Async Rust & local LLM** (Phase 1 exit met at 10h) |
| Last reviewed | **Stage 14a**, 2026-10-03. 4/4 `engine_thread`; 167 across twenty-seven files, `fmt` and `clippy -D warnings` clean. Committed `8e12a19`. |
| Uncommitted | The docs audit: `CLAUDE.md`, `README.md`, `docs/PROGRESS.md`, `DECISIONS.md`, `ROADMAP.md`, `STAGE-LOG.md`, `adr/ADR-0001`. |
| Next action | **Mine:** write 14b in the six-section shape (model: `docs/stages/stage-14a-the-engine-gets-its-own-thread.md`). **His:** commit the audit. |
| Blocked on | nothing |

## Stage queue

Stages 1–14a ✅ — `STAGE-LOG.md`. After 14b: Stage 15, sharing across threads (`ROADMAP.md`, Phase 2).

| # | Stage | New thing | Est |
|---|---|---|---|
| 14b | `ask` goes through the engine thread | **zero new elements** — he types `channel()`, `send` and `recv` himself (recall of 14a). `AppState` holds the `Sender`; `ask` sends a `Question` and waits with `spawn_blocking` (13). Shape measured, not written (`DECISIONS.md`, 2026-09-30). | 20 |

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
