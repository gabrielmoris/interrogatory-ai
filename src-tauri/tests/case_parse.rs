//! Stage 6d — one function, and an error that needs help
//!
//! Run with:  cargo test --test case_parse      (from src-tauri/)
//!
//! Stages 6a to 6c built the two halves: text to `RawCase`, then `RawCase` to
//! a checked `Case`. This file is the front door that joins them —
//! `parse_case(text, path)` — and the one interesting thing that happens on the
//! way, which is that `?` cannot carry the TOML parser's error out of your
//! function on its own.
//!
//! Nothing here touches the filesystem. `path` is a label, not a location;
//! reading real files is Stage 8.

use interrogatory_ai_lib::case_file::parse_case;
use interrogatory_ai_lib::error::AppError;
use interrogatory_ai_lib::ids::SuspectId;

const THE_LEDGER: &str = include_str!("cases/the-ledger.toml");

#[test]
fn a_good_file_goes_all_the_way_through() {
    let case = parse_case(THE_LEDGER, "cases/the-ledger.toml").expect("the-ledger.toml is a case");

    assert_eq!(case.title, "The Ledger");
    assert_eq!(case.suspect_count(), 2);
    assert_eq!(case.fact_count(), 4);
}

#[test]
fn text_that_is_not_toml_names_the_file_it_came_from() {
    const NOT_TOML: &str = "title = \"The Ledger\"\nbriefing =\n[[suspects\n";

    match parse_case(NOT_TOML, "cases/broken.toml") {
        Err(AppError::Parse { path, message }) => {
            // The path is ours, so we assert on it exactly.
            assert_eq!(path, "cases/broken.toml");
            // The message came out of the TOML parser. We carry it and we do
            // not pretend to know its wording — that is the rule from Stage 5:
            // our failures are structured, foreign diagnostics are text.
            assert!(!message.is_empty(), "the parser's own complaint is kept");
        }
        other => panic!("expected AppError::Parse, got {other:?}"),
    }
}

#[test]
fn a_missing_required_field_is_a_parse_error_not_a_panic() {
    // Valid TOML, but serde cannot build a `RawCase` out of it: no `briefing`.
    // That failure comes from the same place as broken syntax does.
    const NO_BRIEFING: &str = r#"
title = "The Ledger"

[[suspects]]
id = 1
name = "Marta Reyes"
"#;

    assert!(matches!(
        parse_case(NO_BRIEFING, "cases/no-briefing.toml"),
        Err(AppError::Parse { .. })
    ));
}

#[test]
fn a_file_that_parses_but_does_not_check_out_still_fails() {
    // Nothing wrong with this as text, so the TOML parser is happy. The
    // failure comes from the conversion instead, and `parse_case` passes it
    // straight through without dressing it up as a Parse error.
    const KNOWN_BY_A_STRANGER: &str = r#"
title = "The Stranger"
briefing = "Somebody who is not in this file knows something."

[[suspects]]
id = 1
name = "Marta Reyes"

[[facts]]
id = 1
statement = "The safe was opened at 21:40."
known_by = [1, 7]
"#;

    assert_eq!(
        parse_case(KNOWN_BY_A_STRANGER, "cases/stranger.toml"),
        Err(AppError::SuspectNotFound {
            id: SuspectId::new(7)
        })
    );
}
