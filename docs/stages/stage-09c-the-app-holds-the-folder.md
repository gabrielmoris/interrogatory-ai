# Stage 9c — the app holds the case folder

*Reissued 2026-09-16: the worked example below now contains every construct the task asks of you —
`CLAUDE.md` Rule 2, evidence in `docs/TEACHING-EVIDENCE.md`.*

Test:    `src-tauri/tests/commands.rs` — 12 tests
Run:     `cd src-tauri && cargo test --test commands`
Writes:  `src/state.rs` (new) · `src/ipc.rs` (one function becomes two) · `src/lib.rs` (one line)
New:     a value the app owns — `.manage()` at startup, `State<'_, T>` in the command
Recall:  `PathBuf` vs `&Path` — Stage 8
Est.     25 min

## 0. What this has to do, in ordinary words

`ipc.rs` has the folder name baked into it: `const CASES_DIR: &str = "cases"`. The app can only ever
look in one place — not a different folder on Android than on Windows, not one the player chose, not
a temporary one in a test. Take that decision away from the command and give it to the app: one
value, made once when the app starts, that any command can ask for.

## 1. Recall — answer from memory, then check

<details>
<summary>Your <code>load_case</code> takes <code>dir: &Path</code>. Why the borrowed one there, and not <code>PathBuf</code>? <em>(Stage 8)</em></summary>

Because `load_case` only needs to look at the folder for the length of the call. Taking `PathBuf`
would mean taking ownership — the caller would have to give theirs up or clone one. **`AppState` is
the opposite case: it holds the folder for the whole run, so it owns a `PathBuf`.**
</details>

<details>
<summary>Two things had to be true before React could call <code>case_intro</code>. What were they? <em>(Stage 9b)</em></summary>

`#[tauri::command]` on the function, and its name on the `generate_handler![…]` list. Marked but
unlisted is unreachable, and nothing warns you.
</details>

**Predict before reading on.** This stage has the same shape: one thing at startup, one thing in the
command. If you write the command half and forget the startup half — compile error, or something
else? Commit to an answer. §2c settles it.

## 2. The worked example — a value the app owns

**a.** Two sides, and you need both. When the app starts, you hand one value over and say *hold this
for as long as you run*. Later, any command that wants it says so in its parameter list and gets it
handed in. Nothing in between has to carry it.

**b.** You have written this many times:

```tsx
<CasesDirProvider value={casesDir}>   {/* once, at the top */}
  <App />
</CasesDirProvider>

function BriefingScreen() {
  const casesDir = useCasesDir();      // anywhere below, nothing threaded through
}
```

**c.** Two differences.

*The lookup is by type, not by name.* You never label the value: Tauri files it under `DepotState`,
and a command asking for `State<'_, DepotState>` is what matches. A second thing to hold means a
second type, not a second name.

*Nothing checks that you handed it over.* Forget the `.manage(…)` line and it all still compiles;
the failure arrives when the command runs. Same trap as the handler list in 9b, from the other side.

**d.** The depot, in full. Three files, every line:

```rust
// depot/src/state.rs
use std::path::PathBuf;

/// What the app holds on to for as long as it runs.
pub struct DepotState {
    pub manifest_dir: PathBuf,
}

impl DepotState {
    pub fn new(manifest_dir: PathBuf) -> Self {
        Self { manifest_dir }
    }
}
```

```rust
// depot/src/lib.rs
pub mod ipc;
pub mod state;

use state::DepotState;
use std::path::PathBuf;

pub fn run() {
    tauri::Builder::default()
        .manage(DepotState::new(PathBuf::from("manifests")))
        .invoke_handler(tauri::generate_handler![ipc::parcel_summary])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

```rust
// depot/src/ipc.rs
use crate::state::DepotState;
use std::path::Path;
use tauri::State;

/// The work, with the folder handed in. No Tauri here, so tests can call it.
pub fn parcel_summary_from(manifest_dir: &Path, barcode: &str) -> AppResult<ParcelSummary> {
    let parcel = load_parcel(manifest_dir, barcode)?;
    Ok(ParcelSummary::from(&parcel))
}

#[tauri::command]
pub fn parcel_summary(state: State<'_, DepotState>, barcode: String) -> AppResult<ParcelSummary> {
    parcel_summary_from(&state.manifest_dir, &barcode)
}
```

## 3. The same problem, in your code

### 3.1 Step 1 — `src/state.rs`, new file

Hold the folder the app reads cases from. Same two pieces as `DepotState` above: a struct with one
owned field, and a `new` that puts the argument into it. Then `pub mod state;` in `src/lib.rs`.

```rust
use std::path::PathBuf;

/// What the app holds on to for as long as it runs.
pub struct AppState {
    pub cases_dir: PathBuf,
}

impl AppState {
    pub fn new(cases_dir: PathBuf) -> Self {
        ____          // one line, same as DepotState::new
    }
}
```

### 3.2 Step 2 — `src/ipc.rs`, one function becomes two

Your 9b body is not thrown away. It moves into the new function, and only its first line's arguments
change, because the folder now arrives as a parameter. What stays behind in the command is one new
line: reach into the app's value for the folder, and call the function below it.

What you have now:

```rust
#[tauri::command]
pub fn case_intro(slug: String) -> AppResult<CaseIntro> {
    let case = load_case(Path::new(CASES_DIR), &slug)?;
    Ok(CaseIntro::from(&case))
}
```

What it becomes — two holes, both in the §2d example above:

```rust
pub fn case_intro_from(cases_dir: &Path, slug: &str) -> AppResult<CaseIntro> {
    let case = load_case(____, ____)?;          // your line — its two arguments are now the two you were handed
    Ok(CaseIntro::from(&case))                  // your line — untouched
}

#[tauri::command]
pub fn case_intro(state: State<'_, AppState>, slug: String) -> AppResult<CaseIntro> {
    ____                                        // one line
}
```

`CASES_DIR` then has no users. Delete it, and the `Path::new` import goes with it — `Path` itself
stays, it is in the new signature. Add `use crate::state::AppState;` and `use tauri::State;`.

### 3.3 Step 3 — `src/lib.rs`, hand the value over

One line, and two imports it needs.

**What the line does.** Before the app starts listening, build one `AppState` pointing at the folder
`"cases"` and give it to Tauri to hold. Every command asking for `State<'_, AppState>` then gets it.

**The depot's line, here so you do not have to scroll up:**

```rust
        .manage(DepotState::new(PathBuf::from("manifests")))
```

Yours is the same with two substitutions: `AppState` instead of `DepotState`, and **`"cases"`
instead of `"manifests"`** — the depot's folder does not exist in your app, and nothing will
tell you so until you run it.

**Two imports first.** Neither `AppState` nor `PathBuf` is in scope in `lib.rs` yet. Put them at the
top, beside the `use ipc::case_intro;` you already have:

```rust
use ipc::case_intro;
use state::AppState;
use std::path::PathBuf;
```

`PathBuf::from("some text")` makes an **owned** path out of text — the owned half of the pair you
recalled in §1. `AppState` needs an owned one because it keeps the folder for the whole run.

**Where the line goes**, in your file as it stands:

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        ____                          // <- .manage(…) with YOUR type and "cases"
        .invoke_handler(tauri::generate_handler![case_intro])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Then delete `CASES_DIR`** and its doc comment from `ipc.rs` — nothing uses it now. Keep
`use std::path::Path;`, which `case_intro_from`'s signature still needs. Unlike `AppState`, the
compiler *will* catch this one, because the constant is private: `error: constant CASES_DIR is
never used`.

### 3.4 Checkpoints

Measured, running `cargo test --test commands` from `src-tauri/`.

| After | What it says | Next |
|---|---|---|
| now | does not compile — 2 errors, `E0432 unresolved import interrogatory_ai_lib::state` | step 1 |
| 1 | does not compile — 1 error, `E0432 unresolved import …ipc::case_intro_from` | step 2 |
| 2 | `12 passed; 0 failed` | step 3 |
| 3 | `12 passed; 0 failed` — unchanged, and correct: no test can see `.manage`, only a running app can | §3.6 |

### 3.5 The interleaved step

Step 2's first line reaches back to Stages 5 and 8, not to anything taught here: `?` on a failure
that is already an `AppError`, and `load_case`'s two arguments. No refresher — you wrote both.

### 3.6 Seeing it work, and seeing it fail

Do this **between** step 2 and step 3. `bun tauri dev`, then in the console:

```js
await window.__TAURI__.core.invoke('case_intro', { slug: 'the-ledger' })
```

It fails, because you are asking Tauri for a value nobody gave it. Read the message — it is the one
you will meet again the first time you add a second piece of state and forget the same line. Then do
step 3 and run it again.

## 4. Rules

1. `state.rs` gets no `tauri::` import. Only `ipc.rs` knows Tauri exists.
2. The command body is one line. A second line means the work belongs in `case_intro_from`.
3. Do not touch `storage.rs` or `case.rs`.
4. Smallest change → `cargo check --tests` → read the top error → fix.
5. Green on `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` before "ready".
6. Stuck? Attempt, run, read the error — re-reading this page is familiarity, not understanding.

## 5. If you are stuck — the command's one line

<details>
<summary>1 — where it goes</summary>

`src/ipc.rs :: case_intro()`, the `____` in §3.2. One line. No `let`, no `?`, no `Ok(…)`.
</details>

<details>
<summary>2 — the question it has to ask</summary>

*I was handed the app's value. Which part of it does the function below need, and what else was I
given?* Both answers are already in this function's parameter list.
</details>

<details>
<summary>3 — the shape, names blanked</summary>

```rust
    ____(&state.____, &____)
```
</details>

<details>
<summary>4 — the depot's, in full</summary>

```rust
#[tauri::command]
pub fn parcel_summary(state: State<'_, DepotState>, barcode: String) -> AppResult<ParcelSummary> {
    parcel_summary_from(&state.manifest_dir, &barcode)
}
```
</details>
