# Stage 14b — `ask` goes through the engine thread

Test:  src-tauri/tests/ask_engine_thread.rs — 3 tests
Run:   cd src-tauri && cargo test --test ask_engine_thread

## 1. What part of the app

The player types a question. Today this happens:

```
React → ask_suspect (ipc.rs) → AppState::ask (state.rs) → self.engine.reply(text)
```

The engine answers inside `ask`, on whichever thread is running `ask`.

In 14a you gave the engine a thread of its own (`engine_thread.rs`). Only the tests use it so far.
This stage makes the game use it. `ipc.rs`, `lib.rs`, React and the older tests do not change.

### The map: who holds what

Read this before any code. Every line you write in this stage moves one of these things.

```
  THE GAME SIDE                                    THE ENGINE THREAD
                                                   (started once, by start)

  AppState (lives as long as the app)
    questions  ──────────── pipe 1 ────────────▶   inbox
    Sender<Question>        carries Questions      Receiver<Question>

                                                   engine
                                                   Box<dyn InferenceEngine>

  one call to ask (lives for one question)
    answer     ◀─────────── pipe 2 ─────────────   question.answer_to
    Receiver<AppResult<String>>  carries 1 reply   Sender<AppResult<String>>
```

A **pipe** is a channel (14a). It has two ends:

- the **Sender** end — you put a value in with `.send(value)`;
- the **Receiver** end — the value comes out there, on the other thread, with `.recv()` or a `for` loop.

A Sender is a value that *has* a method `send`. It is not a function. You write
`self.questions.send(question)`, never `self.questions(…)`.

| name | what it is | who holds it | made where |
|---|---|---|---|
| `questions` | the Sender end of pipe 1 | `AppState`, as a field | in `start`, once, when the app starts |
| `inbox` | the Receiver end of pipe 1 | the engine thread | in `start` |
| `engine` | the suspect's brain (`MockEngine` today) | the engine thread, and nobody else | in `lib.rs`; `new` hands it to `start` |
| `answer_to` | the Sender end of pipe 2 | travels inside the `Question` | in `ask`, a new one for every question |
| `answer` | the Receiver end of pipe 2 | `ask` | in `ask` |
| `question` | one `Question`: `text` + `answer_to` | built in `ask`, then sent through pipe 1 | in `ask` |

Why two pipes: pipe 1 is shared by every question, so it cannot also carry the answers back — an
answer could reach the wrong asker. So each `ask` makes its own small pipe 2, and puts its Sender
end inside the question: "send the reply here".

### One question, start to end

1. `ask` makes pipe 2: `answer_to` and `answer`.
2. `ask` builds one `Question` holding the player's text and `answer_to`.
3. `ask` sends that `Question` into pipe 1. From now on, `answer_to` is with the engine thread.
4. On the engine thread, the `for` loop wakes up with the `Question`, calls
   `engine.reply(&question.text)`, and sends the reply into `question.answer_to`. **This is your 14a
   code** (§5.2). It already works.
5. The reply comes out of `answer`, back in `ask`. `ask` waits for it with `answer.recv()`, on a
   background thread (§5.8).
6. `ask` keeps the reply in the room, and gives it back to React.

In this stage you write steps 1, 2, 3, 5 and 6, inside `ask`.

## 2. Files and templates

| File | What you do |
|---|---|
| `src/state.rs` | paste the template in three places, then write one line in `new` and the body of `ask` |

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

**Place 3 — `ask`.** Replace the whole function, doc comment included, with:

```rust
    /// The suspect's reply to what the detective just asked.
    /// The engine answers on its own thread. `ask` waits for that answer on a background thread.
    pub async fn ask(&self, text: &str) -> AppResult<String> {
        todo!()
    }
```

The input is now called `text`, not `question`: in this function, `question` is the `Question` you
build (§1). If you pasted an earlier version of this template, rename it in the first line of `ask`.

Run: `0 passed; 3 failed`, and 7 warnings (unused things, and one `unreachable expression`). Each
test stops with `not yet implemented` inside `new`: that is the `todo!()` in the `questions` line.

## 3. What each function does, and how to build it

### Where the inputs come from

- **`engine`, in `new`.** `lib.rs :: run` makes it and passes it to `AppState::new`:
  ```rust
  .manage(AppState::new(
      PathBuf::from("cases"),
      Box::new(MockEngine::new("I have nothing to say to you.")),
  ))
  ```
- **`text`, in `ask`.** What the player typed, as a `&str`. It comes from `ipc.rs :: ask_suspect`:
  ```rust
  state.ask(&question).await
  ```
- **`self.questions`, in `ask`.** The Sender of pipe 1, which `new` keeps.

### `new`

Replace the `todo!()` in the `questions` line. Type:

```rust
            questions: start(engine),
```

`start` (your 14a code, §5.2) makes pipe 1, starts the engine thread, and moves `engine` and `inbox`
into it. It gives back pipe 1's Sender. The field `questions` keeps it. In the map (§1), this line
draws everything on the right side, and the `questions` end on the left.

Run: still `0 passed; 3 failed`, now 4 warnings. Each test now stops with `not yet implemented`
inside `ask`. So `new` works: the engine thread is running.

### `ask`

Delete the `todo!()`. Type the blocks below, in this order. Under each block: what it does in the
map. Syntax is explained in §4 and §5, where the block points.

**Block 1 — keep the player's line in the room.**

```rust
        self.record(Speaker::Detective, text)?;
```

Same as the first line of the old `ask` (§5.1), with the input's new name.

**Block 2 — make pipe 2.** (§1, step 1)

```rust
        let (answer_to, answer) = channel();
```

`channel()` makes a new pipe and gives back both ends: `answer_to` is the Sender, `answer` the
Receiver. This pipe is only for this one reply. Syntax: §5.4.

**Block 3 — build the `Question`.** (§1, step 2)

```rust
        let question = Question {
            text: text.to_string(),
            answer_to,
        };
```

One value, two fields — the struct from 14a. `text.to_string()` copies the player's words into a
`String` that the `Question` owns, because the words travel to another thread (§5.5). `answer_to,`
puts pipe 2's Sender inside, so the engine knows where to reply (short for `answer_to: answer_to,`,
§5.3). After this line, `answer_to` belongs to `question`; you cannot use it on its own any more.

**Block 4 — send it into pipe 1.** (§1, step 3)

```rust
        self.questions
            .send(question)
            .map_err(|e| AppError::Inference {
                message: e.to_string(),
            })?;
```

`self.questions` is pipe 1's Sender. `.send(question)` puts the `Question` in; on the engine thread,
the `for` loop in `answer_all` wakes up with it (§1, step 4). `send` fails only when the engine
thread is gone. Then `.map_err` turns that failure into `AppError::Inference`, and `?` stops `ask`
with it (§5.6, §5.7).

**Block 5 — wait for the reply, on a background thread.** (§1, step 5)

```rust
        let waiting = spawn_blocking(move || answer.recv());
        let waited = waiting.await.map_err(|e| AppError::BackgroundFailed {
            message: e.to_string(),
        })?;
```

Line 1: `answer.recv()` waits until the reply comes out of pipe 2. `spawn_blocking` runs that wait
on a background thread, so no game thread is stopped while the engine thinks. `move` hands `answer`
to that thread. `waiting` is the handle to that background job (§5.8).

Line 2: `.await` pauses `ask` until the background job has finished. If that thread crashed,
`.map_err` turns it into `AppError::BackgroundFailed`, and `?` stops `ask` with it. `waited` is
what `recv()` gave back — §4.1 shows what that is.

**Block 6 — take the reply out.**

```rust
        let answered = waited.map_err(|e| AppError::Inference {
            message: e.to_string(),
        })?;
        let reply = answered?;
```

`waited` says whether a reply came out of pipe 2 at all. If the engine thread died, none can come:
that becomes `AppError::Inference`. `answered` is the engine's own answer: `Ok(the text)`, or the
engine's error. `answered?` takes the text out, or passes the engine's error up as it is. After
these two lines, `reply` is a plain `String` (§4.1).

**Block 7 — keep the reply in the room, give it back.** (§1, step 6)

```rust
        self.record(Speaker::Suspect, &reply)?;
        Ok(reply)
```

The last two lines of the old `ask` (§5.1).

Run: `3 passed; 0 failed`.

Last: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (170 tests). Say ready.

### If it does not compile

| the compiler says | fix |
|---|---|
| `E0599 no method named questions found for reference &AppState` … `field, not a method` | `questions` is a Sender, not a function: `self.questions.send(question)` (block 4). |
| `E0599 no variant named Engine found for enum AppError` | The variant is `AppError::Inference` (§5.7). |
| `E0308 mismatched types` … `expected Result<String, AppError>, found ()` | The end of `ask` is missing: blocks 5 to 7. The last line must be `Ok(reply)`, with no `;`. |
| `E0308 mismatched types` … `expected String, found &str` | Write `text: text.to_string()`, not `text` alone (block 3). |
| `E0382 use of moved value: answer_to` | `answer_to` now belongs to `question` (block 3). Do not use it after that. |
| `E0277 ? couldn't convert the error to AppError` | A `.map_err(…)` is missing before a `?`: in block 4 or block 6 (§4.1). |
| `E0277 Receiver<…> cannot be shared between threads safely` | Put `move` before `\|\|` in block 5 (§5.8). |
| `E0599 no method named map_err found for enum JoinHandle` | Write `waiting.await.map_err(…)`: the `.await` is missing (block 5). |
| `E0308 mismatched types` … `expected &str, found &Result<String, AppError>` (and a second one at `Ok(reply)`) | The `?` after `answered` is missing (block 6). |
| clippy: `unused std::result::Result that must be used` | The `?` at the end of block 4 is missing. |
| clippy: `unused import: tauri::async_runtime::spawn_blocking` | You called `answer.recv()` without `spawn_blocking`. The tests pass that way, but the game would stall (§5.8). Write block 5 as shown. |
| a test `has been running for over 60 seconds` | Block 5 (wait) is before block 4 (send), so no reply can ever come. Swap them. Stop the run with Ctrl+C. |

## 4. New syntax and methods

### 4.1 Three `Result`s, one inside the other

In Stage 13, `spawn_blocking(…).await` gave you two results, one inside the other: did the thread
finish, and did the case load (§5.8). Here there are three. Build them from the inside out:

- `answer` carries an `AppResult<String>`: the engine's own answer. `Ok(text)`, or `Err` when the
  engine failed.
- `answer.recv()` puts a `Result` around it: a reply came out, or `RecvError` — "no reply can ever
  come".
- `spawn_blocking(…)` + `.await` puts one more `Result` around that: the background thread finished,
  or it crashed (`tauri::Error`).

So `waiting.await`, before its `?`, has this type. The compiler writes it like this:

```
Result<Result<Result<String, AppError>, RecvError>, tauri::Error>
```

Read it as boxes, one inside the next. Blocks 5 and 6 open one box per line, with `?`:

| block | the line | opens | its error means | becomes |
|---|---|---|---|---|
| 5 | `let waited = waiting.await.map_err(…)?;` | the outer box | the background thread crashed | `AppError::BackgroundFailed` |
| 6 | `let answered = waited.map_err(…)?;` | the middle box | the engine thread is gone: no reply will ever come | `AppError::Inference` |
| 6 | `let reply = answered?;` | the inner box | the engine answered with an error | stays as it is: it is already an `AppError` |

After the last line, `reply` is a plain `String`.

Two rules you already know make this work:

- `?` takes the `Ok` value out, or stops `ask` and gives back the error.
- `?` only passes on an `AppError`, because `ask` returns `AppResult<String>`. So a box whose error
  is something else (`tauri::Error`, `RecvError`) needs `.map_err` first. The inner box already
  holds an `AppError`, so it needs nothing.

`?` works on a variable too: `answered?` is the same `?` as at the end of a call.

**When the middle box fails.** The test `a_crashed_engine_is_an_error_not_a_crash` shows it. The
engine calls `panic!`, and its thread stops. Everything that thread held is thrown away — including
the question's `answer_to`. With no Sender left, `recv()` stops waiting and returns an error. Its
message is `receiving on a closed channel`, and block 6 turns it into `AppError::Inference`. The game
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

The first line is block 1. The last two lines are block 7. The middle line goes away: `AppState` no
longer has an engine. Blocks 2 to 6 take its place.

### 5.2 `start` and `answer_all` — the right side of the map

`engine_thread.rs`:

```rust
pub fn start(engine: Box<dyn InferenceEngine>) -> Sender<Question> {
    let (questions, inbox) = channel();
    thread::spawn(move || answer_all(engine, inbox));
    questions
}

fn answer_all(engine: Box<dyn InferenceEngine>, inbox: Receiver<Question>) {
    for question in inbox {
        let reply = engine.reply(&question.text);
        let _ = question.answer_to.send(reply);
    }
}
```

`start` takes the engine and gives back a `Sender<Question>`: exactly the type of the new field.
`answer_all` is the other half of every `ask`: it takes each `Question` out of pipe 1 and sends the
reply into that question's `answer_to`.

### 5.3 Filling struct fields

`state.rs :: new`, before this stage:

```rust
Self {
    cases_dir,
    phase: Mutex::new(Phase::Briefing),
    engine,
}
```

`phase: Mutex::new(Phase::Briefing)` fills the field with what the call gives back — like
`questions: start(engine)`.

`cases_dir,` alone is short for `cases_dir: cases_dir,`. That works only when a variable has the same
name as the field. In block 3, `answer_to,` works that way. `text,` alone would not: the variable
`text` is a `&str`, and the field wants a `String`.

### 5.4 `channel()` and `let (a, b)`

`engine_thread.rs :: start`:

```rust
let (questions, inbox) = channel();
```

And in `tests/engine_thread.rs`, a channel for one reply — the same as block 2:

```rust
let (answer_to, answer) = channel();
```

The Sender comes first, the Receiver second. You do not write the type: Rust works it out from the
`Question` you put `answer_to` in.

The same test then waits for the reply with `recv()`:

```rust
assert_eq!(
    answer.recv(),
    Ok(Ok("I was at home all night.".to_string()))
);
```

`recv()` waits until a value comes out of the Receiver. `Ok(Ok(…))` is the middle and inner boxes of
§4.1: a reply came out, and the engine did not fail.

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
copy of the text. Same in block 3: `text` is a `&str`, and `Question`'s field `text` is a `String`.
It must own its copy, because it travels to the engine thread.

### 5.6 Building a `Question` and sending it — from the 14a test

`tests/engine_thread.rs :: a_question_sent_in_gets_its_answer_back`:

```rust
questions
    .send(Question {
        text: "Where were you on Tuesday?".to_string(),
        answer_to,
    })
    .expect("the engine thread is running");
```

Blocks 3 and 4 do the same thing in two steps: build the `Question` and keep it in `question`, then
`.send(question)`. Three things are different in `ask`:

- The Sender is a field of `AppState`, so it is `self.questions` — like `self.phase` in your `begin`.
- `text` is `text.to_string()`.
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

`.map_err` turns the error into an `AppError`. `e.to_string()` keeps the error's message. `?` passes
it up. In blocks 4 and 6 the variant is `Inference`. Its definition, in `error.rs`:

```rust
#[error("the inference engine failed: {message}")]
Inference { message: String },
```

### 5.8 `spawn_blocking(move || …)`, then `.await`

`ipc.rs :: case_intro_in_background`:

```rust
spawn_blocking(move || case_intro_from(&cases_dir, &slug))
    .await
    .map_err(|e| AppError::BackgroundFailed {
        message: e.to_string(),
    })?
```

Block 5 is the same, split over two lines: `spawn_blocking(…)` first, kept in `waiting`, then
`waiting.await.map_err(…)?`. Its closure is `move || answer.recv()`. `move` hands `answer` to the
background thread. Without `move`, the closure would only borrow `answer`, and a Receiver cannot be
used from another thread through a borrow. That is the `E0277 … cannot be shared between threads
safely` error.

**Why `recv()` goes inside `spawn_blocking`.** `recv()` waits by stopping the thread it runs on.
`ask` is `async`, so it runs on one of a small, fixed group of threads that run all the app's async
work, other commands included. With the real model, a reply takes seconds. Calling `recv()` directly
would stop one of those threads for all that time. `spawn_blocking` moves the waiting to a thread
made for waiting, and `.await` lets `ask` pause without stopping anything.

The tests cannot see this: they pass either way. Clippy can. `spawn_blocking` is in the imports, so if
you do not use it, `unused import` fails the run.

## 6. What you built

In order, for one question in the game:

1. When the app starts, `lib.rs` calls `AppState::new` with the `MockEngine`.
2. `new` hands the engine to `start`. Pipe 1 and the engine thread exist. `AppState` keeps only
   `questions`.
3. The player asks. React calls `ask_suspect`, which calls `ask` with the text.
4. `ask` keeps the player's line in the room.
5. `ask` makes pipe 2, builds `Question { text, answer_to }`, and sends it into pipe 1.
6. On the engine thread, `answer_all` wakes up, calls `engine.reply(&question.text)`, and sends the
   reply into `question.answer_to`.
7. Meanwhile `ask` waits: `recv()` runs on a background thread, and `.await` lets `ask` pause.
8. The reply comes out of `answer`. `ask` opens the three boxes (§4.1), keeps the reply in the room,
   and gives it to React.

If the engine fails, its error comes back through pipe 2, like any reply. If the engine thread
crashes, `ask` gives back `AppError::Inference`, and the game keeps running.

New in this stage:

- **Three `Result`s inside each other**: open one per line with `?`, and put `.map_err` first when
  the error is not an `AppError`.
- Everything else you already knew, used in a new place: `channel()`, `send` and `recv()` (14a);
  `spawn_blocking`, `move` and `.await` (Stage 13).

Next, Stage 14c: see what `ask` does — the moves, the waiting and the three results, shown by the
compiler and by the running tests.
