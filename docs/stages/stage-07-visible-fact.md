# Stage 7 — the only fact a prompt is allowed to see

```
Test:    src-tauri/tests/visible_fact.rs — 9 tests
Run:     cd src-tauri && cargo test --test visible_fact
Writes:  src-tauri/src/case.rs :: VisibleFact, ::id(), ::statement(),
         Case::visible_to(), visible_statements()
Assumes: lifetimes as a named region (4) · tuple structs (2) · private fields (2)
         · iterators and .collect() (3) · suspect_facts (4) · todo!() (4)
Est.     45 min
```

---

## 0. What this has to do, in ordinary words

Later, one function will build the text we send to the model. If it accepts any fact, somebody will
one day hand it the solution and the suspect confesses in the first message.

So: make a second kind of thing — *a fact this suspect may talk about* — and make the case the only
place one can be made. Hand out a list of them per suspect. Anything downstream accepts only that
list. The solution cannot get in, because there is no way to make one of these out of it.

---

## 1. The new idea

### 1a. Headline — a struct that holds a borrow

**Plain English.** A struct usually owns its contents. This one owns nothing: it holds a pointer to
something living somewhere else, and it may only exist as long as that something does. So the struct
says in its own definition: *I come with a borrow attached.*

**TypeScript** — you write this daily without noticing. Nothing stops you keeping the wrapper after
the depot's parcel list is emptied; it compiles, and breaks later at the worst time.

```ts
class DeliveredParcel {
  constructor(private parcel: Parcel) {}   // a reference. No copy.
  get label() { return this.parcel.label }
}
```

**Rust puts the tie in the type:**

```rust
pub struct DeliveredParcel<'a>(&'a Parcel);
```

`<'a>` does not say *how long*. It says **which** borrow — Stage 4's rule, now on a struct instead of
a function. A `DeliveredParcel<'a>` cannot outlive the `Parcel` it points at, and the compiler is the
one enforcing that.

**Where the analogy breaks:** in TypeScript the wrapper and the wrapped thing have unrelated
lifetimes and nobody checks. In Rust they are one relationship, written down and checked. That is the
stage: the guarantee lives in the *type*, not in a rule someone has to remember.

The field is private — no `pub`. Outside the module nobody can build one and nobody can reach past it
to the thing inside. That is what makes it proof rather than convention.

### 1b. Supporting — the `impl` block needs the parameter too

```rust
impl<'a> DeliveredParcel<'a> {
    pub fn label(&self) -> &'a str {
        &self.0.label
    }
}
```

`impl<'a>` reads "for any borrow, call it `'a`" — introducing the name before using it, as
`fn longer_statement<'a>` did in Stage 4. `self.0` is the first field of a tuple struct.

Note `&'a str`, not `&str`: the text belongs to the parcel, not to the wrapper, so the borrow handed
out is the parcel's and stays valid after the wrapper is gone.

### 1c. Supporting — a tuple struct's name is also a function

`DeliveredParcel` is a type, and also a function taking `&Parcel` and returning `DeliveredParcel`. So
it goes straight into `.map`:

```rust
depot.parcels_for(id).map(DeliveredParcel).collect()
```

You will probably write `.map(|p| DeliveredParcel(p))` first. It works, and clippy will tell you to
drop the closure. **Let it** — that lint is how this one is meant to land.

---

## 2. Refresher

- **A lifetime names a region, not a duration** — Stage 4, `longer_statement`.
- **Tuple structs and `self.0`** — Stage 2.
- **A private field is a boundary with no escape hatch** — Stage 2.
- **`case.suspect_facts(id)`** — Stage 4. Already excludes ground-truth-only facts. It stays; this
  stage wraps it, it does not replace it.
- **`todo!()` type-checks as anything** — Stage 4.

---

## 3. Tasks

### 3.1 The logic, step by step

1. In `case.rs`, declare a type holding one borrowed `Fact` and nothing else, with the lifetime
   parameter. Field stays private.
2. In `case.rs :: impl VisibleFact`, give it two ways to be read: the fact's id, and its statement.
   Nothing else — a caller may read it, not reach through it.
3. In `case.rs :: Case::visible_to()`, ask `suspect_facts` for what this suspect may talk about, put
   each one inside the wrapper, collect into a `Vec`.
4. In `case.rs :: visible_statements()`, walk the slice and collect each statement. This is the
   stand-in for the real prompt builder — it exists to prove the gate works.

### 3.2 Scaffolding

```rust
/// A fact one suspect is allowed to talk about, borrowed from the case it came from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VisibleFact<'a>(&'a Fact);

impl<'a> VisibleFact<'a> {
    pub fn id(&self) -> FactId { todo!() }
    pub fn statement(&self) -> &'a str { todo!() }
}

/// The statements of some visible facts, in order.
pub fn visible_statements<'a>(facts: &[VisibleFact<'a>]) -> Vec<&'a str> { todo!() }
```

Inside the existing `impl Case`, next to `suspect_facts`:

```rust
    /// Every fact this suspect may talk about, wrapped so a prompt cannot be
    /// handed anything else.
    pub fn visible_to<'a>(&'a self, suspect: SuspectId) -> Vec<VisibleFact<'a>> {
        todo!()
    }
```

### 3.3 Checkpoints — measured

| After | `cargo test --test visible_fact` |
|---|---|
| step 1 — struct declared, three `todo!()` bodies | 1 passed, 8 failed |
| step 2 — both accessors written | **1 passed, 8 failed — unchanged, and correct** |
| step 3 — `visible_to` written | 8 passed, 1 failed |
| step 4 — `visible_statements` written | 9 passed |

The one passing from the start only asks how big the type is. Step 2 moves nothing because no test
can reach an accessor until `visible_to` hands out a value to call it on.

### 3.4 Cold call

Step 3's body is a shape you wrote in Stage 3 and have not used since — `facts_known_by`. Same
pieces, same order. Go and look at it.

---

## 4. Rules

1. Do not edit `tests/visible_fact.rs`.
2. No `clone()`, no `to_string()`. Reaching for one means the borrow is wrong.
3. Do not touch `suspect_facts` — `visible_to` is built on it. One owner of the rule, still.
4. The field stays private. No `pub` on it, no method handing out the whole `&Fact`.
5. `cargo test`, then `cargo fmt`, then `cargo clippy --all-targets -- -D warnings`. **Clippy has
   exactly one thing to say about this stage.** It is right; the fix is one word.
6. Say **"ready"** and I review.

---

## 5. Hints — parcel depot, translate it yourself

<details>
<summary><b>Hint 1 — where each piece goes</b></summary>

Struct and free function at the bottom of `case.rs`, after `longer_statement`. The accessors in their
own `impl` block beside it. `visible_to` *inside* the existing `impl Case`, next to `suspect_facts`.

</details>

<details>
<summary><b>Hint 2 — the question each body asks</b></summary>

`id()` / `statement()`: "what is inside me?" One field access each — one of them needs a `&`, because
a `String` field is not a `&str`.
`visible_to`: "which facts may this suspect talk about, each put in a box?" You already have the
first half; the rest is one call and a `.collect()`.
`visible_statements`: "for each of these, what does it say?"

</details>

<details>
<summary><b>Hint 3 — the shape, names blanked</b></summary>

```rust
impl<'a> DeliveredParcel<'a> {
    pub fn label(&self) -> &'a str { &self.0.____ }
}

pub fn ____<'a>(parcels: &[DeliveredParcel<'a>]) -> Vec<&'a str> {
    parcels.iter().map(|p| p.____()).collect()
}
```

</details>

<details>
<summary><b>Hint 4 — the depot's step 3, and the lint</b></summary>

```rust
pub fn delivered_for<'a>(&'a self, van: VanId) -> Vec<DeliveredParcel<'a>> {
    self.parcels_for(van).map(DeliveredParcel).collect()
}
```

`.map(DeliveredParcel)`, not `.map(|p| DeliveredParcel(p))` — rule 5's lint,
`clippy::redundant_closure`. The name is already a function taking the borrow and returning the
wrapper; the closure adds a step that does nothing.

</details>
