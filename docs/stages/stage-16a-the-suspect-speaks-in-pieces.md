# Stage 16a — the suspect speaks in pieces

Test:  src-tauri/tests/pieces.rs — 5 tests
Run:   cd src-tauri && cargo test --test pieces

## 1. What part of the app

The path of one question, as in Stage 15:

```
React → ask_suspect (ipc.rs) → AppState::ask (state.rs) → pipe 1 → engine thread → engine.reply (llm.rs)
```

This stage changes the **last arrow**: how the engine hands its reply to the engine thread.

**The problem.** A real model does not write its reply all at once. It makes it one small piece at
a time: a word, or part of a word. Today `reply` gives back the whole `String` at the end:

```rust
    fn reply(&self, prompt: &str) -> AppResult<String>;
```

So nothing can be shown until the very last piece is made. In Stage 16c the player will see the
words appear while the suspect is still "typing". For that, the engine must hand over each piece the
moment it has it.

**The fix.** `reply` gets a second input, `say`. `say` is a **function**. The engine calls it once
for each piece: `say("I ")`, `say("was ")`, … Whoever calls `reply` decides what `say` does with a
piece. In TypeScript you would type such an input like this:

```ts
say: (piece: string) => void
```

In Rust it is written `say: &mut dyn FnMut(&str)` (§4.1).

In this stage, the engine thread is the one who calls `reply`. Its `say` adds each piece to a
`String`. When the engine is done, that `String` is the whole reply, and it goes into pipe 2 as
before. So `ask` gets exactly what it got in Stage 15. Nothing changes for the player yet.

### The map

```
  THE ENGINE THREAD — answer_all (engine_thread.rs)      THE ENGINE — reply (llm.rs)

  question.text  ──────────── prompt ────────────────▶   reply(&self, prompt, say)
  heard = ""
  the closure  ─────────────── say ──────────────────▶     for each word of the line:
    |piece| heard.push_str(piece)                              say(word)
          ▲                                                       │
          └──────────── runs once per piece: piece = word ────────┘
                                                             Ok(())   or   Err(error)
  said  ◀────────────────────────────────────────────────────────┘
  answer = Ok(heard)  or  Err(error)
  question.answer_to.blocking_send(answer)  ──── pipe 2 ────▶  ask   (no change)
```

Every name in the map:

| name | what it is | where it is made, who holds it |
|---|---|---|
| `say` | a function the engine calls once per piece. Its type is `&mut dyn FnMut(&str)` (§4.1) | `reply`'s second input. `answer_all` makes it |
| the closure | `\|piece\| heard.push_str(piece)` — the function `answer_all` hands over as `say` (§4.2) | written in `answer_all`; called by the engine |
| `piece` | one piece of the reply, a `&str` such as `"was "` | the closure's input; the engine gives it on each call |
| `heard` | a `String` that grows by one piece at each call | `answer_all`. A new, empty one for each question |
| `said` | what `engine.reply(…)` gives back: `Ok(())` when the reply is complete, or the engine's error. The text is **not** in it — the text went out through `say` | `answer_all` |
| `answer` | `Ok(heard)` or `Err(error)` — the same type as pipe 2 carried in Stage 15 | `answer_all` → pipe 2 → `ask` |
| `word` | one word of `MockEngine`'s line, with its space | `MockEngine::reply`'s loop |

`question`, `inbox`, `engine`, `answer_to`, pipe 1 and pipe 2 stay as in Stage 15.

### One question, start to end

With `MockEngine::new("I was at home all night.")`. Values printed with `dbg!`, as in 14c.

1. `ask` sends a `Question` into pipe 1. (No change.)
2. `answer_all` takes it out and makes `heard`, empty.
3. `answer_all` calls `engine.reply(&question.text, &mut |piece| heard.push_str(piece))`.
4. Inside `MockEngine::reply`, the loop calls `say` six times. Each call runs the closure:

   | the engine calls | `piece` | `heard` after the call |
   |---|---|---|
   | `say("I ")` | `"I "` | `"I "` |
   | `say("was ")` | `"was "` | `"I was "` |
   | `say("at ")` | `"at "` | `"I was at "` |
   | `say("home ")` | `"home "` | `"I was at home "` |
   | `say("all ")` | `"all "` | `"I was at home all "` |
   | `say("night.")` | `"night."` | `"I was at home all night."` |

5. `MockEngine::reply` gives back `Ok(())`. So `said` is `Ok(())`.
6. `answer` is `Ok("I was at home all night.")`. `answer_all` puts it into pipe 2.
7. `ask` gets it, keeps it in the room, and gives it to React. (No change.)

If the engine fails halfway — it says `"I was "`, then fails — `said` is
`Err(Inference { message: "out of memory" })`. `answer` is that same error. `"I was "` is thrown away:
half a sentence is not an answer.

## 2. Files and templates

| File | What you do |
|---|---|
| `src/llm.rs` | paste the new trait (place 1) and the new `MockEngine::reply` template (place 2); type its body |
| `src/engine_thread.rs` | in `answer_all`, replace two lines with four short blocks you type |
| `tests/pieces.rs` | nothing — new, mine |
| `tests/engine.rs`, `tests/app_engine.rs`, `tests/engine_thread.rs`, `tests/ask_engine_thread.rs` | nothing — I changed their test engines to the new `reply` |

Run now, before you change anything: it does not compile — `5 previous errors`, all in
`tests/pieces.rs`: `2 × error[E0050]: method reply has 3 parameters but the declaration in trait
reply has 2`, then `E0061`, `E0282`, `E0308`. The tests already use the new `reply`. Your trait does
not yet.

**Place 1 — `llm.rs`, the trait.** Replace it with:

```rust
/// Anything that can answer the detective. The game only talks to this.
pub trait InferenceEngine: Send + Sync {
    /// Answers the detective one piece at a time: calls `say` once per piece.
    /// `Ok(())` when the reply is complete, `Err` when the engine failed.
    fn reply(&self, prompt: &str, say: &mut dyn FnMut(&str)) -> AppResult<()>;
}
```

Two things changed:

- the second input, `say` (§4.1);
- what comes back: `AppResult<()>`. The text no longer comes back here — it went out through
  `say`. `()` means "nothing", as in `Phase::begin` (§5.5).

**Place 2 — `llm.rs`, `impl InferenceEngine for MockEngine`.** Replace it with:

```rust
impl InferenceEngine for MockEngine {
    /// Says its line one word at a time.
    fn reply(&self, _prompt: &str, say: &mut dyn FnMut(&str)) -> AppResult<()> {
        todo!()
    }
}
```

The first line of `reply` is the trait's line, with `_prompt` because the mock never reads it.

Run: it does not compile — `2 previous errors`, both in `engine_thread.rs`, at the old
`engine.reply(&question.text)`:

```
error[E0061]: this method takes 2 arguments but 1 argument was supplied
  --> src/engine_thread.rs:22:28
   |
22 |         let reply = engine.reply(&question.text);
   |                            ^^^^^---------------- argument #2 of type `&mut dyn for<'a> FnMut(&'a str)` is missing

error[E0308]: mismatched types
    --> src/engine_thread.rs:23:50
     |
  23 |         let _ = question.answer_to.blocking_send(reply);
     |                                    ------------- ^^^^^ expected `Result<String, AppError>`, found `Result<(), AppError>`
```

and `warning: unused variable: say` — that is the `todo!()`. The first error: `answer_all` does not
hand over a `say` yet. The second: `reply` no longer gives back the text. `answer_all` is next.

## 3. What each function does, and how to build it

### `answer_all` (engine_thread.rs)

What it does: the same as in Stage 15 — takes each question out of pipe 1, asks the engine, puts the
answer into pipe 2. What changes is how it gets the text: it hands the engine a `say` that collects
the pieces in `heard`.

Where the inputs come from: `engine` and `inbox` are handed in by `start`, as in 14a. `question`
comes out of `inbox` in the `for` loop.

Delete these two lines in the loop:

```rust
        let reply = engine.reply(&question.text);
        let _ = question.answer_to.blocking_send(reply);
```

and type the four blocks below in their place, inside the loop, in this order.

**Block 1 — an empty `String` for this question.**

```rust
        let mut heard = String::new();
```

In the map: `heard` starts empty (§4.3). It is `mut` because the closure will add to it. It is
inside the loop, so each question gets a new, empty one.

**Block 2 — ask the engine, and hand it `say`.**

```rust
        let said = engine.reply(&question.text, &mut |piece| heard.push_str(piece));
```

In the map: `|piece| heard.push_str(piece)` is the closure: it becomes `say` inside `reply`
(§4.2). Each time the engine calls `say("was ")`, this closure runs, and `"was "` is added to the
end of `heard` (§4.3). When `reply` is done, `said` holds `Ok(())` or the error, and `heard` holds
the whole text.

**Block 3 — make the answer.**

```rust
        let answer = match said {
            Ok(()) => Ok(heard),
            Err(error) => Err(error),
        };
```

In the map: if the engine finished, the answer is everything heard. If it failed, the answer is its
error, and what was heard is thrown away. `Ok` and `Err` are the two variants of `Result`; you match
them like the variants of `Phase` (§5.4).

**Block 4 — send it, as before.**

```rust
        let _ = question.answer_to.blocking_send(answer);
```

In the map: the answer goes into pipe 2. The same line as in Stage 15, with `answer` in place of
`reply`.

Run: `4 passed; 1 failed`. The one that fails is `the_mock_says_its_line_one_word_at_a_time`, with
`panicked at src/llm.rs:26:9: not yet implemented` — that is `MockEngine`'s `todo!()`. The other four
use test engines that already say pieces. You also get `warning: field line is never read`: the
`todo!()` again.

### `MockEngine::reply` (llm.rs)

What it does: says its line one word at a time. Each word keeps its space, so the pieces put
together are exactly the line.

Where the inputs come from: `self.line` is the line given to `MockEngine::new`. `say` is whatever the
caller hands over — on the engine thread, the closure from block 2.

Replace `todo!()` with:

```rust
        for word in self.line.split_inclusive(' ') {
            say(word);
        }
        Ok(())
```

In the map: `split_inclusive(' ')` cuts the line after each space (§4.4). The `for` loop takes the
words one at a time (§5.1). `say(word)` calls the function the engine was handed — so on the engine
thread, `word` is added to `heard` (§4.1). `Ok(())` says the reply is complete (§5.5).

Run: `5 passed; 0 failed`.

Last: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (175 tests). Say ready.

### If it does not compile

| the compiler says | fix |
|---|---|
| after place 2 only: `E0061 this method takes 2 arguments but 1 argument was supplied` and `E0308 … expected Result<String, AppError>, found Result<(), AppError>`, in `engine_thread.rs` | Expected. Do `answer_all`. |
| `E0308 mismatched types … expected &mut dyn FnMut(&str), found closure` | `&mut` is missing before the closure: `&mut \|piece\| heard.push_str(piece)` (§4.2). |
| `E0596 cannot borrow heard as mutable, as it is not declared as mutable` | `let mut heard = String::new();` |
| `E0382 borrow of moved value: heard` … `value moved here, in previous iteration of loop` | `let mut heard` is above the `for` line. Move it inside the loop. Do **not** take the compiler's `.clone()` hint: it compiles, but the second answer then holds both replies, and `every_answer_starts_from_nothing` fails with `left: Some(Ok("I... I was home.I... I was home."))`. |
| `E0308 mismatched types … expected Result<String, AppError>, found String` | Block 4 sends `heard`. Send `answer`. |
| `E0308 match arms have incompatible types … expected String, found Result<_, AppError>` | The first arm must wrap it: `Ok(()) => Ok(heard),` |
| in `llm.rs`: `E0308 … expected Result<(), AppError>, found ()` and `for loops evaluate to unit type ()` | `Ok(())` is missing after the loop. |
| a test fails with `left: ["I", "was", "at", "home", "all", "night."]` | You wrote `split(' ')`, which throws the spaces away. `split_inclusive(' ')` keeps them (§4.4). |

## 4. New syntax and methods

### 4.1 A function as an input: `say: &mut dyn FnMut(&str)`

**What it is.** An input whose value is a function. The function that receives it can call it,
like any other function: `say(word)`.

**Form.**

```rust
name: &mut dyn FnMut(<the types it takes>)
```

Read it from the inside out:

- `FnMut(&str)` — a function that takes one `&str` and gives back nothing. `FnMut` means calling it
  may **change** something: our closure changes `heard` on every call.
- `dyn` — "any type that is such a function". The same `dyn` as in `Box<dyn InferenceEngine>`
  (§5.3). Every closure has its own type that nobody can name, so `dyn` is how to accept them all.
- `&mut` — the function is **lent** to `reply`. `reply` may call it as often as it likes, but does not
  keep it. `mut`, because calling it changes it.

Calling it is the same as calling any function: `say(word);`.

**Example**, from `tests/pieces.rs` — a test engine that says three pieces:

```rust
impl InferenceEngine for HesitantEngine {
    fn reply(&self, _prompt: &str, say: &mut dyn FnMut(&str)) -> AppResult<()> {
        say("I... ");
        say("I was ho");
        say("me.");
        Ok(())
    }
}
```

**When to use it.** When a function makes things one at a time, and its caller decides what to do
with each one.

**Why here.** The engine makes the pieces; the caller decides what they are for. Today
`answer_all` adds them up. In 16b it will send each one into pipe 2. The tests put them in a `Vec`.
`llm.rs` never needs to know — it stays free of channels and Tauri.

**The compiler's spelling.** Error messages write this type as `&mut dyn for<'a> FnMut(&'a str)`.
It is the same type, written out in full. You never write it.

### 4.2 Handing a closure over: `&mut |piece| …`

You already write closures (§5.2). To hand one over as `say`, put `&mut` in front of it.

**Form.**

```rust
&mut |piece| heard.push_str(piece)
```

- `|piece|` — the closure's one input. Its type, `&str`, comes from `FnMut(&str)` — you do not write it.
- `heard.push_str(piece)` — what it does each time it is called. It uses `heard`, a variable from
  around it, and changes it. That is why `heard` must be `let mut`.
- `&mut` — lends the closure to `reply` (§4.1).

**Example**, from `tests/pieces.rs` — the test collects the pieces in a `Vec` instead:

```rust
    let mut pieces: Vec<String> = Vec::new();

    let said = engine.reply("Where were you on Tuesday?", &mut |piece| {
        pieces.push(piece.to_string())
    });
```

**Who runs it, and when.** The engine runs it, inside `reply`, each time it calls `say`. That is on
the engine thread, because `reply` runs there. When `reply` returns, the closure is gone, and `heard`
is free again — so block 3 can move it into `Ok(heard)`.

### 4.3 `String::new()` and `push_str`

`String::new()` makes an empty `String`. `heard.push_str("was ")` adds `"was "` to its end, so
`heard` must be `mut`. It is `push` for text: `turns.push(…)` adds to a `Vec` (§5.6). In TypeScript:
`heard += "was "`.

### 4.4 `split_inclusive(' ')`

Cuts a text after each space. Each part keeps its space.

```rust
"I was home".split_inclusive(' ')     // "I ", "was ", "home"
"I was home".split(' ')               // "I",  "was",  "home"  — the spaces are gone
```

It hands the parts over one at a time, so `for word in …` walks them, like `for question in inbox`.
`' '` with single quotes is one character; `"I was home"` with double quotes is text.

## 5. Syntax you already know — from your code

### 5.1 `for … in …`

`engine_thread.rs :: answer_all`:

```rust
    for question in inbox {
```

`case_file.rs :: try_from`:

```rust
        for raw_suspect in &raw.suspects {
```

### 5.2 Closures

`ipc.rs :: CaseIntro::from` — a closure that takes `suspect` and builds a summary:

```rust
            suspects: case
                .suspects()
                .map(|suspect| SuspectSummary {
                    id: suspect.id,
                    name: suspect.name.clone(),
                })
                .collect(),
```

`engine_thread.rs :: start` — a closure with no inputs, `||`:

```rust
    thread::spawn(move || answer_all(engine, inbox));
```

### 5.3 `dyn`

`engine_thread.rs :: start`:

```rust
pub fn start(engine: Box<dyn InferenceEngine>) -> Sender<Question> {
```

`dyn InferenceEngine`: any type that has `reply`. In §4.1, `dyn FnMut(&str)`: any type that can be
called with a `&str`.

### 5.4 `match` on the variants of an enum

`transcript.rs :: Phase::record`:

```rust
        match self {
            Phase::Interrogating { turns, .. } => {
                turns.push(Turn {
                    speaker,
                    text: text.to_string(),
                });
                Ok(())
            }
            _ => Err(self.refusal("record a line")),
        }
```

`Result` is an enum too. Its two variants are `Ok(value)` and `Err(error)`. Block 3 matches them the
same way, and names what is inside: `Err(error)`.

### 5.5 `AppResult<()>` and `Ok(())`

`transcript.rs :: Phase::begin`:

```rust
    pub fn begin(&mut self, suspect: SuspectId) -> AppResult<()> {
        if !matches!(self, Phase::Briefing) {
            return Err(self.refusal("begin an interrogation"));
        }

        *self = Phase::Interrogating {
            suspect,
            turns: Vec::new(),
        };
        Ok(())
    }
```

`()` is "nothing". `AppResult<()>` is "it worked, and there is nothing to give back — or here is the
error". `reply` now says the same.

### 5.6 `push` on a `Vec`

`transcript.rs :: Phase::record`, above: `turns.push(Turn { … })` adds one turn to the end of the
`Vec`. `push_str` does the same for a `String`.

## 6. What you built

For one question in the game:

1. `ask` sends the question into pipe 1.
2. `answer_all` makes an empty `heard`, and calls `engine.reply` with the closure as `say`.
3. The engine calls `say` once per piece. Each call adds the piece to `heard`.
4. The engine returns `Ok(())`. `answer_all` turns that into `Ok(heard)`, the whole reply.
5. The answer goes into pipe 2, and `ask` gets it, as in Stage 15.

If the engine fails halfway, `said` is its error, the answer is that error, and the half sentence in
`heard` is thrown away.

New in this stage:

- **A function as an input** — `say: &mut dyn FnMut(&str)`, called as `say(word)`.
- **Handing a closure over** — `&mut |piece| heard.push_str(piece)`: the closure changes a variable
  around it, each time it is called.
- `String::new()`, `push_str`, `split_inclusive`.

Next, Stage 16b: each piece travels to `ask` by itself, through pipe 2, the moment the engine says
it. Then 16c: React shows the pieces as they arrive.
