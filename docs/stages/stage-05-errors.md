# Stage 5 — errors

**Test:** `src-tauri/tests/errors.rs` — 14 tests
**Run:** `cd src-tauri && cargo test --test errors`
**You write:** a new file `src-tauri/src/error.rs`, plus three methods in `src-tauri/src/case.rs`
**Est.** 90–120 min

---

## What you are building, and why

Right now, if you ask a case for a suspect that is not in it, you get `None`:

```rust
case.suspect(SuspectId::new(99))    // -> None
```

`None` means "nothing here". It does not say *why*, and it does not say *which id* was missing.

That was fine for Stage 4. It stops being fine soon. In a couple of stages, a button in your React
app will call into Rust, and when something goes wrong the app has to show the player an actual
message. You cannot build a message out of `None`.

So this stage builds something that can. Three pieces:

1. A new file, `src/error.rs`, holding a type called `AppError` — a list of everything that can go
   wrong in this app, each item carrying the details.
2. Two methods on `Case` that hand back an `AppError` instead of `None`.
3. One method, `reveal`, that has to check two things before it is allowed to do its work.

Five short sections below. Read one, write that piece, run the test, come back.

---

## Before you start

**One command.** From `src-tauri/`:

```
cargo add thiserror
```

`thiserror` is a small helper crate. Its job is explained in section 2 — for now, just install it.
It adds one line to `Cargo.toml` and downloads nothing, because Tauri already uses it.

**Declare the new module.** Rust does not see a file until you say so. In `src/lib.rs`:

```rust
pub mod case;
pub mod difficulty;
pub mod error;
pub mod ids;
```

**Order matters here.** The three new methods mention two types — `AppError` and `AppResult` — that
do not exist yet. So write `error.rs` first (sections 2 and 3 below), then come back and stub the
methods. If you stub first you get `E0425: cannot find type AppResult in this scope`, which is the
compiler saying "you used a name I have never heard of".

**Then stub the three new methods** so everything compiles before anything works. Two parts.

At the top of `src/case.rs`, next to the `use` line that is already there:

```rust
use crate::error::{AppError, AppResult};
```

Rust files do not share names automatically. Every file imports what it uses, the same as an
`import` in TypeScript — `error.rs` being in the same project changes nothing.

Then in the existing `impl Case` block, lower down:

```rust
pub fn require_suspect(&self, id: SuspectId) -> AppResult<&Suspect> { todo!() }
pub fn require_fact_mut(&mut self, id: FactId) -> AppResult<&mut Fact> { todo!() }
pub fn reveal(&mut self, fact: FactId, to: SuspectId) -> AppResult<()> { todo!() }
```

`todo!()` is a placeholder, same as in Stage 4: the compiler accepts it anywhere a value is
expected, and if the program actually reaches it, it stops with "not yet implemented". This turns
"nothing builds" into "here is a list of failing tests", which is much easier to work down.

At that point `cargo test --test errors` should report **2 passed, 12 failed**. That is your
starting line.

---

## 1. `Result` — the half of `Option` you have not met

You have used `Option` since Stage 3. Here it is next to its sibling:

```rust
enum Option<T> {          enum Result<T, E> {
    Some(T),                  Ok(T),
    None,                     Err(E),
}                         }
```

Read them out loud:

- `Option` — "either a value, or nothing."
- `Result` — "either a value, or **a reason it failed**."

That is the whole difference. The failure side of `Result` carries something, and you get to choose
what. Both are ordinary enums from Rust's standard library, like the `Difficulty` enum you wrote in
Stage 1 — not special syntax.

### The TypeScript comparison

In TypeScript you have two ways to say a function can fail:

```ts
function findSuspect(id: number): Suspect | undefined    // return nothing
function findSuspect(id: number): Suspect                // throw
```

`Option` is the first one. There is no Rust equivalent of the second, and this is the part worth
sitting with: **Rust has no `throw` for this kind of failure.**

Look at what that costs you in TS. The signature `(id: number): Suspect` does not mention failure
anywhere. You find out it can throw from the docs, from reading the body, or from production. Then
you write `catch (e: unknown)` and you are holding a value that could be anything.

In Rust, a function that can fail says so in its return type or it cannot fail:

```rust
fn require_suspect(&self, id: SuspectId) -> AppResult<&Suspect>
```

`AppResult<&Suspect>` is your name for `Result<&Suspect, AppError>` — you will define it in section
3. Anyone reading that line knows two things without opening the body: it can fail, and when it
does, it fails with an `AppError`.

(Rust does have `panic!`, which stops the program. That is for "this code is broken", not for "the
file was missing". It is why `CLAUDE.md` bans `unwrap()` outside tests.)

Nothing to write yet. On to the type.

---

## 2. Writing the error type

Here is a complete, small example. Different domain — a parcel depot — so you translate it rather
than copy it:

```rust
#[derive(Debug, thiserror::Error)]
pub enum DepotError {
    #[error("no locker numbered {number} here")]
    NoSuchLocker { number: u32 },

    #[error("locker {number} is already taken")]
    Occupied { number: u32 },
}
```

Four things in there, one at a time.

**It is an enum**, like `Difficulty` in Stage 1: a list of possibilities, exactly one of which is
true at a time.

**Each possibility carries data, with names.** `NoSuchLocker { number: u32 }` means "when this
happens, there is a `number` attached". The closest TypeScript is a union of object types:

```ts
type DepotError =
  | { kind: "noSuchLocker"; number: number }
  | { kind: "occupied";     number: number }
```

You build one by naming the fields:

```rust
DepotError::NoSuchLocker { number: 4 }
```

**`#[error("...")]` is the sentence a human reads.** `{number}` pulls in the field of that same
variant, exactly like `${number}` in a JavaScript template string. So:

```rust
DepotError::NoSuchLocker { number: 4 }.to_string()
// "no locker numbered 4 here"
```

**`#[derive(...)]` generates code for you.** You met `derive` in Stage 1 — writing
`#[derive(Clone)]` means "compiler, write the copying code for me". `thiserror::Error` is the same
deal: it reads your `#[error("...")]` lines and writes the code that turns a `DepotError` value into
that sentence. Without it you would hand-write a `match` with one arm per variant, like the
`Display` you wrote by hand in Stage 2, and keep it in sync forever.

That is all `thiserror` is. Not a framework, not error handling — a code generator for messages.

### Your seven variants

| Variant | Fields | For |
|---|---|---|
| `SuspectNotFound` | `id: SuspectId` | a suspect lookup missed |
| `FactNotFound` | `id: FactId` | a fact lookup missed |
| `CaseNotFound` | `slug: String` | no case file by that name (Stage 6) |
| `Io` | `path: String`, `message: String` | reading a file failed (Stage 6) |
| `Parse` | `path: String`, `message: String` | the file is not a valid case (Stage 6) |
| `Inference` | `message: String` | the AI model failed (Phase 2) |
| `InvalidState` | `action: String`, `state: String` | e.g. reporting with no case loaded (Phase 3) |

The exact sentences are pinned by the test `every_variant_reads_like_a_sentence`. Open it and copy
the strings from there — it is the spec for this enum.

Two rules to follow while you write it:

**Always use named fields.** `SuspectNotFound { id: SuspectId }`, never `SuspectNotFound(SuspectId)`.
Section 5 shows what breaks otherwise. Take it on trust for now.

**Our own problems carry data; problems from elsewhere carry text.** `SuspectNotFound` carries a real
`SuspectId`, because we know exactly what happened. `Io` carries a `String`, because that sentence
came out of Windows and we did not write it and cannot pull it apart.

### The derive line

`AppError` needs six:

```rust
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, serde::Serialize)]
```

**Add them one at a time**, running `cargo check --tests` between each. Same instruction as Stage 2,
same reason: six derives can fail in six different ways and you want to meet them one at a time.

`Serialize` is section 5's business. It will not compile yet — that is expected, and section 5 says
what to do about it.

**Checkpoint:** with the enum written and the three `Case` methods still `todo!()`, run
`cargo test --test errors`. **2 passed, 12 failed.** The two that pass are the ones that only look at
your error type. Come back.

---

## 3. From `None` to a reason

Your Stage 4 lookup already finds the suspect. All that changes is what happens when it does not:

```rust
// depot version
pub fn require_locker(&self, number: u32) -> DepotResult<&Locker> {
    self.locker(number).ok_or(DepotError::NoSuchLocker { number })
}
```

`ok_or` is a method on `Option`. It does one thing:

- `Some(x)` becomes `Ok(x)`
- `None` becomes `Err(whatever you passed in)`

That is it. Three tokens on top of the lookup you already have. Do not run the search again.

There is a sibling called `ok_or_else` that takes a closure — `.ok_or_else(|| Error { .. })` — a
function you pass as an argument, the same idea as an arrow function in JS. It exists so the error
is only built when there actually is a failure. That matters when building the error is expensive.
Ours is a struct with a number in it, so it is free, so `ok_or` is the right one. Reach for the
closure version and clippy will stop you:

```
error: unnecessary closure used to substitute value for `Option::None`
help: use `ok_or` instead
```

### The `AppResult` shorthand

Also in `error.rs`:

```rust
pub type AppResult<T> = Result<T, AppError>;
```

This is a nickname for a type, the same as `type Foo = Bar` in TypeScript. It creates nothing new —
`AppResult<&Suspect>` and `Result<&Suspect, AppError>` are the same thing to the compiler. It exists
because from here on every fallible function in the project returns `Result<Something, AppError>`,
and the shorthand keeps the interesting part — the `Something` — where your eye lands.

**Checkpoint:** write `require_suspect` and `require_fact_mut`. **7 passed, 7 failed.**

---

## 4. `?` — early return in one character

The last method needs to check two things before it acts. Here is the shape, depot version:

```rust
pub fn assign(&mut self, locker: u32, to: CourierId) -> DepotResult<()> {
    self.require_courier(to)?;
    self.require_locker_mut(locker)?.assign(to);
    Ok(())
}
```

**What `?` does.** Put it after anything that returns a `Result`:

- If it is `Ok(value)` — take the value out and carry on.
- If it is `Err(e)` — stop right here and return that error from **this** function.

So if line 1 fails, line 2 never runs. This is why the test
`reveal_checks_the_suspect_before_it_checks_the_fact` exists: with both ids wrong, only the first
check ever happens, so only the first error comes back. Which check you write first is visible
behaviour, not a detail.

Closest TypeScript is `await` on a promise that rejects: the rest of the function is skipped and the
failure goes to the caller. The difference is that `?` is visible in the code — you can see the exact
character where the function might leave.

**Two small things you will meet.**

`Ok(())` at the end. `()` is Rust's "no value worth returning" — TypeScript's `void`, except it is a
real value you can put inside things. `AppResult<()>` means "it worked; nothing to hand back".

Leave the `?` off and you get a warning, `unused Result that must be used`. Rust will not let you
quietly ignore something that might have failed. Our clippy settings turn that warning into an
error.

### The trap in this one

Write it the natural way and it will not compile:

```rust
let suspect = self.require_suspect(to)?;      // borrows the case to read
let fact = self.require_fact_mut(fact)?;      // wants to borrow it to write  <- E0502
fact.reveal_to(suspect.id);
```

This is Stage 4's rule, unchanged: **many readers, or one writer, never both.** Line 1 takes a read
borrow, line 3 still uses it, so line 2's write borrow is refused.

The fix is not `clone()` and not a scope block. It is to notice you never needed to *keep* the
suspect — you only wanted to know it exists, and `to` is an id you already have. Throw the result
away with a bare `?;` and the borrow ends at that semicolon.

Write the broken version first and read `E0502` before you fix it. It is the same error you
uncommented on purpose in Stage 4, arriving on its own this time.

**Checkpoint:** write `reveal`. **12 passed, 2 failed.**

---

## 5. Getting it to React

The last two tests are about what your error looks like once it leaves Rust.

When a Tauri command fails, Tauri turns the error into JSON and hands it to your React code. Turning
a Rust value into JSON is `serde`'s job — the crate already in your `Cargo.toml`, the one you used
in `ids.rs`. Where `thiserror` writes the *sentence*, `serde` writes the *JSON*. Two different jobs,
which is why `AppError` derives both.

Add one line above the enum:

```rust
#[serde(tag = "kind", rename_all = "camelCase")]
```

- `tag = "kind"` says: put the variant's name in a field called `"kind"`, and the variant's own
  fields beside it.
- `rename_all = "camelCase"` says: write `suspectNotFound` rather than `SuspectNotFound`, because
  the JSON is going to a TypeScript app.

What comes out:

```json
{ "kind": "suspectNotFound", "id": 99 }
```

Which is the discriminated union from section 2's TypeScript sketch, arriving in the browser ready
for a `switch` on `error.kind`.

**The id is a plain `99`.** `SuspectId` wraps a number, and serde looks straight through the wrapper.
The wrapper protects you inside Rust; it is not part of the JSON. That also means `SuspectId` and
`FactId` each need their own `#[derive(Serialize)]` in `ids.rs`. Leave it off and the compiler will
point at the field and say the trait bound is not satisfied — which is Rust's way of saying "this
type does not know how to become JSON".

**And now the reason for the named-fields rule.** Write a variant as `SuspectNotFound(SuspectId)`
with `tag = "kind"` set, and it compiles perfectly. Then, when it runs:

```
cannot serialize tagged newtype variant AppError::SuspectNotFound containing an integer
```

Because `"kind"` has to be added as a field, and `99` on its own has nowhere to put a field. The
compiler could not catch this; the test did. Same lesson as your Stage 2 `Display` that compiled
happily and printed `"3 suspect #"`.

**Checkpoint:** **14 passed.** Then `cargo fmt` and
`cargo clippy --all-targets -- -D warnings`.

---

## Your task

Make `src-tauri/tests/errors.rs` pass. Shapes given, bodies are yours.

**New file — `src-tauri/src/error.rs`:**

```rust
use serde::Serialize;
use thiserror::Error;

use crate::ids::{FactId, SuspectId};

/// Every fallible operation in Interrogator fails with one of these.
#[derive(/* six derives, added one at a time */)]
#[serde(/* section 5 */)]
pub enum AppError {
    // seven variants, named fields, one #[error("...")] each.
    // Sentences are pinned by `every_variant_reads_like_a_sentence` in the test.
}

/// Shorthand for the `Result` every fallible function in this crate returns.
pub type AppResult<T> = /* ... */;
```

**`src-tauri/src/ids.rs`** — add `Serialize` to both derive lists.

**`src-tauri/src/lib.rs`** — `pub mod error;`

**`src-tauri/src/case.rs`** — three methods in the existing `impl Case`:

```rust
/// The suspect with this id, or `AppError::SuspectNotFound`.
pub fn require_suspect(&self, id: SuspectId) -> AppResult<&Suspect> { todo!() }

/// Exclusive access to one fact, or `AppError::FactNotFound`.
pub fn require_fact_mut(&mut self, id: FactId) -> AppResult<&mut Fact> { todo!() }

/// Let one suspect in on one fact. Both ids must exist.
pub fn reveal(&mut self, fact: FactId, to: SuspectId) -> AppResult<()> { todo!() }
```

---

## Rules

1. Do not edit `tests/errors.rs`. If a test looks wrong, say so — you have been right before.
2. Add the derives on `AppError` **one at a time**, `cargo check --tests` between each.
3. No `clone()` in `require_suspect` or `require_fact_mut`. They hand back borrows; that is the point.
4. Write `reveal` the natural way first, read `E0502`, then fix it by not keeping the borrow.
5. No `unwrap()` / `expect()` in `src/`. Tests are exempt.
6. `cargo fmt`, then `cargo clippy --all-targets -- -D warnings` must pass.
7. Say **"ready"** and I review.

---

## Three decisions I already made

You do not need to think about these. Reasoning is in `docs/PROGRESS.md` if you ever want it.

- The `Io` variant carries `{ path, message }` rather than Rust's built-in file error type. The
  built-in one cannot become JSON.
- No real Tauri command in this stage. That needs app state, which is Stage 7. Here the test checks
  the JSON directly.
- `error.rs` follows the same rule as `case.rs` and `ids.rs`: no `tauri::`, no `tokio::`, no
  `std::fs`. `serde` is fine.

---

## Hints

Open only what you need, in order. Parcel depot, not detective case — translate it yourself.

<details>
<summary><b>Hint 1 — the whole error file</b></summary>

```rust
use serde::Serialize;
use thiserror::Error;

use crate::ids::LockerId;

#[derive(Debug, Clone, PartialEq, Eq, Error, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum DepotError {
    #[error("this depot has no {id}")]
    LockerNotFound { id: LockerId },

    #[error("could not reach {endpoint}: {message}")]
    Network { endpoint: String, message: String },
}

pub type DepotResult<T> = Result<T, DepotError>;
```

Notes:

- `Error` in the derive list is the code generator from `thiserror`. It is not the same thing as
  Rust's built-in `std::error::Error`, which is what it generates the code *for*. Confusing that
  they share a name; that is the ecosystem's fault, not yours.
- `{id}` inside `#[error("...")]` prints the field using the `Display` you wrote back in Stage 2 —
  which is why a `SuspectId` comes out as `suspect #99` and not as a bare number.
- The `#[serde(...)]` line sits above the enum and applies to all of it.
</details>

<details>
<summary><b>Hint 2 — turning a lookup into a requirement</b></summary>

```rust
impl Depot {
    // Stage 4 style: the caller decides what "missing" means.
    pub fn locker(&self, id: LockerId) -> Option<&Locker> {
        self.lockers.iter().find(|l| l.id == id)
    }

    // Stage 5 style: missing is a failure, and it says which one.
    pub fn require_locker(&self, id: LockerId) -> DepotResult<&Locker> {
        self.locker(id).ok_or(DepotError::LockerNotFound { id })
    }

    pub fn require_locker_mut(&mut self, id: LockerId) -> DepotResult<&mut Locker> {
        self.locker_mut(id).ok_or(DepotError::LockerNotFound { id })
    }
}
```

Keep both versions. They answer different questions and both have callers.
</details>

<details>
<summary><b>Hint 3 — two checks, one action</b></summary>

```rust
impl Depot {
    pub fn assign(&mut self, locker: LockerId, to: CourierId) -> DepotResult<()> {
        self.require_courier(to)?;                    // check, and drop the borrow
        self.require_locker_mut(locker)?.assign(to);  // now free to borrow for writing
        Ok(())
    }
}
```

The bare `?;` on line 1 is the trick. It runs the check, and the semicolon throws away the
`&Courier`, so nothing is holding the depot when line 2 asks to write to it.

Do the broken version once on purpose — `let courier = ...` on line 1 and `courier.id` on line 3 —
and read `E0502`. It names all three lines.
</details>

<details>
<summary><b>Hint 4 — the two renderings</b></summary>

```rust
let e = DepotError::LockerNotFound { id: LockerId::new(4) };

e.to_string()               // "this depot has no locker #4"      <- thiserror wrote this
serde_json::to_string(&e)   // Ok("{\"kind\":\"lockerNotFound\",\"id\":4}")   <- serde wrote this
```

One value, two outputs, two different generators. Neither is built from the other, and the test
checks both.

If `serde_json::to_string` gives you an `Err` when you run it, you wrote the variant as
`LockerNotFound(LockerId)` instead of `LockerNotFound { id: LockerId }` — section 5.

If it will not compile at all, and the error points at the field, `LockerId` is missing its own
`#[derive(Serialize)]`.
</details>

<details>
<summary><b>Hint 5 — near-complete shape, your types</b></summary>

```rust
// src-tauri/src/error.rs

#[derive(Debug, Clone, PartialEq, Eq, Error, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum AppError {
    #[error("this case has no {id}")]
    SuspectNotFound { id: SuspectId },

    #[error("this case has no {id}")]
    FactNotFound { id: FactId },

    /* CaseNotFound { slug }, Io { path, message }, Parse { path, message },
       Inference { message }, InvalidState { action, state } */
}

pub type AppResult<T> = Result<T, AppError>;
```

```rust
// src-tauri/src/case.rs

pub fn require_suspect(&self, id: SuspectId) -> AppResult<&Suspect> {
    self.suspect(id).ok_or(/* ... */)
}

pub fn require_fact_mut(&mut self, id: FactId) -> AppResult<&mut Fact> {
    /* the same two lines, one word different */
}

pub fn reveal(&mut self, fact: FactId, to: SuspectId) -> AppResult<()> {
    /* check the suspect and drop the borrow; then require the fact and reveal_to */
}
```

One detail you will otherwise hunt for: `CaseNotFound`'s sentence uses `{slug:?}`, not `{slug}`. The
`:?` prints a `String` with quotation marks around it, which is why the expected message has
`"the-ledger"` in quotes.
</details>

---

## If you finish early

Add a variant with no fields at all — just `Cancelled,` — give it an `#[error("...")]`, and predict
the JSON before you run it. Phase 2 will want this one when the player interrupts the AI
mid-sentence. Then either delete it or keep it and tell me why; an unused variant is not free.
