# Stage 16c — React shows the pieces

Test:  src-tauri/tests/pieces_to_react.rs — 4 tests
Run:   cd src-tauri && cargo test --test pieces_to_react

## 1. What part of the app

The path of one question, as in 16b, with one new arrow at the bottom:

```
React (App.tsx) → ask_suspect (ipc.rs) → AppState::ask (state.rs) → pipe 1 → engine thread → engine.reply (llm.rs)
      ▲                                          │      ▲                                   │
      │                                          │      └──────────── pipe 2 ───────────────┘
      └─────────────── on_piece (new) ───────────┘
```

This stage adds the bottom arrow: a channel from `ask` back to React.

**The problem.** Since 16b each piece reaches `ask` by itself. But `ask` keeps them all in `reply`,
and React gets only the whole reply, at the very end. With a real model that is several seconds of
an empty screen, then the whole answer at once.

**The fix.**

- For each question, React makes a **channel**: a Tauri `Channel`. React hands it to `ask_suspect`
  together with the question.
- `ask_suspect` hands it to `ask`. There it is called `on_piece`.
- Each time `ask` takes a piece out of pipe 2, it adds it to `reply` (as in 16b), **and sends it into
  `on_piece`**.
- On the React side, the channel runs a function, `onmessage`, once for each piece. It adds the piece
  to the text on screen.

That is the new thing: a Tauri `Channel` — a pipe from Rust to React, made by React for one call.
§4.1 is the Rust side, §4.2 the TypeScript side.

`ask` still keeps the whole reply in the room and still gives it back. React gets it as the result of
`invoke` too, but by then the screen already shows it.

**One new error.** If the piece cannot reach React (the window has closed), `send` fails. `ask` then
stops with a new error, `AppError::WindowUnreachable`, and keeps no reply in the room: a reply nobody
saw is not part of the interrogation. (Why: `docs/DECISIONS.md`, 2026-10-14.)

### The map

```
  REACT — ask() in App.tsx                 RUST — ask (state.rs)                   THE ENGINE THREAD

  onPiece = new Channel<string>()
  onPiece.onmessage = (piece) => …
  invoke("ask_suspect",
     { question, onPiece })  ─────────▶  ask_suspect(state, question, on_piece)
                                           └─▶ state.ask(&question, on_piece)
                                                 Question into pipe 1  ──────────────▶  engine.reply(…)
                                                 reply = ""
                                                 loop {                                   say("I ")
                                                   said = Piece("I ")   ◀──── pipe 2 ────   → Said::Piece("I ")
                                                   reply = "I "
  onmessage("I ")  ◀────── on_piece ─────────────  on_piece.send("I ")
    screen: "I "
                                                   …  one round per piece  …
                                                   said = Done          ◀───────────────  Said::Done
                                                   break
                                                 }
                                                 keep the reply in the room
  invoke resolves  ◀───────────────────────────  Ok(reply)
```

Every name in the map:

| name | what it is | where it is made, who holds it |
|---|---|---|
| `Channel` | Tauri's pipe from Rust to React. `Channel<String>` in Rust, `Channel<string>` in TypeScript: the same pipe, one end in each language | React makes it with `new Channel` |
| `onPiece` | React's channel for one question | made in `ask()` in `App.tsx`, then handed to `invoke` |
| `onmessage` | the function React runs once for each piece that arrives, in order | set on `onPiece`, before `invoke` |
| `on_piece` | the same channel, on the Rust side. Tauri turns React's key `onPiece` into the Rust parameter `on_piece` | a parameter of `ask_suspect`; handed to `ask`, which holds it until it ends |
| `on_piece.send(piece)` | puts one piece into the channel. React's `onmessage` runs with it | `ask`, in the `Piece` arm |
| `piece` | one piece, a `String` (as in 16b) | `ask`, from `Said::Piece(piece)` |
| `reply` | a `String` that grows by one piece each round (as in 16b) | `ask` |
| `AppError::WindowUnreachable { message }` | the new error: a piece could not reach React | made in `ask`, when `send` fails |
| `reply` / `setReply` | the text on screen, in React | `App.tsx` |

`said`, `answer`, `Said`, `answer_all`, pipe 1 and pipe 2 stay as in 16b.

### One question, start to end

Measured: I ran the app with the `App.tsx` from §2 in a test window, clicked **Call Viktor in**, then
**Ask**. The engine is the `MockEngine` in `lib.rs`: `"I have nothing to say to you."`

1. React makes `onPiece`, sets `onmessage`, and calls `invoke("ask_suspect", { question, onPiece })`.
2. Tauri calls `ask_suspect` with `question` and `on_piece` — React's channel.
3. `ask_suspect` calls `state.ask(&question, on_piece)`.
4. `ask` keeps the question in the room, sends it into pipe 1, and makes `reply`, empty. (No change.)
5. Each round of the loop:

   | `ask` takes out: `said` | `reply` after that round | `ask` sends into `on_piece` | React's console |
   |---|---|---|---|
   | `Piece("I ")` | `"I "` | `"I "` | `piece: I` |
   | `Piece("have ")` | `"I have "` | `"have "` | `piece: have` |
   | `Piece("nothing ")` | `"I have nothing "` | `"nothing "` | `piece: nothing` |
   | `Piece("to ")` | `"I have nothing to "` | `"to "` | `piece: to` |
   | `Piece("say ")` | `"I have nothing to say "` | `"say "` | `piece: say` |
   | `Piece("to ")` | `"I have nothing to say to "` | `"to "` | `piece: to` |
   | `Piece("you.")` | `"I have nothing to say to you."` | `"you."` | `piece: you.` |

6. `ask` takes out `Done`: `break`. It keeps the whole reply in the room and gives back `Ok(reply)`.
   (As in 16b.)
7. React's `invoke` resolves with `"I have nothing to say to you."`. The screen already shows it.

**If the engine fails halfway** (`FailsHalfwayEngine` in the test): React has already got `"I was "`.
Then `invoke` fails with the engine's error. React's `catch` clears the half reply and shows the error.

**If React's window has closed** (`closed_window()` in the test): the first `send` fails. `ask` stops
with `WindowUnreachable`. The room keeps the question, and no reply.

## 2. Files and templates

| File | What you do |
|---|---|
| `src/error.rs` | paste one new variant (place 1) |
| `src/state.rs` | paste one import and `ask`'s new first line (place 2); then the `Piece` arm (§3.2, steps B and C) |
| `src/ipc.rs` | paste one import and `ask_suspect`'s new first lines (place 3); then its last line (§3.1, step A) |
| `src/App.tsx` — the React file, outside `src-tauri` | replace the whole file (place 4); then type the channel lines (§3.3, step D) |
| `tests/pieces_to_react.rs` | nothing — new, mine |
| `tests/ask_command.rs` | nothing — I changed it: `ask_suspect` takes a third input |
| `tests/app_engine.rs`, `ask_async.rs`, `ask_engine_thread.rs`, `pieces.rs`, `pieces_travel.rs`, `room_exchange.rs` | nothing — I changed them: every `app.ask(…)` now passes `no_window()`, a channel that drops every piece |

Run now, before you change anything: it does not compile — `5 previous errors`:

```
error[E0061]: this method takes 1 argument but 2 arguments were supplied     (4 times)
error[E0599]: no variant named `WindowUnreachable` found for enum `AppError`
```

The test passes `ask` a channel, and uses the new error. Neither exists yet.

**Place 1 — `error.rs`.** At the end of `AppError`, after `BackgroundFailed { message: String },` and
before the closing `}`, paste:

```rust

    #[error("the window could not be reached: {message}")]
    WindowUnreachable { message: String },
```

It reaches React as `{ kind: "windowUnreachable", message: "…" }`, like every other variant.

**Place 2 — `state.rs`.** Under `use tauri::async_runtime::channel;`, add:

```rust
use tauri::ipc::Channel;
```

Then change `ask`'s doc comment and its first line to this. Everything inside `ask` stays.

```rust
    /// The suspect's reply to what the detective just asked.
    /// The engine says it piece by piece on its own thread. `ask` sends each piece to React
    /// through `on_piece` the moment it comes, and puts the pieces together.
    pub async fn ask(&self, text: &str, on_piece: Channel<String>) -> AppResult<String> {
```

**Place 3 — `ipc.rs`.** Under `use tauri::async_runtime::spawn_blocking;`, add:

```rust
use tauri::ipc::Channel;
```

Then replace the first two lines of `ask_suspect` — `#[tauri::command]` and the `pub async fn` line —
with these. **Leave its last line, `state.ask(&question).await`, as it is.** That line is step A.

```rust
/// Asks the suspect in the room. Each piece of the reply goes to React through `on_piece`.
#[tauri::command]
pub async fn ask_suspect(
    state: State<'_, AppState>,
    question: String,
    on_piece: Channel<String>,
) -> AppResult<String> {
```

**Place 4 — `src/App.tsx`** (project root, not `src-tauri`). Replace the whole file with this. It is
the room for now: one button to call Viktor in, one box to ask. The `try` inside `ask()` is empty —
that is step D. Until then, your editor marks `Channel` as unused; that is expected.

```tsx
import { useState } from "react";
import { Channel, invoke } from "@tauri-apps/api/core";
import "./App.css";

// The interrogation room, for now: call Viktor in, ask him something,
// and watch his reply arrive piece by piece.
function App() {
  const [question, setQuestion] = useState("Where were you on Tuesday?");
  const [reply, setReply] = useState("");
  const [problem, setProblem] = useState("");

  async function callViktorIn() {
    setProblem("");
    try {
      await invoke("begin_interrogation", { slug: "the-ledger", suspect: 2 });
    } catch (error) {
      setProblem(JSON.stringify(error));
    }
  }

  async function ask() {
    setReply("");
    setProblem("");
    try {
      // Step D: the channel, then the question.
    } catch (error) {
      setReply("");
      setProblem(JSON.stringify(error));
    }
  }

  return (
    <main className="container">
      <h1>Interrogation room</h1>

      <div className="row">
        <button onClick={callViktorIn}>Call Viktor in</button>
      </div>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          ask();
        }}
      >
        <input
          value={question}
          onChange={(e) => setQuestion(e.currentTarget.value)}
        />
        <button type="submit">Ask</button>
      </form>

      <p>{reply}</p>
      <p>{problem}</p>
    </main>
  );
}

export default App;
```

What each part is for:

- `WindowUnreachable` — the error from §1, for a piece that cannot reach React.
- `use tauri::ipc::Channel;` — the Rust name of Tauri's channel (§4.1).
- `on_piece: Channel<String>` in `ask` — `ask` now gets React's channel. `String` is what travels in it.
- `on_piece: Channel<String>` in `ask_suspect` — Tauri fills it from React's `onPiece`, the way it
  fills `question`.
- `App.tsx` — `callViktorIn` is your Stage 10h console line as a button. `ask()` will be the Stage 11d
  line, plus the channel. `reply` is the text on screen; `problem` shows an error as JSON.

Run: it does not compile — `1 previous error`, in `ask_suspect`:

```
error[E0061]: this method takes 2 arguments but 1 argument was supplied
  --> src/ipc.rs:85:11
   |
85 |     state.ask(&question).await
   |           ^^^----------- argument #2 of type `Channel<std::string::String>` is missing
```

`ask` now wants the channel, and `ask_suspect` does not pass it yet. You also get one warning,
`unused variable: on_piece`, in `state.rs`. Both are next.

## 3. What each function does, and how to build it

Four kinds of step, from most shown to least:

- **Type it** — the code is shown. You type it. Under it, a table says what each part does.
- **Fill it** — the line is shown with gaps, `____`. Each gap is a name you already have, never
  syntax. Your most similar line is right next to it.
- **Order it** — the lines are shown, out of order. You put them in order. If the order is wrong, the
  step shows what the compiler says.
- **Build it** — easy, and yours. What the line must do, your own code right there, and the change.

Where the new parameter comes from: React makes the channel and passes it to `invoke` as `onPiece`.
Tauri hands it to `ask_suspect` as `on_piece`. `ask_suspect` hands it to `ask`.

### 3.1 `ask_suspect` (ipc.rs)

**What it does.** The same thin door as in Stage 11d: it hands everything to `ask`. Now that includes
the channel.

#### Step A — Build it: pass the channel to `ask`

**What it must do.** Call `ask` with the question **and** the channel.

**What you have.** The last line of `ask_suspect`, from Stage 12:

```rust
    state.ask(&question).await
```

**The change.** Add `on_piece` as the second input, after `&question` and a comma. No `&` before it:
`ask` says `on_piece: Channel<String>`, not `&Channel<String>`. `ask` takes the channel itself, because
nothing needs it after `ask` ends (§4.1).

Run: `cargo test --test pieces_to_react` → it compiles, with one warning, `unused variable: on_piece`, in
`state.rs`. `0 passed; 4 failed`. It takes about 5 seconds: one test's engine waits for React to get
its first piece, and gives up after 5 seconds.

The failures say what is missing. For example:

```
  left: []
 right: ["I ", "was ", "at ", "home ", "all ", "night."]
```

React got no pieces: `ask` does not send any yet. That is §3.2.

Also `cargo test --test ask_command` → `1 passed`.

### 3.2 `ask` (state.rs)

- **Now (16b):** the `Piece` arm adds the piece to `reply`, and nothing else:

  ```rust
                  Said::Piece(piece) => reply.push_str(&piece),
  ```

- **After 16c:** the `Piece` arm adds the piece to `reply`, and sends it to React.

Two steps: B sends, C keeps.

#### Step B — Fill it: send the piece to React

Replace the `Piece` arm with this block. For now it only sends; `push_str` comes back in step C.

```rust
                Said::Piece(piece) => {
                    on_piece
                        .send(____)
                        .map_err(|e| AppError::____ {
                            message: e.to_string(),
                        })?;
                }
```

Two gaps:

1. what goes to React;
2. which error this is — you pasted it in place 1.

**Your most similar line.** In `ask`, a few lines above the loop — sending the question into pipe 1:

```rust
        self.questions
            .send(question)
            .map_err(|e| AppError::Inference {
                message: e.to_string(),
            })?;
```

| part | what it does |
|---|---|
| `Said::Piece(piece) => { … }` | the arm's body is now a block in `{ }`, because it holds full lines that end in `;`. Same as the `Phase::Interrogating` arm in `Phase::record` (§5.2) |
| `on_piece.send(…)` | puts one value into React's channel. Gives back `Ok(())` when it got through, `Err` when the window is gone (§4.1) |
| `.map_err(\|e\| AppError::… { message: e.to_string() })` | turns Tauri's error into one of ours, as in your pipe-1 line |
| `?` | if it was an error, `ask` stops here and gives it back. Nothing more is kept in the room |

In the map: this is the arrow `on_piece.send("I ")` → `onmessage("I ")`.

Run: `cargo test --test pieces_to_react` → `2 passed; 2 failed`, and one warning:
`variable does not need to be mutable`, for `reply` — nothing adds to it now.

- Passing: `after_a_failure_react_has_seen_the_pieces_before_it` and
  `if_react_cannot_take_a_piece_ask_stops_and_keeps_no_reply`. React gets every piece.
- Failing: `each_piece_reaches_react_in_order` and `react_gets_a_piece_before_the_engine_says_the_next`,
  both with `left: Ok("")`. `reply` stays empty. Step C.

#### Step C — Order it: keep the piece too

The arm must do two things: add the piece to `reply`, and send it to React. These are its two parts,
out of order:

- your step B lines, from `on_piece` to `})?;`
- this line, from 16b:

  ```rust
                      reply.push_str(&piece);
  ```

Put both inside the arm's `{ }`, in the right order. Only one order compiles.

The question to ask yourself: what does `send` do with `piece`, and what does `push_str(&piece)` need?
§4.1 and §5.3 have what you need.

With the wrong order, the compiler says:

```
error[E0382]: borrow of moved value: `piece`
  --> src/state.rs:88:36
   |
82 |                 Said::Piece(piece) => {
   |                             ----- move occurs because `piece` has type `std::string::String`, which does not implement the `Copy` trait
83 |                     on_piece
84 |                         .send(piece)
   |                               ----- value moved here
...
88 |                     reply.push_str(&piece);
   |                                    ^^^^^^ value borrowed here after move
...
help: consider cloning the value if the performance cost is acceptable
```

Do not follow the `help` about `.clone()`. The other order needs no copy.

Run: `cargo test --test pieces_to_react` → `4 passed; 0 failed`. The warning is gone.

### 3.3 `ask()` in App.tsx (React)

**What it does.** Makes a channel for this one question, says what to do with each piece, then asks.

#### Step D — Type it: the channel, then the question

Inside the `try { }` of `ask()`, replace the comment line with:

```ts
      const onPiece = new Channel<string>();
      onPiece.onmessage = (piece) => {
        console.log("piece:", piece);
        setReply((before) => before + piece);
      };
      await invoke("ask_suspect", { question, onPiece });
```

| part | what it does |
|---|---|
| `new Channel<string>()` | makes a new channel, for this question only. `<string>`: each message is a string — Rust's `Channel<String>` |
| `onPiece.onmessage = (piece) => { … }` | the function that runs once per piece, in the order Rust sent them. Set it **before** `invoke`: the pieces arrive while `invoke` is still waiting |
| `console.log("piece:", piece)` | one console line per piece, so you can see them arrive one by one |
| `setReply((before) => before + piece)` | adds the piece to the end of the text on screen |
| `invoke("ask_suspect", { question, onPiece })` | your Stage 11d call, plus the channel. Tauri turns the key `onPiece` into the Rust parameter `on_piece` |
| `await` | waits until `ask` is done. By then the screen already shows the whole reply |

In the map: this is the left column, top to bottom.

**See it work.**

1. Your project folder has no `node_modules` folder right now, so first, from the project root:
   `bun install`, once.
2. From the project root: `bun tauri dev`. A window titled **interrogatory-ai** opens.
3. Right-click inside the window → Inspect → Console. Then, in the window:

   | you click | the page shows | the console shows |
   |---|---|---|
   | **Ask** (before anyone is in the room) | `{"action":"record a line","kind":"invalidState","state":"the briefing"}` | nothing |
   | **Call Viktor in** | nothing — no error | nothing |
   | **Ask** | `I have nothing to say to you.` | seven lines: `piece: I`, `piece: have`, … `piece: you.` |
   | **Call Viktor in** again | `{"action":"begin an interrogation","kind":"invalidState","state":"an interrogation"}` | nothing |

`MockEngine` answers at once, so on screen the words seem to appear together. The seven console lines
show they came as seven messages. With a real model (Stage 19) you will see them arrive.

Last: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (180 tests: you had 176,
`pieces_to_react.rs` adds 4). Say ready.

### If it does not compile

`ipc.rs`:

| the compiler says | what it means |
|---|---|
| after §2: `E0061 this method takes 2 arguments but 1 argument was supplied`, at `state.ask(&question)` | Expected. Step A. |
| `E0308 … expected Channel<String>, found &Channel<String>` | Take the `&` off `on_piece`. `ask` takes the channel itself. |

`state.rs`:

| the compiler says | what it means |
|---|---|
| `E0599 no variant named WindowUnreachable found for enum AppError` | Place 1 is missing, or the name is spelled differently. |
| `E0308 … expected String, found &String`, at `.send(&piece)` | Gap 1 is `piece`, without `&`. `send` takes the `String` itself. |
| it compiles, but `if_react_cannot_take_a_piece_ask_stops_and_keeps_no_reply` fails with `left: Err(Inference { message: "webview not found" })` | Gap 2 is the variant you pasted in place 1. |
| a warning `unused Result that must be used`, and `if_react_cannot_take_a_piece_ask_stops_and_keeps_no_reply` fails | The `?` at the end of the send line is missing. |
| `E0382 borrow of moved value: piece` | Step C: `reply.push_str(&piece);` goes first. `send` takes `piece`; after that, `ask` cannot read it. |
| a test fails with `left: Ok("")` | Step C is not done: nothing adds the piece to `reply`. |

`App.tsx` — what the page shows:

| the page shows | what it means |
|---|---|
| ``"invalid args `onPiece` for command `ask_suspect`: command ask_suspect missing required key onPiece"`` | `onPiece` is missing from `{ question, onPiece }`. |
| the reply, but the console shows no `piece:` lines | `console.log` is missing from `onmessage`. |

## 4. New syntax and methods

### 4.1 Tauri's `Channel` — the Rust side

**What it is.** A pipe from Rust to React, made by React for one call. Rust only sends into it; React
only receives. In Rust its type is `tauri::ipc::Channel<T>`, where `T` is what travels. Here `T` is
`String`.

**Form.**

```rust
use tauri::ipc::Channel;

#[tauri::command]
fn some_command(…, on_x: Channel<T>) -> … {
    on_x.send(value)   // gives back Result<(), tauri::Error>
    …
}
```

**Example**, with other names — a command that counts down to React:

```rust
#[tauri::command]
fn count_down(on_tick: Channel<u32>) -> AppResult<()> {
    for n in [3, 2, 1] {
        on_tick
            .send(n)
            .map_err(|e| AppError::WindowUnreachable {
                message: e.to_string(),
            })?;
    }
    Ok(())
}
```

**What to know.**

- **You never make one in the app.** React makes it, and Tauri hands it to the command, the way it
  hands over `State`. (The tests make their own with `Channel::new`, to stand in for React. You do not
  need that.)
- **`send(value)` takes the value itself** — a move, like `self.questions.send(question)`. After
  `on_piece.send(piece)`, `ask` no longer has `piece` (§5.3).
- **`send` gives back a `Result`.** It is `Err` when the value cannot reach React — for example, the
  window has closed. The error is Tauri's (`tauri::Error`), so `.map_err` turns it into an `AppError`.
- **One channel per question.** `ask` takes it, and when `ask` ends, the channel ends too. A late piece
  can never reach the wrong question. That is also why `ask` says `Channel<String>` and not
  `&Channel<String>`: nothing needs the channel after `ask`, so `ask` can take it.
- **What travels is JSON.** `send` turns each value into JSON text, and React turns it back: `"I "`
  travels as `"\"I \""`. So `T` must be a type serde can turn into JSON. `String` is.

**When to use it.** When one command must give React many values, one after another, before it ends.
A command's return value gives React one value, at the end.

**Why here.** With a real model, the pieces come over several seconds. `Ok(reply)` comes only at the
end.

### 4.2 Tauri's `Channel` — the TypeScript side

**What it is.** React's end of the same pipe. You make it, say what to do with each message, and pass
it to `invoke` like any other argument.

**Form.**

```ts
import { Channel, invoke } from "@tauri-apps/api/core";

const onX = new Channel<T>();
onX.onmessage = (message) => { … };
await invoke("some_command", { …, onX });
```

**Example**, for `count_down` above:

```ts
const onTick = new Channel<number>();
onTick.onmessage = (n) => console.log(n);   // runs with 3, then 2, then 1
await invoke("count_down", { onTick });
```

**What to know.**

- **The name changes on the way.** Tauri turns React's camelCase keys into Rust's snake_case
  parameters: `onPiece` → `on_piece`. (`slug`, `suspect` and `question` look the same in both.)
  Without the key, `invoke` fails — measured:
  ``invalid args `onPiece` for command `ask_suspect`: command ask_suspect missing required key onPiece``.
- **`onmessage` runs once per `send`, in the order Rust sent them.**
- **It runs while `invoke` is still waiting.** So set `onmessage` before `invoke`.

**When to use it.** Whenever a command takes a `Channel` parameter: make one per call.

**Why here.** One per question: each `ask()` makes a fresh channel, so pieces of an old reply can never
land in a new one.

## 5. Syntax you already know — from your code

Everything §3 used from your code, in one place.

### 5.1 Send, `.map_err`, `?`

`state.rs :: ask` — sending the question into pipe 1:

```rust
        self.questions
            .send(question)
            .map_err(|e| AppError::Inference {
                message: e.to_string(),
            })?;
```

`send` takes `question`. If that fails, `.map_err` turns the error into an `AppError`, and `?` stops
`ask` with it. Step B has the same shape.

### 5.2 A match arm whose body is a block

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

After `=>`, a `{ }` block holds several lines. No comma is needed after its closing `}`.

### 5.3 After a move, the name is gone: E0382

Stage 14c, part A2. You put `let again = question;` right after the send in `ask`, and got:

```
error[E0382]: use of moved value: `question`
...
67 |             .send(question)
   |                   -------- value moved here
...
71 |         let again = question;
   |                     ^^^^^^^^ value used here after move
```

`send` took the whole `Question`; `ask` did not have it any more. A borrow — `&piece` — only looks
and gives it back. So a borrow can come before a move, never after it.

### 5.4 Adding to the end of a `String`

`state.rs :: ask`, your 16b `Piece` arm:

```rust
                Said::Piece(piece) => reply.push_str(&piece),
```

`push_str` borrows the piece (`&piece`) and copies its text onto the end of `reply`.

### 5.5 A command React calls

`ipc.rs :: ask_suspect`, before this stage:

```rust
#[tauri::command]
pub async fn ask_suspect(state: State<'_, AppState>, question: String) -> AppResult<String> {
    state.ask(&question).await
}
```

And how React called it, Stage 11d step 4:

```js
await window.__TAURI__.core.invoke('ask_suspect', { question: 'Where were you?' })
```

## 6. What you built

For one question in the game:

1. React makes a channel, `onPiece`, says what to do with each piece, and calls `ask_suspect` with the
   question and the channel.
2. Tauri hands the channel to `ask_suspect` as `on_piece`, and `ask_suspect` hands it to `ask`.
3. `ask` sends the question to the engine thread. (No change.)
4. For each piece that comes out of pipe 2, `ask` adds it to `reply`, then sends it into `on_piece`.
   React's `onmessage` adds it to the screen.
5. At `Done`, `ask` keeps the whole reply in the room and gives it back. React's `invoke` resolves.

If React's window is gone, the first `send` fails: `ask` gives back `WindowUnreachable` and keeps no
reply.

New in this stage:

- **Tauri's `Channel`** — Rust side: a command parameter `Channel<T>`, and `send`. TypeScript side:
  `new Channel<T>()`, `onmessage`, and passing it to `invoke`.

Yours: the channel into `ask` (step A), the send (step B), the order of the two lines (step C), and
React's channel (step D).

Next, Stage 17: the player can stop the suspect mid-sentence.
