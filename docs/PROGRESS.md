# PROGRESS — where we are

Last updated: 2026-10-07 (Stage 14b reviewed; 14c issued).

|  |  |
|---|---|
| Phase | **2 — Async Rust & local LLM** (Phase 1 exit met at 10h) |
| Last reviewed | **Stage 14b**, 2026-10-07. 3/3 `ask_engine_thread` (10 repeat runs); 170 across twenty-eight files, `fmt` and `clippy -D warnings` clean. Committed `4452c29`. |
| Uncommitted | This file, `STAGE-LOG.md` (the 14b review), `docs/stages/stage-14c-see-what-ask-does.md`, the 14b doc's last line. |
| Next action | **His:** Stage 14c — `docs/stages/stage-14c-see-what-ask-does.md`. **Mine:** review when he says ready, then cut Stage 15. |
| Blocked on | nothing |

## Stage queue

Stages 1–14b ✅ — `STAGE-LOG.md`.

| # | Stage | New thing | Est |
|---|---|---|---|
| 14c | see what `ask` does | consolidation, no feature. He picked three unclear parts of 14b: who holds what, waiting (`spawn_blocking` + `.await`), the three `Result`s. Three `E0382` experiments, then `dbg!` in the real tests. New tool: `dbg!`. All output measured on the reference. | 30 |
| 15 | sharing across threads | not cut yet — `ROADMAP.md` Phase 2 lists `Arc<Mutex<T>>` and why a lock must be let go before `.await`. Must pass Rule 2; likely splits. | — |

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
