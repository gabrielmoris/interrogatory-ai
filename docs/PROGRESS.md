# PROGRESS — where we are

Last updated: 2026-10-19 (Stage 17a reviewed).

|  |  |
|---|---|
| Phase | **2 — Async Rust & local LLM** (Phase 1 exit met at 10h) |
| Last reviewed | **Stage 17a**, 2026-10-19. 4/4 `no_one_listens` (20 repeat runs); 184 across thirty-two files, `fmt` and `clippy -D warnings` clean. `src/` identical to the reference. Passed first submission. |
| Uncommitted | All of Stage 17a — his `llm.rs` and `engine_thread.rs`, the stage doc, `tests/no_one_listens.rs` and the seven test files I changed — plus the 16c and 17a review notes in `STAGE-LOG.md`, `DECISIONS.md`, `ROADMAP.md`, this file. One commit. |
| Next action | **His:** commit Stage 17a. **Mine:** cut Stage 17b — the player presses Stop (`CancellationToken`). The plan and the prototype are in `DECISIONS.md`, 2026-10-18. |
| Blocked on | nothing |

## Stage queue

Stages 1–17a ✅ — `STAGE-LOG.md`.

| # | Stage | New thing | Est |
|---|---|---|---|
| 17b | the player presses Stop | `CancellationToken` | planned, prototyped (`DECISIONS.md`, 2026-10-18) |
| 17c | Stop works while the suspect is silent | `select!` | planned, prototyped |

## Open, not blocking

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
