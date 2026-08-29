//! Stage 6a — the file's own vocabulary
//!
//! Run with:  cargo test --test case_raw      (from src-tauri/)
//!
//! Every case so far has been built by hand, line by line, inside a test. A
//! real case comes from a file that a person wrote.
//!
//! This file is only about the first half of that journey: turning the text of
//! a file into Rust data. Nothing here checks whether the case makes sense.
//! `RawCase` is allowed to hold nonsense — that is the point of it, and Stage 6c
//! is where the nonsense gets caught.

use interrogatory_ai_lib::case_file::RawCase;

/// `include_str!` reads the file at compile time and drops its text into the
/// binary as a `&'static str`. No filesystem call happens at run time — that is
/// Stage 8's job. These are real files; open them next to this one.
const THE_LEDGER: &str = include_str!("cases/the-ledger.toml");
const THE_LIGHTHOUSE: &str = include_str!("cases/the-lighthouse.toml");

fn raw(text: &str) -> RawCase {
    toml::from_str(text).expect("this text is valid TOML and a complete RawCase")
}

#[test]
fn a_case_file_becomes_raw_data() {
    let raw = raw(THE_LEDGER);

    assert_eq!(raw.title, "The Ledger");
    assert_eq!(
        raw.briefing,
        "A bookkeeper is dead and the safe is short 40,000."
    );
    assert_eq!(raw.suspects.len(), 2);
    assert_eq!(raw.facts.len(), 4);
}

#[test]
fn the_file_speaks_in_plain_numbers() {
    let raw = raw(THE_LEDGER);

    // Not `SuspectId::new(1)`. The file has no idea what a SuspectId is, and
    // neither does RawCase. Bare numbers, exactly as written.
    assert_eq!(raw.suspects[0].id, 1);
    assert_eq!(raw.suspects[0].name, "Marta Reyes");
    assert_eq!(raw.facts[0].id, 1);
    assert_eq!(raw.facts[0].known_by, vec![1, 2]);
}

#[test]
fn a_missing_known_by_line_means_nobody() {
    let raw = raw(THE_LIGHTHOUSE);

    // Fact 4 in the lighthouse file has no `known_by` line at all. Without a
    // fallback, serde would refuse the whole file over it.
    let orphan = raw
        .facts
        .iter()
        .find(|fact| fact.id == 4)
        .expect("the lighthouse file has a fact 4");

    assert!(orphan.known_by.is_empty());
}

#[test]
fn a_missing_ground_truth_line_means_a_normal_fact() {
    let raw = raw(THE_LIGHTHOUSE);

    let ordinary = raw
        .facts
        .iter()
        .find(|fact| fact.id == 1)
        .expect("the lighthouse file has a fact 1");
    let solution = raw
        .facts
        .iter()
        .find(|fact| fact.id == 9)
        .expect("the lighthouse file has a fact 9");

    // Most facts never mention the flag, so "not mentioned" has to mean false.
    assert!(!ordinary.is_ground_truth_only);
    // And the one that does mention it is still read.
    assert!(solution.is_ground_truth_only);
}

#[test]
fn a_missing_required_field_is_an_error_not_a_guess() {
    // No `briefing` anywhere in the file. That one has no fallback, so serde
    // refuses rather than inventing an empty string.
    const NO_BRIEFING: &str = r#"
title = "The Ledger"

[[suspects]]
id = 1
name = "Marta Reyes"
"#;

    assert!(toml::from_str::<RawCase>(NO_BRIEFING).is_err());
}

#[test]
fn raw_data_accepts_what_a_case_will_not() {
    // Valid TOML. Not a valid case: fact 1 is known by suspect 7, who is not
    // in the cast. RawCase takes it without complaint, because nothing about
    // `known_by = [1, 7]` is malformed text.
    //
    // This is the whole reason the raw type exists. Stage 6c is where suspect
    // #7 gets caught.
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

    let raw = raw(KNOWN_BY_A_STRANGER);

    assert_eq!(raw.title, "The Stranger");
    assert_eq!(raw.facts.len(), 1);
    assert_eq!(raw.facts[0].known_by, vec![1, 7]);
}
