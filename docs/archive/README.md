# archive

Superseded documents, kept only so a git blame or an old link still resolves.

**Nothing in this folder is current. Do not act on it.**

- `2026-08-21-stage-01-kickoff.md` — the Stage 1 kickoff message, formerly `task.md` at the repo
  root. It asserts a `crates/core` workspace that was rejected on 2026-08-21, tells you to run
  `cargo test -p interrogator-core` and `npm run tauri dev`, and references a `package-lock.json`
  that no longer exists. Three settled decisions contradicted in one file. Archived 2026-08-29.
- `2026-08-27-CLAUDE.md` — `CLAUDE.md` as it stood before the 2026-08-29 rewrite. Kept because
  `MENTOR-NOTES.md` quotes it.
- `2026-08-27-tests-case_file.rs` — the single 16-test Stage 6 spec, before it was split into
  `case_raw.rs` / `case_convert.rs` / `case_checks.rs` / `case_parse.rs`. Do not re-add it to
  `src-tauri/tests/`; cargo compiles every `.rs` directly in that folder and it would collide.
- `2026-08-27-stage-06-case-files.md` — the single 657-line Stage 6 brief, ten new concepts in one
  stage. Superseded by `stage-06a` … `stage-06d`. Kept as the worked example of what the concept
  budget in `CLAUDE.md` Rule 1 exists to prevent.
