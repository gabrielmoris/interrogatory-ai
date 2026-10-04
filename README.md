# Interrogator

A local-first detective game. You read a case, interrogate the suspects, and write a report on who
did what. The suspects are played by a language model running on your own machine. The case's facts
and the score live in Rust, never in the model.

It is also a learning project: Rust and Tauri, learned one stage at a time.

**Stack:** Tauri v2 · Rust · React 19 + TypeScript (Vite) · bun. The local model (`llama-cpp-2`)
arrives in Phase 2; until then a `MockEngine` answers.

## Run

```sh
bun install
bun tauri dev
```

## Test

```sh
cd src-tauri
cargo test
cargo clippy --all-targets -- -D warnings
```

## Where things are

| Path | What |
|---|---|
| `src-tauri/src/` | the Rust backend |
| `src-tauri/tests/` | one test file per stage |
| `src-tauri/cases/` | case files (TOML) |
| `src/` | the React front end (still mostly the Tauri template) |
| `docs/PROGRESS.md` | where the project is, and what comes next |
| `docs/stages/` | one learning doc per stage |
| `docs/ROADMAP.md` | the phases ahead |
| `CLAUDE.md` | how the mentor works in this repo |
