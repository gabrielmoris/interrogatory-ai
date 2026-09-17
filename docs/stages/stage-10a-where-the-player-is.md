# Stage 10a — where the player is

Test:    `src-tauri/tests/phase.rs` — 6 tests
Run:     `cd src-tauri && cargo test --test phase`
Writes:  `src/transcript.rs` (new) :: `Phase::suspect()`, `Phase::turn_count()` · `src/lib.rs` (one line)
New:     data that lives inside one variant, and the `match` arm that takes it out
Recall:  `*` — reading a value out of a borrow (Stage 6b)
Est.     25 min

## 0. What this has to do, in ordinary words

A case is played in three parts: the briefing, the interrogation room, the report. Only in the room
is somebody sitting across the table, and only there has anything been said. So the app does not
keep a "who is in the room" box that sits empty two parts out of three — it keeps it inside the room.
To find out who is there: first ask which part the player is in. The room → read the name off it.
Anywhere else → nobody. Counting the lines said works the same way, with zero instead of nobody.

## 1. Recall — answer from memory, then check

<details>
<summary>Your <code>case_intro</code> takes <code>state: State&lt;'_, AppState&gt;</code>. What has to be in <code>lib.rs</code> for that to work, and what happens if it is missing? <em>(Stage 9c)</em></summary>

`.manage(AppState::new(PathBuf::from("cases")))` on the builder. Missing, everything still compiles —
the command fails when React calls it, because nobody gave Tauri the value.
</details>

<details>
<summary>In <code>try_from</code> you wrote <code>SuspectId::new(*raw_known_by)</code>. What does the <code>*</code> do there? <em>(Stage 6b)</em></summary>

`for raw_known_by in &raw_fact.known_by` hands you a borrow of each number, `&u32`. `SuspectId::new`
wants the number itself. `*` reads the value out of the borrow. You need exactly this in step 2.
</details>

<details>
<summary><em>Shaky, asked until right twice.</em> A function must fail early when its input is bad, then carry on. What shape does the body take — and where does the rest of the code go?</summary>

Your own `load_case`, Stage 8. Return early, then carry straight on — no `else`:

```rust
    if !is_slug(slug) {
        return Err(AppError::CaseNotFound { slug: slug.to_string() });
    }

    let path = case_path(dir, slug);
```
</details>

<details>
<summary><em>Shaky, asked until right twice.</em> Inside <code>.map_err(|e| …)</code>, do you write <code>Err(AppError::…)</code> or <code>AppError::…</code>?</summary>

`AppError::…`. The closure hands back the *new error*; `.map_err` puts it inside `Err` for you. Your
`parse_case`: `.map_err(|e| AppError::Parse { path: path.to_string(), message: e.to_string() })?`
</details>

**Predict before reading on.** A parcel on a van has a list of scans. In TypeScript you would read
`d.scans` once you know it is on a van. In Rust, can you write `delivery.scans`? Compiles, or
compile error? Commit to an answer. §2c settles it.

## 2. The worked example — data that lives inside one variant

**a.** What the screens show in each part. Only the middle one has a name at the top and lines under it:

```
BRIEFING                  INTERROGATION                          REPORT
The Ledger                Viktor Lang                            Who did it, and how?
A bookkeeper is dead…     > Where were you on Tuesday night?     [                    ]
                          < At home. Alone.
                          > Who can confirm that?
```

A parcel is the same: at the depot, on a van, or delivered. Only "on a van" has a van and a list of scans.

**b.** You have written this many times:

```ts
type Delivery =
  | { kind: 'atDepot' }
  | { kind: 'inTransit'; van: number; scans: Scan[] }
  | { kind: 'delivered' };

function van(d: Delivery): number | undefined {
  switch (d.kind) {
    case 'inTransit': return d.van;   // narrowed — d.van exists here
    default:          return undefined;
  }
}
```

**c.** Four differences.

*The variant is the tag.* There is no `kind` field. `InTransit` is the name, and the data sits inside it.

*No dot, ever.* `delivery.scans` does not compile, not even after a check:
`` error[E0609]: no field `scans` on type `&Delivery` ``. The fields belong to the variant, not to
`Delivery`. The only door is a `match` arm.

*The arm names what it wants.* `Delivery::InTransit { van, .. }` reads: *if it is in transit, call
its van `van`; ignore the other fields.* That `..` is the "ignore". Leave it out and the compiler
says `` E0027: pattern does not mention field `scans` ``.

*What the arm hands you is a borrow.* The method has `&self`, so `van` is `&u32`, not `u32`.
`*van` reads the value out — your Stage 6b `*`. `scans.len()` needs no `*`: calling a method through
a borrow already works, as it has since Stage 3.

Where the analogy breaks: in TypeScript, `d.van` is a property you are *allowed* to read after the
check. In Rust there is no property. The name `van` exists only inside the arm that made it.

**d.** The depot, in full:

```rust
// depot/src/delivery.rs

/// One beep of the barcode scanner.
#[derive(Debug, Clone, PartialEq)]
pub struct Scan {
    pub place: String,
}

/// Where one parcel is.
#[derive(Debug, Clone, PartialEq)]
pub enum Delivery {
    AtDepot,
    InTransit { van: u32, scans: Vec<Scan> },
    Delivered,
}

impl Delivery {
    /// The van carrying the parcel, or `None` if it is not on one.
    pub fn van(&self) -> Option<u32> {
        match self {
            Delivery::InTransit { van, .. } => Some(*van),
            _ => None,
        }
    }

    /// How many times the parcel was scanned on the road. `0` off the road.
    pub fn scan_count(&self) -> usize {
        match self {
            Delivery::InTransit { scans, .. } => scans.len(),
            _ => 0,
        }
    }
}
```

## 3. The same problem, in your code

### 3.1 Step 1 — `src/transcript.rs`, new file

Type it as printed. Nothing here is new: a plain enum, a struct, and the enum with its data inside
`Interrogating` — the same shape as `Delivery`. The two bodies are `todo!()` so the tests can run.

```rust
use crate::ids::SuspectId;

/// Who said one line in the interrogation room.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Speaker {
    Detective,
    Suspect,
}

/// One line said in the interrogation room.
#[derive(Debug, Clone, PartialEq)]
pub struct Turn {
    pub speaker: Speaker,
    pub text: String,
}

/// Which part of the case the player is in.
#[derive(Debug, Clone, PartialEq)]
pub enum Phase {
    Briefing,
    Interrogating {
        suspect: SuspectId,
        turns: Vec<Turn>,
    },
    Reporting,
}

impl Phase {
    /// The suspect in the room, or `None` outside it.
    pub fn suspect(&self) -> Option<SuspectId> {
        todo!()
    }

    /// How many lines have been said in the room. `0` outside it.
    pub fn turn_count(&self) -> usize {
        todo!()
    }
}
```

Then declare it in `src/lib.rs`, at the end of the list you already have:

```rust
pub mod state;
pub mod storage;
pub mod transcript;
```

### 3.2 Step 2 — `transcript.rs :: Phase::suspect()`

If the player is in the room, hand back who is sitting there. Anywhere else, hand back nobody.
Replace the `todo!()` with this, and fill the two holes:

```rust
        match self {
            Phase::Interrogating { ____, .. } => Some(____),
            // depot: Delivery::InTransit { van, .. } => Some(*van),
            //        YOUR field is `suspect`, and it arrives as a borrow
            _ => None,
        }
```

### 3.3 Step 3 — `transcript.rs :: Phase::turn_count()`

If the player is in the room, count the lines said. Anywhere else, zero. Same again:

```rust
        match self {
            Phase::Interrogating { ____, .. } => ____,
            // depot: Delivery::InTransit { scans, .. } => scans.len(),
            //        YOUR field is `turns`
            _ => 0,
        }
```

### 3.4 Checkpoints

Measured, running `cargo test --test phase` from `src-tauri/`.

| After | What it says | Next |
|---|---|---|
| now | does not compile — 1 error, `E0432 unresolved import interrogatory_ai_lib::transcript` | §3.1 |
| step 1 | `0 passed; 6 failed` — every test reaches a `todo!()` | §3.2 |
| step 2 | `3 passed; 3 failed` — the three "who is in the room" tests pass | §3.3 |
| step 3 | `6 passed; 0 failed` | §4 |

### 3.5 The interleaved step

`Some(*suspect)` — the `*` is Stage 6b's `SuspectId::new(*raw_known_by)`, for the same reason. No refresher.

## 4. Rules

1. `transcript.rs` imports only `crate::ids` — no `tauri::`, no `serde`, no `std::fs`.
2. Do not touch `case.rs`, `ipc.rs` or `state.rs`.
3. Stuck: attempt, run `cargo check --tests`, read the top error. Re-reading this page is not the fix.
4. Green on `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` before "ready".
5. The commit message gets one sentence, in your words, saying what `{ suspect, .. }` in an arm does.

## 5. If you are stuck — step 2's arm

Before opening one, say what you think the answer is.

<details>
<summary>1 — where it goes</summary>

`src/transcript.rs :: Phase::suspect()`, the line above `_ => None`. One line, two holes.
</details>

<details>
<summary>2 — the question it has to ask</summary>

*Is this the interrogation? If so, which field says who is in the room — and am I holding the id
itself, or a borrow of it?*
</details>

<details>
<summary>3 — the shape, names blanked</summary>

```rust
            Phase::____ { ____, .. } => Some(*____),
```
</details>

<details>
<summary>4 — the depot's, in full</summary>

```rust
            Delivery::InTransit { van, .. } => Some(*van),
```
</details>
