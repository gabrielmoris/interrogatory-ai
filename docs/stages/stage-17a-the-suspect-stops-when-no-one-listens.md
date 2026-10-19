# Stage 17a — the suspect stops when no one listens

Test:  src-tauri/tests/no_one_listens.rs — 4 tests
Run:   cd src-tauri && cargo test --test no_one_listens

## 1. What part of the app

Stage 17 is about stopping the suspect mid-sentence. It comes in three parts, one new thing each:

| | what happens | new |
|---|---|---|
| **17a** (this one) | when no one listens any more, the engine stops talking | `say` gives back an answer: `FnMut(&str) -> bool` |
| 17b | the player presses **Stop**, and `ask` stops | a stop sign that two places hold: `CancellationToken` |
| 17c | **Stop** works even while the suspect is silent | `select!`: wait for two things, take whichever comes first |

This stage works on the far end of the path, between the engine and the engine thread:

```
React → ask_suspect → AppState::ask → pipe 1 → engine thread → engine.reply
                            ▲                     │      ▲            │
                            └────── pipe 2 ───────┘      └─── say ────┘
                                                    new: say answers back
```

**The problem.** Since 16c, `ask` can stop before the reply is over: when a piece cannot reach React,
`ask` gives back `WindowUnreachable`. In 17b the player's Stop button will stop it too. But the engine
does not know. It keeps calling `say`, piece after piece, until its reply is over. Each piece goes
into pipe 2, where no one reads it any more.

With `MockEngine` that costs nothing. With a real model (Stage 19), every piece is real work on your
Mac's GPU. And the engine thread answers one question at a time: the player's next question waits
until the engine has finished talking to no one.

**Why the engine has to stop itself.** Rust has no way to stop a thread from outside. Only the
engine can end the engine's loop. So something has to tell it. The engine talks to the rest of the
app through one thing only: `say`. So `say` carries the news.

**The fix.**

- `say` now gives back an answer: `true` while someone is listening, `false` once no one is.
- On the engine thread, `say` sends the piece into pipe 2, as before. If the piece got in, `say` gives
  back `true`. If the send failed, it gives back `false`.
- The engine looks at the answer after each piece. On `false` it stops: it says nothing more and gives
  back `Ok(())`. Stopping early is not a failure.

That is the new thing: a function input that answers back, `&mut dyn FnMut(&str) -> bool` (§4.1).

**How the engine thread finds out that `ask` is gone.** When `ask` ends, everything it owns is
dropped — thrown away. That includes `answer`, its end of pipe 2. A pipe with no one at the other end
takes nothing: the next `blocking_send` gives back `Err`. It is the same failure your pipe-1 send in
`ask` already handles, for when the engine thread is gone (§5.1).

Nothing changes in `ask`, in `state.rs`, or in React.

### The map

```
  RUST — ask (state.rs)            THE ENGINE THREAD — answer_all           THE ENGINE — reply (llm.rs)
                                   (engine_thread.rs)

                                                                            say("and ")
                                   the closure:                    ◀─────────
                                     blocking_send(Piece("and "))
  answer.recv()  ◀──── pipe 2 ────     → Ok(())
                                     .is_ok() → true               ─────────▶  true: go on
  on_piece.send(…) fails
  ask gives back WindowUnreachable
  ask ends: `answer` is dropped
                                                                            say("and ")
                                     blocking_send(Piece("and "))  ◀─────────
                 no one here  ✗──      → Err
                                     .is_ok() → false              ─────────▶  false: stop
                                                                            reply gives back Ok(())
                                   Said::Done → fails, thrown away
                                   takes the next question from pipe 1
```

Every name in the map:

| name | what it is | where it is made, who holds it |
|---|---|---|
| `say` | the function the engine calls once per piece (16a). **Now it gives back a `bool`** | made in `answer_all` as a closure, handed to `engine.reply`; the engine calls it |
| `true` / `false` | what `say` gives back: `true` = someone is listening, go on; `false` = no one is, stop | the closure in `answer_all` |
| `piece` | one piece of the reply, a `&str` (16a) | what the engine passes to `say` |
| `question.answer_to` | the engine thread's end of pipe 2 (Stage 15) | inside each `Question` |
| `blocking_send(…)` | puts a message into pipe 2. Gives back `Ok(())` if it got in, `Err` if no one is at the other end | called in the closure |
| `.is_ok()` | turns that `Ok` or `Err` into `true` or `false` (§4.2) | the closure's last line, after step A |
| `answer` | `ask`'s end of pipe 2 (Stage 15). Dropped when `ask` ends | `ask` |
| `word` | one piece, in `MockEngine` | `MockEngine::reply`'s `for` loop |
| `break` | stops `MockEngine`'s loop when `say` gives back `false` | `MockEngine::reply`, after step B |

`Said`, `Said::Done`, `finished`, pipe 1 and `on_piece` stay as in 16c.

### One question, start to end

This is the test `when_ask_stops_early_the_engine_stops_and_the_next_question_is_answered`. Its
engine, `LongWindedEngine`, says `"and "` up to 1000 times, and stops early if `say` gives back
`false`. React's window has closed (`closed_window()`), so the first piece cannot reach React.

1. `ask` keeps the question in the room and sends it into pipe 1. (No change.)
2. The engine calls `say("and ")`. The closure sends `Said::Piece("and ")` into pipe 2. It gets in:
   `blocking_send` gives back `Ok(())`, `.is_ok()` makes that `true`. The engine goes on.
3. `ask` takes the piece out of pipe 2 and sends it into `on_piece`. That fails, so `ask` gives back
   `WindowUnreachable` and ends. `answer` is dropped. (No change — that is 16c.)
4. The engine calls `say("and ")` again. `blocking_send` fails: no one is at the other end.
   `.is_ok()` makes that `false`. The engine stops and gives back `Ok(())`.
5. `answer_all` sends `Said::Done`. That fails too, and `let _ =` throws the failure away. (No
   change.) It takes the next question out of pipe 1.
6. The test asks a second question. The engine thread is free, so the whole answer comes back: 1000
   pieces.

Measured over 300 runs: the engine said 2 or 3 pieces, never more. Why sometimes 3: pipe 2 holds one
piece. If `ask` has not let go yet, the second piece still fits, and only the third finds no one.

With a closure that gives back `true` whatever happens — what 16c's did, in effect — the same test
measures 1000: the engine said every piece to no one.

## 2. Files and templates

| File | What you do |
|---|---|
| `src/llm.rs` | paste the new trait (place 1) and `MockEngine::reply`'s new first two lines (place 2); then step B |
| `src/engine_thread.rs` | paste one doc-comment line (place 3); then step A |
| `tests/no_one_listens.rs` | nothing — new, mine |
| `tests/engine.rs`, `pieces.rs`, `app_engine.rs` | nothing — I changed them: their test engines take the new `say`, and the closures they hand to `reply` give back `true` |
| `tests/engine_thread.rs`, `ask_engine_thread.rs`, `pieces_travel.rs`, `pieces_to_react.rs` | nothing — I changed their test engines to the new `say` |

Run now, before you change anything: it does not compile — `3 previous errors`, all in
`tests/no_one_listens.rs`:

```
error[E0053]: method `reply` has an incompatible type for trait
error[E0308]: mismatched types     (2 times)
```

The test's engine already takes the new `say`, and its closures already give back a `bool`. Your
trait does not have that yet.

**Place 1 — `llm.rs`, the trait.** Replace it with:

```rust
/// Anything that can answer the detective. The game only talks to this.
pub trait InferenceEngine: Send + Sync {
    /// Answers the detective one piece at a time: calls `say` once per piece.
    /// `say` gives back `true` while someone is listening, and `false` once no one is:
    /// then the engine stops talking.
    /// `Ok(())`: the reply ended, whole or stopped. `Err`: the engine failed.
    fn reply(&self, prompt: &str, say: &mut dyn FnMut(&str) -> bool) -> AppResult<()>;
}
```

**Place 2 — `llm.rs`, `impl InferenceEngine for MockEngine`.** Replace the doc comment and the
`fn reply` line with these two. **The body stays as it is** — it is step B.

```rust
    /// Says its line one word at a time. Stops as soon as no one is listening.
    fn reply(&self, _prompt: &str, say: &mut dyn FnMut(&str) -> bool) -> AppResult<()> {
```

**Place 3 — `engine_thread.rs`.** Under the doc comment line of `answer_all`,
`/// Runs on the engine's thread. Answers each question until no one can ask any more.`, add:

```rust
/// If `ask` stops reading pipe 2, `say` gives back `false`, and the engine stops talking.
```

What each part is for:

- `-> bool` after `FnMut(&str)` — each call to `say` now gives back a `bool` (§4.1).
- The trait's doc comment — what `true` and `false` mean for the engine, and what `Ok(())` and `Err`
  mean. (The `Ok`/`Err` line had gone missing in 16a; this puts it back.)
- Place 2 — the trait's line again, with `_prompt` because the mock never reads it.
- Place 3 — says what step A makes `answer_all` do.

Run: it does not compile — `1 previous error`, in `engine_thread.rs`:

```
error[E0308]: mismatched types
  --> src/engine_thread.rs:34:66
   |
34 |           let finished = engine.reply(&question.text, &mut |piece| {
   |  __________________________________________________________________^
35 | |             let _ = question
36 | |                 .answer_to
37 | |                 .blocking_send(Said::Piece(piece.to_string()));
38 | |         });
   | |_________^ expected `bool`, found `()`
```

The trait now says `say` gives back a `bool`. The closure that `answer_all` hands over gives back
nothing — `()` — because its only line throws the result away with `let _ =` and ends in `;`.
Step A.

`MockEngine`'s body still compiles: its line `say(word);` calls `say` and ignores the answer. Rust
allows that for a `bool`. That is why only `engine_thread.rs` fails — and why step B needs a test.

## 3. What each function does, and how to build it

Two kinds of step in this stage:

- **Fill it** — the code is shown with a gap, `____`. The gap is a name, never syntax. Your most
  similar line is right next to it.
- **Build it** — easy, and yours. What the line must do, your own code right there, and the change.

Where the inputs come from: `answer_all` makes the closure and hands it to `engine.reply` as `say`.
The engine calls it with each `piece`. `question` is the `Question` that `answer_all` took out of
pipe 1; `question.answer_to` is pipe 2's sending end inside it.

### 3.1 `answer_all` (engine_thread.rs)

**What it does.** The same as in 16b: for each question, it asks the engine, sends each piece into
pipe 2, then sends the last message. What changes: the closure now tells the engine whether the piece
got in.

#### Step A — Fill it: `say` gives back whether the piece got in

Replace the closure — the lines from `let finished` to `});` — with this:

```rust
        let finished = engine.reply(&question.text, &mut |piece| {
            question
                .answer_to
                .blocking_send(Said::Piece(piece.to_string()))
                .____()
        });
```

One gap: the method that turns `blocking_send`'s `Result` into `true` (it got in) or `false` (it did
not). §4.2.

**Your most similar line.** In `case_file.rs`, inside `try_from`:

```rust
            if case.suspect(suspect_id).is_some() {
```

`is_some()` asks an `Option` a yes-or-no question: "is there something in you?" The gap asks a
`Result` the same kind of question: "did it work?"

| part | what it does |
|---|---|
| `question.answer_to.blocking_send(…)` | your 16b send, unchanged: puts the piece into pipe 2. Gives back `Ok(())` if it got in, `Err` if `ask` is gone |
| `.____()` | turns that `Ok` or `Err` into `true` or `false` |
| no `let _ =` in front | `let _ =` threw the result away. Now the result is what the closure gives back |
| no `;` at the end | the last line of a closure, without `;`, is what it gives back — like the last line of your `is_slug` (§5.2). With `;` it gives back nothing |

In the map: this is the arrow `true` / `false`, from the engine thread back to the engine.

Run: `cargo test --test no_one_listens` → `3 passed; 1 failed`:

```
---- the_mock_stops_as_soon_as_say_gives_back_false stdout ----
assertion `left == right` failed
  left: ["I ", "was ", "at ", "home ", "all ", "night."]
 right: ["I ", "was "]
```

- Passing: the two tests that use `LongWindedEngine`, and `the_mock_says_its_whole_line_while_someone_listens`.
  The engine thread now says `false` when no one listens, and the test's engine listens.
- Failing: `MockEngine` does not listen yet. The test's `say` gave back `false` after `"was "`, and
  the mock went on to the end of its line. Step B.

### 3.2 `MockEngine::reply` (llm.rs)

- **Now (16a):** says every word, and ignores what `say` gives back:

  ```rust
          for word in self.line.split_inclusive(' ') {
              say(word);
          }
          Ok(())
  ```

- **After 17a:** stops at the first `false`.

#### Step B — Build it: stop when `say` gives back `false`

**What it must do.** Call `say(word)`. If it gives back `false`, stop the loop. The `Ok(())` after
the loop stays as it is.

**What you have.** In `storage.rs`, `load_case` starts with an `if` whose condition is a call with
`!` in front:

```rust
    if !is_slug(slug) {
        return Err(AppError::CaseNotFound {
            slug: slug.to_string(),
        });
    }
```

And in `state.rs`, the loop in `ask` stops with `break`:

```rust
                Said::Done => break,
```

**The change.** Replace the line `say(word);` with an `if` shaped like the one in `load_case`:

- the condition: `!` and the call `say(word)`, where `load_case` has `!is_slug(slug)`;
- inside the `{ }`: `break;`, where `load_case` has `return Err(…);`.

`!` turns `false` into `true`, so the `if` runs exactly when no one is listening. `break` stops a
`for` loop the same way it stops your `loop` in `ask`; then the line after the loop runs: `Ok(())`
(§5.4).

Run: `cargo test --test no_one_listens` → `4 passed; 0 failed`.

Last: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (184 tests: you had 180,
`no_one_listens.rs` adds 4). Say ready.

### If it does not compile

`engine_thread.rs`:

| the compiler says | what it means |
|---|---|
| after §2: `E0308 … expected bool, found ()`, at the closure | Expected. Step A. |
| `E0308 … expected bool, found ()`, with `help: remove this semicolon to return this value` under `.is_ok();` | Take the `;` off the last line. With it, the closure gives back nothing. |
| `E0308 … expected bool, found ()`, and the closure still starts with `let _ =` | Take `let _ =` off. It throws the answer away. |
| it compiles, but two tests fail with `the engine said 1000 pieces; it should stop by the 3rd` | The closure gives back `true` whatever happens. It must give back whether the send worked: the gap. |
| it compiles, but `when_ask_stops_early_…` fails with `left: Ok(1)`, `right: Ok(1000)` — and 7 tests fail in the whole suite | The gap is `is_err`. It says `true` when the send *failed*, so every engine stops after one piece. |

`llm.rs`:

| the compiler says | what it means |
|---|---|
| `the_mock_stops_as_soon_as_say_gives_back_false` fails with `left: ["I ", "was ", "at ", "home ", "all ", "night."]` | Step B is not done yet, or the `if` does not stop the loop. |
| both mock tests fail with `left: ["I "]` | The `!` is missing: the loop stops when someone *is* listening. |
| both mock tests fail with every word twice: `left: ["I ", "I ", "was ", "was ", …]` | The old `say(word);` is still above the `if`. The `if` calls `say` itself — take the old line out. |
| clippy: `equality checks against false can be replaced by a negation` | You wrote `say(word) == false`. Write `!say(word)`. |

## 4. New syntax and methods

### 4.1 A function input that answers back: `FnMut(&str) -> bool`

**What it is.** In 16a, `say` became a function the engine could call: `&mut dyn FnMut(&str)`. It
took a `&str` and gave back nothing. Now it also gives back a `bool`. The part after `->` is what each
call gives back — the same `->` as in a `fn` line.

**Form.**

```rust
fn reply(&self, prompt: &str, say: &mut dyn FnMut(&str) -> bool) -> AppResult<()>;
//                                               ────── ───────
//                                               takes  gives back
```

A call gives you the `bool`. You can keep it in a name, or use the call straight away wherever a
`bool` goes — in a condition, for example:

```rust
let listening = say("I ");     // listening is true or false
```

**The closure you hand over must give back a `bool`.** Its last line, without `;`, is what it gives
back. An example from the test file:

```rust
    let said = engine.reply("Where were you on Tuesday?", &mut |piece| {
        pieces.push(piece.to_string());
        pieces.len() < 2
    });
```

The first line ends in `;`: it just runs. The last line has no `;`: `pieces.len() < 2` is the answer.
It is `true` after the first piece and `false` after the second, so `MockEngine` says `"I "` and
`"was "`, then stops.

With a `;` after the last line, the closure gives back nothing, and the compiler says (measured):

```
error[E0308]: mismatched types
...
38 | |                 .is_ok();
   | |                         - help: remove this semicolon to return this value
39 | |         });
   | |_________^ expected `bool`, found `()`
```

**What to know.** The one who calls may ignore the answer: `say(word);` compiles. The trait's doc
comment says to stop on `false`, but the compiler cannot force an engine to. That is why
`MockEngine` has its own test.

**When to use it.** When the code that calls a function needs an answer back from it. Here the
engine calls `say`, and needs to know whether to go on.

**Why here.** The engine cannot see pipe 2 — only the engine thread can. `say` is the only line
between the two. So the answer travels back along it.

### 4.2 `.is_ok()` — did it work?

**What it is.** A method every `Result` has. It gives back `true` if the result is `Ok(…)`, and
`false` if it is `Err(…)`. What is inside is not used. It is the twin of `is_some()` on an `Option`
(§5.5).

**Form.**

```rust
some_result.is_ok()      // a bool
```

**Example**, with other names — sending the question into pipe 1, in `ask`:

```rust
let got_in = self.questions.send(question).is_ok();
```

`got_in` is `true` if the question got into pipe 1, `false` if the engine thread is gone.

**When to use it.** When you only need to know whether something worked, not what is inside the
`Ok` or the `Err`.

**Why here.** `say` must answer yes or no. The engine does not need the error — only whether to go on.

## 5. Syntax you already know — from your code

Everything §3 used from your code, in one place.

### 5.1 A send fails when no one is at the other end

`state.rs :: ask` — sending the question into pipe 1:

```rust
        self.questions
            .send(question)
            .map_err(|e| AppError::Inference {
                message: e.to_string(),
            })?;
```

If the engine thread is gone, nothing holds the other end of pipe 1, and `send` gives back `Err`.
Pipe 2 works the same way, the other way round: once `ask` is gone, `blocking_send` gives back `Err`.

`engine_thread.rs :: answer_all` — the closure before this stage (16b). A closure with a `{ }` body;
`let _ =` throws `blocking_send`'s result away:

```rust
        let finished = engine.reply(&question.text, &mut |piece| {
            let _ = question
                .answer_to
                .blocking_send(Said::Piece(piece.to_string()));
        });
```

### 5.2 The last line, without `;`, is what comes back

`storage.rs :: is_slug`:

```rust
fn is_slug(slug: &str) -> bool {
    !slug.is_empty()
        && slug
            .bytes()
            .all(|b| matches!(b, b'a'..=b'z' | b'0'..=b'9' | b'-'))
}
```

One expression, no `;`, and it is a `bool`: that is what `is_slug` gives back. A closure's `{ }` body
works the same way.

### 5.3 `if !` with a call as the condition

`storage.rs :: load_case`:

```rust
    if !is_slug(slug) {
        return Err(AppError::CaseNotFound {
            slug: slug.to_string(),
        });
    }
```

`is_slug(slug)` gives back a `bool`; `!` turns it round. The body runs only when the slug is *not* a
plain name.

### 5.4 `break`

`state.rs :: ask`, your 16b loop:

```rust
        loop {
            ...
            match said {
                ...
                Said::Done => break,
                ...
            }
        }

        self.record(Speaker::Suspect, &reply)?;
```

`break` leaves the loop at once, and the line after the loop runs next. It does the same in a
`for … in …` loop: the loop ends early, and the line after it runs.

### 5.5 `is_some()`

`case_file.rs :: try_from`:

```rust
            if case.suspect(suspect_id).is_some() {
```

`case.suspect(…)` gives back an `Option`; `is_some()` gives back `true` if there is a suspect in it.

## 6. What you built

When `ask` stops early:

1. `ask` ends, and its end of pipe 2, `answer`, is dropped.
2. The engine says its next piece. On the engine thread, `say` tries to put it into pipe 2. The send
   fails, so `say` gives back `false`.
3. The engine sees `false`, says nothing more, and gives back `Ok(())`.
4. `answer_all` takes the next question out of pipe 1. The engine said 2 or 3 pieces, not the whole
   reply.

While someone listens, nothing changes: `say` gives back `true` every time, and the whole reply goes
out.

New in this stage:

- **A function input that answers back** — `&mut dyn FnMut(&str) -> bool`. The closure gives back its
  last line; the caller gets a `bool` from each call.
- **`.is_ok()`** — a `Result` as `true` or `false`.
- The idea under both: **a thread cannot be stopped from outside.** The work checks, at each step,
  whether to go on, and stops itself.

Yours: the closure's answer (step A) and the mock's stop (step B).

Next, Stage 17b: the player presses **Stop**. `ask` will stop, and — because of this stage — the
engine will stop right after it.
