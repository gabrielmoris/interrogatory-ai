# PROGRESS — where we are

Last updated: 2026-09-17.

|  |  |
|---|---|
| Phase | **1 — Rust core & Tauri foundations** |
| Last reviewed | **Stage 9c**, 2026-09-17. 12/12; 106 tests across twelve files, `fmt` and `clippy -D warnings` clean. |
| Issued | **Stage 10a** — `docs/stages/stage-10a-where-the-player-is.md`, spec `tests/phase.rs` (6 tests, measured). |
| Next action | **His.** Stage 10a. Then mentor: review it, write 10b into `tests/phase.rs`. |
| Blocked on | nothing |

## Stage queue

Stages 1–9c ✅ — `STAGE-LOG.md`. Phases 2–3: `ROADMAP.md`. Stage 10's shape: `DECISIONS.md`, 2026-09-17.

| # | Stage | New thing | Est |
|---|---|---|---|
| 10a | where the player is | data inside one variant, and the `match` arm that takes it out · recall `*` | 25 |
| 10b | moving between phases | replacing the whole value behind `&mut self` (`*self = …`) · recall guard clause *(shaky)* | 25 |
| 10c | recording a line | a `match` on `&mut self` hands out `&mut` to the data inside · recall `Vec::push` | 25 |
| 10d | the app holds the phase | `Mutex` — changing a value the app shares · recall `.map_err` *(shaky)*. Decide the poisoned-lock variant before writing. | 30 |
| 10e | React starts an interrogation | `Deserialize` on `SuspectId` for a command argument · recall `generate_handler!` | 25 |

10d and 10e: re-count against Rule 1 before issuing — either may split.

## Open, not blocking

- [ ] Rule 4a: all three habits asked once, in 10a (§4 rule 3, rule 5, §5 opening line). Do not ask again.
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
