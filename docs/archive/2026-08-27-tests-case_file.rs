//! Stage 6 — case files: parse, don't validate
//!
//! Run with:  cargo test --test case_file      (from src-tauri/)
//!
//! Every case so far has been built by hand, line by line, inside a test. A
//! real case comes from a file on disk, written by a person, and a person can
//! write anything: a fact known by a suspect who is not in the cast, the same
//! id twice, a witness with nothing to say.
//!
//! This file pins the shape of the answer. Two types instead of one: `RawCase`
//! is whatever was in the file, and `Case` is a case that has been checked. The
//! only road from the first to the second is `TryFrom`, so once you are holding
//! a `Case`, the checks have already happened — there is no way to have skipped
//! them.

use interrogatory_ai_lib::case::Case;
use interrogatory_ai_lib::case_file::{parse_case, RawCase};
use interrogatory_ai_lib::error::AppError;
use interrogatory_ai_lib::ids::{FactId, SuspectId};

/// `include_str!` reads the file at compile time and drops its text into the
/// binary as a `&'static str`. No filesystem call happens at run time — that is
/// Stage 8's job. These are real files; open them next to this one.
const THE_LEDGER: &str = include_str!("cases/the-ledger.toml");
const THE_LIGHTHOUSE: &str = include_str!("cases/the-lighthouse.toml");

fn marta() -> SuspectId {
    SuspectId::new(1)
}

fn viktor() -> SuspectId {
    SuspectId::new(2)
}

fn ledger() -> Case {
    parse_case(THE_LEDGER, "cases/the-ledger.toml").expect("the-ledger.toml is a valid case")
}

/// The fact ids this suspect is allowed to talk about, in file order.
fn visible_ids(case: &Case, suspect: SuspectId) -> Vec<FactId> {
    case.suspect_facts(suspect).map(|fact| fact.id).collect()
}

// ------------------------------------------------------ a real file becomes a case

#[test]
fn a_case_file_becomes_a_case() {
    let case = ledger();

    assert_eq!(case.title, "The Ledger");
    assert_eq!(
        case.briefing,
        "A bookkeeper is dead and the safe is short 40,000."
    );
    assert_eq!(case.suspect_count(), 2);
    assert_eq!(case.fact_count(), 4);
}

#[test]
fn ids_arrive_as_newtypes_not_as_numbers() {
    let case = ledger();

    // The file says `id = 1`. Everything past the conversion says
    // `SuspectId::new(1)`. The bare number exists only inside the file and
    // inside `RawCase`; it never reaches the rest of the app.
    let suspect = case.suspect(marta()).expect("Marta is in this case");
    assert_eq!(suspect.name, "Marta Reyes");
    assert_eq!(suspect.id, marta());
}

#[test]
fn known_by_survives_the_conversion() {
    let case = ledger();

    assert_eq!(
        visible_ids(&case, marta()),
        vec![FactId::new(1), FactId::new(2)]
    );
    assert_eq!(
        visible_ids(&case, viktor()),
        vec![FactId::new(1), FactId::new(3)]
    );
}

#[test]
fn the_solution_is_loaded_but_never_visible() {
    let case = ledger();

    // Fact 9 is in the file, is marked `is_ground_truth_only`, and is listed as
    // known by Marta. It counts towards `fact_count`, and the visibility rule
    // from Stage 4 still keeps it out of what Marta can say.
    assert_eq!(case.fact_count(), 4);
    assert!(!visible_ids(&case, marta()).contains(&FactId::new(9)));
}

#[test]
fn missing_optional_fields_fall_back_to_empty() {
    let case = parse_case(THE_LIGHTHOUSE, "cases/the-lighthouse.toml")
        .expect("the-lighthouse.toml is a valid case");

    // Fact 4 has no `known_by` line at all, so nobody knows it...
    for id in [1, 2, 3].map(SuspectId::new) {
        assert!(
            !visible_ids(&case, id).contains(&FactId::new(4)),
            "fact 4 has no known_by line, so {id} cannot know it"
        );
    }

    // ...and fact 1 has no `is_ground_truth_only` line, so it is a normal fact.
    assert!(visible_ids(&case, SuspectId::new(1)).contains(&FactId::new(1)));
}

#[test]
fn a_second_case_file_parses_with_the_same_code() {
    let case = parse_case(THE_LIGHTHOUSE, "cases/the-lighthouse.toml")
        .expect("the-lighthouse.toml is a valid case");

    assert_eq!(case.title, "The Lighthouse");
    assert_eq!(case.suspect_count(), 3);
    assert_eq!(case.fact_count(), 5);
}

// ------------------------------------------------------------ parse, don't validate

/// Valid TOML. Not a valid case: fact 1 is known by suspect 7, who is not here.
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

#[test]
fn raw_case_accepts_what_case_rejects() {
    // This is the whole idea of the stage, in one test. `RawCase` is the file's
    // vocabulary: numbers, strings, lists. It deserializes happily, because
    // nothing about `known_by = [1, 7]` is malformed TOML.
    let raw: RawCase = toml::from_str(KNOWN_BY_A_STRANGER).expect("this is valid TOML");
    assert_eq!(raw.title, "The Stranger");
    assert_eq!(raw.facts.len(), 1);
    assert_eq!(raw.facts[0].known_by, vec![1, 7]);

    // `Case` is the game's vocabulary, and suspect #7 does not exist in it.
    // Same text, different type, different answer.
    assert!(parse_case(KNOWN_BY_A_STRANGER, "cases/stranger.toml").is_err());
}

// -------------------------------------------------------------- what gets rejected

#[test]
fn a_fact_known_by_a_stranger_is_rejected() {
    assert_eq!(
        parse_case(KNOWN_BY_A_STRANGER, "cases/stranger.toml"),
        Err(AppError::SuspectNotFound {
            id: SuspectId::new(7)
        })
    );
}

#[test]
fn the_same_suspect_id_twice_is_rejected() {
    const TWO_MARTAS: &str = r#"
title = "Two Martas"
briefing = "One id, two people."

[[suspects]]
id = 1
name = "Marta Reyes"

[[suspects]]
id = 1
name = "Marta Reyes the second"

[[facts]]
id = 1
statement = "The safe was opened at 21:40."
known_by = [1]
"#;

    assert_eq!(
        parse_case(TWO_MARTAS, "cases/two-martas.toml"),
        Err(AppError::DuplicateSuspect { id: marta() })
    );
}

#[test]
fn the_same_fact_id_twice_is_rejected() {
    const TWO_FACTS: &str = r#"
title = "Two Facts"
briefing = "One id, two statements."

[[suspects]]
id = 1
name = "Marta Reyes"

[[facts]]
id = 1
statement = "The safe was opened at 21:40."
known_by = [1]

[[facts]]
id = 1
statement = "The safe was opened at 22:10."
known_by = [1]
"#;

    assert_eq!(
        parse_case(TWO_FACTS, "cases/two-facts.toml"),
        Err(AppError::DuplicateFact { id: FactId::new(1) })
    );
}

#[test]
fn a_suspect_with_nothing_to_say_is_rejected() {
    const SILENT_VIKTOR: &str = r#"
title = "The Silent Witness"
briefing = "Viktor is in the cast and knows nothing at all."

[[suspects]]
id = 1
name = "Marta Reyes"

[[suspects]]
id = 2
name = "Viktor Lang"

[[facts]]
id = 1
statement = "The safe was opened at 21:40."
known_by = [1]
"#;

    // An interrogation with this suspect could only ever be an empty room.
    // Better to refuse the file than to ship the case and find out in play.
    assert_eq!(
        parse_case(SILENT_VIKTOR, "cases/silent-viktor.toml"),
        Err(AppError::SuspectKnowsNothing { id: viktor() })
    );
}

#[test]
fn knowing_only_the_solution_counts_as_knowing_nothing() {
    const VIKTOR_KNOWS_THE_ENDING: &str = r#"
title = "The Silent Culprit"
briefing = "Viktor knows exactly one thing, and it is the answer."

[[suspects]]
id = 1
name = "Marta Reyes"

[[suspects]]
id = 2
name = "Viktor Lang"

[[facts]]
id = 1
statement = "The safe was opened at 21:40."
known_by = [1]

[[facts]]
id = 9
statement = "Viktor forged the second signature."
known_by = [2]
is_ground_truth_only = true
"#;

    // Viktor's `known_by` list is not empty, so counting the raw lists would
    // pass this file. The check has to ask the same question the game asks:
    // what can this suspect actually say?
    assert_eq!(
        parse_case(VIKTOR_KNOWS_THE_ENDING, "cases/silent-culprit.toml"),
        Err(AppError::SuspectKnowsNothing { id: viktor() })
    );
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
    // No `briefing` anywhere in the file. serde needs it and says so.
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

// ------------------------------------------------------- the three new failures

#[test]
fn the_new_variants_read_like_sentences() {
    assert_eq!(
        AppError::DuplicateSuspect { id: marta() }.to_string(),
        "suspect #1 appears twice in this case file"
    );
    assert_eq!(
        AppError::DuplicateFact { id: FactId::new(1) }.to_string(),
        "fact #1 appears twice in this case file"
    );
    assert_eq!(
        AppError::SuspectKnowsNothing { id: viktor() }.to_string(),
        "suspect #2 knows nothing they are allowed to talk about"
    );
}

#[test]
fn the_new_variants_still_reach_react_as_json() {
    let json = serde_json::to_string(&AppError::DuplicateSuspect { id: marta() })
        .expect("AppError is Serialize");

    assert_eq!(json, r#"{"kind":"duplicateSuspect","id":1}"#);
}
