//! Stage 10g — the player picks a suspect
//!
//! Run with:  cargo test --test pick_suspect      (from src-tauri/)
//!
//! The briefing screen lists the suspects. When the player clicks one, React
//! sends two things: which case (its slug) and which suspect (a plain number,
//! the id the briefing screen was given). Rust turns that number back into a
//! `SuspectId`, checks the case really has that suspect, and only then calls
//! them into the room.
//!
//! In 10h a `#[tauri::command]` wraps this, exactly as `case_intro` wraps
//! `case_intro_from`. Here there is no Tauri: tests call the plain function.

use interrogatory_ai_lib::error::AppError;
use interrogatory_ai_lib::ids::SuspectId;
use interrogatory_ai_lib::ipc::begin_interrogation_from;
use interrogatory_ai_lib::state::AppState;
use serde_json::json;
use std::path::Path;

/// The app, reading cases from `src-tauri/tests/cases`.
fn app() -> AppState {
    AppState::new(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("cases"),
    )
}

// ---- the number React sends -------------------------------------------------

#[test]
fn a_suspect_id_arrives_as_a_plain_number() {
    let id: Result<SuspectId, _> = serde_json::from_value(json!(2));

    assert_eq!(id.ok(), Some(SuspectId::new(2)));
}

#[test]
fn text_is_not_a_suspect_id() {
    let id: Result<SuspectId, _> = serde_json::from_value(json!("Viktor"));

    assert!(id.is_err());
}

#[test]
fn any_number_at_all_becomes_a_suspect_id() {
    // The Ledger has suspects 1 and 2. Nothing here knows that.
    let id: Result<SuspectId, _> = serde_json::from_value(json!(99));

    assert_eq!(id.ok(), Some(SuspectId::new(99)));
}

// ---- picking a suspect ------------------------------------------------------

#[test]
fn picking_a_suspect_in_the_case_calls_them_in() {
    let app = app();

    assert_eq!(
        begin_interrogation_from(&app, "the-ledger", SuspectId::new(2)),
        Ok(())
    );
    assert_eq!(app.suspect(), Ok(Some(SuspectId::new(2))));
}

#[test]
fn a_suspect_the_case_does_not_have_is_refused_and_the_room_stays_empty() {
    let app = app();

    assert_eq!(
        begin_interrogation_from(&app, "the-ledger", SuspectId::new(99)),
        Err(AppError::SuspectNotFound {
            id: SuspectId::new(99)
        })
    );
    assert_eq!(app.suspect(), Ok(None), "nobody was called in");
}

#[test]
fn a_case_that_does_not_exist_is_refused() {
    let app = app();

    assert_eq!(
        begin_interrogation_from(&app, "no-such-case", SuspectId::new(1)),
        Err(AppError::CaseNotFound {
            slug: "no-such-case".to_string()
        })
    );
}
