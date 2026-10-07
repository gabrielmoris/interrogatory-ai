# Stage 14c — see what `ask` does

Test:  src-tauri/tests/ask_engine_thread.rs — the same 3 tests as 14b. Nothing new to pass.
Run:   cd src-tauri && cargo check                                                  (part A)
       cd src-tauri && cargo test --test ask_engine_thread -- --nocapture --test-threads=1   (part C)

This stage adds nothing to the game. After 14b, three things were still unclear: who holds what,
why the waiting needs `spawn_blocking` and `.await`, and the three `Result`s. Here the compiler and
the running tests show you each one, in your own `ask`. Every line you add, you delete again.

## 1. What part of the app

Only `ask`, in `src/state.rs`. This is your 14b code, after step 0 (§3), with the block numbers
from the 14b doc:

```rust
    pub async fn ask(&self, text: &str) -> AppResult<String> {
        self.record(Speaker::Detective, text)?; // block 1

        let (answer_to, answer) = channel(); // block 2

        let question = Question { // block 3
            text: text.to_string(),
            answer_to,
        };

        self.questions // block 4
            .send(question)
            .map_err(|e| AppError::Inference {
                message: e.to_string(),
            })?;

        let waiting = spawn_blocking(move || answer.recv()); // block 5
        let waited = waiting.await.map_err(|e| AppError::BackgroundFailed {
            message: e.to_string(),
        })?;

        let answered = waited.map_err(|e| AppError::Inference { // block 6
            message: e.to_string(),
        })?;
        let reply = answered?;

        self.record(Speaker::Suspect, &reply)?; // block 7
        Ok(reply)
    }
```

### A. Who holds what, moment by moment

In Rust, every value has exactly one owner at a time. When you hand a value on — put it in a
struct, `send` it, or `move` it into a closure — it leaves. Its old name cannot be used any more.
That is a **move** (§5.1).

`ask` makes three values. Follow them through the blocks:

| after block | `ask` still holds | where the others went |
|---|---|---|
| 2 | `answer_to`, `answer` | — |
| 3 | `question`, `answer` | `answer_to` is now *inside* `question` |
| 4 | `answer` | `question` went into pipe 1. The engine thread's `for` loop takes it out — with `answer_to` inside. |
| 5 | nothing | `answer` went to the background thread, which waits on it with `recv()` |

By the time `ask` waits, it holds none of the three. Each one sits where it is needed:

- the **engine thread** holds `answer_to`: it is the one that must reply;
- the **background thread** holds `answer`: it is the one that waits for the reply.

**Why `answer_to` travels inside the `Question`.** The engine thread has one inbox for every
question. When a question comes out of it, the engine must know where *this* reply goes. The
`Question` carries that with it: `answer_to` is the way back to the one `ask` that sent it. Your 14a
test `each_answer_goes_back_to_whoever_asked` sends two questions, each with its own `answer_to`. Each
reply comes back to its own `answer`, never to the other one.

Part A of §3 makes the compiler show you each of these moves.

### B. Waiting: why `spawn_blocking` and `.await`

There are two kinds of waiting:

- **`answer.recv()` blocks.** The thread that runs it stops completely until the reply arrives. It
  can do nothing else meanwhile.
- **`.await` pauses.** Only this `async` function stops. The thread that was running it is free to
  run other async work, and comes back to `ask` when the result is ready.

In the game, `ask` runs on one of the few threads that Tauri uses for all `async` commands
(`ask_suspect`, `case_intro`). If `ask` called `answer.recv()` itself, that thread would be stuck for
as long as the engine thinks — seconds, with the real model.

So block 5 splits the work across three threads:

```
           ask (an async thread)           background thread          engine thread
block 4    send(question)  ──────────────────────────────────────────▶ wakes up with question
block 5    spawn_blocking(…) ────────────▶ answer.recv()               engine.reply(…)
           waiting.await                   …waiting…                   answer_to.send(reply)
           (ask pauses; its thread                                          │
            runs other work meanwhile)     reply arrives ◀──────────────────┘
           ask resumes  ◀───────────────── job done
block 6    open the results
```

This is the same as in Node.js:

```ts
const data = fs.readFileSync(path);            // blocks: nothing else runs until it is done
const data = await fs.promises.readFile(path); // the read runs on Node's thread pool; await pauses only this function
```

`answer.recv()` is the `readFileSync`. `spawn_blocking(move || answer.recv())` followed by `.await`
is the `await fs.promises.readFile`. Stage 13 did the same for reading a case file from disk.

The tests cannot show this waiting, so part B has nothing to run.

### C. The three `Result`s: the real values

Forget the types. Look at the values, for the three things that can happen. These are printed from
the real tests in part C of §3.

| what happened | `waited` | `answered` | `reply` |
|---|---|---|---|
| the engine answered | `Ok(Ok("I was at home all night."))` | `Ok("I was at home all night.")` | `"I was at home all night."` |
| the engine failed | `Ok(Err(Inference { message: "out of memory" }))` | `Err(Inference { … })` — `?` stops `ask` here | never reached |
| the engine thread crashed | `Err(RecvError)` — becomes `Inference`, and `?` stops `ask` here | never reached | never reached |

Each line takes one `Ok( … )` off — or, if it finds an `Err`, stops `ask` and gives that error back.

Why only some lines have `.map_err`: `?` can only hand back an `AppError`, because `ask` returns
`AppResult<String>`. So look at the error each line can find:

- `waited` can be `Err(RecvError)`. A `RecvError` is not an `AppError`, so `.map_err` turns it into one.
- `answered` can be `Err(Inference { … })`. That is already an `AppError` — the engine made it. No
  `.map_err` needed.
- Block 5 opened one more box, before `waited` existed: did the background thread finish? Its error
  is a `tauri::Error`, not an `AppError`, so `.map_err` turns it into `BackgroundFailed`.

## 2. Files and templates

| File | What you do |
|---|---|
| `src/state.rs` | step 0: rename one name. Then add one line at a time, look at what the compiler or the tests say, and delete it again. |

No template. At the end, `ask` is exactly as after step 0.

## 3. What to do

### Step 0 — one name for one thing

Your input is called `question`, and block 3 makes a second `question`. From block 3 on, `question`
means the `Question`, and the player's text has no name of its own. Rename the input to `text`.
Change these three lines:

```rust
    pub async fn ask(&self, text: &str) -> AppResult<String> {
        self.record(Speaker::Detective, text)?;
```

and, inside block 3:

```rust
            text: text.to_string(),
```

Run: `cargo test --test ask_engine_thread` → `3 passed; 0 failed`.

### Part A — make the compiler show you each move

Each time: add the one line, run `cargo check`, read the error, delete the line. Your line numbers
will be different from the ones below.

**A1. `answer_to` goes into the `Question`.** Add this line right after block 3, after its `};`:

```rust
        let again = answer_to;
```

`cargo check` says:

```
error[E0382]: use of moved value: `answer_to`
...
63 |             answer_to,
   |             --------- value moved here
64 |         };
65 |         let again = answer_to;
   |                     ^^^^^^^^^ value used here after move
```

"Value moved here" points into block 3: `answer_to` now lives inside `question`. Ignore the `help:`
about `.clone()` that comes after it — a copy is not what we want. Delete the line.

**A2. `question` goes into pipe 1.** Add this line right after block 4, after its `})?;`:

```rust
        let again = question;
```

`cargo check` says:

```
error[E0382]: use of moved value: `question`
...
67 |             .send(question)
   |                   -------- value moved here
...
71 |         let again = question;
   |                     ^^^^^^^^ value used here after move
```

`send` took the whole `Question`. `ask` does not have it any more; the engine thread does. Delete the line.

**A3. `answer` goes to the background thread.** Add this line right after the first line of block 5:

```rust
        let again = answer;
```

`cargo check` says:

```
error[E0382]: use of moved value: `answer`
...
72 |         let waiting = spawn_blocking(move || answer.recv());
   |                                      ------- ------ variable moved due to use in closure
   |                                      |
   |                                      value moved into closure here
73 |         let again = answer;
   |                     ^^^^^^ value used here after move
```

`move` gave `answer` to the closure, and `spawn_blocking` took the closure to the background thread.
Delete the line.

These three errors are the table in §1 A, written by the compiler.

### Part C — print the three results while the tests run

1. Add three `dbg!` lines (§4.1), in blocks 5 and 6. They are the lines with `dbg!`:

```rust
        let waited = waiting.await.map_err(|e| AppError::BackgroundFailed {
            message: e.to_string(),
        })?;
        dbg!(&waited);

        let answered = waited.map_err(|e| AppError::Inference {
            message: e.to_string(),
        })?;
        dbg!(&answered);
        let reply = answered?;
        dbg!(&reply);
```

2. Run: `cargo test --test ask_engine_thread -- --nocapture --test-threads=1` (§4.2). You will see
   this, test by test. The numbers (in brackets, after `ThreadId`, and the line numbers) will be
   different.

**`a_crashed_engine_is_an_error_not_a_crash`:**

```
thread '<unnamed>' (2733) panicked at tests/ask_engine_thread.rs:45:9:
the model crashed
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[src/state.rs:76:9] &waited = Err(
    RecvError,
)
ok
```

- The first three lines are the engine thread dying. `'<unnamed>'` is the engine thread: 14a never
  gave it a name.
- `waited` is `Err(RecvError)`: no reply can ever come. The next line turns it into `Inference`, and
  `?` stops `ask`. That is why `answered` and `reply` are never printed.

**`an_engine_failure_comes_back_from_ask`:**

```
[src/state.rs:76:9] &waited = Ok(
    Err(
        Inference {
            message: "out of memory",
        },
    ),
)
[src/state.rs:81:9] &answered = Err(
    Inference {
        message: "out of memory",
    },
)
ok
```

- A reply did come (the outer `Ok`), but the reply is the engine's error.
- One `Ok` off: `answered` is that error. `answered?` stops `ask` with it. `reply` is never printed.

**`every_answer_comes_from_the_one_engine_thread`** (it asks twice, so this prints twice):

```
[src/state.rs:76:9] &waited = Ok(
    Ok(
        "ThreadId(10)",
    ),
)
[src/state.rs:81:9] &answered = Ok(
    "ThreadId(10)",
)
[src/state.rs:83:9] &reply = "ThreadId(10)"
```

- This test's engine answers with the name of the thread it runs on, so the reply is `"ThreadId(10)"`.
- Each line takes one `Ok` off. `reply` is the plain text.

3. Delete the three `dbg!` lines.

Last: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (170 tests). Say ready.

## 4. New syntax and methods

### 4.1 `dbg!(&value)` — print a value while the code runs

`dbg!` prints three things — the file and line, the code you gave it, and its value — and then the
program carries on. Like `console.log`, with the location added.

Form:

```rust
dbg!(&waited);
```

The `&` lends the value to `dbg!`, so the next line can still use `waited`.

The value is printed spread out, one layer per line, so you can count the `Ok(`s.

It works for any type that can be printed for debugging, which means its type has `Debug` in its
`#[derive(…)]` (§5.2). `Result`, `String` and your `AppError` all have it.

**When to use it:** to look inside a value while you learn or debug. Delete it afterwards; it is
not for finished code.

### 4.2 `cargo test … -- --nocapture --test-threads=1`

`cargo test` normally hides everything a passing test prints. Everything after the lone `--` is
for the test runner, not for cargo:

- `--nocapture` shows the printing;
- `--test-threads=1` runs one test at a time, so the outputs of different tests do not mix.

## 5. Syntax you already know — from your code

### 5.1 A move, and E0382

`case_file.rs :: try_from` — your first loop has a `&`:

```rust
for raw_suspect in &raw.suspects {
```

With the `&`, the loop only borrows the `Vec`. Without it, the loop would take the whole `Vec`, and
your last loop over `&raw.suspects`, further down in `try_from`, would fail:

```
error[E0382]: borrow of moved value: `raw.suspects`
37 |         for raw_suspect in raw.suspects {
   |                            ------------ `raw.suspects` moved due to this implicit call to `.into_iter()`
66 |         for raw_suspect in &raw.suspects {
   |                            ^^^^^^^^^^^^^ value borrowed here after move
```

Same rule as part A: after a move, the old name is gone.

`engine_thread.rs :: start` — here a move is what you want:

```rust
thread::spawn(move || answer_all(engine, inbox));
```

`move` hands `engine` and `inbox` to the new thread. After this line, `start` cannot use them. Part
A3 is the same: `move || answer.recv()` hands `answer` to the background thread.

### 5.2 `#[derive(Debug)]`

`error.rs`, the line above `pub enum AppError`:

```rust
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, serde::Serialize)]
```

`Debug` is what lets `dbg!` (and `assert_eq!`, when a test fails) print an `AppError`.

## 6. What you now know

- **Who holds what.** Each value has one owner. `ask` makes three values and hands each one on:
  `answer_to` into the `Question`, the `Question` into pipe 1 (so `answer_to` ends up with the
  engine thread), and `answer` into the background thread. The compiler showed you each move.
- **Waiting.** `recv()` blocks a whole thread. `.await` pauses only one function. `spawn_blocking`
  gives the blocking wait a thread of its own, so `ask` can `.await` it and the async threads stay
  free. It is the difference between `readFileSync` and `await fs.promises.readFile`.
- **The three `Result`s.** Each line takes one `Ok` off, or stops `ask` with the error it finds.
  `.map_err` goes only where that error is not an `AppError` yet. You saw the real values printed.
- **New tools:** `dbg!(&value)`, and `cargo test … -- --nocapture --test-threads=1` to see it.

Next, Stage 15: sharing one value between threads.
