# Stage 17b — the player presses Stop

Test:  src-tauri/tests/press_stop.rs — 5 tests
Run:   cd src-tauri && cargo test --test press_stop

## 1. What part of the app

Stage 17 is about stopping the suspect mid-sentence. Three parts, one new thing each:

| | what happens | new |
|---|---|---|
| 17a ✅ | when no one listens any more, the engine stops talking | `say` gives back an answer: `FnMut(&str) -> bool` |
| **17b** (this one) | the player presses **Stop**, and `ask` stops | a stop sign that two places hold: `CancellationToken` |
| 17c | **Stop** works even while the suspect is silent | `select!`: wait for two things, take whichever comes first |

**The problem.** Once the player asks, the suspect talks until the reply is over. The player cannot
cut in. With a real model (Stage 19) a reply takes seconds, and an interrogator interrupts.

**Two commands at the same time.** Until now React called one command and waited for it. Now, while
`ask_suspect` is still running — `ask` is waiting for the next piece — the player presses **Stop**, and
React calls a second command: `stop_suspect`. Tauri runs it right away. `ask` is `async`, so while it
waits, it holds nothing up.

`stop_suspect` calls a new method, `AppState::stop`. But `stop` cannot reach into `ask`. They are two
separate calls, and neither calls the other. They need one thing that both of them can see.

**The stop sign.** That thing is a `CancellationToken`. In this project we call it a **stop sign**,
because that is all it does:

- `CancellationToken::new()` makes a new sign. It starts **down**.
- `.clone()` gives a **second hand on the same sign** — not a second sign.
- `.cancel()` puts the sign **up**. Every hand on it sees it up.
- `.is_cancelled()` asks: is the sign up? `true` or `false`.
- Once up, a sign stays up. Nothing puts it down again.

**Who holds what.**

- `AppState` holds one hand, in a new field, `stop_sign`. That is how `stop` reaches it.
- `ask` holds the other hand, in a name of its own, also `stop_sign`.
- A sign stays up, so each question needs a new one. At its start, `ask` calls `new_stop_sign`: it
  puts a new sign in `AppState` and gives `ask` a second hand on it.
- A Stop pressed between two questions puts up the old sign. No one looks at it any more, and the
  next question replaces it.

The field is a `Mutex<CancellationToken>`, as `phase` is a `Mutex<Phase>`: `new_stop_sign` changes what
is inside, and `stop` may be looking at it from the other command at the same moment.

**What `ask` does with its hand.** After each message comes out of pipe 2, and before it uses it,
`ask` looks at the sign. If it is up, `ask` leaves the loop with `break` — as `Said::Done` does. The
reply so far is kept in the room and given back as `Ok`: stopping is not a failure. Then `ask` ends,
its end of pipe 2 is dropped, and the engine stops too. That is 17a's work.

**Why after the message, before using it.** The player presses Stop while `ask` waits for the next
message. That message was said after Stop. It must not reach the screen.

**What this does not fix.** `ask` looks at the sign only when a message comes. If the suspect says
nothing for five seconds, Stop waits five seconds. That is 17c.

### The map

```
  REACT                           RUST — AppState (state.rs)                    THE ENGINE THREAD

  Ask ─── ask_suspect ───▶  ask:
                              new_stop_sign():
                                self.stop_sign  ← a new sign, down        hand 1
                                stop_sign       = .clone()                hand 2, same sign
                              the question → pipe 1 ─────────────────────────▶  the engine says "I "
                              answer.recv()  ◀────────────── pipe 2 ───────────  Piece("I ")
                              stop_sign.is_cancelled() → false
  "I " on screen  ◀───────── on_piece.send("I ")
                              answer.recv() … waits

  Stop ── stop_suspect ──▶  stop():
                                self.stop_sign.lock()                     hand 1
                                .cancel()  → the sign is up
                                                                     ◀────────  Piece("was home.")
                              answer.recv() → Piece("was home.")
                              stop_sign.is_cancelled() → true: break
                              keeps "I " in the room, gives back Ok("I ")
                              ask ends: `answer` is dropped ───✗───  no one reads pipe 2 any more (17a)
```

Every name in the map:

| name | what it is | where it is made, who holds it |
|---|---|---|
| `CancellationToken` | a stop sign. It comes from the crate `tokio-util` (§4.1) | — |
| `self.stop_sign` | `AppState`'s hand on the sign of the question being answered now. A `Mutex<CancellationToken>` | a field of `AppState`. Made in `new` (step A), replaced by `new_stop_sign` |
| `stop_sign`, in `ask` | `ask`'s hand on the same sign. A `CancellationToken` | given back by `new_stop_sign` (step F). `ask` holds it until it ends |
| `new_stop_sign()` | puts a new sign in `self.stop_sign` and gives back a second hand on it | `AppState`, private. `ask` calls it first |
| `stop()` | puts up the sign in `self.stop_sign` | `AppState`. `stop_suspect` calls it |
| `stop_suspect` | the command React calls when the player presses Stop | `ipc.rs` |
| `.cancel()` | puts a sign up (§4.3) | called in `stop` (step C) |
| `.is_cancelled()` | `true` once the sign is up (§4.3) | called in `ask`'s loop (step G) |
| `.clone()` | on a sign: a second hand on the same sign (§4.2) | called in `new_stop_sign` (step E) |
| `break` | leaves `ask`'s loop, as `Said::Done => break` does | `ask`, step G |

`answer`, `said`, `reply`, `on_piece`, pipe 1 and pipe 2 stay as in 16c.

### One question, start to end

This is the test `a_piece_that_comes_after_stop_is_not_shown`. Its engine, `PausesEngine`, says
`"I "`, then waits until the test lets it go on, then says `"was home."`. A second thread plays the
player.

1. `ask` calls `new_stop_sign`. `AppState` gets a new sign, down. `ask` gets a second hand on it.
2. `ask` keeps the question in the room and sends it into pipe 1. (No change.)
3. The engine says `"I "`. `ask` takes it out of pipe 2 and looks at the sign: down. It adds `"I "` to
   `reply` and sends it to React. (No change after the look.)
4. The engine waits. `ask` waits for the next message.
5. The player sees `"I "` and presses Stop: `stop` puts the sign up. Then the test lets the engine go
   on.
6. The engine says `"was home."`. `ask` takes it out of pipe 2 and looks at the sign: up. `break`.
   `"was home."` is not added to `reply` and not sent to React.
7. `ask` keeps `"I "` in the room and gives back `Ok("I ")`.
8. `ask` ends, and `answer` is dropped. No one reads the engine's `Said::Done`. An engine that still
   had pieces to say would hear `false` at the next one and stop. (No change — 17a.)

The next question gets a new sign, down. Without `new_stop_sign` it would get the old one — still up
— and stop at its first message. The test `stop_between_questions_does_not_stop_the_next_reply`
checks that.

## 2. Files and templates

| File | What you do |
|---|---|
| `Cargo.toml` (in `src-tauri`) | paste one line (place 1) |
| `src/state.rs` | paste the import (place 2), the field (place 3), two functions with `todo!()` (place 4) and one doc-comment line (place 5); then steps A–G |
| `src/ipc.rs` | paste `stop_suspect` with `todo!()` (place 6); then step H |
| `src/lib.rs` | add the new command to two lines (place 7). Nothing else |
| `src/App.tsx` — the React file, outside `src-tauri` | paste an empty `stop` function and the Stop button (place 8); then step I |
| `tests/press_stop.rs` | nothing — new, mine |

No other test file changes.

Run now, before you change anything: it does not compile — `4 previous errors`, all in
`tests/press_stop.rs`:

```
error[E0432]: unresolved import `interrogatory_ai_lib::ipc::stop_suspect`
error[E0599]: no method named `stop` found for struct `Arc<AppState>` in the current scope     (3 times)
```

The test calls `stop_suspect` and `app.stop()`. Neither exists yet. (`Arc<AppState>`: the test keeps
the app in an `Arc` so that the test and the player's thread can both use it. Your code does not need
one.)

**Place 1 — `Cargo.toml`.** Under `[dependencies]`, after `toml = "1.1.4"`, add:

```toml
tokio-util = "0.7.19"
```

**Place 2 — `state.rs`.** Under `use tauri::ipc::Channel;`, add:

```rust
use tokio_util::sync::CancellationToken;
```

**Place 3 — `state.rs`, `pub struct AppState`.** After `questions: Sender<Question>,`, add:

```rust
    /// The stop sign of the question being answered now. `stop` puts it up.
    stop_sign: Mutex<CancellationToken>,
```

**Place 4 — `state.rs`, inside `impl AppState`.** After `suspect`, before `ask`'s doc comment, add:

```rust
    /// The player pressed Stop: the reply being said now ends where it is.
    pub fn stop(&self) -> AppResult<()> {
        todo!()
    }

    /// A new stop sign for the question that starts now.
    /// `AppState` keeps one hand on it, for `stop`. `ask` gets the other.
    fn new_stop_sign(&self) -> AppResult<CancellationToken> {
        todo!()
    }
```

**Place 5 — `state.rs`, `ask`'s doc comment.** Under its third line,
`/// through `on_piece` the moment it comes, and puts the pieces together.`, add:

```rust
    /// If the player presses Stop, the reply ends where it is, and that much is kept.
```

**Place 6 — `ipc.rs`.** At the end of the file, after `ask_suspect`, add:

```rust

/// The player pressed Stop: the suspect stops mid-sentence.
#[tauri::command]
pub fn stop_suspect(state: State<'_, AppState>) -> AppResult<()> {
    todo!()
}
```

**Place 7 — `lib.rs`.** Two lines change. The `use ipc` line becomes:

```rust
use ipc::{ask_suspect, begin_interrogation, case_intro, stop_suspect};
```

and the list in `generate_handler!` becomes — note the comma after `ask_suspect`:

```rust
        .invoke_handler(tauri::generate_handler![
            case_intro,
            begin_interrogation,
            ask_suspect,
            stop_suspect
        ])
```

**Place 8 — `src/App.tsx`.** Two parts. Above `  return (`, add the function, empty for now:

```tsx
  // The player presses Stop: the suspect stops mid-sentence.
  async function stop() {
    // step I
  }

```

And under the `</form>` line, add the button, in a row of its own:

```tsx

      <div className="row">
        <button onClick={stop}>Stop</button>
      </div>
```

What each part is for:

- `tokio-util = "0.7.19"` — lets the app use the crate `tokio-util`, where `CancellationToken` lives.
  Your `Cargo.lock` already has this exact version (another crate uses it).
- The `use` line — so `state.rs` can say `CancellationToken` (§4.1). In code, the `-` in a crate's
  name becomes `_`: `tokio_util`.
- The field — `AppState`'s hand on the sign, inside a `Mutex`, as `phase` is.
- `stop` — what `stop_suspect` calls. Steps B and C.
- `new_stop_sign` — no `pub`: only `ask` calls it. It gives back `AppResult<CancellationToken>`, because
  locking can fail with `Poisoned`, as in your other `AppState` methods. Steps D and E.
- The doc-comment line — says what step G makes `ask` do.
- `stop_suspect` — a command, like `begin_interrogation`. Not `async`: `stop` never waits. Step H.
- `lib.rs` — every command React calls must be in that list.
- `App.tsx` — the button calls `stop` when it is clicked. Step I fills `stop`.

Run: `cargo test --test press_stop` → it does not compile — `1 previous error`, in `state.rs`, and one
warning, in `ipc.rs`:

```
error[E0063]: missing field `stop_sign` in initializer of `AppState`
  --> src/state.rs:25:9
   |
25 |         Self {
   |         ^^^^ missing `stop_sign`

warning: unused variable: `state`
  --> src/ipc.rs:90:21
```

The struct has a new field, and `new` does not fill it yet. Step A. The warning stays until step H.

## 3. What each function does, and how to build it

Four kinds of step, from most shown to least:

- **Type it** — the code is shown. You type it. Under it, a table says what each part does.
- **Fill it** — the line is shown with gaps, `____`. Each gap is a name you already have, never
  syntax. Your most similar line is right next to it.
- **Order it** — the lines are shown, out of order. You put them in order. If the order is wrong, the
  step shows what goes wrong.
- **Build it** — easy, and yours. What the line must do, your own code right there, and the change.

Where the inputs come from: `stop` and `new_stop_sign` take only `&self` — the `AppState` that Tauri
holds. `stop_suspect`'s `state` comes from Tauri, as in your other commands. React's `stop` takes
nothing.

### 3.1 `AppState::new` (state.rs)

**What it does.** As before, plus: the app starts with one sign, down. No question is being answered
yet, so no one looks at it.

#### Step A — Type it: the first sign

In `new`, after `questions: start(engine),`, add:

```rust
            stop_sign: Mutex::new(CancellationToken::new()),
```

| part | what it does |
|---|---|
| `stop_sign:` | the field from place 3 |
| `CancellationToken::new()` | a new stop sign, down (§4.1) |
| `Mutex::new(…)` | puts it in a `Mutex` — as `Mutex::new(Phase::Briefing)` does, two lines up |

In the map: `self.stop_sign` before the first question.

Run: `cargo test --test press_stop` → it compiles, with 3 warnings. `1 passed; 4 failed`.

- Passing: `stop_suspect_is_a_command_that_takes_the_state`. It only checks that `stop_suspect` takes
  the state and gives back an `AppResult<()>`. No app starts in a test, so its body never runs.
- Failing: the other four, with `panicked at src/state.rs:61:9: not yet implemented` — the `todo!()`
  in `stop`.
- The warnings: `unused variable: state` (step H), `field stop_sign is never read` (step B),
  `method new_stop_sign is never used` (step F).

### 3.2 `stop` (state.rs)

**What it does.** Reaches `AppState`'s hand on the sign, and puts the sign up. Every hand on that
sign — `ask`'s too — now sees it up.

Two steps: B reaches the sign, C puts it up.

#### Step B — Build it: reach the sign

**What it must do.** Lock `self.stop_sign`, and keep what comes back in a name, `stop_sign`. If the
lock fails, give back `Poisoned`.

**What you have.** The first line of your `suspect`, just above:

```rust
        let phase = self.phase.lock().map_err(|e| AppError::Poisoned {
            message: e.to_string(),
        })?;
```

**The change.** Above `todo!()` in `stop`, add that line, with `stop_sign` in both places where it
says `phase`.

`lock()` gives back a guard. Through it you reach what is inside the `Mutex` — here, `AppState`'s hand
on the sign (§5.1).

Run: → `1 passed; 4 failed`, as before. The panics now point at `src/state.rs:64:9`: the `todo!()`
moved down three lines. `field stop_sign is never read` is gone; `unused variable: stop_sign` came in
its place — step C uses it.

#### Step C — Type it: put the sign up

Replace `todo!()` with:

```rust

        stop_sign.cancel();
        Ok(())
```

| part | what it does |
|---|---|
| `stop_sign.cancel()` | puts the sign up (§4.3). The call goes through the guard to the sign inside the `Mutex`, as `phase.suspect()` does in `suspect` |
| `Ok(())` | `stop` worked. There is nothing to give back |

In the map: `.cancel() → the sign is up`.

Run: → `2 passed; 3 failed`.

- Passing now: `stop_between_questions_does_not_stop_the_next_reply`. `stop` works, and `ask` does
  not look at any sign yet, so nothing stops.
- Failing: the three tests that press Stop in the middle of a reply. For example:

  ```
  ---- stop_keeps_what_the_suspect_said_so_far stdout ----
  assertion `left == right` failed
    left: Ok("I was at home all night.")
   right: Ok("I was ")
  ```

  The player pressed Stop when `"was "` reached the screen, but `ask` went on to the end. `ask` has no
  hand on the sign yet. §3.3 and §3.4.

### 3.3 `new_stop_sign` (state.rs)

**What it does.** At the start of each question, puts a new sign in `self.stop_sign`, and gives back a
second hand on it. A new one each time, because a sign that is up stays up: if the last question was
stopped, its sign is up for good.

Why a function of its own, and not lines inside `ask`: the lock is held only inside this function —
as in `record`. `ask` waits in its loop, and it must not hold a lock while it waits. (Written inside
`ask`, the compiler refuses: see "If it does not compile".)

Two steps: D reaches the sign, E replaces it and hands one back.

#### Step D — Build it: reach the sign, to change it

**What it must do.** The same as step B — but this time, what is inside the `Mutex` will change.

**What you have.** Your step B line. And the first line of `begin`, which changes `phase`:

```rust
        let mut phase = self.phase.lock().map_err(|e| AppError::Poisoned {
            message: e.to_string(),
        })?;
```

**The change.** Above `todo!()` in `new_stop_sign`, add your step B line, with `mut` after `let` — as
in `begin`.

Run: → `2 passed; 3 failed`, as before. Two warnings for the new line, `unused variable: stop_sign`
and `variable does not need to be mutable`: nothing uses it yet. Step E.

#### Step E — Fill it: a new sign, and a second hand on it

Replace `todo!()` with these two lines:

```rust

        *____ = CancellationToken::new();
        Ok(____.clone())
```

Two gaps. Each is a name this function already has.

**Your most similar lines.** In `transcript.rs`, `Phase::finish` replaces what `self` points to:

```rust
        *self = Phase::Reporting;
```

And your `suspect` ends by giving back something reached through its guard:

```rust
        Ok(phase.suspect())
```

| part | what it does |
|---|---|
| `*____ = …` | replaces the sign inside the `Mutex` with a new one. `*` reaches through the guard to what it guards, as `*self` reaches through `&mut self` (§5.2). The old sign is dropped from `AppState` |
| `CancellationToken::new()` | the new sign, down (§4.1) |
| `____.clone()` | a second hand on the **new** sign — not a second sign (§4.2). `AppState` keeps the first hand |
| `Ok(…)` | gives the second hand back to the caller: `ask` |

In the map: `self.stop_sign ← a new sign, down` and `stop_sign = .clone()`.

The order matters: replace first, then `.clone()`. The other way round, `ask` would get a hand on the
old sign — which may be up.

Run: → `2 passed; 3 failed`, as before, and the warning `method new_stop_sign is never used`. Step F
calls it.

### 3.4 `ask` (state.rs)

- **Now (16c):** goes on until `Said::Done` or an error.
- **After 17b:** first gets a hand on a new sign; looks at it after each message; stops when it is up.

Two steps: F gets the hand, G looks at the sign.

#### Step F — Build it: `ask` gets its hand on the sign

**What it must do.** As the very first line of `ask`: call `new_stop_sign`, and keep what it gives
back in a name, `stop_sign`. If it fails, `ask` stops there and gives back the error.

**What you have.** In `ipc.rs`, `case_intro_from` keeps what a call gives back, with `?`:

```rust
    let case = load_case(cases_dir, slug)?;
```

And `ask`'s first line calls a method on `self`:

```rust
        self.record(Speaker::Detective, text)?;
```

**The change.** Above that first line, add a line shaped like the `case_intro_from` one: `stop_sign`
instead of `case`, and `self.new_stop_sign()` instead of `load_case(cases_dir, slug)`. It takes no
inputs.

Why first: the sooner the new sign is in place, the sooner Stop can reach it.

Run: → `2 passed; 3 failed`, as before. `method new_stop_sign is never used` is gone; one new warning,
`unused variable: stop_sign`, in `ask`. Step G.

#### Step G — Order it: look at the sign

These are the parts of `ask`'s `loop` body, out of order:

- your `match said { … }` block, from 16b and 16c;
- these new lines:

  ```rust
              if stop_sign.is_cancelled() {
                  break;
              }
  ```

- your `let said = answer.recv().await.ok_or(…)?;` line, from 16b.

Put the new lines in the right place. The rule, from §1: `ask` looks at the sign **after** a message
comes out of pipe 2, and **before** it uses the message.

| part | what it does |
|---|---|
| `stop_sign.is_cancelled()` | is the sign up? `true` or `false` (§4.3) |
| `if … { break; }` | if it is up, leave the loop, as `Said::Done => break` does. The lines after the loop run: the reply so far is kept, and `Ok(reply)` is given back |

With the wrong order — the new lines first in the loop, or last — it compiles, and one test fails:

```
---- a_piece_that_comes_after_stop_is_not_shown stdout ----
assertion `left == right` failed
  left: Ok("I was home.")
 right: Ok("I ")
```

`ask` looked at the sign, then waited. While it waited, the player pressed Stop, and `"was home."`
came. `ask` used it without looking again: the player saw a piece said after Stop.

Run, right order: → `5 passed; 0 failed`. The warning in `state.rs` is gone; the one in `ipc.rs` stays
until step H.

### 3.5 `stop_suspect` (ipc.rs)

**What it does.** The door React knocks on when the player presses Stop. It hands everything to
`stop`.

#### Step H — Build it: hand it to `stop`

**What it must do.** Call `stop` on `state`, and give back what `stop` gives back.

**What you have.** The body of `ask_suspect`, just above:

```rust
    state.ask(&question, on_piece).await
```

**The change.** Replace `todo!()` with that line, calling `stop` instead of `ask`: no inputs, and no
`.await` — `stop` is not `async`. No `;` at the end: the line is what `stop_suspect` gives back.

Run: → `5 passed; 0 failed`, and no warnings. No test runs this body: a test cannot start the app.
Step J shows it working, and the review checks it.

Then the whole suite: `cargo test` → 189 tests (you had 184; `press_stop.rs` adds 5).

### 3.6 `stop()` in App.tsx (React)

#### Step I — Build it: React calls `stop_suspect`

**What it must do.** Clear the error line, call the command `stop_suspect`, and if that fails, show
the error.

**What you have.** `callViktorIn`, at the top of `App`:

```tsx
  async function callViktorIn() {
    setProblem("");
    try {
      await invoke("begin_interrogation", { slug: "the-ledger", suspect: 2 });
    } catch (error) {
      setProblem(JSON.stringify(error));
    }
  }
```

**The change.** Replace the `// step I` comment with `callViktorIn`'s body, changed in one place: the
`invoke` line calls `"stop_suspect"`, with no second argument — `stop_suspect` takes nothing from
React.

Check, from the project root: `bunx tsc --noEmit` → prints nothing.

#### Step J — Type it, then take it out: see Stop in the app

`MockEngine` says its whole line in a moment — too fast to press Stop. To see Stop work, slow the mock
down, look, then put it back.

1. In `llm.rs`, `MockEngine::reply`, add this as the first line inside the `for` loop:

   ```rust
               std::thread::sleep(std::time::Duration::from_millis(300));
   ```

   | part | what it does |
   |---|---|
   | `std::thread::sleep(…)` | the thread that runs this line waits, doing nothing — here, the engine thread |
   | `std::time::Duration::from_millis(300)` | how long: 300 milliseconds |

   `I have nothing to say to you.` is seven words, so the reply now takes about two seconds.

2. From the project root: `bun tauri dev`.
3. In the window: **Call Viktor in**, then **Ask**, then **Stop** while the words are coming.
   The words stop where you pressed. Measured, with Stop pressed 0.8 seconds after Ask:

   | time | what happened |
   |---|---|
   | 0.3 s | `I ` on screen |
   | 0.6 s | `have ` on screen |
   | 0.8 s | Stop pressed; `stop_suspect` returns at once, while `ask_suspect` is still running |
   | 0.9 s | `nothing ` comes out of pipe 2; the sign is up, so `ask` stops. `ask_suspect` returns `I have ` |
   | later | nothing more: the screen stays `I have ` |

   Note the 0.1 seconds between Stop and the end: `ask` saw the sign only when the next piece came.
   That is what 17c fixes.

4. Take the `sleep` line out of `llm.rs`. If you forget, every test still passes, but a few test files
   take seconds instead of a fraction of one.

Last: `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (189). Say ready.

### If it does not compile

`state.rs`:

| the compiler says | what it means |
|---|---|
| after §2: `E0063 missing field stop_sign in initializer of AppState` | Expected. Step A. |
| clippy: `variable does not need to be mutable`, at `stop`'s lock line | `stop` needs no `mut`: `cancel()` changes nothing inside the `Mutex` — the sign stays the same sign, it only goes up (§4.3). Take `mut` out. |
| `E0596 cannot borrow stop_sign as mutable, as it is not declared as mutable`, at `*stop_sign = …` | Step D: `mut` is missing after `let`. Replacing what is inside needs it. |
| `E0308 mismatched types … expected MutexGuard<'_, CancellationToken>, found CancellationToken`, with `help: consider dereferencing here` | Step E: the `*` is missing. Without it, the line tries to replace the guard itself, not the sign inside. |
| `E0308 mismatched types … expected CancellationToken, found MutexGuard<'_, CancellationToken>`, at `Ok(stop_sign)` | Step E: `.clone()` is missing. `ask` needs its own hand on the sign, not the guard. |
| `E0507 cannot move out of dereference of MutexGuard<'_, CancellationToken>`, at `Ok(*stop_sign)` | Step E: write `stop_sign.clone()`, no `*`. The sign cannot leave the `Mutex` — `AppState` keeps its hand. Do not follow the long `help` line. |
| `E0308 … expected Result<(), AppError>, found ()`, with `help: remove this semicolon`, under `Ok(());` | Step C: take the `;` off `Ok(())`. |
| `E0599 no method named is_cancelled found for enum Result<T, E>` | Step F: the `?` is missing. `new_stop_sign` gives back an `AppResult`; `?` takes the sign out of it. |
| `E0267 break inside async function` | Step G: the `if` is above `loop {`, outside the loop. It goes inside. |
| `a_piece_that_comes_after_stop_is_not_shown` fails with `left: Ok("I was home.")` | Step G: wrong order. |
| two tests fail with `left: Ok(1)`, `right: Ok(2)` | Step G: you wrote `return Ok(reply);` instead of `break;`. That skips the lines after the loop, so the reply is not kept in the room. |
| two tests fail: `stop_between_questions_…` with `left: Ok("")`, and `after_stop_…` with `left: Ok(2)` or `Ok(3)`, `right: Ok(1000)` | Step E: the `*stop_sign = CancellationToken::new();` line is missing. Every question gets the same sign. Once it is up, every reply stops at its first message. |
| `future cannot be sent between threads safely … future returned by ask_suspect is not Send`, with `has type std::sync::MutexGuard<'_, CancellationToken> which is not Send` | You locked `self.stop_sign` inside `ask`. Its guard lives until `ask` ends — across the `.await` in the loop — and Tauri needs `ask_suspect` to be able to move between threads while it waits. Put the lock in `new_stop_sign` (§3.3) and call that (step F). |

`ipc.rs`:

| the compiler says | what it means |
|---|---|
| `E0308 mismatched types … expected Result<(), AppError>, found ()`, with `help: remove this semicolon` | Step H: take the `;` off. With it, `stop_suspect` gives back nothing. |
| `E0308 ? operator has incompatible types` | Step H: no `?`. `stop()` already gives back the `AppResult<()>` that `stop_suspect` gives back. |

The app:

| you see | what it means |
|---|---|
| clicking Stop shows `"Command stop_suspect not found"`, and the reply goes on to the end | Place 7: `stop_suspect` is missing from the `generate_handler!` list. (The build also warns `unused import: stop_suspect`.) |

## 4. New syntax and methods

### 4.1 `CancellationToken` — a stop sign

**What it is.** A type from the crate `tokio-util`, made by the people who make tokio — the async
runtime Tauri runs on. It is a sign that several places can hold. One of them puts it up; all of them
see it up.

**Form.**

```rust
use tokio_util::sync::CancellationToken;

let sign = CancellationToken::new();     // a new sign, down
```

The `use` path reads: the crate `tokio_util` (in `Cargo.toml` it is `tokio-util`), its part `sync`,
the type `CancellationToken`.

**Once up, it stays up.** No method puts it down. For a fresh start, make a new one.

**When to use it.** When one part of the program must tell another to stop, and neither calls the
other.

**Why here.** `stop_suspect` and `ask_suspect` are two commands. Neither calls the other. The sign is
the one thing they share. And because it stays up, each question gets its own: `new_stop_sign`.

### 4.2 `.clone()` on a sign — a second hand, not a second sign

**What it is.** Usually `clone()` makes a copy. In `ipc.rs`, `CaseIntro::from` has
`title: case.title.clone()`: a second `String` with the same text. Change one, and the other stays as
it was.

A `CancellationToken`'s `clone()` is different. It gives a second hand on the **same** sign. Put the
sign up with either hand, and both see it up.

**Example** (other names):

```rust
let sign = CancellationToken::new();
let other_hand = sign.clone();
other_hand.cancel();
sign.is_cancelled()          // true: there is one sign, and it is up
```

**What to know.** Each hand can be kept, moved or dropped on its own. The sign itself lasts as long
as one hand holds it. When `ask` ends, its hand is dropped; `AppState` still holds the other.

**When to use it.** Whenever a second place must hold the same sign.

**Why here.** `AppState` keeps one hand, so that `stop` can put the sign up. `ask` gets the other, so
that it can look.

### 4.3 `.cancel()` and `.is_cancelled()`

**What they are.** `.cancel()` puts the sign up. `.is_cancelled()` gives back `true` if the sign is
up, and `false` if not.

**Form.**

```rust
sign.cancel();              // up. Again: nothing changes — it is already up
sign.is_cancelled()         // a bool
```

Neither needs `mut`: the sign is shared by every hand, and putting it up does not change which sign a
hand holds. That is why `stop`'s lock line has no `mut`, and `new_stop_sign`'s — which replaces the
sign — does.

**When to use them.** `cancel` where the stop comes from; `is_cancelled` where the work checks whether
to go on — as often as it can.

**Why here.** `stop` calls `cancel` when the player presses Stop. `ask` calls `is_cancelled` each time
a message comes out of pipe 2.

## 5. Syntax you already know — from your code

Everything §3 used from your code, in one place.

### 5.1 Locking a `Mutex`, with and without `mut`

`state.rs :: suspect` only reads what is inside:

```rust
        let phase = self.phase.lock().map_err(|e| AppError::Poisoned {
            message: e.to_string(),
        })?;

        Ok(phase.suspect())
```

`state.rs :: begin` changes it, so its guard is `mut`:

```rust
        let mut phase = self.phase.lock().map_err(|e| AppError::Poisoned {
            message: e.to_string(),
        })?;
        phase.begin(suspect)
```

`lock()` gives back a guard. Through it you use what is inside the `Mutex`: `phase.suspect()`. The
lock is let go when the guard is dropped — at the end of the function. That is why each lock lives in
a small function of its own, and `ask` calls them.

### 5.2 `*` replaces what something points to

`transcript.rs :: Phase::finish`:

```rust
        *self = Phase::Reporting;
```

`self` is a `&mut Phase`. `*self = …` replaces the `Phase` it points to. A `mut` guard works the
same way: `*guard = …` replaces what is inside the `Mutex`.

### 5.3 Keeping what a call gives back, with `?`

`ipc.rs :: case_intro_from`:

```rust
    let case = load_case(cases_dir, slug)?;
```

If `load_case` gives back `Err`, the function stops here and gives it back. If `Ok`, `case` is what
was inside.

### 5.4 `break`

`state.rs :: ask`, your 16b loop:

```rust
                Said::Done => break,
```

`llm.rs :: MockEngine::reply`, your 17a stop:

```rust
            if !say(word) {
                break;
            }
```

`break` leaves the loop at once. The line after the loop runs next.

### 5.5 A command that hands everything to `AppState`

`ipc.rs`:

```rust
#[tauri::command]
pub async fn ask_suspect(
    state: State<'_, AppState>,
    question: String,
    on_piece: Channel<String>,
) -> AppResult<String> {
    state.ask(&question, on_piece).await
}
```

`begin_interrogation` is not `async`, because nothing in it waits.

### 5.6 A field in a `Mutex`

`state.rs :: AppState` and `new`:

```rust
    phase: Mutex<Phase>,
```

```rust
            phase: Mutex::new(Phase::Briefing),
```

### 5.7 React calls a command

`App.tsx :: callViktorIn`:

```tsx
    try {
      await invoke("begin_interrogation", { slug: "the-ledger", suspect: 2 });
    } catch (error) {
      setProblem(JSON.stringify(error));
    }
```

A command's `Err` arrives in React as the `error` of the `catch`.

### 5.8 A crate in `Cargo.toml`

`Cargo.toml`, under `[dependencies]`:

```toml
thiserror = "2.0.20"
toml = "1.1.4"
```

One line per crate: its name, and the version.

## 6. What you built

When the player presses Stop:

1. At the start of the question, `ask` called `new_stop_sign`: `AppState` holds a new sign, down, and
   `ask` holds a second hand on it.
2. The player clicks **Stop**. React calls `stop_suspect`, while `ask_suspect` is still running.
3. `stop_suspect` calls `stop`. `stop` locks `self.stop_sign` and puts the sign up.
4. The next message comes out of pipe 2. `ask` looks at its hand on the sign: up. `break`. The message
   is not used.
5. `ask` keeps the reply so far in the room and gives it back as `Ok`. React's screen shows exactly
   that much.
6. `ask` ends; its end of pipe 2 is dropped; the engine's next `say` gives back `false`, and the engine
   stops (17a).
7. The next question gets a new sign, down.

New in this stage:

- **`CancellationToken`** — a stop sign. `new()` makes one, down; `cancel()` puts it up;
  `is_cancelled()` asks; and once up, it stays up.
- **`.clone()` on a sign** — a second hand on the same sign, not a copy.
- The idea under both: **two calls that never call each other can still talk, through one thing they
  both hold.**

Yours: `stop`'s lock (B), `new_stop_sign`'s lock and its two lines (D, E), `ask`'s first line (F), the
place of the check (G), `stop_suspect` (H), and React's `stop` (I).

Next, Stage 17c. `ask` looks at the sign only when a message comes. A real model can think for
seconds before its first word, and the player who presses Stop then waits as long. 17c makes `ask`
wait for two things at once — the next message, or the sign going up — and react to whichever comes
first: `select!`.
