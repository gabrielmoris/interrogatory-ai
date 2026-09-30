# Stage 12 — a question waits until it is awaited

Test:  src-tauri/tests/ask_async.rs — 3 tests
Run:   cd src-tauri && cargo test --test ask_async

## What you build

`AppState::ask` becomes an `async fn`, and so does the command `ask_suspect`.
One word added in `src/state.rs`, two in `src/ipc.rs`. No new function.
I changed three test files for you (`room_exchange.rs`, `app_engine.rs`, `ask_command.rs`). Do not touch them.

## Steps

1. Run the test now. It does not compile:
   4 × `error[E0277]: Result<String, AppError> is not a future`.

2. `src/state.rs`, line 51 — put `async` before `fn`:

   | now | change it to |
   |---|---|
   | `pub fn ask(&self, question: &str) -> AppResult<String> {` | `pub async fn ask(&self, question: &str) -> AppResult<String> {` |

   The body stays exactly as it is. Run: 1 error, in `ipc.rs`, not in your test:
   `error[E0308]: mismatched types … expected Result<String, AppError>, found future`.

3. `src/ipc.rs`, the `ask_suspect` command at the bottom — change its two lines:

   | line | now | change it to |
   |---|---|---|
   | 68 | `pub fn ask_suspect(` … | `pub async fn ask_suspect(` … (rest of the line stays) |
   | 69 | `state.ask(&question)` | `state.ask(&question).await` |

   It looks like this:

   ```rust
   #[tauri::command]
   pub async fn ask_suspect(state: State<'_, AppState>, question: String) -> AppResult<String> {
       state.ask(&question).await
   }
   ```

   Run: `3 passed; 0 failed`.

4. Optional: `bun tauri dev` and paste the three console lines from Stage 11d, step 4.
   Same three results. React does not notice the change.

5. `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (159 tests). Say ready.

## If it does not compile

| the compiler says | fix |
|---|---|
| `E0277 Result<String, AppError> is not a future` | Step 2 is not done: `ask` needs `async`. |
| `E0308 expected Result<String, AppError>, found future` in `ipc.rs` | Add `.await` after `state.ask(&question)`. |
| `E0728 await is only allowed inside async functions and blocks` | Add `async` to `ask_suspect`, after `pub`. |
| `expected one of extern, fn, safe, or unsafe, found keyword pub` | The order is `pub async fn`, not `async pub fn`. |
| a test stops with `0xc0000139` / `STATUS_ENTRYPOINT_NOT_FOUND` | Not your code — a Windows linking problem. Tell me. |

## What you learned

- `async fn` does not run when you call it. It gives back a *future*: the work, written down, not done yet. Test 1 shows it: after `app.ask(…)`, 0 lines are kept.
- `.await` runs the future and gives you its result — here the `AppResult<String>`. A future nobody awaits is never run (test 3).
- `.await` only works inside another `async fn`. That is why `ask_suspect` became `async` too.
- Why the game needs it: Tauri runs a plain command on the thread that draws the window. A real model takes seconds, so the window would freeze. An `async` command runs somewhere else.
- Tests are not `async`, so they cannot write `.await`. They use `block_on(…)`: run this future now and wait here.
