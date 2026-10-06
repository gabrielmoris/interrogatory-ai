# PROGRESS — where we are

Last updated: 2026-10-07 (14b doc rewritten after correction 27).

|  |  |
|---|---|
| Phase | **2 — Async Rust & local LLM** (Phase 1 exit met at 10h) |
| Last reviewed | **Stage 14a**, 2026-10-03. 4/4 `engine_thread`; 167 across twenty-seven files, `fmt` and `clippy -D warnings` clean. Committed `8e12a19`. |
| Uncommitted | His `state.rs` (14b in progress). The rewritten 14b doc, `CLAUDE.md` (Rules 5, 8, 10), this file, `DECISIONS.md`. |
| Next action | **His:** finish `ask` — doc §3, blocks 4–7 (`new` and blocks 1–2 are done). **Mine:** review when he says ready. |
| Blocked on | nothing |

## Stage queue

Stages 1–14a ✅ — `STAGE-LOG.md`. After 14b: Stage 15, sharing across threads (`ROADMAP.md`, Phase 2).

| # | Stage | New thing | Est |
|---|---|---|---|
| 14b | `ask` goes through the engine thread | **zero new Rust** — he types `channel()`, `send` and `recv()` (14a) and `spawn_blocking` (13). One new shape: three nested `Result`s, one `?` per line. Spec `tests/ask_engine_thread.rs`, 3 tests (1/3 before); reference measured 170 across twenty-eight files, `fmt` and `clippy -D warnings` clean. Doc issued 2026-10-04; rewritten 2026-10-07 with the map in §1 and the code shown in each block (correction 27). `ask`'s input renamed `text`. | 25 |

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
