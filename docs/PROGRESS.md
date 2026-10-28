# PROGRESS — where we are

Last updated: 2026-10-19 (Stage 17a committed `db99f2f`; Stage 17b cut).

|  |  |
|---|---|
| Phase | **2 — Async Rust & local LLM** (Phase 1 exit met at 10h) |
| Last reviewed | **Stage 17a**, 2026-10-19. 4/4 `no_one_listens` (20 repeat runs); 184 across thirty-two files, `fmt` and `clippy -D warnings` clean. `src/` identical to the reference. Passed first submission. |
| Uncommitted | Stage 17b's doc, `tests/press_stop.rs`, and the 17b notes in `DECISIONS.md` and this file. Commit them with your 17b code. |
| Next action | **His:** Stage 17b — `docs/stages/stage-17b-the-player-presses-stop.md`. **Mine:** review 17b, then cut 17c. |
| Blocked on | nothing |

## Stage queue

Stages 1–17a ✅ — `STAGE-LOG.md`.

| # | Stage | New thing | Est |
|---|---|---|---|
| 17b | the player presses Stop | `CancellationToken` | cut 2026-10-19, measured (`DECISIONS.md`); awaiting him |
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
