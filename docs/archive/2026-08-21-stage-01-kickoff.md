Deal. Here's the loop, and Stage 1 is live.

**What I set up in your repo** (harness only — no Rust written for you):

- Root `Cargo.toml` — workspace, `resolver = "2"`, members `crates/core` and `src-tauri`.
- `crates/core/` — the pure crate. `Cargo.toml` with zero dependencies (we add each one at the stage that needs it), and a `lib.rs` that's just the invariant comment.
- `crates/core/tests/difficulty.rs` — **your failing test.**
- `docs/stages/stage-01-difficulty.md` — the brief, with collapsible hints.
- `.gitattributes` with `* text=auto eol=lf`.
- Rewrote `.gitignore`: added `/target`, `models/`, `*.gguf` — and **removed the `docs` line that was in there**, which was silently excluding everything I've written you so far. Check that was unintentional.

Two things for you before Stage 1:

```
git add --renormalize .
git commit -m "chore: normalize line endings, add cargo workspace + core crate"
```

That collapses the 2479-line phantom diff. And run `npm run tauri dev` once — the workspace moves build output from `src-tauri/target/` to `./target/`, so expect one slow rebuild. If the CLI can't find the binary afterward, tell me. (You can delete the stale `src-tauri/target/` yourself; my shell on your machine can't remove files.)

---

## Stage 1 — `Difficulty` and `Tuning`

```
cargo test -p interrogator-core --test difficulty
```

That should fail to compile right now. That's the starting line.

**What you're building:** a `difficulty` module in `crates/core` with a four-variant `Difficulty` enum (`Easy`/`Normal`/`Hard`/`Nightmare`), a `Tuning` struct holding `temperature: f32`, `facts_volunteered_per_turn: u8`, `will_lie: bool`, an associated constant `Difficulty::ALL`, and a `tuning()` method. Exact values are in the brief.

**What it's actually teaching.** Not enum syntax. Two things:

First, in TypeScript you'd write a union plus a `Record<Difficulty, Tuning>` lookup, and nothing forces you to update that lookup when you add a variant. In Rust the mapping lives on the type and `match` is exhaustive — add a fifth difficulty and the compiler refuses to build until you've handled it.

Second, and this is the one that matters: the test calls `Difficulty::ALL.iter().map(|d| d.tuning())`. `.iter()` hands you `&Difficulty` — a borrow. If `tuning` takes `self` by value and `Difficulty` isn't `Copy`, you get **"cannot move out of `*d` which is behind a shared reference."** That error is deliberate. It's your first real ownership decision, and I want you to hit it, read it, and then work out why the right fix is to make the type `Copy` rather than to make the method borrow. Note that `Tuning` gets the opposite answer — think about why before you look it up.

Three of the seven tests check properties rather than values: temperature strictly rises, facts volunteered never rises, dishonesty never reverses. Those encode game design, and they'll still mean something in six months when the numbers have changed.

**Loop rules:**

1. Don't edit the test file. If you think a test is wrong, argue it — you'll sometimes be right, and I'd rather you push back than contort code to satisfy a bad assertion.
2. Green, then stop. Don't build ahead.
3. `cargo clippy -p interrogator-core --all-targets -- -D warnings` also has to pass.
4. Hints are in the brief, collapsed, five of them in increasing order of spoiler. Open the fewest you need — and tell me which number you got to, since that tells me where to aim the next stage.
5. Say **"ready"** and I'll review the actual code, not just the test result.

One open decision I've left alone: `tauri.conf.json` still calls `bun run dev` while you have a `package-lock.json`. Pick one when it next annoys you; it doesn't block Stage 1.
