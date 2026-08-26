# Stage 5 — `AppError`, `thiserror`, and `Result` at the boundary

**Crate:** `src-tauri`
**New file:** `src-tauri/src/error.rs`
**Test:** `src-tauri/tests/errors.rs`
**Run:** `cd src-tauri && cargo test --test errors`
**Est.** 90–120 min.

---

## Background — why this task exists

Stage 4 gave `Case` two lookups that can come up empty, and both answer with `Option`:

```rust
pub fn suspect(&self, id: SuspectId) -> Option<&Suspect>
pub fn fact_mut(&mut self, id: FactId) -> Option<&mut Fact>
```

`Option` is the right answer for a caller who has a sensible fallback — "is Marta in this case? no?
fine, skip her." It is the wrong answer for a caller who is about to *tell the player something went
wrong*, because `None` carries nothing: no reason, no id, nothing a UI can render beyond a shrug.

Two stages from now a `#[tauri::command]` will sit at the top of this call chain, and whatever it
returns gets serialized and handed to React. That is the real constraint driving this stage: the
failure type has to survive the trip to TypeScript as *data*, not as a sentence. `Option<T>` cannot;
`Result<T, AppError>` can, once `AppError` exists and is designed for the wire.

So this stage builds the error type the whole project will use, and converts the two lookups' harsh
cousins — `require_suspect` and `require_fact_mut` — plus one operation that has to check two things
before it can do anything, which is where `?` earns its keep.

After this: parse-don't-validate and real TOML case files (Stage 6/7), which is the first code that
*produces* the `Io` and `Parse` variants you are about to write.

**One concept at a time below.** Six sections. Read one, write that piece, run the test, come back.
Do not read section 5 before section 1 compiles.

---

## Decisions already made — you do not need to adjudicate these

Three questions came up while writing this stage. They are architecture, so they are mine; the
reasoning is in the decisions log in `PROGRESS.md` (2026-08-25) if you want to argue with it, and
you are welcome to.

1. **`AppError` holds no `std::io::Error`.** The `Io` variant carries `{ path: String, message:
   String }`. See section 3 for what happens if you try the other thing.
2. **No `#[tauri::command]` in this stage.** `error.rs` gets `serde`, and the test proves the JSON
   shape with `serde_json`. Wiring an actual command needs managed state, which is Stage 7.
3. **`error.rs` is a domain module**, subject to the same rule as `case.rs` and `ids.rs`: no
   `tauri::`, no `tokio::`, no `std::fs`. `serde` is fine — it is a pure data-shape crate.

---

## 0. Adding a dependency

First time you add a crate to this project yourself.

```
cd src-tauri
cargo add thiserror
```

That edits `Cargo.toml` for you and picks the current major version (2.x). `serde` and `serde_json`
are already there from the Tauri template, so that is the only new one.

**"Tauri already depends on `thiserror` — can I not just use that one?"** No, and the reason is a
real difference from npm: **Cargo gives you no access to your dependencies' dependencies.** Only
crates named in your own `Cargo.toml` enter scope. Tauri's dependency on `thiserror` is private to
Tauri, and `use thiserror::Error;` will not compile until the line is in your manifest.

Node works the other way — `node_modules` is flat, so you can `require` anything a dependency
happened to install, right up until it stops installing it in a patch release and your build breaks
for reasons unrelated to your code. Cargo makes that unrepresentable.

The cost is nil either way: `src-tauri/Cargo.lock` already contains `thiserror 2.0.20` — `tauri`,
`wry`, `muda` and several others pull it in — so `cargo add` resolves to a crate that is already
being compiled. One line in `Cargo.toml`, no download, no extra build time. It is also why you do
not add `serde`: the Tauri template already listed it as a *direct* dependency of yours, which is
the only reason `use serde::Serialize;` works in `ids.rs` today.

(And no, `tauri::Error` is not a substitute for `AppError`. It is Tauri's error type for Tauri's
failures — window creation, IPC plumbing, asset resolution — it has no variant meaning "this case
has no suspect #99", and you cannot add one to another crate's enum. It would also drag `tauri::`
into a domain module, which is the boundary rule from `CLAUDE.md` and the thing that keeps
`cargo test` runnable without launching a window.)

While you are in `Cargo.toml`, look at what `cargo add` wrote and note that `serde` has
`features = ["derive"]`. Rust crates ship optional pieces behind **feature flags**; `serde` without
`derive` gives you the traits but not the `#[derive(Serialize)]` macro, because compiling a proc
macro is not free and not everyone wants it. It is `package.json` optional peer deps, except the
resolution happens at compile time and unused features cost nothing in the binary.

Then declare the module. `src-tauri/src/lib.rs`, alphabetical with the others:

```rust
pub mod case;
pub mod difficulty;
pub mod error;
pub mod ids;
```

**Checkpoint:** create an empty `src/error.rs`, run `cargo check`. Clean build, nothing to show for
it yet.

---

## 1. `Result` is `Option` with a reason attached

You have been using one half of this pair for three stages. Here is the other:

```rust
enum Option<T> {          enum Result<T, E> {
    Some(T),                  Ok(T),
    None,                     Err(E),
}                         }
```

That is the entire difference: the failure arm carries a value. Everything else — `match`, `if let`,
`.map()`, `.expect()` — works the same way on both, and both are ordinary enums from the standard
library rather than language features.

The TypeScript comparison is worth making carefully, because this is one of the places Rust is
genuinely different and not just differently spelled:

| TypeScript | Rust |
|---|---|
| `Suspect \| undefined` | `Option<&Suspect>` |
| `throw new NotFoundError(id)` | `return Err(AppError::SuspectNotFound { id })` |
| `try { … } catch (e: unknown)` | `match f() { Ok(v) => …, Err(e) => … }` |

The one that matters: **a thrown exception is invisible in a TS signature, and an `Err` is not.**
`function loadCase(slug: string): Case` tells you nothing about failure; you find out from the docs,
or from production. `fn load_case(slug: &str) -> AppResult<Case>` cannot hide it. That is why Rust
has no exceptions for recoverable failure — the failure is part of the return type or it does not
exist. (`panic!` exists and is not this. A panic means "this program is broken", not "the file was
missing"; it is why `CLAUDE.md` bans `unwrap()` outside `main.rs` and tests.)

`catch (e: unknown)` versus a typed `E` is the second difference, and it is the one this stage is
built around: because `E` is a type you designed, the caller can `match` on *which* failure happened
and the compiler will tell them when you add a variant they have not handled.

---

## 2. What `thiserror` actually does

Nothing magic, and it is worth knowing exactly what before you use it.

Rust's `std::error::Error` is a trait, and it has a supertrait requirement: to implement it, your
type must already implement `Display` and `Debug`. So a hand-written error type is three impls — the
enum, a `Display` with a `match` arm per variant formatting a message, and an empty
`impl std::error::Error for AppError {}`. You wrote a `Display` by hand in Stage 2, so you know the
shape and you know it is tedious.

`thiserror` is a **derive macro** that generates those two impls from an attribute:

```rust
#[derive(Debug, thiserror::Error)]
pub enum DepotError {
    #[error("no locker numbered {number} at this depot")]
    NoSuchLocker { number: u32 },
}
```

`#[error("...")]` is not a doc comment and not a description — it *is* the body of the generated
`Display` impl, and `{number}` interpolates the field of that variant. Which means
`DepotError::NoSuchLocker { number: 4 }.to_string()` is `"no locker numbered 4 at this depot"`, and
`println!("{e}")` prints the same. Same relationship as Stage 1's `derive(Clone)`: the macro writes
code you could have written, and you should be able to picture the code it writes.

Two consequences worth having in your head now:

- Interpolation goes through `Display`, so `#[error("this case has no {id}")]` on a field of type
  `SuspectId` calls the `Display` **you wrote in Stage 2** and produces `"this case has no suspect
  #99"`. Your work from two stages ago is what makes the message read properly.
- Leave `#[error(...)]` off a variant and you get `error: missing #[error("...")] display
  attribute`, pointing at the variant. The macro checks.

**`thiserror` vs `anyhow`**, since you will meet both and the internet conflates them:
`anyhow::Error` is one opaque type that can hold any error at all, great for a binary's `main` where
you only ever print it. `thiserror` builds a *specific* enum a caller can match on. Rule from
`ROADMAP.md`: `thiserror` at library boundaries, and this whole crate is a library boundary. Do not
reach for `anyhow` here.

---

## 3. Designing the variants

Seven of them. `ROADMAP.md` §1.2 named five; two are split out because "not found" wants to say
*which* id:

| Variant | Fields | Raised when |
|---|---|---|
| `SuspectNotFound` | `id: SuspectId` | a lookup by suspect id misses |
| `FactNotFound` | `id: FactId` | a lookup by fact id misses |
| `CaseNotFound` | `slug: String` | no case file by that name (Stage 6) |
| `Io` | `path: String`, `message: String` | reading a file failed (Stage 6) |
| `Parse` | `path: String`, `message: String` | the file is not a valid case (Stage 6) |
| `Inference` | `message: String` | the model layer failed (Phase 2) |
| `InvalidState` | `action: String`, `state: String` | "submit a report" with no case loaded (Phase 3) |

Two design rules are visible in that table, and both are load-bearing.

**Every variant uses named fields**, `Variant { field: T }`, never `Variant(T)`. Partly it reads
better at the construction site — `AppError::SuspectNotFound { id }` beats
`AppError::SuspectNotFound(id)` when you are three call layers deep. Mostly it is section 6's
problem: serde's tagging mode cannot serialize a tuple variant holding a number, and it will let you
compile and then fail at *runtime*. Take the rule now, see the reason in section 6.

**Our failures are structured; foreign diagnostics are text.** `SuspectNotFound` carries a
`SuspectId`, because we know exactly what went wrong and the frontend may want to do something with
it. `Io` carries a `String` message, because that sentence came out of the operating system and we
did not write it and cannot decompose it. Inventing structure you do not have is worse than
admitting you have none.

### Why not `Io(#[from] std::io::Error)`

Because you will see that shape in every `thiserror` tutorial, and it is wrong *here*. `thiserror`'s
`#[from]` generates an `impl From<std::io::Error> for AppError`, which would let `?` convert io
failures automatically. Try it and the four errors you get are the argument:

```
error[E0277]: the trait bound `std::io::Error: Clone` is not satisfied
error[E0369]: binary operation `==` cannot be applied to type `&std::io::Error`
error[E0277]: the trait bound `std::io::Error: Eq` is not satisfied
error[E0277]: the trait bound `std::io::Error: serde::Serialize` is not satisfied
```

`std::io::Error` is not `Clone`, not `PartialEq`, and not `Serialize`. Our error must be all three:
`Serialize` to cross to React, `PartialEq` so tests can `assert_eq!` on it, `Clone` because a
session will want to hang on to the last failure. Keeping `#[from]` means hand-writing `Serialize`
and giving up `assert_eq!`.

And the thing `#[from]` buys is smaller than it looks. A bare `std::io::Error` says "No such file or
directory (os error 2)" and **does not know which file** — the path only exists at the call site. So
even with `#[from]`, that call site has to add context, which means writing the conversion by hand
anyway. Stage 6 will do it explicitly:

```rust
std::fs::read_to_string(&path).map_err(|e| AppError::Io {
    path: path.display().to_string(),
    message: e.to_string(),
})?
```

One `.map_err` at the one place that touches the filesystem. That is the whole cost.

---

## 4. `ok_or` — the bridge from `Option`

`require_suspect` is not a new lookup. It is Stage 4's lookup with the empty case given a reason:

```rust
pub fn require_suspect(&self, id: SuspectId) -> AppResult<&Suspect> {
    self.suspect(id).ok_or(AppError::SuspectNotFound { id })
}
```

`Option::ok_or` is the standard-library method for exactly this: `Some(v)` becomes `Ok(v)`, `None`
becomes `Err(the value you passed)`. Note the borrow survives it untouched — `Option<&Suspect>` in,
`Result<&Suspect, AppError>` out, still pointing into the case. The test asserts that with
`std::ptr::eq`.

There is a sibling, `ok_or_else`, which takes a closure instead of a value. The difference is *when*
the error is built: `ok_or` builds it always, including on the happy path where it is thrown away;
`ok_or_else` builds it only on `None`. Use `ok_or_else` when constructing the error costs something
— an allocation, a `format!`, a filesystem call. Use `ok_or` when it is a struct literal of `Copy`
fields, like this one.

Guess wrong toward the closure and clippy stops you:

```
error: unnecessary closure used to substitute value for `Option::None`
  = note: `#[warn(clippy::unnecessary_lazy_evaluations)]` on by default
help: use `ok_or` instead
```

Which is the lint doing its job — read the name, not just the fix.

**`AppResult<T>`.** Also define, in `error.rs`:

```rust
pub type AppResult<T> = Result<T, AppError>;
```

A type alias, the same idea as TypeScript's `type X = …`: a new name for an existing type, no new
type, fully interchangeable. It exists because `Result<T, AppError>` appears in every signature in
the crate from here on, and the alias makes the interesting half — `T` — the part you read. The test
uses it in a signature, so it has to exist and has to be public.

**Checkpoint:** write `AppError`, `AppResult`, and both `require_*` methods. Run
`cargo test --test errors`. Four tests should pass, several should fail. Come back.

---

## 5. `?` — early return, spelled with one character

```rust
pub fn reveal(&mut self, fact: FactId, to: SuspectId) -> AppResult<()> {
    self.require_suspect(to)?;
    self.require_fact_mut(fact)?.reveal_to(to);
    Ok(())
}
```

`expr?` on a `Result` means, roughly:

```rust
match expr {
    Ok(v) => v,
    Err(e) => return Err(From::from(e)),
}
```

Three things in that desugaring are worth naming.

**It returns from the enclosing function**, not from the expression. `?` is control flow. So the
second line of `reveal` never runs when the first fails, which is what
`reveal_checks_the_suspect_before_it_checks_the_fact` pins down: with both ids wrong, only the first
error is ever constructed. The order of your two checks is observable behaviour.

**It calls `From::from` on the error.** Every error here is already `AppError`, so that conversion
is the identity and costs nothing — but it is the hook that lets a function returning `AppError`
call something returning a different error type, as long as an `impl From<ThatError> for AppError`
exists. That is what `#[from]` in section 3 generates, and it is how `?` composes across crate
boundaries. You will use it properly in Stage 6.

**The first line discards its `Ok` value on purpose.** `self.require_suspect(to)?;` — the semicolon
throws away the `&Suspect`, and all we wanted was the check. Leave the `?` off and the compiler
notices:

```
warning: unused `Result` that must be used
  = note: this `Result` may be an `Err` variant, which should be handled
```

`Result` is `#[must_use]`, so ignoring one is a warning — and `-D warnings` in our clippy line makes
it an error. Rust will not let you silently drop a failure.

### The borrow trap in `reveal`

Write the body the natural way and you will hit this:

```rust
let suspect = self.require_suspect(to)?;      // immutable borrow of *self
let fact = self.require_fact_mut(fact)?;      // E0502: mutable borrow of *self
fact.reveal_to(suspect.id);                   //        immutable borrow used here
```

This is Stage 4's rule, unchanged: many readers XOR one writer. The shared borrow is still alive on
line 3 because `suspect` is used there, so the exclusive borrow on line 2 is rejected.

The fix is not a `clone()` and not a scope block. It is to **not keep the borrow you do not need**.
You only wanted to know the suspect exists; `to` is a `Copy` id you already hold. Discard the `&`
with a bare `?;` and the borrow ends at that semicolon — non-lexical lifetimes, section 2 of the
last brief — leaving `self` free for the mutable borrow on the next line.

Worth doing deliberately: write the broken version first, read `E0502`, then fix it. It is the same
error you were told to uncomment-and-read in Stage 4, arriving on its own this time.

**Checkpoint:** write `reveal`. Eleven tests should pass. One section left.

---

## 6. Crossing to TypeScript

`invoke("reveal", …)` on the React side returns a promise. When the Rust command returns `Err(e)`,
Tauri serializes `e` and rejects the promise with it. So `AppError` needs `Serialize`, and the
question is what shape it should take on the wire.

Derive it with an attribute:

```rust
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error, serde::Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum AppError { … }
```

`tag = "kind"` selects serde's **internally tagged** representation: the variant name goes into a
`"kind"` field alongside the variant's own fields, rather than nesting them under it.
`rename_all = "camelCase"` converts the variant names, because `SuspectNotFound` in JSON would be
shouting in the wrong language. Result:

```json
{ "kind": "suspectNotFound", "id": 99 }
{ "kind": "io", "path": "cases/the-ledger.toml", "message": "No such file or directory (os error 2)" }
```

Three things to notice.

**The id is a bare `99`.** serde serializes a newtype struct as its inner value — `SuspectId(99)`
becomes `99`. The newtype is a Rust-side guarantee, not a wire format. Which also means `SuspectId`
and `FactId` need `Serialize` themselves; add the derive in `ids.rs` and read the error you get if
you forget (`E0277`, pointing at the field, not at the id type).

**There is no `message` field on the wire, and that is deliberate.** The English sentence lives in
`Display`, for logs and for `tracing`. The frontend gets `kind` plus data and writes its own copy —
because a UI that needs to restyle or translate a message should not be handed a fixed English
string from the backend. The `message` fields on `Io`, `Parse` and `Inference` are the exception
that proves it: those strings came from the OS, the TOML parser and llama.cpp, and we have nothing
better to offer.

**The trap that made section 3's "named fields always" rule.** Write a variant as
`SuspectNotFound(SuspectId)` with `tag = "kind"` set and it compiles perfectly. Then at runtime:

```
Error("cannot serialize tagged newtype variant AppError::SuspectNotFound containing an integer")
```

Internal tagging works by inserting a key into a map, and `99` is not a map. The type system had
nothing to say; the test caught it. Same lesson as your Stage 2 `Display` that compiled and printed
`"3 suspect #"` — types check shapes, tests check behaviour, and you need both.

**Checkpoint:** add the two `#[serde]` attributes and the derives on the ids. Fourteen tests.

---

## Your task

Make `src-tauri/tests/errors.rs` compile and pass. Signatures given, bodies elided.

**New file — `src-tauri/src/error.rs`:**

```rust
use serde::Serialize;
use thiserror::Error;

use crate::ids::{FactId, SuspectId};

/// Every fallible operation in Interrogator fails with one of these.
#[derive(/* Debug, Clone, PartialEq, Eq, Error, Serialize — one at a time */)]
#[serde(/* see section 6 */)]
pub enum AppError {
    // seven variants, named fields, one `#[error("...")]` each.
    // The exact message strings are pinned by `every_variant_reads_like_a_sentence`
    // in the test — read that test, it is the spec for this enum.
}

/// Shorthand for the `Result` every fallible function in this crate returns.
pub type AppResult<T> = /* ... */;
```

**`src-tauri/src/ids.rs`:** add `Serialize` to both derive lists.

**`src-tauri/src/lib.rs`:** `pub mod error;`

**`src-tauri/src/case.rs`:** three methods on `Case`, alongside the Stage 4 ones.

```rust
impl Case {
    /// The suspect with this id, or `AppError::SuspectNotFound`.
    pub fn require_suspect(&self, id: SuspectId) -> AppResult<&Suspect> { todo!() }

    /// Exclusive access to one fact, or `AppError::FactNotFound`.
    pub fn require_fact_mut(&mut self, id: FactId) -> AppResult<&mut Fact> { todo!() }

    /// Let one suspect in on one fact. Both ids must exist.
    pub fn reveal(&mut self, fact: FactId, to: SuspectId) -> AppResult<()> { todo!() }
}
```

`todo!()` works for all three this time — none of them returns `impl Trait`.

---

## Rules of the loop

1. Do not edit `src-tauri/tests/errors.rs`. If a test looks wrong, say so — you have been right
   before.
2. **Add the derives on `AppError` one at a time**, `cargo check --tests` between each. Six derives
   is six different failure modes and you want to meet them separately. Same instruction as Stage 2,
   same reason.
3. No `clone()` in `require_suspect` or `require_fact_mut`. They return borrows; that is the point.
4. Write `reveal` the natural way first, read `E0502`, then fix it by dropping the borrow you do not
   need. Do not fix it with a `clone()` or a scope block.
5. No `unwrap()` / `expect()` in `src/`. Tests are exempt, as always.
6. `cargo fmt`, then `cargo clippy --all-targets -- -D warnings` must pass.
7. Say **"ready"** and I review.

---

## Hints

Open only what you need, in order. Parcel depot, not detective case — translate it yourself.

<details>
<summary><b>Hint 1 — the error enum</b></summary>

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

    #[error("cannot {action} while {state}")]
    InvalidState { action: String, state: String },
}

pub type DepotResult<T> = Result<T, DepotError>;
```

Things to copy and things to notice:

- `Error` in the derive list is `thiserror::Error`, a derive macro that happens to share a name with
  `std::error::Error`, the trait. They are different items: the macro *writes* the impl of the
  trait. `use thiserror::Error;` brings the macro into scope; if you ever need to name the trait,
  spell it out as `std::error::Error` — which is what the test does.
- `{id}` in the message interpolates the field named `id` and formats it with `Display`.
- The `#[serde(...)]` line applies to the whole enum, not to one variant.
- `rename_all = "camelCase"` renames the *variants*. The field names here are single words already,
  so nothing happens to them.
</details>

<details>
<summary><b>Hint 2 — turning a lookup into a requirement</b></summary>

```rust
impl Depot {
    // Stage-4 style: caller decides what "missing" means.
    pub fn locker(&self, id: LockerId) -> Option<&Locker> {
        self.lockers.iter().find(|l| l.id == id)
    }

    // Stage-5 style: missing is a failure, and it says which one.
    pub fn require_locker(&self, id: LockerId) -> DepotResult<&Locker> {
        self.locker(id).ok_or(DepotError::LockerNotFound { id })
    }

    pub fn require_locker_mut(&mut self, id: LockerId) -> DepotResult<&mut Locker> {
        self.locker_mut(id).ok_or(DepotError::LockerNotFound { id })
    }
}
```

The second method is three tokens of work on top of the first, and that is the correct amount. Do
not re-run the `find`. Keep both the `Option` and the `Result` version — they answer different
questions and both have callers.

`ok_or`, not `ok_or_else`: the error is a struct literal of `Copy` fields, so building it eagerly is
free and clippy will tell you off for the closure.
</details>

<details>
<summary><b>Hint 3 — two checks, one operation</b></summary>

```rust
impl Depot {
    pub fn assign(&mut self, locker: LockerId, to: CourierId) -> DepotResult<()> {
        self.require_courier(to)?;                    // check, then drop the borrow
        self.require_locker_mut(locker)?.assign(to);  // now `self` is free to be borrowed mutably
        Ok(())
    }
}
```

The bare `?;` on the first line is the whole trick. It runs the check, and the semicolon throws away
the `&Courier` so nothing is borrowing `self` when line 2 asks for `&mut self`.

Write it as `let courier = self.require_courier(to)?;` and use `courier.id` on line 3 instead of
`to`, and you get `E0502`. Do that once on purpose before you write the version above — the error
message names all three lines and is worth reading.

Note also what `Ok(())` is: `()` is the unit type, Rust's "no meaningful value" — TypeScript's
`void`, except it is a real value you can put inside things. `AppResult<()>` is "succeeded, nothing
to hand back".
</details>

<details>
<summary><b>Hint 4 — what the wire format looks like</b></summary>

```rust
let e = DepotError::LockerNotFound { id: LockerId::new(4) };

serde_json::to_string(&e)   // Ok("{\"kind\":\"lockerNotFound\",\"id\":4}")
e.to_string()               // "this depot has no locker #4"
```

Two different renderings of the same value, from two different traits. `Serialize` produces the
structured one for the frontend; `Display` — the one `thiserror` generated from your
`#[error("...")]` — produces the sentence for logs. Neither is derived from the other, and the test
checks both.

If `serde_json::to_string` returns an `Err` at runtime instead of a string, you wrote a variant as
`LockerNotFound(LockerId)` rather than `LockerNotFound { id: LockerId }`. Section 6.

If it will not compile at all, with `E0277` on the field, `LockerId` is missing its own
`#[derive(Serialize)]`.
</details>

<details>
<summary><b>Hint 5 — near-complete shape</b></summary>

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
       Inference { message }, InvalidState { action, state } —
       message strings are pinned by `every_variant_reads_like_a_sentence` */
}

pub type AppResult<T> = Result<T, AppError>;
```

```rust
// src-tauri/src/case.rs, in the existing `impl Case`

pub fn require_suspect(&self, id: SuspectId) -> AppResult<&Suspect> {
    self.suspect(id).ok_or(/* ... */)
}

pub fn require_fact_mut(&mut self, id: FactId) -> AppResult<&mut Fact> {
    /* the same two lines, one letter different */
}

pub fn reveal(&mut self, fact: FactId, to: SuspectId) -> AppResult<()> {
    /* check the suspect, discarding the borrow; then require the fact and reveal_to */
}
```

`CaseNotFound`'s message uses `{slug:?}` rather than `{slug}` — the `:?` is `Debug` formatting,
which for a `String` adds the quotes. That is why the expected message has `"the-ledger"` in
quotation marks.
</details>

---

## Optional, if you finish early

**One.** Add a variant `Cancelled` with no fields at all — a **unit variant**, `Cancelled,` — give it
an `#[error(...)]`, and serialize it. Predict the JSON before you run it. (Phase 2 will want this
one when the player interrupts a generation mid-sentence.) Then remove it again, or keep it and tell
me why; either is defensible, an unused variant is not free.

**Two.** In a scratch test, write a function returning `AppResult<()>` that calls `reveal` twice with
`?`, and give it a `main`-like caller that prints the error. Then try to make it return
`Result<(), String>` instead and watch `?` stop compiling. The error message will mention `From` —
that is section 5's third point arriving in person, and it is the thing Stage 6 turns on.
