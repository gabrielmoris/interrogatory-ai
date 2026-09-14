//! Stage 9a — the first command
//!
//! Run with:  cargo test --test commands      (from src-tauri/)
//!
//! Every stage so far has been Rust talking to itself. This one is the first
//! time something outside the process asks Rust a question, and that changes
//! two things.
//!
//! The first is mechanical: what crosses has to survive being flattened into
//! JSON and rebuilt on the other side, so only types serde can write are
//! allowed out. The second is the point of the stage: `Case` is not one of
//! them, and never will be. The intro screen gets a type built for the intro
//! screen — title, briefing, and who you may go and talk to. The facts of the
//! crime have no route out of Rust, which is a much stronger promise than
//! remembering not to send them.

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
    // `u32` — the front end receives 2, not { "0": 2 }.
    let summary = SuspectSummary {
        id: SuspectId::new(2),
        name: "Viktor Lang".to_string(),
    };

    assert_eq!(
        serde_json::to_value(&summary).expect("a suspect summary is JSON"),
        json!({ "id": 2, "name": "Viktor Lang" })
    );
}

#[test]
fn the_intro_has_exactly_three_fields_on_the_wire() {
    let wire = serde_json::to_value(CaseIntro::from(&the_ledger())).expect("an intro is JSON");
    let object = wire.as_object().expect("an intro is a JSON object");

    let mut keys: Vec<&str> = object.keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(keys, ["briefing", "suspects", "title"]);
}

#[test]
fn the_whole_intro_on_the_wire() {
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
fn no_statement_ever_reaches_the_wire() {
    // The-ledger holds four facts. One of them is the solution. Neither the
    // ordinary ones nor the solution have a route to the front end, because
    // the type that crosses has nowhere to put them.
    let wire = serde_json::to_string(&CaseIntro::from(&the_ledger())).expect("an intro is JSON");

    assert!(
        !wire.contains("21:40"),
        "an ordinary fact stayed in Rust: {wire}"
    );
    assert!(
        !wire.contains("forged"),
        "the ground truth stayed in Rust: {wire}"
    );
}

#[test]
fn the_command_is_an_ordinary_function() {
    // No app, no window, no front end running. `#[tauri::command]` writes a
    // second function beside this one; it does not change the one you wrote.
    assert_eq!(
        case_intro("no-such-case".to_string()),
        Err(AppError::CaseNotFound {
            slug: "no-such-case".to_string()
        })
    );
}

#[test]
fn a_slug_from_the_front_end_is_still_not_a_path() {
    // Stage 8 put the slug check at the door of `storage.rs`. This is the
    // stage where the slug genuinely arrives from outside, and the check is
    // still the only thing standing between a web page and the filesystem.
    assert_eq!(
        case_intro("../the-ledger".to_string()),
        Err(AppError::CaseNotFound {
            slug: "../the-ledger".to_string()
        })
    );
}

#[test]
fn a_failure_crosses_as_json_too() {
    // `Err` rejects the promise on the page, and what the page catches is
    // `AppError` serialized — the wire format settled in Stage 5.
    let failure = case_intro("no-such-case".to_string()).expect_err("there is no such case");

    assert_eq!(
        serde_json::to_value(failure).expect("AppError is JSON"),
        json!({ "kind": "caseNotFound", "slug": "no-such-case" })
    );
}
