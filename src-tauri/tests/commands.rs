//! Stage 9a — the shape the screen gets
//!
//! Run with:  cargo test --test commands      (from src-tauri/)
//!
//! A `Case` holds everything: the title, the briefing, the cast, and every
//! fact of the crime including the solution. The briefing screen is allowed
//! three of those things and no more.
//!
//! So this file is about a second, smaller type — one that has only what the
//! screen draws, and that knows how to turn itself into text to be sent. The
//! promise is not that we remember to leave the facts out. It is that a `Case`
//! cannot be turned into text at all, so there is no way to send one by
//! accident.

use interrogatory_ai_lib::case::Case;
use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::ipc::{CaseIntro, SuspectSummary};
use interrogatory_ai_lib::storage::load_case;
use serde_json::json;
use std::path::{Path, PathBuf};

/// `src-tauri/tests/cases` — the two real case files, wherever this repo sits.
fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("cases")
}

fn the_ledger() -> Case {
    load_case(&fixtures(), "the-ledger").expect("the-ledger.toml is there, and is a case")
}

#[test]
fn the_intro_carries_the_title_and_the_briefing() {
    let intro = CaseIntro::from(&the_ledger());

    assert_eq!(intro.title, "The Ledger");
    assert_eq!(
        intro.briefing,
        "A bookkeeper is dead and the safe is short 40,000."
    );
}

#[test]
fn the_intro_carries_every_suspect_in_file_order() {
    // The screen lists the cast in the order the case file gave them, so the
    // conversion walks the case rather than picking suspects out one by one.
    let intro = CaseIntro::from(&the_ledger());

    assert_eq!(
        intro.suspects,
        vec![
            SuspectSummary {
                id: SuspectId::new(1),
                name: "Marta Reyes".to_string(),
            },
            SuspectSummary {
                id: SuspectId::new(2),
                name: "Viktor Lang".to_string(),
            },
        ]
    );
}

#[test]
fn a_suspect_crosses_as_an_id_and_a_name() {
    // `SuspectId` is a newtype around one `u32`, and serde sends it as that
    // `u32` — what arrives is 2, not { "0": 2 }.
    let summary = SuspectSummary {
        id: SuspectId::new(2),
        name: "Viktor Lang".to_string(),
    };

    assert_eq!(
        serde_json::to_value(&summary).expect("a suspect summary turns into JSON"),
        json!({ "id": 2, "name": "Viktor Lang" })
    );
}

#[test]
fn the_intro_has_exactly_three_fields() {
    let wire = serde_json::to_value(CaseIntro::from(&the_ledger())).expect("an intro is JSON");
    let object = wire.as_object().expect("an intro is a JSON object");

    let mut keys: Vec<&str> = object.keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(keys, ["briefing", "suspects", "title"]);
}

#[test]
fn the_whole_intro_as_it_will_be_sent() {
    // This is the contract the React side will be written against.
    assert_eq!(
        serde_json::to_value(CaseIntro::from(&the_ledger())).expect("an intro is JSON"),
        json!({
            "title": "The Ledger",
            "briefing": "A bookkeeper is dead and the safe is short 40,000.",
            "suspects": [
                { "id": 1, "name": "Marta Reyes" },
                { "id": 2, "name": "Viktor Lang" },
            ],
        })
    );
}

#[test]
fn no_statement_ever_gets_out() {
    // The-ledger holds four facts. One of them is the solution. Neither the
    // ordinary ones nor the solution can leave, because the type that leaves
    // has nowhere to put them.
    let wire = serde_json::to_string(&CaseIntro::from(&the_ledger())).expect("an intro is JSON");

    assert!(
        !wire.contains("21:40"),
        "an ordinary fact stayed in Rust: {wire}"
    );
    assert!(
        !wire.contains("forged"),
        "the solution stayed in Rust: {wire}"
    );
}
