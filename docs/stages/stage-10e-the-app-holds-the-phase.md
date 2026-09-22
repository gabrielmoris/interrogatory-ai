# Stage 10e — the app holds the phase

Test:   `src-tauri/tests/app_phase.rs` — 4 tests
Run:    `cd src-tauri && cargo test --test app_phase`
New:    Rust semantics — changing a value every command shares, through a lock. Recall: `.map_err`.
Est.    30 min

## 0. What you are building, and where

**File:** `src-tauri/src/state.rs` — `AppState` and its `impl`. One variant in `error.rs`.

**What:** `AppState` holds the game's `Phase` beside `cases_dir` for as long as the app runs, and one
method, `begin(suspect)`, calls a suspect in — in 10f, when the player picks one on screen.

**Do this first.** Replace the whole of `state.rs` with:

```rust
use crate::error::AppResult;
use crate::ids::SuspectId;
use crate::transcript::Phase;
use std::path::PathBuf;

/// What the app holds on to for as long as it runs.
pub struct AppState {
    pub cases_dir: PathBuf,
    phase: Phase,
}

impl AppState {
    pub fn new(cases_dir: PathBuf) -> Self {
        Self {
            cases_dir,
            phase: Phase::Briefing,
        }
    }

    /// Calls a suspect in. Only from the briefing.
    pub fn begin(&self, suspect: SuspectId) -> AppResult<()> {
        todo!()
    }
}
```

Run it. Measured: **0 passed; 4 failed**, two warnings. The stage replaces this `todo!()` — twice.

## 1. Step 1 — the body of `begin()`, first try

**What to do.** `AppState` decides nothing here: it asks the phase to call the suspect in, and gives
back the phase's answer. Write this where the `todo!()` is — the whole body, one line:

```rust
        self.phase.begin(suspect)
```

| the piece | what it is |
|---|---|
| `self` | the `AppState` this method was called on |
| `.phase` | its field — the `Phase` value stored inside it |
| `.begin(…)` | calls the method you wrote in 10b **on that value** — the shape of `self.suspects.len()` in `case.rs :: suspect_count()`. A dot is "on the thing I already have"; `::` is "reach into a type", as in `Phase::Briefing` |
| `suspect` | the argument this method was given, passed straight through |
| no `;` | a last line without a semicolon is what the function gives back — and `Phase::begin` already gives back `AppResult<()>`, exactly what this signature promises |

**Predict** (`sure` / `guessing`): does it compile? Run. Measured: ``E0596 cannot borrow
`self.phase` as mutable, as it is behind a `&` reference``, and the help says: `&mut self`. **Take
it**, predict again, run. Measured: eight errors, all in the test file. The last:

```
error[E0524]: two closures require unique access to `app` at the same time
```

**The new thing, part 1 — what was refused.** (Compiler.) `&self` is a shared view — many readers
*or* one writer, Stage 4 — and `begin` writes. Tauri only ever gives a command that shared view:
every command holds the same `AppState`, and some run at once, on other threads — the last test is
one, a double-click. That is why `&mut self` is refused as well.

You have shipped this: the double-submit — two clicks read `isSubmitting` as `false` from one
render, so the form posts twice. Where it stops: in React that gap is only at a render or an
`await`; between threads it is between any two instructions, and two can corrupt one `Vec`.

## 2. Step 2 — the body of `begin()`, through a lock

**What to do.** Put the phase behind a lock. Take the lock, ask the phase to begin as in step 1, and
give back its answer — or an `AppError` if the lock is broken.

**The new thing, part 2: `Mutex`** (*mutual exclusion*): a value behind a lock. `.lock()` waits
until nobody else holds it (runtime) and hands you a *guard*, the key. While you hold it nobody
else gets in; it unlocks itself when dropped, at `begin`'s `}`. Through it you change the phase
with only `&self` — *interior mutability*: one writer at a time, checked as the program runs.

`.lock()` fails one way: a thread panicked (crashed) holding the key, so the phase may be
half-changed. The lock is *poisoned*; every later `.lock()` is an `Err` — ours, an `AppError`
(`DECISIONS.md`). Block 1 goes in `error.rs` after `InvalidState`; block 2 is all of `state.rs`.

```rust
    #[error("a crash may have left the game half-changed: {message}")]
    Poisoned { message: String },
```

```rust
use crate::error::{AppError, AppResult};
use crate::ids::SuspectId;
use crate::transcript::Phase;
use std::path::PathBuf;
use std::sync::Mutex;

/// What the app holds on to for as long as it runs.
pub struct AppState {
    pub cases_dir: PathBuf,
    phase: Mutex<Phase>,
}

impl AppState {
    pub fn new(cases_dir: PathBuf) -> Self {
        Self {
            cases_dir,
            phase: Mutex::new(Phase::Briefing),
        }
    }

    /// Calls a suspect in. Only from the briefing.
    pub fn begin(&self, suspect: SuspectId) -> AppResult<()> {
        todo!()
    }
}
```

**Your own code with this shape** — `case_file.rs :: parse_case()`:

```rust
    let raw: RawCase = toml::from_str(text).map_err(|e| AppError::Parse {
        path: path.to_string(),
        message: e.to_string(),
    })?;
```

| `parse_case` | line 1 of `begin` |
|---|---|
| `let raw: RawCase = toml::from_str(text)` — a raw case, or a TOML error | `let phase = self.phase.lock()` — the key, or a poison error |
| `AppError::Parse { path: …, message: … }` | `AppError::Poisoned { message: … }` |

Line 2 is step 1's line, with the key where `self.phase` was. **Predict:** `let` or `let mut`? Run.
Measured when right: **4 passed; 0 failed**. Then `cargo fmt`, `cargo clippy --all-targets -- -D
warnings`, `cargo test` (134 tests, seventeen files), and say ready.

## 3. The version that passes every test and is still wrong

```rust
        let mut phase = self.phase.lock().unwrap();
        phase.begin(suspect)
```

All 4 green and clippy clean once `AppError` leaves the import. I ran it — most examples write it.

## 4. If it does not compile — errors I ran (rustc 1.95), and what each means

| the compiler says | what it means |
|---|---|
| ``E0277 … `From<Result<_, AppError>>` is not implemented for `AppError` `` | the closure wrapped its error in `Err(…)`; it hands back the `AppError` itself |

## 5. When it is green — closed-book, in chat, in your own words

1. Why did the compiler refuse your first `begin` — and then `&mut self` too?
2. What would actually have gone wrong if it had allowed `self.phase.begin(suspect)`?
3. A command panics holding the key. What happens next under §3's `.unwrap()`, and under yours?
4. Was your first `begin` really unsafe, or fine and the compiler just could not prove it?
