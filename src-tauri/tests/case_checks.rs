//! Stage 6c — the four things a case file is not allowed to do
//!
//! Run with:  cargo test --test case_checks      (from src-tauri/)
//!
//! A person writing a case file can write anything. They can give two suspects
//! the same id, or say a fact is known by somebody who is not in the cast, or
//! put a witness in the room who has nothing to say.
//!
//! The conversion is where that gets caught, and it is the only place it needs
//! to be caught. Once a `Case` exists, every one of these is already known to
//! be false, everywhere in the app, forever.

use interrogatory_ai_lib::case::Case;
use interrogatory_ai_lib::case_file::RawCase;
use interrogatory_ai_lib::error::{AppError, AppResult};
use interrogatory_ai_lib::ids::{FactId, SuspectId};

const THE_LEDGER: &str = include_str!("cases/the-ledger.toml");
const THE_LIGHTHOUSE: &str = include_str!("cases/the-lighthouse.toml");

fn marta() -> SuspectId {
    SuspectId::new(1)
}

fn viktor() -> SuspectId {
    SuspectId::new(2)
}

/// Every file in this test is valid TOML. What is being tested is the step
/// after that.
fn convert(text: &str) -> AppResult<Case> {
    let raw: RawCase = toml::from_str(text).expect("this text is valid TOML");
    Case::try_from(raw)
}

// ------------------------------------------------------------ the four checks

#[test]
fn a_fact_known_by_a_stranger_is_rejected() {
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
        convert(KNOWN_BY_A_STRANGER),
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
        convert(TWO_MARTAS),
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
        convert(TWO_FACTS),
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
        convert(SILENT_VIKTOR),
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
    // let this file through. The check has to ask the same question the game
    // asks: what can this suspect actually say?
    assert_eq!(
        convert(VIKTOR_KNOWS_THE_ENDING),
        Err(AppError::SuspectKnowsNothing { id: viktor() })
    );
}

#[test]
fn the_real_case_files_still_pass_every_check() {
    // The checks have to reject bad files without rejecting good ones.
    assert!(convert(THE_LEDGER).is_ok());
    assert!(convert(THE_LIGHTHOUSE).is_ok());
}

// -------------------------------------------------- the three new failures

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
