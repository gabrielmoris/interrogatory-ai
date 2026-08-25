# Stage 4 — borrowing, `Option<&T>` and lifetimes

**Crate:** `src-tauri`
**Test:** `src-tauri/tests/borrowing.rs`
**Run:** `cd src-tauri && cargo test --test borrowing`
**Est.** 60–90 min.

---

## Background — why this task exists

Every function you have written so far hands back something the caller *owns*. `tuning()` returns a
fresh `Tuning`. `facts_known_by` returns a `Vec<FactId>` full of copied numbers. Nothing you have
written leaves a thread running back into the data it came from.

That was on purpose, and it does not scale. A `Fact` holds a `String`; a suspect's context window
will hold dozens of them. Copying the whole set of facts every time the LLM layer asks "what does
Marta know" is wasteful, and worse, it means the caller is looking at a snapshot rather than at the
case. The real signature is the one that hands out a **view**:

```rust
pub fn suspect_facts(&self, suspect: SuspectId) -> impl Iterator<Item = &Fact>
```

That `&Fact` is the entire subject of this stage. Once a function returns a reference, the compiler
has to answer a question it never had to ask before: *how long is that reference allowed to live?*
The machinery for answering it is called lifetimes, and it is the last of Rust's big ideas you have
not met. After this stage there is no more new ownership theory — Stage 5 onwards is error handling,
parsing, and Tauri.

**One concept at a time below.** Read a section, write that method, run the test, come back. There
are six sections and you should not read section 6 before you have section 1 compiling.

---

## 0. First: how to stub a file

You hit this in Stage 3 and it cost you time. When you add three methods at once, you want the code
to *compile* immediately so the test suite runs red rather than not building at all. A failing test
tells you something; a build error tells you nothing.

The tool is the `todo!()` macro:

```rust
pub fn suspect(&self, id: SuspectId) -> Option<&Suspect> {
    todo!()
}
```

`todo!()` type-checks as **any** type at all. The compiler accepts it wherever a value is expected,
and if that line is ever reached at runtime it panics with `not yet implemented`. So a file full of
`todo!()` bodies compiles cleanly, the test binary links, and you get a red suite you can drive one
test at a time. (`unimplemented!()` is the same thing with a less friendly message. Both are fine
outside `main.rs` — the no-`unwrap` rule in `CLAUDE.md` is about ignoring errors, not about
deliberately-unfinished code.)

One place it does **not** work, and it is worth knowing why now rather than being surprised later:
a function returning `impl Iterator<Item = &Fact>` cannot be stubbed with `todo!()`. You get

```
error[E0277]: `()` is not an iterator
```

`impl Trait` in a return type means "there is one concrete type here, the compiler works out which
one from the body, and the caller is told only the trait". `todo!()` never returns a value, so there
is no type to work it out from, and the inference falls back to `()` — which is not an iterator.
Stub that one with `std::iter::empty()` instead: a real iterator, of the right item type, that
yields nothing.

**Do this first:** add all four Stage 4 signatures with stub bodies — `todo!()` for three of them,
`std::iter::empty()` for `suspect_facts` — run `cargo test --test borrowing`, and confirm you get
twelve failures rather than a compile error. Then start filling them in.

---

## 1. What `&` actually is

In TypeScript, `const b = a` for an object copies a pointer, and both names now mutate the same
thing. That is the *only* semantics available, and it is invisible in the syntax.

Rust splits that into three, and makes you choose in the type:

| Rust | What the caller gives up | TS equivalent |
|---|---|---|
| `fact: Fact` | ownership — the caller's variable is dead afterwards | no equivalent |
| `fact: &Fact` | nothing — a read-only view, caller keeps the value | `const b = a`, but frozen |
| `fact: &mut Fact` | exclusive access for the duration | `const b = a` |

You have already met the first one: `case.add_fact(f)` **moved** `f` into the case and killed the
variable (`E0382`). A **borrow** — the vocabulary word for `&x` — is the alternative: it lets a
function look at your value and give it back.

The one thing that has no TS analogue at all: a reference in Rust is a *compile-time* concept.
`&Fact` and `Fact` are the same bytes at runtime — there is no wrapper object, no reference count,
no indirection cost beyond the pointer itself. The whole borrow system is erased before codegen.
It exists to make the compiler reject programs, not to make them do anything.

---

## 2. The rule: many readers, or one writer, never both

This is the borrow checker in one line, and it is worth memorising verbatim:

> At any point in the program, a value may have **any number of `&` borrows**, **or exactly one
> `&mut` borrow**, but never both at once.

Why that specific rule: it is the compile-time version of what a mutex gives you at runtime. If
someone can hold a `&Fact` while someone else holds a `&mut Fact`, the reader can be looking at a
`String` that the writer just reallocated. In C++ that is a use-after-free; in JavaScript the
equivalent is mutating an array while iterating it and getting silently wrong results. Rust makes
it a compile error instead.

Second half of the rule, which trips people up until it is said out loud: **a borrow ends at its
last use, not at the end of the block.**

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];      // borrow starts
println!("{first}");    // borrow's last use — it is over here
v.push(4);              // fine, nothing is borrowing v any more
```

Move the `println!` below the `push` and the same code stops compiling. The feature is called
**non-lexical lifetimes** (NLL) and it is why "just add a scope block" advice from old Rust
tutorials is usually unnecessary now.

The test `one_writer_excludes_every_reader` has a commented-out line that violates the rule.
Uncomment it, read `E0502`, comment it back. Five seconds, worth more than another paragraph.

---

## 3. `Option<&T>` — "maybe there is one"

`case.suspect(SuspectId::new(99))` has to be allowed to fail: no such suspect. Rust has no `null`,
so the answer type is `Option<&Suspect>`, an enum with exactly two variants:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

It is `Suspect | undefined` from TypeScript, with the difference that matters: the compiler will not
let you use the value without dealing with the `None` case first. There is no `?.` escape hatch that
silently gives you `undefined` three lines later.

Three ways to open one, in the order you will reach for them:

```rust
match case.suspect(id) {
    Some(s) => s.name.clone(),
    None => "unknown".to_string(),
}

if let Some(s) = case.suspect(id) {     // "match on just this one variant"
    println!("{}", s.name);
}

let s = case.suspect(id).expect("Marta is in this case");   // tests only
```

You do not need to open one in the code you write this stage — you need to *produce* one. The
standard-library method that does it for you is `find`:

```rust
self.suspects.iter().find(|s| s.id == id)      // -> Option<&Suspect>
```

`find` takes the same `|s| ...` you passed to `.filter()` last stage — a **closure**, Rust's arrow
function — and returns the first element it says `true` for, wrapped in `Some`, or `None` if it
never says `true`. Note what it does *not* do: it does not copy the suspect out of the vector. The
`Option` contains a reference into `self.suspects`.

**Checkpoint:** write `suspect`, run `cargo test --test borrowing`. Three tests should pass. Come
back.

---

## 4. `&mut` — editing a fact in place

At the end of Stage 3 you wanted `case.facts[2].reveal_to(...)` and could not have it, because
`facts` is private. This is the method that gives it to you properly:

```rust
pub fn fact_mut(&mut self, id: FactId) -> Option<&mut Fact>
```

Read the signature left to right and every piece is doing work. `&mut self`: this method hands out
write access, so it needs write access itself. `Option`: the id might not exist. `&mut Fact`: the
caller gets to mutate the fact *where it sits in the vector*, without the case ever handing over the
vector.

The iterator you need is `iter_mut()` rather than `iter()`. `iter()` yields `&Fact`; `iter_mut()`
yields `&mut Fact`. Same `find` on top of it.

The `_mut` suffix is a std-library naming convention, not a language feature — `get`/`get_mut`,
`iter`/`iter_mut`, `last`/`last_mut`. Follow it and your API reads like the standard library's.

**Checkpoint:** write `fact_mut`, run the test, then do the `E0502` experiment from section 2.
Six tests should pass.

---

## 5. Lifetimes — what `'a` actually means

Now the part everyone finds hard, mostly because it is usually explained backwards.

A lifetime is **not** a duration, and writing `'a` does not make anything live longer. `'a` is a
*name for a region of code*, and putting it in a signature states a **relationship** the compiler
must then verify:

```rust
pub fn longer_statement<'a>(a: &'a Fact, b: &'a Fact) -> &'a Fact
```

This says: "the reference I return is valid for whatever region both inputs are valid for." It is a
claim about the returned reference, checked at every call site. If you call it with one `Fact` that
dies at the end of the block and try to keep the result past that point, the call site — not the
function — fails to compile.

So why have you never written one? Because of **lifetime elision**: three rules that let the
compiler fill them in silently when the answer is obvious. The relevant one:

> If a method takes `&self`, the elided output lifetime is `&self`'s.

That covers essentially every method on `Case`, which is why `fn suspect(&self, id: SuspectId) ->
Option<&Suspect>` needs no annotation. There is exactly one input reference it could have come from.

`longer_statement` is the case where that breaks: two input references, one output, and the compiler
has no basis for guessing which one it borrows from. Write it without the annotation first and read
`E0106` — the error message spells out the reasoning and then hands you the exact fix. This function
is admittedly a small thing to build (it picks the longer of two statements; the UI will want it for
choosing a pull-quote). It is in the stage because it is the smallest place where the annotation is
mandatory, and you need to be able to read `'a` in an error message.

**Checkpoint:** write `longer_statement`, run the test. Eight tests pass. One section left.

---

## 6. `impl Iterator<Item = &Fact>` — returning a view, lazily

```rust
pub fn suspect_facts(&self, suspect: SuspectId) -> impl Iterator<Item = &Fact>
```

Two new things in that return type.

**`impl Trait` in return position.** `.iter().filter(...)` does not produce a `Vec`. It produces a
`std::iter::Filter<std::slice::Iter<'_, Fact>, [closure@src/case.rs:69:21]>` — a type that names
your specific closure and cannot be written down by hand. `impl Iterator<Item = &Fact>` means "some
concrete type I am not going to name, and all you may assume about it is that it iterates and yields
`&Fact`". TypeScript has nothing quite like it; the nearest thing is returning `Iterable<Fact>` from
a generator, except that Rust's version is fully static — no boxing, no dynamic dispatch, the
adapters compile down to the same loop you would have written by hand.

**Laziness.** Nothing is visited when the function returns. `filter` builds a struct holding the
underlying iterator and your closure; the work happens inside `next()`, one element at a time, when
someone calls `.collect()` or `.count()` or a `for` loop. That is what the test
`suspect_facts_does_no_work_until_it_is_asked` is checking. It also means `.take(3)` on a million-element case visits three
elements, not a million.

Now the error you are going to hit, and it is the reason this method is last:

```
error[E0373]: closure may outlive the current function, but it borrows `suspect`,
              which is owned by the current function
help: to force the closure to take ownership of `suspect` (and any other
      referenced variables), use the `move` keyword
```

Think about the timing. `suspect: SuspectId` is a parameter — a local variable, gone the moment the
function returns. But the iterator you return *outlives the function* and still needs to consult
`suspect` every time someone calls `next()`. By default a closure borrows what it mentions, which
would leave it pointing at a dead local. `move` changes the capture mode: the closure takes
ownership of `suspect` and carries a copy inside itself.

Doing that is free here — `SuspectId` is `Copy`, which is exactly why Stage 2 made it so.

**Watch for this in review:** `suspect_facts` and `facts_known_by` now answer the same question in
two shapes, owned ids versus borrowed facts. Keeping both is defensible and so is deleting one.
Have an opinion ready.

---

## Your task

Make `src-tauri/tests/borrowing.rs` compile and pass. Everything goes in `src-tauri/src/case.rs`.
Signatures given, bodies elided — the shapes are scaffolding, the bodies are the exercise:

```rust
impl Case {
    // ... Stage 3 methods stay as they are ...

    /// The suspect with this id, or `None` if the case has no such suspect.
    pub fn suspect(&self, id: SuspectId) -> Option<&Suspect> { todo!() }

    /// Exclusive access to one fact, so callers can reveal it or edit it in place.
    pub fn fact_mut(&mut self, id: FactId) -> Option<&mut Fact> { todo!() }

    /// Every fact this suspect knows and is allowed to see, borrowed from the case.
    /// Same visibility rule as `facts_known_by`: ground-truth-only facts never appear.
    pub fn suspect_facts(&self, suspect: SuspectId) -> impl Iterator<Item = &Fact> {
        std::iter::empty() // `todo!()` does not compile here — see section 0
    }
}

/// Whichever of the two facts has the longer statement; `a` on a tie.
pub fn longer_statement<'a>(a: &'a Fact, b: &'a Fact) -> &'a Fact { todo!() }
```

`longer_statement` is a free function at module level, not in the `impl` block — it belongs to no
particular `Fact`. Byte length (`.len()`) is fine; do not go near grapheme clusters.

---

## Rules of the loop

1. Do not edit `src-tauri/tests/borrowing.rs`. If a test looks wrong, say so — that is a legitimate
   move and you have been right before.
2. `Case`'s collections stay private. No method returns `&Vec<Fact>` or `&mut Vec<Fact>`; that hands
   out the invariant along with the data.
3. No `clone()` anywhere in this stage's code. Cloning your way out of a borrow error is the thing
   this stage exists to teach you not to do. If you want one, that is the moment to stop and ask.
4. Write `longer_statement` without the lifetime annotation first and read `E0106` before fixing it.
5. `cargo fmt`, then `cargo clippy --all-targets -- -D warnings` must pass.
6. Say **"ready"** and I review.

---

## Hints

Open only what you need, in order. Weather domain, as always — translate it yourself.

<details>
<summary><b>Hint 1 — find one element and return a reference to it</b></summary>

```rust
pub struct Network {
    stations: Vec<Station>,
}

impl Network {
    pub fn station(&self, id: u32) -> Option<&Station> {
        self.stations.iter().find(|s| s.id == id)
    }
}
```

No `&` on the return of `find` — it already yields `Option<&Station>`, because `iter()` yields
`&Station` and `find` hands back the element it stopped on.

The closure receives `&&Station` (a reference to the reference `iter` produced). Field access and
method calls auto-dereference through both, so `s.id` just works. You met this in Stage 3's Hint 4.
</details>

<details>
<summary><b>Hint 2 — the same thing, but writable</b></summary>

```rust
impl Network {
    pub fn station_mut(&mut self, id: u32) -> Option<&mut Station> {
        self.stations.iter_mut().find(|s| s.id == id)
    }
}
```

Three `mut`s and each one is load-bearing: `&mut self` because the method hands out write access,
`iter_mut` because `iter` would only yield shared references, `&mut Station` in the return type
because that is what the caller asked for.

Caller side, note that there is no `.clone()` and no reassignment:

```rust
if let Some(s) = net.station_mut(7) {
    s.record(21);          // mutating the station inside the vector
}
```
</details>

<details>
<summary><b>Hint 3 — the annotated free function</b></summary>

```rust
pub fn colder<'a>(a: &'a Reading, b: &'a Reading) -> &'a Reading {
    if b.celsius < a.celsius {
        b
    } else {
        a
    }
}
```

Read the `<'a>` as a declaration, exactly like a generic type parameter: "this function is generic
over some region `'a`". Then `&'a Reading` in three places says all three references share it.

Note the tie behaviour falls out of the `<` versus `<=` choice, and the test pins it down. Read
`longer_statement_prefers_the_first_on_a_tie` before you pick the operator.
</details>

<details>
<summary><b>Hint 4 — returning a lazy iterator</b></summary>

```rust
impl Network {
    pub fn stations_above(&self, threshold: i32) -> impl Iterator<Item = &Station> {
        self.stations
            .iter()
            .filter(move |s| s.is_online && s.celsius > threshold)
    }
}
```

Differences from Stage 3's `facts_known_by`, which is worth putting side by side:

- No `.map()` — you are yielding the elements themselves, not a field of them.
- No `.collect()` — collecting is what turns the view back into an owned `Vec`, which is the thing
  you are deliberately not doing here.
- `move` before the closure parameters. Take it out, read `E0373`, put it back. Section 6 explains
  what the compiler is objecting to.

The `&Station` in the return type has no explicit lifetime and does not need one: elision ties it to
`&self`, which is the only input reference in sight.
</details>

<details>
<summary><b>Hint 5 — near-complete shape</b></summary>

```rust
// appended to src-tauri/src/case.rs

impl Case {
    pub fn suspect(&self, id: SuspectId) -> Option<&Suspect> {
        self.suspects.iter().find(/* ... */)
    }

    pub fn fact_mut(&mut self, id: FactId) -> Option<&mut Fact> {
        self.facts.iter_mut().find(/* ... */)
    }

    pub fn suspect_facts(&self, suspect: SuspectId) -> impl Iterator<Item = &Fact> {
        self.facts
            .iter()
            .filter(move |f| /* the same two conditions as facts_known_by */)
    }
}

pub fn longer_statement<'a>(a: &'a Fact, b: &'a Fact) -> &'a Fact {
    /* compare .statement.len(), return a or b — never a new Fact */
}
```
</details>

---

## Optional, if you finish early

Take the `the_ledger()` builder out of the top of `tests/borrowing.rs` and think about where it
should really live — a `#[cfg(test)]` module in `case.rs`, a `pub fn` used by both test files, or
duplicated on purpose. There is a real trade-off (shared fixtures couple tests to each other) and
Stage 5 turns this builder into a TOML file anyway. Come to review with a position rather than code.

Second one, if you want to feel the rule from section 2 properly: write a method

```rust
pub fn reveal_all_to(&mut self, suspect: SuspectId)
```

that reveals every non-ground-truth fact to one suspect. Try to write it using `suspect_facts` to
pick the facts and `fact_mut` to edit them, and watch it fail — you cannot hold a shared borrow of
the case and ask for an exclusive one. Then find the shape that does work. That collision is the
single most common thing you will fight in real Rust code.
