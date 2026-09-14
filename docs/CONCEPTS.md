# CONCEPTS — what Gabriel has been taught, and when

This file is the mentor's working memory for pitch and vocabulary. It exists because
"don't use a term he hasn't been taught" is impossible to obey without a list.

## How to use it

The rules for *writing* a brief live in `CLAUDE.md`, Rules 1 and 2. This file answers one question:
for a given concept, how much explaining does it get?

| Status | Means | What the brief gives it |
|---|---|---|
| *(absent)* | never taught | §1 in full: plain English, the TypeScript if one exists, how it actually works, and the shape in another domain — **printed in complete lines of code**, never named in prose alone. |
| `defined` | explained once, in the stage named | §2, one line that recalls it **and says where**: "`?` returns early on an error — Stage 5 §4." |
| `used` | applied again later without being re-taught | §2, shorter — the name and the stage number. |
| `solid` | reached for unprompted, more than once | mention it and point at two or three places he has already used it. No explanation, no refresher. |
| `shaky` | **he got it wrong again after a refresher** | back to the §1 treatment: reprint the earlier examples and explain it again from zero. A refresher line has already failed on this one; repeating it is the mistake. |

`shaky` outranks everything else in the row. It is set at review time, from what actually went
wrong in the stage, and it is cleared only when he uses the thing correctly without help.

A Rust word that is neither in this ledger nor in the Vocabulary section below must be **defined in
the same sentence it first appears**, or replaced with plain English.

**Finishing a stage** means adding its rows here, bumping anything he used again unaided, and
marking anything he got wrong twice. Part of the review, not bookkeeping.

**How he works, so gaps are not misread as weakness.** He pushes back when he disagrees and is
frequently right — engage the argument. He reaches for iterator chains unprompted and reads compiler
errors rather than asking. What he does not have is Rust's machinery, and only that.

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
| **Lifetimes (`'a`) as a named region relating inputs to outputs**, not a duration | no equivalent | used |
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
| A guard clause as the **shape of a function body** — return early, then carry straight on | an early `return` in TS, same idea | **shaky** (Stage 8: wrapped the rest in `else`, three attempts) |
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

### Stage 6c — the four checks ✅

| Concept | TypeScript anchor | Status |
|---|---|---|
| Validation at the boundary — check once, at the door | `schema.parse()` at the edge of the app | defined |
| `Option` when absence is normal, `Result` when absence is a failure | `T \| undefined` vs throwing | defined |
| `?` as the whole check — a line that stores nothing | `await` a rejecting promise for its side effect | used |
| `.next().is_none()` — asking an iterator whether it has anything | no `.length` on a generator | defined |

### Stage 6d — `parse_case` and the front door ✅

| Concept | TypeScript anchor | Status |
|---|---|---|
| **`?` calls `From::from` on the error on its way out** (`E0277`) | nothing — `await` rethrows the same object | defined |
| `.map_err` — `.map` for the failure side, and the closure **hands back** a value rather than returning | `.catch(e => { throw new MyError(e) })` | **shaky** (Stage 8: wrote `Err(..)` inside it) |
| Parse-don't-validate as a named pattern | a zod schema at the edge, typed everywhere after | defined |
| Only the caller knows the context, so only the caller builds the error | — | defined |

### Stage 7 — `VisibleFact<'a>` ✅

| Concept | TypeScript anchor | Status |
|---|---|---|
| **A struct that holds a borrow — the lifetime parameter on a type** | a class field holding a reference, with nothing checking it | defined |
| `impl<'a>` — introducing the name on the block as well | — | defined |
| A tuple struct's name is also a function (`.map(VisibleFact)`) | a constructor used point-free | defined |
| A private field as the thing that makes a type unforgeable | a factory function plus `#private`, but enforced | used |

### Stage 8 — `storage.rs`, the first shell module ✅

| Concept | TypeScript anchor | Status |
|---|---|---|
| **`Path` / `PathBuf` — the borrowed/owned pair for locations** | one `string` for both | defined |
| `.display()`, because a filename is not guaranteed to be text | `String(p)`, always valid | defined |
| `std::fs::read_to_string` — the whole file, or `std::io::Error` | `readFile(p, 'utf8')`, which throws | defined |
| `io::Error::kind()` / `ErrorKind::NotFound` — which failure it was | `err.code === 'ENOENT'` | defined |
| `.all()` on an empty iterator is `true` | `[].every(…)` is `true` too | defined |
| The shell / domain split as a rule about imports | — | defined |
| `matches!` — "does this value fit this pattern", as a `bool` | a regex/`switch` test collapsed to one expression | used (reached for unprompted) |
| Byte scanning (`.bytes()`) as a safe way to ask an ASCII-only question | no equivalent — JS strings decode either way | used (his choice, not the brief's) |

**Three planned, six landed.** The two that were never meant to be concepts — `read_to_string` and
`.display()` — were the ones he got stuck on, because they were named in prose and never printed as
code. That is where Rule 1's printed-line test comes from: `MENTOR-NOTES.md`, 2026-09-13.

---

## Vocabulary

### Safe to use — introduced and defined

trait, derive, impl block, variant, field, module, borrow, move, own, closure, iterator,
lifetime, elision, match arm, associated constant, associated type, newtype, attribute,
crate, stub, guard clause, dereference (`*`), deserialize, early return, the `?` operator's
conversion, `.map_err`, parse-don't-validate, `Path` / `PathBuf`, `.display()`, `ErrorKind`,
shell module vs domain module.

### Banned until defined in the same sentence

These were used in chat or in a brief before he had ever met them. Each one cost time.

extern prelude · transitive · orthogonal · supertrait · semver-compatible · desugars ·
internally tagged representation · `&dyn std::error::Error` · monomorphisation · variance ·
interior mutability · zero-cost abstraction · trait object · blanket impl *(named in Stage 2 —
still needs the one-line refresher)*

**Due next, so plan the sentence now:** *interior mutability* and *trait object* come off this list
in Stages 10 and 11 respectively — they are those stages' headline concepts, not asides. Do not use
either word before then, including in a roadmap pointer he might read.

*(Phrasings that have already failed moved to `CLAUDE.md`, Rule 4 — they are a
teaching rule, not a concept ledger.)*
