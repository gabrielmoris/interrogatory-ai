# CONCEPTS — what Gabriel has been taught, and when

This file is the mentor's working memory for pitch and vocabulary. It exists because
"don't use a term he hasn't been taught" is impossible to obey without a list.

## How to use it

**Writing a stage brief:**

- A concept **in this file** gets a *one-line refresher* and a pointer to the stage that defined it.
  Never re-explain it from scratch. Example: "`?` returns early on an error — Stage 5 §4."
- A concept **not in this file** either gets its own numbered section in the brief, or it does not
  belong in this stage. There is no third option.
- A Rust word not in this file and not in the Vocabulary section below must be **defined in the same
  sentence it first appears**, or replaced with plain English.

**Finishing a stage:** add its new rows here, and bump the status of anything he used again without
help. This is part of the review, not optional bookkeeping.

**Status meanings:**

| Status | Means |
|---|---|
| `defined` | Explained once, in the stage named. Assume he needs a refresher line. |
| `used` | Applied again in a later stage without it being re-taught. Refresher can be shorter. |
| `solid` | Reached for unprompted, more than once. Use the term freely, no refresher needed. |

---

## Ledger

### Stage 1 — `Difficulty` and `Tuning`

| Concept | TypeScript anchor | Status |
|---|---|---|
| Module system: `pub mod`, files invisible until declared | `export` / `import`, but the file must be *listed* | solid |
| `struct` vs `enum` — "payload" vs "which one" | interface vs union of literals | solid |
| Field-level `pub` | everything is public in TS unless `#private` | solid |
| Narrow integer types (`u8`) as a way to make nonsense unrepresentable | no equivalent — TS has one `number` | used |
| `impl` blocks | methods on a class, written separately from the data | solid |
| Associated constants (`Difficulty::ALL`) | `static readonly` on a class | used |
| Fixed-size array `[T; N]` vs `Vec<T>` | tuple type vs `T[]` | used |
| `match` as an **expression** that returns a value | `switch` is a statement; closest is a ternary chain | solid |
| Traits, and what `derive` actually generates | interfaces, but `derive` writes the implementation for you | solid |
| **Move semantics and `Copy`** (`E0507`) | nothing in TS — assignment never invalidates the source | used |

### Stage 2 — `SuspectId` and `FactId`

| Concept | TypeScript anchor | Status |
|---|---|---|
| The newtype pattern | branded types (`string & { __brand: 'X' }`), but real | solid |
| Tuple structs (`struct SuspectId(u32)`) | a one-element tuple with a name | solid |
| Field privacy as an escape-hatch-free boundary | `#private`, but enforced at compile time | used |
| `derive` vs a hand-written `impl` | codegen vs writing the method yourself | solid |
| `Debug` vs `Display` | `console.log` shape vs `toString()` | used |
| Trait hierarchy: `Copy: Clone`, `Eq: PartialEq`, `Ord: PartialOrd + Eq` | interface extending another interface | defined |
| The `Hash` / `Eq` contract, and why `f32` can't be a map key | `Map` keys use reference identity; no contract to break | defined |
| Blanket impls — where `.to_string()` and `.into()` come from | a mixin applied to every type that satisfies a condition | defined |
| The orphan rule | no equivalent; TS lets you patch any prototype | defined |
| `From<T>` — a conversion that always works | a constructor overload / mapper function | solid |

### Stage 3 — `Fact`, `Suspect`, `Case`

| Concept | TypeScript anchor | Status |
|---|---|---|
| `String` vs `&str` — structs own, parameters borrow | one `string` type; the split has no TS equivalent | solid |
| `Vec<T>` | `T[]` | solid |
| `HashSet<T>` — the shape for a membership question | `Set<T>` | used |
| `&self` / `&mut self` / `self` as an explicit first parameter | implicit `this` | solid |
| No uninitialized fields — "no data yet" means "what is the empty value" | `undefined` fills the gap in TS | used |
| `E0204` — `Copy` is impossible once a `String` is in the struct | no equivalent | defined |
| `Clone` is not free | structuredClone, but explicit and visible | used |
| `E0382` — use of a moved value | no equivalent | used |
| Closures and `.filter().map().collect()` | arrow functions and array methods; `collect()` is the new part | solid |

### Stage 4 — borrowing, `Option<&T>`, lifetimes

| Concept | TypeScript anchor | Status |
|---|---|---|
| `&T`, `&mut T`, `T` as three distinct things | TS collapses all three into one reference | used |
| The borrow rule: many readers **or** one writer, never both (`E0502`) | no equivalent | used |
| Non-lexical lifetimes — a borrow ends at its last use | no equivalent | defined |
| `Option<&T>` as a return type, with no `null` and no `?.` | `T \| undefined` plus optional chaining | used |
| `iter()` vs `iter_mut()`, and the `_mut` naming convention | one iterator; mutation is unrestricted | used |
| **Lifetimes (`'a`) as a named region relating inputs to outputs**, not a duration | no equivalent | defined |
| Lifetime elision, and why `&self` methods rarely need an annotation | no equivalent | defined |
| `impl Trait` in return position | `ReturnType` inference / an opaque return type | defined |
| Iterator laziness | generators — nothing runs until you pull | defined |
| Closure capture modes, and why `move` is needed (`E0373`) | closures capture by reference implicitly, forever | defined |
| `todo!()` as a stub that type-checks as anything | `throw new Error('TODO')`, but it satisfies any return type | solid |

### Stage 5 — `AppError`, `thiserror`, `Result`

| Concept | TypeScript anchor | Status |
|---|---|---|
| `Result<T, E>` — `Option` with a reason attached | a discriminated union `{ok:true,…} \| {ok:false,…}` | used |
| Why Rust has no exceptions for recoverable failure | `throw` is invisible in the signature; `Result` is not | used |
| `thiserror` as a derive macro that writes `Display` + `std::error::Error` | a decorator that generates `toString()` for you | used |
| `#[error("…")]` as the *body* of the generated `Display` | a template string on the class | used |
| `thiserror` vs `anyhow`, and why this crate is a library boundary | library throws typed errors; an app can catch `unknown` | defined |
| `Option::ok_or` as the bridge from `None` to an error | `?? Promise.reject(…)` | used |
| `ok_or` vs `ok_or_else` — eager vs lazy construction | passing a value vs passing a thunk | defined |
| `?` — early return, and that it calls `From::from` on the error | `await` on a rejected promise inside `try` | used |
| `#[must_use]` on `Result` | `no-floating-promises` lint | defined |
| The unit type `()` | `void` | used |
| `cargo add` and feature flags | `npm i` plus opt-in build flags; no close analogy | defined |
| Type aliases (`pub type AppResult<T>`) | `type Foo<T> = …` | solid |
| serde `Serialize` / internally-tagged enums (`#[serde(tag = "kind")]`) | a discriminated union with a `kind` field | defined |
| Attribute stacking — two `#[…]` lines above one item, order irrelevant | stacked decorators | defined |
| Variable shadowing in the same scope | illegal in TS with `let` | defined |

### Stage 6a — `RawCase` and `Deserialize` ✅

| Concept | TypeScript anchor | Status |
|---|---|---|
| `Deserialize` — text in, Rust value out; `Serialize` backwards | `JSON.parse` into a typed shape | defined |
| `#[serde(default)]` — an optional field with a fallback | an optional property with a default in a zod schema | defined |
| Raw types speak the file's vocabulary (plain `u32`, not `SuspectId`) | the `unknown` you validate before trusting | defined |

### Stage 6b — `TryFrom` and the one road ✅

| Concept | TypeScript anchor | Status |
|---|---|---|
| `TryFrom` — `From` for a conversion that can fail | a parse function returning a result instead of throwing | defined |
| Associated types (`type Error = …`) — a trait can ask for a *type*, not only functions | a generic parameter you fill in on the interface | defined |
| `try_into()` arriving free once `TryFrom` exists | — | defined |
| **`*` — reading the value out of a borrow** | nothing; references are invisible in TS | defined |
| `E0382` from a `for` loop consuming a `Vec` without `&` | — | not met (he wrote the `&` unprompted) |

### Stage 6c–6d — scheduled, not yet taught

Nothing below is `defined` until the stage that owns it has been reviewed. Rows move up into the
ledger proper at review time.

| Concept | Owning stage |
|---|---|
| Validation at the boundary; check order as observable behaviour | 6c |
| Parse-don't-validate as a named pattern | 6c |
| `.map_err` — `.map` for the failure side | 6d |
| Why `?` cannot convert an error it has no `From` for | 6d |

---

## Vocabulary

### Safe to use — introduced and defined

trait, derive, impl block, variant, field, module, borrow, move, own, closure, iterator,
lifetime, elision, match arm, associated constant, associated type, newtype, attribute,
crate, stub, guard clause, dereference (`*`).

### Banned until defined in the same sentence

These were used in chat or in a brief before he had ever met them. Each one cost time.

extern prelude · transitive · orthogonal · supertrait · semver-compatible · desugars ·
internally tagged representation · `&dyn std::error::Error` · monomorphisation · variance ·
interior mutability · zero-cost abstraction · trait object · blanket impl *(named in Stage 2 —
still needs the one-line refresher)*

### Phrasings that have failed

| Said | Why it failed | Say instead |
|---|---|---|
| "a line that keeps nothing" | reads as "a line that does nothing" | "`?` does two jobs: stop and return the error, and hand back the value. Here only job one matters." |
| "one boolean in your filter closure" | jargon stacked three deep, though he'd written the closure | point at the line and name the missing `!` |
| "add one line above the enum" | he could not place it | show the two or three surrounding lines of the real file |
| a "two-line step" containing a `let` annotation, `?` on a foreign error, and `try_into()` | three untaught things in two lines — short is not small | one of them, alone, named first |
| four headed sections in answer to "what have I done wrong?" | he asked about one error and got a diagnosis of everything | answer the error, stop |
