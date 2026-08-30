# archive — ROADMAP sections removed 2026-08-30

Stale or duplicated. Removed by the 2026-08-30 audit. **Do not act on any of it.**

- §0 described the repo as an unmodified scaffold at one commit with CRLF churn — all three defects
  were cleared in Phase 0.
- The Phase 0 checklist was a second, divergent copy of `PROGRESS.md`'s "Phase 0 leftovers".
- "Next three actions" was struck through in full and superseded by the stage queue in `PROGRESS.md`.

---

## 0. Current state of the repo (as audited)

`interrogatory-ai` is an unmodified `create-tauri-app` scaffold: Tauri v2, React 19, Vite 7, TypeScript 5.8. One commit (`0d27598 init commit`). `src-tauri/src/lib.rs` contains only the `greet` demo command. There is no module structure, no error type, no managed state, no domain model.

Three defects to clear before any feature work:

1. **CRLF churn.** `git status` shows all 20 files modified; `git diff --stat` is `2479 insertions(+), 2479 deletions(-)`. Every file was committed with LF and checked out as CRLF. There is no `.gitattributes` and `core.autocrlf` is unset. Every future diff will be unreadable until this is fixed.
2. **Package-manager mismatch.** `tauri.conf.json` runs `bun run dev` / `bun run build`, but the repo has `package-lock.json` and no bun lockfile. Pick one, delete the other's lockfile.
3. **Demo code still present.** `greet`, the Vite/React/Tauri logo page, `src/assets/react.svg`. Delete it in Phase 0 — leaving it invites cargo-culting its shape.

---


---

## Phase 0 — Repo hygiene (1 session, ~2 h)

No Rust learning here. This is removing friction that would otherwise tax every future session.

- [ ] Add `.gitattributes`: `* text=auto eol=lf`, plus `*.png binary`, `*.ico binary`, `*.icns binary`. Then `git add --renormalize . && git commit`.
- [ ] Choose npm **or** bun. Align `tauri.conf.json`'s `beforeDevCommand`/`beforeBuildCommand` with the lockfile you keep.
- [ ] Delete `greet`, the demo `App.tsx`, `src/assets/react.svg`, `public/vite.svg`.
- [x] ~~Convert to a Cargo workspace~~ — proposed and rejected 2026-08-21, see `DECISIONS.md`.
- [ ] Add deps: `thiserror`, `tracing`, `tracing-subscriber`, `serde`, `toml`.
- [ ] Add `rust-toolchain.toml` pinning a stable version, and `rustfmt.toml`.
- [ ] `.gitignore`: `*.gguf`, `models/`. Model weights never enter git.
- [ ] Rewrite `README.md` to describe Interrogator, not the Tauri template.

**Exit criterion:** `git status` is clean, `cargo clippy -D warnings` passes, `bun run tauri dev` opens a blank window with your own title.

---


---

## Next three actions

*(Superseded 2026-08-29 — the live queue is the stage table in `PROGRESS.md`. Kept for the record:
items 1 and 3 are done, item 2 was rejected.)*

1. ~~Phase 0 in one sitting~~ — done, apart from the leftovers listed in `PROGRESS.md`.
2. ~~Convert to a Cargo workspace with an empty `crates/core`~~ — rejected, see `DECISIONS.md`.
3. ~~Write `Case`, `Suspect`, `Fact`, `Difficulty` and one real case file in TOML~~ — done, Stages
   1–3, with two case files landing in Stage 6.

---

