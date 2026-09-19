# PROGRESS — where we are

Last updated: 2026-09-17.

|  |  |
|---|---|
| Phase | **1 — Rust core & Tauri foundations** |
| Last reviewed | **Stage 10b**, 2026-09-19. 7/7; 119 tests across fourteen files, `fmt` and `clippy -D warnings` clean. |
| Uncommitted | his 10b fix to `src/transcript.rs` — commit it. |
| Next action | **Mentor's.** Write 10c. Open it by folding the two guards' duplicated error into one helper (`record` would be the third copy), and ask the `shaky` question: which phase does a move's check name? |
| Blocked on | nothing |

## Stage queue

Stages 1–10b ✅ — `STAGE-LOG.md`. Phases 2–3: `ROADMAP.md`. Stage 10's shape: `DECISIONS.md`, 2026-09-17.

| # | Stage | New thing | Est |
|---|---|---|---|
| 10c | recording a line | a `match` on `&mut self` hands out `&mut` to the data inside · recall `Vec::push` | 25 |
| 10d | the app holds the phase | `Mutex` — changing a value the app shares · recall `.map_err` *(shaky)*. Decide the poisoned-lock variant before writing. | 30 |
| 10e | React starts an interrogation | `Deserialize` on `SuspectId` for a command argument · recall `generate_handler!` | 25 |

10d and 10e: re-count against Rule 1 before issuing — either may split.

## Open, not blocking

- [ ] Rule 4a: all three habits were asked once, in 10a. Do not ask again.
- [ ] Doc comments: `ipc.rs :: case_intro_from` / `case_intro` (9c), the three Stage 5 `Case` methods
      and both items in `error.rs` have none.
      `parse_case`'s says "but it it fails"; `RawCase`'s says only "Raw case". Tidying.
- [ ] `#[serde(default)]` on `RawCase::facts` should come off — `suspects` is required and both are
      the same kind of thing. Changes no test.
- [ ] Template leftovers: `greetMsg` naming in `App.tsx`, `public/vite.svg`, `src/assets/react.svg`.
- [ ] Rewrite `README.md` — it still describes the Tauri template.
- [ ] Add `rust-toolchain.toml` pinning a stable version.
- [ ] `withGlobalTauri` was turned on in 9b for the console demo. Turn it off before any release
      build — it exposes the whole API to any script in the page.
