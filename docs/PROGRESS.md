# PROGRESS — where we are

> Resume order: `CLAUDE.md` → this file → `CONCEPTS.md` → the current stage brief.
> Last updated: 2026-09-15.

## Status

|  |  |
|---|---|
| Phase | **1 — Rust core & Tauri foundations** |
| Last done | **Stage 9b, 2026-09-15. 11/11.** 105 tests across twelve files, `fmt` and `clippy -D warnings` clean. First stage since 7 to land in one sitting. |
| Stage | **9c issued 2026-09-15** — `stages/stage-09c-the-app-holds-the-folder.md`. Spec `tests/commands.rs`, now 12 tests. 74 prose lines, one new thing. **Last stage of 9.** |
| Next action | **His.** `state.rs`, split `case_intro` in two, one `.manage` line. Then Stage 9 is done. |
| Blocked on | nothing |
| Carried | **The re-cut format works — his words, 2026-09-15: *"This way works for me, I learned."*** Keep it: one new thing, the artifact shown before the type is named, and defects named flat out with the why when he asks. Budget set 2026-09-14 — `CLAUDE.md` Rule 1 amended, and Rule 4 gains *show the artifact before naming the type*. Stage 9 re-cut a second time; `Mutex` moved to Stage 10. Corrections 10 and 11 in `MENTOR-NOTES.md`. |

A **session** is 2–3 stages, ending on a green suite and a commit. Stage estimates are minutes; the
session boundary is the commit.

---

## The stage queue

One headline concept each, at most two supporting. Sized to the budget in `CLAUDE.md` Rule 1.

### Phase 1 — Rust core (Stages 1–10)

| # | Stage | Headline concept | Est |
|---|---|---|---|
| ~~1~~ | ~~`Difficulty` / `Tuning`~~ | ~~moves and `Copy`~~ | ✅ |
| ~~2~~ | ~~`SuspectId` / `FactId`~~ | ~~the newtype pattern~~ | ✅ |
| ~~3~~ | ~~`Fact`, `Suspect`, `Case`~~ | ~~`String` vs `&str`~~ | ✅ |
| ~~4~~ | ~~borrowing and lifetimes~~ | ~~lifetimes as regions~~ | ✅ |
| ~~5~~ | ~~`AppError` and `Result`~~ | ~~`Result` and `?`~~ | ✅ |
| ~~6a~~ | ~~the file's own vocabulary~~ | ~~`Deserialize`~~ | ✅ |
| ~~6b~~ | ~~the one road~~ | ~~`TryFrom`~~ | ✅ |
| ~~6c~~ | ~~the four checks~~ | ~~validation at the boundary~~ | ✅ |
| ~~6d~~ | ~~the front door~~ | ~~`?` converts the error with `From`~~ | ✅ |
| ~~7~~ | ~~`VisibleFact<'a>`~~ | ~~a struct that holds a borrow~~ | ✅ |
| ~~8~~ | ~~`storage.rs` — a case off the disk~~ | ~~`Path` / `PathBuf`, `io::ErrorKind`~~ | ✅ |
| ~~9a~~ | ~~the box the screen gets~~ | ~~a type of your own that can be sent out~~ | ✅ |
| ~~9b~~ | ~~a function React can call~~ | ~~`#[tauri::command]` and the handler list~~ | ✅ |
| 9c | the app holds the case folder | `.manage()` **and** `State<'_, T>` | 25 |
| 10 | `Transcript` and `Phase` | a state machine as an enum with data, **and the lock** | 50+ |

One new thing each, plus at most one recall — Rule 1, as amended 2026-09-14. Stage 10 carries two
and will be split before it is issued.

**9d folded into 9c, 2026-09-15.** `.manage()` and `State<'_, T>` are two halves of one idea, the way
the attribute and the handler list were in 9b: neither half does anything observable alone, and a
stage that ends on a suite it cannot move is not a stage. Stage 9 is three, not four.

### Phase 2 — async and inference (Stages 11–19, the hard phase)

| # | Stage | Headline concept | Est |
|---|---|---|---|
| 11 | `trait InferenceEngine` + `MockEngine` | trait objects vs generics | 55 |
| 12 | the first `async fn` | a future does nothing until polled | 45 |
| 13 | where blocking work goes | blocking work must leave the runtime | 50 |
| 14 | channels | an `mpsc` pipe moves ownership | 50 |
| 15 | sharing across threads | `Arc<Mutex<T>>`, and why a std guard cannot cross `.await` | 55 |
| 16 | streaming to React | Tauri events with a typed payload | 45 |
| 17 | cancellation | `select!` and cooperative cancellation | 55 |
| 18 | **toolchain session** — `llama-cpp-2` on Windows/CUDA | **zero new concepts. Environment only.** | 2 h |
| 19 | owning the model | one thread owns the FFI handle | 60 |

Stage 13 is done wrong once on purpose — watch the UI freeze, then fix it.

### Phase 3 — game engine (Stages 20–23)

| # | Stage | Headline concept | Est |
|---|---|---|---|
| 20 | `build_prompt` | prompts are code — snapshot tests | 55 |
| 21 | tier-1 scoring | pure functions over ids, reproducible | 55 |
| 22 | the generator skeleton | determinism from a seed | 60 |
| 23 | grammar-constrained output | constraining a model at decode time | 55 |

Phases 4–5 are his home turf and carry no Rust teaching budget — `ROADMAP.md`.

---

## Open, not blocking

- [ ] Doc comments: the three Stage 5 `Case` methods and both items in `error.rs` have none, while
      everything else public in `case.rs` does. `parse_case`'s says "but it it fails". Tidying.
- [ ] `#[serde(default)]` on `RawCase::facts` should come off — `suspects` is required and both are
      the same kind of thing. Changes no test.
- [ ] `greet` was deleted in 9b but `App.tsx` still calls it, so the template UI now rejects with
      "command not found". Finish the job: the template `App.tsx`, `public/vite.svg`,
      `src/assets/react.svg`.
- [ ] Rewrite `README.md` — it still describes the Tauri template.
- [ ] Add `rust-toolchain.toml` pinning a stable version.
- [x] `src-tauri/cases/` — shipped with 9b, so the command has something real to find.
- [ ] `CASES_DIR` stops being a constant in 9c, when `AppState` owns the folder. Its doc comment in
      `ipc.rs` says 9b; correct it then.
- [ ] `withGlobalTauri` was turned on in 9b to make the console demo possible. Turn it off before
      any release build — it exposes the whole API surface to any script in the page.
