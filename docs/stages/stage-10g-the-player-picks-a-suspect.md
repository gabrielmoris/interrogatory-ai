# Stage 10g — the player picks a suspect

Test:   `src-tauri/tests/pick_suspect.rs` — 6 tests
Run:    `cd src-tauri && cargo test --test pick_suspect`
Est.    20 min

**Goal.** A new function `begin_interrogation_from` in `ipc.rs`: given a case name and a suspect id,
it calls that suspect into the room — but only if the case really has that suspect.

## Instructions — do them in order

**1. `ids.rs` — let a `SuspectId` be read from JSON.** Add `serde::Deserialize` to the end of the
derive list above `pub struct SuspectId(u32);`. Don't touch `FactId`. After `cargo fmt` it looks like:

```rust
#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Serialize, serde::Deserialize,
)]
pub struct SuspectId(u32);
```

**2. `ipc.rs` — paste the empty function.** At the very end of the file, after the last `}` of
`case_intro`. It is a plain function, not inside any `impl`.

```rust
/// Calls in the suspect the player picked, once the case is known to have them.
pub fn begin_interrogation_from(state: &AppState, slug: &str, suspect: SuspectId) -> AppResult<()> {
    todo!()
}
```

**3. Run the tests.** Expected: `3 passed; 3 failed`.

**4. `ipc.rs` — replace `todo!()` with three lines.** Each one is a line you already wrote, with
one thing changed:

| # | the line does | copy your line from | change |
|---|---|---|---|
| a | loads the case named `slug` | `ipc.rs :: case_intro_from()` — `let case = load_case(cases_dir, slug)?;` | `cases_dir` → `&state.cases_dir` |
| b | stops with an error if the case has no such suspect | `case.rs :: reveal()` — `self.require_suspect(to)?;` | `self` → `case`, `to` → `suspect` |
| c | calls the suspect in, and returns the answer | `state.rs :: begin()` — `phase.begin(suspect)` | `phase` → `state`. Result: `state.begin(suspect)` — the **last line**, no `;`, nothing after it |

**5. Run the tests.** Expected: `6 passed; 0 failed`.

**6. Finish.** `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test`
(expected: 144 tests pass). Say ready.

## If it does not compile

| the compiler says | fix |
|---|---|
| ``expected value, found module `self` `` | there is no `self` here: this function is not inside an `impl`. The `AppState` is the variable `state` from the brackets — call `state.begin(suspect)` |
| ``E0432 unresolved import `…::begin_interrogation_from` `` | instruction 2 is missing |
| ``E0277 … `SuspectId: serde::de::DeserializeOwned` is not satisfied`` | instruction 1 is missing |
| ``E0599 no method named `require_suspect` found for enum `Result` `` | line a lost its `?` |
| ``E0308 mismatched types … expected `&Path`, found `PathBuf` `` | line a lost the `&` before `state.cases_dir` |

## Why — read this after it is green

- Step 1 lets serde turn React's `2` into `SuspectId(2)`. It checks only that it is a whole number.
  Test `any_number_at_all_becomes_a_suspect_id` shows `99` passes too, though the case has no 99.
- So line b is the real check. Without it, a wrong id from React would put a suspect in the room
  who does not exist in the case.
- Line b must come before line c: check first, then change the room.

## When it is green — one question, in chat, from memory

`suspect` already arrives as a `SuspectId`. Why does the function still ask the case about it?
