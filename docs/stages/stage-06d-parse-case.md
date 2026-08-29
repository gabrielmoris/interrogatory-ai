# Stage 6d — the front door, and an error that needs help

**Test:** `src-tauri/tests/case_parse.rs` — 4 tests
**Run:** `cd src-tauri && cargo test --test case_parse`
**You write:** the body of `parse_case` in `src-tauri/src/case_file.rs`
**Est.** 20–35 min

---

## What you're building, and why

You have both halves now: text → `RawCase` (6a), and `RawCase` → checked `Case` (6b, 6c). This is the
one function that joins them, and it is four lines.

One of those lines does not compile, on purpose. It is the reason this is its own stage.

---

## Refresher

- **`?`** stops the function and returns the error — Stage 5 §4.
- **Closures**, `|e| ...`, are arrow functions — Stage 4.
- **`AppError::Parse { path, message }`** carries a `String` because the sentence came from somebody
  else's parser. Our failures are structured, foreign diagnostics are text — Stage 5.
- **`try_into()`** came free with your `TryFrom` — 6b.

---

## 1. The line that does not compile

```rust
pub fn parse_case(text: &str, path: &str) -> AppResult<Case> {
    let raw: RawCase = toml::from_str(text)?;   // <- does not compile
    raw.try_into()
}
```

`toml::from_str` is the parser: give it text, tell it what type you want, and it hands back
`Result<RawCase, toml::de::Error>`. Note `let raw: RawCase` — that annotation is how it knows what to
build. Take it away and the compiler asks you for it.

Now the `?`:

```
error[E0277]: `?` couldn't convert the error to `AppError`
   |
   |     let raw: RawCase = toml::from_str(text)?;
   |                        --------------------^ the trait `From<toml::de::Error>` is not
   |                                              implemented for `AppError`
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value
           using the `From` trait
```

**Read the last note twice.** It fills in something Stage 5 mentioned and never showed you. `?` does
not only return early. On the way out it calls `From::from` on the error, to turn *the failure you
got* into *the failure your function promised*.

In Stage 5 both sides were `AppError`, so that conversion was invisible — `From` from a type to
itself does nothing. Here the two sides differ, there is no conversion between them, and it stops.

## 2. `.map_err` — `.map` for the failure side

`.map` on an `Option` changes the value inside and leaves `None` alone. `.map_err` on a `Result` does
the mirror image: leaves `Ok` alone, runs a closure on the error.

Parcel depot:

```rust
let raw: RawManifest = toml::from_str(text).map_err(|e| DepotError::Parse {
    path: path.to_string(),
    message: e.to_string(),
})?;
```

`e` is the TOML parser's own error and `e.to_string()` is its sentence, which `Parse` carries as
text. Then `?` on the outside, which now has an `AppError` to work with and is happy.

## 3. Why not write the `From` and keep the bare `?`

Because `AppError::Parse` has a `path` field, and the TOML parser does not know which file it was
reading — it was handed a string. **Only the caller knows the path, so only the caller can build that
error.** `.map_err` is where the caller says what it knows.

That is also why `path` is a parameter rather than something the function works out. Nothing here
touches the filesystem: the text arrives as a `&str` and `path` is a label for the error message.
Reading real files is Stage 8.

**Last line.** `raw.try_into()` — 6b's conversion, called from the other end. Its return type is
already `AppResult<Case>`, which is what `parse_case` returns, so it is the value of the function. No
`?`, no `Ok(...)`.

---

## Your task

Make `src-tauri/tests/case_parse.rs` pass. Uncomment the `parse_case` you stubbed in 6b:

```rust
/// Parse the text of a case file into a checked `Case`.
/// `path` is not read from — it only names the file in error messages.
pub fn parse_case(text: &str, path: &str) -> AppResult<Case> {
    todo!()
}
```

**Write it with the bare `?` first and read `E0277` before you fix it.** That error is the stage.

---

## Checkpoints

| After | `cargo test --test case_parse` |
|---|---|
| the bare `?` | does not compile — `E0277` |
| `.map_err` added, `raw.try_into()` as the last line | 4 passed |

Then `cargo test` — all eight files, Stages 1 through 6 — `cargo fmt`, and
`cargo clippy --all-targets -- -D warnings`. That closes Stage 6.

---

## Rules

1. Do not edit `tests/case_parse.rs`.
2. Write the bare `?` first and read the error. Rule 1 of this stage exists so you meet it.
3. Do not add `impl From<toml::de::Error> for AppError`. Section 3 says why.
4. No `unwrap()` / `expect()` in `src/`.
5. Say **"ready"** and I review — this is the review for 6d and the close-out of Stage 6.

---

## Hints — parcel depot, translate it yourself

<details>
<summary><b>Hint 1 — the shape of the fix</b></summary>

The failing line becomes one statement across four lines: the same call, then `.map_err(...)`, then
`?` at the very end. The `?` does not move — it stays last, and `.map_err` goes in front of it.

</details>

<details>
<summary><b>Hint 2 — what goes in the closure</b></summary>

A closure that takes the parser's error and returns *your* error. Two fields to fill: the path, which
came in as a parameter and is a `&str` you need to own, and the message, which is whatever the parser
said.

</details>

<details>
<summary><b>Hint 3 — nearly all of it</b></summary>

```rust
let raw: RawManifest = toml::from_str(text).map_err(|e| DepotError::Parse {
    path: path.to_string(),
    message: e.to_string(),
})?;
```

`path.to_string()` because `Parse` owns a `String` and you were handed a borrow — Stage 3's rule.

</details>

<details>
<summary><b>Hint 4 — if only the last test fails</b></summary>

`a_file_that_parses_but_does_not_check_out_still_fails` wants `SuspectNotFound`, not `Parse`. If you
get a `Parse` there, you have wrapped the *whole function* in `map_err` instead of just the
`toml::from_str` call. Only the parser's failure gets converted; the conversion's failure is already
an `AppError` and travels untouched.

</details>
