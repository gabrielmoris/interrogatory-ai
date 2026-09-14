# Stage 9a — the first thing the screen can ask for

Test:    `src-tauri/tests/commands.rs` — 9 tests
Run:     `cd src-tauri && cargo test --test commands`
Writes:  `src/case.rs :: Case::suspects()` · `src/ipc.rs` (new) · `src/lib.rs` (two lines)
Assumes: `Serialize` (5), `From<T>` (2), `?` (5), `Path` + `load_case` (8),
         `impl Iterator<Item = &T>` (4), `.iter().map().collect()` (3), `String` vs `&str` (3), `Clone` (3)
Est.     45 min

## 0. What this has to do, in ordinary words

The game opens on a briefing screen: the title of the case, the paragraph that sets it up, and the
list of people you can go and talk to. That screen is a web page, so everything it shows it has to
ask for. The job: the page asks for a case by name; Rust reads that case off the disk, copies out
only what the screen draws — the title, the briefing, and each person's name and number — and hands
that back. The facts of the crime stay behind. Not by remembering to leave them out, but because
what gets handed back has nowhere to put them.

## 1. Concept introduction

### The hatch between the page and Rust  *(headline)*

**a.** The page and your Rust code run in different worlds; neither can call the other directly.
Tauri puts a hatch between them. You mark a function as reachable and the page asks for it by name.
Everything going through the hatch is flattened into JSON text on one side and rebuilt on the other,
so only values that know how to become JSON are allowed through, in either direction.

**b.** What you already write when the far side is a server:

```ts
const intro = await fetch(`/api/case/${slug}`).then((r) => r.json());
```

and what the page writes here instead:

```ts
import { invoke } from '@tauri-apps/api/core';
const intro = await invoke('case_intro', { slug: 'the-ledger' });
```

**c.** In Rust the marking is one attribute line above an otherwise ordinary function:

```rust
#[tauri::command]
pub fn case_intro(slug: String) -> AppResult<CaseIntro> { … }
```

That line writes a *second*, hidden function beside yours. The hidden one digs each argument out of
the JSON the page sent — so every argument type must be one serde can read — calls yours, and turns
what comes back into JSON — so the returned type must be one serde can write. `Ok(..)` resolves the
promise on the page and `Err(..)` rejects it, with the error serialized too. That is what
`AppError`'s `Serialize` has been for since Stage 5.

**Where the `fetch` analogy breaks.** With a server you write the encoding and a mistake is a 500 at
run time; here the app will not build if a type cannot cross. And your function is left exactly as
you wrote it, so the test file calls `case_intro(..)` directly, with no app and no window anywhere.

**d.** The shape, at the parcel depot:

```rust
// depot/src/ipc.rs
use std::path::Path;

/// Where the depot keeps its manifests.
const MANIFEST_DIR: &str = "manifests";

#[tauri::command]
pub fn parcel_summary(barcode: String) -> AppResult<ParcelSummary> {
    let parcel = load_parcel(Path::new(MANIFEST_DIR), &barcode)?;
    Ok(ParcelSummary::from(&parcel))
}
```

and the one line in the depot's `lib.rs` that makes it answer to that name:

```rust
.invoke_handler(tauri::generate_handler![parcel_summary])
```

A command written and not listed there is simply unreachable, and nothing warns you.

### What crosses is its own type  *(supporting)*

**a.** The value you hand back is a shape designed for the screen. Not your internal shape with
some fields skipped — a second, smaller type, filled by copying across the parts allowed out.

**b.** In TypeScript this is the response type that is not the database row:

```ts
type SuspectRow = { id: number; name: string; alibi: string; guilty: boolean };
type SuspectDto = { id: number; name: string };
```

Nothing enforces the gap. Forget to strip `guilty` on one route and it ships.

**c.** Here the gap is enforced by absence: `Case`, `Fact` and `Suspect` have never derived
`Serialize`, so a fact has no route to the page at all — not one you must remember not to take.
This is Stage 6a's rule pointed the other way: `RawCase` speaks the *file's* vocabulary coming in,
`CaseIntro` speaks the *screen's* vocabulary going out, and `Case` in the middle speaks neither.

**d.** Two types for the depot's counter screen, and the copy across:

```rust
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct StopSummary {
    pub id: StopId,
    pub town: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ParcelSummary {
    pub barcode: String,
    pub stops: Vec<StopSummary>,
}

impl From<&Parcel> for ParcelSummary {
    fn from(parcel: &Parcel) -> Self {
        Self {
            barcode: parcel.barcode.clone(),
            stops: parcel
                .stops()
                .map(|stop| StopSummary {
                    id: stop.id,
                    town: stop.town.clone(),
                })
                .collect(),
        }
    }
}
```

Owned `String`s, not borrows: what goes through the hatch outlives the call that built it.

## 2. Refresher

- `From<T>` — Stage 2, `impl From<u32> for SuspectId`. Same trait, source type a borrow this time.
- `?` — Stage 5 §4, and every `try_from` check in 6c.
- `load_case(dir, slug)` and `Path` — Stage 8, `storage.rs`.
- `impl Iterator<Item = &T>` — Stage 4, `Case::suspect_facts`; `todo!()` cannot stub one, so the scaffolding uses `std::iter::empty()`.
- `.iter().map(..).collect()` — Stage 3's `facts_known_by`, Stage 7's `visible_to`.

## 3. Tasks

### 3.1 What to do, in order

1. `src/case.rs :: Case::suspects()` — hand back every suspect in this case, in the order they were added. One line: `suspect_facts` with nothing filtered out.
2. `src/lib.rs` — create `src/ipc.rs`, paste in the scaffolding below, and add the one line that
   makes the crate see the file — alphabetical, with the others.
3. `src/ipc.rs` — fill in the fields of the two screen types. For each candidate field ask: does
   the briefing screen draw it? If not, it does not belong.
4. `src/ipc.rs :: CaseIntro::from()` — copy across. The title, the briefing, then walk the case's
   suspects and build one small summary from each.
5. `src/ipc.rs :: case_intro()` — two lines: load the case named by `slug` out of the directory
   `CASES_DIR` names, and hand back the intro built from it. Then add the command to the handler
   list in `src/lib.rs`, leaving `greet` where it is.

### 3.2 Scaffolding

In `src/case.rs`, inside the existing `impl Case` block:

```rust
    /// Every suspect in this case, in the order the case file listed them.
    pub fn suspects(&self) -> impl Iterator<Item = &Suspect> {
        std::iter::empty()
    }
```

The whole of the new `src/ipc.rs`:

```rust
use crate::case::Case;
use crate::error::AppResult;
use crate::ids::SuspectId;
use crate::storage::load_case;
use std::path::Path;

/// Where case files live, until Stage 9b lets the app choose the directory.
const CASES_DIR: &str = "cases";

/// One suspect as the briefing screen needs them.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct SuspectSummary {
    // step 3
}

/// Everything the briefing screen is allowed to know about a case.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct CaseIntro {
    // step 3
}

impl From<&Case> for CaseIntro {
    fn from(case: &Case) -> Self {
        todo!()
    }
}

#[tauri::command]
pub fn case_intro(slug: String) -> AppResult<CaseIntro> {
    todo!()
}
```

`src/lib.rs` as it stands today, and the two lines that change:

```rust
pub mod difficulty;
pub mod error;
pub mod ids;
// step 2 adds the ipc line here
pub mod storage;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])   // step 5 extends this list
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 3.3 Checkpoints

Measured, running `cargo test --test commands` from `src-tauri/` after each step.

| After | What it says | Where that is explained |
|---|---|---|
| 1 | does not compile — `error[E0432]: unresolved import interrogatory_ai_lib::ipc` | §3.1 step 2 |
| 2 | does not compile — 9 errors, `E0560 SuspectSummary has no field named id` first | §3.1 step 3 |
| 3 | `1 passed; 8 failed` | §3.1 step 4 |
| 4 | `6 passed; 3 failed` | §3.1 step 5 |
| 5 | `9 passed; 0 failed` | done |

### 3.4 Cold call

Step 5 reaches back to Stage 8: `load_case` already decides what a missing or unsafe name means, and this stage adds nothing to it.

## 4. Rules

1. Never put `Serialize` on `Case`, `Fact` or `Suspect` — their not having it *is* the stage.
2. `ipc.rs` is a shell module and may use `std::path` and `storage`; `case.rs` gains no new import.
3. Leave `greet` in the handler list; `App.tsx` still calls it, and that is Phase 4's to clean up.
4. Smallest change → `cargo check --tests` → read the top error → fix; four unused-import warnings are expected until step 5. Never add derives in bulk.
5. `CASES_DIR` is a placeholder — Stage 9b hands the app a real directory. `DECISIONS.md`, 2026-09-13.
6. Green on `cargo fmt`, `cargo clippy --all-targets -- -D warnings` and `cargo test` before "ready".

## 5. Hints — step 5, the command body

<details>
<summary>1 — where it goes</summary>

`src/ipc.rs :: case_intro()`, replacing the `todo!()`. Two lines, no branching, no `match`.
</details>

<details>
<summary>2 — the question the code has to ask</summary>

First line: *where does this case live, and did it actually load?* — and if it did not, stop here
and hand the caller the reason. Second line: *wrap what loaded in the type that is allowed out.*
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
