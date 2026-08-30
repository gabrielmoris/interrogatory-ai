# Stage 6b — the one road

**Test:** `src-tauri/tests/case_convert.rs` — 6 tests
**Run:** `cd src-tauri && cargo test --test case_convert`
**You write:** the body of `impl TryFrom<RawCase> for Case` in `src-tauri/src/case_file.rs`
**Est.** 40–55 min

---

## What you're building, and why

You have a `RawCase`: what the file said. You need a `Case`: what the game plays. This stage is the
trip between them, and what makes it worth doing is that there is **exactly one road** — nobody can
arrive at a `Case` any other way. No checking yet; every file here is a good one, and 6c adds the
checks to the same function.

---

## Refresher

- **`From<u32> for SuspectId`** — a conversion that always works. You wrote it in Stage 2, and it
  gave you `.into()` for free.
- **`impl Trait for Type`** — Stage 2.
- **`Suspect::new(id, &str)` and `Fact::new(id, &str)`** take a borrowed string and copy what they
  need — Stage 3. Nothing here has to own the raw case.
- **`fact.reveal_to(suspect)`** adds one suspect to a fact's `known_by` — Stage 3.
- **`E0382`, use of a moved value** — Stage 3. It comes back in section 3, in a new costume.

---

## 1. `TryFrom` is `From` for conversions that can fail

Parcel depot. A manifest file is loaded, and it might not describe a real depot:
```rust
impl TryFrom<RawManifest> for Manifest {
    type Error = DepotError;

    fn try_from(raw: RawManifest) -> DepotResult<Self> {
        // build it, or give a reason you could not
    }
}
```

Same idea as `From`, one difference: the return type is a `Result`, so failing is allowed and is
visible in the signature. You also get `try_into()` on the other side for free — `raw.try_into()`
works the moment `TryFrom` exists, exactly as `.into()` did in Stage 2. One test uses it.

**Why a trait rather than a function called `build_case`?** Because `try_into()` comes with it, and
because "the conversion from raw to checked" is a thing the language already has a name for.

## 2. `type Error = …` — a trait can ask you for a type

This line is the new thing:

```rust
type Error = DepotError;
```

Every trait so far asked you for **functions**: `Display` wanted `fmt`, `From` wanted `from`.
`TryFrom` wants a function *and a type* — "tell me what you fail with, then write `try_from`."
Leave it out and you get:

```
error[E0046]: not all trait items implemented, missing: `Error`
```

which is the compiler saying you answered one of the two questions.

The TypeScript picture is a generic parameter you fill in: `interface TryFrom<E>` used as
`TryFrom<DepotError>`. Rust puts it inside the implementation instead, because there is only ever one
answer per type pair.

## 3. Writing the loops

You loop over the suspects, then over the facts. Write the first one like this:

```rust
for raw_suspect in &raw.suspects {
```

**Note the `&`.** Without it the loop *consumes* `raw.suspects` — takes ownership — and the next
loop that touches it gets:

```
error[E0382]: borrow of moved value: `raw.suspects`
```

This is Stage 3's move rule in a new costume: a `for` loop over a `Vec` eats the `Vec` unless you
hand it a reference. You do not need to own anything here — `Suspect::new` and `Fact::new` both take
`&str` and copy what they need, so borrowing is enough all the way through.

The rest is assembly: build each `Fact`, turn each `u32` in `known_by` into a `SuspectId` and reveal
it, carry the ground-truth flag across, add it to the case.

---

## Your task

Make `src-tauri/tests/case_convert.rs` pass. The shape is already in your file:

```rust
impl TryFrom<RawCase> for Case {
    type Error = AppError;

    fn try_from(raw: RawCase) -> AppResult<Self> {
        todo!()
    }
}
```

Your `parse_case` does not compile yet — that is 6d, and it is meant to be broken. Comment it out
until then so this stage's test can run.

---

## Checkpoints

| After | `cargo test --test case_convert` |
|---|---|
| `todo!()` still in place | 0 passed, 6 failed |
| title, briefing and the suspects loop | 2 passed, 4 failed |
| the facts loop, without `known_by` or the flag | 3 passed, 3 failed |
| `reveal_to` for each id in `known_by` | 3 passed, 3 failed |
| the `is_ground_truth_only` flag carried across | 6 passed |

The count does not move on the `known_by` step: the three remaining tests each check `known_by`
**and** the flag, so all three flip together on the last line. Measured, not guessed — the first
version of this table said 5 and was wrong.

---

## Rules

1. Do not edit `tests/case_convert.rs` or the two `.toml` files.
2. No `clone()` in `case_file.rs`. Borrow the raw case and let `new` copy what it needs.
3. Do not add new methods to `Case`. Everything you need exists.
4. Write the first loop **without** the `&` and read `E0382` before you fix it.
5. No `unwrap()` / `expect()` in `src/`.
6. `cargo test`, then `cargo fmt`, then `cargo clippy --all-targets -- -D warnings`.
7. Say **"ready"** and I review.

---

## Hints — parcel depot again, translate it yourself

<details>
<summary><b>Hint 1 — where to start</b></summary>

The first line of the body builds an empty thing to fill in, and the last line hands it back wrapped
in `Ok`. Everything between is loops.

```rust
let mut manifest = Manifest::new(&raw.depot, &raw.opened);
// ... loops ...
Ok(manifest)
```

Note `let mut` — you are going to push into it.

</details>

<details>
<summary><b>Hint 2 — the suspects loop</b></summary>

```rust
for raw_locker in &raw.lockers {
    manifest.add_locker(Locker::new(LockerId::new(raw_locker.id), &raw_locker.label));
}
```

`raw_locker.id` is a `u32`; `LockerId::new` takes one. `&raw_locker.label` is the `&str` that
`Locker::new` wants, borrowed out of the `String` the file gave you.

</details>

<details>
<summary><b>Hint 3 — a fact needs three things done to it</b></summary>

`Fact::new` only takes an id and a statement. The other two fields start empty, so you set them
after you have built it and before you add it to the case:

```rust
let mut parcel = Parcel::new(ParcelId::new(raw_parcel.id), &raw_parcel.label);
for holder in &raw_parcel.held_for {
    parcel.hold_for(HolderId::new(*holder));
}
parcel.is_sealed = raw_parcel.is_sealed;
manifest.add_parcel(parcel);
```

`*holder` — the loop hands you `&u32` because you borrowed the vector, and `HolderId::new` wants a
`u32`. The `*` reads the value out of the borrow.

</details>

<details>
<summary><b>Hint 4 — if the flag test is the only one failing</b></summary>

`is_ground_truth_only` is a public field on `Fact`, not a constructor argument. If you never assign
it, every fact comes out `false`, the solution stays visible, and exactly one test fails.

</details>
