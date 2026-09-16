# PROGRESS — where we are

Last updated: 2026-09-17.

|  |  |
|---|---|
| Phase | **1 — Rust core & Tauri foundations** |
| Last reviewed | **Stage 9b**, 2026-09-15. 11/11; 105 tests across twelve files, `fmt` and `clippy -D warnings` clean. |
| Submitted | **Stage 9c** — committed `c49dcaf` ("finish 9c"), **not yet reviewed**. |
| Next action | **Mentor's.** Review 9c, update `CONCEPTS.md` / `STAGE-LOG.md` / this file. Then split Stage 10 and issue its first half. |
| Blocked on | nothing |

## Stage queue

Stages 1–9b ✅ — `STAGE-LOG.md`. Phases 2–3: `ROADMAP.md`.

| # | Stage | New thing | Est |
|---|---|---|---|
| 9c | the app holds the case folder | `.manage()` and `State<'_, T>` — two halves of one idea | 25 |
| 10 | `Transcript` and `Phase` | a state machine as an enum with data, **and the lock** — two things, split before issuing | 50+ |

## Open, not blocking

- [ ] Doc comments: the three Stage 5 `Case` methods and both items in `error.rs` have none.
      `parse_case`'s says "but it it fails"; `RawCase`'s says only "Raw case". Tidying.
- [ ] `#[serde(default)]` on `RawCase::facts` should come off — `suspects` is required and both are
      the same kind of thing. Changes no test.
- [ ] Template leftovers: `greetMsg` naming in `App.tsx`, `public/vite.svg`, `src/assets/react.svg`.
- [ ] Rewrite `README.md` — it still describes the Tauri template.
- [ ] Add `rust-toolchain.toml` pinning a stable version.
- [ ] `withGlobalTauri` was turned on in 9b for the console demo. Turn it off before any release
      build — it exposes the whole API to any script in the page.
