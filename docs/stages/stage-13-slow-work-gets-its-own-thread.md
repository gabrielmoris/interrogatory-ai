# Stage 13 — slow work gets its own thread

Test:  src-tauri/tests/load_in_background.rs — 4 tests
Run:   cd src-tauri && cargo test --test load_in_background

## What you build

Opening a case reads a file from disk. The program waits while it reads.
A new function in `src/ipc.rs`, `case_intro_in_background`, does that reading on a separate thread.
The command `case_intro` becomes `async` and uses it. One new error in `src/error.rs`.

## Steps

1. Run the test now. It does not compile — 3 errors:
   `E0432 unresolved import … case_intro_in_background`, `E0599 no variant named BackgroundFailed`,
   `E0277 Result<CaseIntro, AppError> is not a future`.

2. `src/error.rs` — add this variant at the end of `AppError`, after `SuspectKnowsNothing`:

   ```rust
       #[error("work on a background thread stopped before it finished: {message}")]
       BackgroundFailed { message: String },
   ```

   Run: 2 errors left (`E0432`, `E0277`).

3. `src/ipc.rs`, the imports at the top — change two lines, add one:

   | now | change it to |
   |---|---|
   | `use crate::error::AppResult;` | `use crate::error::{AppError, AppResult};` |
   | `use std::path::Path;` | `use std::path::{Path, PathBuf};` |
   | *(new line, above `use tauri::State;`)* | `use tauri::async_runtime::spawn_blocking;` |

4. `src/ipc.rs` — paste this between `case_intro_from` and the `case_intro` command. It has no `move` yet, on purpose:

   ```rust
   /// Reads the case on a separate thread, so the window never waits for the disk.
   pub async fn case_intro_in_background(cases_dir: PathBuf, slug: String) -> AppResult<CaseIntro> {
       spawn_blocking(|| case_intro_from(&cases_dir, &slug))
           .await
           .map_err(|e| AppError::BackgroundFailed {
               message: e.to_string(),
           })?
   }
   ```

   Run: 2 × `error[E0373]: closure may outlive the current function, but it borrows slug`
   (and the same for `cases_dir`).

5. Same line — put `move` before the `||`:

   | now | change it to |
   |---|---|
   | `spawn_blocking(\|\| case_intro_from(&cases_dir, &slug))` | `spawn_blocking(move \|\| case_intro_from(&cases_dir, &slug))` |

   Run the test: 1 error left, `E0277 … is not a future` — that is the command, next step.

6. `src/ipc.rs`, the `case_intro` command — change both lines:

   | line | now | change it to |
   |---|---|---|
   | signature | `pub fn case_intro(` … | `pub async fn case_intro(` … (rest stays) |
   | body | `case_intro_from(&state.cases_dir, &slug)` | `case_intro_in_background(state.cases_dir.clone(), slug).await` |

   Run: `4 passed; 0 failed`.

7. Optional: `bun tauri dev`, then in the console: `await window.__TAURI__.core.invoke('case_intro', { slug: 'the-ledger' })`.
   Same intro as Stage 9b. React does not notice.

8. `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (163 tests). Say ready.

## If it does not compile

| the compiler says | fix |
|---|---|
| `E0373 closure may outlive the current function, but it borrows …` | Add `move` before `\|\|` (step 5). |
| `E0521 borrowed data escapes outside of function` | The inputs must be `PathBuf` and `String`, not `&Path` and `&str`. |
| `E0308 expected Result<CaseIntro, AppError>, found Result<Result<…>…>` | Put the `?` back after the `map_err(…)`. |
| `E0599 no method named map_err found for … JoinHandle` | `.await` is missing before `.map_err`. |
| `E0507 cannot move out of dereference of State` | Write `state.cases_dir.clone()`, not `state.cases_dir`. |
| a test stops with `0xc0000139` / `STATUS_ENTRYPOINT_NOT_FOUND` | Not your code — a Windows linking problem. Tell me. |

## What you learned

- `|| …` is a closure: a function with no name, written in place. TypeScript `() => …`. The inputs go between the bars — your `.map(|suspect| …)` in `CaseIntro::from` is `(suspect) => …`; `||` means no inputs.
- `spawn_blocking` takes the closure *uncalled* and calls it on another thread. `.await` waits for it without blocking the window.
- `move` makes the closure *own* `cases_dir` and `slug` instead of borrowing them, so they travel with it. Without `move` it would borrow values owned by this function, and the thread might still be using them after the function ends. That is why the inputs are `PathBuf` and `String`, and `case_intro` clones the folder.
- You get two results back: did the thread finish (the outer one, mapped to `BackgroundFailed` and unwrapped by `?`), and did the case load (the inner one, returned as it is).
- Why the game needs it: a plain command runs on the thread that draws the window (Stage 12), so a slow disk froze the screen. Now the slow part runs where waiting hurts nobody.
