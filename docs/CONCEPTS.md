# CONCEPTS — what Gabriel has been taught, and when

The ledger `CLAUDE.md` Rules 1–2 read from. A concept's status decides where it appears in a brief:
anything he has met is quoted into §1 from his own code, and only a new concept gets §2. The gather
test in Rule 2 is the only copy of that rule.

**At review time:** add the stage's rows, bump what he used again unaided, and mark `shaky` anything
he got wrong after it had been recalled. `shaky` clears only when he uses it correctly without help.

---

## Recall due — the 1–2–4 schedule

Taught at stage N → asked at N+1, N+3, N+7 (6a–6d count as one). Asked in chat once the stage is
green — not as a section in the next brief (Rule 2, rewritten 2026-09-20).

| Writing stage | Ask about |
|---|---|
| 10 | **Stage 9c** — `.manage` / `State` *(asked in 10a ✅)* · **Stage 9a** — a type built only to be sent *(asked in 10b ✅)* |
| 11 | **Stage 9b**, second pass · **Stage 10** |
| 12 | **Stage 9c**, second pass |
| 13 | **Stage 8**, third pass |
| 14 | **Stage 9a**, third pass |

**`shaky` — asked after every stage until he gets it right twice, and quoted in §1 when the next
stage needs it:**

- **Which phase a move's check names** — the phase you must be in *now*, never the one you end up
  in. Stage 10b: `finish` was written against `Briefing`, then `Reporting`, before `Interrogating`.
- A guard clause as the shape of a function body — Stage 5, wrong again in Stage 8. **1 of 2**: both
  10b guards were the right shape, unaided.
- `.map_err`, and that its closure *hands back* a value rather than returning one — Stage 6d, wrong
  again in Stage 8.

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
| **`*` — reading the value out of a borrow** | nothing; references are invisible in TS | used *(Stage 10a, unprompted)* |
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

### Stage 9a — `CaseIntro`, the type built to be handed out ✅

| Concept | TypeScript anchor | Status |
|---|---|---|
| **A type whose only job is to be sent** — small, owned, built by copying across | the API response type that is not the database row | defined |
| `derive(Serialize)` on a type of his own, and `Case` deliberately never getting it | — | used |
| `impl From<&T>` — the same trait as Stage 2, with a borrow as the source | a mapper that takes an object it does not own | defined |
| **A struct literal must name its struct** — Rust has no anonymous objects (`error: struct literal body without path`) | `{ id, name }` is enough; the shape *is* the type | defined *(wrong once, fixed in one message)* |
| An id is already a `SuspectId` by the time it is inside a `Suspect` — the conversion happened at the door | — | defined |
| `.map(closure)` where Stage 7 used `.map(TypeName)` — named fields need a closure | — | used |

### Stage 9b — the command, and the list ✅

| Concept | TypeScript anchor | Status |
|---|---|---|
| **`#[tauri::command]`** — marks a function and writes a second one beside it; yours is untouched, so tests call it directly | a route handler, with the request/response wiring generated | defined |
| **`generate_handler![…]`** — the list the app actually consults. Marked but unlisted is unreachable, and nothing warns you | the router's route table | defined |
| `pub mod ipc;` declares the module; it does **not** put its items in scope. `ipc::case_intro`, or `use crate::ipc::case_intro` | `import './ipc'` vs a named import | defined |
| Uniform paths — `use ipc::case_intro;` resolves a local module without `crate::` | — | used *(his choice, via rust-analyzer)* |
| `Ok` resolves the promise on the React side, `Err` rejects it with `AppError` as JSON | `resolve` / `reject` | defined |
| `Path::new("some text")` — a borrowed path from a string | — | used |

### Stage 9c — the app holds the case folder ✅

| Concept | TypeScript anchor | Status |
|---|---|---|
| **`.manage(value)`** — hand the app one value at startup, to hold for as long as it runs | a context provider at the top of the tree | defined |
| **`State<'_, T>`** — a command asks for that value in its parameter list, matched by *type*, not name | `useContext(…)` | defined |
| Nothing checks that `.manage` happened — a missing one fails when the command is called, not at compile time | — | defined |
| A command split in two: a plain `_from` function tests call, and a one-line `#[tauri::command]` wrapper | a route handler delegating to a service function | defined |
| `PathBuf::from("some text")` — an owned path from a string | — | defined |

### Stage 10b — moving between phases ✅

| Concept | TypeScript anchor | Status |
|---|---|---|
| **`*self = …`** — a method replacing the whole value it was handed, instead of handing one back | reassigning the state variable, but through the borrow | defined |
| A move names two phases: the check names where you are **now**, `*self =` names where you **end up** | — | **shaky** (crossed them twice) |
| `matches!(self, Variant { .. })` — asking "is it this variant" where the variant carries data; without `{ .. }`, `E0533` | — | used |
| An error variant built at the call site, with a `name()` for the human half | — | used |

### Stage 10a — where the player is ✅

| Concept | TypeScript anchor | Status |
|---|---|---|
| **Data that lives inside one variant** — `Interrogating { suspect, turns }`, and the other variants have no such fields | a union whose members carry different properties | defined |
| **A `match` arm that names the data** — `Phase::Interrogating { suspect, .. }`, where `..` is "ignore the rest" (`E0027` without it) | narrowing on `kind`, then reading the property | defined |
| No field access on an enum — `phase.turns` is `E0609` even after a check; the arm is the only door | — | defined |
| What the arm hands you is a borrow, so `Some(*suspect)` — and `.len()` needs no `*` | — | defined |

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
in Stages 10d and 11 respectively — they are those stages' headline concepts, not asides. Do not use
either word before then, including in a roadmap pointer he might read.

