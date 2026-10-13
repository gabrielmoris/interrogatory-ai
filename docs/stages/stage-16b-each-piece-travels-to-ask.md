# Stage 16b — each piece travels to `ask`

Test:  src-tauri/tests/pieces_travel.rs — 4 tests
Run:   cd src-tauri && cargo test --test pieces_travel

## 1. What part of the app

The path of one question, as in 16a:

```
React → ask_suspect (ipc.rs) → AppState::ask (state.rs) → pipe 1 → engine thread → engine.reply (llm.rs)
                                       ▲                                   │
                                       └──────────── pipe 2 ───────────────┘
```

This stage changes **pipe 2**: what travels in it, and how `ask` takes it out.

**The problem.** Since 16a the engine says its reply in pieces. But the engine thread still waits
for the last piece, puts them all together in `heard`, and only then sends the whole reply. So `ask`
gets nothing until the suspect has finished talking. In 16c, `ask` must pass each piece to React the
moment it comes. So each piece must reach `ask` by itself.

**The fix.**

- The engine thread sends each piece into pipe 2 **the moment the engine says it**.
- When the engine is done, the engine thread sends **one last message** that says how it ended:
  done, or failed with an error.
- `ask` takes messages out of pipe 2, one at a time, **until the last message**. It puts the pieces
  together in a `String`, `reply`. That is the new thing: a loop that runs until you tell it to stop
  (§4.1).

`ask` still gives back the whole reply, so the player sees no change yet. 16c changes that.

**What travels in pipe 2.** Three kinds of message. A new enum, `Said`, has one variant for each:

```rust
pub enum Said {
    Piece(String),     // one piece of the reply, e.g. Said::Piece("was ".to_string())
    Done,              // the reply is complete — always the last message
    Failed(AppError),  // the engine failed — always the last message
}
```

`Piece` and `Failed` each carry one value inside `( )`, the way `Ok(…)` and `Err(…)` do.

### The map

```
  THE ENGINE THREAD — answer_all (engine_thread.rs)               ask (state.rs)

  engine.reply(&question.text, say)                              reply = ""
    the engine calls say("I ")                                   loop {
      → the closure sends  Said::Piece("I ")   ──── pipe 2 ────▶    said = Piece("I ")   → reply = "I "
    the engine calls say("was ")
      → the closure sends  Said::Piece("was ") ────────────────▶    said = Piece("was ") → reply = "I was "
    …                                                                …
  finished = Ok(())
  last = Said::Done
  question.answer_to.blocking_send(last)       ────────────────▶    said = Done          → break
                                                                 }
                                                                 record the reply in the room
                                                                 Ok(reply)  → back to React
```

Every name in the map:

| name | what it is | where it is made, who holds it |
|---|---|---|
| `Said` | the enum of messages that travel in pipe 2 | `engine_thread.rs`, next to `Question` |
| `Said::Piece(String)` | one piece of the reply | made in the closure, on the engine thread |
| `Said::Done` | "the reply is complete" — the last message | made in `answer_all`, after `reply` |
| `Said::Failed(AppError)` | "the engine failed, with this error" — the last message | made in `answer_all`, after `reply` |
| the closure | the function `answer_all` hands to `engine.reply` as `say`. Each time the engine calls it with a `piece`, it sends `Said::Piece` into pipe 2 | written in `answer_all`; called by the engine |
| `piece` | one piece, a `&str` such as `"was "` (as in 16a) | the closure's input |
| `finished` | what `engine.reply(…)` gives back: `Ok(())` or the engine's error. (In 16a you called it `said`. That name now goes to `ask`.) | `answer_all` |
| `last` | `Said::Done` or `Said::Failed(error)`, made from `finished` | `answer_all` → pipe 2 |
| `question.answer_to` | pipe 2's sending end. Its type is now `Sender<Said>` | inside each `Question`; used by the closure and by `answer_all` |
| `answer` | pipe 2's receiving end (no change from Stage 15) | `ask` |
| `said` | one message taken out of pipe 2 | `ask`, a new one each time round the loop |
| `reply` | a `String` that grows by one piece each time round the loop | `ask`. Empty before the loop |

`question`, `inbox`, `engine`, `questions` and pipe 1 stay as in Stage 15.

**Pipe 2 holds one message.** You made it with `channel(1)` in Stage 15: "When it is full, putting
another value in waits until one is taken out." So after the closure sends `Piece("I ")`, the engine
thread waits at the next send until `ask` has taken `Piece("I ")` out. The engine thread and `ask`
take turns, one message at a time.

### One question, start to end

With `MockEngine::new("I was at home all night.")`. Values printed with `dbg!` from the test
`ask_puts_the_pieces_back_together`, as in 14c.

1. `ask` keeps the question in the room and sends a `Question` into pipe 1. (No change.)
2. `ask` makes `reply`, empty.
3. `answer_all` takes the question out of pipe 1 and calls `engine.reply`, with the closure as `say`.
4. The engine calls `say` six times. Each call sends one `Said::Piece`. `ask` takes each one out:

   | the engine calls | `ask` takes out: `said` | `reply` after that round |
   |---|---|---|
   | `say("I ")` | `Piece("I ")` | `"I "` |
   | `say("was ")` | `Piece("was ")` | `"I was "` |
   | `say("at ")` | `Piece("at ")` | `"I was at "` |
   | `say("home ")` | `Piece("home ")` | `"I was at home "` |
   | `say("all ")` | `Piece("all ")` | `"I was at home all "` |
   | `say("night.")` | `Piece("night.")` | `"I was at home all night."` |

5. `engine.reply` gives back `Ok(())`. So `finished` is `Ok(())`, `last` is `Said::Done`, and
   `answer_all` sends it.
6. `ask` takes out `said = Done`. `break`: the loop stops.
7. `ask` keeps `"I was at home all night."` in the room and gives it back. (As in Stage 15.)

**If the engine fails halfway** (`FailsHalfwayEngine` in the test): the engine says `"I was "`, then
gives back an error.

| `ask` takes out: `said` | what `ask` does |
|---|---|
| `Piece("I was ")` | `reply` becomes `"I was "` |
| `Failed(Inference { message: "out of memory" })` | stops at once and gives back that error. `"I was "` is thrown away: half a sentence is not an answer |

**If the engine thread crashes:** nothing more comes, and the pipe closes. `recv` gives `None`, and
`.ok_or(…)?` stops `ask` with "the engine stopped before it answered" — the same line as Stage 15.

## 2. Files and templates

| File | What you do |
|---|---|
| `src/engine_thread.rs` | paste `Said` and the new `Question` (place 1); then change `answer_all`'s loop in three small steps (§3.1) |
| `src/state.rs` | paste the import and the `ask` template (place 2); then type the loop, and build the line before it and the two after it (§3.2) |
| `tests/pieces_travel.rs` | nothing — new, mine |
| `tests/engine_thread.rs` | nothing — I changed it to read `Said` messages |
| `tests/pieces.rs` | nothing — I took out the three tests about the engine thread putting pieces together; their 16b form is in `pieces_travel.rs`. 2 tests are left |

Run now, before you change anything: it does not compile — `1 previous error`:

```
error[E0432]: unresolved import `interrogatory_ai_lib::engine_thread::Said`
  --> tests/pieces_travel.rs:13:60
```

The test uses `Said`. It does not exist yet.

**Place 1 — `engine_thread.rs`.**

At the top, change the first line from `use crate::error::AppResult;` to:

```rust
use crate::error::AppError;
```

Then replace the whole `Question` struct, with its doc comment, with this:

```rust
/// One message on pipe 2: a piece of the reply, or how the reply ended.
#[derive(Debug, PartialEq)]
pub enum Said {
    /// One piece of the reply, sent the moment the engine says it.
    Piece(String),
    /// The reply is complete. Always the last message.
    Done,
    /// The engine failed. Always the last message.
    Failed(AppError),
}

/// One question for the engine, and where to send what the suspect says.
pub struct Question {
    pub text: String,
    pub answer_to: tauri::async_runtime::Sender<Said>,
}
```

What each part is for:

- `Said` — the three messages from §1.
- `#[derive(Debug, PartialEq)]` — the tests compare messages with `assert_eq!`, which needs both.
  `AppError` already has them, so `Failed(AppError)` is fine.
- `answer_to` — pipe 2 now carries `Said`, not `AppResult<String>`.
- The import: `Said` uses `AppError`. Nothing in this file uses `AppResult` any more.

Leave `start` and `answer_all` as they are for now.

**Place 2 — `state.rs`.**

Change the first line to:

```rust
use crate::engine_thread::{start, Question, Said};
```

Then make `ask` look like this. The doc comment's second line changes, and the last four statements
(from `let answered` to `Ok(reply)`) are replaced by `todo!()`. Everything else stays.

```rust
    /// The suspect's reply to what the detective just asked.
    /// The engine says it piece by piece on its own thread. `ask` puts the pieces together.
    pub async fn ask(&self, text: &str) -> AppResult<String> {
        self.record(Speaker::Detective, text)?;

        let (answer_to, mut answer) = channel(1);

        let question = Question {
            text: text.to_string(),
            answer_to,
        };

        self.questions
            .send(question)
            .map_err(|e| AppError::Inference {
                message: e.to_string(),
            })?;

        todo!()
    }
```

The lines you deleted are in §5.2. You will need two of them again.

Run: it does not compile — `1 previous error`, in `answer_all`:

```
error[E0308]: mismatched types
    --> src/engine_thread.rs:39:50
     |
  39 |         let _ = question.answer_to.blocking_send(answer);
     |                                    ------------- ^^^^^^ expected `Said`, found `Result<String, AppError>`
```

Pipe 2 now carries `Said`, and `answer_all` still sends the whole reply. You also get three
warnings from `ask` — `unused import: Said`, `variable does not need to be mutable`, `unused variable:
answer`. They are the `todo!()`. They go away in §3.2.

## 3. What each function does, and how to build it

Two kinds of step:

- **Type it** — the code is shown. You type it. Under it, a table says what each part does.
- **Build it** — easy, and yours. It says what the line must do, and shows code you already have,
  right there. You change it.

After each step: **Run**, and what you should see.

### 3.1 `answer_all` (engine_thread.rs)

- **Now (16a):** it collects the pieces in `heard`, and sends the whole reply at the end.
- **After 16b:** it sends each piece the moment the engine says it, then one last message.

Your loop today:

```rust
    for question in inbox {
        let mut heard = String::new();
        let said = engine.reply(&question.text, &mut |piece| heard.push_str(piece));
        let answer = match said {
            Ok(()) => Ok(heard),
            Err(error) => Err(error),
        };
        let _ = question.answer_to.blocking_send(answer);
    }
```

You change it in three steps — A, B, C — from top to bottom.

#### Step A — Type it: the closure sends each piece

Replace the first two lines inside the loop (`let mut heard …` and `let said …`) with:

```rust
        let finished = engine.reply(&question.text, &mut |piece| {
            let _ = question.answer_to.blocking_send(Said::Piece(piece.to_string()));
        });
```

| part | what it does |
|---|---|
| `let finished =` | keeps what `reply` gives back when it is done: `Ok(())`, or the engine's error. In 16a this was `said`. That name now goes to `ask` |
| `&mut \|piece\| { … }` | the closure, handed to the engine as `say` — as in 16a. This time its body is in `{ … }`, like a function body, because it holds a full line ending in `;` |
| `Said::Piece(piece.to_string())` | makes the message. The engine lends `piece` (a `&str`) only during this one call. The message goes to another thread and lives on after the call, so it needs its own text: `.to_string()` makes a `String` |
| `let _ = question.answer_to.blocking_send(…);` | your 16a send line, with the message in it. `let _ =` throws away the `Result` that `blocking_send` gives back — so the closure gives back nothing, which is what `say` must do |

`heard` is gone: nothing is collected here any more.

In the map: each time the engine calls `say("was ")`, this closure runs once, and
`Said::Piece("was ")` goes into pipe 2.

`cargo fmt` will later spread the long line over three lines. Only the layout changes.

Run: does not compile — `2 previous errors`:

```
error[E0425]: cannot find value `said` in this scope
error[E0425]: cannot find value `heard` in this scope
```

Both are in your old `match`. `said` and `heard` do not exist any more. That is step B.

#### Step B — Build it: the last message

**What it must do.** Turn `finished` into the last message. Call it `last`.

- `Ok(())` → `Said::Done`
- `Err(error)` → `Said::Failed(error)`

**What you have.** Your 16a `match`, right below step A:

```rust
        let answer = match said {
            Ok(()) => Ok(heard),
            Err(error) => Err(error),
        };
```

It is the same shape. Change three things:

1. the name on the left: `answer` → `last`;
2. what you match on: `said` → `finished`;
3. what each arm makes, on the right of `=>`.

How to write the two messages: `Said::Done` has no `( )`, because it carries nothing.
`Said::Failed(error)` carries the error inside `( )` — the same way you wrote `Err(error)`.

Run: `1 previous error` — `E0425 cannot find value answer in this scope`, on the send line. Step C.

#### Step C — Build it: send the last message

The last line of the loop still sends `answer`. Make it send `last`.

Run: `cargo test --test pieces_travel` → `2 passed; 2 failed`.

- Passing: `each_piece_is_sent_on_by_itself_then_done` and `a_failure_is_the_last_message`. They read
  pipe 2 directly, so `answer_all` is done.
- Failing: the two `ask_…` tests, with `panicked at src/state.rs:72:9: not yet implemented`. That is
  `ask`'s `todo!()`. Next.

Also `cargo test --test engine_thread` → `4 passed`.

### 3.2 `ask` (state.rs)

**What it must do.** After sending the question (no change), take messages out of pipe 2 one at a
time, until the last one:

- a piece → add it to the end of `reply`, then take the next message;
- `Done` → stop the loop: the reply is complete;
- `Failed(error)` → stop `ask` at once, with the error.

After the loop, as in Stage 15: keep `reply` in the room, and give it back.

Replace `todo!()` with three parts, in this order: step D, the loop, step E.

#### Step D — Build it: an empty `reply`

**What it must do.** Make a `String` called `reply`: empty, and one you can add to.

**What you have.** In your 16a `answer_all`:

```rust
        let mut heard = String::new();
```

Same line, another name.

#### Type it: the loop

Under step D, type:

```rust
        loop {
            let said = answer.recv().await.ok_or(AppError::Inference {
                message: "the engine stopped before it answered".to_string(),
            })?;
            match said {
                Said::Piece(piece) => reply.push_str(&piece),
                Said::Done => break,
                Said::Failed(error) => return Err(error),
            }
        }
```

| part | what it does |
|---|---|
| `loop { … }` | runs its body again and again, until `break` or `return` (§4.1). One round = one message |
| `let said = answer.recv().await.ok_or(…)?;` | your Stage 15 line, with `answered` renamed `said`. Waits for the next message from pipe 2. If the pipe has closed (the engine thread crashed), stops `ask` with that error |
| `match said { … }` | one arm for each kind of message |
| `Said::Piece(piece) => reply.push_str(&piece),` | a piece: `piece` names the `String` inside, and `push_str` adds it to the end of `reply`. `push_str` takes a `&str`, and `&piece` gives one — as `&reply` does in `self.record(Speaker::Suspect, &reply)` |
| `Said::Done => break,` | the reply is complete: leave the loop. The code after the loop's `}` runs next (step E) |
| `Said::Failed(error) => return Err(error),` | the engine failed: leave `ask` itself, with the error — like `return Err(…)` in `Phase::begin` (§5.3). Step E never runs, so half a reply is never kept |

In the map: one round of the loop is one row of the table in §1, "One question, start to end".

#### Step E — Build it: keep the reply, give it back

**What it must do.** After the loop's closing `}`, put back the two lines that ended `ask` in Stage 15.
You deleted them in §2. They are in §5.2 if you need them.

Run: `cargo test --test pieces_travel` → `4 passed; 0 failed`. The three warnings are gone.

Last: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (176 tests: you had
175, `pieces.rs` lost 3, `pieces_travel.rs` adds 4). Say ready.

### If it does not compile

`answer_all`:

| the compiler says | what it means |
|---|---|
| after §2: `E0308 … expected Said, found Result<String, AppError>`, at `blocking_send(answer)` | Expected. Step A. |
| after step A: `E0425 cannot find value said` and `… heard` | Expected. Step B. |
| after step B: `E0425 cannot find value answer` | Expected. Step C. |
| `E0308 match arms have incompatible types … expected Said, found Result<_, AppError>` | One arm still makes `Err(error)`. It must make `Said::Failed(error)`. |
| `E0308 match arms have incompatible types … expected Result<(), _>, found Said` | The `Ok(())` arm still makes `Ok(…)`. It must make `Said::Done`. |
| `E0308 … expected Said, found enum constructor` | You wrote `Said::Failed` without the error. Write `Said::Failed(error)`. |
| `E0308 … expected Said, found Result<(), AppError>`, at the send line | You sent `finished`. Send `last`. |

`ask`:

| the compiler says | what it means |
|---|---|
| `E0425 cannot find value reply in this scope` (3 times) | Step D is missing, or it is below `loop {`. Put it above. |
| `E0596 cannot borrow reply as mutable` | `let mut reply`, not `let reply`. |
| `E0308 … expected Result<String, AppError>, found ()`, pointing at `break`, with `help: give the break a value` | Step E is missing: after the loop, `ask` gives nothing back. Do not follow the help — add step E. |
| `E0267 break inside async function` | `loop {` is missing. |
| a warning `unreachable statement`, and a test fails with `the engine stopped before it answered` | The `Done` arm has no `break`. |
| a test fails with `left: Ok("I was ")` | The `Failed` arm uses `break`. It must be `return Err(error)` (§4.1). |

## 4. New syntax and methods

### 4.1 `loop` and `break`

**What it is.** `loop { … }` runs the code inside the braces, then again, then again — with no end of
its own. `break` stops it: the program jumps to the first line after the loop's closing `}`.

**Form.**

```rust
loop {
    // code that runs each time round
    // somewhere inside: break;
}
// break jumps here
```

**Example**, with other names:

```rust
let mut tries = 0;
loop {
    tries = tries + 1;
    if tries == 3 {
        break;
    }
}
// here, tries is 3
```

`break` can also be a `match` arm, as in the loop you type: `Said::Done => break,`.

**`break` and `return` are different.**

| | leaves | then runs |
|---|---|---|
| `break` | the loop only | the first line after the loop's `}` |
| `return Err(error)` | the whole function, with that value | nothing more in the function |

In `ask`: `Done` → `break`, because after the loop there is still work to do (keep the reply, give it
back). `Failed` → `return Err(error)`, because there is nothing left to do but give back the error.

**When to use it.** When you do not know beforehand how many times to go round, and the code inside
finds out when to stop. `for` is for when there is a list to walk (`for question in inbox` in
`answer_all` stops by itself when pipe 1 closes).

**Why here.** `ask` cannot know how many pieces a reply has. It knows it is done only when the last
message comes. Pipe 2 does not close at `Done` either: `answer_all` lets go of it a moment later.
So `ask` must decide by itself when to stop.

**What the compiler checks.**

- `break` outside a loop does not compile: `E0267 break inside async function`.
- Code after a loop that has no `break` can never run. The compiler warns: `unreachable statement`.

## 5. Syntax you already know — from your code

Everything §3 used from your code, in one place.

### 5.1 Your 16a `answer_all` — where step A starts

`engine_thread.rs :: answer_all`:

```rust
fn answer_all(engine: Box<dyn InferenceEngine>, inbox: Receiver<Question>) {
    for question in inbox {
        let mut heard = String::new();
        let said = engine.reply(&question.text, &mut |piece| heard.push_str(piece));
        let answer = match said {
            Ok(()) => Ok(heard),
            Err(error) => Err(error),
        };
        let _ = question.answer_to.blocking_send(answer);
    }
}
```

### 5.2 The end of `ask` — Stage 15

`state.rs :: ask`, the lines you deleted in §2:

```rust
        let answered = answer.recv().await.ok_or(AppError::Inference {
            message: "the engine stopped before it answered".to_string(),
        })?;

        let reply = answered?;

        self.record(Speaker::Suspect, &reply)?;
        Ok(reply)
```

The first one moves into the loop, as `let said = …`. `let reply = answered?;` is gone for good: a
failure now comes as `Said::Failed`. The last two are step E.

### 5.3 Leaving a function early with an error: `return Err(…)`

`transcript.rs :: Phase::begin`:

```rust
        if !matches!(self, Phase::Briefing) {
            return Err(self.refusal("begin an interrogation"));
        }
```

`return` leaves `begin` at once, with that error. The lines below it do not run.

## 6. What you built

For one question in the game:

1. `ask` sends the question into pipe 1, and makes an empty `reply`.
2. `answer_all` calls `engine.reply`, with a closure as `say`.
3. Each time the engine says a piece, the closure sends `Said::Piece` into pipe 2. `ask` takes it
   out and adds it to `reply`. Pipe 2 holds one message, so they take turns.
4. When the engine is done, `answer_all` sends `Said::Done`. `ask` takes it out, and `break` ends
   the loop.
5. `ask` keeps the whole reply in the room and gives it back, as before.

If the engine fails halfway, the last message is `Said::Failed`, and `ask` returns its error. The
half reply in `reply` is thrown away.

New in this stage:

- **`loop` and `break`** — go round until the code inside says stop. `break` leaves the loop;
  `return` leaves the whole function.

Yours: the last message in `answer_all` (steps B and C), and the start and end of `ask` (D and E).

Next, Stage 16c: `ask` passes each piece to React the moment it takes it out of pipe 2, through a
Tauri channel. The player sees the suspect's words appear one by one.
