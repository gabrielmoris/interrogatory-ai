# Stage 8 — where the case actually comes from

```
Test:    src-tauri/tests/storage.rs — 9 tests
Run:     cd src-tauri && cargo test --test storage
Writes:  src-tauri/src/storage.rs :: case_path(), is_slug(), load_case()
         src-tauri/src/lib.rs     :: one module line
Assumes: parse_case (6d) · .map_err (6d) · ? early return (5) · match as an expression (1)
         · closures and iterator methods (3) · AppError::{Io, CaseNotFound} (5) · pub mod (1)
Est.     50 min
```

---

## 0. What this has to do, in ordinary words

The app will be asked for a case by name — "the-ledger" — and has to come back with the case. So:
turn the name into a place on disk, read what is there, hand the text to the reader you already
wrote. Three things can go wrong and they are not the same thing: there is no such case; the file is
there but cannot be read; the file reads fine but is not a case. And the name arrives from outside
the app, so before it goes anywhere near a folder, check that it is a plain name and not a route to
somewhere else.

---

## 1. The new ideas

### 1a. Headline — a filename is not a string

**Plain English.** A location on disk is not text the way a sentence is text. Rust gives it its own
pair of types, and it is exactly the pair you already know for text: one that owns and can be built
up, one that is borrowed and only looked at. `PathBuf` is to `Path` what `String` is to `&str` —
Stage 3, same rule, different subject. Parameters take `&Path`; a function that *builds* a location
returns `PathBuf`.

**TypeScript** — one type for both, and nothing separates a location from any other string:

```ts
const p = join(dir, `${slug}.toml`)   // a string, indistinguishable from a sentence
```

**Rust:**

```rust
fn manifest_path(depot: &Path, van: &str) -> PathBuf {
    depot.join(format!("{van}.csv"))
}
```

`.join` is the one that glues a name onto a folder. It is not string concatenation: it knows what a
separator is on this machine, which is why the same code is correct on your Windows box and on a
Linux CI runner.

**Where the analogy breaks.** A filename is not guaranteed to be valid text at all — the operating
system will accept bytes that are not. So a `Path` cannot simply be printed or handed to something
expecting a `String`. You ask it for a printable view with `.display()`, and `.display().to_string()`
is how you get a `String` out of one for an error message.

### 1b. Supporting — one read, and more than one way for it to fail

`std::fs::read_to_string(&path)` hands back `Result<String, std::io::Error>`. That error is not ours,
so `?` cannot carry it out alone — `.map_err` first, Stage 6d §2.

The new part: `std::io::Error` answers `.kind()` with a short enum saying **which** kind of failure.
`ErrorKind::NotFound` means nothing is there. That one is worth telling apart; the rest — permission,
a directory where a file should be, bytes that are not text — are one bucket.

```rust
let text = fs::read_to_string(&path).map_err(|e| match e.kind() {
    ErrorKind::NotFound => /* one error */,
    _ => /* the other */,
})?;
```

---

## 2. Refresher

- **`.map_err` wraps only the failing call**, not the whole function — Stage 6d.
- **`AppError::Io { path, message }` holds text, never a foreign error** — Stage 5, and
  `DECISIONS.md` 2026-08-25.
- **A file is invisible until `lib.rs` declares it** — Stage 1.
- **`.all()`** on an iterator is `Array.prototype.every`.

---

## 3. Tasks

### 3.1 The logic, step by step

1. Create `src-tauri/src/storage.rs`, declare it in `lib.rs`, and put the three signatures in with
   `todo!()` bodies.
2. In `storage.rs :: case_path()` — glue the folder, the name and the ending `.toml` into one
   location. It only builds a location. It does not go looking.
3. In `storage.rs :: is_slug()` — true only when the name is not empty and every character in it is
   an ASCII lowercase letter, a digit, or `-`. Then in `storage.rs :: load_case()`: if the answer is
   false, stop there and report that there is no such case.
4. In `storage.rs :: load_case()` — ask `case_path` where to look, read the whole file as text, and
   if the read fails report the `Io` failure carrying the printable location and the system's own
   message. Then hand the text and that same printable location to `parse_case`, and return whatever
   it says.
5. In `storage.rs :: load_case()` — go back to the failed read and split it in two: nothing there is
   "no such case", everything else stays `Io`.

### 3.2 Scaffolding

```rust
//! Reading case files from disk.
//!
//! Shell module: the filesystem lives here and nowhere else.

use crate::case::Case;
use crate::case_file::parse_case;
use crate::error::{AppError, AppResult};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

/// Where the case called `slug` would live inside `dir`. Says nothing about
/// whether anything is there.
pub fn case_path(dir: &Path, slug: &str) -> PathBuf {
    todo!()
}

/// A case slug is a plain name: ASCII lowercase letters, digits and `-`.
fn is_slug(slug: &str) -> bool {
    todo!()
}

/// Reads the case called `slug` out of `dir` and parses it.
///
/// `CaseNotFound` if the slug is not a plain name or nothing is there, `Io` if
/// the file cannot be read, and whatever `parse_case` says otherwise.
pub fn load_case(dir: &Path, slug: &str) -> AppResult<Case> {
    todo!()
}
```

`lib.rs` as it stands, and the line that joins it:

```rust
pub mod case;
pub mod case_file;
pub mod difficulty;
pub mod error;
pub mod ids;
pub mod storage;     // <- add this one. Alphabetical, like the rest.
```

### 3.3 Checkpoints — measured

| After | `cargo test --test storage` |
|---|---|
| step 1 — three stubs, module declared | 0 passed, 9 failed |
| step 2 — `case_path` written | 1 passed, 8 failed |
| step 3 — `is_slug` and the guard | 2 passed, 7 failed |
| step 4 — the read, every failure an `Io` | 8 passed, 1 failed |
| step 5 — the missing file told apart | **9 passed** |

Until step 4 the compiler will tell you `dir` is unused. Expected; it goes away at step 4.

### 3.4 Cold call

Step 4's last line calls something you wrote in Stage 6d whose second parameter was "a label, not a
location". Go and read its signature before you write the call.

---

## 4. Rules

1. Do not edit `tests/storage.rs`.
2. `case_file.rs` gains no `use std::fs`. The read lives in `storage.rs` and only there.
3. No `unwrap()` / `expect()`. A shell module is where the outside world fails; it is the last place
   for one.
4. The slug check runs before any location is built. A rejected slug must not reach the disk.
5. `cargo test`, then `cargo fmt`, then `cargo clippy --all-targets -- -D warnings`.
6. Say **"ready"** and I review.

---

## 5. Hints — parcel depot, translate it yourself

<details>
<summary><b>Hint 1 — where each piece goes</b></summary>

One new file, `src-tauri/src/storage.rs`, three functions in the order they are listed in §3.2. One
line added to `lib.rs`. Nothing in `case_file.rs` changes, and nothing in `case.rs`.

</details>

<details>
<summary><b>Hint 2 — the question each body asks</b></summary>

`case_path`: "where would this live?" One method call on `dir`, with a `format!` inside it.
`is_slug`: "is every character in this name one of the ones I allow?" — and separately, "is there
anything in it at all?"
`load_case`: four questions in order — is this a plain name, where does it live, did the read work,
what does the parser say. Each is one statement.

</details>

<details>
<summary><b>Hint 3 — the shape, names blanked</b></summary>

```rust
fn manifest_path(depot: &Path, van: &str) -> PathBuf {
    depot.____(format!("{van}.csv"))
}

fn is_van(van: &str) -> bool {
    !van.is_empty()
        && van.chars().____(|c| c.is_ascii_lowercase() || ____ || c == '-')
}
```

</details>

<details>
<summary><b>Hint 4 — the depot's load, in full</b></summary>

```rust
pub fn load_manifest(depot: &Path, van: &str) -> AppResult<Manifest> {
    if !is_van(van) {
        return Err(AppError::VanNotFound { van: van.to_string() });
    }

    let path = manifest_path(depot, van);

    let text = fs::read_to_string(&path).map_err(|e| match e.kind() {
        ErrorKind::NotFound => AppError::VanNotFound { van: van.to_string() },
        _ => AppError::Io {
            path: path.display().to_string(),
            message: e.to_string(),
        },
    })?;

    parse_manifest(&text, &path.display().to_string())
}
```

Two `.display().to_string()` calls, not one stored in a variable and reused — the closure would take
ownership of it and the last line would find it gone. Stage 3's `E0382`, in a new place.

</details>
