# Stage 11c — the room keeps the exchange

Test:  src-tauri/tests/room_exchange.rs — 4 tests
Run:   cd src-tauri && cargo test --test room_exchange

## What you build

Asking a question is two lines in the room: the detective's question and the suspect's reply.
`AppState::ask` now keeps both, and refuses when nobody is in the room. A new method,
`AppState::turn_count`, says how many lines were kept. File: `src/state.rs` only.

Nothing new in this stage. Every line is a copy of something you already wrote.

I changed `tests/app_engine.rs`: its tests now call a suspect in before they ask. It passes before
and after this stage.

## Steps

1. Run the test now. It does not compile: 3 errors,
   `E0599 no method named turn_count found for struct AppState`.

2. `src/state.rs` — add `turn_count`. Copy your whole `suspect` method and paste the copy right
   under it, before `ask`. In the copy, change three things:

   | in your copy of `suspect` | change to |
   |---|---|
   | ``/// The suspect in the room, or `None` outside it.`` | ``/// How many lines have been said in the room. `0` outside it.`` |
   | `pub fn suspect(&self) -> AppResult<Option<SuspectId>> {` | `pub fn turn_count(&self) -> AppResult<usize> {` |
   | `Ok(phase.suspect())` | `Ok(phase.turn_count())` |

   The lock lines in the middle stay exactly as they are. It looks like this:

   ```rust
       /// How many lines have been said in the room. `0` outside it.
       pub fn turn_count(&self) -> AppResult<usize> {
           let phase = self.phase.lock().map_err(|e| AppError::Poisoned {
               message: e.to_string(),
           })?;

           Ok(phase.turn_count())
       }
   ```

   Run: `1 passed; 3 failed`. The one that passes is `a_refused_question_keeps_nothing`.

3. `src/state.rs` — the body of `ask`. Delete the one line `self.engine.reply(question)` and write
   four lines in its place, in this order:

   | # | what it does | the line |
   |---|---|---|
   | 1 | keep the detective's question. If nobody is in the room, stop here with that error. | `self.record(Speaker::Detective, question)?;` |
   | 2 | ask the engine. If it fails, stop here with its error. | `let reply = self.engine.reply(question)?;` |
   | 3 | keep the suspect's reply | `self.record(Speaker::Suspect, &reply)?;` |
   | 4 | hand the reply back | `Ok(reply)` |

   Run: `4 passed; 0 failed`.

4. `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (155 tests). Say ready.

## If it does not compile

| the compiler says | fix |
|---|---|
| warning, `unused Result that must be used` (clippy turns it into an error) | A `record` line is missing its `?` at the end. |
| `E0308 mismatched types … expected &str, found String` (in line 3) | Write `&reply`, not `reply`. `record` wants a view of the text, like `question`. |
| two `E0308 … found Result<String, AppError>` | Line 2 is missing its `?`: `self.engine.reply(question)?;` |

## What you learned

- `ask` is built only from methods you already had: `record` twice, `reply` once.
- The `?` after `record` is what makes `ask` refuse in the briefing. `record` says no, and `?` hands
  that "no" straight back to whoever called `ask`, so the engine is never asked.
- `&reply`: `reply` is a `String` that `ask` owns. `record` only needs to look at it, so it gets `&`.
  After that, `ask` still owns `reply` and can hand it back with `Ok(reply)`.
- `turn_count` is `suspect` again with another question to the phase. Same lock, same shape.
