# Stage 9b — a function React can call

Test:    `src-tauri/tests/commands.rs` — 11 tests (your 6, plus 5 new)
Run:     `cd src-tauri && cargo test --test commands`
Writes:  `src/ipc.rs :: case_intro()` — two lines · `src/lib.rs` — one name on a list
New:     `#[tauri::command]`, and the list that makes it reachable
Recall:  `load_case` and `?` — Stages 8 and 5
Est.     20 min

## 0. What this has to do, in ordinary words

You can build a briefing box now, but nothing outside Rust can ask for one. There is no door. This
stage opens the door for exactly one function: the front end says a case's name, and gets a
briefing box back.

## 1. The one new thing — letting React call a Rust function

**a. What it looks like from the side you already know.** This is the line your React code will
write, and it is the whole point of the stage:

```ts
import { invoke } from '@tauri-apps/api/core';

const intro = await invoke('case_intro', { slug: 'the-ledger' });
// -> { title: "The Ledger", briefing: "A bookkeeper is dead…", suspects: [...] }
```

It looks like `fetch`, and it behaves like `fetch`: you get a promise, it resolves with data or it
rejects with an error. There is no server — the other end is a Rust function in the same program.

**b. What makes a Rust function reachable.** Two things, and you need both.

The first is one line above the function. You already have it, from the scaffolding:

```rust
#[tauri::command]
pub fn case_intro(slug: String) -> AppResult<CaseIntro> { … }
```

It writes a second function next to yours — one that pulls `slug` out of the message React sent,
calls your function, and turns what you return back into JSON. Your function is untouched, which is
why the tests can just call it.

The second is putting the name on a list, in `lib.rs`:

```rust
.invoke_handler(tauri::generate_handler![greet, case_intro])
```

That list is what the app actually looks in when a message arrives. **A command that is marked but
not listed is unreachable, and nothing warns you** — no compile error, no runtime warning, just
"command not found" on the React side.

**c. What `Ok` and `Err` mean out there.** `Ok(value)` resolves the promise. `Err(e)` rejects it,
and what React catches is your `AppError` as JSON: `{ kind: "caseNotFound", slug: "…" }`. That is
what the `Serialize` on `AppError` has been for since Stage 5.

**d. The depot's, both halves:**

```rust
// depot/src/ipc.rs
use std::path::Path;

const MANIFEST_DIR: &str = "manifests";

#[tauri::command]
pub fn parcel_summary(barcode: String) -> AppResult<ParcelSummary> {
    let parcel = load_parcel(Path::new(MANIFEST_DIR), &barcode)?;
    Ok(ParcelSummary::from(&parcel))
}
```

```rust
// depot/src/lib.rs
.invoke_handler(tauri::generate_handler![parcel_summary])
```

## 2. Recall

- `load_case(dir, slug)` — Stage 8, your `storage.rs`. Hands back the case, or says why not.
- `?` — Stage 5. The failure goes straight out to your caller; you carry on with the value.
- `load_case` wants a `&Path`, and `CASES_DIR` is text. `Path::new("some text")` is how you turn
  one into the other — it is in the depot code above, in full.

## 3. Tasks

### 3.1 What to do, in order

1. `src/ipc.rs :: case_intro()` — two lines, replacing the `todo!()`. Load the case whose name is
   `slug`, out of the folder `CASES_DIR` names; if that fails, stop and pass the failure out. Then
   hand back a briefing box built from the case you got.
2. `src/lib.rs` — add `case_intro` to the list next to `greet`. It lives in another module, so it
   needs saying where it is, the same way you would call it: `ipc::case_intro`.

Then tidy: `use crate::case::{Case, Suspect};` at the top of `ipc.rs` imports a `Suspect` you never
use. Drop it and the file is clippy-clean.

### 3.2 Scaffolding

`src/lib.rs`, the line that changes — everything else in the file stays as it is:

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])   // <- step 2 adds one name here
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 3.3 Checkpoints

Measured, running `cargo test --test commands` from `src-tauri/`.

| After | What it says | Where that is explained |
|---|---|---|
| now | `6 passed; 5 failed` — the five new ones all hit the `todo!()` | §3.1 step 1 |
| 1 | `11 passed; 0 failed` | §3.1 step 2 |
| 2 | `11 passed; 0 failed` — **unchanged, and that is correct.** The list is invisible to tests; only a running app reads it. §3.4 is how you check it | §3.4 |

### 3.4 Seeing it actually work

This is the first stage whose result you can look at. Two new case files ship in `src-tauri/cases/`,
and `withGlobalTauri` is now on in `tauri.conf.json`, so after step 2:

```
bun tauri dev
```

Right-click the window → Inspect → Console, and paste:

```js
await window.__TAURI__.core.invoke('case_intro', { slug: 'the-ledger' })
```

You should get the briefing object back. Try `'nope'` too, and watch it reject with
`{ kind: "caseNotFound", slug: "nope" }`. If you comment out the name you added in step 2 and
re-run, the same call fails with "command not found" — which is the whole reason the list matters.

## 4. Rules

1. Do not change `storage.rs`. Stage 8 already decided what a missing or unsafe name means.
2. `CASES_DIR` stays a constant this stage. Stage 9c is where the app gets to choose the folder.
3. Smallest change → `cargo check --tests` → read the top error → fix.
4. Green on `cargo fmt`, `cargo clippy --all-targets -- -D warnings` and `cargo test` before "ready".

## 5. Hints — step 1, the two lines

<details>
<summary>1 — where it goes</summary>

`src/ipc.rs :: case_intro()`, replacing the `todo!()`. Two lines. No `if`, no `match`, no loop.
</details>

<details>
<summary>2 — the question each line asks</summary>

Line one: *get me the case with this name, out of that folder — and if there isn't one, stop here
and hand my caller the reason.* Line two: *turn the case I got into a briefing box, and hand it
back as a success.*
</details>

<details>
<summary>3 — the shape, names blanked</summary>

```rust
    let ____ = ____(Path::new(____), &____)?;
    Ok(____::from(&____))
```
</details>

<details>
<summary>4 — the depot's, in full</summary>

```rust
#[tauri::command]
pub fn parcel_summary(barcode: String) -> AppResult<ParcelSummary> {
    let parcel = load_parcel(Path::new(MANIFEST_DIR), &barcode)?;
    Ok(ParcelSummary::from(&parcel))
}
```
</details>
