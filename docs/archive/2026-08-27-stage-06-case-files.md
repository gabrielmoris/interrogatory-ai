# Stage 6 — case files

**Test:** `src-tauri/tests/case_file.rs` — 16 tests
**Run:** `cd src-tauri && cargo test --test case_file`
**You write:** a new file `src-tauri/src/case_file.rs`, plus three variants in `src-tauri/src/error.rs`
**Est.** 90–120 min

_One leftover from Stage 5 before you start: your three `Case` methods (`require_suspect`,
`require_fact_mut`, `reveal`) and both items in `error.rs` have no doc comments, while everything
else public in `case.rs` does. Add them whenever you like — not a rule, just tidying._

---

## What you are building, and why

Every case so far has been typed out by hand inside a test:

```rust
let mut case = Case::new("The Ledger", "A bookkeeper is dead...");
case.add_suspect(Suspect::new(marta(), "Marta Reyes"));
```

That is not how a real case arrives. A real case is a file somebody wrote:

```toml
[[facts]]
id = 1
statement = "The safe was opened at 21:40."
known_by = [1, 2]
```

Two of those files are already on your disk — open `src-tauri/tests/cases/the-ledger.toml` and
`the-lighthouse.toml` now, they are short.

And here is the problem the whole stage is about. A person writing that file can write
`known_by = [7]` when there is no suspect 7. Or give two suspects the same `id`. Or add a witness who
knows nothing at all, which means the player walks into an interrogation room with someone who
cannot say a single word.

You could load the file and then check it everywhere it gets used. You are going to do the other
thing: check it once, at the door, and make it impossible for an unchecked case to exist anywhere
in the app.

Five short sections. Read one, write that piece, run the test, come back.

---

## Before you start

**One command.** From `src-tauri/`:

```
cargo add toml
```

`toml` is the parser for the file format. It reads TOML text and hands it to serde, the same crate
that turned `AppError` into JSON in Stage 5.

**Declare the new module** in `src/lib.rs`:

```rust
pub mod case;
pub mod case_file;
pub mod difficulty;
pub mod error;
pub mod ids;
```

**Add the three new error variants** to `src/error.rs`, inside the existing enum, below
`InvalidState`. Exact sentences are pinned by the test `the_new_variants_read_like_sentences` —
copy them from there. The names are `DuplicateSuspect { id: SuspectId }`,
`DuplicateFact { id: FactId }` and `SuspectKnowsNothing { id: SuspectId }`.

Nothing else in `error.rs` changes. Adding a variant to an enum you already shipped is a normal
afternoon in Rust; the derives you wrote in Stage 5 pick the new ones up for free.

**Then stub the new file** so everything compiles before anything works:

```rust
// src-tauri/src/case_file.rs

use crate::case::Case;
use crate::error::{AppError, AppResult};

impl TryFrom<RawCase> for Case {
    type Error = AppError;

    fn try_from(raw: RawCase) -> AppResult<Self> {
        todo!()
    }
}

pub fn parse_case(text: &str, path: &str) -> AppResult<Case> {
    todo!()
}
```

That will not compile until `RawCase` exists — section 2 writes it. Once it does,
`cargo test --test case_file` should report **2 passed, 14 failed**. Those two only look at your new
error variants. That is your starting line.

---

## 1. Two types, one road

Here is the idea, in the smallest possible form. A parcel depot loads a manifest file:

```rust
struct RawManifest {          struct Manifest {
    depot: String,                depot: DepotId,
    lockers: Vec<u32>,            lockers: Vec<Locker>,
}                             }
```

`RawManifest` is **what the file said**: strings and numbers, in the file's vocabulary. `Manifest`
is **what the program works with**: real types, and every locker checked to exist.

The rule that makes this worth doing: there is exactly one way to get from the left to the right,
and it can fail. Nobody can build a `Manifest` any other way.

So by the time you are holding a `Manifest`, the checking has already happened. Not "should have
happened" — _has_. A function taking `&Manifest` does not need to re-check anything, and cannot
forget to.

That is the whole pattern. It has a name — **parse, don't validate** — and the difference it names
is small but real:

- _Validate_ is `checkManifest(raw)` returning `true`, and then you carry on holding the same raw
  thing. Nothing in the type says it was checked. The next function checks it again, or forgets to.
- _Parse_ is `Manifest::try_from(raw)` returning a **different type**. The check and the value are
  the same event.

### The TypeScript version

You have written this before, probably with zod:

```ts
const CaseSchema = z.object({ title: z.string(), suspects: z.array(SuspectSchema) })
type Case = z.infer<typeof CaseSchema>

const case = CaseSchema.parse(json)   // throws, or hands back a typed value
```

Same shape: unknown input on one side, a type you trust on the other, one function between them.
Rust's version differs in two ways. It returns a `Result` instead of throwing, so the failure is in
the signature. And the type on the right is one you wrote by hand, with private fields and methods —
`Case` already refuses to let anyone touch `suspects` directly, which zod's inferred type cannot do.

In your app the two types are `RawCase` (new, this stage) and `Case` (already built, Stage 3).

---

## 2. `RawCase` — the file's vocabulary

Here is the depot version, complete:

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawManifest {
    pub depot: String,
    #[serde(default)]
    pub lockers: Vec<u32>,
}
```

**`Deserialize` is `Serialize` backwards.** In Stage 5 you derived `Serialize` on `AppError`: Rust
value in, JSON out. `Deserialize` is the other direction: text in, Rust value out. Same crate, same
`derive`, opposite arrow. Note it is spelled without the "de" in `derive(Deserialize)` — that is
serde's name for the trait, not a typo.

**Field names match the file's keys.** `pub depot: String` reads `depot = "north"` from the file. If
they do not match, serde says the field is missing.

**`#[serde(default)]` makes a field optional.** With it, a file that never mentions `lockers` gets
the empty vector rather than an error. Without it, serde refuses the file. Your two case files rely
on this: `the-lighthouse.toml` has a fact with no `known_by` line and several with no
`is_ground_truth_only` line.

Attributes stack, and the order between them does not matter. This is what it looks like in place —
`#[serde(default)]` sits directly above the field it applies to, inside the struct:

```rust
#[derive(Debug, Deserialize)]
pub struct RawFact {
    pub id: u32,
    pub statement: String,
    #[serde(default)]
    pub known_by: Vec<u32>,
    #[serde(default)]
    pub is_ground_truth_only: bool,
}
```

### Plain numbers, on purpose

`known_by: Vec<u32>` — not `Vec<SuspectId>`.

This looks like a step backwards after Stage 2, where the whole point was to stop passing bare
numbers around. It is not. `RawCase` speaks the _file's_ vocabulary, and the file only has numbers
in it. `SuspectId` means "an id of a suspect who exists in this case" — and whether that is true is
exactly what has not been checked yet.

The bare `u32` lives inside `RawCase` and nowhere else. The moment the check passes, it becomes a
`SuspectId` and stays one for the rest of the program.

You need three of these: `RawCase`, `RawSuspect`, `RawFact`. The fields are the keys in the two
`.toml` files. All of them `pub` — the test reads them directly, and a raw type has no invariant of
its own to protect.

**Checkpoint:** with the three structs written and both stubs still `todo!()`, run
`cargo test --test case_file`. Still **2 passed, 14 failed** — but it compiles now, and
`raw_case_accepts_what_case_rejects` gets past its first three lines before it hits the stub.

---

## 3. `TryFrom` — the one road

You met `From` in Stage 2, on `From<u32> for SuspectId`: a conversion that always works.
`TryFrom` is the same idea for a conversion that can fail.

```rust
impl TryFrom<RawManifest> for Manifest {
    type Error = DepotError;

    fn try_from(raw: RawManifest) -> DepotResult<Self> {
        // ...
    }
}
```

**`type Error = DepotError;` is new.** A trait can require a type from you, not just functions.
`TryFrom` says: "tell me what you fail with, then write `try_from`." Leave the line out and you get

```
error[E0046]: not all trait items implemented, missing: `Error`
```

which is the compiler saying you answered two of the three questions.

**What you get in return.** Implementing `TryFrom` also gives you `try_into()` on the other side,
for free — `raw.try_into()` works the moment `TryFrom` exists, the same way `.into()` appeared in
Stage 2 once you wrote `From`. Section 5 uses it.

**Why a trait rather than just a function called `build_case`?** Because `?` knows about `TryFrom`,
because `try_into()` comes with it, and because "this is the conversion from raw to checked" is a
thing the language already has a name for. Naming it yourself would be a private word for a public
idea.

### One thing to watch in the loops

You will loop over the suspects, then over the facts, then over the suspects again for the last
check. Write the first loop like this:

```rust
for raw_suspect in &raw.suspects {
```

Note the `&`. Without it, the `for` loop **consumes** `raw.suspects` — takes ownership of it — and
the third loop then gets:

```
error[E0382]: borrow of moved value: `raw.suspects`
```

This is Stage 1's move rule in a new costume: a `for` loop over a `Vec` eats the `Vec` unless you
hand it a reference instead. You do not need to own anything here — `Suspect::new` and `Fact::new`
both take `&str` and copy what they need, so borrowing is enough all the way through.

---

## 4. The four checks

The conversion builds a `Case` as it goes and refuses at the first thing that is wrong. Four rules,
each two or three lines.

**1. No suspect id twice.** Before adding a suspect, ask the case whether it already has one with
that id. `case.suspect(id)` is your Stage 4 lookup and answers exactly that. If it does, return
`Err(AppError::DuplicateSuspect { id })`.

**2. No fact id twice.** Same shape. The only lookup you have for facts is `fact_mut`, which needs
`&mut self` — fine, your case is a `let mut` here.

**3. Every id in `known_by` must be a suspect in this case.** You already wrote this check last
stage, and it already returns the right error:

```rust
case.require_suspect(suspect)?;
```

One line. The `?` returns `AppError::SuspectNotFound { id }` from `try_from`, and the test asks for
precisely that. This is the first time a Stage 5 method has been used in anger.

**4. Every suspect must have at least one thing to say.** Not "at least one entry in some
`known_by` list" — one fact they are _allowed to talk about_. Those are different, and the test
`knowing_only_the_solution_counts_as_knowing_nothing` is there to keep them apart: a suspect whose
only fact is the ground-truth solution still has nothing to say.

There is already exactly one function in this codebase that answers "what may this suspect talk
about", and you wrote it in Stage 4:

```rust
if case.suspect_facts(id).next().is_none() {
```

`.next()` pulls the first item out of the iterator. `None` means there was not one. Do this check
last, in its own loop, after every fact has been added — the answer is not final until they are.

**Order is behaviour.** These checks run in the order you write them, and a file can break two rules
at once. The tests pin the order above; that is the same lesson as `reveal` in Stage 5, where which
check came first was visible from the outside.

**Checkpoint:** with `try_from` and `parse_case` (section 5) written but no checks at all, you get
**10 passed, 6 failed**. Add checks 1–3: **14 passed, 2 failed**. Add check 4: **16 passed**.

---

## 5. `parse_case`, and an error that needs help

The last piece is four lines, and one of them teaches the stage's real lesson.

```rust
pub fn parse_case(text: &str, path: &str) -> AppResult<Case> {
    let raw: RawCase = toml::from_str(text)?;   // <- does not compile
    raw.try_into()
}
```

`toml::from_str` is the parser. Give it text, tell it what type you want, and it hands back
`Result<RawCase, toml::de::Error>`. Note `let raw: RawCase` — the type annotation is how it knows
what to build. Take it away and the compiler asks you for it.

Now the interesting part. That `?` does not compile:

```
error[E0277]: `?` couldn't convert the error to `AppError`
   |
   |     let raw: RawCase = toml::from_str(text)?;
   |                        --------------------^ the trait `From<toml::de::Error>` is not
   |                                              implemented for `AppError`
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value
           using the `From` trait
```

Read the last note twice, because it fills in something Stage 5 mentioned and never showed. `?` does
not only return early. On the way out it calls `From::from` on the error, to turn _the failure you
got_ into _the failure your function promised_. In Stage 5 both sides were `AppError`, so the
conversion was invisible. Here they differ, and there is no conversion, so it stops.

**The fix is `.map_err`.** It is `.map` for the failure side of a `Result`: leave `Ok` alone, run a
closure on the error.

```rust
let raw: RawCase = toml::from_str(text).map_err(|e| AppError::Parse {
    path: path.to_string(),
    message: e.to_string(),
})?;
```

`|e| ...` is the closure — an arrow function, from Stage 4. `e` is the TOML parser's own error, and
`e.to_string()` is its sentence, which `Parse` carries as text.

**Why not write `From<toml::de::Error> for AppError` and keep the bare `?`?** Because
`AppError::Parse` has a `path` field, and the TOML parser does not know which file it was reading —
it was handed a string. Only the caller knows the path, so only the caller can build that error.
`.map_err` is where the caller says what it knows.

That is also why `path` is a parameter of `parse_case` rather than something it works out. Nothing
in this function touches the filesystem. Reading the file is Stage 8; today the text arrives as a
`&str` and `path` is just a label to put in the error message.

**Last line.** `raw.try_into()` — the conversion from section 3, called from the other end. Its
return type is already `AppResult<Case>`, which is what `parse_case` returns, so it is the value of
the function. No `?`, no `Ok(...)`.

**Checkpoint:** `cargo test --test case_file` → **16 passed**. Then `cargo test` (all six files —
Stages 1–5 must still be green), `cargo fmt`, and
`cargo clippy --all-targets -- -D warnings`.

---

## Your task

Make `src-tauri/tests/case_file.rs` pass. Shapes given, bodies are yours.

**`src-tauri/src/error.rs`** — three new variants in the existing enum, sentences copied from the
test.

**`src-tauri/src/lib.rs`** — `pub mod case_file;`

**New file — `src-tauri/src/case_file.rs`:**

```rust
use serde::Deserialize;

use crate::case::{Case, Fact, Suspect};
use crate::error::{AppError, AppResult};
use crate::ids::{FactId, SuspectId};

/// A case exactly as it was written in the file: numbers, strings, lists.
/// Nothing here has been checked against anything.
#[derive(Debug, Deserialize)]
pub struct RawCase {
    // title, briefing, suspects, facts
}

#[derive(Debug, Deserialize)]
pub struct RawSuspect {
    // id, name
}

#[derive(Debug, Deserialize)]
pub struct RawFact {
    // id, statement, known_by, is_ground_truth_only
}

/// The only road from a file to a playable case.
impl TryFrom<RawCase> for Case {
    type Error = AppError;

    fn try_from(raw: RawCase) -> AppResult<Self> {
        todo!()
    }
}

/// Parse the text of a case file into a checked `Case`.
/// `path` is not read from — it only names the file in error messages.
pub fn parse_case(text: &str, path: &str) -> AppResult<Case> {
    todo!()
}
```

---

## Rules

1. Do not edit `tests/case_file.rs` or the two `.toml` files. If a test looks wrong, say so — you
   have been right before.
2. No `clone()` anywhere in `case_file.rs`. Borrow the raw case and let `new` copy what it needs.
3. Do not add new methods to `Case`. Everything the four checks need already exists.
4. Write `parse_case` with the bare `?` first and read `E0277` before you fix it.
5. No `unwrap()` / `expect()` in `src/`. Tests are exempt.
6. `cargo test` (all six files), then `cargo fmt`, then
   `cargo clippy --all-targets -- -D warnings` must pass.
7. Say **"ready"** and I review.

---

## Three decisions I already made

Reasoning is in `docs/PROGRESS.md` if you want it; you do not need it to do the stage.

- **The file format is TOML with `[[suspects]]` and `[[facts]]` tables, ids as plain integers.**
  The two files on your disk are the format.
- **No filesystem call in this stage.** The test uses `include_str!`, which pastes the file's text
  into the binary at compile time. Reading a real file at run time brings in `std::fs`, the `Io` and
  `CaseNotFound` variants, and Windows path handling — that is Stage 8, and it has exactly one
  caller, which does not exist yet.
- **The `VisibleFact` newtype moved to Stage 7.** An older note in `ROADMAP.md` had it as Stage 6.
  Case files come first because they give every later stage real data to work with.

---

## Hints

Open only what you need, in order. Parcel depot again, not detective case — translate it yourself.

<details>
<summary><b>Hint 1 — the raw types</b></summary>

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawManifest {
    pub depot: String,
    #[serde(default)]
    pub lockers: Vec<RawLocker>,
}

#[derive(Debug, Deserialize)]
pub struct RawLocker {
    pub id: u32,
    pub label: String,
    #[serde(default)]
    pub reserved_for: Vec<u32>,
    #[serde(default)]
    pub out_of_service: bool,
}
```

Reading the matching file:

```toml
depot = "north"

[[lockers]]
id = 4
label = "by the door"
reserved_for = [1, 2]
```

`[[lockers]]` in TOML means "another item in the `lockers` list". Every `[[lockers]]` block becomes
one `RawLocker`.

</details>

<details>
<summary><b>Hint 2 — the shape of the conversion</b></summary>

```rust
impl TryFrom<RawManifest> for Manifest {
    type Error = DepotError;

    fn try_from(raw: RawManifest) -> DepotResult<Self> {
        let mut manifest = Manifest::new(&raw.depot);

        for raw_locker in &raw.lockers {
            let id = LockerId::new(raw_locker.id);
            // ... check, build, add
        }

        // ... the check that can only be made once everything is in

        Ok(manifest)
    }
}
```

Three things worth copying: the `&` in `&raw.lockers`, the `let mut` on the thing being built, and
`Ok(manifest)` on the last line rather than in the middle.

</details>

<details>
<summary><b>Hint 3 — one locker, built and checked</b></summary>

```rust
for raw_locker in &raw.lockers {
    let id = LockerId::new(raw_locker.id);

    if manifest.locker(id).is_some() {
        return Err(DepotError::DuplicateLocker { id });
    }

    let mut locker = Locker::new(id, &raw_locker.label);
    locker.out_of_service = raw_locker.out_of_service;

    for &courier in &raw_locker.reserved_for {
        let courier = CourierId::new(courier);
        manifest.require_courier(courier)?;
        locker.reserve_for(courier);
    }

    manifest.add_locker(locker);
}
```

Two details you would otherwise hunt for:

- `for &courier in &raw_locker.reserved_for` — the `&` in the _pattern_ unwraps the reference, so
  `courier` is a `u32` and not a `&u32`. Without it you would write `CourierId::new(*courier)`.
  Both are fine.
- `manifest.require_courier(courier)?;` with nothing on the left is the Stage 5 move: run the
check, drop what comes back.
</details>

<details>
<summary><b>Hint 4 — the last check, and the parser</b></summary>

```rust
for raw_locker in &raw.lockers {
    let id = LockerId::new(raw_locker.id);
    if manifest.usable_slots(id).next().is_none() {
        return Err(DepotError::LockerUnusable { id });
    }
}
```

Its own loop, after the first one has finished. Inside the first loop the answer would be wrong for
every locker but the last.

```rust
pub fn parse_manifest(text: &str, path: &str) -> DepotResult<Manifest> {
    let raw: RawManifest = toml::from_str(text).map_err(|e| DepotError::Parse {
        path: path.to_string(),
        message: e.to_string(),
    })?;
    raw.try_into()
}
```

If `try_into()` gives you "type annotations needed", it is because you wrote it somewhere the
compiler cannot see what you want. On the last line of a function returning `DepotResult<Manifest>`,
it can.

</details>

<details>
<summary><b>Hint 5 — near-complete shape, your types</b></summary>

```rust
// src-tauri/src/case_file.rs

#[derive(Debug, Deserialize)]
pub struct RawCase {
    pub title: String,
    pub briefing: String,
    #[serde(default)]
    pub suspects: Vec<RawSuspect>,
    #[serde(default)]
    pub facts: Vec<RawFact>,
}

/* RawSuspect { id: u32, name: String }
   RawFact { id, statement, known_by: Vec<u32>, is_ground_truth_only: bool } */

impl TryFrom<RawCase> for Case {
    type Error = AppError;

    fn try_from(raw: RawCase) -> AppResult<Self> {
        let mut case = Case::new(&raw.title, &raw.briefing);

        for raw_suspect in &raw.suspects {
            /* duplicate check, then case.add_suspect(Suspect::new(id, &raw_suspect.name)) */
        }

        for raw_fact in &raw.facts {
            /* duplicate check; build the Fact; set is_ground_truth_only;
               for each id in known_by: require_suspect(..)? then fact.reveal_to(..);
               then case.add_fact(fact) */
        }

        for raw_suspect in &raw.suspects {
            /* suspect_facts(id).next().is_none() -> SuspectKnowsNothing */
        }

        Ok(case)
    }
}
```

`Fact` has public fields, so `is_ground_truth_only` is set by assignment after `Fact::new`, not by
a constructor argument.

</details>

---

## If you finish early

Add `#[serde(deny_unknown_fields)]` above `RawCase` and put a line like `dificulty = "hard"` in one
of the case files. Predict what happens before you run it: today a misspelled key is silently
ignored, and a case file with a typo in it loads as if the line were not there. Then decide whether
you want that attribute on all three raw structs, and tell me why — this one is genuinely a
trade-off, not a right answer.
