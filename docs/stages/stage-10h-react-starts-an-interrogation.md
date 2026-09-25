# Stage 10h — React starts an interrogation

Test:  src-tauri/tests/start_interrogation.rs — 1 test
Run:   cd src-tauri && cargo test --test start_interrogation

## What you build

React can call in a suspect: it sends a case name and a suspect number, and Rust does the rest.
In `src/ipc.rs` you add the command `begin_interrogation`. In `src/lib.rs` you put it on the list.
Nothing new: this is Stage 9b again, for the function you wrote in 10g.

## Steps

1. Run the test now. It does not compile yet:
   `error[E0432]: unresolved import interrogatory_ai_lib::ipc::begin_interrogation`.

2. `src/ipc.rs` — copy your `case_intro` command (from `#[tauri::command]` to its `}`, 4 lines).
   Paste the copy at the bottom of the file. In the copy, change:

   | this | to this |
   |---|---|
   | `case_intro` (the name) | `begin_interrogation` |
   | `slug: String` | `slug: String, suspect: SuspectId` |
   | `AppResult<CaseIntro>` | `AppResult<()>` |
   | the line inside `{ }` | `begin_interrogation_from(&state, &slug, suspect)` |

   Everything else in the copy stays, including the first parameter `state: State<'_, AppState>`.
   Tauri fills `state` in; React only sends `slug` and `suspect`. After `cargo fmt` it looks like this:

   ```rust
   #[tauri::command]
   pub fn begin_interrogation(
       state: State<'_, AppState>,
       slug: String,
       suspect: SuspectId,
   ) -> AppResult<()> {
       begin_interrogation_from(&state, &slug, suspect)
   }
   ```

   Run: `1 passed; 0 failed`.

3. `src/lib.rs` — change two lines that are already there:

   | line | now | change it to |
   |---|---|---|
   | 2 | `use ipc::case_intro;` | `use ipc::{begin_interrogation, case_intro};` |
   | 20 | `.invoke_handler(tauri::generate_handler![case_intro])` | `.invoke_handler(tauri::generate_handler![case_intro, begin_interrogation])` |

   Run: still `1 passed; 0 failed`. The tests do not read the list. Step 4 does.

4. See it work. `bun tauri dev`. A desktop window titled **interrogatory-ai** opens — not a browser tab.
   Right-click inside that window → Inspect → Console. Paste one line at a time:

   ```js
   await window.__TAURI__.core.invoke('begin_interrogation', { slug: 'the-ledger', suspect: 99 })
   await window.__TAURI__.core.invoke('begin_interrogation', { slug: 'the-ledger', suspect: 2 })
   await window.__TAURI__.core.invoke('begin_interrogation', { slug: 'the-ledger', suspect: 1 })
   ```

   | line | you get |
   |---|---|
   | 99 | an error: `{ kind: "suspectNotFound", id: 99 }` |
   | 2 | `null` — Viktor is in the room |
   | 1 | an error: `{ kind: "invalidState", action: "begin an interrogation", state: "an interrogation" }` |

   If you get `Cannot read properties of undefined (reading 'core')`, you are in a browser tab at
   `localhost:1420`. Only the app window has `window.__TAURI__`.

   Close the app and it forgets: the room is empty again next time.

5. `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` (145 tests). Say ready.

## If it does not compile

| the compiler says | fix |
|---|---|
| `unresolved import interrogatory_ai_lib::ipc::begin_interrogation` | Step 2 is not done, or the name is spelled differently. |
| `expected &AppState, found State<'_, AppState>` | Put `&` before `state`: `&state`. |
| `expected &str, found String` | Put `&` before `slug`: `&slug`. |
| `expected Result<(), AppError>, found ()` | Remove the `;` at the end of the line inside `{ }`. |
| in `lib.rs`: `cannot find function begin_interrogation in this scope` | Add it to the `use ipc::…` line (step 3). |

## What you learned

- A command is a thin door. It takes what React sent and hands it to a plain function that does the work: `begin_interrogation` → `begin_interrogation_from`.
- `&state` lends the `AppState` that sits inside Tauri's `State`. Your function asks for `&AppState`, so that is what it gets.
- A command must be on the `generate_handler!` list. If it is not, React gets "command not found", and no test notices.
- `Ok(())` reaches React as `null`. `Err` reaches it as the `AppError` in JSON.
