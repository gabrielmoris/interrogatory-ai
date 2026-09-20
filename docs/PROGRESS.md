# PROGRESS — where we are

Last updated: 2026-09-20.

|  |  |
|---|---|
| Phase | **1 — Rust core & Tauri foundations** |
| Last reviewed | **Stage 10b**, 2026-09-19. 7/7; 119 tests across fourteen files, `fmt` and `clippy -D warnings` clean. |
| Uncommitted | `tests/phase_record.rs` and the 10c brief — mentor's files, commit them with 10c. |
| Next action | **His.** Stage 10c step 2 — brief rewritten 2026-09-20 in the new format. Step 1 is already in his `transcript.rs` (2 passed; 3 failed). On "ready": review, then `CONCEPTS.md`, `STAGE-LOG.md`, this file. |
| Blocked on | nothing |

## Stage queue

Stages 1–10b ✅ — `STAGE-LOG.md`. Phases 2–3: `ROADMAP.md`. Stage 10's shape: `DECISIONS.md`, 2026-09-17.

| # | Stage | New thing | Est |
|---|---|---|---|
| 10c | keeping what was said — **in progress** | with `&mut self`, the list named in a `match` line can be pushed to. Step 1 (the `refusal` helper, the arm) done; step 2 is the push. | 15 |
| 10d | the app holds the phase | `Mutex` — changing a value the app shares · recall `.map_err` *(shaky)*. Decide the poisoned-lock variant before writing. | 30 |
| 10e | React starts an interrogation | `Deserialize` on `SuspectId` for a command argument · recall `generate_handler!` | 25 |

10d and 10e: re-count against Rule 1 before issuing — either may split.

## Open, not blocking

- [ ] 10d and 10e briefs use the rewritten Rule 2: §1 is his own code, no other domain, one blank per
      value, recall asked in chat. Stages 1–10b were written to the old rule — do not copy their shape.

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
