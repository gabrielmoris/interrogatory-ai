# Stage 6c — the four things a file is not allowed to do

**Test:** `src-tauri/tests/case_checks.rs` — 8 tests
**Run:** `cd src-tauri && cargo test --test case_checks`
**You write:** four checks inside the `try_from` body you wrote in 6b
**Est.** 40–55 min

---

## What you're building, and why

A person writing a case file can write anything: the same id twice, a fact known by somebody who is
not in the cast, a witness with nothing to say. Right now your conversion accepts all of it.

You could load the file and then check it everywhere it gets used. You are going to do the other
thing — **check it once, at the door.** After this stage, holding a `Case` is proof the checks
already ran. Not "should have run": *did*.

---

## Refresher

- **`case.suspect(id) -> Option<&Suspect>`** — Stage 4. Answers "is this id in the cast?"
- **`case.fact_mut(id) -> Option<&mut Fact>`** — Stage 4. The only lookup you have for facts.
- **`case.require_suspect(id) -> AppResult<&Suspect>`** — Stage 5. Same question, but it fails with
  `SuspectNotFound { id }` instead of `None`.
- **`case.suspect_facts(id)`** — Stage 4. The one function in the codebase that answers "what may
  this suspect talk about", ground-truth facts already excluded.
- **`?`** stops the function and returns the error — Stage 5 §4.
- **`AppError`'s three new variants** — `DuplicateSuspect`, `DuplicateFact`, `SuspectKnowsNothing`.
  You have already added them; two tests here just confirm they still read and serialize correctly.

---

## 1. The pattern is the same four times

Ask a question, and if the answer is wrong, return an error. Parcel depot:

```rust
if manifest.locker(id).is_some() {
    return Err(DepotError::DuplicateLocker { id });
}
```

Two lines. `return Err(...)` is an early exit with a reason, and it is the long-hand version of what
`?` does for you when you are calling something that already returns a `Result`.

## 2. Your four

**1. No suspect id twice.** Before adding a suspect, ask whether the case already has one with that
id. `case.suspect(id)` answers exactly that.

**2. No fact id twice.** Same shape. Your only fact lookup is `fact_mut`, which needs `&mut self` —
fine, your case is a `let mut` here.

**3. Every id in `known_by` is a suspect in this case.** You already wrote this check last stage, and
it already returns the right error:

```rust
case.require_suspect(suspect)?;
```

One line. The `?` returns `AppError::SuspectNotFound { id }` out of `try_from`, and the test asks for
precisely that. First time a Stage 5 method has been used in anger.

**4. Every suspect has at least one thing to say.** Not "at least one entry in some `known_by` list"
— one fact they are *allowed to talk about*. Those are different, and
`knowing_only_the_solution_counts_as_knowing_nothing` exists to keep them apart: a suspect whose only
fact is the solution still has nothing to say.

```rust
if case.suspect_facts(id).next().is_none() {
```

`.next()` pulls the first item out of the iterator; `None` means there was not one.

**Do check 4 last, in its own loop, after every fact has been added.** The answer is not final until
they are.

## 3. Order is behaviour

These run in the order you write them, and one file can break two rules at once. The tests pin the
order above — same lesson as `reveal` in Stage 5, where which check came first was visible from
outside the function.

---

## Your task

Make `src-tauri/tests/case_checks.rs` pass, by adding checks to the `try_from` body from 6b. No new
function, no new signature.

`tests/case_convert.rs` must stay green — the checks have to reject bad files without rejecting good
ones, and `the_real_case_files_still_pass_every_check` is here to catch that from this side too.

---

## Checkpoints

| After | `cargo test --test case_checks` |
|---|---|
| 6b's conversion, no checks | 2 passed, 6 failed |
| check 1 (duplicate suspect) | 3 passed, 5 failed |
| check 2 (duplicate fact) | 4 passed, 4 failed |
| check 3 (`require_suspect`) | 6 passed, 2 failed |
| check 4 (nothing to say) | 8 passed |

The two that pass from the start are the `AppError` ones. They are your proof the harness runs.

---

## Rules

1. Do not edit `tests/case_checks.rs` or the two `.toml` files.
2. No `clone()` in `case_file.rs`.
3. Do not add new methods to `Case`. All four checks are answerable with what you already wrote.
4. No `unwrap()` / `expect()` in `src/`.
5. `cargo test` — all eight test files — then `cargo fmt`, then
   `cargo clippy --all-targets -- -D warnings`.
6. Say **"ready"** and I review.

---

## Hints — parcel depot, translate it yourself

<details>
<summary><b>Hint 1 — where each check goes</b></summary>

Three of them go *inside* a loop you already have, just before the `add_` call. The fourth is a new
loop, after everything else. If you find yourself checking a suspect before the facts exist, you have
put check 4 in the wrong place.

</details>

<details>
<summary><b>Hint 2 — the duplicate check</b></summary>

```rust
for raw_locker in &raw.lockers {
    let id = LockerId::new(raw_locker.id);
    if manifest.locker(id).is_some() {
        return Err(DepotError::DuplicateLocker { id });
    }
    manifest.add_locker(Locker::new(id, &raw_locker.label));
}
```

Naming the id in a `let` first keeps you from writing `LockerId::new(raw_locker.id)` three times.

</details>

<details>
<summary><b>Hint 3 — the one that is already written</b></summary>

Check 3 is one line and you wrote it in Stage 5. Inside the loop over a parcel's holders, before
`hold_for`:

```rust
manifest.require_holder(holder)?;
```

If you catch yourself writing an `if` and a `return Err` here, stop — the method already does both
halves, and `?` is the whole check.

</details>

<details>
<summary><b>Hint 4 — the last loop</b></summary>

```rust
for raw_locker in &raw.lockers {
    let id = LockerId::new(raw_locker.id);
    if manifest.parcels_in(id).next().is_none() {
        return Err(DepotError::EmptyLocker { id });
    }
}
```

Note it loops over `raw.lockers` again, not over anything inside the manifest — and note it is a
third loop over that same vector, which is why 6b insisted on the `&`.

</details>
