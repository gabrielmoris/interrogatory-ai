# Stage 2 — `SuspectId` and `FactId`

**Crate:** `src-tauri`
**Test:** `src-tauri/tests/ids.rs`
**Run:** `cd src-tauri && cargo test --test ids`
**Est.** 45–75 min.

---

## Background — why this task exists

Stage 1 was about *which one* (enum) and *payload* (struct). This stage is about **giving a
number a name the compiler enforces**, and it introduces the split between traits you *derive*
and traits you *write*.

### 1. Why not just `u32`?

The `Case` type is coming, and it will be full of numbers:

```rust
fn reveal(case: &mut Case, suspect: u32, fact: u32) { ... }

reveal(&mut case, 7, 3);   // ...or was it (3, 7)?
```

Both arguments are `u32`, so swapping them compiles, runs, and produces a subtly wrong game.
This is not a hypothetical class of bug — it is *the* bug that ID-heavy code produces.

Wrap each in its own type and the swap becomes a compile error with a message that names both
types. You pay nothing for it: the last test asserts `size_of::<SuspectId>() == size_of::<u32>()`.
The type exists for the compiler and evaporates in the binary.

### 2. Why not `String`?

Tempting, because JSON case files will have `"suspect": "marta"`. But `String` ids are 24 bytes
plus a heap allocation, are not `Copy`, invite typos that no compiler catches (`"marta"` vs
`"Marta"`), and make every map lookup a string hash. Names belong in a `name` field; identity
belongs in an integer. Stage 5 (`RawCase` → `TryFrom` → `Case`) is where the string in the file
gets turned into one of these — that is the parse-don't-validate boundary, and it only works if
the validated type is distinct from the raw one.

### 3. The TypeScript analogy, and where it breaks

You have probably written branded types:

```ts
type FactId = number & { readonly __brand: unique symbol };
const f = 7 as FactId;   // the escape hatch
```

Same idea, weaker guarantee. The TS brand is a comment to the type checker: it is erased, and
`as FactId` defeats it from anywhere. Rust's newtype is a genuine nominal type — and because
you will make the inner field **private**, code outside `ids.rs` has no cast, no escape hatch,
and no way to construct one except through the constructor you wrote. That is the part TS cannot
give you. The part it *can* give you, and Rust gives too, is zero runtime cost.

**Keep the inner field private.** `pub struct SuspectId(u32)` — not `pub struct SuspectId(pub u32)`.
The test never touches `.0`; if you make it public, the test still passes and the stage is still
wrong.

### 4. Derive vs. hand-written `impl` — the real lesson

Two kinds of trait in this stage:

- **Derived** (`Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord`).
  These are *structural*: their implementation is mechanically determined by the fields.
  `#[derive(...)]` is a macro that writes the obvious code for you.
- **Hand-written** (`Display`, `From<u32>`). These are *decisions*. There is no
  `#[derive(Display)]` in std, on purpose: only you know that a `SuspectId` should read as
  `suspect #3` and not `3` or `SuspectId(3)`. std refuses to guess.

`Debug` and `Display` are not redundant. `Debug` (`{:?}`) is for you — programmer-facing, derived,
shows structure. `Display` (`{}`) is for the user — hand-written, shows meaning. The test pins
both, differently, so you can feel the difference.

Two things in the test come free and you should work out why rather than take my word for it:

- `SuspectId::new(3).to_string()` — you will never write `to_string`.
- `let f: FactId = 7u32.into();` — you will never write `into`.

Both fall out of **blanket impls** in std: one `impl` written once for *every* type that satisfies
a bound. This has no TypeScript equivalent at all, and it is one of the genuinely good ideas in
the language. Hint 4 names them.

### 5. The `Hash` / `Eq` contract

`ids_work_as_map_keys` and `ids_work_in_sets_and_dedupe` are not padding — they are the shape the
game actually needs (`Fact.known_by: HashSet<SuspectId>`, and scoring diffing the player's cited
`HashSet<FactId>` against ground truth).

A `HashMap` key must be `Eq + Hash`, and the two must agree: **if `a == b` then `a` and `b` must
hash identically.** Deriving both keeps them consistent by construction. Deriving one and writing
the other by hand is the classic way to end up with a map that silently loses entries. Derive both
or write both; never mix.

Note also `Eq` vs `PartialEq`. `Eq` is a marker meaning "equality here is reflexive — `x == x`
always holds". `f32` is only `PartialEq` because `NaN != NaN`, which is exactly why `Tuning` in
Stage 1 could not be `Eq` and these can. That is also why `f32` can never be a `HashMap` key.

---

## Your task

Make `src-tauri/tests/ids.rs` compile and pass.

Create `src-tauri/src/ids.rs` (and register it — you did this once already in Stage 1) containing
two tuple structs, `SuspectId` and `FactId`, each wrapping a single **private** `u32`.

Each needs:

| | |
|---|---|
| `pub fn new(u32) -> Self` | the only way in |
| `pub fn get(self) -> u32` | the way out — take `self` **by value**, not `&self`, and be ready to defend that in review |
| `Debug` | derived; must print `SuspectId(3)` |
| `Clone`, `Copy` | same reasoning as `Difficulty` |
| `PartialEq`, `Eq` | for `assert_eq!` and map keys |
| `Hash` | for `HashMap` / `HashSet` |
| `PartialOrd`, `Ord` | for `<` and `.sort()` |
| `Display`, hand-written | `"suspect #3"` / `"fact #7"` |
| `From<u32>`, hand-written | so `.into()` works |

**Do not paste all eight derives in at once.** Start with none. Run `cargo test --test ids`, read
the *top* error only, add the one trait it names, run again. The sequence of errors is the lesson;
a complete derive list up front skips it entirely. You will see, in some order: `Debug` demanded by
`assert_eq!`'s failure message, `PartialEq` demanded by the comparison itself, `Eq + Hash` demanded
by `HashMap`, `Ord` demanded by `.sort()`, and `Copy` demanded by `.iter().map(|f| f.get())` —
the same `E0507` you met in Stage 1, in a new costume.

---

## Rules of the loop

1. Do not edit `src-tauri/tests/ids.rs`. If a test looks wrong, say so — that is a legitimate move.
2. Inner fields private. No `pub u32`.
3. Get to green, then stop. No `Suspect`, no `Fact`, no `Case` yet — those are Stage 3.
4. `cargo fmt`, then `cargo clippy --all-targets -- -D warnings` must pass before you call it done.
5. Say **"ready"** and I review.

---

## Hints

Open only what you need, in order.

<details>
<summary><b>Hint 1 — a struct with no field names</b></summary>

A struct whose fields are positional rather than named is a **tuple struct**. One line, and the
field is reached as `.0`:

```rust
// weather, not ids — translate it yourself
pub struct Celsius(f64);

impl Celsius {
    pub fn new(degrees: f64) -> Self {
        Celsius(degrees)
    }
}
```

Note that `pub struct Celsius(f64)` makes the *type* public and the *field* private. Inside
`ids.rs` you can read `.0`; the test file cannot. That asymmetry is the whole point.

Look up: "Rust tuple structs", "Rust `pub` and module privacy".
</details>

<details>
<summary><b>Hint 2 — let the compiler ask for the traits</b></summary>

Do not guess the derive list. Add `#[derive()]` empty above the type, run the test, and read the
first error. Each one names the missing trait explicitly, usually in a note like
``the trait `Hash` is not implemented for `SuspectId` ``.

Two you will have to reason about rather than copy:

- `Copy` requires `Clone` — the compiler will tell you, but ask yourself why the dependency runs
  in that direction.
- `Eq` requires `PartialEq`, and `Ord` requires `PartialOrd` **and** `Eq`. There is a hierarchy
  here; the error messages sketch it if you read them.

Look up: "Rust derivable traits", "Rust `PartialOrd` vs `Ord`".
</details>

<details>
<summary><b>Hint 3 — `Display` is written, not derived</b></summary>

`Display` lives in `std::fmt`. Implementing it means writing one method that receives a formatter
and returns `fmt::Result`:

```rust
use std::fmt;

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}°C", self.0)
    }
}
```

Three things worth noticing before you translate this:

- `write!` is `format!`'s sibling that writes into a target instead of allocating a `String`,
  and it returns the `fmt::Result` you need — so it is usually the whole body, no `return`.
- `&self`, not `self`. Trait signatures are fixed by the trait; you do not get to choose.
- `Formatter<'_>` — your first lifetime. `'_` means "there is one here, infer it". You will meet
  it properly in Stage 4.

Look up: "Rust implement Display", "Rust `write!` macro".
</details>

<details>
<summary><b>Hint 4 — where `to_string()` and `.into()` come from</b></summary>

You are not asked to write either. Both come from **blanket impls** in std:

```rust
impl<T: Display> ToString for T { ... }      // Display  ⇒  .to_string()
impl<T, U: From<T>> Into<U> for T { ... }    // From<A> for B  ⇒  A: .into() -> B
```

So implementing `Display` hands you `to_string()`, and implementing `From<u32> for FactId` hands
you `7u32.into()`. Write the `From`; never write the `Into`.

The `From` impl itself is small:

```rust
impl From<f64> for Celsius {
    fn from(degrees: f64) -> Self {
        Celsius::new(degrees)
    }
}
```

And a question worth chasing: *why are you allowed to write this at all?* You are implementing a
trait for a type — but `u32` is std's and `From` is std's. The rule that decides this is the
**orphan rule**: the impl is legal if the trait or the type is local to your crate. Work out
which half saves you here, and what would happen if you tried `impl From<SuspectId> for u32`
versus `impl From<u32> for SuspectId`.

Look up: "Rust blanket implementations", "Rust orphan rule / coherence".
</details>

<details>
<summary><b>Hint 5 — near-complete shape</b></summary>

```rust
// src-tauri/src/ids.rs

use std::fmt;

/// Identifies one suspect within a `Case`.
#[derive(/* the compiler will tell you */)]
pub struct SuspectId(u32);      // note: no `pub` on the field

impl SuspectId {
    pub fn new(id: u32) -> Self { /* ... */ }
    pub fn get(self) -> u32 { /* ... */ }
}

impl fmt::Display for SuspectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // "suspect #3"
    }
}

impl From<u32> for SuspectId {
    fn from(id: u32) -> Self { /* ... */ }
}

// ...and the same four blocks again for FactId.
```

Yes, you will write it twice, near-identically. Notice the duplication and resist the urge to fix
it — a `macro_rules!` newtype macro is the standard cure and it is a Phase 3 topic, not this one.
Two copies is not yet a problem worth a macro.
</details>

---

## Optional, if you finish early

Prove the swap bug is actually dead. Add this doc comment on `SuspectId` and run
`cargo test --doc`:

```rust
/// ```compile_fail
/// use interrogatory_ai_lib::ids::{FactId, SuspectId};
/// let s: SuspectId = FactId::new(3);
/// ```
```

A `compile_fail` doctest passes when the code inside *fails to compile*. It is how you write a
test for a type error — something TypeScript needs a separate tool (`tsd`, `expect-error`) to do.
