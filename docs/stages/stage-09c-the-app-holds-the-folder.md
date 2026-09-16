# Stage 9c — the app holds the case folder

Test:    `src-tauri/tests/commands.rs` — 12 tests
Run:     `cd src-tauri && cargo test --test commands`
Writes:  `src/state.rs` (new) · `src/ipc.rs` (one function becomes two) · `src/lib.rs` (one line)
New:     a value the app owns — `.manage()` at the top, `State<'_, T>` at the bottom
Recall:  `PathBuf` — Stage 8 · writing a plain struct — Stage 3
Est.     25 min

## 0. What this has to do, in ordinary words

`ipc.rs` currently has the folder name baked into it: `const CASES_DIR: &str = "cases"`. That means
the app can only ever look in one place — not a different folder on Android than on Windows, not one
the player chose, not a temporary one in a test. This stage takes that decision away from the
command and gives it to the app: one value, made once when the app starts, that any command can ask
for.

## 1. The one new thing — a value the app owns

**a.** Two sides, and you need both. When the app starts, you hand one value over and say *hold this
for as long as you run*. Later, any command that wants it says so in its parameter list, and it gets
handed in. Nothing in between has to carry it.

**b.** You have done this many times:

```tsx
<CasesDirProvider value={casesDir}>   {/* once, at the top */}
  <App />
</CasesDirProvider>

function BriefingScreen() {
  const casesDir = useCasesDir();      // anywhere below, no threading
}
```

**c.** Two differences worth knowing.

*The lookup is by type, not by name.* You do not label the value; Tauri files it under `AppState`
and a command asking for `State<'_, AppState>` is what matches it. One value per type, so if you
later want a second thing held, it is a second type, not a second name.

*Nothing checks that you handed it over.* Forget the `.manage(…)` line and everything still
compiles — the failure shows up when the command actually runs. Same trap as the handler list last
stage, from a different angle.

**d.** The depot's, both halves:

```rust
// depot/src/state.rs
use std::path::PathBuf;

pub struct DepotState {
    pub manifest_dir: PathBuf,
}
```

```rust
// depot/src/lib.rs — hand it over once, at startup
tauri::Builder::default()
    .manage(DepotState::new(PathBuf::from("manifests")))
    .invoke_handler(tauri::generate_handler![ipc::parcel_summary])
```

```rust
// depot/src/ipc.rs — ask for it by putting it in the parameter list
use tauri::State;

#[tauri::command]
pub fn parcel_summary(state: State<'_, DepotState>, barcode: String) -> AppResult<ParcelSummary> {
    parcel_summary_from(&state.manifest_dir, &barcode)
}
```

## 2. Recall

- `PathBuf` — Stage 8. The owned half of the pair; `&Path` is the borrowed half a function takes.
- A plain struct with a `new` — Stage 3, `Suspect::new`. `AppState` is nothing more than that.
- Shell modules may touch the outside world, domain modules may not — Stage 8. `state.rs` is a shell
  module, but notice it imports no `tauri` at all: only `ipc.rs` does.

## 3. Tasks

### 3.1 What to do, in order

1. `src/state.rs` — a new file holding one struct with one field: the folder the app should read
   cases from. Give it a `new`. Then one line in `src/lib.rs` so the crate sees the file.
2. `src/ipc.rs` — `case_intro` becomes **two** functions. A plain one, `case_intro_from`, that takes
   the folder and the slug and does exactly what the body does today. And the command, which now
   asks for the app's value and passes its folder straight through — one line. `CASES_DIR` and the
   `Path` import go with it.
3. `src/lib.rs` — hand the app a value at startup, next to where the handler list is.

**Why step 2 splits it in two.** `State<'_, AppState>` only exists inside a running app, so a
command that takes one cannot be called from a test at all. Keeping the work in an ordinary function
keeps it testable, and leaves the command doing what a command should: take what arrived, pass it
on. That is the roadmap's rule for this module — *deserialize, delegate, map errors, nothing else.*

### 3.2 Scaffolding

The whole of the new `src/state.rs`:

```rust
use std::path::PathBuf;

/// What the app holds on to for as long as it runs.
pub struct AppState {
    // step 1
}

impl AppState {
    pub fn new(cases_dir: PathBuf) -> Self {
        todo!()
    }
}
```

`src/ipc.rs` — the two signatures. Bodies are yours:

```rust
/// The work, with the folder handed in. No Tauri here, so tests can call it.
pub fn case_intro_from(cases_dir: &Path, slug: &str) -> AppResult<CaseIntro> {
    todo!()
}

#[tauri::command]
pub fn case_intro(state: State<'_, AppState>, slug: String) -> AppResult<CaseIntro> {
    todo!()
}
```

`src/lib.rs` — the line that changes:

```rust
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // step 3 goes here
        .invoke_handler(tauri::generate_handler![case_intro])
```

### 3.3 Checkpoints

Measured, running `cargo test --test commands` from `src-tauri/`.

| After | What it says | Where that is explained |
|---|---|---|
| now | does not compile — 2 errors, `E0432 unresolved import interrogatory_ai_lib::state` | §3.1 step 1 |
| 1 | does not compile — 1 error, `E0432 unresolved import …ipc::case_intro_from` | §3.1 step 2 |
| 2 | `12 passed; 0 failed` | §3.1 step 3 |
| 3 | `12 passed; 0 failed` — **unchanged, and correct.** No test can see `.manage`; only a running app can. §3.4 is how you check it | §3.4 |

### 3.4 Seeing it actually work

Do this **between** step 2 and step 3, then again after. `bun tauri dev`, console:

```js
await window.__TAURI__.core.invoke('case_intro', { slug: 'the-ledger' })
```

Before step 3 it fails, because you are asking Tauri for a value nobody gave it. Read what it says —
that message is the one you will see again the first time you add a second piece of state and forget
the same line. After step 3 it returns the briefing, exactly as before.

## 4. Rules

1. `state.rs` gets no `tauri::` import. `AppState` is a plain struct; only `ipc.rs` knows about Tauri.
2. The command body is one line. If it grows a second, the work belongs in `case_intro_from`.
3. Do not touch `storage.rs` or `case.rs`.
4. Smallest change → `cargo check --tests` → read the top error → fix.
5. Green on `cargo fmt`, `cargo clippy --all-targets -- -D warnings` and `cargo test` before "ready".

## 5. Hints — step 2, the split

<details>
<summary>1 — where it goes</summary>

`src/ipc.rs`. Two `todo!()`s to replace. The first one's body is the two lines that are already in
`case_intro` today, with `Path::new(CASES_DIR)` swapped for the folder that was handed in.
</details>

<details>
<summary>2 — the question each one asks</summary>

`case_intro_from`: *given a folder and a name, get the case and turn it into a briefing box.* Its
body already exists — you wrote it last stage.

`case_intro`: *I have been handed the app's value; reach into it for the folder and pass that along
with the slug.* Nothing else. No loading, no converting.
</details>

<details>
<summary>3 — the shape, names blanked</summary>

```rust
pub fn case_intro_from(cases_dir: &Path, slug: &str) -> AppResult<CaseIntro> {
    let ____ = load_case(____, ____)?;
    Ok(____::from(&____))
}

#[tauri::command]
pub fn case_intro(state: State<'_, AppState>, slug: String) -> AppResult<CaseIntro> {
    ____(&state.____, &____)
}
```
</details>

<details>
<summary>4 — the depot's, in full</summary>

```rust
pub fn parcel_summary_from(manifest_dir: &Path, barcode: &str) -> AppResult<ParcelSummary> {
    let parcel = load_parcel(manifest_dir, barcode)?;
    Ok(ParcelSummary::from(&parcel))
}

#[tauri::command]
pub fn parcel_summary(state: State<'_, DepotState>, barcode: String) -> AppResult<ParcelSummary> {
    parcel_summary_from(&state.manifest_dir, &barcode)
}
```
</details>
