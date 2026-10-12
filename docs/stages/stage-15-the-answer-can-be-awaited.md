# Stage 15 — the answer can be awaited

Test:  src-tauri/tests/engine_thread.rs — 4 tests, and src-tauri/tests/ask_engine_thread.rs — 3 tests
Run:   cd src-tauri && cargo test --test engine_thread --test ask_engine_thread

## 1. What part of the app

The same path as in 14b:

```
React → ask_suspect (ipc.rs) → AppState::ask (state.rs) → pipe 1 → engine thread → pipe 2 → back in ask
```

This stage changes **pipe 2**, and **how `ask` waits** on it.

In 14b and 14c, `ask` waited like this:

```rust
        let waiting = spawn_blocking(move || answer.recv());
        let waited = waiting.await.map_err(|e| AppError::BackgroundFailed {
            message: e.to_string(),
        })?;
```

`answer.recv()` blocks a whole thread. So `ask` borrowed a background thread to do the blocking,
and awaited that thread. Each question kept one background thread busy doing nothing but wait.

After this stage, `ask` waits like this:

```rust
        let answered = answer.recv().await.ok_or(AppError::Inference {
            message: "the engine stopped before it answered".to_string(),
        })?;
```

Pipe 2 becomes a different kind of channel: one whose receiving end can be **awaited**. No
background thread. `ask` pauses at `.await`, and wakes up when the reply comes out of the pipe.

**Why now.** In Stage 16 the suspect's reply will come in pieces, word by word, so that React can
show it being typed. `ask` will then wait for many pieces per question. One background thread per
piece would be wasteful. An awaitable channel waits for each piece the same way it waits for one.

### The map

```
  THE GAME SIDE                                     THE ENGINE THREAD

  AppState
    questions  ──────────── pipe 1 ────────────▶    inbox
    std Sender<Question>    (no change)             std Receiver<Question>
                                                    engine

  one call to ask
    answer     ◀─────────── pipe 2 ─────────────    question.answer_to
    async Receiver          NEW KIND                async Sender
    ask: answer.recv().await                        engine thread: blocking_send(reply)
```

Now there are two kinds of channel. Here is how they differ:

|  | pipe 1 — std kind (14a) | pipe 2 — async kind (new) |
|---|---|---|
| made with | `std::sync::mpsc::channel()` | `tauri::async_runtime::channel(1)` |
| put a value in | `send(value)` — returns at once | `blocking_send(value)` — from a plain thread (§4.2) |
| take a value out | `for` loop, or `recv()` — **blocks** the thread | `recv().await` — **pauses** only the async function (§4.3) |
| who takes values out | the engine thread: a plain thread, blocking is its job | `ask`: async, it must never block |

The rule behind the table: **the end that `ask` waits on must be awaitable.** `ask` never waits on
pipe 1 — std's `send` returns at once — so pipe 1 stays as it is. `ask` does wait on pipe 2, so
pipe 2 changes kind.

| name | what it is now | who holds it |
|---|---|---|
| `answer_to` | the Sender end of pipe 2 — async kind | travels inside the `Question` to the engine thread |
| `answer` | the Receiver end of pipe 2 — async kind; must be `mut` (§4.3) | `ask` |

Everything else in the 14b map (`questions`, `inbox`, `engine`, `question`) stays the same.

### One question, start to end

The changed steps are in bold.

1. `ask` makes pipe 2 — **with `channel(1)`, the async kind**.
2. `ask` builds one `Question` with the text and `answer_to`.
3. `ask` sends it into pipe 1. (No change.)
4. The engine thread takes it out, calls `engine.reply`, and **puts the reply into `answer_to` with
   `blocking_send`**.
5. **`ask` awaits `answer.recv()`. No background thread.**
6. `recv` gives `Some(the reply)` — or `None` if the engine thread died. `ask` keeps the reply in the
   room and gives it back.

## 2. Files and templates

| File | What you do |
|---|---|
| `src/engine_thread.rs` | paste one struct (place 1); change one word in `answer_all` |
| `src/state.rs` | paste the imports (place 2) and one doc comment line (place 3); change one line; replace eight lines with three |
| `tests/engine_thread.rs`, `tests/ask_engine_thread.rs` | nothing — I changed them. The tests make pipe 2 the async kind too, and the crash test expects the new message. |

Run now, before you change anything: it does not compile — `5 × error[E0308]: mismatched types`, in
`tests/engine_thread.rs`. The tests already use the new kind of pipe 2; your code does not yet.

**Place 1 — `engine_thread.rs`, the `Question` struct.** Replace it with:

```rust
/// One question for the engine, and where to send its answer.
pub struct Question {
    pub text: String,
    pub answer_to: tauri::async_runtime::Sender<AppResult<String>>,
}
```

Only the type of `answer_to` changes. Why it is written with its full path: §4.4.

**Place 2 — `state.rs`, the imports.** Replace all of them with:

```rust
use crate::engine_thread::{start, Question};
use crate::error::{AppError, AppResult};
use crate::ids::SuspectId;
use crate::llm::InferenceEngine;
use crate::transcript::{Phase, Speaker};
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use tauri::async_runtime::channel;
```

- `std::sync::mpsc::Sender` stays: it is pipe 1's, for the `questions` field.
- `channel` now comes from `tauri::async_runtime`: it makes pipe 2.
- `spawn_blocking` is gone: `ask` no longer needs a background thread.

**Place 3 — `state.rs`, the second line of `ask`'s doc comment.** Replace it with:

```rust
    /// The engine answers on its own thread. `ask` awaits that answer.
```

## 3. What each function does, and how to build it

### `answer_all` (engine_thread.rs)

What it does: the same as in 14a — takes each question out of pipe 1, asks the engine, puts the
reply into that question's `answer_to`. Only the last step changes, because `answer_to` is now the
async kind.

After place 1, change the last line of the loop. Type:

```rust
        let _ = question.answer_to.blocking_send(reply);
```

In the map: the engine thread puts the reply into pipe 2. `blocking_send` is how a **plain** thread
puts a value into an async channel (§4.2). `let _ =` stays for the same reason as in 14a (§5.4).

Run: it does not compile — `6 previous errors`, all in `state.rs`: `2 × error[E0308]: mismatched
types` … `expected Sender<Result<String, AppError>>, found Sender<_>`, and `4 × error[E0277]: the
size for values of type str cannot be known at compilation time`. That is `ask` still making the old
kind of pipe 2. The `E0277`s are follow-on noise from the same line. Do not fix them one by one:
the next steps remove all six.

### `ask` (state.rs)

What it does: the same as in 14b. Only blocks 2 and 5–6 change: pipe 2 is made the async kind, and
`ask` awaits it directly.

After places 2 and 3:

**Block 2 — make pipe 2, the async kind.** Change the `channel` line to:

```rust
        let (answer_to, mut answer) = channel(1);
```

`channel(1)` makes an async channel with room for one value: the one reply (§4.1). `answer` is
`mut` because taking a value out changes the Receiver (§4.3, §5.3).

**Blocks 5 and 6 — await the reply.** Delete these eight lines:

```rust
        let waiting = spawn_blocking(move || answer.recv());
        let waited = waiting.await.map_err(|e| AppError::BackgroundFailed {
            message: e.to_string(),
        })?;

        let answered = waited.map_err(|e| AppError::Inference {
            message: e.to_string(),
        })?;
```

and type these three in their place:

```rust
        let answered = answer.recv().await.ok_or(AppError::Inference {
            message: "the engine stopped before it answered".to_string(),
        })?;
```

Keep the next line, `let reply = answered?;`, as it is.

In the map: `ask` pauses at `.await` until a value comes out of pipe 2. `recv()` gives back an
`Option`: `Some(the reply)`, or `None` when the engine thread died and no reply can come (§4.3).
`.ok_or(…)` turns `None` into an `AppError`, and `?` stops `ask` with it (§5.1). After this line,
`answered` is the engine's own answer, and `answered?` opens it, as in 14b.

Run: `3 passed; 0 failed` (ask_engine_thread) and `4 passed; 0 failed` (engine_thread).

Last: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (170 tests). Say ready.

### If it does not compile

| the compiler says | fix |
|---|---|
| after `answer_all` only: 6 errors in `state.rs`, two `E0308` and four `E0277 the size for values of type str` | Expected. Do the `ask` steps. |
| `E0061 this function takes 1 argument but 0 arguments were supplied` | The async kind needs a size: `channel(1)` (§4.1). |
| `E0061 this function takes 0 arguments but 1 argument was supplied`, and `E0308 mismatched types` | `channel` is still std's. Paste place 2's imports. |
| `E0596 cannot borrow answer as mutable, as it is not declared as mutable` | `let (answer_to, mut answer) = channel(1);` (§4.3). |
| `E0599 no method named ok_or found for opaque type impl Future` | `.await` is missing: `answer.recv().await.ok_or(…)` (§4.3). |
| `E0277 the ? operator can only be used on Results, not Options` | `.ok_or(…)` is missing before the `?` (§5.1). |
| a test panics with `Cannot block the current thread from within a runtime` | You wrote `answer.blocking_recv()` in `ask`. That is for plain threads. `ask` awaits: `answer.recv().await` (§4.3). |
| clippy: `non-binding let on a future` (and 2 tests fail with `the engine stopped before it answered`) | In `answer_all` you wrote `send(reply)`. A plain thread uses `blocking_send(reply)` (§4.2). |

## 4. New syntax and methods

### 4.1 An async channel: `tauri::async_runtime::channel(size)`

**What it is.** A pipe like 14a's — a Sender end and a Receiver end — but its receiving end can be
**awaited**. It comes from tokio, the library that runs all of Tauri's async code. Tauri passes it on
as `tauri::async_runtime::channel`, so there is no new dependency in `Cargo.toml`.

**Form.**

```rust
let (sender, mut receiver) = channel(size);
```

`size` is how many values the pipe can hold at once. When it is full, putting another value in
waits until one is taken out. std's channel has no size; this kind always has one.

**Example**, from `tests/engine_thread.rs`:

```rust
let (answer_to, mut answer) = channel(1);
```

**When to use it.** When async code must wait for values that another thread produces.

**Why here.** `ask` is async and must never block (14c, part B). Pipe 2 is the pipe it waits on.
`channel(1)`, because each question gets exactly one reply.

### 4.2 `blocking_send` — putting a value in from a plain thread

An async Sender has two ways to put a value in:

- `sender.send(value).await` — for async code;
- `sender.blocking_send(value)` — for plain threads. If the pipe is full, it blocks the thread until
  there is room.

The engine thread is a plain thread (`thread::spawn`, 14a), not async. It cannot write `.await`. So
it uses `blocking_send`. Blocking is fine there: waiting is all that thread does anyway.

Like std's `send`, it gives back a `Result`: `Err` if nobody holds the Receiver any more.

### 4.3 `receiver.recv().await` — waiting without blocking, and `Option`

`recv()` on the async kind does not wait by itself. It gives back a future, and `.await` pauses
`ask` until a value comes out. (A future does nothing until it is awaited: Stage 12.)

It needs `&mut`: taking a value out changes the Receiver. That is why `answer` is declared `mut`.

It gives back an `Option`:

- `Some(value)` — a value came out of the pipe;
- `None` — the pipe is empty, and every Sender is gone, so nothing can ever come.

For `ask`, these are the real values (printed with `dbg!` from the tests, as in 14c):

| what happened | `answer.recv().await` | `answered` | `reply` |
|---|---|---|---|
| the engine answered | `Some(Ok("I was at home all night."))` | `Ok("I was at home all night.")` | `"I was at home all night."` |
| the engine failed | `Some(Err(Inference { message: "out of memory" }))` | `Err(Inference { … })` — `?` stops `ask` here | never reached |
| the engine thread crashed | `None` — `.ok_or` makes it `Inference { message: "the engine stopped before it answered" }`, `?` stops `ask` here | never reached | never reached |

In 14b there were three boxes. Now there are two. The outer box — "did the background thread
finish?" — is gone, because there is no background thread.

**`blocking_recv()`.** The async Receiver also has `blocking_recv()`, for plain threads. The tests use
it, because a test function is not async:

```rust
assert_eq!(
    answer.blocking_recv(),
    Some(Ok("I was at home all night.".to_string()))
);
```

Inside `ask`, `blocking_recv()` compiles, but the tests stop with this:

```
Cannot block the current thread from within a runtime. This happens because a function attempted
to block the current thread while the thread is being used to drive asynchronous tasks.
```

That is 14c's rule — an async thread must never block — checked by tokio while the program runs.

### 4.4 A type written with its full path

`engine_thread.rs` now uses two different types called `Sender`:

- std's, for pipe 1. The top of the file already brings it in:
  ```rust
  use std::sync::mpsc::{channel, Receiver, Sender};
  ```
  So plain `Sender` in this file means std's.
- tauri's, for pipe 2.

To name the second one, write its full path: library, module, name, joined with `::`.

```rust
pub answer_to: tauri::async_runtime::Sender<AppResult<String>>,
```

A `use` line is a shortcut for a path like this. You have seen the same path in the tests:
`use tauri::async_runtime::block_on;`.

## 5. Syntax you already know — from your code

### 5.1 `.ok_or(error)` — turning `None` into an error

`case.rs :: require_suspect`:

```rust
    pub fn require_suspect(&self, id: SuspectId) -> AppResult<&Suspect> {
        self.suspect(id).ok_or(AppError::SuspectNotFound { id })
    }
```

`self.suspect(id)` gives `Some(suspect)` or `None`. `.ok_or(…)` turns `Some(x)` into `Ok(x)`, and
`None` into `Err(the error you gave)`. In `ask`, `?` then passes that `Err` up.

### 5.2 `Option`: `Some` and `None`

`transcript.rs :: Phase::suspect`:

```rust
    pub fn suspect(&self) -> Option<SuspectId> {
        match self {
            Phase::Interrogating { suspect, .. } => Some(*suspect),
            _ => None,
        }
    }
```

`Some(value)` when there is one, `None` when there is not. `recv()` gives the same kind of answer.

### 5.3 `let mut`

`state.rs :: begin`:

```rust
        let mut phase = self.phase.lock().map_err(|e| AppError::Poisoned {
            message: e.to_string(),
        })?;
        phase.begin(suspect)
```

`phase` is `mut` because `phase.begin(…)` changes it. `answer` is `mut` because `answer.recv()`
changes it: it takes a value out.

### 5.4 `let _ =` — from 14a

`engine_thread.rs :: answer_all`, before this stage:

```rust
        let _ = question.answer_to.send(reply);
```

If the asker has gone, there is nobody to tell, so the `Result` is thrown away. Same with
`blocking_send`.

### 5.5 What goes away — from 14b

```rust
        let waiting = spawn_blocking(move || answer.recv());
        let waited = waiting.await.map_err(|e| AppError::BackgroundFailed {
            message: e.to_string(),
        })?;
```

These lines borrowed a background thread to do the blocking `recv()`. The async kind of pipe 2
can be awaited directly, so they are not needed. `spawn_blocking` and `AppError::BackgroundFailed`
are still used in `ipc.rs :: case_intro_in_background` (Stage 13), for reading a case file.

## 6. What you built

For one question in the game:

1. `ask` keeps the player's line in the room.
2. `ask` makes pipe 2 with `channel(1)` — the async kind.
3. `ask` builds `Question { text, answer_to }` and sends it into pipe 1.
4. The engine thread wakes up, calls `engine.reply`, and puts the reply into pipe 2 with
   `blocking_send`.
5. `ask` has been paused at `answer.recv().await` — no thread is blocked. The reply comes out:
   `Some(Ok(text))`.
6. `.ok_or(…)?` and `answered?` open it. `ask` keeps the reply in the room, and gives it to React.

If the engine thread dies, `recv()` gives `None`, and `ask` returns `AppError::Inference` with the
message "the engine stopped before it answered".

New in this stage:

- **An async channel** — `tauri::async_runtime::channel(size)`. Its receiving end is awaited:
  `recv().await`, which gives `Some(value)` or `None`.
- **`blocking_send`** — how a plain thread puts a value into an async channel.
- **A type's full path** — `tauri::async_runtime::Sender<…>`, when two types share a name.

Next, Stage 16: the suspect's reply comes in pieces, and React shows them as they arrive.
