# Stage 11d — React asks a question

Test:  src-tauri/tests/ask_command.rs — 1 test
Run:   cd src-tauri && cargo test --test ask_command

## What you build

React can ask the suspect in the room a question and get the reply back.
In `src/ipc.rs` you add the command `ask_suspect`. In `src/lib.rs` you put it on the list.
Nothing new: this is Stage 10h again, for the `ask` you finished in 11c.

## Steps

1. Run the test now. It does not compile yet:
   `error[E0432]: unresolved import interrogatory_ai_lib::ipc::ask_suspect`.

2. `src/ipc.rs` — copy your `case_intro` command (from `#[tauri::command]` to its `}`, 4 lines).
   Paste the copy at the bottom of the file. In the copy, change:

   | this | to this |
   |---|---|
   | `case_intro` (the name) | `ask_suspect` |
   | `slug: String` | `question: String` |
   | `AppResult<CaseIntro>` | `AppResult<String>` |
   | the line inside `{ }` | `state.ask(&question)` |

   `#[tauri::command]` and the first parameter `state: State<'_, AppState>` stay. It looks like this:

   ```rust
   #[tauri::command]
   pub fn ask_suspect(state: State<'_, AppState>, question: String) -> AppResult<String> {
       state.ask(&question)
   }
   ```

   Run: `1 passed; 0 failed`.

3. `src/lib.rs` — change two lines that are already there:

   | line | now | change it to |
   |---|---|---|
   | 2 | `use ipc::{begin_interrogation, case_intro};` | `use ipc::{ask_suspect, begin_interrogation, case_intro};` |
   | 25 | `.invoke_handler(tauri::generate_handler![case_intro, begin_interrogation])` | `.invoke_handler(tauri::generate_handler![case_intro, begin_interrogation, ask_suspect])` |

   The line is now too long, so `cargo fmt` splits it. After `cargo fmt` it looks like this:

   ```rust
           .invoke_handler(tauri::generate_handler![
               case_intro,
               begin_interrogation,
               ask_suspect
           ])
   ```

   Run: still `1 passed; 0 failed`. The tests do not read the list. Step 4 does.

4. See it work. `bun tauri dev`. In the app window (not a browser tab): right-click → Inspect →
   Console. Paste one line at a time:

   ```js
   await window.__TAURI__.core.invoke('ask_suspect', { question: 'Where were you?' })
   await window.__TAURI__.core.invoke('begin_interrogation', { slug: 'the-ledger', suspect: 2 })
   await window.__TAURI__.core.invoke('ask_suspect', { question: 'Where were you?' })
   ```

   | line | you get |
   |---|---|
   | 1 | an error: `{ kind: "invalidState", action: "record a line", state: "the briefing" }` |
   | 2 | `null` — Viktor is in the room |
   | 3 | `"I have nothing to say to you."` |

   Line 3 is the `MockEngine` line from `lib.rs`. A real model replaces it in a later stage.

5. `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (156 tests). Say ready.

## If it does not compile

| the compiler says | fix |
|---|---|
| `unresolved import interrogatory_ai_lib::ipc::ask_suspect` | Step 2 is not done, or the name is spelled differently. |
| `E0308 … expected &str, found String` | Put `&` before `question`: `state.ask(&question)`. |
| `E0308 … expected Result<String, AppError>, found ()` | Remove the `;` at the end of the line inside `{ }`. |
| in `lib.rs`: `cannot find function ask_suspect in this scope` | Add it to the `use ipc::…` line (step 3). |

## What you learned

- A command is a thin door again: `ask_suspect` hands the question to `AppState::ask`.
- No `_from` function this time. `ask` already is the plain function that the tests call.
- `state.ask(…)` works on Tauri's `State` directly, the same way `state.cases_dir` does in `case_intro`.
- `Ok(reply)` reaches React as a plain string. `Err` reaches it as the `AppError` in JSON.
