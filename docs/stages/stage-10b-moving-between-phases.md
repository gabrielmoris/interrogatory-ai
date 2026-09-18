# Stage 10b — moving between phases

Test:    `src-tauri/tests/phase_moves.rs` — 7 tests
Run:     `cd src-tauri && cargo test --test phase_moves`
Writes:  `src/transcript.rs` :: `Phase::name()`, `Phase::begin()`, `Phase::finish()`
New:     replacing the whole value through `&mut self` — `*self = …`
Recall:  the guard clause (Stage 5, shaky) · `Case` has no `Serialize` (Stage 9a)
Est.     25 min

## 0. What this has to do, in ordinary words

A case runs in one direction. From the briefing the player calls a suspect in; from the room they
walk out to write the report. Those are the only two moves. Anything else — calling somebody in
while already questioning somebody, walking out of a room you are not in — is refused, and refusing
leaves everything exactly as it was. Each move does two things: check where the player is now, and
if it is the wrong place hand back an error saying what they tried and where they were. Otherwise,
replace where they are with the new place.

## 1. Recall — answer from memory, then check

<details>
<summary>When the briefing screen asks for a case, what crosses to React — and what stops the whole <code>Case</code> going instead? <em>(Stage 9a)</em></summary>

`CaseIntro` and `SuspectSummary`, built in `ipc.rs` by `impl From<&Case>`. `Case` does not derive
`Serialize`, so it cannot be turned into JSON at all — the guarantee is the missing trait, not care.
</details>

<details>
<summary><em>Shaky, asked until right twice.</em> Your <code>load_case</code> refuses a bad slug before doing anything. What shape is that, and where does the rest of the body go?</summary>

Return early, then carry straight on — no `else`, no extra nesting. You need exactly this shape twice
today:

```rust
    if !is_slug(slug) {
        return Err(AppError::CaseNotFound { slug: slug.to_string() });
    }

    let path = case_path(dir, slug);
```
</details>

<details>
<summary><em>Shaky, asked until right twice.</em> Inside <code>.map_err(|e| …)</code>, do you hand back <code>Err(AppError::…)</code> or <code>AppError::…</code>?</summary>

`AppError::…`. The closure hands back the *new error*; `.map_err` puts it inside `Err` for you.
</details>

**Predict before reading on.** Inside a method whose first parameter is `&mut self`, can you write
`self = Phase::Reporting;` to change the phase? Compiles, or compile error? Commit to an answer.
§2c settles it.

## 2. The worked example — replacing the value you were handed

**a.** The moves are the buttons. Only two arrows exist; the rest of what the UI could ask for has
to be refused:

```
THE BRIEFING              THE INTERROGATION ROOM            THE REPORT
[ Question Marta ]   →    Viktor Lang                  →    Who did it, and how?
[ Question Viktor ]       > Where were you on Tuesday…      [                    ]
                          [ Leave and write my report ]
```

**b.** In TypeScript the state lives in a variable, and a move hands back a new object:

```ts
function load(d: Delivery, van: number): Delivery {
  if (d.kind !== 'atDepot') throw new Error(`cannot load a parcel that is ${name(d)}`);
  return { kind: 'inTransit', van, scans: [] };   // the caller reassigns
}
```

**c.** Three differences.

*The move writes into the caller's value.* Rust hands the method a borrow it is allowed to change,
`&mut self`, and the move replaces what is behind it. No new object handed back, nothing to reassign
— the caller's `Phase` simply *is* the new one afterwards.

*`self` is the borrow; `*self` is the value.* `self = Delivery::Delivered;` does not compile:
`error[E0308]: mismatched types`, with `help: consider dereferencing here to assign to the mutably
borrowed value`. `*self = Delivery::Delivered;` is the line that works. (That was the prediction.)

*Replacing is not merging.* The old variant and everything inside it is gone — which is why the
check has to come first. A refused move never reaches the replacement line, so nothing changes.

One note on the check: `matches!(self, Delivery::AtDepot)` asks *is it this variant* and needs no
names. For a variant that carries data, write `Delivery::InTransit { .. }` — leave the `..` out and
the compiler says `E0533: expected unit struct, unit variant or constant, found struct variant`.

**d.** The depot, in full. `AppError` and `AppResult` are yours, unchanged:

```rust
// depot/src/delivery.rs

impl Delivery {
    /// Where the parcel is, in words, for an error message.
    pub fn name(&self) -> &str {
        match self {
            Delivery::AtDepot => "at the depot",
            Delivery::InTransit { .. } => "on a van",
            Delivery::Delivered => "delivered",
        }
    }

    /// Loads the parcel onto a van. Only a parcel still at the depot can be loaded.
    pub fn load(&mut self, van: u32) -> AppResult<()> {
        if !matches!(self, Delivery::AtDepot) {
            return Err(AppError::InvalidState {
                action: "load this parcel".to_string(),
                state: self.name().to_string(),
            });
        }

        *self = Delivery::InTransit {
            van,
            scans: Vec::new(),
        };
        Ok(())
    }

    /// Hands the parcel over. Only a parcel on a van can be delivered.
    pub fn deliver(&mut self) -> AppResult<()> {
        if !matches!(self, Delivery::InTransit { .. }) {
            return Err(AppError::InvalidState {
                action: "deliver this parcel".to_string(),
                state: self.name().to_string(),
            });
        }

        *self = Delivery::Delivered;
        Ok(())
    }
}
```

## 3. The same problem, in your code

### 3.1 Step 1 — the three doors

One new import line at the top of `transcript.rs`, beside the one already there, and three functions
added to the `impl Phase` block you wrote in 10a. `suspect()` and `turn_count()` stay as they are.

`name()` is the depot's `name()` with your three phases. The words are what the tests check:
**"the briefing"**, **"an interrogation"**, **"the report"**. The other two are stubs for now.

```rust
use crate::error::{AppError, AppResult};
use crate::ids::SuspectId;
```

```rust
    /// Where the player is, in words, for an error message.
    pub fn name(&self) -> &str {
        ____   // three arms, same shape as the depot's name()
    }

    /// Calls a suspect in. Only from the briefing.
    pub fn begin(&mut self, suspect: SuspectId) -> AppResult<()> {
        todo!()
    }

    /// Leaves the room to write the report. Only from an interrogation.
    pub fn finish(&mut self) -> AppResult<()> {
        todo!()
    }
```

Until step 2, `cargo check` warns `unused import: AppError` and `unused variable: suspect`. Expected —
both go away when you fill `begin`.

### 3.2 Step 2 — `transcript.rs :: Phase::begin()`

Refuse it unless the player is at the briefing, saying what they tried and where they were. Then
replace the phase with the interrogation: this suspect, and nothing said yet. Two holes:

```rust
        if !matches!(self, ____) {
            // depot: matches!(self, Delivery::AtDepot)
            return Err(AppError::InvalidState {
                action: "begin an interrogation".to_string(),
                state: self.name().to_string(),
            });
        }

        ____
        // depot: *self = Delivery::InTransit { van, scans: Vec::new() };
        //        YOURS is Interrogating, holding `suspect` and an empty Vec::new()
        Ok(())
```

### 3.3 Step 3 — `transcript.rs :: Phase::finish()`

Same two pieces, no holes — you have just written them. Three differences from step 2: the phase
allowed through is the interrogation, so the check needs the `{ .. }` form; the phase you replace it
with carries nothing; and the action reads `"finish an interrogation"`.

### 3.4 Checkpoints

Measured, running `cargo test --test phase_moves` from `src-tauri/`.

| After | What it says | Next |
|---|---|---|
| now | does not compile — 10 errors, all `E0599 no method named …` | §3.1 |
| step 1 | `1 passed; 6 failed` — `name()`'s test, plus the two warnings above | §3.2 |
| step 2 | `5 passed; 2 failed` — the two left need `finish` | §3.3 |
| step 3 | `7 passed; 0 failed` | §4 |

### 3.5 The interleaved step

`matches!` is Stage 8's — you reached for it unprompted inside `is_slug`. No refresher.

## 4. Rules

1. `transcript.rs` imports only `crate::ids` and `crate::error` — still a domain module.
2. A refused move changes nothing: the check comes before the replacement, always.
3. Do not touch `case.rs`, `ipc.rs`, `state.rs`, or 10a's two functions.
4. Stuck: attempt, run `cargo check --tests`, read the top error.
5. Green on `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` before "ready".

## 5. If you are stuck — step 2

<details>
<summary>1 — where it goes</summary>

`src/transcript.rs :: Phase::begin()`, replacing the `todo!()`. Two pieces: a check that returns
early, then one line that replaces the phase, then `Ok(())`.
</details>

<details>
<summary>2 — the question it has to ask</summary>

*Which single phase is allowed to call somebody in?* And, on the replacement line: *what does the
new phase need inside it — who is in the room, and how many lines have been said so far?*
</details>

<details>
<summary>3 — the shape, names blanked</summary>

```rust
        if !matches!(self, Phase::____) {
            return Err(AppError::InvalidState { /* as printed in §3.2 */ });
        }

        *self = Phase::____ {
            ____,
            turns: ____,
        };
        Ok(())
```
</details>

<details>
<summary>4 — the depot's, in full</summary>

```rust
        if !matches!(self, Delivery::AtDepot) {
            return Err(AppError::InvalidState {
                action: "load this parcel".to_string(),
                state: self.name().to_string(),
            });
        }

        *self = Delivery::InTransit {
            van,
            scans: Vec::new(),
        };
        Ok(())
```
</details>
