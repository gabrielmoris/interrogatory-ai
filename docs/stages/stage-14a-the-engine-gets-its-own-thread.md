# Stage 14a — the engine gets its own thread

Test:  src-tauri/tests/engine_thread.rs — 4 tests
Run:   cd src-tauri && cargo test --test engine_thread

## What you build

The real model will be slow, and it must live on one thread of its own.
A new file, `src/engine_thread.rs`: `start` puts the engine on its own thread and gives back a
`Sender` — the end of a channel you drop `Question`s into. Each answer goes back to whoever asked.

## Steps

1. Run the test now. 1 error: `E0432 unresolved import interrogatory_ai_lib::engine_thread`.

2. `src/lib.rs` — add this line under `pub mod difficulty;`:

   ```rust
   pub mod engine_thread;
   ```

   Create an empty file `src/engine_thread.rs`.
   Run: 1 error, `E0432 unresolved imports …engine_thread::start, …engine_thread::Question`.

3. `src/engine_thread.rs` — paste this. The `println!` line is one line too many, on purpose:

   ```rust
   use crate::error::AppResult;
   use crate::llm::InferenceEngine;
   use std::sync::mpsc::{channel, Receiver, Sender};
   use std::thread;

   /// One question for the engine, and where to send its answer.
   pub struct Question {
       pub text: String,
       pub answer_to: Sender<AppResult<String>>,
   }

   /// Puts the engine on a thread of its own. Send questions to what comes back.
   pub fn start(engine: Box<dyn InferenceEngine>) -> Sender<Question> {
       let (questions, inbox) = channel();
       thread::spawn(move || answer_all(engine, inbox));
       questions
   }

   /// Runs on the engine's thread. Answers each question until no one can ask any more.
   fn answer_all(engine: Box<dyn InferenceEngine>, inbox: Receiver<Question>) {
       for question in inbox {
           let reply = engine.reply(&question.text);
           let _ = question.answer_to.send(reply);
           println!("sent: {:?}", reply);
       }
   }
   ```

   Run: 1 error, `E0382 borrow of moved value: reply`. The compiler points at `.send(reply)` —
   "value moved here" — and at the `println!` — "value borrowed here after move".

4. Delete the `println!` line.
   Run: `4 passed; 0 failed`.

5. `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (167 tests). Say ready.

## If it does not compile

| the compiler says | fix |
|---|---|
| `E0382 borrow of moved value: reply` | Delete the `println!` line (step 4). `send` took `reply`. |
| clippy: `unused Result that must be used` | Keep `let _ =` in front of `question.answer_to.send(reply)`. |
| `E0106 missing lifetime specifier` | `text` is `String`, not `&str`. |
| `E0432 unresolved import …engine_thread` | `pub mod engine_thread;` is missing from `lib.rs`. |

## What you learned

- `channel()` gives two ends: a `Sender` that puts values in, a `Receiver` that takes them out. `let (questions, inbox) = …` names both ends at once.
- `send(reply)` *moves* `reply` into the channel. Afterwards it is gone from here (step 3's `E0382`); whoever receives it owns it. Nothing is copied or shared.
- `for question in inbox` waits for each question, and ends when every `Sender` is gone. So the thread stops by itself when the game drops its end.
- A `Sender` is a value too, so a question can carry one: that is how each answer finds its asker. `let _ =` ignores `send`'s result on purpose — it only fails if the asker stopped waiting.
- `thread::spawn` is Stage 13's `spawn_blocking` for work that never ends: a thread of its own. Why the game needs it: one thread owns the engine; everyone else only talks to it.
