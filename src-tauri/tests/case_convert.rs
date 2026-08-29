//! Stage 6b — the one road from raw data to a case
//!
//! Run with:  cargo test --test case_convert      (from src-tauri/)
//!
//! Stage 6a turned a file into a `RawCase`: numbers, strings, lists. This file
//! pins the trip from there to a `Case` — the type the rest of the app works
//! with, where an id is a `SuspectId` and the visibility rule from Stage 4
//! applies.
//!
//! There is exactly one road between the two, and it is `TryFrom`. Nobody can
//! build a `Case` from a file any other way.
//!
//! Nothing here is about rejecting bad files. Every case in this file is a good
//! one. Stage 6c adds the checks.

use interrogatory_ai_lib::case::Case;
use interrogatory_ai_lib::case_file::RawCase;
use interrogatory_ai_lib::ids::{FactId, SuspectId};

const THE_LEDGER: &str = include_str!("cases/the-ledger.toml");
const THE_LIGHTHOUSE: &str = include_str!("cases/the-lighthouse.toml");

fn marta() -> SuspectId {
    SuspectId::new(1)
}

fn viktor() -> SuspectId {
    SuspectId::new(2)
}

fn raw(text: &str) -> RawCase {
    toml::from_str(text).expect("this text is valid TOML and a complete RawCase")
}

fn ledger() -> Case {
    Case::try_from(raw(THE_LEDGER)).expect("the-ledger.toml is a good case")
}

/// The fact ids this suspect is allowed to talk about, in file order.
fn visible_ids(case: &Case, suspect: SuspectId) -> Vec<FactId> {
    case.suspect_facts(suspect).map(|fact| fact.id).collect()
}

#[test]
fn a_raw_case_becomes_a_case() {
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

    // The file said `id = 1`. Everything past the conversion says
    // `SuspectId::new(1)`. The bare number lives inside `RawCase` and nowhere
    // else in the program.
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
    // known by Marta. It counts towards `fact_count`, and your Stage 4
    // visibility rule still keeps it out of what Marta can say. The conversion
    // has to carry the flag across for that to work.
    assert_eq!(case.fact_count(), 4);
    assert!(!visible_ids(&case, marta()).contains(&FactId::new(9)));
}

#[test]
fn try_into_is_the_same_road_from_the_other_end() {
    // Writing `TryFrom` gives you `try_into()` for free, the same way writing
    // `From<u32>` in Stage 2 gave you `.into()`. Same conversion, read in the
    // other direction.
    let case: Case = raw(THE_LEDGER)
        .try_into()
        .expect("the-ledger.toml is a good case");

    assert_eq!(case.title, "The Ledger");
}

#[test]
fn a_second_case_file_converts_with_the_same_code() {
    let case = Case::try_from(raw(THE_LIGHTHOUSE)).expect("the-lighthouse.toml is a good case");

    assert_eq!(case.title, "The Lighthouse");
    assert_eq!(case.suspect_count(), 3);
    assert_eq!(case.fact_count(), 5);

    // Fact 4 had no `known_by` line, so it belongs to nobody...
    for id in [1, 2, 3].map(SuspectId::new) {
        assert!(
            !visible_ids(&case, id).contains(&FactId::new(4)),
            "fact 4 has no known_by line, so {id} cannot know it"
        );
    }

    // ...and fact 9 is Sofia's, but it is the solution, so she cannot say it.
    assert!(!visible_ids(&case, SuspectId::new(3)).contains(&FactId::new(9)));
}
