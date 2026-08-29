# Stage 6a — the file's own vocabulary

**Test:** `src-tauri/tests/case_raw.rs` — 6 tests
**Run:** `cd src-tauri && cargo test --test case_raw`
**You write:** `src-tauri/src/case_file.rs` — three structs, nothing else
**Est.** 25–40 min

> **You have already written this one**, ahead of the brief, and as far as I can tell it is right.
> Run the test and say "ready". The brief stays as the record, because 6b–6d refer back to it.
> (Old Stage 6 was one 657-line brief teaching ten things at once. Now four sittings, same tests.)
>
> **One thing first:** the `parse_case` you stubbed has a bare `?` in it, which does not compile, so
> nothing in the crate builds right now. Comment that function out and the suite runs. Putting it
> back is 6d, and that error is the whole point of that stage — do not fix it early.

---

## What you're building, and why

A case has always been typed by hand inside a test. A real case is a file somebody wrote, and two of
them are on your disk — open `src-tauri/tests/cases/*.toml` now, they are short.

This stage is the first half of the journey only: **file text in, Rust data out.** Nothing here asks
whether the case makes sense. That is 6c.

---

## Refresher

- **`derive`** writes an implementation for you, from the shape of your type — Stage 1.
- **`Serialize`** turned `AppError` into JSON: Rust value in, text out — Stage 5.
- **Attributes stack.** Two `#[…]` lines above the same item, order irrelevant — Stage 5.
- **Field-level `pub`** — Stage 1. These structs are all `pub`, fields included; a raw type has no
  invariant of its own to protect, and the test reads the fields directly.

---

## 1. `Deserialize` is `Serialize` backwards

Same crate, same `derive`, opposite arrow. Text in, Rust value out.

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawManifest {
    pub depot: String,
    pub lockers: Vec<u32>,
}
```

Give that to a parser along with some text and you get a `RawManifest` back. **Field names match the
file's keys**: `pub depot: String` reads `depot = "north"`. If they do not match, serde says the
field is missing.

## 2. `#[serde(default)]` makes a field optional

Most facts in a case file never mention `is_ground_truth_only`. Without a fallback, serde refuses the
whole file over a line that isn't there.

`#[serde(default)]` sits directly above the field it applies to, **inside** the struct:

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

Missing `known_by` now means the empty vector. Missing `is_ground_truth_only` means `false`. Both of
your case files rely on this — `the-lighthouse.toml` has a fact with no `known_by` line at all.

`briefing` deliberately does **not** get one. A case with no briefing is a bug, not a default.

## 3. The raw type speaks the file's vocabulary

`known_by: Vec<u32>` — not `Vec<SuspectId>`.

That looks like a step backwards after Stage 2, where the whole point was to stop passing bare
numbers around. It is not. `SuspectId` means *"an id of a suspect who exists in this case"* — and
whether that is true is exactly what has not been checked yet.

The bare `u32` lives inside `RawCase` and nowhere else in the program. The moment the check passes
(6b and 6c), it becomes a `SuspectId` and stays one.

---

## Your task

Make `src-tauri/tests/case_raw.rs` pass.

**`src-tauri/src/lib.rs`** — `pub mod case_file;`

**New file — `src-tauri/src/case_file.rs`:**

```rust
use serde::Deserialize;

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
```

One command first, from `src-tauri/`: `cargo add toml`. That is the parser for the file format; it
reads TOML text and hands it to serde.

---

## Checkpoints

| After | `cargo test --test case_raw` |
|---|---|
| the three structs, no `#[serde(default)]` anywhere | 3 passed, 3 failed |
| `#[serde(default)]` on `known_by` and `is_ground_truth_only` | 6 passed |

---

## Rules

1. Do not edit `tests/case_raw.rs` or the two `.toml` files. If a test looks wrong, say so — you
   have been right before.
2. No `unwrap()` / `expect()` in `src/`. Tests are exempt.
3. `cargo test`, then `cargo fmt`, then `cargo clippy --all-targets -- -D warnings`.
4. Say **"ready"** and I review.

---

## Hints — parcel depot, not detective case; translate it yourself

<details>
<summary><b>Hint 1 — how many structs, and why three</b></summary>

TOML's `[[facts]]` is an array of tables: many facts, each with the same keys. In Rust that is
`Vec<RawFact>`, which means `RawFact` has to be a type of its own. Same for suspects. So: one struct
for the file, one for the repeating suspect block, one for the repeating fact block.

</details>

<details>
<summary><b>Hint 2 — the top-level struct</b></summary>

```rust
#[derive(Debug, Deserialize)]
pub struct RawManifest {
    pub depot: String,
    pub opened: String,
    pub lockers: Vec<RawLocker>,
}
```

`depot` and `opened` are the plain keys at the top of the file. `lockers` is the `[[lockers]]` array.

</details>

<details>
<summary><b>Hint 3 — which fields get `#[serde(default)]`</b></summary>

Read `the-lighthouse.toml` and ask, for each field: *is there a fact in this file that leaves this
line out?* Exactly two answers are yes. The others must stay required, or a broken file would parse
into a case with an empty title.

</details>

<details>
<summary><b>Hint 4 — the whole shape</b></summary>

```rust
#[derive(Debug, Deserialize)]
pub struct RawLocker {
    pub id: u32,
    pub label: String,
    #[serde(default)]
    pub held_for: Vec<u32>,
    #[serde(default)]
    pub is_sealed: bool,
}
```

Four fields, two of them optional, ids as plain `u32`. Yours is the same shape with different names.

</details>
