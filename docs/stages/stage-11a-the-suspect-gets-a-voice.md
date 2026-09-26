# Stage 11a — the suspect gets a voice

Test:  src-tauri/tests/engine.rs — 3 tests
Run:   cd src-tauri && cargo test --test engine

## What you build

Something that can answer the detective. Later it will be a real language model; today it is a
fake one that always says the same line, so tests stay fast.
New file `src/llm.rs`: the trait `InferenceEngine` (one method, `reply`) and `MockEngine`, which has it.

## Steps

1. Run the test now. It does not compile:
   `error[E0432]: unresolved import interrogatory_ai_lib::llm`.

2. `src/lib.rs` — under line 10, `pub mod ipc;`, add one line:

   ```rust
   pub mod llm;
   ```

   Create the file `src/llm.rs`, empty. Run:
   `error[E0432]: unresolved imports …::llm::InferenceEngine, …::llm::MockEngine`.

3. `src/llm.rs` — paste this at the top. It is the new thing of this stage.

   ```rust
   use crate::error::AppResult;

   /// Anything that can answer the detective. The game only talks to this.
   pub trait InferenceEngine {
       /// The suspect's reply to what the detective just said.
       fn reply(&self, prompt: &str) -> AppResult<String>;
   }
   ```

   Run: `error[E0432]: unresolved import …::llm::MockEngine`.

4. `src/llm.rs` — paste this under the trait. It is a struct and a `new`, like `AppState::new`.

   ```rust
   /// A fake engine for tests: it always gives the same line.
   pub struct MockEngine {
       line: String,
   }

   impl MockEngine {
       pub fn new(line: &str) -> Self {
           Self {
               line: line.to_string(),
           }
       }
   }
   ```

   Run: 4 errors. Three `E0599 no method named reply found for struct MockEngine`, one
   `E0277 the trait bound MockEngine: InferenceEngine is not satisfied`. Both say the same thing:
   `MockEngine` does not have the trait yet.

5. `src/llm.rs` — give `MockEngine` the trait. At the bottom of the file:

   | copy this | from | change |
   |---|---|---|
   | `impl From<&Case> for CaseIntro {` | `ipc.rs` | `From<&Case>` → `InferenceEngine`, `CaseIntro` → `MockEngine` |
   | `fn reply(&self, prompt: &str) -> AppResult<String>;` | the trait, step 3 | `prompt` → `_prompt`, and the `;` → `{ todo!() }` |

   Close the `impl` with `}`. Nothing else goes in it. After `cargo fmt` it looks like this:

   ```rust
   impl InferenceEngine for MockEngine {
       fn reply(&self, _prompt: &str) -> AppResult<String> {
           todo!()
       }
   }
   ```

   Run: `0 passed; 3 failed`. It compiles now; the body is still missing.

6. `src/llm.rs` — replace `todo!()` with one line: hand back a copy of the mock's line, wrapped in
   `Ok`. The copy is `self.line.clone()`, like `case.title.clone()` in `ipc.rs`.
   Run: `3 passed; 0 failed`.

7. `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (148 tests). Say ready.

## If it does not compile

| the compiler says | fix |
|---|---|
| `E0308 mismatched types … expected Result<String, AppError>, found String` | Wrap the line in `Ok( … )`. |
| `E0507 cannot move out of self.line which is behind a shared reference` | Add `.clone()`: `self.line.clone()`. |
| `E0053 method reply has an incompatible type for trait` | The `fn reply` line must match the trait's exactly. Copy it again from step 3. |
| `E0449 visibility qualifiers are not permitted here` | Remove `pub` before `fn reply`. Inside `impl … for …`, the trait decides what is public. |
| clippy: `unused variable: prompt` | Write `_prompt`. The `_` says: this mock ignores the question, on purpose. |

## What you learned

- A trait is a list of methods a type promises to have. You wrote your own: `InferenceEngine` has `reply`.
- `impl InferenceEngine for MockEngine` keeps the promise — same shape as your `impl From<&Case> for CaseIntro`.
- The trait only has the `fn` line, ending in `;`. Each type that has the trait writes the body.
- The game will ask "anything with `reply`", never "a `MockEngine`". So the real model can replace the mock later and no game code changes.
