# Stage 11b — the game holds an engine

Test:  src-tauri/tests/app_engine.rs — 3 tests
Run:   cd src-tauri && cargo test --test app_engine

## What you build

`AppState` keeps one engine for as long as the app runs, and a new method `ask` passes the
detective's question to it. `AppState` does not know which engine it holds — today `lib.rs` gives
it a `MockEngine`; later it gives it the real model, and `state.rs` does not change.
Files: `src/llm.rs` (one line), `src/state.rs`, `src/lib.rs`.

I already changed four old test files (`app_phase`, `app_room`, `pick_suspect`, `commands`) to the
new `AppState::new`. They stop compiling at step 1 and compile again after step 4.

## Steps

1. Run the test now. It does not compile: 4 errors, two `E0061 this function takes 1 argument but
   2 arguments were supplied` and two `E0599 no method named ask found for struct AppState`.

2. `src/llm.rs` — line 4. Change the whole line

   ```rust
   pub trait InferenceEngine {
   ```

   to

   ```rust
   pub trait InferenceEngine: Send + Sync {
   ```

   Run: the same 4 errors. Nothing changes yet; step 3 needs it.

3. `src/state.rs` — three edits. The middle one is the new thing of this stage.

   | where | add or change |
   |---|---|
   | under line 2, `use crate::ids::SuspectId;` | add `use crate::llm::InferenceEngine;` |
   | under `phase: Mutex<Phase>,` in the struct | add `engine: Box<dyn InferenceEngine>,` |
   | `pub fn new(cases_dir: PathBuf) -> Self {` | change to `pub fn new(cases_dir: PathBuf, engine: Box<dyn InferenceEngine>) -> Self {` |
   | under `phase: Mutex::new(Phase::Briefing),` in `new` | add `engine,` |

   After `cargo fmt` the top of the file looks like this:

   ```rust
   /// What the app holds on to for as long as it runs.
   pub struct AppState {
       pub cases_dir: PathBuf,
       phase: Mutex<Phase>,
       engine: Box<dyn InferenceEngine>,
   }

   impl AppState {
       pub fn new(cases_dir: PathBuf, engine: Box<dyn InferenceEngine>) -> Self {
           Self {
               cases_dir,
               phase: Mutex::new(Phase::Briefing),
               engine,
           }
       }
   ```

   Run: `E0061 this function takes 2 arguments but 1 argument was supplied`, in `src/lib.rs:20`.
   The app itself still calls the old `new`.

4. `src/lib.rs` — two edits.

   | where | add or change |
   |---|---|
   | under line 2, `use ipc::{begin_interrogation, case_intro};` | add `use llm::MockEngine;` |
   | line 20, `.manage(AppState::new(PathBuf::from("cases")))` | change to `.manage(AppState::new(PathBuf::from("cases"), Box::new(MockEngine::new("I have nothing to say to you."))))` |

   `cargo fmt` splits the long line over four lines. That is fine.
   Run: two `E0599 no method named ask`, and one warning, `field engine is never read`. Step 5 reads it.

5. `src/state.rs` — paste this at the bottom of `impl AppState`, after `suspect`, before the last `}`:

   ```rust
       /// The suspect's reply to what the detective just asked.
       pub fn ask(&self, question: &str) -> AppResult<String> {
           todo!()
       }
   ```

   Run: `1 passed; 2 failed`. Two warnings (`question` unused, `engine` never read) — step 6 uses both.
   The test that passes does not touch `AppState`; it only puts two engines in one list.

6. `src/state.rs` — replace `todo!()` with one line: ask the engine. In `tests/engine.rs` the
   helper `ask` does it with `engine.reply(question)`. Here the engine is a field, so it is
   `self.engine.reply(question)`. No `Ok(…)`: `reply` already hands back an `AppResult<String>`.
   Run: `3 passed; 0 failed`.

7. `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (151 tests). Say ready.

## If it does not compile

| the compiler says | fix |
|---|---|
| 14 errors, `dyn InferenceEngine cannot be sent between threads safely` / `cannot be shared between threads safely` | Step 2 is missing: `pub trait InferenceEngine: Send + Sync {` in `llm.rs`. |
| `E0405 cannot find trait InferenceEngine in this scope` (in `state.rs`) | Add `use crate::llm::InferenceEngine;` at the top of `state.rs`. |
| `E0433 cannot find type MockEngine in this scope` (in `lib.rs`) | Add `use llm::MockEngine;` at the top of `lib.rs`. |
| `E0308 mismatched types … expected Box<dyn InferenceEngine>, found MockEngine` (in `lib.rs`) | Wrap it: `Box::new(MockEngine::new(…))`. |
| `E0308 mismatched types … expected String, found Result<String, AppError>` (in `state.rs`) | Remove the `Ok( … )` around `self.engine.reply(question)`. |

## What you learned

- `Box<dyn InferenceEngine>` means "one value of some type that has the trait — we do not say which".
- `dyn` = "any type with this trait". `Box` = the value lives in its own spot in memory, and the box
  is always the same small size, so it can sit in a struct field whatever the engine is.
- `Box::new(MockEngine::new(…))` puts a mock in a box. `Box::new(EchoEngine)` puts a different
  type in the same kind of box — see `tests/app_engine.rs`.
- `: Send + Sync` on the trait (step 2): Tauri may run two commands at the same time, on different
  threads, and they all share the one `AppState`. So everything inside `AppState` must be safe to
  use from several threads. Rust checks this by itself for `String`, `PathBuf`, `Mutex<Phase>`.
  But a `Box<dyn …>` hides which type is inside, so Rust cannot check it and says no. The trait
  line adds a rule: only types safe to share may be an engine. Now Rust knows the box is safe.
  (`Send` = can move to another thread. `Sync` = can be used from several threads at once.)
