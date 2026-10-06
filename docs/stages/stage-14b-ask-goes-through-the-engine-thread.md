# Stage 14b — `ask` goes through the engine thread

Test:  src-tauri/tests/ask_engine_thread.rs — 3 tests
Run:   cd src-tauri && cargo test --test ask_engine_thread

## 1. What part of the app

The player types a question. Today this happens:

```
React → ask_suspect (ipc.rs) → AppState::ask (state.rs) → self.engine.reply(question)
```

The engine answers inside `ask`, on whichever thread is running `ask`.

In 14a you gave the engine a thread of its own (`engine_thread.rs`). Only the tests use it so far.
This stage connects the game to it:

```
React → ask_suspect (ipc.rs) → AppState::ask (state.rs)
      → ask sends a Question into `questions`
      → the engine thread (engine_thread.rs :: answer_all) calls engine.reply(…)
        and sends the reply back
      → ask receives the reply, keeps it in the room, gives it to React
```

What changes:

- `AppState` stops holding the engine. It holds `questions`: the Sender that `start` gives back (14a).
- `AppState::new` still receives the engine, and hands it to `start`. From then on, only the engine
  thread has it.
- `ask` sends each question to that thread, and waits for the answer.

What does not change: `ipc.rs`, `lib.rs`, React, and all the older tests. `AppState::new` takes the
same two inputs as before, so everything that calls it stays as it is.

Almost nothing here is new Rust. You type `channel()`, `send` and `recv()` yourself — you met them in
14a — and the waiting uses `spawn_blocking` from Stage 13. One thing you have not seen yet: three `Result`s
inside each other (§4.1).

## 2. Files and templates

| File | What you do |
|---|---|
| `src/state.rs` | paste the template in three places, then write one line in `new` and the body of `ask` |

Nothing else changes.

**Place 1 — the imports at the top of `state.rs`.** Replace all of them with:

```rust
use crate::engine_thread::{start, Question};
use crate::error::{AppError, AppResult};
use crate::ids::SuspectId;
use crate::llm::InferenceEngine;
use crate::transcript::{Phase, Speaker};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use tauri::async_runtime::spawn_blocking;
```

Three lines are new: `start, Question` (14a), `channel, Sender` (14a), `spawn_blocking` (Stage 13).

**Place 2 — the struct and `new`.** Replace everything from `/// What the app holds on to…` down to
the end of `new` with:

```rust
/// What the app holds on to for as long as it runs.
pub struct AppState {
    pub cases_dir: PathBuf,
    phase: Mutex<Phase>,
    /// Where questions for the engine go. The engine itself lives on its own thread.
    questions: Sender<Question>,
}

impl AppState {
    pub fn new(cases_dir: PathBuf, engine: Box<dyn InferenceEngine>) -> Self {
        Self {
            cases_dir,
            phase: Mutex::new(Phase::Briefing),
            questions: todo!(),
        }
    }
```

The field `engine: Box<dyn InferenceEngine>` is gone. `questions: Sender<Question>` takes its place.

**Place 3 — `ask`.** Replace the whole function, doc comment included, with:

```rust
    /// The suspect's reply to what the detective just asked.
    /// The engine answers on its own thread. `ask` waits for that answer on a background thread.
    pub async fn ask(&self, question: &str) -> AppResult<String> {
        todo!()
    }
```

The old body is in §5.1. You will type three of its lines again.

Run: `0 passed; 3 failed`, and 7 warnings (unused things, and one `unreachable expression`). Each
test stops with `not yet implemented` inside `new`: that is the `todo!()` in the `questions` line.
The warnings go away as you write the code.

What each part is:

- `questions` — the slot where `ask` drops questions for the engine. It is the Sender end of the
  channel that `start` makes (§5.2).
- `new` — still receives the engine. Now it passes it on to `start`, instead of keeping it.
- `ask` — the same job as before: keep the question, get the reply, keep the reply, give it back.
  Only the middle changes: the reply now comes from the engine thread.

## 3. What each function does, and how to build it

### Where the inputs come from

- **`engine`, in `new`.** `lib.rs :: run` makes it and passes it to `AppState::new`:
  ```rust
  .manage(AppState::new(
      PathBuf::from("cases"),
      Box::new(MockEngine::new("I have nothing to say to you.")),
  ))
  ```
  In the tests, `viktor_in_the_room(Box::new(WhereAmIEngine))` passes it.
- **`question`, in `ask`.** The text the player typed, as a `&str`. It comes from
  `ipc.rs :: ask_suspect`:
  ```rust
  state.ask(&question).await
  ```
  Careful with the two names: `question` (small q) is that text. `Question` (capital Q) is the
  struct from 14a that carries a text to the engine.
- **`self.questions`, in `ask`.** The Sender that `new` keeps.

### `new` — step by step

What it does: gives the engine to `start`, and keeps the Sender that `start` gives back.

1. In the `questions:` line, replace `todo!()` with a call to `start`, giving it `engine`.
   Syntax: §5.2 (what `start` takes and gives back) and §5.3 (a field filled by a function call).

Run: still `0 passed; 3 failed`, now 4 warnings. Each test now stops with `not yet implemented`
inside `ask`. That means `new` works: the engine thread is running.

### `ask` — step by step

What it does: keeps the question in the room, sends it to the engine thread, waits for the answer
on a background thread, keeps the answer in the room, and gives it back.

Delete the `todo!()`, then write these lines, in this order:

1. Keep the question in the room: call `self.record` with `Speaker::Detective` and `question`, with
   `?` at the end. Same line as in the old `ask` (§5.1).
2. Make a channel for the answer. Name its two ends `answer_to` (the Sender) and `answer` (the
   Receiver). Syntax: §5.4.
3. Send a `Question` into `self.questions`. Its `text` is `question` turned into a `String`. Its
   `answer_to` is your `answer_to`. Then turn `send`'s error into `AppError::Inference` with
   `.map_err`, and end with `?`. Syntax: §5.5 (`.to_string()`), §5.6 (building and sending a
   `Question`), §5.7 (`.map_err`).
4. Wait for the answer: call `spawn_blocking` with a `move` closure that calls `answer.recv()`.
   Then `.await`, then `.map_err` into `AppError::BackgroundFailed`, then `?`. Keep the result in
   `waited`. Syntax: §5.8. What `waited` holds: §4.1.
5. Turn `waited`'s error into `AppError::Inference` with `.map_err`, then `?`. Keep the result in
   `answered`. Syntax: §4.1 and §5.7.
6. Put `?` after `answered`, and keep the result in `reply`. Syntax: §4.1.
7. Keep the reply in the room, and give it back: the last two lines of the old `ask` (§5.1).

Run: `3 passed; 0 failed`.

Last: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (170 tests). `cargo fmt`
may move the line breaks in step 4 — that is fine. Say ready.

### If it does not compile

| the compiler says | fix |
|---|---|
| `E0425 cannot find value questions in this scope`, in `new` | The field needs its value after a `:` — `questions: start(engine),` (§5.3). |
| `E0425 cannot find value questions in this scope`, in `ask` | It is a field: `self.questions` (§5.6). |
| `E0308 mismatched types` … `expected String, found &str` | `text: question.to_string()` (§5.5). |
| `E0277 ? couldn't convert the error to AppError` | A `.map_err(…)` is missing before a `?`: at the end of step 3, or in step 5 (§4.1). |
| `E0277 Receiver<…> cannot be shared between threads safely` | Put `move` before `\|\|` in step 4 (§5.8). |
| `E0599 no method named map_err found for enum JoinHandle` | `.await` is missing before `.map_err` in step 4 (§5.8). |
| `E0308 mismatched types` … `expected &str, found &Result<String, AppError>` (and a second one at `Ok(reply)`) | The `?` after `answered` is missing (step 6, §4.1). |
| clippy: `unused std::result::Result that must be used` | The `?` at the end of step 3 is missing. |
| clippy: `unused import: tauri::async_runtime::spawn_blocking` | You called `answer.recv()` without `spawn_blocking`. The tests pass that way, but the game would stall (§5.8). Write step 4 as it says. |
| a test `has been running for over 60 seconds` | Step 4 (wait) comes before step 3 (send), so no answer can ever come. Swap them. Stop the run with Ctrl+C. |

## 4. New syntax and methods

### 4.1 Three `Result`s, one inside the other

In Stage 13, `spawn_blocking(…).await` gave you two results, one inside the other: did the thread
finish, and did the case load (§5.8). Here there are three.

Build it from the inside out:

- `answer` carries an `AppResult<String>`: the engine's own answer. It is `Ok(text)`, or `Err` when
  the engine failed.
- `answer.recv()` puts a `Result` around it: an answer arrived, or `RecvError` — "no answer can ever
  come".
- `spawn_blocking(…).await` puts one more `Result` around that: the background thread finished, or
  it crashed (`tauri::Error`).

So step 4's line, before its `?`, has this type. The compiler writes it like this:

```
Result<Result<Result<String, AppError>, RecvError>, tauri::Error>
```

Read it as boxes, one inside the next. Steps 4, 5 and 6 open one box each, with `?`:

| step | the line | opens | its error means | becomes |
|---|---|---|---|---|
| 4 | `let waited = spawn_blocking(…).await.map_err(…)?;` | the outer box | the background thread crashed | `AppError::BackgroundFailed` |
| 5 | `let answered = waited.map_err(…)?;` | the middle box | the engine thread is gone: no answer will ever come | `AppError::Inference` |
| 6 | `let reply = answered?;` | the inner box | the engine answered with an error | stays as it is: it is already an `AppError` |

After step 6, `reply` is a plain `String`.

Two rules you already know make this work:

- `?` takes the `Ok` value out, or stops `ask` and gives back the error.
- `?` only passes on an `AppError`, because `ask` returns `AppResult<String>`. So a box whose error
  is something else (`tauri::Error`, `RecvError`) needs `.map_err` first. The inner box already
  holds an `AppError`, so it needs nothing.

`?` works on a variable too: `answered?` is the same `?` as at the end of a call.

**When the middle box fails.** The test `a_crashed_engine_is_an_error_not_a_crash` shows it. The
engine calls `panic!`, and its thread stops. Everything that thread held is thrown away — including
the question's `answer_to`. With no Sender left, `recv()` stops waiting and returns an error. Its
message is `receiving on a closed channel`, and step 5 turns it into `AppError::Inference`. The game
gets an error it can show the player, and keeps running.

**When to use it:** whenever one value carries several things that could have failed on the way.
Open one box per line.

**Why here:** three different things can fail between asking and getting a reply. Each one becomes
an `AppError`, so React can tell them apart by `kind`, and none of them crashes the game.

## 5. Syntax you already know — from your code

### 5.1 The old `ask`

`state.rs :: ask`, before this stage:

```rust
pub async fn ask(&self, question: &str) -> AppResult<String> {
    self.record(Speaker::Detective, question)?;
    let reply = self.engine.reply(question)?;
    self.record(Speaker::Suspect, &reply)?;
    Ok(reply)
}
```

The first line is your step 1. The last two lines are your step 7. The middle line goes away:
`AppState` no longer has an engine.

### 5.2 `start` — what it takes and gives back

`engine_thread.rs`:

```rust
pub fn start(engine: Box<dyn InferenceEngine>) -> Sender<Question> {
    let (questions, inbox) = channel();
    thread::spawn(move || answer_all(engine, inbox));
    questions
}
```

It takes the engine, and gives back a `Sender<Question>`: exactly the type of the new field. Calling
it starts the engine thread.

### 5.3 A field filled by calling a function

`state.rs :: new`:

```rust
Self {
    cases_dir,
    phase: Mutex::new(Phase::Briefing),
    engine,
}
```

`phase: Mutex::new(Phase::Briefing)` fills the field with what the call gives back.

`cases_dir,` alone is short for `cases_dir: cases_dir,`. That works only when a variable has the
same name as the field. There is no variable called `questions` in `new`, so that line needs the
long form: the field name, `:`, then the call.

### 5.4 `channel()` and `let (a, b)`

`engine_thread.rs :: start`:

```rust
let (questions, inbox) = channel();
```

And in `tests/engine_thread.rs`, a channel for one answer — the same as your step 2:

```rust
let (answer_to, answer) = channel();
```

The Sender comes first, the Receiver second. You do not write the type: Rust works it out from the
`Question` you put `answer_to` in.

The same test then waits for the answer with `recv()`:

```rust
assert_eq!(
    answer.recv(),
    Ok(Ok("I was at home all night.".to_string()))
);
```

`recv()` waits until a value arrives in the Receiver. `Ok(Ok(…))` is the middle and inner boxes of
§4.1: an answer arrived, and the engine did not fail.

### 5.5 `&str` to `String` with `.to_string()`

`llm.rs :: MockEngine::new`:

```rust
pub fn new(line: &str) -> Self {
    Self {
        line: line.to_string(),
    }
}
```

`line` is a `&str`, and the field wants a `String`. `.to_string()` makes a `String` that owns its own
copy of the text. Same in step 3: `question` is a `&str`, and `Question`'s `text` is a `String`. It
must own its copy, because it travels to the engine thread.

### 5.6 Building a `Question` and sending it

`tests/engine_thread.rs :: a_question_sent_in_gets_its_answer_back`:

```rust
questions
    .send(Question {
        text: "Where were you on Tuesday?".to_string(),
        answer_to,
    })
    .expect("the engine thread is running");
```

`answer_to,` alone is short for `answer_to: answer_to,` (§5.3).

In `ask`, three things are different:

- The Sender is a field of `AppState`: `self.questions.send(…)`. Same shape as `self.phase.lock()`
  in your `begin`.
- `text` is `question.to_string()`.
- No `.expect(…)`: that would crash the game if the engine thread were gone. Use `.map_err(…)?`
  instead (§5.7).

`send` gives back `Err` only when nobody holds the Receiver any more — here, when the engine thread
has stopped. In 14a, `answer_all` ignored that error with `let _ =`: if the asker had gone, there was
nobody to tell. Here it is the other way round: if the engine is gone, the player must be told.

### 5.7 `.map_err(…)` then `?`

`state.rs :: begin`:

```rust
let mut phase = self.phase.lock().map_err(|e| AppError::Poisoned {
    message: e.to_string(),
})?;
```

`.map_err` turns the error into an `AppError`. `e.to_string()` keeps the error's message. `?`
passes it up. In steps 3 and 5 the variant is `Inference`. Its definition, in `error.rs`:

```rust
#[error("the inference engine failed: {message}")]
Inference { message: String },
```

### 5.8 `spawn_blocking(move || …)`, then `.await`, `.map_err`, `?`

`ipc.rs :: case_intro_in_background`:

```rust
spawn_blocking(move || case_intro_from(&cases_dir, &slug))
    .await
    .map_err(|e| AppError::BackgroundFailed {
        message: e.to_string(),
    })?
```

Step 4 has the same shape. Its closure is `move || answer.recv()`. `move` hands `answer` to the
background thread. Without `move`, the closure would only borrow `answer`, and a Receiver cannot be
used from another thread through a borrow. That is the `E0277 … cannot be shared between threads
safely` error.

**Why `recv()` goes inside `spawn_blocking`.** `recv()` waits by stopping the thread it runs on.
`ask` is `async`, so it runs on one of a small, fixed group of threads that run all the app's async
work, other commands included. With the real model, an answer takes seconds. Calling `recv()`
directly would stop one of those threads for all that time. `spawn_blocking` moves the waiting to a
thread made for waiting, and `.await` lets `ask` pause without stopping anything.

The tests cannot see this: they pass either way. Clippy can. `spawn_blocking` is in the imports, so
if you do not use it, `unused import` fails the run.

## 6. What you built

In order, for one question in the game:

1. When the app starts, `lib.rs` calls `AppState::new` with the `MockEngine`.
2. `new` hands the engine to `start`. The engine thread starts and waits for questions. `AppState`
   keeps only `questions`.
3. The player asks. React calls `ask_suspect`, which calls `ask`.
4. `ask` keeps the question in the room.
5. `ask` makes a channel for the answer, and sends `Question { text, answer_to }` into `questions`.
6. On the engine thread, `answer_all` wakes up, calls `engine.reply(&question.text)`, and sends the
   reply into `answer_to`.
7. Meanwhile `ask` waits: `recv()` runs on a background thread (`spawn_blocking`), and `.await`
   lets `ask` pause without stopping any other work.
8. The reply arrives. `ask` opens the three results, one per line (§4.1), keeps the reply in the
   room, and gives it to React.

If the engine fails, its error comes back through the same channel, like any answer. If the engine
thread crashes, `ask` gives back `AppError::Inference`, and the game keeps running.

New in this stage:

- **Three `Result`s inside each other**: open one per line with `?`, and put `.map_err` first when
  the error is not an `AppError`.
- Everything else you already knew, used in a new place: `channel()`, `send` and `recv` (14a);
  `spawn_blocking`, `move` and `.await` (Stage 13).

Next, Stage 15: sharing one value between threads — `Arc<Mutex<T>>`, and why a `Mutex` lock must
be let go before an `.await`.
