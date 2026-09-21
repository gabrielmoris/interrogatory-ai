# PROGRESS — where we are

Last updated: 2026-09-20.

|  |  |
|---|---|
| Phase | **1 — Rust core & Tauri foundations** |
| Last reviewed | **Stage 10c**, 2026-09-20. 5/5; 124 tests across fifteen files, `clippy -D warnings` clean. `cargo fmt` not yet run. |
| Uncommitted | the `cargo fmt` diff in `transcript.rs`; the teaching-rules rewrite; Stage 10d's brief and spec. 10c itself is committed (`02a7c33`). |
| Next action | **His.** Stage 10d — brief at `docs/stages/stage-10d-what-the-room-will-show.md`, spec at `src-tauri/tests/phase_transcript.rs`. Open the session by asking 10c's recall question (`CONCEPTS.md`). |
| Blocked on | nothing |

**The teaching rules were rewritten 2026-09-20 (correction 18)** after *"I finish the stage without
understanding why Rust needed it. I feel I am copying."* `CLAUDE.md` gained Rule 0 (when a thing is
explained decides whether it teaches) and Rule 3 (the concept must live in his head — predict before
running, one version that passes and is wrong, four closed-book questions once green). Rules were
renumbered: old 3→4, 4+4a→5, 5→7; 6 is unchanged. `TEACHING-EVIDENCE.md` is the research base and is
what to read before relaxing any of it. **Stages 1–10c were written to the old rules — do not copy
their shape.**

## Stage queue

Stages 1–10c ✅ — `STAGE-LOG.md`. Phases 2–3: `ROADMAP.md`. Stage 10's shape: `DECISIONS.md`, 2026-09-17.

| # | Stage | New thing | Est |
|---|---|---|---|
| 10d | what the room will show | **consolidation — zero new elements.** `&[Turn]` as the borrowed form of `Vec<Turn>`, which is Stage 3's `String`/`&str` on a second type. Two whole bodies, signatures given. | 25 |
| 10e | the app holds the phase | `Mutex` — changing a value the app shares · recall `.map_err` *(shaky)*. Decide the poisoned-lock variant before writing. | 30 |
| 10f | React starts an interrogation | `Deserialize` on `SuspectId` for a command argument · recall `generate_handler!` | 25 |

10e and 10f: re-count against Rule 1 before issuing — either may split. Both must be written to the
rewritten rules, not to the shape of 10a–10c.

## Open, not blocking

- [ ] Rule 5's three habits were asked once, in 10a. Do not ask again.
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
