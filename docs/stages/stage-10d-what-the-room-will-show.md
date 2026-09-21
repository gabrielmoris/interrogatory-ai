# Stage 10d — what the room will show

Test:   `src-tauri/tests/phase_transcript.rs` — 6 tests
Run:    `cd src-tauri && cargo test --test phase_transcript`
New:    Rust syntax only — `&[Turn]`. Everything else you have typed before.
Est.    25 min

## 0. What you are building, and where

**File:** `src-tauri/src/transcript.rs`. **Where:** inside `impl Phase`, after `record()`, before
the block's closing `}`.

**What:** two new methods. The interrogation screen will call them to draw the chat.

- `transcript()` — hands back every line said so far in the room, oldest first.
- `last_line()` — hands back only the newest line.

Outside the room there is no conversation, so both hand back "nothing". Neither changes anything.

**Do this first.** Paste these two empty methods where described above:

```rust
    /// Every line said in this room, oldest first. Empty outside the room.
    pub fn transcript(&self) -> &[Turn] {
        todo!()
    }

    /// The line said most recently, or `None` if nothing has been said.
    pub fn last_line(&self) -> Option<&Turn> {
        todo!()
    }
```

Run `cargo test --test phase_transcript`. Measured: **0 passed; 6 failed** — it compiles, and every
test hits a `todo!()`. The whole stage is replacing those two `todo!()`s.

## 1. Step 1 — the body of `transcript()`

**What to do.** Look at which phase this is. In the interrogation, hand back the list of lines. In
any other phase, hand back an empty list.

**Your own code with this shape** — `transcript.rs :: turn_count()`, a few lines above:

```rust
    pub fn turn_count(&self) -> usize {
        match self {
            Phase::Interrogating { turns, .. } => turns.len(),
            _ => 0,
        }
    }
```

`transcript()` is the same `match`. Only the two answers change:

| | `turn_count()` hands back | `transcript()` hands back |
|---|---|---|
| in the room | `turns.len()` — how many lines | `turns` — the lines themselves |
| anywhere else | `0` | `&[]` — an empty list |

**The one new thing: `&[Turn]`**, the return type. Read it as "a view onto a list of `Turn`s that
someone else keeps". It is to `Vec<Turn>` what `&str` is to `String` (Stage 3): the phase keeps the
list, the screen only looks at it. `&[]` is a view onto an empty list, and needs no list to exist —
that is why the briefing and the report can answer.

Why Rust bothers: in TypeScript a getter returning `this.turns` hands out your real array, and the
caller can push to it behind your back. `&[Turn]` is look-but-don't-touch, checked by the compiler.
Where the analogy stops: TypeScript has no way to say that at all.

**Before running:** write how many tests you expect to pass, marked `sure` or `guessing`. Then run
`cargo test --test phase_transcript`. Measured: **2 passed; 4 failed** — the four left all call
`last_line()`.

## 2. Step 2 — the body of `last_line()`

**What to do.** Hand back the newest line if there is one, or "nothing" if the list is empty.

**Your own code with this shape** — `case.rs :: suspect()`:

```rust
    pub fn suspect(&self, id: SuspectId) -> Option<&Suspect> {
        self.suspects.iter().find(|suspect| suspect.id == id)
    }
```

`Option<&Suspect>` — a view onto one suspect, or `None`. `last_line()` is the same, for a `Turn`.

A list answers "what is your newest item?" the way it answers "how long are you?":

```rust
turns.len()    // how many lines — you wrote this in turn_count()
turns.last()   // Some(view onto the newest line), or None if there are none
```

You can reach the list by matching again, as in step 1, or by calling the method you just wrote.

**Before running:** predict again, `sure` or `guessing`. Measured: **6 passed; 0 failed.** Then
`cargo fmt`, `cargo clippy --all-targets -- -D warnings`, `cargo test` — 130 tests across sixteen
files — and say ready.

## 3. The version that passes every test and is still wrong

With `last_line` written as its own `match`, this compiles and all 6 tests go green. I ran it. The
screen calls it on every keystroke — keep that in mind for question 2.

```rust
    pub fn transcript(&self) -> Vec<Turn> {
        match self {
            Phase::Interrogating { turns, .. } => turns.clone(),
            _ => Vec::new(),
        }
    }
```

## 4. If it does not compile — errors I ran, and what each means

| the compiler says | what it means |
|---|---|
| ``expected `Vec<Turn>`, found `&Vec<Turn>` `` | the signature asks for the list; the arm hands a view onto it |
| ``expected `&[Turn]`, found `[_; 0]` `` | the empty arm is missing its `&` |
| ``expected `Option<Turn>`, found `Option<&Turn>` `` | the signature says a line; `.last()` hands back a view onto one |
| `error[E0515]: cannot return value referencing temporary value` | the empty arm built a list inside the method, and the view would outlive it. `&[]` builds nothing |

## 5. When it is green

Closed-book, in chat, in your own words. Two minutes.

1. Why did the compiler refuse your first version?
2. The `.clone()` version in §3 passes every test. What does it cost each time the screen asks, and
   what can its caller do that a caller of `&[Turn]` cannot?
3. `&[Turn]` or `&Vec<Turn>` — which did you have to use, and what stopped the other one?
4. Was any version you tried genuinely unsafe, or fine but unprovable to the compiler?
