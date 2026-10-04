# Stage 14a — the engine gets its own thread

Test:  src-tauri/tests/engine_thread.rs — 4 tests
Run:   cd src-tauri && cargo test --test engine_thread

## 1. What part of the app

The suspect's voice: the **engine**. Today, when the player asks a question, this happens:

```
React → ask_suspect (ipc.rs) → AppState::ask (state.rs) → self.engine.reply(question)
```

The engine is the suspect's brain: anything that has a `reply` method (the `InferenceEngine` trait,
Stage 11a). Today it is `MockEngine`, which always says the same line. Later it is the real model.

The real model has two problems:

- It is slow. One reply can take seconds.
- It must have one owner. Only one part of the program may use it.

So the engine gets a thread of its own. Nobody else touches it. The rest of the game sends it
questions and gets answers back.

This stage builds that thread. Stage 14b connects `AppState::ask` to it. Until then, only the tests use it.

## 2. Files and templates

| File | What you do |
|---|---|
| `src/lib.rs` | add `pub mod engine_thread;` under `pub mod difficulty;` |
| `src/engine_thread.rs` | new file — paste the template, then write the two `todo!()` bodies |

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
    todo!()
}

/// Runs on the engine's thread. Answers each question until no one can ask any more.
fn answer_all(engine: Box<dyn InferenceEngine>, inbox: Receiver<Question>) {
    todo!()
}
```

Run: `0 passed; 4 failed`, and 6 warnings about unused things. They go away when both bodies are written.

What each part is:

- `Question` — one question for the engine. `text` is what the detective asked. `answer_to` is where
  the engine puts the answer (§4.4).
- `start` — the game calls it once. It takes the engine and gives back a `Sender<Question>`: the slot
  where you drop questions.
- `answer_all` — runs on the engine's thread until nobody can ask any more. No `pub`: only `start` calls it.

## 3. What each function does, and how to build it

### Where `engine` comes from

Both functions have an `engine` parameter. It is the same engine, travelling:

1. Whoever calls `start` hands it in. In the tests (`tests/engine_thread.rs`):
   ```rust
   let questions = start(Box::new(MockEngine::new("I was at home all night.")));
   ```
   In the game, the engine is made in `lib.rs :: run` and given to `AppState::new`:
   ```rust
   .manage(AppState::new(
       PathBuf::from("cases"),
       Box::new(MockEngine::new("I have nothing to say to you.")),
   ))
   ```
   In 14b, `AppState::new` passes it on to `start`.
2. `start` moves it into the new thread (`move ||`, §5.4).
3. On that thread, `answer_all` receives it and keeps it until the thread ends.

So `start` has the parameter to *pass it on*. `answer_all` has it to *use it*: `engine.reply(…)`.

### `start` — step by step

What it does: makes a channel, starts the engine's thread with the receiving end, gives back the sending end.

1. Make a channel. Name its two ends `questions` (the Sender) and `inbox` (the Receiver).
   Syntax: §4.1 and §4.2.
2. Start a new thread that runs `answer_all(engine, inbox)`. Syntax: §4.3. It needs `move`, like
   your Stage 13 line (§5.4).
3. Give back `questions`: write it as the last line, with no `;` (§5.5).

Run: still `0 passed; 4 failed`. Each test now also prints `not yet implemented` from a thread
named `'<unnamed>'`. That is your engine thread, reaching `answer_all`'s `todo!()`.

### `answer_all` — step by step

What it does: waits for questions, one at a time. For each one: asks the engine, sends the reply to
whoever asked.

1. Write a `for` loop over `inbox`. Name each item `question`. Syntax: §4.5 (new) and §5.7 (known).
2. Inside the loop: ask the engine to reply to `question.text`, and keep the result in `reply`.
   Syntax: §5.6.
3. Still inside the loop: send `reply` through `question.answer_to`, with `let _ =` in front.
   Syntax: §4.6 and §4.7.

Run: `4 passed; 0 failed`.

Last: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (167 tests). Say ready.

### If it does not compile

| the compiler says | fix |
|---|---|
| `E0308 mismatched types` … `expected &str, found String` | `engine.reply(&question.text)` — lend the `String` with `&` (§5.6). |
| `E0308 mismatched types` … `expected Sender<Question>, found ()` | Remove the `;` after `questions` on the last line of `start` (§5.5). |
| `E0382 borrow of moved value: reply` | You used `reply` after `send`. `send` took it (§4.6). |
| clippy: `unused std::result::Result that must be used` | Put `let _ =` in front of the `send` line (§4.7). |
| a test `has been running for over 60 seconds` | `answer_all(…)` must run inside `thread::spawn(move \|\| …)`. Called directly, `start` never finishes. Stop with Ctrl+C. |

## 4. New syntax and methods

### 4.1 `channel()` — a pipe between threads

A channel is a pipe. One thread puts values in at one end. Another thread takes them out at the
other end, in the same order.

`channel()` makes one pipe and gives you both ends at once:

- a `Sender<T>` — the end you put values into.
- a `Receiver<T>` — the end you take values out of.

`T` is the type that travels through the pipe. You do not write it: Rust works it out from how you
use the ends. In `start`, you give back the Sender as a `Sender<Question>`, so `T` is `Question`.

It comes from `std::sync::mpsc`, the standard library's channels. `mpsc` means "many senders, one
receiver": the Sender can be copied (`.clone()`), the Receiver cannot.

**When to use one:** one thread owns something, and other threads need it to do work. They never
touch the thing; they only send it messages. If you have used Web Workers, `postMessage` is the same idea.

**Why here:** the engine lives on its own thread, and every question reaches it through the pipe.
A `Mutex` (like `phase` in `state.rs`) is the wrong tool here: whoever asks would run the slow
model on their own thread, and hold the lock for seconds.

### 4.2 `let (a, b) = …` — taking a pair apart

`channel()` gives back its two ends together, as a **tuple**: a fixed group of values in round
brackets, `(Sender, Receiver)`.

`let (a, b) = pair;` makes two variables at once: `a` gets the first value, `b` the second.
TypeScript: `const [a, b] = pair;`.

Example, from `tests/engine_thread.rs`:

```rust
let (answer_to, answer) = channel();
```

`answer_to` is the Sender, `answer` is the Receiver. The Sender always comes first.

### 4.3 `thread::spawn` — starting a thread

`thread::spawn(…)` starts a new thread and runs the closure you give it on that thread. It returns
straight away: the new thread keeps running on its own, and your function carries on.

Form:

```rust
thread::spawn(move || some_function(a, b));
```

`move || some_function(a, b)` is a closure (§5.4) that hands `a` and `b` over to the new thread.

`thread::spawn` also gives back a handle to the thread. We do not need it, so we do not keep it.

How it differs from Stage 13's `spawn_blocking`: that one is for a short job, and you `.await` it to
get the result. The engine thread runs for as long as the game does, and answers through channels.
So: no `.await`.

### 4.4 A Sender inside a struct: `answer_to`

`Question` carries a `Sender<AppResult<String>>`. Read it from the outside in: a Sender, for
`AppResult<String>` values — `Ok(text)` or `Err(AppError)` (§5.2).

Why: the engine has one inbox but many askers. So each asker makes its own small channel for the
answer, puts the Sender end in the question, and keeps the Receiver to wait on. From the test:

```rust
let questions = start(Box::new(MockEngine::new("I was at home all night.")));
let (answer_to, answer) = channel();

questions
    .send(Question {
        text: "Where were you on Tuesday?".to_string(),
        answer_to,
    })
    .expect("the engine thread is running");

assert_eq!(
    answer.recv(),
    Ok(Ok("I was at home all night.".to_string()))
);
```

`answer.recv()` waits until a value arrives in that Receiver. `answer_to,` alone is short for
`answer_to: answer_to,` — the same shorthand as in TypeScript objects.

### 4.5 `for question in inbox` — looping over a Receiver

A Receiver can be looped over with `for`, like a list (§5.7). Two differences:

- **It waits.** Each turn of the loop waits until the next value arrives.
- **It ends only when every Sender is gone.** While anyone could still send, it keeps waiting.

So `answer_all` runs for as long as someone holds `questions`. When the game drops its Sender, the
loop ends, `answer_all` returns, and the thread finishes by itself.

One more difference from your `for raw_suspect in &raw.suspects`: there is no `&`. Each `question`
comes out of the pipe as yours (owned), not borrowed.

### 4.6 `send` — it moves the value

`sender.send(value)` puts `value` into the pipe. It **moves** it: after `send(reply)`, `reply`
belongs to whoever takes it out. Nothing is copied. Use `reply` after that line and the compiler
stops you with `E0382 borrow of moved value`.

`send` gives back a `Result`: `Ok(())` if the value went in, `Err` if nobody holds the Receiver any
more (for example, the asker stopped waiting).

Here the Sender is a field of `question`, so the call is `question.answer_to.send(…)`.

### 4.7 `let _ =` — ignoring a result on purpose

Rust warns when you ignore a `Result`: it could be an error you forgot. `let _ = something;` says
"I know — throw it away." `_` means "don't keep this".

Here ignoring is right: if the asker stopped waiting, there is nobody to tell. Without `let _ =`,
clippy fails with `unused std::result::Result that must be used`.

## 5. Syntax you already know — from your code

### 5.1 `use` — several items, or a whole module

`ipc.rs`, top of the file:

```rust
use tauri::async_runtime::spawn_blocking;
use tauri::State;

use crate::case::Case;
use crate::error::{AppError, AppResult};
```

`{AppError, AppResult}` brings in several items from one place, like the template's
`use std::sync::mpsc::{channel, Receiver, Sender};`.

`use tauri::async_runtime::spawn_blocking;` brings in a function, so you write `spawn_blocking(…)`.
`use std::thread;` brings in a whole module, so you write `thread::spawn(…)`.

### 5.2 `AppResult<T>`

`error.rs`:

```rust
pub type AppResult<T> = Result<T, AppError>;
```

So `AppResult<String>` is `Result<String, AppError>`: `Ok(text)` or `Err(error)`.

### 5.3 `Box<dyn InferenceEngine>` — "some engine"

`llm.rs`:

```rust
pub trait InferenceEngine: Send + Sync {
    /// The suspect's reply to what the detective just said.
    fn reply(&self, prompt: &str) -> AppResult<String>;
}
```

`state.rs`, inside `AppState`:

```rust
engine: Box<dyn InferenceEngine>,
```

`dyn InferenceEngine` means any type that has the trait. `Box` holds it. The `Send` in the trait's
first line is what allows the engine to move to another thread; without it, `thread::spawn` would
refuse. Stage 15 explains `Send`.

### 5.4 Closures and `move` — Stage 13

`ipc.rs :: case_intro_in_background`:

```rust
spawn_blocking(move || case_intro_from(&cases_dir, &slug))
```

`|| …` is a function with no arguments, like `() => …` in TypeScript. `move` means the closure takes
ownership of the variables it uses (`cases_dir`, `slug`), so it can take them to another thread.
In `start`, the variables are `engine` and `inbox`.

### 5.5 The last line is what the function gives back

`state.rs :: ask`, its last line:

```rust
Ok(reply)
```

No `return`, no `;`: the last line of a function, without `;`, is its return value.

### 5.6 Calling `reply`, and lending a `String` with `&`

`state.rs :: ask`:

```rust
let reply = self.engine.reply(question)?;
self.record(Speaker::Suspect, &reply)?;
```

Line 1 asks the engine and keeps the result in `reply`. In `answer_all` the engine is the parameter
`engine`, so there is no `self.`. And no `?`: a failed reply is still an answer, and goes back to the
asker as an `Err`.

Line 2 lends a `String` (`reply`) where a `&str` is expected, with `&`. Same in `answer_all`:
`reply` wants `prompt: &str`, and `question.text` is a `String`, so you write `&question.text`.

### 5.7 `for` over a list

`case_file.rs :: try_from`, the last loop:

```rust
for raw_suspect in &raw.suspects {
    let suspect_id = SuspectId::new(raw_suspect.id);

    if case.suspect_facts(suspect_id).next().is_none() {
        return Err(AppError::SuspectKnowsNothing { id: suspect_id });
    }
}
```

### 5.8 A struct with `pub` fields

`case.rs`:

```rust
pub struct Suspect {
    pub id: SuspectId,
    pub name: String,
}
```

`Question`'s fields are `pub` for the same reason: the tests, outside the file, build one directly.

## 6. What you built

In order, for one question:

1. The test calls `start(engine)`.
2. `start` makes the question pipe: `questions` (Sender) and `inbox` (Receiver).
3. `start` starts the engine thread, and moves `engine` and `inbox` into it. From now on, only that
   thread has the engine.
4. `start` gives `questions` back to the test.
5. The test makes its own answer pipe (`answer_to`, `answer`) and sends
   `Question { text, answer_to }` into `questions`.
6. On the engine thread, the `for` loop wakes up with that question, calls
   `engine.reply(&question.text)`, and sends the reply into `question.answer_to`.
7. The test's `answer.recv()` wakes up with the reply.
8. The loop goes back to waiting. When the test ends, `questions` is dropped, the loop ends, and the
   thread finishes.

New in this stage:

- **A channel:** `channel()` gives a Sender and a Receiver. Values go in one end and come out the
  other, on another thread.
- **`let (a, b) = …`** takes a pair apart.
- **`thread::spawn(move || …)`** starts a thread that runs on its own.
- **`for x in receiver`** waits for each value, and ends when every Sender is gone.
- **`send` moves the value.** `let _ =` ignores a `Result` on purpose.

Next, 14b: `AppState` keeps `questions`, and `ask` sends its question through it. Then the game
itself uses the engine thread.
