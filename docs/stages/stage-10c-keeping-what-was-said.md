# Stage 10c — keeping what was said

Test:   `src-tauri/tests/phase_record.rs` — 5 tests
Run:    `cd src-tauri && cargo test --test phase_record`
Write:  `src/transcript.rs` :: `Phase::record()`
New:    with `&mut self`, the list you name in a `match` line can be added to
Est.    15 min from where you are now

Rewritten 2026-09-20. The first version made you read a parcel-depot example and translate it. It is
gone. Everything this task needs is printed below, taken from your own files. You do not have to open
another file to do this stage.

## 0. What this does

Someone speaks in the interrogation room, and that line is kept: who said it and what they said, at
the end of the list, in the order it was said. During the briefing and during the report there is no
room and no list, so the answer there is the same refusal you already built for the other two moves.

## 1. Your own code, gathered here

**a. `transcript.rs :: turn_count()`** — the line that asks which phase this is:

```rust
    pub fn turn_count(&self) -> usize {
        match self {
            Phase::Interrogating { turns, .. } => turns.len(),
            _ => 0,
        }
    }
```

- `match self {` — look at which phase this is.
- the next line — if it is the interrogation, call the list inside it `turns` (`..` means "and ignore
  the other field"), then hand back how long it is.
- `_ => 0,` — any other phase.

**b. `transcript.rs`** — what one spoken line is:

```rust
pub struct Turn {
    pub speaker: Speaker,
    pub text: String,
}
```

`Speaker` is your own enum, `Detective` or `Suspect`. `text` is a `String`, so a `Turn` owns its text.

**c. `case.rs :: add_suspect()`** — how you put one item at the end of a list:

```rust
        self.suspects.push(suspect);
```

**d. `case.rs :: Case::new()`** — how you turn borrowed text into text a struct can own:

```rust
            title: title.to_string(),
```

## 2. The one new thing in this stage

`turn_count` and `record` write the same line, `Phase::Interrogating { turns, .. }`. What you may do
with `turns` afterwards is not the same:

| the method's first parameter | `turns` is | you may write |
|---|---|---|
| `turn_count(&self)` | lent for reading | `turns.len()` |
| `record(&mut self)` | lent for changing | `turns.len()` **and** `turns.push(…)` |

That is the whole lesson of this stage. The `&mut` in `record`'s signature is what makes adding a
line legal. In a `&self` method the compiler refuses it: `` error[E0596]: cannot borrow `*turns` as
mutable, as it is behind a `&` reference ``.

## 3. What to do

**Step 1 — already done.** Your `record` now reads:

```rust
            Phase::Interrogating { suspect, turns } => Ok(()),
```

and `cargo test --test phase_record` gives `2 passed; 3 failed`. The 2 that pass are the refusals,
handled by your `_` arm. The 3 that fail expect a spoken line to be kept — that is step 2.

One small fix while you are on that line: you named `suspect` and never use it, which `clippy`
counts as an error. Write `..` in its place, as in §1a.

**Step 2 — keep the line.** Make that arm a block holding two things: first one statement that adds
a `Turn` to the end of `turns`, then `Ok(())` as the arm's answer.

```rust
    pub fn record(&mut self, speaker: Speaker, text: &str) -> AppResult<()> {
        match self {
            Phase::Interrogating { turns, .. } => {
                // ← your statement goes here: put a Turn at the end of turns. §1c is the shape.
                Ok(())
            }
            _ => Err(self.refusal("record a line")),
        }
    }
```

The `Turn` you add has two fields, and both values are in the signature on the first line above:

- `speaker` — the `speaker` this function was given, as it is.
- `text` — the `text` this function was given, made owned. §1d does exactly that.

Run `cargo test --test phase_record` again. Measured: `5 passed; 0 failed`.

**Then:** `cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` — 124 tests across
fifteen files — and say ready.

## 4. If it does not compile — errors I ran, and what each one means

| the compiler says | what it means |
|---|---|
| ``expected `;`, found `Ok` `` | the statement above `Ok(())` is unfinished |
| `unused variable: turns` | the arm names the list, and the statement is still missing |
| ``cannot borrow `*turns` as mutable`` | that method says `&self`; `record` takes `&mut self` |
| ``expected `Result<(), AppError>`, found `()` `` | `Ok(())` is missing, or has a `;` after it |
| `unused variable: suspect` (as an error, from clippy) | write `..` instead of naming `suspect` |
