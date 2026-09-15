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
use interrogatory_ai_lib::error::AppError;
use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::ipc::{case_intro, CaseIntro, SuspectSummary};
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

// ---------------------------------------------------------------------------
// Stage 9b — the command itself.
//
// `#[tauri::command]` does not change the function it sits on; it writes a
// second one beside it for Tauri to call. So these tests just call yours,
// with no app, no window and no front end running.
// ---------------------------------------------------------------------------

#[test]
fn the_command_finds_a_real_case() {
    // `src-tauri/cases/the-ledger.toml` — the case files the app ships with,
    // as opposed to the fixtures the other tests read.
    let intro = case_intro("the-ledger".to_string()).expect("the-ledger ships with the app");

    assert_eq!(intro.title, "The Ledger");
    assert_eq!(intro.suspects.len(), 2);
}

#[test]
fn the_other_shipped_case_loads_too() {
    let intro = case_intro("the-lighthouse".to_string()).expect("the-lighthouse ships too");

    assert_eq!(intro.title, "The Lighthouse");
    assert_eq!(intro.suspects.len(), 3);
}

#[test]
fn a_slug_with_no_case_behind_it_is_case_not_found() {
    assert_eq!(
        case_intro("the-missing-hour".to_string()),
        Err(AppError::CaseNotFound {
            slug: "the-missing-hour".to_string()
        })
    );
}

#[test]
fn a_slug_from_the_front_end_is_still_not_a_path() {
    // Stage 8 put this check at the door of `storage.rs`. This is the stage
    // where the slug genuinely arrives from outside, and that check is still
    // the only thing between a web page and the filesystem.
    assert_eq!(
        case_intro("../cases/the-ledger".to_string()),
        Err(AppError::CaseNotFound {
            slug: "../cases/the-ledger".to_string()
        })
    );
}

#[test]
fn a_failure_comes_back_as_json_too() {
    // `Err` rejects the promise on the React side, and what it catches is
    // `AppError` turned into JSON — the shape settled back in Stage 5.
    let failure = case_intro("the-missing-hour".to_string()).expect_err("there is no such case");

    assert_eq!(
        serde_json::to_value(failure).expect("AppError turns into JSON"),
        json!({ "kind": "caseNotFound", "slug": "the-missing-hour" })
    );
}
