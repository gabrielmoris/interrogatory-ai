# Stage 3 — `Fact`, `Suspect` and `Case`

**Crate:** `src-tauri`
**Test:** `src-tauri/tests/case.rs`
**Run:** `cd src-tauri && cargo test --test case`
**Est.** 60–90 min.

---

## Background — why this task exists

Stages 1 and 2 built types that are secretly just numbers. `Difficulty` is a number the compiler
pretends is four names. `SuspectId` is a `u32` in a costume. Everything you have written so far
fits in a CPU register, copies itself for free, and never touches the heap.

This stage ends that. `Fact` holds a sentence of English. Sentences have no fixed length, so they
cannot live in a register — they live on the heap, and something has to own them. That single
change is what makes this stage the hardest one so far, and it is the last big idea in Rust you
have not met.

**One concept at a time below.** Read section 1, write the `Suspect` struct, run the test. Come
back for section 2. Do not read all seven before you start.

---

### 1. `String` and `&str` — the one that matters

In TypeScript there is one string type. In Rust there are two, and they are not
interchangeable:

| | `String` | `&str` |
|---|---|---|
| What it is | a growable buffer **you own** | a **borrowed view** into someone else's buffer |
| Where the bytes live | the heap | wherever the owner put them |
| Lives as long as | you keep it | the owner keeps it |
| `"hello"` in source | no | **yes** — literals are `&'static str` |

The rule that decides which one you want:

> **Structs own their data. Function parameters borrow it.**

So `Suspect` stores a `String`, and `Suspect::new` accepts a `&str`. The conversion happens once,
inside the constructor. That is the shape of nearly every constructor you will write in Rust.

The test `a_statement_is_owned_not_borrowed` is there to make this concrete:

```rust
let f = {
    let scratch = String::from("The safe was opened at 21:40.");
    Fact::new(FactId::new(1), &scratch)
};   // <- scratch is destroyed here
assert_eq!(f.statement, "...");
```

If `Fact` stored the `&str` you handed it, `f` would now be pointing at freed memory. Rust will not
let you write that — it is the class of bug that use-after-free CVEs are made of. Storing a
`String` means `Fact` made its own copy of the bytes and owes nothing to anyone.

To go from `&str` to `String`: `.to_string()`. (`String::from(s)` and `s.to_owned()` do the same
thing. Pick one and be consistent; `.to_string()` is what this codebase uses.)

**Checkpoint:** write `Suspect` and `Suspect::new`, add `pub mod case;` to `lib.rs`, and run
`cargo test --test case`. Two tests should pass. Then come back.

---

### 2. `Vec<T>` — the array that grows

Stage 1 gave you `[Self; 4]`: four difficulties, known at compile time, stored inline. A case has
however many suspects the case file happens to list, so the length is not known at compile time and
the array is out.

`Vec<T>` is the heap-allocated, growable version. It is `Array<T>` from TypeScript, with one
difference that will bite you: a `Vec` owns its elements. `push` **takes the value**, it does not
copy a pointer to it.

```rust
let mut v: Vec<Suspect> = Vec::new();
v.push(some_suspect);   // some_suspect is now gone from this scope
```

That is the same move you met in Stage 1 as `E0507`, arriving under a new error code. More on it
in section 5.

---

### 3. `HashSet<T>` — because `known_by` is a question, not a list

`known_by` answers exactly one question: *does this suspect know this fact?* A `Vec` would answer
it by scanning, would happily store the same suspect twice, and would make "who knows this" depend
on insertion order for no reason.

`HashSet<SuspectId>` answers it in one hash lookup, cannot hold duplicates, and has no order at
all. This is why Stage 2 made you derive `Hash` and `Eq` on the ids — that was the setup for this
line.

Two methods do everything you need:

```rust
set.insert(value);      // returns bool: true if it was not already there
set.contains(&value);   // note the &
```

`contains` takes a **reference** because in general the value might be expensive to hand over —
imagine `HashSet<String>`, where passing by value would mean giving up ownership just to ask a
question. Your `SuspectId` is `Copy` and would not care, but the signature is fixed by the
standard library, so the `&` is not optional.

---

### 4. `&self`, `&mut self`, `self` — the three receivers

Every method starts by declaring what it does to the value it is called on. Rust makes you say
which:

| Receiver | Means | Use it when |
|---|---|---|
| `&self` | I will **read** this | `is_known_by`, `fact_count`, `facts_known_by` |
| `&mut self` | I will **change** this | `reveal_to`, `add_fact`, `add_suspect` |
| `self` | I **consume** this; it is gone afterwards | `SuspectId::get` in Stage 2 |

TypeScript has no equivalent — every method there can mutate `this` and nothing in the signature
warns you. In Rust the caller sees it too: calling a `&mut self` method requires the variable be
declared `let mut`. The test files are full of `let mut f = ...` for exactly this reason.

The rule the borrow checker enforces underneath: **many readers, or one writer, never both.** You
will not hit that wall in this stage. You will hit it in Stage 4.

---

### 5. Why `Copy` is gone now

Try `#[derive(Clone, Copy)]` on `Fact` and read what you get:

```
error[E0204]: the trait `Copy` cannot be implemented for this type
  |            ^^^^   --------------------- this field does not implement `Copy`
```

`Copy` means "duplicating this is a bit-for-bit memcpy, and both halves are equally valid". A
`Fact` contains a pointer to a heap buffer. Memcpy it and you get two `Fact`s pointing at the same
buffer, both of which will try to free it when they die. That is a double free, so `String` refuses
to be `Copy`, and anything containing a `String` inherits the refusal.

`Clone` is the explicit version: it allocates a second buffer and copies the bytes in. It is not
free, so Rust makes you type `.clone()` and see the cost at the call site.

Consequence you will feel immediately: after `case.add_fact(f)`, the variable `f` is dead. The
test `adding_a_fact_moves_it_into_the_case` has a commented-out line — uncomment it, run the test,
read `E0382: borrow of moved value`, then comment it back. That five-second exercise is worth more
than another paragraph from me.

---

### 6. What is public and what is not

Not uniform, and deliberately so:

- **`Fact` and `Suspect`: all fields `pub`.** They are plain records. There is no invariant a
  `Fact` can violate on its own — any `FactId`, any sentence, any set of suspects is a coherent
  `Fact`. Hiding the fields would buy you nothing and cost you six accessors.
- **`Case`: `title` and `briefing` are `pub`; `suspects` and `facts` are private.** Those two
  collections *do* carry an invariant, starting in Stage 5: every `SuspectId` mentioned in a
  fact's `known_by` must correspond to a suspect that actually exists in the case. A `Case` that
  hands out `&mut Vec<Fact>` cannot promise that. So the only ways in are `add_suspect` and
  `add_fact`, which is where the checking will eventually live.

The test never touches `case.facts`. If you make those fields `pub`, the test still passes and
the stage is still wrong — same trap as Stage 2's `.0`.

---

### 7. `facts_known_by` — the game rule, in four lines

```rust
pub fn facts_known_by(&self, suspect: SuspectId) -> Vec<FactId>
```

Return the ids of every fact this suspect knows **and** that is not marked
`is_ground_truth_only`. That flag marks facts that exist only so the scorer can check the player's
report against them — the actual solution. They must never reach a suspect's context window, which
is the architectural rule this whole project is built around (`docs/ROADMAP.md`, "Ground truth
lives in Rust"). Right now the rule is one `!` in a filter. Later it is enforced by the type
system.

You can write this with a `for` loop and a `Vec` you push into, and that is a perfectly good first
answer. The iterator version is `.iter().filter(...).map(...).collect()`, which is `.filter().map()`
from JavaScript with two Rust-specific wrinkles: `.iter()` is explicit (Rust does not assume you
want to borrow rather than consume), and `.collect()` is explicit (nothing is evaluated until you
ask for the result). Write whichever you can defend. We will talk about the difference in review.

Note that it returns `Vec<FactId>` — **owned ids**, not references into the case. That is a
deliberate sidestep. The version that returns references is Stage 4:

```rust
fn suspect_facts<'a>(&'a self, s: SuspectId) -> impl Iterator<Item = &'a Fact>
```

Do not try to write that today.

---

## Your task

Make `src-tauri/tests/case.rs` compile and pass. Create `src-tauri/src/case.rs` and register it in
`lib.rs`.

**`Suspect`**

| | |
|---|---|
| `pub id: SuspectId` | |
| `pub name: String` | |
| `pub fn new(id: SuspectId, name: &str) -> Self` | `&str` in, `String` stored |

**`Fact`**

| | |
|---|---|
| `pub id: FactId` | |
| `pub statement: String` | |
| `pub known_by: HashSet<SuspectId>` | starts empty |
| `pub is_ground_truth_only: bool` | starts `false` |
| `pub fn new(id: FactId, statement: &str) -> Self` | |
| `pub fn reveal_to(&mut self, suspect: SuspectId)` | |
| `pub fn is_known_by(&self, suspect: SuspectId) -> bool` | |

**`Case`**

| | |
|---|---|
| `pub title: String`, `pub briefing: String` | |
| `suspects: Vec<Suspect>`, `facts: Vec<Fact>` | **private** |
| `pub fn new(title: &str, briefing: &str) -> Self` | both collections start empty |
| `pub fn add_suspect(&mut self, Suspect)` / `add_fact(&mut self, Fact)` | |
| `pub fn suspect_count(&self) -> usize` / `fact_count(&self) -> usize` | |
| `pub fn facts_known_by(&self, SuspectId) -> Vec<FactId>` | section 7 |

All three need `Debug`, `Clone` and `PartialEq`. **Do not paste the derive list in.** Same
instruction as Stage 2, third time: start with none, run, read the *top* error only, add the one
trait it names. And at some point deliberately try `Copy` and read `E0204` — that one is not on the
list, and finding out why is the lesson.

---

## Rules of the loop

1. Do not edit `src-tauri/tests/case.rs`. If a test looks wrong, say so — that is a legitimate move.
2. `Case`'s collections stay private.
3. No lifetimes, no `Option`, no returning references. Get to green and stop.
4. `cargo fmt`, then `cargo clippy --all-targets -- -D warnings` must pass.
5. Say **"ready"** and I review.

---

## Hints

Open only what you need, in order.

<details>
<summary><b>Hint 1 — the struct with an owned string</b></summary>

Named-field struct this time, not a tuple struct. The constructor takes the borrowed form and
stores the owned form:

```rust
// weather, not suspects — translate it yourself
pub struct Station {
    pub id: u32,
    pub label: String,        // owned
}

impl Station {
    pub fn new(id: u32, label: &str) -> Self {   // borrowed
        Self {
            id,                                   // field init shorthand, same as TS { id }
            label: label.to_string(),             // the one conversion
        }
    }
}
```

`Self { id, label: ... }` — `id` alone works because the parameter and the field share a name.
Exactly the object-literal shorthand you already use.

Look up: "Rust String vs &str", "Rust field init shorthand".
</details>

<details>
<summary><b>Hint 2 — empty collections, and the module</b></summary>

```rust
use std::collections::HashSet;

let names: Vec<String> = Vec::new();          // or vec![]
let seen: HashSet<u32> = HashSet::new();
```

Inside a struct literal you do not need the type annotation — the field's declared type tells the
compiler what `Vec::new()` should produce.

And the thing that will cost you ten minutes if you forget it, as it did in Stage 1: a file in
`src/` does not exist until it is declared. `lib.rs` needs `pub mod case;` next to
`pub mod difficulty;`.

Inside `case.rs`, reach for the ids with `use crate::ids::{FactId, SuspectId};` — `crate` is the
root of *this* crate, the equivalent of an absolute import from `src/`.
</details>

<details>
<summary><b>Hint 3 — a method that changes the thing</b></summary>

```rust
pub struct Station {
    pub readings: HashSet<u32>,
}

impl Station {
    pub fn record(&mut self, reading: u32) {
        self.readings.insert(reading);
    }

    pub fn has_recorded(&self, reading: u32) -> bool {
        self.readings.contains(&reading)
    }
}
```

Three things to notice before translating:

- `&mut self` on the writer, `&self` on the reader.
- `insert` returns a `bool` you are ignoring. That is fine and clippy will not complain.
- `contains(&reading)` — the `&` is required, see section 3.

Caller side: `let mut s = Station::new(...)`. Without `mut`, `s.record(1)` is a compile error that
names the variable, not the method.
</details>

<details>
<summary><b>Hint 4 — filtering into a new Vec</b></summary>

The loop version, which is not a worse answer:

```rust
pub fn warm_station_ids(&self) -> Vec<u32> {
    let mut out = Vec::new();
    for station in &self.stations {        // &  — borrow the vec, do not consume it
        if station.is_warm() {
            out.push(station.id);          // id is Copy, so this takes nothing away
        }
    }
    out                                     // no `return`, no semicolon
}
```

`for station in &self.stations` borrows; `for station in self.stations` would try to *move* the
vec out of `self`, and `self` is only borrowed, so that is `E0507` again.

The iterator version, same thing:

```rust
self.stations
    .iter()
    .filter(|s| s.is_warm())
    .map(|s| s.id)
    .collect()
```

`|s| ...` is `s => ...`. The closure in `filter` receives `&&Station` — a reference to the
reference — which is why you can call methods on it without extra syntax but would need `**s` to
get at the value itself. Do not fight it; method calls auto-dereference.
</details>

<details>
<summary><b>Hint 5 — near-complete shape</b></summary>

```rust
// src-tauri/src/case.rs

use std::collections::HashSet;

use crate::ids::{FactId, SuspectId};

/// One person the player can interrogate.
#[derive(/* let the compiler ask */)]
pub struct Suspect {
    pub id: SuspectId,
    pub name: String,
}

impl Suspect {
    pub fn new(id: SuspectId, name: &str) -> Self { /* ... */ }
}

/// One true statement about the crime, plus who is allowed to know it.
#[derive(/* ... */)]
pub struct Fact {
    pub id: FactId,
    pub statement: String,
    pub known_by: HashSet<SuspectId>,
    pub is_ground_truth_only: bool,
}

impl Fact {
    pub fn new(id: FactId, statement: &str) -> Self { /* known_by empty, flag false */ }
    pub fn reveal_to(&mut self, suspect: SuspectId) { /* ... */ }
    pub fn is_known_by(&self, suspect: SuspectId) -> bool { /* ... */ }
}

/// One playable case: the briefing, the cast, and the ground truth.
#[derive(/* ... */)]
pub struct Case {
    pub title: String,
    pub briefing: String,
    suspects: Vec<Suspect>,     // no pub
    facts: Vec<Fact>,           // no pub
}

impl Case {
    pub fn new(title: &str, briefing: &str) -> Self { /* ... */ }
    pub fn add_suspect(&mut self, suspect: Suspect) { /* ... */ }
    pub fn add_fact(&mut self, fact: Fact) { /* ... */ }
    pub fn suspect_count(&self) -> usize { /* ... */ }
    pub fn fact_count(&self) -> usize { /* ... */ }
    pub fn facts_known_by(&self, suspect: SuspectId) -> Vec<FactId> { /* ... */ }
}
```
</details>

---

## Optional, if you finish early

Write the first real case as Rust code — a `fn the_ledger() -> Case` in a `#[cfg(test)]` module
inside `case.rs`, with three suspects, six or seven facts, overlapping `known_by` sets, and one
`is_ground_truth_only` fact that is the actual solution. Nothing depends on it yet, but Stage 5
turns it into a TOML file and it is much easier to design the file format once you have felt what
the data actually looks like.

Watch for the moment you want to write `case.facts[2].reveal_to(...)` and cannot, because the
field is private. Note down what method you wished existed; that is real API design feedback and
we will discuss it in review.
