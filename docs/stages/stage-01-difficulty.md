# Stage 1 — `Difficulty` and `Tuning`

**Crate:** `src-tauri`
**Test:** `src-tauri/tests/difficulty.rs`
**Run:** `cd src-tauri && cargo test --test difficulty`
**Est.** 30–45 min if Rust is new to you.

---

## Background — why this task exists

Two things are being taught here, and neither is "enum syntax".

**1. Sum types carry data; TypeScript unions mostly don't.**

In TypeScript you'd reach for a union of string literals and a lookup object:

```ts
type Difficulty = "easy" | "normal" | "hard" | "nightmare";
const TUNING: Record<Difficulty, Tuning> = { ... };
```

That works, but the mapping lives *beside* the type, and nothing forces you to update it when you add a variant. In Rust the mapping lives *on* the type, in an `impl` block, and `match` is **exhaustive** — add a fifth variant and the compiler refuses to build until you have handled it. This is the first taste of the property that makes Rust worth the effort: the compiler enforces things your TS lookup table only hopes for.

**2. `Copy` — your first real ownership decision.**

`Difficulty` has no fields. It is four bytes at most. Copying it is free, and moving it around would be pure friction. So it should be `Copy`: assigning it or passing it to a function duplicates it instead of moving out of the original.

Note the signature the test forces on you:

```rust
Difficulty::ALL.iter().map(|d| d.tuning())
```

`.iter()` yields `&Difficulty` — a *borrow*. If `tuning` takes `self` by value and `Difficulty` is not `Copy`, this fails with **"cannot move out of `*d` which is behind a shared reference"**. That error is the point of the exercise. Read it, understand it, then fix it the right way.

`Tuning`, by contrast, is a struct of real values that will grow over time. Leave it non-`Copy` and return it by value. Ask yourself why the two answers differ.

**3. Where this lands in the game.** `Difficulty` is the knob that turns one case into four experiences. `Tuning` is what Phase 3 hands to the prompt builder and the sampler: `temperature` goes to llama.cpp, `facts_volunteered_per_turn` and `will_lie` go to the interrogation state machine that decides what the suspect is allowed to say this turn. Small type, load-bearing later.

---

## Your task

In `src-tauri`, make `src-tauri/tests/difficulty.rs` compile and pass.

You need:

- A public module `difficulty`, reachable as `interrogatory_ai_lib::difficulty`.
- `pub enum Difficulty` with exactly four variants: `Easy`, `Normal`, `Hard`, `Nightmare`.
- `pub struct Tuning` with three public fields:
  - `temperature: f32`
  - `facts_volunteered_per_turn: u8`
  - `will_lie: bool`
- An associated constant `Difficulty::ALL` — an array of all four, easiest first.
- A method `tuning()` on `Difficulty` returning a `Tuning`.

**Suggested values** (the tests pin Easy and Nightmare exactly; the middle two only have to keep the ordering rules):

| Difficulty | temperature | facts/turn | will_lie |
|---|---|---|---|
| Easy | 0.5 | 2 | false |
| Normal | 0.7 | 1 | false |
| Hard | 0.9 | 1 | true |
| Nightmare | 1.1 | 0 | true |

Three of the seven tests check *properties*, not values: temperature strictly rises, facts volunteered never rises, and dishonesty never reverses. Those encode game-design intent. They are the tests that will still be meaningful in six months when the numbers have changed.

---

## Rules of the loop

1. Do not edit `src-tauri/tests/difficulty.rs`. If you think a test is wrong, say so — that is a legitimate move, and sometimes you will be right.
2. Get to green, then stop. Do not build ahead.
3. From `src-tauri/`, `cargo clippy --all-targets -- -D warnings` must also pass before you call it done.
4. Say **"ready"** and I review.

---

## Hints

Open only what you need, in order.

<details>
<summary><b>Hint 1 — the module doesn't exist yet</b></summary>

`src-tauri/src/lib.rs` currently holds the `greet` demo and the Tauri builder. A file at `src-tauri/src/difficulty.rs` is not part of the crate until `lib.rs` declares it. One line does that, and it needs to be `pub` for the test to reach it.

Look up: "Rust module system — `mod` vs `pub mod`, one file per module."
</details>

<details>
<summary><b>Hint 2 — the compiler is asking for traits</b></summary>

`assert_eq!` needs two things from a type: the ability to compare it, and the ability to print it when the comparison fails. `format!("{:?}", ...)` needs the printable one. You do not implement these by hand — you ask for them in one line above the type.

Look up: "Rust derive macros — `Debug`, `PartialEq`, `Eq`."
</details>

<details>
<summary><b>Hint 3 — "cannot move out of `*d` which is behind a shared reference"</b></summary>

You hit the intended error. `.iter()` lends you each element; it does not give it away. Your method wants to take ownership.

Two ways out. One is to change the method to borrow (`&self`). The other is to make the type cheap enough that the compiler will silently duplicate it instead of moving. For a fieldless enum, the second is the idiomatic answer, and it is one word added to a derive list — plus its prerequisite trait, which the compiler will name if you forget it.

Look up: "Rust `Copy` and `Clone` — why `Copy` requires `Clone`."
</details>

<details>
<summary><b>Hint 4 — the constant</b></summary>

`ALL` is namespaced under the type (`Difficulty::ALL`), not free-floating. That means it is declared inside an `impl Difficulty` block. Its type is a fixed-size array, written `[Difficulty; 4]`, not a `Vec`.

Also note `for d in Difficulty::ALL` in the last test iterates the array **by value** — another place `Copy` quietly saves you.

Look up: "Rust associated constants."
</details>

<details>
<summary><b>Hint 5 — near-complete shape</b></summary>

```rust
// src-tauri/src/difficulty.rs

#[derive(/* ??? */)]
pub enum Difficulty {
    // four variants
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tuning {
    // three pub fields
}

impl Difficulty {
    pub const ALL: [Difficulty; 4] = [/* ... */];

    pub fn tuning(self) -> Tuning {
        match self {
            // one arm per variant, each building a Tuning
        }
    }
}
```

Fill the blanks. Do not add anything not listed.
</details>
