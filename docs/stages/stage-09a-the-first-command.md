# Stage 9a — the shape the screen gets

_Cut down 2026-09-14. Step 5 and the `#[tauri::command]` machinery moved out to Stage 9b, on their
own. Steps 1–4 are unchanged, so nothing you have already written is wasted._

Test: `src-tauri/tests/commands.rs` — 6 tests
Run: `cd src-tauri && cargo test --test commands`
Writes: `src/case.rs :: Case::suspects()` · `src/ipc.rs` — two structs and one `From`
Assumes: `Serialize` (5), `From<T>` (2), `.iter().map().collect()` (3), `Clone` (3),
`impl Iterator<Item = &T>` (4), `String` vs `&str` (3)
Est. 25 min

## 0. What this has to do, in ordinary words

The game opens on a briefing screen: the title of the case, the paragraph that sets it up, and the
list of people you can go and talk to. A case holds all of that, and also every fact of the crime,
including who did it. So: build a second, smaller box that holds only the three things the screen
draws, and one function that fills that box from a case. The case itself never leaves the program.
The screen gets the copy.

## 1. Concept introduction

### A second type whose only job is to be handed out _(headline, and the only new idea here)_

**a.** You have one big value in memory holding everything. Something outside wants a small part of
it. Rather than hiding fields on the big one, you declare a new and smaller type that holds only the
parts you are willing to give away, and you fill it by copying those parts across.

**b.** You already do this in TypeScript, every time the database row is not the API response:

```ts
type CaseRow = { title: string; briefing: string; suspects: Suspect[]; facts: Fact[] };
type CaseIntro = { title: string; briefing: string; suspects: { id: number; name: string }[] };

function toIntro(c: CaseRow): CaseIntro {
  return {
    title: c.title,
    briefing: c.briefing,
    suspects: c.suspects.map((s) => ({ id: s.id, name: s.name })),
  };
}
```

**c.** Two things Rust does differently, and nothing else is new in this stage.

_Being sendable is a capability a type has to be given._ `#[derive(serde::Serialize)]` is what gives
it — you put that on `AppError` in Stage 5. `Case`, `Fact` and `Suspect` have never had it and never
will, so there is no way to send one. Not a rule you must remember; the code does not compile.

_And `toIntro` has a name here._ It is the `From` trait — the one you wrote in Stage 2 as
`impl From<u32> for SuspectId`. Write the conversion that way and `CaseIntro::from(&case)` exists
for free.

**Where the analogy breaks.** In TypeScript, returning `CaseRow` by mistake is a code-review
problem. Here it is a compile error, because the other type cannot be turned into text at all.

**d.** The shape, at the parcel depot — one small type, and the function that fills it:

```rust
// depot/src/ipc.rs
use crate::depot::Parcel;

/// What the counter screen is allowed to know about a parcel.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct ParcelSummary {
    pub barcode: String,
    pub town: String,
}

impl From<&Parcel> for ParcelSummary {
    fn from(parcel: &Parcel) -> Self {
        Self {
            barcode: parcel.barcode.clone(),
            town: parcel.destination.town.clone(),
        }
    }
}
```

`.clone()` because the summary owns its own text — it outlives the parcel it was built from. Yours
has one field the depot's does not, a list, and that list is the only part that is more than
straight copying.

## 2. Refresher

- `From<T>` — Stage 2, `impl From<u32> for SuspectId`. Same trait; the source is a borrow this time.
- `.iter().map(..).collect()` — Stage 3's `facts_known_by`, Stage 7's `visible_to`.
- `impl Iterator<Item = &T>` — Stage 4, `Case::suspect_facts`; `todo!()` cannot stub one, so the scaffolding uses `std::iter::empty()`.

## 3. Tasks

### 3.1 What to do, in order

1. **done** — `src/case.rs :: Case::suspects()`, every suspect in the order they were added.
2. **done** — `src/ipc.rs` created, scaffolding in, `pub mod ipc;` added to `src/lib.rs`.
3. `src/ipc.rs` — say what each of the two boxes holds, replacing the two `// step 3` lines. The
   small box holds one person: their number and their name. The big box holds the case's title, its
   opening paragraph, and a list of small boxes.
4. `src/ipc.rs :: CaseIntro::from()` — fill the big box from a case, replacing the `todo!()`. The
   title, the briefing, and then go through the case's suspects making one small box from each.

### 3.2 Scaffolding

In `src/case.rs`, inside the existing `impl Case` block — step 1, already written:

```rust
    /// Every suspect in this case, in the order the case file listed them.
    pub fn suspects(&self) -> impl Iterator<Item = &Suspect> {
        std::iter::empty()
    }
```

`src/ipc.rs` as it stands in your repo right now. Steps 3 and 4 are the two marked spots; the
command at the bottom stays exactly as it is until Stage 9b:

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
        todo!()          // step 4
    }
}

#[tauri::command]
pub fn case_intro(slug: String) -> AppResult<CaseIntro> {
    todo!()              // Stage 9b. Leave it.
}
```

### 3.3 Checkpoints

Measured, running `cargo test --test commands` from `src-tauri/` after each step.

| After | What it says                                                                    | Where that is explained |
| ----- | ------------------------------------------------------------------------------- | ----------------------- |
| 2     | does not compile — 9 errors, `E0560 SuspectSummary has no field named id` first | §3.1 step 3             |
| 3     | `1 passed; 5 failed`                                                            | §3.1 step 4             |
| 4     | `6 passed; 0 failed`                                                            | done                    |

### 3.4 Cold call

Step 1 reaches back to Stage 4: the return type `impl Iterator<Item = &Suspect>` is the one
`Case::suspect_facts` already uses.

## 4. Rules

1. Never put `Serialize` on `Case`, `Fact` or `Suspect` — their not having it _is_ the stage.
2. Leave `case_intro` as `todo!()`. It is all of Stage 9b, and its four unused imports warn until then.
3. Smallest change → `cargo check --tests` → read the top error → fix. Never add derives in bulk.
4. `case.rs` gains no new import.
5. Green on `cargo fmt` and `cargo test` before "ready". `clippy -D warnings` waits for 9b.

## 5. Hints — step 4, filling the big box

<details>
<summary>1 — where it goes</summary>

`src/ipc.rs :: CaseIntro::from()`, replacing the `todo!()`. One `Self { … }` and nothing else — no
`let`, no loop, no `match`.

</details>

<details>
<summary>2 — the question the code has to ask</summary>

For the first two fields: _where on the case is this same thing already sitting?_ For the third:
_how do I turn a run of suspects into a run of small boxes?_ — which is the shape you wrote in
`visible_to`, with a different thing being built inside the closure.

</details>

<details>
<summary>3 — the shape, names blanked</summary>

```rust
        Self {
            title: case.____.clone(),
            briefing: case.____.clone(),
            suspects: case
                .____()
                .map(|____| ____ { … })
                .collect(),
        }
```

</details>

<details>
<summary>4 — the depot's, in full, list and all</summary>

```rust
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

</details>
